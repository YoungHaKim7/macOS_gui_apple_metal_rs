#![allow(deprecated)] // the `cocoa` crate is deprecated; still works.

//! An analog clock face drawn with NSBezierPath.
//!
//! Demonstrates: custom NSView subclass, NSTimer firing once per second,
//! drawing arcs and lines, computing clock-hand angles from the current
//! time, and using the system's `time()` function to read the wall clock.
//!
//! Run with: `cargo run -p clock`

use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
        NSMenu, NSMenuItem, NSRunningApplication, NSView, NSWindow, NSWindowStyleMask,
    },
    base::{NO, YES, id, nil, selector},
    foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString},
};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    {class, msg_send, sel, sel_impl},
};
use std::{
    sync::Once,
    time::{SystemTime, UNIX_EPOCH},
};

const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

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
                NSRect::new(NSPoint::new(200., 200.), NSSize::new(300., 300.)),
                NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Clock"));
        window.makeKeyAndOrderFront_(nil);

        let cls = register_clock_view_class();
        let content_view: id = window.contentView();
        let bounds: NSRect = NSView::bounds(content_view);
        let view: id = msg_send![cls, alloc];
        let view: id = msg_send![view, initWithFrame: bounds];
        let _: () = msg_send![view, setAutoresizingMask: VIEW_WIDTH_SIZABLE | VIEW_HEIGHT_SIZABLE];
        content_view.addSubview_(view);

        // Timer fires once per second to trigger a redraw.
        let timer: id = msg_send![
            class!(NSTimer),
            scheduledTimerWithTimeInterval: 1.0f64
            target: view
            selector: selector("tick:")
            userInfo: nil
            repeats: YES
        ];
        let _: () = msg_send![timer, retain];

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}

fn register_clock_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    static mut CLS: Option<&'static Class> = None;

    unsafe {
        REGISTER.call_once(|| {
            let mut decl =
                ClassDecl::new("ClockView", class!(NSView)).expect("Failed to declare ClockView");
            decl.add_method(
                sel!(drawRect:),
                draw_rect as extern "C" fn(&Object, Sel, NSRect),
            );
            decl.add_method(sel!(tick:), tick as extern "C" fn(&Object, Sel, id));
            CLS = Some(decl.register());
        });
        CLS.unwrap()
    }
}

extern "C" fn draw_rect(_this: &Object, _cmd: Sel, _dirty: NSRect) {
    unsafe {
        let cls = class!(NSBezierPath);
        let bounds: NSRect = msg_send![_this, bounds];

        // White background.
        let white: id = msg_send![class!(NSColor), whiteColor];
        let _: () = msg_send![white, set];
        let _: () = msg_send![cls, fillRect: bounds];

        // Clock center and radius.
        let cx = bounds.size.width / 2.0;
        let cy = bounds.size.height / 2.0;
        let radius = cx.min(cy) - 10.0;

        // Draw the clock face circle (black outline).
        let black: id = msg_send![class!(NSColor), blackColor];
        let _: () = msg_send![black, set];
        let _: () = msg_send![cls, strokeRect: bounds]; // dummy to ensure path exists
        // Use NSBezierPath's bezierPathWithOvalInRect: for the face outline.
        let face_rect = NSRect::new(
            NSPoint::new(cx - radius, cy - radius),
            NSSize::new(radius * 2.0, radius * 2.0),
        );
        let face_path: id = msg_send![cls, bezierPathWithOvalInRect: face_rect];
        let lw: f64 = 3.0;
        let _: () = msg_send![face_path, setLineWidth: lw];
        let _: () = msg_send![face_path, stroke];

        // Draw the 12 hour ticks.
        for hour in 0..12 {
            let angle = (hour as f64) * (std::f64::consts::TAU / 12.0);
            let (x1, y1, x2, y2) = tick_endpoints(cx, cy, radius, angle, 0.85, 1.0);
            let path: id = msg_send![class!(NSBezierPath), bezierPath];
            let _: () = msg_send![path, moveToPoint: NSPoint::new(x1, y1)];
            let _: () = msg_send![path, lineToPoint: NSPoint::new(x2, y2)];
            let _: () = msg_send![path, setLineWidth: 2.0f64];
            let _: () = msg_send![path, stroke];
        }

        // Get the current time.
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let secs = now.as_secs();
        let (h, m, s) = unix_to_hms(secs);

        // Hour hand: angle = (h % 12 + m/60) * 360/12 = ... in radians from top
        let hour_angle = ((h % 12) as f64 + m as f64 / 60.0) * (std::f64::consts::TAU / 12.0);
        draw_hand(cls, cx, cy, radius * 0.5, hour_angle, 4.0);

        // Minute hand.
        let min_angle = (m as f64 + s as f64 / 60.0) * (std::f64::consts::TAU / 60.0);
        draw_hand(cls, cx, cy, radius * 0.75, min_angle, 3.0);

        // Second hand (red).
        let sec_angle = (s as f64) * (std::f64::consts::TAU / 60.0);
        let red: id = msg_send![class!(NSColor), redColor];
        let _: () = msg_send![red, set];
        draw_hand(cls, cx, cy, radius * 0.85, sec_angle, 1.0);

        // Center dot.
        let dot_rect = NSRect::new(NSPoint::new(cx - 4.0, cy - 4.0), NSSize::new(8.0, 8.0));
        let _: () = msg_send![cls, fillRect: dot_rect];
    }
}

/// Compute endpoints (start, end) for a tick at the given angle.
/// `inner_frac` and `outer_frac` are fractions of `radius`.
fn tick_endpoints(
    cx: f64,
    cy: f64,
    radius: f64,
    angle: f64,
    inner_frac: f64,
    outer_frac: f64,
) -> (f64, f64, f64, f64) {
    // We want angle 0 = top (12 o'clock). Top is +y in non-flipped AppKit,
    // but here for simplicity we treat angle 0 as rightward and rotate so
    // the hour hand at h=3 points right (which matches our visual intuition
    // since we're not flipping the view).
    //
    // Actually, let's use the convention that angle 0 = top, increasing
    // clockwise. The mapping to (x, y) in standard (non-flipped) NSView
    // coordinates (where +y is up) is:
    //   x = cx + r * sin(angle)
    //   y = cy + r * cos(angle)
    let x1 = cx + radius * inner_frac * angle.sin();
    let y1 = cy + radius * inner_frac * angle.cos();
    let x2 = cx + radius * outer_frac * angle.sin();
    let y2 = cy + radius * outer_frac * angle.cos();
    (x1, y1, x2, y2)
}

/// Draw a clock hand from the center along the given angle.
unsafe fn draw_hand(cls: &'static Class, cx: f64, cy: f64, length: f64, angle: f64, width: f64) {
    let ex = cx + length * angle.sin();
    let ey = cy + length * angle.cos();
    let path: id = msg_send![cls, bezierPath];
    let _: () = msg_send![path, moveToPoint: NSPoint::new(cx, cy)];
    let _: () = msg_send![path, lineToPoint: NSPoint::new(ex, ey)];
    let _: () = msg_send![path, setLineWidth: width];
    let _: () = msg_send![path, stroke];
}

/// Convert Unix seconds to (hours, minutes, seconds) in local time.
/// For simplicity we use UTC; a real clock would call `localtime_r`.
fn unix_to_hms(secs: u64) -> (u64, u64, u64) {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    (h, m, s)
}

extern "C" fn tick(this: &Object, _cmd: Sel, _timer: id) {
    unsafe {
        let _: () = msg_send![this, setNeedsDisplay: YES];
        let _: () = msg_send![this, display];
    }
}

// Suppress unused warning for the trait-style helper.
#[allow(non_snake_case)]
fn _unused() {
    let _ = VIEW_WIDTH_SIZABLE;
}
