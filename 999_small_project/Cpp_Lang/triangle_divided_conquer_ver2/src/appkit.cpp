#include "appkit.hpp"

#include "renderer.hpp"

#include <Foundation/Foundation.hpp>

#include <objc/runtime.h>

id NSApp() {
    static id app = send<id>((id)objc_getClass("NSApplication"),
                             sel_registerName("sharedApplication"));
    return app;
}

id createWindow(CGRect frame) {
    id cls = (id)objc_getClass("NSWindow");
    id allocd = send<id>(cls, sel_registerName("alloc"));
    NS::UInteger style = 1 | 2 | 4;
    NS::UInteger backing = 2;
    id win = send<id>(
        allocd,
        sel_registerName("initWithContentRect:styleMask:backing:defer:"), frame,
        style, backing, (BOOL)0);
    id title = send<id>((id)objc_getClass("NSString"),
                        sel_registerName("stringWithUTF8String:"),
                        "Metal-CPP Triangle");
    send<void>(win, sel_registerName("setTitle:"), title);
    send<void>(win, sel_registerName("makeKeyAndOrderFront:"), (id) nullptr);
    return win;
}

id createView(CGRect frame) {
    id cls = (id)objc_getClass("NSView");
    id allocd = send<id>(cls, sel_registerName("alloc"));
    id view = send<id>(allocd, sel_registerName("initWithFrame:"), frame);
    send<void>(view, sel_registerName("setWantsLayer:"), (BOOL)1);
    return view;
}

static NS::UInteger onShouldTerminateAfterLastWindowClosed(id, SEL, id) {
    return 1;
}

id createAppDelegate() {
    Class baseCls = (Class)objc_getClass("NSObject");
    Class newCls = objc_allocateClassPair(baseCls, "TriangleAppDelegate", 0);
    if (newCls) {
        class_addMethod(newCls,
                        sel_registerName(
                            "applicationShouldTerminateAfterLastWindowClosed:"),
                        (IMP)onShouldTerminateAfterLastWindowClosed, "L@:@");
        objc_registerClassPair(newCls);
        return send<id>((id)newCls, sel_registerName("new"));
    }
    return send<id>((id)objc_getClass("TriangleAppDelegate"),
                    sel_registerName("new"));
}

void renderTick(void *ctx) { static_cast<Renderer *>(ctx)->render(); }
