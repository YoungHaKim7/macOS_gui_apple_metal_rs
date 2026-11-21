#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

#include <CoreGraphics/CoreGraphics.h>
#include <dispatch/dispatch.h>
#include <objc/message.h>
#include <objc/objc.h>
#include <objc/runtime.h>

#include <cstddef>
#include <cstdio>

constexpr NS::UInteger kSubdivisionLevels = 6;
constexpr NS::UInteger kInitialVertexCount = 3;

struct MeshVertex {
    float position[2];
    float color[4];
};

static_assert(sizeof(MeshVertex) == 24);
static_assert(offsetof(MeshVertex, color) == 8);

// objc_msgSend is declared as `void objc_msgSend(void)` in the SDK (a
// placeholder). The Apple-documented arm64 pattern is to cast it to the
// exact function-pointer type at each call site. The send<> wrapper does
// that, and fixes the ABI for narrow types (BOOL etc.).
template <typename R, typename... Args>
static inline R send(id self, SEL sel, Args... args) {
    return reinterpret_cast<R (*)(id, SEL, Args...)>(objc_msgSend)(self, sel,
                                                                   args...);
}

// ---------------------------------------------------------------------------
// Inline Metal shader source (compiled at runtime)
// ---------------------------------------------------------------------------
static const char *kShaderSrc = R"(
#include <metal_stdlib>
using namespace metal;

struct VOut {
    float4 pos [[position]];
    float4 col;
};

struct MeshVertex {
    float2 position;
    float4 color;
};

vertex VOut vs_main(
    const device MeshVertex* vertices [[buffer(0)]],
    uint vid [[vertex_id]]
) {
    VOut o;
    o.pos = float4(vertices[vid].position, 0, 1);
    o.col = vertices[vid].color;
    return o;
}

fragment float4 fs_main(VOut in [[stage_in]]) {
    return in.col;
}

kernel void subdivideTriangle(
    const device MeshVertex* input [[buffer(0)]],
    device MeshVertex* output [[buffer(1)]],
    uint id [[thread_position_in_grid]]
) {
    MeshVertex a = input[id * 3];
    MeshVertex b = input[id * 3 + 1];
    MeshVertex c = input[id * 3 + 2];

    MeshVertex ab;
    ab.position = (a.position + b.position) * 0.5;
    ab.color = (a.color + b.color) * 0.5;

    MeshVertex bc;
    bc.position = (b.position + c.position) * 0.5;
    bc.color = (b.color + c.color) * 0.5;

    MeshVertex ca;
    ca.position = (c.position + a.position) * 0.5;
    ca.color = (c.color + a.color) * 0.5;

    uint out = id * 12;
    output[out] = a;
    output[out + 1] = ab;
    output[out + 2] = ca;
    output[out + 3] = ab;
    output[out + 4] = b;
    output[out + 5] = bc;
    output[out + 6] = ca;
    output[out + 7] = bc;
    output[out + 8] = c;
    output[out + 9] = ab;
    output[out + 10] = ca;
    output[out + 11] = bc;
}
)";

// ---------------------------------------------------------------------------
// Renderer
// ---------------------------------------------------------------------------
struct Renderer {
    MTL::Device *device = nullptr;
    MTL::CommandQueue *queue = nullptr;
    MTL::Library *library = nullptr;
    MTL::RenderPipelineState *pso = nullptr;
    MTL::ComputePipelineState *computePso = nullptr;
    MTL::Buffer *meshBuffers[2] = {nullptr, nullptr};
    MTL::Buffer *mesh = nullptr;
    NS::UInteger vertexCount = kInitialVertexCount;
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

        // Compile shaders from source and report any compiler diagnostics.
        NS::String *src =
            NS::String::string(kShaderSrc, NS::UTF8StringEncoding);
        NS::Error *shaderError = nullptr;
        library = device->newLibrary(src, nullptr, &shaderError);
        if (!library) {
            std::printf("shader compile error: %s\n",
                        shaderError
                            ? shaderError->localizedDescription()->utf8String()
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

        NS::Error *err = nullptr;
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

        MTL::Function *subdivideFn = library->newFunction(
            NS::String::string("subdivideTriangle", NS::UTF8StringEncoding));
        NS::Error *computeError = nullptr;
        computePso =
            device->newComputePipelineState(subdivideFn, &computeError);
        if (subdivideFn)
            subdivideFn->release();
        if (!computePso) {
            std::printf("compute pipeline error: %s\n",
                        computeError
                            ? computeError->localizedDescription()->utf8String()
                            : "(unknown)");
            return false;
        }

        NS::UInteger triangleCount = 1;
        for (NS::UInteger level = 0; level < kSubdivisionLevels; ++level)
            triangleCount *= 4;
        const NS::UInteger maxVertexCount = triangleCount * kInitialVertexCount;
        const NS::UInteger bufferLength = maxVertexCount * sizeof(MeshVertex);

        meshBuffers[0] =
            device->newBuffer(bufferLength, MTL::ResourceStorageModeShared);
        meshBuffers[1] =
            device->newBuffer(bufferLength, MTL::ResourceStorageModeShared);
        if (!meshBuffers[0] || !meshBuffers[1]) {
            std::puts("could not allocate mesh buffers");
            return false;
        }

        MeshVertex *initial =
            static_cast<MeshVertex *>(meshBuffers[0]->contents());
        initial[0] = {{0.0f, 0.60f}, {1.0f, 0.0f, 0.0f, 1.0f}};
        initial[1] = {{-0.6f, -0.45f}, {0.0f, 1.0f, 0.0f, 1.0f}};
        initial[2] = {{0.6f, -0.45f}, {0.0f, 0.0f, 1.0f, 1.0f}};

        subdivide();
        return true;
    }

    void subdivide() {
        MTL::CommandBuffer *cmd = queue->commandBuffer();
        MTL::Buffer *input = meshBuffers[0];
        MTL::Buffer *output = meshBuffers[1];
        NS::UInteger triangleCount = 1;

        for (NS::UInteger level = 0; level < kSubdivisionLevels; ++level) {
            MTL::ComputeCommandEncoder *enc = cmd->computeCommandEncoder();
            enc->setComputePipelineState(computePso);
            enc->setBuffer(input, 0, 0);
            enc->setBuffer(output, 0, 1);
            enc->dispatchThreads(MTL::Size::Make(triangleCount, 1, 1),
                                 MTL::Size::Make(64, 1, 1));
            enc->endEncoding();

            MTL::Buffer *temporary = input;
            input = output;
            output = temporary;
            triangleCount *= 4;
        }

        cmd->commit();
        cmd->waitUntilCompleted();
        mesh = input;
        vertexCount = triangleCount * kInitialVertexCount;
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
        enc->setVertexBuffer(mesh, 0, 0);
        enc->drawPrimitives(MTL::PrimitiveTypeTriangle, NS::UInteger(0),
                            vertexCount);
        enc->endEncoding();

        cmd->presentDrawable(drawable);
        cmd->commit();

        framePool->release();
    }

    void shutdown() {
        if (meshBuffers[0])
            meshBuffers[0]->release();
        if (meshBuffers[1])
            meshBuffers[1]->release();
        if (computePso)
            computePso->release();
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
// AppKit wrappers via objc runtime
//   metal-cpp doesn't wrap AppKit classes (NSApplication / NSWindow / NSView),
//   so we reach them through the objc runtime. Class methods (e.g. +alloc,
//   +sharedApplication, +stringWithUTF8String:) are sent to the Class object;
//   instance methods (initWith…, set…, makeKey…) are sent to the instance
//   returned by +alloc.
// ---------------------------------------------------------------------------
static id NSApp() {
    static id app = send<id>((id)objc_getClass("NSApplication"),
                             sel_registerName("sharedApplication"));
    return app;
}

static id createWindow(CGRect frame) {
    id cls = (id)objc_getClass("NSWindow");
    id allocd = send<id>(cls, sel_registerName("alloc")); // +alloc -> instance
    // NSWindowStyleMaskTitled(1) | Closable(2) | Miniaturizable(4)
    NS::UInteger style = 1 | 2 | 4;
    NS::UInteger backing = 2; // NSBackingStoreBuffered
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

static id createView(CGRect frame) {
    id cls = (id)objc_getClass("NSView");
    id allocd = send<id>(cls, sel_registerName("alloc"));
    id view = send<id>(allocd, sel_registerName("initWithFrame:"), frame);
    send<void>(view, sel_registerName("setWantsLayer:"), (BOOL)1);
    return view;
}

// AppDelegate: terminate when the user closes the last window.
// NSApplicationTerminateReply is NSUInteger, so the encoding char is 'L'
// (unsigned long on LP64). Type string "L@:@" = NSUInteger(id, SEL, id).
static NS::UInteger onShouldTerminateAfterLastWindowClosed(id, SEL, id) {
    return 1; // NSTerminateNow
}

static id createAppDelegate() {
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
    // Fallback if the class name was already registered (e.g. on re-run).
    return send<id>((id)objc_getClass("TriangleAppDelegate"),
                    sel_registerName("new"));
}

// Frame callback — function-pointer variant of dispatch_source handler,
// so we don't need -fblocks for the render loop.
static void renderTick(void *ctx) { static_cast<Renderer *>(ctx)->render(); }

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
