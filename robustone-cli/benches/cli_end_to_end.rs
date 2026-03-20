use clap::Parser;
use criterion::{Criterion, criterion_group, criterion_main};
use robustone_cli::{Cli, CliExecutor, DisasmConfig};
use std::hint::black_box;

fn bench_cli_end_to_end(c: &mut Criterion) {
    let executor = CliExecutor::new();
    let args = ["robustone", "riscv32", "93001000"];

    c.bench_function("riscv32_cli_end_to_end", |b| {
        b.iter(|| {
            let cli = Cli::try_parse_from(args).unwrap();
            let config = DisasmConfig::config_from_cli(&cli).unwrap();
            let rendered = executor.execute_to_string(&config).unwrap();
            black_box(rendered)
        });
    });
}

criterion_group!(benches, bench_cli_end_to_end);
criterion_main!(benches);
