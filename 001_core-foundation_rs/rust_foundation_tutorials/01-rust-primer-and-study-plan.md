# core-foundation-rs: A 200-Day Study Guide

**File 1 of 8 — Rust Primer & 200-Day Study Plan**

> **Audience:** A learner with some C++ background who is new to Rust and wants to deeply understand the `core-foundation-rs` codebase.
> **Source code under study:** `core-foundation-rs-main.zip` — Rust bindings to Apple's Core Foundation, Core Graphics, Core Text, and Cocoa frameworks, maintained by the Servo project.
> **Goal:** Read every `.rs` file in the project (106 files, ~25,500 lines), understand what each section does, and build a durable mental model of (a) how Rust wraps foreign C APIs safely and (b) how each Rust idiom compares to its C++ analogue.
> **Pace:** ~200 days, 30–60 minutes per day.

This is **File 1 of 8**. The eight files are:

| # | File | Covers |
|---|---|---|
| 1 | `01-rust-primer-and-study-plan.md` | **This file.** Rust primer with side-by-side C++ comparisons + the 200-day plan. |
| 2 | `02-core-foundation-sys-walkthrough.md` | The 40 FFI files in `core-foundation-sys/` — raw `extern "C"` bindings. |
| 3 | `03-core-foundation-walkthrough.md` | The 22 safe-wrapper files in `core-foundation/` — the heart of the library. |
| 4 | `04-core-graphics-and-types-walkthrough.md` | The 20 files in `core-graphics/` + `core-graphics-types/`. |
| 5 | `05-core-text-walkthrough.md` | The 10 files in `core-text/`. |
| 6 | `06-cocoa-and-io-surface-walkthrough.md` | The 14 files in `cocoa/`, `cocoa-foundation/`, `io-surface/` — including example programs. |
| 7 | `07-rust-code-examples.md` | 100 progressive Rust code examples derived from this codebase. |
| 8 | `08-appendix.md` | Patterns reference, glossary, where-to-go-next. |

Read this file first, in order, before opening any of the others. The primer (Part 1) teaches you the Rust you'll need; the study plan (Part 2) tells you what to read each day.

---

## Table of Contents (this file)

**Part 1 — Rust Primer for C++ Programmers**
- 1.1 The 10,000-foot view: how Rust differs from C++
- 1.2 Variables, types, and mutability
- 1.3 Functions and control flow
- 1.4 Structs (vs. C++ classes/structs)
- 1.5 Enums and pattern matching (Rust's killer feature)
- 1.6 Traits (vs. C++ classes, virtual functions, concepts)
- 1.7 Generics (vs. C++ templates)
- 1.8 Ownership (the Big Idea)
- 1.9 Borrowing and references
- 1.10 Lifetimes
- 1.11 Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`
- 1.12 Error handling: `Option` and `Result` (vs. exceptions)
- 1.13 Modules, crates, and visibility
- 1.14 Macros (vs. C preprocessor)
- 1.15 `unsafe` Rust (and why this codebase needs it)
- 1.16 FFI: calling C from Rust (and vice versa)
- 1.17 The Cargo build system (vs. CMake/Make)

**Part 2 — The 200-Day Study Plan**
- 2.1 The seven phases at a glance
- 2.2 Phase 1 (Days 1–20): Rust primer + setup
- 2.3 Phase 2 (Days 21–55): `core-foundation-sys/` — the FFI layer (40 files)
- 2.4 Phase 3 (Days 56–105): `core-foundation/` — the safe wrapper (22 files)
- 2.5 Phase 4 (Days 106–140): `core-graphics/` + `core-graphics-types/` (20 files)
- 2.6 Phase 5 (Days 141–165): `core-text/` (10 files)
- 2.7 Phase 6 (Days 166–185): `cocoa/` + `cocoa-foundation/` + `io-surface/` (14 files)
- 2.8 Phase 7 (Days 186–200): The 100 code examples + capstone
- 2.9 How to study each day
- 2.10 Self-check questions to use throughout

---

# Part 1 — Rust Primer for C++ Programmers

## 1.1 The 10,000-foot view: how Rust differs from C++

If you remember only five things about Rust vs. C++, remember these:

1. **No null pointers.** Where C++ has `T*` that may or may not be valid, Rust has `&T` (a borrow that's statically guaranteed valid) and `Option<&T>` (a borrow that may be absent). The concept of "null" doesn't exist in safe Rust.

2. **No default inheritance.** C++ lets you inherit classes and override virtual functions. Rust does *not* have class inheritance. Instead it has **traits** (like C++ concepts or Java interfaces) and **trait objects** (`dyn Trait`, which is roughly `std::unique_ptr<Base>` with a vtable). Composition over inheritance.

3. **Ownership is statically enforced.** Every value has exactly one owner; when the owner goes out of scope, the value is dropped. There's no `new`/`delete` and no `malloc`/`free` in user code; the compiler inserts the drops.

4. **Borrowing is statically enforced.** At any moment, a value either has **one mutable borrow** (`&mut T`) or **any number of immutable borrows** (`&T`), never both. The borrow checker enforces this at compile time. This is the thing that drives newcomers crazy and pays off forever once it clicks.

5. **`unsafe` is an opt-in escape hatch.** Anything that violates the above rules (raw pointers, calling C, mutating globals) requires the `unsafe` keyword. `unsafe` doesn't turn off safety; it just shifts responsibility to you. Most Rust code is safe; a small amount (FFI wrappers like `core-foundation-rs`) is `unsafe` internally but exposes a safe API.

**Why this codebase is interesting for learning Rust:** `core-foundation-rs` is a *thin safe wrapper around an unsafe C API*. You'll see both halves: the `core-foundation-sys` crate is pure FFI (`extern "C"` declarations, raw pointers, `unsafe` everywhere); the `core-foundation` crate wraps each unsafe operation in a safe method that enforces Rust's ownership rules. This is *exactly* the pattern you'd use to wrap any C library in Rust.

**The parallel to the metal-cpp guide you read earlier:** metal-cpp wraps Objective-C objects using C++ templates and inline functions that call `objc_msgSend`. core-foundation-rs wraps C functions using Rust traits and methods that call the same C functions via FFI. Both libraries solve the same problem — give C++/Rust programmers safe access to Apple's frameworks — but they do it with completely different language idioms. Comparing them side-by-side is one of the fastest ways to internalise both.

## 1.2 Variables, types, and mutability

### Rust

```rust
let x = 5;            // immutable i32 (inferred)
let mut y = 10;       // mutable i32
let z: i64 = 100;     // explicit type
let s = "hello";      // &str (string slice, borrowed)
let owned = String::from("hi");  // String (owned, heap-allocated)
const MAX: usize = 1024;         // compile-time constant
static GLOBAL: i32 = 42;         // program-lifetime variable
```

### C++ equivalent

```cpp
auto x = 5;            // int (inferred)
int y = 10;            // there's no "let mut" — you just declare mutable
auto z = long{100};    // long
const char* s = "hello";
std::string owned = "hi";
constexpr size_t MAX = 1024;
static int global = 42;     // or thread_local, or inline
```

**Key differences:**

- Rust variables are **immutable by default**. `let x = 5;` means you can't later write `x = 6;`. You must write `let mut x = 5;`. In C++, you have to *add* `const` to make something immutable; in Rust you have to *add* `mut`. This is a deliberate flip: the safe default wins.

- Rust has no implicit numeric widening. `let x: i32 = 5; let y: u64 = x;` won't compile — you need `let y = x as u64;`. C++ often silently widens (with sign-extension rules that bite people).

- Rust's `&str` and `String` are different types with different roles. `&str` is a borrow of some string data (could be in the binary, on the heap, wherever); `String` is the owned heap-allocated growable version. The C++ analogue is `std::string_view` vs. `std::string` (added in C++17).

- Rust distinguishes `i32`/`u32`/`isize`/`usize` strictly. `isize`/`usize` are pointer-sized (like `intptr_t`/`uintptr_t` in C). Most indexing uses `usize`.

### Type table

| Rust | C++ | Notes |
|---|---|---|
| `i8`/`u8` | `int8_t`/`uint8_t` | |
| `i16`/`u16` | `int16_t`/`uint16_t` | |
| `i32`/`u32` | `int32_t`/`uint32_t` | |
| `i64`/`u64` | `int64_t`/`uint64_t` | |
| `isize`/`usize` | `intptr_t`/`uintptr_t` | pointer-sized |
| `f32`/`f64` | `float`/`double` | |
| `bool` | `bool` | |
| `char` | (no equivalent) | Rust `char` is a 4-byte Unicode scalar value, not a byte |
| `&str` / `String` | `std::string_view` / `std::string` | |
| `()` | `void` (sort of) | Rust's unit type, the empty tuple |
| `(T, U)` | `std::tuple<T, U>` | |
| `&T` / `&mut T` | `const T&` / `T&` | Rust borrows; C++ references |
| `*const T` / `*mut T` | `const T*` / `T*` | Raw pointers — only usable in `unsafe` |

## 1.3 Functions and control flow

### Rust

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b            // no semicolon — last expression is the return value
}

fn early_return(x: i32) -> i32 {
    if x < 0 {
        return -1;   // explicit return works too
    }
    x * 2
}

// Closures (lambdas)
let double = |x| x * 2;
let add_one = |x: i32| -> i32 { x + 1 };
let captures: Vec<i32> = vec![1, 2, 3];
let sum: i32 = captures.iter().map(|x| x * 2).sum();
```

### C++ equivalent

```cpp
int add(int a, int b) {
    return a + b;
}

int early_return(int x) {
    if (x < 0) return -1;
    return x * 2;
}

// C++14 lambdas
auto double_fn = [](int x) { return x * 2; };
auto add_one = [](int x) -> int { return x + 1; };
std::vector<int> captures = {1, 2, 3};
int sum = std::accumulate(captures.begin(), captures.end(), 0,
                          [](int acc, int x) { return acc + x * 2; });
```

**Key differences:**

- Rust functions are expressions: the last expression **without a semicolon** is the return value. Add a semicolon and you've made it a statement, returning `()`. This takes getting used to.

- `if`/`else`/`match` are also expressions:
  ```rust
  let abs = if x < 0 { -x } else { x };
  ```
  C++ has the ternary operator for this; Rust just uses `if`.

- Closures in Rust capture by reference by default (like C++ `[&]`). You can force other capture modes (`move`), but you rarely need to.

- `for` loops use iterators, not index variables:
  ```rust
  for x in &vec { /* x is &T */ }
  for x in vec.iter() { /* same */ }
  ```
  C++'s range-for is similar: `for (auto& x : vec) { ... }`.

- Rust has no `while (true)` style loop with implicit break conditions; it has `loop { ... break; }` (infinite loop with explicit break), `while cond { ... }`, and `for x in iter { ... }`.

## 1.4 Structs (vs. C++ classes/structs)

### Rust

```rust
// Tuple struct (anonymous fields)
struct Point(f64, f64);

// Named-field struct
struct User {
    name: String,
    age: u32,
    active: bool,
}

// Unit struct (no fields)
struct Marker;

impl User {
    // Associated function (like a static member function in C++)
    fn new(name: String, age: u32) -> User {
        User { name, age, active: true }
    }

    // Method (takes &self)
    fn birthday(&mut self) {
        self.age += 1;
    }

    // Borrowing method
    fn display(&self) -> &str {
        &self.name
    }
}

let mut u = User::new("Alice".into(), 30);
u.birthday();
println!("{}", u.display());
```

### C++ equivalent

```cpp
struct Point { double x, y; };        // or: class Point { public: double x, y; };

class User {
public:
    std::string name;
    uint32_t age;
    bool active;

    User(std::string n, uint32_t a) : name(std::move(n)), age(a), active(true) {}

    void birthday() { age++; }
    const std::string& display() const { return name; }
};

User u("Alice", 30);
u.birthday();
std::cout << u.display();
```

**Key differences:**

- Rust **separates the data layout (`struct`) from the methods (`impl`)**. A `struct` defines fields and nothing else; methods go in a separate `impl` block. C++ mixes them in the class definition.

- Rust has **no constructors**. You build a struct by writing `User { name, age, active: true }` directly. By convention, factories are associated functions named `new` (no `new` keyword, no heap allocation implied).

- `&self` / `&mut self` / `self` (by value) are explicit; you write them in the method signature. The C++ analogue is `const User&` / `User&` / `User` in the implicit `this` slot.

- Rust has **no `private`/`public`/`protected` keywords on fields**. Visibility is per-module: if a struct is in module `foo`, then `foo`'s code can access private fields; code outside `foo` cannot. Use `pub` to make a field visible outside the module.

- Rust has no inheritance. You can't say `struct Admin : User { ... }`. You compose instead — embed a `User` field inside `Admin`.

## 1.5 Enums and pattern matching (Rust's killer feature)

Rust enums are far more powerful than C++ enums. Each variant can carry data, and pattern matching destructures them safely.

### Rust

```rust
enum Shape {
    Circle(f64),                          // variant carries a tuple
    Rectangle { width: f64, height: f64 }, // variant carries named fields
    Square(f64),
    Empty,                                 // variant carries nothing
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Square(s) => s * s,
        Shape::Empty => 0.0,
    }
}

// The compiler checks that the match is exhaustive.
// `_ =>` is a wildcard catch-all.
```

### C++ equivalent (closest)

```cpp
// C++ enum class + std::variant — clunky
enum class ShapeTag { Circle, Rectangle, Square, Empty };

struct Shape {
    ShapeTag tag;
    double r;            // for Circle
    double width, height; // for Rectangle
};

double area(const Shape& s) {
    switch (s.tag) {
        case ShapeTag::Circle: return M_PI * s.r * s.r;
        case ShapeTag::Rectangle: return s.width * s.height;
        case ShapeTag::Square: return s.r * s.r;     // reuses r
        case ShapeTag::Empty: return 0.0;
    }
    // No default — compiler may warn but won't error.
}
```

**Or with `std::variant` (C++17+):**

```cpp
struct Circle { double r; };
struct Rectangle { double width, height; };
struct Square { double s; };
struct Empty {};

using Shape = std::variant<Circle, Rectangle, Square, Empty>;

double area(const Shape& shape) {
    return std::visit([](auto& s) -> double {
        using T = std::decay_t<decltype(s)>;
        if constexpr (std::is_same_v<T, Circle>)    return M_PI * s.r * s.r;
        else if constexpr (std::is_same_v<T, Rectangle>) return s.width * s.height;
        else if constexpr (std::is_same_v<T, Square>)    return s.s * s.s;
        else return 0.0;
    }, shape);
}
```

**Key differences:**

- Rust's `enum` is roughly `std::variant` plus a tag plus pattern matching — but built into the language, with exhaustive-match checking.
- `match` is the Rust equivalent of `switch`, but it's an expression (returns a value), it's exhaustive (the compiler errors if you miss a variant), and it can destructure (pull fields out of variants).
- **The most important Rust enum you'll meet in this codebase is `Option<T>`** — the Rust replacement for `nullptr`. It's defined as:

  ```rust
  pub enum Option<T> {
      None,
      Some(T),
  }
  ```

  Where C++ would say "returns `T*` or `nullptr`", Rust says "returns `Option<T>`". The compiler forces you to handle `None` before you can use the value.

- **The other key enum is `Result<T, E>`** — the Rust replacement for exceptions:

  ```rust
  pub enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

  Fallible functions return `Result` rather than throwing. The compiler forces you to handle the error case.

## 1.6 Traits (vs. C++ classes, virtual functions, concepts)

A **trait** in Rust is a set of method signatures that a type can implement. It's simultaneously:

- A **concept** (C++20) — a constraint on what types can be used as generic arguments.
- An **interface** (Java) — when used as a trait object (`dyn Trait`).
- A **vtable source** — `dyn Trait` is a fat pointer `(data_ptr, vtable_ptr)`.

### Rust

```rust
// Define a trait (an interface / concept)
trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

// Implement it for a concrete type
struct Color(u8, u8, u8);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

// Generic function constrained by a trait bound
fn print_all<T: Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

// Trait object (dynamic dispatch, like virtual in C++)
fn print_dyn(items: &[Box<dyn Display>]) {
    for item in items {
        println!("{}", item);
    }
}
```

### C++ equivalent

```cpp
// C++ pure-virtual class — the closest analogue of a Rust trait used as a trait object
class Display {
public:
    virtual ~Display() = default;
    virtual void fmt(std::ostream& os) const = 0;
};

class Color : public Display {
    uint8_t r, g, b;
public:
    Color(uint8_t r, uint8_t g, uint8_t b) : r(r), g(g), b(b) {}
    void fmt(std::ostream& os) const override {
        os << "rgb(" << (int)r << ", " << (int)g << ", " << (int)b << ")";
    }
};

// Template — equivalent to Rust's generic-with-trait-bound
template <typename T>
void print_all(const std::vector<T>& items) {
    for (const auto& item : items) {
        item.fmt(std::cout);
        std::cout << "\n";
    }
}

// Vector of pointers — equivalent to Rust's Vec<Box<dyn Display>>
void print_dyn(const std::vector<std::unique_ptr<Display>>& items) {
    for (const auto& item : items) {
        item->fmt(std::cout);
        std::cout << "\n";
    }
}
```

**Key differences:**

- **No "is-a" relationship.** In C++, `Color` *is-a* `Display` because it inherits from it. In Rust, `Color` is its own type that *implements the `Display` trait`; there's no inheritance relationship.

- **Implementation is separate from the type.** In Rust, you define `struct Color` and then in a separate `impl Display for Color` block you add the trait implementation. You can even add trait implementations to types you didn't define (the "orphan rule" restricts this somewhat — either the trait or the type must be in your crate).

- **Two dispatch modes:**
  - **Static dispatch** (`fn foo<T: Trait>(x: T)`) — the compiler monomorphises; you get a fresh `foo` for each concrete `T`. Like C++ templates.
  - **Dynamic dispatch** (`fn foo(x: &dyn Trait)`) — the argument is a fat pointer `(data, vtable)`; method calls go through the vtable at runtime. Like C++ `virtual`.

- **No virtual destructors.** Rust doesn't have constructors or destructors in the C++ sense; it has the `Drop` trait (covered in §1.8). When you drop a `Box<dyn Trait>`, the box's `Drop` calls the underlying type's `Drop` via the vtable — so you get correct destruction without explicit `virtual ~Base()`.

- **Object safety.** Not every trait can be used as `dyn Trait`. Roughly: the trait must not have methods that return `Self` or take `Self` by value, and must not have generic methods. C++ doesn't have this restriction (any pure-virtual class can be a base).

### The most important trait in core-foundation-rs: `TCFType`

You'll meet this trait in §3.3 of File 3 (`core-foundation/src/base.rs`). It's the Rust equivalent of metal-cpp's `Referencing<T>` template — every wrapper type implements it. Roughly:

```rust
pub trait TCFType {
    type Ref: TCFTypeRef;              // the underlying raw pointer type
    fn as_concrete_TypeRef(&self) -> Self::Ref;
    unsafe fn wrap_under_create_rule(obj: Self::Ref) -> Self;
    unsafe fn wrap_under_get_rule(obj: Self::Ref) -> Self;
    fn type_id() -> CFTypeID;
    // ... more methods
}
```

The associated type `Ref` (analogous to a C++ `using Ref = ...` typedef inside a class) tells the trait what raw pointer type this wrapper wraps. So `CFString`'s `Ref` is `CFStringRef`, `CFArray<T>`'s `Ref` is `CFArrayRef`, and so on.

## 1.7 Generics (vs. C++ templates)

### Rust

```rust
// Generic function
fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

// Generic struct
struct Pair<A, B> {
    a: A,
    b: B,
}

// With trait bounds
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Multiple bounds with `where` clause (cleaner for complex cases)
fn foo<T, U>(t: T, u: U)
where
    T: Clone + std::fmt::Debug,
    U: PartialEq,
{
    // ...
}
```

### C++ equivalent

```cpp
template <typename T>
const T* first(const std::vector<T>& v) {
    return v.empty() ? nullptr : &v.front();
}

template <typename A, typename B>
struct Pair { A a; B b; };

template <typename T>
T max(T a, T b) { return a > b ? a : b; }    // no constraint; error at instantiation if T has no operator>

// C++20 concepts — closest to Rust trait bounds
template <typename T, typename U>
    requires std::copyable<T> && std::equality_comparable<U>
void foo(T t, U u) { /* ... */ }
```

**Key differences:**

- Rust generics are **checked at definition time**, not at instantiation time. If you write `fn foo<T>(x: T) { x.frobnicate(); }` and `T` has no `frobnicate` method, Rust errors at the function definition. C++ templates are checked only when instantiated (which is why template errors are famously ugly).

- Rust trait bounds (`<T: Trait>`) are checked early. C++20 concepts finally bring this to C++, but it's newer than Rust's approach.

- Rust uses **monomorphisation** by default — each concrete `T` used with `foo<T>` produces a fresh `foo` in the binary. Same as C++ templates. Code bloat is a real risk; use `dyn Trait` (dynamic dispatch) where appropriate.

- Rust doesn't have **template metaprogramming** in the C++ sense. There's no `template <int N>` for compile-time integer parameters (you use `const N: usize` instead, called "const generics"); there's no SFINAE; there's no full-powered constexpr evaluation. Macros and proc-macros cover some of the use cases.

## 1.8 Ownership (the Big Idea)

This is the single most important concept in Rust. Internalise it before reading any Rust code.

**Three rules:**

1. Each value has **one owner** — the variable that holds it.
2. When the owner goes out of scope, the value is **dropped** (its destructor runs).
3. You can **move** ownership to another variable; after the move, the original is no longer usable.

### Rust

```rust
let s1 = String::from("hello");   // s1 owns the String
let s2 = s1;                       // ownership moves to s2; s1 is now invalid
// println!("{}", s1);             // ERROR: value borrowed here after move
println!("{}", s2);                // OK
// When s2 goes out of scope, the String is dropped (heap memory freed).
```

### C++ analogue (sort of)

```cpp
auto s1 = std::make_unique<std::string>("hello");   // s1 owns the string
auto s2 = std::move(s1);                            // ownership moves; s1 is now nullptr
// std::cout << *s1;                                // would crash: s1 is null
std::cout << *s2;                                   // OK
// When s2 goes out of scope, unique_ptr's destructor frees the string.
```

Rust's `let s2 = s1;` for a non-`Copy` type is **equivalent to** C++'s `auto s2 = std::move(s1);`. There is no implicit copy constructor in Rust — moves are the default, and copying must be explicit (via `Clone::clone()` or by being a `Copy` type).

### The `Copy` trait

Small types (integers, floats, bools, raw pointers, fixed-size arrays of `Copy` types) implement the `Copy` trait. `Copy` types are **bitwise-copied** on assignment rather than moved. So:

```rust
let a: i32 = 5;
let b = a;            // copied, not moved
println!("{}", a);    // still works — a was copied, not moved
```

C++ doesn't distinguish this way; every type has an implicitly-declared copy constructor (if its members are copyable), and the compiler chooses copy vs. move based on context.

### The `Drop` trait

When a value goes out of scope, if its type implements `Drop`, the compiler calls `drop()` on it. This is the Rust equivalent of a C++ destructor.

```rust
struct File {
    fd: i32,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }
}

{
    let f = File { fd: 3 };
    // ... use f ...
}   // <- f's Drop is called here; libc::close(3) runs.
```

**C++ equivalent:**

```cpp
struct File {
    int fd;
    ~File() { close(fd); }
};

{
    File f{3};
    // ... use f ...
}   // <- f's destructor runs
```

**In core-foundation-rs, every wrapper implements `Drop` to call `CFRelease`** — exactly analogous to metal-cpp's `NSSharedPtr` destructor calling `release`. So:

```rust
// In core-foundation/src/base.rs (paraphrased):
pub struct CFString(CFStringRef);

impl Drop for CFString {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}
```

When a `CFString` goes out of scope, `CFRelease` runs, the underlying `CFStringRef`'s refcount drops, and if it hits 0 the C side frees the string.

## 1.9 Borrowing and references

A **borrow** is a reference to a value without taking ownership. Borrows come in two flavours:

- `&T` — shared borrow. You can have many of these at once. Read-only.
- `&mut T` — mutable borrow. You can have exactly one. Read-write.

**The borrow checker's rule:** at any moment, a value has *either* one `&mut T` *or* any number of `&T`, never both.

### Rust

```rust
let mut s = String::from("hello");
let r1 = &s;            // shared borrow
let r2 = &s;            // another shared borrow — OK
println!("{} {}", r1, r2);
// r1, r2 are no longer used after this point — the compiler tracks this (NLL)

let r3 = &mut s;        // mutable borrow — OK because r1, r2 are done
r3.push_str(" world");
println!("{}", r3);
```

### C++ equivalent

```cpp
std::string s = "hello";
const std::string& r1 = s;
const std::string& r2 = s;
std::cout << r1 << " " << r2 << "\n";

std::string& r3 = s;
r3 += " world";
std::cout << r3 << "\n";
```

C++ allows this; C++ also allows you to take `std::string& r3 = s;` while `r1` and `r2` are still alive, then mutate `s` through `r3` while `r1`/`r2` are dangling. **That's the bug Rust prevents.** The borrow checker rejects code like:

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;        // ERROR: cannot borrow `s` as mutable because it's also borrowed as immutable
println!("{}", r1);
```

This is the hardest part of Rust for newcomers. Once it clicks, you'll write code that's correct-by-construction and wonder how you ever lived without it.

### Why this matters for core-foundation-rs

The library uses borrows heavily. When you call `array.get(i)` it returns an `ItemRef<'a, T>` — a borrow of an item inside the array, with lifetime `'a` tied to the array's borrow. The compiler enforces that you can't drop the array while you still hold the borrow.

## 1.10 Lifetimes

A **lifetime** is a compile-time tag that records "this reference is valid for at least this long." Lifetimes are how the borrow checker reasons across function boundaries.

### Rust

```rust
// 'a is a generic lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// The signature says: "the returned reference lives at least as long as the shorter of x and y."
```

### C++ analogue (sort of)

C++ doesn't have lifetimes; it has dangling-reference bugs:

```cpp
const std::string& longest(const std::string& x, const std::string& y) {
    return x.size() > y.size() ? x : y;     // looks safe…
}

const std::string& bad() {
    std::string local = "short";
    static std::string other = "longer";
    return longest(local, other);   // returns a reference to `local`, which is destroyed here!
}
```

Rust's lifetime system catches this at compile time:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn bad() -> &'static str {
    let local = String::from("short");
    let other: &'static str = "longer";
    longest(&local, other)    // ERROR: `local` does not live long enough
}
```

You'll see lifetime parameters throughout `core-foundation-rs`. For example, `ItemRef<'a, T>` (from `base.rs`) carries a lifetime `'a` saying "this borrow is tied to the container it came from."

**Lifetime elision:** Rust lets you omit lifetimes in many common patterns. `fn first(v: &Vec<T>) -> &T` is interpreted as `fn first<'a>(v: &'a Vec<T>) -> &'a T` by the elision rules. You'll see this a lot.

## 1.11 Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`

Rust's smart pointers correspond closely to C++'s:

| Rust | C++ | Ownership |
|---|---|---|
| `Box<T>` | `std::unique_ptr<T>` | Single owner, heap |
| `Rc<T>` | (no stdlib equivalent) | Shared owner, single-threaded, refcounted |
| `Arc<T>` | `std::shared_ptr<T>` | Shared owner, atomic refcount (thread-safe) |
| `RefCell<T>` | (no equivalent) | Single-threaded **interior mutability** — runtime borrow check |
| `Mutex<T>` / `RwLock<T>` | `std::mutex<T>` (with `lock()` returning a guard) | Multi-threaded interior mutability |
| `Cell<T>` | (no equivalent) | Single-threaded interior mutability for `Copy` types |
| `Cow<'a, B>` | (no equivalent) | "Borrowed or Owned" — clone-on-write |

### `Box<T>` — single-owner heap pointer

```rust
let b: Box<i32> = Box::new(42);
println!("{}", b);        // Box<T> implements Deref, so it's usable as &T
```

```cpp
auto b = std::make_unique<int>(42);
std::cout << *b << "\n";
```

### `Rc<T>` / `Arc<T>` — shared refcount

```rust
use std::rc::Rc;
let a = Rc::new(42);
let b = Rc::clone(&a);     // increments refcount; both `a` and `b` own the value
// When both go out of scope, refcount hits 0 and the value is dropped.
```

```cpp
auto a = std::make_shared<int>(42);
auto b = a;                // increments refcount
```

`Rc` is **single-threaded** (non-atomic refcount). `Arc` (Atomic Rc) is **thread-safe** (atomic refcount) — same as `std::shared_ptr`.

### `RefCell<T>` — interior mutability

Sometimes you have a value behind an `&T` (immutable borrow) but you need to mutate it. Rust provides "interior mutability" patterns: the borrow check moves from compile-time to runtime.

```rust
use std::cell::RefCell;
let cell = RefCell::new(42);
{
    let mut borrow = cell.borrow_mut();    // panics if already borrowed
    *borrow += 1;
}
println!("{}", cell.borrow());             // 43
```

C++ doesn't really have this; you'd just use a mutable member. The Rust pattern exists so that you can mutate parts of an otherwise-immutable structure.

## 1.12 Error handling: `Option` and `Result` (vs. exceptions)

Rust has no exceptions. Fallible operations return `Option<T>` or `Result<T, E>`. The compiler forces you to handle the failure case.

### Rust

```rust
// Option — for "might not exist"
fn first(v: &[i32]) -> Option<&i32> {
    v.first()
}

match first(&vec![1, 2, 3]) {
    Some(x) => println!("first is {}", x),
    None    => println!("empty!"),
}

// Result — for "might fail with an error"
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

match parse_int("42") {
    Ok(n)  => println!("got {}", n),
    Err(e) => println!("error: {}", e),
}

// The `?` operator propagates errors upward
fn read_config(path: &str) -> Result<Config, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;     // returns Err if read fails
    Ok(parse(&contents))
}
```

### C++ equivalent

```cpp
// C++ has exceptions; the typical pattern
std::optional<int> first(const std::vector<int>& v) {
    if (v.empty()) return std::nullopt;
    return v.front();
}

try {
    int n = std::stoi("42");
    std::cout << "got " << n << "\n";
} catch (const std::exception& e) {
    std::cout << "error: " << e.what() << "\n";
}

// Or with std::expected (C++23) — closest to Result
std::expected<Config, std::error_code> read_config(const std::string& path) {
    auto contents = read_file(path);
    if (!contents) return std::unexpected(contents.error());
    return parse(*contents);
}
```

**Key differences:**

- **No exceptions in Rust.** Errors are values returned via `Result`. You can't "throw and forget" — you have to either handle the `Result` or propagate it with `?`.
- **The `?` operator** is Rust's `try`/`catch` shortcut. `expr?` evaluates to the success value, or immediately returns the error from the enclosing function.
- **Panics (`panic!()`)** are unrecoverable and roughly equivalent to `std::terminate()` or `abort()`. Used for invariant violations, not regular errors.

In `core-foundation-rs`, you'll see `Option<T>` everywhere a Core Foundation function can return `NULL`, and `Result<T, CFError>` for operations that can fail with a `CFError`.

## 1.13 Modules, crates, and visibility

### Rust

```
// File: src/lib.rs
pub mod array;            // pulls in src/array.rs
pub mod string;           // pulls in src/string.rs
mod private;              // private — not visible outside the crate

// File: src/array.rs
pub struct CFArray<T>(...);          // pub — visible outside the crate
fn helper() { ... }                  // private — visible only inside array.rs

pub fn public_fn() {
    helper();                        // OK — same module
    crate::private::thing();         // OK — crate-private
}
```

- A **crate** is the unit of compilation. It produces a library or binary.
- A **module** is a namespace inside a crate. Modules form a tree rooted at the crate root (`lib.rs` for libraries, `main.rs` for binaries).
- A **package** is what Cargo manages; it can contain multiple crates (one library + N binaries).
- `pub` makes an item visible outside its module. Items are private by default.

### C++ equivalent

C++ has **translation units** (one `.cpp` file), **namespaces**, and **headers**. There's no formal concept of "crate" or "module" pre-C++20. C++20 adds modules but they're not widely adopted yet.

```cpp
// File: array.h
namespace mylib {
    class CFArray { ... };     // public — anyone who #includes array.h sees this
}

// File: array.cpp
namespace mylib {
    void helper() { ... }       // has internal linkage — invisible outside this TU
}
```

In `core-foundation-rs`:

- The **workspace** (top-level `Cargo.toml`) contains **8 crates**: `core-foundation-sys`, `core-foundation`, `core-graphics-types`, `core-graphics`, `core-text`, `cocoa`, `cocoa-foundation`, `io-surface`.
- Each crate has its own `Cargo.toml` and `src/lib.rs`.
- `core-foundation` depends on `core-foundation-sys`; `core-graphics` depends on `core-foundation` and `core-graphics-types`; etc.

## 1.14 Macros (vs. C preprocessor)

Rust has **two macro systems**:

1. **Declarative macros** (`macro_rules!`) — pattern-matching expansion, similar in spirit to C preprocessor macros but far more powerful (hygienic, can match complex syntax).
2. **Procedural macros** — actual Rust programs that run at compile time to generate code. Three kinds: `derive` (adds trait implementations), attribute-style (`#[my_macro]`), and function-style.

### Rust

```rust
// Declarative macro
macro_rules! vec_of_strings {
    ( $( $x:expr ),* ) => {
        vec![ String::from($x) ]
    };
}

let v = vec_of_strings!("a", "b", "c");

// derive macro — generates implementations of Debug, Clone
#[derive(Debug, Clone)]
struct Point { x: f64, y: f64 }
```

### C++ equivalent

```cpp
// Preprocessor macro — far less powerful
#define VEC_OF_STRINGS(...) { __VA_ARGS__ }

// No equivalent of #[derive]; you write the boilerplate by hand or use code generators
```

**In `core-foundation-rs`**, the most important macros are `declare_TCFType!`, `impl_TCFType!`, `impl_CFTypeDescription!`, and `impl_CFComparison!` — defined in `core-foundation/src/lib.rs`. They generate the `Drop`, `Clone`, `PartialEq`, `Eq`, `Debug`, and `TCFType` implementations for each wrapper type. Without them, every wrapper would have ~50 lines of boilerplate. With them, it's one line per type.

## 1.15 `unsafe` Rust (and why this codebase needs it)

The Rust compiler's safety guarantees rely on knowing what your code does. **`unsafe` is the escape hatch** for things the compiler can't verify:

- Dereference raw pointers (`*const T` / `*mut T`).
- Call `extern "C"` functions (FFI).
- Access or modify mutable `static` variables.
- Implement `unsafe trait`s or call `unsafe fn`s.

**`unsafe` does NOT turn off the borrow checker.** It just lets you do these specific four things. The rest of the language is still safe.

### Rust

```rust
let x: *const i32 = &42;
unsafe {
    println!("{}", *x);       // OK in unsafe block
}
// println!("{}", *x);        // ERROR: dereference of raw pointer requires unsafe
```

### Why `core-foundation-rs` is full of `unsafe`

Every call into Core Foundation is a C function call. Calling C from Rust requires `unsafe`. So every `CFRelease`, every `CFStringCreateWithCString`, every `CFArrayCreate` — all `unsafe`. The library wraps each in a safe method that upholds Rust's invariants:

```rust
// Safe method wrapping an unsafe FFI call
impl CFString {
    pub fn new(s: &str) -> CFString {
        unsafe {
            let ref_ = CFStringCreateWithCString(
                kCFAllocatorDefault,
                s.as_ptr() as *const c_char,
                kCFStringEncodingUTF8,
            );
            TCFType::wrap_under_create_rule(ref_)     // safe wrapper around the unsafe ref
        }
    }
}
```

The `unsafe { ... }` block is small and audited. Users of `CFString::new` don't see any `unsafe` — they get a safe API. **This is the entire pattern of `core-foundation-rs`.**

## 1.16 FFI: calling C from Rust (and vice versa)

### Declaring a C function

```rust
// Declaring an extern C function (from a linked library)
extern "C" {
    pub fn CFRelease(cf: *const c_void);
}

// Calling it requires unsafe
unsafe { CFRelease(ptr); }
```

### C++ equivalent

```cpp
// Declaring an extern C function
extern "C" void CFRelease(const void* cf);

// No special block needed to call
CFRelease(ptr);
```

### Declaring a struct that matches a C struct

```rust
#[repr(C)]                              // guarantees C-compatible layout
struct CFRange {
    location: isize,
    length: isize,
}
```

```cpp
struct CFRange {
    intptr_t location;
    intptr_t length;
};
```

The `#[repr(C)]` attribute is critical — without it, Rust is free to reorder fields for alignment or padding. With it, the layout matches C exactly, so you can pass the struct across the FFI boundary by pointer.

### Function pointers

```rust
type Comparator = extern "C" fn(
    val1: *const c_void,
    val2: *const c_void,
    context: *mut c_void,
) -> CFComparisonResult;
```

```cpp
using Comparator = CFComparisonResult (*)(const void*, const void*, void*);
```

### Strings

C strings (`*const c_char`, null-terminated) are different from Rust strings. To pass a Rust string to C:

```rust
let s = std::ffi::CString::new("hello").unwrap();
unsafe { some_c_function(s.as_ptr()); }
```

In `core-foundation-rs`, you'll see this pattern everywhere — `CString`, `CStr`, `*const c_char`, and `*const c_void`.

## 1.17 The Cargo build system (vs. CMake/Make)

A Rust project is described by `Cargo.toml`:

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
libc = "0.2"

[features]
default = ["foo"]
foo = []
```

Commands:

- `cargo build` — compile
- `cargo run` — compile + run
- `cargo test` — run unit tests
- `cargo doc --open` — generate HTML docs and open in browser
- `cargo fmt` — auto-format
- `cargo clippy` — lints

C++ analogue: CMake (with `CMakeLists.txt`) or Meson or Bazel. Cargo is generally simpler and more uniform.

In `core-foundation-rs`, the top-level `Cargo.toml` declares a **workspace** of 8 crates. Each subcrate has its own `Cargo.toml` that depends on others via `path = "..."`. Dependencies between crates use `workspace = true` to inherit version info.

---

# Part 2 — The 200-Day Study Plan

## 2.1 The seven phases at a glance

The 200 days are split into **seven phases**. The first phase is pure Rust learning; the remaining six read the codebase crate by crate, with a final phase for the 100 code examples.

| Phase | Days | Files | What you'll be able to do at the end |
|---|---|---|---|
| **1. Rust primer + setup** | 1–20 | This file, in full. | Read simple Rust code and explain in plain English how each construct maps to C++. |
| **2. `core-foundation-sys/`** | 21–55 | 40 FFI files (~5,600 lines). | Explain Rust FFI: `extern "C"`, `#[repr(C)]`, raw pointers, type aliases. |
| **3. `core-foundation/`** | 56–105 | 22 safe-wrapper files (~3,800 lines). The **heart** of the library. | Explain traits, generics, lifetimes, `Drop`, `unsafe` containment, and the macro system that generates per-type boilerplate. |
| **4. `core-graphics/` + types** | 106–140 | 20 files (~4,300 lines). | Apply the patterns from Phase 3 to a new domain (graphics). Explain `CGFloat`, `CGPoint`, `CGRect`, contexts, paths, colors. |
| **5. `core-text/`** | 141–165 | 10 files (~2,400 lines). | Read a domain-specific binding (text layout). Explain fonts, font descriptors, frames, lines, runs. |
| **6. `cocoa/` + cocoa-foundation + io-surface** | 166–185 | 14 files (~9,000 lines, includes examples). | Read higher-level Cocoa bindings and learn from runnable example programs (`hello_world.rs`, `color.rs`, etc.). |
| **7. 100 code examples + capstone** | 186–200 | File 7 of this guide (100 examples). | Write small Rust programs that use `core-foundation-rs`. |

> **Pacing note.** 30–60 minutes per day. Don't binge. The brain consolidates during sleep, not during 8-hour marathons.

## 2.2 Phase 1 (Days 1–20): Rust primer + setup

| Day | Read | Notes |
|---|---|---|
| 1 | This file §1.1 (the 10,000-foot view). | Don't open any code yet. Get the mental model. |
| 2 | This file §1.2 (variables, types, mutability). | Install Rust if you haven't: <https://rustup.rs>. |
| 3 | This file §1.3 (functions and control flow). Write a 10-line Rust program. | `cargo new hello && cd hello && cargo run`. |
| 4 | This file §1.4 (structs). Write a `Point` struct with an `add` method. | Compare your Rust `Point` to a C++ `Point` you'd write. |
| 5 | This file §1.5 (enums and match). Write a `Shape` enum and an `area` function. | Pattern matching is Rust's best feature. Spend an extra day if needed. |
| 6 | This file §1.6 (traits). Write a `Display`-like trait for `Point` and `Shape`. | Traits are how Rust does what C++ does with virtual functions and templates. |
| 7 | This file §1.7 (generics). Write a generic `fn first<T>(v: &[T]) -> Option<&T>`. | |
| 8 | This file §1.8 (ownership). Move a `String` between variables; observe the compile error when you try to use the moved-from one. | This is the concept C++ developers struggle with most. Spend two days if needed. |
| 9 | Re-read §1.8. Write `Box::new(42)`, move it, observe. | |
| 10 | This file §1.9 (borrowing). Try to write code that violates the borrow rule; observe the compiler errors. | |
| 11 | This file §1.10 (lifetimes). Read once. Don't expect to fully understand yet. | Lifetimes click after you've seen them in real code. We'll revisit in Phase 3. |
| 12 | This file §1.11 (smart pointers). Write a `Rc<RefCell<Vec<i32>>>`. Add and remove items. | The `Rc<RefCell<T>>` pattern is the single-threaded shared-mutable-state pattern in Rust. |
| 13 | This file §1.12 (Option and Result). Write a `parse_int(s: &str) -> Result<i32, ()>` and call it with `?`. | Error handling without exceptions takes practice. |
| 14 | This file §1.13 (modules and crates). Create a two-crate workspace: a library and a binary that depends on it. | |
| 15 | This file §1.14 (macros). Read once. | You'll see `macro_rules!` macros in Phase 3 — don't worry about writing your own yet. |
| 16 | This file §1.15 (`unsafe`). Write a tiny program that dereferences a raw pointer inside `unsafe { }`. | |
| 17 | This file §1.16 (FFI). Read once. | This is the bridge `core-foundation-rs` is built on. |
| 18 | This file §1.17 (Cargo). `cargo new`, `cargo build`, `cargo test`, `cargo doc --open`. | |
| 19 | Phase-1 review, day 1. Re-skim §1.1 through §1.8. | |
| 20 | Phase-1 review, day 2. Write down the 5 things you remember from §1.1 without looking. | If you can't, re-read §1.1. |

## 2.3 Phase 2 (Days 21–55): `core-foundation-sys/` — the FFI layer

This crate is **40 files** of mostly type aliases and `extern "C"` function declarations. It's the closest thing to a C header file in the Rust world. The pattern is mechanical, so each file takes 10–15 minutes once you've seen a few.

| Day | Read | Notes |
|---|---|---|
| 21 | File 2 of this guide, intro + `core-foundation-sys/src/lib.rs` (~80 lines). | The crate root. Mostly `pub mod` declarations. |
| 22 | File 2 § on `base.rs` (198 lines). | The most important file — defines `CFTypeRef`, `CFIndex`, `CFRange`, `TCFTypeRef` trait, `CFAllocatorContext`, all the callback function-pointer types. |
| 23 | Re-read `base.rs`. Trace `TCFTypeRef` (the trait that lets raw pointers convert to/from `*const c_void`). | |
| 24 | File 2 § on `string.rs`. | Defines `CFStringRef`, encoding constants, `CFStringCreate*`, `CFStringGet*`. |
| 25 | File 2 § on `array.rs`. | `CFArrayRef`, `CFArrayCreate`, `CFArrayGetCount`, `CFArrayGetValueAtIndex`. |
| 26 | File 2 § on `dictionary.rs`. | `CFDictionaryRef`, key/value callbacks. |
| 27 | File 2 § on `set.rs` and `bag.rs`. | Two small variations on the dictionary pattern. |
| 28 | File 2 § on `data.rs`. | `CFDataRef` — raw byte buffer. |
| 29 | File 2 § on `number.rs` and `boolean.rs`. | Boxed scalars. |
| 30 | File 2 § on `date.rs`. | Timestamps. |
| 31 | File 2 § on `error.rs`. | `CFErrorRef` — analogous to `NSError`. |
| 32 | File 2 § on `url.rs`. | `CFURLRef`. |
| 33 | File 2 § on `bundle.rs`. | App bundles. |
| 34 | File 2 § on `runloop.rs` and `mach_port.rs`. | Event loops + Mach ports. |
| 35 | File 2 § on `filedescriptor.rs`. | File-descriptor-based run-loop sources. |
| 36 | File 2 § on `characterset.rs` and `bit_vector.rs`. | Two small collection types. |
| 37 | File 2 § on `timezone.rs`, `calendar.rs`, `locale.rs`. | Date/time utilities. |
| 38 | File 2 § on `propertylist.rs` and `preferences.rs`. | Property lists + user defaults. |
| 39 | File 2 § on `notification_center.rs`, `messageport.rs`, `socket.rs`, `stream.rs`. | IPC primitives. |
| 40 | File 2 § on `attributed_string.rs`, `string_tokenizer.rs`. | String utilities. |
| 41 | File 2 § on `tree.rs`, `binary_heap.rs`. | Less-common collection types. |
| 42 | File 2 § on `xml_node.rs`, `xml_parser.rs`. | XML support. |
| 43 | File 2 § on `date_formatter.rs`, `number_formatter.rs`. | Formatters. |
| 44 | File 2 § on `plugin.rs`, `user_notification.rs`, `url_enumerator.rs`, `file_security.rs`. | Remaining small files. |
| 45 | File 2 § on `uuid.rs`. | UUIDs. |
| 46–50 | Re-skim the whole crate. | By now you should be able to look at any `extern "C" fn` declaration and know what it does. |
| 51–55 | Phase-2 capstone. From memory, write the FFI declaration for `CFArrayCreate` — just the Rust signature. Compare against the source. | You should be able to write it. |

## 2.4 Phase 3 (Days 56–105): `core-foundation/` — the safe wrapper

**This is the most important phase of the whole 200 days.** The `core-foundation/` crate is the safe Rust wrapper around the unsafe FFI from Phase 2. Every Rust idiom worth knowing shows up here.

| Day | Read | Notes |
|---|---|---|
| 56 | File 3 intro. Read `core-foundation/src/lib.rs` (260 lines). | Defines `ConcreteCFType` trait + four macros (`declare_TCFType!`, `impl_TCFType!`, `impl_CFTypeDescription!`, `impl_CFComparison!`). **Conceptual heart of the library.** |
| 57 | Re-read `lib.rs`. The macros are hard; spend a second day. | Trace `impl_TCFType!` expansion by hand for a simple type. |
| 58 | File 3 § on `base.rs` (446 lines). The `TCFType` trait, `CFType` struct, downcast, `ItemRef`/`ItemMutRef`, `FromVoid`/`ToVoid`. | The most important file in the whole library. |
| 59 | Re-read `base.rs`. | |
| 60 | File 3 § on `string.rs` (210 lines). `CFString`, `From<&str>`, `FromStr`, `Cow<'a, str>` conversion, `Display` impl. | See how a safe wrapper exposes Rust idioms (From, FromStr, Display) built on unsafe FFI calls. |
| 61 | Re-read `string.rs`. | |
| 62 | File 3 § on `array.rs` (310 lines). `CFArray<T>` — generic over element type. `CFArrayIterator`, `ExactSizeIterator`, `from_copyable`, `from_CFTypes`. | A great example of generics + lifetimes + iterators in Rust. |
| 63 | Re-read `array.rs`. Try to write the iterator's `next()` method from memory. | |
| 64 | File 3 § on `dictionary.rs`. | Similar pattern to `CFArray` but for key/value pairs. |
| 65 | File 3 § on `set.rs`. | Unordered unique collection. |
| 66 | File 3 § on `data.rs`. | Raw byte buffer. |
| 67 | File 3 § on `number.rs` and `boolean.rs`. | Boxed scalars. |
| 68 | File 3 § on `date.rs`. | Timestamps. |
| 69 | File 3 § on `error.rs`. | `CFError` — wrap `CFErrorRef` and expose as `Result`. |
| 70 | File 3 § on `url.rs`. | URLs. |
| 71 | File 3 § on `bundle.rs`. | App bundles. |
| 72 | File 3 § on `runloop.rs` and `mach_port.rs`. | Event loops. |
| 73 | File 3 § on `filedescriptor.rs`. | File descriptor sources. |
| 74 | File 3 § on `characterset.rs`. | Character sets. |
| 75 | File 3 § on `propertylist.rs`. | Property lists — a great example of an enum-based wrapper. |
| 76 | File 3 § on `timezone.rs`. | Timezones. |
| 77 | File 3 § on `uuid.rs`. | UUIDs (feature-gated). |
| 78 | File 3 § on `attributed_string.rs`. | Strings with attributes. |
| 79 | Phase-3 review, day 1. Re-read §3.1–§3.5 (the foundational files). | |
| 80 | Phase-3 review, day 2. Pick one wrapper file and write its skeleton from memory: the struct, the `Drop`, the `TCFType` impl, the `declare_TCFType!`/`impl_TCFType!` macros. | |
| 81–85 | Re-skim the whole crate. | You should now be able to read any wrapper file in 5 minutes. |
| 86–90 | Trace one method end-to-end. Pick `CFString::new("hello")` and follow the call: safe method → `unsafe { CFStringCreateWithCString(...) }` → `TCFType::wrap_under_create_rule(ref_)` → struct construction. | |
| 91–95 | Pick `CFArray::from_CFTypes(&[...])` and trace it the same way. Note how the `Drop` for the array calls `CFRelease`, which transitively releases each element. | |
| 96–100 | Phase-3 capstone, week 1. From memory, write the trait `TCFType`'s signature (just the method names, not bodies). | |
| 101–105 | Phase-3 capstone, week 2. Explain in writing, without looking at the source, what `wrap_under_get_rule` vs. `wrap_under_create_rule` mean and why both exist. | (Hint: it's the Rust analogue of `RetainPtr` vs. `TransferPtr` from the metal-cpp guide.) |

## 2.5 Phase 4 (Days 106–140): `core-graphics/` + `core-graphics-types/`

Two crates, 20 files total. Same patterns as Phase 3, applied to a new domain.

| Day | Read | Notes |
|---|---|---|
| 106 | File 4 intro. Read `core-graphics-types/src/lib.rs` (small). | The crate that defines `CGFloat`, `CGPoint`, `CGSize`, `CGRect`, `CGAffineTransform`. |
| 107 | File 4 § on `core-graphics-types/src/geometry.rs`. | Geometric primitives. Note `#[repr(C)]` and the `derive(Debug, Clone, Copy)` — these are plain old data types. |
| 108 | File 4 § on `core-graphics/src/base.rs`. | FFI type aliases for CG. |
| 109 | File 4 § on `core-graphics/src/lib.rs`. | Crate root. |
| 110 | File 4 § on `core-graphics/src/color.rs`. | `CGColor`. |
| 111 | File 4 § on `core-graphics/src/color_space.rs`. | `CGColorSpace`. |
| 112 | File 4 § on `core-graphics/src/context.rs`. | `CGContext` — the most important CG type. Drawing happens here. |
| 113–115 | Re-read `context.rs`. It's the biggest file in the crate. | |
| 116 | File 4 § on `core-graphics/src/path.rs`. | `CGPath` — vector paths. |
| 117 | File 4 § on `core-graphics/src/image.rs`. | `CGImage`. |
| 118 | File 4 § on `core-graphics/src/data_provider.rs`. | `CGDataProvider` — feeds data into images. |
| 119 | File 4 § on `core-graphics/src/font.rs`. | `CGFont`. |
| 120 | File 4 § on `core-graphics/src/gradient.rs`. | `CGGradient`. |
| 121 | File 4 § on `core-graphics/src/display.rs`. | Display enumeration. |
| 122 | File 4 § on `core-graphics/src/event.rs` and `event_source.rs`. | Event handling. |
| 123 | File 4 § on `core-graphics/src/window.rs`. | `CGWindow`. |
| 124 | File 4 § on `core-graphics/src/sys.rs`. | System-level bindings. |
| 125 | File 4 § on `core-graphics/src/access.rs`. | Accessibility constants. |
| 126–130 | Phase-4 review. Re-skim the crate. | |
| 131–140 | Phase-4 capstone. Write a small Rust program that creates a `CGContext`, draws a red rectangle, and saves the result as a PNG. | You'll need to combine `core-graphics` with `image` crate. |

## 2.6 Phase 5 (Days 141–165): `core-text/`

10 files, ~2,400 lines. Domain-specific: text layout.

| Day | Read | Notes |
|---|---|---|
| 141 | File 5 intro. Read `core-text/src/lib.rs`. | Crate root. |
| 142 | File 5 § on `font.rs`. | `CTFont`. |
| 143 | File 5 § on `font_descriptor.rs`. | `CTFontDescriptor`. |
| 144 | File 5 § on `font_collection.rs`. | `CTFontCollection`. |
| 145 | File 5 § on `font_manager.rs`. | Font manager utilities. |
| 146 | File 5 § on `string_attributes.rs`. | String attribute constants. |
| 147 | File 5 § on `line.rs`. | `CTLine` — a laid-out line of text. |
| 148 | File 5 § on `run.rs`. | `CTRun` — a run of text with the same style. |
| 149 | File 5 § on `frame.rs`. | `CTFrame` — a laid-out frame. |
| 150 | File 5 § on `framesetter.rs`. | `CTFramesetter` — creates frames from attributed strings. |
| 151–155 | Phase-5 review. | |
| 156–165 | Phase-5 capstone. Write a small program that loads a font, lays out "Hello, world!" using Core Text, and renders it to a `CGContext`. |

## 2.7 Phase 6 (Days 166–185): `cocoa/` + `cocoa-foundation/` + `io-surface/`

14 files total, ~9,000 lines. This includes example programs that are gold for learning.

| Day | Read | Notes |
|---|---|---|
| 166 | File 6 intro. Read `cocoa-foundation/src/lib.rs` and `base.rs`. | Cocoa-foundation is a small shim. |
| 167 | File 6 § on `cocoa-foundation/src/foundation.rs`. | Cocoa-specific Foundation extensions. |
| 168 | File 6 § on `io-surface/src/lib.rs` (233 lines). | `IOSurface` — shared GPU/CPU memory surfaces. |
| 169 | File 6 § on `cocoa/src/lib.rs`. | The cocoa crate root. |
| 170 | File 6 § on `cocoa/src/appkit.rs` (massive — ~5,000 lines). | AppKit bindings: `NSWindow`, `NSView`, `NSApplication`, etc. Skim; don't memorise every method. |
| 171–175 | Continue `appkit.rs` in chunks. | |
| 176 | File 6 § on `cocoa/src/quartzcore.rs`. | QuartzCore / Core Animation bindings. |
| 177 | File 6 § on `cocoa/src/macros.rs`. | `msg_send!` macro — how cocoa calls Objective-C. |
| 178 | **Read** `cocoa/examples/hello_world.rs`. | A complete runnable Cocoa program in Rust. The best learning artifact in the whole project. |
| 179 | **Read** `cocoa/examples/color.rs`. | Another runnable example. |
| 180 | **Read** `cocoa/examples/fullscreen.rs`. | Fullscreen window. |
| 181 | **Read** `cocoa/examples/tab_view.rs`. | Tab views. |
| 182 | **Read** `cocoa/examples/nsvisualeffectview_blur.rs`. | Visual effect view (blur). |
| 183–185 | Phase-6 review. Compile and run `hello_world.rs` if you have a Mac. | |

## 2.8 Phase 7 (Days 186–200): The 100 code examples + capstone

| Day | Read | Notes |
|---|---|---|
| 186 | File 7 — Examples 1–10 (Rust fundamentals). | These reinforce Phase 1. |
| 187 | File 7 — Examples 11–20 (FFI basics). | |
| 188 | File 7 — Examples 21–35 (`core-foundation-sys` patterns). | |
| 189 | File 7 — Examples 36–55 (`core-foundation` safe wrappers). | |
| 190 | File 7 — Examples 56–65 (ownership and lifetimes). | |
| 191 | File 7 — Examples 66–75 (traits and generics). | |
| 192 | File 7 — Examples 76–85 (error handling). | |
| 193 | File 7 — Examples 86–95 (graphics + text). | |
| 194 | File 7 — Examples 96–100 (cocoa, putting it together). | |
| 195 | File 8 — Appendix A (recurring patterns). | |
| 196 | File 8 — Appendix B (glossary). | |
| 197 | File 8 — Appendix C (where to go next). | |
| 198 | Capstone: pick one example from File 7 and modify it. | E.g. make `hello_world.rs` say "Hello, Rust!" instead of "Hello, World!". |
| 199 | Capstone: write a small original program that uses `CFString`, `CFArray`, and `CFDictionary`. | |
| 200 | Final review. Re-read this file (File 1) end-to-end. | You're done. |

## 2.9 How to study each day

A productive 30–60 minute session:

1. **Recall yesterday (5 min).** Without opening anything, say out loud what you read yesterday. If you can't, re-read yesterday's section first.
2. **Read today's source file once, cold (10–20 min).** Don't take notes. Just let the shape sink in.
3. **Read the matching section in this guide (10–15 min).** Now the bits that confused you should click.
4. **Read the source file a second time (10–15 min).** With the commentary in mind, the code should feel different.
5. **Write a 3-sentence summary in your own notes (5 min).** Anywhere — notebook, markdown file, anything.
6. **(Optional, weekends) Code experiment.** Modify the source, see if it still compiles.

If you fall behind, **don't binge**. Resume at the next day's slot. Spaced repetition produces learning; binging produces the illusion of learning.

## 2.10 Self-check questions to use throughout

After finishing any file, you should be able to answer these without looking:

1. **What crate is this file in?** (`core-foundation-sys` vs. `core-foundation` vs. `core-graphics` etc.)
2. **Is this file mostly `extern "C"` declarations (FFI), or is it a safe wrapper?**
3. **What's the unsafe boundary?** Where does `unsafe { ... }` appear, and what does it protect?
4. **What Rust traits does the file's main type implement?** (`TCFType`, `Drop`, `Clone`, `Debug`, etc.)
5. **What's the C++ analogue of the main type?** (E.g. `CFString` ↔ `std::string` or `NSString*`.)
6. **What lifetime parameters appear, and what do they tie?** (E.g. `ItemRef<'a, T>` ties the borrow to the container.)
7. **What macros does the file use?** (`declare_TCFType!`, `impl_TCFType!`, etc.)
8. **Could I write the skeleton of this file from memory?**

If you can answer all eight, move on.

---

## Final notes before you start

You're about to spend 200 days with this codebase. That's serious. Three pieces of advice:

1. **The hard part of Rust is ownership and borrowing (§1.8–§1.10).** Everything else is syntax. If you find yourself stuck on borrow-checker errors in Phase 3, come back to §1.8–§1.10 and re-read. It will click.

2. **The `unsafe` boundary is the most important architectural concept in this library.** Every file in `core-foundation/` is structured as: safe API → small `unsafe` block → FFI call → safe wrapper around the result. Once you see this pattern, you'll see it everywhere in Rust FFI code, not just here.

3. **The macros are harder than the code.** `declare_TCFType!` and `impl_TCFType!` (covered in File 3) are declarative macros that generate `Drop`, `Clone`, `PartialEq`, `Eq`, and `TCFType` implementations. They look intimidating; once you trace one expansion by hand, they become readable.

Good luck. Open File 2 when you're ready to start Phase 2 (Day 21).

— End of File 1 —
