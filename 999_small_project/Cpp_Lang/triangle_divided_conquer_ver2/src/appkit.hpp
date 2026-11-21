#pragma once

#include <CoreGraphics/CoreGraphics.h>
#include <objc/message.h>
#include <objc/objc.h>

struct Renderer;

template <typename R, typename... Args>
static inline R send(id self, SEL sel, Args... args) {
    return reinterpret_cast<R (*)(id, SEL, Args...)>(objc_msgSend)(self, sel,
                                                                   args...);
}

id NSApp();
id createWindow(CGRect frame);
id createView(CGRect frame);
id createAppDelegate();
void renderTick(void *ctx);
