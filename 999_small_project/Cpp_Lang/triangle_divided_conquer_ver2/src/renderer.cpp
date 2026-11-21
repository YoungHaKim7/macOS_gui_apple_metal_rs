#include "renderer.hpp"

#include <cstdio>

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

    MTL::Function *subdivideFn = library->newFunction(
        NS::String::string("subdivideTriangle", NS::UTF8StringEncoding));
    NS::Error *computeError = nullptr;
    computePso = device->newComputePipelineState(subdivideFn, &computeError);
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

    MeshVertex *initial = static_cast<MeshVertex *>(meshBuffers[0]->contents());
    initial[0] = {{0.0f, 0.60f}, {1.0f, 0.0f, 0.0f, 1.0f}};
    initial[1] = {{-0.6f, -0.45f}, {0.0f, 1.0f, 0.0f, 1.0f}};
    initial[2] = {{0.6f, -0.45f}, {0.0f, 0.0f, 1.0f, 1.0f}};

    subdivide();
    return true;
}

void Renderer::subdivide() {
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
    enc->setVertexBuffer(mesh, 0, 0);
    enc->drawPrimitives(MTL::PrimitiveTypeTriangle, NS::UInteger(0),
                        vertexCount);
    enc->endEncoding();

    cmd->presentDrawable(drawable);
    cmd->commit();

    framePool->release();
}

void Renderer::shutdown() {
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
