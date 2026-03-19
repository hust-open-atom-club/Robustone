# Robustone

Robustone is an experimental disassembly engine written in Rust by the HUST Open Atom Club. Inspired by the Capstone project, it explores how Rust's strong safety guarantees can be used to deliver a Capstone-compatible experience with a cleaner codebase and modern tooling.

## Compatibility Boundary

Robustone tracks Capstone compatibility in three separate layers:

- CLI compatibility: keep the command-line UX close to `cstool`, including `arch+mode` syntax, raw hex input, and detail-oriented output flags.
- Semantic compatibility: keep decoded mnemonics, operand formatting, register naming, and detail output aligned on the instruction streams covered by the parity harness.
- API compatibility: expose equivalent Rust semantics where practical, while explicitly documenting where the current Rust API is not a Capstone handle/options/detail clone.

Current repository status:

- Implemented decode backends: `riscv`, `riscv32`, and `riscv64`
- Public support matrix: [docs/support-matrix.md](docs/support-matrix.md)
- Versioned roadmap: [docs/roadmap.md](docs/roadmap.md)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or newer (edition 2021).
- [Python](https://www.python.org/) 3.8 or newer for parity tests.
- `git` and basic build tools for fetching the Capstone reference implementation.

## Project layout

```
robustone/         # Metadata crate including both library and binary
robustone-core/    # Architecture-specific decoding and formatting (Rust port of Capstone)
robustone-cli/     # Command-line parsing, input validation, and presentation logic
docs/              # Roadmap, support matrix, and project documentation
Makefile           # Repository entrypoints for build, check, run, and test
test/
	architectures/ # Parity-test configs and curated instruction corpora
	run_tests.py   # Main parity-test entrypoint
third_party/
	capstone/      # Optional checkout of the original Capstone project (used by tests)
Cargo.toml     	   # Workspace manifest
```

## Getting started

Clone the repository (including the submodules, if any) and install the toolchain requirements above. The bundled `Makefile` offers shortcuts for common workflows:

| Target        | Description |
| ------------- | ----------- |
| `make build`  | Compile the crate in debug mode. |
| `make check`  | Run repository checks (`rustfmt`, `clippy`, `black`, and `pylint`). |
| `make format` | Format the Rust codebase with `rustfmt`. |
| `make run`    | Launch the CLI in debug mode (accepts the same arguments as `cargo run`). |
| `make test`   | Build Capstone (if missing), run parity tests, and execute the Rust unit tests. |
| `make test-quick` | Run a smaller parity-test slice for faster iteration. |
| `make help`   | Print the repository command summary. |

The `test` target downloads Capstone into `third_party/capstone` on first use, builds the comparison tool with `test/scripts/build_cstool.sh`, runs `python3 test/run_tests.py --all`, and finally executes `cargo test --manifest-path robustone/Cargo.toml`.

## Running the CLI

The CLI mirrors the classic `cstool` UX for the RISC-V backends that are implemented today. For example, to decode a RISC-V instruction with detailed output:

```bash
make run -- riscv32 130101ff -d
```

Alternatively, use the `RUN_ARGS` variable. This prevents `make` from misinterpreting flags like `-d`:

```bash
make run RUN_ARGS="riscv32 130101ff -d"
```

To inspect the currently advertised CLI surface:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --help
```

To emit structured JSON from the shared decode IR:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

## Testing

Run the full regression suite from the repository root:

```bash
make test
```

This command:

1. Ensures Capstone is available under `third_party/capstone` (clones the repository if necessary).
2. Builds Capstone's `cstool` helper using `test/scripts/build_cstool.sh`.
3. Executes the Python parity harness `python3 test/run_tests.py --all`.
4. Runs `cargo test --manifest-path robustone/Cargo.toml` for the top-level crate tests.

Additional useful verification commands:

```bash
python3 test/run_tests.py --list
python3 test/run_tests.py --arch riscv32 --limit 20 --verbose
cargo test --workspace --all-features
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

The commands above were verified locally on 2026-03-19.

## CI and Project Docs

- CI workflow: `.github/workflows/ci.yml`
- Support matrix: [docs/support-matrix.md](docs/support-matrix.md)
- Roadmap: [docs/roadmap.md](docs/roadmap.md)
- New ISA checklist: [docs/isa-checklist.md](docs/isa-checklist.md)
- Benchmark baselines: [docs/benchmark-baselines.md](docs/benchmark-baselines.md)
- Release checklist: [docs/release-checklist.md](docs/release-checklist.md)
- Versioning policy: [docs/versioning-policy.md](docs/versioning-policy.md)
- Test framework guide: [test/README.md](test/README.md)

## Contributing

We welcome contributions. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Setting up the development environment
- Installing and using pre-commit hooks
- Code style requirements
- Testing procedures
- Submitting pull requests
