# Makefile for Rust project

# Default target
all: spreadsheet

# Build the release version and rename it to "sheet"
spreadsheet:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -f sheet