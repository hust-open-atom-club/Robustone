# Benchmark Baselines

These measurements are in-repository baselines for regression tracking, not performance claims across machines. New benchmark updates should append a dated row instead of overwriting prior entries.

## Benchmark Commands

- `cargo bench -p robustone-core --bench riscv_decode`
- `cargo bench -p robustone-cli --bench cli_end_to_end`

## History

| Date | Commit | Benchmark | Result | Notes |
|------|--------|-----------|--------|-------|
| 2026-03-20 | `258a109` | `riscv32_decode_ir` | `815.06 ns` to `819.98 ns` | Pure shared-IR decode throughput baseline. |
| 2026-03-20 | `258a109` | `riscv32_detail_overhead` | `1.0724 us` to `1.0748 us` | Detail rendering overhead on top of decode. |
| 2026-03-20 | `258a109` | `riscv32_compat_text_render` | `1.6670 us` to `1.6872 us` | Capstone-style text rendering baseline. |
| 2026-03-20 | `258a109` | `riscv32_cli_end_to_end` | `16.121 us` to `16.617 us` | End-to-end CLI path through parsing, config, execution, and formatting. |

## Update Rules

- Add a new dated row for each benchmark refresh.
- Keep the command name and the commit that produced the sample.
- Do not delete older rows unless the benchmark itself is removed from the repository, and document that removal in the same change.

## Fuzz Smoke Notes

Local smoke runs completed successfully on 2026-03-20 with:

- `cargo +nightly fuzz run decode_riscv -- -max_total_time=5`
- `cargo +nightly fuzz run format_riscv_json -- -max_total_time=5`

Both runs completed without crashes in that environment.
