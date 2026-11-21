#![allow(deprecated)] // the `cocoa` crate is deprecated; still works.

//! A minimal Tetris-like game in Rust + Cocoa.
//!
//! Demonstrates the same patterns as the snake example (custom NSView
//! subclass, NSTimer, keyDown:) but with a grid-based falling-block game.
//!
//! Simplifications vs. real Tetris:
//!   * No tetromino rotation (the falling piece is always a 1×1 block).
//!   * No line clearing (we just stack blocks until the column overflows).
//!   * No score; the goal is to keep the stack below the top.
//!
//! This makes the code short enough to read in one sitting while still
//! demonstrating every pattern you need to build a real Tetris.
//!
//! Controls: Left/Right to move, Down to drop faster.
//!
//! Run with: `cargo run -p tetris`

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSAutoresizingMaskOptions, NSBackingStoreBuffered,
    NSMenu, NSMenuItem, NSRunningApplication, NSView, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::sync::Once;

const COLS: i32 = 10;
const ROWS: i32 = 20;
const CELL: f64 = 22.0;
const TICK_HZ: f64 = 4.0; // slow gravity

const VIEW_WIDTH_SIZABLE: NSAutoresizingMaskOptions = 2;
const VIEW_HEIGHT_SIZABLE: NSAutoresizingMaskOptions = 16;

#[derive(Clone)]
struct Game {
    // `grid[x][y]` is true if that cell is occupied by a settled block.
    grid: Vec<Vec<bool>>,
    // Position of the currently-falling block.
    piece: (i32, i32),
    // Color of the currently-falling block (for drawing).
    piece_color: (f64, f64, f64),
}

impl Game {
    fn new() -> Self {
        Game {
            grid: vec![vec![false; ROWS as usize]; COLS as usize],
            piece: (COLS / 2, ROWS - 1),
            piece_color: random_color(),
        }
    }

    /// Move the piece left/right if possible.
    fn move_piece(&mut self, dx: i32) {
        let new_x = self.piece.0 + dx;
        if new_x >= 0 && new_x < COLS && !self.collides(new_x, self.piece.1) {
            self.piece.0 = new_x;
        }
    }

    /// Try to drop the piece by one cell. If it lands, settle it and spawn
    /// a new piece. Returns false if the new piece immediately collides
    /// (game over).
    fn tick(&mut self) -> bool {
        let new_y = self.piece.1 - 1;
        if new_y < 0 || self.collides(self.piece.0, new_y) {
            // Settle the piece at its current position.
            let (px, py) = self.piece;
            if px >= 0 && px < COLS && py >= 0 && py < ROWS {
                self.grid[px as usize][py as usize] = true;
            }
            // Spawn a new piece at the top.
            self.piece = (COLS / 2, ROWS - 1);
            self.piece_color = random_color();
            if self.collides(self.piece.0, self.piece.1) {
                return false; // game over
            }
        } else {
            self.piece.1 = new_y;
        }
        true
    }

    fn collides(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= COLS || y < 0 || y >= ROWS {
            return true;
        }
        self.grid[x as usize][y as usize]
    }

    /// Hard-drop the piece to the bottom.
    fn hard_drop(&mut self) {
        while self.piece.1 > 0 && !self.collides(self.piece.0, self.piece.1 - 1) {
            self.piece.1 -= 1;
        }
        // Force settling on next tick.
        let _ = self.tick();
    }
}

fn random_color() -> (f64, f64, f64) {
    // A small fixed palette so colors look nice.
    let palette = [
        (0.0, 0.5, 1.0), // blue
        (1.0, 0.4, 0.4), // red
        (0.4, 1.0, 0.4), // green
        (1.0, 0.8, 0.2), // yellow
        (0.8, 0.4, 1.0), // purple
        (0.2, 0.9, 0.9), // cyan
    ];
    palette[(rand_u32() as usize) % palette.len()]
}

fn rand_u32() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static SEED: AtomicU32 = AtomicU32::new(0xcafe_babe);
    let mut x = SEED.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    SEED.store(x, Ordering::Relaxed);
    x
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
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

        let win_w = COLS as f64 * CELL;
        let win_h = ROWS as f64 * CELL;
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(100., 100.), NSSize::new(win_w, win_h)),
                NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setTitle_(NSString::alloc(nil).init_str("Tetris — ←/→ move, ↓ drop"));
        window.makeKeyAndOrderFront_(nil);

        let cls = register_tetris_view_class();
        let content_view: id = window.contentView();
        let bounds: NSRect = NSView::bounds(content_view);
        let view: id = msg_send![cls, alloc];
        let view: id = msg_send![view, initWithFrame: bounds];
        let _: () = msg_send![view, setAutoresizingMask: VIEW_WIDTH_SIZABLE | VIEW_HEIGHT_SIZABLE];
        content_view.addSubview_(view);

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

fn register_tetris_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    static mut CLS: Option<&'static Class> = None;

    unsafe {
        REGISTER.call_once(|| {
            let mut decl =
                ClassDecl::new("TetrisView", class!(NSView)).expect("Failed to declare TetrisView");
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

extern "C" fn draw_rect(_this: &Object, _cmd: Sel, _dirty: NSRect) {
    unsafe {
        let cls = class!(NSBezierPath);
        let bounds: NSRect = msg_send![_this, bounds];

        // Dark background.
        let bg: id = msg_send![class!(NSColor), colorWithCalibratedRed:green:blue:alpha: 0.1f64, 0.1f64, 0.1f64, 1.0f64];
        let _: () = msg_send![bg, set];
        let _: () = msg_send![cls, fillRect: bounds];

        GAME.with(|g| {
            let g = g.borrow();

            // Draw settled blocks.
            for x in 0..COLS {
                for y in 0..ROWS {
                    if g.grid[x as usize][y as usize] {
                        let color: id = msg_send![class!(NSColor), grayColor];
                        let _: () = msg_send![color, set];
                        fill_cell(cls, x, y);
                    }
                }
            }

            // Draw the falling piece in its color.
            let (r, gg, b) = g.piece_color;
            let color: id = msg_send![
                class!(NSColor),
                colorWithCalibratedRed:green:blue:alpha: r, gg, b, 1.0f64
            ];
            let _: () = msg_send![color, set];
            fill_cell(cls, g.piece.0, g.piece.1);
        });
    }
}

unsafe fn fill_cell(cls: &'static Class, x: i32, y: i32) {
    let rect = NSRect::new(
        NSPoint::new(x as f64 * CELL, y as f64 * CELL),
        NSSize::new(CELL, CELL),
    );
    let _: () = msg_send![cls, fillRect: rect];
}

extern "C" fn key_down(_this: &Object, _cmd: Sel, event: id) {
    unsafe {
        let key_code: u16 = msg_send![event, keyCode];
        GAME.with(|g| {
            let mut g = g.borrow_mut();
            match key_code {
                123 => g.move_piece(-1), // Left
                124 => g.move_piece(1),  // Right
                125 => {
                    let _ = g.tick();
                } // Down (drop one)
                126 => g.hard_drop(),    // Up (hard drop)
                _ => {}
            }
        });
    }
}

extern "C" fn tick(this: &Object, _cmd: Sel, _timer: id) {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        if !g.tick() {
            println!("Game over! Resetting...");
            *g = Game::new();
        }
    });
    unsafe {
        let _: () = msg_send![this, setNeedsDisplay: YES];
        let _: () = msg_send![this, display];
    }
}
