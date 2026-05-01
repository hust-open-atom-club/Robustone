#![forbid(unsafe_code)]

//! Minimal x86 backend using the shared ISA framework.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec,
    ModeSet, RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Mode {
    X64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct X86Feature: u8 {
        const BASE = 1 << 0;
    }
}

impl FeatureSet for X86Feature {
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
pub enum X86Field {
    ModRM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86RegisterClass {
    Gpr,
}

robustone_isa::format_specs! {
    format R[X86Field] {
        modrm: robustone_isa::field("modrm", 0, 8, X86Field::ModRM),
    }
}

robustone_isa::isa_specs! {
    backend = X86Backend;
    spec NOP {
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0000_0090);
        format = &R;
        operands = &[];
        features = X86Feature::BASE;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "Intel SDM";
    }
}

pub static X86_SPECS: &[InstructionSpec<X86Backend>] = &[NOP];

pub struct X86Backend;

impl ArchitectureBackend for X86Backend {
    type Word = u32;
    type Mode = X86Mode;
    type Feature = X86Feature;
    type Field = X86Field;
    type RegisterClass = X86RegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::X86
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("x86".to_string()),
                "need at least 1 byte",
            ));
        }
        // Minimal: treat first byte as the instruction
        let word = bytes[0] as u32;
        Ok(InstructionRead {
            raw: word,
            length: 1,
        })
    }

    fn lookup(
        word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        X86_SPECS
            .iter()
            .find(|spec| (word & spec.pattern().mask) == spec.pattern().value)
    }

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        RegisterId::x86(raw)
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
            Some("x86".to_string()),
            format!("field {:?} not found in format {}", field, format.name()),
        ))
    }
}
