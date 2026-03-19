# Release Checklist

1. Run `cargo test --workspace --all-features`.
2. Run `make test`.
3. Run `cargo bench -p robustone-core --bench riscv_decode` and record the result.
4. Review `tests/differential/known-differences.toml`.
5. Review fuzz smoke results for `fuzz/fuzz_targets/*`.
6. Update `CHANGELOG.md`.
7. Update `docs/support-matrix.md` if support or known gaps changed.
