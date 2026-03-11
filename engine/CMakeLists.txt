cmake_minimum_required(VERSION 3.16)
project(Engine VERSION 0.1 LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_subdirectory(external/bgfx.cmake)

find_package(glfw3 CONFIG REQUIRED)

add_executable(engine
    src/main.cpp
)

target_link_libraries(engine
    PRIVATE
    glfw
    bgfx
    bimg
    bx
)
