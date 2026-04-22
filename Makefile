CARGO ?= cargo
PYTHON ?= python3

MANIFEST := robustone/Cargo.toml
CAPSTONE_REPO := https://github.com/capstone-engine/capstone.git
CAPSTONE_DIR := third_party/capstone
CAPSTONE_BUILD_SCRIPT := test/scripts/build_cstool.sh
PARITY_SCRIPT := test/run_tests.py

VENV_DIR := virt-py
VENV_PIP := $(VENV_DIR)/bin/pip
VENV_PYTHON := $(CURDIR)/$(VENV_DIR)/bin/python
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

.PHONY: format run build check check-clippy check-pylint check-fmt check-all test test-parity test-validate test-list test-quick clean-help help virt-env pre-commit-install

virt-env:
	$(PYTHON) -m venv virt-py
	$(VENV_PIP) install -r requirements.txt

format: virt-env
	$(CARGO) fmt --all
	$(VENV_BLACK) test

build:
	$(CARGO) build --manifest-path $(MANIFEST)

run:
	$(CARGO) run --manifest-path $(MANIFEST) -- $(RUN_ARGS)

check: virt-env
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --workspace --all-features -- -D warnings
	$(VENV_PYLINT) $$(find test/ -type f -name "*.py")
	$(VENV_BLACK) --check test/

check-clippy: virt-env
	$(CARGO) clippy --workspace --all-features -- -D warnings

check-pylint: virt-env
	$(VENV_PYLINT) $$(find test/ -type f -name "*.py")

check-fmt: virt-env
	$(CARGO) fmt --all -- --check
	$(VENV_BLACK) --check test

check-all: check check-clippy check-pylint check-fmt
	@echo "Running Rust workspace tests..."
	$(CARGO) test --workspace --all-features
	@echo "All checks passed!"

test: virt-env
	@mkdir -p $(dir $(CAPSTONE_DIR))
	@if [ ! -d "$(CAPSTONE_DIR)" ]; then \
		echo "Cloning Capstone into $(CAPSTONE_DIR)..."; \
		git clone --depth 1 $(CAPSTONE_REPO) $(CAPSTONE_DIR); \
	else \
		echo "Capstone already present at $(CAPSTONE_DIR)."; \
	fi
	@bash $(CAPSTONE_BUILD_SCRIPT) $(CAPSTONE_DIR)
	@echo "Running Python unit tests..."
	@$(VENV_PYTHON) -m unittest discover -s test -p "test_*.py"
	@echo "Running parity tests with new framework..."
	@cd test && $(VENV_PYTHON) run_tests.py --all
	@echo "Running Rust workspace tests..."
	$(CARGO) test --workspace --all-features

test-parity: virt-env
	@echo "Running parity tests only..."
	@cd test && $(VENV_PYTHON) run_tests.py --all

test-validate: virt-env
	@echo "Validating test configurations..."
	@cd test && $(VENV_PYTHON) scripts/validate_configs.py

test-list: virt-env
	@echo "Available test architectures:"
	@cd test && $(VENV_PYTHON) run_tests.py --list

test-quick: virt-env
	@echo "Running quick parity test (limited cases)..."
	@cd test && $(VENV_PYTHON) run_tests.py --all --limit 20

clean-help:
	@echo "Available targets:"
	@echo ""
	@echo "Build & Check:"
	@echo "  build        - Build the CLI crate in debug mode"
	@echo "  check        - Run repository checks on workspace code and test harness scripts"
	@echo "  check-clippy - Run Rust clippy lints (with -D warnings)"
	@echo "  check-fmt    - Check Rust and Python formatting"
	@echo "  check-all    - Run the full repository check suite"
	@echo "  format       - Format code with rustfmt"
	@echo ""
	@echo "Testing:"
	@echo "  test         - Run full test suite (parity + workspace tests)"
	@echo "  test-parity  - Run parity tests only"
	@echo "  test-validate - Validate test configurations"
	@echo "  test-list    - List available test architectures"
	@echo "  test-quick   - Run quick parity test (limited cases)"
	@echo ""
	@echo "Utility:"
	@echo "  run          - Run the CLI with args (usage: make run -- <args>)"
	@echo "  help         - Show this help message"
	@echo "  clean-help   - Backward-compatible alias for help"
	@echo "  pre-commit-install - Install pre-commit hooks"
	@echo ""
	@echo "For more test options, see test/Makefile or run:"
	@echo "  cd test && make help"

pre-commit-install: virt-env
	$(VENV_PIP) install pre-commit
	$(VENV_DIR)/bin/pre-commit install --install-hooks

help: clean-help
