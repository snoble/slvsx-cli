cmake_minimum_required(VERSION 3.10)
project(libslvs-static VERSION 3.0 LANGUAGES C CXX)

# This is a fork of SolveSpace libslvs for static linking
# Licensed under GPL-3.0 - see LICENSE and COPYING.txt

# Set C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD 99)

# Options
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)

# Always build as static
set(BUILD_SHARED_LIBS OFF)

# Add include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/extlib/eigen
    ${CMAKE_CURRENT_SOURCE_DIR}/extlib/eigen/Eigen
    ${CMAKE_CURRENT_SOURCE_DIR}/extlib
    ${CMAKE_CURRENT_SOURCE_DIR}/extlib/mimalloc/include
)

# Build mimalloc using its own CMakeLists
# This ensures proper configuration and all dependencies
set(MI_BUILD_SHARED OFF CACHE BOOL "Build shared library" FORCE)
set(MI_BUILD_STATIC ON CACHE BOOL "Build static library" FORCE)
set(MI_BUILD_TESTS OFF CACHE BOOL "Build tests" FORCE)
set(MI_BUILD_OBJECT OFF CACHE BOOL "Build object library" FORCE)
add_subdirectory(extlib/mimalloc EXCLUDE_FROM_ALL)

# Core solver sources
set(SOLVER_SOURCES
    src/util.cpp
    src/entity.cpp
    src/expr.cpp
    src/constrainteq.cpp
    src/system.cpp
    src/platform/platformbase.cpp
)

# Build the solver as an object library
add_library(slvs-solver-obj OBJECT ${SOLVER_SOURCES})
target_compile_definitions(slvs-solver-obj PRIVATE LIBRARY)
set_target_properties(slvs-solver-obj PROPERTIES POSITION_INDEPENDENT_CODE ON)

# Build libslvs static library
add_library(slvs STATIC
    src/slvs/lib.cpp
    $<TARGET_OBJECTS:slvs-solver-obj>
)

target_compile_definitions(slvs PRIVATE 
    LIBRARY
    STATIC_LIB
)

# Link mimalloc (the target is created by mimalloc's CMakeLists)
target_link_libraries(slvs PRIVATE mimalloc-static)

# Set properties
set_target_properties(slvs PROPERTIES
    OUTPUT_NAME slvs
    POSITION_INDEPENDENT_CODE ON
    PUBLIC_HEADER ${CMAKE_CURRENT_SOURCE_DIR}/include/slvs.h
)

# Installation rules (for local builds)
install(TARGETS slvs
    ARCHIVE DESTINATION lib
    PUBLIC_HEADER DESTINATION include
)

install(TARGETS mimalloc-static
    ARCHIVE DESTINATION lib
)

# Create a combined static library that includes everything
add_custom_target(slvs-combined ALL
    COMMAND ${CMAKE_AR} -x $<TARGET_FILE:slvs>
    COMMAND ${CMAKE_AR} -x $<TARGET_FILE:mimalloc-static>
    COMMAND ${CMAKE_AR} -qcs ${CMAKE_CURRENT_BINARY_DIR}/libslvs-combined.a *.o
    COMMAND ${CMAKE_COMMAND} -E remove *.o
    DEPENDS slvs mimalloc-static
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Creating combined static library with all dependencies"
)

# Install the combined library
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/libslvs-combined.a
    DESTINATION lib
    RENAME libslvs.a
)

# Print configuration
message(STATUS "Building libslvs-static:")
message(STATUS "  Version: ${PROJECT_VERSION}")
message(STATUS "  Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  Static library: libslvs.a")
message(STATUS "  Includes mimalloc: Yes")
message(STATUS "  GPL-3.0 Licensed")