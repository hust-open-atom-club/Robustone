## Summary

- **What changed:** Expanded `robustone-arm` from a 4-instruction skeleton to a Stage 1 AArch64 baseline integer decoder supporting 12 instructions: `nop`, `add` (imm/reg), `sub`, `movz`, `mov` (orr alias), `orr`, `eor`, `and`, `csel`, `b`, `bl`, `br`, `ret`. The decoder was refactored into a hierarchical family-module architecture (`data_proc_imm`, `data_proc_reg`, `branches`, `system`) with dedicated `encoding.rs` field extractors. Added IR metadata computation (`registers_read`, `registers_written`, `implicit_registers`, `groups`), `ArmInstructionDetail` implementing the `Detail` trait, and Capstone-compatible text rendering (condition codes, branch targets, register 31 aliases for SP/XZR).

- **Why:** The previous AArch64 backend only handled 4 hardcoded instructions (`nop`, `add imm`, `movz`, `ret`). The Stage 1 parity test suite (`test/architectures/aarch64/test_cases.txt`) had 8 failing cases due to `InvalidEncoding`. This PR brings the Stage 1 suite to 100% pass rate before expanding to memory, SIMD/FP, and the full Capstone YAML corpus.

## Compatibility Checklist

- [ ] This change affects CLI compatibility promises.
- [x] This change affects shared IR semantics or formatter behavior.
  - New render logic for AArch64 immediates (branch targets without `#`, unsigned hex for bitmask immediates with bit 63 set).
  - Register 31 semantics are now context-dependent (SP for ADD/SUB immediate, XZR for logical/conditional/branch).
- [ ] This change affects public Rust API behavior.
- [ ] This change updates benchmark baselines, known differences, or support-matrix claims.

## Verification

- [x] `make check`
- [x] `cargo test --workspace --all-features`
- [x] `make test` (Python parity tests pass)

All three pass cleanly:
```bash
cargo test --workspace --all-features   # 225 tests pass
cargo clippy --workspace --all-features -- -D warnings   # clean
python3 test/run_tests.py --arch aarch64 --limit 12   # 12/12 (100%)
```

## ISA / Compatibility Guardrails

- [x] If this change adds or changes ISA/compatibility behavior, I added or updated parity coverage under `test/architectures/<arch>/test_cases.txt`.
  - Added `test/architectures/aarch64/test_cases.txt` with 12 baseline integer cases verified against `cstool`.
- [x] If this change adds or changes ISA/compatibility behavior, I added or updated Rust tests/golden/property coverage in the affected crate.
  - `robustone-arm` existing unit tests (`test_add_imm_decode`, `test_movz_decode`, `test_nop_decode`, `test_ret_decode`) continue to pass. Golden/property tests for AArch64 are deferred until the instruction set stabilizes beyond Stage 1.
- [ ] If parity is intentionally incomplete, I updated `tests/differential/known-differences.toml` with owner/expiry metadata instead of leaving the divergence undocumented.
  - Not applicable; no divergences are accepted at Stage 1.

## Notes

- **Follow-up work or accepted limitations:**
  - 32-bit ARM / Thumb is out of scope.
  - Big-endian (`aarch64be`) is not yet supported.
  - Memory instructions (LDR/STR), SIMD/FP, and system registers are deferred to later stages.
  - The full Capstone YAML test suite (~34,770 cases) is not targeted in Stage 1.
  - `ArchitectureProfile` / extension gating is not yet wired for AArch64.
