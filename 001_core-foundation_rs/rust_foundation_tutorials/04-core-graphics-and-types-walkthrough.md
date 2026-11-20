# File 4 — `core-graphics/` + `core-graphics-types/` Walkthrough (20 files)

**File 4 of 8 — the 20 files in `core-graphics/src/` + `core-graphics-types/src/`.**

> **What these crates are:**
> - **`core-graphics-types`** (3 files, ~237 lines): Plain old data types — `CGFloat`, `CGPoint`, `CGSize`, `CGRect`, `CGAffineTransform`. No FFI, just `#[repr(C)]` structs.
> - **`core-graphics`** (17 files, ~4,065 lines): Safe Rust wrappers around Core Graphics, Apple's 2D drawing API. Wraps `CGContext`, `CGPath`, `CGColor`, `CGImage`, `CGFont`, `CGGradient`, etc.
>
> **Why study them:** Same wrapper pattern as File 3, applied to a new domain. The novelty is the `foreign_types` crate (used by `CGContext`), which is an alternative to `declare_TCFType!` for types where the opaque-pointer pattern is more involved.

---

## Table of Contents

- 4.1 `core-graphics-types/` — the POD types (3 files)
  - `lib.rs`, `base.rs`, `geometry.rs`
- 4.2 `core-graphics/src/base.rs` — type aliases + enums
- 4.3 `core-graphics/src/lib.rs` — crate root
- 4.4 `core-graphics/src/color.rs` — `CGColor`
- 4.5 `core-graphics/src/color_space.rs` — `CGColorSpace`
- 4.6 `core-graphics/src/context.rs` — `CGContext` (the big one, uses `foreign_types`)
- 4.7 `core-graphics/src/path.rs` — `CGPath`
- 4.8 `core-graphics/src/image.rs` — `CGImage`
- 4.9 `core-graphics/src/data_provider.rs` — `CGDataProvider`
- 4.10 `core-graphics/src/font.rs` — `CGFont`
- 4.11 `core-graphics/src/gradient.rs` — `CGGradient`
- 4.12 `core-graphics/src/display.rs` — display enumeration (macOS only)
- 4.13 `core-graphics/src/event.rs` + `event_source.rs` — events (macOS only)
- 4.14 `core-graphics/src/window.rs` — `CGWindow` (macOS only)
- 4.15 `core-graphics/src/sys.rs` — raw FFI declarations
- 4.16 `core-graphics/src/access.rs` — accessibility (macOS only)
- 4.17 Summary

---

## 4.1 `core-graphics-types/` — the POD types (3 files)

**Crate:** `core-graphics-types`
**Files:** `lib.rs` (~30 lines), `base.rs` (~20 lines), `geometry.rs` (~188 lines)

### `base.rs`

```rust
#![allow(non_camel_case_types)]

/// A 32-bit or 64-bit floating point type, depending on the platform.
pub type CGFloat = f64;
```

A single type alias. `CGFloat` is `f32` on 32-bit iOS (historically) and `f64` on 64-bit. For simplicity this crate always uses `f64` — which matches 64-bit Apple platforms.

### `geometry.rs`

Four `#[repr(C)]` structs: `CGSize`, `CGPoint`, `CGRect`, `CGAffineTransform`. Each is `#[derive(Clone, Copy, Debug, Default)]` — they're plain old data, fully `Copy` (bitwise-copied on assignment), and have sensible `Default` values (zeros).

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint { CGPoint { x, y } }
    pub fn apply_transform(&self, t: &CGAffineTransform) -> CGPoint {
        unsafe { ffi::CGPointApplyAffineTransform(*self, *t) }
    }
}
```

Methods on each struct call the corresponding C function (`CGPointApplyAffineTransform`, `CGRectInset`, `CGRectContainsPoint`, etc.) — wrapped in a small `unsafe` block. The `ffi` module is at the bottom of the file:

```rust
mod ffi {
    #[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
    extern "C" {
        pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
        pub fn CGRectIsEmpty(rect: CGRect) -> boolean_t;
        // etc.
    }
}
```

**C++ analogue:**

```cpp
struct CGPoint { CGFloat x, y; };
inline CGPoint CGPointApplyAffineTransform(CGPoint p, CGAffineTransform t);
// etc.
```

The Rust version is almost identical to the C version — same struct layout, same functions. The only Rust-specific bits are `#[repr(C)]`, the `Default` derive, and the `unsafe` blocks around FFI calls.

**Key constants:**

```rust
pub const CG_ZERO_POINT: CGPoint = CGPoint { x: 0.0, y: 0.0 };
pub const CG_ZERO_SIZE: CGSize = CGSize { width: 0.0, height: 0.0 };
pub const CG_ZERO_RECT: CGRect = CGRect { origin: CG_ZERO_POINT, size: CG_ZERO_SIZE };
pub const CG_AFFINE_TRANSFORM_IDENTITY: CGAffineTransform = CGAffineTransform {
    a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0,
};
```

`pub const` declares compile-time constants. `CG_AFFINE_TRANSFORM_IDENTITY` is the identity matrix `[1 0 0 1 0 0]` — same as C's `CGAffineTransformIdentity`.

---

## 4.2 `core-graphics/src/base.rs`

**Path:** `core-graphics/src/base.rs` (~70 lines)

Type aliases for CG-specific C types: `boolean_t = i32`, `cg_float = f32`, plus enums for `CGBlendMode`, `CGLineCap`, `CGLineJoin`, `CGPathDrawingMode`, `CGTextDrawingMode`, `CGInterpolationQuality`, `CGPathElementType` (with associated function-pointer type `CGPathApplierFunction`).

Same `#[repr(C)] #[derive(...)] pub enum` pattern as Phase 2.

---

## 4.3 `core-graphics/src/lib.rs`

**Path:** `core-graphics/src/lib.rs` (31 lines)

Just module declarations. Note the `#[cfg(target_os = "macos")]` attributes on `access`, `display`, `event`, `event_source`, `window` — these are macOS-only APIs.

---

## 4.4 `core-graphics/src/color.rs` — `CGColor`

**Path:** `core-graphics/src/color.rs` (58 lines)

A small wrapper. Uses the `declare_TCFType!` / `impl_TCFType!` macros from `core-foundation`:

```rust
declare_TCFType! { CGColor, CGColorRef }
impl_TCFType!(CGColor, CGColorRef, CGColorGetTypeID);

impl CGColor {
    pub fn rgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let ptr = CGColorCreateGenericRGB(red, green, blue, alpha);
            CGColor::wrap_under_create_rule(ptr)
        }
    }

    #[cfg(feature = "catalina")]
    pub fn srgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self { /* ... */ }
}
```

Same pattern as `CFString::new()` in File 3. The `#[cfg(feature = "catalina")]` attribute gates the `srgb` method on the `catalina` feature flag — it requires macOS 10.15+.

---

## 4.5 `core-graphics/src/color_space.rs` — `CGColorSpace`

Same pattern. Wraps `CGColorSpaceRef`. Methods: `new_with_name`, `new_srgb`, `new_display_p3`, `new_device_rgb`, `new_device_gray`.

---

## 4.6 `core-graphics/src/context.rs` — `CGContext`

**Path:** `core-graphics/src/context.rs` (786 lines — the biggest file in the crate)

`CGContext` is the heart of Core Graphics — it's the canvas you draw on. **This file uses the `foreign_types` crate instead of `declare_TCFType!`** for the wrapper, because `CGContextRef` is conventionally borrowed (you get one from `CGBitmapContextCreate` or from a Cocoa `NSGraphicsContext`) rather than owned.

### The `foreign_type!` macro

```rust
foreign_type! {
    type CType = crate::sys::CGContext;
    type Bridge = CGContextBridge;
    fn shared = CG_context_shared;    // these are made-up names for illustration
    fn own = CGContextRetain;

    pub struct CGContext;
    pub struct CGContextRef;
}
```

The `foreign_type!` macro (from the `foreign-types` crate) generates:
- A `CGContext` owned type (with `Drop` calling `CFRelease`).
- A `CGContextRef` borrowed type (no `Drop`).
- `Deref` impls from `CGContext` to `CGContextRef`.
- `Clone` for `CGContext` (via retain).

It's an alternative to `declare_TCFType!`/`impl_TCFType!` that handles the owned/borrowed distinction more cleanly. You'll see this pattern in modern Rust FFI code.

### Drawing methods

`CGContext` has dozens of drawing methods, each wrapping a C function:

```rust
impl CGContextRef {
    pub fn set_fill_color(&self, color: &CGColor) {
        unsafe { CGContextSetFillColorWithColor(self.as_ptr(), color.as_concrete_TypeRef()) }
    }

    pub fn fill_rect(&self, rect: CGRect) {
        unsafe { CGContextFillRect(self.as_ptr(), rect) }
    }

    pub fn add_rect(&self, rect: CGRect) {
        unsafe { CGContextAddRect(self.as_ptr(), rect) }
    }

    pub fn draw_image(&self, rect: CGRect, image: &CGImage) {
        unsafe { CGContextDrawImage(self.as_ptr(), rect, image.as_concrete_TypeRef()) }
    }

    // ... ~50 more methods
}
```

Note: methods are on `CGContextRef` (the borrowed type), not `CGContext` (the owned type). The auto-`Deref` makes both work — `my_owned_context.fill_rect(r)` derefs to `my_owned_context.fill_rect(r)` on the `CGContextRef`.

### Bitmap context creation

```rust
impl CGContext {
    pub fn create_bitmap_context(
        data: Option<*mut c_void>,
        width: usize, height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        color_space: &CGColorSpace,
        bitmap_info: u32,
    ) -> CGContext {
        unsafe {
            let ctx = CGBitmapContextCreate(
                data.unwrap_or(ptr::null_mut()),
                width, height,
                bits_per_component, bytes_per_row,
                color_space.as_concrete_TypeRef(),
                bitmap_info,
            );
            CGContext::from_type(ctx)
        }
    }
}
```

The standard "create wrapper around C allocator function" pattern. `from_type` is the `foreign_types` equivalent of `wrap_under_create_rule`.

---

## 4.7 `core-graphics/src/path.rs` — `CGPath`

Wraps `CGPathRef`. A vector path (lines, curves, arcs). Methods: `new`, `add_rect`, `add_ellipse`, `add_line`, `add_curve`, `add_arc`, `contains_point`, `apply` (iterate over path elements via a callback).

The `apply` method is interesting because it uses a C-style callback:

```rust
pub fn apply<F>(&self, mut f: F)
where F: FnMut(CGPathElement),
{
    extern "C" fn trampoline<F: FnMut(CGPathElement)>(
        info: *mut c_void,
        element: *const CGPathElement,
    ) {
        let f = unsafe { &mut *(info as *mut F) };
        f(unsafe { *element });
    }

    unsafe {
        CGPathApply(
            self.as_concrete_TypeRef(),
            &mut f as *mut F as *mut c_void,
            trampoline::<F>,
        );
    }
}
```

This is the **Rust closure → C function pointer trampoline pattern**. Core Graphics wants a C function pointer + a `void* context`. We:

1. Define a `trampoline` extern "C" function that knows how to recover the closure from the `void*` and call it.
2. Pass a pointer to the closure (reinterpreted as `*mut c_void`) as the context.
3. Pass the trampoline as the function pointer.

This is the standard way to bridge Rust closures to C callback APIs. metal-cpp's `std::function`-to-block adapter (from the previous guide) solves the same problem in a different idiom.

---

## 4.8 `core-graphics/src/image.rs` — `CGImage`

Wraps `CGImageRef` — a bitmap image. Methods: `new`, `from_data` (decode from bytes), `width`, `height`, `bits_per_component`, `bits_per_pixel`, `bytes_per_row`, `color_space`, `data_provider`.

---

## 4.9 `core-graphics/src/data_provider.rs` — `CGDataProvider`

Wraps `CGDataProviderRef` — a source of bytes for images and other data-driven CG types. Methods: `from_buffer` (wrap a Rust `&[u8]`), `from_filename`, `data` (read all bytes into a `Vec<u8>`).

---

## 4.10 `core-graphics/src/font.rs` — `CGFont`

Wraps `CGFontRef`. Methods: `from_name`, `post_script_name`, `copy_table_tags`, `copy_table_for_tag`. Used for low-level font access — Core Text (Phase 5) provides higher-level APIs.

---

## 4.11 `core-graphics/src/gradient.rs` — `CGGradient`

Wraps `CGGradientRef`. A gradient (for drawing smooth color transitions). Methods: `new_with_color_space` (takes a slice of color stops). Used with `CGContext::draw_linear_gradient` / `draw_radial_gradient`.

---

## 4.12 `core-graphics/src/display.rs` — display enumeration (macOS only)

`#[cfg(target_os = "macos")]`. Functions for enumerating displays: `CGGetActiveDisplayList`, `CGDisplayBounds`, `CGDisplayPixelsWide`, etc. Mostly free functions (not wrapper structs).

---

## 4.13 `core-graphics/src/event.rs` + `event_source.rs` — events (macOS only)

`#[cfg(target_os = "macos")]`. `CGEvent` wraps `CGEventRef` — a low-level input event. `CGEventSource` wraps `CGEventSourceRef` — the source state (keyboard modifier state, etc.) for synthesised events.

Used for low-level event programming (synthesising keyboard input, intercepting global events).

---

## 4.14 `core-graphics/src/window.rs` — `CGWindow` (macOS only)

`#[cfg(target_os = "macos")]`. Functions for enumerating on-screen windows (`CGWindowListCopyWindowInfo`, `CGWindowListCreateImage`) — used for screen-capture apps.

---

## 4.15 `core-graphics/src/sys.rs` — raw FFI declarations

The `extern "C"` block declaring all the CG C functions. Similar in spirit to `core-foundation-sys` — pure FFI declarations, no safe wrappers. The safe wrappers in the other modules call into these.

---

## 4.16 `core-graphics/src/access.rs` — accessibility (macOS only)

`#[cfg(target_os = "macos")]`. Constants for the accessibility API (`kCGPostNotificationCtrl`, etc.). Small.

---

## 4.17 Summary

After Phase 4, you should be able to:

1. **Recognise both wrapper patterns**: the `declare_TCFType!`/`impl_TCFType!` pattern from File 3 (used by `CGColor`, `CGColorSpace`, `CGPath`, `CGImage`, etc.) and the `foreign_type!` pattern (used by `CGContext`).
2. **Explain the Rust closure trampoline**: the standard pattern for passing a Rust closure to a C function that expects a function pointer + `void*` context.
3. **Read any `core-graphics` file** and understand which CG type it wraps and what methods it exposes.
4. **Use `#[cfg(target_os = "macos")]`** to gate macOS-only APIs.
5. **Use `#[cfg(feature = "...")]`** to gate feature-flagged APIs (e.g. `catalina` for macOS 10.15+).

The patterns from File 3 (safe wrapper around unsafe FFI, traits for idiomatic API, `wrap_under_create_rule`/`wrap_under_get_rule`) all carry over to `core-graphics`. The novelty here is the `foreign_types` crate, which gives a slightly cleaner owned/borrowed distinction for types where both forms are useful.

**Open File 5 when ready** to start the `core-text` walkthrough.

— End of File 4 —
