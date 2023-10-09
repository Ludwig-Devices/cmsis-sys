# CMake shim.
# This is needed as the CMSIS-DSP CMake files are not set up as a self-contained
# project, but instead are required to be included in a parent project.

cmake_minimum_required(VERSION 3.14)
project(cmsis-dsp-rs VERSION 1.15.0)
set(CMSISCORE "${CMAKE_CURRENT_SOURCE_DIR}/CMSIS_5/CMSIS/Core")
add_subdirectory(CMSIS-DSP)
if (${CMAKE_CROSSCOMPILING})
    add_subdirectory(picolibc)
    # Link the picolibc implementation of the C standard library.
    target_link_libraries(CMSISDSP PUBLIC c)
    set(HOST OFF)
else()
    set(HOST ON)
endif()
