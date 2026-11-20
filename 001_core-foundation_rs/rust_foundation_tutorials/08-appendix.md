# File 8 — Appendix: Patterns, Glossary, Where to Go Next

**File 8 of 8 — the wrap-up.**

> **What this file contains:** A reference of the recurring patterns you've seen across all 106 source files, a glossary of every term used in the seven preceding files, and concrete pointers for what to study after Day 200.

---

## Table of Contents

- Appendix A — The recurring code patterns in core-foundation-rs
- Appendix B — Glossary
- Appendix C — Where to go next (after Day 200)
- Appendix D — Rust-vs-C++ cheat sheet (one-page summary)
- Appendix E — Common pitfalls for C++ developers learning Rust

---

## Appendix A — The recurring code patterns in core-foundation-rs

After 200 days of reading this codebase, you'll have seen the same patterns dozens of times. Here's the complete checklist, with file references.

### Pattern 1: The opaque struct + ref alias

**Where:** Every file in `core-foundation-sys/src/` and some in `core-graphics/`.

```rust
#[repr(C)]
pub struct __CFString(c_void);
pub type CFStringRef = *const __CFString;
```

**Why:** Gives the ref type a distinct Rust type so the compiler can catch pointer-type mixups.

**C++ analogue:**
```cpp
struct __CFString;   // forward declaration; internals are private
using CFStringRef = const __CFString*;
```

### Pattern 2: `extern "C" { ... }` block

**Where:** Every file in `core-foundation-sys/src/`.

```rust
extern "C" {
    pub fn CFRelease(cf: CFTypeRef);
    pub static kCFAllocatorDefault: CFAllocatorRef;
}
```

**Why:** Declares C functions and globals for the linker to resolve. Calling any of them requires `unsafe`.

**C++ analogue:** Just declaring the functions in a header — no `unsafe` requirement.

### Pattern 3: `#[repr(C)]` for ABI compatibility

**Where:** All structs that cross the FFI boundary (`CFRange`, `CFAllocatorContext`, `CFArrayCallBacks`, `CGPoint`, etc.).

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CFRange { pub location: CFIndex, pub length: CFIndex }
```

**Why:** Forces Rust to lay out the struct exactly as C would. Without it, Rust could reorder/pad fields for alignment.

**C++ analogue:** C++ already matches C layout for aggregates; no annotation needed.

### Pattern 4: `#[derive(...)]` for boilerplate traits

**Where:** Every plain-data struct.

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGPoint { /* ... */ }
```

**Why:** Auto-generates implementations of the standard traits. `Copy` means bitwise copy on assignment; `Default` gives a zero-valued default.

**C++ analogue:** No direct equivalent; you'd write copy constructors, comparison operators, etc. by hand (or generate them with codegen tools).

### Pattern 5: The `declare_TCFType!` + `impl_TCFType!` macro pair

**Where:** Every file in `core-foundation/src/` (and some in `core-graphics/`, `core-text/`).

```rust
declare_TCFType!(CFString, CFStringRef);
impl_TCFType!(CFString, CFStringRef, CFStringGetTypeID);
impl_CFTypeDescription!(CFString);
```

**Why:** Generates ~200 lines of boilerplate (`Drop`, `Clone`, `PartialEq`, `Eq`, `Debug`, `TCFType` impl, `ToVoid` impls) for each wrapper type.

**C++ analogue:** CRTP base class + inline function definitions. metal-cpp uses the same idea via templates.

### Pattern 6: `wrap_under_create_rule` vs `wrap_under_get_rule`

**Where:** Every `TCFType` impl.

```rust
unsafe fn wrap_under_create_rule(reference: CFStringRef) -> CFString {
    assert!(!reference.is_null());
    CFString(reference)
}

unsafe fn wrap_under_get_rule(reference: CFStringRef) -> CFString {
    assert!(!reference.is_null());
    let reference = CFRetain(reference);   // bump refcount
    TCFType::wrap_under_create_rule(reference)
}
```

**Why:** Reflects Core Foundation's "Create Rule" (you own the reference, refcount is 1) vs "Get Rule" (you don't own it, must retain before storing).

**C++ analogue:** metal-cpp's `TransferPtr` vs `RetainPtr`.

### Pattern 7: Safe method wrapping unsafe FFI

**Where:** Every wrapper crate.

```rust
impl CFString {
    pub fn new(s: &str) -> CFString {
        unsafe {
            let ref_ = CFStringCreateWithBytes(/* ... */);
            CFString::wrap_under_create_rule(ref_)
        }
    }
}
```

**Why:** The whole point of the library. Safe signature outside, `unsafe` block inside.

**C++ analogue:** metal-cpp's `NS::String::string(...)` — same idea, different language.

### Pattern 8: `mem::forget` to skip Drop

**Where:** `into_CFType`, `downcast_into`, `into_untyped`.

```rust
fn into_CFType(self) -> CFType {
    let reference = self.as_CFTypeRef();
    mem::forget(self);    // don't run self's Drop
    unsafe { TCFType::wrap_under_create_rule(reference) }
}
```

**Why:** Transfers ownership without refcount changes. If `self`'s Drop ran, it would call `CFRelease` and decrement the count.

**C++ analogue:** `std::unique_ptr<T>` release — `auto ptr = std::move(up).release();` skips the destructor.

### Pattern 9: The trampoline pattern for C callbacks

**Where:** `CGPath::apply`, `CFArrayApplyFunction`, etc.

```rust
pub fn apply<F: FnMut(CGPathElement)>(&self, mut f: F) {
    extern "C" fn trampoline<F: FnMut(CGPathElement)>(
        info: *mut c_void, element: *const CGPathElement,
    ) {
        let f = unsafe { &mut *(info as *mut F) };
        f(unsafe { *element });
    }
    unsafe {
        CGPathApply(self.as_concrete_TypeRef(),
                    &mut f as *mut F as *mut c_void,
                    trampoline::<F>);
    }
}
```

**Why:** Bridges a Rust closure to a C function-pointer-plus-void*-context API.

**C++ analogue:** The same trampoline trick — typically a static member function that `static_cast`s the void* to a `std::function*` and invokes it.

### Pattern 10: Extension traits

**Where:** `core-text/font_descriptor.rs` (`SymbolicTraitAccessors`), `core-foundation/base.rs` (`CFIndexConvertible`).

```rust
pub trait SymbolicTraitAccessors {
    fn is_italic(&self) -> bool;
    fn is_bold(&self) -> bool;
    // ...
}

impl SymbolicTraitAccessors for CTFontSymbolicTraits {  // CTFontSymbolicTraits is just u32
    fn is_italic(&self) -> bool { (*self & kCTFontItalicTrait) != 0 }
    // ...
}
```

**Why:** Adds methods to an existing type (often a type alias for `u32` or `usize`) without modifying it.

**C++ analogue:** Free functions taking the value by value or const-ref. Less elegant than the trait approach.

### Pattern 11: The `ItemRef<'a, T>` lifetimed borrow

**Where:** `core-foundation/base.rs`.

```rust
pub struct ItemRef<'a, T: 'a>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<T> Deref for ItemRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
```

**Why:** Represents a borrow of an item inside a container. The lifetime `'a` ties the borrow to the container; `ManuallyDrop` prevents the borrow from being released when the `ItemRef` drops.

**C++ analogue:** A `T&` returned from a container method — except C++ doesn't enforce the lifetime at compile time.

### Pattern 12: `Cow<'a, B>` for borrowed-or-owned

**Where:** `core-foundation/string.rs` (`impl From<&CFString> for Cow<str>`).

```rust
impl<'a> From<&'a CFString> for Cow<'a, str> {
    fn from(cf_str: &'a CFString) -> Cow<'a, str> {
        // Try borrowed path first; fall back to owned.
    }
}
```

**Why:** Avoids allocation when possible.

**C++ analogue:** `std::variant<std::string_view, std::string>` — or just always copy. Rust's `Cow` is more ergonomic.

### Pattern 13: The `foreign_type!` macro (alternative to `declare_TCFType!`)

**Where:** `core-graphics/context.rs` (`CGContext`), `core-graphics/path.rs` (`CGPath`).

```rust
foreign_type! {
    type CType = sys::CGContext;
    type Bridge = CGContextBridge;
    pub struct CGContext;
    pub struct CGContextRef;
}
```

**Why:** Generates both an owned type (`CGContext` with `Drop`) and a borrowed type (`CGContextRef` without `Drop`), with `Deref` from owned to borrowed. Cleaner separation than `declare_TCFType!` for types where both forms are commonly used.

**C++ analogue:** A `unique_ptr<T>` (owned) vs `T&` (borrowed) distinction, but enforced by type rather than by convention.

### Pattern 14: `#[cfg(target_os = "...")]` for platform gating

**Where:** `core-foundation-sys/lib.rs` (XML modules macOS-only), `core-graphics/lib.rs` (display/event/window macOS-only), etc.

```rust
#[cfg(target_os = "macos")]
pub mod user_notification;
```

**Why:** Some APIs exist only on macOS, not iOS. Conditional compilation removes them from the build on other platforms.

**C++ analogue:** `#ifdef TARGET_OS_MACOS` / `#if defined(__APPLE__)` preprocessor guards.

### Pattern 15: `#[cfg(feature = "...")]` for feature gating

**Where:** `core-foundation/Cargo.toml` (`mac_os_10_8_features`, `with-uuid`), `core-graphics/color.rs` (`#[cfg(feature = "catalina")]`).

```rust
#[cfg(feature = "catalina")]
pub fn srgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> CGColor { /* ... */ }
```

**Why:** Some APIs require a minimum OS version; users opt in via Cargo features.

**C++ analogue:** CMake options + preprocessor guards.

### Pattern 16: Trait impls for idiomatic Rust API

**Where:** Every wrapper crate — `impl From<&str> for CFString`, `impl FromStr`, `impl Display`, `impl PartialEq<&str>`, etc.

```rust
impl<'a> From<&'a str> for CFString { /* ... */ }
impl FromStr for CFString { type Err = (); /* ... */ }
impl fmt::Display for CFString { /* ... */ }
impl PartialEq<&str> for CFString { /* ... */ }
```

**Why:** Makes the wrapper feel native to Rust — `CFString::from("hi")`, `"hi".parse::<CFString>()`, `format!("{}", cfstr)`, `cfstr == "hi"` all work.

**C++ analogue:** Implicit conversion constructors, `operator==`, `operator<<`, etc. — but C++'s implicit conversions can be surprising; Rust's `From`/`Into` are explicit.

---

## Appendix B — Glossary

| Term | Meaning |
|---|---|
| **ABI** | Application Binary Interface — the binary conventions for function calls, struct layout, etc. |
| **Allocation (Create rule)** | A CF function that returns a +1 reference (caller owns it, must release). Names start with `CFXxxCreate`. |
| **`#[allow(...)]`** | Attribute that suppresses a lint. Used at crate root to allow non-snake-case names etc. |
| **`Arc<T>`** | Atomic Reference Counted — thread-safe shared ownership. C++'s `std::shared_ptr`. |
| **Associated type** | A type declared inside a trait, like `type Ref;` in `TCFType`. Each implementor picks a concrete type. |
| **`as`** | Casting operator: `x as u32`. |
| **Borrow** | A reference (`&T` or `&mut T`) that doesn't transfer ownership. |
| **Borrow checker** | The compile-time analysis that enforces Rust's borrowing rules. |
| **`Box<T>`** | Heap-allocated single-owner pointer. C++'s `std::unique_ptr<T>`. |
| **Cocoa** | Apple's Objective-C UI framework for macOS. Wrapped by the (deprecated) `cocoa` crate. |
| **`Cow<'a, B>`** | "Clone on write" — either a borrow (`Cow::Borrowed`) or owned (`Cow::Owned`). |
| **Core Foundation (CF)** | Apple's C-level foundation framework. The thing being wrapped. |
| **`core-foundation` crate** | Safe Rust wrappers around Core Foundation. |
| **`core-foundation-sys` crate** | Raw FFI bindings to Core Foundation. No safe wrappers. |
| **Core Graphics (CG)** | Apple's C 2D drawing framework. |
| **Core Text (CT)** | Apple's C text layout framework. |
| **`crate`** | Rust's compilation unit. Either a library or a binary. |
| **`#[derive(...)]`** | Auto-implements traits for a type. |
| **`dyn Trait`** | A trait object — fat pointer `(data, vtable)`. Dynamic dispatch. |
| **`enum`** | Rust's tagged-union enum. Variants can carry data. |
| **`extern "C"`** | C-ABI function declaration or function pointer type. |
| **FFI** | Foreign Function Interface — calling C (or other languages) from Rust. |
| **`fn`** | A function pointer type. `extern "C" fn(...)` is a C function pointer. |
| **`foreign_types` crate** | A popular alternative to `declare_TCFType!` for some wrappers. |
| **`From<X> for Y`** | Trait for infallible conversion. Implies `Into<Y> for X`. |
| **`FromStr`** | Trait for parsing from `&str`. Returns `Result<Self, Err>`. |
| **`get` rule** | A CF function that returns a non-owned reference (caller must retain to keep). |
| **Generic** | A type or function parameterised over other types: `fn foo<T>(x: T)`. |
| **`impl`** | Implementation block for a type's methods. |
| **`impl Trait for Type`** | Implementing a trait for a type. |
| **`#[inline]`** | Hint to inline a function. |
| **` IOSurface `** | Cross-process shared-memory image surface. |
| **`let`** | Variable binding. `let x = 5;` (immutable) or `let mut x = 5;` (mutable). |
| **Lifetime (`'a`)** | A compile-time tag recording how long a reference is valid. |
| **`macro_rules!`** | Declarative macro definition. |
| **`ManuallyDrop<T>`** | Wraps T but doesn't auto-drop. |
| **`mem::forget`** | Consumes a value without running its Drop. |
| **`match`** | Pattern-matching expression. Exhaustive for enums. |
| **`msg_send!`** | Macro for sending an Objective-C message. From the `objc` crate. |
| **`nil`** | Objective-C's null. In cocoa-rs, a constant representing the null `id`. |
| **`objc` crate** | Rust bindings to the Objective-C runtime. Used by `cocoa`. |
| **`objc2` crate** | The newer, safer replacement for `objc`. Not in this zip. |
| **Opaque struct** | A struct whose internals aren't exposed. `pub struct __CFString(c_void);`. |
| **`Option<T>`** | Rust's replacement for null. `Some(x)` or `None`. |
| **`panic!`** | Abort the current thread with a message. Roughly `std::terminate`. |
| **`PhantomData<T>`** | Zero-sized marker for "this type conceptually owns a T". |
| **`pub`** | Makes an item visible outside its module. |
| **`#[repr(C)]`** | Forces C-compatible struct/enum layout. |
| **`#[repr(isize)]`** | Forces enum to be `isize`-sized. |
| **`Result<T, E>`** | Rust's replacement for exceptions. `Ok(t)` or `Err(e)`. |
| **`?` operator** | Propagates an error from a `Result`. |
| **Retain count** | The reference count of a CF object. `CFRetain` increments, `CFRelease` decrements. |
| **`Rc<T>`** | Single-threaded shared ownership. No C++ stdlib equivalent. |
| **`RefCell<T>`** | Single-threaded runtime-checked mutable borrow. |
| **`Send`** | Marker trait: a type can be moved across threads. |
| **`SEL`** | Objective-C selector — an interned method-name string. |
| **`static`** | Program-lifetime variable. `static` (immutable) or `static mut` (mutable, requires `unsafe`). |
| **`struct`** | A record type. Either named-field, tuple, or unit. |
| **`Sync`** | Marker trait: a type can be shared (via `&T`) across threads. |
| **`TCFType` trait** | The trait every CF wrapper implements. The Rust analogue of metal-cpp's `Referencing<T>`. |
| **`TCFTypeRef` trait** | Conversion trait between typed raw pointers and `*const c_void`. |
| **`to_CFIndex`** | Bounds-checked conversion from `usize` to `CFIndex` (isize). |
| **Trait** | A set of method signatures a type can implement. Like a C++ concept + Java interface combined. |
| **Trait bound** | A constraint on a generic: `fn foo<T: Trait>(...)`. |
| **`unsafe`** | Keyword marking code that the compiler can't verify. Five uses: deref raw pointer, call extern fn, access static mut, impl unsafe trait, call unsafe fn. |
| **`use`** | Import statement. `use std::collections::HashMap;`. |
| **Workspace** | A Cargo feature for managing multiple related crates together. |
| **`#[macro_export]`** | Makes a macro visible outside its crate. |

---

## Appendix C — Where to go next (after Day 200)

By Day 200 you'll have read every `.rs` file in this codebase. Natural next steps:

### 1. Migrate to `objc2`

Both `cocoa` and `io-surface` are deprecated in favour of `objc2` and `objc2-io-surface`. The newer crates provide:

- Better type safety (each Objective-C class gets its own Rust type, not the raw `id`).
- Less `unsafe` in user code.
- Active maintenance.

Web search: "objc2 crate Rust" — start with <https://github.com/madsmtm/objc2>.

### 2. Read Apple's official Core Foundation docs

The conceptual docs explain the design philosophy behind CF (the "Create Rule", "Get Rule", reference counting, type IDs):

- <https://developer.apple.com/documentation/corefoundation>
- "Core Foundation Design Concepts" — search Apple's developer site.

### 3. Read the Rust FFI guide

The Rust Programming Language book has a chapter on FFI:

- <https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html>
- <https://doc.rust-lang.org/nomicon/> — the "Rustonomicon" — covers unsafe Rust in depth.

### 4. Compare with `metal-cpp-rs` (or other Rust Apple-framework wrappers)

The metal-cpp guide you read earlier covered the C++ wrapper. There's a Rust equivalent: search for "metal-rs" or "objc2-metal". Compare the approaches:

- C++: templates + inline functions + CRTP.
- Rust: traits + macros + `unsafe` containment.

### 5. Write a real program

Pick something small and achievable:

- **Days 201–210:** Render "Hello, World!" to a window using `cocoa` + `core-text` + `core-graphics`. (Use the deprecated crates first since they're in your zip; then migrate to `objc2`.)
- **Days 211–230:** Build a small plist editor — read a `.plist` file, modify a value, write it back.
- **Days 231–260:** Build a font inspector — given a `.ttf` file, display its name, metrics, glyph count, and a few sample glyphs.

### 6. Read other Rust FFI libraries

To see the same patterns applied elsewhere:

- **`libsqlite3-sys`** — FFI for SQLite. Same sys + safe-wrapper split.
- **`nix`** — Safe wrappers around Unix syscalls.
- **`winapi` / `windows`** — Windows API bindings.
- **`glib` / `gtk`** — GNOME Rust bindings.

### 7. Read the Rust standard library source

Rust's stdlib uses many of the same patterns you've seen here (traits, generics, lifetimes, macros). Pick a module and read it:

- `std::collections::HashMap` — generic, lifetimes, iterators.
- `std::sync::Arc` — interior mutability, atomic refcounting.
- `std::ffi::CString` / `CStr` — FFI string types.

### 8. Contribute

`core-foundation-rs` is open source at <https://github.com/servo/core-foundation-rs>. PRs welcome:

- Documentation improvements.
- New bindings for missing CF functions.
- Migration help for the deprecated `cocoa` crate to `objc2`.

---

## Appendix D — Rust-vs-C++ cheat sheet (one-page summary)

| Concept | Rust | C++ | Notes |
|---|---|---|---|
| Immutable variable | `let x = 5;` | `const int x = 5;` | Rust: default immutable |
| Mutable variable | `let mut x = 5;` | `int x = 5;` | Rust: explicit `mut` |
| Function | `fn add(a: i32, b: i32) -> i32 { a + b }` | `int add(int a, int b) { return a + b; }` | Rust: last expr is return |
| Struct | `struct Point { x: f64, y: f64 }` | `struct Point { double x, y; };` | Same |
| Method | `impl Point { fn len(&self) -> f64 { ... } }` | `double len() const { ... }` (inside struct) | Rust: separate `impl` block |
| Trait / interface | `trait Foo { fn bar(&self); }` | `class Foo { public: virtual void bar() const = 0; };` | Rust: no inheritance |
| Generic | `fn foo<T: Trait>(x: T)` | `template <typename T> void foo(T x)` | Rust: trait bounds checked early |
| Smart pointer (owned) | `Box<T>` | `std::unique_ptr<T>` | Same |
| Smart pointer (shared) | `Arc<T>` | `std::shared_ptr<T>` | Same |
| Reference | `&T` / `&mut T` | `const T&` / `T&` | Rust: borrow-checked |
| Null | `Option<T>` | `T*` (may be null) | Rust: no null |
| Error handling | `Result<T, E>` + `?` | exceptions (`try`/`catch`) | Rust: errors are values |
| Destructor | `impl Drop for T` | `~T()` | Rust: auto-called at end of scope |
| Move semantics | `let y = x;` (moves) | `auto y = std::move(x);` | Rust: moves are the default |
| Copy semantics | `let y = x;` (if `T: Copy`) | `auto y = x;` (always copies) | Rust: only for `Copy` types |
| Pattern matching | `match s { A => ..., B(x) => ... }` | `switch (s.tag) { case A: ...; case B: ... }` | Rust: exhaustive, expressions |
| Closure | `\|x\| x * 2` | `[](int x) { return x * 2; }` | Similar |
| Module | `pub mod foo;` (file: `foo.rs`) | `namespace foo { }` | Rust: file-based |
| Crates | `Cargo.toml` | CMake target | Rust: Cargo manages |
| Macro | `macro_rules! foo { ... }` | `#define foo(...)` (preprocessor) | Rust: hygienic, pattern-based |
| `unsafe` | `unsafe { ... }` | (no equivalent — all code is "unsafe") | Rust: explicit escape hatch |
| FFI | `extern "C" { fn foo(); }` | `extern "C" void foo();` | Similar |
| Conditional compilation | `#[cfg(target_os = "linux")]` | `#ifdef __linux__` | Similar |

---

## Appendix E — Common pitfalls for C++ developers learning Rust

If you're coming from C++, these are the things that will trip you up:

### Pitfall 1: "Why can't I just write `x = 5;` without `let`?"

In C++, you can declare a variable anywhere with `int x = 5;`. In Rust, every variable binding requires `let`. `x = 5;` (without `let`) is an assignment to an existing variable, not a declaration.

### Pitfall 2: "Why can't I return a reference to a local?"

```rust
fn bad() -> &String {
    let s = String::from("hello");
    &s   // ERROR: s doesn't live long enough
}
```

C++ lets you do this and crash at runtime. Rust catches it at compile time via lifetimes.

### Pitfall 3: "Why do I have to use `mut`?"

C++ variables are mutable by default; you add `const` to make them immutable. Rust flips this: variables are immutable by default; you add `mut`. The safe default wins.

### Pitfall 4: "Why can't I have circular references?"

```rust
struct Node { next: Option<Box<Node>> }   // singly linked: OK
struct Bad { a: Box<Bad>, b: Box<Bad> }   // infinite size: ERROR
```

Rust's ownership model doesn't naturally handle cycles. You need `Rc<RefCell<T>>` (or `Arc<Mutex<T>>` for thread-safe) for graph-like structures.

### Pitfall 5: "Why does my `Vec<&T>` not work after the Vec is moved?"

```rust
let v = vec![1, 2, 3];
let r = &v[0];
drop(v);   // ERROR: v is borrowed by r
println!("{}", r);
```

The borrow checker prevents use-after-free. In C++ this would be a dangling reference and crash at runtime.

### Pitfall 6: "Why are there so many string types?"

Rust has `&str`, `String`, `CString`, `CStr`, `OsString`, `OsStr`, `PathBuf`, `Path`. Each has a specific purpose:

- `&str` / `String` — UTF-8 Rust strings.
- `CString` / `CStr` — null-terminated, for FFI.
- `OsString` / `OsStr` — platform-native strings (may not be UTF-8 on Windows).
- `PathBuf` / `Path` — filesystem paths.

C++ has `std::string` (which is bytes, not UTF-8) and `std::filesystem::path`. Rust's distinction is more granular.

### Pitfall 7: "Why do I have to write `impl Trait for Type` and not just `class Type : Trait`?"

Rust has no inheritance. You don't extend a base class; you implement a trait. This means:

- No virtual dispatch by default (use `dyn Trait` for that).
- No upcasting (a `String` is not a `Display`; it just implements `Display`).
- No "is-a" relationships — composition over inheritance.

### Pitfall 8: "Why can't I overload functions by argument type?"

Rust doesn't have function overloading. You achieve similar effects via:

- Multiple functions with different names (`from_i32`, `from_f64`).
- Traits (`From<i32>`, `From<f64>` — both can be implemented).
- Default arguments don't exist either.

### Pitfall 9: "Why is compilation so slow?"

Rust compiles slower than C++ for several reasons:

- Monomorphisation generates fresh code for each generic instantiation.
- The borrow checker does significant analysis.
- Macros expand at compile time.
- Cargo downloads and compiles all dependencies.

Trade-off: slower compile, but more optimisation and safety.

### Pitfall 10: "Why is the error message so long?"

Rust's compile errors, especially borrow-checker errors, can be verbose. They're trying to teach you what's wrong. Read them carefully — they often include suggestions for how to fix the problem.

---

## Final word

You've spent 200 days reading the `core-foundation-rs` codebase. Along the way you learned:

- **Rust fundamentals** — ownership, borrowing, lifetimes, traits, generics, enums, pattern matching, `Option`/`Result`, `unsafe`.
- **FFI patterns** — `extern "C"`, `#[repr(C)]`, opaque structs, function pointer types, callback trampolines.
- **The safe-wrapper architecture** — `TCFType` trait, `declare_TCFType!`/`impl_TCFType!` macros, Create vs Get rule, `mem::forget` for ownership transfer.
- **Domain knowledge** — Core Foundation, Core Graphics, Core Text, Cocoa, IOSurface.
- **100 concrete code examples** — from `let x = 5;` to building a typed CFDictionary.

The patterns you've internalised will transfer to every other Rust FFI library you encounter. And the C++ comparisons throughout the guide mean you can now translate fluently between the two languages — a rare and valuable skill.

The next step is to **write code**. Pick a small project (the suggestions in Appendix C), start typing, and let the borrow checker teach you the rest. Reading is necessary; writing is what makes it stick.

Good luck.

— End of File 8 (and the whole 8-file guide) —
