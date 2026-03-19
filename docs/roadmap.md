# Robustone Roadmap

This roadmap translates the refactor plan into repository-facing milestones. It is intentionally scoped around a RISC-V-first implementation instead of trying to add many architectures at once.

## Compatibility Model

Robustone tracks Capstone compatibility in three layers:

- CLI compatibility: `cstool`-like command-line syntax, raw hex input, and familiar inspection flags.
- Semantic compatibility: matching decoded instruction meaning, operand formatting, register naming, and detail output on the instruction streams covered by the test harness.
- API compatibility: Rust APIs that can express the same semantics, even when they are not yet a drop-in replacement for Capstone's C handle/options/detail APIs.

## Current Baseline

As of 2026-03-19, the repository has:

- RISC-V decode backends for `riscv`, `riscv32`, and `riscv64`
- A top-level `Makefile` with build, check, run, and parity-test entrypoints
- A Python parity harness under `test/` with `riscv32` and `riscv64` suites
- GitHub Actions workflows that run formatting, linting, workspace tests, parity integration tests, and multi-platform builds

The project does not yet have:

- A Capstone-style handle/options/detail API
- Published golden/property/fuzz coverage beyond the current parity harness
- Shared architecture scaffolding for a second ISA
- Benchmark and release-engineering baselines

## Milestones

### v0.1: RISC-V Baseline You Can Trust

- Keep the repository contract accurate: README, contributor docs, support matrix, roadmap, and CI should all point at the same commands and paths.
- Introduce the first IR types in `robustone-core` and route RISC-V decode through them without breaking the current CLI.
- Keep `make test` and the parity harness green while this refactor lands.

### v0.2: Structured RISC-V Semantics

- Replace string-first decoder output with IR-first decode results.
- Publish explicit RISC-V support/error boundaries, including canonical-vs-alias formatting policy.
- Keep the machine-readable/JSON path aligned with the same decode result used by the human-readable formatter.

### v0.3: Correctness Matrix and API Split

- Add repository-native golden, differential, property, and fuzz layers.
- Separate low-level decode APIs from formatter profiles and compatibility facades.
- Keep known differences with Capstone documented instead of implicit.

### v0.4: Extensibility and Engineering Baselines

- Extract shared architecture scaffolding that a second ISA can reuse.
- Add benchmark baselines for decoder throughput, detail overhead, and CLI end-to-end cost.
- Publish release-engineering policy, versioning expectations, and changelog discipline.

## Immediate Priorities

The next concrete slices are:

1. Keep repository docs and entrypoints synchronized with executable behavior.
2. Land the IR-first core refactor behind compatibility shims.
3. Publish explicit RISC-V support/error behavior as code, not only as roadmap text.
4. Expand the current parity harness into the larger correctness matrix described above.

## Deferred Scope

The roadmap explicitly defers new high-complexity backends such as x86 until the RISC-V path has become the reference implementation for decoder structure, testing, and public API boundaries.
