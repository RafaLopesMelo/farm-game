.PHONY: default help testbed engine.test clean

default: help

help: ## Show this help message
	@grep -E '^[a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

testbed.run: ## Build and run the testbed
	cd testbed && zig build run

engine.test: ## Run engine tests
	cd engine && zig build test

clean: ## Remove all build artifacts
	rm -rf engine/zig-out engine/.zig-cache
	rm -rf testbed/zig-out testbed/.zig-cache
	rm -rf game/zig-out game/.zig-cache
