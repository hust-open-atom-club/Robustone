# Robustone

Robustone is an experimental disassembly engine written in Rust by the HUST Open Atom Club. Inspired by the Capstone project, it explores how Rust's strong safety guarantees can be used to deliver a Capstone-compatible experience with a cleaner codebase and modern tooling.

## Compatibility Boundary

Robustone tracks Capstone compatibility in three separate layers:

- CLI compatibility: keep the command-line UX close to `cstool`, including `arch+mode` syntax, raw hex input, and detail-oriented output flags.
- Semantic compatibility: keep decoded mnemonics, operand formatting, register naming, and detail output aligned on the instruction streams covered by the parity harness.
- API compatibility: expose equivalent Rust semantics where practical, while explicitly documenting where the current Rust API is not a Capstone handle/options/detail clone.

Current repository status:

- Public support matrix: [docs/support-matrix.md](docs/support-matrix.md)
- Backend stability levels:
  - **Beta**: `riscv32`, `riscv64` — parity-tested, fuzz-covered, suitable for controlled internal use.
  - **ExperimentalDecode**: `aarch64`, `x32`, `x64`, `loongarch64` — decode backends exist but `x32`, `x64` coverage is limited; `loongarch64` relies on corpus exact-word matching and does not yet generalize to arbitrary valid encodings. The `aarch64` backend is spec-driven, covering base integer, branch, load/store, scalar FP, vector (SIMD/SVE/SME), crypto, and system instructions; detail output is not yet implemented.
  - **ParserOnly**: all remaining tokens — accepted by the CLI for capability discovery, but no decode backend is registered.
- This is a multi-ISA framework in active development, not yet a drop-in Capstone replacement.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.85 or newer (edition 2024 support).
- [Python](https://www.python.org/) 3.8 or newer for parity tests.
- `git` and basic build tools for fetching the Capstone reference implementation.

## Project layout

```
robustone/         # Metadata crate including both library and binary
robustone-core/    # Architecture-specific decoding and formatting (Rust port of Capstone)
robustone-cli/     # Command-line parsing, input validation, and presentation logic
robustone-arm/     # AArch64 spec-driven decode backend
docs/              # Support matrix and project documentation
xtask/             # TableGen-based code generator pipeline for architecture specs
tests/             # Golden/property/differential test assets
fuzz/              # Fuzz targets for decoder and JSON formatting smoke runs
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
| `make check`  | Run repository checks on workspace code and test-harness Python (`rustfmt`, `clippy`, `black`, and `pylint`). |
| `make format` | Format the Rust workspace and the Python parity harness. |
| `make run`    | Launch the CLI in debug mode (accepts the same arguments as `cargo run`). |
| `make test`   | Build Capstone (if missing), run parity tests, and execute Rust workspace tests. |
| `make test-quick` | Run a smaller parity-test slice for faster iteration. |
| `make help`   | Print the repository command summary. |

The `test` target downloads Capstone into `third_party/capstone` on first use, builds the comparison tool with `test/scripts/build_cstool.sh`, runs `python3 test/run_tests.py --all`, and finally executes `cargo test --workspace --all-features`.

## Running the CLI

The CLI mirrors the classic `cstool` UX for the RISC-V backends that are implemented today. For example, to decode a RISC-V instruction with detailed output:

```bash
make run -- riscv32 93001000 -d
```

Hex input follows `cstool` conventions and is interpreted as raw instruction bytes in memory order.

Alternatively, use the `RUN_ARGS` variable. This prevents `make` from misinterpreting flags like `-d`:

```bash
make run RUN_ARGS="riscv32 93001000 -d"
```

To inspect the currently advertised CLI surface:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --help
```

To inspect the registry-derived capability surface directly:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --capabilities
cargo run --manifest-path robustone/Cargo.toml -- --json --capabilities
```

To emit structured JSON from the shared decode IR:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

To decode an AArch64 instruction via the spec-driven backend:

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json aarch64 d503201f
```

## Docker image

Build and run the production CLI image locally:

```bash
docker build -t robustone:local .
docker run --rm robustone:local --help
docker run --rm robustone:local riscv32 93001000
```

The multi-stage `Dockerfile` builds the `robustone` binary with Rust 1.99.0 and
the checked-in `Cargo.lock`, then runs it as a non-root user on Debian Bookworm.
The separate `Dockerfile.dev` is not used by the production-image workflow.

### Manually build and publish to GHCR

The [Docker Image workflow](.github/workflows/docker.yml) runs only through
`workflow_dispatch`; pushes and pull requests do not trigger it.

1. Merge the workflow into **your repository's default branch** first. GitHub
   requires it there before the **Run workflow** button is available; adding it
   only to a feature branch does not expose that button.
2. Enable GitHub Actions in your fork if GitHub prompts you, then open
   **Actions → Docker Image → Run workflow**.
3. Select the branch to build and run the workflow. It checks out the selected
   revision, builds a `linux/amd64` image, tests the CLI, and only then pushes the
   tested image to GHCR.

The image name is `ghcr.io/<owner>/<repository>` in lowercase (for example,
`ghcr.io/paulkm2006/robustone` in that fork). Every run publishes
`sha-<full-commit-SHA>` for the checked-out revision. Only a run targeting the
default branch also updates `latest`; feature-branch and tag runs leave it alone.
The run summary lists the published image tags.

The workflow uses the built-in `GITHUB_TOKEN` with `contents: read` and
`packages: write`; no additional registry secret is required. If a package with
the same name already exists, grant this repository Actions write access to that
package. New GHCR packages are private by default, so pulling them requires
appropriate registry access unless you make the package public.

```bash
# Replace the owner and SHA with the values from the workflow run summary.
docker pull ghcr.io/<owner>/robustone:sha-<full-commit-SHA>
docker run --rm ghcr.io/<owner>/robustone:sha-<full-commit-SHA> riscv32 93001000
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
4. Runs `cargo test --workspace --all-features` for the Rust workspace tests.

Additional useful verification commands:

```bash
python3 test/run_tests.py --list
python3 test/run_tests.py --arch riscv32 --limit 20 --verbose
cargo test --workspace --all-features
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

The commands above were verified locally on 2026-05-29.

## CI and Project Docs

- CI workflow: `.github/workflows/ci.yml` (`make check`, `cargo test --workspace --all-features`, `make test`, plus scheduled fuzz smoke)
- Support matrix: [docs/support-matrix.md](docs/support-matrix.md)
- Known differences: [tests/differential/known-differences.toml](tests/differential/known-differences.toml)
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
