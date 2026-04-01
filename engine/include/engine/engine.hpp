#pragma once

#include <memory>
namespace engine {
class Window;

class Engine {
public:
    Engine();
    ~Engine();
    void run();

private:
    std::unique_ptr<Window> w;
};
} // namespace engine
