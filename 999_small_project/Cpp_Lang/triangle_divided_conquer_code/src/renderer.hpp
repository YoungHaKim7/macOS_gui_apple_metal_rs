#pragma once

#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

struct Renderer {
    MTL::Device *device = nullptr;
    MTL::CommandQueue *queue = nullptr;
    MTL::Library *library = nullptr;
    MTL::RenderPipelineState *pso = nullptr;
    CA::MetalLayer *layer = nullptr;

    bool init(CA::MetalLayer *layer);
    void render();
    void shutdown();
};
