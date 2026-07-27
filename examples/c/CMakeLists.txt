# gloam examples — small C programs consuming checked-in generated loaders.
#
# Each example is self-contained: its gloam/ subdirectory holds the loader
# exactly as `gloam` generated it (see each example's README for the command
# line, which is also recorded in gloam/.gloam/manifest.json).
#
# Build:
#   cmake -B build -S examples
#   cmake --build build
#
# SDL3 (needed by gl-triangle) is taken from the system when a CMake package
# is installed, otherwise fetched and built from source.

cmake_minimum_required(VERSION 3.24)
project(gloam-examples C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

find_package(SDL3 CONFIG QUIET)
if(NOT SDL3_FOUND)
    message(STATUS "SDL3 not found — fetching and building from source")
    include(FetchContent)
    set(SDL_TEST_LIBRARY OFF)
    set(SDL_EXAMPLES OFF)
    FetchContent_Declare(
        SDL3
        GIT_REPOSITORY https://github.com/libsdl-org/SDL
        GIT_TAG release-3.4.12
        GIT_SHALLOW TRUE
    )
    FetchContent_MakeAvailable(SDL3)
endif()

add_subdirectory(gl-triangle)
add_subdirectory(vk-info)
