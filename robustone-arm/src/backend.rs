#![forbid(unsafe_code)]

//! Minimal ARM backend using the shared ISA framework.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec,
    ModeSet, RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmMode {
    AArch64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ArmFeature: u8 {
        const BASE = 1 << 0;
    }
}

impl FeatureSet for ArmFeature {
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
pub enum ArmField {
    Rd,
    Rn,
    Imm12,
    Imm16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmRegisterClass {
    Gpr,
}

robustone_isa_macros::define_formats! {
    arch = Arm; extern_fields;
    fields { Rd; Rn; Imm12; Imm16; };
    format R {
        Rd: bits(0, 5) as Rd,
    };
    format I_ADD {
        Rd: bits(0, 5) as Rd,
        Rn: bits(5, 5) as Rn,
        Imm12: bits(10, 12) as Imm12,
    };
    format I_MOVZ {
        Rd: bits(0, 5) as Rd,
        Imm16: bits(5, 16) as Imm16,
    }
}

robustone_isa_macros::define_instructions! {
    arch = Arm; module = base;
    insn NOP {
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD5_03_20_1F);
        format = &R;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        effect = None;
        manual = "ARM ARM";
    }
    insn RET {
        mnemonic = "ret";
        opcode_id = "RET";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD6_5F_03_C0);
        format = &R;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Jump];
        effect = Return;
        manual = "ARM ARM";
    }
    insn ADD_IMM {
        mnemonic = "add";
        opcode_id = "ADD_IMM";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x9100_0000);
        format = &I_ADD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn MOVZ {
        mnemonic = "mov";
        opcode_id = "MOVZ";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xD280_0000);
        format = &I_MOVZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
}

pub struct ArmBackend;

impl ArchitectureBackend for ArmBackend {
    type Word = u32;
    type Mode = ArmMode;
    type Feature = ArmFeature;
    type Field = ArmField;
    type RegisterClass = ArmRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Arm
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("arm".to_string()),
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
        SPECS
            .iter()
            .find(|spec| (word & spec.pattern().mask) == spec.pattern().value)
    }

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        RegisterId::arm(raw)
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
    ) -> Result<u32, DisasmError> {
        for f in format.fields() {
            if f.field_type() == field {
                let mask = ((1u64 << f.length()) - 1) as u32;
                return Ok((word >> f.start()) & mask);
            }
        }
        Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidField,
            Some("aarch64".to_string()),
            format!("field {:?} not found in format {}", field, format.name()),
        ))
    }
}
