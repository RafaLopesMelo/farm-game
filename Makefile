.PHONY: help build test run watch release

.DEFAULT_GOAL := help

run:
	@echo "🎮 Running game (debug mode)..."
	cargo run -p game

watch:
	@echo "👀 Watching for changes..."
	@command -v cargo-watch >/dev/null 2>&1 || { echo "❌ cargo-watch not installed. Run: make install-tools"; exit 1; }
	cargo watch -x "run -p game"

test:
	@echo "🧪 Running tests..."
	cargo test --workspace

build:
	@echo "🔨 Building workspace..."
	cargo build

release:
	@echo "🚀 Running game (release mode)..."
	cargo run -p game --release

help:
	@echo "🎮 Game Engine Makefile Commands"
	@echo "================================"
	@echo ""
	@echo "Main Commands:"
	@echo "  make run              - Run game (debug mode)"
	@echo "  make release          - Run game (release mode) ⭐"
	@echo "  make build            - Build everything"
	@echo "  make test             - Run tests"
	@echo ""
	@echo "Development:"
	@echo "  make watch            - Auto-reload on changes"
	@echo ""
