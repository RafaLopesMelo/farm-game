# Repository Guidelines

## Project Structure & Module Organization
The repository is split into a reusable engine library and a runnable sample app. Core engine code lives in `engine/src`, with public headers in `engine/include/engine`. The `testbed/src` directory contains the executable entry point used to exercise the engine locally. Third-party code is vendored under `external/bgfx.cmake`. Top-level `build/`, `engine/build/`, and `testbed/build/` are generated output directories and should not be edited by hand.

## Build, Test, and Development Commands
Use the root `Makefile` for common workflows:

- `make configure`: configures CMake with the Ninja generator, `clang`, and the vcpkg toolchain.
- `make build`: runs configure, then builds all targets into `build/`.
- `cmake --build build --target testbed`: rebuilds only the sample executable.
- `./build/testbed/testbed`: runs the local sample after a successful build.
- `make clean`: removes the root build directory.

## Coding Style & Naming Conventions
This project targets C++20 with `clang`/`clang++`. Follow the existing style: 4-space indentation in `.cpp` files, braces on the same line for functions, and `namespace engine { ... }` blocks. Use `PascalCase` for classes (`Engine`), `snake_case` for directory names, and keep public headers under `engine/include/engine` aligned with their implementation files. Prefer small translation units and explicit includes over umbrella headers.

## Commit & Pull Request Guidelines
Recent commits use short, imperative subjects such as `link testbed into engine` and `general configurations`. Keep commit titles concise, lower-case, and focused on one change. Pull requests should include:

- a brief summary of behavior changed
- linked issue or task context when available
- build/run validation steps performed
- screenshots or logs if the change affects rendering or startup behavior
