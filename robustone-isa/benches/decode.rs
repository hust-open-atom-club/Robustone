use criterion::{Criterion, criterion_group, criterion_main};
use robustone_isa::{
    AliasPolicy, ArchitectureBackend, DecodeProfile, FeatureSet, ModeSet, RenderDialect,
    RenderPolicy,
};
use std::hint::black_box;

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

    c.bench_function("isa_decode_one", |b| {
        b.iter(|| {
            black_box(robustone_isa::decode_one::<BenchBackend>(&bytes, 0x1000, &profile).unwrap())
        });
    });
}

criterion_group!(benches, bench_decode_one);
criterion_main!(benches);
