BUILD_FLAGS := -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_TOOLCHAIN_FILE=$(VCPKG_ROOT)/scripts/buildsystems/vcpkg.cmake
BUILD_DIR := build
GENERATOR := Ninja

.PHONY: help build configure clean rebuild

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

configure: ## Configure the project with CMake
	cmake -S . -B $(BUILD_DIR) -G $(GENERATOR) $(BUILD_FLAGS)

build: configure ## Build all targets
	cmake --build $(BUILD_DIR)

clean: ## Remove build directory
	rm -rf $(BUILD_DIR)
