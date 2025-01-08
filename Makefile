# Default rclone remote and path
RCLONE_REMOTE ?= drive_Crypt

# Detect OS for path construction
ifdef COMSPEC
    # Windows path (COMSPEC is set on Windows)
    RCLONE_PATH ?= Machines/$(shell hostname)/Users/$(shell echo %USERNAME%)
else
    # Unix-like systems (Linux/macOS)
    RCLONE_PATH ?= Machines/$(shell hostname)/$(shell basename $(shell dirname $(HOME)))/$(shell whoami)
endif

.PHONY: build build-release test clean run preview run-debug

# Default target
all: build

# Development build
build:
	cargo build

# Release build with optimizations
build-release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install release version locally
install: build-release
	cargo install --path .

# Check code formatting
fmt-check:
	cargo fmt --all -- --check

# Format code
fmt:
	cargo fmt --all

# Run clippy lints
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: fmt-check lint test

# Run the program with all CLI options
run: build
	./target/debug/backup-home \
		--source $(HOME) \
		--destination "$(RCLONE_REMOTE):$(RCLONE_PATH)" \

# Preview what would be done without actually doing it
preview: build
	./target/debug/backup-home \
		--source $(HOME) \
		--destination "$(RCLONE_REMOTE):$(RCLONE_PATH)" \
		--preview

# Run with debug logging
run-debug: build
	RUST_LOG=debug ./target/debug/backup-home \
		--source $(HOME) \
		--destination "$(RCLONE_REMOTE):$(RCLONE_PATH)" \
