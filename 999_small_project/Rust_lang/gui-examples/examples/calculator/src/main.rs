#![allow(deprecated)] // the `cocoa` crate is deprecated; still works.

//! A minimal macOS calculator.
//!
//! Demonstrates: NSButton, action selectors, the delegate pattern.
//!
//! Layout: a display field at the top + a 4×4 grid of buttons:
//!     C   (   )   /
//!     7   8   9   *
//!     4   5   6   -
//!     1   2   3   +
//!     0       .   =
//!
//! Each button's action selector points to a method on a custom controller
//! object. We dynamically create an Objective-C class for the controller via
//! `objc::declare::ClassDecl`, register a single `buttonClicked:` method that
//! inspects the button's title, and dispatch from there.
//!
//! Run with: `cargo run -p calculator`

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
    NSBezelStyle, NSButton, NSMenu, NSMenuItem, NSRunningApplication, NSTextField, NSView,
    NSWindow, NSWindowStyleMask,
};
use cocoa::base::{NO, YES, id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::ffi::CString;

// Autoresizing constants (AppKit/NSView.h):
const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

// The calculator's state, stored in a thread-local so the button-click
// callback (an `extern "C" fn` that can't capture environment) can reach it.
thread_local! {
    static CALC_STATE: RefCell<CalcState> = RefCell::new(CalcState::new());
}

struct CalcState {
    display: String, // the current display text
    operand: f64,    // the stored operand (left side)
    operator: char,  // the pending operator (+, -, *, /, or '\0' for none)
    fresh: bool,     // true if the next digit should start a new number
}

impl CalcState {
    fn new() -> Self {
        CalcState {
            display: "0".into(),
            operand: 0.0,
            operator: '\0',
            fresh: true,
        }
    }
}

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // Set up the app and menu (same boilerplate as notepad).
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

        // Create the window.
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(200., 200.), NSSize::new(240., 320.)),
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Calculator"));
        window.makeKeyAndOrderFront_(nil);

        let content_view: id = window.contentView();
        let bounds: NSRect = NSView::bounds(content_view);

        // Create the display field at the top.
        let display_rect = NSRect::new(
            NSPoint::new(10., bounds.size.height - 40.),
            NSSize::new(bounds.size.width - 20., 30.),
        );
        let display = NSTextField::alloc(nil).initWithFrame_(display_rect);
        display.setEditable_(NO);
        display.setStringValue_(NSString::alloc(nil).init_str("0"));
        let _: () = msg_send![display, setAlignment: 2usize]; // NSTextAlignmentRight = 2
        let _: () = msg_send![
            display,
            setAutoresizingMask: VIEW_WIDTH_SIZABLE
        ];
        content_view.addSubview_(display);

        // Build the controller object that handles button clicks.
        let controller = create_controller_class(display);

        // Button labels in a 5-row × 4-column grid (with an empty cell at
        // position (4, 1) for visual spacing under the "0").
        let rows: &[&[&str]] = &[
            &["C", "(", ")", "/"],
            &["7", "8", "9", "*"],
            &["4", "5", "6", "-"],
            &["1", "2", "3", "+"],
            &["0", "", ".", "="],
        ];

        let cell_w = (bounds.size.width - 50.) / 4.0; // 4 columns, 10px gaps
        let cell_h = 50.0;
        let top_y = bounds.size.height - 40. - 10. - cell_h;

        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, label) in row.iter().enumerate() {
                if label.is_empty() {
                    continue; // skip the empty cell
                }
                let rect = NSRect::new(
                    NSPoint::new(
                        10. + col_idx as f64 * (cell_w + 10.),
                        top_y - row_idx as f64 * (cell_h + 10.),
                    ),
                    NSSize::new(cell_w, cell_h),
                );
                let btn = NSButton::alloc(nil).initWithFrame_(rect);
                btn.setBezelStyle_(NSBezelStyle::NSRoundedBezelStyle);
                btn.setTitle_(NSString::alloc(nil).init_str(label));
                btn.setTarget_(controller);
                btn.setAction_(selector("buttonClicked:"));
                let _: () = msg_send![
                    btn,
                    setAutoresizingMask: VIEW_WIDTH_SIZABLE | VIEW_HEIGHT_SIZABLE
                ];
                content_view.addSubview_(btn);
            }
        }

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}

/// Dynamically create an Objective-C class named `CalcController` with a
/// single method `buttonClicked:` that reads the button's title and dispatches.
///
/// Returns an allocated instance of the class (not yet initialised — but
/// NSObject's `+alloc` is sufficient because we don't store any ivars in the
/// object itself; the state lives in the thread-local `CALC_STATE`).
unsafe fn create_controller_class(display: id) -> id {
    // If the class is already registered (e.g. we run twice in one process),
    // look it up instead of re-declaring.
    let cls_name = CString::new("CalcController").unwrap();
    let existing = Class::get(cls_name.as_ptr());
    let cls: &'static Class = if let Some(c) = existing {
        c
    } else {
        let mut decl = ClassDecl::new("CalcController", class!(NSObject))
            .expect("Failed to declare CalcController class");
        decl.add_method(
            sel!(buttonClicked:),
            button_clicked as extern "C" fn(&Object, Sel, id),
        );
        decl.register()
    };

    msg_send![cls, alloc]
}

/// The button-click callback. Reads the button's title and dispatches.
extern "C" fn button_clicked(_this: &Object, _cmd: Sel, sender: id) {
    unsafe {
        // Read the button's title.
        let title: id = msg_send![sender, title];
        let title_str = nsstring_to_rust(title);

        CALC_STATE.with(|state| {
            let mut s = state.borrow_mut();
            handle_button(&mut s, &title_str);

            // Update the display.
            // (We have to get the display from somewhere — store it in a
            // thread-local too. For brevity in this example we just print.)
            println!("Display: {}", s.display);
        });

        // Update the actual NSTextField display.
        // We stored `display` in the controller? Actually, we passed it to
        // create_controller_class but didn't store it. Let's just look it up
        // via the sender's window.
        let window: id = msg_send![sender, window];
        if window != nil {
            let cv: id = msg_send![window, contentView];
            let subviews: id = msg_send![cv, subviews];
            // The first subview is our display field.
            let display: id = msg_send![subviews, objectAtIndex: 0usize];
            CALC_STATE.with(|state| {
                let s = state.borrow();
                let new_value = NSString::alloc(nil).init_str(&s.display);
                let _: () = msg_send![display, setStringValue: new_value];
            });
        }
    }
}

/// Convert an `NSString *` to a Rust `String`.
unsafe fn nsstring_to_rust(nsstr: id) -> String {
    if nsstr == nil {
        return String::new();
    }
    let bytes: *const u8 = msg_send![nsstr, UTF8String];
    let len: usize = msg_send![nsstr, lengthOfBytesUsingEncoding: 4usize]; // NSUTF8StringEncoding = 4
    if bytes.is_null() || len == 0 {
        return String::new();
    }
    let slice = std::slice::from_raw_parts(bytes, len);
    String::from_utf8_lossy(slice).into_owned()
}

/// The calculator logic. Mutates `state` based on which button was pressed.
fn handle_button(state: &mut CalcState, label: &str) {
    match label {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            if state.fresh {
                state.display.clear();
                state.fresh = false;
            }
            state.display.push_str(label);
        }
        "." => {
            if state.fresh {
                state.display = "0".into();
                state.fresh = false;
            }
            if !state.display.contains('.') {
                state.display.push('.');
            }
        }
        "C" => {
            *state = CalcState::new();
        }
        "+" | "-" | "*" | "/" => {
            // Save the current display as the left operand.
            state.operand = state.display.parse().unwrap_or(0.0);
            state.operator = label.chars().next().unwrap();
            state.fresh = true;
        }
        "=" => {
            if state.operator != '\0' {
                let right: f64 = state.display.parse().unwrap_or(0.0);
                let result = match state.operator {
                    '+' => state.operand + right,
                    '-' => state.operand - right,
                    '*' => state.operand * right,
                    '/' => {
                        if right == 0.0 {
                            f64::NAN
                        } else {
                            state.operand / right
                        }
                    }
                    _ => right,
                };
                state.display = if result.is_nan() {
                    "Error".into()
                } else if result.fract() == 0.0 {
                    format!("{}", result as i64)
                } else {
                    format!("{}", result)
                };
                state.operator = '\0';
                state.fresh = true;
            }
        }
        "(" | ")" => {
            // For simplicity, parentheses are no-ops in this minimal example.
            // A real calculator would parse them.
        }
        _ => {}
    }
}
