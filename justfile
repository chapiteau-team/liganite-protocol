#!/usr/bin/env just --justfile

CARGO := "cargo"
CARGO_NIGHTLY := "cargo +nightly"
CARGO_BUILD_FLAGS := "--locked"
RUNTIME_PROD := "./target/production/wbuild/liganite-runtime/liganite_runtime.compact.compressed.wasm"
WEIGHTS_PATH := "./runtime/src/weights"

# Add color support for better readability

BOLD := `tput bold 2>/dev/null || echo ''`
GREEN := `tput setaf 2 2>/dev/null || echo ''`
RED := `tput setaf 1 2>/dev/null || echo ''`
RESET := `tput sgr0 2>/dev/null || echo ''`

################################################################################
# UTILITIES SECTION
#
# This section contains targets for general-purpose tasks unrelated to the
# codebase itself (e.g., installing dependencies, printing system specific
# information, etc).
################################################################################

# List available commands
help:
    @just --list

# Install all dependencies
install:
    @chmod +x ./.maintenance/scripts/install-openssl.sh
    @./.maintenance/scripts/install-openssl.sh || (echo "{{ RED }}Failed to install openssl{{ RESET }}" && exit 1)

install-bencher:
    @cargo install frame-omni-bencher

# Check if the stable toolchain is installed
check-stable-toolchain:
    @if ! rustup toolchain list | grep -q stable; then \
    	echo "Stable toolchain not found. Installing..."; \
    	rustup toolchain install stable; \
    else \
    	echo "Stable toolchain already installed."; \
    fi

# Check if the nightly toolchain is installed
check-nightly-toolchain:
    @if ! rustup toolchain list | grep -q nightly; then \
    	echo "Nightly toolchain not found. Installing..."; \
    	rustup toolchain install nightly; \
    else \
    	echo "Nightly toolchain already installed."; \
    fi

################################################################################
# CODE QUALITY SECTION
#
# This section contains targets for maintaining and improving the quality of the
# codebase.
################################################################################

# Run Clippy on the codebase
clippy: check-stable-toolchain
    {{ CARGO }} clippy --locked --all-targets --all-features -- -D warnings

# Run rustfmt on the codebase
fmt: check-nightly-toolchain
    {{ CARGO_NIGHTLY }} fmt --all -- --check

################################################################################
# BUILD SECTION
#
# This section contains targets related to building the codebase.
################################################################################

# Generate a Cargo.lock file
gen-lockfile:
    {{ CARGO }} generate-lockfile

# Clean up the project
clean:
    {{ CARGO }} clean

# Compile all projects without code generation
check-all: check-stable-toolchain
    {{ CARGO }} check {{ CARGO_BUILD_FLAGS }}

# Compile a specific crate (e.g. just check-crate runtime) without code generation
check crate: check-stable-toolchain
    {{ CARGO }} check -p liganite-{{ crate }} {{ CARGO_BUILD_FLAGS }}

# Build all projects
build-all: check-stable-toolchain
    {{ CARGO }} build {{ CARGO_BUILD_FLAGS }}

# Build benchmarks
build-bench: check-stable-toolchain
    {{ CARGO }} build --features runtime-benchmarks --profile=production {{ CARGO_BUILD_FLAGS }}

# Build a specific project (e.g. just build-crate runtime)
build crate: check-stable-toolchain
    {{ CARGO }} build -p liganite-{{ crate }} {{ CARGO_BUILD_FLAGS }}

################################################################################
# TESTING SECTION
#
# This section contains targets for running tests to validate the functionality
# and reliability of the project.
################################################################################

# Run all tests
test-all: check-stable-toolchain
    {{ CARGO }} test --features runtime-benchmarks {{ CARGO_BUILD_FLAGS }}

################################################################################
# RUN SECTION
#
# This section contains targets for launching components for development or
# testing purposes.
################################################################################

# Benchmark all pallets
bench-pallets: install-bencher build-bench
    @chmod +x ./.maintenance/scripts/benchmark-pallets.sh
    @./.maintenance/scripts/benchmark-pallets.sh {{ RUNTIME_PROD }} {{ WEIGHTS_PATH }}

# Benchmark overhead
bench-overhead: install-bencher build-bench
    @frame-omni-bencher v1 benchmark overhead --runtime {{ RUNTIME_PROD }} --weight-path {{ WEIGHTS_PATH }}

# Run all benchmarks
bench-all: bench-pallets bench-overhead

# Run the node in development mode
run-dev: check-stable-toolchain
    {{ CARGO }} run -- --dev
