#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

#include <CoreGraphics/CoreGraphics.h>
#include <dispatch/dispatch.h>
#include <objc/message.h>
#include <objc/objc.h>
#include <objc/runtime.h>

#include <cstdint>
#include <cstdio>

// objc runtime linkage
extern "C" id objc_getClass(const char *);
extern "C" SEL sel_registerName(const char *);
extern "C" id objc_msgSend(id, SEL, ...);

// Typed objc_msgSend wrapper
template <typename R, typename... Args>
static inline R send(id self, SEL sel, Args... args) {
    return reinterpret_cast<R (*)(id, SEL, Args...)>(objc_msgSend)(self, sel,
                                                                   args...);
}

// ---------------------------------------------------------------------------
// Inline Metal shader source (compiled at runtime via newLibraryWithSource)
// ---------------------------------------------------------------------------
static const char *kShaderSrc = R"(
#include <metal_stdlib>
using namespace metal;

struct VOut {
    float4 pos [[position]];
    float4 col;
};

vertex VOut vs_main(uint vid [[vertex_id]]) {
    float2 p[3] = {
        float2( 0.0,  0.60),
        float2(-0.6, -0.45),
        float2( 0.6, -0.45)
    };
    float4 c[3] = {
        float4(1, 0, 0, 1),
        float4(0, 1, 0, 1),
        float4(0, 0, 1, 1)
    };
    VOut o;
    o.pos = float4(p[vid], 0, 1);
    o.col = c[vid];
    return o;
}

fragment float4 fs_main(VOut in [[stage_in]]) {
    return in.col;
}
)";

// ---------------------------------------------------------------------------
// Renderer: owns device, command queue, shader library, pipeline state
// ---------------------------------------------------------------------------
struct Renderer {
    MTL::Device *device = nullptr;
    MTL::CommandQueue *queue = nullptr;
    MTL::Library *library = nullptr;
    MTL::RenderPipelineState *pso = nullptr;
    CA::MetalLayer *layer = nullptr;

    bool init(CA::MetalLayer *lyr) {
        layer = lyr;
        device = MTL::CreateSystemDefaultDevice();
        if (!device) {
            std::puts("no Metal device");
            return false;
        }

        layer->setDevice(device);
        layer->setPixelFormat(MTL::PixelFormatBGRA8Unorm);
        layer->setDrawableSize(CGSize{800, 600});

        queue = device->newCommandQueue();

        NS::Error *err = nullptr;
        NS::String *src =
            NS::String::string(kShaderSrc, NS::UTF8StringEncoding);
        library = device->newLibraryWithSource(src, nullptr, &err);
        if (!library) {
            std::printf("shader compile error: %s\n",
                        err ? err->localizedDescription()->utf8String()
                            : "(unknown)");
            return false;
        }

        MTL::RenderPipelineDescriptor *desc =
            MTL::RenderPipelineDescriptor::alloc()->init();
        MTL::Function *vsFn = library->newFunction(
            NS::String::string("vs_main", NS::UTF8StringEncoding));
        MTL::Function *fsFn = library->newFunction(
            NS::String::string("fs_main", NS::UTF8StringEncoding));
        desc->setVertexFunction(vsFn);
        desc->setFragmentFunction(fsFn);
        desc->colorAttachments()->object(0)->setPixelFormat(
            MTL::PixelFormatBGRA8Unorm);

        pso = device->newRenderPipelineState(desc, &err);
        if (vsFn)
            vsFn->release();
        if (fsFn)
            fsFn->release();
        desc->release();
        if (!pso) {
            std::printf("pipeline state error: %s\n",
                        err ? err->localizedDescription()->utf8String()
                            : "(unknown)");
            return false;
        }
        return true;
    }

    void render() {
        NS::AutoreleasePool *framePool = NS::AutoreleasePool::alloc()->init();

        CA::MetalDrawable *drawable = layer->nextDrawable();
        if (!drawable) {
            framePool->release();
            return;
        }

        MTL::RenderPassDescriptor *desc =
            MTL::RenderPassDescriptor::renderPassDescriptor();
        auto *att = desc->colorAttachments()->object(0);
        att->setTexture(drawable->texture());
        att->setLoadAction(MTL::LoadActionClear);
        att->setStoreAction(MTL::StoreActionStore);
        att->setClearColor(MTL::ClearColor::Make(0.10, 0.10, 0.12, 1.0));

        MTL::CommandBuffer *cmd = queue->commandBuffer();
        MTL::RenderCommandEncoder *enc = cmd->renderCommandEncoder(desc);
        enc->setRenderPipelineState(pso);
        enc->drawPrimitives(MTL::PrimitiveTypeTriangle, NS::UInteger(0),
                            NS::UInteger(3));
        enc->endEncoding();

        cmd->presentDrawable(drawable);
        cmd->commit();

        framePool->release();
    }

    void shutdown() {
        if (pso)
            pso->release();
        if (library)
            library->release();
        if (queue)
            queue->release();
        if (device)
            device->release();
    }
};

// ---------------------------------------------------------------------------
// AppKit wrappers via objc runtime (metal-cpp doesn't wrap AppKit)
// ---------------------------------------------------------------------------
static id NSApp() {
    static id app = send<id>((id)objc_getClass("NSApplication"),
                             sel_registerName("sharedApplication"));
    return app;
}

static id createWindow(CGRect frame) {
    id cls = (id)objc_getClass("NSWindow");
    // NSWindowStyleMaskTitled(1) | Closable(2) | Miniaturizable(4)
    NSUInteger style = 1 | 2 | 4;
    id win = send<id>(
        cls, sel_registerName("initWithContentRect:styleMask:backing:defer:"),
        frame, style, 2 /* NSBackingStoreBuffered */, (BOOL)0);
    id title = send<id>((id)objc_getClass("NSString"),
                        sel_registerName("stringWithUTF8String:"),
                        "Metal-CPP Triangle");
    send<void>(win, sel_registerName("setTitle:"), title);
    send<void>(win, sel_registerName("makeKeyAndOrderFront:"), (id) nullptr);
    return win;
}

static id createView(CGRect frame) {
    id cls = (id)objc_getClass("NSView");
    id view = send<id>(cls, sel_registerName("initWithFrame:"), frame);
    send<void>(view, sel_registerName("setWantsLayer:"), (BOOL)1);
    return view;
}

// AppDelegate: terminate when the user closes the last window.
static NSUInteger onShouldTerminateAfterLastWindowClosed(id, SEL, id) {
    return 1; // NSTerminateNow
}

static id createAppDelegate() {
    Class baseCls = (Class)objc_getClass("NSObject");
    Class newCls = objc_allocateClassPair(baseCls, "TriangleAppDelegate", 0);
    if (newCls) {
        class_addMethod(newCls,
                        sel_registerName(
                            "applicationShouldTerminateAfterLastWindowClosed:"),
                        (IMP)onShouldTerminateAfterLastWindowClosed, "Q@:@");
        objc_registerClassPair(newCls);
        return send<id>((id)newCls, sel_registerName("new"));
    }
    // Fallback if the class name was already registered (e.g. re-run)
    return send<id>((id)objc_getClass("TriangleAppDelegate"),
                    sel_registerName("new"));
}

// Frame callback (function-pointer variant; no -fblocks needed)
static void renderTick(void *ctx) { static_cast<Renderer *>(ctx)->render(); }

// ---------------------------------------------------------------------------
int main() {
    NS::AutoreleasePool *pool = NS::AutoreleasePool::alloc()->init();

    // 1. NSApplication (regular policy => dock + menu bar)
    id app = NSApp();
    send<void>(app, sel_registerName("setActivationPolicy:"), (NSInteger)0);
    send<void>(app, sel_registerName("setDelegate:"), createAppDelegate());

    // 2. Window + content view
    CGRect frame{CGPoint{0, 0}, CGSize{800, 600}};
    id win = createWindow(frame);
    id view = createView(frame);
    send<void>(win, sel_registerName("setContentView:"), view);

    // 3. CAMetalLayer on the view
    CA::MetalLayer *layer = CA::MetalLayer::layer();
    send<void>(view, sel_registerName("setLayer:"), (id)layer);

    // 4. Renderer
    Renderer r;
    if (!r.init(layer)) {
        pool->release();
        return 1;
    }

    // 5. ~60 fps timer on the main queue (function-pointer variant; no blocks)
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
