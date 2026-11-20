# File 3 — `core-foundation/` Walkthrough (22 safe-wrapper files)

**File 3 of 8 — the 22 files in `core-foundation/src/`.**

> **What this crate is:** Safe Rust wrappers around the unsafe FFI bindings in `core-foundation-sys/`. Every C function call is wrapped in a small `unsafe { ... }` block that upholds Rust's ownership and type-safety invariants. Users of this crate get a clean, idiomatic Rust API with no `unsafe` in their own code.
>
> **Why this is the most important phase of the whole 200 days:** This crate is *the textbook example* of how to wrap a C API in safe Rust. Every Rust idiom worth knowing shows up here — traits, generics, lifetimes, `Drop`, `unsafe` containment, declarative macros, conversion traits (`From`/`Into`/`AsRef`), iterator implementations, and more.
>
> **Lines:** ~3,837 across 22 files.

---

## Table of Contents

- The wrapper pattern (read this once — every file follows it)
- 3.1 `lib.rs` — crate root, `ConcreteCFType`, and the four macros
- 3.2 `base.rs` — the `TCFType` trait (the conceptual heart)
- 3.3 `string.rs` — `CFString` (idiomatic Rust API on top of FFI)
- 3.4 `array.rs` — `CFArray<T>` (generics + iterators + lifetimes)
- 3.5 `dictionary.rs` — `CFDictionary<K, V>` + `CFMutableDictionary`
- 3.6 `set.rs` — `CFSet`
- 3.7 `data.rs` — `CFData` (raw bytes)
- 3.8 `number.rs` — `CFNumber` (with `From` impls for every numeric type)
- 3.9 `boolean.rs` — `CFBoolean`
- 3.10 `date.rs` — `CFDate`
- 3.11 `error.rs` — `CFError` (the Rust error pattern)
- 3.12 `url.rs` — `CFURL`
- 3.13 `bundle.rs` — `CFBundle`
- 3.14 `runloop.rs` — `CFRunLoop`
- 3.15 `mach_port.rs` — `CFMachPort`
- 3.16 `filedescriptor.rs` — `CFFileDescriptor`
- 3.17 `characterset.rs` — `CFCharacterSet`
- 3.18 `propertylist.rs` — `CFPropertyList` (enum-based wrapper)
- 3.19 `timezone.rs` — `CFTimeZone`
- 3.20 `uuid.rs` — `CFUUID` (feature-gated)
- 3.21 `attributed_string.rs` — `CFAttributedString`
- 3.22 Summary

---

## The wrapper pattern (read this once — every file follows it)

Every wrapper file in this crate follows the same six-section template. Learn it once and you can read any file.

### Section 1: License header + module doc + imports

```rust
// Copyright ... (9 lines)

//! Immutable strings.

pub use core_foundation_sys::string::*;

use crate::base::{TCFType, CFIndexConvertible};
use core_foundation_sys::base::{kCFAllocatorDefault, CFIndex, CFRange};
use std::borrow::Cow;
use std::ffi::CStr;
// etc.
```

- `pub use core_foundation_sys::string::*;` re-exports all the FFI declarations (`CFStringRef`, `CFStringCreateWithCString`, `kCFStringEncodingUTF8`, etc.) from the underlying sys crate. Users of `core-foundation` can `use core_foundation::string::*` and get both the safe wrapper AND the raw FFI in one import.
- Other `use` statements bring in traits, types, and standard library items needed by the wrapper.

### Section 2: Declare the wrapper struct + Drop impl via macro

```rust
declare_TCFType! {
    /// An immutable string in one of a variety of encodings.
    CFString, CFStringRef
}
impl_TCFType!(CFString, CFStringRef, CFStringGetTypeID);
impl_CFTypeDescription!(CFString);
```

Three macros invoked in sequence. They generate (respectively):

1. **`declare_TCFType!(CFString, CFStringRef)`** — declares `pub struct CFString(CFStringRef);` and implements `Drop` to call `CFRelease`.
2. **`impl_TCFType!(CFString, CFStringRef, CFStringGetTypeID)`** — implements the `TCFType` trait for `CFString` (associated type `Ref = CFStringRef`, all the trait methods), plus `Clone`, `PartialEq`, `Eq`, and `ToVoid` impls.
3. **`impl_CFTypeDescription!(CFString)`** — implements `std::fmt::Debug` by calling `CFCopyDescription`.

These macros are the Rust analogue of metal-cpp's `Referencing<T>` template + the C++ wrapper pattern. They're the "boilerplate generator" that lets each wrapper file be ~100 lines instead of ~300. We'll dissect them in §3.1 below.

### Section 3: Implement standard Rust traits for idiomatic use

```rust
impl FromStr for CFString {
    type Err = ();
    fn from_str(string: &str) -> Result<CFString, ()> {
        Ok(CFString::new(string))
    }
}

impl<'a> From<&'a str> for CFString {
    fn from(string: &'a str) -> CFString {
        CFString::new(string)
    }
}

impl fmt::Display for CFString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}
```

These impls let you write idiomatic Rust:

- `CFString::from("hello")` — uses the `From<&str>` impl.
- `"hello".parse::<CFString>()` — uses the `FromStr` impl.
- `format!("{}", my_cfstring)` — uses the `Display` impl.
- `my_cfstring.to_string()` — works because `Display` implies `ToString`.

**The pattern is:** wrap the C API in a Rust struct, then add Rust trait impls that call into the C API. The result is a type that feels native to Rust while talking to C under the hood.

### Section 4: The inherent `impl` block with safe methods

```rust
impl CFString {
    #[inline]
    pub fn new(string: &str) -> CFString {
        unsafe {
            let string_ref = CFStringCreateWithBytes(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
            );
            CFString::wrap_under_create_rule(string_ref)
        }
    }

    #[inline]
    pub fn from_static_string(string: &'static str) -> CFString {
        unsafe { /* similar but with NoCopy allocator */ }
    }

    #[inline]
    pub fn char_len(&self) -> CFIndex {
        unsafe { CFStringGetLength(self.0) }
    }
}
```

Each safe method is the same pattern:

1. **Safe signature** — takes Rust types (`&str`, `usize`, etc.), returns Rust types (`CFString`, `CFIndex`, etc.).
2. **`unsafe { ... }` body** — wraps the C call.
3. **`wrap_under_create_rule(ref)` or `wrap_under_get_rule(ref)`** to convert the raw pointer back into a safe wrapper.

The `#[inline]` attribute suggests the compiler inline the call (same as C++'s `inline`).

### Section 5: PartialEq with various Rust types

```rust
impl PartialEq<&str> for CFString {
    fn eq(&self, other: &&str) -> bool {
        // Create a temp CFString from `other`, then compare via CFEqual.
    }
}

impl PartialEq<CFString> for &str {
    fn eq(&self, other: &CFString) -> bool { other.eq(self) }
}
```

These let you write `cfstr == "hello"` and `"hello" == cfstr`. Both directions need an impl because Rust's `PartialEq` is symmetric in declaration but not in trait resolution.

### Section 6: Unit tests

```rust
#[test]
fn str_cmp() {
    let cfstr = CFString::new("hello");
    assert_eq!("hello", cfstr);
    assert_eq!(cfstr, "hello");
    // etc.
}
```

Tests live in the same file, gated by `#[cfg(test)]` for the test module. Rust's convention is `cargo test` runs every `#[test]` function in the crate.

---

With the pattern in mind, let's walk through each file.

## 3.1 `lib.rs` — crate root, `ConcreteCFType`, and the four macros

**Path:** `core-foundation/src/lib.rs` (260 lines)

**The conceptual heart of the library.** Defines:

- The `ConcreteCFType` trait (a marker trait like `Send` or `Sync`).
- Four declarative macros: `declare_TCFType!`, `impl_TCFType!`, `impl_CFTypeDescription!`, `impl_CFComparison!`.
- Module declarations.

### The `ConcreteCFType` marker trait (line 20)

```rust
pub unsafe trait ConcreteCFType: TCFType {}
```

`ConcreteCFType` is a **marker trait** — it has no methods, just constrains the type. It says "this type is a real, instantiable CFType wrapper (as opposed to the abstract `CFType` itself)". The `unsafe` keyword on the trait means: only the `impl_TCFType!` macro should implement it (because the macro verifies the right invariants).

Used as a bound on `CFType::downcast<T: ConcreteCFType>()` — only concrete types can be downcast to.

### The `declare_TCFType!` macro (lines 51–74)

```rust
#[macro_export]
macro_rules! declare_TCFType {
    (
        $(#[$doc:meta])*
        $ty:ident, $raw:ident
    ) => {
        declare_TCFType!($(#[$doc])* $ty<>, $raw);
    };

    (
        $(#[$doc:meta])*
        $ty:ident<$($p:ident $(: $bound:path)*),*>, $raw:ident
    ) => {
        $(#[$doc])*
        pub struct $ty<$($p $(: $bound)*),*>($raw, $(::std::marker::PhantomData<$p>),*);

        #[allow(unused_imports)]
        impl<$($p $(: $bound)*),*> Drop for $ty<$($p),*> {
            fn drop(&mut self) {
                use $crate::base::TCFType;
                unsafe { $crate::base::CFRelease(self.as_CFTypeRef()) }
            }
        }
    };
}
```

Let's break this down. This is a **declarative macro** (defined with `macro_rules!`). It's the Rust analogue of a C++ template — except it expands at compile time based on pattern matching, not type substitution.

**`#[macro_export]`** — makes the macro visible outside the crate.

**`macro_rules! declare_TCFType { ... }`** — defines a macro named `declare_TCFType`. The body has **two arms** (separated by `;`), each a pattern => expansion pair.

**Arm 1** (the simple form): matches `declare_TCFType!(CFString, CFStringRef);`. Captures:
- `$(#[$doc:meta])*` — zero or more doc attributes (e.g. `/// Some doc`).
- `$ty:ident` — the wrapper type name (`CFString`), as an identifier.
- `$raw:ident` — the raw ref type (`CFStringRef`).
Then forwards to arm 2 with an empty type-parameter list (`$ty<>`).

**Arm 2** (the generic form): matches `declare_TCFType!(CFArray<T>, CFArrayRef);` or the forwarded form from arm 1. The `<$($p:ident $(: $bound:path)*),*>` captures an optional list of generic parameters with optional trait bounds. Then expands to:

1. `pub struct $ty<$($p ...),*>($raw, $(::std::marker::PhantomData<$p>),*);` — declares the struct with one field (the raw ref) plus a `PhantomData` for each type parameter. The `PhantomData` is needed because Rust requires that generic parameters be "used" — `PhantomData` marks them as conceptually used without actually storing them.

2. `impl<...> Drop for $ty<...> { fn drop(&mut self) { ... } }` — implements `Drop` to call `CFRelease(self.as_CFTypeRef())`. The `as_CFTypeRef()` method comes from the `TCFType` trait (provided by the next macro). The `unsafe { ... }` block is needed because `CFRelease` is an FFI call.

**C++ analogue:** There isn't a clean one — C++ templates do something similar, but C++ doesn't have a "drop" hook that runs automatically. The closest is:

```cpp
template <typename Self, typename Ref>
class CFWrapper {
public:
    ~CFWrapper() { CFRelease(as_CFTypeRef()); }
    virtual RefType as_CFTypeRef() const = 0;  // subclass provides
};
```

But C++ templates are *monomorphised at instantiation*, whereas Rust macros are *expanded at parse time*. Different mechanism, similar effect.

### The `impl_TCFType!` macro (lines 84–178)

The biggest macro. Implements the `TCFType` trait, `Clone`, `PartialEq`, `Eq`, and `ToVoid` for the wrapper type.

The full body is large; key parts:

```rust
impl<$($p ...),*> $crate::base::TCFType for $ty<$($p),*> {
    type Ref = $ty_ref;

    fn as_concrete_TypeRef(&self) -> $ty_ref { self.0 }

    unsafe fn wrap_under_get_rule(reference: $ty_ref) -> Self {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        let reference = $crate::base::CFRetain(reference as *const ::core::ffi::c_void) as $ty_ref;
        $crate::base::TCFType::wrap_under_create_rule(reference)
    }

    fn as_CFTypeRef(&self) -> $crate::base::CFTypeRef {
        self.as_concrete_TypeRef() as $crate::base::CFTypeRef
    }

    unsafe fn wrap_under_create_rule(reference: $ty_ref) -> Self {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        $ty(reference $(, impl_TCFType!(@Phantom $p))*)
    }

    fn type_id() -> $crate::base::CFTypeID {
        unsafe { $ty_id() }
    }
}
```

The key concepts:

- **`type Ref = $ty_ref;`** — the associated type that says "this wrapper wraps `$ty_ref`". For `CFString`, `Ref = CFStringRef`.
- **`as_concrete_TypeRef(&self) -> $ty_ref`** — returns the underlying raw pointer. `self.0` is the first (and only non-Phantom) field of the wrapper struct.
- **`wrap_under_create_rule(reference)`** — wraps a freshly-Created reference (refcount already at 1; don't bump it). Used when you just called `CFStringCreate*` or similar.
- **`wrap_under_get_rule(reference)`** — wraps a "Get" reference (refcount not bumped; bump it via `CFRetain`). Used when you got a reference from a "get" function like `CFArrayGetValueAtIndex`.
- **`type_id()`** — returns the CFTypeID by calling the type-specific `XXXGetTypeID()` C function.

The "Create Rule" vs "Get Rule" distinction is **the same as metal-cpp's `TransferPtr` vs `RetainPtr`**. Same Core Foundation convention, just exposed differently:

| metal-cpp | core-foundation-rs | C rule |
|---|---|---|
| `TransferPtr(p)` | `wrap_under_create_rule(p)` | Create rule: refcount already 1, take ownership |
| `RetainPtr(p)` | `wrap_under_get_rule(p)` | Get rule: refcount unchanged, bump it |

The macro also generates `Clone` (calls `wrap_under_get_rule`), `PartialEq` (calls `CFEqual`), `Eq` (empty marker), and three `ToVoid` impls (for `&T`, `T`, and the raw ref — used for converting to `*const c_void`).

### The `impl_CFTypeDescription!` macro (lines 192–207)

Small macro that implements `std::fmt::Debug` by calling `CFCopyDescription` (which returns a `CFString` describing the object) and formatting it.

### The `impl_CFComparison!` macro (lines 210–238)

Implements `PartialOrd` and `Ord` by calling a type-specific compare function (e.g. `CFNumberCompare` for `CFNumber`).

### Module declarations (lines 240–259)

```rust
pub mod array;
pub mod attributed_string;
pub mod base;
// ... etc, one per file
```

Each `pub mod foo;` exposes `src/foo.rs` as a public module.

---

## 3.2 `base.rs` — the `TCFType` trait (the conceptual heart)

**Path:** `core-foundation/src/base.rs` (446 lines)

**The most important file in the whole library.** Defines:

- The `TCFType` trait (the Rust analogue of metal-cpp's `Referencing<T>` template).
- The `CFType` struct (the universal wrapper).
- The `ItemRef` / `ItemMutRef` lifetimed reference types.
- The `FromVoid` / `FromMutVoid` / `ToVoid` conversion traits.
- `CFIndexConvertible` (a bounds-checking helper).

### The `CFIndexConvertible` trait (lines 23–37)

```rust
pub trait CFIndexConvertible {
    fn to_CFIndex(self) -> CFIndex;
}

impl CFIndexConvertible for usize {
    #[inline]
    fn to_CFIndex(self) -> CFIndex {
        if self > (CFIndex::MAX as usize) {
            panic!("value out of range")
        }
        self as CFIndex
    }
}
```

A small safety helper. `CFIndex` is `isize` (signed), but most Rust code uses `usize` for lengths. Naively casting `usize → isize` could overflow if the usize is huge. This trait's `to_CFIndex` checks the bound and panics on overflow.

You'll see `.to_CFIndex()` called everywhere a Rust `usize` needs to become a `CFIndex` for an FFI call.

### The `CFType` struct + downcast (lines 39–110)

```rust
declare_TCFType! {
    /// Superclass of all Core Foundation objects.
    CFType, CFTypeRef
}

impl CFType {
    pub fn downcast<T: ConcreteCFType>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.0);
                Some(T::wrap_under_get_rule(reference))
            }
        } else {
            None
        }
    }

    pub fn downcast_into<T: ConcreteCFType>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.0);
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}
```

`CFType` is the **universal wrapper** — it can hold any Core Foundation object. The `downcast<T>()` method does a runtime type check (`instance_of::<T>()` calls `CFGetTypeID` and compares to `T::type_id()`), and returns `Some(T)` if the types match or `None` otherwise. This is the Rust equivalent of a C++ `dynamic_cast<T*>`.

The two variants:

- `downcast(&self) -> Option<T>` — borrows `self`, returns an `Option`. Bumps the refcount.
- `downcast_into(self) -> Option<T>` — consumes `self`, avoids the refcount bump via `mem::forget(self)`.

`mem::forget` is Rust's "don't run the destructor" function. It's used here because we're transferring ownership of the underlying CFType from `self` to the new wrapper — if we let `self`'s `Drop` run, it would call `CFRelease` and decrement the refcount, which we don't want.

### The `TCFType` trait (lines 156–218)

```rust
pub trait TCFType {
    type Ref: TCFTypeRef;

    fn as_concrete_TypeRef(&self) -> Self::Ref;
    unsafe fn wrap_under_create_rule(obj: Self::Ref) -> Self;
    fn type_id() -> CFTypeID;

    fn as_CFType(&self) -> CFType {
        unsafe { TCFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    fn into_CFType(self) -> CFType
    where Self: Sized,
    {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { TCFType::wrap_under_create_rule(reference) }
    }

    fn as_CFTypeRef(&self) -> CFTypeRef;
    unsafe fn wrap_under_get_rule(reference: Self::Ref) -> Self;

    fn retain_count(&self) -> CFIndex {
        unsafe { CFGetRetainCount(self.as_CFTypeRef()) }
    }

    fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    fn show(&self) {
        unsafe { CFShow(self.as_CFTypeRef()) }
    }

    fn instance_of<OtherCFType: TCFType>(&self) -> bool {
        self.type_of() == OtherCFType::type_id()
    }
}
```

This is the master trait. Walkthrough:

- **`type Ref: TCFTypeRef;`** — an **associated type** saying what raw pointer type this wrapper wraps. For `CFString`, this is `CFStringRef`. The bound `: TCFTypeRef` ensures the ref type can convert to/from `*const c_void`.
- **`as_concrete_TypeRef(&self) -> Self::Ref`** — get the raw pointer out.
- **`wrap_under_create_rule(obj)` / `wrap_under_get_rule(obj)`** — the two constructors (Create rule: take ownership; Get rule: bump refcount then take ownership). Both `unsafe` because the caller must ensure `obj` is a valid pointer.
- **`type_id()`** — the CF type identifier (class ID). Used for runtime type checks.
- **`as_CFType(&self) -> CFType`** — upcast to the universal `CFType`. Bumps refcount.
- **`into_CFType(self) -> CFType`** — upcast consuming self. Avoids refcount bump via `mem::forget`.
- **`as_CFTypeRef(&self) -> CFTypeRef`** — get the pointer as a generic `*const c_void`.
- **`retain_count(&self) -> CFIndex`** — the underlying refcount. Mostly useful for debugging.
- **`type_of(&self) -> CFTypeID`** — runtime type ID of the actual object.
- **`show(&self)`** — print to stderr (CFShow).
- **`instance_of<OtherCFType>(&self) -> bool`** — runtime type check.

**C++ equivalent (roughly):**

```cpp
template <typename Self, typename RefType>
class TCFType {
public:
    using Ref = RefType;

    virtual Ref as_concrete_TypeRef() const = 0;
    virtual CFTypeRef as_CFTypeRef() const = 0;
    static Self wrap_under_create_rule(Ref) { /* unsafe; user must implement */ }
    static Self wrap_under_get_rule(Ref)    { /* unsafe; user must implement */ }
    static CFTypeID type_id() = 0;            // pure virtual

    CFType as_CFType() const;                  // upcast, bumps refcount
    CFType into_CFType();                       // upcast, transfers ownership
    CFIndex retain_count() const { return CFGetRetainCount(as_CFTypeRef()); }
    CFTypeID type_of() const { return CFGetTypeID(as_CFTypeRef()); }
    bool instance_of(CFTypeID other) const { return type_of() == other; }
};
```

The Rust trait version is statically dispatched by default (no `virtual`), and the `Self::Ref` associated type gives per-type aliasing that C++ achieves with `using` declarations inside derived classes.

### `ItemRef<'a, T>` and `ItemMutRef<'a, T>` (lines 253–303)

```rust
pub struct ItemRef<'a, T: 'a>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<T> Deref for ItemRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
```

`ItemRef<'a, T>` is a **borrowed reference to an item inside a container**. The lifetime `'a` ties it to the container — the compiler ensures you can't drop the container while you still hold the `ItemRef`.

`ManuallyDrop<T>` wraps `T` but inhibits its `Drop`. Why? Because `ItemRef` represents a borrow — when it goes out of scope, the underlying item should NOT be released (the container still owns it). `ManuallyDrop` prevents the auto-`Drop`.

`Deref` lets you use `*item_ref` or `item_ref.method()` to call methods on the inner `T`.

### `FromVoid`, `FromMutVoid`, `ToVoid` traits (lines 306–390)

```rust
pub unsafe trait FromVoid {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self>
    where Self: Sized;
}

unsafe impl FromVoid for u32 { /* cast pointer to u32 */ }
unsafe impl FromVoid for *const c_void { /* identity cast */ }
unsafe impl<T: TCFType> FromVoid for T {
    unsafe fn from_void<'a>(x: *const c_void) -> ItemRef<'a, Self> {
        ItemRef(ManuallyDrop::new(TCFType::wrap_under_create_rule(T::Ref::from_void_ptr(x))), PhantomData)
    }
}
```

`FromVoid` is a trait that says "this type can be constructed from a `*const c_void`". Three implementations:

1. **For `u32`** — cast the pointer to a `u32` (used for collections that store small integers in pointers, like `CGFontCopyTableTags`).
2. **For `*const c_void`** — identity (the pointer is the value).
3. **For any `T: TCFType`** — wrap the pointer into a `TCFType` wrapper (this is how `CFArray<CFString>::get(i)` returns an `ItemRef<CFString>`).

`ToVoid<T>` is the reverse: convert a `T` to a `*const c_void`. The impls cover `&CFType`, `CFType`, and `CFTypeRef`. Used when calling C functions that need `*const c_void` arguments.

These traits let the container wrappers (`CFArray`, `CFDictionary`, `CFSet`) be generic over element type while still being able to convert between Rust types and the raw `*const c_void` that the C API uses.

### The `CFAllocator` wrapper (lines 136–147)

```rust
declare_TCFType!(CFAllocator, CFAllocatorRef);
impl_TCFType!(CFAllocator, CFAllocatorRef, CFAllocatorGetTypeID);

impl CFAllocator {
    pub fn new(mut context: CFAllocatorContext) -> CFAllocator {
        unsafe {
            let allocator_ref = CFAllocatorCreate(kCFAllocatorDefault, &mut context);
            TCFType::wrap_under_create_rule(allocator_ref)
        }
    }
}
```

A small wrapper for custom allocators. You'll rarely use this — most code uses `kCFAllocatorDefault` everywhere.

---

## 3.3 `string.rs` — `CFString`

**Path:** `core-foundation/src/string.rs` (210 lines)

The wrapper for `CFString`. Worth reading in full because it shows the full pattern: macros to declare, trait impls for idiomatic Rust, an inherent impl with safe methods.

### Key methods

```rust
impl CFString {
    pub fn new(string: &str) -> CFString {
        unsafe {
            let string_ref = CFStringCreateWithBytes(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
            );
            CFString::wrap_under_create_rule(string_ref)
        }
    }

    pub fn from_static_string(string: &'static str) -> CFString {
        unsafe {
            let string_ref = CFStringCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
                kCFAllocatorNull,    // don't try to free the static buffer
            );
            TCFType::wrap_under_create_rule(string_ref)
        }
    }

    pub fn char_len(&self) -> CFIndex {
        unsafe { CFStringGetLength(self.0) }
    }
}
```

- `new(string: &str)` — creates a `CFString` from any Rust `&str`. Uses `CFStringCreateWithBytes` with UTF-8 encoding.
- `from_static_string(string: &'static str)` — for string literals. Uses `CFStringCreateWithBytesNoCopy` with `kCFAllocatorNull`, so the CFString points directly at the binary's read-only data segment (no allocation, no copy). Same idea as metal-cpp's `MTLSTR("hello")` macro.
- `char_len(&self) -> CFIndex` — the number of UTF-16 code units (CFString's native length).

### The `From<&str>` impl

```rust
impl<'a> From<&'a str> for CFString {
    fn from(string: &'a str) -> CFString {
        CFString::new(string)
    }
}
```

Lets you write `CFString::from("hello")` or `"hello".into()`.

### The `Cow<'a, str>` conversion (lines 47–99)

```rust
impl<'a> From<&'a CFString> for Cow<'a, str> {
    fn from(cf_str: &'a CFString) -> Cow<'a, str> {
        unsafe {
            // Try to get a pointer to the underlying UTF-8 buffer directly.
            let c_string = CFStringGetCStringPtr(cf_str.0, kCFStringEncodingUTF8);
            if !c_string.is_null() {
                // Best case: the CFString already stores UTF-8 internally.
                let c_str = CStr::from_ptr(c_string);
                Cow::Borrowed(str::from_utf8_unchecked(c_str.to_bytes()))
            } else {
                // Fallback: ask CFString how many bytes we need, allocate a buffer,
                // and call CFStringGetBytes to copy.
                // ... ~30 lines of buffer-allocation code ...
                Cow::Owned(String::from_utf8_unchecked(buffer))
            }
        }
    }
}
```

This is one of the most interesting impls. `Cow<'a, str>` ("copy-on-write string") is either a borrow of some existing string data (`Cow::Borrowed(&str)`) or an owned `String` (`Cow::Owned(String)`). The impl tries the cheap path first — if the CFString happens to store UTF-8 internally, you get a zero-copy borrow. Otherwise it allocates a `String` and copies.

**This is exactly the kind of impl that makes a Rust API feel native** — without ever exposing the user to the underlying C complexity.

### The `Display` and `Debug` impls

```rust
impl fmt::Display for CFString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}

impl fmt::Debug for CFString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}
```

- `Display` lets `format!("{}", cfstr)` work.
- `Debug` lets `format!("{:?}", cfstr)` work, with quotes around the string for clarity.

### The `PartialEq` impls

```rust
impl PartialEq<&str> for CFString { /* compare via temp CFString + CFEqual */ }
impl PartialEq<CFString> for &str { /* delegate */ }
impl PartialEq<CFString> for String { /* delegate via as_str() */ }
impl PartialEq<String> for CFString { /* delegate */ }
```

Four impls so that all of `cfstr == "hello"`, `"hello" == cfstr`, `cfstr == my_string`, and `my_string == cfstr` work.

---

## 3.4 `array.rs` — `CFArray<T>`

**Path:** `core-foundation/src/array.rs` (310 lines)

A pedagogically rich file — shows **generics**, **iterators**, **lifetimes**, and the `FromVoid` trait integration.

### The struct (line 25)

```rust
pub struct CFArray<T = *const c_void>(CFArrayRef, PhantomData<T>);
```

- Generic over `T` (the element type), with a default of `*const c_void` (untyped).
- The struct holds the raw `CFArrayRef` plus a `PhantomData<T>` to "use" the type parameter (Rust requires this).
- The `Drop` impl is hand-written (not via `declare_TCFType!`) because the struct uses an explicit generic.

### The iterator (lines 33–57)

```rust
pub struct CFArrayIterator<'a, T: 'a> {
    array: &'a CFArray<T>,
    index: CFIndex,
    len: CFIndex,
}

impl<'a, T: FromVoid> Iterator for CFArrayIterator<'a, T> {
    type Item = ItemRef<'a, T>;

    fn next(&mut self) -> Option<ItemRef<'a, T>> {
        if self.index >= self.len {
            None
        } else {
            let value = unsafe { self.array.get_unchecked(self.index) };
            self.index += 1;
            Some(value)
        }
    }
}

impl<T: FromVoid> ExactSizeIterator for CFArrayIterator<'_, T> {
    fn len(&self) -> usize {
        (self.array.len() - self.index) as usize
    }
}
```

The iterator borrows the array (`'a` lifetime), tracks the current `index` and total `len`. The `Iterator` trait's `next()` returns `Option<ItemRef<'a, T>>` — `None` when done, `Some(item)` otherwise.

`ExactSizeIterator` is a sub-trait that adds a `len()` method — lets you ask how many items remain without consuming the iterator.

**C++ analogue:**

```cpp
template <typename T>
class CFArrayIterator {
    const CFArray<T>* array;
    CFIndex index, len;
public:
    using iterator_category = std::input_iterator_tag;
    using value_type = T;
    using difference_type = std::ptrdiff_t;
    using pointer = T*;
    using reference = T&;

    bool operator!=(const CFArrayIterator& other) const { return index != other.index; }
    reference operator*() const { /* unsafe get */ }
    CFArrayIterator& operator++() { ++index; return *this; }
};
```

Rust's iterator pattern is conceptually identical but with a single `next()` method instead of the C++ iterator protocol.

### The `from_CFTypes` constructor (lines 82–96)

```rust
pub fn from_CFTypes(elems: &[T]) -> CFArray<T>
where T: TCFType,
{
    unsafe {
        let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()). collect();
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            elems.as_ptr(),
            elems.len().to_CFIndex(),
            &kCFTypeArrayCallBacks,
        );
        TCFType::wrap_under_create_rule(array_ref)
    }
}
```

Creates a `CFArray` from a slice of `TCFType` wrappers. The `T: TCFType` bound ensures each element can be converted to a `CFTypeRef`. Uses `kCFTypeArrayCallBacks` (the prebuilt callbacks that make the array retain/release its elements).

### The `get` and `get_unchecked` methods (lines 138–155)

```rust
pub unsafe fn get_unchecked(&self, index: CFIndex) -> ItemRef<'_, T>
where T: FromVoid,
{
    T::from_void(CFArrayGetValueAtIndex(self.0, index))
}

pub fn get(&self, index: CFIndex) -> Option<ItemRef<'_, T>>
where T: FromVoid,
{
    if index < self.len() {
        Some(unsafe { T::from_void(CFArrayGetValueAtIndex(self.0, index)) })
    } else {
        None
    }
}
```

- `get_unchecked` — no bounds check. `unsafe` because passing an out-of-bounds index is UB.
- `get` — bounds-checked. Returns `None` if out of bounds.

Both return `ItemRef<'_, T>` — a borrow tied to the array's lifetime (the `'_` is elided lifetime syntax).

### The `IntoIterator` impl (lines 174–181)

```rust
impl<'a, T: FromVoid> IntoIterator for &'a CFArray<T> {
    type Item = ItemRef<'a, T>;
    type IntoIter = CFArrayIterator<'a, T>;

    fn into_iter(self) -> CFArrayIterator<'a, T> {
        self.iter()
    }
}
```

This lets you write `for elem in &array { ... }`. The `for` desugars to `array.into_iter()` then loops calling `next()`.

---

## 3.5 `dictionary.rs` — `CFDictionary<K, V>` + `CFMutableDictionary`

**Path:** `core-foundation/src/dictionary.rs` (482 lines)

Generic over both keys and values. Has both an immutable and a mutable variant.

### The struct

```rust
pub struct CFDictionary<K = *const c_void, V = *const c_void>(
    CFDictionaryRef,
    PhantomData<K>,
    PhantomData<V>,
);
```

Two type parameters, each with a `PhantomData` to satisfy Rust's "type params must be used" rule.

### `from_CFType_pairs` (lines 43–64)

```rust
pub fn from_CFType_pairs(pairs: &[(K, V)]) -> CFDictionary<K, V>
where K: TCFType, V: TCFType,
{
    let (keys, values): (Vec<CFTypeRef>, Vec<CFTypeRef>) = pairs
        .iter()
        .map(|(key, value)| (key.as_CFTypeRef(), value.as_CFTypeRef()))
        .unzip();

    unsafe {
        let dictionary_ref = CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_ptr(),
            values.as_ptr(),
            keys.len().to_CFIndex(),
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        TCFType::wrap_under_create_rule(dictionary_ref)
    }
}
```

Takes a slice of `(K, V)` tuples, unzips into separate `keys` and `values` vectors, calls `CFDictionaryCreate`. The `unzip()` is a standard Rust iterator trick — it splits an iterator of `(A, B)` into two separate `Vec<A>` and `Vec<B>`.

### `find` and `get` (lines 106–134)

```rust
pub fn find<T: ToVoid<K>>(&self, key: T) -> Option<ItemRef<'_, V>>
where V: FromVoid, K: ToVoid<K>,
{
    unsafe {
        let mut value: *const c_void = ptr::null();
        if CFDictionaryGetValueIfPresent(self.0, key.to_void(), &mut value) != 0 {
            Some(V::from_void(value))
        } else {
            None
        }
    }
}

pub fn get<T: ToVoid<K>>(&self, key: T) -> ItemRef<'_, V>
where V: FromVoid, K: ToVoid<K>,
{
    self.find(key).unwrap_or_else(|| panic!("No entry found for key"))
}
```

- `find(key) -> Option<ItemRef<V>>` — returns `Some` if the key exists, `None` otherwise.
- `get(key) -> ItemRef<V>` — panics if not found. (Same convention as `HashMap::get` vs `HashMap::get_mut` in stdlib.)

The bound `T: ToVoid<K>` lets you pass either a `K` directly or a `&K` (both impl `ToVoid<K>`).

### `CFMutableDictionary` (lines 151–331)

A second struct with mutable operations: `add`, `set`, `replace`, `remove`, `remove_all`. Same pattern as the immutable version but with mutable method signatures.

Implements `Default` (line 333) so `CFMutableDictionary::default()` works.

---

## 3.6 `set.rs` — `CFSet`

**Path:** `core-foundation/src/set.rs`

Same pattern as `CFArray<T>` but for unordered unique elements. ~80 lines. Methods: `from_CFTypes`, `len`, `iter`, `get_values`.

---

## 3.7 `data.rs` — `CFData` (raw bytes)

**Path:** `core-foundation/src/data.rs`

A byte buffer wrapper. The interesting part is the `Deref` impl:

```rust
impl Deref for CFData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(CFDataGetBytePtr(self.0), CFDataGetLength(self.0) as usize) }
    }
}
```

`Deref<Target = [u8]>` lets you treat a `CFData` as a `&[u8]` — you can call `data.len()`, `data.iter()`, `&data[0..10]`, etc. directly. This is the standard Rust idiom for "newtype that exposes a slice view".

**C++ analogue:**

```cpp
class CFData {
    CFDataRef ref;
public:
    const uint8_t* data() const { return CFDataGetBytePtr(ref); }
    size_t size() const { return CFDataGetLength(ref); }
    const uint8_t& operator[](size_t i) const { return data()[i]; }
    // etc.
};
```

Rust's `Deref` achieves the same thing more uniformly — every type that implements `Deref<Target = T>` is implicitly usable as `&T`.

---

## 3.8 `number.rs` — `CFNumber` (with `From` impls for every numeric type)

**Path:** `core-foundation/src/number.rs` (151 lines)

Wraps `CFNumberRef`. Shows the pattern of implementing `From` for every Rust numeric type so you can write `CFNumber::from(42)`.

```rust
impl From<i32> for CFNumber {
    fn from(value: i32) -> Self {
        unsafe {
            let number_ref = CFNumberCreate(
                kCFAllocatorDefault,
                kCFNumberSInt32Type,
                &value as *const i32 as *const c_void,
            );
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}

// Same pattern for i64, f32, f64.
```

Four `From` impls (i32, i64, f32, f64). The wrapper also has `to_i32`, `to_i64`, `to_f32`, `to_f64` methods returning `Option<T>` — `None` if the underlying CFNumber can't be converted to that type without loss.

Plus `impl_CFComparison!(CFNumber, CFNumberCompare)` — implements `Ord` via the C `CFNumberCompare` function.

---

## 3.9 `boolean.rs` — `CFBoolean`

Tiny file (~50 lines). Wraps `CFBooleanRef`. Has `true_value()` and `false_value()` constructors that return the singleton `kCFBooleanTrue` / `kCFBooleanFalse` wrapped in `CFBoolean`.

```rust
impl From<bool> for CFBoolean {
    fn from(value: bool) -> Self {
        if value { Self::true_value() } else { Self::false_value() }
    }
}
```

---

## 3.10 `date.rs` — `CFDate`

Wraps `CFDateRef`. Constructor `new(time: CFAbsoluteTime) -> CFDate`. Method `abs_time(&self) -> CFAbsoluteTime`. Plus comparison via `impl_CFComparison!`.

---

## 3.11 `error.rs` — `CFError`

Wraps `CFErrorRef`. Methods: `new(domain, code, user_info)`, `domain()`, `code()`, `to_string()`. Implements `std::error::Error` and `Display` so it integrates with Rust's error-handling ecosystem (you can use it with `?`).

This is the file that shows how to make a C error type feel native to Rust's `Result`-based error handling.

---

## 3.12 `url.rs` — `CFURL`

Wraps `CFURLRef`. Methods: `new_with_file_system_path`, `from_file_system_path`, `get_string`, `get_file_system_representation`. Plus conversions to/from Rust paths.

---

## 3.13 `bundle.rs` — `CFBundle`

Wraps `CFBundleRef`. Methods: `new`, `main_bundle`, `get_package_identifier`, `copy_info_dictionary_for_url`, `copy_resource_url`, etc.

---

## 3.14 `runloop.rs` — `CFRunLoop`

Wraps `CFRunLoopRef`. Methods: `get_current`, `get_main`, `run`, `stop`, `add_source`, `remove_source`, `add_timer`, `remove_timer`. Used for event loops.

---

## 3.15 `mach_port.rs` — `CFMachPort`

Wraps `CFMachPortRef`. Methods: `create`, `get_port`, `invalidate`. Used for kernel IPC.

---

## 3.16 `filedescriptor.rs` — `CFFileDescriptor`

Wraps `CFFileDescriptorRef`. Methods: `create`, `get_native_descriptor`, `enable_callbacks`, `disable_callbacks`. Used for run-loop-integrated file I/O.

---

## 3.17 `characterset.rs` — `CFCharacterSet`

Wraps `CFCharacterSetRef`. Methods: `from_characters`, `is_character_member`, `is_superset_of`, `invert`. Used for Unicode character classes.

---

## 3.18 `propertylist.rs` — `CFPropertyList`

The interesting one. `CFPropertyList` is defined as an enum with variants for each plist type:

```rust
pub enum CFPropertyList {
    CFArray(CFArray),
    CFBoolean(CFBoolean),
    CFData(CFData),
    CFDate(CFDate),
    CFDictionary(CFDictionary),
    CFNumber(CFNumber),
    CFString(CFString),
}
```

This is the Rust-idiomatic way to model a "any of these types" value — the enum carries the actual value, and pattern matching lets you dispatch on it safely.

The file also has `CFPropertyList::from_CFType(cf_type)` which downcasts the input `CFType` to the right variant, and `into_CFType(self) -> CFType` for the reverse direction.

Plus methods for serialising/deserialising plist data: `from_data`, `to_data`.

---

## 3.19 `timezone.rs` — `CFTimeZone`

Wraps `CFTimeZoneRef`. Methods: `new`, `get_name`, `get_seconds_from_gmt`, etc.

---

## 3.20 `uuid.rs` — `CFUUID` (feature-gated)

Feature-gated behind `with-uuid`. When enabled, provides conversions between `CFUUID` and the `uuid` crate's `Uuid` type.

---

## 3.21 `attributed_string.rs` — `CFAttributedString`

Wraps `CFAttributedStringRef`. An attributed string is a `CFString` plus a `CFDictionary` of attributes (font, color, etc.) per character range.

---

## 3.22 Summary

After Phase 3, you should be able to:

1. **Explain the wrapper pattern**: macros declare struct + Drop + TCFType impl; trait impls add idiomatic Rust API; inherent impl provides safe methods.
2. **Explain Create vs Get rule**: `wrap_under_create_rule` (refcount already 1) vs `wrap_under_get_rule` (bump refcount first).
3. **Explain `TCFType`**: the trait that every wrapper implements, with associated type `Ref`.
4. **Explain `ItemRef<'a, T>`**: a lifetimed borrow of an item inside a container.
5. **Explain `FromVoid` / `ToVoid`**: the conversion traits that let containers be generic over element type.
6. **Read any wrapper file** in 5 minutes and understand what it does.
7. **Explain how the macros reduce boilerplate**: each wrapper is ~100 lines thanks to `declare_TCFType!` / `impl_TCFType!` generating ~200 lines of boilerplate.
8. **Explain the C++ analogues**: `TCFType` ↔ abstract base class; `wrap_under_*` ↔ `RetainPtr` / `TransferPtr`; `Drop` ↔ destructor; trait impls ↔ implicit conversions; `ItemRef` ↔ iterator-with-lifetime.

The patterns you've learned here — safe wrapper around unsafe FFI, traits for idiomatic API, lifetimes for borrow tracking, macros for boilerplate reduction — are **universal**. You'll see them in every Rust FFI library, from `libsqlite3-sys` to `winsafe` to `glib`. Phase 3 is the conceptual heart of this whole guide.

**Open File 4 when ready** to start the `core-graphics` walkthrough.

— End of File 3 —
