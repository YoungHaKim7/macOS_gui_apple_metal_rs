# File 2 — `core-foundation-sys/` Walkthrough (40 FFI files)

**File 2 of 8 — the 40 files in `core-foundation-sys/src/`.**

> **What this crate is:** Pure FFI bindings. Every file is a transcription of an Apple C header (`CFBase.h`, `CFString.h`, `CFArray.h`, etc.) into Rust `extern "C"` declarations. There is no safe wrapper here — calling any of these functions requires `unsafe`. The safe wrappers live in `core-foundation/` (File 3).
>
> **Why it exists as a separate crate:** Separation of concerns. Other crates (`core-foundation`, `core-graphics`, `cocoa`) need to call these C functions but don't want to redeclare them. By putting the declarations in their own crate, every consumer can `use core_foundation_sys::*` and get consistent types.
>
> **Lines:** ~5,610 across 40 files.

---

## Table of Contents

- The FFI pattern (read this once — every file follows it)
- 2.1 `lib.rs` — crate root
- 2.2 `base.rs` — the foundational types (most important file)
- 2.3 `string.rs` — `CFString` FFI
- 2.4 `array.rs` — `CFArray` FFI
- 2.5 `dictionary.rs` — `CFDictionary` FFI
- 2.6 `set.rs` and `bag.rs` — collection variants
- 2.7 `data.rs` — `CFData`
- 2.8 `number.rs` and `boolean.rs` — boxed scalars
- 2.9 `date.rs` — timestamps
- 2.10 `error.rs` — `CFError`
- 2.11 `url.rs` — URLs
- 2.12 `bundle.rs` — app bundles
- 2.13 `runloop.rs` and `mach_port.rs` — event loops
- 2.14 `filedescriptor.rs` — file descriptor sources
- 2.15 `characterset.rs` and `bit_vector.rs` — small collections
- 2.16 `timezone.rs`, `calendar.rs`, `locale.rs` — date/time utilities
- 2.17 `propertylist.rs` and `preferences.rs` — plist + user defaults
- 2.18 `notification_center.rs`, `messageport.rs`, `socket.rs`, `stream.rs` — IPC
- 2.19 `attributed_string.rs`, `string_tokenizer.rs` — string utilities
- 2.20 `tree.rs`, `binary_heap.rs` — less-common collections
- 2.21 `xml_node.rs`, `xml_parser.rs` — XML (macOS only)
- 2.22 `date_formatter.rs`, `number_formatter.rs` — formatters
- 2.23 `plugin.rs`, `user_notification.rs`, `url_enumerator.rs`, `file_security.rs` — remaining
- 2.24 `uuid.rs` — UUIDs
- Summary: what you've learned

---

## The FFI pattern (read this once — every file follows it)

Every file in this crate follows the same five-section template. Learn it once and you can read any file in the crate.

### Section 1: License header (9 lines)

```rust
// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
```

Identical across every file. Skip on subsequent reads.

### Section 2: `use` imports

```rust
use core::ffi::c_void;
use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID};
use crate::string::CFStringRef;
```

Pulls in types defined in sibling modules. `core::ffi::c_void` is Rust's standard `void` type (used for opaque pointers). `crate::base::*` are the foundational types defined in `base.rs` (covered in §2.2).

### Section 3: Type aliases for callback function pointers

```rust
pub type CFArrayRetainCallBack =
    extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;
pub type CFArrayReleaseCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFArrayEqualCallBack =
    extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
```

**C++ equivalent:**

```cpp
using CFArrayRetainCallBack = const void* (*)(CFAllocatorRef, const void*);
using CFArrayReleaseCallBack = void (*)(CFAllocatorRef, const void*);
using CFArrayEqualCallBack = Boolean (*)(const void*, const void*);
```

`extern "C" fn(...)` is Rust's syntax for a C-compatible function pointer. The arguments are typed just like Rust function arguments. These are used in callback tables (next section).

### Section 4: Callback table structs

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CFArrayCallBacks {
    pub version: CFIndex,
    pub retain: CFArrayRetainCallBack,
    pub release: CFArrayReleaseCallBack,
    pub copyDescription: CFArrayCopyDescriptionCallBack,
    pub equal: CFArrayEqualCallBack,
}
```

**C++ equivalent:**

```cpp
struct CFArrayCallBacks {
    CFIndex version;
    CFArrayRetainCallBack retain;
    CFArrayReleaseCallBack release;
    CFArrayCopyDescriptionCallBack copyDescription;
    CFArrayEqualCallBack equal;
};
```

`#[repr(C)]` is critical: it tells the Rust compiler to lay out the struct exactly as C would (no reordering, no padding optimisation). Without it, Rust is free to reorder fields and the struct wouldn't match the C side.

`#[derive(Clone, Copy, Debug)]` auto-implements the `Clone`, `Copy`, and `Debug` traits. `Copy` means the struct is bitwise-copied on assignment (it's small — just 5 pointers — so this is fine). `Debug` lets you print it with `{:?}`.

### Section 5: The opaque struct + reference alias

```rust
#[repr(C)]
pub struct __CFArray(c_void);

pub type CFArrayRef = *const __CFArray;
pub type CFMutableArrayRef = *mut __CFArray;
```

This is the **opaque struct pattern**. Core Foundation's C API works with opaque pointers — you never see the internals of a `CFArray`. The Rust binding models this as:

- A struct `__CFArray` with a single private field of type `c_void` (literally nothing — it's a zero-sized placeholder).
- A type alias `CFArrayRef = *const __CFArray` — a typed pointer that's nominally to a `__CFArray` but is really just an opaque address.

**Why bother with the `__CFArray` struct at all?** Because Rust's type system is strict about pointer types. If you wrote `pub type CFArrayRef = *const c_void;` (like the underlying C type really is), then `CFArrayRef` and `CFStringRef` and `CFDictionaryRef` would all be the same type — and the compiler couldn't catch you passing a `CFStringRef` to a function expecting a `CFArrayRef`. By introducing distinct opaque struct types, each `...Ref` alias becomes a distinct Rust type, and the compiler catches type mixups.

**C++ analogue:** C doesn't enforce this either, but C++ with strong typedefs (or just distinct `class` declarations) can. The pattern is the same idea as metal-cpp's `MTL4::ArgumentTable` vs. `MTL4::CommandEncoder` — distinct types for distinct concepts, even though under the hood they're all `objc_object*`.

### Section 6: The `extern "C"` block

```rust
extern "C" {
    pub static kCFTypeArrayCallBacks: CFArrayCallBacks;

    pub fn CFArrayCreate(
        allocator: CFAllocatorRef,
        values: *const *const c_void,
        numValues: CFIndex,
        callBacks: *const CFArrayCallBacks,
    ) -> CFArrayRef;

    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    // ... more functions
}
```

The `extern "C" { ... }` block declares C functions and globals that the linker will resolve at link time. Each item is:

- `pub static NAME: Type;` — a C global variable (e.g. `kCFTypeArrayCallBacks` is a pre-built callbacks struct provided by Core Foundation).
- `pub fn NAME(args) -> ret;` — a C function. The signature must match the C header exactly.

Calling any of these requires `unsafe` (Rust can't verify that the C side upholds its contracts). You'll see this in the `core-foundation` crate's safe wrappers.

**C++ equivalent:** Just declaring the functions in a header:

```cpp
extern const CFArrayCallBacks kCFTypeArrayCallBacks;

CFArrayRef CFArrayCreate(CFAllocatorRef allocator,
                         const void* const* values,
                         CFIndex numValues,
                         const CFArrayCallBacks* callBacks);

CFIndex CFArrayGetCount(CFArrayRef theArray);
const void* CFArrayGetValueAtIndex(CFArrayRef theArray, CFIndex idx);
```

C++ doesn't require `unsafe` to call them — you can call into UB freely. That's the safety win Rust gives you.

---

## 2.1 `lib.rs` — crate root

**Path:** `core-foundation-sys/src/lib.rs` (81 lines)

The crate root. Three things happen here:

### The `#![allow(...)]` attributes (lines 9–14)

```rust
#![allow(
    non_snake_case,        // CFArrayCreate is not snake_case
    non_camel_case_types,  // __CFArray is not CamelCase
    non_upper_case_globals,// kCFAllocatorDefault is not UPPER_CASE
    improper_ctypes        // *const c_void in extern fn args is technically improper
)]
```

Rust has naming conventions enforced by lints: functions should be `snake_case`, types `CamelCase`, constants `UPPER_CASE`. Core Foundation's C API violates all of these (Apple uses `CFArrayCreate`, `__CFArray`, `kCFAllocatorDefault`). The `#![allow(...)]` attributes at the crate root suppress those lints for the entire crate.

The `#![...]` form (with `!`) means "crate-level attribute" — applies to the whole crate, not just the module. Without the `!`, it'd be module-level.

### The framework link directive (lines 24–37)

```rust
#[cfg_attr(
    all(
        any(target_os = "macos", target_os = "ios", target_os = "tvos",
            target_os = "watchos", target_os = "visionos"),
        feature = "link"
    ),
    link(name = "CoreFoundation", kind = "framework")
)]
extern "C" {}
```

This tells the linker: "if we're compiling for an Apple OS and the `link` feature is on, link against the `CoreFoundation` framework." The empty `extern "C" {}` block is just a syntactic hook for the `#[link(...)]` attribute — it declares no functions itself, but the link directive applies to the whole crate.

`#[cfg_attr(cond, attr)]` means "if `cond` is true, apply `attr`". It's shorthand for `#[cfg(cond)] #[attr]`.

**C++ equivalent:** `-framework CoreFoundation` on the linker command line, or in CMake: `target_link_libraries(my_target "-framework CoreFoundation")`.

### The module declarations (lines 39–80)

```rust
pub mod array;
pub mod attributed_string;
pub mod bag;
pub mod base;
// ... 36 more
#[cfg(target_os = "macos")]
pub mod user_notification;
#[cfg(target_os = "macos")]
pub mod xml_node;
```

Each `pub mod foo;` declaration tells Rust: "there's a file `src/foo.rs`; expose its contents as a public module named `foo`." The `#[cfg(target_os = "macos")]` attributes make some modules macOS-only — `user_notification`, `xml_node`, `xml_parser` aren't available on iOS.

---

## 2.2 `base.rs` — the foundational types

**Path:** `core-foundation-sys/src/base.rs` (198 lines)

**The most important file in the crate.** Every other file depends on the types defined here.

### Type aliases (lines 14–36)

```rust
pub type Boolean = u8;                // CF's Boolean is an unsigned byte
pub type mach_port_t = c_uint;
pub type CFAllocatorRef = *const c_void;
pub type CFNullRef = *const c_void;
pub type CFTypeRef = *const c_void;   // generic "any CF object" pointer
pub type ConstStr255Param = *const c_uchar;
pub type StringPtr = *mut c_uchar;
pub type ConstStringPtr = *const c_uchar;
pub type OSStatus = i32;
pub type UInt8 = c_uchar;
pub type UInt16 = c_ushort;
pub type SInt16 = c_short;
pub type SInt32 = c_int;
pub type UInt32 = c_uint;
pub type CFTypeID = usize;            // type identifier
pub type CFOptionFlags = usize;       // bit-flags
pub type CFHashCode = usize;          // hash result
pub type CFIndex = isize;             // signed count/index
```

`pub type X = Y;` is Rust's syntax for a type alias — exactly C++'s `using X = Y;` (or the older `typedef Y X;`).

Note the use of `core::ffi::c_int`, `c_uchar`, etc. — these are Rust's portable aliases for C's `int`, `unsigned char`, etc. They ensure the Rust type matches the C ABI on every platform.

`CFTypeRef = *const c_void` is the master type — every CF object is ultimately passed around as a `CFTypeRef`. `CFStringRef`, `CFArrayRef`, etc. are more specific typed pointers (to their respective opaque structs) but can be cast to `CFTypeRef` when needed.

### `CFComparisonResult` enum (lines 38–44)

```rust
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFComparisonResult {
    LessThan = -1,
    EqualTo = 0,
    GreaterThan = 1,
}
```

`#[repr(isize)]` makes the enum's underlying representation `isize` (C's `intptr_t`-sized integer). The three variants get explicit values `-1`, `0`, `1` — matching `NSComparisonResult` in Apple's APIs.

**C++ equivalent:**

```cpp
enum class CFComparisonResult : intptr_t {
    LessThan = -1,
    EqualTo = 0,
    GreaterThan = 1,
};
```

### `From<CFComparisonResult> for Ordering` impl (lines 52–60)

```rust
impl From<CFComparisonResult> for Ordering {
    fn from(val: CFComparisonResult) -> Self {
        match val {
            CFComparisonResult::LessThan => Ordering::Less,
            CFComparisonResult::EqualTo => Ordering::Equal,
            CFComparisonResult::GreaterThan => Ordering::Greater,
        }
    }
}
```

This implements the standard `From` trait, letting you convert `CFComparisonResult` into Rust's standard `std::cmp::Ordering` with `.into()`. **This is the first bit of "Rust idiom" in the file** — wrapping the C enum in a way that integrates with Rust's standard library.

**C++ equivalent:** No built-in conversion; you'd write a helper function.

### `CFRange` struct (lines 62–74)

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

impl CFRange {
    pub fn init(location: CFIndex, length: CFIndex) -> CFRange {
        CFRange { location, length }
    }
}
```

A simple `{ location, length }` struct matching C's `CFRange`. The `init` method is a constructor — `CFRange::init(0, 10)` makes a range from index 0 to 10. The comment "// for back-compat" explains why it's named `init` (matching the old C convention) rather than `new` (the Rust convention).

### Callback function pointer types (lines 76–89)

```rust
pub type CFAllocatorRetainCallBack = extern "C" fn(info: *mut c_void) -> *mut c_void;
pub type CFAllocatorReleaseCallBack = extern "C" fn(info: *mut c_void);
pub type CFAllocatorCopyDescriptionCallBack = extern "C" fn(info: *mut c_void) -> CFStringRef;
pub type CFAllocatorAllocateCallBack =
    extern "C" fn(allocSize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void;
// ... etc
```

These define the signatures of the callbacks a custom `CFAllocator` can provide. Each is a C function pointer.

### `CFAllocatorContext` struct (lines 91–103)

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CFAllocatorContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<CFAllocatorRetainCallBack>,
    pub release: Option<CFAllocatorReleaseCallBack>,
    pub copyDescription: Option<CFAllocatorCopyDescriptionCallBack>,
    pub allocate: Option<CFAllocatorAllocateCallBack>,
    pub reallocate: Option<CFAllocatorReallocateCallBack>,
    pub deallocate: Option<CFAllocatorDeallocateCallBack>,
    pub preferredSize: Option<CFAllocatorPreferredSizeCallBack>,
}
```

The configuration struct for a custom allocator. Each callback is `Option<SomeFnPtr>` — `Option<extern "C" fn(...)>` is Rust's representation of a nullable function pointer: `Some(ptr)` for a real function, `None` for "no callback" (i.e. `NULL` on the C side).

**C++ equivalent:** C++ would just use null function pointers; Rust's `Option` makes the nullability explicit.

### The `TCFTypeRef` trait (lines 105–130)

```rust
pub trait TCFTypeRef {
    fn as_void_ptr(&self) -> *const c_void;
    unsafe fn from_void_ptr(ptr: *const c_void) -> Self;
}

impl<T> TCFTypeRef for *const T {
    fn as_void_ptr(&self) -> *const c_void {
        (*self) as *const c_void
    }
    unsafe fn from_void_ptr(ptr: *const c_void) -> Self {
        ptr as *const T
    }
}

impl<T> TCFTypeRef for *mut T {
    // same idea for mutable pointers
}
```

**This is the bridge between typed pointers and the generic `CFTypeRef`.** The trait says "any raw pointer type can convert itself to a `*const c_void` and back." The blanket implementations (`impl<T> TCFTypeRef for *const T` and `*mut T`) provide the implementation for ALL raw pointers automatically.

Why is this needed? Because Core Foundation has functions like `CFRelease(CFTypeRef)` and `CFGetTypeID(CFTypeRef)` that take a generic pointer, but you usually have a typed pointer like `CFStringRef`. You need to convert between them. `TCFTypeRef` provides a uniform conversion path.

The `from_void_ptr` is `unsafe` because there's no runtime check that the pointer really points to the type you claim — you could lie. (The safe wrappers in `core-foundation/` add runtime type checks via `CFGetTypeID`.)

### `kCFNotFound` constant (line 133)

```rust
pub static kCFNotFound: CFIndex = -1;
```

A constant `static` (program-lifetime variable) for the "not found" sentinel. Returned by functions like `CFArrayGetFirstIndexOfValue` when the search finds nothing.

**C++ equivalent:** `static const CFIndex kCFNotFound = -1;`

### The `extern "C"` block (lines 135–197)

This is the meat of the file: declarations of every C function and global in `CFBase.h`. Key entries:

- **Allocator functions:** `CFAllocatorCreate`, `CFAllocatorAllocate`, `CFAllocatorDeallocate`, etc.
- **Allocator globals:** `kCFAllocatorDefault`, `kCFAllocatorSystemDefault`, `kCFAllocatorMalloc`, `kCFAllocatorMallocZone`, `kCFAllocatorNull`, `kCFAllocatorUseContext`.
- **Universal CFType functions:** `CFRelease`, `CFRetain`, `CFGetRetainCount`, `CFGetTypeID`, `CFEqual`, `CFHash`, `CFCopyDescription`, `CFShow`, `CFGetAllocator`.
- **Misc:** `kCFNull` (the singleton null object), `CFNullGetTypeID`, `CFCopyTypeIDDescription`.

These are the functions that operate on ANY Core Foundation object. You'll see them used heavily in the safe wrappers.

---

## 2.3 `string.rs` — `CFString` FFI

**Path:** `core-foundation-sys/src/string.rs` (547 lines)

The longest file in the crate because `CFString` has so many operations. Structure:

### Compare flags (lines 21–30)

```rust
pub type CFStringCompareFlags = CFOptionFlags;
pub const kCFCompareCaseInsensitive: CFStringCompareFlags = 1;
pub const kCFCompareBackwards: CFStringCompareFlags = 4;
pub const kCFCompareAnchored: CFStringCompareFlags = 8;
// ... etc
```

Constants for the bitmask passed to `CFStringCompare`. `pub const X: T = value;` is Rust's syntax for a named constant.

### Encoding constants (lines 32–185)

A huge block:

```rust
pub type CFStringEncoding = UInt32;
pub const kCFStringEncodingMacRoman: CFStringEncoding = 0;
pub const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
pub const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
pub const kCFStringEncodingUTF16: CFStringEncoding = 0x0100;
// ... 100+ more
```

Every string encoding Core Foundation supports. You'll mostly use `kCFStringEncodingUTF8` (value `0x08000100`).

### Opaque struct + ref aliases (lines 193–197)

```rust
#[repr(C)]
pub struct __CFString(c_void);

pub type CFStringRef = *const __CFString;
pub type CFMutableStringRef = *mut __CFString;
```

The same opaque-struct pattern from §"The FFI pattern" Section 5.

### `extern "C"` block (lines 209–546)

About 60 C functions for creating, comparing, searching, and mutating strings. Grouped by task:

- **Creating:** `CFStringCreateWithCString`, `CFStringCreateWithBytes`, `CFStringCreateWithCharacters`, `CFStringCreateWithFormat`, `CFStringCreateByCombiningStrings`, etc.
- **Searching:** `CFStringFind`, `CFStringFindWithOptions`, `CFStringFindCharacterFromSet`, etc.
- **Comparing:** `CFStringCompare`, `CFStringHasPrefix`, `CFStringHasSuffix`.
- **Accessing characters:** `CFStringGetBytes`, `CFStringGetCharacterAtIndex`, `CFStringGetCharacters`, `CFStringGetCString`, `CFStringGetCStringPtr`, `CFStringGetLength`.
- **Encoding conversions:** `CFStringConvertEncodingToNSStringEncoding`, `CFStringConvertEncodingToWindowsCodepage`, etc.
- **Numeric parsing:** `CFStringGetDoubleValue`, `CFStringGetIntValue`.
- **Filesystem:** `CFStringCreateWithFileSystemRepresentation`, `CFStringGetFileSystemRepresentation`.
- **Mutable string ops:** `CFStringAppend`, `CFStringDelete`, `CFStringInsert`, `CFStringReplace`, `CFStringTrim`, `CFStringUppercase`, etc.

You don't need to memorise these — when you need one, search the file for the function name pattern (`Create`, `Find`, `Get`, etc.) and read the signature.

---

## 2.4 `array.rs` — `CFArray` FFI

**Path:** `core-foundation-sys/src/array.rs` (143 lines)

A typical collection FFI file. Key elements:

- **Callback types** (lines 15–21): `CFArrayRetainCallBack`, `CFArrayReleaseCallBack`, `CFArrayCopyDescriptionCallBack`, `CFArrayEqualCallBack`, `CFArrayApplierFunction`.
- **Callback table** (lines 23–31): `CFArrayCallBacks` struct.
- **Opaque struct + refs** (lines 33–37): `__CFArray`, `CFArrayRef`, `CFMutableArrayRef`.
- **The global callback table** (line 44): `pub static kCFTypeArrayCallBacks: CFArrayCallBacks;` — a pre-built callback table for arrays of CFType objects (i.e. objects that participate in retain/release).
- **Functions**: `CFArrayCreate`, `CFArrayCreateCopy`, `CFArrayGetCount`, `CFArrayGetValueAtIndex`, `CFArrayGetValues`, `CFArrayBSearchValues`, `CFArrayContainsValue`, `CFArrayApplyFunction`, `CFArrayGetTypeID`, plus mutable-array operations (`CFArrayAppendValue`, `CFArrayInsertValueAtIndex`, `CFArrayRemoveValueAtIndex`, `CFArraySortValues`, etc.).

---

## 2.5 `dictionary.rs` — `CFDictionary` FFI

Same pattern as `array.rs` but for key-value maps. Has separate callback types for keys and values (`CFDictionaryKeyCallBacks`, `CFDictionaryValueCallBacks`), plus globals `kCFCopyStringDictionaryKeyCallBacks` (for dictionaries keyed by `CFString` that copy keys) and `kCFTypeDictionaryValueCallBacks` (for values that are CFType objects).

Functions: `CFDictionaryCreate`, `CFDictionaryCreateMutable`, `CFDictionaryGetCount`, `CFDictionaryGetValue`, `CFDictionarySetValue`, `CFDictionaryApplyFunction`, etc.

---

## 2.6 `set.rs` and `bag.rs` — collection variants

Two small files mirroring `dictionary.rs` but for sets (unique elements) and bags (duplicates allowed). Each has its own `CallBacks` struct and `kCFType*CallBacks` global.

- **`set.rs`** — `CFSetRef`, `CFSetCallBacks`, `kCFTypeSetCallBacks`. Functions: `CFSetCreate`, `CFSetGetCount`, `CFSetGetValue`, `CFSetApplyFunction`, etc.
- **`bag.rs`** — `CFBagRef`, `CFBagCallBacks`, `kCFTypeBagCallBacks`. Same function pattern.

These are rarely used in practice — `CFArray` and `CFDictionary` cover most needs.

---

## 2.7 `data.rs` — `CFData`

`CFData` is a raw byte buffer. FFI declares:

- `CFDataRef`, `CFMutableDataRef`.
- `CFDataCreate`, `CFDataCreateWithBytesNoCopy` (wraps an existing buffer without copying).
- `CFDataGetLength`, `CFDataGetBytePtr`, `CFDataGetMutableBytePtr`.
- Mutable ops: `CFDataAppendBytes`, `CFDataReplaceBytes`, `CFDataIncreaseLength`.

You'll use this in `core-foundation::data` (File 3) — the safe wrapper exposes `bytes()` and `length()`.

---

## 2.8 `number.rs` and `boolean.rs` — boxed scalars

**`number.rs`:** `CFNumberRef`, `CFNumberType` (enum of `kCFNumberSInt8Type`, `kCFNumberSInt16Type`, ..., `kCFNumberFloat32Type`, `kCFNumberFloat64Type`, `kCFNumberCharType` — 12 numeric types). Functions: `CFNumberCreate`, `CFNumberGetValue`, `CFNumberGetType`, `CFNumberCompare`, `CFNumberIsFloatType`, `CFNumberGetTypeID`.

**`boolean.rs`:** A tiny file. `CFBooleanRef`, plus the singleton globals `kCFBooleanTrue` and `kCFBooleanFalse`. Functions: `CFBooleanGetValue`, `CFBooleanGetTypeID`.

Why a separate type for booleans? Because Core Foundation collections are heterogeneous (they store `CFTypeRef`), so even booleans need to be wrapped in an object. Same idea as Java's `Boolean` vs `boolean`.

---

## 2.9 `date.rs` — timestamps

`CFDateRef`, `CFAbsoluteTime` (= `f64`, seconds since 2001-01-01). Functions: `CFDateCreate`, `CFDateGetAbsoluteTime`, `CFDateCompare`, `CFDateAddInterval`, `CFDateGetTimeIntervalSinceDate`, `CFDateGetTypeID`. Plus `CFTimeZone`-aware functions for getting the current absolute time.

---

## 2.10 `error.rs` — `CFError`

`CFErrorRef`. Functions: `CFErrorCreate`, `CFErrorGetDomain`, `CFErrorGetCode`, `CFErrorCopyUserInfo`, `CFErrorCopyDescription`, `CFErrorGetTypeID`. Plus domain globals: `kCFErrorDomainCocoa`, `kCFErrorDomainPOSIX`, `kCFErrorDomainOSStatus`, `kCFErrorDomainMach`.

This is the C-level analogue of `NSError`. You'll see it wrapped in `core-foundation::error::CFError` (File 3).

---

## 2.11 `url.rs` — URLs

`CFURLRef`. Functions for creating from file paths (`CFURLCreateWithFileSystemPath`, `CFURLCreateFromFileSystemRepresentation`) and from string (`CFURLCreateWithString`), plus accessors (`CFURLGetString`, `CFURLGetFileSystemRepresentation`, `CFURLGetBaseURL`). Plus `CFURLByteRange` struct for ranged URL operations.

---

## 2.12 `bundle.rs` — app bundles

`CFBundleRef`. Functions: `CFBundleCreate`, `CFBundleGetMainBundle`, `CFBundleGetIdentifier`, `CFBundleCopyInfoDictionaryForURL`, `CFBundleCopyResourceURL`, `CFBundleCopyLocalizedString`, etc. Used to find resources inside an `.app` or `.framework` package.

---

## 2.13 `runloop.rs` and `mach_port.rs` — event loops

**`runloop.rs`:** `CFRunLoopRef`, `CFRunLoopSourceRef`, `CFRunLoopObserverRef`, `CFRunLoopTimerRef`. Functions: `CFRunLoopGetCurrent`, `CFRunLoopGetMain`, `CFRunLoopRun`, `CFRunLoopStop`, `CFRunLoopAddSource`, `CFRunLoopRemoveSource`, `CFRunLoopAddTimer`, etc.

**`mach_port.rs`:** `CFMachPortRef` — a wrapper around a Mach port (the kernel IPC primitive on Apple platforms). Functions: `CFMachPortCreate`, `CFMachPortGetPort`, `CFMachPortInvalidate`.

Run loops are how Cocoa apps wait for events. The main run loop runs your app's event handler; a background thread can run its own run loop to wait for I/O.

---

## 2.14 `filedescriptor.rs` — file descriptor sources

`CFFileDescriptorRef` — a run-loop source that fires when a file descriptor becomes readable or writable. Functions: `CFFileDescriptorCreate`, `CFFileDescriptorGetNativeDescriptor`, `CFFileDescriptorEnableCallBacks`, `CFFileDescriptorDisableCallBacks`. Useful for integrating POSIX file I/O with a CFRunLoop.

---

## 2.15 `characterset.rs` and `bit_vector.rs` — small collections

**`characterset.rs`:** `CFCharacterSetRef` — a set of Unicode code points. Functions: `CFCharacterSetGetPredefined` (returns predefined sets like `kCFCharacterSetWhitespace`), `CFCharacterSetIsCharacterMember`, `CFCharacterSetCreateWithCharactersInRange`, etc.

**`bit_vector.rs`:** `CFBitVectorRef` — a packed bit array. Functions: `CFBitVectorCreate`, `CFBitVectorGetCount`, `CFBitVectorGetBitAtIndex`, `CFBitVectorSetBitAtIndex`.

---

## 2.16 `timezone.rs`, `calendar.rs`, `locale.rs` — date/time utilities

Three files for calendar-aware date operations:

- **`timezone.rs`:** `CFTimeZoneRef`, `CFTimeZoneCreate`, `CFTimeZoneGetName`, `CFTimeZoneGetSecondsFromGMT`, etc.
- **`calendar.rs`:** `CFCalendarRef`, `CFCalendarCreateWithIdentifier`, `CFCalendarComposeAbsoluteTime`, etc.
- **`locale.rs`:** `CFLocaleRef`, `CFLocaleCreate`, `CFLocaleGetIdentifier`, `CFLocaleCopyCurrent`, etc.

---

## 2.17 `propertylist.rs` and `preferences.rs` — plist + user defaults

**`propertylist.rs`:** Defines `CFPropertyListRef` (= `CFTypeRef` — any plist value is a CFType) and the `CFPropertyListMutabilityOptions` enum. Functions: `CFPropertyListCreateFromXMLData` (deprecated), `CFPropertyListCreateWithData`, `CFPropertyListWriteData`, `CFPropertyListCreateDeepCopy`, `CFPropertyListIsValid`.

Property lists are Apple's structured data format — XML or binary, supporting arrays, dictionaries, strings, numbers, dates, data, booleans. Used for `.plist` files everywhere on macOS/iOS.

**`preferences.rs`:** Wrappers around `CFPreferencesCopyAppValue`, `CFPreferencesSetAppValue`, etc. — the user defaults system (analogous to `NSUserDefaults` in Cocoa).

---

## 2.18 `notification_center.rs`, `messageport.rs`, `socket.rs`, `stream.rs` — IPC

Four IPC primitives:

- **`notification_center.rs`:** `CFNotificationCenterRef`. Observer pattern across processes — `CFNotificationCenterPostNotification`, `CFNotificationCenterAddObserver`.
- **`messageport.rs`:** `CFMessagePortRef`. Mach-port-based bidirectional messaging.
- **`socket.rs`:** `CFSocketRef`. Wraps a BSD socket as a run-loop source.
- **`stream.rs`:** `CFReadStreamRef`, `CFWriteStreamRef`. Async streaming I/O.

You'll rarely use these directly in a Cocoa app — higher-level APIs (`NSURLSession`, etc.) are usually better.

---

## 2.19 `attributed_string.rs`, `string_tokenizer.rs` — string utilities

- **`attributed_string.rs`:** `CFAttributedStringRef`. A `CFString` plus a dictionary of attributes per character range. Used by Core Text (Phase 5).
- **`string_tokenizer.rs`:** `CFStringTokenizerRef`. Iterates over the words, sentences, lines, or grapheme clusters in a `CFString`. Locale-aware.

---

## 2.20 `tree.rs`, `binary_heap.rs` — less-common collections

- **`tree.rs`:** `CFTreeRef`. A tree of `CFTypeRef` values with parent/sibling pointers. Rarely used.
- **`binary_heap.rs`:** `CFBinaryHeapRef`. A min-heap of `CFTypeRef` values ordered by a comparator callback. Also rarely used.

---

## 2.21 `xml_node.rs`, `xml_parser.rs` — XML (macOS only)

Both files are `#[cfg(target_os = "macos")]` — iOS doesn't expose these. They wrap `CFXMLNodeRef` and `CFXMLParserRef` for parsing XML. Deprecated by Apple in favour of Foundation's `NSXMLParser`.

---

## 2.22 `date_formatter.rs`, `number_formatter.rs` — formatters

- **`date_formatter.rs`:** `CFDateFormatterRef`. Format `CFDate` into localised strings.
- **`number_formatter.rs`:** `CFNumberFormatterRef`. Format `CFNumber` into localised strings (currency, percentages, etc.).

---

## 2.23 `plugin.rs`, `user_notification.rs`, `url_enumerator.rs`, `file_security.rs` — remaining

A grab-bag of less-common modules:

- **`plugin.rs`:** `CFPluginRef`. The old-style CF plug-in system (mostly superseded by `NSBundle`).
- **`user_notification.rs`** (macOS only): `CFUserNotificationRef`. Modal system dialogs (rarely used outside system software).
- **`url_enumerator.rs`:** `CFURLEnumeratorRef`. Iterate over the contents of a URL (e.g. directory listing on a file URL).
- **`file_security.rs`:** `CFFileSecurityRef`. Wraps a `struct stat` and security attributes for a file.

---

## 2.24 `uuid.rs` — UUIDs

`CFUUIDRef`. Functions: `CFUUIDCreate`, `CFUUIDCreateFromString`, `CFUUIDCreateString`, `CFUUIDGetConstantUUIDWithBytes`. Used for creating and parsing UUIDs in CF-style code.

---

## Summary: what you've learned

After Phase 2, you should be able to:

1. **Read any `extern "C"` declaration** in the crate and know what C function it binds.
2. **Recognise the opaque struct + ref alias pattern**: `pub struct __CFXxx(c_void); pub type CFXxxRef = *const __CFXxx;`.
3. **Understand `#[repr(C)]`** — guarantees C-compatible struct layout.
4. **Understand `pub static`** — a C global variable.
5. **Understand `pub const`** — a compile-time constant.
6. **Understand `extern "C" fn(...)`** — a C function pointer type.
7. **Understand `Option<extern "C" fn(...)>`** — a nullable function pointer.
8. **Know why each function requires `unsafe`** to call — the Rust compiler can't verify C side contracts.
9. **Know which crate to look in** for any given C function — `CFString*` → `string.rs`, `CFArray*` → `array.rs`, etc.
10. **Recognise the `TCFTypeRef` trait** as the bridge between typed pointers and the generic `CFTypeRef`.

**Open File 3 when ready** to start the safe-wrapper crate. That's where the real Rust idioms live.

— End of File 2 —
