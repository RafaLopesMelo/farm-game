#pragma once

#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>
#include <string>

namespace engine {

enum class WindowPlatform { WAYLAND, X11, COCOA, WIN32 };

struct NativeWindow {
    WindowPlatform type;

    void *window;
    void *display;
};

enum class WindowStatus { CREATED, RUNNING, CLOSED };
class Window {
public:
    Window(std::string title, int width, int height);
    ~Window();

    void run();
    NativeWindow native();

    bool isRunning();

private:
    std::string title;
    int width;
    int height;

    WindowStatus status;

    GLFWwindow *w;

    static void _handleCloseRequest(GLFWwindow *w);
    static void _handleError(int error, const char *description);
    static void _handleKey(GLFWwindow *w, int key, int scancode, int action, int mods);
};

} // namespace engine
