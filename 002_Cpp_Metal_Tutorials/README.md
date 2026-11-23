# apple 공식 코드
- https://github.com/apple/metal-cpp

# (영상모아보기xcode이용) macOS Metal with C++ | GetIntoGameDev
- https://youtube.com/playlist?list=PLn3eTxaOtL2N8v1zUWS1fxaQVI3L3wG0w&si=5KTgn9iUgWiXSgBh

# C++코드
- https://metaltutorial.com/
  - This tutorial will teach you the basics of Apple's Metal Graphics and Compute API, and help you understand how to program with it in C++ via the metal-cpp library that Apple has now officially released. 
  - https://github.com/wmarti/MetalTutorial
  - https://developer.apple.com/metal/cpp/

# metal-cpp: A 100-Day Line-by-Line Study Guide

> **Audience:** A learner who has some C++ background but is new to Apple's Metal / GPU programming.
> **Source code under study:** `metal-cpp-main.zip` — Apple's C++ wrapper for the Metal GPU framework (the MTL4 generation).
> **Goal:** Read every header in the project, understand what each section does, and build a durable mental model of how the library bridges C++ to Objective-C and to the GPU.
> **Pace:** ~100 days, ~30–60 minutes per day. Treat it like learning a foreign language: short, consistent daily sessions beat occasional long ones.

---

## Table of Contents

**Part 1 — Introduction**
- 1.1 What is metal-cpp?
- 1.2 What is Metal, and why does Apple ship a C++ wrapper for it?
- 1.3 The big idea: how metal-cpp talks to Objective-C without you writing Objective-C
- 1.4 Concepts you should be comfortable with before Day 1
- 1.5 How to read each "file walkthrough" section in this guide
- 1.6 A 5-minute tour of the whole repository

**Part 2 — The 100-Day Study Plan**
- 2.1 The five phases at a glance
- 2.2 Phase 1 (Days 1–20): Setup & Foundation scaffolding
- 2.3 Phase 2 (Days 21–45): Foundation containers, errors, numbers, bundles
- 2.4 Phase 3 (Days 46–65): Metal descriptors — the small building blocks
- 2.5 Phase 4 (Days 66–85): Metal command infrastructure (queues, buffers, encoders, compiler)
- 2.6 Phase 5 (Days 86–100): Advanced Metal — compute pipelines & acceleration structures
- 2.7 How to study each day
- 2.8 Self-check questions to use throughout

**Part 3 — Foundation Files (23 files, in learning order)**
- 3.1 `Foundation/Foundation.hpp` — the umbrella
- 3.2 `Foundation/NSDefines.hpp` — the macro vocabulary
- 3.3 `Foundation/NSTypes.hpp` — `Integer`, `UInteger`, `TimeInterval`
- 3.4 `Foundation/NSObjCRuntime.hpp` — `ComparisonResult`, `NotFound`
- 3.5 `Foundation/NSRange.hpp` — a simple struct with methods
- 3.6 `Foundation/NSObject.hpp` — **the root of everything** (read carefully)
- 3.7 `Foundation/NSSharedPtr.hpp` — RAII wrapper for retain/release
- 3.8 `Foundation/NSPrivate.hpp` — selectors and class lookups
- 3.9 `Foundation/NSString.hpp`
- 3.10 `Foundation/NSArray.hpp`
- 3.11 `Foundation/NSData.hpp`
- 3.12 `Foundation/NSDictionary.hpp`
- 3.13 `Foundation/NSSet.hpp`
- 3.14 `Foundation/NSEnumerator.hpp`
- 3.15 `Foundation/NSError.hpp`
- 3.16 `Foundation/NSNumber.hpp` (and `NSValue`)
- 3.17 `Foundation/NSLock.hpp`
- 3.18 `Foundation/NSNotification.hpp`
- 3.19 `Foundation/NSDate.hpp`
- 3.20 `Foundation/NSAutoreleasePool.hpp`
- 3.21 `Foundation/NSURL.hpp`
- 3.22 `Foundation/NSBundle.hpp`
- 3.23 `Foundation/NSProcessInfo.hpp`

**Part 4 — Metal Files (22 files, in learning order)**
- 4.1 `Metal/MTL4FunctionDescriptor.hpp`
- 4.2 `Metal/MTL4LibraryFunctionDescriptor.hpp`
- 4.3 `Metal/MTL4BinaryFunctionDescriptor.hpp`
- 4.4 `Metal/MTL4LibraryDescriptor.hpp`
- 4.5 `Metal/MTL4BinaryFunction.hpp`
- 4.6 `Metal/MTL4LinkingDescriptor.hpp`
- 4.7 `Metal/MTL4ArgumentTable.hpp`
- 4.8 `Metal/MTL4Counters.hpp`
- 4.9 `Metal/MTL4Archive.hpp`
- 4.10 `Metal/MTL4CommandAllocator.hpp`
- 4.11 `Metal/MTL4CommitFeedback.hpp`
- 4.12 `Metal/MTL4CompilerTask.hpp`
- 4.13 `Metal/MTL4Compiler.hpp`
- 4.14 `Metal/MTL4CommandEncoder.hpp`
- 4.15 `Metal/MTL4MachineLearningCommandEncoder.hpp`
- 4.16 `Metal/MTL4MachineLearningPipeline.hpp`
- 4.17 `Metal/MTL4CommandBuffer.hpp`
- 4.18 `Metal/MTL4ComputePipeline.hpp`
- 4.19 `Metal/MTL4ComputeCommandEncoder.hpp`
- 4.20 `Metal/MTL4CommandQueue.hpp`
- 4.21 `Metal/MTL4MeshRenderPipeline.hpp` (empty placeholder)
- 4.22 `Metal/MTL4AccelerationStructure.hpp` — the giant (1,395 lines)

**Part 5 — Appendices**
- A. The recurring code patterns you will see in every file
- B. Glossary of terms
- C. Where to go next

---

# Part 1 — Introduction

## 1.1 What is metal-cpp?

`metal-cpp` is a **single-header C++ library published by Apple** that exposes Apple's **Metal** GPU framework to plain C++ code. It lets you write a renderer or compute pipeline in C++ without ever touching Objective-C (`.m` / `.mm`) source files.

Inside the zip you'll find two folders:

```
metal-cpp-main/
├── Foundation/      ← C++ wrappers for the Foundation classes (NSString, NSArray, …)
│   └── 23 .hpp files
├── Metal/           ← C++ wrappers for the Metal GPU API (MTL4 generation)
│   └── 22 .hpp files
└── LICENSE.txt      ← Apache 2.0
```

Total: **45 header files**, roughly **8,500 lines of code**. There is no `.cpp` file to compile — everything is `inline` template code in headers, so adding metal-cpp to a project is just a matter of `#include`-ing one umbrella header.

## 1.2 What is Metal, and why does Apple ship a C++ wrapper for it?

**Metal** is Apple's official low-level GPU API, introduced in 2014 for iOS and 2015 for macOS. It is the Apple-platform alternative to Vulkan / Direct3D / OpenGL. You use Metal to:

- **Render graphics**: draw triangles into textures (games, UI, 3D apps).
- **Run compute shaders**: general-purpose parallel computation on the GPU (image processing, simulations, ML inference, ray tracing).
- **Ray-trace**: starting with Metal 3, hardware-accelerated ray tracing via acceleration structures.

Metal's native API is **Objective-C**. The class names start with prefixes `NS` (Foundation, e.g. `NSString`) and `MTL` (Metal, e.g. `MTLDevice`). Apple's frameworks are *implemented* as Objective-C, so any wrapper must at some point call into the Objective-C runtime.

Apple ships `metal-cpp` for developers who:

- Want to write a cross-platform engine in C++ and use Metal only on Apple platforms.
- Don't want `.mm` (Objective-C++) files mixed into their build.
- Want strong type safety (templates) and RAII without manual `retain`/`release`.

The MTL4 generation you have here is the newest version (copyright `2020-2025`), corresponding to the **Metal 4** API layer introduced with macOS 26 / iOS 26. The classes live in the `MTL4::` namespace and supplement (rather than replace) the older `MTL::` classes.

## 1.3 The big idea: how metal-cpp talks to Objective-C without you writing Objective-C

This is the single most important concept in the whole codebase. If you understand this, every file becomes readable.

An Objective-C method call:

```objc
// Objective-C
NSString* s = [NSString stringWithCString:"hello" encoding:NSUTF8StringEncoding];
NSUInteger len = [s length];
```

is implemented under the hood as a C function call:

```c
// What the compiler turns the above into
id cls  = objc_lookUpClass("NSString");
SEL sel  = sel_registerName("stringWithCString:encoding:");
NSString* s = ((NSString* (*)(id, SEL, const char*, NSUInteger))objc_msgSend)(
                  cls, sel, "hello", 4 /* NSUTF8StringEncoding */);

SEL lenSel = sel_registerName("length");
NSUInteger len = ((NSUInteger (*)(id, SEL))objc_msgSend)(s, lenSel);
```

The function `objc_msgSend` is the **heart of the Objective-C runtime**. Every Objective-C method call eventually goes through it. It takes a receiver object (or class), a **selector** (`SEL`, essentially an interned string naming the method), and the method's arguments, then dispatches to the right implementation.

`metal-cpp` works by:

1. **Looking up** each Objective-C class once (e.g. `objc_lookUpClass("NSString")`) and storing the pointer in a global variable.
2. **Registering** each selector once (e.g. `sel_registerName("stringWithCString:encoding:")`) and storing the `SEL` in a global variable.
3. **Wrapping** every method as a tiny C++ `inline` function that calls `objc_msgSend` with those cached values.

So when you write:

```cpp
NS::String* s = NS::String::string("hello", NS::UTF8StringEncoding);
```

the C++ method `NS::String::string(...)` is actually an inline function that expands to roughly:

```cpp
return Object::sendMessage<NS::String*>(
    Private::Class::s_kNSString,           // cached pointer to the NSString class
    Private::Selector::s_kstringWithCString_encoding_,  // cached SEL
    pString, encoding);
```

which in turn calls `objc_msgSend`.

The same trick — *cache classes and selectors at startup, then use them via `objc_msgSend` from inline C++ functions* — is used in **every** file in this library. Once you've read one wrapper file (say `NSString.hpp`), you've essentially read them all. The rest is just vocabulary.

## 1.4 Concepts you should be comfortable with before Day 1

You don't need to be a C++ expert, but you should at least recognise the following. If any of these are alien, spend Day 1–2 on a C++ primer (see Appendix C) before diving in.

| Concept | Where it shows up | Quick reminder |
|---|---|---|
| `namespace` | `namespace NS { ... }`, `namespace MTL4 { ... }` | Groups names so they don't collide. `NS::String` is the `String` inside the `NS` namespace. |
| `class` / `struct` | everywhere | User-defined types. In C++ they're nearly identical; only difference is default member visibility (`class`→private, `struct`→public). |
| Templates | `template <class _Class> class SharedPtr { ... };` | "Generic" types. `SharedPtr<NS::String>` is a `SharedPtr` specialised for `NS::String`. |
| `inline` functions | every method body | Tells the compiler it's OK to emit the function body in multiple translation units (must be defined in a header). |
| `static_cast<T>(x)` | `_NS_CAST_TO_UINT(value)` | C++ cast — converts between related types. Safer than C-style `(T)x`. |
| `reinterpret_cast<T>(x)` | bridging casts | Reinterpret bits as a different type. Used to "bridge" between `objc_object*` and `NS::Object*`. |
| `__attribute__((...))` | `_NS_EXPORT`, `_NS_PACKED`, `always_inline` | GCC/Clang compiler-specific attributes. E.g. `visibility("default")` controls whether a symbol is exported from the library. |
| `enum class` / plain `enum` | `_NS_ENUM`, `_NS_OPTIONS` | Named integer constants. `_NS_ENUM` is for mutually exclusive values; `_NS_OPTIONS` is for bit-flags you can OR together. |
| RAII | `NS::SharedPtr<T>` dtor calls `release()` | "Resource Acquisition Is Initialization": clean up in destructors so you can't forget. |
| `friend class` | `Object`'s private constructors | A `friend` can access private members of the class that declares it as a friend. |
| `constexpr` | `doesRequireMsgSendStret()` | "Constant expression": computed by the compiler, not at runtime. |
| variadic templates (`template<typename... _Args>`) | `sendMessage<_Ret>(_Args... args)` | A template that takes any number of type parameters. Used so a single function can wrap any selector signature. |

You will also meet a few Objective-C runtime concepts:

- **`objc_object`** — the C-level type representing any Objective-C object. Every `NS::Object` ultimately *is* an `objc_object*` reinterpreted.
- **`SEL`** — a "selector". An interned C string naming a method (e.g. `"length"`, `"initWithString:"`).
- **`objc_msgSend`** — the dispatcher. Plain C function from `<objc/message.h>`.
- **`objc_lookUpClass(name)`** — looks up an Objective-C class by name; returns its `Class` object.
- **`sel_registerName(name)`** — returns the `SEL` for a given method-name string.
- **Reference counting** — every `NSObject` has a `retainCount`. `retain` increments it, `release` decrements it, and when it hits 0 the object is deallocated. `autorelease` defers a `release` to the next drain of the current autorelease pool.

## 1.5 How to read each "file walkthrough" section in this guide

Every file walkthrough follows the same shape:

```
### 3.x <filename> — <one-line summary>

**Path:** <where it lives in the zip>
**Lines:** <approx line count>
**Why it matters:** <when/why you'd care>

#### Section 1: <name>     (file lines A–B)
<explanation, with code snippets copied from the file>

#### Section 2: <name>     (file lines C–D)
<explanation>
...
```

I group related lines into "sections" instead of explaining literally every `}`. For each section I will:

- Quote the actual code (sometimes abbreviated with `…` if a block is repeated).
- Explain **what** it does.
- Explain **why** it's written that way.
- Point out **gotchas** a beginner might miss.

When you read the file yourself, **keep the source file open in another window** and follow along line by line. That's the whole point of this guide: it's a commentary you read *alongside* the code, not a substitute for it.

## 1.6 A 5-minute tour of the whole repository

| File | LOC | One-sentence summary |
|---|---|---|
| `Foundation/Foundation.hpp` | 47 | Umbrella header — just includes everything else. |
| `Foundation/NSDefines.hpp` | 46 | Macro vocabulary (`_NS_EXPORT`, `_NS_ENUM`, …). |
| `Foundation/NSTypes.hpp` | 52 | Type aliases: `Integer`, `UInteger`, `TimeInterval`, `OperatingSystemVersion`. |
| `Foundation/NSObjCRuntime.hpp` | 44 | `ComparisonResult` enum, `NotFound` constant. |
| `Foundation/NSRange.hpp` | 84 | A `{location, length}` struct with helper methods. |
| `Foundation/NSObject.hpp` | 302 | **The root class.** Defines `Referencing`, `Copying`, `SecureCoding`, `Object`, and the `sendMessage` machinery. |
| `Foundation/NSSharedPtr.hpp` | 324 | RAII smart pointer that calls `retain`/`release`. |
| `Foundation/NSPrivate.hpp` | 533 | Cached Objective-C class pointers and selectors (the bridge). |
| `Foundation/NSString.hpp` | 255 | `NS::String` wrapper. |
| `Foundation/NSArray.hpp` | 125 | `NS::Array` wrapper. |
| `Foundation/NSData.hpp` | 55 | `NS::Data` — raw byte buffer accessor. |
| `Foundation/NSDictionary.hpp` | 128 | `NS::Dictionary` — key/value map. |
| `Foundation/NSSet.hpp` | 88 | `NS::Set` — unordered unique collection. |
| `Foundation/NSEnumerator.hpp` | 79 | `NS::Enumerator<T>` — iterator protocol. |
| `Foundation/NSError.hpp` | 173 | `NS::Error` + the standard error-domain constants. |
| `Foundation/NSNumber.hpp` | 501 | `NS::Number` and `NS::Value` (boxed scalars). |
| `Foundation/NSLock.hpp` | 117 | `NS::Locking<T>` mixin and `NS::Condition`. |
| `Foundation/NSNotification.hpp` | 110 | Observer-pattern dispatch (`NSNotificationCenter`). |
| `Foundation/NSDate.hpp` | 53 | `NS::Date` — single factory method. |
| `Foundation/NSAutoreleasePool.hpp` | 84 | `NS::AutoreleasePool` — drain at end of event-loop tick. |
| `Foundation/NSURL.hpp` | 91 | `NS::URL` wrapper. |
| `Foundation/NSBundle.hpp` | 383 | `NS::Bundle` — app/package resources. |
| `Foundation/NSProcessInfo.hpp` | 386 | `NS::ProcessInfo` — process metadata + thermal/power state. |
| `Metal/MTL4FunctionDescriptor.hpp` | 50 | Tiny base descriptor for Metal functions. |
| `Metal/MTL4LibraryFunctionDescriptor.hpp` | 87 | Descriptor pointing at a function inside a library. |
| `Metal/MTL4BinaryFunctionDescriptor.hpp` | 97 | Descriptor for a pre-compiled binary function. |
| `Metal/MTL4LibraryDescriptor.hpp` | 98 | Descriptor for compiling a library from source. |
| `Metal/MTL4BinaryFunction.hpp` | 51 | The compiled binary function itself. |
| `Metal/MTL4LinkingDescriptor.hpp` | 204 | Static- and dynamic-linking descriptors (function composition). |
| `Metal/MTL4ArgumentTable.hpp` | 187 | The table that binds buffers/textures/samplers to shader slots. |
| `Metal/MTL4Counters.hpp` | 138 | GPU counter heaps (e.g. timestamps). |
| `Metal/MTL4Archive.hpp` | 93 | A serialised archive of precompiled pipelines. |
| `Metal/MTL4CommandAllocator.hpp` | 100 | Pool that backs command-buffer memory. |
| `Metal/MTL4CommitFeedback.hpp` | 62 | Async feedback after a `commit()` (GPU start/end times, error). |
| `Metal/MTL4CompilerTask.hpp` | 63 | Async compilation task handle (status, wait). |
| `Metal/MTL4Compiler.hpp` | 345 | The big compiler: builds libraries, pipelines, binary functions. |
| `Metal/MTL4CommandEncoder.hpp` | 134 | Base class for all MTL4 command encoders (barriers, fences, debug). |
| `Metal/MTL4MachineLearningCommandEncoder.hpp` | 66 | Encoder that dispatches ML networks. |
| `Metal/MTL4MachineLearningPipeline.hpp` | 172 | ML pipeline descriptor + reflection + state. |
| `Metal/MTL4CommandBuffer.hpp` | 193 | Records encoder work and submits it to a queue. |
| `Metal/MTL4ComputePipeline.hpp` | 200 | Compute pipeline descriptor (threadgroup sizing, linking). |
| `Metal/MTL4ComputeCommandEncoder.hpp` | 307 | Encoder for compute dispatches, blits, ray-tracing builds. |
| `Metal/MTL4CommandQueue.hpp` | 283 | The queue that owns command buffers, events, residency sets. |
| `Metal/MTL4MeshRenderPipeline.hpp` | 0 | **Empty placeholder** — reserved for a future API. |
| `Metal/MTL4AccelerationStructure.hpp` | 1395 | Ray-tracing acceleration structures (triangles, curves, bounding boxes, motion variants, instances). The biggest file in the library. |

Read that table once now, then come back to it whenever you lose the big picture.

---

# Part 2 — The 100-Day Study Plan

## 2.1 The five phases at a glance

The plan is divided into **five phases** of decreasing scaffolding and increasing ambition. The Foundation half of the library (Phase 1 + Phase 2) takes ~45 days because it's the conceptual foundation. The Metal half (Phases 3–5) takes ~55 days because the individual files are larger and the domain is new.

| Phase | Days | Files | What you'll be able to do at the end |
|---|---|---|---|
| **1. Setup & Foundation scaffolding** | 1–20 | Overview + the small Foundation files (`Foundation.hpp`, `NSDefines`, `NSTypes`, `NSObjCRuntime`, `NSRange`, **`NSObject`**, `NSSharedPtr`, `NSPrivate`) | Read any wrapper file and explain in plain English what `Object::sendMessage<...>(this, _NS_PRIVATE_SEL(...), ...)` is doing. |
| **2. Foundation containers & utilities** | 21–45 | `NSString`, `NSArray`, `NSData`, `NSDictionary`, `NSSet`, `NSEnumerator`, `NSError`, `NSNumber`, `NSLock`, `NSNotification`, `NSDate`, `NSAutoreleasePool`, `NSURL`, `NSBundle`, `NSProcessInfo` | Use Foundation classes fluently in C++ and explain how each method maps to an Objective-C selector. |
| **3. Metal descriptors** | 46–65 | `MTL4FunctionDescriptor`, `MTL4LibraryFunctionDescriptor`, `MTL4BinaryFunctionDescriptor`, `MTL4LibraryDescriptor`, `MTL4BinaryFunction`, `MTL4LinkingDescriptor`, `MTL4ArgumentTable`, `MTL4Counters`, `MTL4Archive` | Describe what a Metal "descriptor" is, why everything in Metal is built from a descriptor + an allocator, and how to compose descriptors. |
| **4. Metal command infrastructure** | 66–85 | `MTL4CommandAllocator`, `MTL4CommitFeedback`, `MTL4CompilerTask`, `MTL4Compiler`, `MTL4CommandEncoder`, `MTL4MachineLearningCommandEncoder`, `MTL4MachineLearningPipeline`, `MTL4CommandBuffer`, `MTL4ComputePipeline`, `MTL4ComputeCommandEncoder`, `MTL4CommandQueue` | Walk through the full command-recording lifecycle: queue → allocator → buffer → encoder → commit, in order. |
| **5. Advanced Metal — acceleration structures** | 86–100 | `MTL4AccelerationStructure.hpp` (1,395 lines, the giant) + `MTL4MeshRenderPipeline.hpp` (empty placeholder) | Understand the descriptors for ray-tracing acceleration structures (triangle / curve / bounding-box / motion / instance variants) and how they're built by `MTL4::ComputeCommandEncoder`. |

> **Pacing note.** Each day's workload is intentionally small (~30–60 min). If a day feels too light, read the same lines *again the next day* and try to explain them out loud without looking. Repetition matters more than throughput.

## 2.2 Phase 1 (Days 1–20): Setup & Foundation scaffolding

| Day | Read | Notes |
|---|---|---|
| 1 | This guide §1.1–§1.6 (the whole introduction) + a C++ refresher if needed. | Don't open the code yet. Get the mental model. |
| 2 | This guide §2 (the study plan) + skim §3.1–§3.23 just to see the section titles. | You're orienting yourself; no need to retain detail. |
| 3 | Open `metal-cpp-main/LICENSE.txt` (202 lines) and skim it. Read this guide §3.1 (`Foundation.hpp`). | Apache 2.0; you should know what you're allowed to do. `Foundation.hpp` itself is just `#include`s. |
| 4 | `NSDefines.hpp` (46 lines) — read once. Read this guide §3.2. | The macros are vocabulary. You'll see `_NS_EXPORT`, `_NS_ENUM`, `_NS_PACKED`, `_NS_PRIVATE_SEL` forever after. |
| 5 | Re-read `NSDefines.hpp` and try to write out from memory what each macro expands to. Cross-check. | Active recall beats passive re-reading. |
| 6 | `NSTypes.hpp` (52 lines) — read once. Read this guide §3.3. | Notice that `Integer`/`UInteger` are just `intptr_t`/`uintptr_t` — pointer-sized. |
| 7 | `NSObjCRuntime.hpp` (44 lines) + this guide §3.4. | The `ComparisonResult` enum will come back in `NSString::caseInsensitiveCompare`, `NSNumber::compare`, etc. |
| 8 | `NSRange.hpp` (84 lines) + this guide §3.5. | Tiny but teaches a key pattern: a `struct` with member functions defined **out-of-line** as `_NS_INLINE`. |
| 9 | Re-read `NSRange.hpp`. Trace through `LocationInRange` by hand with `location=3, length=5, loc=4`. | The expression `(!(loc < location)) && ((loc - location) < length)` is a careful way to say "loc in [location, location+length)". Why? Because `loc` and `location` are unsigned — subtraction can wrap. |
| 10 | Open `NSObject.hpp`. Read **only the class declarations** (lines 36–99). Don't worry about the implementations yet. | You're meeting `Referencing`, `Copying`, `SecureCoding`, and `Object`. Notice the inheritance: `Object : public Referencing<Object, objc_object>`. |
| 11 | Read this guide §3.6 in full (it's the longest single explanation in the guide). | This is the conceptual heart of the library. Don't rush. |
| 12 | Re-read `NSObject.hpp` lines 36–99. Now read lines 103–172 (the `Referencing` and `Copying` implementations + `bridgingCast` + `doesRequireMsgSendStret`). | You're now seeing *how* the wrappers are implemented. |
| 13 | Read `NSObject.hpp` lines 183–272 (`sendMessage`, `sendMessageSafe`, `alloc`, `init`, `hash`, `isEqual`, `description`). | By the end of today you should understand exactly what `Object::sendMessage<T>(this, _NS_PRIVATE_SEL(retain))` expands to. |
| 14 | Read `NSObject.hpp` lines 273–302 (the rest). Re-read the whole file end-to-end. | You should now be able to explain the whole file from memory. |
| 15 | Read `NSSharedPtr.hpp` (324 lines) class declaration only (lines 28–131). | Notice the API surface — it's similar to `std::shared_ptr` but built on `retain`/`release`. |
| 16 | Read this guide §3.7 + `NSSharedPtr.hpp` implementations (lines 161–324). | Pay attention to the destructor: `m_pObject->release();`. Note `__attribute__((no_sanitize("undefined")))` — used because `release()` on a null pointer is technically UB but Apple relies on it being a no-op on their runtime. |
| 17 | Re-read `NSSharedPtr.hpp`. Compare the copy-constructor (which calls `retain`) vs. the move-constructor (which doesn't). | Why does the move constructor *not* call retain? Because it steals the pointer rather than sharing it. |
| 18 | Open `NSPrivate.hpp`. Skim lines 1–86 (the macro definitions + the `#ifdef NS_PRIVATE_IMPLEMENTATION` blocks). | This is where the "is this header being compiled into the implementation file, or just consumed?" switch lives. Most users only see the `extern` branch. |
| 19 | Read this guide §3.8 + `NSPrivate.hpp` lines 90–115 (the `Class` namespace). Then skim lines 119–531 (the giant `Selector` namespace). | The `Selector` namespace is just a big list of `s_k<name>` globals — one per selector the library knows about. You don't need to memorise them. |
| 20 | Phase-1 review. Without looking at the source, write a paragraph explaining how `NS::String::string("hi", NS::UTF8StringEncoding)` ends up calling `objc_msgSend`. | If you can do this, you've understood the entire Foundation scaffolding. |

## 2.3 Phase 2 (Days 21–45): Foundation containers, errors, numbers, bundles

The files in this phase follow a near-identical template. By Day 25 you'll be reading each new file in 5–10 minutes.

| Day | Read | Notes |
|---|---|---|
| 21 | `NSString.hpp` (255 lines) — the enum `StringEncoding` + the class declaration. | Skim the enum values; you'll recognise `UTF8StringEncoding = 4`. |
| 22 | Read this guide §3.9 + `NSString.hpp` implementations (lines 122–255). | Notice the `MTLSTR("literal")` macro at line 110 — it builds a compile-time `CFString`. |
| 23 | `NSArray.hpp` (125 lines) + this guide §3.10. | Pattern: `static Array* array();` (class method) vs `Array* init();` (instance method). |
| 24 | `NSData.hpp` (55 lines) + this guide §3.11. | Smallest container. Just `bytes()` and `length()`. |
| 25 | `NSDictionary.hpp` (128 lines) + this guide §3.12. | Note the templated `object<_Object>(pKey)` — lets you cast the returned `Object*` to a more specific type. |
| 26 | `NSSet.hpp` (88 lines) + this guide §3.13. | Same idea as `Array` but unordered, unique. |
| 27 | `NSEnumerator.hpp` (79 lines) + this guide §3.14. | Look up the **Fast Enumeration Protocol** (`countByEnumeratingWithState:objects:count:`) — this is what powers `for (x in collection)` in Objective-C. |
| 28 | Re-skim Days 21–27. Pick one method from each container (e.g. `Array::object`, `Dictionary::object`, `Set::objectEnumerator`) and write down which selector each maps to. | Active recall. |
| 29 | `NSError.hpp` (173 lines) — first read the class declaration (lines 56–74) and the constants (lines 34–54). | The `_NS_PRIVATE_DEF_CONST` lines (78–94) define the well-known error-domain strings (`Cocoa`, `POSIX`, etc.). |
| 30 | Read this guide §3.15 + `NSError.hpp` implementations. | Pattern: same as before — each getter is a one-line `sendMessage`. |
| 31 | `NSNumber.hpp` (501 lines) — first the `Value` class (lines 33–49) and the `Number` class declaration (lines 51–107). | Don't be intimidated by the length: most of it is overloads for every C numeric type (`char`, `short`, `int`, `long`, `long long`, plus `unsigned` variants, plus `float`, `double`, `bool`). |
| 32 | Read this guide §3.16 + `NSNumber.hpp` lines 111–170 (the `Value` implementations). | Then skim lines 174–499 — you'll see the same pattern repeated ~30 times for each numeric type. You don't need to study each one. |
| 33 | Re-read `NSNumber.hpp` and notice that `Number` inherits from `Copying<Number, Value>` — it's a subclass of `Value`. | That's how `NSNumber` works in Objective-C too: it's a subclass of `NSValue`. |
| 34 | `NSLock.hpp` (117 lines) + this guide §3.17. | `Locking<T>` is a *mixin* template — any class that inherits it gains `lock()` / `unlock()` methods. |
| 35 | `NSNotification.hpp` (110 lines) + this guide §3.18. | Observer pattern. Notice `ObserverFunction` = `std::function<void(Notification*)>` and the trick at line 99 where it captures a `std::function` into a `__block` variable so an Objective-C block can call back into C++. |
| 36 | `NSDate.hpp` (53 lines) + this guide §3.19. | Tiny. Just one factory method. |
| 37 | `NSAutoreleasePool.hpp` (84 lines) + this guide §3.20. | Important: in metal-cpp you'll often create a pool at the top of each frame and `drain()` at the bottom. |
| 38 | `NSURL.hpp` (91 lines) + this guide §3.21. | Mostly file URLs. `fileSystemRepresentation()` returns a `const char*` suitable for `open()`. |
| 39 | `NSBundle.hpp` (383 lines) — first the class declaration (lines 42–94) and the free functions (lines 37–40). | Don't worry about memorising every method — there are ~30 accessors. |
| 40 | Read this guide §3.22 + `NSBundle.hpp` implementations. | Pattern is the same. |
| 41 | `NSProcessInfo.hpp` (386 lines) — first the enums (lines 41–63) and the class declaration (lines 66–116). | `ProcessInfo` is how you ask "what OS am I on?", "how many cores?", "how much RAM?", "is low-power mode on?". |
| 42 | Read this guide §3.23 + the implementations (lines 132–386). | Notice `sendMessageSafe` (vs. `sendMessage`) is used for methods that might not exist on older OS versions — it checks `respondsToSelector` first. |
| 43 | Phase-2 review, day 1. Pick one file from Days 21–30 and write out the wrapper pattern from memory. | You should be able to write the body of e.g. `NS::String::length()` from scratch: `return Object::sendMessage<UInteger>(this, _NS_PRIVATE_SEL(length));` |
| 44 | Phase-2 review, day 2. Pick a file from Days 31–42 and do the same. | |
| 45 | Phase-2 review, day 3. Skim every Foundation file end-to-end (about 4,000 lines — set aside an hour). | By now the wrapper pattern should be invisible to you; you'll just see "method calls". |

## 2.4 Phase 3 (Days 46–65): Metal descriptors — the small building blocks

This is where the domain shifts from Foundation to Metal. You'll be learning **GPU concepts** as much as code.

| Day | Read | Notes |
|---|---|---|
| 46 | **Background reading:** Apple's "Metal Programming Guide" — skim the sections on **devices**, **command queues**, **command buffers**, **encoders**, and **pipelines**. (Web search: "Metal programming guide Apple developer".) No code today. | If you skip this, the next 20 days will feel like random vocabulary. |
| 47 | This guide §4 intro + `Metal/MTL4FunctionDescriptor.hpp` (50 lines) + this guide §4.1. | The simplest Metal file. Just `alloc()` + `init()`. |
| 48 | `MTL4LibraryFunctionDescriptor.hpp` (87 lines) + §4.2. | Adds `library` and `name` accessors. |
| 49 | `MTL4BinaryFunctionDescriptor.hpp` (97 lines) + §4.3. | Notice the `BinaryFunctionOptions` enum (`_MTL_OPTIONS`). |
| 50 | `MTL4LibraryDescriptor.hpp` (98 lines) + §4.4. | Adds `source` (the MSL source code) and `options` (compile options). |
| 51 | `MTL4BinaryFunction.hpp` (51 lines) + §4.5. | The compiled object, not a descriptor. |
| 52 | Re-skim Days 47–51. Describe in your own words: what's the difference between a *function*, a *library*, and a *binary function*? | A *library* is a compiled unit of code; a *function* is a name inside that library; a *binary function* is a pre-compiled function you can ship without source. |
| 53 | `MTL4LinkingDescriptor.hpp` (204 lines) — read the three class declarations only (lines 34–89). + §4.6. | Three flavours of "linking" descriptors. The `RenderPipelineDynamicLinkingDescriptor` has sub-descriptors for vertex / fragment / mesh / object / tile stages — these are the stages of a render pipeline. |
| 54 | Re-read `MTL4LinkingDescriptor.hpp` implementations (lines 91–204). | Same wrapper pattern as Foundation. |
| 55 | `MTL4ArgumentTable.hpp` (187 lines) — read both class declarations (lines 37–81). + §4.7. | **Key concept.** The Argument Table is how MTL4 binds resources (buffers, textures, samplers) to shader slots without per-draw overhead. |
| 56 | Read `MTL4ArgumentTable.hpp` implementations (lines 84–187). | Notice the `setResource`, `setTexture`, `setSamplerState`, `setAddress` overloads — each maps to a different binding type. |
| 57 | Re-read `MTL4ArgumentTable.hpp` end-to-end and write a paragraph explaining when you'd use an argument table vs. setting buffers directly on an encoder. | (Answer: argument tables are the MTL4 way of binding many resources once and reusing them across draws; older MTL bound per-encoder-call.) |
| 58 | `MTL4Counters.hpp` (138 lines) + §4.8. | GPU performance counters — primarily timestamps. The `TimestampHeapEntry` struct is just a `uint64_t`. |
| 59 | `MTL4Archive.hpp` (93 lines) + §4.9. | An `Archive` is a serialised blob of precompiled pipelines and binary functions — used to skip compile time at app startup. |
| 60 | Re-skim Days 47–59. | Checkpoint. |
| 61 | Open `MTL4FunctionDescriptor.hpp` and write from memory the bodies of `alloc()` and `init()`. | They should be: `return NS::Object::alloc<...>(_MTL_PRIVATE_CLS(MTL4FunctionDescriptor));` and `return NS::Object::init<...>();`. |
| 62 | Make a list of every MTL4 descriptor class you've seen so far. For each, list: which file, which base class, and what it describes. | Should be ~10 entries. |
| 63 | Background: read about **pipeline state objects (PSOs)** in any GPU API (Vulkan / D3D12 / Metal). Understand that creating a PSO is expensive (compiles shaders) and you cache them. | |
| 64 | Re-read `MTL4Archive.hpp` and `MTL4LinkingDescriptor.hpp`. Connect the concept of "archive" (serialised PSOs) to "pipeline state object". | |
| 65 | Phase-3 review. Without looking, sketch the relationship: function descriptor → binary function descriptor → library descriptor → archive → argument table. | If you can draw the data-flow diagram, you're ready for Phase 4. |

## 2.5 Phase 4 (Days 66–85): Metal command infrastructure

This is where you see how a frame is actually recorded and submitted to the GPU.

| Day | Read | Notes |
|---|---|---|
| 66 | `MTL4CommandAllocator.hpp` (100 lines) + §4.10. | The allocator pools memory for command buffers. `reset()` recycles it. |
| 67 | `MTL4CommitFeedback.hpp` (62 lines) + §4.11. | After you `commit()` a buffer to the queue, this object tells you when the GPU actually started/ended work, and whether anything went wrong. |
| 68 | `MTL4CompilerTask.hpp` (63 lines) + §4.12. | Async compilation. `status()` returns `Scheduled` / `Compiling` / `Finished`. `waitUntilCompleted()` blocks. |
| 69 | `MTL4Compiler.hpp` (345 lines) — read just the class declarations (lines 31–138). + §4.13. | The `Compiler` class has ~30 overloads of `newComputePipelineState`, `newRenderPipelineState`, `newLibrary`, `newDynamicLibrary`, `newBinaryFunction`, `newMachineLearningPipelineState`. Each has a sync variant (returns the object directly, with `NS::Error**`), an async-block variant, and an async-`std::function` variant. |
| 70 | Read `MTL4Compiler.hpp` lines 140–200 (the descriptor options + `Compiler::device`/`label`). | |
| 71 | Skim `MTL4Compiler.hpp` lines 200–345 — the implementations. You'll see the same pattern repeated for every overload. | |
| 72 | Re-read `MTL4Compiler.hpp` lines 230–266 (the `std::function` adapter pattern). | This is how the library converts a `std::function` callback into an Objective-C block. The trick: capture the function in a `__block` variable, then call it from inside the block literal. |
| 73 | `MTL4CommandEncoder.hpp` (134 lines) + §4.14. | Base class. Barriers, fences, debug groups, signposts. |
| 74 | Re-read `MTL4CommandEncoder.hpp`. Note the three different `barrierAfter*` methods — what's the difference between "encoder stages" and "queue stages"? | (Encoder stages = within this encoder's work; queue stages = across the whole queue's submitted work.) |
| 75 | `MTL4MachineLearningCommandEncoder.hpp` (66 lines) + §4.15. | Tiny: just `dispatchNetwork(heap)`, `setArgumentTable`, `setPipelineState`. |
| 76 | `MTL4MachineLearningPipeline.hpp` (172 lines) + §4.16. | Three classes: descriptor, reflection, state. The pipeline state is the compiled, ready-to-dispatch object. |
| 77 | Re-read §4.15 + §4.16 together. You should understand the lifecycle: descriptor → compiler → state → encoder → dispatch. | |
| 78 | `MTL4CommandBuffer.hpp` (193 lines) — read the class declaration (lines 51–93). + §4.17. | A command buffer is what you record encoders into. `beginCommandBuffer(allocator)`, get an encoder, do work, `endCommandBuffer()`. |
| 79 | Read `MTL4CommandBuffer.hpp` implementations (lines 95–193). | |
| 80 | `MTL4ComputePipeline.hpp` (200 lines) + §4.18. | The compute pipeline descriptor: threadgroup sizes, persistent kernels, indirect command buffer support, binary linking. |
| 81 | Re-read `MTL4ComputePipeline.hpp` and look up any terms you don't know: `threadgroup`, `threadExecutionWidth`, `persistentKernel`, `indirectCommandBuffer`. | Don't move on until each is clear. |
| 82 | `MTL4ComputeCommandEncoder.hpp` (307 lines) + §4.19. | The biggest "encoder" file. Three big groups of methods: blits (copy between buffers/textures), dispatches (run compute shaders), and ray-tracing acceleration-structure ops (build, refit, copy, compact). |
| 83 | Read `MTL4ComputeCommandEncoder.hpp` lines 124–187 (the blit/copy implementations). | The selector names are *long* — that's normal for Objective-C; the runtime interns them so the length doesn't cost anything at runtime. |
| 84 | `MTL4CommandQueue.hpp` (283 lines) + §4.20. | The top of the command-recording stack. Owns command buffers; commits them; manages events, fences, residency sets, sparse texture mappings. |
| 85 | Phase-4 capstone. Without looking at the code, write out the full lifecycle: how would you create a queue, allocate a buffer, begin recording, encode a compute dispatch, end, and commit? List the calls in order. | This is the practical "I want to write a Metal program" checklist. |

## 2.6 Phase 5 (Days 86–100): Advanced Metal — acceleration structures & the giant file

| Day | Read | Notes |
|---|---|---|
| 86 | `MTL4MeshRenderPipeline.hpp` (0 lines — empty placeholder) + §4.21. | A free day. Apple has reserved this filename for a future mesh-render pipeline; right now it's empty. Use today to read about **mesh shaders** in general (web search "Metal mesh shaders"). | 
| 87 | **Background reading:** Apple's "Ray Tracing with Metal" sample code documentation. Web search "Metal ray tracing tutorial Apple". | Ray tracing is the domain of `MTL4AccelerationStructure.hpp`. Without this background the next two weeks will feel like alphabet soup. |
| 88 | Open `MTL4AccelerationStructure.hpp`. Read **only** the class declarations, lines 32–425. Don't read the inline implementations yet. | You'll see 11 classes — a hierarchy of descriptors for ray-tracing acceleration structures. |
| 89 | Read this guide §4.22 sections 1–3 (the high-level map). | This guide groups the 1,395 lines into ~10 conceptual sections so you don't drown. |
| 90 | Read `AccelerationStructureDescriptor` (lines 46–52) + `AccelerationStructureGeometryDescriptor` (lines 53–87). | The two root classes. Everything else inherits from one of these. |
| 91 | Read `PrimitiveAccelerationStructureDescriptor` (lines 88–117) + `AccelerationStructureTriangleGeometryDescriptor` (lines 119–155) + `AccelerationStructureBoundingBoxGeometryDescriptor` (lines 158–175). | Triangles are for triangle meshes; bounding boxes are for procedural geometry (where the intersection function is custom). |
| 92 | Read the "Motion" variants (lines 177–233) — `MotionTriangle`, `MotionBoundingBox`. | These add a time dimension so the same structure can be ray-traced across an animated frame. |
| 93 | Read `AccelerationStructureCurveGeometryDescriptor` (lines 235–296) + `AccelerationStructureMotionCurveGeometryDescriptor` (lines 298–359). | Curves are for hair, fur, tubes. The motion variants animate them. |
| 94 | Read `InstanceAccelerationStructureDescriptor` (lines 361–402). | An "instance" accel structure references other accel structures with per-instance transforms — this is how you ray-trace a scene made of many objects. |
| 95 | Read `IndirectInstanceAccelerationStructureDescriptor` (lines 404–425). | Like the instance descriptor, but the instances themselves come from a GPU buffer (built by a compute shader) rather than from CPU-side descriptors. |
| 96 | Now read the implementations (lines 426–end). Skim; the pattern is identical to every other file. | You're not expected to memorise 1,000 lines of inline boilerplate. |
| 97 | Re-skim the class declarations (lines 32–425) and draw the inheritance tree on paper. | 11 nodes; two roots (`AccelerationStructureDescriptor` and `AccelerationStructureGeometryDescriptor`). |
| 98 | Connect back to Phase 4: re-read the `buildAccelerationStructure`, `refitAccelerationStructure`, `copyAccelerationStructure`, `copyAndCompactAccelerationStructure` methods in `MTL4ComputeCommandEncoder.hpp`. | Now you see the full picture: descriptors from this file → `ComputeCommandEncoder::buildAccelerationStructure(...)` → GPU builds the structure. |
| 99 | Phase-5 capstone, day 1: write a one-page summary of the ray-tracing data flow in MTL4. | |
| 100 | Phase-5 capstone, day 2: re-read this guide's Part 5 (appendices) end-to-end. Celebrate. | You're done. |

## 2.7 How to study each day

A productive 30–60 minute session looks like this:

1. **Recall yesterday (5 min).** Without opening anything, say out loud what you read yesterday. If you can't, re-read yesterday's lines first.
2. **Read today's source file once, cold (10–20 min).** Don't take notes. Just let the shape of the file sink in. You will not understand everything. That's fine.
3. **Read the matching section in this guide (10–15 min).** Now the bits that confused you should click.
4. **Read the source file a second time (10–15 min).** With the commentary in mind, the code should feel different — clearer, smaller.
5. **Write a 3-sentence summary in your own notes (5 min).** Doesn't matter where — a notebook, a markdown file, anything. The act of writing forces you to digest.
6. **(Optional, weekends only) Code experiment.** If you have a Mac, try to actually use the file's classes in a tiny C++ program. You'll need Xcode and the Metal framework linked. This is the best way to cement the abstract API into intuition.

If you fall behind, **don't binge**. Just resume at the next day's slot. Binging produces the illusion of learning; spaced repetition produces learning.

## 2.8 Self-check questions to use throughout

After finishing any file, you should be able to answer these without looking:

1. **What is the file's namespace?** (`NS::`, `MTL4::`, `MTL::`.)
2. **What classes does it declare?** List them.
3. **What does each class inherit from?** (Most inherit from `NS::Copying<T>` or `NS::Referencing<T>`. Some inherit from another wrapper class.)
4. **What selectors does each method map to?** (Look at the `_NS_PRIVATE_SEL(...)` / `_MTL_PRIVATE_SEL(...)` argument.)
5. **Where is the actual Objective-C class pointer cached?** (In `NS::Private::Class::s_k<Name>` or the MTL equivalent.)
6. **Is there anything unusual about this file?** (An enum, a struct, a `std::function` adapter, a special macro like `MTLSTR`?)
7. **Could I write the body of one method from memory?** (You should always be able to write the body of a simple getter: `return Object::sendMessage<T>(this, _NS_PRIVATE_SEL(name));`)

If you can answer all seven for a file, you've understood it. Move on.

---

# Part 3 — Foundation Files (23 files, in learning order)

The Foundation half of metal-cpp reimplements a subset of Apple's Foundation framework in C++. Every class you meet here (`NS::String`, `NS::Array`, …) is a thin wrapper around an Objective-C class of the same name (`NSString`, `NSArray`, …). The classes exist so Metal-using C++ code can pass strings, arrays, dictionaries and errors to and from the Metal API without dragging in Objective-C source files.

The 23 files are presented in dependency order: the foundational ones (macros, types, the root `Object`) come first; the concrete container classes come last.

---

## 3.1 `Foundation/Foundation.hpp` — the umbrella

**Path:** `metal-cpp-main/Foundation/Foundation.hpp`
**Lines:** 47
**Why it matters:** It's the single `#include` you'll add at the top of your own `.cpp` files to pull in the whole Foundation half of the library.

### Section 1: License header (lines 1–19)

Every file in this library starts with the same 19-line Apache 2.0 license boilerplate inside a decorative `//----…` border. The header identifies the file path (e.g. `Foundation/Foundation.hpp`), the copyright years (`2020-2024`), and the license terms. You can safely skip this block on every subsequent file.

### Section 2: Include guard (line 21)

```cpp
#pragma once
```

A modern, non-portable-but-supported-everywhere include guard. It tells the compiler "if this file has already been included in this translation unit, don't include it again." Equivalent to the older `#ifndef NS_FOUNDATION_HPP / #define … / #endif` pattern but without naming a macro.

### Section 3: The includes (lines 25–45)

```cpp
#include "NSArray.hpp"
#include "NSAutoreleasePool.hpp"
#include "NSBundle.hpp"
#include "NSData.hpp"
#include "NSDate.hpp"
#include "NSDefines.hpp"
#include "NSDictionary.hpp"
#include "NSEnumerator.hpp"
#include "NSError.hpp"
#include "NSLock.hpp"
#include "NSNotification.hpp"
#include "NSNumber.hpp"
#include "NSObject.hpp"
#include "NSPrivate.hpp"
#include "NSProcessInfo.hpp"
#include "NSRange.hpp"
#include "NSSet.hpp"
#include "NSSharedPtr.hpp"
#include "NSString.hpp"
#include "NSTypes.hpp"
#include "NSURL.hpp"
```

That's the whole file. Twenty-one sibling headers, included in **alphabetical order** (which is the convention Apple uses throughout this library). Note that there is no `Foundation/NSObjCRuntime.hpp` in this list — a small inconsistency in Apple's umbrella header; `NSObjCRuntime.hpp` is reached transitively via `NSString.hpp` and `NSNumber.hpp`, both of which include it.

**Takeaway:** When you write `#include "Foundation/Foundation.hpp"` you get every Foundation wrapper. If you only need one class (say `NS::String`), you can include `NSString.hpp` directly and skip the umbrella.

---

## 3.2 `Foundation/NSDefines.hpp` — the macro vocabulary

**Path:** `metal-cpp-main/Foundation/NSDefines.hpp`
**Lines:** 46
**Why it matters:** Every subsequent file uses these macros. Without recognising them you'll think the code is full of magic; with them you'll see it's just abbreviations.

### Section 1: License header + `#pragma once` (lines 1–23)

Same boilerplate as every other file. Skip.

### Section 2: Compiler-attribute macros (lines 25–33)

```cpp
#define _NS_WEAK_IMPORT __attribute__((weak_import))
#ifdef METALCPP_SYMBOL_VISIBILITY_HIDDEN
#define _NS_EXPORT __attribute__((visibility("hidden")))
#else
#define _NS_EXPORT __attribute__((visibility("default")))
#endif // METALCPP_SYMBOL_VISIBILITY_HIDDEN
#define _NS_EXTERN extern "C" _NS_EXPORT
#define _NS_INLINE inline __attribute__((always_inline))
#define _NS_PACKED __attribute__((packed))
```

These five macros wrap common GCC/Clang attributes so the rest of the library can use short, consistent names:

- **`_NS_WEAK_IMPORT`** — marks a symbol as "weakly imported": if it's not present at runtime (e.g. an OS-version-gated API on an older device), the program still loads; the symbol is just `nullptr`. Used for forward-compatibility.
- **`_NS_EXPORT`** — controls symbol visibility. By default, "default" visibility (the symbol is exported from the library). If you compile metal-cpp with `-DMETALCPP_SYMBOL_VISIBILITY_HIDDEN`, every symbol becomes "hidden", which lets you statically link metal-cpp into your own binary without symbol collisions.
- **`_NS_EXTERN`** — combines `extern "C"` (so the symbol has C linkage, no name mangling) with `_NS_EXPORT`. Used in `_NS_CONST` declarations.
- **`_NS_INLINE`** — `inline` plus `__attribute__((always_inline))`. The latter tells Clang "inline this *even in debug builds* where you'd normally not." Critical for the wrapper functions — without it, every method call would be a real function call into a tiny 3-line stub, adding overhead.
- **`_NS_PACKED`** — `__attribute__((packed))` removes padding between struct members. Used for structs that must match an Objective-C ABI exactly (e.g. `NS::Range`, `NS::OperatingSystemVersion`).

### Section 3: Enum / options / const macros (lines 35–39)

```cpp
#define _NS_CONST(type, name) _NS_EXTERN type const name
#define _NS_ENUM(type, name) enum name : type
#define _NS_OPTIONS(type, name) \
    using name = type;          \
    enum : name
```

These mimic the Objective-C `NS_ENUM` and `NS_OPTIONS` macros:

- **`_NS_CONST(type, name)`** declares a constant of the given type with C linkage. Example from `NSError.hpp`: `_NS_CONST(ErrorDomain, CocoaErrorDomain);` expands to `extern "C" _NS_EXPORT ErrorDomain const CocoaErrorDomain;`.
- **`_NS_ENUM(type, name)`** declares a **strongly-typed** enum. Used when the values are mutually exclusive (e.g. `ComparisonResult` is either `OrderedAscending`, `OrderedSame`, or `OrderedDescending`).
- **`_NS_OPTIONS(type, name)`** declares a **bitmask** type whose values can be OR-ed together. It expands to `using name = type; enum : name { ... };` — first a type alias (so you can write `StringCompareOptions opts;` as a plain integer variable), then an anonymous enum with the same name (so `CaseInsensitiveSearch` is a constant of that type). Used for `StringCompareOptions`, `ActivityOptions`, etc.

### Section 4: Validation macros (lines 41–43)

```cpp
#define _NS_CAST_TO_UINT(value) static_cast<NS::UInteger>(value)
#define _NS_VALIDATE_SIZE(ns, name) static_assert(sizeof(ns::name) == sizeof(ns##name), "size mismatch " #ns "::" #name)
#define _NS_VALIDATE_ENUM(ns, name) static_assert(_NS_CAST_TO_UINT(ns::name) == _NS_CAST_TO_UINT(ns##name), "value mismatch " #ns "::" #name)
```

These are compile-time safety checks used in the corresponding `MTLDefines.hpp` and in the implementation file:

- **`_NS_CAST_TO_UINT(value)`** — a short alias for casting to `NS::UInteger`. Used inside `_NS_VALIDATE_ENUM` so the comparison works regardless of the enum's underlying type.
- **`_NS_VALIDATE_SIZE(ns, name)`** — `static_assert` that the C++ wrapper struct `ns::name` is the same size as the Objective-C struct `ns##name` (the `##` token-pastes; e.g. `NS::Range` vs `NSRange`). If they ever diverge, your build breaks — protecting against ABI drift.
- **`_NS_VALIDATE_ENUM(ns, name)`** — same idea, for enum values: the C++ enum constant must equal the Objective-C enum constant.

These are *only* used inside `NS_PRIVATE_IMPLEMENTATION` (the implementation side of the library), so most users never see them. But they explain why the library is so confident about ABI compatibility.

---

## 3.3 `Foundation/NSTypes.hpp` — `Integer`, `UInteger`, `TimeInterval`

**Path:** `metal-cpp-main/Foundation/NSTypes.hpp`
**Lines:** 52
**Why it matters:** Defines the basic integer and time types used throughout the library.

### Section 1: Includes (lines 25–28)

```cpp
#include "NSDefines.hpp"
#include <CoreFoundation/CoreFoundation.h>
#include <cstdint>
```

`<CoreFoundation/CoreFoundation.h>` is Apple's C-level Foundation. `metal-cpp` doesn't actually use much of it directly, but it pulls in the C types `intptr_t`/`uintptr_t` (via `<stdint.h>`, which CoreFoundation transitively includes) and ensures the Objective-C runtime symbols are available.

### Section 2: The `NS` namespace (lines 32–49)

```cpp
namespace NS
{
using TimeInterval = double;

using Integer  = std::intptr_t;
using UInteger = std::uintptr_t;

const Integer  IntegerMax  = INTPTR_MAX;
const Integer  IntegerMin  = INTPTR_MIN;
const UInteger UIntegerMax = UINTPTR_MAX;

struct OperatingSystemVersion
{
    Integer majorVersion;
    Integer minorVersion;
    Integer patchVersion;
} _NS_PACKED;
}
```

Walkthrough:

- **`TimeInterval`** — alias for `double`. Represents seconds (usually since some reference date). `double` rather than `float` because sub-microsecond precision matters for GPU timing.
- **`Integer`** — `std::intptr_t`. A **signed** integer the same width as a pointer: 64-bit on 64-bit platforms, 32-bit on 32-bit. This is the type Objective-C's `NSInteger` maps to.
- **`UInteger`** — `std::uintptr_t`. The **unsigned** companion. Used for counts, lengths, indices — anything that's conceptually non-negative.
- The `*Max` / `*Min` constants are the type's limits. You'll see `IntegerMax` again as the value of `NS::NotFound` (next file, §3.4).
- **`OperatingSystemVersion`** — a packed struct with three `Integer` fields (`majorVersion`, `minorVersion`, `patchVersion`). Returned by `NS::ProcessInfo::operatingSystemVersion()`. The `_NS_PACKED` attribute ensures no padding, so it matches the Objective-C `NSOperatingSystemVersion` ABI exactly.

**Why pointer-sized integers?** On older 32-bit iOS, `int` was 32 bits and `NSInteger` was 32 bits; on 64-bit iOS/macOS, `NSInteger` is 64 bits. Using `intptr_t` makes the C++ wrapper match Objective-C's chosen width automatically.

---

## 3.4 `Foundation/NSObjCRuntime.hpp` — `ComparisonResult`, `NotFound`

**Path:** `metal-cpp-main/Foundation/NSObjCRuntime.hpp`
**Lines:** 44
**Why it matters:** Tiny file that declares the `ComparisonResult` enum used by every comparison method (`NSString::caseInsensitiveCompare`, `NSNumber::compare`, …) and the `NotFound` sentinel used when a search fails.

### Section 1: Includes + namespace (lines 25–30)

Just `NSDefines.hpp` + `NSTypes.hpp`, then `namespace NS {`. Two lines of substance.

### Section 2: The enum and the constant (lines 33–39)

```cpp
_NS_ENUM(Integer, ComparisonResult) {
    OrderedAscending  = -1L,
    OrderedSame,
    OrderedDescending
};

const Integer NotFound = IntegerMax;
```

`_NS_ENUM(Integer, ComparisonResult)` expands (per §3.2) to `enum ComparisonResult : Integer { … };` — a strongly-typed enum whose underlying type is `NS::Integer`.

The values:
- **`OrderedAscending = -1L`** — explicitly initialised to `-1`. The `L` suffix makes it a `long` literal so it widens correctly on 64-bit.
- **`OrderedSame`** — implicitly gets the next value, `0`.
- **`OrderedDescending`** — implicitly gets the next value, `1`.

This is the classic three-way comparison result (`-1` / `0` / `+1`) used by `qsort`-style comparators since the 1970s.

**`NotFound`** is set to `IntegerMax` (which is `INTPTR_MAX`, e.g. `9223372036854775807` on 64-bit). Returned by methods like `NSString::rangeOfString(...).location` when the substring isn't found. Always check for it before using a returned index.

---

## 3.5 `Foundation/NSRange.hpp` — a simple struct with methods

**Path:** `metal-cpp-main/Foundation/NSRange.hpp`
**Lines:** 84
**Why it matters:** A small but teaching example: a C++ `struct` with **member functions defined out-of-line as `_NS_INLINE`**. This pattern (declare the API in the struct, define the bodies after) is how every wrapper file in the library is structured.

### Section 1: Includes + namespace + struct declaration (lines 25–45)

```cpp
namespace NS
{
struct Range
{
    static Range Make(UInteger loc, UInteger len);

    Range(UInteger loc, UInteger len);

    bool     Equal(const Range& range) const;
    bool     LocationInRange(UInteger loc) const;
    UInteger Max() const;

    UInteger location;
    UInteger length;
} _NS_PACKED;
}
```

`NS::Range` mirrors the Objective-C `NSRange` struct. Two public data members (`location`, `length`) and four member functions:

- A **static factory** `Make(loc, len)` — returns a `Range` by value. (In Objective-C you'd write `NSMakeRange(loc, len)`.)
- A **constructor** `Range(loc, len)`.
- `Equal(other)` — equality test.
- `LocationInRange(loc)` — is the given index inside this range?
- `Max()` — one-past-the-end index (`location + length`).

The `_NS_PACKED` attribute is critical: it makes the C++ struct's memory layout exactly match the Objective-C struct, so the two can be passed across the boundary by reinterpret-cast.

### Section 2: Out-of-line inline definitions (lines 49–82)

```cpp
_NS_INLINE NS::Range::Range(UInteger loc, UInteger len)
    : location(loc)
    , length(len)
{
}

_NS_INLINE NS::Range NS::Range::Make(UInteger loc, UInteger len)
{
    return Range(loc, len);
}

_NS_INLINE bool NS::Range::Equal(const Range& range) const
{
    return (location == range.location) && (length == range.length);
}

_NS_INLINE bool NS::Range::LocationInRange(UInteger loc) const
{
    return (!(loc < location)) && ((loc - location) < length);
}

_NS_INLINE NS::UInteger NS::Range::Max() const
{
    return location + length;
}
```

Each definition is prefixed `_NS_INLINE` (i.e. `inline __attribute__((always_inline))`) and qualifies the method name with `NS::Range::`. This is the standard C++ way to define member functions outside the class body.

**Worth examining: `LocationInRange`**. The natural way to write it would be `return loc >= location && loc < location + length;`. Apple instead writes `return (!(loc < location)) && ((loc - location) < length);`. Why?

Because `loc` and `location` are *unsigned*. If you wrote `loc - location` and `loc < location`, the subtraction would underflow and wrap around to a huge positive number — but since the first clause `!(loc < location)` rules that out, the subtraction is always safe. The pattern is a tiny defensive measure against unsigned underflow; it doesn't change behaviour, it just makes the code's intent explicit.

---

## 3.6 `Foundation/NSObject.hpp` — the root of everything

**Path:** `metal-cpp-main/Foundation/NSObject.hpp`
**Lines:** 302
**Why it matters:** This is the conceptual heart of the entire library. Every wrapper class (`NS::String`, `NS::Array`, … and on the Metal side `MTL4::ArgumentTable`, …) ultimately inherits from `NS::Object`. Every method call you'll see in every other file goes through `Object::sendMessage`. If you understand this file, the rest of metal-cpp becomes trivial; if you don't, nothing will make sense.

### Section 1: Includes (lines 25–32)

```cpp
#include "NSDefines.hpp"
#include "NSPrivate.hpp"
#include "NSTypes.hpp"

#include <objc/message.h>
#include <objc/runtime.h>

#include <type_traits>
```

The interesting includes are the two `<objc/...>` headers — these expose the Objective-C runtime as plain C functions:

- `<objc/runtime.h>` gives `objc_lookUpClass(name)`, `sel_registerName(name)`, `objc_getProtocol(name)`.
- `<objc/message.h>` gives `objc_msgSend` and its variants `objc_msgSend_fpret` (for returning floats on x86) and `objc_msgSend_stret` (for returning large structs by hidden-pointer on x86/ARM).

`<type_traits>` provides `std::is_floating_point_v`, `std::is_class_v`, `std::is_void_v`, `std::is_convertible_v` — used by `sendMessage` to choose the right `objc_msgSend` variant.

### Section 2: The three protocol-like templates (lines 36–60)

```cpp
template <class _Class, class _Base = class Object>
class _NS_EXPORT Referencing : public _Base
{
public:
    _Class*  retain();
    void     release();
    _Class*  autorelease();
    UInteger retainCount() const;
};

template <class _Class, class _Base = class Object>
class Copying : public Referencing<_Class, _Base>
{
public:
    _Class* copy() const;
};

template <class _Class, class _Base = class Object>
class SecureCoding : public Referencing<_Class, _Base>
{
};
```

These three templates model Objective-C protocols:

- **`Referencing<_Class, _Base>`** — corresponds to the `NSObject` protocol that exposes reference-counting. Adds `retain`, `release`, `autorelease`, `retainCount`. The two template parameters let you specify both the **concrete type** (`_Class`, e.g. `String`) and the **base class** (`_Base`, defaults to `Object`). This is the **Curiously Recurring Template Pattern (CRTP)**: it lets the methods return `_Class*` instead of `Referencing*`, so `someString->retain()` returns `String*` not `Referencing*`.
- **`Copying<_Class, _Base>`** — corresponds to the `NSCopying` protocol. Inherits `Referencing` and adds `copy()`. Used by immutable containers (`NSString`, `NSArray`, `NSDictionary`, …) that support a `copy` operation.
- **`SecureCoding<_Class, _Base>`** — corresponds to the `NSSecureCoding` protocol. Currently an empty subclass of `Referencing`; exists so future versions can add secure-archiving methods.

**The forward-declaration `class Object` trick.** Look at the default `_Base = class Object`. The `class` keyword here is unusual — it's a *forward declaration of a class named `Object`* embedded in a template parameter default. The actual `Object` class is defined later in the same file (Section 3 below). C++ allows this because the default argument isn't instantiated until the template is used, by which point `Object` is fully defined.

### Section 3: The `Object` class itself (lines 62–98)

```cpp
class Object : public Referencing<Object, objc_object>
{
public:
    UInteger      hash() const;
    bool          isEqual(const Object* pObject) const;

    class String* description() const;
    class String* debugDescription() const;

protected:
    friend class Referencing<Object, objc_object>;

    template <class _Class>
    static _Class* alloc(const char* pClassName);
    template <class _Class>
    static _Class* alloc(const void* pClass);
    template <class _Class>
    _Class* init();

    template <class _Dst>
    static _Dst                   bridgingCast(const void* pObj);
    static class MethodSignature* methodSignatureForSelector(const void* pObj, SEL selector);
    static bool                   respondsToSelector(const void* pObj, SEL selector);
    template <typename _Type>
    static constexpr bool doesRequireMsgSendStret();
    template <typename _Ret, typename... _Args>
    static _Ret sendMessage(const void* pObj, SEL selector, _Args... args);
    template <typename _Ret, typename... _Args>
    static _Ret sendMessageSafe(const void* pObj, SEL selector, _Args... args);

private:
    Object() = delete;
    Object(const Object&) = delete;
    ~Object() = delete;

    Object& operator=(const Object&) = delete;
};
```

This is the master class. A lot is going on:

- **Inheritance:** `Object : public Referencing<Object, objc_object>`. Note the base-class argument is `objc_object` — the Objective-C runtime's C-level "any object" type. So an `NS::Object` *is-a* `objc_object*` after reinterpret-cast. The CRTP argument is `Object` itself, so `retain()` returns `Object*`.
- **Public API:** `hash()`, `isEqual(other)`, `description()` (the Objective-C `description` method, used like `toString()` in Java), `debugDescription()` (a more verbose version for debuggers). The forward-declared `class String*` return types are needed because `NS::String` is defined in a different header not yet included.
- **Protected API:** The interesting stuff — `alloc`, `init`, `bridgingCast`, `sendMessage`, `sendMessageSafe`. These are `protected` so only subclasses (and the `friend` declaration below) can call them. User code never calls `Object::sendMessage` directly; it goes through e.g. `NS::String::length()` which is itself an inline call to `Object::sendMessage<UInteger>(this, ...)`.
- **The `friend` line:** `friend class Referencing<Object, objc_object>;` — gives the `Referencing` template instantiation permission to call `Object`'s protected constructor (used in `retain()` etc.).
- **Private deleted specials:** The default constructor, copy constructor, destructor, and copy assignment are all `= delete`. This prevents anyone from creating or destroying an `Object` directly with `new`/`delete` or by value — you must use the `alloc`/`init`/`release` lifecycle that matches Objective-C's reference-counting rules.

### Section 4: The `Referencing` implementations (lines 103–139)

```cpp
template <class _Class, class _Base>
_NS_INLINE _Class* NS::Referencing<_Class, _Base>::retain()
{
    return Object::sendMessage<_Class*>(this, _NS_PRIVATE_SEL(retain));
}

template <class _Class, class _Base>
_NS_INLINE void NS::Referencing<_Class, _Base>::release()
{
    return Object::sendMessage<void>(this, _NS_PRIVATE_SEL(release));
}

template <class _Class, class _Base>
_NS_INLINE _Class* NS::Referencing<_Class, _Base>::autorelease()
{
    return Object::sendMessage<_Class*>(this, _NS_PRIVATE_SEL(autorelease));
}

template <class _Class, _Base>
_NS_INLINE NS::UInteger NS::Referencing<_Class, _Base>::retainCount() const
{
    return Object::sendMessage<UInteger>(this, _NS_PRIVATE_SEL(retainCount));
}

template <class _Class, class _Base>
_NS_INLINE _Class* NS::Copying<_Class, _Base>::copy() const
{
    return Object::sendMessage<_Class*>(this, _NS_PRIVATE_SEL(copy));
}
```

These five definitions are the entire reference-counting API. Each is a one-liner that calls `Object::sendMessage` with the right return type and the right cached selector (`_NS_PRIVATE_SEL(retain)` expands to `Private::Selector::s_kretain`, which is a `SEL` registered at startup to `"retain"`).

Notice the **return-type trick**: `retain()` returns `_Class*` (e.g. `String*`), not `Referencing*` or `Object*`. This is why the CRTP is worth its complexity — it means `myString->retain()` chains naturally without casts.

### Section 5: `bridgingCast` (lines 143–151)

```cpp
template <class _Dst>
_NS_INLINE _Dst NS::Object::bridgingCast(const void* pObj)
{
#if defined(__OBJC__) && __has_feature(objc_arc)
    return (__bridge _Dst)pObj;
#else
    return (_Dst)pObj;
#endif
}
```

A "bridging cast" is the term for converting between an Objective-C object pointer (`id` / `objc_object*`) and a CoreFoundation pointer (`CFTypeRef`). Under **ARC** (Automatic Reference Counting) the compiler needs to know whether you're transferring ownership, so it requires the `__bridge` / `__bridge_retained` / `__bridge_transfer` keywords. metal-cpp is **not** compiled under ARC, so the cast is just a plain C cast.

The `#if defined(__OBJC__) && __has_feature(objc_arc)` preprocessor check uses the correct form if you happen to compile this header from an Objective-C++ file with ARC on. In a plain C++ file, the `#else` branch is used.

### Section 6: `doesRequireMsgSendStret` (lines 155–179)

```cpp
template <typename _Type>
_NS_INLINE constexpr bool NS::Object::doesRequireMsgSendStret()
{
#if (defined(__i386__) || defined(__x86_64__))
    constexpr size_t kStructLimit = (sizeof(std::uintptr_t) << 1);
    return sizeof(_Type) > kStructLimit;
#elif defined(__arm64__)
    return false;
#elif defined(__arm__)
    constexpr size_t kStructLimit = sizeof(std::uintptr_t);
    return std::is_class_v<_Type> && (sizeof(_Type) > kStructLimit);
#else
#error "Unsupported architecture!"
#endif
}

template <>
_NS_INLINE constexpr bool NS::Object::doesRequireMsgSendStret<void>()
{
    return false;
}
```

This is where things get architecture-specific. On some architectures, returning a struct from a function is done via a hidden pointer argument (the caller allocates space, passes its address, and the callee writes there). The Objective-C runtime has a separate function for this: `objc_msgSend_stret`. Whether you need it depends on:

- **x86 / x86_64 (Intel Macs):** Structs up to 2 registers (16 bytes on 64-bit) are returned in registers; anything bigger needs `stret`.
- **ARM64 (Apple Silicon, modern iOS):** `objc_msgSend_stret` doesn't exist — the ABI handles structs up to 16 bytes in registers and there's no separate stret variant. So return `false` always.
- **ARMv7 (old 32-bit iOS):** Structs up to 4 bytes return in registers; bigger ones (that are *class types*) need `stret`. The `std::is_class_v<_Type>` check ensures we only apply the stret rule to actual structs/classes, not to scalars passed by value.

The template specialisation `doesRequireMsgSendStret<void>` (line 175) returns `false` for `void` — because a method returning `void` doesn't return a struct at all.

The `constexpr` keyword means this decision is made at **compile time**, so the `if constexpr` in `sendMessage` (next section) can statically select the right branch with zero runtime cost.

### Section 7: `sendMessage` — the dispatcher (lines 183–218)

This is the most important function in the whole library. Read it carefully.

```cpp
template <typename _Ret, typename... _Args>
_NS_INLINE _Ret NS::Object::sendMessage(const void* pObj, SEL selector, _Args... args)
{
#if (defined(__i386__) || defined(__x86_64__))
    if constexpr (std::is_floating_point<_Ret>())
    {
        using SendMessageProcFpret = _Ret (*)(const void*, SEL, _Args...);
        const SendMessageProcFpret pProc = reinterpret_cast<SendMessageProcFpret>(&objc_msgSend_fpret);
        return (*pProc)(pObj, selector, args...);
    }
    else
#endif
#if !defined(__arm64__)
        if constexpr (doesRequireMsgSendStret<_Ret>())
    {
        using SendMessageProcStret = void (*)(_Ret*, const void*, SEL, _Args...);
        const SendMessageProcStret pProc = reinterpret_cast<SendMessageProcStret>(&objc_msgSend_stret);
        _Ret                       ret;
        (*pProc)(&ret, pObj, selector, args...);
        return ret;
    }
    else
#endif
    {
        using SendMessageProc = _Ret (*)(const void*, SEL, _Args...);
        const SendMessageProc pProc = reinterpret_cast<SendMessageProc>(&objc_msgSend);
        return (*pProc)(pObj, selector, args...);
    }
}
```

Walking through:

1. **Template signature:** `template <typename _Ret, typename... _Args>`. The first type parameter is the return type; the variadic pack is the argument types. So `sendMessage<NSUInteger>(this, sel)` instantiates one version, `sendMessage<void>(this, sel, foo)` instantiates another, etc.

2. **x86-only first branch:** If the return type is a floating-point type (`float` or `double`), and we're on x86/x64, use `objc_msgSend_fpret`. Why? Because on x86, floating-point values are returned in the XMM registers (xmm0) while integers are returned in EAX/RAX. The Objective-C runtime has a separate entry point that knows to look in the right register. The `if constexpr` makes this branch **disappear** at compile time when the return type isn't floating-point.

3. **Non-ARM64 second branch:** If `doesRequireMsgSendStret<_Ret>()` returned `true` (i.e. we're returning a struct bigger than the ABI's in-register limit), use `objc_msgSend_stret`. This variant takes an extra first argument — a pointer to where the result should be written. We allocate a local `_Ret ret;`, pass `&ret`, then return it.

4. **Default branch:** Everything else. Cast `objc_msgSend` (a single function the runtime exports) to a function pointer with the exact signature we want, and call it.

**Why the `reinterpret_cast`?** `objc_msgSend` is declared as `id objc_msgSend(id self, SEL op, ...)` — it takes variadic args. But variadic functions on most ABIs have different calling conventions from non-variadic ones (e.g. floating-point arguments in variadic functions go on the stack on x86_64). To get the right calling convention for our specific argument types, we cast `objc_msgSend` to a non-variadic function pointer with the exact signature `_Ret(*)(const void*, SEL, _Args...)`. This is officially documented and supported by Apple.

### Section 8: `methodSignatureForSelector` and `respondsToSelector` (lines 222–232)

```cpp
_NS_INLINE NS::MethodSignature* NS::Object::methodSignatureForSelector(const void* pObj, SEL selector)
{
    return sendMessage<MethodSignature*>(pObj, _NS_PRIVATE_SEL(methodSignatureForSelector_), selector);
}

_NS_INLINE bool NS::Object::respondsToSelector(const void* pObj, SEL selector)
{
    return sendMessage<bool>(pObj, _NS_PRIVATE_SEL(respondsToSelector_), selector);
}
```

These call the Objective-C methods `-methodSignatureForSelector:` and `-respondsToSelector:`. They're used by `sendMessageSafe` to check whether a method exists on the runtime object before calling it.

### Section 9: `sendMessageSafe` (lines 236–248)

```cpp
template <typename _Ret, typename... _Args>
_NS_INLINE _Ret NS::Object::sendMessageSafe(const void* pObj, SEL selector, _Args... args)
{
    if ((respondsToSelector(pObj, selector)) || (nullptr != methodSignatureForSelector(pObj, selector)))
    {
        return sendMessage<_Ret>(pObj, selector, args...);
    }

    if constexpr (!std::is_void<_Ret>::value)
    {
        return _Ret(0);
    }
}
```

The "safe" variant checks first whether the object responds to the selector. If yes, calls `sendMessage`. If no:

- If the return type is `void`: just returns (does nothing).
- Otherwise: returns `_Ret(0)` — i.e. zero-initialised. For a pointer return type, that's `nullptr`. For an integer, `0`. For a struct, value-initialised.

Used for methods that might not exist on older OS versions (forward compatibility). You'll see `sendMessageSafe` in `NSProcessInfo` for things like `isiOSAppOnMac` which didn't exist before macOS 11.

### Section 10: `alloc` and `init` (lines 252–272)

```cpp
template <class _Class>
_NS_INLINE _Class* NS::Object::alloc(const char* pClassName)
{
    return sendMessage<_Class*>(objc_lookUpClass(pClassName), _NS_PRIVATE_SEL(alloc));
}

template <class _Class>
_NS_INLINE _Class* NS::Object::alloc(const void* pClass)
{
    return sendMessage<_Class*>(pClass, _NS_PRIVATE_SEL(alloc));
}

template <class _Class>
_NS_INLINE _Class* NS::Object::init()
{
    return sendMessage<_Class*>(this, _NS_PRIVATE_SEL(init));
}
```

These implement the two-step Objective-C allocation pattern: `[SomeClass alloc]` returns an uninitialised instance; `[instance init]` returns the fully-constructed instance.

- **`alloc(const char* pClassName)`** — looks up the Objective-C class by name at runtime via `objc_lookUpClass("NSString")`, then sends it the `alloc` selector. Used when you only know the class name as a string.
- **`alloc(const void* pClass)`** — same idea, but takes a pre-cached class pointer. This is the form used by every wrapper file: at startup the class pointer is cached in `Private::Class::s_kNSString` (or similar), then `alloc` is called with that cached pointer. No string lookup needed at call time.
- **`init()`** — sends `init` to `this` (an already-allocated instance) and returns the result.

The two-step `alloc`+`init` pattern is why in metal-cpp you'll see code like:

```cpp
NS::String* s = NS::String::alloc()->init("hello", NS::UTF8StringEncoding);
```

which mirrors the Objective-C `[[NSString alloc] initWithCString:"hello" encoding:NSUTF8StringEncoding]`.

### Section 11: The public Object methods (lines 276–300)

```cpp
_NS_INLINE NS::UInteger NS::Object::hash() const
{
    return sendMessage<UInteger>(this, _NS_PRIVATE_SEL(hash));
}

_NS_INLINE bool NS::Object::isEqual(const Object* pObject) const
{
    return sendMessage<bool>(this, _NS_PRIVATE_SEL(isEqual_), pObject);
}

_NS_INLINE NS::String* NS::Object::description() const
{
    return sendMessage<String*>(this, _NS_PRIVATE_SEL(description));
}

_NS_INLINE NS::String* NS::Object::debugDescription() const
{
    return sendMessageSafe<String*>(this, _NS_PRIVATE_SEL(debugDescription));
}
```

Four straightforward wrappers around `NSObject`'s standard methods:

- `hash` — returns an `NSUInteger` hash of the object. Used when the object is a key in `NSDictionary` or a member of `NSSet`.
- `isEqual:` — equality test. Note the trailing `_` in `_NS_PRIVATE_SEL(isEqual_)` — that mirrors the Objective-C selector name `isEqual:` (the colon is part of the name and is preserved as `_` in the C++ symbol).
- `description` — returns a `String*` describing the object (Java's `toString()`).
- `debugDescription` — uses `sendMessageSafe` (not `sendMessage`) because not all objects implement `debugDescription`; falling back to a default is preferable to crashing.

### What you've learned

After working through this file you now understand the **dispatch pipeline** that every subsequent file relies on:

```
user calls NS::SomeClass::someMethod(args...)
       ↓
inline wrapper body:
    return Object::sendMessage<RetType>(this, _NS_PRIVATE_SEL(someMethod), args...);
       ↓
sendMessage chooses the right objc_msgSend variant based on RetType + architecture
       ↓
objc_msgSend(receiver, sel, args...) dispatches into the Objective-C runtime
       ↓
runtime looks up the method implementation for (receiver's class, sel)
       ↓
the implementation runs and returns a value
       ↓
sendMessage returns it to the inline wrapper, which returns it to the user
```

That pipeline is the entire library. The remaining 44 files just declare typed wrappers around it.

---

## 3.7 `Foundation/NSSharedPtr.hpp` — RAII wrapper for retain/release

**Path:** `metal-cpp-main/Foundation/NSSharedPtr.hpp`
**Lines:** 324
**Why it matters:** This is the metal-cpp answer to `std::shared_ptr`. It owns an Objective-C object and calls `release()` in its destructor, so you can use RAII to manage Objective-C object lifetimes from C++.

### Section 1: Includes + class declaration (lines 21–131)

```cpp
template <class _Class>
class SharedPtr
{
public:
    SharedPtr();
    ~SharedPtr();
    SharedPtr(std::nullptr_t) noexcept;
    SharedPtr(const SharedPtr<_Class>& other) noexcept;
    template <class _OtherClass>
    SharedPtr(const SharedPtr<_OtherClass>& other, typename std::enable_if_t<std::is_convertible_v<_OtherClass*, _Class*>>* = nullptr) noexcept;
    SharedPtr(SharedPtr<_Class>&& other) noexcept;
    template <class _OtherClass>
    SharedPtr(SharedPtr<_OtherClass>&& other, typename std::enable_if_t<std::is_convertible_v<_OtherClass*, _Class*>>* = nullptr) noexcept;

    SharedPtr& operator=(const SharedPtr<_Class>& other);
    template <class _OtherClass>
    typename std::enable_if_t<std::is_convertible_v<_OtherClass*, _Class*>, SharedPtr&>
    operator=(const SharedPtr<_OtherClass>& other);
    SharedPtr& operator=(SharedPtr<_Class>&& other);
    template <class _OtherClass>
    typename std::enable_if_t<std::is_convertible_v<_OtherClass*, _Class*>, SharedPtr&>
    operator=(SharedPtr<_OtherClass>&& other);

    _Class* get() const;
    _Class* operator->() const;
    explicit operator bool() const;

    void reset();
    void detach();

    template <class _OtherClass>
    friend SharedPtr<_OtherClass> RetainPtr(_OtherClass* ptr);
    template <class _OtherClass>
    friend SharedPtr<_OtherClass> TransferPtr(_OtherClass* ptr);

private:
    _Class* m_pObject;
};
```

The API surface mirrors `std::shared_ptr`:

- **Constructors:** default (null), from `nullptr`, copy-constructor, move-constructor. Plus converting constructors that let you convert `SharedPtr<Derived>` to `SharedPtr<Base>` (the `std::enable_if_t<std::is_convertible_v<...>>` SFINAE trick enables these only when the pointer types are actually convertible).
- **Assignment operators:** copy and move, plus converting variants.
- **Observers:** `get()` returns the raw pointer; `operator->()` lets you call methods on the pointee directly (`mySharedPtr->length()`); `operator bool()` lets you write `if (mySharedPtr)`.
- **Modifiers:** `reset()` decrements the refcount and sets the pointer to null; `detach()` sets the pointer to null **without** decrementing — useful when you want to give ownership to someone else without an extra retain/release pair.
- **Friend free functions:** `RetainPtr(p)` and `TransferPtr(p)` — the two ways to construct a `SharedPtr` from a raw pointer.

### Section 2: `RetainPtr` and `TransferPtr` (lines 138–157)

```cpp
template <class _Class>
_NS_INLINE NS::SharedPtr<_Class> RetainPtr(_Class* pObject)
{
    NS::SharedPtr<_Class> ret;
    ret.m_pObject = pObject->retain();
    return ret;
}

template <class _Class>
_NS_INLINE NS::SharedPtr<_Class> TransferPtr(_Class* pObject)
{
    NS::SharedPtr<_Class> ret;
    ret.m_pObject = pObject;
    return ret;
}
```

These are the two ways to wrap a raw Objective-C pointer:

- **`RetainPtr(p)`** — *retains* the pointer (increments its refcount) and stores the result. Use this when the pointer came from a method that returns an **autoreleased** reference (most factory methods like `[NSString string]`). The retain removes it from the autorelease pool, so it lives as long as the `SharedPtr` does.
- **`TransferPtr(p)`** — does **not** retain; just takes ownership of the existing reference. Use this when the pointer came from a method that returns a **+1 / owned** reference (like `[[NSString alloc] init]`), which already has refcount 1.

Choosing the wrong one is a memory bug: `RetainPtr` on an owned pointer leaks; `TransferPtr` on an autoreleased pointer crashes when the pool drains. This is the same conceptual distinction as `std::shared_ptr`'s constructor vs. `std::shared_ptr::adopt` (which doesn't exist in the standard, but is provided by `std::auto_ptr`'s old behaviour or `boost::intrusive_ptr`'s `adopt` form).

### Section 3: The destructor (lines 167–171)

```cpp
template <class _Class>
_NS_INLINE NS::SharedPtr<_Class>::~SharedPtr<_Class>() __attribute__((no_sanitize("undefined")))
{
    m_pObject->release();
}
```

The destructor calls `release()` on the held pointer. Two things to note:

1. **The `__attribute__((no_sanitize("undefined")))`** — this disables UBSan for this function. Why? Because if you `detach()`-ed the pointer (set it to `nullptr`) and then the destructor ran, you'd be calling `release()` on a null pointer. In Objective-C's runtime, sending a message to `nil` is a documented no-op (the runtime checks for nil at the entry of `objc_msgSend` and returns zero/nil). But C++'s UBSan flags this as a null-pointer dereference because, technically, calling a member function on a null `this` is undefined behaviour in C++. The attribute tells UBSan to trust Apple's runtime behaviour and not flag it.

2. **The destructor doesn't check for null.** This is intentional — relying on the "messages to nil are no-ops" guarantee means simpler code.

### Section 4: Copy and move constructors (lines 179–205)

```cpp
template <class _Class>
_NS_INLINE NS::SharedPtr<_Class>::SharedPtr(const SharedPtr<_Class>& other) noexcept
    : m_pObject(other.m_pObject->retain())
{
}

template <class _Class>
_NS_INLINE NS::SharedPtr<_Class>::SharedPtr(SharedPtr<_Class>&& other) noexcept
    : m_pObject(other.m_pObject)
{
    other.m_pObject = nullptr;
}
```

The difference between copy and move is the heart of refcounted ownership:

- **Copy constructor:** *retains* the other pointer (refcount goes up by 1) and stores the result. Now both `this` and `other` own the object.
- **Move constructor:** *steals* the pointer (no retain) and sets `other.m_pObject = nullptr`. Now `this` owns the object; `other` is empty (and its destructor will be a no-op message-to-nil).

This is exactly how `std::shared_ptr` works internally — copy shares, move transfers.

### Section 5: `reset` and `detach` (lines 225–236)

```cpp
template <class _Class>
_NS_INLINE void NS::SharedPtr<_Class>::reset() __attribute__((no_sanitize("undefined")))
{
    m_pObject->release();
    m_pObject = nullptr;
}

template <class _Class>
_NS_INLINE void NS::SharedPtr<_Class>::detach()
{
    m_pObject = nullptr;
}
```

- **`reset()`** releases the current pointer (decrementing its refcount) and nulls the member. If the refcount hits 0, the object is deallocated.
- **`detach()`** nulls the member **without** releasing. Use when you want to hand the +1 reference off to some other code (e.g. return it as a raw pointer from a function).

### Section 6: Assignment operators (lines 238–298)

The four assignment operators (same- and cross-type, copy and move) follow the **copy-and-swap-free** pattern:

```cpp
SharedPtr& operator=(const SharedPtr<_Class>& other) __attribute__((no_sanitize("undefined")))
{
    _Class* pOldObject = m_pObject;
    m_pObject = other.m_pObject->retain();
    pOldObject->release();
    return *this;
}
```

The key subtlety: **retain the new pointer before releasing the old**. If `this` and `other` happen to point to the same object (self-assignment), and you released first, the refcount could hit 0 and the object would be deallocated before you had a chance to retain it. By retaining first you make self-assignment safe.

The move-assignment has an extra wrinkle: if the two pointers are equal, you don't want to release (it would deallocate the very thing you're trying to keep), so:

```cpp
SharedPtr& operator=(SharedPtr<_Class>&& other) __attribute__((no_sanitize("undefined")))
{
    if (m_pObject != other.m_pObject)
    {
        m_pObject->release();
        m_pObject = other.m_pObject;
    }
    else
    {
        m_pObject = other.m_pObject;
        other.m_pObject->release();
    }
    other.m_pObject = nullptr;
    return *this;
}
```

When they're equal, the code releases the **other** one (since you'll be left with `this`'s copy) and then nulls `other`. Either branch leaves you in the right state.

### Section 7: Comparison operators and `std::hash` (lines 300–324)

```cpp
template <class _ClassLhs, class _ClassRhs>
_NS_INLINE bool operator==(const NS::SharedPtr<_ClassLhs>& lhs, const NS::SharedPtr<_ClassRhs>& rhs)
{
    return lhs.get() == rhs.get();
}

template <class _ClassLhs, class _ClassRhs>
_NS_INLINE bool operator!=(const NS::SharedPtr<_ClassLhs>& lhs, const NS::SharedPtr<_ClassRhs>& rhs)
{
    return lhs.get() != rhs.get();
}

namespace std
{
template <class T>
struct hash<NS::SharedPtr<T>>
{
    size_t operator()(const NS::SharedPtr<T>& p) const
    {
        return std::hash<T*>{}(p.get());
    }
};
}
```

- `operator==` / `operator!=` compare the **raw pointers** (not the pointees). Two `SharedPtr`s are equal iff they own the same object.
- The `std::hash<NS::SharedPtr<T>>` specialisation lets you use `SharedPtr` as a key in `std::unordered_map` / `std::unordered_set`. It hashes the underlying pointer.

**Note the specialisation is in `namespace std`** — this is one of the few sanctioned places where adding to `std` is allowed by the C++ standard (you may specialise a `std` template for your own type).

---

## 3.8 `Foundation/NSPrivate.hpp` — selectors and class lookups

**Path:** `metal-cpp-main/Foundation/NSPrivate.hpp`
**Lines:** 533
**Why it matters:** This file is the **other half** of the Objective-C bridge: it caches every Objective-C class pointer and every selector the library uses. Without it, every wrapper method would need to call `objc_lookUpClass("NSString")` and `sel_registerName("length")` at runtime — slow and error-prone.

### Section 1: Includes + macro definitions (lines 21–86)

```cpp
#include <objc/runtime.h>

#define _NS_PRIVATE_CLS(symbol)  (Private::Class::s_k##symbol)
#define _NS_PRIVATE_SEL(accessor) (Private::Selector::s_k##accessor)

#if defined(NS_PRIVATE_IMPLEMENTATION)
  // ... implementation branch — defines the cached values
#else
  // ... consumer branch — just externs them
#endif
```

The two key macros:

- **`_NS_PRIVATE_CLS(symbol)`** — expands to `(Private::Class::s_k##symbol)`, i.e. `(Private::Class::s_kNSString)`. It's the cached pointer to the Objective-C `NSString` class.
- **`_NS_PRIVATE_SEL(accessor)`** — expands to `(Private::Selector::s_k##accessor)`, e.g. `(Private::Selector::s_klength)` — the cached `SEL` for the `length` selector.

The `#if defined(NS_PRIVATE_IMPLEMENTATION)` switch is a clever single-file library trick. The same header serves two purposes:

- **In a normal user's code** (no `NS_PRIVATE_IMPLEMENTATION` defined), the macros expand to `extern` declarations — the cached values are promised to exist somewhere else (in a single implementation file Apple ships, or in code you write that does `#define NS_PRIVATE_IMPLEMENTATION` before `#include`-ing the headers).
- **In the implementation file** (where `NS_PRIVATE_IMPLEMENTATION` *is* defined), the macros expand to actual definitions — the cached values are initialised by calling `objc_lookUpClass("NSString")` etc. at static-init time.

So one header declares the cache; the same header *also* defines it when included from the implementation file. No separate `.cpp` file needed.

### Section 2: The implementation branch (lines 34–77, summarised)

Inside the `#if defined(NS_PRIVATE_IMPLEMENTATION)` branch you'll see:

```cpp
#include <dlfcn.h>

namespace NS::Private
{
    template <typename _Type>
    inline _Type const LoadSymbol(const char* pSymbol)
    {
        const _Type* pAddress = static_cast<_Type*>(dlsym(RTLD_DEFAULT, pSymbol));
        return pAddress ? *pAddress : _Type();
    }
}
```

`dlsym(RTLD_DEFAULT, "NSCocoaErrorDomain")` looks up a global symbol by name at runtime. Used for the global `NSError` domain strings — they're exported from Foundation with C linkage, and the implementation file imports them into the `NS::` namespace.

Then four macros define how each cached class / selector / constant is created:

- `_NS_PRIVATE_DEF_CLS(NSString)` → `void* s_kNSString = objc_lookUpClass("NSString");`
- `_NS_PRIVATE_DEF_SEL(length, "length")` → `SEL s_klength = sel_registerName("length");`
- `_NS_PRIVATE_DEF_PRO(NSObject)` → `void* s_kNSObject = objc_getProtocol("NSObject");`
- `_NS_PRIVATE_DEF_CONST(type, symbol)` → loads a Foundation-exported constant via `dlsym`.

### Section 3: The cached class pointers (lines 90–115)

```cpp
namespace NS { namespace Private { namespace Class {
    _NS_PRIVATE_DEF_CLS(NSArray);
    _NS_PRIVATE_DEF_CLS(NSAutoreleasePool);
    _NS_PRIVATE_DEF_CLS(NSBundle);
    _NS_PRIVATE_DEF_CLS(NSCondition);
    _NS_PRIVATE_DEF_CLS(NSDate);
    _NS_PRIVATE_DEF_CLS(NSDictionary);
    _NS_PRIVATE_DEF_CLS(NSError);
    _NS_PRIVATE_DEF_CLS(NSNotificationCenter);
    _NS_PRIVATE_DEF_CLS(NSNumber);
    _NS_PRIVATE_DEF_CLS(NSObject);
    _NS_PRIVATE_DEF_CLS(NSProcessInfo);
    _NS_PRIVATE_DEF_CLS(NSSet);
    _NS_PRIVATE_DEF_CLS(NSString);
    _NS_PRIVATE_DEF_CLS(NSURL);
    _NS_PRIVATE_DEF_CLS(NSValue);
}}}
```

15 Objective-C classes are pre-registered. Each line resolves to `void* s_k<Name> = objc_lookUpClass("<Name>");` in the implementation file, or `extern void* s_k<Name>;` in a normal consumer.

> Note the small typo on line 115 — the closing `}` for the namespace is commented `} // MTL` instead of `} // NS`. A harmless comment copy-paste from the MTL version of this same file.

### Section 4: The cached selectors (lines 132–531)

This is the giant bulk of the file — about 400 selectors, alphabetically ordered, each on two lines:

```cpp
_NS_PRIVATE_DEF_SEL(length, "length");
_NS_PRIVATE_DEF_SEL(load, "load");
_NS_PRIVATE_DEF_SEL(loadAndReturnError_, "loadAndReturnError:");
_NS_PRIVATE_DEF_SEL(localizedDescription, "localizedDescription");
// ... ~400 more
```

You don't need to memorise these. What you should take away:

- **Every Objective-C selector** the Foundation wrapper files need is registered here, exactly once.
- The naming convention: `s_k<accessor>` where `<accessor>` is the selector name with `:` characters replaced by `_`. So `initWithString:` becomes `s_kinitWithString_`, `addObject:` becomes `s_kaddObject_`, `setProcessName:` becomes `s_ksetProcessName_`.
- This is why in every wrapper file you see calls like `Object::sendMessage<UInteger>(this, _NS_PRIVATE_SEL(length))` — `_NS_PRIVATE_SEL(length)` is just a one-token alias for `Private::Selector::s_klength`.

When you encounter a selector name in a wrapper file and want to know what Objective-C method it maps to, search this file for the matching `_NS_PRIVATE_DEF_SEL` line — the second argument is the actual Objective-C selector string.

---

## 3.9 `Foundation/NSString.hpp`

**Path:** `metal-cpp-main/Foundation/NSString.hpp`
**Lines:** 255
**Why it matters:** Strings are everywhere — error messages, labels, file paths, shader function names. `NS::String` is the wrapper you'll use most often.

### Section 1: Includes + the `StringEncoding` enum (lines 25–63)

```cpp
_NS_ENUM(NS::UInteger, StringEncoding) {
    ASCIIStringEncoding              = 1,
    NEXTSTEPStringEncoding           = 2,
    JapaneseEUCStringEncoding        = 3,
    UTF8StringEncoding               = 4,
    ISOLatin1StringEncoding          = 5,
    // ... etc
    UTF16BigEndianStringEncoding     = 0x90000100,
    UTF16LittleEndianStringEncoding  = 0x94000100,
    UTF32StringEncoding              = 0x8c000100,
    // ...
};
```

`StringEncoding` enumerates every string encoding `NSString` supports. You'll mostly use `UTF8StringEncoding` (value `4`). The high-bit-set values (`0x90000100` etc.) are special encoded variants for endian-specific UTF-16/UTF-32.

### Section 2: `StringCompareOptions` (lines 65–75)

```cpp
_NS_OPTIONS(NS::UInteger, StringCompareOptions) {
    CaseInsensitiveSearch           = 1,
    LiteralSearch                   = 2,
    BackwardsSearch                 = 4,
    AnchoredSearch                  = 8,
    NumericSearch                   = 64,
    DiacriticInsensitiveSearch      = 128,
    WidthInsensitiveSearch          = 256,
    ForcedOrderingSearch            = 512,
    RegularExpressionSearch         = 1024
};
```

A bitmask (note `_NS_OPTIONS`, not `_NS_ENUM`). You can OR them together: `CaseInsensitiveSearch | BackwardsSearch` searches backwards and ignores case. `RegularExpressionSearch` makes `rangeOfString` treat the pattern as a regex.

### Section 3: The `unichar` alias and the `String` class (lines 77–107)

```cpp
using unichar = unsigned short;  // 16-bit, for UTF-16 code units

class String : public Copying<String>
{
public:
    static String* string();
    static String* string(const String* pString);
    static String* string(const char* pString, StringEncoding encoding);

    static String* alloc();
    String*        init();
    String*        init(const String* pString);
    String*        init(const char* pString, StringEncoding encoding);
    String*        init(void* pBytes, UInteger len, StringEncoding encoding, bool freeBuffer);

    unichar        character(UInteger index) const;
    UInteger       length() const;

    const char*    cString(StringEncoding encoding) const;
    const char*    utf8String() const;
    UInteger       maximumLengthOfBytes(StringEncoding encoding) const;
    UInteger       lengthOfBytes(StringEncoding encoding) const;

    bool           isEqualToString(const String* pString) const;
    Range          rangeOfString(const String* pString, StringCompareOptions options) const;

    const char*    fileSystemRepresentation() const;

    String*        stringByAppendingString(const String* pString) const;
    ComparisonResult caseInsensitiveCompare(const String* pString) const;
};
```

The shape of the API is what every wrapper class looks like:

1. **Static factory methods** named after the Objective-C class-method family: `string()`, `string(str)`, `string(cstr, enc)`. Each returns an autoreleased `String*`.
2. **`alloc()` + `init()` pairs** for the lower-level alloc-then-init pattern.
3. **Instance accessors**: `character(i)`, `length()`, `cString(enc)`, `utf8String()`, `isEqualToString`, `rangeOfString`, `stringByAppendingString`, `caseInsensitiveCompare`.
4. **`fileSystemRepresentation()`** — returns a `const char*` suitable for passing to `open()` / `fopen()` on macOS (UTF-8, normalised to the canonical filesystem form).

The class inherits `Copying<String>` (which transitively inherits `Referencing<String, Object>`). So `String` automatically gets `retain`, `release`, `autorelease`, `retainCount`, and `copy`.

### Section 4: The `MTLSTR` macro and `MakeConstantString` (lines 109–116)

```cpp
#define MTLSTR(literal) (NS::String*)__builtin___CFStringMakeConstantString("" literal "")

template <std::size_t _StringLen>
[[deprecated("please use MTLSTR(str)")]] constexpr const String* MakeConstantString(const char (&str)[_StringLen])
{
    return reinterpret_cast<const String*>(__CFStringMakeConstantString(str));
}
```

A compile-time way to make an `NS::String*` from a string literal. `__builtin___CFStringMakeConstantString` is a Clang builtin that emits the literal as a constant `CFString` in the binary's read-only data segment. The result is a pointer you can pass directly to any metal-cpp API without allocating or retaining — it's a global constant.

The `MTLSTR("hello")` macro wraps this with a `(NS::String*)` cast and an empty-string concatenation trick (`"" literal ""`) that forces the argument to be a string literal at compile time (preventing misuse with non-literal `const char*`).

The older `MakeConstantString` template is kept for backward compatibility but marked `[[deprecated]]` — new code should use `MTLSTR`.

### Section 5: The inline implementations (lines 122–253)

Each method body is a one-liner. Representative sample:

```cpp
_NS_INLINE NS::String* NS::String::string(const char* pString, StringEncoding encoding)
{
    return Object::sendMessage<String*>(
        _NS_PRIVATE_CLS(NSString),
        _NS_PRIVATE_SEL(stringWithCString_encoding_),
        pString, encoding);
}

_NS_INLINE NS::UInteger NS::String::length() const
{
    return Object::sendMessage<UInteger>(this, _NS_PRIVATE_SEL(length));
}
```

The factory methods (`string(...)`) send their message to the **class** (`_NS_PRIVATE_CLS(NSString)`); the instance methods (`length()`, `character(i)`, etc.) send to `this`. This distinction — class methods vs. instance methods — is invisible in the C++ source except for whether the receiver argument is `_NS_PRIVATE_CLS(NSString)` or `this`.

You should now be able to read every line in this file without help. The remaining wrapper files all follow the same template; I'll point out only what's unique to each.

---

## 3.10 `Foundation/NSArray.hpp`

**Path:** `metal-cpp-main/Foundation/NSArray.hpp`
**Lines:** 125

A small immutable array. Note:

- **Templated `object<_Object>(index)`** at line 46:

```cpp
template <class _Object = Object>
_Object* object(UInteger index) const;
```

This lets you write `arr->object<NS::String>(0)` to get a typed pointer back, instead of having to cast from `Object*` yourself. The default template argument means `arr->object(0)` returns `Object*`, which is fine if you don't know the element type.

- **`objectEnumerator()`** at line 49 returns an `Enumerator<Object>*` — used to iterate the array without indexing. See §3.14 for enumerators.

The rest of the file is the standard wrapper pattern around `NSArray`'s selectors: `array`, `arrayWithObject:`, `arrayWithObjects:count:`, `alloc`, `init`, `initWithObjects:count:`, `initWithCoder:`, `count`, `objectAtIndex:`.

---

## 3.11 `Foundation/NSData.hpp`

**Path:** `metal-cpp-main/Foundation/NSData.hpp`
**Lines:** 55

The smallest container — just two methods:

```cpp
class Data : public Copying<Data>
{
public:
    const void* bytes() const;
    UInteger    length() const;
};
```

`bytes()` returns a read-only pointer to the raw byte buffer; `length()` returns the size in bytes. Used heavily by Metal for returning opaque blobs (e.g. `CounterHeap::resolveCounterRange` returns an `NS::Data*` containing packed `TimestampHeapEntry` structs).

---

## 3.12 `Foundation/NSDictionary.hpp`

**Path:** `metal-cpp-main/Foundation/NSDictionary.hpp`
**Lines:** 128

Immutable key-value map. Worth noting:

- **Two templated methods** for type-safe access:

```cpp
template <class _KeyType = Object>
Enumerator<_KeyType>* keyEnumerator() const;

template <class _Object = Object>
_Object* object(const Object* pKey) const;
```

So `dict->object<MTL::Device>(someKey)` returns `MTL::Device*` directly. The default template arguments mean `dict->object(someKey)` returns `Object*`.

- **`count()`** — number of key/value pairs.

---

## 3.13 `Foundation/NSSet.hpp`

**Path:** `metal-cpp-main/Foundation/NSSet.hpp`
**Lines:** 88

Immutable unordered set of unique objects. Same shape as `Array` but with `count()` and `objectEnumerator()` only — no `objectAtIndex:` because sets are unordered.

A comment at line 28 (`/*****Immutable Set******`) hints at what's not in the wrapper: `NSMutableSet`, the mutable variant, isn't exposed by metal-cpp.

---

## 3.14 `Foundation/NSEnumerator.hpp`

**Path:** `metal-cpp-main/Foundation/NSEnumerator.hpp`
**Lines:** 79

Two pieces:

### `FastEnumerationState` + `FastEnumeration` (lines 32–44)

```cpp
struct FastEnumerationState
{
    unsigned long  state;
    Object**       itemsPtr;
    unsigned long* mutationsPtr;
    unsigned long  extra[5];
} _NS_PACKED;

class FastEnumeration : public Referencing<FastEnumeration>
{
public:
    NS::UInteger countByEnumerating(FastEnumerationState* pState, Object** pBuffer, NS::UInteger len);
};
```

This models Objective-C's `NSFastEnumeration` protocol, which is what powers the `for (x in collection)` syntax. The state struct holds iteration state between calls; `countByEnumeratingWithState:objects:count:` returns a batch of items each call. metal-cpp doesn't actually expose `for(x in y)` syntax, but the protocol is here so collection subclasses inherit it.

### `Enumerator<T>` (lines 46–52)

```cpp
template <class _ObjectType>
class Enumerator : public Referencing<Enumerator<_ObjectType>, FastEnumeration>
{
public:
    _ObjectType* nextObject();
    class Array* allObjects();
};
```

A classic iterator: call `nextObject()` repeatedly until it returns `nil`. `allObjects()` returns the remaining items as an array. The template parameter lets you write `Enumerator<NS::String>*` and get `String*` back from `nextObject()`.

Usage:

```cpp
NS::Enumerator<NS::String>* e = myArray->objectEnumerator<NS::String>();
while (NS::String* s = e->nextObject()) {
    // use s
}
```

---

## 3.15 `Foundation/NSError.hpp`

**Path:** `metal-cpp-main/Foundation/NSError.hpp`
**Lines:** 173

Models `NSError`, the standard Objective-C error object.

### The error domain constants (lines 34–54)

```cpp
using ErrorDomain = class String*;

_NS_CONST(ErrorDomain, CocoaErrorDomain);
_NS_CONST(ErrorDomain, POSIXErrorDomain);
_NS_CONST(ErrorDomain, OSStatusErrorDomain);
_NS_CONST(ErrorDomain, MachErrorDomain);

using ErrorUserInfoKey = class String*;

_NS_CONST(ErrorUserInfoKey, UnderlyingErrorKey);
_NS_CONST(ErrorUserInfoKey, LocalizedDescriptionKey);
// ... 8 more well-known keys
```

`_NS_CONST(type, name)` (per §3.2) declares a constant with C linkage. These are the four standard error domains and twelve well-known keys used in the `userInfo` dictionary. The constants themselves are **defined** in the implementation file via `_NS_PRIVATE_DEF_CONST(NS::ErrorDomain, CocoaErrorDomain)` (lines 78–94) — they look up the Foundation-exported `NSCocoaErrorDomain` etc. via `dlsym` at startup.

### The `Error` class (lines 56–74)

```cpp
class Error : public Copying<Error>
{
public:
    static Error* error(ErrorDomain domain, Integer code, class Dictionary* pDictionary);

    static Error* alloc();
    Error*        init();
    Error*        init(ErrorDomain domain, Integer code, class Dictionary* pDictionary);

    Integer           code() const;
    ErrorDomain       domain() const;
    class Dictionary* userInfo() const;

    class String* localizedDescription() const;
    class Array*  localizedRecoveryOptions() const;
    class String* localizedRecoverySuggestion() const;
    class String* localizedFailureReason() const;
};
```

An `Error` has a `domain` (a `String*` like `CocoaErrorDomain`), a `code` (an integer specific to that domain), and a `userInfo` dictionary that can carry arbitrary details (the well-known keys above point to localised human-readable strings).

You'll see `NS::Error**` parameters all over the Metal API (e.g. `MTL4::Compiler::newComputePipelineState(..., NS::Error** error)`) — this is Objective-C's standard "out parameter for errors" pattern. Pass a pointer to an `NS::Error*` you own; if the call fails, it'll be set to a non-null `Error*` describing the problem.

---

## 3.16 `Foundation/NSNumber.hpp` (and `NSValue`)

**Path:** `metal-cpp-main/Foundation/NSNumber.hpp`
**Lines:** 501

The longest Foundation file, but only because it has to wrap every numeric type C++ exposes.

### `Value` (lines 33–49)

```cpp
class Value : public Copying<Value>
{
public:
    static Value* value(const void* pValue, const char* pType);
    static Value* value(const void* pPointer);

    static Value* alloc();

    Value*        init(const void* pValue, const char* pType);
    Value*        init(const class Coder* pCoder);

    void          getValue(void* pValue, UInteger size) const;
    const char*   objCType() const;

    bool          isEqualToValue(Value* pValue) const;
    void*         pointerValue() const;
};
```

`NSValue` is the Objective-C "boxed scalar" — wraps any C value (struct, pointer, primitive) into an object. You give it the bytes and the Objective-C type encoding string (e.g. `"i"` for `int`, `"{CGPoint=dd}"` for a struct). Mostly used as a building block for `NSNumber`.

### `Number` (lines 51–107)

```cpp
class Number : public Copying<Number, Value>
{
public:
    static Number* number(char value);
    static Number* number(unsigned char value);
    static Number* number(short value);
    static Number* number(unsigned short value);
    static Number* number(int value);
    // ... 13 numeric types total: signed/unsigned char/short/int/long/long long, plus float, double, bool

    static Number* alloc();

    Number* init(const class Coder* pCoder);
    Number* init(char value);
    // ... 13 init overloads to match

    char               charValue() const;
    unsigned char      unsignedCharValue() const;
    // ... 13 accessor overloads to match
    // plus integerValue(), unsignedIntegerValue(), stringValue()

    ComparisonResult   compare(const Number* pOtherNumber) const;
    bool               isEqualToNumber(const Number* pNumber) const;

    class String*      descriptionWithLocale(const Object* pLocale) const;
};
```

`Number` inherits from `Value` (`Copying<Number, Value>` — note the second template argument is `Value`, not the default `Object`). This mirrors Objective-C's `NSNumber : NSValue`.

The 13 overloads per category (factory, init, accessor) for every numeric type are why this file is 501 lines. Each is a one-line wrapper around the appropriate selector (`numberWithChar:`, `initWithChar:`, `charValue`, etc.).

**Why so many overloads?** Because C++ doesn't auto-box primitives. If you want to put an `int` into an `NSArray`, you need an `NS::Number*` first — and `NS::Number::number(42)` chooses the `int` overload by argument type. Without overloads, you'd have to write `NS::Number::numberWithInt(42)` like in Objective-C.

---

## 3.17 `Foundation/NSLock.hpp`

**Path:** `metal-cpp-main/Foundation/NSLock.hpp`
**Lines:** 117

### `Locking<T>` mixin (lines 37–43)

```cpp
template <class _Class, class _Base = class Object>
class Locking : public _Base
{
public:
    void lock();
    void unlock();
};
```

A **mixin** template: any class can inherit `Locking<Self>` to gain `lock()` and `unlock()` methods. Mirrors Objective-C's `NSLocking` protocol. Note it inherits `_Base` (defaulting to `Object`), so you can use it as a base class without losing your place in the inheritance hierarchy.

### `Condition` (lines 45–56)

```cpp
class Condition : public Locking<Condition>
{
public:
    static Condition* alloc();

    Condition* init();

    void wait();
    bool waitUntilDate(Date* pLimit);
    void signal();
    void broadcast();
};
```

A condition variable (monitor). `wait()` blocks until someone calls `signal()` or `broadcast()`. `waitUntilDate(date)` blocks until signalled or until the given date. Wraps `NSCondition`.

`NSLock.hpp` also includes `NSDate.hpp` (line 30) because `waitUntilDate` takes a `Date*`.

---

## 3.18 `Foundation/NSNotification.hpp`

**Path:** `metal-cpp-main/Foundation/NSNotification.hpp`
**Lines:** 110

The observer pattern: a `NotificationCenter` lets objects register interest in named "notifications"; when something posts a notification, all registered observers get called.

### Type aliases + the `Notification` class (lines 36–44)

```cpp
using NotificationName = class String*;

class Notification : public NS::Referencing<Notification>
{
public:
    NS::String*     name() const;
    NS::Object*     object() const;
    NS::Dictionary* userInfo() const;
};
```

A `Notification` has a `name` (a `String*`), an `object` (the poster, usually), and a `userInfo` dictionary of arbitrary details. `NotificationName` is just an alias for `String*` — used to make APIs read better.

### The callback type aliases (lines 46–47)

```cpp
using ObserverBlock         = void(^)(Notification*);
using ObserverFunction      = std::function<void(Notification*)>;
```

Two ways to express a callback:

- **`ObserverBlock`** — an Objective-C block literal `^(Notification*){...}`. Blocks are a clang extension; they capture their context and can be stored as Objective-C objects.
- **`ObserverFunction`** — a `std::function<void(Notification*)>`. Standard C++.

The class supports both because some C++ code prefers `std::function` (no Objective-C runtime dependency in the callback) and some prefers blocks (interop with existing Objective-C code).

### `NotificationCenter` (lines 49–57)

```cpp
class NotificationCenter : public NS::Referencing<NotificationCenter>
{
public:
    static class NotificationCenter* defaultCenter();
    Object* addObserver(NotificationName name, Object* pObj, void* pQueue, ObserverBlock block);
    Object* addObserver(NotificationName name, Object* pObj, void* pQueue, ObserverFunction &handler);
    void    removeObserver(Object* pObserver);
};
```

The `defaultCenter()` is the process-wide singleton (analogous to `NSNotificationCenter.defaultCenter` in Objective-C).

### The `std::function`-to-block adapter (lines 97–102)

```cpp
_NS_INLINE NS::Object* NS::NotificationCenter::addObserver(
    NS::NotificationName name, Object* pObj, void* pQueue, NS::ObserverFunction &handler)
{
    __block ObserverFunction blockFunction = handler;
    return addObserver(name, pObj, pQueue, ^(NS::Notification* pNotif) {blockFunction(pNotif);});
}
```

This is the same pattern used everywhere there's a `std::function` callback variant. The trick:

1. Capture the `std::function` into a `__block` variable. The `__block` storage modifier means the variable lives in a heap-allocated box accessible from the block.
2. Construct an Objective-C block literal `^(Notification* pNotif){ blockFunction(pNotif); }` that, when called, invokes the captured `std::function`.
3. Pass that block to the underlying Objective-C API that wants a block.

You'll see this same three-line idiom in §4.13 (`MTL4Compiler.hpp`) and §4.20 (`MTL4CommandQueue.hpp`).

---

## 3.19 `Foundation/NSDate.hpp`

**Path:** `metal-cpp-main/Foundation/NSDate.hpp`
**Lines:** 53

Tiny. Just one factory:

```cpp
using TimeInterval = double;  // re-declared here (also in NSTypes.hpp)

class Date : public Copying<Date>
{
public:
    static Date* dateWithTimeIntervalSinceNow(TimeInterval secs);
};
```

`dateWithTimeIntervalSinceNow(secs)` returns an `NS::Date` representing `now + secs` seconds. Negative `secs` is the past. Used by `NS::Condition::waitUntilDate(Date*)` (§3.17) and `ProcessInfo::performExpiringActivity` (§3.23).

---

## 3.20 `Foundation/NSAutoreleasePool.hpp`

**Path:** `metal-cpp-main/Foundation/NSAutoreleasePool.hpp`
**Lines:** 84

```cpp
class AutoreleasePool : public Object
{
public:
    static AutoreleasePool* alloc();
    AutoreleasePool*        init();

    void                    drain();

    void                    addObject(Object* pObject);

    static void             showPools();
};
```

Important concepts:

- **`AutoreleasePool`** inherits `Object` directly (not `Copying` — pools aren't copyable). It's the C++ wrapper around `NSAutoreleasePool`.
- **Lifecycle:** Create a pool at the top of a scope, `drain()` at the bottom. Any object you `autorelease()` between those points gets released when the pool drains.
- **`addObject(pObj)`** — adds an arbitrary object to the current pool manually. Rarely needed; usually objects add themselves via `autorelease`.
- **`showPools()`** — a debug helper that prints the pool stack. Useful for finding leaks.

Typical pattern in a render loop:

```cpp
NS::AutoreleasePool* pool = NS::AutoreleasePool::alloc()->init();
// ... do work that creates autoreleased objects ...
pool->drain();
```

In metal-cpp, because user code doesn't use ARC, you'll often use `NS::SharedPtr<T>` instead of explicit autorelease — but Metal framework methods still return autoreleased objects internally, so a top-of-frame pool is good practice.

---

## 3.21 `Foundation/NSURL.hpp`

**Path:** `metal-cpp-main/Foundation/NSURL.hpp`
**Lines:** 91

```cpp
class URL : public Copying<URL>
{
public:
    static URL* fileURLWithPath(const class String* pPath);

    static URL* alloc();
    URL*        init();
    URL*        init(const class String* pString);
    URL*        initFileURLWithPath(const class String* pPath);

    const char* fileSystemRepresentation() const;
};
```

Wraps `NSURL`. Mostly used for file URLs (note the dedicated `fileURLWithPath` / `initFileURLWithPath` factory and init). `fileSystemRepresentation()` returns a `const char*` suitable for POSIX file APIs.

You'll see `NS::URL*` parameters in Metal APIs like `MTL4::Compiler::newDynamicLibrary(const NS::URL* url, NS::Error**)` (§4.13) — loading a Metal dynamic library from a `.metallib` file.

---

## 3.22 `Foundation/NSBundle.hpp`

**Path:** `metal-cpp-main/Foundation/NSBundle.hpp`
**Lines:** 383

A **bundle** is Apple's term for a packaged directory with executable code + resources. Apps are bundles; frameworks are bundles; system services are bundles.

### Free functions for localised strings (lines 37–40)

```cpp
class String* LocalizedString(const String* pKey, const String*);
class String* LocalizedStringFromTable(const String* pKey, const String* pTbl, const String*);
class String* LocalizedStringFromTableInBundle(const String* pKey, const String* pTbl, const class Bundle* pBdle, const String*);
class String* LocalizedStringWithDefaultValue(const String* pKey, const String* pTbl, const class Bundle* pBdle, const String* pVal, const String*);
```

These four free functions wrap the Objective-C `NSLocalizedString` family of macros. They look up a localised string in the bundle's `.strings` files. The implementations (lines 104–128) all just call `Bundle::mainBundle()->localizedString(...)` with appropriate default arguments.

### The `Bundle` class (lines 42–94)

About 30 methods, organised into:

- **Singletons & factories:** `mainBundle()`, `bundle(path)`, `bundle(url)`, `allBundles()`, `allFrameworks()`, `alloc()`.
- **Initialisers:** `init(path)`, `init(url)`.
- **Lifecycle:** `load()`, `unload()`, `isLoaded()`, `preflightAndReturnError(err*)`, `loadAndReturnError(err*)`.
- **URL accessors:** `bundleURL`, `resourceURL`, `executableURL`, `URLForAuxiliaryExecutable`, `URLForResource`, plus paths for the same things (`bundlePath`, `resourcePath`, etc.).
- **Metadata:** `bundleIdentifier`, `infoDictionary`, `localizedInfoDictionary`, `objectForInfoDictionaryKey`, `localizedString`.

The sheer number of methods reflects how feature-rich `NSBundle` is — you can load code, find resources inside the bundle, read its `Info.plist`, fetch localised strings, etc. The wrapper pattern is identical to every other Foundation class.

### The notification constants (lines 34–35, 99–100)

```cpp
_NS_CONST(NotificationName, BundleDidLoadNotification);
_NS_CONST(NotificationName, BundleResourceRequestLowDiskSpaceNotification);
```

Two notification names: posted when a bundle finishes loading, and when the system is low on disk space for bundle resource requests. Used with `NSNotificationCenter` (§3.18).

---

## 3.23 `Foundation/NSProcessInfo.hpp`

**Path:** `metal-cpp-main/Foundation/NSProcessInfo.hpp`
**Lines:** 386

Wraps `NSProcessInfo`, the process metadata accessor. The longest of the "container" Foundation files because `ProcessInfo` exposes a lot of process and device information.

### Notification names + thermal-state enum (lines 37–46)

```cpp
_NS_CONST(NotificationName, ProcessInfoThermalStateDidChangeNotification);
_NS_CONST(NotificationName, ProcessInfoPowerStateDidChangeNotification);
_NS_CONST(NotificationName, ProcessInfoPerformanceProfileDidChangeNotification);

_NS_ENUM(NS::Integer, ProcessInfoThermalState) {
    ProcessInfoThermalStateNominal  = 0,
    ProcessInfoThermalStateFair     = 1,
    ProcessInfoThermalStateSerious  = 2,
    ProcessInfoThermalStateCritical = 3
};
```

Three notification names (thermal state changed, power state changed, performance profile changed) and a four-level thermal state enum. Metal apps use these to scale back GPU work when the device is overheating.

### `ActivityOptions` bitmask (lines 48–57)

```cpp
_NS_OPTIONS(std::uint64_t, ActivityOptions) {
    ActivityIdleDisplaySleepDisabled          = (1ULL << 40),
    ActivityIdleSystemSleepDisabled           = (1ULL << 20),
    ActivitySuddenTerminationDisabled         = (1ULL << 14),
    ActivityAutomaticTerminationDisabled      = (1ULL << 15),
    ActivityUserInitiated                     = (0x00FFFFFFULL | ActivityIdleSystemSleepDisabled),
    ActivityUserInitiatedAllowingIdleSystemSleep = (ActivityUserInitiated & ~ActivityIdleSystemSleepDisabled),
    ActivityBackground                        = 0x000000FFULL,
    ActivityLatencyCritical                  = 0xFF00000000ULL,
};
```

A 64-bit bitmask for "activity" assertions — the mechanism that lets a process tell macOS "don't go to sleep while I'm doing this". Note the high bit positions (`<< 40`, `<< 20`) — Apple reserves the top bytes for system-defined options and leaves the low bytes (`0x00FFFFFF`) for `ActivityUserInitiated`, which is a composite.

### `ProcessInfo` class (lines 66–116)

Highlights of the ~30 methods:

- **Process identity:** `processInfo()` (singleton), `arguments()`, `environment()`, `hostName()`, `processName()`, `setProcessName()`, `processIdentifier()`, `globallyUniqueString()`, `userName()`, `fullUserName()`.
- **OS version:** `operatingSystem()`, `operatingSystemVersion()` (returns the `OperatingSystemVersion` struct from §3.3), `operatingSystemVersionString()`, `isOperatingSystemAtLeastVersion(version)`.
- **Hardware:** `processorCount()`, `activeProcessorCount()`, `physicalMemory()`, `systemUptime()`.
- **Termination:** `disableSuddenTermination()`, `enableSuddenTermination()`, `disableAutomaticTermination(reason)`, `enableAutomaticTermination(reason)`, `automaticTerminationSupportEnabled()`, `setAutomaticTerminationSupportEnabled(bool)`.
- **Activity assertions:** `beginActivity(options, reason)`, `endActivity(activity)`, two `performActivity` overloads (block + `std::function`), two `performExpiringActivity` overloads.
- **State:** `thermalState()`, `isLowPowerModeEnabled()`, `isiOSAppOnMac()`, `isMacCatalystApp()`, `isDeviceCertified(tier)`, `hasPerformanceProfile(profile)`.

### The `sendMessageSafe` pattern (lines 188–384)

Many of these methods — particularly `isiOSAppOnMac`, `isMacCatalystApp`, `isLowPowerModeEnabled`, `disableSuddenTermination`, etc. — were added in newer macOS versions. So they're wrapped with `sendMessageSafe` instead of `sendMessage`:

```cpp
_NS_INLINE bool NS::ProcessInfo::isiOSAppOnMac() const
{
    return Object::sendMessageSafe<bool>(this, _NS_PRIVATE_SEL(isiOSAppOnMac));
}
```

`sendMessageSafe` (per §3.6 Section 9) checks `respondsToSelector` first; if the method doesn't exist on the current OS, it returns `false` (the zero value of `bool`) instead of crashing. This is how metal-cpp gracefully supports code that runs on multiple macOS versions.

### Performance profile constants (lines 59–64, 126–128)

```cpp
typedef NS::Integer DeviceCertification;
_NS_CONST(DeviceCertification, DeviceCertificationiPhonePerformanceGaming);

typedef NS::Integer ProcessPerformanceProfile;
_NS_CONST(ProcessPerformanceProfile, ProcessPerformanceProfileDefault);
_NS_CONST(ProcessPerformanceProfile, ProcessPerformanceProfileSustained);
```

Recent additions to `ProcessInfo` for gaming performance tiers on iPhone (a "Performance Gaming" certification) and "sustained" performance profiles (long-running high-performance work). The comment at line 124 ("The linker searches for these symbols in the Metal framework, be sure to link it in as well") is a hint that these constants live in the **Metal** framework, not Foundation — so a program using them needs to link Metal.

---

That's all 23 Foundation files. You should now be comfortable with the entire wrapper pattern. The Metal half is the same idea applied to a new vocabulary — you'll spend more time learning the domain than the code.

---

# Part 4 — Metal Files (22 files, in learning order)

## A 60-second orientation to Metal and the MTL4 generation

Before the file-by-file walkthrough, here's the mental model you need.

**What Metal actually does.** Metal is Apple's API for talking to the GPU. A typical frame of work looks like this:

1. **Acquire a command buffer** from a command queue (or, in MTL4, allocate it from a `CommandAllocator` then begin it on a queue).
2. **Get an encoder** from the buffer. Different encoders do different things: a `ComputeCommandEncoder` runs compute shaders; a `RenderCommandEncoder` draws triangles; a `BlitCommandEncoder` copies memory; an `AccelerationStructureCommandEncoder` builds ray-tracing structures.
3. **Encode work** into the encoder: bind resources (buffers, textures, samplers) to shader slots, set the pipeline state (the compiled shader program), then issue a `dispatchThreads` / `drawPrimitives` / etc. call.
4. **End the encoder.**
5. **Repeat steps 2–4** for any other work in the frame.
6. **End the command buffer** and **commit** it to the queue.
7. The GPU executes the encoded work asynchronously. The queue's `CommitFeedback` mechanism tells you when it's done.

**What the MTL4 generation changes.** MTL4 (the namespace in `Metal/*.hpp` files here) is the **Metal 4** API layer introduced with macOS 26 / iOS 26. The big idea is **argument tables** and **command allocators**: instead of binding resources per-draw at the encoder level (the MTL / Metal 3 way), MTL4 lets you pre-build an `ArgumentTable` once and reuse it. And instead of creating fresh command buffers each frame, you allocate them from a long-lived `CommandAllocator` and recycle via `reset()`.

The MTL4 wrappers coexist with the older `MTL::` (Metal 3) wrappers in the same library; you'll see `MTL::Device*` and `MTL::Buffer*` referenced from inside `MTL4::` classes. MTL4 is the new "indirect, low-overhead" path; MTL is the older "direct" path.

**How MTL4 wrappers relate to the Foundation wrappers.** Every MTL4 class ultimately inherits from `NS::Object` via `NS::Copying<T>` or `NS::Referencing<T>` — so all the machinery from §3.6 (sendMessage, retain/release) applies identically. The only new vocabulary you need:

- `_MTL_INLINE` — same as `_NS_INLINE` but defined in the MTL side of the library (it lives in `Metal/MTLDefines.hpp`, not in this zip).
- `_MTL_PRIVATE_CLS(symbol)` — same idea as `_NS_PRIVATE_CLS`, but for `MTL4Foo` Objective-C classes.
- `_MTL_PRIVATE_SEL(accessor)` — same idea, but for MTL4 selectors.
- `_MTL_ENUM`, `_MTL_OPTIONS`, `_MTL_PACKED` — MTL versions of the NS macros.
- The `namespace MTL4` (new) and `namespace MTL` (existing) live alongside `namespace NS`.

With that, the files.

---

## 4.1 `Metal/MTL4FunctionDescriptor.hpp`

**Path:** `metal-cpp-main/Metal/MTL4FunctionDescriptor.hpp`
**Lines:** 50

The simplest Metal file. Declares the base descriptor for a Metal **function** (a single shader entry point, identified by name inside a library).

```cpp
namespace MTL4
{
class FunctionDescriptor;

class FunctionDescriptor : public NS::Copying<FunctionDescriptor>
{
public:
    static FunctionDescriptor* alloc();
    FunctionDescriptor*        init();
};
}

_MTL_INLINE MTL4::FunctionDescriptor* MTL4::FunctionDescriptor::alloc()
{
    return NS::Object::alloc<MTL4::FunctionDescriptor>(_MTL_PRIVATE_CLS(MTL4FunctionDescriptor));
}

_MTL_INLINE MTL4::FunctionDescriptor* MTL4::FunctionDescriptor::init()
{
    return NS::Object::init<MTL4::FunctionDescriptor>();
}
```

Just `alloc()` and `init()`. No properties. This is the abstract base; the more useful subclasses (`LibraryFunctionDescriptor`, `BinaryFunctionDescriptor`) inherit from it and add real properties. Note the class is registered under the Objective-C name `MTL4FunctionDescriptor` (the `MTL4` prefix replaces `NS`).

The includes (lines 23–26) pull in `Foundation.hpp`, `MTLDefines.hpp`, `MTLHeaderBridge.hpp`, `MTLPrivate.hpp`. You'll see this same include set at the top of every MTL4 file — they're the MTL equivalents of the Foundation scaffolding you studied in §3.2 / §3.8.

---

## 4.2 `Metal/MTL4LibraryFunctionDescriptor.hpp`

**Path:** `metal-cpp-main/Metal/MTL4LibraryFunctionDescriptor.hpp`
**Lines:** 87

Subclass of `FunctionDescriptor` (note the second template arg to `Copying`):

```cpp
class LibraryFunctionDescriptor : public NS::Copying<LibraryFunctionDescriptor, FunctionDescriptor>
{
public:
    static LibraryFunctionDescriptor* alloc();
    LibraryFunctionDescriptor*        init();

    MTL::Library* library() const;
    NS::String*   name() const;

    void          setLibrary(const MTL::Library* library);
    void          setName(const NS::String* name);
};
```

A `LibraryFunctionDescriptor` says: "Find the function named `name` inside the `MTL::Library*`." Properties:

- `library` — a pointer to an existing `MTL::Library` (a compiled shader library). The `MTL::` prefix tells you this is a Metal 3 library object, not a new MTL4 one — MTL4 reuses the older `Library` type rather than inventing its own.
- `name` — the function's entry-point name (e.g. `"my_compute_main"`).

This `Copying<LibraryFunctionDescriptor, FunctionDescriptor>` form (with the **second** template argument set to `FunctionDescriptor`) is how metal-cpp expresses subclassing. Recall from §3.6 that the second argument defaults to `Object`; setting it to another wrapper class makes the new class a subclass of that one. So `LibraryFunctionDescriptor` IS-A `FunctionDescriptor` IS-A `Referencing<FunctionDescriptor, Object>` IS-A `Object`.

---

## 4.3 `Metal/MTL4BinaryFunctionDescriptor.hpp`

**Path:** `metal-cpp-main/Metal/MTL4BinaryFunctionDescriptor.hpp`
**Lines:** 97

```cpp
_MTL_OPTIONS(NS::UInteger, BinaryFunctionOptions) {
    BinaryFunctionOptionNone             = 0,
    BinaryFunctionOptionPipelineIndependent = 1 << 1,
};

class BinaryFunctionDescriptor : public NS::Copying<BinaryFunctionDescriptor>
{
public:
    static BinaryFunctionDescriptor* alloc();

    FunctionDescriptor*              functionDescriptor() const;

    BinaryFunctionDescriptor*        init();

    NS::String*                      name() const;

    BinaryFunctionOptions            options() const;

    void                             setFunctionDescriptor(const MTL4::FunctionDescriptor* functionDescriptor);
    void                             setName(const NS::String* name);
    void                             setOptions(MTL4::BinaryFunctionOptions options);
};
```

A **binary function** is a pre-compiled shader function — a chunk of GPU machine code shipped inside an `Archive` (§4.9), with no source needed at runtime. Used for two reasons: (a) skipping shader compile time at app startup, (b) protecting proprietary shader source code.

Properties:

- `functionDescriptor` — the original `FunctionDescriptor` (name + library pointer) that this binary function was compiled from. Lets the runtime cross-reference.
- `name` — the function's entry-point name.
- `options` — a `BinaryFunctionOptions` bitmask. Currently only one option: `PipelineIndependent` (value `2`, i.e. `1 << 1`), meaning the binary function can be used by multiple pipelines without recompilation.

Note that this class inherits **directly** from `Copying<BinaryFunctionDescriptor>` (with the default `Object` base), not from `FunctionDescriptor`. That's a slight API asymmetry — `LibraryFunctionDescriptor` inherits `FunctionDescriptor`, but `BinaryFunctionDescriptor` doesn't. Probably because a binary function isn't *found by name inside a library*; it's a self-contained blob.

---

## 4.4 `Metal/MTL4LibraryDescriptor.hpp`

**Path:** `metal-cpp-main/Metal/MTL4LibraryDescriptor.hpp`
**Lines:** 98

```cpp
class LibraryDescriptor : public NS::Copying<LibraryDescriptor>
{
public:
    static LibraryDescriptor* alloc();
    LibraryDescriptor*        init();

    NS::String*          name() const;
    MTL::CompileOptions* options() const;
    NS::String*          source() const;

    void                 setName(const NS::String* name);
    void                 setOptions(const MTL::CompileOptions* options);
    void                 setSource(const NS::String* source);
};
```

Describes how to **compile a Metal library from source**. Properties:

- `source` — the MSL (Metal Shading Language, a C++ dialect) source code as a `String*`.
- `options` — `MTL::CompileOptions*` (a Metal 3 type reused here): preprocessor defines, language version, fast-math, etc.
- `name` — an optional label for diagnostics.

You give one of these to `MTL4::Compiler::newLibrary(descriptor, error)` (§4.13) to compile source into an `MTL::Library*` at runtime.

---

## 4.5 `Metal/MTL4BinaryFunction.hpp`

**Path:** `metal-cpp-main/Metal/MTL4BinaryFunction.hpp`
**Lines:** 51

The compiled binary function itself (as opposed to its descriptor).

```cpp
class BinaryFunction : public NS::Referencing<BinaryFunction>
{
public:
    MTL::FunctionType functionType() const;
    NS::String*       name() const;
};
```

Only two read-only properties:

- `functionType` — an `MTL::FunctionType` enum value (e.g. `FunctionTypeVertex`, `FunctionTypeFragment`, `FunctionTypeCompute`, `FunctionTypeMesh`, …). Defined in the older `MTLFunction.hpp` not in this zip.
- `name` — the entry-point name.

You obtain a `BinaryFunction*` by calling `Archive::newBinaryFunction(descriptor, error)` (§4.9) or `Compiler::newBinaryFunction(...)` (§4.13). Once you have one, you can plug it into a pipeline descriptor's linking configuration.

Note this inherits `Referencing<BinaryFunction>` (not `Copying`) — binary functions aren't copyable, they're owned.

---

## 4.6 `Metal/MTL4LinkingDescriptor.hpp`

**Path:** `metal-cpp-main/Metal/MTL4LinkingDescriptor.hpp`
**Lines:** 204

Metal 4 supports **function linking** — composing a pipeline out of multiple shader functions that call each other. There are two flavours:

### Static linking (lines 34–52)

```cpp
class StaticLinkingDescriptor : public NS::Copying<StaticLinkingDescriptor>
{
public:
    static StaticLinkingDescriptor* alloc();
    NS::Array*                      functionDescriptors() const;
    NS::Dictionary*                 groups() const;
    StaticLinkingDescriptor*        init();
    NS::Array*                      privateFunctionDescriptors() const;
    void                            setFunctionDescriptors(const NS::Array* functionDescriptors);
    void                            setGroups(const NS::Dictionary* groups);
    void                            setPrivateFunctionDescriptors(const NS::Array* privateFunctionDescriptors);
};
```

Static linking is resolved at **pipeline-compile time**. You provide:

- `functionDescriptors` — an `NS::Array` of `FunctionDescriptor*`s that the pipeline's main function can call.
- `privateFunctionDescriptors` — same idea, but the functions are private (only callable from within this pipeline, not from outside).
- `groups` — an `NS::Dictionary` grouping functions by name; lets you organise call sets.

### Dynamic linking — pipeline stage (lines 53–71)

```cpp
class PipelineStageDynamicLinkingDescriptor : public NS::Copying<PipelineStageDynamicLinkingDescriptor>
{
public:
    static PipelineStageDynamicLinkingDescriptor* alloc();
    NS::Array*                                    binaryLinkedFunctions() const;
    PipelineStageDynamicLinkingDescriptor*        init();
    NS::UInteger                                  maxCallStackDepth() const;
    NS::Array*                                    preloadedLibraries() const;
    void                                          setBinaryLinkedFunctions(const NS::Array* binaryLinkedFunctions);
    void                                          setMaxCallStackDepth(NS::UInteger maxCallStackDepth);
    void                                          setPreloadedLibraries(const NS::Array* preloadedLibraries);
};
```

Dynamic linking is resolved at **draw/dispatch time** — you can swap out a linked function between draws without recompiling the pipeline. Properties:

- `binaryLinkedFunctions` — array of `BinaryFunction*`s to make available as call targets.
- `preloadedLibraries` — array of `MTL::Library*`s whose functions are available for linking.
- `maxCallStackDepth` — maximum call-stack depth for the linked functions (the GPU needs to know how much stack to allocate).

### Dynamic linking — render pipeline (lines 72–88)

```cpp
class RenderPipelineDynamicLinkingDescriptor : public NS::Copying<RenderPipelineDynamicLinkingDescriptor>
{
public:
    static RenderPipelineDynamicLinkingDescriptor* alloc();

    PipelineStageDynamicLinkingDescriptor*         fragmentLinkingDescriptor() const;
    RenderPipelineDynamicLinkingDescriptor*        init();
    PipelineStageDynamicLinkingDescriptor*         meshLinkingDescriptor() const;
    PipelineStageDynamicLinkingDescriptor*         objectLinkingDescriptor() const;
    PipelineStageDynamicLinkingDescriptor*         tileLinkingDescriptor() const;
    PipelineStageDynamicLinkingDescriptor*         vertexLinkingDescriptor() const;
};
```

A render pipeline has **multiple stages**, each with its own dynamic linking config:

- **Vertex** — runs first, transforms vertices.
- **Fragment** — runs last, produces pixel colours.
- **Mesh** — replaces vertex in mesh-shader pipelines (Metal 3+).
- **Object** — paired with mesh; runs before mesh.
- **Tile** — runs on a tile of the framebuffer (used for tile-based deferred shading).

This class is a container for five `PipelineStageDynamicLinkingDescriptor*`s, one per stage. Stages you don't care about stay `nullptr`.

The implementations (lines 91–204) are the standard wrapper pattern; each getter and setter is a one-line `Object::sendMessage<...>` call.

---

## 4.7 `Metal/MTL4ArgumentTable.hpp`

**Path:** `metal-cpp-main/Metal/MTL4ArgumentTable.hpp`
**Lines:** 187

**Key MTL4 concept.** An argument table is a pre-built binding table that says "at buffer slot 0, this `MTL::GPUAddress`. At texture slot 1, this `MTL::ResourceID`. At sampler slot 0, this `MTL::ResourceID`." You build it once, then attach it to any number of encoders, avoiding the per-draw overhead of binding resources one at a time.

### The descriptor (lines 37–65)

```cpp
class ArgumentTableDescriptor : public NS::Copying<ArgumentTableDescriptor>
{
public:
    static ArgumentTableDescriptor* alloc();
    ArgumentTableDescriptor*        init();
    bool                            initializeBindings() const;
    NS::String*                     label() const;
    NS::UInteger                    maxBufferBindCount() const;
    NS::UInteger                    maxSamplerStateBindCount() const;
    NS::UInteger                    maxTextureBindCount() const;
    void                            setInitializeBindings(bool initializeBindings);
    void                            setLabel(const NS::String* label);
    void                            setMaxBufferBindCount(NS::UInteger maxBufferBindCount);
    void                            setMaxSamplerStateBindCount(NS::UInteger maxSamplerStateBindCount);
    void                            setMaxTextureBindCount(NS::UInteger maxTextureBindCount);
    void                            setSupportAttributeStrides(bool supportAttributeStrides);
    bool                            supportAttributeStrides() const;
};
```

Properties:

- `maxBufferBindCount`, `maxTextureBindCount`, `maxSamplerStateBindCount` — how many slots of each type the table will hold. The Metal driver uses these to size internal storage.
- `initializeBindings` — if true, all bindings start zeroed out (no stale data).
- `supportAttributeStrides` — enables per-vertex-attribute strides (used for vertex-pipeline argument tables).
- `label` — for debugging.

### The argument table itself (lines 66–81)

```cpp
class ArgumentTable : public NS::Referencing<ArgumentTable>
{
public:
    MTL::Device* device() const;
    NS::String*  label() const;

    void         setAddress(MTL::GPUAddress gpuAddress, NS::UInteger bindingIndex);
    void         setAddress(MTL::GPUAddress gpuAddress, NS::UInteger stride, NS::UInteger bindingIndex);

    void         setResource(MTL::ResourceID resourceID, NS::UInteger bindingIndex);
    void         setSamplerState(MTL::ResourceID resourceID, NS::UInteger bindingIndex);
    void         setTexture(MTL::ResourceID resourceID, NS::UInteger bindingIndex);
};
```

The actual table — what you bind into:

- `setAddress(gpuAddress, index)` — bind a raw GPU virtual address (typically for a buffer) at `index`.
- `setAddress(gpuAddress, stride, index)` — variant with a per-attribute stride, for vertex attribute buffers.
- `setResource(resourceID, index)` — bind a generic `MTL::ResourceID` (an opaque handle for any GPU resource) at `index`. Used for buffers.
- `setTexture(resourceID, index)` — bind a texture by `ResourceID` at `index`.
- `setSamplerState(resourceID, index)` — bind a sampler state at `index`.

`MTL::GPUAddress` and `MTL::ResourceID` are defined in the older `MTLGPUAddress.hpp` / `MTLTypes.hpp` headers (not in this zip). A `GPUAddress` is typically a `uint64_t` representing a virtual address in the GPU's address space; a `ResourceID` is an opaque token the Metal driver hands back when you create a buffer/texture/sampler.

### The implementations (lines 84–187)

Standard one-line wrappers. Note line 151's `supportAttributeStrides()` uses `sendMessageSafe` (not `sendMessage`) — meaning this property might not exist on older OS versions, and the wrapper falls back to `false` rather than crashing.

---

## 4.8 `Metal/MTL4Counters.hpp`

**Path:** `metal-cpp-main/Metal/MTL4Counters.hpp`
**Lines:** 138

GPU performance counters, primarily **timestamps**.

### Enums and struct (lines 34–47)

```cpp
_MTL_ENUM(NS::Integer, CounterHeapType) {
    CounterHeapTypeInvalid,
    CounterHeapTypeTimestamp,
};

_MTL_ENUM(NS::Integer, TimestampGranularity) {
    TimestampGranularityRelaxed = 0,
    TimestampGranularityPrecise = 1,
};

struct TimestampHeapEntry
{
    uint64_t timestamp;
} _MTL_PACKED;
```

- `CounterHeapType` — currently two values: `Invalid` (sentinel) and `Timestamp`. Apple may add more (counters for cycles, bytes, etc.) in future OS versions.
- `TimestampGranularity` — `Relaxed` lets the GPU batch timestamp writes (cheaper, less precise); `Precise` writes immediately.
- `TimestampHeapEntry` — a single 8-byte timestamp value. Returned packed inside an `NS::Data*` when you call `CounterHeap::resolveCounterRange(range)`.

### `CounterHeapDescriptor` (lines 49–62)

```cpp
class CounterHeapDescriptor : public NS::Copying<CounterHeapDescriptor>
{
public:
    static CounterHeapDescriptor* alloc();
    NS::UInteger                  count() const;
    CounterHeapDescriptor*        init();
    void                          setCount(NS::UInteger count);
    void                          setType(MTL4::CounterHeapType type);
    CounterHeapType               type() const;
};
```

Configures a counter heap before creation: how many entries (`count`) and what type of counter (`type`).

### `CounterHeap` (lines 63–76)

```cpp
class CounterHeap : public NS::Referencing<CounterHeap>
{
public:
    NS::UInteger    count() const;
    void            invalidateCounterRange(NS::Range range);
    NS::String*     label() const;
    NS::Data*       resolveCounterRange(NS::Range range);
    void            setLabel(const NS::String* label);
    CounterHeapType type() const;
};
```

A counter heap is a GPU-side ring buffer of counter samples. You write to it (via `MTL4::CommandBuffer::writeTimestampIntoHeap(heap, index)` or `ComputeCommandEncoder::writeTimestamp(granularity, heap, index)`), then later read the values back with `resolveCounterRange(range)`. The returned `NS::Data*` contains `range.length` `TimestampHeapEntry` structs packed together; you cast `data->bytes()` to `const TimestampHeapEntry*` to read them.

`invalidateCounterRange(range)` marks a range of entries as no-longer-needed — the GPU can reclaim the storage.

---

## 4.9 `Metal/MTL4Archive.hpp`

**Path:** `metal-cpp-main/Metal/MTL4Archive.hpp`
**Lines:** 93

An **archive** is a serialised blob of precompiled Metal objects (binary functions, compute pipeline states, render pipeline states). You can save an archive to disk and load it at app startup to skip the cost of compiling shaders from source on every launch.

```cpp
class Archive : public NS::Referencing<Archive>
{
public:
    NS::String*                label() const;

    BinaryFunction*            newBinaryFunction(const MTL4::BinaryFunctionDescriptor* descriptor, NS::Error** error);

    MTL::ComputePipelineState* newComputePipelineState(const MTL4::ComputePipelineDescriptor* descriptor, NS::Error** error);
    MTL::ComputePipelineState* newComputePipelineState(const MTL4::ComputePipelineDescriptor* descriptor, const MTL4::PipelineStageDynamicLinkingDescriptor* dynamicLinkingDescriptor, NS::Error** error);

    MTL::RenderPipelineState*  newRenderPipelineState(const MTL4::PipelineDescriptor* descriptor, NS::Error** error);
    MTL::RenderPipelineState*  newRenderPipelineState(const MTL4::PipelineDescriptor* descriptor, const MTL4::RenderPipelineDynamicLinkingDescriptor* dynamicLinkingDescriptor, NS::Error** error);

    void                       setLabel(const NS::String* label);
};
```

The `newX(...)` methods are **factories** — you pass a descriptor, they return the corresponding compiled object from the archive (or set `*error` if the descriptor doesn't match anything in the archive). Note each pipeline-state method has two overloads: one without and one with a dynamic-linking descriptor — the latter resolves linked functions at fetch time.

The pipeline state objects returned (`MTL::ComputePipelineState*`, `MTL::RenderPipelineState*`) are Metal 3 types — the archive stores precompiled Metal 3 pipeline state objects, even though the descriptor that names them is an MTL4 descriptor.

---

## 4.10 `Metal/MTL4CommandAllocator.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CommandAllocator.hpp`
**Lines:** 100

```cpp
class CommandAllocatorDescriptor : public NS::Copying<CommandAllocatorDescriptor>
{
public:
    static CommandAllocatorDescriptor* alloc();
    CommandAllocatorDescriptor*        init();
    NS::String*                        label() const;
    void                               setLabel(const NS::String* label);
};

class CommandAllocator : public NS::Referencing<CommandAllocator>
{
public:
    uint64_t     allocatedSize();
    MTL::Device* device() const;
    NS::String*  label() const;
    void         reset();
};
```

A `CommandAllocator` pools the GPU memory that command buffers record into. Lifecycle:

1. Create a `CommandAllocator` (via `MTL::Device::newCommandAllocator(descriptor)` — not in this zip).
2. For each frame: get a `CommandBuffer`, call `buffer->beginCommandBuffer(allocator)` — the buffer records into memory the allocator owns.
3. After the frame is done and the GPU has finished executing, call `allocator->reset()` to recycle the memory for the next frame.

`allocatedSize()` reports how much GPU memory the allocator is currently holding; `device()` returns the owning `MTL::Device*`. The `reset()` pattern is the big MTL4 win: instead of allocating fresh command-buffer memory every frame (as in Metal 3), the allocator pools and reuses it.

The implementations (lines 62–100) are the standard wrapper.

---

## 4.11 `Metal/MTL4CommitFeedback.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CommitFeedback.hpp`
**Lines:** 62

```cpp
using CommitFeedbackHandler          = void (^)(MTL4::CommitFeedback*);
using CommitFeedbackHandlerFunction  = std::function<void(MTL4::CommitFeedback*)>;

class CommitFeedback : public NS::Referencing<CommitFeedback>
{
public:
    CFTimeInterval GPUEndTime() const;
    CFTimeInterval GPUStartTime() const;
    NS::Error*     error() const;
};
```

After you `commit()` a command buffer (or batch of buffers) to a queue, the GPU starts executing asynchronously. A `CommitFeedback` object is the call-back channel: when the GPU starts, `GPUStartTime` is filled in; when it ends, `GPUEndTime`; if anything went wrong, `error` is set.

`CFTimeInterval` is a `double` (from CoreFoundation) representing seconds. The two handlers (`CommitFeedbackHandler` = Objective-C block, `CommitFeedbackHandlerFunction` = `std::function`) are the two ways to register a callback — see §4.20 (`CommandQueue::CommitOptions::addFeedbackHandler`) for how they're wired up.

---

## 4.12 `Metal/MTL4CompilerTask.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CompilerTask.hpp`
**Lines:** 63

```cpp
_MTL_ENUM(NS::Integer, CompilerTaskStatus) {
    CompilerTaskStatusNone      = 0,
    CompilerTaskStatusScheduled = 1,
    CompilerTaskStatusCompiling = 2,
    CompilerTaskStatusFinished  = 3,
};

class CompilerTask : public NS::Referencing<CompilerTask>
{
public:
    Compiler*          compiler() const;
    CompilerTaskStatus status() const;
    void               waitUntilCompleted();
};
```

Async compilation in MTL4 works like this: you call `Compiler::newX(..., completionHandler)` (§4.13), and instead of blocking until the compile finishes, you immediately get back a `CompilerTask*`. Poll `status()` to see where the compile is (`None` / `Scheduled` / `Compiling` / `Finished`), or block on `waitUntilCompleted()`. `compiler()` returns the `Compiler*` that owns this task.

This is the same shape as `std::future` / `std::promise` — fire-and-forget with later synchronisation.

---

## 4.13 `Metal/MTL4Compiler.hpp`

**Path:** `metal-cpp-main/Metal/MTL4Compiler.hpp`
**Lines:** 345

The biggest "control" file in the Metal half. The `Compiler` class is the MTL4 entry point for creating **every kind of compiled object**: libraries, binary functions, compute pipelines, render pipelines, ML pipelines, dynamic libraries.

### Includes + forward declarations (lines 21–69)

A long block of forward declarations for every type the Compiler returns or accepts. Note especially lines 56–57:

```cpp
using NewDynamicLibraryCompletionHandler         = void (^)(MTL::DynamicLibrary*, NS::Error*);
using NewDynamicLibraryCompletionHandlerFunction = std::function<void(MTL::DynamicLibrary*, NS::Error*)>;
```

…and five more such pairs (lines 62–69) for compute pipeline, render pipeline, binary function, ML pipeline, library. Each pair defines an Objective-C block type and a matching `std::function` type — same pattern as `NSNotification`'s `ObserverBlock` / `ObserverFunction` (§3.18).

### `CompilerDescriptor` (lines 71–85)

```cpp
class CompilerDescriptor : public NS::Copying<CompilerDescriptor>
{
public:
    static CompilerDescriptor* alloc();
    CompilerDescriptor*        init();
    NS::String*                label() const;
    PipelineDataSetSerializer* pipelineDataSetSerializer() const;
    void                       setLabel(const NS::String* label);
    void                       setPipelineDataSetSerializer(const MTL4::PipelineDataSetSerializer* pipelineDataSetSerializer);
};
```

Used to configure a `Compiler` at creation time: a debug `label` and an optional `PipelineDataSetSerializer*` (a helper for serialising pipeline state into archives).

### `CompilerTaskOptions` (lines 86–95)

```cpp
class CompilerTaskOptions : public NS::Copying<CompilerTaskOptions>
{
public:
    static CompilerTaskOptions* alloc();
    CompilerTaskOptions*        init();
    NS::Array*                  lookupArchives() const;
    void                        setLookupArchives(const NS::Array* lookupArchives);
};
```

Passed to each `newX` call. The single property `lookupArchives` is an array of `MTL4::Archive*`s — precompiled archives the compiler can pull pre-built pipeline states from instead of recompiling.

### `Compiler` (lines 96–137)

This is the meat. The class has ~30 methods, all variations on **"create an X from a descriptor"**. The pattern for each creation kind is three overloads:

1. **Synchronous with error:** `T* newX(const Descriptor*, const TaskOptions*, NS::Error**)` — blocks until compile is done; returns the object directly. If compilation fails, sets `*error` and returns `nullptr`.
2. **Asynchronous with block:** `CompilerTask* newX(const Descriptor*, const TaskOptions*, void(^handler)(T*, NS::Error*))` — returns a `CompilerTask*` immediately; later calls `handler` with the result.
3. **Asynchronous with `std::function`:** `CompilerTask* newX(const Descriptor*, const TaskOptions*, const std::function<void(T*, NS::Error*)>&)` — same as #2 but takes a `std::function`.

The kinds of object you can create:

| Kind | Method prefix | Object returned |
|---|---|---|
| Binary function | `newBinaryFunction` | `MTL4::BinaryFunction*` |
| Compute pipeline state | `newComputePipelineState` | `MTL::ComputePipelineState*` |
| Dynamic library (from library) | `newDynamicLibrary(library, …)` | `MTL::DynamicLibrary*` |
| Dynamic library (from URL) | `newDynamicLibrary(url, …)` | `MTL::DynamicLibrary*` |
| Library | `newLibrary` | `MTL::Library*` |
| ML pipeline state | `newMachineLearningPipelineState` | `MTL4::MachineLearningPipelineState*` |
| Render pipeline state | `newRenderPipelineState` | `MTL::RenderPipelineState*` |
| Render pipeline state by specialisation | `newRenderPipelineStateBySpecialization` | `MTL::RenderPipelineState*` |

The "by specialisation" form (lines 132–134) takes an existing pipeline and a descriptor and produces a *specialised* variant — typically with concrete values substituted for shader constants. This is the MTL4 equivalent of Metal 3's "specialise a function constant at runtime" feature.

`device()` and `label()` (lines 99–101) return the owning device and a debug label.

### The `std::function` adapters (lines 230–340)

About ten of the methods follow the `std::function`-to-block adapter pattern (already seen in §3.18 and called out in §2.5). Representative sample:

```cpp
_MTL_INLINE MTL4::CompilerTask* MTL4::Compiler::newComputePipelineState(
    const MTL4::ComputePipelineDescriptor* pDescriptor,
    const MTL4::CompilerTaskOptions* options,
    const MTL4::NewComputePipelineStateCompletionHandlerFunction& function)
{
    __block MTL4::NewComputePipelineStateCompletionHandlerFunction blockFunction = function;
    return newComputePipelineState(pDescriptor, options,
        ^(MTL::ComputePipelineState* pPipeline, NS::Error* pError) { blockFunction(pPipeline, pError); });
}
```

Same three lines as every other `std::function` adapter:

1. Capture the `std::function` in a `__block` variable.
2. Construct an Objective-C block that calls it.
3. Forward to the block-based overload.

By the time you've seen this pattern three times (in `NSNotification`, `Compiler`, and `CommandQueue`), it should be invisible to you — it's just "how metal-cpp exposes a `std::function` callback variant whenever the underlying Objective-C API uses a block."

### Why this file is so big

345 lines sounds like a lot, but it's just:

- 70 lines of includes/forward declarations/callback-type-aliases.
- 70 lines declaring three small descriptor classes.
- 40 lines declaring `Compiler`'s 30 methods.
- 165 lines implementing those 30 methods.

Once you've read one `newX` overload, you've read them all. The interesting content is in the *design*: the consistent sync / async-block / async-`std::function` triad, and the catalogue of what kinds of objects MTL4 can compile.

---

## 4.14 `Metal/MTL4CommandEncoder.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CommandEncoder.hpp`
**Lines:** 134

Base class for all MTL4 command encoders (compute, render, ML, blit-via-compute, resource-state). Inherits `NS::Referencing<CommandEncoder>` directly (not via `Copying` — encoders aren't copyable, they're transient objects owned by their command buffer).

### `VisibilityOptions` (lines 41–45)

```cpp
_MTL_OPTIONS(NS::UInteger, VisibilityOptions) {
    VisibilityOptionNone           = 0,
    VisibilityOptionDevice         = 1,
    VisibilityOptionResourceAlias = 1 << 1,
};
```

A bitmask used by the barrier methods (below). `Device` means "visible to the whole device" (i.e. other queues); `ResourceAlias` means "visible to resources that alias the same memory."

### The `CommandEncoder` class (lines 47–73)

```cpp
class CommandEncoder : public NS::Referencing<CommandEncoder>
{
public:
    void           barrierAfterEncoderStages(MTL::Stages afterEncoderStages, MTL::Stages beforeEncoderStages, MTL4::VisibilityOptions visibilityOptions);
    void           barrierAfterQueueStages(MTL::Stages afterQueueStages, MTL::Stages beforeStages, MTL4::VisibilityOptions visibilityOptions);
    void           barrierAfterStages(MTL::Stages afterStages, MTL::Stages beforeQueueStages, MTL4::VisibilityOptions visibilityOptions);

    CommandBuffer* commandBuffer() const;
    void           endEncoding();
    void           insertDebugSignpost(const NS::String* string);
    NS::String*    label() const;
    void           popDebugGroup();
    void           pushDebugGroup(const NS::String* string);
    void           setLabel(const NS::String* label);
    void           updateFence(const MTL::Fence* fence, MTL::Stages afterEncoderStages);
    void           waitForFence(const MTL::Fence* fence, MTL::Stages beforeEncoderStages);
};
```

Three categories of method:

**Barriers** — three variants for inserting memory barriers. A barrier says "all writes from stage X must complete before any reads in stage Y begin." The three variants differ in scope:

- `barrierAfterEncoderStages` — orders work **within this encoder**. Use when one part of your compute pass writes data that the next part reads.
- `barrierAfterQueueStages` — orders work **across the queue**. Use when one command buffer's writes must be visible to a later command buffer's reads.
- `barrierAfterStages` — same idea with a different scope; the naming is a bit subtle and the Apple docs are the place to learn the exact distinction.

`MTL::Stages` is a bitmask (defined in `MTLTypes.hpp`, not in this zip) of pipeline stages like `StageVertex`, `StageFragment`, `StageCompute`, `StageMesh`, etc.

**Fences** — `updateFence(fence, afterStages)` and `waitForFence(fence, beforeStages)` together implement GPU-side synchronisation. One encoder updates a fence after its writes complete; another encoder waits on the fence before its reads begin. Fences are reusable, low-overhead sync primitives.

**Debugging** — `pushDebugGroup`/`popDebugGroup` let you label nested spans of encoded work for the Xcode GPU debugger; `insertDebugSignpost` marks a single point in time; `label` is the encoder's overall debug name.

**Lifecycle** — `commandBuffer()` returns the owning buffer; `endEncoding()` finalises the encoder (you can't encode more after this).

---

## 4.15 `Metal/MTL4MachineLearningCommandEncoder.hpp`

**Path:** `metal-cpp-main/Metal/MTL4MachineLearningCommandEncoder.hpp`
**Lines:** 66

The smallest encoder. Three methods:

```cpp
class MachineLearningCommandEncoder : public NS::Referencing<MachineLearningCommandEncoder, CommandEncoder>
{
public:
    void dispatchNetwork(const MTL::Heap* heap);
    void setArgumentTable(const MTL4::ArgumentTable* argumentTable);
    void setPipelineState(const MTL4::MachineLearningPipelineState* pipelineState);
};
```

- Inherits `CommandEncoder` via `Referencing<MachineLearningCommandEncoder, CommandEncoder>` (note the second template arg = `CommandEncoder`, the subclassing trick).
- `setPipelineState(state)` — bind a compiled ML pipeline (§4.16).
- `setArgumentTable(table)` — bind resources (input tensors, weights, output tensors) via an `ArgumentTable` (§4.7).
- `dispatchNetwork(heap)` — actually run the network. The `heap` argument is where intermediate activations get allocated; the pipeline state pre-computed its required size (see `MachineLearningPipelineState::intermediatesHeapSize` in §4.16).

That's the whole ML encoder: set state, set arguments, dispatch. The complexity is in the pipeline itself (the ML compiler produces a self-contained execution graph); the encoder just runs it.

---

## 4.16 `Metal/MTL4MachineLearningPipeline.hpp`

**Path:** `metal-cpp-main/Metal/MTL4MachineLearningPipeline.hpp`
**Lines:** 172

Three classes:

### `MachineLearningPipelineDescriptor` (lines 45–66)

```cpp
class MachineLearningPipelineDescriptor : public NS::Copying<MachineLearningPipelineDescriptor, PipelineDescriptor>
{
public:
    static MachineLearningPipelineDescriptor* alloc();
    MachineLearningPipelineDescriptor*        init();
    MTL::TensorExtents*                       inputDimensionsAtBufferIndex(NS::Integer bufferIndex);
    NS::String*                               label() const;
    FunctionDescriptor*                       machineLearningFunctionDescriptor() const;
    void                                      reset();
    void                                      setInputDimensions(const MTL::TensorExtents* dimensions, NS::Integer bufferIndex);
    void                                      setInputDimensions(const NS::Array* dimensions, NS::Range range);
    void                                      setLabel(const NS::String* label);
    void                                      setMachineLearningFunctionDescriptor(const MTL4::FunctionDescriptor* machineLearningFunctionDescriptor);
};
```

Inherits `PipelineDescriptor` (an abstract base, declared elsewhere). Configures a ML pipeline:

- `machineLearningFunctionDescriptor` — the entry-point function in the ML library.
- `inputDimensions(tensorExtents, bufferIndex)` — declares the shape of input tensors at each buffer binding slot. `MTL::TensorExtents` (not in this zip) describes a multi-dimensional shape.
- `reset()` — clears the descriptor for reuse.
- `label` — debug.

### `MachineLearningPipelineReflection` (lines 67–75)

```cpp
class MachineLearningPipelineReflection : public NS::Referencing<MachineLearningPipelineReflection>
{
public:
    static MachineLearningPipelineReflection* alloc();
    NS::Array*                                bindings() const;
    MachineLearningPipelineReflection*        init();
};
```

After compiling a ML pipeline you can optionally ask for **reflection** — metadata about the pipeline's bindings (input shapes, intermediate shapes, etc.). Returned as an `NS::Array` of binding descriptors. Useful for validating that the pipeline matches what your host code expects.

### `MachineLearningPipelineState` (lines 76–87)

```cpp
class MachineLearningPipelineState : public NS::Referencing<MachineLearningPipelineState, MTL::Allocation>
{
public:
    MTL::Device*                       device() const;
    NS::UInteger                       intermediatesHeapSize() const;
    NS::String*                        label() const;
    MachineLearningPipelineReflection* reflection() const;
};
```

The compiled pipeline state object. Note the inheritance: `NS::Referencing<MachineLearningPipelineState, MTL::Allocation>` — it inherits `MTL::Allocation` (a Metal 3 type for "objects that own GPU memory"). The interesting property is `intermediatesHeapSize()`: when you call `MachineLearningCommandEncoder::dispatchNetwork(heap)`, the heap must be at least this big — the pipeline pre-computed how much intermediate GPU memory it'll need.

---

## 4.17 `Metal/MTL4CommandBuffer.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CommandBuffer.hpp`
**Lines:** 193

### `CommandBufferOptions` (lines 51–60)

```cpp
class CommandBufferOptions : public NS::Copying<CommandBufferOptions>
{
public:
    static CommandBufferOptions* alloc();
    CommandBufferOptions*        init();
    MTL::LogState*               logState() const;
    void                         setLogState(const MTL::LogState* logState);
};
```

A small descriptor with one property: `logState`, an `MTL::LogState*` for capturing detailed GPU logs (used for debugging / profiling).

### `CommandBuffer` (lines 61–92)

```cpp
class CommandBuffer : public NS::Referencing<CommandBuffer>
{
public:
    void                           beginCommandBuffer(const MTL4::CommandAllocator* allocator);
    void                           beginCommandBuffer(const MTL4::CommandAllocator* allocator, const MTL4::CommandBufferOptions* options);

    ComputeCommandEncoder*         computeCommandEncoder();
    MTL::Device*                   device() const;
    void                           endCommandBuffer();
    NS::String*                    label() const;
    MachineLearningCommandEncoder* machineLearningCommandEncoder();
    void                           popDebugGroup();
    void                           pushDebugGroup(const NS::String* string);
    RenderCommandEncoder*          renderCommandEncoder(const MTL4::RenderPassDescriptor* descriptor);
    RenderCommandEncoder*          renderCommandEncoder(const MTL4::RenderPassDescriptor* descriptor, MTL4::RenderEncoderOptions options);

    void                           resolveCounterHeap(const MTL4::CounterHeap* counterHeap, NS::Range range, const MTL4::BufferRange bufferRange, const MTL::Fence* fenceToWait, const MTL::Fence* fenceToUpdate);
    void                           setLabel(const NS::String* label);
    void                           useResidencySet(const MTL::ResidencySet* residencySet);
    void                           useResidencySets(const MTL::ResidencySet* const residencySets[], NS::UInteger count);
    void                           writeTimestampIntoHeap(const MTL4::CounterHeap* counterHeap, NS::UInteger index);
};
```

Lifecycle of a command buffer:

1. **Begin:** `beginCommandBuffer(allocator)` (or with `options`). This links the buffer to a `CommandAllocator` (§4.10) that owns its memory.
2. **Get encoders:** `computeCommandEncoder()`, `machineLearningCommandEncoder()`, `renderCommandEncoder(descriptor)` — each call returns a fresh encoder that records work into this buffer.
3. **Encode** work via the encoders.
4. **End:** `endCommandBuffer()` finalises the buffer — no more encoders can be obtained. The buffer is now ready to commit to a queue (§4.20).

Other notable methods:

- `writeTimestampIntoHeap(heap, index)` — writes a GPU timestamp into a `CounterHeap` (§4.8) at the given index, scoped to this command buffer.
- `resolveCounterHeap(heap, range, bufferRange, fenceToWait, fenceToUpdate)` — copies counter values out of a `CounterHeap` into a buffer, with optional fence sync.
- `useResidencySet(set)` / `useResidencySets(sets, count)` — declares that this command buffer will access the resources in a residency set (a Metal 3 concept for keeping resources resident in GPU memory).
- `pushDebugGroup` / `popDebugGroup` — for the GPU debugger.
- `device()` returns the owning device; `label()` / `setLabel()` for debug.

Note that the `RenderCommandEncoder` returned by `renderCommandEncoder(...)` is **not** the `MTL4::CommandEncoder` base class — it's a `MTL4::RenderCommandEncoder` declared in `MTL4RenderCommandEncoder.hpp` (not in this zip).

---

## 4.18 `Metal/MTL4ComputePipeline.hpp`

**Path:** `metal-cpp-main/Metal/MTL4ComputePipeline.hpp`
**Lines:** 200

```cpp
class ComputePipelineDescriptor : public NS::Copying<ComputePipelineDescriptor, PipelineDescriptor>
{
public:
    static ComputePipelineDescriptor* alloc();
    FunctionDescriptor*               computeFunctionDescriptor() const;
    MTL::ContentionRelief             contentionRelief() const;
    MTL::ForwardProgressUsage         forwardProgressUsage() const;
    ComputePipelineDescriptor*        init();
    NS::UInteger                      maxTotalThreadsPerThreadgroup() const;
    bool                              optimizeForPersistentKernel() const;
    MTL::Size                         requiredThreadsPerThreadgroup() const;
    void                              reset();
    void                              setComputeFunctionDescriptor(const MTL4::FunctionDescriptor* computeFunctionDescriptor);
    void                              setContentionRelief(MTL::ContentionRelief contentionRelief);
    void                              setForwardProgressUsage(MTL::ForwardProgressUsage forwardProgressUsage);
    void                              setMaxTotalThreadsPerThreadgroup(NS::UInteger maxTotalThreadsPerThreadgroup);
    void                              setOptimizeForPersistentKernel(bool optimizeForPersistentKernel);
    void                              setRequiredThreadsPerThreadgroup(MTL::Size requiredThreadsPerThreadgroup);
    void                              setStaticLinkingDescriptor(const MTL4::StaticLinkingDescriptor* staticLinkingDescriptor);
    void                              setSupportBinaryLinking(bool supportBinaryLinking);
    void                              setSupportIndirectCommandBuffers(MTL4::IndirectCommandBufferSupportState supportIndirectCommandBuffers);
    void                              setThreadGroupSizeIsMultipleOfThreadExecutionWidth(bool threadGroupSizeIsMultipleOfThreadExecutionWidth);
    StaticLinkingDescriptor*          staticLinkingDescriptor() const;
    bool                              supportBinaryLinking() const;
    IndirectCommandBufferSupportState supportIndirectCommandBuffers() const;
    bool                              threadGroupSizeIsMultipleOfThreadExecutionWidth() const;
};
```

The compute pipeline descriptor — what you give to `Compiler::newComputePipelineState` (§4.13) to compile a compute pipeline. The properties divide into three groups:

**Shader configuration:**
- `computeFunctionDescriptor` — the entry-point function.
- `staticLinkingDescriptor` — optional static linking info (§4.6).

**Threadgroup sizing:**
- `maxTotalThreadsPerThreadgroup` — cap on threads per threadgroup.
- `requiredThreadsPerThreadgroup` — required size (must match what the shader expects).
- `threadGroupSizeIsMultipleOfThreadExecutionWidth` — performance hint: align threadgroup size to the GPU's SIMD width.

**Advanced flags:**
- `optimizeForPersistentKernel` — hint that this pipeline will be a "persistent kernel" (a long-running compute shader that loops, processing work as it arrives). The compiler can generate different code.
- `supportBinaryLinking` — allow dynamic linking with binary functions.
- `supportIndirectCommandBuffers` — `IndirectCommandBufferSupportState` enum (defined elsewhere): `NotSupported`, `Supported`, `Required`. Whether this pipeline can be invoked from an indirect command buffer (a GPU-built command list).
- `contentionRelief` / `forwardProgressUsage` — scheduling hints for the GPU's work scheduler (`MTL::ContentionRelief` and `MTL::ForwardProgressUsage` are enums defined in Metal 3 headers).

`reset()` clears the descriptor for reuse — important for MTL4's design philosophy of pre-allocating and reusing descriptor objects across frames.

The implementations (lines 87–200) are the standard wrapper pattern; nothing new. Note `supportBinaryLinking()` at line 187 uses `sendMessageSafe` (forward-compatibility).

---

## 4.19 `Metal/MTL4ComputeCommandEncoder.hpp`

**Path:** `metal-cpp-main/Metal/MTL4ComputeCommandEncoder.hpp`
**Lines:** 307

The biggest encoder file. The `ComputeCommandEncoder` does **three** kinds of work, all encoded into the same command buffer:

1. **Compute dispatches** — run a compute shader.
2. **Blits** — copy memory between buffers, between textures, between buffers and textures; fill buffers; generate mipmaps.
3. **Acceleration-structure operations** — build, refit, copy, compact ray-tracing acceleration structures.

### Class declaration (lines 56–121)

```cpp
class ComputeCommandEncoder : public NS::Referencing<ComputeCommandEncoder, CommandEncoder>
{
public:
    void buildAccelerationStructure(const MTL::AccelerationStructure*, const MTL4::AccelerationStructureDescriptor*, const MTL4::BufferRange);
    void copyAccelerationStructure(const MTL::AccelerationStructure*, const MTL::AccelerationStructure*);
    void copyAndCompactAccelerationStructure(const MTL::AccelerationStructure*, const MTL::AccelerationStructure*);
    // ... blits ...
    void copyFromBuffer(...);
    void copyFromTensor(...);
    void copyFromTexture(...);  // five overloads
    void copyIndirectCommandBuffer(...);
    void dispatchThreadgroups(MTL::Size, MTL::Size);
    void dispatchThreadgroups(MTL::GPUAddress, MTL::Size);
    void dispatchThreads(MTL::Size, MTL::Size);
    void dispatchThreads(MTL::GPUAddress);
    void executeCommandsInBuffer(...);
    void fillBuffer(...);
    void generateMipmaps(...);
    void optimizeContentsForCPUAccess(...);
    void optimizeContentsForGPUAccess(...);
    void optimizeIndirectCommandBuffer(...);
    void refitAccelerationStructure(...);
    void resetCommandsInBuffer(...);
    void setArgumentTable(const MTL4::ArgumentTable*);
    void setComputePipelineState(const MTL::ComputePipelineState*);
    void setImageblockWidth(NS::UInteger, NS::UInteger);
    void setThreadgroupMemoryLength(NS::UInteger, NS::UInteger);
    MTL::Stages stages();
    void writeCompactedAccelerationStructureSize(...);
    void writeTimestamp(MTL4::TimestampGranularity, const MTL4::CounterHeap*, NS::UInteger);
};
```

Too many methods to cover each in detail; the Apple docs are the reference. Highlights:

**Compute dispatches** — two flavours:
- `dispatchThreadgroups(threadgroupsPerGrid, threadsPerThreadgroup)` — classic Metal dispatch: you specify how many threadgroups and how many threads per threadgroup.
- `dispatchThreads(threadsPerGrid, threadsPerThreadgroup)` — the driver figures out the threadgroup count for you (more flexible).
- Both have an "indirect" variant that takes a `MTL::GPUAddress` pointing at a GPU buffer holding the dispatch dimensions — lets one compute shader dispatch another.

**Blits** — five `copyFromTexture` overloads, three `copyFromBuffer` overloads, two `copyFromTensor` overloads, plus `fillBuffer`, `generateMipmaps`, `optimizeContentsForCPUAccess`/`ForGPUAccess`, `copyIndirectCommandBuffer`, `executeCommandsInBuffer`, `resetCommandsInBuffer`. The variety is because blits can go buffer→buffer, buffer→texture, texture→buffer, texture→texture, with various sub-rectangle / slice / level / mipmap options.

**Acceleration structures** — `buildAccelerationStructure`, `copyAccelerationStructure`, `copyAndCompactAccelerationStructure`, `refitAccelerationStructure` (two overloads), `writeCompactedAccelerationStructureSize`. These are the GPU-side operations that consume the descriptors from `MTL4AccelerationStructure.hpp` (§4.22).

**Resource binding** — `setArgumentTable(table)` (the MTL4 way; §4.7), `setComputePipelineState(state)`, `setImageblockWidth(w, h)`, `setThreadgroupMemoryLength(length, index)`. Plus `stages()` returns the current `MTL::Stages` mask (used with barriers).

**Counters** — `writeTimestamp(granularity, heap, index)` writes a GPU timestamp into a counter heap, scoped to this encoder.

The implementations (lines 124–307) are the standard wrapper pattern. The selector names are *long* — e.g. `_MTL_PRIVATE_SEL(copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options_)` — but that's normal for Objective-C; selectors are interned at runtime so length doesn't cost anything at execution time.

---

## 4.20 `Metal/MTL4CommandQueue.hpp`

**Path:** `metal-cpp-main/Metal/MTL4CommandQueue.hpp`
**Lines:** 283

The top of the command-recording stack. A `CommandQueue` owns command buffers and submits them to the GPU.

### Includes, structs, enums (lines 21–93)

The file opens with several supporting types:

```cpp
_MTL_ENUM(NS::Integer, CommandQueueError) {
    CommandQueueErrorNone           = 0,
    CommandQueueErrorTimeout        = 1,
    CommandQueueErrorNotPermitted   = 2,
    CommandQueueErrorOutOfMemory    = 3,
    CommandQueueErrorDeviceRemoved  = 4,
    CommandQueueErrorAccessRevoked  = 5,
    CommandQueueErrorInternal       = 6,
};

struct UpdateSparseTextureMappingOperation { /* mode, region, level, slice, heapOffset */ } _MTL_PACKED;
struct CopySparseTextureMappingOperation   { /* sourceRegion, sourceLevel, sourceSlice, destinationOrigin, destinationLevel, destinationSlice */ } _MTL_PACKED;
struct UpdateSparseBufferMappingOperation  { /* mode, bufferRange, heapOffset */ } _MTL_PACKED;
struct CopySparseBufferMappingOperation    { /* sourceRange, destinationOffset */ } _MTL_PACKED;
```

`CommandQueueError` is the error enum returned from queue operations. The four `*MappingOperation` structs describe single operations on **sparse** resources — sparse textures and buffers let you bind only part of a heap's memory to a resource, and update which part is bound at runtime. The queue has methods to update/copy these mappings in bulk.

### `CommitOptions` (lines 95–104)

```cpp
class CommitOptions : public NS::Referencing<CommitOptions>
{
public:
    void                  addFeedbackHandler(const MTL4::CommitFeedbackHandler block);
    void                  addFeedbackHandler(const MTL4::CommitFeedbackHandlerFunction& function);
    static CommitOptions* alloc();
    CommitOptions*        init();
};
```

When you commit a batch of command buffers to a queue, you can pass a `CommitOptions` containing one or more feedback handlers. Each handler will be called when the GPU starts/ends the batch (§4.11). The block- and `std::function`-based overloads follow the standard adapter pattern.

### `CommandQueueDescriptor` (lines 105–119)

```cpp
class CommandQueueDescriptor : public NS::Copying<CommandQueueDescriptor>
{
public:
    static CommandQueueDescriptor* alloc();
    dispatch_queue_t               feedbackQueue() const;
    CommandQueueDescriptor*        init();
    NS::String*                    label() const;
    void                           setFeedbackQueue(const dispatch_queue_t feedbackQueue);
    void                           setLabel(const NS::String* label);
};
```

Used to create a queue. The interesting property is `feedbackQueue` — a Grand Central Dispatch (`dispatch_queue_t`) queue on which feedback handlers will be invoked. Default is the main queue; you can supply a background queue to keep feedback off the main thread.

### `CommandQueue` (lines 120–150)

```cpp
class CommandQueue : public NS::Referencing<CommandQueue>
{
public:
    void         addResidencySet(const MTL::ResidencySet* residencySet);
    void         addResidencySets(const MTL::ResidencySet* const residencySets[], NS::UInteger count);
    void         commit(const MTL4::CommandBuffer* const commandBuffers[], NS::UInteger count);
    void         commit(const MTL4::CommandBuffer* const commandBuffers[], NS::UInteger count, const MTL4::CommitOptions* options);
    void         copyBufferMappingsFromBuffer(...);
    void         copyTextureMappingsFromTexture(...);
    MTL::Device* device() const;
    NS::String*  label() const;
    void         removeResidencySet(const MTL::ResidencySet* residencySet);
    void         removeResidencySets(const MTL::ResidencySet* const residencySets[], NS::UInteger count);
    void         signalDrawable(const MTL::Drawable* drawable);
    void         signalEvent(const MTL::Event* event, uint64_t value);
    void         updateBufferMappings(const MTL::Buffer*, const MTL::Heap*, const MTL4::UpdateSparseBufferMappingOperation*, NS::UInteger);
    void         updateTextureMappings(const MTL::Texture*, const MTL::Heap*, const MTL4::UpdateSparseTextureMappingOperation*, NS::UInteger);
    void         wait(const MTL::Event* event, uint64_t value);
    void         wait(const MTL::Drawable* drawable);
};
```

Three groups:

**Submit work:**
- `commit(buffers, count)` — submit a batch of command buffers. They'll execute in order on the GPU.
- `commit(buffers, count, options)` — same with `CommitOptions` (for feedback handlers).

**Synchronisation:**
- `signalEvent(event, value)` — signal an `MTL::Event` with a value when the queue's work reaches this point.
- `wait(event, value)` — block the queue's work until `event` reaches `value`.
- `signalDrawable(drawable)` / `wait(drawable)` — same idea but for `MTL::Drawable`s (cross-frame sync for rendering).

Events and drawables are Metal 3 types reused here; they let MTL4 queues synchronise with each other, with Metal 3 queues, and with the display system.

**Residency sets:**
- `addResidencySet(set)` / `removeResidencySet(set)` (plus plural variants) — declare which residency sets (groups of resources that should stay GPU-resident) this queue cares about. The Metal driver keeps those resources' memory physically resident on the GPU while any queue that uses them is active.

**Sparse mappings:**
- `updateBufferMappings` / `updateTextureMappings` / `copyBufferMappingsFromBuffer` / `copyTextureMappingsFromTexture` — manipulate sparse resource mappings in bulk. Each takes a pointer to an array of the corresponding `*MappingOperation` struct and a count.

The implementations (lines 154–283) follow the standard wrapper pattern. Two deserve special note:

```cpp
_MTL_INLINE void MTL4::CommitOptions::addFeedbackHandler(const MTL4::CommitFeedbackHandlerFunction& function)
{
    __block MTL4::CommitFeedbackHandlerFunction blockFunction = function;
    addFeedbackHandler(^(MTL4::CommitFeedback* pFeedback) { blockFunction(pFeedback); });
}
```

The `std::function`-to-block adapter, fourth appearance. By now you should recognise the three-line idiom instantly.

---

## 4.21 `Metal/MTL4MeshRenderPipeline.hpp`

**Path:** `metal-cpp-main/Metal/MTL4MeshRenderPipeline.hpp`
**Lines:** 0

**Empty placeholder.** The file exists but contains no code (only the license header would be there in a real distribution — the version in this zip is completely empty).

What it's reserved for: a future **mesh-render pipeline** descriptor class, presumably mirroring `MTL4ComputePipelineDescriptor` (§4.18) but for mesh-shader rendering pipelines. Apple has the file in place so they can add the class in a future OS update without reorganising the SDK's headers.

Use the day you study this file (Day 86 in the study plan) to read about **mesh shaders** in general — they're a relatively recent GPU programming model that replaces the traditional vertex shader with two-stage (object + mesh) shaders that produce geometry more flexibly.

---

## 4.22 `Metal/MTL4AccelerationStructure.hpp` — the giant

**Path:** `metal-cpp-main/Metal/MTL4AccelerationStructure.hpp`
**Lines:** 1,395

The largest file in the library. Declares the descriptors used to build **ray-tracing acceleration structures** — the data structures that let the GPU quickly answer "what does this ray hit?"

### High-level map

The file declares **11 classes**, organised into two inheritance trees rooted at:

- `AccelerationStructureDescriptor` (lines 46–52) — base for descriptors that describe an entire acceleration structure.
- `AccelerationStructureGeometryDescriptor` (lines 53–87) — base for descriptors that describe a single piece of geometry inside a structure.

Here's the inheritance tree:

```
AccelerationStructureDescriptor (root)
├── PrimitiveAccelerationStructureDescriptor
├── InstanceAccelerationStructureDescriptor
└── IndirectInstanceAccelerationStructureDescriptor

AccelerationStructureGeometryDescriptor (root)
├── AccelerationStructureTriangleGeometryDescriptor
├── AccelerationStructureBoundingBoxGeometryDescriptor
├── AccelerationStructureMotionTriangleGeometryDescriptor
├── AccelerationStructureMotionBoundingBoxGeometryDescriptor
├── AccelerationStructureCurveGeometryDescriptor
└── AccelerationStructureMotionCurveGeometryDescriptor
```

Each "geometry" descriptor describes one shape type; each "structure" descriptor combines multiple geometries (or, for the instance variants, references to other structures).

### Why so many lines?

The file is 1,395 lines because each of the 11 classes has:

- ~30–60 lines of declaration (the class body with ~20 properties — getters and setters).
- ~30–100 lines of implementation (one inline body per property).

Multiplying that across 11 classes gives ~1,300 lines. The **pattern** is exactly what you've seen everywhere else in the library — there's nothing new code-wise. The novelty is **domain vocabulary**: curves, motion, instances, bounding boxes, primitive data buffers, etc.

I'll cover each class at a high level. Don't try to memorise every property; the goal is to recognise what each class is *for*.

### Class 1: `AccelerationStructureDescriptor` (lines 46–52)

```cpp
class AccelerationStructureDescriptor : public NS::Copying<AccelerationStructureDescriptor, MTL::AccelerationStructureDescriptor>
{
public:
    static AccelerationStructureDescriptor* alloc();
    AccelerationStructureDescriptor*        init();
};
```

The root. Inherits the Metal 3 `MTL::AccelerationStructureDescriptor` (note the second `Copying` template arg). Just `alloc()` and `init()` — no properties of its own.

### Class 2: `AccelerationStructureGeometryDescriptor` (lines 53–87)

```cpp
class AccelerationStructureGeometryDescriptor : public NS::Copying<AccelerationStructureGeometryDescriptor>
{
public:
    static AccelerationStructureGeometryDescriptor* alloc();
    bool                                            allowDuplicateIntersectionFunctionInvocation() const;
    AccelerationStructureGeometryDescriptor*        init();
    NS::UInteger                                    intersectionFunctionTableOffset() const;
    NS::String*                                     label() const;
    bool                                            opaque() const;
    BufferRange                                     primitiveDataBuffer() const;
    NS::UInteger                                    primitiveDataElementSize() const;
    NS::UInteger                                    primitiveDataStride() const;
    void                                            setAllowDuplicateIntersectionFunctionInvocation(bool);
    void                                            setIntersectionFunctionTableOffset(NS::UInteger);
    void                                            setLabel(const NS::String*);
    void                                            setOpaque(bool);
    void                                            setPrimitiveDataBuffer(const MTL4::BufferRange);
    void                                            setPrimitiveDataElementSize(NS::UInteger);
    void                                            setPrimitiveDataStride(NS::UInteger);
};
```

Common base for all geometry types. Properties every geometry has:

- `primitiveDataBuffer` — a `BufferRange` (offset+length into an `MTL::Buffer`) holding the geometry's vertex/index/etc. data.
- `primitiveDataElementSize` — byte size of one element in that buffer.
- `primitiveDataStride` — byte stride between elements (may differ from element size if there's padding).
- `opaque` — if true, the geometry is solid (rays can't pass through). A perf hint; the ray tracer can skip some checks.
- `intersectionFunctionTableOffset` — when using custom intersection functions (procedural geometry), the index into the intersection function table where this geometry's function lives.
- `allowDuplicateIntersectionFunctionInvocation` — perf hint about whether intersections can be reported multiple times.
- `label` — debug.

### Class 3: `PrimitiveAccelerationStructureDescriptor` (lines 88–117)

```cpp
class PrimitiveAccelerationStructureDescriptor : public NS::Copying<PrimitiveAccelerationStructureDescriptor, AccelerationStructureDescriptor>
{
public:
    static PrimitiveAccelerationStructureDescriptor* alloc();
    NS::Array*                                       geometryDescriptors() const;
    PrimitiveAccelerationStructureDescriptor*        init();
    MTL::MotionBorderMode                            motionEndBorderMode() const;
    float                                            motionEndTime() const;
    float                                            motionStartTime() const;
    void                                             setGeometryDescriptors(const NS::Array*);
    void                                             setMotionEndBorderMode(MTL::MotionBorderMode);
    void                                             setMotionEndTime(float);
    void                                             setMotionStartTime(float);
};
```

A primitive accel structure is built from **geometry descriptors** (the leaves of the tree). Properties:

- `geometryDescriptors` — an `NS::Array` of `AccelerationStructureGeometryDescriptor*`s. This is how you say "build me a structure containing these triangles, these bounding boxes, these curves."
- `motionStartTime` / `motionEndTime` — for motion variants (see Class 4 below), the time range over which the geometry moves.
- `motionEndBorderMode` — `MTL::MotionBorderMode` enum; how to handle rays outside the motion time range.

### Class 4: `AccelerationStructureTriangleGeometryDescriptor` (lines 119–155)

The most commonly used geometry descriptor: a triangle mesh.

Properties include `vertexBuffers` (a `BufferRange` of vertex positions), `vertexFormat` (an `MTL::AttributeFormat` like `Float3`), `vertexStride`, `indexBuffer`, `indexType` (`UInt16`/`UInt32`), `triangleCount`, `transformationMatrixBuffer` (for an optional pre-baked transform), `transformationMatrixLayout`. Together they fully describe a triangle mesh sitting in GPU memory.

### Class 5: `AccelerationStructureBoundingBoxGeometryDescriptor` (lines 158–175)

For **procedural geometry** where you provide an axis-aligned bounding box per primitive and a custom intersection function. Properties: `boundingBoxBuffer` (a `BufferRange` of `MTL::BoundingBox` structs), `boundingBoxCount`, `boundingBoxStride`.

Use this when your "primitive" isn't a triangle — e.g. an algebraic surface, a parametric curve, an SDF — and the GPU's intersection function computes the actual hit point.

### Class 6 & 7: The "Motion" variants (lines 177–233)

`AccelerationStructureMotionTriangleGeometryDescriptor` and `AccelerationStructureMotionBoundingBoxGeometryDescriptor`. Same idea as their non-motion counterparts but the buffers are arrays indexed by **motion step** — the structure stores multiple snapshots of the geometry across the motion time range, and the ray tracer interpolates. Used for motion blur and animated geometry.

### Class 8 & 9: Curve variants (lines 235–359)

`AccelerationStructureCurveGeometryDescriptor` and `AccelerationStructureMotionCurveGeometryDescriptor`. Curves are used for hair, fur, tubes, ribbons — geometry that's defined by control points and a curve type (linear, quadratic, cubic Bézier, etc.).

Properties include `controlPointBuffer`, `controlPointCount`, `controlPointFormat`, `controlPointStride`, `curveCount`, `curveStride`, `ringBuffers`, `ringCount`, `ringStride`, `curveType` (`MTL::CurveType`), `curveBasis` (`MTL::CurveBasis`), `curveEndCaps` (`MTL::CurveEndCaps`). Lots of properties because curves are flexible.

### Class 10: `InstanceAccelerationStructureDescriptor` (lines 361–402)

```cpp
class InstanceAccelerationStructureDescriptor : public NS::Copying<InstanceAccelerationStructureDescriptor, AccelerationStructureDescriptor>
{
public:
    static InstanceAccelerationStructureDescriptor* alloc();
    NS::Array*                                      instanceDescriptors() const;
    InstanceAccelerationStructureDescriptor*        init();
    void                                            setInstanceDescriptors(const NS::Array*);
};
```

An **instance** accel structure doesn't contain geometry directly; it contains **instances** of other accel structures, each with its own transform. Properties:

- `instanceDescriptors` — an `NS::Array` of `MTL::AccelerationStructureInstanceDescriptor` (a struct defined in `MTLAccelerationStructureTypes.hpp`, not in this zip) — each entry says "place accel structure X at transform Y".

This is how you ray-trace a scene made of many objects: build a primitive structure for each unique object, then an instance structure referencing them with per-instance transforms. Saves memory (each object's geometry is stored once) and lets you animate by updating transforms.

### Class 11: `IndirectInstanceAccelerationStructureDescriptor` (lines 404–425)

```cpp
class IndirectInstanceAccelerationStructureDescriptor : public NS::Copying<IndirectInstanceAccelerationStructureDescriptor, AccelerationStructureDescriptor>
{
public:
    static IndirectInstanceAccelerationStructureDescriptor* alloc();
    NS::UInteger                                            instanceCount() const;
    MTL::GPUAddress                                         instanceDescriptors() const;
    IndirectInstanceAccelerationStructureDescriptor*        init();
    void                                                    setInstanceCount(NS::UInteger);
    void                                                    setInstanceDescriptors(MTL::GPUAddress);
};
```

The "indirect" variant: instead of an `NS::Array` of instance descriptors (CPU-side), the instance descriptors live in a **GPU buffer** at the given `MTL::GPUAddress`. The count is `instanceCount`. This lets a compute shader write the instance descriptors (e.g. for culling or LOD selection) and the next frame's ray trace read them — all without round-tripping through CPU memory.

### The implementations (lines 426–end)

About 970 lines of one-line inline wrappers — the same pattern as everywhere else. You don't need to study each one; the goal of reading this section was understanding what each **class** represents, not memorising every selector name.

### How these get used

To actually build an acceleration structure you'd:

1. Create geometry descriptors for each shape (triangles, bounding boxes, curves) and fill their buffers with vertex/index/etc. data.
2. Create a `PrimitiveAccelerationStructureDescriptor`, set its `geometryDescriptors` to the array of geometries.
3. (Optional) Create an `InstanceAccelerationStructureDescriptor` referencing multiple primitive structures with per-instance transforms.
4. Call `ComputeCommandEncoder::buildAccelerationStructure(accelStruct, descriptor, scratchBuffer)` (§4.19) — the GPU builds the structure into `accelStruct`, using `scratchBuffer` as temporary storage.

For ray tracing itself, you'd then write a Metal shader function that calls `MTLIntersection(...)` against the built structure. That's outside the scope of this codebase; the descriptors here are the **host-side** setup.

---

That's all 22 Metal files. You now have a section-by-section understanding of every header in the metal-cpp library. The final part of this guide wraps up with a pattern reference, a glossary, and pointers for what to study next.

---

# Part 5 — Appendices

## Appendix A — The recurring code patterns you'll see in every file

After you've read a handful of files you'll notice the same patterns repeating. Here's a checklist so you can recognise them on sight.

### Pattern 1: The license-header box

Every file starts with 19 lines of Apache 2.0 boilerplate inside a `//----…` border:

```cpp
//-------------------------------------------------------------------------------------------------------------------------------------------------------------
//
// <path/to/file.hpp>
//
// Copyright 2020-<year> Apple Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// ...
//
//-------------------------------------------------------------------------------------------------------------------------------------------------------------
```

You can train yourself to skip these 19 lines on every file. Just scan for `#pragma once` and start there.

### Pattern 2: The include set

Foundation files start with:
```cpp
#include "NSDefines.hpp"
#include "NSObject.hpp"
#include "NSTypes.hpp"
#include "NSPrivate.hpp"   // optional, only when needed
// ... other NS headers as needed
```

MTL4 files start with:
```cpp
#include "../Foundation/Foundation.hpp"
#include "MTLDefines.hpp"
#include "MTLHeaderBridge.hpp"
#include "MTLPrivate.hpp"
// ... other MTL headers as needed
```

The Foundation include pulls in the entire `NS::` scaffolding; the MTL includes pull in the MTL equivalents (which live in `Metal/MTLDefines.hpp` etc., not in this zip).

### Pattern 3: The wrapper-class declaration

A typical wrapper class looks like:

```cpp
namespace NS   // or namespace MTL4
{
class Foo : public Copying<Foo>   // or Referencing<Foo>
{
public:
    // Static factory methods (map to +classMethod)
    static Foo*   foo(...);
    static Foo*   alloc();

    // Instance initialisers (map to -init...)
    Foo*          init();
    Foo*          init(...);

    // Property getters (map to -property)
    Type          property() const;

    // Property setters (map to -setProperty:)
    void          setProperty(Type value);

    // Methods (map to -doSomethingWithArg:)
    ReturnType    doSomething(ArgType arg);
};
}
```

Two inheritance choices to note:

- **`Copying<T>`** — inherits `Referencing<T>` and adds `copy()`. Used for immutable value-like classes (`String`, `Array`, `Number`, descriptors, …).
- **`Referencing<T>`** — just retain/release/autorelease/retainCount. Used for non-copyable classes (`Object`, `AutoreleasePool`, encoders, compiled pipeline states, …).

For subclassing within the wrapper hierarchy, the second template argument is used:

- `class Number : public Copying<Number, Value>` — `Number` is a subclass of `Value`.
- `class MachineLearningCommandEncoder : public Referencing<MachineLearningCommandEncoder, CommandEncoder>` — subclass of `CommandEncoder`.

### Pattern 4: The inline method definition

Every method body is a single `_NS_INLINE` (or `_MTL_INLINE`) function defined outside the class:

```cpp
_NS_INLINE NS::UInteger NS::String::length() const
{
    return Object::sendMessage<UInteger>(this, _NS_PRIVATE_SEL(length));
}
```

Three forms depending on whether the call is a class method or instance method:

```cpp
// Class method (factory) — receiver is the cached class pointer
return Object::sendMessage<T*>(_NS_PRIVATE_CLS(NSString), _NS_PRIVATE_SEL(string), args...);

// Instance method — receiver is `this`
return Object::sendMessage<T>(this, _NS_PRIVATE_SEL(length));

// Safe instance method — checks respondsToSelector first
return Object::sendMessageSafe<T>(this, _NS_PRIVATE_SEL(isiOSAppOnMac));
```

If you see `sendMessageSafe` instead of `sendMessage`, the method may not exist on the current OS version — the wrapper will return a zero value instead of crashing.

### Pattern 5: The selector name → C++ symbol convention

Selectors are cached in `NSPrivate.hpp` (or `MTLPrivate.hpp`) under the name `s_k<accessor>`, where `<accessor>` is the Objective-C selector with `:` replaced by `_`:

| Objective-C selector | Cached as |
|---|---|
| `length` | `s_klength` |
| `init` | `s_kinit` |
| `initWithString:` | `s_kinitWithString_` |
| `setProcessName:` | `s_ksetProcessName_` |
| `copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:` | `s_kcopyFromBuffer_sourceOffset_toBuffer_destinationOffset_size_` |

When reading a wrapper, the selector name is usually self-evident from the C++ method name. If you ever need to look up the actual Objective-C selector string, search `NSPrivate.hpp` / `MTLPrivate.hpp` for `_NS_PRIVATE_DEF_SEL(<accessor>, "selector:string:here:")`.

### Pattern 6: The `std::function`-to-block adapter

Whenever the underlying Objective-C API takes a block callback, the C++ wrapper exposes two overloads: one taking the block, one taking a `std::function`. The `std::function` version is always the same three lines:

```cpp
__block SomeHandlerFunction blockFunction = function;
return someMethod(args, ^(Args... args) { blockFunction(args...); });
```

You've seen this in `NSNotification::addObserver`, `ProcessInfo::performActivity`, every `Compiler::newX` async overload, and `CommandQueue::CommitOptions::addFeedbackHandler`. By the fifth time you see it, it should be invisible.

### Pattern 7: Enums vs. options

- `_NS_ENUM(type, name)` — for mutually-exclusive values (`ComparisonResult`, `ProcessInfoThermalState`, `CompilerTaskStatus`). Strongly typed.
- `_NS_OPTIONS(type, name)` — for bit-flags you can OR together (`StringCompareOptions`, `ActivityOptions`, `VisibilityOptions`). Declared as a `using` alias plus an anonymous enum.

### Pattern 8: Forward-declared return types

You'll see return types like `class String* description() const;` in `NSObject.hpp`. The `class` keyword in front is a forward declaration: "this method returns a pointer to a class named `String` that's declared elsewhere." Used when including the full header for the return type would create a circular dependency.

### Pattern 9: The `__attribute__((no_sanitize("undefined")))` annotations

Used on `NSSharedPtr`'s destructor and assignment operators. Tells UBSan to ignore null-this-pointer calls inside the function — because Objective-C's "messages to nil are no-ops" guarantee makes them safe even though C++ technically forbids them.

### Pattern 10: The CRTP (Curiously Recurring Template Pattern)

Every wrapper class inherits `Referencing<Self>` or `Copying<Self>` — passing itself as the first template argument. This lets `retain()` return `Self*` (e.g. `String*`) instead of `Referencing*`, which means chained calls like `myString->retain()->length()` type-check.

---

## Appendix B — Glossary

| Term | Meaning |
|---|---|
| **ABI** | Application Binary Interface — the binary-level conventions for how arguments are passed, return values are returned, structs are laid out, etc. Two pieces of code with compatible ABIs can call each other. |
| **ARC** | Automatic Reference Counting — an Objective-C compiler feature where the compiler inserts `retain`/`release` calls automatically. metal-cpp is *not* compiled under ARC. |
| **Argument table** | (MTL4) A pre-built binding table that maps shader slots to GPU resources. Built once, attached to many encoders. |
| **Autorelease pool** | A stack of objects scheduled for `release` at the end of the current event-loop tick. Created with `NS::AutoreleasePool::alloc()->init()`, drained with `drain()`. |
| **Binary function** | A pre-compiled shader function, shipped inside an `Archive` as a blob of GPU machine code. Skips runtime compilation. |
| **Block (Objective-C)** | A closure-like language feature: `^(args){ body }`. Captures its context. Stored as a heap object. |
| **Bundle** | A macOS/iOS packaged directory (`.app`, `.framework`, etc.) containing code + resources. `NS::Bundle` exposes it. |
| **Command allocator** | (MTL4) Pools GPU memory for command buffers. `reset()` between frames recycles the memory. |
| **Command buffer** | A recorded sequence of GPU commands. Encoded into by encoders, submitted to a queue. |
| **Command encoder** | An object that records commands into a command buffer. Different encoders for compute / render / blit / ML / ray-tracing. |
| **Command queue** | Submits command buffers to the GPU. Owns the execution order. |
| **Compile options** | (Metal 3) Settings passed to the shader compiler: preprocessor defines, language version, fast-math, etc. Reused by MTL4. |
| **Compiler** | (MTL4) The object that turns descriptors into compiled pipeline states, libraries, binary functions. Supports sync and async (CompilerTask) overloads. |
| **Copying** | A wrapper template that adds `copy()` to its subclasses. Models Objective-C's `NSCopying` protocol. |
| **Counter heap** | A GPU-side ring buffer of counter samples (currently only timestamps). |
| **CRTP** | Curiously Recurring Template Pattern — `class T : public Base<T>`. metal-cpp uses it everywhere so wrapper methods can return `T*` instead of `Base*`. |
| **Descriptor** | A configuration object — fill in its properties, hand it to a factory, get back the compiled/created object. The dominant pattern in modern GPU APIs (Vulkan, D3D12, Metal). |
| **Dispatch** | Running a compute shader — `dispatchThreads(threadsPerGrid, threadsPerThreadgroup)` or `dispatchThreadgroups(...)`. |
| **Drawable** | A displayable surface (a frame of the screen). Render pipelines draw into drawables. |
| **Dynamic library** | (Metal) A loaded Metal library that can be linked against at runtime — other shaders can call its functions. |
| **Encoder** | See "Command encoder". |
| **Event** | A GPU-side sync primitive. `signalEvent(event, value)` / `wait(event, value)`. Used to synchronise between queues and between GPU and CPU. |
| **Fast enumeration** | Objective-C's iteration protocol — what powers `for (x in collection)`. Modelled in C++ by `NSEnumerator`. |
| **Fence** | A lightweight GPU sync primitive scoped to a single command queue. `updateFence`/`waitForFence`. |
| **Function** | (Metal) A single shader entry point inside a library. Has a name and a type (vertex, fragment, compute, mesh, …). |
| **Heap** | (Metal) A pool of GPU memory from which buffers, textures, etc. can be sub-allocated. Cheaper than allocating each resource separately. |
| **Indirect command buffer** | A command buffer whose commands are written by the GPU (rather than by the CPU). Used for GPU-driven rendering. |
| **Instance** | (Ray tracing) A reference to another acceleration structure with a per-instance transform. Lets you ray-trace scenes made of repeated objects. |
| **Library** | (Metal) A compiled collection of Metal functions. Created from source (`Compiler::newLibrary(LibraryDescriptor*)`) or loaded from a `.metallib` file. |
| **Linking** | (MTL4) Composing a pipeline out of multiple functions that call each other. Static = resolved at compile time; dynamic = resolved at draw time. |
| **MSL** | Metal Shading Language — a C++ dialect for writing Metal shaders. Documented in Apple's "Metal Shading Language Specification". |
| **MTL** | The Metal 3 API namespace (`MTL::Device`, `MTL::Buffer`, …). Reused inside MTL4 — many MTL4 classes return or accept MTL types. |
| **MTL4** | The Metal 4 API namespace (`MTL4::Compiler`, `MTL4::CommandQueue`, …). Introduced with macOS 26 / iOS 26. Adds argument tables, command allocators, async compilation, etc. |
| **Objective-C runtime** | The C library that implements Objective-C's dynamic dispatch. Functions like `objc_msgSend`, `objc_lookUpClass`, `sel_registerName`. The bridge metal-cpp uses to talk to Objective-C from C++. |
| **Pipeline state** | The compiled, ready-to-use shader program plus its configuration (vertex layout, blend modes, threadgroup size, etc.). Building one is expensive; cache it. |
| **Primitive** | (Ray tracing) A single triangle / bounding box / curve — the lowest-level element of an acceleration structure. |
| **Referencing** | The wrapper template that adds `retain`/`release`/`autorelease`/`retainCount`. Models Objective-C's `NSObject` protocol. |
| **Reflection** | Metadata about a compiled pipeline's bindings, returned after compilation. Useful for validating that a pipeline matches what your host code expects. |
| **Residency set** | (Metal 3) A group of resources the driver should keep GPU-resident while specified queues are active. |
| **ResourceID** | An opaque token the Metal driver hands back when you create a buffer/texture/sampler. Used in MTL4 argument tables. |
| **Retain count** | The number of strong references to an Objective-C object. `retain` increments, `release` decrements; at 0, the object is deallocated. |
| **Selector** (SEL) | An interned C string naming an Objective-C method (e.g. `"length"`, `"initWithString:"`). Cached at startup in `NSPrivate.hpp` / `MTLPrivate.hpp`. |
| **Sparse resource** | A buffer or texture whose backing memory is mapped in pieces from a heap. Lets you allocate huge virtual resources and only back the parts you use. |
| **Static linking** | (MTL4) Linking resolved at pipeline-compile time — the linked functions are baked into the pipeline state. |
| **Threadgroup** | (Metal compute) A group of threads that share tile memory and synchronise via barriers. The unit of GPU work dispatch. |
| **UBSan** | Undefined Behaviour Sanitizer — a Clang runtime that flags UB at runtime. Disabled in a few metal-cpp functions that rely on Objective-C's nil-messaging guarantee. |
| **Visibility options** | (MTL4) A bitmask controlling who can see writes after a barrier — just this device, or also aliased resources. |

---

## Appendix C — Where to go next

By Day 100 you'll have read every line of every header in this zip. The natural next steps are:

### 1. Read the headers not in this zip

This zip contains only the MTL4 (Metal 4) generation headers plus the Foundation scaffolding. The full metal-cpp distribution also includes:

- **`Foundation/`** — same as here, sometimes with a few extra classes.
- **`Metal/MTL*.hpp`** (without the `4`) — the **Metal 3** wrappers (`MTL::Device`, `MTL::Buffer`, `MTL::Texture`, `MTL::Library`, `MTL::CompileOptions`, `MTL::ComputePipelineState`, `MTL::RenderPipelineState`, …). Roughly 100 more headers. The MTL4 wrappers reference these constantly; you'll need them to actually use MTL4.
- **`Metal/MTLDefines.hpp`** — the MTL versions of the macros from §3.2.
- **`Metal/MTLPrivate.hpp`** — the MTL version of the selector cache from §3.8.
- **`Metal/CA/*.hpp`** — CoreAnimation wrappers (`CAMetalLayer`, etc.) for displaying rendered frames.
- **`Metal/QuartzCore/*.hpp`** — wrappers for display sync.

Download the full distribution from <https://developer.apple.com/metal/cpp/> (search "metal-cpp" on Apple's developer site).

### 2. Write a real program

Reading is necessary but not sufficient. Pick a small, achievable goal:

- **Day 101–110: Hello, Triangle.** Open a `CAMetalLayer`, create an `MTL::Device`, build a render pipeline that draws a single hardcoded triangle. (Use the older MTL API for this — MTL4's render pipeline isn't fully exposed in this zip.)
- **Day 111–125: Compute shader.** Allocate a buffer, run a compute shader that fills it with values, read the values back to CPU. Use `MTL4::Compiler`, `MTL4::ComputePipelineDescriptor`, `MTL4::CommandQueue`, `MTL4::CommandBuffer`, `MTL4::ComputeCommandEncoder`.
- **Day 126–150: Image processing.** Load a texture, run a compute shader that transforms it (e.g. grayscale, blur, edge detection), display the result. Add a counter heap to measure GPU time.
- **Day 151–180: Ray tracing.** Build a triangle acceleration structure, write a ray-gen + closest-hit shader, render a simple scene.

Apple's sample code repository at <https://github.com/apple/sample-code> has Metal examples (mostly Objective-C, but translatable to metal-cpp).

### 3. Read the official Apple docs

The conceptual documentation for Metal (most of which applies to MTL4 too):

- **Metal Programming Guide** — start here. Covers devices, command queues, command buffers, encoders, pipelines, resources.
- **Metal Shading Language Specification** — the C++ dialect you write shaders in. Free PDF from Apple.
- **Metal Best Practices** — performance guidance.
- **Ray Tracing with Metal** — the ray-tracing tutorial series.

Search "Metal programming guide" on Apple's developer site.

### 4. Read the Objective-C runtime docs

To deepen your understanding of the bridge:

- **"Objective-C Runtime Programming Guide"** — Apple's docs on `objc_msgSend`, `objc_lookUpClass`, `sel_registerName`, the metaclass hierarchy.
- **`<objc/message.h>` and `<objc/runtime.h>` man pages** — the actual C functions and their contracts.

### 5. Compare with similar libraries

metal-cpp is one of several "C++ wrappers around a C/Objective-C API" libraries. Studying others will deepen your sense of the design space:

- **`objc-obj`** — Apple's older C++ wrappers around Foundation, used internally.
- **`vulkan-hpp`** — the official C++ wrapper for Vulkan. Same problem (wrap a C API in C++ types) but for a non-Apple GPU API. Compare its `DispatchLoaderStatic` pattern with metal-cpp's `objc_msgSend` dispatch.
- **`d3d12.h` + `winrt`** — Microsoft's C++/WinRT wrappers around Direct3D 12.

### 6. Practice reading other people's C++

metal-cpp uses a lot of advanced C++ idioms (CRTP, SFINAE, variadic templates, `constexpr`, `__attribute__`). Once you can read it fluently, you can read most production C++ code. Try:

- **`libc++`** — LLVM's standard library implementation.
- **`folly`** — Facebook's C++ library (lots of advanced tricks).
- **`abseil-cpp`** — Google's C++ library.

These will feel familiar now.

### 7. Get unstuck

If you hit a wall during the 100 days:

- **Re-read the matching section in this guide.** The explanations are written to be re-readable; you'll catch details you missed the first time.
- **Search the official Apple developer forums** (`developer.apple.com/forums`) for the class or selector name. Many questions have been asked before.
- **Find the Objective-C original.** Every `NS::X` class wraps an `NSX` Objective-C class; the Objective-C docs are usually more thorough than the C++ wrapper's headers. Search "Apple developer documentation NSString" etc.
- **File your own notes.** The act of writing down what confused you (and how you eventually understood it) is one of the most effective learning techniques there is.

---

## Final word

The metal-cpp library looks intimidating — 8,500 lines of advanced C++ across 45 files. But it's a **shallow** library: nearly every line is a one-liner wrapper around the same `Object::sendMessage<T>(this, _NS_PRIVATE_SEL(...))` call. Once you understand the wrapper pattern (§3.6) and the cached-class / cached-selector mechanism (§3.8), the only thing left to learn is **the domain** — Foundation classes, Metal concepts, ray-tracing vocabulary.

That's why this guide spends more time on *what each class is for* than on *what each line of code does*. The code is mechanical; the concepts are what matter.

Study a little every day. Don't binge. Re-read. Write your own notes. By Day 100 you'll have a durable mental model of how Apple's GPU stack fits together — and the same model will transfer to Vulkan, Direct3D 12, WebGPU, and every other modern GPU API you encounter afterwards.

Good luck.

— End of guide —
