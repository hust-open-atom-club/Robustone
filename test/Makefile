# Robustone Test Framework Makefile

.PHONY: test test-all test-riscv32 list validate clean help

# Default target
test: test-all

# Test all architectures
test-all:
	@echo "Running tests for all architectures..."
	python3 run_tests.py --all

# Test specific architectures
test-riscv32:
	@echo "Running RISC-V 32-bit tests..."
	python3 run_tests.py --arch riscv32

# List available architectures
list:
	@echo "Available architectures:"
	python3 run_tests.py --list

# Validate all configurations
validate:
	@echo "Validating architecture configurations..."
	python3 scripts/validate_configs.py

# Quick test (limited cases)
quick-test:
	@echo "Running quick tests (limited cases)..."
	python3 run_tests.py --all --limit 20

# Verbose test output
verbose-test:
	@echo "Running tests with verbose output..."
	python3 run_tests.py --all --verbose

# Initialize a new architecture (usage: make init ARCH=new_arch)
init:
	@if [ -z "$(ARCH)" ]; then \
		echo "Usage: make init ARCH=<architecture_name>"; \
		exit 1; \
	fi
	@echo "Initializing new architecture: $(ARCH)"
	python3 run_tests.py --init $(ARCH)

# Generate test cases (usage: make generate ARCH=riscv32)
generate:
	@if [ -z "$(ARCH)" ]; then \
		echo "Usage: make generate ARCH=<architecture_name>"; \
		exit 1; \
	fi
	@echo "Generating test cases for: $(ARCH)"
	python3 scripts/generate_test_cases.py --arch $(ARCH)

# Build dependencies
build-deps:
	@echo "Building cstool..."
	bash scripts/build_cstool.sh

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cd .. && cargo clean 2>/dev/null || true
	@echo "Done."

# Show help
help:
	@echo "Robustone Test Framework - Available targets:"
	@echo ""
	@echo "  test          - Run tests for all architectures (default)"
	@echo "  test-all      - Same as 'test'"
	@echo "  test-riscv32  - Run RISC-V 32-bit tests only"
	@echo "  list          - List all available architectures"
	@echo "  validate      - Validate all architecture configurations"
	@echo "  quick-test    - Run limited number of test cases for quick verification"
	@echo "  verbose-test  - Run tests with verbose output"
	@echo ""
	@echo "Architecture management:"
	@echo "  init          - Initialize new architecture (usage: make init ARCH=<name>)"
	@echo "  generate      - Generate test cases (usage: make generate ARCH=<name>)"
	@echo ""
	@echo "Maintenance:"
	@echo "  build-deps    - Build required dependencies (cstool)"
	@echo "  clean         - Clean build artifacts"
	@echo "  help          - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make test                     # Test all architectures"
	@echo "  make test-riscv32             # Test only RISC-V 32-bit"
	@echo "  make init ARCH=arm            # Create new ARM architecture"
	@echo "  make generate ARCH=riscv64    # Generate RISC-V 64-bit test cases"