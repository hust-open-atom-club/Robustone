#![forbid(unsafe_code)]

//! Minimal RISC-V backend using the shared ISA framework.
//!
//! This is a shadow backend that demonstrates the framework for RISC-V.
//! Full migration (replacing the legacy extension-based decoder) is tracked
//! under AC-9.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec,
    ModeSet, RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVMode {
    RV32,
    RV64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RiscVFeature: u16 {
        const I = 1 << 0;
        const M = 1 << 1;
        const A = 1 << 2;
        const F = 1 << 3;
        const D = 1 << 4;
        const C = 1 << 5;
        const G = Self::I.bits() | Self::M.bits() | Self::A.bits() | Self::F.bits() | Self::D.bits();
    }
}

impl FeatureSet for RiscVFeature {
    fn empty() -> Self {
        Self::empty()
    }
    fn all_supported_for_tests() -> Self {
        Self::G | Self::C
    }
    fn contains(self, required: Self) -> bool {
        self.bits() & required.bits() == required.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVField {
    Rd,
    Rs1,
    Rs2,
    Funct3,
    Funct7,
    Opcode,
    Imm12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVRegisterClass {
    Gpr,
    Fpr,
}

robustone_isa::format_specs! {
    format R_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        rs2: robustone_isa::field("rs2", 20, 5, RiscVField::Rs2),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
        funct7: robustone_isa::field("funct7", 25, 7, RiscVField::Funct7),
    }
    format I_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        imm12: robustone_isa::field("imm12", 20, 12, RiscVField::Imm12),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
    }
}

robustone_isa::isa_specs! {
    backend = RiscVBackend;
    spec ADD {
        mnemonic = "add";
        opcode_id = "ADD";
        pattern = robustone_isa::mask_value!(0xFE00_007F, 0x0000_0033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec ADDI {
        mnemonic = "addi";
        opcode_id = "ADDI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0013);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}

pub static RISCV_SPECS: &[InstructionSpec<RiscVBackend>] = &[ADD, ADDI];

pub struct RiscVBackend;

impl ArchitectureBackend for RiscVBackend {
    type Word = u32;
    type Mode = RiscVMode;
    type Feature = RiscVFeature;
    type Field = RiscVField;
    type RegisterClass = RiscVRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Riscv
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("riscv".to_string()),
                "need 4 bytes",
            ));
        }
        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(InstructionRead {
            raw: word,
            length: 4,
        })
    }

    fn lookup(
        word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        RISCV_SPECS
            .iter()
            .find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        RegisterId::riscv(raw)
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        RenderPolicy::new(
            robustone_isa::RenderDialect::Canonical,
            robustone_isa::AliasPolicy::None,
        )
    }

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
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

pub type RiscVIsaDecoder = robustone_isa::Decoder<RiscVBackend>;
