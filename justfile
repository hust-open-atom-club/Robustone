# Justfile for Robustone project
# Usage: just <command>

# Default recipe
default:
    @just --list

# Development commands
setup-dev:
    # Install development dependencies
    @echo "Setting up development environment..."
    cargo install cargo-watch cargo-nextest cargo-criterion
    pip3 install --break-system-packages --upgrade pip
    pre-commit install

build:
    # Build the project
    cargo build

build-release:
    # Build release version
    cargo build --release

run:
    # Run the CLI
    cargo run

test:
    # Run all tests
    cargo test --workspace

test-unit:
    # Run unit tests only
    cargo test --workspace --lib

test-integration:
    # Run integration tests
    @echo "Building Capstone..."
    @if [ ! -d "third_party/capstone" ]; then \
        git clone --depth 1 https://github.com/capstone-engine/capstone.git third_party/capstone; \
    fi
    @bash test/scripts/build_cstool.sh third_party/capstone
    @cd test && python3 run_tests.py --all --limit 20

test-quick:
    # Run quick tests
    cargo test --workspace --lib

bench:
    # Run benchmarks
    cargo criterion --workspace

format:
    # Format code
    cargo fmt

check:
    # Check code (no build)
    cargo check

clippy:
    # Run clippy
    cargo clippy --workspace --all-features

doc:
    # Generate documentation
    cargo doc --workspace --all-features

doc-open:
    # Generate and open documentation
    cargo doc --workspace --all-features --open

clean:
    # Clean build artifacts
    cargo clean

# Docker commands
docker-build:
    # Build Docker image
    docker build -t robustone:latest .

docker-run:
    # Run Docker container
    docker run --rm robustone:latest

docker-dev:
    # Run development container
    docker-compose --profile dev up robustone-dev

docker-dev-bash:
    # Get shell in development container
    docker-compose --profile dev run robustone-dev bash

# Release commands
version:
    # Show version
    @cargo run -- --version

publish-dry:
    # Dry run publish to crates.io
    cargo publish --dry-run

# Quality checks
quality-check: format clippy test
    # Run all quality checks

ci: quality-check test-integration
    # Run CI-like checks

# Monitoring
watch:
    # Watch for changes and re-run
    cargo watch -x 'run'

watch-test:
    # Watch for changes and re-test
    cargo watch -x test

# Utility commands
update-deps:
    # Update dependencies
    cargo update

tree:
    # Show dependency tree
    cargo tree

size:
    # Show crate sizes
    cargo du --depth 1

audit:
    # Security audit
    cargo audit

# Workspace management
workspace-status:
    # Show workspace status
    @echo "Workspace members:"
    @cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]' | sed 's/.*#//'

list-targets:
    # List build targets
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("robustone")) | .targets[] | select(.kind[] == "bin") | .name'

# Third party management
setup-capstone:
    # Set up Capstone for testing
    @if [ ! -d "third_party/capstone" ]; then \
        echo "Cloning Capstone..."; \
        git clone --depth 1 https://github.com/capstone-engine/capstone.git third_party/capstone; \
    else \
        echo "Capstone already exists"; \
    fi

clean-capstone:
    # Clean Capstone
    @rm -rf third_party/capstone

# Help commands
help-dev:
    # Show development commands
    @echo "Development commands:"
    @echo "  setup-dev       - Set up development environment"
    @echo "  watch           - Watch for changes and re-run"
    @echo "  watch-test      - Watch for changes and re-test"
    @echo "  docker-dev      - Run development container"
    @echo "  quality-check   - Run all quality checks"

help-test:
    # Show testing commands
    @echo "Testing commands:"
    @echo "  test            - Run all tests"
    @echo "  test-unit       - Run unit tests only"
    @echo "  test-integration - Run integration tests"
    @echo "  test-quick      - Run quick tests"
    @echo "  bench           - Run benchmarks"

help-build:
    # Show build commands
    @echo "Build commands:"
    @echo "  build           - Debug build"
    @echo "  build-release   - Release build"
    @echo "  docker-build    - Build Docker image"
    @echo "  format          - Format code"
    @echo "  clippy          - Run clippy"