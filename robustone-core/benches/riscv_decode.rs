use criterion::{Criterion, criterion_group, criterion_main};
use robustone_core::{ArchitectureDispatcher, common::ArchitectureProfile};

fn bench_riscv_decode(c: &mut Criterion) {
    let dispatcher = ArchitectureDispatcher::new();
    let profile = ArchitectureProfile::riscv32gc();
    let bytes = [0x93, 0x00, 0x10, 0x00];

    c.bench_function("riscv32_decode_ir", |b| {
        b.iter(|| dispatcher.decode_with_profile(&bytes, &profile, 0).unwrap());
    });

    c.bench_function("riscv32_decode_text", |b| {
        b.iter(|| dispatcher.disassemble_bytes(&bytes, "riscv32", 0).unwrap());
    });
}

criterion_group!(benches, bench_riscv_decode);
criterion_main!(benches);
