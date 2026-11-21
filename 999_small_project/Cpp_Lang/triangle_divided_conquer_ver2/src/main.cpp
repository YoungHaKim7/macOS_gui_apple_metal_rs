#include <Foundation/Foundation.hpp>
#include <QuartzCore/QuartzCore.hpp>

#include <CoreGraphics/CoreGraphics.h>
#include <dispatch/dispatch.h>
#include <objc/objc.h>

#include "appkit.hpp"
#include "renderer.hpp"

// ---------------------------------------------------------------------------
int main() {
    NS::AutoreleasePool *pool = NS::AutoreleasePool::alloc()->init();

    // 1. NSApplication (regular policy => dock + menu bar)
    id app = NSApp();
    send<void>(app, sel_registerName("setActivationPolicy:"), (NS::Integer)0);
    send<void>(app, sel_registerName("setDelegate:"), createAppDelegate());

    // 2. Window + content view
    CGRect frame{CGPoint{0, 0}, CGSize{800, 600}};
    id win = createWindow(frame);
    id view = createView(frame);
    send<void>(win, sel_registerName("setContentView:"), view);

    // 3. CAMetalLayer attached to the view
    CA::MetalLayer *layer = CA::MetalLayer::layer();
    send<void>(view, sel_registerName("setLayer:"), (id)layer);

    // 4. Renderer
    Renderer r;
    if (!r.init(layer)) {
        pool->release();
        return 1;
    }

    // 5. ~60 fps timer on the main queue
    dispatch_source_t timer = dispatch_source_create(
        DISPATCH_SOURCE_TYPE_TIMER, 0, 0, dispatch_get_main_queue());
    dispatch_source_set_timer(timer, dispatch_time(DISPATCH_TIME_NOW, 0),
                              16ull * NSEC_PER_MSEC, 1ull * NSEC_PER_MSEC);
    dispatch_set_context(timer, &r);
    dispatch_source_set_event_handler_f(timer, renderTick);
    dispatch_resume(timer);

    // 6. Bring to front and run the event loop
    send<void>(app, sel_registerName("activateIgnoringOtherApps:"), (BOOL)1);
    send<void>(app, sel_registerName("run"));

    // Cleanup (only reached after the app terminates)
    dispatch_source_cancel(timer);
    dispatch_release(timer);
    r.shutdown();
    pool->release();
    return 0;
}
