//! Mock ISA backend for testing the `robustone-isa` framework.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

use crate::{
    Access, ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, ImmediateKind,
    ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, ModeSet, RenderPolicy,
    field,
};

// ============================================================================
// Mock types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MockMode {
    Base,
    Extended,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MockFeature: u8 {
        const BASE = 1 << 0;
        const EXT  = 1 << 1;
    }
}

impl FeatureSet for MockFeature {
    fn empty() -> Self {
        MockFeature::empty()
    }

    fn all_supported_for_tests() -> Self {
        MockFeature::BASE | MockFeature::EXT
    }

    fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockField {
    Rd,
    Rs1,
    Rs2,
    Imm12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockRegisterClass {
    Gpr,
}

// ============================================================================
// Mock format specs (using format_specs! macro)
// ============================================================================

crate::format_specs! {
    format MOCK_FORMAT_R[MockField] {
        rd: field("rd", 0, 5, MockField::Rd),
        rs1: field("rs1", 5, 5, MockField::Rs1),
        rs2: field("rs2", 10, 5, MockField::Rs2),
    }
    format MOCK_FORMAT_I[MockField] {
        rd: field("rd", 0, 5, MockField::Rd),
        rs1: field("rs1", 5, 5, MockField::Rs1),
        imm12: field("imm12", 10, 12, MockField::Imm12),
    }
}

// ============================================================================
// Mock instruction specs (using isa_specs! macro)
// ============================================================================

crate::isa_specs! {
    backend = MockBackend;
    spec ADD {
        mnemonic = "add";
        opcode_id = "ADD";
        pattern = crate::mask_value!(0xFF00_0000, 0x0100_0000);
        format = &MOCK_FORMAT_R;
        operands = &[
            crate::reg!(MockRegisterClass::Gpr, MockField::Rd, Access::Write),
            crate::reg!(MockRegisterClass::Gpr, MockField::Rs1, Access::Read),
            crate::reg!(MockRegisterClass::Gpr, MockField::Rs2, Access::Read),
        ];
        features = MockFeature::BASE;
        modes = ModeSet::All;
        groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
    }
    spec ADDI {
        mnemonic = "addi";
        opcode_id = "ADDI";
        pattern = crate::mask_value!(0xFF00_0000, 0x0200_0000);
        format = &MOCK_FORMAT_I;
        operands = &[
            crate::reg!(MockRegisterClass::Gpr, MockField::Rd, Access::Write),
            crate::reg!(MockRegisterClass::Gpr, MockField::Rs1, Access::Read),
            crate::imm!(MockField::Imm12, ImmediateTransform::SignExtend { bits: 12 }, ImmediateKind::Absolute),
        ];
        features = MockFeature::BASE;
        modes = ModeSet::All;
        groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
    }
    spec EXT_OP {
        mnemonic = "ext_op";
        opcode_id = "EXT_OP";
        pattern = crate::mask_value!(0xFF00_0000, 0x0300_0000);
        format = &MOCK_FORMAT_R;
        operands = &[
            crate::reg!(MockRegisterClass::Gpr, MockField::Rd, Access::Write),
            crate::reg!(MockRegisterClass::Gpr, MockField::Rs1, Access::Read),
        ];
        features = MockFeature::EXT;
        modes = ModeSet::All;
        groups = &[InstructionGroup::Integer];
    }
}

pub static MOCK_SPECS: &[InstructionSpec<MockBackend>] = &[ADD, ADDI, EXT_OP];

// ============================================================================
// Mock backend impl
// ============================================================================

pub struct MockBackend;

impl ArchitectureBackend for MockBackend {
    type Word = u32;
    type Mode = MockMode;
    type Feature = MockFeature;
    type Field = MockField;
    type RegisterClass = MockRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Riscv
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::NeedMoreBytes,
                Some("mock".to_string()),
                "need at least 4 bytes",
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
        MOCK_SPECS
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
        RenderPolicy::new(crate::RenderDialect::Canonical, crate::AliasPolicy::None)
    }

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> Result<u32, DisasmError> {
        for f in format.fields() {
            if f.field_type() == field {
                let mask = ((1u64 << f.length()) - 1) as u32;
                return Ok((word >> f.start()) & mask);
            }
        }
        Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidField,
            Some("mock".to_string()),
            format!("field {:?} not found in format {}", field, format.name()),
        ))
    }
}
