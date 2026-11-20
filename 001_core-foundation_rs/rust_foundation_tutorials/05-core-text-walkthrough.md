# File 5 — `core-text/` Walkthrough (10 files)

**File 5 of 8 — the 10 files in `core-text/src/`.**

> **What this crate is:** Safe Rust wrappers around Apple's Core Text framework — high-quality text layout and font handling. Built on top of `core-foundation` (for strings, arrays, dictionaries) and `core-graphics` (for drawing contexts, paths, geometry).
>
> **Lines:** ~2,411.
>
> **Same patterns as Phase 3 and 4:** `declare_TCFType!` + `impl_TCFType!` macros, safe methods wrapping `unsafe` FFI, `foreign_types` for some types, `#[repr(C)]` structs for POD types.

---

## Table of Contents

- 5.1 `lib.rs` — crate root
- 5.2 `font.rs` — `CTFont` (the biggest file, 1189 lines)
- 5.3 `font_descriptor.rs` — `CTFontDescriptor`
- 5.4 `font_collection.rs` — `CTFontCollection`
- 5.5 `font_manager.rs` — font manager utilities
- 5.6 `string_attributes.rs` — string attribute constants
- 5.7 `run.rs` — `CTRun` (a run of text with the same style)
- 5.8 `line.rs` — `CTLine` (a laid-out line of text)
- 5.9 `frame.rs` — `CTFrame` (a laid-out frame)
- 5.10 `framesetter.rs` — `CTFramesetter` (creates frames from attributed strings)
- 5.11 Summary

---

## 5.1 `lib.rs` — crate root

**Path:** `core-text/src/lib.rs` (28 lines)

Just module declarations and a doc comment warning that many Core Text functions add to the autorelease pool:

```rust
/*!
Many of these functions will add objects to the autorelease pool.
If you don't have one this will cause leaks.
*/
```

This is a hint that on macOS, you should wrap Core Text calls in an autorelease pool. In metal-cpp terms, this is the same reason you'd create an `NS::AutoreleasePool` at the top of a render frame.

---

## 5.2 `font.rs` — `CTFont` (biggest file)

**Path:** `core-text/src/font.rs` (1189 lines)

`CTFont` is the central Core Text type — an opaque reference to a loaded font. The file is long because it wraps ~80 CTFont functions and declares ~40 font-related constants.

### Constants (lines 40–80)

```rust
pub type CTFontUIFontType = u32;
pub const kCTFontUserFontType: CTFontUIFontType = 0;
pub const kCTFontUserFixedPitchFontType: CTFontUIFontType = 1;
pub const kCTFontSystemFontType: CTFontUIFontType = 2;
// ... 24 more font-type constants

pub type CTFontTableTag = u32;
pub type CTFontTableOptions = u32;
pub const kCTFontTableOptionsNoOptions: CTFontTableOptions = 0;
pub const kCTFontTableOptionsExcludeSynthetic: CTFontTableOptions = 1 << 0;

pub type CTFontOptions = CFOptionFlags;
pub const kCTFontOptionsDefault: CTFontOptions = 0;
pub const kCTFontOptionsPreventAutoActivation: CTFontOptions = 1 << 0;
pub const kCTFontOptionsPreferSystemFont: CTFontOptions = 1 << 2;
```

All `pub const` — compile-time constants matching Core Text's C headers.

### The `CTFont` wrapper

```rust
declare_TCFType! { CTFont, CTFontRef }
impl_TCFType!(CTFont, CTFontRef, CTFontGetTypeID);
impl_CFTypeDescription!(CTFont);
```

Same three-line pattern from File 3. Note: this file uses `core_foundation::declare_TCFType` and `impl_TCFType` — the macros from the `core-foundation` crate, not local ones. This is the **crate-reuse pattern**: a single macro crate provides the boilerplate generator, and many wrapper crates use it.

### Constructors

```rust
impl CTFont {
    pub fn new(name: &str, size: CGFloat) -> CTFont { /* ... */ }
    pub fn from_name(name: &CFString, size: CGFloat) -> CTFont { /* ... */ }
    pub fn from_descriptor(descriptor: &CTFontDescriptor, size: CGFloat) -> CTFont { /* ... */ }
    pub fn from_cg_font(cg_font: &CGFont, size: CGFloat) -> CTFont { /* ... */ }
    pub fn from_core_text_font(ct_font: CTFontRef) -> CTFont { /* ... */ }
    pub fn ui_font(font_type: CTFontUIFontType, size: CGFloat) -> CTFont { /* ... */ }
}
```

Multiple constructors covering the various ways to obtain a `CTFont`: by name, by descriptor, from a CGFont, from a raw ref, from a UI font type.

### Properties (~30 methods)

```rust
pub fn family_name(&self) -> CFString { /* ... */ }
pub fn display_name(&self) -> CFString { /* ... */ }
pub fn post_script_name(&self) -> CFString { /* ... */ }
pub fn full_name(&self) -> CFString { /* ... */ }
pub fn bounding_box(&self) -> CGRect { /* ... */ }
pub fn underline_position(&self) -> CGFloat { /* ... */ }
pub fn underline_thickness(&self) -> CGFloat { /* ... */ }
pub fn x_height(&self) -> CGFloat { /* ... */ }
pub fn ascent(&self) -> CGFloat { /* ... */ }
pub fn descent(&self) -> CGFloat { /* ... */ }
pub fn leading(&self) -> CGFloat { /* ... */ }
pub fn units_per_em(&self) -> usize { /* ... */ }
pub fn glyph_count(&self) -> usize { /* ... */ }
pub fn symbolic_traits(&self) -> CTFontSymbolicTraits { /* ... */ }
// etc.
```

Each is a one-line wrapper around the corresponding `CTFontGet*` C function.

### Glyph operations

```rust
pub fn glyph_for_name(&self, name: &CFString) -> CGGlyph { /* ... */ }
pub fn glyph_for_character(&self, character: u16) -> CGGlyph { /* ... */ }
pub fn glyphs_for_characters(&self, characters: &[u16], glyphs: &mut [CGGlyph]) -> usize { /* ... */ }
pub fn advancement(&self, glyph: CGGlyph) -> CGSize { /* ... */ }
pub fn advancements(&self, glyphs: &[CGGlyph]) -> Vec<CGSize> { /* ... */ }
pub fn bounding_rects(&self, glyphs: &[CGGlyph]) -> Vec<CGRect> { /* ... */ }
```

These let you map characters to glyphs and query per-glyph metrics.

### Drawing

```rust
pub fn draw_glyphs(&self, glyphs: &[CGGlyph], positions: &[CGPoint], context: &CGContext) {
    unsafe {
        CTFontDrawGlyphs(
            self.as_concrete_TypeRef(),
            glyphs.as_ptr(),
            positions.as_ptr(),
            glyphs.len().to_CFIndex(),
            context.as_ptr(),
        )
    }
}
```

Draws a run of glyphs into a `CGContext`. Note the safe Rust signature: takes `&[CGGlyph]` and `&[CGPoint]` (Rust slices), not raw pointers + length. Inside the `unsafe` block, the slices are converted to the C calling convention (`as_ptr()`, `len()`).

### Table access

```rust
pub fn copy_table(&self, table: CTFontTableTag) -> Option<CFData> { /* ... */ }
pub fn copy_table_tags(&self) -> Vec<CTFontTableTag> { /* ... */ }
```

Accesses raw font tables (the binary data inside a `.ttf`/`.otf` file). Useful for advanced typography — OpenType features, kerning tables, etc.

---

## 5.3 `font_descriptor.rs` — `CTFontDescriptor`

**Path:** `core-text/src/font_descriptor.rs` (459 lines)

A font descriptor describes a font by its attributes (family, weight, style, etc.) without instantiating it. Used to search for fonts and to specify desired fonts to `CTFont::from_descriptor`.

### Constants

Lots of constants: `CTFontFormat` (TrueType, OpenType, etc.), `CTFontSymbolicTraits` (italic, bold, monospace, etc.), `CTFontStylisticClass` (sans-serif, serif, etc.).

### The `SymbolicTraitAccessors` trait

```rust
pub trait SymbolicTraitAccessors {
    fn is_italic(&self) -> bool;
    fn is_bold(&self) -> bool;
    fn is_monospace(&self) -> bool;
    // etc.
}

impl SymbolicTraitAccessors for CTFontSymbolicTraits {
    fn is_italic(&self) -> bool { (*self & kCTFontItalicTrait) != 0 }
    fn is_bold(&self) -> bool { (*self & kCTFontBoldTrait) != 0 }
    // etc.
}
```

A nice Rust-idiomatic layer on top of the bitmask constants. Instead of writing `(traits & kCTFontItalicTrait) != 0` everywhere, you write `traits.is_italic()`. This is the **extension trait pattern** — adding methods to an existing type (in this case, the `u32` alias `CTFontSymbolicTraits`) via a trait.

**C++ equivalent:** free functions, or a wrapper class that holds the bitmask. Less elegant than the trait approach.

### `CTFontDescriptor` wrapper

```rust
declare_TCFType! { CTFontDescriptor, CTFontDescriptorRef }
impl_TCFType!(CTFontDescriptor, CTFontDescriptorRef, CTFontDescriptorGetTypeID);
impl_CFTypeDescription!(CTFontDescriptor);
```

Methods: `new_with_name`, `new_with_attributes`, `copy_attributes`, `copy_attribute`, `matching_font_descriptors` (for searching).

### The `CTFontTraits` struct + traits

```rust
pub struct CTFontTraits(pub CFDictionary);

impl TraitAccessors for CTFontTraits {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits { /* ... */ }
    fn weight(&self) -> f32 { /* ... */ }
    fn width(&self) -> f32 { /* ... */ }
    fn slant(&self) -> f32 { /* ... */ }
}
```

A newtype wrapper around a `CFDictionary` (the underlying CFDictionary holds trait attributes). The `TraitAccessors` trait provides typed accessors.

---

## 5.4 `font_collection.rs` — `CTFontCollection`

A collection of `CTFontDescriptor`s. Methods: `new`, `matching_descriptors` (query the collection), `count`, `descriptors`.

---

## 5.5 `font_manager.rs` — font manager utilities

Free functions for font management: `create_font_descriptor` (parse a font file), `register_fonts_for_url` (load a font into the system), `unregister_fonts_for_url`, `enabled_font_descriptors`. No wrapper struct — just `pub fn`s wrapping `extern "C"` calls.

---

## 5.6 `string_attributes.rs` — string attribute constants

Constants used in `CFAttributedString` (Phase 3) when working with Core Text. E.g. `kCTFontAttributeName`, `kCTForegroundColorAttributeName`, `kCTUnderlineStyleAttributeName`. Each is a `CFStringRef` global from the CoreText framework, accessed via:

```rust
extern "C" {
    pub static kCTFontAttributeName: CFStringRef;
    pub static kCTForegroundColorAttributeName: CFStringRef;
    // etc.
}
```

These are `pub static` C globals. To use one as an attribute key in a `CFDictionary`, you `unsafe { kCTFontAttributeName.as_void_ptr() }` or wrap it in a `CFString`.

---

## 5.7 `run.rs` — `CTRun`

A run is a maximal sequence of consecutive characters that share the same font and style attributes within a laid-out line. `CTRun` wraps `CTRunRef`.

Methods: `attributes`, `glyph_count`, `glyphs`, `positions`, `advancements`, `string_indices`, `string_range`, `typographic_bounds`, `draw`.

Same wrapper pattern as `CTFont`.

---

## 5.8 `line.rs` — `CTLine`

A laid-out line of text. Wraps `CTLineRef`.

```rust
declare_TCFType! { CTLine, CTLineRef }
impl_TCFType!(CTLine, CTLineRef, CTLineGetTypeID);
impl_CFTypeDescription!(CTLine);
```

Methods:
- `new_with_attributed_string(string)` — lay out an attributed string as a line.
- `glyph_runs()` — get the `CFArray<CTRun>` of glyph runs in the line.
- `get_string_range()` — the range of the source string this line covers.
- `draw(context)` — draw the line into a `CGContext`.
- `get_image_bounds(context)` — the bounding rect of the line's drawn pixels.
- `get_typographic_bounds()` — a `TypographicBounds { width, ascent, descent, leading }` struct.

Note the `TypographicBounds` struct — a small `#[derive(Debug)]` plain struct returned by value. This is a common Rust pattern: when a C function returns multiple values via out-pointers, the Rust wrapper returns a single struct by value.

---

## 5.9 `frame.rs` — `CTFrame`

A laid-out frame (text laid out within a rectangular path). Wraps `CTFrameRef`.

Methods: `get_lines` (returns `CFArray<CTLine>`), `get_line_origins` (Vec of `CGPoint`), `get_visible_string_range`, `draw`. Used to render multi-line text.

---

## 5.10 `framesetter.rs` — `CTFramesetter`

Creates `CTFrame`s from attributed strings. Wraps `CTFramesetterRef`.

```rust
impl CTFramesetter {
    pub fn new_with_attributed_string(string: CFAttributedStringRef) -> Self { /* ... */ }
    pub fn create_frame(&self, string_range: CFRange, path: &CGPath) -> CTFrame { /* ... */ }
    pub fn suggest_frame_size_with_constraints(
        &self, string_range: CFRange, options: CFDictionaryRef,
        constraints: CGSize,
    ) -> (CGSize, CTFrameRef) { /* ... */ }
}
```

The lifecycle: create a framesetter from an attributed string, then create frames (each covering a range of the string) sized to fit a `CGPath` (usually a rectangle).

---

## 5.11 Summary

After Phase 5, you should be able to:

1. **Read any `core-text` file** and understand which CT type it wraps.
2. **Use the extension-trait pattern** to add idiomatic methods to type aliases (e.g. `SymbolicTraitAccessors` on `CTFontSymbolicTraits`).
3. **Recognise the multi-return-value pattern**: when a C function returns multiple values via out-pointers, the Rust wrapper returns a struct by value (e.g. `TypographicBounds`).
4. **Use `pub static` C globals** like `kCTFontAttributeName` as attribute keys in `CFDictionary`.
5. **Trace a text-layout pipeline end-to-end**: `CFAttributedString` → `CTFramesetter` → `CTFrame` → `CTLine` → `CTRun` → glyphs → `CGContext`.

The Core Text crate is the most domain-specific of the crates you've studied so far. The wrapper pattern is identical to Phase 3 and 4; the novelty is the typography vocabulary.

**Open File 6 when ready** to start the cocoa walkthrough.

— End of File 5 —
