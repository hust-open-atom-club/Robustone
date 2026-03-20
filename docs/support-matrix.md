# Robustone Support Matrix

This matrix documents what the repository supports today and what it intentionally does not claim yet. It was verified against the current workspace on 2026-03-20.

## Capstone Compatibility Layers

| Layer | Current status | Notes |
|-------|----------------|-------|
| CLI compatibility | Partial | `robustone` accepts `arch+mode` style input, raw hex bytes, addresses, and detail-oriented output flags for the implemented backends. |
| Semantic compatibility | Partial | The parity harness currently verifies curated `riscv32` and `riscv64` instruction suites against Capstone. |
| API compatibility | Partial | The public Rust API now exposes both the legacy `Instruction` wrapper and the lower-level `ArchitectureDispatcher::decode_instruction` IR entrypoint, but it is not yet a Capstone handle/options/detail clone. |

## Decode Backends

| Surface | Status | Notes |
|---------|--------|-------|
| `riscv`, `riscv32`, `riscv64` decode | Implemented | Backed by `robustone-core` and exercised by the parity harness. |
| `riscv32e` token parsing | Parsed in CLI, not implemented as a decode backend | The CLI can parse the token, but `ArchitectureDispatcher` does not expose a dedicated handler for it. |
| ARM/x86/MIPS/PowerPC/SPARC parser tokens | Parser-only placeholders | These names appear in the CLI parser/help surface, but there is no corresponding decode backend registered in `ArchitectureDispatcher` today. |

## RISC-V Feature Status

| Area | Status | Notes |
|------|--------|-------|
| Base integer decoding (`riscv32` / `riscv64`) | Implemented and parity-tested | Verified locally with `make test` and `python3 test/run_tests.py --all --limit 20 --verbose`. |
| Standard extension modules (`I`, `M`, `A`, `F`, `D`, `C`) | Present in code | Decoder modules exist; broader published instruction-coverage percentages are not available yet. |
| CSR instructions | Implemented for the covered cases | Core CSR decode and pseudo forms (`csrr`, `csrc`, `csrw`) are present on the current RISC-V path. |
| Privileged instructions | Partial | `ecall` / `ebreak` are implemented, but the repository does not yet publish a fuller privileged-instruction coverage claim. |
| Fence instructions | Implemented for the covered cases | `fence` and `fence.i` decode on the current RISC-V path. |
| Detailed text output (`-d`) | Implemented | Verified locally with `make run RUN_ARGS="riscv32 93001000 -d"`. |
| Real detail output (`-r`) | Implemented in current CLI formatter | Existing tests cover detail display, but the project does not yet publish a stable structured detail schema. |
| Canonical shared IR | Implemented for the RISC-V decode path | `ArchitectureDispatcher::decode_instruction` returns canonical mnemonics plus rendering hints for Capstone-style aliases. |
| Register read/write detail | Implemented | The RISC-V backend populates read/write register detail today. |
| Groups / implicit register sets as structured public data | Partial | The shared IR now carries group and implicit-register fields; the RISC-V path populates them for the implemented instruction families, but the model is still evolving. |
| Canonical-vs-alias formatter profiles | Partial | The RISC-V printer now has Capstone-style and canonical profiles, but only the Capstone-style path is exposed through the CLI by default. |
| Structured JSON output | Implemented | `robustone --json ...` renders structured JSON built from the shared decode IR. |
| Structured decode-error taxonomy | Implemented for the current RISC-V path | The low-level decode API now emits `need_more_bytes`, `invalid_encoding`, `unsupported_extension`, `unsupported_mode`, and `unimplemented_instruction`, including RV64-only standard / atomic / compressed encodings recognized under `riscv32`. |
| Golden/property/fuzz scaffolding | Implemented as repository structure | `tests/golden/`, `tests/property/`, `tests/differential/`, `robustone-core/tests/*.rs`, and `fuzz/` are now present, though coverage is still early. |

## Repository Entry Points

| Command | Status | Notes |
|---------|--------|-------|
| `make build` | Verified | Builds the top-level crate in debug mode. |
| `make check` | Verified | Runs `rustfmt`, `clippy`, `black`, and `pylint` against workspace code and repository-owned Python test scripts. |
| `make run RUN_ARGS="riscv32 93001000 -d"` | Verified | Produces a RISC-V disassembly with detail output. |
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

- Parser coverage is broader than decode-backend coverage; the CLI help surface should not be read as proof that every architecture listed there can be decoded today.
- The repository now exposes a shared decode IR, but the Capstone-style CLI formatter still keeps some compatibility-oriented display behavior for parity purposes.
- The repository now contains golden/property/fuzz scaffolding, but the coverage remains RISC-V-first and does not yet amount to a complete cross-architecture claim.
- Accepted or investigated output differences belong in `tests/differential/known-differences.toml`; that file is the canonical repository record for parity exceptions.
- `--alias-regs` is currently a compatibility-accepted no-op for the RISC-V CLI because Capstone-style alias names are already the default outward register view.
- `--unsigned-immediate` is implemented for the current RISC-V formatter path and renders negative immediates using an unsigned hexadecimal view.
