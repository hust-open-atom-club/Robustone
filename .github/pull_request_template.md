## Summary

- What changed:
- Why:

## Compatibility Checklist

- [ ] This change affects CLI compatibility promises.
- [ ] This change affects shared IR semantics or formatter behavior.
- [ ] This change affects public Rust API behavior.
- [ ] This change updates benchmark baselines, known differences, or support-matrix claims.

## Verification

- [ ] `make check`
- [ ] `cargo test --workspace --all-features`
- [ ] `make test`

## ISA / Compatibility Guardrails

- [ ] If this change adds or changes ISA/compatibility behavior, I added or updated parity coverage under `test/architectures/<arch>/test_cases.txt`.
- [ ] If this change adds or changes ISA/compatibility behavior, I added or updated Rust tests/golden/property coverage in the affected crate.
- [ ] If parity is intentionally incomplete, I updated `tests/differential/known-differences.toml` with owner/expiry metadata instead of leaving the divergence undocumented.

## Notes

- Follow-up work or accepted limitations:
