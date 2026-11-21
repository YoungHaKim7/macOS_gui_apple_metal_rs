#pragma once

#include "common.hpp"

#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

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

    bool init(CA::MetalLayer *lyr);
    void subdivide();
    void render();
    void shutdown();
};
