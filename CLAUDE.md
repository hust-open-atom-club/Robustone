# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Robustone is an experimental disassembly engine written in Rust, inspired by the Capstone project. It is a Cargo workspace that explores Rust's safety guarantees to deliver a Capstone-compatible experience. Currently, only the **RISC-V** backend (`riscv32`, `riscv64`) is fully implemented. The `arm`, `x86`, and `loongarch` crates are skeleton placeholders.

## Workspace Architecture

The workspace has a layered architecture with clear separation of concerns:

| Crate | Role |
|-------|------|
| `robustone` | **Meta-crate** — re-exports all sub-crates and provides the `dispatcher()` helper that registers all architecture handlers. The binary entry point is `robustone/src/main.rs`. |
| `robustone-core` | **Core engine** — defines the `ArchitectureHandler` trait, the `ArchitectureDispatcher` runtime registry, the shared `DecodedInstruction` IR, rendering infrastructure, and common types/errors. This crate must remain free of architecture-specific code. |
| `robustone-cli` | **CLI library** — argument parsing (via `clap`), input validation, output formatting, and the `CliExecutor` that wires the dispatcher to user-facing output. |
| `robustone-riscv` | **RISC-V backend** (~8K lines) — instruction decoder, standard extensions (I, M, A, F, D, C), T-HEAD extensions, printer, and render. The only fully functional backend. |
| `robustone-arm`, `robustone-x86`, `robustone-loongarch` | **Skeleton crates** — minimal placeholder implementations of `ArchitectureHandler`. |

### Key Architectural Patterns

1. **Trait-based dispatch**: All backends implement `ArchitectureHandler` (defined in `robustone-core/src/traits/architecture.rs`). The `ArchitectureDispatcher` selects the handler at runtime by matching `handler.supports(arch_name)`.

2. **Shared IR**: Backends populate `DecodedInstruction` (in `robustone-core/src/ir.rs`), a structured representation with operands, registers, groups, and render hints. Text rendering is delegated back to the architecture crate via a function pointer (`RenderFn`) attached to the IR.

3. **Two-tier API**:
   - `decode_instruction(bytes, arch, addr)` → returns `(DecodedInstruction, bytes_consumed)`
   - `disassemble(bytes, arch, addr)` → returns `(Instruction, bytes_consumed)` (text-oriented, includes detail metadata)

4. **Architecture profiles**: `ArchitectureProfile` (in `robustone-core/src/common/`) allows explicit extension-set gating at decode time. For example, a profile with only `["I", "M"]` will reject instructions requiring the `A` or `F` extensions with `DisasmError::UnsupportedExtension`.

5. **Mode-sensitive decode**: The RISC-V backend has separate `RiscVDecoder` instances for RV32 and RV64. Some encodings decode to different mnemonics (or are rejected) depending on the mode — e.g., `0x6085` is `c.addiw` on RV64 but `c.jal` on RV32.

## Common Commands

All daily development commands are exposed through the **Makefile** (`make help` for the full list). A `justfile` also exists but is secondary.

### Build & Run

```bash
make build                          # Debug build of the CLI crate
make run -- riscv32 93001000 -d     # Run CLI (pass args after --)
make run RUN_ARGS="riscv32 93001000 -d"  # Alternative for flags that make might intercept

# Direct cargo equivalents
cargo build --manifest-path robustone/Cargo.toml
cargo run --manifest-path robustone/Cargo.toml -- riscv32 93001000 -d
```

### Python Environment

The project has **two Python virtual environments** that coexist:

- **`.venv/`** — created with `uv`, contains `pylint` (used for ad-hoc linting).
- **`virt-py/`** — created by `make test`/`make check` via `python3 -m venv`, contains all dependencies from `requirements.txt` (black, pylint, PyYAML, pre-commit, etc.). This is the environment used by Makefile targets.

```bash
# uv environment (ad-hoc use)
source .venv/bin/activate
uv pip install pylint

# Makefile-managed environment (used by make test / make check)
# Automatically created on first make test invocation; no manual action needed.
```

### Testing

```bash
make test                           # Full suite: Python unit tests + parity tests + Rust workspace tests
make test-quick                     # Quick parity test (20 cases per arch)
make test-parity                    # Parity tests only (no Rust workspace tests)
make test-list                      # List available test architectures
make test-validate                  # Validate test configuration files
make capstone-tests                 # Capstone YAML parity tests

cargo test --workspace --all-features                                    # Rust tests only
cargo test --workspace --all-features <filter>                          # Single test by name
cargo test -p robustone-core --lib test_profile_matrix_enforces_extension_boundaries  # Specific test
```

Capstone is cloned at `third_party/capstone/` and `cstool` is built at `third_party/capstone/cstool/cstool`. Test scripts locate it by full path, so it does not need to be on `PATH`. If missing, run `make ensure-capstone`.

### Lint & Format

```bash
make check                          # rustfmt + clippy (-D warnings) + pylint + black
cargo fmt --all
cargo clippy --workspace --all-features -- -D warnings
```

Pre-commit hooks enforce `rustfmt`, `clippy` (with `-D warnings`), `black`, `pylint`, and `cargo test` on push. Install with `make pre-commit-install`.

## Testing Framework

Parity tests compare Robustone CLI output against Capstone's `cstool` reference implementation:

- **Configs**: `test/architectures/<arch>/config.json` defines how to invoke both tools.
- **Cases**: `test/architectures/<arch>/test_cases.txt` contains hex bytes and optional expected output.
- **Runner**: `test/run_tests.py` is the main entrypoint. Supports `--arch`, `--all`, `--limit`, `--verbose`, `--show-details`, `--loose-match`.

Rust-side test assets live in `tests/`:
- `tests/golden/` — JSON golden fixtures for instruction decode verification
- `tests/property/` — property-based tests (proptest)
- `tests/differential/known-differences.toml` — ledger of intentionally accepted divergences from Capstone

## Code Style Rules (from CONTRIBUTING.md)

- All **public items require `///` documentation comments**.
- **Never use `unwrap()` in library code**; use `?` or proper error handling with `DisasmError`.
- Use `Result<T, DisasmError>` for fallible operations.
- Prefer `&str` over `String` for function parameters.
- Import order: `std`, external crates, `crate::`, `super::`.
- Python: PEP 8, max line length 120, type hints where practical. Configured in `pyproject.toml`.

## ISA / Compatibility Triple Gate

For any ISA addition, decode-behavior change, formatter change, or Capstone-compatibility change, contributors must satisfy **all three**:

1. **Parity coverage**: add/update cases in `test/architectures/<arch>/test_cases.txt`
2. **Rust coverage**: add/update unit/golden/property tests as appropriate
3. **Known-difference accounting**: if parity is intentionally incomplete, record it in `tests/differential/known-differences.toml` with `owner` and `expires_on`

Do not leave compatibility gaps undocumented. See `docs/isa-checklist.md` for the full new-ISA checklist.

## Important File References

| File | Purpose |
|------|---------|
| `robustone-core/src/traits/architecture.rs` | `ArchitectureHandler` trait definition |
| `robustone-core/src/ir.rs` | Shared `DecodedInstruction` IR and operand types |
| `robustone-core/src/lib.rs` | `ArchitectureDispatcher` implementation |
| `robustone/src/lib.rs` | Meta-crate `dispatcher()` that registers all handlers |
| `robustone-cli/src/lib.rs` | CLI library public API |
| `robustone-riscv/src/lib.rs` | `RiscVHandler` implementing `ArchitectureHandler` |
| `docs/support-matrix.md` | Public feature support matrix |
| `docs/isa-checklist.md` | Checklist for adding a new ISA backend |
| `tests/differential/known-differences.toml` | Accepted Capstone divergences |
| `pyproject.toml` | Python linting config (pylint, black) |
| `.pre-commit-config.yaml` | Pre-commit hook definitions |
