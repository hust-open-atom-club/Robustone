# Robustone Support Matrix

This matrix documents what the repository supports today and what it intentionally does not claim yet. It was verified against the current workspace on 2026-03-19.

## Capstone Compatibility Layers

| Layer | Current status | Notes |
|-------|----------------|-------|
| CLI compatibility | Partial | `robustone` accepts `arch+mode` style input, raw hex bytes, addresses, and detail-oriented output flags for the implemented backends. |
| Semantic compatibility | Partial | The parity harness currently verifies curated `riscv32` and `riscv64` instruction suites against Capstone. |
| API compatibility | Limited | The public Rust API exposes `ArchitectureDispatcher` and `Instruction`; it is not yet a Capstone handle/options/detail clone. |

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
| Detailed text output (`-d`) | Implemented | Verified locally with `make run RUN_ARGS="riscv32 93001000 -d"`. |
| Real detail output (`-r`) | Implemented in current CLI formatter | Existing tests cover detail display, but the project does not yet publish a stable structured detail schema. |
| Register read/write detail | Implemented | The RISC-V backend populates read/write register detail today. |
| Groups / implicit register sets as structured public data | Not implemented | These are not yet exposed as first-class IR data. |
| Canonical-vs-alias formatter profiles | Not implemented | The current backend prints Capstone-style aliases by default. |
| Structured JSON output | Not implemented | Text output is the current public output path. |
| Structured decode-error taxonomy | Not implemented | Decode fallback is still string-first and relies on `unknown`-style behavior in several paths. |

## Repository Entry Points

| Command | Status | Notes |
|---------|--------|-------|
| `make build` | Verified | Builds the top-level crate in debug mode. |
| `make run RUN_ARGS="riscv32 93001000 -d"` | Verified | Produces a RISC-V disassembly with detail output. |
| `make test` | Verified | Builds Capstone if needed, runs parity tests, then runs top-level crate tests. |
| `python3 test/run_tests.py --list` | Verified | Lists the currently configured parity suites. |
| `cargo test --workspace --all-features` | Verified | Runs workspace Rust tests and doctests. |

## Known Gaps and Non-Goals

- Parser coverage is broader than decode-backend coverage; the CLI help surface should not be read as proof that every architecture listed there can be decoded today.
- The current public API is string-first. A shared decoder IR and structured output remain roadmap items.
- The repository does not yet publish golden/property/fuzz results as first-class support claims.
- `--alias-regs` and `--unsigned-immediate` remain reserved CLI options rather than implemented output modes.
