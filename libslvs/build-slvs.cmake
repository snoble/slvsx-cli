# Minimal CMakeLists.txt to build only libslvs
cmake_minimum_required(VERSION 3.10)
project(libslvs)

# Set C/C++ standards
set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 11)

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/SolveSpaceLib/include
    ${CMAKE_CURRENT_SOURCE_DIR}/SolveSpaceLib/src
    ${CMAKE_CURRENT_SOURCE_DIR}/SolveSpaceLib/extlib/eigen
)

# Source files for libslvs
set(SLVS_SOURCES
    SolveSpaceLib/src/slvs/lib.cpp
    SolveSpaceLib/src/slvs/system.cpp
    SolveSpaceLib/src/slvs/entity.cpp
    SolveSpaceLib/src/slvs/constraint.cpp
    SolveSpaceLib/src/slvs/group.cpp
    SolveSpaceLib/src/slvs/request.cpp
    SolveSpaceLib/src/slvs/entityutil.cpp
    SolveSpaceLib/src/slvs/constrainteq.cpp
)

# Build static library
add_library(slvs STATIC ${SLVS_SOURCES})
target_compile_definitions(slvs PRIVATE STATIC_LIB)

# Install target
install(TARGETS slvs DESTINATION lib)
install(FILES SolveSpaceLib/include/slvs.h DESTINATION include)