#include "renderer.hpp"

#include <CoreGraphics/CoreGraphics.h>
#include <cstdio>

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

bool Renderer::init(CA::MetalLayer *lyr) {
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

    NS::String *src = NS::String::string(kShaderSrc, NS::UTF8StringEncoding);
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
    return true;
}

void Renderer::render() {
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

void Renderer::shutdown() {
    if (pso)
        pso->release();
    if (library)
        library->release();
    if (queue)
        queue->release();
    if (device)
        device->release();
}
