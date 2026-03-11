#include "engine/engine.hpp"
#include <iostream>

namespace engine {

Engine::Engine() = default;

void Engine::run() { std::cout << "Engine running\n"; }

} // namespace engine
