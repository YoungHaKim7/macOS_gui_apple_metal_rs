#pragma once

#include <Foundation/Foundation.hpp>

#include <cstddef>

constexpr NS::UInteger kSubdivisionLevels = 6;
constexpr NS::UInteger kInitialVertexCount = 3;

struct MeshVertex {
    float position[2];
    float color[4];
};

static_assert(sizeof(MeshVertex) == 24);
static_assert(offsetof(MeshVertex, color) == 8);
