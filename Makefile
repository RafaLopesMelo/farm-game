BUILD_FLAGS := -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++

.PHONY: help build

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $1, $2}' $(MAKEFILE_LIST)

build:
	@echo "Building engine..."
	cd engine && \
		cmake -S . -B build $(BUILD_FLAGS) && \
		cmake --build build

