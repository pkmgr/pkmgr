# Makefile for pkmgr universal package manager
# NEVER use this in CI/CD - CI has explicit commands with env vars

PROJECT_NAME = pkmgr
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo 'dev')
COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo 'unknown')
BUILD_DATE := $(shell date -u +%Y-%m-%dT%H:%M:%SZ)

# Build flags (for local dev only - CI has its own)
LDFLAGS := -s -w -X 'main.Version=$(VERSION)' -X 'main.CommitID=$(COMMIT)' -X 'main.BuildDate=$(BUILD_DATE)'

# Cross-compilation targets
TARGETS = \
	x86_64-unknown-linux-musl \
	aarch64-unknown-linux-musl \
	x86_64-apple-darwin \
	aarch64-apple-darwin \
	x86_64-pc-windows-msvc \
	aarch64-pc-windows-msvc \
	x86_64-unknown-freebsd \
	aarch64-unknown-freebsd

.PHONY: help build release docker test clean install dev

# Default target
help:
	@echo "🚀 $(PROJECT_NAME) Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build    - Build for current platform (Docker)"
	@echo "  dev      - Quick build for development"
	@echo "  release  - Build all platforms (Docker)"
	@echo "  docker   - Build Docker image"
	@echo "  test     - Run tests (Docker + Incus)"
	@echo "  install  - Install to ~/.local/bin"
	@echo "  clean    - Clean build artifacts"

# Build for current platform using Docker
build:
	@echo "🔨 Building for current platform..."
	@docker build -f Dockerfile --target builder -t $(PROJECT_NAME)-builder .
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target x86_64-unknown-linux-musl"
	@mkdir -p binaries
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)
	@echo "✅ Binary ready: ./binaries/$(PROJECT_NAME)"

# Quick development build (native cargo)
dev:
	@echo "🚀 Development build..."
	@cargo build
	@echo "✅ Dev binary: ./target/debug/$(PROJECT_NAME)"
	@./target/debug/$(PROJECT_NAME) --version

# Build all platforms using Docker
release:
	@echo "🔨 Building for all platforms..."
	@docker build -f Dockerfile --target builder -t $(PROJECT_NAME)-builder .
	@mkdir -p binaries
	@echo "Building Linux amd64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target x86_64-unknown-linux-musl" && \
		cp target/x86_64-unknown-linux-musl/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-linux-amd64
	@echo "Building Linux arm64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target aarch64-unknown-linux-musl" && \
		cp target/aarch64-unknown-linux-musl/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-linux-arm64
	@echo "Building macOS amd64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target x86_64-apple-darwin" && \
		cp target/x86_64-apple-darwin/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-darwin-amd64
	@echo "Building macOS arm64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target aarch64-apple-darwin" && \
		cp target/aarch64-apple-darwin/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-darwin-arm64
	@echo "Building Windows amd64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target x86_64-pc-windows-msvc" && \
		cp target/x86_64-pc-windows-msvc/release/$(PROJECT_NAME).exe binaries/$(PROJECT_NAME)-windows-amd64.exe
	@echo "Building Windows arm64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target aarch64-pc-windows-msvc" && \
		cp target/aarch64-pc-windows-msvc/release/$(PROJECT_NAME).exe binaries/$(PROJECT_NAME)-windows-arm64.exe
	@echo "Building FreeBSD amd64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target x86_64-unknown-freebsd" && \
		cp target/x86_64-unknown-freebsd/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-freebsd-amd64
	@echo "Building FreeBSD arm64..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder \
		sh -c "cargo build --release --target aarch64-unknown-freebsd" && \
		cp target/aarch64-unknown-freebsd/release/$(PROJECT_NAME) binaries/$(PROJECT_NAME)-freebsd-arm64
	@echo "Generating checksums..."
	@cd binaries && sha256sum * > SHA256SUMS
	@echo "✅ All binaries ready in ./binaries/"
	@ls -lh binaries/

# Install to local system
install: build
	@echo "📦 Installing to ~/.local/bin..."
	@mkdir -p ~/.local/bin
	@cp binaries/$(PROJECT_NAME) ~/.local/bin/$(PROJECT_NAME)
	@chmod +x ~/.local/bin/$(PROJECT_NAME)
	@echo "✅ Installed: ~/.local/bin/$(PROJECT_NAME)"

# Build Docker image
docker:
	@echo "🐳 Building Docker image..."
	@docker build -f Dockerfile -t $(PROJECT_NAME):latest .
	@docker build -f Dockerfile -t $(PROJECT_NAME):$(VERSION) .
	@echo "✅ Docker images built:"
	@echo "  - $(PROJECT_NAME):latest"
	@echo "  - $(PROJECT_NAME):$(VERSION)"

# Run comprehensive tests (Docker + Incus)
test: build
	@echo "🧪 Running comprehensive tests..."
	@echo "Running unit tests..."
	@docker run --rm -v $(PWD):/workspace -w /workspace $(PROJECT_NAME)-builder cargo test
	@echo "Testing binary functionality..."
	@docker run --rm -v $(PWD):/workspace ubuntu:22.04 \
		/workspace/binaries/$(PROJECT_NAME) --version
	@echo "Testing on multiple distros with Docker..."
	@docker run --rm -v $(PWD):/workspace ubuntu:22.04 \
		/workspace/binaries/$(PROJECT_NAME) --help
	@docker run --rm -v $(PWD):/workspace debian:12 \
		/workspace/binaries/$(PROJECT_NAME) --help
	@docker run --rm -v $(PWD):/workspace alpine:latest \
		/workspace/binaries/$(PROJECT_NAME) --help
	@docker run --rm -v $(PWD):/workspace fedora:39 \
		/workspace/binaries/$(PROJECT_NAME) --help
	@echo "Testing with Incus (full OS containers)..."
	@bash tests/test_incus.sh
	@echo "✅ All tests passed"

# Clean up build artifacts
clean:
	@echo "🧹 Cleaning up..."
	@cargo clean
	@rm -rf binaries/ releases/ target/
	@docker image rm $(PROJECT_NAME)-builder 2>/dev/null || true
	@echo "✅ Cleanup completed"