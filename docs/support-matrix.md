# Robustone Support Matrix

This matrix documents what the repository supports today and what it intentionally does not claim yet. It was synchronized with the shared architecture capability registry on 2026-04-08.

## Capstone Compatibility Layers

| Layer | Current status | Notes |
|-------|----------------|-------|
| CLI compatibility | Partial | `robustone` accepts `arch+mode` style input, raw hex bytes, addresses, and detail-oriented output flags for the implemented backends. Parser-only architectures are accepted by the CLI only as capability placeholders, not as decode promises. |
| Semantic compatibility | Partial | The parity harness currently verifies curated `riscv32` and `riscv64` instruction suites against Capstone. |
| API compatibility | Partial | The public Rust API exposes both the legacy `Instruction` wrapper and the lower-level `ArchitectureDispatcher::decode_instruction` IR entrypoint, but it is not yet a Capstone handle/options/detail clone. |

## Architecture Capability Registry

Canonical tokens below are the normalized architecture names emitted by the shared capability registry. A `no` in `Decode`, `Detail`, or `JSON` means the token is parser-only today.

| Canonical token | Category | Parse | Decode | Detail | JSON |
|-----------------|----------|-------|--------|--------|------|
| `riscv32` | `RISC-V` | yes | yes | yes | yes |
| `riscv64` | `RISC-V` | yes | yes | yes | yes |
| `riscv32e` | `RISC-V` | yes | no | no | no |
| `arm` | `ARM` | yes | no | no | no |
| `armle` | `ARM` | yes | no | no | no |
| `armbe` | `ARM` | yes | no | no | no |
| `thumb` | `ARM` | yes | no | no | no |
| `aarch64` | `ARM` | yes | no | no | no |
| `aarch64be` | `ARM` | yes | no | no | no |
| `x16` | `x86` | yes | no | no | no |
| `x32` | `x86` | yes | no | no | no |
| `x64` | `x86` | yes | no | no | no |
| `mips` | `MIPS` | yes | no | no | no |
| `mipsel` | `MIPS` | yes | no | no | no |
| `mips64` | `MIPS` | yes | no | no | no |
| `mips64el` | `MIPS` | yes | no | no | no |
| `powerpc32` | `PowerPC` | yes | no | no | no |
| `powerpc32be` | `PowerPC` | yes | no | no | no |
| `powerpc64` | `PowerPC` | yes | no | no | no |
| `powerpc64be` | `PowerPC` | yes | no | no | no |
| `sparc` | `SPARC` | yes | no | no | no |
| `sparcle` | `SPARC` | yes | no | no | no |
| `sparc64` | `SPARC` | yes | no | no | no |
| `systemz` | `Other` | yes | no | no | no |
| `xcore` | `Other` | yes | no | no | no |
| `m68k` | `Other` | yes | no | no | no |
| `tms320c64x` | `Other` | yes | no | no | no |
| `m680x` | `Other` | yes | no | no | no |
| `evm` | `Other` | yes | no | no | no |
| `bpf` | `Other` | yes | no | no | no |

## Decode Backends

| Surface | Status | Notes |
|---------|--------|-------|
| `riscv32`, `riscv64` decode | Implemented | Backed by `robustone-core` plus `robustone-riscv` and exercised by the parity harness. |
| `riscv32e` | Parser-only | Accepted as a canonical token so capability and help surfaces can be honest about the missing backend. |
| All non-RISC-V canonical tokens above | Parser-only placeholders | Accepted for CLI parsing and version/help reporting only; no decode backend is registered in `ArchitectureDispatcher` for them today. |

## RISC-V Feature Status

| Area | Status | Notes |
|------|--------|-------|
| Base integer decoding (`riscv32` / `riscv64`) | Implemented and parity-tested | Verified locally with `make test` and `python3 test/run_tests.py --all --limit 20 --verbose`. |
| Standard extension modules (`I`, `M`, `A`, `F`, `D`, `C`) | Present in code | Decoder modules exist; broader published instruction-coverage percentages are not available yet. |
| CSR instructions | Implemented for the covered cases | Core CSR decode and pseudo forms (`csrr`, `csrc`, `csrw`) are present on the current RISC-V path. |
| Privileged instructions | Partial | `ecall` and `ebreak` are implemented, but the repository does not yet publish a fuller privileged-instruction coverage claim. |
| Fence instructions | Implemented for the covered cases | `fence` and `fence.i` decode on the current RISC-V path. |
| Detailed text output (`-d`) | Implemented | Verified locally with `make run RUN_ARGS="riscv32 93001000 -d"`. |
| Real detail output (`-r`) | Implemented in current CLI formatter | Existing tests cover detail display, but the project does not yet publish a stable structured detail schema. |
| Canonical shared IR | Implemented for the RISC-V decode path | `ArchitectureDispatcher::decode_instruction` returns canonical mnemonics plus rendering hints for Capstone-style aliases. |
| Register read/write detail | Implemented | The RISC-V backend populates read/write register detail today. |
| Groups / implicit register sets as structured public data | Partial | The shared IR carries group and implicit-register fields; the RISC-V path populates them for the implemented instruction families, but the model is still evolving. |
| Canonical-vs-alias formatter profiles | Partial | The RISC-V printer supports Capstone-style and canonical profiles, but the Capstone-style path remains the default CLI surface. |
| Structured JSON output | Implemented | `robustone --json ...` renders structured JSON built from the shared decode IR. |
| Structured decode-error taxonomy | Implemented for the current RISC-V path | The low-level decode API emits `need_more_bytes`, `invalid_encoding`, `unsupported_extension`, `unsupported_mode`, and `unimplemented_instruction`, including mode-sensitive RV64-only encodings recognized under `riscv32`. |
| Golden/property/fuzz scaffolding | Implemented as repository structure | `tests/golden/`, `tests/property/`, `tests/differential/`, `robustone-core/tests/*.rs`, and `fuzz/` are present, though coverage remains RISC-V-first. |

## Repository Entry Points

| Command | Status | Notes |
|---------|--------|-------|
| `make build` | Verified | Builds the top-level crate in debug mode. |
| `make check` | Verified | Runs `rustfmt`, `clippy`, `black`, and `pylint` against workspace code and repository-owned Python test scripts. |
| `make run RUN_ARGS="riscv32 93001000 -d"` | Verified | Produces a RISC-V disassembly with detail output. |
| `cargo run --manifest-path robustone/Cargo.toml -- --capabilities` | Verified | Prints the registry-derived architecture capability report without requiring disassembly input. |
| `cargo run --manifest-path robustone/Cargo.toml -- --json --capabilities` | Verified | Prints the same capability report as structured JSON for automation. |
| `cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000` | Verified | Produces structured JSON backed by the shared decode IR. |
| `make test` | Verified | Builds Capstone if needed, runs parity tests, then runs `cargo test --workspace --all-features`. |
| `python3 test/run_tests.py --list` | Verified | Lists the currently configured parity suites. |
| `make test-quick` | Verified | Runs a reduced parity slice for fast local feedback. |
| `cargo test --workspace --all-features` | Verified | Runs workspace Rust tests and doctests. |
| `cargo bench -p robustone-core --bench riscv_decode` | Verified | Records pure decode throughput and detail-overhead baselines in `docs/benchmark-baselines.md`; it is not part of the default repository validation set. |
| `cargo bench -p robustone-cli --bench cli_end_to_end` | Verified | Records the real CLI end-to-end baseline through `robustone-cli` argument parsing, config building, executor wiring, and formatter rendering. |
| `cd fuzz && cargo fuzz run decode_riscv -- -max_total_time=5` | Documented | Used for scheduled fuzz smoke validation; not required for the default local workflow. |
| `cd fuzz && cargo fuzz run format_riscv_json -- -max_total_time=5` | Documented | Used for scheduled fuzz smoke validation; not required for the default local workflow. |
| `cd fuzz && cargo fuzz run hex_parser -- -max_total_time=5` | Documented | Used for scheduled fuzz smoke validation of the shared hex parser surface. |
| `cd fuzz && cargo fuzz run format_riscv_text -- -max_total_time=5` | Documented | Used for scheduled fuzz smoke validation of the shared text-rendering surface. |
| `cd fuzz && cargo fuzz run cli_argument_combinations -- -max_total_time=5` | Documented | Used for scheduled fuzz smoke validation of CLI parser/config/executor combinations. |

## Known Gaps and Non-Goals

- Parser coverage is intentionally broader than decode-backend coverage; the capability registry and this document should be read as proof of what can be parsed and what can actually be decoded, not just one or the other.
- Parser-only architecture failures should point users to `robustone --capabilities` / `--support-matrix` and the registry-derived help appendix instead of pretending the backend exists.
- The repository exposes a shared decode IR, but the Capstone-style CLI formatter still keeps some compatibility-oriented display behavior for parity purposes.
- The repository contains golden/property/fuzz scaffolding, but the coverage remains RISC-V-first and does not yet amount to a complete cross-architecture claim.
- Accepted or investigated output differences belong in `tests/differential/known-differences.toml`; that file is the canonical repository record for parity exceptions.
- `--alias-regs` is currently a compatibility-accepted no-op for the RISC-V CLI because Capstone-style alias names are already the default outward register view.
- `--unsigned-immediate` is implemented for the current RISC-V formatter path and renders negative immediates using an unsigned hexadecimal view.
