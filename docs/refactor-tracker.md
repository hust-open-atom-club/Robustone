# Refactor Tracker

This tracker records the current execution slice defined in `docs/plan.md`. Status values are intentionally narrow so CI, review, and local validation can reference the same progress language.

| Milestone | Status | Owner | Verification Signal | Remaining Gap |
|-----------|--------|-------|---------------------|---------------|
| Shared architecture capability registry | In Progress | Codex / CLI-core work | `cargo test -p robustone-core architecture --lib`, `cargo test -p robustone-cli --lib` | Finish wiring remaining CLI/docs call sites and complete round validation |
| Capability-honest CLI and version surfaces | In Progress | Codex / CLI work | CLI unit tests plus parser-only rejection coverage | Ensure final unsupported-backend messaging and version/help output stay aligned after full validation |
| Support-matrix drift detection | In Progress | Codex / docs work | `cargo test -p robustone-core --test support_matrix_sync` | Keep table and registry synchronized as round work lands |
| Known-difference governance schema | Planned | Codex / test harness work | Python unit tests in `test/test_known_differences.py` | Enforce `owner` and `expires_on` on active entries and reject stale duplicates |
| Benchmark history and CI summaries | Planned | Codex / CI work | Markdown docs plus `.github/workflows/ci.yml` summary steps | Convert one-shot benchmark snapshot into dated history and emit concise job summaries |
| RISC-V immediate extraction helper adoption | In Progress | Codex / decoder work | `cargo test --workspace --all-features`, parity suite | Complete decoder migration to shared extraction helpers and lock coverage with focused tests |
