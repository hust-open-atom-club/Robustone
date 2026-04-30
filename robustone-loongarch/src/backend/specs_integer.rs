// Base integer instruction specs
// ============================================================================

macro_rules! loongarch_insn {
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $fmt:expr, $operands:expr, $groups:expr) => {
        loongarch_insn!($name, $mnemonic, $opcode_id, $mask, $value, $fmt, $operands, $groups, 0);
    };
    ($name:ident, $mnemonic:expr, $opcode_id:expr, $mask:expr, $value:expr, $fmt:expr, $operands:expr, $groups:expr, $priority:expr) => {
        pub static $name: ::robustone_isa::InstructionSpec<LoongArchBackend> = ::robustone_isa::InstructionSpec {
            mnemonic: $mnemonic,
            opcode_id: $opcode_id,
            pattern: ::robustone_isa::mask_value!($mask, $value),
            format: $fmt,
            operands: $operands,
            features: LoongArchFeature::BASE,
            modes: ::robustone_isa::ModeSet::All,
            groups: $groups,
            manual_ref: Some("LoongArch Reference Manual"),
            priority: $priority,
        };
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
load_r2i12_insn!(LD_BU, "ld.bu", "LD_BU", 0xFFC0_0000, 0x2A00_0000);
load_r2i12_insn!(LD_HU, "ld.hu", "LD_HU", 0xFFC0_0000, 0x2A40_0000);
load_r2i12_insn!(LDL_W, "ldl.w", "LDL_W", 0xFFC0_0000, 0x2E00_0000);
load_r2i12_insn!(LDR_W, "ldr.w", "LDR_W", 0xFFC0_0000, 0x2E40_0000);
load_r2i12_insn!(LDL_D, "ldl.d", "LDL_D", 0xFFC0_0000, 0x2E80_0000);
load_r2i12_insn!(LDR_D, "ldr.d", "LDR_D", 0xFFC0_0000, 0x2EC0_0000);

store_r2i12_insn!(ST_B, "st.b", "ST_B", 0xFFC0_0000, 0x2900_0000);
store_r2i12_insn!(ST_H, "st.h", "ST_H", 0xFFC0_0000, 0x2940_0000);
store_r2i12_insn!(ST_W, "st.w", "ST_W", 0xFFC0_0000, 0x2980_0000);
store_r2i12_insn!(ST_D, "st.d", "ST_D", 0xFFC0_0000, 0x29C0_0000);
store_r2i12_insn!(STL_W, "stl.w", "STL_W", 0xFFC0_0000, 0x2F00_0000);
store_r2i12_insn!(STR_W, "str.w", "STR_W", 0xFFC0_0000, 0x2F40_0000);
store_r2i12_insn!(STL_D, "stl.d", "STL_D", 0xFFC0_0000, 0x2F80_0000);
store_r2i12_insn!(STR_D, "str.d", "STR_D", 0xFFC0_0000, 0x2FC0_0000);

loongarch_insn!(
    PRELD,
    "preld",
    "PRELD",
    0xFFC0_0000,
    0x2AC0_0000,
    &FMT_R2I12,
    &[
        robustone_isa::imm!(LoongArchField::Rd, ImmediateTransform::None, ImmediateKind::Unsigned),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(LoongArchField::Si12, ImmediateTransform::None, ImmediateKind::Unsigned),
    ],
    &[InstructionGroup::Memory]
);

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
r2_insn!(LLACQ_W, "llacq.w", "LLACQ_W", 0xFFFF_FC00, 0x3857_8000);
r2_insn!(SCREL_W, "screl.w", "SCREL_W", 0xFFFF_FC00, 0x3857_8400);

// Atomic RMW (R3) — operand order: rd, rk, rj (matching upstream atomic layout)
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
barrier_insn!(DBCL, "dbcl", "DBCL", 0xFFFF_8000, 0x002A_8000);
barrier_insn!(HVCL, "hvcl", "HVCL", 0xFFFF_8000, 0x002B_8000);

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
