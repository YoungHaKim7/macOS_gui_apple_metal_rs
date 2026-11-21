cmake_minimum_required(VERSION 4.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

get_filename_component(ProjectId ${CMAKE_CURRENT_SOURCE_DIR} NAME)
string(REPLACE " " "_" ProjectId ${ProjectId})
project(${ProjectId} LANGUAGES CXX)

add_compile_options(
    -pedantic-errors
    -pthread
    -Wall
    -Wextra
    -ggdb
)

add_executable(${ProjectId}
    src/main.cpp
    src/implementation.cpp
    src/appkit.cpp
    src/renderer.cpp
)

target_link_options(${ProjectId} PRIVATE -pthread -lm)

# ---- Frameworks ----
if(APPLE)
    find_library(METAL_FW      Metal      REQUIRED)
    find_library(FOUNDATION_FW Foundation REQUIRED)
    find_library(QUARTZCORE_FW QuartzCore REQUIRED)
    find_library(APPKIT_FW     AppKit     REQUIRED)
    target_link_libraries(${ProjectId} PRIVATE
        ${METAL_FW} ${FOUNDATION_FW} ${QUARTZCORE_FW} ${APPKIT_FW}
    )
endif()

# ---- Fetch metal-cpp into include/metal-cpp ----
include(FetchContent)

FetchContent_Declare(
    metal_cpp
    GIT_REPOSITORY https://github.com/apple/metal-cpp.git
    GIT_TAG        main
    SOURCE_DIR     "${CMAKE_CURRENT_SOURCE_DIR}/include/metal-cpp"
)
FetchContent_MakeAvailable(metal_cpp)

target_include_directories(${ProjectId} SYSTEM PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/include/metal-cpp"
)

set_target_properties(${ProjectId} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY
        "${CMAKE_BINARY_DIR}/$<LOWER_CASE:$<CONFIG>>"
)
