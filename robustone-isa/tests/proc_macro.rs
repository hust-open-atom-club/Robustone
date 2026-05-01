//! Integration tests for robustone-isa-macros proc-macros.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    Access, AliasPolicy, ArchitectureBackend, DecodeProfile, FormatSpec, InstructionGroup,
    InstructionRead, InstructionSpec, ModeSet, RenderDialect, RenderPolicy, field,
};

robustone_isa_macros::define_arch! {
    pub arch Macro {
        word = u32;
        endian = little;
        instruction_length = fixed(4);
        modes { Base = "base"; };
        features: u8 { BASE = 0; };
        registers = macro_registers;
        formats = macro_formats;
        specs = macro_specs;
        render = MacroRenderPolicy;
    }
}

robustone_isa_macros::define_registers! {
    arch = Macro;
    bank Gpr { count = 32; prefix = "$r"; }
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
            robustone_isa::reg!(MacroRegisterClass::Gpr, MacroField::Rd, Access::Write),
            robustone_isa::reg!(MacroRegisterClass::Gpr, MacroField::Rs1, Access::Read),
            robustone_isa::reg!(MacroRegisterClass::Gpr, MacroField::Rs2, Access::Read),
        ];
        modes = ModeSet::All;
        features = MacroFeature::BASE;
        groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
        manual = "Test";
    }
}

robustone_isa_macros::define_aliases! {
    arch = Macro;
    alias "nop" for "MACRO_ADD" {
        when [operand(0) == reg(0), operand(1) == reg(0), operand(2) == reg(0)];
        mnemonic = "nop";
        visible_operands = [];
    }
}

impl ArchitectureBackend for MacroBackend {
    type Word = u32;
    type Mode = MacroMode;
    type Feature = MacroFeature;
    type Field = MacroField;
    type RegisterClass = MacroRegisterClass;

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
            .find(|spec| (word & spec.pattern().mask) == spec.pattern().value)
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
    ) -> Result<u32, DisasmError> {
        for f in format.fields {
            if f.field_type == field {
                let mask = ((1u64 << f.length) - 1) as u32;
                return Ok((word >> f.start) & mask);
            }
        }
        Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidField,
            None::<String>,
            format!("field not found"),
        ))
    }
}

#[test]
fn proc_macro_define_formats_generates_format_spec() {
    assert_eq!(MACRO_R3.name, "MACRO_R3");
    assert_eq!(MACRO_R3.fields.len(), 3);
}

#[test]
fn proc_macro_define_instructions_generates_spec() {
    assert_eq!(MACRO_ADD.mnemonic(), "add");
    assert_eq!(MACRO_ADD.opcode_id(), "MACRO_ADD");
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

#[test]
fn proc_macro_define_aliases_applies_compat_mnemonic() {
    let profile = DecodeProfile {
        mode: MacroMode::Base,
        features: MacroFeature::BASE,
        render_dialect: RenderDialect::Canonical,
        alias_policy: AliasPolicy::None,
    };
    // word = 0x0100_0000 => rd=0, rs1=0, rs2=0
    let bytes = &[0x00, 0x00, 0x00, 0x01];
    let mut decoded = robustone_isa::decode_one::<MacroBackend>(bytes, 0x1000, &profile).unwrap();
    assert_eq!(decoded.mnemonic, "add");
    assert!(decoded.render_hints.compat_mnemonic.is_none());
    apply_aliases(&mut decoded);
    assert_eq!(
        decoded.render_hints.compat_mnemonic,
        Some("nop".to_string())
    );
    assert_eq!(decoded.render_hints.compat_hidden_operands, vec![0, 1, 2]);
}
