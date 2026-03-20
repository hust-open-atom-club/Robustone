# Benchmark Baselines

These measurements were recorded locally on 2026-03-20 with:

- `cargo bench -p robustone-core --bench riscv_decode`

## Current Baseline

| Benchmark | Result |
|-----------|--------|
| `riscv32_decode_ir` | `829.03 ns` to `837.87 ns` |
| `riscv32_detail_overhead` | `1.0591 us` to `1.0633 us` |
| `riscv32_cli_end_to_end` | `1.7032 us` to `1.8287 us` |

These numbers are intended as in-repo baselines, not performance claims across machines.

## Fuzz Smoke Notes

Local smoke runs completed successfully with:

- `cargo +nightly fuzz run decode_riscv -- -max_total_time=5`
- `cargo +nightly fuzz run format_riscv_json -- -max_total_time=5`

Both runs completed without crashes in this environment.
