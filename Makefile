CARGO ?= cargo
MANIFEST := robustone/Cargo.toml
CAPSTONE_REPO := https://github.com/capstone-engine/capstone.git
CAPSTONE_DIR := third_party/capstone

.PHONY: format run build check test

format:
	$(CARGO) fmt --manifest-path $(MANIFEST)

build:
	$(CARGO) build --manifest-path $(MANIFEST)

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
	@bash test/build_cstool.sh $(CAPSTONE_DIR)
	@python3 test/riscv32/compare_with_cstool.py $(CAPSTONE_DIR)
	$(CARGO) test --manifest-path $(MANIFEST)