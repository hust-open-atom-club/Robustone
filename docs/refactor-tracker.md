# Refactor Tracker

This tracker records the current execution slice defined in `docs/plan.md`. Status values are intentionally narrow so CI, review, and local validation can reference the same progress language.

| Milestone | Status | Owner | Verification Signal | Remaining Gap |
|-----------|--------|-------|---------------------|---------------|
| Shared architecture capability registry | Complete | Codex / CLI-core work | `cargo test -p robustone-core --lib`, `cargo test -p robustone-cli --lib`, `cargo run --manifest-path robustone/Cargo.toml -- -v`, `cargo run --manifest-path robustone/Cargo.toml -- --help` | None for the Round 2 slice |
| Capability-honest CLI and version surfaces | Complete | Codex / CLI work | CLI unit tests, parser-only rejection coverage, `cargo run --manifest-path robustone/Cargo.toml -- x86 90`, `cargo run --manifest-path robustone/Cargo.toml -- --json x86 90`, `cargo run --manifest-path robustone/Cargo.toml -- --help` | None for the Round 2 slice |
| Support-matrix drift detection | Complete | Codex / docs work | `cargo test -p robustone-core --test support_matrix_sync` | None for the Round 1 slice |
| Known-difference governance schema | Complete | Codex / test harness work | `python3 -m unittest discover -s test -p 'test_*.py'` | None for the Round 1 slice |
| Benchmark history and CI summaries | Complete | Codex / CI work | Markdown updates plus `.github/workflows/ci.yml` summary steps validated by `make check` | None for the Round 1 slice |
| RISC-V immediate extraction helper adoption | Complete | Codex / decoder work | `cargo test --workspace --all-features`, `make test` | None for the Round 1 slice |
