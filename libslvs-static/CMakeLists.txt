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
)

# mimalloc removed to fix memory allocator conflicts

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

# mimalloc removed to fix memory allocator conflicts
# target_link_libraries(slvs PRIVATE mimalloc-static)

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

# mimalloc removed
# install(TARGETS mimalloc-static
#     ARCHIVE DESTINATION lib
# )

# Create a combined static library (mimalloc removed)
add_custom_target(slvs-combined ALL
    COMMAND ${CMAKE_AR} -x $<TARGET_FILE:slvs>
    COMMAND ${CMAKE_AR} -qcs ${CMAKE_CURRENT_BINARY_DIR}/libslvs-combined.a *.o
    COMMAND ${CMAKE_COMMAND} -E remove *.o
    DEPENDS slvs
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Creating combined static library"
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
message(STATUS "  Includes mimalloc: No (removed to fix memory conflicts)")
message(STATUS "  GPL-3.0 Licensed")