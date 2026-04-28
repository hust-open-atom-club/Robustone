//! LoongArch backend implementing the `robustone-isa` `ArchitectureBackend` trait.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::DisasmError;
use robustone_isa::{
    Access, ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, ImmediateKind,
    ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, ModeSet, RenderPolicy,
    field,
};

// ============================================================================
// LoongArch types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchMode {
    LA32,
    LA64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoongArchFeature: u16 {
        const BASE       = 1 << 0;
        const LA64       = 1 << 1;
        const FLOAT32    = 1 << 2;
        const FLOAT64    = 1 << 3;
        const PRIVILEGED = 1 << 4;
        const ATOMIC     = 1 << 5;
        const LSX        = 1 << 6;
        const LASX       = 1 << 7;
        const LVZ        = 1 << 8;
        const LBT        = 1 << 9;
    }
}

impl FeatureSet for LoongArchFeature {
    fn empty() -> Self {
        LoongArchFeature::empty()
    }

    fn all_supported_for_tests() -> Self {
        LoongArchFeature::all()
    }

    fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchField {
    Rd,
    Rj,
    Rk,
    Ra,
    I8,
    Si12,
    Ui12,
    Si14,
    Si16,
    Si20,
    Si21,
    I26Lo,
    I26Hi,
    Code,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchRegisterClass {
    Gpr,
    Fpr,
    Xr,
    Fcc,
    Scr,
}

// ============================================================================
// Format specs
// ============================================================================

robustone_isa::format_specs! {
    format FMT_R2[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
    }
    format FMT_R3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
    }
    format FMT_R4[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        ra: field("ra", 15, 5, LoongArchField::Ra),
    }
    format FMT_R2I8[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        i8: field("i8", 10, 8, LoongArchField::I8),
    }
    format FMT_R2I12[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si12: field("si12", 10, 12, LoongArchField::Si12),
    }
    format FMT_R2I14[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si14: field("si14", 10, 14, LoongArchField::Si14),
    }
    format FMT_R2I16[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_1RI21[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        si21: field("si21", 5, 21, LoongArchField::Si21),
    }
    format FMT_I26[LoongArchField] {
        i26_lo: field("i26_lo", 0, 16, LoongArchField::I26Lo),
        i26_hi: field("i26_hi", 16, 10, LoongArchField::I26Hi),
    }
    format FMT_R1[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
    }
}

// ============================================================================
// Base integer instruction specs
// ============================================================================

macro_rules! loongarch_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $fmt:expr, $operands:expr) => {
        robustone_isa::isa_specs! {
            backend = LoongArchBackend;
            spec $name {
                mnemonic = $mnemonic;
                opcode_id = $opcode_id;
                pattern = robustone_isa::mask_value!($mask, $value);
                format = $fmt;
                operands = $operands;
                features = LoongArchFeature::BASE;
                modes = ModeSet::All;
                groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
            }
        }
    };
}

macro_rules! r3_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R3,
            &[
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rd,
                    Access::Write
                ),
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rj,
                    Access::Read
                ),
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rk,
                    Access::Read
                ),
            ]
        );
    };
}

macro_rules! r2i12_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I12,
            &[
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rd,
                    Access::Write
                ),
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rj,
                    Access::Read
                ),
                robustone_isa::imm!(
                    LoongArchField::Si12,
                    ImmediateTransform::SignExtend { bits: 12 },
                    ImmediateKind::Absolute
                ),
            ]
        );
    };
}

r3_insn!(ADD_W, "add.w", "ADD_W", 0xFFFF_8000, 0x0010_0000);
r3_insn!(ADD_D, "add.d", "ADD_D", 0xFFFF_8000, 0x0010_8000);
r3_insn!(SUB_W, "sub.w", "SUB_W", 0xFFFF_8000, 0x0011_0000);
r3_insn!(SUB_D, "sub.d", "SUB_D", 0xFFFF_8000, 0x0011_8000);
r3_insn!(SLT, "slt", "SLT", 0xFFFF_8000, 0x0012_0000);
r3_insn!(SLTU, "sltu", "SLTU", 0xFFFF_8000, 0x0012_8000);
r3_insn!(AND, "and", "AND", 0xFFFF_8000, 0x0014_8000);
r3_insn!(OR, "or", "OR", 0xFFFF_8000, 0x0015_0000);
r3_insn!(XOR, "xor", "XOR", 0xFFFF_8000, 0x0015_8000);
r3_insn!(NOR, "nor", "NOR", 0xFFFF_8000, 0x0016_0000);

r2i12_insn!(ADDI_W, "addi.w", "ADDI_W", 0xFFC0_0000, 0x0280_0000);
r2i12_insn!(ADDI_D, "addi.d", "ADDI_D", 0xFFC0_0000, 0x02C0_0000);

// ============================================================================
// Spec table
// ============================================================================

pub static LOONGARCH_BASE_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    ADD_W, ADD_D, SUB_W, SUB_D, SLT, SLTU, AND, OR, XOR, NOR, ADDI_W, ADDI_D,
];

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use robustone_isa::{AliasPolicy, DecodeProfile, RenderDialect, decode_one};

    fn la64_base_profile() -> DecodeProfile<LoongArchBackend> {
        DecodeProfile {
            mode: LoongArchMode::LA64,
            features: LoongArchFeature::BASE | LoongArchFeature::LA64,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        }
    }

    #[test]
    fn backend_reads_fixed_4byte_le() {
        let bytes = &[0xAC, 0x39, 0x10, 0x00];
        let result = LoongArchBackend::read_instruction(bytes);
        assert!(result.is_ok());
        let read = result.unwrap();
        assert_eq!(read.raw, 0x0010_39AC);
        assert_eq!(read.length, 4);
    }

    #[test]
    fn backend_decode_add_w() {
        // add.w $t0, $t1, $t2
        // rd=12, rj=13, rk=14
        let word: u32 = 0x0010_0000 | 12 | (13 << 5) | (14 << 10);
        let bytes = word.to_le_bytes();
        let result = decode_one::<LoongArchBackend>(&bytes, 0x1000, &la64_base_profile());
        assert!(result.is_ok(), "{:?}", result);
        let insn = result.unwrap();
        assert_eq!(insn.mnemonic, "add.w");
        assert_eq!(insn.size, 4);
        assert_eq!(insn.registers_written.len(), 1);
        assert_eq!(insn.registers_read.len(), 2);
    }

    #[test]
    fn backend_decode_addi_w() {
        // addi.w $t0, $t1, 42
        // rd=12, rj=13, si12=42
        let word: u32 = 0x0280_0000 | 12 | (13 << 5) | (42 << 10);
        let bytes = word.to_le_bytes();
        let result = decode_one::<LoongArchBackend>(&bytes, 0x1000, &la64_base_profile());
        assert!(result.is_ok(), "{:?}", result);
        let insn = result.unwrap();
        assert_eq!(insn.mnemonic, "addi.w");
        assert_eq!(insn.size, 4);
        assert_eq!(insn.registers_written.len(), 1);
        assert_eq!(insn.registers_read.len(), 1);
        // Immediate operand
        assert_eq!(insn.operands.len(), 3);
        match &insn.operands[2] {
            robustone_core::ir::Operand::Immediate { value } => {
                assert_eq!(*value, 42);
            }
            other => panic!("expected immediate operand, got {:?}", other),
        }
    }

    #[test]
    fn base_specs_have_no_overlaps() {
        assert!(robustone_isa::validate_no_overlaps(LOONGARCH_BASE_SPECS).is_ok());
    }
}

// ============================================================================
// Backend implementation
// ============================================================================

pub struct LoongArchBackend;

impl ArchitectureBackend for LoongArchBackend {
    type Word = u32;
    type Mode = LoongArchMode;
    type Feature = LoongArchFeature;
    type Field = LoongArchField;
    type RegisterClass = LoongArchRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::LoongArch
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::NeedMoreBytes,
                Some("loongarch".to_string()),
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
        LOONGARCH_BASE_SPECS
            .iter()
            .find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }

    fn lower_register(
        class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        let id = match class {
            LoongArchRegisterClass::Gpr => raw,
            LoongArchRegisterClass::Fpr => raw + 32,
            LoongArchRegisterClass::Xr => raw + 64,
            LoongArchRegisterClass::Fcc => raw + 96,
            LoongArchRegisterClass::Scr => raw + 104,
        };
        RegisterId::loongarch(id)
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        RenderPolicy::new(
            robustone_isa::RenderDialect::Assembler,
            robustone_isa::AliasPolicy::PreferPseudo,
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
