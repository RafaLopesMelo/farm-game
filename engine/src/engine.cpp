#include "engine/engine.hpp"
#include "bgfx/defines.h"
#include "platform/window.hpp"
#include <GLFW/glfw3.h>
#include <bgfx/bgfx.h>
#include <iostream>
#include <memory>
#include <string>

namespace engine {

Engine::Engine() = default;
Engine::~Engine() = default;

void Engine::run() {
    int width = 720;
    int height = 480;

    this->w = std::make_unique<Window>(std::string("engine"), 720, 480);
    this->w->run();
    std::cout << "Engine running\n";

    bgfx::Init init;

    init.type = bgfx::RendererType::Count;

    auto native = this->w->native();
    init.platformData.nwh = native.window;
    init.platformData.ndt = native.display;

    if (native.type == WindowPlatform::WAYLAND) {
        init.platformData.type = bgfx::NativeWindowHandleType::Wayland;
    }

    init.resolution.width = width;
    init.resolution.height = height;
    init.resolution.reset = BGFX_RESET_VSYNC;

    bgfx::init(init);
    bgfx::setViewClear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0x000000ff, 1.0f, 0);

    while (this->w->isRunning()) {
        glfwPollEvents();

        bgfx::touch(0);
        bgfx::frame();
    }

    bgfx::shutdown();
}

} // namespace engine
