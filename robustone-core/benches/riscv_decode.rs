use criterion::{Criterion, criterion_group, criterion_main};
use robustone_core::ir::TextRenderProfile;
use robustone_core::{ArchitectureDispatcher, common::ArchitectureProfile};
use std::hint::black_box;

fn bench_riscv_decode(c: &mut Criterion) {
    let dispatcher = ArchitectureDispatcher::new();
    let profile = ArchitectureProfile::riscv32gc();
    let bytes = [0x93, 0x00, 0x10, 0x00];
    let hex = "93001000";

    c.bench_function("riscv32_decode_ir", |b| {
        b.iter(|| black_box(dispatcher.decode_with_profile(&bytes, &profile, 0).unwrap()));
    });

    c.bench_function("riscv32_detail_overhead", |b| {
        b.iter(|| {
            let (instruction, _) = dispatcher.disassemble_bytes(&bytes, "riscv32", 0).unwrap();
            let detail = instruction.detail.as_ref().map(|detail| {
                (
                    detail.registers_read().len(),
                    detail.registers_written().len(),
                )
            });
            black_box((instruction, detail))
        });
    });

    c.bench_function("riscv32_cli_end_to_end", |b| {
        b.iter(|| {
            let instruction = dispatcher.disassemble(hex, "riscv32".to_string());
            let rendered = instruction.rendered_text_parts(TextRenderProfile::Capstone);
            black_box((instruction, rendered))
        });
    });
}

criterion_group!(benches, bench_riscv_decode);
criterion_main!(benches);
