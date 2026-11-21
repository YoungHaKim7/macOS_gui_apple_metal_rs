#![allow(deprecated)] // the `cocoa` crate is deprecated; still works.

//! Snake game in pure macOS Cocoa + Rust.
//!
//! Demonstrates:
//!   * Dynamically subclassing NSView via `objc::declare::ClassDecl`
//!     so we can override `drawRect:` and `keyDown:`.
//!   * Scheduling an NSTimer that fires ~10 times per second to advance
//!     the game state.
//!   * Drawing with NSGraphicsContext / NSBezierPath (via raw `msg_send!`).
//!   * Reading keyboard input via the `keyDown:` method.
//!
//! Controls: Arrow keys to steer. Eat the red food to grow. If you hit the
//! wall or yourself, the game resets.
//!
//! Run with: `cargo run -p snake`

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
    NSMenu, NSMenuItem, NSRunningApplication, NSView, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{NO, YES, id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::ffi::CString;
use std::sync::Once;
use std::time::Instant;

// Grid configuration
const GRID_W: i32 = 30;
const GRID_H: i32 = 20;
const CELL: f64 = 20.0; // pixel size of each cell
const TICK_HZ: f64 = 10.0; // game ticks per second

const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: Vec<(i32, i32)>, // head is snake[0]
    dir: Dir,
    food: (i32, i32),
    score: u32,
    last_tick: Instant,
    tick_interval: std::time::Duration,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            snake: vec![(GRID_W / 2, GRID_H / 2)],
            dir: Dir::Right,
            food: (0, 0),
            score: 0,
            last_tick: Instant::now(),
            tick_interval: std::time::Duration::from_millis((1000.0 / TICK_HZ) as u64),
        };
        g.place_food();
        g
    }

    fn place_food(&mut self) {
        // Place food at a random cell not occupied by the snake.
        loop {
            let x = (rand_u32() % GRID_W as u32) as i32;
            let y = (rand_u32() % GRID_H as u32) as i32;
            if !self.snake.iter().any(|&(sx, sy)| sx == x && sy == y) {
                self.food = (x, y);
                return;
            }
        }
    }

    fn step(&mut self) -> bool {
        let (hx, hy) = self.snake[0];
        let (nx, ny) = match self.dir {
            Dir::Up => (hx, hy + 1),
            Dir::Down => (hx, hy - 1),
            Dir::Left => (hx - 1, hy),
            Dir::Right => (hx + 1, hy),
        };

        // Wall collision
        if nx < 0 || nx >= GRID_W || ny < 0 || ny >= GRID_H {
            return false;
        }
        // Self collision (ignore the tail since it will move out of the way)
        if self
            .snake
            .iter()
            .take(self.snake.len().saturating_sub(1))
            .any(|&(sx, sy)| sx == nx && sy == ny)
        {
            return false;
        }

        self.snake.insert(0, (nx, ny));

        if (nx, ny) == self.food {
            self.score += 1;
            self.place_food();
            // Don't pop the tail → snake grows.
        } else {
            self.snake.pop();
        }
        true
    }
}

// Simple PRNG (xorshift32) — we don't need cryptographic randomness.
fn rand_u32() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static SEED: AtomicU32 = AtomicU32::new(0xdead_beef);
    let mut x = SEED.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    SEED.store(x, Ordering::Relaxed);
    x
}

// Game state is stored in a thread-local so the timer callback can reach it.
thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // App + menu setup (boilerplate).
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

        // Create the window sized to fit the grid.
        let win_w = GRID_W as f64 * CELL;
        let win_h = GRID_H as f64 * CELL;
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(100., 100.), NSSize::new(win_w, win_h)),
                NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Snake — arrow keys to steer"));
        window.makeKeyAndOrderFront_(nil);

        // Register our custom NSView subclass.
        let cls = register_snake_view_class();

        // Create an instance of it sized to fill the window's content view.
        let content_view: id = window.contentView();
        let bounds: NSRect = NSView::bounds(content_view);
        let view: id = msg_send![cls, alloc];
        let view: id = msg_send![view, initWithFrame: bounds];
        let _: () = msg_send![view, setAutoresizingMask: VIEW_WIDTH_SIZABLE | VIEW_HEIGHT_SIZABLE];
        content_view.addSubview_(view);

        // Schedule a timer that fires ~10× per second. The timer calls
        // `tick:` on our view, which advances the game and requests a redraw.
        let timer: id = msg_send![
            class!(NSTimer),
            scheduledTimerWithTimeInterval: 1.0 / TICK_HZ
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

/// Register a custom NSView subclass `SnakeView` with three methods:
///   * `drawRect:` — draws the snake, food, and background
///   * `keyDown:` — changes direction based on arrow keys
///   * `tick:` — advances the game and triggers a redraw
fn register_snake_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    static mut CLS: Option<&'static Class> = None;

    unsafe {
        REGISTER.call_once(|| {
            let mut decl =
                ClassDecl::new("SnakeView", class!(NSView)).expect("Failed to declare SnakeView");
            decl.add_method(
                sel!(drawRect:),
                draw_rect as extern "C" fn(&Object, Sel, NSRect),
            );
            decl.add_method(sel!(keyDown:), key_down as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(tick:), tick as extern "C" fn(&Object, Sel, id));
            CLS = Some(decl.register());
        });
        CLS.unwrap()
    }
}

/// `drawRect:` — draw the entire game.
extern "C" fn draw_rect(_this: &Object, _cmd: Sel, _dirty: NSRect) {
    unsafe {
        // Lock focus on the current graphics context (the view).
        // In modern AppKit this is implicit in drawRect:, but for clarity:
        let cls = class!(NSBezierPath);

        // Black background.
        let black: id = msg_send![class!(NSColor), blackColor];
        let _: () = msg_send![black, set];
        let bounds: NSRect = msg_send![_this, bounds];
        let _: () = msg_send![cls, fillRect: bounds];

        GAME.with(|g| {
            let g = g.borrow();

            // Draw the food as a red square.
            let red: id = msg_send![class!(NSColor), redColor];
            let _: () = msg_send![red, set];
            let (fx, fy) = g.food;
            let food_rect = NSRect::new(
                NSPoint::new(fx as f64 * CELL, fy as f64 * CELL),
                NSSize::new(CELL, CELL),
            );
            let _: () = msg_send![cls, fillRect: food_rect];

            // Draw the snake as a series of green squares.
            let green: id = msg_send![class!(NSColor), greenColor];
            let _: () = msg_send![green, set];
            for &(x, y) in g.snake.iter() {
                let rect = NSRect::new(
                    NSPoint::new(x as f64 * CELL, y as f64 * CELL),
                    NSSize::new(CELL, CELL),
                );
                let _: () = msg_send![cls, fillRect: rect];
            }
        });

        // Reset fill color to black to avoid side effects.
        let black: id = msg_send![class!(NSColor), blackColor];
        let _: () = msg_send![black, set];
    }
}

/// `keyDown:` — handle arrow keys.
extern "C" fn key_down(_this: &Object, _cmd: Sel, event: id) {
    unsafe {
        // Read the event's key code. Arrow key codes (on macOS):
        //   Left = 123, Right = 124, Down = 125, Up = 126
        let key_code: u16 = msg_send![event, keyCode];

        GAME.with(|g| {
            let mut g = g.borrow_mut();
            // Prevent 180° reversal.
            let new_dir = match key_code {
                123 if g.dir != Dir::Right => Dir::Left,
                124 if g.dir != Dir::Left => Dir::Right,
                125 if g.dir != Dir::Up => Dir::Down,
                126 if g.dir != Dir::Down => Dir::Up,
                _ => return,
            };
            g.dir = new_dir;
        });
        // We consume all key events in the snake view; no need to pass up
        // the responder chain.
    }
}

/// `tick:` — advance the game and request a redraw.
extern "C" fn tick(this: &Object, _cmd: Sel, _timer: id) {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        if !g.step() {
            // Game over — reset.
            println!("Game over! Score: {}. Restarting...", g.score);
            *g = Game::new();
        }
    });
    unsafe {
        // Mark the view as needing redraw and display immediately.
        let _: () = msg_send![this, setNeedsDisplay: YES];
        let _: () = msg_send![this, display];
    }
}
