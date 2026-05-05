// Auto-generated LASX/LSX vector instruction specs.

loongarch_insn!(
    VEXT2XV_D_B,
    "vext2xv.d.b",
    "VEXT2XV_D_B",
    0xFFFFFFFF,
    0x769F1B39,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_D_H,
    "vext2xv.d.h",
    "VEXT2XV_D_H",
    0xFFFFFFFF,
    0x769F2268,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_D_W,
    "vext2xv.d.w",
    "VEXT2XV_D_W",
    0xFFFFFFFF,
    0x769F2724,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_DU_BU,
    "vext2xv.du.bu",
    "VEXT2XV_DU_BU",
    0xFFFFFFFF,
    0x769F332C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_DU_HU,
    "vext2xv.du.hu",
    "VEXT2XV_DU_HU",
    0xFFFFFFFF,
    0x769F38D2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_DU_WU,
    "vext2xv.du.wu",
    "VEXT2XV_DU_WU",
    0xFFFFFFFF,
    0x769F3EAA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_H_B,
    "vext2xv.h.b",
    "VEXT2XV_H_B",
    0xFFFFFFFF,
    0x769F127E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_HU_BU,
    "vext2xv.hu.bu",
    "VEXT2XV_HU_BU",
    0xFFFFFFFF,
    0x769F2999,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_W_B,
    "vext2xv.w.b",
    "VEXT2XV_W_B",
    0xFFFFFFFF,
    0x769F14BB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_W_H,
    "vext2xv.w.h",
    "VEXT2XV_W_H",
    0xFFFFFFFF,
    0x769F1E94,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_WU_BU,
    "vext2xv.wu.bu",
    "VEXT2XV_WU_BU",
    0xFFFFFFFF,
    0x769F2DBF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VEXT2XV_WU_HU,
    "vext2xv.wu.hu",
    "VEXT2XV_WU_HU",
    0xFFFFFFFF,
    0x769F3597,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VREPLVEI_B,
    "vreplvei.b",
    "VREPLVEI_B",
    0xFFFFFFFF,
    0x72F78C77,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VREPLVEI_D,
    "vreplvei.d",
    "VREPLVEI_D",
    0xFFFFFFFF,
    0x72F7F58F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VREPLVEI_H,
    "vreplvei.h",
    "VREPLVEI_H",
    0xFFFFFFFF,
    0x72F7C21B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    VREPLVEI_W,
    "vreplvei.w",
    "VREPLVEI_W",
    0xFFFFFFFF,
    0x72F7EEF2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector]
);

loongarch_insn!(
    XVABSD_B,
    "xvabsd.b",
    "XVABSD_B",
    0xFFFFFFFF,
    0x74604436,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_BU,
    "xvabsd.bu",
    "XVABSD_BU",
    0xFFFFFFFF,
    0x74623C90,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_D,
    "xvabsd.d",
    "XVABSD_D",
    0xFFFFFFFF,
    0x7461CEFE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_DU,
    "xvabsd.du",
    "XVABSD_DU",
    0xFFFFFFFF,
    0x7463915A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_H,
    "xvabsd.h",
    "XVABSD_H",
    0xFFFFFFFF,
    0x7460A711,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_HU,
    "xvabsd.hu",
    "XVABSD_HU",
    0xFFFFFFFF,
    0x7462EEED,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_W,
    "xvabsd.w",
    "XVABSD_W",
    0xFFFFFFFF,
    0x7461753C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVABSD_WU,
    "xvabsd.wu",
    "XVABSD_WU",
    0xFFFFFFFF,
    0x74633E5F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADD_B,
    "xvadd.b",
    "XVADD_B",
    0xFFFFFFFF,
    0x740A1674,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADD_D,
    "xvadd.d",
    "XVADD_D",
    0xFFFFFFFF,
    0x740BB4D3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADD_H,
    "xvadd.h",
    "XVADD_H",
    0xFFFFFFFF,
    0x740AB8F8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADD_Q,
    "xvadd.q",
    "XVADD_Q",
    0xFFFFFFFF,
    0x752D1B84,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADD_W,
    "xvadd.w",
    "XVADD_W",
    0xFFFFFFFF,
    0x740B5433,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDA_B,
    "xvadda.b",
    "XVADDA_B",
    0xFFFFFFFF,
    0x745C6F0A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDA_D,
    "xvadda.d",
    "XVADDA_D",
    0xFFFFFFFF,
    0x745DE42A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDA_H,
    "xvadda.h",
    "XVADDA_H",
    0xFFFFFFFF,
    0x745CF780,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDA_W,
    "xvadda.w",
    "XVADDA_W",
    0xFFFFFFFF,
    0x745D253F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDI_BU,
    "xvaddi.bu",
    "XVADDI_BU",
    0xFFFFFFFF,
    0x768A0AC1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDI_DU,
    "xvaddi.du",
    "XVADDI_DU",
    0xFFFFFFFF,
    0x768B9C06,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDI_HU,
    "xvaddi.hu",
    "XVADDI_HU",
    0xFFFFFFFF,
    0x768AF543,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDI_WU,
    "xvaddi.wu",
    "XVADDI_WU",
    0xFFFFFFFF,
    0x768B0D65,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_D_W,
    "xvaddwev.d.w",
    "XVADDWEV_D_W",
    0xFFFFFFFF,
    0x741F6528,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_D_WU,
    "xvaddwev.d.wu",
    "XVADDWEV_D_WU",
    0xFFFFFFFF,
    0x742F5210,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_D_WU_W,
    "xvaddwev.d.wu.w",
    "XVADDWEV_D_WU_W",
    0xFFFFFFFF,
    0x743F21A0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_H_B,
    "xvaddwev.h.b",
    "XVADDWEV_H_B",
    0xFFFFFFFF,
    0x741E13D7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_H_BU,
    "xvaddwev.h.bu",
    "XVADDWEV_H_BU",
    0xFFFFFFFF,
    0x742E69BE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_H_BU_B,
    "xvaddwev.h.bu.b",
    "XVADDWEV_H_BU_B",
    0xFFFFFFFF,
    0x743E24E3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_Q_D,
    "xvaddwev.q.d",
    "XVADDWEV_Q_D",
    0xFFFFFFFF,
    0x741FF6DD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_Q_DU,
    "xvaddwev.q.du",
    "XVADDWEV_Q_DU",
    0xFFFFFFFF,
    0x742FCA4A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_Q_DU_D,
    "xvaddwev.q.du.d",
    "XVADDWEV_Q_DU_D",
    0xFFFFFFFF,
    0x743F8D53,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_W_H,
    "xvaddwev.w.h",
    "XVADDWEV_W_H",
    0xFFFFFFFF,
    0x741EFE74,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_W_HU,
    "xvaddwev.w.hu",
    "XVADDWEV_W_HU",
    0xFFFFFFFF,
    0x742EC3EF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWEV_W_HU_H,
    "xvaddwev.w.hu.h",
    "XVADDWEV_W_HU_H",
    0xFFFFFFFF,
    0x743EEE1A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_D_W,
    "xvaddwod.d.w",
    "XVADDWOD_D_W",
    0xFFFFFFFF,
    0x7423512C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_D_WU,
    "xvaddwod.d.wu",
    "XVADDWOD_D_WU",
    0xFFFFFFFF,
    0x74332E7A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_D_WU_W,
    "xvaddwod.d.wu.w",
    "XVADDWOD_D_WU_W",
    0xFFFFFFFF,
    0x74417F8C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_H_B,
    "xvaddwod.h.b",
    "XVADDWOD_H_B",
    0xFFFFFFFF,
    0x742262AE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_H_BU,
    "xvaddwod.h.bu",
    "XVADDWOD_H_BU",
    0xFFFFFFFF,
    0x743224C6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_H_BU_B,
    "xvaddwod.h.bu.b",
    "XVADDWOD_H_BU_B",
    0xFFFFFFFF,
    0x74406355,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_Q_D,
    "xvaddwod.q.d",
    "XVADDWOD_Q_D",
    0xFFFFFFFF,
    0x7423A04B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_Q_DU,
    "xvaddwod.q.du",
    "XVADDWOD_Q_DU",
    0xFFFFFFFF,
    0x7433A2D5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_Q_DU_D,
    "xvaddwod.q.du.d",
    "XVADDWOD_Q_DU_D",
    0xFFFFFFFF,
    0x7441B09D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_W_H,
    "xvaddwod.w.h",
    "XVADDWOD_W_H",
    0xFFFFFFFF,
    0x7422DF53,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_W_HU,
    "xvaddwod.w.hu",
    "XVADDWOD_W_HU",
    0xFFFFFFFF,
    0x7432E761,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVADDWOD_W_HU_H,
    "xvaddwod.w.hu.h",
    "XVADDWOD_W_HU_H",
    0xFFFFFFFF,
    0x7440C0DF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAND_V,
    "xvand.v",
    "XVAND_V",
    0xFFFFFFFF,
    0x75264EEE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVANDI_B,
    "xvandi.b",
    "XVANDI_B",
    0xFFFFFFFF,
    0x77D108EB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVANDN_V,
    "xvandn.v",
    "XVANDN_V",
    0xFFFFFFFF,
    0x75280DE3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_B,
    "xvavg.b",
    "XVAVG_B",
    0xFFFFFFFF,
    0x746457C5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_BU,
    "xvavg.bu",
    "XVAVG_BU",
    0xFFFFFFFF,
    0x7466408B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_D,
    "xvavg.d",
    "XVAVG_D",
    0xFFFFFFFF,
    0x7465EC1B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_DU,
    "xvavg.du",
    "XVAVG_DU",
    0xFFFFFFFF,
    0x7467F697,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_H,
    "xvavg.h",
    "XVAVG_H",
    0xFFFFFFFF,
    0x7464D632,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_HU,
    "xvavg.hu",
    "XVAVG_HU",
    0xFFFFFFFF,
    0x7466CC22,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_W,
    "xvavg.w",
    "XVAVG_W",
    0xFFFFFFFF,
    0x746552E3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVG_WU,
    "xvavg.wu",
    "XVAVG_WU",
    0xFFFFFFFF,
    0x74676E9B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_B,
    "xvavgr.b",
    "XVAVGR_B",
    0xFFFFFFFF,
    0x74681DFD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_BU,
    "xvavgr.bu",
    "XVAVGR_BU",
    0xFFFFFFFF,
    0x746A6456,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_D,
    "xvavgr.d",
    "XVAVGR_D",
    0xFFFFFFFF,
    0x746982FD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_DU,
    "xvavgr.du",
    "XVAVGR_DU",
    0xFFFFFFFF,
    0x746BB562,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_H,
    "xvavgr.h",
    "XVAVGR_H",
    0xFFFFFFFF,
    0x7468BF40,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_HU,
    "xvavgr.hu",
    "XVAVGR_HU",
    0xFFFFFFFF,
    0x746AD559,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_W,
    "xvavgr.w",
    "XVAVGR_W",
    0xFFFFFFFF,
    0x74690017,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVAVGR_WU,
    "xvavgr.wu",
    "XVAVGR_WU",
    0xFFFFFFFF,
    0x746B0DD1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLR_B,
    "xvbitclr.b",
    "XVBITCLR_B",
    0xFFFFFFFF,
    0x750C38B8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLR_D,
    "xvbitclr.d",
    "XVBITCLR_D",
    0xFFFFFFFF,
    0x750DE4AE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLR_H,
    "xvbitclr.h",
    "XVBITCLR_H",
    0xFFFFFFFF,
    0x750CB53E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLR_W,
    "xvbitclr.w",
    "XVBITCLR_W",
    0xFFFFFFFF,
    0x750D1C62,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLRI_B,
    "xvbitclri.b",
    "XVBITCLRI_B",
    0xFFFFFFFF,
    0x77103F56,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLRI_D,
    "xvbitclri.d",
    "XVBITCLRI_D",
    0xFFFFFFFF,
    0x77111D8A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLRI_H,
    "xvbitclri.h",
    "XVBITCLRI_H",
    0xFFFFFFFF,
    0x771075C2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITCLRI_W,
    "xvbitclri.w",
    "XVBITCLRI_W",
    0xFFFFFFFF,
    0x77108043,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREV_B,
    "xvbitrev.b",
    "XVBITREV_B",
    0xFFFFFFFF,
    0x75100E90,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREV_D,
    "xvbitrev.d",
    "XVBITREV_D",
    0xFFFFFFFF,
    0x7511EC2D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREV_H,
    "xvbitrev.h",
    "XVBITREV_H",
    0xFFFFFFFF,
    0x7510D070,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREV_W,
    "xvbitrev.w",
    "XVBITREV_W",
    0xFFFFFFFF,
    0x75115F58,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREVI_B,
    "xvbitrevi.b",
    "XVBITREVI_B",
    0xFFFFFFFF,
    0x77183567,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREVI_D,
    "xvbitrevi.d",
    "XVBITREVI_D",
    0xFFFFFFFF,
    0x77192461,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREVI_H,
    "xvbitrevi.h",
    "XVBITREVI_H",
    0xFFFFFFFF,
    0x77187CA1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITREVI_W,
    "xvbitrevi.w",
    "XVBITREVI_W",
    0xFFFFFFFF,
    0x7718CAAD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSEL_V,
    "xvbitsel.v",
    "XVBITSEL_V",
    0xFFFFFFFF,
    0x0D2ABFB2,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSELI_B,
    "xvbitseli.b",
    "XVBITSELI_B",
    0xFFFFFFFF,
    0x77C5E6AD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSET_B,
    "xvbitset.b",
    "XVBITSET_B",
    0xFFFFFFFF,
    0x750E7206,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSET_D,
    "xvbitset.d",
    "XVBITSET_D",
    0xFFFFFFFF,
    0x750FB204,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSET_H,
    "xvbitset.h",
    "XVBITSET_H",
    0xFFFFFFFF,
    0x750EFDA5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSET_W,
    "xvbitset.w",
    "XVBITSET_W",
    0xFFFFFFFF,
    0x750F2387,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSETI_B,
    "xvbitseti.b",
    "XVBITSETI_B",
    0xFFFFFFFF,
    0x7714207A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSETI_D,
    "xvbitseti.d",
    "XVBITSETI_D",
    0xFFFFFFFF,
    0x771508F4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSETI_H,
    "xvbitseti.h",
    "XVBITSETI_H",
    0xFFFFFFFF,
    0x77146669,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBITSETI_W,
    "xvbitseti.w",
    "XVBITSETI_W",
    0xFFFFFFFF,
    0x77148A6C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBSLL_V,
    "xvbsll.v",
    "XVBSLL_V",
    0xFFFFFFFF,
    0x768E52AE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVBSRL_V,
    "xvbsrl.v",
    "XVBSRL_V",
    0xFFFFFFFF,
    0x768EF4A4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLO_B,
    "xvclo.b",
    "XVCLO_B",
    0xFFFFFFFF,
    0x769C0189,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLO_D,
    "xvclo.d",
    "XVCLO_D",
    0xFFFFFFFF,
    0x769C0CBF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLO_H,
    "xvclo.h",
    "XVCLO_H",
    0xFFFFFFFF,
    0x769C05D0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLO_W,
    "xvclo.w",
    "XVCLO_W",
    0xFFFFFFFF,
    0x769C0A5E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLZ_B,
    "xvclz.b",
    "XVCLZ_B",
    0xFFFFFFFF,
    0x769C10C5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLZ_D,
    "xvclz.d",
    "XVCLZ_D",
    0xFFFFFFFF,
    0x769C1C01,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLZ_H,
    "xvclz.h",
    "XVCLZ_H",
    0xFFFFFFFF,
    0x769C14E4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVCLZ_W,
    "xvclz.w",
    "XVCLZ_W",
    0xFFFFFFFF,
    0x769C180C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_B,
    "xvdiv.b",
    "XVDIV_B",
    0xFFFFFFFF,
    0x74E02329,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_BU,
    "xvdiv.bu",
    "XVDIV_BU",
    0xFFFFFFFF,
    0x74E47AC0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_D,
    "xvdiv.d",
    "XVDIV_D",
    0xFFFFFFFF,
    0x74E1B35B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_DU,
    "xvdiv.du",
    "XVDIV_DU",
    0xFFFFFFFF,
    0x74E59F27,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_H,
    "xvdiv.h",
    "XVDIV_H",
    0xFFFFFFFF,
    0x74E0EC32,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_HU,
    "xvdiv.hu",
    "XVDIV_HU",
    0xFFFFFFFF,
    0x74E4E6FF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_W,
    "xvdiv.w",
    "XVDIV_W",
    0xFFFFFFFF,
    0x74E16F45,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVDIV_WU,
    "xvdiv.wu",
    "XVDIV_WU",
    0xFFFFFFFF,
    0x74E51F21,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_D_W,
    "xvexth.d.w",
    "XVEXTH_D_W",
    0xFFFFFFFF,
    0x769EEB62,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_DU_WU,
    "xvexth.du.wu",
    "XVEXTH_DU_WU",
    0xFFFFFFFF,
    0x769EFB3B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_H_B,
    "xvexth.h.b",
    "XVEXTH_H_B",
    0xFFFFFFFF,
    0x769EE14F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_HU_BU,
    "xvexth.hu.bu",
    "XVEXTH_HU_BU",
    0xFFFFFFFF,
    0x769EF3D5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_Q_D,
    "xvexth.q.d",
    "XVEXTH_Q_D",
    0xFFFFFFFF,
    0x769EEF36,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_QU_DU,
    "xvexth.qu.du",
    "XVEXTH_QU_DU",
    0xFFFFFFFF,
    0x769EFF90,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_W_H,
    "xvexth.w.h",
    "XVEXTH_W_H",
    0xFFFFFFFF,
    0x769EE57A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTH_WU_HU,
    "xvexth.wu.hu",
    "XVEXTH_WU_HU",
    0xFFFFFFFF,
    0x769EF57C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTL_Q_D,
    "xvextl.q.d",
    "XVEXTL_Q_D",
    0xFFFFFFFF,
    0x7709019D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTL_QU_DU,
    "xvextl.qu.du",
    "XVEXTL_QU_DU",
    0xFFFFFFFF,
    0x770D029B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTRINS_B,
    "xvextrins.b",
    "XVEXTRINS_B",
    0xFFFFFFFF,
    0x778FF2FE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTRINS_D,
    "xvextrins.d",
    "XVEXTRINS_D",
    0xFFFFFFFF,
    0x77821FDF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTRINS_H,
    "xvextrins.h",
    "XVEXTRINS_H",
    0xFFFFFFFF,
    0x778B21A0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVEXTRINS_W,
    "xvextrins.w",
    "XVEXTRINS_W",
    0xFFFFFFFF,
    0x778662AE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFADD_D,
    "xvfadd.d",
    "XVFADD_D",
    0xFFFFFFFF,
    0x7531051B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFADD_S,
    "xvfadd.s",
    "XVFADD_S",
    0xFFFFFFFF,
    0x7530BEA6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCLASS_D,
    "xvfclass.d",
    "XVFCLASS_D",
    0xFFFFFFFF,
    0x769CD956,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCLASS_S,
    "xvfclass.s",
    "XVFCLASS_S",
    0xFFFFFFFF,
    0x769CD4E3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CAF_D,
    "xvfcmp.caf.d",
    "XVFCMP_CAF_D",
    0xFFFFFFFF,
    0x0CA053F3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CAF_S,
    "xvfcmp.caf.s",
    "XVFCMP_CAF_S",
    0xFFFFFFFF,
    0x0C907D01,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CEQ_D,
    "xvfcmp.ceq.d",
    "XVFCMP_CEQ_D",
    0xFFFFFFFF,
    0x0CA252FD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CEQ_S,
    "xvfcmp.ceq.s",
    "XVFCMP_CEQ_S",
    0xFFFFFFFF,
    0x0C920020,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CLE_D,
    "xvfcmp.cle.d",
    "XVFCMP_CLE_D",
    0xFFFFFFFF,
    0x0CA33335,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CLE_S,
    "xvfcmp.cle.s",
    "XVFCMP_CLE_S",
    0xFFFFFFFF,
    0x0C933ED6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CLT_D,
    "xvfcmp.clt.d",
    "XVFCMP_CLT_D",
    0xFFFFFFFF,
    0x0CA15493,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CLT_S,
    "xvfcmp.clt.s",
    "XVFCMP_CLT_S",
    0xFFFFFFFF,
    0x0C910524,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CNE_D,
    "xvfcmp.cne.d",
    "XVFCMP_CNE_D",
    0xFFFFFFFF,
    0x0CA80332,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CNE_S,
    "xvfcmp.cne.s",
    "XVFCMP_CNE_S",
    0xFFFFFFFF,
    0x0C986A27,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_COR_D,
    "xvfcmp.cor.d",
    "XVFCMP_COR_D",
    0xFFFFFFFF,
    0x0CAA5E6C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_COR_S,
    "xvfcmp.cor.s",
    "XVFCMP_COR_S",
    0xFFFFFFFF,
    0x0C9A3841,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUEQ_D,
    "xvfcmp.cueq.d",
    "XVFCMP_CUEQ_D",
    0xFFFFFFFF,
    0x0CA61EC4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUEQ_S,
    "xvfcmp.cueq.s",
    "XVFCMP_CUEQ_S",
    0xFFFFFFFF,
    0x0C967DA5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CULE_D,
    "xvfcmp.cule.d",
    "XVFCMP_CULE_D",
    0xFFFFFFFF,
    0x0CA72CA0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CULE_S,
    "xvfcmp.cule.s",
    "XVFCMP_CULE_S",
    0xFFFFFFFF,
    0x0C977441,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CULT_D,
    "xvfcmp.cult.d",
    "XVFCMP_CULT_D",
    0xFFFFFFFF,
    0x0CA51A34,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CULT_S,
    "xvfcmp.cult.s",
    "XVFCMP_CULT_S",
    0xFFFFFFFF,
    0x0C950E2F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUN_D,
    "xvfcmp.cun.d",
    "XVFCMP_CUN_D",
    0xFFFFFFFF,
    0x0CA472D3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUN_S,
    "xvfcmp.cun.s",
    "XVFCMP_CUN_S",
    0xFFFFFFFF,
    0x0C947528,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUNE_D,
    "xvfcmp.cune.d",
    "XVFCMP_CUNE_D",
    0xFFFFFFFF,
    0x0CAC33D4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_CUNE_S,
    "xvfcmp.cune.s",
    "XVFCMP_CUNE_S",
    0xFFFFFFFF,
    0x0C9C1235,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SAF_D,
    "xvfcmp.saf.d",
    "XVFCMP_SAF_D",
    0xFFFFFFFF,
    0x0CA09D87,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SAF_S,
    "xvfcmp.saf.s",
    "XVFCMP_SAF_S",
    0xFFFFFFFF,
    0x0C908977,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SEQ_D,
    "xvfcmp.seq.d",
    "XVFCMP_SEQ_D",
    0xFFFFFFFF,
    0x0CA28ECF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SEQ_S,
    "xvfcmp.seq.s",
    "XVFCMP_SEQ_S",
    0xFFFFFFFF,
    0x0C92EEEF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SLE_D,
    "xvfcmp.sle.d",
    "XVFCMP_SLE_D",
    0xFFFFFFFF,
    0x0CA3DC23,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SLE_S,
    "xvfcmp.sle.s",
    "XVFCMP_SLE_S",
    0xFFFFFFFF,
    0x0C93C0A1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SLT_D,
    "xvfcmp.slt.d",
    "XVFCMP_SLT_D",
    0xFFFFFFFF,
    0x0CA1E351,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SLT_S,
    "xvfcmp.slt.s",
    "XVFCMP_SLT_S",
    0xFFFFFFFF,
    0x0C91FE59,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SNE_D,
    "xvfcmp.sne.d",
    "XVFCMP_SNE_D",
    0xFFFFFFFF,
    0x0CA8C694,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SNE_S,
    "xvfcmp.sne.s",
    "XVFCMP_SNE_S",
    0xFFFFFFFF,
    0x0C98F99B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SOR_D,
    "xvfcmp.sor.d",
    "XVFCMP_SOR_D",
    0xFFFFFFFF,
    0x0CAA9B86,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SOR_S,
    "xvfcmp.sor.s",
    "XVFCMP_SOR_S",
    0xFFFFFFFF,
    0x0C9A89AB,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUEQ_D,
    "xvfcmp.sueq.d",
    "XVFCMP_SUEQ_D",
    0xFFFFFFFF,
    0x0CA6C645,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUEQ_S,
    "xvfcmp.sueq.s",
    "XVFCMP_SUEQ_S",
    0xFFFFFFFF,
    0x0C96A74C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SULE_D,
    "xvfcmp.sule.d",
    "XVFCMP_SULE_D",
    0xFFFFFFFF,
    0x0CA7C54B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SULE_S,
    "xvfcmp.sule.s",
    "XVFCMP_SULE_S",
    0xFFFFFFFF,
    0x0C978577,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SULT_D,
    "xvfcmp.sult.d",
    "XVFCMP_SULT_D",
    0xFFFFFFFF,
    0x0CA59484,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SULT_S,
    "xvfcmp.sult.s",
    "XVFCMP_SULT_S",
    0xFFFFFFFF,
    0x0C95C9E8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUN_D,
    "xvfcmp.sun.d",
    "XVFCMP_SUN_D",
    0xFFFFFFFF,
    0x0CA4F964,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUN_S,
    "xvfcmp.sun.s",
    "XVFCMP_SUN_S",
    0xFFFFFFFF,
    0x0C94F8E0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUNE_D,
    "xvfcmp.sune.d",
    "XVFCMP_SUNE_D",
    0xFFFFFFFF,
    0x0CACECBE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCMP_SUNE_S,
    "xvfcmp.sune.s",
    "XVFCMP_SUNE_S",
    0xFFFFFFFF,
    0x0C9CA20B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVT_H_S,
    "xvfcvt.h.s",
    "XVFCVT_H_S",
    0xFFFFFFFF,
    0x75465E29,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVT_S_D,
    "xvfcvt.s.d",
    "XVFCVT_S_D",
    0xFFFFFFFF,
    0x7546F55B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVTH_D_S,
    "xvfcvth.d.s",
    "XVFCVTH_D_S",
    0xFFFFFFFF,
    0x769DF63D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVTH_S_H,
    "xvfcvth.s.h",
    "XVFCVTH_S_H",
    0xFFFFFFFF,
    0x769DEF29,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVTL_D_S,
    "xvfcvtl.d.s",
    "XVFCVTL_D_S",
    0xFFFFFFFF,
    0x769DF0B8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFCVTL_S_H,
    "xvfcvtl.s.h",
    "XVFCVTL_S_H",
    0xFFFFFFFF,
    0x769DE9D0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFDIV_D,
    "xvfdiv.d",
    "XVFDIV_D",
    0xFFFFFFFF,
    0x753B795F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFDIV_S,
    "xvfdiv.s",
    "XVFDIV_S",
    0xFFFFFFFF,
    0x753AB0BD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINT_D_L,
    "xvffint.d.l",
    "XVFFINT_D_L",
    0xFFFFFFFF,
    0x769E0A65,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINT_D_LU,
    "xvffint.d.lu",
    "XVFFINT_D_LU",
    0xFFFFFFFF,
    0x769E0FBF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINT_S_L,
    "xvffint.s.l",
    "XVFFINT_S_L",
    0xFFFFFFFF,
    0x75480F6A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINT_S_W,
    "xvffint.s.w",
    "XVFFINT_S_W",
    0xFFFFFFFF,
    0x769E00A3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINT_S_WU,
    "xvffint.s.wu",
    "XVFFINT_S_WU",
    0xFFFFFFFF,
    0x769E0783,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINTH_D_W,
    "xvffinth.d.w",
    "XVFFINTH_D_W",
    0xFFFFFFFF,
    0x769E1787,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFFINTL_D_W,
    "xvffintl.d.w",
    "XVFFINTL_D_W",
    0xFFFFFFFF,
    0x769E10E2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFLOGB_D,
    "xvflogb.d",
    "XVFLOGB_D",
    0xFFFFFFFF,
    0x769CC83A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFLOGB_S,
    "xvflogb.s",
    "XVFLOGB_S",
    0xFFFFFFFF,
    0x769CC591,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMADD_D,
    "xvfmadd.d",
    "XVFMADD_D",
    0xFFFFFFFF,
    0x0A2CFE09,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMADD_S,
    "xvfmadd.s",
    "XVFMADD_S",
    0xFFFFFFFF,
    0x0A1DFFE5,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMAX_D,
    "xvfmax.d",
    "XVFMAX_D",
    0xFFFFFFFF,
    0x753D5F3F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMAX_S,
    "xvfmax.s",
    "XVFMAX_S",
    0xFFFFFFFF,
    0x753CA31D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMAXA_D,
    "xvfmaxa.d",
    "XVFMAXA_D",
    0xFFFFFFFF,
    0x75417682,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMAXA_S,
    "xvfmaxa.s",
    "XVFMAXA_S",
    0xFFFFFFFF,
    0x7540964F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMIN_D,
    "xvfmin.d",
    "XVFMIN_D",
    0xFFFFFFFF,
    0x753F67CD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMIN_S,
    "xvfmin.s",
    "XVFMIN_S",
    0xFFFFFFFF,
    0x753EC0BF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMINA_D,
    "xvfmina.d",
    "XVFMINA_D",
    0xFFFFFFFF,
    0x75434A8C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMINA_S,
    "xvfmina.s",
    "XVFMINA_S",
    0xFFFFFFFF,
    0x7542C77D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMSUB_D,
    "xvfmsub.d",
    "XVFMSUB_D",
    0xFFFFFFFF,
    0x0A6741FE,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMSUB_S,
    "xvfmsub.s",
    "XVFMSUB_S",
    0xFFFFFFFF,
    0x0A5B8C71,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMUL_D,
    "xvfmul.d",
    "XVFMUL_D",
    0xFFFFFFFF,
    0x75394F5C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFMUL_S,
    "xvfmul.s",
    "XVFMUL_S",
    0xFFFFFFFF,
    0x7538F9C9,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFNMADD_D,
    "xvfnmadd.d",
    "XVFNMADD_D",
    0xFFFFFFFF,
    0x0AA65FC1,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFNMADD_S,
    "xvfnmadd.s",
    "XVFNMADD_S",
    0xFFFFFFFF,
    0x0A9C5ECE,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFNMSUB_D,
    "xvfnmsub.d",
    "XVFNMSUB_D",
    0xFFFFFFFF,
    0x0AEE7408,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFNMSUB_S,
    "xvfnmsub.s",
    "XVFNMSUB_S",
    0xFFFFFFFF,
    0x0AD590B6,
    &FMT_R4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Ra, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRECIP_D,
    "xvfrecip.d",
    "XVFRECIP_D",
    0xFFFFFFFF,
    0x769CFB11,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRECIP_S,
    "xvfrecip.s",
    "XVFRECIP_S",
    0xFFFFFFFF,
    0x769CF603,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRECIPE_D,
    "xvfrecipe.d",
    "XVFRECIPE_D",
    0xFFFFFFFF,
    0x769D1B11,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRECIPE_S,
    "xvfrecipe.s",
    "XVFRECIPE_S",
    0xFFFFFFFF,
    0x769D1603,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINT_D,
    "xvfrint.d",
    "XVFRINT_D",
    0xFFFFFFFF,
    0x769D3A5F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINT_S,
    "xvfrint.s",
    "XVFRINT_S",
    0xFFFFFFFF,
    0x769D3715,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRM_D,
    "xvfrintrm.d",
    "XVFRINTRM_D",
    0xFFFFFFFF,
    0x769D4B6E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRM_S,
    "xvfrintrm.s",
    "XVFRINTRM_S",
    0xFFFFFFFF,
    0x769D45BB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRNE_D,
    "xvfrintrne.d",
    "XVFRINTRNE_D",
    0xFFFFFFFF,
    0x769D7BAC,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRNE_S,
    "xvfrintrne.s",
    "XVFRINTRNE_S",
    0xFFFFFFFF,
    0x769D7633,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRP_D,
    "xvfrintrp.d",
    "XVFRINTRP_D",
    0xFFFFFFFF,
    0x769D5B81,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRP_S,
    "xvfrintrp.s",
    "XVFRINTRP_S",
    0xFFFFFFFF,
    0x769D561A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRZ_D,
    "xvfrintrz.d",
    "XVFRINTRZ_D",
    0xFFFFFFFF,
    0x769D68BD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRINTRZ_S,
    "xvfrintrz.s",
    "XVFRINTRZ_S",
    0xFFFFFFFF,
    0x769D652A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSQRT_D,
    "xvfrsqrt.d",
    "XVFRSQRT_D",
    0xFFFFFFFF,
    0x769D0ACE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSQRT_S,
    "xvfrsqrt.s",
    "XVFRSQRT_S",
    0xFFFFFFFF,
    0x769D073F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSQRTE_D,
    "xvfrsqrte.d",
    "XVFRSQRTE_D",
    0xFFFFFFFF,
    0x769D2ACE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSQRTE_S,
    "xvfrsqrte.s",
    "XVFRSQRTE_S",
    0xFFFFFFFF,
    0x769D273F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSTP_B,
    "xvfrstp.b",
    "XVFRSTP_B",
    0xFFFFFFFF,
    0x752B4A57,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSTP_H,
    "xvfrstp.h",
    "XVFRSTP_H",
    0xFFFFFFFF,
    0x752B9BCD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSTPI_B,
    "xvfrstpi.b",
    "XVFRSTPI_B",
    0xFFFFFFFF,
    0x769A7F98,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFRSTPI_H,
    "xvfrstpi.h",
    "XVFRSTPI_H",
    0xFFFFFFFF,
    0x769ACB16,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFSQRT_D,
    "xvfsqrt.d",
    "XVFSQRT_D",
    0xFFFFFFFF,
    0x769CE85A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFSQRT_S,
    "xvfsqrt.s",
    "XVFSQRT_S",
    0xFFFFFFFF,
    0x769CE764,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFSUB_D,
    "xvfsub.d",
    "XVFSUB_D",
    0xFFFFFFFF,
    0x75333F24,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFSUB_S,
    "xvfsub.s",
    "XVFSUB_S",
    0xFFFFFFFF,
    0x75328C16,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINT_L_D,
    "xvftint.l.d",
    "XVFTINT_L_D",
    0xFFFFFFFF,
    0x769E36C7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINT_LU_D,
    "xvftint.lu.d",
    "XVFTINT_LU_D",
    0xFFFFFFFF,
    0x769E5C42,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINT_W_D,
    "xvftint.w.d",
    "XVFTINT_W_D",
    0xFFFFFFFF,
    0x7549F6C7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINT_W_S,
    "xvftint.w.s",
    "XVFTINT_W_S",
    0xFFFFFFFF,
    0x769E332B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINT_WU_S,
    "xvftint.wu.s",
    "XVFTINT_WU_S",
    0xFFFFFFFF,
    0x769E58CE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTH_L_S,
    "xvftinth.l.s",
    "XVFTINTH_L_S",
    0xFFFFFFFF,
    0x769E84AF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTL_L_S,
    "xvftintl.l.s",
    "XVFTINTL_L_S",
    0xFFFFFFFF,
    0x769E817F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRM_L_D,
    "xvftintrm.l.d",
    "XVFTINTRM_L_D",
    0xFFFFFFFF,
    0x769E3E2C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRM_W_D,
    "xvftintrm.w.d",
    "XVFTINTRM_W_D",
    0xFFFFFFFF,
    0x754A1EFD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRM_W_S,
    "xvftintrm.w.s",
    "XVFTINTRM_W_S",
    0xFFFFFFFF,
    0x769E3AE8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRMH_L_S,
    "xvftintrmh.l.s",
    "XVFTINTRMH_L_S",
    0xFFFFFFFF,
    0x769E8E6A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRML_L_S,
    "xvftintrml.l.s",
    "XVFTINTRML_L_S",
    0xFFFFFFFF,
    0x769E89F6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRNE_L_D,
    "xvftintrne.l.d",
    "XVFTINTRNE_L_D",
    0xFFFFFFFF,
    0x769E55DE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRNE_W_D,
    "xvftintrne.w.d",
    "XVFTINTRNE_W_D",
    0xFFFFFFFF,
    0x754B968D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRNE_W_S,
    "xvftintrne.w.s",
    "XVFTINTRNE_W_S",
    0xFFFFFFFF,
    0x769E51B4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRNEH_L_S,
    "xvftintrneh.l.s",
    "XVFTINTRNEH_L_S",
    0xFFFFFFFF,
    0x769EA7B0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRNEL_L_S,
    "xvftintrnel.l.s",
    "XVFTINTRNEL_L_S",
    0xFFFFFFFF,
    0x769EA39F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRP_L_D,
    "xvftintrp.l.d",
    "XVFTINTRP_L_D",
    0xFFFFFFFF,
    0x769E470A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRP_W_D,
    "xvftintrp.w.d",
    "XVFTINTRP_W_D",
    0xFFFFFFFF,
    0x754AFF4E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRP_W_S,
    "xvftintrp.w.s",
    "XVFTINTRP_W_S",
    0xFFFFFFFF,
    0x769E4032,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRPH_L_S,
    "xvftintrph.l.s",
    "XVFTINTRPH_L_S",
    0xFFFFFFFF,
    0x769E9417,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRPL_L_S,
    "xvftintrpl.l.s",
    "XVFTINTRPL_L_S",
    0xFFFFFFFF,
    0x769E900E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZ_L_D,
    "xvftintrz.l.d",
    "XVFTINTRZ_L_D",
    0xFFFFFFFF,
    0x769E4F41,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZ_LU_D,
    "xvftintrz.lu.d",
    "XVFTINTRZ_LU_D",
    0xFFFFFFFF,
    0x769E7478,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZ_W_D,
    "xvftintrz.w.d",
    "XVFTINTRZ_W_D",
    0xFFFFFFFF,
    0x754B6D0D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZ_W_S,
    "xvftintrz.w.s",
    "XVFTINTRZ_W_S",
    0xFFFFFFFF,
    0x769E48AE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZ_WU_S,
    "xvftintrz.wu.s",
    "XVFTINTRZ_WU_S",
    0xFFFFFFFF,
    0x769E726D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZH_L_S,
    "xvftintrzh.l.s",
    "XVFTINTRZH_L_S",
    0xFFFFFFFF,
    0x769E9D4E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVFTINTRZL_L_S,
    "xvftintrzl.l.s",
    "XVFTINTRZL_L_S",
    0xFFFFFFFF,
    0x769E9BBB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_D_W,
    "xvhaddw.d.w",
    "XVHADDW_D_W",
    0xFFFFFFFF,
    0x7455603E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_DU_WU,
    "xvhaddw.du.wu",
    "XVHADDW_DU_WU",
    0xFFFFFFFF,
    0x74594F06,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_H_B,
    "xvhaddw.h.b",
    "XVHADDW_H_B",
    0xFFFFFFFF,
    0x7454767F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_HU_BU,
    "xvhaddw.hu.bu",
    "XVHADDW_HU_BU",
    0xFFFFFFFF,
    0x74580A2E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_Q_D,
    "xvhaddw.q.d",
    "XVHADDW_Q_D",
    0xFFFFFFFF,
    0x7455C5F0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_QU_DU,
    "xvhaddw.qu.du",
    "XVHADDW_QU_DU",
    0xFFFFFFFF,
    0x7459B58A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_W_H,
    "xvhaddw.w.h",
    "XVHADDW_W_H",
    0xFFFFFFFF,
    0x7454DE1F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHADDW_WU_HU,
    "xvhaddw.wu.hu",
    "XVHADDW_WU_HU",
    0xFFFFFFFF,
    0x7458A055,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_D_W,
    "xvhsubw.d.w",
    "XVHSUBW_D_W",
    0xFFFFFFFF,
    0x74574EFE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_DU_WU,
    "xvhsubw.du.wu",
    "XVHSUBW_DU_WU",
    0xFFFFFFFF,
    0x745B52E5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_H_B,
    "xvhsubw.h.b",
    "XVHSUBW_H_B",
    0xFFFFFFFF,
    0x745640F6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_HU_BU,
    "xvhsubw.hu.bu",
    "XVHSUBW_HU_BU",
    0xFFFFFFFF,
    0x745A404A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_Q_D,
    "xvhsubw.q.d",
    "XVHSUBW_Q_D",
    0xFFFFFFFF,
    0x7457F1B4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_QU_DU,
    "xvhsubw.qu.du",
    "XVHSUBW_QU_DU",
    0xFFFFFFFF,
    0x745BA09F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_W_H,
    "xvhsubw.w.h",
    "XVHSUBW_W_H",
    0xFFFFFFFF,
    0x7456BD13,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVHSUBW_WU_HU,
    "xvhsubw.wu.hu",
    "XVHSUBW_WU_HU",
    0xFFFFFFFF,
    0x745ACB41,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVH_B,
    "xvilvh.b",
    "XVILVH_B",
    0xFFFFFFFF,
    0x751C6AD3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVH_D,
    "xvilvh.d",
    "XVILVH_D",
    0xFFFFFFFF,
    0x751D8858,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVH_H,
    "xvilvh.h",
    "XVILVH_H",
    0xFFFFFFFF,
    0x751C9EEA,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVH_W,
    "xvilvh.w",
    "XVILVH_W",
    0xFFFFFFFF,
    0x751D7805,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVL_B,
    "xvilvl.b",
    "XVILVL_B",
    0xFFFFFFFF,
    0x751A01DD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVL_D,
    "xvilvl.d",
    "XVILVL_D",
    0xFFFFFFFF,
    0x751BAA99,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVL_H,
    "xvilvl.h",
    "XVILVL_H",
    0xFFFFFFFF,
    0x751AD53E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVILVL_W,
    "xvilvl.w",
    "XVILVL_W",
    0xFFFFFFFF,
    0x751B26D8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVINSGR2VR_D,
    "xvinsgr2vr.d",
    "XVINSGR2VR_D",
    0xFFFFFFFF,
    0x76EBE6BB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVINSGR2VR_W,
    "xvinsgr2vr.w",
    "XVINSGR2VR_W",
    0xFFFFFFFF,
    0x76EBDFD9,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVINSVE0_D,
    "xvinsve0.d",
    "XVINSVE0_D",
    0xFFFFFFFF,
    0x76FFE03C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVINSVE0_W,
    "xvinsve0.w",
    "XVINSVE0_W",
    0xFFFFFFFF,
    0x76FFDC26,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLD,
    "xvld",
    "XVLD",
    0xFFFFFFFF,
    0x2CB5B863,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLDREPL_B,
    "xvldrepl.b",
    "XVLDREPL_B",
    0xFFFFFFFF,
    0x329D92B3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLDREPL_D,
    "xvldrepl.d",
    "XVLDREPL_D",
    0xFFFFFFFF,
    0x3213DD9C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLDREPL_H,
    "xvldrepl.h",
    "XVLDREPL_H",
    0xFFFFFFFF,
    0x324DC620,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLDREPL_W,
    "xvldrepl.w",
    "XVLDREPL_W",
    0xFFFFFFFF,
    0x322A0F4B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVLDX,
    "xvldx",
    "XVLDX",
    0xFFFFFFFF,
    0x38483937,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADD_B,
    "xvmadd.b",
    "XVMADD_B",
    0xFFFFFFFF,
    0x74A823E5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADD_D,
    "xvmadd.d",
    "XVMADD_D",
    0xFFFFFFFF,
    0x74A9C913,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADD_H,
    "xvmadd.h",
    "XVMADD_H",
    0xFFFFFFFF,
    0x74A8F004,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADD_W,
    "xvmadd.w",
    "XVMADD_W",
    0xFFFFFFFF,
    0x74A961A2,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_D_W,
    "xvmaddwev.d.w",
    "XVMADDWEV_D_W",
    0xFFFFFFFF,
    0x74AD6317,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_D_WU,
    "xvmaddwev.d.wu",
    "XVMADDWEV_D_WU",
    0xFFFFFFFF,
    0x74B5737D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_D_WU_W,
    "xvmaddwev.d.wu.w",
    "XVMADDWEV_D_WU_W",
    0xFFFFFFFF,
    0x74BD0B8A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_H_B,
    "xvmaddwev.h.b",
    "XVMADDWEV_H_B",
    0xFFFFFFFF,
    0x74AC25F9,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_H_BU,
    "xvmaddwev.h.bu",
    "XVMADDWEV_H_BU",
    0xFFFFFFFF,
    0x74B469B7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_H_BU_B,
    "xvmaddwev.h.bu.b",
    "XVMADDWEV_H_BU_B",
    0xFFFFFFFF,
    0x74BC7F5E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_Q_D,
    "xvmaddwev.q.d",
    "XVMADDWEV_Q_D",
    0xFFFFFFFF,
    0x74ADD927,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_Q_DU,
    "xvmaddwev.q.du",
    "XVMADDWEV_Q_DU",
    0xFFFFFFFF,
    0x74B5A95D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_Q_DU_D,
    "xvmaddwev.q.du.d",
    "XVMADDWEV_Q_DU_D",
    0xFFFFFFFF,
    0x74BDE290,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_W_H,
    "xvmaddwev.w.h",
    "XVMADDWEV_W_H",
    0xFFFFFFFF,
    0x74AC803A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_W_HU,
    "xvmaddwev.w.hu",
    "XVMADDWEV_W_HU",
    0xFFFFFFFF,
    0x74B48C6D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWEV_W_HU_H,
    "xvmaddwev.w.hu.h",
    "XVMADDWEV_W_HU_H",
    0xFFFFFFFF,
    0x74BCFE26,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_D_W,
    "xvmaddwod.d.w",
    "XVMADDWOD_D_W",
    0xFFFFFFFF,
    0x74AF3680,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_D_WU,
    "xvmaddwod.d.wu",
    "XVMADDWOD_D_WU",
    0xFFFFFFFF,
    0x74B72E17,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_D_WU_W,
    "xvmaddwod.d.wu.w",
    "XVMADDWOD_D_WU_W",
    0xFFFFFFFF,
    0x74BF380B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_H_B,
    "xvmaddwod.h.b",
    "XVMADDWOD_H_B",
    0xFFFFFFFF,
    0x74AE4910,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_H_BU,
    "xvmaddwod.h.bu",
    "XVMADDWOD_H_BU",
    0xFFFFFFFF,
    0x74B61EFF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_H_BU_B,
    "xvmaddwod.h.bu.b",
    "XVMADDWOD_H_BU_B",
    0xFFFFFFFF,
    0x74BE2C5B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_Q_D,
    "xvmaddwod.q.d",
    "XVMADDWOD_Q_D",
    0xFFFFFFFF,
    0x74AFCAEF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_Q_DU,
    "xvmaddwod.q.du",
    "XVMADDWOD_Q_DU",
    0xFFFFFFFF,
    0x74B7CD49,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_Q_DU_D,
    "xvmaddwod.q.du.d",
    "XVMADDWOD_Q_DU_D",
    0xFFFFFFFF,
    0x74BFFE7D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_W_H,
    "xvmaddwod.w.h",
    "XVMADDWOD_W_H",
    0xFFFFFFFF,
    0x74AEBB0B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_W_HU,
    "xvmaddwod.w.hu",
    "XVMADDWOD_W_HU",
    0xFFFFFFFF,
    0x74B6A21D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMADDWOD_W_HU_H,
    "xvmaddwod.w.hu.h",
    "XVMADDWOD_W_HU_H",
    0xFFFFFFFF,
    0x74BECF0C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_B,
    "xvmax.b",
    "XVMAX_B",
    0xFFFFFFFF,
    0x74703517,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_BU,
    "xvmax.bu",
    "XVMAX_BU",
    0xFFFFFFFF,
    0x74742FDD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_D,
    "xvmax.d",
    "XVMAX_D",
    0xFFFFFFFF,
    0x7471B622,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_DU,
    "xvmax.du",
    "XVMAX_DU",
    0xFFFFFFFF,
    0x7475A6C5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_H,
    "xvmax.h",
    "XVMAX_H",
    0xFFFFFFFF,
    0x7470F24D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_HU,
    "xvmax.hu",
    "XVMAX_HU",
    0xFFFFFFFF,
    0x7474EEE4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_W,
    "xvmax.w",
    "XVMAX_W",
    0xFFFFFFFF,
    0x7471083A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAX_WU,
    "xvmax.wu",
    "XVMAX_WU",
    0xFFFFFFFF,
    0x7475001F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_B,
    "xvmaxi.b",
    "XVMAXI_B",
    0xFFFFFFFF,
    0x769004E6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_BU,
    "xvmaxi.bu",
    "XVMAXI_BU",
    0xFFFFFFFF,
    0x7694736C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_D,
    "xvmaxi.d",
    "XVMAXI_D",
    0xFFFFFFFF,
    0x7691D4B5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_DU,
    "xvmaxi.du",
    "XVMAXI_DU",
    0xFFFFFFFF,
    0x7695A5BF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_H,
    "xvmaxi.h",
    "XVMAXI_H",
    0xFFFFFFFF,
    0x7690E558,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_HU,
    "xvmaxi.hu",
    "XVMAXI_HU",
    0xFFFFFFFF,
    0x7694C099,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_W,
    "xvmaxi.w",
    "XVMAXI_W",
    0xFFFFFFFF,
    0x76916258,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMAXI_WU,
    "xvmaxi.wu",
    "XVMAXI_WU",
    0xFFFFFFFF,
    0x769554FB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_B,
    "xvmin.b",
    "XVMIN_B",
    0xFFFFFFFF,
    0x74721F55,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_BU,
    "xvmin.bu",
    "XVMIN_BU",
    0xFFFFFFFF,
    0x74760E0F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_D,
    "xvmin.d",
    "XVMIN_D",
    0xFFFFFFFF,
    0x74738B7B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_DU,
    "xvmin.du",
    "XVMIN_DU",
    0xFFFFFFFF,
    0x7477947B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_H,
    "xvmin.h",
    "XVMIN_H",
    0xFFFFFFFF,
    0x7472A4BD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_HU,
    "xvmin.hu",
    "XVMIN_HU",
    0xFFFFFFFF,
    0x7476EFE4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_W,
    "xvmin.w",
    "XVMIN_W",
    0xFFFFFFFF,
    0x7473531F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMIN_WU,
    "xvmin.wu",
    "XVMIN_WU",
    0xFFFFFFFF,
    0x747771AF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_B,
    "xvmini.b",
    "XVMINI_B",
    0xFFFFFFFF,
    0x76922636,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_BU,
    "xvmini.bu",
    "XVMINI_BU",
    0xFFFFFFFF,
    0x76961F06,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_D,
    "xvmini.d",
    "XVMINI_D",
    0xFFFFFFFF,
    0x7693AFEA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_DU,
    "xvmini.du",
    "XVMINI_DU",
    0xFFFFFFFF,
    0x7697FAF0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_H,
    "xvmini.h",
    "XVMINI_H",
    0xFFFFFFFF,
    0x7692C6EC,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_HU,
    "xvmini.hu",
    "XVMINI_HU",
    0xFFFFFFFF,
    0x7696F4A8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_W,
    "xvmini.w",
    "XVMINI_W",
    0xFFFFFFFF,
    0x76934E21,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMINI_WU,
    "xvmini.wu",
    "XVMINI_WU",
    0xFFFFFFFF,
    0x76974DB1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_B,
    "xvmod.b",
    "XVMOD_B",
    0xFFFFFFFF,
    0x74E20068,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_BU,
    "xvmod.bu",
    "XVMOD_BU",
    0xFFFFFFFF,
    0x74E66830,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_D,
    "xvmod.d",
    "XVMOD_D",
    0xFFFFFFFF,
    0x74E3C94B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_DU,
    "xvmod.du",
    "XVMOD_DU",
    0xFFFFFFFF,
    0x74E7986E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_H,
    "xvmod.h",
    "XVMOD_H",
    0xFFFFFFFF,
    0x74E2F222,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_HU,
    "xvmod.hu",
    "XVMOD_HU",
    0xFFFFFFFF,
    0x74E681AF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_W,
    "xvmod.w",
    "XVMOD_W",
    0xFFFFFFFF,
    0x74E3350E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMOD_WU,
    "xvmod.wu",
    "XVMOD_WU",
    0xFFFFFFFF,
    0x74E7526B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKGEZ_B,
    "xvmskgez.b",
    "XVMSKGEZ_B",
    0xFFFFFFFF,
    0x769C50BE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKLTZ_B,
    "xvmskltz.b",
    "XVMSKLTZ_B",
    0xFFFFFFFF,
    0x769C40AE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKLTZ_D,
    "xvmskltz.d",
    "XVMSKLTZ_D",
    0xFFFFFFFF,
    0x769C4EE7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKLTZ_H,
    "xvmskltz.h",
    "XVMSKLTZ_H",
    0xFFFFFFFF,
    0x769C472B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKLTZ_W,
    "xvmskltz.w",
    "XVMSKLTZ_W",
    0xFFFFFFFF,
    0x769C4B6E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSKNZ_B,
    "xvmsknz.b",
    "XVMSKNZ_B",
    0xFFFFFFFF,
    0x769C62D6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSUB_B,
    "xvmsub.b",
    "XVMSUB_B",
    0xFFFFFFFF,
    0x74AA1E96,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSUB_D,
    "xvmsub.d",
    "XVMSUB_D",
    0xFFFFFFFF,
    0x74AB8B4B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSUB_H,
    "xvmsub.h",
    "XVMSUB_H",
    0xFFFFFFFF,
    0x74AAB240,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMSUB_W,
    "xvmsub.w",
    "XVMSUB_W",
    0xFFFFFFFF,
    0x74AB76C3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_B,
    "xvmuh.b",
    "XVMUH_B",
    0xFFFFFFFF,
    0x74861104,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_BU,
    "xvmuh.bu",
    "XVMUH_BU",
    0xFFFFFFFF,
    0x7488628F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_D,
    "xvmuh.d",
    "XVMUH_D",
    0xFFFFFFFF,
    0x7487A406,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_DU,
    "xvmuh.du",
    "XVMUH_DU",
    0xFFFFFFFF,
    0x7489FD13,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_H,
    "xvmuh.h",
    "XVMUH_H",
    0xFFFFFFFF,
    0x7486EAE5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_HU,
    "xvmuh.hu",
    "XVMUH_HU",
    0xFFFFFFFF,
    0x7488ED9C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_W,
    "xvmuh.w",
    "XVMUH_W",
    0xFFFFFFFF,
    0x7487647C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUH_WU,
    "xvmuh.wu",
    "XVMUH_WU",
    0xFFFFFFFF,
    0x748928D9,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUL_B,
    "xvmul.b",
    "XVMUL_B",
    0xFFFFFFFF,
    0x74846CF2,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUL_D,
    "xvmul.d",
    "XVMUL_D",
    0xFFFFFFFF,
    0x7485A1E0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUL_H,
    "xvmul.h",
    "XVMUL_H",
    0xFFFFFFFF,
    0x7484CAE9,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMUL_W,
    "xvmul.w",
    "XVMUL_W",
    0xFFFFFFFF,
    0x74856D15,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_D_W,
    "xvmulwev.d.w",
    "XVMULWEV_D_W",
    0xFFFFFFFF,
    0x74913F10,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_D_WU,
    "xvmulwev.d.wu",
    "XVMULWEV_D_WU",
    0xFFFFFFFF,
    0x74997B01,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_D_WU_W,
    "xvmulwev.d.wu.w",
    "XVMULWEV_D_WU_W",
    0xFFFFFFFF,
    0x74A144ED,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_H_B,
    "xvmulwev.h.b",
    "XVMULWEV_H_B",
    0xFFFFFFFF,
    0x749040E2,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_H_BU,
    "xvmulwev.h.bu",
    "XVMULWEV_H_BU",
    0xFFFFFFFF,
    0x749874F4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_H_BU_B,
    "xvmulwev.h.bu.b",
    "XVMULWEV_H_BU_B",
    0xFFFFFFFF,
    0x74A0338D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_Q_D,
    "xvmulwev.q.d",
    "XVMULWEV_Q_D",
    0xFFFFFFFF,
    0x74919211,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_Q_DU,
    "xvmulwev.q.du",
    "XVMULWEV_Q_DU",
    0xFFFFFFFF,
    0x7499EEC1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_Q_DU_D,
    "xvmulwev.q.du.d",
    "XVMULWEV_Q_DU_D",
    0xFFFFFFFF,
    0x74A1BE89,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_W_H,
    "xvmulwev.w.h",
    "XVMULWEV_W_H",
    0xFFFFFFFF,
    0x7490996C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_W_HU,
    "xvmulwev.w.hu",
    "XVMULWEV_W_HU",
    0xFFFFFFFF,
    0x7498C70D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWEV_W_HU_H,
    "xvmulwev.w.hu.h",
    "XVMULWEV_W_HU_H",
    0xFFFFFFFF,
    0x74A09E1B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_D_W,
    "xvmulwod.d.w",
    "XVMULWOD_D_W",
    0xFFFFFFFF,
    0x7493237E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_D_WU,
    "xvmulwod.d.wu",
    "XVMULWOD_D_WU",
    0xFFFFFFFF,
    0x749B52D8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_D_WU_W,
    "xvmulwod.d.wu.w",
    "XVMULWOD_D_WU_W",
    0xFFFFFFFF,
    0x74A3046A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_H_B,
    "xvmulwod.h.b",
    "XVMULWOD_H_B",
    0xFFFFFFFF,
    0x74920A50,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_H_BU,
    "xvmulwod.h.bu",
    "XVMULWOD_H_BU",
    0xFFFFFFFF,
    0x749A1F53,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_H_BU_B,
    "xvmulwod.h.bu.b",
    "XVMULWOD_H_BU_B",
    0xFFFFFFFF,
    0x74A271F8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_Q_D,
    "xvmulwod.q.d",
    "XVMULWOD_Q_D",
    0xFFFFFFFF,
    0x7493BEB4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_Q_DU,
    "xvmulwod.q.du",
    "XVMULWOD_Q_DU",
    0xFFFFFFFF,
    0x749B9FFC,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_Q_DU_D,
    "xvmulwod.q.du.d",
    "XVMULWOD_Q_DU_D",
    0xFFFFFFFF,
    0x74A389EF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_W_H,
    "xvmulwod.w.h",
    "XVMULWOD_W_H",
    0xFFFFFFFF,
    0x7492DC5E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_W_HU,
    "xvmulwod.w.hu",
    "XVMULWOD_W_HU",
    0xFFFFFFFF,
    0x749A9A2E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVMULWOD_W_HU_H,
    "xvmulwod.w.hu.h",
    "XVMULWOD_W_HU_H",
    0xFFFFFFFF,
    0x74A28518,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNEG_B,
    "xvneg.b",
    "XVNEG_B",
    0xFFFFFFFF,
    0x769C3097,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNEG_D,
    "xvneg.d",
    "XVNEG_D",
    0xFFFFFFFF,
    0x769C3E34,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNEG_H,
    "xvneg.h",
    "XVNEG_H",
    0xFFFFFFFF,
    0x769C35C8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNEG_W,
    "xvneg.w",
    "XVNEG_W",
    0xFFFFFFFF,
    0x769C39D7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNOR_V,
    "xvnor.v",
    "XVNOR_V",
    0xFFFFFFFF,
    0x75278EE4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVNORI_B,
    "xvnori.b",
    "XVNORI_B",
    0xFFFFFFFF,
    0x77DF4427,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVOR_V,
    "xvor.v",
    "XVOR_V",
    0xFFFFFFFF,
    0x7526D7A6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVORI_B,
    "xvori.b",
    "XVORI_B",
    0xFFFFFFFF,
    0x77D7BC46,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVORN_V,
    "xvorn.v",
    "XVORN_V",
    0xFFFFFFFF,
    0x752897B1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKEV_B,
    "xvpackev.b",
    "XVPACKEV_B",
    0xFFFFFFFF,
    0x75162055,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKEV_D,
    "xvpackev.d",
    "XVPACKEV_D",
    0xFFFFFFFF,
    0x75179120,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKEV_H,
    "xvpackev.h",
    "XVPACKEV_H",
    0xFFFFFFFF,
    0x75169A48,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKEV_W,
    "xvpackev.w",
    "XVPACKEV_W",
    0xFFFFFFFF,
    0x751778C0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKOD_B,
    "xvpackod.b",
    "XVPACKOD_B",
    0xFFFFFFFF,
    0x75187FBC,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKOD_D,
    "xvpackod.d",
    "XVPACKOD_D",
    0xFFFFFFFF,
    0x75198932,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKOD_H,
    "xvpackod.h",
    "XVPACKOD_H",
    0xFFFFFFFF,
    0x7518994E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPACKOD_W,
    "xvpackod.w",
    "XVPACKOD_W",
    0xFFFFFFFF,
    0x75190AB6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPCNT_B,
    "xvpcnt.b",
    "XVPCNT_B",
    0xFFFFFFFF,
    0x769C2368,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPCNT_D,
    "xvpcnt.d",
    "XVPCNT_D",
    0xFFFFFFFF,
    0x769C2D9A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPCNT_H,
    "xvpcnt.h",
    "XVPCNT_H",
    0xFFFFFFFF,
    0x769C248C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPCNT_W,
    "xvpcnt.w",
    "XVPCNT_W",
    0xFFFFFFFF,
    0x769C2AFF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPERM_W,
    "xvperm.w",
    "XVPERM_W",
    0xFFFFFFFF,
    0x757D42F8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPERMI_D,
    "xvpermi.d",
    "XVPERMI_D",
    0xFFFFFFFF,
    0x77EA0CD1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPERMI_Q,
    "xvpermi.q",
    "XVPERMI_Q",
    0xFFFFFFFF,
    0x77EEE1EA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPERMI_W,
    "xvpermi.w",
    "XVPERMI_W",
    0xFFFFFFFF,
    0x77E59587,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKEV_B,
    "xvpickev.b",
    "XVPICKEV_B",
    0xFFFFFFFF,
    0x751E1B76,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKEV_D,
    "xvpickev.d",
    "XVPICKEV_D",
    0xFFFFFFFF,
    0x751FA701,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKEV_H,
    "xvpickev.h",
    "XVPICKEV_H",
    0xFFFFFFFF,
    0x751E8D6E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKEV_W,
    "xvpickev.w",
    "XVPICKEV_W",
    0xFFFFFFFF,
    0x751F379E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKOD_B,
    "xvpickod.b",
    "XVPICKOD_B",
    0xFFFFFFFF,
    0x75203ECE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKOD_D,
    "xvpickod.d",
    "XVPICKOD_D",
    0xFFFFFFFF,
    0x7521C0AA,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKOD_H,
    "xvpickod.h",
    "XVPICKOD_H",
    0xFFFFFFFF,
    0x7520B2BF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKOD_W,
    "xvpickod.w",
    "XVPICKOD_W",
    0xFFFFFFFF,
    0x7521781F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE_D,
    "xvpickve.d",
    "XVPICKVE_D",
    0xFFFFFFFF,
    0x7703E02D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE_W,
    "xvpickve.w",
    "XVPICKVE_W",
    0xFFFFFFFF,
    0x7703C799,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE2GR_D,
    "xvpickve2gr.d",
    "XVPICKVE2GR_D",
    0xFFFFFFFF,
    0x76EFE0C8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE2GR_DU,
    "xvpickve2gr.du",
    "XVPICKVE2GR_DU",
    0xFFFFFFFF,
    0x76F3E10A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE2GR_W,
    "xvpickve2gr.w",
    "XVPICKVE2GR_W",
    0xFFFFFFFF,
    0x76EFD96E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVPICKVE2GR_WU,
    "xvpickve2gr.wu",
    "XVPICKVE2GR_WU",
    0xFFFFFFFF,
    0x76F3D02C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPL128VEI_B,
    "xvrepl128vei.b",
    "XVREPL128VEI_B",
    0xFFFFFFFF,
    0x76F78A6A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPL128VEI_D,
    "xvrepl128vei.d",
    "XVREPL128VEI_D",
    0xFFFFFFFF,
    0x76F7F2FF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPL128VEI_H,
    "xvrepl128vei.h",
    "XVREPL128VEI_H",
    0xFFFFFFFF,
    0x76F7CA66,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPL128VEI_W,
    "xvrepl128vei.w",
    "XVREPL128VEI_W",
    0xFFFFFFFF,
    0x76F7E5AB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLGR2VR_B,
    "xvreplgr2vr.b",
    "XVREPLGR2VR_B",
    0xFFFFFFFF,
    0x769F0210,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLGR2VR_D,
    "xvreplgr2vr.d",
    "XVREPLGR2VR_D",
    0xFFFFFFFF,
    0x769F0F10,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLGR2VR_H,
    "xvreplgr2vr.h",
    "XVREPLGR2VR_H",
    0xFFFFFFFF,
    0x769F06C7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLGR2VR_W,
    "xvreplgr2vr.w",
    "XVREPLGR2VR_W",
    0xFFFFFFFF,
    0x769F09E4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE_B,
    "xvreplve.b",
    "XVREPLVE_B",
    0xFFFFFFFF,
    0x75222E14,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE_D,
    "xvreplve.d",
    "XVREPLVE_D",
    0xFFFFFFFF,
    0x7523DC64,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE_H,
    "xvreplve.h",
    "XVREPLVE_H",
    0xFFFFFFFF,
    0x7522E2A0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE_W,
    "xvreplve.w",
    "XVREPLVE_W",
    0xFFFFFFFF,
    0x75234A54,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE0_B,
    "xvreplve0.b",
    "XVREPLVE0_B",
    0xFFFFFFFF,
    0x7707028B,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE0_D,
    "xvreplve0.d",
    "XVREPLVE0_D",
    0xFFFFFFFF,
    0x7707E094,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE0_H,
    "xvreplve0.h",
    "XVREPLVE0_H",
    0xFFFFFFFF,
    0x7707834D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE0_Q,
    "xvreplve0.q",
    "XVREPLVE0_Q",
    0xFFFFFFFF,
    0x7707F291,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVREPLVE0_W,
    "xvreplve0.w",
    "XVREPLVE0_W",
    0xFFFFFFFF,
    0x7707C188,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTR_B,
    "xvrotr.b",
    "XVROTR_B",
    0xFFFFFFFF,
    0x74EE78C0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTR_D,
    "xvrotr.d",
    "XVROTR_D",
    0xFFFFFFFF,
    0x74EFAEEB,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTR_H,
    "xvrotr.h",
    "XVROTR_H",
    0xFFFFFFFF,
    0x74EEAA33,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTR_W,
    "xvrotr.w",
    "XVROTR_W",
    0xFFFFFFFF,
    0x74EF1C52,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTRI_B,
    "xvrotri.b",
    "XVROTRI_B",
    0xFFFFFFFF,
    0x76A02CA1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTRI_D,
    "xvrotri.d",
    "XVROTRI_D",
    0xFFFFFFFF,
    0x76A19707,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTRI_H,
    "xvrotri.h",
    "XVROTRI_H",
    0xFFFFFFFF,
    0x76A04E21,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVROTRI_W,
    "xvrotri.w",
    "XVROTRI_W",
    0xFFFFFFFF,
    0x76A0CEF9,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_B,
    "xvsadd.b",
    "XVSADD_B",
    0xFFFFFFFF,
    0x74465BDB,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_BU,
    "xvsadd.bu",
    "XVSADD_BU",
    0xFFFFFFFF,
    0x744A729D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_D,
    "xvsadd.d",
    "XVSADD_D",
    0xFFFFFFFF,
    0x7447EA45,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_DU,
    "xvsadd.du",
    "XVSADD_DU",
    0xFFFFFFFF,
    0x744BBB12,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_H,
    "xvsadd.h",
    "XVSADD_H",
    0xFFFFFFFF,
    0x7446841D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_HU,
    "xvsadd.hu",
    "XVSADD_HU",
    0xFFFFFFFF,
    0x744A9A07,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_W,
    "xvsadd.w",
    "XVSADD_W",
    0xFFFFFFFF,
    0x74477F96,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSADD_WU,
    "xvsadd.wu",
    "XVSADD_WU",
    0xFFFFFFFF,
    0x744B3D42,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_B,
    "xvsat.b",
    "XVSAT_B",
    0xFFFFFFFF,
    0x772428F6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_BU,
    "xvsat.bu",
    "XVSAT_BU",
    0xFFFFFFFF,
    0x772830C6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_D,
    "xvsat.d",
    "XVSAT_D",
    0xFFFFFFFF,
    0x77250503,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_DU,
    "xvsat.du",
    "XVSAT_DU",
    0xFFFFFFFF,
    0x77291E85,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_H,
    "xvsat.h",
    "XVSAT_H",
    0xFFFFFFFF,
    0x77245403,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_HU,
    "xvsat.hu",
    "XVSAT_HU",
    0xFFFFFFFF,
    0x7728732C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_W,
    "xvsat.w",
    "XVSAT_W",
    0xFFFFFFFF,
    0x77248209,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSAT_WU,
    "xvsat.wu",
    "XVSAT_WU",
    0xFFFFFFFF,
    0x77288C34,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQ_B,
    "xvseq.b",
    "XVSEQ_B",
    0xFFFFFFFF,
    0x74004C83,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQ_D,
    "xvseq.d",
    "XVSEQ_D",
    0xFFFFFFFF,
    0x7401B5A8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQ_H,
    "xvseq.h",
    "XVSEQ_H",
    0xFFFFFFFF,
    0x740096A0,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQ_W,
    "xvseq.w",
    "XVSEQ_W",
    0xFFFFFFFF,
    0x74014E06,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQI_B,
    "xvseqi.b",
    "XVSEQI_B",
    0xFFFFFFFF,
    0x7680032C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQI_D,
    "xvseqi.d",
    "XVSEQI_D",
    0xFFFFFFFF,
    0x76819CEB,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQI_H,
    "xvseqi.h",
    "XVSEQI_H",
    0xFFFFFFFF,
    0x7680A889,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSEQI_W,
    "xvseqi.w",
    "XVSEQI_W",
    0xFFFFFFFF,
    0x76815099,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETALLNEZ_B,
    "xvsetallnez.b",
    "XVSETALLNEZ_B",
    0xFFFFFFFF,
    0x769CB3A5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETALLNEZ_D,
    "xvsetallnez.d",
    "XVSETALLNEZ_D",
    0xFFFFFFFF,
    0x769CBE87,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETALLNEZ_H,
    "xvsetallnez.h",
    "XVSETALLNEZ_H",
    0xFFFFFFFF,
    0x769CB485,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETALLNEZ_W,
    "xvsetallnez.w",
    "XVSETALLNEZ_W",
    0xFFFFFFFF,
    0x769CB8A4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETANYEQZ_B,
    "xvsetanyeqz.b",
    "XVSETANYEQZ_B",
    0xFFFFFFFF,
    0x769CA105,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETANYEQZ_D,
    "xvsetanyeqz.d",
    "XVSETANYEQZ_D",
    0xFFFFFFFF,
    0x769CAE26,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETANYEQZ_H,
    "xvsetanyeqz.h",
    "XVSETANYEQZ_H",
    0xFFFFFFFF,
    0x769CA685,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETANYEQZ_W,
    "xvsetanyeqz.w",
    "XVSETANYEQZ_W",
    0xFFFFFFFF,
    0x769CA8C7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETEQZ_V,
    "xvseteqz.v",
    "XVSETEQZ_V",
    0xFFFFFFFF,
    0x769C9827,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSETNEZ_V,
    "xvsetnez.v",
    "XVSETNEZ_V",
    0xFFFFFFFF,
    0x769C9DA7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSHUF4I_B,
    "xvshuf4i.b",
    "XVSHUF4I_B",
    0xFFFFFFFF,
    0x7792A395,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSHUF4I_D,
    "xvshuf4i.d",
    "XVSHUF4I_D",
    0xFFFFFFFF,
    0x779D8C98,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSHUF4I_H,
    "xvshuf4i.h",
    "XVSHUF4I_H",
    0xFFFFFFFF,
    0x77945872,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSHUF4I_W,
    "xvshuf4i.w",
    "XVSHUF4I_W",
    0xFFFFFFFF,
    0x77994B20,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSIGNCOV_B,
    "xvsigncov.b",
    "XVSIGNCOV_B",
    0xFFFFFFFF,
    0x752E3701,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSIGNCOV_D,
    "xvsigncov.d",
    "XVSIGNCOV_D",
    0xFFFFFFFF,
    0x752FFE3A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSIGNCOV_H,
    "xvsigncov.h",
    "XVSIGNCOV_H",
    0xFFFFFFFF,
    0x752EBAE8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSIGNCOV_W,
    "xvsigncov.w",
    "XVSIGNCOV_W",
    0xFFFFFFFF,
    0x752F2B23,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_B,
    "xvsle.b",
    "XVSLE_B",
    0xFFFFFFFF,
    0x740277D8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_BU,
    "xvsle.bu",
    "XVSLE_BU",
    0xFFFFFFFF,
    0x74040B69,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_D,
    "xvsle.d",
    "XVSLE_D",
    0xFFFFFFFF,
    0x7403A34D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_DU,
    "xvsle.du",
    "XVSLE_DU",
    0xFFFFFFFF,
    0x7405C8C5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_H,
    "xvsle.h",
    "XVSLE_H",
    0xFFFFFFFF,
    0x7402D1B7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_HU,
    "xvsle.hu",
    "XVSLE_HU",
    0xFFFFFFFF,
    0x7404DB3D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_W,
    "xvsle.w",
    "XVSLE_W",
    0xFFFFFFFF,
    0x740363EA,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLE_WU,
    "xvsle.wu",
    "XVSLE_WU",
    0xFFFFFFFF,
    0x74053B30,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_B,
    "xvslei.b",
    "XVSLEI_B",
    0xFFFFFFFF,
    0x7682596E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_BU,
    "xvslei.bu",
    "XVSLEI_BU",
    0xFFFFFFFF,
    0x76842B51,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_D,
    "xvslei.d",
    "XVSLEI_D",
    0xFFFFFFFF,
    0x7683ABD3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_DU,
    "xvslei.du",
    "XVSLEI_DU",
    0xFFFFFFFF,
    0x7685E3F9,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_H,
    "xvslei.h",
    "XVSLEI_H",
    0xFFFFFFFF,
    0x7682BEC2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_HU,
    "xvslei.hu",
    "XVSLEI_HU",
    0xFFFFFFFF,
    0x7684C974,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_W,
    "xvslei.w",
    "XVSLEI_W",
    0xFFFFFFFF,
    0x768331C3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLEI_WU,
    "xvslei.wu",
    "XVSLEI_WU",
    0xFFFFFFFF,
    0x76852BA1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLL_B,
    "xvsll.b",
    "XVSLL_B",
    0xFFFFFFFF,
    0x74E827A8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLL_D,
    "xvsll.d",
    "XVSLL_D",
    0xFFFFFFFF,
    0x74E9E8D3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLL_H,
    "xvsll.h",
    "XVSLL_H",
    0xFFFFFFFF,
    0x74E8F795,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLL_W,
    "xvsll.w",
    "XVSLL_W",
    0xFFFFFFFF,
    0x74E92BD1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLI_B,
    "xvslli.b",
    "XVSLLI_B",
    0xFFFFFFFF,
    0x772C2759,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLI_D,
    "xvslli.d",
    "XVSLLI_D",
    0xFFFFFFFF,
    0x772DBB8A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLI_H,
    "xvslli.h",
    "XVSLLI_H",
    0xFFFFFFFF,
    0x772C7B91,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLI_W,
    "xvslli.w",
    "XVSLLI_W",
    0xFFFFFFFF,
    0x772CF7FA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_D_W,
    "xvsllwil.d.w",
    "XVSLLWIL_D_W",
    0xFFFFFFFF,
    0x7708E283,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_DU_WU,
    "xvsllwil.du.wu",
    "XVSLLWIL_DU_WU",
    0xFFFFFFFF,
    0x770CFCA3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_H_B,
    "xvsllwil.h.b",
    "XVSLLWIL_H_B",
    0xFFFFFFFF,
    0x77083AAD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_HU_BU,
    "xvsllwil.hu.bu",
    "XVSLLWIL_HU_BU",
    0xFFFFFFFF,
    0x770C39EF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_W_H,
    "xvsllwil.w.h",
    "XVSLLWIL_W_H",
    0xFFFFFFFF,
    0x770843B4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLLWIL_WU_HU,
    "xvsllwil.wu.hu",
    "XVSLLWIL_WU_HU",
    0xFFFFFFFF,
    0x770C43B6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_B,
    "xvslt.b",
    "XVSLT_B",
    0xFFFFFFFF,
    0x740637FE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_BU,
    "xvslt.bu",
    "XVSLT_BU",
    0xFFFFFFFF,
    0x740875B4,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_D,
    "xvslt.d",
    "XVSLT_D",
    0xFFFFFFFF,
    0x7407FD43,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_DU,
    "xvslt.du",
    "XVSLT_DU",
    0xFFFFFFFF,
    0x74098E9E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_H,
    "xvslt.h",
    "XVSLT_H",
    0xFFFFFFFF,
    0x740682F3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_HU,
    "xvslt.hu",
    "XVSLT_HU",
    0xFFFFFFFF,
    0x7408EBAC,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_W,
    "xvslt.w",
    "XVSLT_W",
    0xFFFFFFFF,
    0x74070F57,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLT_WU,
    "xvslt.wu",
    "XVSLT_WU",
    0xFFFFFFFF,
    0x74097F3A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_B,
    "xvslti.b",
    "XVSLTI_B",
    0xFFFFFFFF,
    0x76861B7F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_BU,
    "xvslti.bu",
    "XVSLTI_BU",
    0xFFFFFFFF,
    0x76880881,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_D,
    "xvslti.d",
    "XVSLTI_D",
    0xFFFFFFFF,
    0x76878A4D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_DU,
    "xvslti.du",
    "XVSLTI_DU",
    0xFFFFFFFF,
    0x7689F4AA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_H,
    "xvslti.h",
    "XVSLTI_H",
    0xFFFFFFFF,
    0x76869A65,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_HU,
    "xvslti.hu",
    "XVSLTI_HU",
    0xFFFFFFFF,
    0x7688D0A0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_W,
    "xvslti.w",
    "XVSLTI_W",
    0xFFFFFFFF,
    0x76872D14,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSLTI_WU,
    "xvslti.wu",
    "XVSLTI_WU",
    0xFFFFFFFF,
    0x76896320,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRA_B,
    "xvsra.b",
    "XVSRA_B",
    0xFFFFFFFF,
    0x74EC004B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRA_D,
    "xvsra.d",
    "XVSRA_D",
    0xFFFFFFFF,
    0x74ED85E6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRA_H,
    "xvsra.h",
    "XVSRA_H",
    0xFFFFFFFF,
    0x74EC9B71,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRA_W,
    "xvsra.w",
    "XVSRA_W",
    0xFFFFFFFF,
    0x74ED318D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAI_B,
    "xvsrai.b",
    "XVSRAI_B",
    0xFFFFFFFF,
    0x77342C50,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAI_D,
    "xvsrai.d",
    "XVSRAI_D",
    0xFFFFFFFF,
    0x7735128A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAI_H,
    "xvsrai.h",
    "XVSRAI_H",
    0xFFFFFFFF,
    0x7734706E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAI_W,
    "xvsrai.w",
    "XVSRAI_W",
    0xFFFFFFFF,
    0x7734D651,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAN_B_H,
    "xvsran.b.h",
    "XVSRAN_B_H",
    0xFFFFFFFF,
    0x74F68DBE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAN_H_W,
    "xvsran.h.w",
    "XVSRAN_H_W",
    0xFFFFFFFF,
    0x74F71352,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAN_W_D,
    "xvsran.w.d",
    "XVSRAN_W_D",
    0xFFFFFFFF,
    0x74F7D67B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRANI_B_H,
    "xvsrani.b.h",
    "XVSRANI_B_H",
    0xFFFFFFFF,
    0x77587EEE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRANI_D_Q,
    "xvsrani.d.q",
    "XVSRANI_D_Q",
    0xFFFFFFFF,
    0x775BC4F1,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRANI_H_W,
    "xvsrani.h.w",
    "XVSRANI_H_W",
    0xFFFFFFFF,
    0x77589502,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRANI_W_D,
    "xvsrani.w.d",
    "XVSRANI_W_D",
    0xFFFFFFFF,
    0x77593965,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAR_B,
    "xvsrar.b",
    "XVSRAR_B",
    0xFFFFFFFF,
    0x74F22E49,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAR_D,
    "xvsrar.d",
    "XVSRAR_D",
    0xFFFFFFFF,
    0x74F399F3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAR_H,
    "xvsrar.h",
    "XVSRAR_H",
    0xFFFFFFFF,
    0x74F2874F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRAR_W,
    "xvsrar.w",
    "XVSRAR_W",
    0xFFFFFFFF,
    0x74F33A71,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARI_B,
    "xvsrari.b",
    "XVSRARI_B",
    0xFFFFFFFF,
    0x76A82F8A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARI_D,
    "xvsrari.d",
    "XVSRARI_D",
    0xFFFFFFFF,
    0x76A9213D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARI_H,
    "xvsrari.h",
    "XVSRARI_H",
    0xFFFFFFFF,
    0x76A8783C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARI_W,
    "xvsrari.w",
    "XVSRARI_W",
    0xFFFFFFFF,
    0x76A8B0ED,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARN_B_H,
    "xvsrarn.b.h",
    "XVSRARN_B_H",
    0xFFFFFFFF,
    0x74FABE92,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARN_H_W,
    "xvsrarn.h.w",
    "XVSRARN_H_W",
    0xFFFFFFFF,
    0x74FB102C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARN_W_D,
    "xvsrarn.w.d",
    "XVSRARN_W_D",
    0xFFFFFFFF,
    0x74FBEA49,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARNI_B_H,
    "xvsrarni.b.h",
    "XVSRARNI_B_H",
    0xFFFFFFFF,
    0x775C7FF5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARNI_D_Q,
    "xvsrarni.d.q",
    "XVSRARNI_D_Q",
    0xFFFFFFFF,
    0x775E1CA7,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARNI_H_W,
    "xvsrarni.h.w",
    "XVSRARNI_H_W",
    0xFFFFFFFF,
    0x775CE6C4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRARNI_W_D,
    "xvsrarni.w.d",
    "XVSRARNI_W_D",
    0xFFFFFFFF,
    0x775DA518,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRL_B,
    "xvsrl.b",
    "XVSRL_B",
    0xFFFFFFFF,
    0x74EA7714,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRL_D,
    "xvsrl.d",
    "XVSRL_D",
    0xFFFFFFFF,
    0x74EBEBCD,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRL_H,
    "xvsrl.h",
    "XVSRL_H",
    0xFFFFFFFF,
    0x74EAFE2B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRL_W,
    "xvsrl.w",
    "XVSRL_W",
    0xFFFFFFFF,
    0x74EB2142,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLI_B,
    "xvsrli.b",
    "XVSRLI_B",
    0xFFFFFFFF,
    0x77302C9D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLI_D,
    "xvsrli.d",
    "XVSRLI_D",
    0xFFFFFFFF,
    0x7731B880,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLI_H,
    "xvsrli.h",
    "XVSRLI_H",
    0xFFFFFFFF,
    0x773071DC,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLI_W,
    "xvsrli.w",
    "XVSRLI_W",
    0xFFFFFFFF,
    0x77309E4C,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLN_B_H,
    "xvsrln.b.h",
    "XVSRLN_B_H",
    0xFFFFFFFF,
    0x74F495A7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLN_H_W,
    "xvsrln.h.w",
    "XVSRLN_H_W",
    0xFFFFFFFF,
    0x74F51646,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLN_W_D,
    "xvsrln.w.d",
    "XVSRLN_W_D",
    0xFFFFFFFF,
    0x74F5F18C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLNI_B_H,
    "xvsrlni.b.h",
    "XVSRLNI_B_H",
    0xFFFFFFFF,
    0x77404905,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLNI_D_Q,
    "xvsrlni.d.q",
    "XVSRLNI_D_Q",
    0xFFFFFFFF,
    0x77437F8F,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLNI_H_W,
    "xvsrlni.h.w",
    "XVSRLNI_H_W",
    0xFFFFFFFF,
    0x7740D087,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLNI_W_D,
    "xvsrlni.w.d",
    "XVSRLNI_W_D",
    0xFFFFFFFF,
    0x774145FE,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLR_B,
    "xvsrlr.b",
    "XVSRLR_B",
    0xFFFFFFFF,
    0x74F01572,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLR_D,
    "xvsrlr.d",
    "XVSRLR_D",
    0xFFFFFFFF,
    0x74F19F64,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLR_H,
    "xvsrlr.h",
    "XVSRLR_H",
    0xFFFFFFFF,
    0x74F0D4BF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLR_W,
    "xvsrlr.w",
    "XVSRLR_W",
    0xFFFFFFFF,
    0x74F104A7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRI_B,
    "xvsrlri.b",
    "XVSRLRI_B",
    0xFFFFFFFF,
    0x76A433DD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRI_D,
    "xvsrlri.d",
    "XVSRLRI_D",
    0xFFFFFFFF,
    0x76A5D294,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRI_H,
    "xvsrlri.h",
    "XVSRLRI_H",
    0xFFFFFFFF,
    0x76A478D0,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRI_W,
    "xvsrlri.w",
    "XVSRLRI_W",
    0xFFFFFFFF,
    0x76A4F158,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRN_B_H,
    "xvsrlrn.b.h",
    "XVSRLRN_B_H",
    0xFFFFFFFF,
    0x74F8EB24,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRN_H_W,
    "xvsrlrn.h.w",
    "XVSRLRN_H_W",
    0xFFFFFFFF,
    0x74F904B1,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRN_W_D,
    "xvsrlrn.w.d",
    "XVSRLRN_W_D",
    0xFFFFFFFF,
    0x74F9C43D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRNI_B_H,
    "xvsrlrni.b.h",
    "XVSRLRNI_B_H",
    0xFFFFFFFF,
    0x7744722A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRNI_D_Q,
    "xvsrlrni.d.q",
    "XVSRLRNI_D_Q",
    0xFFFFFFFF,
    0x7746A919,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRNI_H_W,
    "xvsrlrni.h.w",
    "XVSRLRNI_H_W",
    0xFFFFFFFF,
    0x7744B6F6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSRLRNI_W_D,
    "xvsrlrni.w.d",
    "XVSRLRNI_W_D",
    0xFFFFFFFF,
    0x7745EAD2,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_B_H,
    "xvssran.b.h",
    "XVSSRAN_B_H",
    0xFFFFFFFF,
    0x74FE8491,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_BU_H,
    "xvssran.bu.h",
    "XVSSRAN_BU_H",
    0xFFFFFFFF,
    0x7506E183,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_H_W,
    "xvssran.h.w",
    "XVSSRAN_H_W",
    0xFFFFFFFF,
    0x74FF379C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_HU_W,
    "xvssran.hu.w",
    "XVSSRAN_HU_W",
    0xFFFFFFFF,
    0x75070719,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_W_D,
    "xvssran.w.d",
    "XVSSRAN_W_D",
    0xFFFFFFFF,
    0x74FFFC35,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRAN_WU_D,
    "xvssran.wu.d",
    "XVSSRAN_WU_D",
    0xFFFFFFFF,
    0x7507A9DE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_B_H,
    "xvssrani.b.h",
    "XVSSRANI_B_H",
    0xFFFFFFFF,
    0x77607ADA,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_BU_H,
    "xvssrani.bu.h",
    "XVSSRANI_BU_H",
    0xFFFFFFFF,
    0x77646866,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_D_Q,
    "xvssrani.d.q",
    "XVSSRANI_D_Q",
    0xFFFFFFFF,
    0x7762ED49,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_DU_Q,
    "xvssrani.du.q",
    "XVSSRANI_DU_Q",
    0xFFFFFFFF,
    0x77663C50,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_H_W,
    "xvssrani.h.w",
    "XVSSRANI_H_W",
    0xFFFFFFFF,
    0x7760E9D3,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_HU_W,
    "xvssrani.hu.w",
    "XVSSRANI_HU_W",
    0xFFFFFFFF,
    0x77649934,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_W_D,
    "xvssrani.w.d",
    "XVSSRANI_W_D",
    0xFFFFFFFF,
    0x77616F61,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRANI_WU_D,
    "xvssrani.wu.d",
    "XVSSRANI_WU_D",
    0xFFFFFFFF,
    0x77652178,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_B_H,
    "xvssrarn.b.h",
    "XVSSRARN_B_H",
    0xFFFFFFFF,
    0x750281A7,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_BU_H,
    "xvssrarn.bu.h",
    "XVSSRARN_BU_H",
    0xFFFFFFFF,
    0x750A8984,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_H_W,
    "xvssrarn.h.w",
    "XVSSRARN_H_W",
    0xFFFFFFFF,
    0x75033856,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_HU_W,
    "xvssrarn.hu.w",
    "XVSSRARN_HU_W",
    0xFFFFFFFF,
    0x750B0F0F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_W_D,
    "xvssrarn.w.d",
    "XVSSRARN_W_D",
    0xFFFFFFFF,
    0x7503C0ED,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARN_WU_D,
    "xvssrarn.wu.d",
    "XVSSRARN_WU_D",
    0xFFFFFFFF,
    0x750BA13E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_B_H,
    "xvssrarni.b.h",
    "XVSSRARNI_B_H",
    0xFFFFFFFF,
    0x77687480,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_BU_H,
    "xvssrarni.bu.h",
    "XVSSRARNI_BU_H",
    0xFFFFFFFF,
    0x776C4275,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_D_Q,
    "xvssrarni.d.q",
    "XVSSRARNI_D_Q",
    0xFFFFFFFF,
    0x776B4FE8,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_DU_Q,
    "xvssrarni.du.q",
    "XVSSRARNI_DU_Q",
    0xFFFFFFFF,
    0x776F79CF,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_H_W,
    "xvssrarni.h.w",
    "XVSSRARNI_H_W",
    0xFFFFFFFF,
    0x7768A408,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_HU_W,
    "xvssrarni.hu.w",
    "XVSSRARNI_HU_W",
    0xFFFFFFFF,
    0x776C85B6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_W_D,
    "xvssrarni.w.d",
    "XVSSRARNI_W_D",
    0xFFFFFFFF,
    0x7769A8A5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRARNI_WU_D,
    "xvssrarni.wu.d",
    "XVSSRARNI_WU_D",
    0xFFFFFFFF,
    0x776D68B5,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_B_H,
    "xvssrln.b.h",
    "XVSSRLN_B_H",
    0xFFFFFFFF,
    0x74FC9098,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_BU_H,
    "xvssrln.bu.h",
    "XVSSRLN_BU_H",
    0xFFFFFFFF,
    0x7504E93A,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_H_W,
    "xvssrln.h.w",
    "XVSSRLN_H_W",
    0xFFFFFFFF,
    0x74FD01E5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_HU_W,
    "xvssrln.hu.w",
    "XVSSRLN_HU_W",
    0xFFFFFFFF,
    0x75050687,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_W_D,
    "xvssrln.w.d",
    "XVSSRLN_W_D",
    0xFFFFFFFF,
    0x74FDFB20,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLN_WU_D,
    "xvssrln.wu.d",
    "XVSSRLN_WU_D",
    0xFFFFFFFF,
    0x7505D1AF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_B_H,
    "xvssrlni.b.h",
    "XVSSRLNI_B_H",
    0xFFFFFFFF,
    0x77486653,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_BU_H,
    "xvssrlni.bu.h",
    "XVSSRLNI_BU_H",
    0xFFFFFFFF,
    0x774C5559,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_D_Q,
    "xvssrlni.d.q",
    "XVSSRLNI_D_Q",
    0xFFFFFFFF,
    0x774BE568,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_DU_Q,
    "xvssrlni.du.q",
    "XVSSRLNI_DU_Q",
    0xFFFFFFFF,
    0x774EAC88,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_H_W,
    "xvssrlni.h.w",
    "XVSSRLNI_H_W",
    0xFFFFFFFF,
    0x77488FBD,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_HU_W,
    "xvssrlni.hu.w",
    "XVSSRLNI_HU_W",
    0xFFFFFFFF,
    0x774CEA49,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_W_D,
    "xvssrlni.w.d",
    "XVSSRLNI_W_D",
    0xFFFFFFFF,
    0x7749ADE9,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLNI_WU_D,
    "xvssrlni.wu.d",
    "XVSSRLNI_WU_D",
    0xFFFFFFFF,
    0x774D36D4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_B_H,
    "xvssrlrn.b.h",
    "XVSSRLRN_B_H",
    0xFFFFFFFF,
    0x7500CA88,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_BU_H,
    "xvssrlrn.bu.h",
    "XVSSRLRN_BU_H",
    0xFFFFFFFF,
    0x7508CAEF,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_H_W,
    "xvssrlrn.h.w",
    "XVSSRLRN_H_W",
    0xFFFFFFFF,
    0x75014DA2,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_HU_W,
    "xvssrlrn.hu.w",
    "XVSSRLRN_HU_W",
    0xFFFFFFFF,
    0x750941D6,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_W_D,
    "xvssrlrn.w.d",
    "XVSSRLRN_W_D",
    0xFFFFFFFF,
    0x750194F8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRN_WU_D,
    "xvssrlrn.wu.d",
    "XVSSRLRN_WU_D",
    0xFFFFFFFF,
    0x75099794,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_B_H,
    "xvssrlrni.b.h",
    "XVSSRLRNI_B_H",
    0xFFFFFFFF,
    0x7750635A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_BU_H,
    "xvssrlrni.bu.h",
    "XVSSRLRNI_BU_H",
    0xFFFFFFFF,
    0x77544F97,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_D_Q,
    "xvssrlrni.d.q",
    "XVSSRLRNI_D_Q",
    0xFFFFFFFF,
    0x77530208,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_DU_Q,
    "xvssrlrni.du.q",
    "XVSSRLRNI_DU_Q",
    0xFFFFFFFF,
    0x7756B132,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_H_W,
    "xvssrlrni.h.w",
    "XVSSRLRNI_H_W",
    0xFFFFFFFF,
    0x7750CC06,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_HU_W,
    "xvssrlrni.hu.w",
    "XVSSRLRNI_HU_W",
    0xFFFFFFFF,
    0x7754C959,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_W_D,
    "xvssrlrni.w.d",
    "XVSSRLRNI_W_D",
    0xFFFFFFFF,
    0x7751DDFC,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSRLRNI_WU_D,
    "xvssrlrni.wu.d",
    "XVSSRLRNI_WU_D",
    0xFFFFFFFF,
    0x77553F90,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_B,
    "xvssub.b",
    "XVSSUB_B",
    0xFFFFFFFF,
    0x7448626E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_BU,
    "xvssub.bu",
    "XVSSUB_BU",
    0xFFFFFFFF,
    0x744C45AB,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_D,
    "xvssub.d",
    "XVSSUB_D",
    0xFFFFFFFF,
    0x74498A1C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_DU,
    "xvssub.du",
    "XVSSUB_DU",
    0xFFFFFFFF,
    0x744DEF52,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_H,
    "xvssub.h",
    "XVSSUB_H",
    0xFFFFFFFF,
    0x7448CD0D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_HU,
    "xvssub.hu",
    "XVSSUB_HU",
    0xFFFFFFFF,
    0x744CF150,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_W,
    "xvssub.w",
    "XVSSUB_W",
    0xFFFFFFFF,
    0x7449737C,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSSUB_WU,
    "xvssub.wu",
    "XVSSUB_WU",
    0xFFFFFFFF,
    0x744D3415,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVST,
    "xvst",
    "XVST",
    0xFFFFFFFF,
    0x2CCEBD8E,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSTELM_B,
    "xvstelm.b",
    "XVSTELM_B",
    0xFFFFFFFF,
    0x33AA5C54,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSTELM_D,
    "xvstelm.d",
    "XVSTELM_D",
    0xFFFFFFFF,
    0x331DE3D6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSTELM_H,
    "xvstelm.h",
    "XVSTELM_H",
    0xFFFFFFFF,
    0x33514028,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSTELM_W,
    "xvstelm.w",
    "XVSTELM_W",
    0xFFFFFFFF,
    0x33219E53,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSTX,
    "xvstx",
    "XVSTX",
    0xFFFFFFFF,
    0x384C5527,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUB_B,
    "xvsub.b",
    "XVSUB_B",
    0xFFFFFFFF,
    0x740C438B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUB_D,
    "xvsub.d",
    "XVSUB_D",
    0xFFFFFFFF,
    0x740D9DA5,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUB_H,
    "xvsub.h",
    "XVSUB_H",
    0xFFFFFFFF,
    0x740CE06B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUB_Q,
    "xvsub.q",
    "XVSUB_Q",
    0xFFFFFFFF,
    0x752DFF4D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUB_W,
    "xvsub.w",
    "XVSUB_W",
    0xFFFFFFFF,
    0x740D1AEE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBI_BU,
    "xvsubi.bu",
    "XVSUBI_BU",
    0xFFFFFFFF,
    0x768C0772,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBI_DU,
    "xvsubi.du",
    "XVSUBI_DU",
    0xFFFFFFFF,
    0x768DBB9A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBI_HU,
    "xvsubi.hu",
    "XVSUBI_HU",
    0xFFFFFFFF,
    0x768CCEE6,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBI_WU,
    "xvsubi.wu",
    "XVSUBI_WU",
    0xFFFFFFFF,
    0x768D146D,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_D_W,
    "xvsubwev.d.w",
    "XVSUBWEV_D_W",
    0xFFFFFFFF,
    0x74212C86,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_D_WU,
    "xvsubwev.d.wu",
    "XVSUBWEV_D_WU",
    0xFFFFFFFF,
    0x74315C3F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_H_B,
    "xvsubwev.h.b",
    "XVSUBWEV_H_B",
    0xFFFFFFFF,
    0x7420703D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_H_BU,
    "xvsubwev.h.bu",
    "XVSUBWEV_H_BU",
    0xFFFFFFFF,
    0x74300A81,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_Q_D,
    "xvsubwev.q.d",
    "XVSUBWEV_Q_D",
    0xFFFFFFFF,
    0x7421B7FB,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_Q_DU,
    "xvsubwev.q.du",
    "XVSUBWEV_Q_DU",
    0xFFFFFFFF,
    0x7431C79F,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_W_H,
    "xvsubwev.w.h",
    "XVSUBWEV_W_H",
    0xFFFFFFFF,
    0x7420FE98,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWEV_W_HU,
    "xvsubwev.w.hu",
    "XVSUBWEV_W_HU",
    0xFFFFFFFF,
    0x7430B0D3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_D_W,
    "xvsubwod.d.w",
    "XVSUBWOD_D_W",
    0xFFFFFFFF,
    0x74250DC8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_D_WU,
    "xvsubwod.d.wu",
    "XVSUBWOD_D_WU",
    0xFFFFFFFF,
    0x74356B01,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_H_B,
    "xvsubwod.h.b",
    "XVSUBWOD_H_B",
    0xFFFFFFFF,
    0x74244523,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_H_BU,
    "xvsubwod.h.bu",
    "XVSUBWOD_H_BU",
    0xFFFFFFFF,
    0x7434045B,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_Q_D,
    "xvsubwod.q.d",
    "XVSUBWOD_Q_D",
    0xFFFFFFFF,
    0x7425C9F8,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_Q_DU,
    "xvsubwod.q.du",
    "XVSUBWOD_Q_DU",
    0xFFFFFFFF,
    0x74359F5D,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_W_H,
    "xvsubwod.w.h",
    "XVSUBWOD_W_H",
    0xFFFFFFFF,
    0x7424D4AE,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVSUBWOD_W_HU,
    "xvsubwod.w.hu",
    "XVSUBWOD_W_HU",
    0xFFFFFFFF,
    0x7434D8F3,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVXOR_V,
    "xvxor.v",
    "XVXOR_V",
    0xFFFFFFFF,
    0x75272B4E,
    &FMT_R3,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

loongarch_insn!(
    XVXORI_B,
    "xvxori.b",
    "XVXORI_B",
    0xFFFFFFFF,
    0x77DA551A,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Xr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Vector, InstructionGroup::Vector256]
);

pub static VECTOR_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    VEXT2XV_D_B,
    VEXT2XV_D_H,
    VEXT2XV_D_W,
    VEXT2XV_DU_BU,
    VEXT2XV_DU_HU,
    VEXT2XV_DU_WU,
    VEXT2XV_H_B,
    VEXT2XV_HU_BU,
    VEXT2XV_W_B,
    VEXT2XV_W_H,
    VEXT2XV_WU_BU,
    VEXT2XV_WU_HU,
    VREPLVEI_B,
    VREPLVEI_D,
    VREPLVEI_H,
    VREPLVEI_W,
    XVABSD_B,
    XVABSD_BU,
    XVABSD_D,
    XVABSD_DU,
    XVABSD_H,
    XVABSD_HU,
    XVABSD_W,
    XVABSD_WU,
    XVADD_B,
    XVADD_D,
    XVADD_H,
    XVADD_Q,
    XVADD_W,
    XVADDA_B,
    XVADDA_D,
    XVADDA_H,
    XVADDA_W,
    XVADDI_BU,
    XVADDI_DU,
    XVADDI_HU,
    XVADDI_WU,
    XVADDWEV_D_W,
    XVADDWEV_D_WU,
    XVADDWEV_D_WU_W,
    XVADDWEV_H_B,
    XVADDWEV_H_BU,
    XVADDWEV_H_BU_B,
    XVADDWEV_Q_D,
    XVADDWEV_Q_DU,
    XVADDWEV_Q_DU_D,
    XVADDWEV_W_H,
    XVADDWEV_W_HU,
    XVADDWEV_W_HU_H,
    XVADDWOD_D_W,
    XVADDWOD_D_WU,
    XVADDWOD_D_WU_W,
    XVADDWOD_H_B,
    XVADDWOD_H_BU,
    XVADDWOD_H_BU_B,
    XVADDWOD_Q_D,
    XVADDWOD_Q_DU,
    XVADDWOD_Q_DU_D,
    XVADDWOD_W_H,
    XVADDWOD_W_HU,
    XVADDWOD_W_HU_H,
    XVAND_V,
    XVANDI_B,
    XVANDN_V,
    XVAVG_B,
    XVAVG_BU,
    XVAVG_D,
    XVAVG_DU,
    XVAVG_H,
    XVAVG_HU,
    XVAVG_W,
    XVAVG_WU,
    XVAVGR_B,
    XVAVGR_BU,
    XVAVGR_D,
    XVAVGR_DU,
    XVAVGR_H,
    XVAVGR_HU,
    XVAVGR_W,
    XVAVGR_WU,
    XVBITCLR_B,
    XVBITCLR_D,
    XVBITCLR_H,
    XVBITCLR_W,
    XVBITCLRI_B,
    XVBITCLRI_D,
    XVBITCLRI_H,
    XVBITCLRI_W,
    XVBITREV_B,
    XVBITREV_D,
    XVBITREV_H,
    XVBITREV_W,
    XVBITREVI_B,
    XVBITREVI_D,
    XVBITREVI_H,
    XVBITREVI_W,
    XVBITSEL_V,
    XVBITSELI_B,
    XVBITSET_B,
    XVBITSET_D,
    XVBITSET_H,
    XVBITSET_W,
    XVBITSETI_B,
    XVBITSETI_D,
    XVBITSETI_H,
    XVBITSETI_W,
    XVBSLL_V,
    XVBSRL_V,
    XVCLO_B,
    XVCLO_D,
    XVCLO_H,
    XVCLO_W,
    XVCLZ_B,
    XVCLZ_D,
    XVCLZ_H,
    XVCLZ_W,
    XVDIV_B,
    XVDIV_BU,
    XVDIV_D,
    XVDIV_DU,
    XVDIV_H,
    XVDIV_HU,
    XVDIV_W,
    XVDIV_WU,
    XVEXTH_D_W,
    XVEXTH_DU_WU,
    XVEXTH_H_B,
    XVEXTH_HU_BU,
    XVEXTH_Q_D,
    XVEXTH_QU_DU,
    XVEXTH_W_H,
    XVEXTH_WU_HU,
    XVEXTL_Q_D,
    XVEXTL_QU_DU,
    XVEXTRINS_B,
    XVEXTRINS_D,
    XVEXTRINS_H,
    XVEXTRINS_W,
    XVFADD_D,
    XVFADD_S,
    XVFCLASS_D,
    XVFCLASS_S,
    XVFCMP_CAF_D,
    XVFCMP_CAF_S,
    XVFCMP_CEQ_D,
    XVFCMP_CEQ_S,
    XVFCMP_CLE_D,
    XVFCMP_CLE_S,
    XVFCMP_CLT_D,
    XVFCMP_CLT_S,
    XVFCMP_CNE_D,
    XVFCMP_CNE_S,
    XVFCMP_COR_D,
    XVFCMP_COR_S,
    XVFCMP_CUEQ_D,
    XVFCMP_CUEQ_S,
    XVFCMP_CULE_D,
    XVFCMP_CULE_S,
    XVFCMP_CULT_D,
    XVFCMP_CULT_S,
    XVFCMP_CUN_D,
    XVFCMP_CUN_S,
    XVFCMP_CUNE_D,
    XVFCMP_CUNE_S,
    XVFCMP_SAF_D,
    XVFCMP_SAF_S,
    XVFCMP_SEQ_D,
    XVFCMP_SEQ_S,
    XVFCMP_SLE_D,
    XVFCMP_SLE_S,
    XVFCMP_SLT_D,
    XVFCMP_SLT_S,
    XVFCMP_SNE_D,
    XVFCMP_SNE_S,
    XVFCMP_SOR_D,
    XVFCMP_SOR_S,
    XVFCMP_SUEQ_D,
    XVFCMP_SUEQ_S,
    XVFCMP_SULE_D,
    XVFCMP_SULE_S,
    XVFCMP_SULT_D,
    XVFCMP_SULT_S,
    XVFCMP_SUN_D,
    XVFCMP_SUN_S,
    XVFCMP_SUNE_D,
    XVFCMP_SUNE_S,
    XVFCVT_H_S,
    XVFCVT_S_D,
    XVFCVTH_D_S,
    XVFCVTH_S_H,
    XVFCVTL_D_S,
    XVFCVTL_S_H,
    XVFDIV_D,
    XVFDIV_S,
    XVFFINT_D_L,
    XVFFINT_D_LU,
    XVFFINT_S_L,
    XVFFINT_S_W,
    XVFFINT_S_WU,
    XVFFINTH_D_W,
    XVFFINTL_D_W,
    XVFLOGB_D,
    XVFLOGB_S,
    XVFMADD_D,
    XVFMADD_S,
    XVFMAX_D,
    XVFMAX_S,
    XVFMAXA_D,
    XVFMAXA_S,
    XVFMIN_D,
    XVFMIN_S,
    XVFMINA_D,
    XVFMINA_S,
    XVFMSUB_D,
    XVFMSUB_S,
    XVFMUL_D,
    XVFMUL_S,
    XVFNMADD_D,
    XVFNMADD_S,
    XVFNMSUB_D,
    XVFNMSUB_S,
    XVFRECIP_D,
    XVFRECIP_S,
    XVFRECIPE_D,
    XVFRECIPE_S,
    XVFRINT_D,
    XVFRINT_S,
    XVFRINTRM_D,
    XVFRINTRM_S,
    XVFRINTRNE_D,
    XVFRINTRNE_S,
    XVFRINTRP_D,
    XVFRINTRP_S,
    XVFRINTRZ_D,
    XVFRINTRZ_S,
    XVFRSQRT_D,
    XVFRSQRT_S,
    XVFRSQRTE_D,
    XVFRSQRTE_S,
    XVFRSTP_B,
    XVFRSTP_H,
    XVFRSTPI_B,
    XVFRSTPI_H,
    XVFSQRT_D,
    XVFSQRT_S,
    XVFSUB_D,
    XVFSUB_S,
    XVFTINT_L_D,
    XVFTINT_LU_D,
    XVFTINT_W_D,
    XVFTINT_W_S,
    XVFTINT_WU_S,
    XVFTINTH_L_S,
    XVFTINTL_L_S,
    XVFTINTRM_L_D,
    XVFTINTRM_W_D,
    XVFTINTRM_W_S,
    XVFTINTRMH_L_S,
    XVFTINTRML_L_S,
    XVFTINTRNE_L_D,
    XVFTINTRNE_W_D,
    XVFTINTRNE_W_S,
    XVFTINTRNEH_L_S,
    XVFTINTRNEL_L_S,
    XVFTINTRP_L_D,
    XVFTINTRP_W_D,
    XVFTINTRP_W_S,
    XVFTINTRPH_L_S,
    XVFTINTRPL_L_S,
    XVFTINTRZ_L_D,
    XVFTINTRZ_LU_D,
    XVFTINTRZ_W_D,
    XVFTINTRZ_W_S,
    XVFTINTRZ_WU_S,
    XVFTINTRZH_L_S,
    XVFTINTRZL_L_S,
    XVHADDW_D_W,
    XVHADDW_DU_WU,
    XVHADDW_H_B,
    XVHADDW_HU_BU,
    XVHADDW_Q_D,
    XVHADDW_QU_DU,
    XVHADDW_W_H,
    XVHADDW_WU_HU,
    XVHSUBW_D_W,
    XVHSUBW_DU_WU,
    XVHSUBW_H_B,
    XVHSUBW_HU_BU,
    XVHSUBW_Q_D,
    XVHSUBW_QU_DU,
    XVHSUBW_W_H,
    XVHSUBW_WU_HU,
    XVILVH_B,
    XVILVH_D,
    XVILVH_H,
    XVILVH_W,
    XVILVL_B,
    XVILVL_D,
    XVILVL_H,
    XVILVL_W,
    XVINSGR2VR_D,
    XVINSGR2VR_W,
    XVINSVE0_D,
    XVINSVE0_W,
    XVLD,
    XVLDREPL_B,
    XVLDREPL_D,
    XVLDREPL_H,
    XVLDREPL_W,
    XVLDX,
    XVMADD_B,
    XVMADD_D,
    XVMADD_H,
    XVMADD_W,
    XVMADDWEV_D_W,
    XVMADDWEV_D_WU,
    XVMADDWEV_D_WU_W,
    XVMADDWEV_H_B,
    XVMADDWEV_H_BU,
    XVMADDWEV_H_BU_B,
    XVMADDWEV_Q_D,
    XVMADDWEV_Q_DU,
    XVMADDWEV_Q_DU_D,
    XVMADDWEV_W_H,
    XVMADDWEV_W_HU,
    XVMADDWEV_W_HU_H,
    XVMADDWOD_D_W,
    XVMADDWOD_D_WU,
    XVMADDWOD_D_WU_W,
    XVMADDWOD_H_B,
    XVMADDWOD_H_BU,
    XVMADDWOD_H_BU_B,
    XVMADDWOD_Q_D,
    XVMADDWOD_Q_DU,
    XVMADDWOD_Q_DU_D,
    XVMADDWOD_W_H,
    XVMADDWOD_W_HU,
    XVMADDWOD_W_HU_H,
    XVMAX_B,
    XVMAX_BU,
    XVMAX_D,
    XVMAX_DU,
    XVMAX_H,
    XVMAX_HU,
    XVMAX_W,
    XVMAX_WU,
    XVMAXI_B,
    XVMAXI_BU,
    XVMAXI_D,
    XVMAXI_DU,
    XVMAXI_H,
    XVMAXI_HU,
    XVMAXI_W,
    XVMAXI_WU,
    XVMIN_B,
    XVMIN_BU,
    XVMIN_D,
    XVMIN_DU,
    XVMIN_H,
    XVMIN_HU,
    XVMIN_W,
    XVMIN_WU,
    XVMINI_B,
    XVMINI_BU,
    XVMINI_D,
    XVMINI_DU,
    XVMINI_H,
    XVMINI_HU,
    XVMINI_W,
    XVMINI_WU,
    XVMOD_B,
    XVMOD_BU,
    XVMOD_D,
    XVMOD_DU,
    XVMOD_H,
    XVMOD_HU,
    XVMOD_W,
    XVMOD_WU,
    XVMSKGEZ_B,
    XVMSKLTZ_B,
    XVMSKLTZ_D,
    XVMSKLTZ_H,
    XVMSKLTZ_W,
    XVMSKNZ_B,
    XVMSUB_B,
    XVMSUB_D,
    XVMSUB_H,
    XVMSUB_W,
    XVMUH_B,
    XVMUH_BU,
    XVMUH_D,
    XVMUH_DU,
    XVMUH_H,
    XVMUH_HU,
    XVMUH_W,
    XVMUH_WU,
    XVMUL_B,
    XVMUL_D,
    XVMUL_H,
    XVMUL_W,
    XVMULWEV_D_W,
    XVMULWEV_D_WU,
    XVMULWEV_D_WU_W,
    XVMULWEV_H_B,
    XVMULWEV_H_BU,
    XVMULWEV_H_BU_B,
    XVMULWEV_Q_D,
    XVMULWEV_Q_DU,
    XVMULWEV_Q_DU_D,
    XVMULWEV_W_H,
    XVMULWEV_W_HU,
    XVMULWEV_W_HU_H,
    XVMULWOD_D_W,
    XVMULWOD_D_WU,
    XVMULWOD_D_WU_W,
    XVMULWOD_H_B,
    XVMULWOD_H_BU,
    XVMULWOD_H_BU_B,
    XVMULWOD_Q_D,
    XVMULWOD_Q_DU,
    XVMULWOD_Q_DU_D,
    XVMULWOD_W_H,
    XVMULWOD_W_HU,
    XVMULWOD_W_HU_H,
    XVNEG_B,
    XVNEG_D,
    XVNEG_H,
    XVNEG_W,
    XVNOR_V,
    XVNORI_B,
    XVOR_V,
    XVORI_B,
    XVORN_V,
    XVPACKEV_B,
    XVPACKEV_D,
    XVPACKEV_H,
    XVPACKEV_W,
    XVPACKOD_B,
    XVPACKOD_D,
    XVPACKOD_H,
    XVPACKOD_W,
    XVPCNT_B,
    XVPCNT_D,
    XVPCNT_H,
    XVPCNT_W,
    XVPERM_W,
    XVPERMI_D,
    XVPERMI_Q,
    XVPERMI_W,
    XVPICKEV_B,
    XVPICKEV_D,
    XVPICKEV_H,
    XVPICKEV_W,
    XVPICKOD_B,
    XVPICKOD_D,
    XVPICKOD_H,
    XVPICKOD_W,
    XVPICKVE_D,
    XVPICKVE_W,
    XVPICKVE2GR_D,
    XVPICKVE2GR_DU,
    XVPICKVE2GR_W,
    XVPICKVE2GR_WU,
    XVREPL128VEI_B,
    XVREPL128VEI_D,
    XVREPL128VEI_H,
    XVREPL128VEI_W,
    XVREPLGR2VR_B,
    XVREPLGR2VR_D,
    XVREPLGR2VR_H,
    XVREPLGR2VR_W,
    XVREPLVE_B,
    XVREPLVE_D,
    XVREPLVE_H,
    XVREPLVE_W,
    XVREPLVE0_B,
    XVREPLVE0_D,
    XVREPLVE0_H,
    XVREPLVE0_Q,
    XVREPLVE0_W,
    XVROTR_B,
    XVROTR_D,
    XVROTR_H,
    XVROTR_W,
    XVROTRI_B,
    XVROTRI_D,
    XVROTRI_H,
    XVROTRI_W,
    XVSADD_B,
    XVSADD_BU,
    XVSADD_D,
    XVSADD_DU,
    XVSADD_H,
    XVSADD_HU,
    XVSADD_W,
    XVSADD_WU,
    XVSAT_B,
    XVSAT_BU,
    XVSAT_D,
    XVSAT_DU,
    XVSAT_H,
    XVSAT_HU,
    XVSAT_W,
    XVSAT_WU,
    XVSEQ_B,
    XVSEQ_D,
    XVSEQ_H,
    XVSEQ_W,
    XVSEQI_B,
    XVSEQI_D,
    XVSEQI_H,
    XVSEQI_W,
    XVSETALLNEZ_B,
    XVSETALLNEZ_D,
    XVSETALLNEZ_H,
    XVSETALLNEZ_W,
    XVSETANYEQZ_B,
    XVSETANYEQZ_D,
    XVSETANYEQZ_H,
    XVSETANYEQZ_W,
    XVSETEQZ_V,
    XVSETNEZ_V,
    XVSHUF4I_B,
    XVSHUF4I_D,
    XVSHUF4I_H,
    XVSHUF4I_W,
    XVSIGNCOV_B,
    XVSIGNCOV_D,
    XVSIGNCOV_H,
    XVSIGNCOV_W,
    XVSLE_B,
    XVSLE_BU,
    XVSLE_D,
    XVSLE_DU,
    XVSLE_H,
    XVSLE_HU,
    XVSLE_W,
    XVSLE_WU,
    XVSLEI_B,
    XVSLEI_BU,
    XVSLEI_D,
    XVSLEI_DU,
    XVSLEI_H,
    XVSLEI_HU,
    XVSLEI_W,
    XVSLEI_WU,
    XVSLL_B,
    XVSLL_D,
    XVSLL_H,
    XVSLL_W,
    XVSLLI_B,
    XVSLLI_D,
    XVSLLI_H,
    XVSLLI_W,
    XVSLLWIL_D_W,
    XVSLLWIL_DU_WU,
    XVSLLWIL_H_B,
    XVSLLWIL_HU_BU,
    XVSLLWIL_W_H,
    XVSLLWIL_WU_HU,
    XVSLT_B,
    XVSLT_BU,
    XVSLT_D,
    XVSLT_DU,
    XVSLT_H,
    XVSLT_HU,
    XVSLT_W,
    XVSLT_WU,
    XVSLTI_B,
    XVSLTI_BU,
    XVSLTI_D,
    XVSLTI_DU,
    XVSLTI_H,
    XVSLTI_HU,
    XVSLTI_W,
    XVSLTI_WU,
    XVSRA_B,
    XVSRA_D,
    XVSRA_H,
    XVSRA_W,
    XVSRAI_B,
    XVSRAI_D,
    XVSRAI_H,
    XVSRAI_W,
    XVSRAN_B_H,
    XVSRAN_H_W,
    XVSRAN_W_D,
    XVSRANI_B_H,
    XVSRANI_D_Q,
    XVSRANI_H_W,
    XVSRANI_W_D,
    XVSRAR_B,
    XVSRAR_D,
    XVSRAR_H,
    XVSRAR_W,
    XVSRARI_B,
    XVSRARI_D,
    XVSRARI_H,
    XVSRARI_W,
    XVSRARN_B_H,
    XVSRARN_H_W,
    XVSRARN_W_D,
    XVSRARNI_B_H,
    XVSRARNI_D_Q,
    XVSRARNI_H_W,
    XVSRARNI_W_D,
    XVSRL_B,
    XVSRL_D,
    XVSRL_H,
    XVSRL_W,
    XVSRLI_B,
    XVSRLI_D,
    XVSRLI_H,
    XVSRLI_W,
    XVSRLN_B_H,
    XVSRLN_H_W,
    XVSRLN_W_D,
    XVSRLNI_B_H,
    XVSRLNI_D_Q,
    XVSRLNI_H_W,
    XVSRLNI_W_D,
    XVSRLR_B,
    XVSRLR_D,
    XVSRLR_H,
    XVSRLR_W,
    XVSRLRI_B,
    XVSRLRI_D,
    XVSRLRI_H,
    XVSRLRI_W,
    XVSRLRN_B_H,
    XVSRLRN_H_W,
    XVSRLRN_W_D,
    XVSRLRNI_B_H,
    XVSRLRNI_D_Q,
    XVSRLRNI_H_W,
    XVSRLRNI_W_D,
    XVSSRAN_B_H,
    XVSSRAN_BU_H,
    XVSSRAN_H_W,
    XVSSRAN_HU_W,
    XVSSRAN_W_D,
    XVSSRAN_WU_D,
    XVSSRANI_B_H,
    XVSSRANI_BU_H,
    XVSSRANI_D_Q,
    XVSSRANI_DU_Q,
    XVSSRANI_H_W,
    XVSSRANI_HU_W,
    XVSSRANI_W_D,
    XVSSRANI_WU_D,
    XVSSRARN_B_H,
    XVSSRARN_BU_H,
    XVSSRARN_H_W,
    XVSSRARN_HU_W,
    XVSSRARN_W_D,
    XVSSRARN_WU_D,
    XVSSRARNI_B_H,
    XVSSRARNI_BU_H,
    XVSSRARNI_D_Q,
    XVSSRARNI_DU_Q,
    XVSSRARNI_H_W,
    XVSSRARNI_HU_W,
    XVSSRARNI_W_D,
    XVSSRARNI_WU_D,
    XVSSRLN_B_H,
    XVSSRLN_BU_H,
    XVSSRLN_H_W,
    XVSSRLN_HU_W,
    XVSSRLN_W_D,
    XVSSRLN_WU_D,
    XVSSRLNI_B_H,
    XVSSRLNI_BU_H,
    XVSSRLNI_D_Q,
    XVSSRLNI_DU_Q,
    XVSSRLNI_H_W,
    XVSSRLNI_HU_W,
    XVSSRLNI_W_D,
    XVSSRLNI_WU_D,
    XVSSRLRN_B_H,
    XVSSRLRN_BU_H,
    XVSSRLRN_H_W,
    XVSSRLRN_HU_W,
    XVSSRLRN_W_D,
    XVSSRLRN_WU_D,
    XVSSRLRNI_B_H,
    XVSSRLRNI_BU_H,
    XVSSRLRNI_D_Q,
    XVSSRLRNI_DU_Q,
    XVSSRLRNI_H_W,
    XVSSRLRNI_HU_W,
    XVSSRLRNI_W_D,
    XVSSRLRNI_WU_D,
    XVSSUB_B,
    XVSSUB_BU,
    XVSSUB_D,
    XVSSUB_DU,
    XVSSUB_H,
    XVSSUB_HU,
    XVSSUB_W,
    XVSSUB_WU,
    XVST,
    XVSTELM_B,
    XVSTELM_D,
    XVSTELM_H,
    XVSTELM_W,
    XVSTX,
    XVSUB_B,
    XVSUB_D,
    XVSUB_H,
    XVSUB_Q,
    XVSUB_W,
    XVSUBI_BU,
    XVSUBI_DU,
    XVSUBI_HU,
    XVSUBI_WU,
    XVSUBWEV_D_W,
    XVSUBWEV_D_WU,
    XVSUBWEV_H_B,
    XVSUBWEV_H_BU,
    XVSUBWEV_Q_D,
    XVSUBWEV_Q_DU,
    XVSUBWEV_W_H,
    XVSUBWEV_W_HU,
    XVSUBWOD_D_W,
    XVSUBWOD_D_WU,
    XVSUBWOD_H_B,
    XVSUBWOD_H_BU,
    XVSUBWOD_Q_D,
    XVSUBWOD_Q_DU,
    XVSUBWOD_W_H,
    XVSUBWOD_W_HU,
    XVXOR_V,
    XVXORI_B,
];
