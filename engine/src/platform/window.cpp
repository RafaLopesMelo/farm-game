#include <iostream>
#if defined(_WIN32)
#define GLFW_EXPOSE_NATIVE_WIN32
#elif defined(__linux__)
#ifdef HAS_WAYLAND
#define GLFW_EXPOSE_NATIVE_WAYLAND
#endif
#ifdef HAS_X11
#define GLFW_EXPOSE_NATIVE_X11
#endif
#elif defined(__APPLE__)
#define GLFW_EXPOSE_NATIVE_COCOA
#endif

#include "window.hpp"
#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>
#include <format>
#include <string>

using namespace std;

namespace engine {

Window::Window(string title, int width, int height) {
    this->title = title;
    this->width = width;
    this->height = height;

    this->status = WindowStatus::CREATED;

    this->w = nullptr;
}

Window::~Window() {
    glfwDestroyWindow(this->w);
    glfwTerminate();
}

void Window::run() {
    std::cout << "Window running" << std::endl;

    if (!glfwInit()) {
        // @TODO handle glfw initialization error
        exit(1);
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    this->w = glfwCreateWindow(width, height, title.c_str(), nullptr, nullptr);

    if (!this->w) {
        // @TODO handle glfw window initialization error
        exit(1);
    }

    // This method allows me to store my own Window structure inside GLFW
    // So we can easily retrieve it in callbacks functions
    glfwSetWindowUserPointer(this->w, this);

    glfwSetErrorCallback(Window::_handleError);
    glfwSetWindowCloseCallback(this->w, Window::_handleCloseRequest);
    glfwSetKeyCallback(this->w, Window::_handleKey);

    this->status = WindowStatus::RUNNING;
}

void Window::_handleCloseRequest(GLFWwindow *w) {
    auto *self = static_cast<Window *>(glfwGetWindowUserPointer(w));
    self->status = WindowStatus::CLOSED;
}

void Window::_handleError(int error, const char *description) {
    std::cerr << std::format("WINDOW_ERROR: {}\n", description);
}

void Window::_handleKey(GLFWwindow *w, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(w, GLFW_TRUE);
        return;
    }
}

bool Window::isRunning() { return this->status == WindowStatus::RUNNING; }

NativeWindow Window::native() {
    NativeWindow n{};

#if defined(_WIN32)
    n.type = WindowPlatform::WIN32;
    n.window = glfwGetWin32Window(this->w);
    n.display = nullptr;
#elif defined(__linux__)
#ifdef HAS_WAYLAND
    if (glfwGetPlatform() == GLFW_PLATFORM_WAYLAND) {
        n.type = WindowPlatform::WAYLAND;
        n.window = (void *)glfwGetWaylandWindow(this->w);
        n.display = glfwGetWaylandDisplay();
    }
#endif
#ifdef HAS_X11
    if (glfwGetPlatform() == GLFW_PLATFORM_X11) {
        n.type = WindowPlatform::X11;
        n.window = (void *)(uintptr_t)glfwGetX11Window(this->w);
        n.display = glfwGetX11Display();
    }
#endif
#elif defined(__APPLE__)
    n.type = WindowPlatform::COCOA;
    n.window = glfwGetCocoaWindow(this->w);
    n.display = nullptr;
#endif

    return n;
}

} // namespace engine
