file(GLOB_RECURSE ENGINE_SOURCES src/*.cpp)

add_library(engine STATIC ${ENGINE_SOURCES})

target_include_directories(engine
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(engine
    PRIVATE
    glfw
    bgfx
    bimg
    bx
)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    find_package(PkgConfig)
    pkg_check_modules(WAYLAND wayland-client)
    pkg_check_modules(X11 x11)

    if(WAYLAND_FOUND)
        target_compile_definitions(engine PRIVATE HAS_WAYLAND)
        target_link_libraries(engine PRIVATE wayland-client)
    endif()
    if(X11_FOUND)
        target_compile_definitions(engine PRIVATE HAS_X11)
        target_link_libraries(engine PRIVATE X11)
    endif()
endif()

add_library(engine::engine ALIAS engine)
