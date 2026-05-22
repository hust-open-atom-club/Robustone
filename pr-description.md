## Summary

- Add a clean first-stage AArch64 backend with canonical `aarch64` handler identity, IR/register/error metadata, and text rendering.
- Add the TableGen-first AArch64 generator pipeline plus deterministic generated Rust specs for common baseline instruction groups.
- Lock first-stage coverage with focused AArch64 smoke tests and document that full SVE/SME/FP8 and full upstream Capstone AArch64 MC parity remain follow-up milestones.

## Scope

This PR is an AArch64 basic-support baseline. It does not claim complete AArch64 ISA coverage, 32-bit ARM/AArch32 support, full SVE/SME/FP8 support, or full upstream Capstone AArch64 MC acceptance.

Capstone is used only as a validation reference. Generated specs are produced through the AArch64 generator pipeline from external instruction metadata and are kept boundary-clean in production sources.

## Test plan

- [ ] `cargo xtask verify-boundary`
- [ ] `cargo run -p xtask -- aarch64-gen-check --llvm-project third_party/llvm-project --out-dir robustone-arm/src/backend/generated --artifact-dir target/aarch64-gen`
- [ ] `cargo test -p robustone-arm -j64 --test aarch64_basic_smoke -- --nocapture`
- [ ] `cargo test -p robustone-arm -j64 --test aarch64_generated -- --nocapture`
- [ ] `cargo test --workspace --all-features -j64`
