CARGO ?= cargo
PYTHON ?= python3

MANIFEST := robustone/Cargo.toml
CAPSTONE_REPO := https://github.com/capstone-engine/capstone.git
CAPSTONE_DIR := third_party/capstone
CAPSTONE_BUILD_SCRIPT := test/scripts/build_cstool.sh
PARITY_SCRIPT := test/run_tests.py

VENV_DIR := virt-py
VENV_PIP := $(VENV_DIR)/bin/pip
VENV_BLACK := $(VENV_DIR)/bin/black
VENV_PYLINT := $(VENV_DIR)/bin/pylint

ifeq ($(firstword $(MAKECMDGOALS)),run)
RUN_EXTRA := $(filter-out --,$(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS)))
ifneq ($(RUN_EXTRA),)
ifeq ($(origin RUN_ARGS),undefined)
RUN_ARGS := $(RUN_EXTRA)
endif
$(foreach target,$(RUN_EXTRA),$(eval $(target): ; @:))
endif
endif

RUN_ARGS ?=

.PHONY: format run build check check-clippy check-pylint check-fmt check-all test test-parity test-validate clean-help virt-env

virt-env:
	$(PYTHON) -m venv virt-py
	$(VENV_PIP) install -r requirements.txt

format: virt-env
	$(CARGO) fmt --manifest-path $(MANIFEST)
	$(VENV_BLACK) test

build:
	$(CARGO) build --manifest-path $(MANIFEST)

run:
	$(CARGO) run --manifest-path $(MANIFEST) -- $(RUN_ARGS)

check: virt-env
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --workspace --all-features -- -D warnings
	$(VENV_PYLINT) $$(find . -type f -name "*.py")
	$(VENV_BLACK) --check test/

check-clippy: virt-env
	$(CARGO) clippy --workspace --all-features -- -D warnings

check-pylint: virt-env
	$(VENV_PYLINT) $$(find test/ -type f -name "*.py")

check-fmt: virt-env
	$(CARGO) fmt --all -- --check
	$(VENV_BLACK) --check test

check-all: check check-clippy check-pylint check-fmt
	@echo "All checks passed!"

test:
	@mkdir -p $(dir $(CAPSTONE_DIR))
	@if [ ! -d "$(CAPSTONE_DIR)" ]; then \
		echo "Cloning Capstone into $(CAPSTONE_DIR)..."; \
		git clone --depth 1 $(CAPSTONE_REPO) $(CAPSTONE_DIR); \
	else \
		echo "Capstone already present at $(CAPSTONE_DIR)."; \
	fi
	@bash $(CAPSTONE_BUILD_SCRIPT) $(CAPSTONE_DIR)
	@echo "Running parity tests with new framework..."
	@cd test && $(PYTHON) run_tests.py --all
	@echo "Running Rust unit tests..."
	$(CARGO) test --manifest-path $(MANIFEST)

test-parity:
	@echo "Running parity tests only..."
	@cd test && $(PYTHON) run_tests.py --all

test-validate:
	@echo "Validating test configurations..."
	@cd test && $(PYTHON) scripts/validate_configs.py

test-list:
	@echo "Available test architectures:"
	@cd test && $(PYTHON) run_tests.py --list

test-quick:
	@echo "Running quick parity test (limited cases)..."
	@cd test && $(PYTHON) run_tests.py --all --limit 20

clean-help:
	@echo "Available targets:"
	@echo ""
	@echo "Build & Check:"
	@echo "  build        - Build the project"
	@echo "  check        - Run cargo check (basic compilation check)"
	@echo "  check-clippy - Run clippy lints (with -D warnings)"
	@echo "  check-fmt    - Check code formatting"
	@echo "  check-all    - Run all checks (check + clippy + fmt)"
	@echo "  format       - Format code with rustfmt"
	@echo ""
	@echo "Testing:"
	@echo "  test         - Run full test suite (parity + unit tests)"
	@echo "  test-parity  - Run parity tests only"
	@echo "  test-validate - Validate test configurations"
	@echo "  test-list    - List available test architectures"
	@echo "  test-quick   - Run quick parity test (limited cases)"
	@echo ""
	@echo "Utility:"
	@echo "  run          - Run the CLI with args (usage: make run -- <args>)"
	@echo "  clean-help   - Show this help message"
	@echo ""
	@echo "For more test options, see test/Makefile or run:"
	@echo "  cd test && make help"
