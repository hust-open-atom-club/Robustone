use criterion::{Criterion, criterion_group, criterion_main};
use robustone_isa::{
    AliasPolicy, ArchitectureBackend, DecodeProfile, FeatureSet, RenderDialect, RenderPolicy,
};
use std::hint::black_box;

// Real-backend benchmarks
use robustone_loongarch::backend::{LoongArchBackend, LoongArchFeature, LoongArchMode};
use robustone_riscv::backend::{RiscVBackend, RiscVFeature, RiscVMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchMode {
    Base,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BenchFeature: u8 {
        const BASE = 1 << 0;
    }
}

impl FeatureSet for BenchFeature {
    fn empty() -> Self {
        Self::empty()
    }
    fn all_supported_for_tests() -> Self {
        Self::BASE
    }
    fn contains(self, required: Self) -> bool {
        self.bits() & required.bits() == required.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchField {
    Rd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchRegClass {
    Gpr,
}

robustone_isa::format_specs! {
    format BENCH_FORMAT[BenchField] {
        rd: robustone_isa::field("rd", 0, 5, BenchField::Rd),
    }
}

robustone_isa_macros::define_instructions! {
    arch = Bench; module = base;
    insn NOP {
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0000_0000);
        format = &BENCH_FORMAT;
        operands = &[];
        modes = robustone_isa::ModeSet::All;
        features = BenchFeature::BASE;
        groups = &[];
        manual = "Benchmark";
    }
}

pub struct BenchBackend;

impl ArchitectureBackend for BenchBackend {
    type Word = u32;
    type Mode = BenchMode;
    type Feature = BenchFeature;
    type Field = BenchField;
    type RegisterClass = BenchRegClass;

    fn architecture_id() -> robustone_core::ir::ArchitectureId {
        robustone_core::ir::ArchitectureId::Riscv
    }

    fn read_instruction(
        bytes: &[u8],
    ) -> Result<robustone_isa::InstructionRead<Self::Word>, robustone_core::types::error::DisasmError>
    {
        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(robustone_isa::InstructionRead {
            raw: word,
            length: 4,
        })
    }

    fn lookup(
        word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static robustone_isa::InstructionSpec<Self>> {
        SPECS
            .iter()
            .find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> robustone_core::ir::RegisterId {
        robustone_core::ir::RegisterId::riscv(raw)
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        RenderPolicy::new(RenderDialect::Canonical, AliasPolicy::None)
    }

    fn extract_field(
        word: Self::Word,
        format: &robustone_isa::FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> u32 {
        for f in format.fields {
            if f.field_type == field {
                let mask = ((1u64 << f.length) - 1) as u32;
                return (word >> f.start) & mask;
            }
        }
        0
    }
}

fn bench_decode_one(c: &mut Criterion) {
    let profile = DecodeProfile {
        mode: BenchMode::Base,
        features: BenchFeature::BASE,
        render_dialect: RenderDialect::Canonical,
        alias_policy: AliasPolicy::None,
    };
    let bytes = [0x00, 0x00, 0x00, 0x00];

    c.bench_function("isa_decode_one_nop", |b| {
        b.iter(|| {
            black_box(robustone_isa::decode_one::<BenchBackend>(&bytes, 0x1000, &profile).unwrap())
        });
    });
}

fn bench_decode_loongarch(c: &mut Criterion) {
    let profile = DecodeProfile {
        mode: LoongArchMode::LA64,
        features: LoongArchFeature::all_supported_for_tests(),
        render_dialect: RenderDialect::Canonical,
        alias_policy: AliasPolicy::None,
    };

    // add.w $t0, $t1, $t2
    let add_bytes = [0x00, 0x10, 0x00, 0x00];
    c.bench_function("loongarch_decode_add", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<LoongArchBackend>(&add_bytes, 0x1000, &profile)
                    .unwrap(),
            )
        });
    });

    // beq $t0, $t1, offset
    let beq_bytes = [0x00, 0x10, 0x00, 0x58];
    c.bench_function("loongarch_decode_beq", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<LoongArchBackend>(&beq_bytes, 0x1000, &profile)
                    .unwrap(),
            )
        });
    });

    // ld.w $t0, $t1, 0
    let ld_bytes = [0x00, 0x10, 0x00, 0x28];
    c.bench_function("loongarch_decode_ld_w", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<LoongArchBackend>(&ld_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });
}

fn bench_decode_riscv(c: &mut Criterion) {
    let profile = DecodeProfile {
        mode: RiscVMode::RV64,
        features: RiscVFeature::all_supported_for_tests(),
        render_dialect: RenderDialect::Canonical,
        alias_policy: AliasPolicy::None,
    };

    // add x1, x2, x3  =>  0x001100B3
    let add_bytes = [0xB3, 0x00, 0x11, 0x00];
    c.bench_function("riscv_decode_add", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<RiscVBackend>(&add_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });

    // addi x1, x0, 42  =>  0x02A00093
    let addi_bytes = [0x93, 0x00, 0xA0, 0x02];
    c.bench_function("riscv_decode_addi", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<RiscVBackend>(&addi_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });

    // beq x1, x2, +8  =>  0x00208463
    let beq_bytes = [0x63, 0x04, 0x20, 0x00];
    c.bench_function("riscv_decode_beq", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<RiscVBackend>(&beq_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });

    // jal x0, +16  =>  0x0100006F
    let jal_bytes = [0x6F, 0x00, 0x00, 0x01];
    c.bench_function("riscv_decode_jal", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<RiscVBackend>(&jal_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });

    // lw x1, 4(x2)  =>  0x00412083
    let lw_bytes = [0x83, 0x20, 0x41, 0x00];
    c.bench_function("riscv_decode_lw", |b| {
        b.iter(|| {
            black_box(
                robustone_isa::decode_one::<RiscVBackend>(&lw_bytes, 0x1000, &profile).unwrap(),
            )
        });
    });

    // invalid encoding
    let invalid_bytes = [0xFF, 0xFF, 0xFF, 0xFF];
    c.bench_function("riscv_decode_invalid", |b| {
        b.iter(|| {
            let _ = black_box(robustone_isa::decode_one::<RiscVBackend>(
                &invalid_bytes,
                0x1000,
                &profile,
            ));
        });
    });
}

criterion_group!(
    benches,
    bench_decode_one,
    bench_decode_loongarch,
    bench_decode_riscv
);
criterion_main!(benches);
