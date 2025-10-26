CARGO ?= cargo
PYTHON ?= python3

MANIFEST := robustone/Cargo.toml
CAPSTONE_REPO := https://github.com/capstone-engine/capstone.git
CAPSTONE_DIR := third_party/capstone
CAPSTONE_BUILD_SCRIPT := test/build_cstool.sh
PARITY_SCRIPT := test/riscv32/test_vs_cstool.py

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

.PHONY: format run build check test

format:
	$(CARGO) fmt --manifest-path $(MANIFEST)

build:
	$(CARGO) build --manifest-path $(MANIFEST)

run:
	$(CARGO) run --manifest-path $(MANIFEST) -- $(RUN_ARGS)

check:
	$(CARGO) check --manifest-path $(MANIFEST)

test:
	@mkdir -p $(dir $(CAPSTONE_DIR))
	@if [ ! -d "$(CAPSTONE_DIR)" ]; then \
		echo "Cloning Capstone into $(CAPSTONE_DIR)..."; \
		git clone --depth 1 $(CAPSTONE_REPO) $(CAPSTONE_DIR); \
	else \
		echo "Capstone already present at $(CAPSTONE_DIR)."; \
	fi
	@bash $(CAPSTONE_BUILD_SCRIPT) $(CAPSTONE_DIR)
	@$(PYTHON) $(PARITY_SCRIPT)
	$(CARGO) test --manifest-path $(MANIFEST)
