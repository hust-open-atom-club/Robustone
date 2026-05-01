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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmRegisterClass {
    Gpr,
}

robustone_isa::format_specs! {
    format R[ArmField] {
        rd: robustone_isa::field("rd", 0, 5, ArmField::Rd),
    }
}

robustone_isa::isa_specs! {
    backend = ArmBackend;
    spec NOP {
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD5_03_20_DF);
        format = &R;
        operands = &[];
        features = ArmFeature::BASE;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
}

pub static ARM_SPECS: &[InstructionSpec<ArmBackend>] = &[NOP];

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
        ARM_SPECS
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
