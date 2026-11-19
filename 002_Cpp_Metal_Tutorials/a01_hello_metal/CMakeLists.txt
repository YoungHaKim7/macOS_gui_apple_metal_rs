cmake_minimum_required(VERSION 4.0)

get_filename_component(ProjectId ${CMAKE_CURRENT_SOURCE_DIR} NAME)
string(REPLACE " " "_" ProjectId ${ProjectId})
project(${ProjectId} LANGUAGES CXX)
 
# --------------------------------------------------
# C++ Standard
# --------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Force GCC 15(LinuxOS)
# set(CMAKE_CXX_COMPILER "/opt/gcc-15/bin/g++")
# Force Clang 20(LinuxOS)
# set(CMAKE_CXX_COMPILER "/usr/bin/clang++-21")
# Force Clang 20(macOS)
# set(CMAKE_CXX_COMPILER "/opt/homebrew/opt/llvm/bin/clang")

# --------------------------------------------------
# Compile Options
# --------------------------------------------------

target_compile_options(${PROJECT_ID}
    PRIVATE
        -Wall
        -Wextra
        -Wpedantic
        -Werror
        # -g
)

# --------------------------------------------------
# Metal-CPP Headers
# --------------------------------------------------
# Set this to the directory containing:
# Foundation/
# Metal/
# QuartzCore/

set(METAL_CPP_ROOT "" CACHE PATH
    "Path to the Metal-CPP headers"
)

if(NOT METAL_CPP_ROOT)
    message(FATAL_ERROR
        "Please specify METAL_CPP_ROOT"
    )
endif()

# --------------------------------------------------
# Apple Platform
# --------------------------------------------------

if(NOT APPLE)
    message(FATAL_ERROR
        "This project requires macOS and Apple Metal."
    )
endif()

# --------------------------------------------------
# Apple Frameworks
# --------------------------------------------------

find_library(FOUNDATION_FRAMEWORK
    Foundation REQUIRED
)
find_library(METAL_FRAMEWORK
    Metal REQUIRED
)
find_library(QUARTZCORE_FRAMEWORK
    QuartzCore REQUIRED
)

# --------------------------------------------------
# Main executable with C++ sources
# Executable
# --------------------------------------------------
add_executable(${ProjectId}
    src/main.cpp
    src/mtl_implementation.cpp
)

# --------------------------------------------------
# Include Directories
# --------------------------------------------------

target_include_directories(${PROJECT_ID}
    PRIVATE
        "${METAL_CPP_ROOT}"
)


# --------------------------------------------------
# Link Frameworks
# --------------------------------------------------

target_link_libraries(${PROJECT_ID}
    PRIVATE
        ${FOUNDATION_FRAMEWORK}
        ${METAL_FRAMEWORK}
        ${QUARTZCORE_FRAMEWORK}
)

# --------------------------------------------------
# Link-Time Optimization (LTO)
# --------------------------------------------------

set_property(TARGET ${PROJECT_ID}
    PROPERTY INTERPROCEDURAL_OPTIMIZATION_RELEASE TRUE
)

# --------------------------------------------------
# Output Directory
# --------------------------------------------------
set_target_properties(${ProjectId} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY
        "${CMAKE_BINARY_DIR}/$<LOWER_CASE:$<CONFIG>>"
)
# --------------------------------------------------
# Build Information
# --------------------------------------------------

message(STATUS "Project: ${PROJECT_ID}")
message(STATUS "Compiler: ${CMAKE_CXX_COMPILER}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Metal-CPP: ${METAL_CPP_ROOT}")
message(STATUS "Foundation: ${FOUNDATION_FRAMEWORK}")
message(STATUS "Metal: ${METAL_FRAMEWORK}")
message(STATUS "QuartzCore: ${QUARTZCORE_FRAMEWORK}")
