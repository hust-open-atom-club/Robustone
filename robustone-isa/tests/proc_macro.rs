//! Integration tests for robustone-isa-macros proc-macros.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    Access, AliasPolicy, ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, ImmediateKind,
    ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, ModeSet, RenderDialect,
    RenderPolicy, field,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroMode {
    Base,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MacroFeature: u8 {
        const BASE = 1 << 0;
    }
}

impl FeatureSet for MacroFeature {
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
pub enum MacroField {
    Rd,
    Rs1,
    Rs2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroRegClass {
    Gpr,
}

robustone_isa_macros::define_formats! {
    arch = Macro;
    fields { rd; rs1; rs2; };
    format MACRO_R3 { rd: bits(0, 5), rs1: bits(5, 5), rs2: bits(10, 5) }
}

robustone_isa_macros::define_instructions! {
    arch = Macro; module = base;
    insn MACRO_ADD {
        mnemonic = "add";
        opcode_id = "MACRO_ADD";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x0100_0000);
        format = &MACRO_R3;
        operands = &[
            robustone_isa::reg!(MacroRegClass::Gpr, MacroField::Rd, Access::Write),
            robustone_isa::reg!(MacroRegClass::Gpr, MacroField::Rs1, Access::Read),
            robustone_isa::reg!(MacroRegClass::Gpr, MacroField::Rs2, Access::Read),
        ];
        modes = ModeSet::All;
        features = MacroFeature::BASE;
        groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
        manual = "Test";
    }
}

pub struct MacroBackend;

impl ArchitectureBackend for MacroBackend {
    type Word = u32;
    type Mode = MacroMode;
    type Feature = MacroFeature;
    type Field = MacroField;
    type RegisterClass = MacroRegClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Riscv
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("macro".to_string()),
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
        RenderPolicy::new(RenderDialect::Canonical, AliasPolicy::None)
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

#[test]
fn proc_macro_define_formats_generates_format_spec() {
    assert_eq!(MACRO_R3.name, "MACRO_R3");
    assert_eq!(MACRO_R3.fields.len(), 3);
}

#[test]
fn proc_macro_define_instructions_generates_spec() {
    assert_eq!(MACRO_ADD.mnemonic, "add");
    assert_eq!(MACRO_ADD.opcode_id, "MACRO_ADD");
}

#[test]
fn proc_macro_specs_decode_correctly() {
    let profile = DecodeProfile {
        mode: MacroMode::Base,
        features: MacroFeature::BASE,
        render_dialect: RenderDialect::Canonical,
        alias_policy: AliasPolicy::None,
    };
    let bytes = &[0x00, 0x00, 0x00, 0x01];
    let decoded = robustone_isa::decode_one::<MacroBackend>(bytes, 0x1000, &profile).unwrap();
    assert_eq!(decoded.mnemonic, "add");
}
