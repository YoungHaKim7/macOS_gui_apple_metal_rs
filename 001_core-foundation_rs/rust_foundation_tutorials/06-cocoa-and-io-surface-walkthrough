# File 6 — `cocoa/` + `cocoa-foundation/` + `io-surface/` Walkthrough (14 files)

**File 6 of 8 — the 14 files in `cocoa/`, `cocoa-foundation/`, and `io-surface/`.**

> **What these crates are:**
> - **`cocoa`** (9 files, ~7,263 lines): Wrappers around Apple's Cocoa framework — AppKit (`NSWindow`, `NSView`, `NSApplication`, etc.) and QuartzCore (`CALayer`).
> - **`cocoa-foundation`** (4 files, ~1,906 lines): A small shim re-exporting Foundation constants with some Cocoa-specific extensions.
> - **`io-surface`** (1 file, ~233 lines): Wraps `IOSurface` — shared GPU/CPU memory surfaces used for cross-process image transfer.
>
> **⚠️ Important:** Both `cocoa` and `io-surface` are **deprecated** in favour of the newer `objc2` crates. The deprecation notice is at the top of `cocoa/src/lib.rs` and `io-surface/src/lib.rs`. We're studying them because (a) they're in the zip you uploaded, (b) the patterns are still educational, (c) lots of existing code still uses them.
>
> **Different pattern from Phase 3–5:** Cocoa wrappers use the `objc` crate's `msg_send!` macro to call Objective-C methods, rather than the `declare_TCFType!`/`impl_TCFType!` pattern from core-foundation. This is because Cocoa is Objective-C (not C like Core Foundation), so calling methods requires `objc_msgSend` — the same dispatch mechanism as metal-cpp.

---

## Table of Contents

- 6.1 `cocoa/src/lib.rs` — crate root (deprecated)
- 6.2 `cocoa/src/macros.rs` — the `delegate!` macro
- 6.3 `cocoa/src/appkit.rs` — AppKit bindings (massive, ~5000 lines)
- 6.4 `cocoa/src/quartzcore.rs` — Core Animation bindings
- 6.5 `cocoa-foundation/src/lib.rs` + `base.rs` + `foundation.rs` — Foundation shim
- 6.6 `cocoa-foundation/tests/foundation.rs` — tests
- 6.7 `io-surface/src/lib.rs` — `IOSurface`
- 6.8 `cocoa/examples/hello_world.rs` — **a complete working Cocoa app in Rust**
- 6.9 `cocoa/examples/color.rs`, `fullscreen.rs`, `tab_view.rs`, `nsvisualeffectview_blur.rs`
- 6.10 Summary

---

## 6.1 `cocoa/src/lib.rs` — crate root (deprecated)

**Path:** `cocoa/src/lib.rs` (23 lines)

```rust
//! This crate has been deprecated in favour of the `objc2` crates.
#![crate_name = "cocoa"]
#![crate_type = "rlib"]
#![allow(non_snake_case, deprecated)]

#[cfg(target_os = "macos")]
pub mod appkit;
pub use cocoa_foundation::base;
pub use cocoa_foundation::foundation;
#[cfg(target_os = "macos")]
pub mod quartzcore;
#[macro_use]
mod macros;
```

Three things:

1. The deprecation notice in the doc comment.
2. The `#![allow(... deprecated)]` attribute — lets the crate compile without warnings about its own deprecated status.
3. Module declarations. `appkit` and `quartzcore` are macOS-only (Cocoa isn't on iOS — UIKit is, but this crate doesn't wrap UIKit). `base` and `foundation` are re-exported from `cocoa-foundation` rather than declared locally.

The `#[macro_use]` attribute on `mod macros;` imports the macros from `macros.rs` into the crate's macro namespace.

---

## 6.2 `cocoa/src/macros.rs` — the `delegate!` macro

**Path:** `cocoa/src/macros.rs` (80 lines)

Defines a single macro, `delegate!`, for creating Objective-C delegate classes from Rust. The pattern:

```rust
delegate!("MyWindowDelegate", {
    window: id = my_window,                        // instance variable
    (onWindowWillEnterFullscreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id)  // method
});
```

The macro expands to code that:

1. Creates a new Objective-C class declaration (`objc::declare::ClassDecl::new`).
2. Adds instance variables (`decl.add_ivar`).
3. Adds methods (`decl.add_method`).
4. Registers the class (`decl.register`).
5. Allocates an instance (`msg_send![cl, alloc]`).
6. Sets the instance variables' values.
7. Returns the delegate as an `id` (Objective-C object pointer).

**This is the Rust analogue of metal-cpp's `sendMessage` pattern** — both libraries need to dynamically create Objective-C classes that call back into Rust. metal-cpp uses C++ templates; cocoa uses a Rust macro.

The macro is marked `#[deprecated = "use the objc2::define_class! macro instead"]` — the newer `objc2` crate has a better mechanism.

---

## 6.3 `cocoa/src/appkit.rs` — AppKit bindings (massive, ~5000 lines)

**Path:** `cocoa/src/appkit.rs` (~5000 lines)

The biggest file in the whole project by far. Wraps AppKit — the macOS UI framework. Key types:

- **`NSApplication`** — the application object. Methods: `setActivationPolicy_`, `run`, `stop`, `setMainMenu_`.
- **`NSWindow`** — a window. Methods: `initWithContentRect_styleMask_backing_defer_`, `setTitle_`, `center`, `makeKeyAndOrderFront_`, `setDelegate_`, `orderOut_`.
- **`NSView`** — a view (rectangle inside a window that draws content). Methods: `initWithFrame_`, `setNeedsDisplay_`, `display`, `addSubview_`.
- **`NSMenu` / `NSMenuItem`** — menus.
- **`NSRunningApplication`** — process info.
- **`NSColor`** — colors.
- **`NSEvent`** — input events.

### The pattern: trait + ext trait

Each type is declared as a **trait** that the `id` type (Objective-C object pointer) implements:

```rust
pub trait NSWindow: Sized {
    unsafe fn alloc(_: Self) -> id;   // factory
    unsafe fn initWithContentRect_styleMask_backing_defer_(
        self, contentRect: NSRect, styleMask: NSUInteger, backing: NSUInteger, defer: BOOL,
    ) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn center(self);
    // ... ~30 more methods
}

impl NSWindow for id {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!("NSWindow"), alloc]
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_(
        self, contentRect: NSRect, styleMask: NSUInteger, backing: NSUInteger, defer: BOOL,
    ) -> id {
        msg_send![self, initWithContentRect:contentRect.as_raw()
                                      styleMask:styleMask
                                      backing:backing
                                      defer:defer]
    }
    // etc.
}
```

This is a different pattern from Phase 3. Instead of wrapping each Cocoa type in a Rust struct (`pub struct NSWindow(CFTypeRef)`), the cocoa crate keeps everything as raw `id` (Objective-C object pointer) and adds methods via traits.

The `msg_send!` macro (from the `objc` crate) is the Rust equivalent of Objective-C's `[obj method:arg]` syntax. It expands to a call to `objc_msgSend`.

**C++/metal-cpp comparison:**

| metal-cpp (C++) | cocoa (Rust) |
|---|---|
| `NS::Window` is a struct wrapping `objc_object*` | `id` is the raw pointer; `NSWindow` is a trait |
| `window->setTitle(s)` calls `sendMessage` | `window.setTitle_(s)` calls `msg_send!` |
| Method binding via cached `SEL` | Method binding via `sel!(setTitle:)` macro |

The metal-cpp approach is more type-safe (a `Window*` is distinct from a `View*`); the cocoa approach is more flexible (any `id` can have any trait method called on it, matching Objective-C's dynamic dispatch) but less safe.

### Why everything is `unsafe`

Almost every method is `unsafe fn`. This is because:

1. The `id` type is untyped — you can call any trait's methods on any object, even if the object doesn't actually implement them. The Objective-C runtime will throw an exception at runtime, but Rust can't catch this at compile time.
2. The arguments are often raw pointers (`id`, `*const c_void`).
3. Memory management is manual — you must `retain`/`release` correctly.

This is the **opposite philosophy** from `core-foundation`: CF wrappers hide `unsafe` inside safe method bodies; cocoa wrappers expose `unsafe` to the caller. The deprecation warning suggests using `objc2` instead, which provides safer abstractions.

---

## 6.4 `cocoa/src/quartzcore.rs` — Core Animation bindings

Wraps `CALayer` and related Core Animation types. Same trait-on-`id` pattern as AppKit. Methods: `init`, `setFrame_`, `setBounds_`, `setPosition_`, `setBackgroundColor_`, `addSublayer_`, `removeFromSuperlayer`.

Used for animation-rich UIs — Core Animation layers are GPU-accelerated and can be animated with implicit animations.

---

## 6.5 `cocoa-foundation/src/lib.rs` + `base.rs` + `foundation.rs` — Foundation shim

**Path:** `cocoa-foundation/src/` (3 files, ~700 lines combined)

A small crate that re-exports Foundation constants (`nil`, `id`, `selector`, `BOOL`, `NO`, `YES`, `NSPoint`, `NSRect`, `NSSize`, `NSRange`) and declares the Foundation traits (`NSObject`, `NSString`, `NSArray`, `NSProcessInfo`, `NSAutoreleasePool`).

The same trait-on-`id` pattern as AppKit. Example from `foundation.rs`:

```rust
pub trait NSString: Sized {
    unsafe fn alloc(_: Self) -> id;
    unsafe fn init_str(self, string: &str) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
}

impl NSString for id {
    unsafe fn alloc(_: Self) -> id { msg_send![class!("NSString"), alloc] }
    unsafe fn init_str(self, string: &str) -> id {
        let bytes = string.as_ptr() as *const c_void;
        msg_send![self, initWithBytes:bytes
                              length:string.len()
                            encoding:UTF8_ENCODING]
    }
    // etc.
}
```

---

## 6.6 `cocoa-foundation/tests/foundation.rs`

A small integration test file. Tests basic `NSString` and `NSArray` operations.

---

## 6.7 `io-surface/src/lib.rs` — `IOSurface`

**Path:** `io-surface/src/lib.rs` (233 lines)

Wraps `IOSurface` — Apple's cross-process shared memory surface, used for transferring image data between processes (e.g. between a video decoder and a renderer).

This crate is **deprecated** in favour of `objc2-io-surface`.

### The wrapper

```rust
pub struct IOSurface {
    pub obj: IOSurfaceRef,
}

impl Drop for IOSurface {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl TCFType for IOSurface {
    type Ref = IOSurfaceRef;
    // ... standard impl_TCFType! body, hand-written instead of macro
}
```

Note: `IOSurface` uses the `TCFType` trait from `core-foundation` but with a **hand-written implementation** instead of the `impl_TCFType!` macro. The struct stores `obj` as a public field (`pub obj: IOSurfaceRef`) — slightly different from the `declare_TCFType!` convention which makes the field private.

### OpenGL interop

```rust
pub fn bind_to_gl_texture(&self, target: GLenum) {
    unsafe {
        let context = CGLGetCurrentContext();
        CGLTexImageIOSurface2D(
            context,
            target,
            RGBA,
            self.width(), self.height(),
            UNSIGNED_INT_8_8_8_8_REV,
            TEXTURE_RECTANGLE_ARB,
            self.get_base_address() as *const c_void,
        );
    }
}
```

The interesting method: binds the IOSurface as an OpenGL texture. This is how you'd render video frames via OpenGL without copying pixel data.

### The `LeakyCow` dependency

```rust
use leaky_cow::LeakyCow;
```

A small utility crate that lets you get a raw `*const c_char` from a `Cow<str>` without leaking memory. Used for converting Rust strings to C strings for IOSurface attribute keys.

---

## 6.8 `cocoa/examples/hello_world.rs` — **a complete working Cocoa app in Rust**

**Path:** `cocoa/examples/hello_world.rs` (54 lines)

The single most valuable file for learning in this whole project. A complete, runnable macOS app in Rust. Read it carefully — it demonstrates every major pattern you need to write a Cocoa app.

```rust
#![allow(deprecated)]
use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSMenu, NSMenuItem,
    NSRunningApplication, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{nil, selector, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(
            NSProcessInfo::processInfo(nil).processName()
        );
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // create Window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        let title = NSString::alloc(nil).init_str("Hello World!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}
```

### Walkthrough

1. **`unsafe { ... }`** — the entire `main` is `unsafe` because every Cocoa call is `unsafe`. This matches the cocoa crate's philosophy (expose `unsafe` to the caller).

2. **`let _pool = NSAutoreleasePool::new(nil);`** — create an autorelease pool at the top. The `_` prefix means "I'm not going to use this variable, but I want it to live until the end of the scope so its `Drop` runs." When `_pool` goes out of scope at the end of `main`, the pool is drained.

3. **`let app = NSApp();`** — get the global `NSApplication` singleton. This is the entry point for any Cocoa app.

4. **`app.setActivationPolicy_(NSApplicationActivationPolicyRegular);`** — set the app to "regular" (a normal GUI app with a Dock icon, as opposed to a background agent or accessory).

5. **Menu bar setup** — create an `NSMenu`, add an `NSMenuItem` to it, set it as the main menu, then add a "Quit" submenu item. The `selector("terminate:")` call registers the Objective-C selector `terminate:` — when the user clicks Quit, the app's `terminate:` method is called.

6. **Window setup** — allocate an `NSWindow`, initialise it with a content rect (origin `{0, 0}`, size `{200, 200}`), set its title, center it, and bring it to front.

7. **`app.run();`** — start the main event loop. This call blocks until the app quits (e.g. when the user clicks Quit).

### The `nil` parameter

Notice how almost every call takes `nil` as the first argument: `NSMenu::new(nil)`, `NSMenuItem::new(nil)`, `NSString::alloc(nil)`. This is an Objective-C convention — many Cocoa methods take the receiver (`nil` here means "class method") as an explicit first parameter. In Objective-C source, `[NSString alloc]` reads as "send `alloc` to the `NSString` class"; in this Rust binding, it becomes `NSString::alloc(nil)` where `nil` is a placeholder.

### The `selector(...)` function

```rust
let quit_action = selector("terminate:");
```

`selector("foo:")` is the Rust equivalent of Objective-C's `@selector(foo:)` — it registers a method-name string with the runtime and returns the `SEL` (selector) value. Same concept as metal-cpp's `sel_registerName("terminate:")`.

### How to run it

If you have a Mac:

```bash
cd core-foundation-rs-main/cocoa
cargo run --example hello_world
```

A 200×200 window titled "Hello World!" should appear. Clicking Quit in the menu (or pressing Cmd-Q) terminates the app.

---

## 6.9 `cocoa/examples/color.rs`, `fullscreen.rs`, `tab_view.rs`, `nsvisualeffectview_blur.rs`

Four more example programs:

- **`color.rs`** — creates a window with a custom `NSView` that draws a colored background. Shows the `drawRect_` delegate pattern.
- **`fullscreen.rs`** — creates a window that can enter fullscreen mode. Shows `NSWindowDelegate` usage via the `delegate!` macro.
- **`tab_view.rs`** — creates a window with an `NSTabView`. Shows multiple subviews.
- **`nsvisualeffectview_blur.rs`** — uses `NSVisualEffectView` for a frosted-glass blur effect. Shows modern macOS UI.

Each is 50–150 lines. Read them in order; they build on the patterns from `hello_world.rs`.

---

## 6.10 Summary

After Phase 6, you should be able to:

1. **Recognise the trait-on-`id` pattern** — the cocoa crate's approach of declaring traits that the raw `id` type implements, vs. `core-foundation`'s approach of wrapping each type in its own struct.
2. **Read `msg_send!` macro calls** and translate them to Objective-C method syntax.
3. **Use `selector("name:")` and `class!("NSWindow")`** to register selectors and look up classes.
4. **Understand the `delegate!` macro** — dynamically creates Objective-C classes that call back into Rust.
5. **Write a small Cocoa app** in Rust using `cocoa/examples/hello_world.rs` as a template.
6. **Understand why the crate is deprecated** — the trait-on-`id` approach is too unsafe; the newer `objc2` crate provides better type safety.
7. **Recognise `IOSurface`** as a cross-process shared-memory surface, used for GPU texture sharing.

**The deprecated status is itself a teaching opportunity.** Comparing cocoa's `unsafe`-everywhere approach with `objc2`'s safer approach (which you'd study if you switched to the newer crate) shows the evolution of Rust FFI design: early crates expose the unsafe layer; later crates wrap it in safer abstractions. The same evolution happened with `libc` → `nix`, `libsqlite3-sys` → `rusqlite`, etc.

**Open File 7 when ready** to start the 100 Rust code examples.

— End of File 6 —
