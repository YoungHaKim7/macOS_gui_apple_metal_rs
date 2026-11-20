# File 7 — 100 Progressive Rust Code Examples

**File 7 of 8 — 100 short, progressive code examples derived from the `core-foundation-rs` codebase.**

> **Purpose:** Concrete examples that take you from Rust basics to advanced FFI patterns. Each example illustrates ONE concept, in 5–30 lines. The examples are numbered and progress in difficulty: 1–10 teach Rust fundamentals, 11–20 teach FFI basics, 21–35 teach `core-foundation-sys` patterns, 36–55 teach the safe wrappers, 56–75 teach advanced patterns (lifetimes, traits, generics), 76–95 teach graphics/text/cocoa integration, 96–100 put it all together.
>
> **How to use this file:**
> - Read each example in order.
> - Try to predict what it does before reading the explanation.
> - For examples that compile and run, try them — `cargo new example_X && cd example_X && cargo run` (you'll need to add `core-foundation` as a dependency in `Cargo.toml`).
> - If you don't understand one, go back to the matching section in Files 1–6.

Each example has:

- **The code** (sometimes simplified from real codebase usage).
- **What it teaches** — the one concept.
- **Explanation** — what's happening line by line.
- **Try it** — how to make it concrete (when applicable).

---

## Part 1: Rust Fundamentals (Examples 1–10)

### Example 1 — Variable declaration and mutability

```rust
fn main() {
    let x = 5;          // immutable
    let mut y = 10;     // mutable
    // x = 6;           // ERROR: cannot assign to immutable variable
    y = 20;             // OK
    println!("x = {}, y = {}", x, y);
}
```

**Teaches:** `let` vs `let mut`. Rust variables are immutable by default.

**C++ comparison:**
```cpp
const int x = 5;
int y = 10;
y = 20;
```

### Example 2 — Type inference and explicit types

```rust
fn main() {
    let a = 42;          // inferred as i32
    let b: i64 = 100;    // explicit
    let c = b as i32;    // explicit cast
    let d: usize = 0;
    println!("{} {} {} {}", a, b, c, d);
}
```

**Teaches:** Type inference, explicit annotations, casting with `as`.

### Example 3 — String types

```rust
fn main() {
    let s1: &str = "hello";                  // borrowed string slice
    let s2: String = String::from("world");  // owned heap string
    let s3 = "literal".to_string();          // another owned String
    println!("{} {} {}", s1, s2, s3);
}
```

**Teaches:** `&str` (borrowed) vs `String` (owned). Equivalent to `std::string_view` vs `std::string` in C++.

### Example 4 — Structs and methods

```rust
struct Point { x: f64, y: f64 }

impl Point {
    fn new(x: f64, y: f64) -> Point { Point { x, y } }
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Distance: {}", p1.distance_to(&p2));  // 5.0
}
```

**Teaches:** Struct definition, `impl` block, `&self` methods, associated functions.

**C++ comparison:**
```cpp
struct Point {
    double x, y;
    Point(double x, double y) : x(x), y(y) {}
    double distance_to(const Point& other) const { /* ... */ }
};
```

### Example 5 — Enums and pattern matching

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Empty,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Empty => 0.0,
    }
}

fn main() {
    let s = Shape::Rectangle { width: 3.0, height: 4.0 };
    println!("Area: {}", area(&s));
}
```

**Teaches:** Enums with data, pattern matching, destructuring.

### Example 6 — Option<T> (replacing null)

```rust
fn first(v: &[i32]) -> Option<&i32> {
    if v.is_empty() { None } else { Some(&v[0]) }
}

fn main() {
    let v = vec![10, 20, 30];
    match first(&v) {
        Some(x) => println!("First: {}", x),
        None => println!("Empty!"),
    }

    // Or use if-let
    if let Some(x) = first(&v) {
        println!("Got: {}", x);
    }
}
```

**Teaches:** `Option<T>` as Rust's replacement for null. The compiler forces you to handle `None`.

**C++ comparison:**
```cpp
std::optional<int> first(const std::vector<int>& v) {
    return v.empty() ? std::nullopt : std::optional(v.front());
}
```

### Example 7 — Result<T, E> and the ? operator

```rust
use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;   // ? propagates errors
    Ok(n * 2)
}

fn main() {
    match parse_and_double("21") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Teaches:** `Result<T, E>` for fallible operations, `?` operator for error propagation.

### Example 8 — Traits and generics

```rust
trait Summary {
    fn summary(&self) -> String;
}

struct Article { title: String }
struct Tweet { content: String }

impl Summary for Article {
    fn summary(&self) -> String { format!("Article: {}", self.title) }
}

impl Summary for Tweet {
    fn summary(&self) -> String { format!("Tweet: {}", self.content) }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summary());
}

fn main() {
    let a = Article { title: "Hello".into() };
    let t = Tweet { content: "Hi!".into() };
    print_summary(&a);
    print_summary(&t);
}
```

**Teaches:** Traits (analogous to C++ concepts/interfaces), generic functions with trait bounds, static dispatch via monomorphisation.

### Example 9 — Ownership and moves

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;           // s1 is MOVED to s2
    // println!("{}", s1); // ERROR: value borrowed here after move
    println!("{}", s2);

    // But Copy types are bitwise-copied:
    let a = 5;
    let b = a;
    println!("{} {}", a, b);  // both still valid
}
```

**Teaches:** Ownership transfer for non-Copy types, bitwise copy for Copy types.

### Example 10 — Borrowing

```rust
fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let a = String::from("long string");
    let b = String::from("short");
    let result = longest(&a, &b);
    println!("Longer: {}", result);
}
```

**Teaches:** Borrowing, lifetimes elided, references without ownership transfer.

---

## Part 2: FFI Basics (Examples 11–20)

### Example 11 — Calling a C function

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    let x = -42;
    let abs_x = unsafe { abs(x) };
    println!("|{}| = {}", x, abs_x);
}
```

**Teaches:** `extern "C"` block declaring a C function, `unsafe` block to call it.

### Example 12 — Linking a framework

```rust
#[cfg_attr(target_os = "macos", link(name = "CoreFoundation", kind = "framework"))]
extern "C" {}

fn main() {
    // Just linking the framework; no calls yet.
    println!("Linked CoreFoundation.");
}
```

**Teaches:** `#[link(name = "...", kind = "framework")]` for linking Apple frameworks.

### Example 13 — Type aliases for C types

```rust
use core::ffi::{c_void, c_int};

type CFTypeRef = *const c_void;   // any CF object
type CFIndex = isize;             // signed count

fn main() {
    let _: CFTypeRef = std::ptr::null();
    let _: CFIndex = 0;
    println!("Type aliases defined.");
}
```

**Teaches:** `pub type X = Y` for aliasing C types in Rust.

### Example 14 — An opaque struct + ref alias

```rust
#[repr(C)]
pub struct __CFString(core::ffi::c_void);

pub type CFStringRef = *const __CFString;

fn main() {
    let null_ref: CFStringRef = std::ptr::null();
    println!("null_ref is null: {}", null_ref.is_null());
}
```

**Teaches:** The opaque-struct + ref-alias pattern. The empty struct gives the ref type distinctness.

### Example 15 — A real FFI call: CFRelease

```rust
use core_foundation_sys::base::{CFRelease, CFTypeRef};

extern "C" {
    fn CFRelease(cf: CFTypeRef);
}

fn main() {
    let null_ptr: CFTypeRef = std::ptr::null();
    // Calling CFRelease(NULL) is safe in CF (it's a no-op).
    unsafe { CFRelease(null_ptr); }
    println!("Called CFRelease(null).");
}
```

**Teaches:** Calling a real CF function via FFI.

### Example 16 — `#[repr(C)]` for struct layout

```rust
#[repr(C)]
struct CFRange {
    location: isize,
    length: isize,
}

extern "C" {
    fn some_c_function_taking_range(r: CFRange) -> i32;
}

fn main() {
    let r = CFRange { location: 0, length: 10 };
    let _ = unsafe { some_c_function_taking_range(r) };
}
```

**Teaches:** `#[repr(C)]` guarantees C-compatible struct layout. Without it, Rust could reorder fields.

### Example 17 — Function pointer types

```rust
type Comparator = extern "C" fn(
    val1: *const core::ffi::c_void,
    val2: *const core::ffi::c_void,
    context: *mut core::ffi::c_void,
) -> i32;

extern "C" fn compare_ints(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    let a = unsafe { *(a as *const i32) };
    let b = unsafe { *(b as *const i32) };
    (a - b).signum()
}

fn main() {
    let cmp: Comparator = compare_ints;
    println!("Function pointer assigned.");
}
```

**Teaches:** `extern "C" fn(...)` as a function pointer type, assigning a real function to it.

### Example 18 — Nullable function pointers via Option

```rust
type Callback = Option<extern "C" fn(i32) -> i32>;

fn main() {
    let none: Callback = None;
    let some: Callback = Some(|x: i32| x * 2);   // NOTE: closures that don't capture can be cast to fn pointers
    let some: Callback = Some(extern "C" fn(x: i32) -> i32 { x * 2 });

    match none {
        Some(f) => println!("{}", f(5)),
        None => println!("no callback"),
    }
    if let Some(f) = some {
        println!("{}", f(5));
    }
}
```

**Teaches:** `Option<fn>` is Rust's nullable function pointer — `None` = NULL, `Some(f)` = real function.

### Example 19 — C strings: CString and CStr

```rust
use std::ffi::{CString, CStr};

extern "C" {
    fn puts(s: *const i8);
}

fn main() {
    let rust_string = String::from("Hello from Rust!");
    let c_string = CString::new(rust_string).unwrap();
    unsafe { puts(c_string.as_ptr()); }

    // Convert back
    let bytes = b"Hi from C!\0";
    let cstr = unsafe { CStr::from_ptr(bytes.as_ptr() as *const i8) };
    let rust_str = cstr.to_string_lossy();
    println!("{}", rust_str);
}
```

**Teaches:** `CString` (owned, null-terminated), `CStr` (borrowed, null-terminated).

### Example 20 — Static C globals

```rust
extern "C" {
    static kCFAllocatorDefault: *const core::ffi::c_void;
}

fn main() {
    let allocator = unsafe { kCFAllocatorDefault };
    println!("kCFAllocatorDefault pointer: {:p}", allocator);
}
```

**Teaches:** `static` declarations of C globals. Reading them requires `unsafe`.

---

## Part 3: `core-foundation-sys` Patterns (Examples 21–35)

### Example 21 — Re-exporting FFI declarations

```rust
// In real code: pub use core_foundation_sys::base::*;
// Simulated:
pub mod ffi {
    pub type CFTypeRef = *const core::ffi::c_void;
    pub type CFIndex = isize;

    extern "C" {
        pub fn CFRelease(cf: CFTypeRef);
        pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    }
}

pub use ffi::*;

fn main() {
    println!("FFI re-exported.");
}
```

**Teaches:** The `pub use module::*` pattern that `core-foundation` uses to re-export FFI declarations from `core-foundation-sys`.

### Example 22 — Constants and enums

```rust
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFComparisonResult {
    LessThan = -1,
    EqualTo = 0,
    GreaterThan = 1,
}

pub const kCFNotFound: isize = -1;

fn main() {
    let r = CFComparisonResult::LessThan;
    println!("{:?} (value {})", r, r as isize);
    println!("NotFound = {}", kCFNotFound);
}
```

**Teaches:** `#[repr(isize)]` enum with explicit values, `pub const` constants.

### Example 23 — A full callback table

```rust
use core::ffi::c_void;

type RetainCb = extern "C" fn(allocator: *const c_void, value: *const c_void) -> *const c_void;
type ReleaseCb = extern "C" fn(allocator: *const c_void, value: *const c_void);
type EqualCb = extern "C" fn(v1: *const c_void, v2: *const c_void) -> u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFArrayCallBacks {
    pub version: isize,
    pub retain: RetainCb,
    pub release: ReleaseCb,
    pub equal: EqualCb,
}

fn main() {
    let _cb = CFArrayCallBacks {
        version: 0,
        retain: |_, v| v,
        release: |_, _| {},
        equal: |a, b| a == b,
    };
    println!("Callback table built.");
}
```

**Teaches:** Building a CF callbacks struct in Rust. Note closures assigned to function pointer slots (works only for non-capturing closures).

### Example 24 — The TCFTypeRef trait

```rust
use core::ffi::c_void;

pub trait TCFTypeRef {
    fn as_void_ptr(&self) -> *const c_void;
    unsafe fn from_void_ptr(ptr: *const c_void) -> Self;
}

impl<T> TCFTypeRef for *const T {
    fn as_void_ptr(&self) -> *const c_void { (*self) as *const c_void }
    unsafe fn from_void_ptr(ptr: *const c_void) -> Self { ptr as *const T }
}

fn main() {
    let x: *const i32 = &42;
    let void_ptr = x.as_void_ptr();
    let back: *const i32 = unsafe { TCFTypeRef::from_void_ptr(void_ptr) };
    println!("Round-trip: {}", unsafe { *back });
}
```

**Teaches:** The `TCFTypeRef` trait pattern from `core-foundation-sys/src/base.rs`. A blanket impl provides the conversion for all raw pointers.

### Example 25 — Listing what's in `core-foundation-sys`

```rust
// This example is illustrative; you'd need core-foundation-sys as a dependency.
// cargo add core-foundation-sys

use core_foundation_sys::base::{CFTypeRef, CFIndex, CFRelease};
use core_foundation_sys::string::{CFStringRef, CFStringGetLength};

fn main() {
    let null_str: CFStringRef = std::ptr::null();
    // DON'T actually call CFStringGetLength(null_str) — UB.
    let _ = null_str;
    println!("Types imported.");
}
```

**Teaches:** How to import specific FFI declarations from the crate.

### Example 26 — Calling CFRetain / CFRelease

```rust
use core_foundation_sys::base::{CFRetain, CFRelease, CFTypeRef, CFGetRetainCount};

fn main() {
    let null_ptr: CFTypeRef = std::ptr::null();
    // SAFETY: CFRelease(NULL) is a documented no-op in CF.
    unsafe {
        // CFRetain(NULL) returns NULL.
        let _ = CFRetain(null_ptr);
        CFRelease(null_ptr);
        let count = CFGetRetainCount(null_ptr);
        println!("Retain count of NULL: {}", count);
    }
}
```

**Teaches:** Using the retain/release functions on a (null) pointer.

### Example 27 — Encoding constants

```rust
use core_foundation_sys::string::kCFStringEncodingUTF8;

fn main() {
    println!("UTF-8 encoding value: 0x{:08x}", kCFStringEncodingUTF8);
}
```

**Teaches:** Using the encoding constants from `core-foundation-sys::string`.

### Example 28 — Compare flags

```rust
use core_foundation_sys::string::{
    kCFCompareCaseInsensitive, kCFCompareBackwards, kCFCompareAnchored,
};

fn main() {
    let flags = kCFCompareCaseInsensitive | kCFCompareBackwards;
    println!("Flags: 0x{:x}", flags);
    let with_anchored = flags | kCFCompareAnchored;
    println!("With anchored: 0x{:x}", with_anchored);
}
```

**Teaches:** Bitmask operations on `_NS_OPTIONS`-style constants.

### Example 29 — CFRange struct

```rust
use core_foundation_sys::base::CFRange;

fn main() {
    let r = CFRange { location: 5, length: 10 };
    println!("Range: location={}, length={}", r.location, r.length);
    println!("End: {}", r.location + r.length);
}
```

**Teaches:** Constructing and using a `#[repr(C)]` CFRange.

### Example 30 — `#[cfg(target_os = "macos")]`

```rust
#[cfg(target_os = "macos")]
fn platform_message() -> &'static str { "Running on macOS" }

#[cfg(target_os = "ios")]
fn platform_message() -> &'static str { "Running on iOS" }

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn platform_message() -> &'static str { "Running on something else" }

fn main() {
    println!("{}", platform_message());
}
```

**Teaches:** Conditional compilation by target OS — used by the sys crate to gate macOS-only modules.

### Example 31 — Linking vs not linking

```rust
// In Cargo.toml: [features] default = ["link"]  link = []
//
// In code:
#[cfg_attr(feature = "link", link(name = "CoreFoundation", kind = "framework"))]
extern "C" {}

fn main() {
    println!("If 'link' feature is on, CoreFoundation is linked.");
}
```

**Teaches:** Feature-gated linking — lets users opt out of automatic linking if they're managing it themselves.

### Example 32 — A complete FFI binding file (mini version)

```rust
// Simplified version of core-foundation-sys/src/data.rs

use core::ffi::c_void;
use crate::base::{CFAllocatorRef, CFIndex, CFTypeID};

#[repr(C)]
pub struct __CFData(c_void);

pub type CFDataRef = *const __CFData;
pub type CFMutableDataRef = *mut __CFData;

extern "C" {
    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    pub fn CFDataGetTypeID() -> CFTypeID;
}

fn main() {
    println!("Binding declared.");
}
```

**Teaches:** The complete pattern of a `core-foundation-sys` file: opaque struct + ref alias + extern block.

### Example 33 — Boolean type

```rust
use core_foundation_sys::base::Boolean;  // type alias for u8

fn main() {
    let b: Boolean = 1;
    if b != 0 {
        println!("truthy");
    }
}
```

**Teaches:** CF's `Boolean` is just `u8`. C++ has `bool`; CF predates wide `bool` adoption.

### Example 34 — Function-pointer typedef

```rust
type CFComparatorFunction = extern "C" fn(
    val1: *const core::ffi::c_void,
    val2: *const core::ffi::c_void,
    context: *mut core::ffi::c_void,
) -> core_foundation_sys::base::CFComparisonResult;

fn main() {
    let _type_name = std::any::type_name::<CFComparatorFunction>();
    println!("Type: {}", _type_name);
}
```

**Teaches:** Defining a typedef for a complex C function pointer type.

### Example 35 — The `kCFNotFound` sentinel

```rust
use core_foundation_sys::base::{kCFNotFound, CFIndex};

fn find_in(vec: &[i32], target: i32) -> CFIndex {
    for (i, v) in vec.iter().enumerate() {
        if *v == target { return i as CFIndex; }
    }
    kCFNotFound
}

fn main() {
    let v = vec![10, 20, 30];
    let i = find_in(&v, 20);
    if i == kCFNotFound {
        println!("Not found");
    } else {
        println!("Found at {}", i);
    }
}
```

**Teaches:** Using `kCFNotFound` as the "not found" sentinel — same pattern CF uses internally.

---

## Part 4: Safe Wrappers (Examples 36–55)

### Example 36 — The declare_TCFType! macro

```rust
// From core-foundation/src/lib.rs (simplified)
macro_rules! declare_TCFType {
    ($ty:ident, $raw:ident) => {
        pub struct $ty($raw);
        impl Drop for $ty {
            fn drop(&mut self) {
                // unsafe { CFRelease(self.as_CFTypeRef()) }
                println!("Dropping {}", std::any::type_name::<$ty>());
            }
        }
    };
}

declare_TCFType!(MyType, usize);

fn main() {
    let _x = MyType(42);
    // When _x goes out of scope, "Dropping ..." prints.
}
```

**Teaches:** A simplified version of the `declare_TCFType!` macro. Run it and observe the Drop message.

### Example 37 — The TCFType trait

```rust
// Simplified TCFType trait
trait TCFType: Sized {
    type Ref;
    fn as_concrete_TypeRef(&self) -> Self::Ref;
    unsafe fn wrap_under_create_rule(obj: Self::Ref) -> Self;
    unsafe fn wrap_under_get_rule(obj: Self::Ref) -> Self;
    fn type_id() -> usize;
}

struct MyType(usize);

impl TCFType for MyType {
    type Ref = usize;
    fn as_concrete_TypeRef(&self) -> usize { self.0 }
    unsafe fn wrap_under_create_rule(obj: usize) -> Self { MyType(obj) }
    unsafe fn wrap_under_get_rule(obj: usize) -> Self { MyType(obj) }   // simplified
    fn type_id() -> usize { 42 }
}

fn main() {
    let t = unsafe { MyType::wrap_under_create_rule(100) };
    println!("Ref: {}, type_id: {}", t.as_concrete_TypeRef(), MyType::type_id());
}
```

**Teaches:** The shape of the `TCFType` trait and a simple implementation.

### Example 38 — Create vs Get rule

```rust
// Illustrates the difference between wrap_under_create_rule and wrap_under_get_rule
// (in real code, get_rule bumps the refcount via CFRetain)

trait Wrapper {
    unsafe fn wrap_create(n: i32) -> Self;
    unsafe fn wrap_get(n: i32) -> Self;
}

struct Counter { value: i32 }

impl Wrapper for Counter {
    unsafe fn wrap_create(n: i32) -> Self { Counter { value: n } }
    unsafe fn wrap_get(n: i32) -> Self { Counter { value: n + 1 } }   // simulate retain
}

fn main() {
    let c1 = unsafe { Counter::wrap_create(5) };  // Create rule: refcount is 1
    let c2 = unsafe { Counter::wrap_get(5) };     // Get rule: refcount bumped to 2
    println!("c1={}, c2={}", c1.value, c2.value);
}
```

**Teaches:** The Create Rule (refcount stays at 1) vs Get Rule (refcount bumped). Same concept as metal-cpp's TransferPtr vs RetainPtr.

### Example 39 — Creating a CFString

```rust
// cargo add core-foundation
use core_foundation::string::CFString;

fn main() {
    let s = CFString::new("Hello, Core Foundation!");
    println!("Length: {}", s.char_len());
    println!("Display: {}", s);
}
```

**Teaches:** Creating a `CFString` from a Rust `&str`. The safe `new` method wraps the unsafe `CFStringCreateWithBytes`.

### Example 40 — From<&str> for CFString

```rust
use core_foundation::string::CFString;

fn main() {
    // Using From impl
    let s1: CFString = "hello".into();
    // Using parse / FromStr
    let s2: CFString = "world".parse().unwrap();
    println!("{} {}", s1, s2);
}
```

**Teaches:** The `From<&str>` and `FromStr` impls make `CFString` feel native.

### Example 41 — CFString from a static literal

```rust
use core_foundation::string::CFString;

fn main() {
    let s = CFString::from_static_string("I live in the binary's read-only data.");
    println!("{}", s);
    // No allocation! The CFString points directly at the string literal's bytes.
}
```

**Teaches:** `from_static_string` uses `CFStringCreateWithBytesNoCopy` to avoid allocation — same idea as metal-cpp's `MTLSTR("...")`.

### Example 42 — CFString equality with &str

```rust
use core_foundation::string::CFString;

fn main() {
    let cfstr = CFString::new("hello");
    assert_eq!(cfstr, "hello");        // PartialEq<&str> for CFString
    assert_eq!("hello", cfstr);        // PartialEq<CFString> for &str
    assert_ne!(cfstr, "world");
}
```

**Teaches:** Cross-type `PartialEq` impls — both directions needed for `==` to work both ways.

### Example 43 — CFArray from CFType elements

```rust
use core_foundation::base::{CFType, TCFType};
use core_foundation::array::CFArray;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;

fn main() {
    let n1 = CFNumber::from(1);
    let n2 = CFNumber::from(2);
    let n3 = CFNumber::from(3);

    let arr: CFArray<CFType> = CFArray::from_CFTypes(&[
        n1.as_CFType(), n2.as_CFType(), n3.as_CFType(),
    ]);

    println!("Array length: {}", arr.len());
    for elem in arr.iter() {
        if let Some(num) = elem.downcast::<CFNumber>() {
            println!("  Number: {:?}", num);
        }
    }
}
```

**Teaches:** Building a heterogeneous `CFArray<CFType>` from elements, then downcasting each.

### Example 44 — CFArray of CFString

```rust
use core_foundation::array::CFArray;
use core_foundation::string::CFString;

fn main() {
    let arr: CFArray<CFString> = CFArray::from_CFTypes(&[
        CFString::from_static_string("alpha"),
        CFString::from_static_string("beta"),
        CFString::from_static_string("gamma"),
    ]);

    for s in &arr {
        println!("{}", s.to_string());
    }
}
```

**Teaches:** A typed `CFArray<CFString>` — generic over the element type.

### Example 45 — CFDictionary

```rust
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;

fn main() {
    let key1 = CFString::from_static_string("one");
    let key2 = CFString::from_static_string("two");
    let val1 = CFNumber::from(1);
    let val2 = CFNumber::from(2);

    let dict = CFDictionary::from_CFType_pairs(&[
        (key1.as_CFType(), val1.as_CFType()),
        (key2.as_CFType(), val2.as_CFType()),
    ]);

    println!("Dict len: {}", dict.len());

    let lookup = CFString::from_static_string("one");
    if let Some(val) = dict.find(&lookup) {
        if let Some(num) = val.clone().downcast::<CFNumber>() {
            println!("one = {:?}", num);
        }
    }
}
```

**Teaches:** Creating a `CFDictionary` from key/value pairs and looking up a value.

### Example 46 — CFMutableDictionary

```rust
use core_foundation::dictionary::CFMutableDictionary;
use core_foundation::string::CFString;
use core_foundation::number::CFNumber;

fn main() {
    let mut dict: CFMutableDictionary<CFString, CFString> = CFMutableDictionary::new();
    dict.add(&CFString::from_static_string("name"), &CFString::from_static_string("Alice"));
    dict.add(&CFString::from_static_string("city"), &CFString::from_static_string("Seoul"));
    println!("After adding: len={}", dict.len());

    dict.remove(CFString::from_static_string("city"));
    println!("After remove: len={}", dict.len());
}
```

**Teaches:** The mutable variant of `CFDictionary`.

### Example 47 — CFNumber conversions

```rust
use core_foundation::number::CFNumber;

fn main() {
    let n_i32 = CFNumber::from(42_i32);
    let n_f64 = CFNumber::from(3.14159_f64);

    println!("i32 -> {:?}", n_i32.to_i32());
    println!("i32 -> f64? {:?}", n_i32.to_f64());   // Some (lossless)
    println!("f64 -> {:?}", n_f64.to_f64());
    println!("f64 -> i32? {:?}", n_f64.to_i32());   // Some (truncates)
}
```

**Teaches:** The four `From` impls (i32, i64, f32, f64) and the four `to_*` methods returning `Option`.

### Example 48 — CFBoolean

```rust
use core_foundation::boolean::CFBoolean;

fn main() {
    let t = CFBoolean::true_value();
    let f = CFBoolean::false_value();
    let from_bool: CFBoolean = true.into();

    println!("t = {:?}, f = {:?}", t, f);
    println!("from_bool = {:?}", from_bool);
}
```

**Teaches:** `CFBoolean::true_value()` and `false_value()` (return singletons), plus `From<bool>`.

### Example 49 — CFData

```rust
use core_foundation::data::CFData;
use std::ops::Deref;

fn main() {
    let bytes = b"hello, world";
    let data = CFData::from_byte_buffer(bytes);  // or similar — API varies
    println!("Length: {}", data.len());

    // Because CFData: Deref<Target=[u8]>, you can:
    println!("First byte: {}", data[0]);
    println!("All bytes: {:?}", &data[..]);
}
```

**Teaches:** `CFData` as a byte buffer with `Deref<Target = [u8]>` — feels just like a `&[u8]`.

### Example 50 — CFError and the `?` operator

```rust
use core_foundation::error::CFError;
use core_foundation::string::CFString;

fn do_something(fail: bool) -> Result<i32, CFError> {
    if fail {
        let domain = CFString::from_static_string("com.example.myapp");
        let err = CFError::new(domain, 42, None);
        return Err(err);
    }
    Ok(42)
}

fn main() {
    match do_something(false) {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    match do_something(true) {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Teaches:** Using `CFError` as a Rust `Result` error type.

### Example 51 — CFDate

```rust
use core_foundation::date::{CFDate, CFAbsoluteTime};

fn main() {
    let now: CFAbsoluteTime = unsafe { core_foundation_sys::date::CFAbsoluteTimeGetCurrent() };
    let date = CFDate::new(now);
    println!("Now: {} seconds since 2001-01-01", date.abs_time());
}
```

**Teaches:** `CFDate` for timestamps.

### Example 52 — CFBundle

```rust
use core_foundation::bundle::CFBundle;

fn main() {
    let bundle = CFBundle::main_bundle();
    if let Some(id) = bundle.bundle_identifier() {
        println!("Bundle ID: {}", id);
    } else {
        println!("No bundle ID (probably running outside an .app).");
    }
}
```

**Teaches:** Getting the main bundle's identifier.

### Example 53 — CFURL

```rust
use core_foundation::string::CFString;
use core_foundation::url::CFURL;
use core_foundation::base::TCFType;

fn main() {
    let path = CFString::from_static_string("/tmp/example.txt");
    let url = CFURL::new_with_file_system_path(&path, core_foundation_sys::url::kCFURLPOSIXPathStyle, false);
    if let Some(u) = url {
        println!("URL string: {}", u.get_string());
    }
}
```

**Teaches:** Creating a file URL from a path.

### Example 54 — CFPropertyList

```rust
use core_foundation::base::TCFType;
use core_foundation::propertylist::CFPropertyList;
use core_foundation::string::CFString;
use core_foundation::number::CFNumber;

fn main() {
    let s = CFString::from_static_string("hello");
    let plist = CFPropertyList::from_CFType(&s.as_CFType()).unwrap();
    match plist {
        CFPropertyList::CFString(s) => println!("Got string: {}", s),
        _ => println!("Got something else"),
    }

    let n = CFNumber::from(42);
    let plist = CFPropertyList::from_CFType(&n.as_CFType()).unwrap();
    match plist {
        CFPropertyList::CFNumber(n) => println!("Got number: {:?}", n),
        _ => println!("Got something else"),
    }
}
```

**Teaches:** The `CFPropertyList` enum — pattern-match on the variant to extract the typed value.

### Example 55 — into_CFType and downcast_into

```rust
use core_foundation::base::{CFType, TCFType};
use core_foundation::string::CFString;

fn main() {
    let s = CFString::from_static_string("upcast me");
    println!("Retain count before: {}", s.retain_count());

    let cftype: CFType = s.into_CFType();   // consumes s, no retain bump
    println!("Retain count after upcast: {}", cftype.retain_count());

    let s2: CFString = cftype.downcast_into().unwrap();
    println!("Got back: {}", s2);
    println!("Retain count after downcast: {}", s2.retain_count());
}
```

**Teaches:** The `into_CFType` (consumes, no refcount change) and `downcast_into` (consumes, no refcount change) pattern — efficient upcast/downcast.

---

## Part 5: Advanced Patterns (Examples 56–75)

### Example 56 — Lifetimes in iterators

```rust
struct IntArray<'a> { data: &'a [i32] }

struct IntIter<'a> { data: &'a [i32], idx: usize }

impl<'a> Iterator for IntIter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<&'a i32> {
        if self.idx >= self.data.len() { None }
        else {
            let item = &self.data[self.idx];
            self.idx += 1;
            Some(item)
        }
    }
}

impl<'a> IntArray<'a> {
    fn iter(&self) -> IntIter<'a> {
        IntIter { data: self.data, idx: 0 }
    }
}

fn main() {
    let v = vec![10, 20, 30];
    let arr = IntArray { data: &v };
    for x in arr.iter() {
        println!("{}", x);
    }
}
```

**Teaches:** Lifetimes in iterators — the `'a` ties the iterator and its items to the container's borrow. Same pattern as `CFArrayIterator`.

### Example 57 — ItemRef with ManuallyDrop

```rust
use std::mem::ManuallyDrop;
use std::marker::PhantomData;
use std::ops::Deref;

struct ItemRef<'a, T: 'a>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<T> Deref for ItemRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn main() {
    let s = String::from("borrowed");
    let item: ItemRef<String> = ItemRef(ManuallyDrop::new(s.clone()), PhantomData);
    println!("Item: {}", *item);
    // s is still alive because ItemRef didn't drop its copy
    println!("Original: {}", s);
}
```

**Teaches:** `ManuallyDrop` to prevent the Drop from running — same pattern as `core-foundation::base::ItemRef`.

### Example 58 — The FromVoid trait

```rust
use core::ffi::c_void;
use std::mem::ManuallyDrop;
use std::marker::PhantomData;

unsafe trait FromVoid {
    unsafe fn from_void<'a>(x: *const c_void) -> ManuallyDrop<Self>;
}

unsafe impl FromVoid for u32 {
    unsafe fn from_void<'a>(x: *const c_void) -> ManuallyDrop<Self> {
        ManuallyDrop::new(x as u32)
    }
}

unsafe impl FromVoid for *const c_void {
    unsafe fn from_void<'a>(x: *const c_void) -> ManuallyDrop<Self> {
        ManuallyDrop::new(x)
    }
}

fn main() {
    let x: u32 = 42;
    let ptr = &x as *const u32 as *const c_void;
    let val = unsafe { u32::from_void(ptr) };
    println!("Got: {}", *val);
}
```

**Teaches:** The `FromVoid` trait — converts a `*const c_void` to a typed value.

### Example 59 — The ToVoid trait

```rust
use core::ffi::c_void;

unsafe trait ToVoid<T> {
    fn to_void(&self) -> *const c_void;
}

unsafe impl ToVoid<*const c_void> for *const c_void {
    fn to_void(&self) -> *const c_void { *self }
}

fn main() {
    let x: i32 = 42;
    let ptr = &x as *const i32 as *const c_void;
    println!("Void ptr: {:p}", ptr.to_void());
}
```

**Teaches:** The `ToVoid<T>` trait — converts a `T` to `*const c_void`.

### Example 60 — Generic newtype with PhantomData

```rust
use std::marker::PhantomData;

// Simulating CFArray<T>
struct MyArray<T> {
    ptr: usize,  // pretend it's a raw pointer
    _marker: PhantomData<T>,
}

impl<T> MyArray<T> {
    fn new() -> MyArray<T> {
        MyArray { ptr: 0, _marker: PhantomData }
    }
}

fn main() {
    let arr: MyArray<i32> = MyArray::new();
    let arr2: MyArray<String> = MyArray::new();
    println!("Made two typed arrays.");
    // (Type T is "used" by PhantomData even though we don't store any T.)
}
```

**Teaches:** `PhantomData<T>` marks a type parameter as "conceptually used" so the compiler is satisfied.

### Example 61 — The impl_CFComparison macro effect

```rust
use std::cmp::Ordering;

#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum CFComparisonResult {
    LessThan = -1,
    EqualTo = 0,
    GreaterThan = 1,
}

impl From<CFComparisonResult> for Ordering {
    fn from(r: CFComparisonResult) -> Self {
        match r {
            CFComparisonResult::LessThan => Ordering::Less,
            CFComparisonResult::EqualTo => Ordering::Equal,
            CFComparisonResult::GreaterThan => Ordering::Greater,
        }
    }
}

// Simulating impl_CFComparison! for a wrapper
struct MyNumber(i32);

impl MyNumber {
    fn compare(&self, other: &MyNumber) -> CFComparisonResult {
        if self.0 < other.0 { CFComparisonResult::LessThan }
        else if self.0 > other.0 { CFComparisonResult::GreaterThan }
        else { CFComparisonResult::EqualTo }
    }
}

impl PartialEq for MyNumber {
    fn eq(&self, other: &Self) -> bool { self.compare(other) == CFComparisonResult::EqualTo }
}

impl PartialOrd for MyNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other).into())
    }
}

impl Eq for MyNumber {}

impl Ord for MyNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let a = MyNumber(5);
    let b = MyNumber(10);
    println!("a < b? {}", a < b);
    println!("a cmp b: {:?}", a.cmp(&b));
}
```

**Teaches:** The full comparison trait stack — `PartialEq`, `Eq`, `PartialOrd`, `Ord` — built on top of a C comparison function. Same pattern as `impl_CFComparison!`.

### Example 62 — The foreign_types pattern

```rust
// Simulating foreign_type! macro behavior
use std::ops::Deref;

struct CGContext {
    ptr: *mut std::ffi::c_void,
}

struct CGContextRef {
    ptr: *mut std::ffi::c_void,
}

impl Drop for CGContext {
    fn drop(&mut self) {
        println!("Releasing CGContext");
        // unsafe { CFRelease(self.ptr as *const _) }
    }
}

impl Deref for CGContext {
    type Target = CGContextRef;
    fn deref(&self) -> &CGContextRef {
        // SAFETY: in real code this is sound because CGContextRef is a transparent wrapper
        unsafe { std::mem::transmute::<&CGContext, &CGContextRef>(self) }
    }
}

impl CGContextRef {
    fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        println!("Filling rect ({}, {}, {}, {})", x, y, w, h);
    }
}

fn main() {
    let ctx = CGContext { ptr: std::ptr::null_mut() };
    // ctx.fill_rect(...) works because CGContext: Deref<Target=CGContextRef>
    ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
}
```

**Teaches:** The owned/borrowed distinction via `Deref`. `CGContext` (owned, has Drop) derefs to `CGContextRef` (borrowed, no Drop). Same pattern as `foreign_types::foreign_type!`.

### Example 63 — Trampoline for C callbacks

```rust
use core::ffi::c_void;

extern "C" fn trampoline<F: FnMut(i32)>(info: *mut c_void, value: i32) {
    let f = unsafe { &mut *(info as *mut F) };
    f(value);
}

fn apply_with_callback<F: FnMut(i32)>(f: F) {
    let mut f = f;
    let info = &mut f as *mut F as *mut c_void;
    trampoline::<F>(info, 42);
}

fn main() {
    let mut sum = 0;
    apply_with_callback(|x| sum += x);
    apply_with_callback(|x| sum += x * 2);
    println!("Sum: {}", sum);
}
```

**Teaches:** The Rust closure → C callback trampoline pattern. Used by `CGPath::apply` and other CG functions.

### Example 64 — Extension trait

```rust
// Adding methods to u32 via a trait (the SymbolicTraitAccessors pattern)
trait BitFlags {
    fn has_flag(&self, flag: u32) -> bool;
    fn add_flag(&self, flag: u32) -> u32;
}

impl BitFlags for u32 {
    fn has_flag(&self, flag: u32) -> bool { (self & flag) != 0 }
    fn add_flag(&self, flag: u32) -> u32 { self | flag }
}

fn main() {
    let x: u32 = 0b1010;
    println!("Has 0b0010? {}", x.has_flag(0b0010));
    println!("Has 0b0100? {}", x.has_flag(0b0100));
    println!("Add 0b0100: {:b}", x.add_flag(0b0100));
}
```

**Teaches:** The extension-trait pattern from `core-text/font_descriptor.rs`.

### Example 65 — Trait objects (dyn)

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle { fn draw(&self) { println!("Drawing circle"); } }
impl Drawable for Square { fn draw(&self) { println!("Drawing square"); } }

fn draw_all(items: &[Box<dyn Drawable>]) {
    for item in items {
        item.draw();
    }
}

fn main() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
        Box::new(Circle),
    ];
    draw_all(&items);
}
```

**Teaches:** `dyn Trait` — trait objects with dynamic dispatch via vtable.

**C++ comparison:**
```cpp
std::vector<std::unique_ptr<Drawable>> items;
items.push_back(std::make_unique<Circle>());
// ...
for (const auto& item : items) item->draw();
```

### Example 66 — Generic dispatch (static)

```rust
trait Drawable { fn draw(&self); }

struct Circle;
struct Square;

impl Drawable for Circle { fn draw(&self) { println!("Circle"); } }
impl Drawable for Square { fn draw(&self) { println!("Square"); } }

fn draw_all<T: Drawable>(items: &[T]) {
    for item in items { item.draw(); }
}

fn main() {
    let circles = vec![Circle, Circle];
    let squares = vec![Square, Square, Square];
    draw_all(&circles);
    draw_all(&squares);
}
```

**Teaches:** Static dispatch via generics — compiler generates a fresh `draw_all` for each concrete `T`. Faster than `dyn`, at the cost of code size.

### Example 67 — Arc and Send/Sync

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("Thread sees: {:?}", data);
        }));
    }

    for h in handles { h.join().unwrap(); }
}
```

**Teaches:** `Arc<T>` for thread-safe shared ownership. Same as `std::shared_ptr<T>` in C++.

### Example 68 — RefCell for interior mutability

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2, 3]);

    {
        let mut borrow = cell.borrow_mut();
        borrow.push(4);
    }

    let borrow = cell.borrow();
    println!("Cell: {:?}", *borrow);
}
```

**Teaches:** `RefCell<T>` — runtime-checked mutable borrow. Same idea as a `std::mutex` but single-threaded.

### Example 69 — Mutex

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(0);

    let mut handles = vec![];
    for _ in 0..10 {
        let data = data.clone();
        handles.push(thread::spawn(move || {
            let mut guard = data.lock().unwrap();
            *guard += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }

    println!("Final: {}", *data.lock().unwrap());
}
```

**Teaches:** `Mutex<T>` for thread-safe interior mutability.

### Example 70 — Cow (copy-on-write)

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if input.contains("bad") {
        // Need to allocate a cleaned version
        Cow::Owned(input.replace("bad", "good"))
    } else {
        // No allocation needed — just borrow the input
        Cow::Borrowed(input)
    }
}

fn main() {
    let a = process("hello");
    let b = process("this is bad");
    println!("a: {} (owned={})", a, !matches!(a, Cow::Borrowed(_)));
    println!("b: {}", b);
}
```

**Teaches:** `Cow<'a, B>` — either a borrow or an owned value, decided at runtime. Same pattern as `CFString`'s `Cow<'a, str>` conversion.

### Example 71 — `mem::forget` to skip Drop

```rust
struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let a = Loud("a");
    let b = Loud("b");
    std::mem::forget(b);  // b's Drop won't run!
    // When the scope ends:
    //   - a is dropped normally → "Dropping a" prints
    //   - b is forgotten → no print, memory leaked
}
```

**Teaches:** `mem::forget` — prevents Drop from running. Used by `into_CFType()` and `downcast_into()` to transfer ownership without refcount changes.

### Example 72 — ManuallyDrop

```rust
use std::mem::ManuallyDrop;

struct Loud(&'static str);
impl Drop for Loud {
    fn drop(&mut self) { println!("Dropping {}", self.0); }
}

fn main() {
    let manual = ManuallyDrop::new(Loud("manual"));
    // When `manual` goes out of scope, Loud::drop is NOT called.

    // To actually drop, you'd call ManuallyDrop::drop(&mut manual) explicitly.
    // We're not, so this leaks.
    println!("Made: {:?}", manual.0);
}
```

**Teaches:** `ManuallyDrop<T>` — wraps T but doesn't auto-drop. Used by `ItemRef` to prevent the borrowed item from being released.

### Example 73 — Send and Sync markers

```rust
// A type is Send if it can be moved across threads safely.
// A type is Sync if it can be shared (via &T) across threads safely.

struct NotSend(*mut i32);  // raw pointers are not Send or Sync

// This would fail to compile:
// fn main() {
//     let x = NotSend(std::ptr::null_mut());
//     std::thread::spawn(move || { /* x moved here */ });
// }

// To explicitly opt in (you take responsibility):
struct NotSend(*mut i32);
unsafe impl Send for NotSend {}
unsafe impl Sync for NotSend {}

fn main() {
    let x = NotSend(std::ptr::null_mut());
    std::thread::spawn(move || {
        let _ = x;
    }).join().unwrap();
}
```

**Teaches:** `Send` and `Sync` marker traits. `unsafe impl` opts in to thread-safety — you're promising the compiler the type is safe to share.

### Example 74 — cfg gate for macOS

```rust
#[cfg(target_os = "macos")]
fn mac_only() -> &'static str { "I'm on macOS" }

fn main() {
    #[cfg(target_os = "macos")]
    println!("{}", mac_only());

    #[cfg(not(target_os = "macos"))]
    println!("I'm not on macOS");

    println!("Done.");
}
```

**Teaches:** `#[cfg(target_os = "...")]` for platform-specific code.

### Example 75 — Feature flags

```rust
// In Cargo.toml: [features] fancy = []

#[cfg(feature = "fancy")]
fn greet() -> &'static str { "Fancy hello!" }

#[cfg(not(feature = "fancy"))]
fn greet() -> &'static str { "Plain hello." }

fn main() {
    println!("{}", greet());
}
```

**Teaches:** `#[cfg(feature = "...")]` for feature-gated code. Run with `cargo run --features fancy` to see the fancy version.

---

## Part 6: Graphics, Text, Cocoa Integration (Examples 76–95)

### Example 76 — CGPoint, CGSize, CGRect

```rust
use core_graphics_types::geometry::{CGPoint, CGSize, CGRect};

fn main() {
    let origin = CGPoint::new(10.0, 20.0);
    let size = CGSize::new(100.0, 50.0);
    let rect = CGRect::new(&origin, &size);

    println!("Origin: ({}, {})", rect.origin.x, rect.origin.y);
    println!("Size: {} × {}", rect.size.width, rect.size.height);

    let p = CGPoint::new(50.0, 30.0);
    println!("Point in rect? {}", rect.contains(&p));
}
```

**Teaches:** The POD geometry types from `core-graphics-types`.

### Example 77 — CGAffineTransform

```rust
use core_graphics_types::geometry::{CGAffineTransform, CGPoint};

fn main() {
    let identity = CGAffineTransform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);  // identity
    let p = CGPoint::new(5.0, 10.0);
    let p2 = p.apply_transform(&identity);
    println!("({}, {}) -> ({}, {})", p.x, p.y, p2.x, p2.y);

    let inverted = identity.invert();
    let p3 = p2.apply_transform(&inverted);
    println!("Back: ({}, {})", p3.x, p3.y);
}
```

**Teaches:** Affine transforms — the C equivalent of `CGAffineTransformIdentity` etc.

### Example 78 — CGColor

```rust
use core_graphics::color::CGColor;

fn main() {
    let red = CGColor::rgb(1.0, 0.0, 0.0, 1.0);
    println!("Made red color.");
    // (CGColor doesn't implement Display by default; you'd inspect its components via CGColorGetComponents.)
}
```

**Teaches:** Creating a `CGColor` from RGB+A values.

### Example 79 — CGContext (bitmap context)

```rust
use core_graphics::color::CGColor;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use core_graphics_types::geometry::{CGRect, CGSize};

fn main() {
    let width = 100;
    let height = 100;
    let color_space = CGColorSpace::new_device_rgb();
    let ctx = CGContext::create_bitmap_context(
        None, width, height, 8, width * 4, &color_space,
        core_graphics::base::kCGImageAlphaPremultipliedLast,
    );

    ctx.set_fill_color(&CGColor::rgb(1.0, 0.0, 0.0, 1.0));   // red
    ctx.fill_rect(CGRect::new(&core_graphics_types::geometry::CGPoint::new(0.0, 0.0),
                              &CGSize::new(50.0, 50.0)));

    println!("Drew a 50x50 red rect on a 100x100 bitmap context.");
}
```

**Teaches:** Creating a bitmap context and drawing into it.

### Example 80 — Drawing a CGImage

```rust
// This example requires more setup; illustrative only.
use core_graphics::context::CGContext;
use core_graphics::image::CGImage;
use core_graphics_types::geometry::{CGPoint, CGRect, CGSize};

fn draw_image_somewhere(ctx: &CGContext, image: &CGImage) {
    let rect = CGRect::new(&CGPoint::new(0.0, 0.0), &CGSize::new(100.0, 100.0));
    ctx.draw_image(rect, image);
}

fn main() {
    println!("This example is illustrative — needs a real image to draw.");
}
```

**Teaches:** The `CGContext::draw_image` API.

### Example 81 — CGPath

```rust
use core_graphics::path::{CGPath, CGPathRef};

fn main() {
    let path = CGPath::new();
    // Adding a rectangle (simplified — real API uses CGRectMakeWithRect or add_rect)
    println!("Made empty CGPath.");
}
```

**Teaches:** Creating a `CGPath`. Full path manipulation requires more API surface.

### Example 82 — CGFont

```rust
use core_graphics::font::CGFont;
use core_foundation::string::CFString;

fn main() {
    let name = CFString::from_static_string("Helvetica");
    let font = CGFont::from_name(&name);
    match font {
        Some(f) => println!("Got font: {:?}", f.post_script_name()),
        None => println!("Font not found."),
    }
}
```

**Teaches:** Loading a `CGFont` by name.

### Example 83 — CTFont

```rust
use core_text::font::CTFont;
use core_foundation::string::CFString;
use core_graphics::base::CGFloat;

fn main() {
    let font = CTFont::new("Helvetica", 12.0);
    println!("Family: {}", font.family_name());
    println!("PostScript: {}", font.post_script_name());
    println!("Ascent: {}, Descent: {}", font.ascent(), font.descent());
}
```

**Teaches:** Creating a `CTFont` and reading its properties.

### Example 84 — CTFontDescriptor

```rust
use core_text::font_descriptor::{CTFontDescriptor, CTFontSymbolicTraits, SymbolicTraitAccessors};
use core_foundation::string::CFString;
use core_foundation::dictionary::CFDictionary;

fn main() {
    let name = CFString::from_static_string("Helvetica-Bold");
    let desc = CTFontDescriptor::new_with_name(&name);
    let traits = desc.symbolic_traits();
    println!("Bold? {}", traits.is_bold());
    println!("Italic? {}", traits.is_italic());
    println!("Monospace? {}", traits.is_monospace());
}
```

**Teaches:** Using `CTFontDescriptor` and the `SymbolicTraitAccessors` extension trait.

### Example 85 — CFAttributedString

```rust
use core_foundation::attributed_string::CFAttributedString;
use core_foundation::string::CFString;

fn main() {
    let s = CFString::from_static_string("Hello, attributed!");
    let attrs = core_foundation::dictionary::CFDictionary::new();  // empty attrs
    let attr_str = CFAttributedString::new(&s, &attrs);
    println!("Length: {}", attr_str.char_len());
}
```

**Teaches:** Creating a `CFAttributedString`.

### Example 86 — CTLine from attributed string

```rust
use core_text::line::CTLine;
use core_foundation::attributed_string::CFAttributedString;
use core_foundation::string::CFString;
use core_foundation::dictionary::CFDictionary;

fn main() {
    let s = CFString::from_static_string("Hello, line!");
    let attrs = CFDictionary::new();
    let attr_str = CFAttributedString::new(&s, &attrs);
    let line = CTLine::new_with_attributed_string(attr_str.as_concrete_TypeRef());
    let bounds = line.get_typographic_bounds();
    println!("Line width: {}", bounds.width);
    println!("Ascent: {}, Descent: {}", bounds.ascent, bounds.descent);
}
```

**Teaches:** Laying out a `CTLine` and reading its metrics.

### Example 87 — CFRunLoop (just get current)

```rust
use core_foundation::runloop::CFRunLoop;

fn main() {
    let rl = CFRunLoop::get_current();
    println!("Got current run loop. Mode count: {}", rl.count_current_mode_runs());
}
```

**Teaches:** Getting the current run loop.

### Example 88 — CFBundle main bundle resources

```rust
use core_foundation::bundle::CFBundle;
use core_foundation::string::CFString;

fn main() {
    let bundle = CFBundle::main_bundle();
    let resource = bundle.copy_resource_url(
        &CFString::from_static_string("example"),
        &CFString::from_static_string("txt"),
    );
    match resource {
        Some(url) => println!("Found: {}", url.get_string()),
        None => println!("example.txt not found in bundle resources."),
    }
}
```

**Teaches:** Finding a resource in the main bundle.

### Example 89 — IOSurface (illustrative)

```rust
// This is illustrative; the io-surface crate is deprecated.
fn main() {
    println!("IOSurface is the cross-process shared memory surface.");
    println!("Used for transferring image data between processes,");
    println!("e.g. video decoder → renderer, or for GPU texture sharing.");
}
```

**Teaches:** What `IOSurface` is for, conceptually.

### Example 90 — Cocoa: NSAutoreleasePool

```rust
// This requires macOS + cocoa crate
#![allow(deprecated)]
use cocoa::foundation::NSAutoreleasePool;
use cocoa::base::nil;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        // Any autoreleased Cocoa objects created here will be released when _pool drops.
        println!("Inside autorelease pool.");
    }
    println!("Pool drained.");
}
```

**Teaches:** The autorelease pool pattern from `cocoa/examples/hello_world.rs`.

### Example 91 — Cocoa: NSString

```rust
#![allow(deprecated)]
use cocoa::foundation::NSString;
use cocoa::base::{id, nil};
use cocoa::base::YES;

fn main() {
    unsafe {
        let s: id = NSString::alloc(nil).init_str("Hello from NSString");
        let len: usize = msg_send_string_length(s);
        println!("NSString length: {}", len);
    }
}

extern "C" {
    #[link(name = "Foundation", kind = "framework")]
    fn msg_send_string_length(s: id) -> usize;
}
```

**Teaches:** Creating a Cocoa `NSString` via the trait-on-`id` pattern. (This example is illustrative — real code uses `msg_send!` from the `objc` crate.)

### Example 92 — Cocoa: msg_send! macro

```rust
// In real code: use objc::{msg_send, sel, sel_impl};
// Illustrative:
macro_rules! msg_send {
    ($obj:expr, $sel:ident) => {
        unsafe { $crate::send($obj, stringify!($sel)) }
    };
}

fn send(_obj: usize, _sel: &str) -> usize { 42 }

fn main() {
    let result = msg_send!(0, description);
    println!("Result: {}", result);
}
```

**Teaches:** A simplified `msg_send!` macro. The real one calls `objc_msgSend`.

### Example 93 — Cocoa: hello_world.rs skeleton

```rust
// A condensed skeleton of cocoa/examples/hello_world.rs
#![allow(deprecated)]
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        // ... create menu, window, etc.
        // app.run();  // would block
        println!("App setup complete.");
    }
}
```

**Teaches:** The skeleton of any Cocoa app — autorelease pool, get NSApp, set activation policy.

### Example 94 — Cocoa: NSWindow creation

```rust
#![allow(deprecated)]
use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSBackingStoreBuffered};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize};

fn main() {
    unsafe {
        let window: id = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
            NSWindowStyleMask::NSTitledWindowMask,
            NSBackingStoreBuffered,
            NO,
        );
        println!("Window allocated at: {:p}", window);
        // (In real code, you'd also setTitle_, center, makeKeyAndOrderFront_.)
    }
}
```

**Teaches:** Allocating and initialising an `NSWindow` — same code from `hello_world.rs`.

### Example 95 — Cocoa: full hello world

```rust
// Re-read cocoa/examples/hello_world.rs as the canonical example.
// It's covered in detail in File 6 §6.8.
fn main() {
    println!("See File 6 §6.8 for the complete hello_world.rs walkthrough.");
}
```

**Teaches:** Where to find the full working example.

---

## Part 7: Putting It All Together (Examples 96–100)

### Example 96 — A complete FFI wrapper (mini)

```rust
use std::ffi::c_void;

// Simulated: a complete safe wrapper around a fictional C library

// FFI declarations (like core-foundation-sys)
mod ffi {
    use std::ffi::c_void;
    extern "C" {
        pub fn widget_create(name: *const i8) -> *mut c_void;
        pub fn widget_destroy(w: *mut c_void);
        pub fn widget_get_name(w: *const c_void) -> *const i8;
    }
}

// Safe wrapper (like core-foundation)
pub struct Widget {
    ptr: *mut c_void,
}

impl Widget {
    pub fn new(name: &str) -> Widget {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let ptr = ffi::widget_create(c_name.as_ptr());
            if ptr.is_null() {
                panic!("widget_create returned NULL");
            }
            Widget { ptr }
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let c_str = ffi::widget_get_name(self.ptr);
            std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        unsafe { ffi::widget_destroy(self.ptr) }
    }
}

fn main() {
    let w = Widget::new("my_widget");
    println!("Widget name: {}", w.name());
    // w is dropped here; widget_destroy runs.
}
```

**Teaches:** The complete safe-wrapper pattern in miniature. Same architecture as `core-foundation`.

### Example 97 — A custom TCFType impl

```rust
// Declaring a custom type that works with the core-foundation infrastructure
use core_foundation::base::{TCFType, CFTypeRef, CFTypeID};
use core_foundation::{declare_TCFType, impl_TCFType, impl_CFTypeDescription};

// Pretend we have an extern "C" function that returns a type ID
extern "C" {
    fn MyThingGetTypeID() -> CFTypeID;
}

#[repr(C)]
pub struct __MyThing(core::ffi::c_void);
pub type MyThingRef = *const __MyThing;

declare_TCFType!(MyThing, MyThingRef);
impl_TCFType!(MyThing, MyThingRef, MyThingGetTypeID);
impl_CFTypeDescription!(MyThing);

fn main() {
    // Now MyThing has Drop, Clone, PartialEq, Eq, Debug, TCFType, ToVoid impls.
    println!("MyThing declared.");
}
```

**Teaches:** How to declare your own CF-compatible type using the library's macros. This is how the metal-cpp guide's "make your own wrapper" pattern looks in Rust.

### Example 98 — Reading a plist file (illustrative)

```rust
use core_foundation::propertylist::CFPropertyList;
use core_foundation::data::CFData;
use core_foundation::error::CFError;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("Info.plist")?;
    let cf_data = CFData::from_buffer(&bytes);
    let plist = CFPropertyList::from_data(&cf_data)?;
    match plist {
        CFPropertyList::CFDictionary(dict) => {
            println!("Plist is a dictionary with {} entries", dict.len());
        }
        CFPropertyList::CFArray(arr) => {
            println!("Plist is an array with {} entries", arr.len());
        }
        _ => println!("Plist is something else"),
    }
    Ok(())
}
```

**Teaches:** Putting together `CFData`, `CFPropertyList`, and Rust's `Result` for a real use case.

### Example 99 — Measuring GPU work (illustrative)

```rust
// Illustrative — combining several wrappers
use core_foundation::string::CFString;
use core_foundation::number::CFNumber;
use core_foundation::dictionary::CFMutableDictionary;

fn main() {
    let mut dict: CFMutableDictionary<CFString, CFNumber> = CFMutableDictionary::new();
    dict.add(&CFString::from_static_string("width"), &CFNumber::from(1920));
    dict.add(&CFString::from_static_string("height"), &CFNumber::from(1080));
    dict.add(&CFString::from_static_string("fps"), &CFNumber::from(60));

    println!("Config has {} entries", dict.len());
}
```

**Teaches:** Combining `CFString`, `CFNumber`, and `CFMutableDictionary` for a config-like use case.

### Example 100 — The full picture

```rust
// A capstone example: build a CFDictionary of CFStrings and CFNumbers,
// upcast to CFType, downcast back, and extract values.

use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;

fn main() {
    // Build the dictionary
    let key1 = CFString::from_static_string("name");
    let val1 = CFString::from_static_string("Alice");
    let key2 = CFString::from_static_string("age");
    let val2 = CFNumber::from(30);

    let dict = CFDictionary::from_CFType_pairs(&[
        (key1.as_CFType(), val1.as_CFType()),
        (key2.as_CFType(), val2.as_CFType()),
    ]);

    println!("Dict has {} entries", dict.len());

    // Look up "name"
    let lookup = CFString::from_static_string("name");
    if let Some(value_ref) = dict.find(&lookup) {
        // value_ref is ItemRef<CFType>. Downcast it to CFString.
        if let Some(s) = value_ref.clone().downcast::<CFString>() {
            println!("name = {}", s);
        }
    }

    // Look up "age"
    let lookup = CFString::from_static_string("age");
    if let Some(value_ref) = dict.find(&lookup) {
        if let Some(n) = value_ref.clone().downcast::<CFNumber>() {
            if let Some(age) = n.to_i32() {
                println!("age = {}", age);
            }
        }
    }

    // The dictionary and all its keys/values are released when `dict` goes out of scope.
    // No manual CFRelease needed — Rust's Drop handles it.
}
```

**Teaches:** Everything you've learned, combined. Type-safe heterogeneous dictionary, lookup, downcast, and automatic memory management — all built on top of the unsafe FFI layer you studied in Phase 2.

---

## Final notes on the 100 examples

- Examples 1–10 (Rust fundamentals) are the foundation. If you struggled with any of them, re-read File 1 §1.2–§1.10.
- Examples 11–20 (FFI basics) teach the C-Rust bridge. Most useful for understanding Phase 2.
- Examples 21–35 (`core-foundation-sys` patterns) are mini-versions of what's in the real FFI files.
- Examples 36–55 (safe wrappers) are the heart — they show how the `core-foundation` crate's patterns work.
- Examples 56–75 (advanced patterns) go deeper into Rust idioms: lifetimes, traits, generics, interior mutability, `Send`/`Sync`.
- Examples 76–95 (graphics, text, cocoa) show domain-specific usage.
- Examples 96–100 put it all together.

**Try them yourself.** For each example, predict the output before running. If your prediction was wrong, figure out why. This is the fastest way to learn.

**Open File 8 when ready** for the appendix.

— End of File 7 —
