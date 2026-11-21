#![allow(deprecated)] // the `cocoa` crate is deprecated in favour of objc2; still works fine.

//! A minimal macOS notepad.
//!
//! Creates a window containing a single editable `NSTextView` that fills the
//! window. You can type into it; the text view handles its own keyboard input
//! and scrolling. The window is resizable.
//!
//! Run with: `cargo run -p notepad`

use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
        NSMenu, NSMenuItem, NSRunningApplication, NSView, NSWindow, NSWindowStyleMask,
    },
    base::{NO, YES, id, nil, selector},
    foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString},
};
// `msg_send!`, `sel!`, `sel_impl!` are macros from the `objc` crate.
// `msg_send![obj, setFoo: arg]` expands to a call to `objc_msgSend` with
// a selector registered via `sel!(setFoo:)`.
use objc::{class, msg_send, sel, sel_impl};

// Autoresizing mask constants from <AppKit/NSView.h>:
const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

fn main() {
    unsafe {
        // 1. Every Cocoa app needs an autorelease pool.
        let _pool = NSAutoreleasePool::new(nil);

        // 2. Set up the application object + a basic menu (so Cmd-Q works).
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                quit_title,
                selector("terminate:"),
                NSString::alloc(nil).init_str("q"),
            )
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // 3. Create the main window.
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(100., 100.), NSSize::new(480., 360.)),
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Notepad"));
        window.makeKeyAndOrderFront_(nil);

        // 4. Create the text view that fills the window's content view.
        //    The `cocoa` crate doesn't wrap NSTextView, so we use `objc`
        //    directly via `msg_send!`.
        let content_view: id = window.contentView();
        let frame: NSRect = NSView::bounds(content_view);

        let text_view: id = msg_send![class!(NSTextView), alloc];
        let text_view: id = msg_send![text_view, initWithFrame: frame];

        // Make it editable, plain-text, and resize with the window.
        let _: () = msg_send![text_view, setEditable: YES];
        let _: () = msg_send![text_view, setRichText: NO];
        let _: () = msg_send![
            text_view,
            setAutoresizingMask: VIEW_WIDTH_SIZABLE | VIEW_HEIGHT_SIZABLE
        ];

        // Insert some welcome text.
        let welcome = NSString::alloc(nil).init_str(
            "Welcome to your Rust notepad.\n\
             Type anything here.\n\
             \n\
             (Cmd-Q to quit.)",
        );
        let _: () = msg_send![text_view, setString: welcome];

        // Add the text view to the window.
        content_view.addSubview_(text_view);

        // 5. Bring the app to the front and run the event loop.
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

        app.run(); // blocks until the user quits
    }
}
