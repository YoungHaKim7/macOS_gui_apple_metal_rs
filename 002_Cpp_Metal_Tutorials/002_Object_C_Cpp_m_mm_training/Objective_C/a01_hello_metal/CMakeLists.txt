cmake_minimum_required(VERSION 4.0)

set(CMAKE_C_STANDARD 23)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

# set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Objective-C++ standard
set(CMAKE_OBJC_STANDARD 23)
set(CMAKE_OBJC_STANDARD_REQUIRED ON)
set(CMAKE_OBJC_EXTENSIONS OFF)

get_filename_component(ProjectId ${CMAKE_CURRENT_SOURCE_DIR} NAME)
string(REPLACE " " "_" ProjectId ${ProjectId})
project(${ProjectId} LANGUAGES C OBJC)
# Enable Objective-C
enable_language(OBJC)

# Common compile flags
add_compile_options(
    -pedantic
    -pedantic-errors
    -Wall
    -Wextra
    # -std=c23
)

# Main executable with Objective-C sources
add_executable(${ProjectId}
    src/main.m
    # src/add_source.m
)

# Threads
find_package(Threads REQUIRED)

# macOS frameworks
if(APPLE)
    target_link_libraries(${ProjectId} PRIVATE
        "-framework Foundation"
    )
endif()

# Output directory
set_target_properties(${ProjectId} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY
        "${CMAKE_BINARY_DIR}/$<LOWER_CASE:$<CONFIG>>"
)
