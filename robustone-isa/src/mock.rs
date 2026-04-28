//! Mock ISA backend for testing the `robustone-isa` framework.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::DisasmError;

use crate::{
    Access, ArchitectureBackend, DecodeProfile, EncodingPattern, FeatureSet, FormatSpec,
    ImmediateKind, ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, ModeSet,
    OperandSpec, RenderPolicy, field,
};

// ============================================================================
// Mock types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
// Mock format specs
// ============================================================================

pub static MOCK_FORMAT_R: FormatSpec<MockField> = FormatSpec {
    name: "R",
    fields: &[
        field("rd", 0, 5, MockField::Rd),
        field("rs1", 5, 5, MockField::Rs1),
        field("rs2", 10, 5, MockField::Rs2),
    ],
};

pub static MOCK_FORMAT_I: FormatSpec<MockField> = FormatSpec {
    name: "I",
    fields: &[
        field("rd", 0, 5, MockField::Rd),
        field("rs1", 5, 5, MockField::Rs1),
        field("imm12", 10, 12, MockField::Imm12),
    ],
};

// ============================================================================
// Mock instruction specs
// ============================================================================

pub static MOCK_SPECS: &[InstructionSpec<MockBackend>] = &[
    // add: mask=0xFF00_0000, value=0x0100_0000
    InstructionSpec {
        mnemonic: "add",
        opcode_id: "ADD",
        pattern: EncodingPattern::new(0xFF00_0000, 0x0100_0000),
        format: &MOCK_FORMAT_R,
        operands: &[
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rd,
                access: Access::Write,
            },
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rs1,
                access: Access::Read,
            },
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rs2,
                access: Access::Read,
            },
        ],
        features: MockFeature::BASE,
        modes: ModeSet::All,
        groups: &[InstructionGroup::Integer, InstructionGroup::Arithmetic],
        manual_ref: None,
    },
    // addi: mask=0xFF00_0000, value=0x0200_0000
    InstructionSpec {
        mnemonic: "addi",
        opcode_id: "ADDI",
        pattern: EncodingPattern::new(0xFF00_0000, 0x0200_0000),
        format: &MOCK_FORMAT_I,
        operands: &[
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rd,
                access: Access::Write,
            },
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rs1,
                access: Access::Read,
            },
            OperandSpec::Immediate {
                field: MockField::Imm12,
                transform: ImmediateTransform::SignExtend { bits: 12 },
                kind: ImmediateKind::Absolute,
            },
        ],
        features: MockFeature::BASE,
        modes: ModeSet::All,
        groups: &[InstructionGroup::Integer, InstructionGroup::Arithmetic],
        manual_ref: None,
    },
    // ext_op: mask=0xFF00_0000, value=0x0300_0000 (requires EXT feature)
    InstructionSpec {
        mnemonic: "ext_op",
        opcode_id: "EXT_OP",
        pattern: EncodingPattern::new(0xFF00_0000, 0x0300_0000),
        format: &MOCK_FORMAT_R,
        operands: &[
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rd,
                access: Access::Write,
            },
            OperandSpec::Register {
                class: MockRegisterClass::Gpr,
                field: MockField::Rs1,
                access: Access::Read,
            },
        ],
        features: MockFeature::EXT,
        modes: ModeSet::All,
        groups: &[InstructionGroup::Integer],
        manual_ref: None,
    },
];

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
    ) -> u32 {
        for f in format.fields {
            if f.field_type == field {
                let mask = ((1u64 << f.length) - 1) as u32;
                return ((word >> f.start) & mask) as u32;
            }
        }
        0
    }
}
