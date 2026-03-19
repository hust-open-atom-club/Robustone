# Benchmark Baselines

These measurements were recorded locally on 2026-03-19 with:

- `cargo bench -p robustone-core --bench riscv_decode`

## Current Baseline

| Benchmark | Result |
|-----------|--------|
| `riscv32_decode_ir` | `505.04 ns` to `507.73 ns` |
| `riscv32_decode_text` | `915.95 ns` to `971.34 ns` |

These numbers are intended as a first in-repo baseline, not a performance claim across machines.

## Fuzz Smoke Notes

Local smoke runs completed successfully with:

- `cargo +nightly fuzz run decode_riscv -- -max_total_time=5`
- `cargo +nightly fuzz run format_riscv_json -- -max_total_time=5`

Both runs completed without crashes in this environment.
