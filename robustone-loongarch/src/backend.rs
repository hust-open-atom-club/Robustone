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
    Cd,
    I8,
    Si12,
    Ui12,
    Si14,
    Si16,
    Si20,
    Si21,
    Ui5,
    Ui6,
    MsbW,
    LsbW,
    MsbD,
    LsbD,
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
    format FMT_R3I2[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        sa2: field("sa2", 15, 2, LoongArchField::Ui5),
    }
    format FMT_R3I3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        ui3: field("ui3", 15, 3, LoongArchField::Ui5),
    }
    format FMT_R4[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        ra: field("ra", 15, 5, LoongArchField::Ra),
    }
    format FMT_FCMP[LoongArchField] {
        cd: field("cd", 0, 3, LoongArchField::Cd),
        fj: field("fj", 5, 5, LoongArchField::Rj),
        fk: field("fk", 10, 5, LoongArchField::Rk),
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
    format FMT_BJ[LoongArchField] {
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_R1I16_BJ[LoongArchField] {
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_I15[LoongArchField] {
        code: field("code", 0, 15, LoongArchField::Code),
    }
    format FMT_R2I5[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui5: field("ui5", 10, 5, LoongArchField::Ui5),
    }
    format FMT_R2I6[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui6: field("ui6", 10, 6, LoongArchField::Ui6),
    }
    format FMT_R2I3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui3: field("ui3", 10, 3, LoongArchField::Ui5),
    }
    format FMT_R2I4[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui4: field("ui4", 10, 4, LoongArchField::Ui5),
    }
    format FMT_BSTR_W[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        msb: field("msb", 16, 5, LoongArchField::MsbW),
        lsb: field("lsb", 10, 5, LoongArchField::LsbW),
    }
    format FMT_BSTR_D[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        msb: field("msb", 16, 6, LoongArchField::MsbD),
        lsb: field("lsb", 10, 6, LoongArchField::LsbD),
    }
    format FMT_1RI20[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        si20: field("si20", 5, 20, LoongArchField::Si20),
    }
    format FMT_R2I12_U[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui12: field("ui12", 10, 12, LoongArchField::Ui12),
    }
}

// ============================================================================
// Base integer instruction specs
// ============================================================================

macro_rules! loongarch_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $fmt:expr, $operands:expr, $groups:expr) => {
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
                groups = $groups;
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Arithmetic]
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Arithmetic]
        );
    };
}

// Shift instructions (R3)
r3_insn!(SLL_W, "sll.w", "SLL_W", 0xFFFF_8000, 0x0017_0000);
r3_insn!(SRL_W, "srl.w", "SRL_W", 0xFFFF_8000, 0x0017_8000);
r3_insn!(SRA_W, "sra.w", "SRA_W", 0xFFFF_8000, 0x0018_0000);
r3_insn!(SLL_D, "sll.d", "SLL_D", 0xFFFF_8000, 0x0018_8000);
r3_insn!(SRL_D, "srl.d", "SRL_D", 0xFFFF_8000, 0x0019_0000);
r3_insn!(SRA_D, "sra.d", "SRA_D", 0xFFFF_8000, 0x0019_8000);
r3_insn!(ROTR_B, "rotr.b", "ROTR_B", 0xFFFF_8000, 0x001A_0000);
r3_insn!(ROTR_H, "rotr.h", "ROTR_H", 0xFFFF_8000, 0x001A_8000);
r3_insn!(ROTR_W, "rotr.w", "ROTR_W", 0xFFFF_8000, 0x001B_0000);
r3_insn!(ROTR_D, "rotr.d", "ROTR_D", 0xFFFF_8000, 0x001B_8000);
r3_insn!(ADC_B, "adc.b", "ADC_B", 0xFFFF_8000, 0x0030_0000);
r3_insn!(ADC_H, "adc.h", "ADC_H", 0xFFFF_8000, 0x0030_8000);
r3_insn!(ADC_W, "adc.w", "ADC_W", 0xFFFF_8000, 0x0031_0000);
r3_insn!(ADC_D, "adc.d", "ADC_D", 0xFFFF_8000, 0x0031_8000);
r3_insn!(SBC_B, "sbc.b", "SBC_B", 0xFFFF_8000, 0x0032_0000);
r3_insn!(SBC_H, "sbc.h", "SBC_H", 0xFFFF_8000, 0x0032_8000);
r3_insn!(SBC_W, "sbc.w", "SBC_W", 0xFFFF_8000, 0x0033_0000);
r3_insn!(SBC_D, "sbc.d", "SBC_D", 0xFFFF_8000, 0x0033_8000);
r3_insn!(RCR_B, "rcr.b", "RCR_B", 0xFFFF_8000, 0x0034_0000);
r3_insn!(RCR_H, "rcr.h", "RCR_H", 0xFFFF_8000, 0x0034_8000);
r3_insn!(RCR_W, "rcr.w", "RCR_W", 0xFFFF_8000, 0x0035_0000);
r3_insn!(RCR_D, "rcr.d", "RCR_D", 0xFFFF_8000, 0x0035_8000);

// Shift immediate (R2I5 / R2I6)
macro_rules! shift_imm5_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I5,
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
                    LoongArchField::Ui5,
                    ImmediateTransform::ZeroExtend { bits: 5 },
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Shift]
        );
    };
}
macro_rules! shift_imm6_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I6,
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
                    LoongArchField::Ui6,
                    ImmediateTransform::ZeroExtend { bits: 6 },
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Shift]
        );
    };
}

shift_imm5_insn!(SLLI_W, "slli.w", "SLLI_W", 0xFFFF_8000, 0x0040_8000);
shift_imm6_insn!(SLLI_D, "slli.d", "SLLI_D", 0xFFFF_0000, 0x0041_0000);
shift_imm5_insn!(SRLI_W, "srli.w", "SRLI_W", 0xFFFF_8000, 0x0044_8000);
shift_imm6_insn!(SRLI_D, "srli.d", "SRLI_D", 0xFFFF_0000, 0x0045_0000);
shift_imm5_insn!(SRAI_W, "srai.w", "SRAI_W", 0xFFFF_8000, 0x0048_8000);
shift_imm6_insn!(SRAI_D, "srai.d", "SRAI_D", 0xFFFF_0000, 0x0049_0000);

// Rotate immediate (R2I3 / R2I4 / R2I5 / R2I6)
macro_rules! rot_imm_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $format:expr, $field:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            $format,
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
                    $field,
                    ImmediateTransform::ZeroExtend { bits: 6 },
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Shift]
        );
    };
}

rot_imm_insn!(
    ROTRI_B,
    "rotri.b",
    "ROTRI_B",
    0xFFFF_E000,
    0x004C_2000,
    &FMT_R2I3,
    LoongArchField::Ui5
);
rot_imm_insn!(
    ROTRI_H,
    "rotri.h",
    "ROTRI_H",
    0xFFFF_C000,
    0x004C_4000,
    &FMT_R2I4,
    LoongArchField::Ui5
);
rot_imm_insn!(
    ROTRI_W,
    "rotri.w",
    "ROTRI_W",
    0xFFFF_8000,
    0x004C_8000,
    &FMT_R2I5,
    LoongArchField::Ui5
);
rot_imm_insn!(
    ROTRI_D,
    "rotri.d",
    "ROTRI_D",
    0xFFFF_0000,
    0x004D_0000,
    &FMT_R2I6,
    LoongArchField::Ui6
);
rot_imm_insn!(
    RCRI_B,
    "rcri.b",
    "RCRI_B",
    0xFFFF_E000,
    0x0050_2000,
    &FMT_R2I3,
    LoongArchField::Ui5
);
rot_imm_insn!(
    RCRI_H,
    "rcri.h",
    "RCRI_H",
    0xFFFF_C000,
    0x0050_4000,
    &FMT_R2I4,
    LoongArchField::Ui5
);
rot_imm_insn!(
    RCRI_W,
    "rcri.w",
    "RCRI_W",
    0xFFFF_8000,
    0x0050_8000,
    &FMT_R2I5,
    LoongArchField::Ui5
);
rot_imm_insn!(
    RCRI_D,
    "rcri.d",
    "RCRI_D",
    0xFFFF_0000,
    0x0051_0000,
    &FMT_R2I6,
    LoongArchField::Ui6
);

// ALU instructions (R3)
r3_insn!(ADD_W, "add.w", "ADD_W", 0xFFFF_8000, 0x0010_0000);
r3_insn!(ADD_D, "add.d", "ADD_D", 0xFFFF_8000, 0x0010_8000);
r3_insn!(SUB_W, "sub.w", "SUB_W", 0xFFFF_8000, 0x0011_0000);
r3_insn!(SUB_D, "sub.d", "SUB_D", 0xFFFF_8000, 0x0011_8000);
r3_insn!(SLT, "slt", "SLT", 0xFFFF_8000, 0x0012_0000);
r3_insn!(SLTU, "sltu", "SLTU", 0xFFFF_8000, 0x0012_8000);
r3_insn!(MASKEQZ, "maskeqz", "MASKEQZ", 0xFFFF_8000, 0x0013_0000);
r3_insn!(MASKNEZ, "masknez", "MASKNEZ", 0xFFFF_8000, 0x0013_8000);
r3_insn!(AND, "and", "AND", 0xFFFF_8000, 0x0014_8000);
r3_insn!(OR, "or", "OR", 0xFFFF_8000, 0x0015_0000);
r3_insn!(XOR, "xor", "XOR", 0xFFFF_8000, 0x0015_8000);
r3_insn!(NOR, "nor", "NOR", 0xFFFF_8000, 0x0014_0000);
r3_insn!(ORN, "orn", "ORN", 0xFFFF_8000, 0x0016_0000);
r3_insn!(ANDN, "andn", "ANDN", 0xFFFF_8000, 0x0016_8000);

// Bit manipulation / extend (R2)
macro_rules! r2_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2,
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
        );
    };
}

r2_insn!(CLO_W, "clo.w", "CLO_W", 0xFFFF_FC00, 0x0000_1000);
r2_insn!(CLZ_W, "clz.w", "CLZ_W", 0xFFFF_FC00, 0x0000_1400);
r2_insn!(CTO_W, "cto.w", "CTO_W", 0xFFFF_FC00, 0x0000_1800);
r2_insn!(CTZ_W, "ctz.w", "CTZ_W", 0xFFFF_FC00, 0x0000_1C00);
r2_insn!(CLO_D, "clo.d", "CLO_D", 0xFFFF_FC00, 0x0000_2000);
r2_insn!(CLZ_D, "clz.d", "CLZ_D", 0xFFFF_FC00, 0x0000_2400);
r2_insn!(CTO_D, "cto.d", "CTO_D", 0xFFFF_FC00, 0x0000_2800);
r2_insn!(CTZ_D, "ctz.d", "CTZ_D", 0xFFFF_FC00, 0x0000_2C00);
r2_insn!(REVB_2H, "revb.2h", "REVB_2H", 0xFFFF_FC00, 0x0000_3000);
r2_insn!(REVB_4H, "revb.4h", "REVB_4H", 0xFFFF_FC00, 0x0000_3400);
r2_insn!(REVB_2W, "revb.2w", "REVB_2W", 0xFFFF_FC00, 0x0000_3800);
r2_insn!(REVB_D, "revb.d", "REVB_D", 0xFFFF_FC00, 0x0000_3C00);
r2_insn!(REVH_2W, "revh.2w", "REVH_2W", 0xFFFF_FC00, 0x0000_4000);
r2_insn!(REVH_D, "revh.d", "REVH_D", 0xFFFF_FC00, 0x0000_4400);
r2_insn!(
    BITREV_4B,
    "bitrev.4b",
    "BITREV_4B",
    0xFFFF_FC00,
    0x0000_4800
);
r2_insn!(
    BITREV_8B,
    "bitrev.8b",
    "BITREV_8B",
    0xFFFF_FC00,
    0x0000_4C00
);
r2_insn!(BITREV_W, "bitrev.w", "BITREV_W", 0xFFFF_FC00, 0x0000_5000);
r2_insn!(BITREV_D, "bitrev.d", "BITREV_D", 0xFFFF_FC00, 0x0000_5400);
r2_insn!(EXT_W_H, "ext.w.h", "EXT_W_H", 0xFFFF_FC00, 0x0000_5800);
r2_insn!(EXT_W_B, "ext.w.b", "EXT_W_B", 0xFFFF_FC00, 0x0000_5C00);
r2_insn!(ASRTLE_D, "asrtle.d", "ASRTLE_D", 0xFFFF_FC00, 0x0001_0000);
r2_insn!(ASRTGT_D, "asrtgt.d", "ASRTGT_D", 0xFFFF_FC00, 0x0001_8000);

// Bit-field insert / pick
loongarch_insn!(
    BSTRINS_W,
    "bstrins.w",
    "BSTRINS_W",
    0xFFE0_8000,
    0x0060_0000,
    &FMT_BSTR_W,
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
            LoongArchField::MsbW,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
        robustone_isa::imm!(
            LoongArchField::LsbW,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
);
loongarch_insn!(
    BSTRPICK_W,
    "bstrpick.w",
    "BSTRPICK_W",
    0xFFE0_8000,
    0x0060_8000,
    &FMT_BSTR_W,
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
            LoongArchField::MsbW,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
        robustone_isa::imm!(
            LoongArchField::LsbW,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
);
loongarch_insn!(
    BSTRINS_D,
    "bstrins.d",
    "BSTRINS_D",
    0xFFC0_0000,
    0x0080_0000,
    &FMT_BSTR_D,
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
            LoongArchField::MsbD,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
        robustone_isa::imm!(
            LoongArchField::LsbD,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
);
loongarch_insn!(
    BSTRPICK_D,
    "bstrpick.d",
    "BSTRPICK_D",
    0xFFC0_0000,
    0x00C0_0000,
    &FMT_BSTR_D,
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
            LoongArchField::MsbD,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
        robustone_isa::imm!(
            LoongArchField::LsbD,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
);

// Multiply / Divide / Modulo (R3)
// ALSL (R3I2)
macro_rules! alsl_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R3I2,
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
                robustone_isa::imm!(
                    LoongArchField::Ui5,
                    ImmediateTransform::AddConst { value: 1 },
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Shift]
        );
    };
}

alsl_insn!(ALSL_W, "alsl.w", "ALSL_W", 0xFFFE_0000, 0x0004_0000);
alsl_insn!(ALSL_WU, "alsl.wu", "ALSL_WU", 0xFFFE_0000, 0x0006_0000);
alsl_insn!(ALSL_D, "alsl.d", "ALSL_D", 0xFFFE_0000, 0x002C_0000);

// Bytepick (R3I2 / R3I3)
macro_rules! bytepick_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $format:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            $format,
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
                robustone_isa::imm!(
                    LoongArchField::Ui5,
                    ImmediateTransform::None,
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::BitManipulation]
        );
    };
}

bytepick_insn!(
    BYTEPICK_W,
    "bytepick.w",
    "BYTEPICK_W",
    0xFFFE_0000,
    0x0008_0000,
    &FMT_R3I2
);
bytepick_insn!(
    BYTEPICK_D,
    "bytepick.d",
    "BYTEPICK_D",
    0xFFFC_0000,
    0x000C_0000,
    &FMT_R3I3
);

r3_insn!(MUL_W, "mul.w", "MUL_W", 0xFFFF_8000, 0x001C_0000);
r3_insn!(MULH_W, "mulh.w", "MULH_W", 0xFFFF_8000, 0x001C_8000);
r3_insn!(MULH_WU, "mulh.wu", "MULH_WU", 0xFFFF_8000, 0x001D_0000);
r3_insn!(MUL_D, "mul.d", "MUL_D", 0xFFFF_8000, 0x001D_8000);
r3_insn!(MULH_D, "mulh.d", "MULH_D", 0xFFFF_8000, 0x001E_0000);
r3_insn!(MULH_DU, "mulh.du", "MULH_DU", 0xFFFF_8000, 0x001E_8000);
r3_insn!(MULW_D_W, "mulw.d.w", "MULW_D_W", 0xFFFF_8000, 0x001F_0000);
r3_insn!(
    MULW_D_WU,
    "mulw.d.wu",
    "MULW_D_WU",
    0xFFFF_8000,
    0x001F_8000
);
r3_insn!(DIV_W, "div.w", "DIV_W", 0xFFFF_8000, 0x0020_0000);
r3_insn!(MOD_W, "mod.w", "MOD_W", 0xFFFF_8000, 0x0020_8000);
r3_insn!(DIV_WU, "div.wu", "DIV_WU", 0xFFFF_8000, 0x0021_0000);
r3_insn!(MOD_WU, "mod.wu", "MOD_WU", 0xFFFF_8000, 0x0021_8000);
r3_insn!(DIV_D, "div.d", "DIV_D", 0xFFFF_8000, 0x0022_0000);
r3_insn!(MOD_D, "mod.d", "MOD_D", 0xFFFF_8000, 0x0022_8000);
r3_insn!(DIV_DU, "div.du", "DIV_DU", 0xFFFF_8000, 0x0023_0000);
r3_insn!(MOD_DU, "mod.du", "MOD_DU", 0xFFFF_8000, 0x0023_8000);

// Immediate ALU (R2I12)
r2i12_insn!(ADDI_W, "addi.w", "ADDI_W", 0xFFC0_0000, 0x0280_0000);
r2i12_insn!(ADDI_D, "addi.d", "ADDI_D", 0xFFC0_0000, 0x02C0_0000);

// addu16i.d (R2I16)
loongarch_insn!(
    ADDU16I_D,
    "addu16i.d",
    "ADDU16I_D",
    0xFC00_0000,
    0x1000_0000,
    &FMT_R2I16,
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
            LoongArchField::Si16,
            ImmediateTransform::SignExtend { bits: 16 },
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::Arithmetic]
);

r2i12_insn!(SLTI, "slti", "SLTI", 0xFFC0_0000, 0x0200_0000);
r2i12_insn!(SLTUI, "sltui", "SLTUI", 0xFFC0_0000, 0x0240_0000);

// Logical immediate (R2I12_U)
macro_rules! logical_imm_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I12_U,
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
                    LoongArchField::Ui12,
                    ImmediateTransform::ZeroExtend { bits: 12 },
                    ImmediateKind::Unsigned
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Logical]
        );
    };
}

logical_imm_insn!(ANDI, "andi", "ANDI", 0xFFC0_0000, 0x0340_0000);
logical_imm_insn!(ORI, "ori", "ORI", 0xFFC0_0000, 0x0380_0000);
logical_imm_insn!(XORI, "xori", "XORI", 0xFFC0_0000, 0x03C0_0000);

// addu12i (R2I5)
macro_rules! addu12i_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I5,
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
                    LoongArchField::Ui5,
                    ImmediateTransform::SignExtend { bits: 5 },
                    ImmediateKind::Absolute
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Arithmetic]
        );
    };
}

addu12i_insn!(
    ADDU12I_W,
    "addu12i.w",
    "ADDU12I_W",
    0xFFFF_8000,
    0x0029_0000
);
addu12i_insn!(
    ADDU12I_D,
    "addu12i.d",
    "ADDU12I_D",
    0xFFFF_8000,
    0x0029_8000
);

// Branch compare (R2I16) — operand order: rj, rd, offset
macro_rules! branch_r2i16_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I16,
            &[
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rj,
                    Access::Read
                ),
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rd,
                    Access::Read
                ),
                robustone_isa::imm!(
                    LoongArchField::Si16,
                    ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
                    ImmediateKind::PcRelative
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Branch]
        );
    };
}

branch_r2i16_insn!(BEQ, "beq", "BEQ", 0xFC00_0000, 0x5800_0000);
branch_r2i16_insn!(BNE, "bne", "BNE", 0xFC00_0000, 0x5C00_0000);
branch_r2i16_insn!(BLT, "blt", "BLT", 0xFC00_0000, 0x6000_0000);
branch_r2i16_insn!(BGE, "bge", "BGE", 0xFC00_0000, 0x6400_0000);
branch_r2i16_insn!(BLTU, "bltu", "BLTU", 0xFC00_0000, 0x6800_0000);
branch_r2i16_insn!(BGEU, "bgeu", "BGEU", 0xFC00_0000, 0x6C00_0000);

// JIRL (R2I16) — operand order: rd, rj, offset
loongarch_insn!(
    JIRL,
    "jirl",
    "JIRL",
    0xFC00_0000,
    0x4C00_0000,
    &FMT_R2I16,
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
            LoongArchField::Si16,
            ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
            ImmediateKind::PcRelative
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::Branch]
);

// B / BL (FMT_BJ) — operand: 16-bit immediate at bits 10..25, shifted left by 2
loongarch_insn!(
    B,
    "b",
    "B",
    0xFC00_0000,
    0x5000_0000,
    &FMT_BJ,
    &[robustone_isa::imm!(
        LoongArchField::Si16,
        ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
        ImmediateKind::PcRelative
    ),],
    &[InstructionGroup::Integer, InstructionGroup::Branch]
);
loongarch_insn!(
    BL,
    "bl",
    "BL",
    0xFC00_0000,
    0x5400_0000,
    &FMT_BJ,
    &[robustone_isa::imm!(
        LoongArchField::Si16,
        ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
        ImmediateKind::PcRelative
    ),],
    &[InstructionGroup::Integer, InstructionGroup::Branch]
);

// BEQZ / BNEZ (FMT_R1I16_BJ) — operand order: rj, offset
loongarch_insn!(
    BEQZ,
    "beqz",
    "BEQZ",
    0xFC00_0000,
    0x4000_0000,
    &FMT_R1I16_BJ,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::imm!(
            LoongArchField::Si16,
            ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
            ImmediateKind::PcRelative
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::Branch]
);
loongarch_insn!(
    BNEZ,
    "bnez",
    "BNEZ",
    0xFC00_0000,
    0x4400_0000,
    &FMT_R1I16_BJ,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::imm!(
            LoongArchField::Si16,
            ImmediateTransform::SignExtendThenShift { bits: 16, shift: 2 },
            ImmediateKind::PcRelative
        ),
    ],
    &[InstructionGroup::Integer, InstructionGroup::Branch]
);

// Memory load / store (R2I12)
macro_rules! load_r2i12_insn {
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Memory]
        );
    };
}
macro_rules! store_r2i12_insn {
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
                    Access::Read
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Memory]
        );
    };
}

load_r2i12_insn!(LD_B, "ld.b", "LD_B", 0xFFC0_0000, 0x2800_0000);
load_r2i12_insn!(LD_H, "ld.h", "LD_H", 0xFFC0_0000, 0x2840_0000);
load_r2i12_insn!(LD_W, "ld.w", "LD_W", 0xFFC0_0000, 0x2880_0000);
load_r2i12_insn!(LD_D, "ld.d", "LD_D", 0xFFC0_0000, 0x28C0_0000);

store_r2i12_insn!(ST_B, "st.b", "ST_B", 0xFFC0_0000, 0x2900_0000);
store_r2i12_insn!(ST_H, "st.h", "ST_H", 0xFFC0_0000, 0x2940_0000);
store_r2i12_insn!(ST_W, "st.w", "ST_W", 0xFFC0_0000, 0x2980_0000);
store_r2i12_insn!(ST_D, "st.d", "ST_D", 0xFFC0_0000, 0x29C0_0000);

// Memory indexed (R3) — operand order: rd, rj, rk
macro_rules! ldx_r3_insn {
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Memory]
        );
    };
}
macro_rules! stx_r3_insn {
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
                    Access::Read
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
            ],
            &[InstructionGroup::Integer, InstructionGroup::Memory]
        );
    };
}

ldx_r3_insn!(LDX_B, "ldx.b", "LDX_B", 0xFFFF_8000, 0x3800_0000);
ldx_r3_insn!(LDX_H, "ldx.h", "LDX_H", 0xFFFF_8000, 0x3804_0000);
ldx_r3_insn!(LDX_W, "ldx.w", "LDX_W", 0xFFFF_8000, 0x3808_0000);
ldx_r3_insn!(LDX_D, "ldx.d", "LDX_D", 0xFFFF_8000, 0x380C_0000);

stx_r3_insn!(STX_B, "stx.b", "STX_B", 0xFFFF_8000, 0x3810_0000);
stx_r3_insn!(STX_H, "stx.h", "STX_H", 0xFFFF_8000, 0x3814_0000);
stx_r3_insn!(STX_W, "stx.w", "STX_W", 0xFFFF_8000, 0x3818_0000);
stx_r3_insn!(STX_D, "stx.d", "STX_D", 0xFFFF_8000, 0x381C_0000);

// Atomic LL / SC (R2I14)
macro_rules! atomic_r2i14_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_R2I14,
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
                    LoongArchField::Si14,
                    ImmediateTransform::SignExtendThenShift { bits: 14, shift: 2 },
                    ImmediateKind::Absolute
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Atomic]
        );
    };
}

atomic_r2i14_insn!(LL_W, "ll.w", "LL_W", 0xFF00_0000, 0x2000_0000);
atomic_r2i14_insn!(SC_W, "sc.w", "SC_W", 0xFF00_0000, 0x2100_0000);

// Atomic RMW (R3) — operand order: rd, rk, rj (matching Capstonen atomic layout)
macro_rules! atomic_r3_insn {
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
                    LoongArchField::Rk,
                    Access::Read
                ),
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rj,
                    Access::Read
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Atomic]
        );
    };
}

atomic_r3_insn!(AMSWAP_W, "amswap.w", "AMSWAP_W", 0xFFFF_8000, 0x3860_0000);
atomic_r3_insn!(AMADD_W, "amadd.w", "AMADD_W", 0xFFFF_8000, 0x3861_0000);
atomic_r3_insn!(AMAND_W, "amand.w", "AMAND_W", 0xFFFF_8000, 0x3862_0000);
atomic_r3_insn!(AMOR_W, "amor.w", "AMOR_W", 0xFFFF_8000, 0x3863_0000);
atomic_r3_insn!(AMXOR_W, "amxor.w", "AMXOR_W", 0xFFFF_8000, 0x3864_0000);
atomic_r3_insn!(AMMAX_W, "ammax.w", "AMMAX_W", 0xFFFF_8000, 0x3865_0000);
atomic_r3_insn!(AMMIN_W, "ammin.w", "AMMIN_W", 0xFFFF_8000, 0x3866_0000);

// Barrier / system (FMT_I15)
macro_rules! barrier_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_I15,
            &[robustone_isa::imm!(
                LoongArchField::Code,
                ImmediateTransform::ZeroExtend { bits: 15 },
                ImmediateKind::Unsigned
            ),],
            &[InstructionGroup::Integer, InstructionGroup::Barrier]
        );
    };
}

barrier_insn!(DBAR, "dbar", "DBAR", 0xFFFF_8000, 0x3872_0000);
barrier_insn!(IBAR, "ibar", "IBAR", 0xFFFF_8000, 0x3872_8000);
barrier_insn!(SYSCALL, "syscall", "SYSCALL", 0xFFFF_8000, 0x002B_0000);
barrier_insn!(BREAK, "break", "BREAK", 0xFFFF_8000, 0x002A_0000);

// Upper immediate (1RI20)
macro_rules! upper_imm_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr) => {
        loongarch_insn!(
            $name,
            $mnemonic,
            $opcode_id,
            $mask,
            $value,
            &FMT_1RI20,
            &[
                robustone_isa::reg!(
                    LoongArchRegisterClass::Gpr,
                    LoongArchField::Rd,
                    Access::Write
                ),
                robustone_isa::imm!(
                    LoongArchField::Si20,
                    ImmediateTransform::SignExtend { bits: 20 },
                    ImmediateKind::Absolute
                ),
            ],
            &[InstructionGroup::Integer, InstructionGroup::Arithmetic]
        );
    };
}

// Float arithmetic
loongarch_insn!(
    FABS_D,
    "fabs.d",
    "FABS_D",
    0xFFFFFC00,
    0x01140800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FABS_S,
    "fabs.s",
    "FABS_S",
    0xFFFFFC00,
    0x01140400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FADD_D,
    "fadd.d",
    "FADD_D",
    0xFFFF8000,
    0x01010000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FADD_S,
    "fadd.s",
    "FADD_S",
    0xFFFF8000,
    0x01008000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCLASS_D,
    "fclass.d",
    "FCLASS_D",
    0xFFFFFC00,
    0x01143800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCLASS_S,
    "fclass.s",
    "FCLASS_S",
    0xFFFFFC00,
    0x01143400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCOPYSIGN_D,
    "fcopysign.d",
    "FCOPYSIGN_D",
    0xFFFF8000,
    0x01130000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCOPYSIGN_S,
    "fcopysign.s",
    "FCOPYSIGN_S",
    0xFFFF8000,
    0x01128000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FDIV_D,
    "fdiv.d",
    "FDIV_D",
    0xFFFF8000,
    0x01070000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FDIV_S,
    "fdiv.s",
    "FDIV_S",
    0xFFFF8000,
    0x01068000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FLOGB_D,
    "flogb.d",
    "FLOGB_D",
    0xFFFFFC00,
    0x01142800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FLOGB_S,
    "flogb.s",
    "FLOGB_S",
    0xFFFFFC00,
    0x01142400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMADD_D,
    "fmadd.d",
    "FMADD_D",
    0xFFF00000,
    0x08200000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMADD_S,
    "fmadd.s",
    "FMADD_S",
    0xFFF00000,
    0x08100000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMAX_D,
    "fmax.d",
    "FMAX_D",
    0xFFFF8000,
    0x01090000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMAX_S,
    "fmax.s",
    "FMAX_S",
    0xFFFF8000,
    0x01088000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMAXA_D,
    "fmaxa.d",
    "FMAXA_D",
    0xFFFF8000,
    0x010D0000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMAXA_S,
    "fmaxa.s",
    "FMAXA_S",
    0xFFFF8000,
    0x010C8000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMIN_D,
    "fmin.d",
    "FMIN_D",
    0xFFFF8000,
    0x010B0000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMIN_S,
    "fmin.s",
    "FMIN_S",
    0xFFFF8000,
    0x010A8000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMINA_D,
    "fmina.d",
    "FMINA_D",
    0xFFFF8000,
    0x010F0000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMINA_S,
    "fmina.s",
    "FMINA_S",
    0xFFFF8000,
    0x010E8000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMSUB_D,
    "fmsub.d",
    "FMSUB_D",
    0xFFF00000,
    0x08600000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMSUB_S,
    "fmsub.s",
    "FMSUB_S",
    0xFFF00000,
    0x08500000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMUL_D,
    "fmul.d",
    "FMUL_D",
    0xFFFF8000,
    0x01050000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FMUL_S,
    "fmul.s",
    "FMUL_S",
    0xFFFF8000,
    0x01048000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNEG_D,
    "fneg.d",
    "FNEG_D",
    0xFFFFFC00,
    0x01141800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNEG_S,
    "fneg.s",
    "FNEG_S",
    0xFFFFFC00,
    0x01141400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNMADD_D,
    "fnmadd.d",
    "FNMADD_D",
    0xFFF00000,
    0x08A00000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNMADD_S,
    "fnmadd.s",
    "FNMADD_S",
    0xFFF00000,
    0x08900000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNMSUB_D,
    "fnmsub.d",
    "FNMSUB_D",
    0xFFF00000,
    0x08E00000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FNMSUB_S,
    "fnmsub.s",
    "FNMSUB_S",
    0xFFF00000,
    0x08D00000,
    &FMT_R4,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Ra,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRECIP_D,
    "frecip.d",
    "FRECIP_D",
    0xFFFFFC00,
    0x01145800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRECIP_S,
    "frecip.s",
    "FRECIP_S",
    0xFFFFFC00,
    0x01145400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRECIPE_D,
    "frecipe.d",
    "FRECIPE_D",
    0xFFFFFC00,
    0x01147800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRECIPE_S,
    "frecipe.s",
    "FRECIPE_S",
    0xFFFFFC00,
    0x01147400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRINT_D,
    "frint.d",
    "FRINT_D",
    0xFFFFFC00,
    0x011E4800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRINT_S,
    "frint.s",
    "FRINT_S",
    0xFFFFFC00,
    0x011E4400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRSQRT_D,
    "frsqrt.d",
    "FRSQRT_D",
    0xFFFFFC00,
    0x01146800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRSQRT_S,
    "frsqrt.s",
    "FRSQRT_S",
    0xFFFFFC00,
    0x01146400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRSQRTE_D,
    "frsqrte.d",
    "FRSQRTE_D",
    0xFFFFFC00,
    0x01148800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FRSQRTE_S,
    "frsqrte.s",
    "FRSQRTE_S",
    0xFFFFFC00,
    0x01148400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSCALEB_D,
    "fscaleb.d",
    "FSCALEB_D",
    0xFFFF8000,
    0x01110000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSCALEB_S,
    "fscaleb.s",
    "FSCALEB_S",
    0xFFFF8000,
    0x01108000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSQRT_D,
    "fsqrt.d",
    "FSQRT_D",
    0xFFFFFC00,
    0x01144800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSQRT_S,
    "fsqrt.s",
    "FSQRT_S",
    0xFFFFFC00,
    0x01144400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSUB_D,
    "fsub.d",
    "FSUB_D",
    0xFFFF8000,
    0x01030000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FSUB_S,
    "fsub.s",
    "FSUB_S",
    0xFFFF8000,
    0x01028000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CAF_D,
    "fcmp.caf.d",
    "FCMP_CAF_D",
    0xFFFF8018,
    0x0C200000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CAF_S,
    "fcmp.caf.s",
    "FCMP_CAF_S",
    0xFFFF8018,
    0x0C100000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CEQ_D,
    "fcmp.ceq.d",
    "FCMP_CEQ_D",
    0xFFFF8018,
    0x0C220000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CEQ_S,
    "fcmp.ceq.s",
    "FCMP_CEQ_S",
    0xFFFF8018,
    0x0C120000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CLE_D,
    "fcmp.cle.d",
    "FCMP_CLE_D",
    0xFFFF8018,
    0x0C230000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CLE_S,
    "fcmp.cle.s",
    "FCMP_CLE_S",
    0xFFFF8018,
    0x0C130000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CLT_D,
    "fcmp.clt.d",
    "FCMP_CLT_D",
    0xFFFF8018,
    0x0C210000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CLT_S,
    "fcmp.clt.s",
    "FCMP_CLT_S",
    0xFFFF8018,
    0x0C110000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CNE_D,
    "fcmp.cne.d",
    "FCMP_CNE_D",
    0xFFFF8018,
    0x0C280000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CNE_S,
    "fcmp.cne.s",
    "FCMP_CNE_S",
    0xFFFF8018,
    0x0C180000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_COR_D,
    "fcmp.cor.d",
    "FCMP_COR_D",
    0xFFFF8018,
    0x0C2A0000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_COR_S,
    "fcmp.cor.s",
    "FCMP_COR_S",
    0xFFFF8018,
    0x0C1A0000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUEQ_D,
    "fcmp.cueq.d",
    "FCMP_CUEQ_D",
    0xFFFF8018,
    0x0C260000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUEQ_S,
    "fcmp.cueq.s",
    "FCMP_CUEQ_S",
    0xFFFF8018,
    0x0C160000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CULE_D,
    "fcmp.cule.d",
    "FCMP_CULE_D",
    0xFFFF8018,
    0x0C270000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CULE_S,
    "fcmp.cule.s",
    "FCMP_CULE_S",
    0xFFFF8018,
    0x0C170000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CULT_D,
    "fcmp.cult.d",
    "FCMP_CULT_D",
    0xFFFF8018,
    0x0C250000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CULT_S,
    "fcmp.cult.s",
    "FCMP_CULT_S",
    0xFFFF8018,
    0x0C150000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUN_D,
    "fcmp.cun.d",
    "FCMP_CUN_D",
    0xFFFF8018,
    0x0C240000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUN_S,
    "fcmp.cun.s",
    "FCMP_CUN_S",
    0xFFFF8018,
    0x0C140000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUNE_D,
    "fcmp.cune.d",
    "FCMP_CUNE_D",
    0xFFFF8018,
    0x0C2C0000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_CUNE_S,
    "fcmp.cune.s",
    "FCMP_CUNE_S",
    0xFFFF8018,
    0x0C1C0000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SAF_D,
    "fcmp.saf.d",
    "FCMP_SAF_D",
    0xFFFF8018,
    0x0C208000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SAF_S,
    "fcmp.saf.s",
    "FCMP_SAF_S",
    0xFFFF8018,
    0x0C108000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SEQ_D,
    "fcmp.seq.d",
    "FCMP_SEQ_D",
    0xFFFF8018,
    0x0C228000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SEQ_S,
    "fcmp.seq.s",
    "FCMP_SEQ_S",
    0xFFFF8018,
    0x0C128000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SLE_D,
    "fcmp.sle.d",
    "FCMP_SLE_D",
    0xFFFF8018,
    0x0C238000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SLE_S,
    "fcmp.sle.s",
    "FCMP_SLE_S",
    0xFFFF8018,
    0x0C138000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SLT_D,
    "fcmp.slt.d",
    "FCMP_SLT_D",
    0xFFFF8018,
    0x0C218000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SLT_S,
    "fcmp.slt.s",
    "FCMP_SLT_S",
    0xFFFF8018,
    0x0C118000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SNE_D,
    "fcmp.sne.d",
    "FCMP_SNE_D",
    0xFFFF8018,
    0x0C288000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SNE_S,
    "fcmp.sne.s",
    "FCMP_SNE_S",
    0xFFFF8018,
    0x0C188000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SOR_D,
    "fcmp.sor.d",
    "FCMP_SOR_D",
    0xFFFF8018,
    0x0C2A8000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SOR_S,
    "fcmp.sor.s",
    "FCMP_SOR_S",
    0xFFFF8018,
    0x0C1A8000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUEQ_D,
    "fcmp.sueq.d",
    "FCMP_SUEQ_D",
    0xFFFF8018,
    0x0C268000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUEQ_S,
    "fcmp.sueq.s",
    "FCMP_SUEQ_S",
    0xFFFF8018,
    0x0C168000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SULE_D,
    "fcmp.sule.d",
    "FCMP_SULE_D",
    0xFFFF8018,
    0x0C278000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SULE_S,
    "fcmp.sule.s",
    "FCMP_SULE_S",
    0xFFFF8018,
    0x0C178000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SULT_D,
    "fcmp.sult.d",
    "FCMP_SULT_D",
    0xFFFF8018,
    0x0C258000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SULT_S,
    "fcmp.sult.s",
    "FCMP_SULT_S",
    0xFFFF8018,
    0x0C158000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUN_D,
    "fcmp.sun.d",
    "FCMP_SUN_D",
    0xFFFF8018,
    0x0C248000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUN_S,
    "fcmp.sun.s",
    "FCMP_SUN_S",
    0xFFFF8018,
    0x0C148000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUNE_D,
    "fcmp.sune.d",
    "FCMP_SUNE_D",
    0xFFFF8018,
    0x0C2C8000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCMP_SUNE_S,
    "fcmp.sune.s",
    "FCMP_SUNE_S",
    0xFFFF8018,
    0x0C1C8000,
    &FMT_FCMP,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fcc,
            LoongArchField::Cd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCVT_D_LD,
    "fcvt.d.ld",
    "FCVT_D_LD",
    0xFFFF8000,
    0x01150000,
    &FMT_R3,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rk,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCVT_D_S,
    "fcvt.d.s",
    "FCVT_D_S",
    0xFFFFFC00,
    0x01192400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCVT_LD_D,
    "fcvt.ld.d",
    "FCVT_LD_D",
    0xFFFFFC00,
    0x0114E000,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCVT_S_D,
    "fcvt.s.d",
    "FCVT_S_D",
    0xFFFFFC00,
    0x01191800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FCVT_UD_D,
    "fcvt.ud.d",
    "FCVT_UD_D",
    0xFFFFFC00,
    0x0114E400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FFINT_D_L,
    "ffint.d.l",
    "FFINT_D_L",
    0xFFFFFC00,
    0x011D2800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FFINT_D_W,
    "ffint.d.w",
    "FFINT_D_W",
    0xFFFFFC00,
    0x011D2000,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FFINT_S_L,
    "ffint.s.l",
    "FFINT_S_L",
    0xFFFFFC00,
    0x011D1800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FFINT_S_W,
    "ffint.s.w",
    "FFINT_S_W",
    0xFFFFFC00,
    0x011D1000,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINT_L_D,
    "ftint.l.d",
    "FTINT_L_D",
    0xFFFFFC00,
    0x011B2800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINT_L_S,
    "ftint.l.s",
    "FTINT_L_S",
    0xFFFFFC00,
    0x011B2400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINT_W_D,
    "ftint.w.d",
    "FTINT_W_D",
    0xFFFFFC00,
    0x011B0800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINT_W_S,
    "ftint.w.s",
    "FTINT_W_S",
    0xFFFFFC00,
    0x011B0400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRM_L_D,
    "ftintrm.l.d",
    "FTINTRM_L_D",
    0xFFFFFC00,
    0x011A2800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRM_L_S,
    "ftintrm.l.s",
    "FTINTRM_L_S",
    0xFFFFFC00,
    0x011A2400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRM_W_D,
    "ftintrm.w.d",
    "FTINTRM_W_D",
    0xFFFFFC00,
    0x011A0800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRM_W_S,
    "ftintrm.w.s",
    "FTINTRM_W_S",
    0xFFFFFC00,
    0x011A0400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRNE_L_D,
    "ftintrne.l.d",
    "FTINTRNE_L_D",
    0xFFFFFC00,
    0x011AE800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRNE_L_S,
    "ftintrne.l.s",
    "FTINTRNE_L_S",
    0xFFFFFC00,
    0x011AE400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRNE_W_D,
    "ftintrne.w.d",
    "FTINTRNE_W_D",
    0xFFFFFC00,
    0x011AC800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRNE_W_S,
    "ftintrne.w.s",
    "FTINTRNE_W_S",
    0xFFFFFC00,
    0x011AC400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRP_L_D,
    "ftintrp.l.d",
    "FTINTRP_L_D",
    0xFFFFFC00,
    0x011A6800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRP_L_S,
    "ftintrp.l.s",
    "FTINTRP_L_S",
    0xFFFFFC00,
    0x011A6400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRP_W_D,
    "ftintrp.w.d",
    "FTINTRP_W_D",
    0xFFFFFC00,
    0x011A4800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRP_W_S,
    "ftintrp.w.s",
    "FTINTRP_W_S",
    0xFFFFFC00,
    0x011A4400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRZ_L_D,
    "ftintrz.l.d",
    "FTINTRZ_L_D",
    0xFFFFFC00,
    0x011AA800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRZ_L_S,
    "ftintrz.l.s",
    "FTINTRZ_L_S",
    0xFFFFFC00,
    0x011AA400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRZ_W_D,
    "ftintrz.w.d",
    "FTINTRZ_W_D",
    0xFFFFFC00,
    0x011A8800,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

loongarch_insn!(
    FTINTRZ_W_S,
    "ftintrz.w.s",
    "FTINTRZ_W_S",
    0xFFFFFC00,
    0x011A8400,
    &FMT_R2,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::reg!(
            LoongArchRegisterClass::Fpr,
            LoongArchField::Rj,
            Access::Read
        ),
    ],
    &[InstructionGroup::Float, InstructionGroup::Arithmetic]
);

upper_imm_insn!(LU12I_W, "lu12i.w", "LU12I_W", 0xFE00_0000, 0x1400_0000);
upper_imm_insn!(PCADDI, "pcaddi", "PCADDI", 0xFE00_0000, 0x1800_0000);
upper_imm_insn!(
    PCALAU12I,
    "pcalau12i",
    "PCALAU12I",
    0xFE00_0000,
    0x1A00_0000
);
upper_imm_insn!(
    PCADDU12I,
    "pcaddu12i",
    "PCADDU12I",
    0xFE00_0000,
    0x1C00_0000
);

// ============================================================================
// Spec table
// ============================================================================

pub static LOONGARCH_BASE_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    // ALU (R3)
    ADD_W, ADD_D, SUB_W, SUB_D, SLT, SLTU, MASKEQZ, MASKNEZ, AND, OR, XOR, NOR, ORN, ANDN,
    // Bit manipulation
    CLO_W, CLZ_W, CTO_W, CTZ_W, CLO_D, CLZ_D, CTO_D, CTZ_D, REVB_2H, REVB_4H, REVB_2W, REVB_D,
    REVH_2W, REVH_D, BITREV_4B, BITREV_8B, BITREV_W, BITREV_D, EXT_W_H, EXT_W_B, ASRTLE_D,
    ASRTGT_D, BSTRINS_W, BSTRPICK_W, BSTRINS_D, BSTRPICK_D, // Multiply / Divide
    ALSL_W, ALSL_WU, ALSL_D, BYTEPICK_W, BYTEPICK_D, MUL_W, MULH_W, MULH_WU, MUL_D, MULH_D,
    MULH_DU, MULW_D_W, MULW_D_WU, DIV_W, MOD_W, DIV_WU, MOD_WU, DIV_D, MOD_D, DIV_DU,
    MOD_DU, // Shift immediate
    SLLI_W, SLLI_D, SRLI_W, SRLI_D, SRAI_W, SRAI_D, ROTRI_B, ROTRI_H, ROTRI_W, ROTRI_D, RCRI_B,
    RCRI_H, RCRI_W, RCRI_D, // Shift (R3)
    SLL_W, SRL_W, SRA_W, SLL_D, SRL_D, SRA_D, ROTR_B, ROTR_H, ROTR_W, ROTR_D, ADC_B, ADC_H, ADC_W,
    ADC_D, SBC_B, SBC_H, SBC_W, SBC_D, RCR_B, RCR_H, RCR_W, RCR_D, // Immediate ALU
    ADDI_W, ADDI_D, ADDU16I_D, SLTI, SLTUI, ANDI, ORI, XORI, ADDU12I_W,
    ADDU12I_D, // Upper immediate
    LU12I_W, PCADDI, PCALAU12I, PCADDU12I, // Branch
    BEQ, BNE, BLT, BGE, BLTU, BGEU, B, BL, JIRL, BEQZ, BNEZ, // Memory
    LD_B, LD_H, LD_W, LD_D, ST_B, ST_H, ST_W, ST_D, LDX_B, LDX_H, LDX_W, LDX_D, STX_B, STX_H,
    STX_W, STX_D, // Atomic
    LL_W, SC_W, AMSWAP_W, AMADD_W, AMAND_W, AMOR_W, AMXOR_W, AMMAX_W, AMMIN_W, // Barrier
    DBAR, IBAR, SYSCALL, BREAK,
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
