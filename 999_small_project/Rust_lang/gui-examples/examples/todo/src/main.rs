#![allow(deprecated)] // the `cocoa` crate is deprecated; still works.

//! A minimal macOS todo list.
//!
//! Demonstrates: NSTextField for input, NSButton with action selector,
//! dynamically creating NSTextField labels to display the items, and a
//! controller object to manage the list state.
//!
//! Layout:
//!   ┌─────────────────────────────────────────────────────────┐
//!   │ [ Type a todo...                                ] [Add]  │  ← input row
//!   ├─────────────────────────────────────────────────────────┤
//!   │ ☐ Buy milk                                               │  ← item
//!   │ ☐ Walk the dog                                           │  ← item
//!   │ ☑ Learn Rust (done)                                      │  ← completed
//!   └─────────────────────────────────────────────────────────┘
//!
//! Type into the top field, press Add (or Enter), and the todo appears below.
//! Click on a todo to toggle its done state.
//!
//! Run with: `cargo run -p todo`

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
    NSBezelStyle, NSButton, NSMenu, NSMenuItem, NSRunningApplication, NSTextField, NSView,
    NSWindow, NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::sync::Once;

const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

struct TodoItem {
    text: String,
    done: bool,
    // The label view that displays this item. We hold it so we can update it
    // when the done state toggles.
    label: id,
}

thread_local! {
    static TODOS: RefCell<Vec<TodoItem>> = RefCell::new(Vec::new());
    // The vertical position (in flipped coords) where the next todo label
    // should be placed.
    static NEXT_Y: RefCell<f64> = RefCell::new(50.0);
    // The content view of the window, so we can add subviews from the
    // controller callback.
    static CONTENT_VIEW: RefCell<id> = RefCell::new(nil);
}

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

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

        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(200., 200.), NSSize::new(380., 480.)),
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Todo"));
        window.makeKeyAndOrderFront_(nil);

        let content_view: id = window.contentView();
        let bounds: NSRect = NSView::bounds(content_view);

        CONTENT_VIEW.with(|cv| cv.replace(content_view));

        // Make the content view "flipped" so y=0 is the top. This makes
        // stacking todos vertically much easier.
        let _: () = msg_send![content_view, setFlipped: YES];

        // Input text field.
        let input_rect = NSRect::new(
            NSPoint::new(10., 10.),
            NSSize::new(bounds.size.width - 90., 25.),
        );
        let input = NSTextField::alloc(nil).initWithFrame_(input_rect);
        let placeholder = NSString::alloc(nil).init_str("Type a todo and press Enter...");
        let _: () = msg_send![input, setPlaceholderString: placeholder];
        let _: () = msg_send![input, setAutoresizingMask: VIEW_WIDTH_SIZABLE];
        content_view.addSubview_(input);

        // "Add" button.
        let btn_rect = NSRect::new(
            NSPoint::new(bounds.size.width - 70., 10.),
            NSSize::new(60., 25.),
        );
        let add_btn = NSButton::alloc(nil).initWithFrame_(btn_rect);
        add_btn.setBezelStyle_(NSBezelStyle::NSRoundedBezelStyle);
        add_btn.setTitle_(NSString::alloc(nil).init_str("Add"));
        let _: () = msg_send![
            add_btn,
            setAutoresizingMask: VIEW_WIDTH_SIZABLE
        ];
        content_view.addSubview_(add_btn);

        // Build the controller class.
        let controller = register_controller_class(input);

        // Wire up: pressing Enter in the input field calls addTodo: on the controller.
        let _: () = msg_send![input, setTarget: controller];
        let _: () = msg_send![input, setAction: selector("addTodo:")];
        // Wire up: clicking the Add button calls the same method.
        add_btn.setTarget_(controller);
        add_btn.setAction_(selector("addTodo:"));

        // Add a couple of starter items so the user sees something.
        add_todo("Buy milk");
        add_todo("Walk the dog");
        add_todo("Learn Rust");

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}

fn register_controller_class(input: id) -> id {
    static REGISTER: Once = Once::new();
    static mut CLS: Option<&'static Class> = None;

    unsafe {
        REGISTER.call_once(|| {
            let mut decl = ClassDecl::new("TodoController", class!(NSObject))
                .expect("Failed to declare TodoController");
            decl.add_method(
                sel!(addTodo:),
                add_todo_callback as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(toggleTodo:),
                toggle_todo_callback as extern "C" fn(&Object, Sel, id),
            );
            CLS = Some(decl.register());
        });
        let cls = CLS.unwrap();
        let controller: id = msg_send![cls, alloc];
        let controller: id = msg_send![controller, init];

        // Stash the input field in a thread-local so the callback can read it.
        // (For simplicity we don't store it as an ivar on the controller.)
        // We'll look it up via the sender's window in the callback instead.
        let _ = input; // suppress unused warning
        controller
    }
}

/// The `addTodo:` callback. Reads the input field, adds a label, clears input.
extern "C" fn add_todo_callback(_this: &Object, _cmd: Sel, sender: id) {
    unsafe {
        // Find the input field. If the sender is the button, walk up to the
        // window; if it's the field itself, sender IS the field.
        let input: id = if msg_send![sender, isKindOfClass: class!(NSTextField)] {
            sender
        } else {
            // It's the button — find the text field in the window's subviews.
            let window: id = msg_send![sender, window];
            let cv: id = msg_send![window, contentView];
            let subs: id = msg_send![cv, subviews];
            msg_send![subs, objectAtIndex: 0usize]
        };

        // Read the input's string value.
        let value: id = msg_send![input, stringValue];
        let bytes: *const u8 = msg_send![value, UTF8String];
        let text = if bytes.is_null() {
            String::new()
        } else {
            let cstr = std::ffi::CStr::from_ptr(bytes as *const i8);
            cstr.to_string_lossy().into_owned()
        };

        if !text.trim().is_empty() {
            add_todo(text.trim());

            // Clear the input field.
            let empty = NSString::alloc(nil).init_str("");
            let _: () = msg_send![input, setStringValue: empty];
        }
    }
}

/// Add a todo item to the list and to the UI.
fn add_todo(text: &str) {
    unsafe {
        let label_text = format!("☐ {}", text);
        let label_str = NSString::alloc(nil).init_str(&label_text);

        let (y, content_view) = (
            NEXT_Y.with(|y| *y.borrow()),
            CONTENT_VIEW.with(|cv| *cv.borrow()),
        );

        let bounds: NSRect = msg_send![content_view, bounds];
        let rect = NSRect::new(
            NSPoint::new(10., y),
            NSSize::new(bounds.size.width - 20., 20.),
        );
        let label = NSTextField::alloc(nil).initWithFrame_(rect);
        label.setEditable_(NO);
        label.setDrawsBackground_(NO);
        let _: () = msg_send![label, setBezeled: NO];
        label.setStringValue_(label_str);
        let _: () = msg_send![
            label,
            setAutoresizingMask: VIEW_WIDTH_SIZABLE
        ];

        // Make the label clickable to toggle done. We give it the same
        // target/action as the toggle button would have.
        let controller = lookup_controller();
        let _: () = msg_send![label, setTarget: controller];
        let _: () = msg_send![label, setAction: selector("toggleTodo:")];
        // Clickable text fields need to be set selectable and have a cursor,
        // but for simplicity we just leave them as labels and the user can
        // click anywhere on them.

        content_view.addSubview_(label);

        TODOS.with(|todos| {
            todos.borrow_mut().push(TodoItem {
                text: text.to_string(),
                done: false,
                label,
            });
        });

        NEXT_Y.with(|y| *y.borrow_mut() += 26.0);
    }
}

trait Extras {
    unsafe fn setDrawsBackground_(&self, draws: bool);
}
impl Extras for id {
    unsafe fn setDrawsBackground_(&self, draws: bool) {
        let _: () = msg_send![self, setDrawsBackground: draws as cocoa::base::BOOL];
    }
}
use Extras as _Extras;

/// The `toggleTodo:` callback. Find the matching item and toggle its done state.
extern "C" fn toggle_todo_callback(_this: &Object, _cmd: Sel, sender: id) {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        for item in todos.iter_mut() {
            if item.label == sender {
                item.done = !item.done;
                let new_text = if item.done {
                    format!("☑ {} (done)", item.text)
                } else {
                    format!("☐ {}", item.text)
                };
                unsafe {
                    let s = NSString::alloc(nil).init_str(&new_text);
                    let _: () = msg_send![item.label, setStringValue: s];
                }
                break;
            }
        }
    });
}

/// Look up the TodoController singleton. Since we registered the class
/// globally, we can find an instance by going through the input field's
/// target (which we set in main).
unsafe fn lookup_controller() -> id {
    // Walk any NSTextField's target. We stored the controller on every
    // label's target, so we can grab it from the first todo item if any.
    TODOS.with(|todos| {
        let todos = todos.borrow();
        if let Some(first) = todos.first() {
            let target: id = msg_send![first.label, target];
            return target;
        }
    });
    // Fall back to nil if no items yet.
    nil
}
