# Versioning Policy

Robustone is still pre-1.0, but the repository is converging on these rules:

1. Document breaking API changes in `CHANGELOG.md`.
2. Keep the compatibility boundary explicit:
   - CLI compatibility
   - semantic compatibility
   - API compatibility
3. Treat the shared decode IR and JSON output as evolving interfaces until the project declares a stable `0.x` contract for them.
4. Once the low-level API and formatter profile boundaries stabilize, follow semantic versioning for public crates.

## Version `0.0.0` Scope

The repository currently treats `0.0.0` as a **RISC-V-mainline experimental release**, not as a general Capstone replacement across multiple ISAs.

That means `0.0.0` should only claim:

- implemented `riscv32` / `riscv64` decode backends,
- the current shared-IR / JSON / detail surfaces for the RISC-V path,
- compatibility claims backed by the parity harness, golden tests, and documented known differences.

It should **not** claim:

- a second decode backend,
- full Capstone API replacement,
- undocumented parser-only architectures as release-ready support.

Before moving past `0.0.0`, the project should keep narrowing gaps inside the RISC-V-first scope: parity coverage, known-difference discipline, release-checklist evidence, and the remaining mainline extension/profile compatibility work.
