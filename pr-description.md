## Summary

- What changed: Completed decode support for all remaining standard RISC-V instructions across the I/M/A/F/D/C extensions, fixed multiple long-standing immediate-extraction bugs in the compressed-instruction decoder, and added comprehensive test coverage.
- Why: The decoder previously returned `unimplemented_instruction` for RV64C integer loads/stores (`c.ld`, `c.sd`, `c.ldsp`, `c.sdsp`) and had no decode paths for any C.FP instructions (`c.flw`, `c.fsw`, `c.fld`, `c.fsd`, and their stack-pointer variants). Privileged system instructions (`mret`, `sret`, `uret`, `wfi`, `sfence.vma`) were misclassified as CSR operations, and `fcvt.s.d` was missing from the RVF decoder.

## Detailed Changes

### Phase 1 — Privileged & Conversion Instructions
- **RVI**: `decode_system()` now correctly decodes `mret`, `sret`, `uret`, `wfi`, and `sfence.vma` as system instructions instead of falling through to CSR logic.
- **Decoder**: `infer_groups()` tags the above mnemonics with the `"system"` group.
- **RVF**: Added `fcvt.s.d` decode path for `funct5=01000, fmt=00, rs2=00001`.

### Phase 2 — RV64C Integer Compressed Instructions
- Implemented `c.ld`, `c.sd`, `c.ldsp`, `c.sdsp` with correct CL/CS/CI/CSS operand formatting and Capstone aliases (`ld` / `sd`).
- Added correct immediate extractions in `decoder.rs`:
  - `uimm_cld` for `c.ld` / `c.sd`
  - `uimm_cldsp` for `c.ldsp`
  - `uimm_sdsp` for `c.sdsp`

### Phase 3 — C.FP (Compressed Floating-Point) Instructions
- Implemented all 8 C.FP instructions:
  - `c.fld`, `c.fsd`, `c.fldsp`, `c.fsdsp` (gated on **D** extension)
  - `c.flw`, `c.fsw`, `c.flwsp`, `c.fswsp` (gated on **F** extension, RV32 only)
- Extended `InstructionExtension::try_decode_compressed` signature with `extensions: &Extensions` so the C extension can check F/D availability at decode time.
- Updated all extension implementations (RVI, RVA, RVM, RVF, RVD, T-Head CMov) to match the new trait signature.

### Immediate Extraction Fixes
Fixed several bugs in `decoder.rs` that produced incorrect displacements for compressed load/store instructions:
- `uimm_cl` / `uimm_cs` (used by `c.lw` / `c.sw`)
- `uimm_css` (used by `c.swsp`)
- `uimm_fldsp` / `uimm_cldsp` (used by `c.fldsp` / `c.ldsp`)

### Phase 4 — Tests & Cleanup
- Added 10+ new unit tests in `decoder.rs` covering:
  - `fcvt.s.d`
  - System privileged instructions
  - RV64C (`c.ld`, `c.sd`, `c.ldsp`, `c.sdsp`)
  - C.FP on RV32 and RV64
  - Extension gating (C.FP rejected without F/D)
- Added E2E CLI test `test_riscv_new_compressed_instructions_decode`.
- Updated existing tests whose expectations were invalidated by the new decode paths:
  - `test_rv64_only_compressed_families_report_unsupported_mode_on_rv32`
  - `test_invalid_compressed_encoding_reports_failure`
  - `test_json_formatter_emits_data_pseudo_for_unsupported_compressed`
- Removed dead code: `decode_c_unimplemented()` and the `unimplemented_instruction` error helper (no longer called anywhere).

## Compatibility Checklist

- [ ] This change affects CLI compatibility promises.
- [x] This change affects shared IR semantics or formatter behavior.
  *New mnemonics (`c.ld`, `c.sd`, `c.flw`, `c.fsw`, `c.fld`, `c.fsd`, and stack variants) are now emitted instead of errors for previously-unhandled encodings.*
- [ ] This change affects public Rust API behavior.
- [ ] This change updates benchmark baselines, known differences, or support-matrix claims.

## Verification

- [x] `cargo test --workspace --all-features` — **222 tests pass, 0 failures**

## ISA / Compatibility Guardrails

- [ ] If this change adds or changes ISA/compatibility behavior, I added or updated parity coverage under `test/architectures/<arch>/test_cases.txt`.
- [x] If this change adds or changes ISA/compatibility behavior, I added or updated Rust tests/golden/property coverage in the affected crate.
- [ ] If parity is intentionally incomplete, I updated `tests/differential/known-differences.toml` with owner/expiry metadata instead of leaving the divergence undocumented.

## Notes

- Follow-up work or accepted limitations:
  - The existing `uimm_cl` / `uimm_cs` / `uimm_css` immediate extractions contained latent bugs that were fixed as part of this PR. Existing test cases happened to use encodings where the bugs did not manifest (e.g. `inst[6]=0` and `inst[11]=0`), so no golden fixtures needed updating.
  - Capstone parity coverage for the new instructions is not yet present in `test_cases.txt` because `cstool` is not available in the current environment. Parity data should be regenerated once the Capstone binary is built.
