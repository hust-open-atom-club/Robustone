# Robustone

Robustone is an experimental disassembly engine written in Rust by the HUST Open Atom Club. Inspired by the Capstone project, it explores how Rust's strong safety guarantees can be used to deliver a Capstone-compatible experience with a cleaner codebase and modern tooling.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or newer (edition 2021).
- [Python](https://www.python.org/) 3.8 or newer for parity tests.
- `git` and basic build tools for fetching the Capstone reference implementation.

## Project layout

```
robustone/
	src/
		cli/         # Command-line parsing, input validation, and presentation logic
	Cargo.toml     # Primary crate manifest
test/
	riscv32/       # Python scripts and fixtures for RISC-V parity checks
transfer/    # Architecture-specific decoding and formatting (Rust port of Capstone)
third_party/
	capstone/      # Optional checkout of the original Capstone project (used by tests)
```

## Getting started

Clone the repository (including the submodules, if any) and install the toolchain requirements above. The bundled `Makefile` offers shortcuts for common workflows:

| Target        | Description |
| ------------- | ----------- |
| `make build`  | Compile the crate in debug mode. |
| `make check`  | Run `cargo check` for fast type verification. |
| `make format` | Format the Rust codebase with `rustfmt`. |
| `make run`    | Launch the CLI in debug mode (accepts the same arguments as `cargo run`). |
| `make test`   | Build Capstone (if missing), run parity tests, and execute the Rust unit tests. |

The `test` target downloads Capstone into `third_party/capstone` on first use, builds the comparison tool, runs `test/riscv32/test_vs_cstool.py`, and finally executes `cargo test`.

## Running the CLI

The CLI mirrors the classic `cstool` UX. For example, to decode a RISC-V instruction with detailed output:

```bash
make run -- riscv32 13000513 -d
```

The command also accepts arguments without the explicit `--` separator:

```bash
make run riscv32 13000513 -d
```

Internally the target forwards any trailing words to the binary (or you can pass them via `RUN_ARGS="..."`).

## Testing

Run the full regression suite from the repository root:

```bash
make test
```

This command:

1. Ensures Capstone is available under `third_party/capstone` (clones the repository if necessary).
2. Builds Capstone's `cstool` helper using `test/build_cstool.sh`.
3. Executes the Python parity harness `test/riscv32/test_vs_cstool.py` and compares Robustone output with Capstone across the curated instruction list.
4. Runs `cargo test` for Rust unit coverage.

If you only need to validate the Python comparison script, run it directly:

```bash
python3 test/riscv32/test_vs_cstool.py
```
