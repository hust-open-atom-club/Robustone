barrier_insn!(IDLE, "idle", "IDLE", 0xFFFF8000, 0x06488000);

loongarch_insn!(
    TLBCLR,
    "tlbclr",
    "TLBCLR",
    0xFFFFFFFF,
    0x06482000,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    TLBFLUSH,
    "tlbflush",
    "TLBFLUSH",
    0xFFFFFFFF,
    0x06482400,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    TLBSRCH,
    "tlbsrch",
    "TLBSRCH",
    0xFFFFFFFF,
    0x06482800,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    TLBRD,
    "tlbrd",
    "TLBRD",
    0xFFFFFFFF,
    0x06482C00,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    TLBWR,
    "tlbwr",
    "TLBWR",
    0xFFFFFFFF,
    0x06483000,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    TLBFILL,
    "tlbfill",
    "TLBFILL",
    0xFFFFFFFF,
    0x06483400,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    ERTN,
    "ertn",
    "ERTN",
    0xFFFFFFFF,
    0x06483800,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRRD_B,
    "iocsrrd.b",
    "IOCSRRD_B",
    0xFFFFFC00,
    0x06480000,
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
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRRD_H,
    "iocsrrd.h",
    "IOCSRRD_H",
    0xFFFFFC00,
    0x06480400,
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
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRRD_W,
    "iocsrrd.w",
    "IOCSRRD_W",
    0xFFFFFC00,
    0x06480800,
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
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRRD_D,
    "iocsrrd.d",
    "IOCSRRD_D",
    0xFFFFFC00,
    0x06480C00,
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
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRWR_B,
    "iocsrwr.b",
    "IOCSRWR_B",
    0xFFFFFC00,
    0x06481000,
    &FMT_R2,
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
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRWR_H,
    "iocsrwr.h",
    "IOCSRWR_H",
    0xFFFFFC00,
    0x06481400,
    &FMT_R2,
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
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRWR_W,
    "iocsrwr.w",
    "IOCSRWR_W",
    0xFFFFFC00,
    0x06481800,
    &FMT_R2,
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
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    IOCSRWR_D,
    "iocsrwr.d",
    "IOCSRWR_D",
    0xFFFFFC00,
    0x06481C00,
    &FMT_R2,
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
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    CSRRD,
    "csrrd",
    "CSRRD",
    0xFF0003E0,
    0x04000000,
    &FMT_CSR,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::imm!(
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System],
    1
);

loongarch_insn!(
    CSRWR,
    "csrwr",
    "CSRWR",
    0xFF0003E0,
    0x04000020,
    &FMT_CSR,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rd,
            Access::Read
        ),
        robustone_isa::imm!(
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System],
    1
);

loongarch_insn!(
    CSRXCHG,
    "csrxchg",
    "CSRXCHG",
    0xFF000000,
    0x04000000,
    &FMT_CSRXCHG,
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
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    GCSRRD,
    "gcsrrd",
    "GCSRRD",
    0xFF0003E0,
    0x05000000,
    &FMT_CSR,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rd,
            Access::Write
        ),
        robustone_isa::imm!(
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System],
    1
);

loongarch_insn!(
    GCSRWR,
    "gcsrwr",
    "GCSRWR",
    0xFF0003E0,
    0x05000020,
    &FMT_CSR,
    &[
        robustone_isa::reg!(
            LoongArchRegisterClass::Gpr,
            LoongArchField::Rd,
            Access::Read
        ),
        robustone_isa::imm!(
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System],
    1
);

loongarch_insn!(
    GCSRXCHG,
    "gcsrxchg",
    "GCSRXCHG",
    0xFF000000,
    0x05000000,
    &FMT_CSRXCHG,
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
            LoongArchField::Si14,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
        ),
    ],
    &[InstructionGroup::Privileged, InstructionGroup::System]
);

loongarch_insn!(
    INVTLB,
    "invtlb",
    "INVTLB",
    0xFFFF8000,
    0x06498000,
    &FMT_INVTLB,
    &[
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Unsigned
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
    &[InstructionGroup::Privileged, InstructionGroup::System]
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



loongarch_insn!(
    CACOP,
    "cacop",
    "CACOP",
    0xFFC0_0000,
    0x0600_0000,
    &FMT_R2I12,
    &[
        robustone_isa::imm!(LoongArchField::Rd, ImmediateTransform::None, ImmediateKind::Unsigned),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(LoongArchField::Si12, ImmediateTransform::None, ImmediateKind::Unsigned),
    ],
    &[InstructionGroup::System, InstructionGroup::Privileged]
);

loongarch_insn!(
    LDDIR,
    "lddir",
    "LDDIR",
    0xFFFC_0000,
    0x0640_0000,
    &FMT_R2I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(LoongArchField::I8, ImmediateTransform::None, ImmediateKind::Unsigned),
    ],
    &[InstructionGroup::System, InstructionGroup::Privileged]
);

loongarch_insn!(
    LDPTE,
    "ldpte",
    "LDPTE",
    0xFFFC_001F,
    0x0644_0000,
    &FMT_LDPTE,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(LoongArchField::I8, ImmediateTransform::None, ImmediateKind::Unsigned),
    ],
    &[InstructionGroup::System, InstructionGroup::Privileged],
    1
);

loongarch_insn!(
    GTLBFLUSH,
    "gtlbflush",
    "GTLBFLUSH",
    0xFFFF_FFFF,
    0x0648_2401,
    &FMT_NONE,
    &[],
    &[InstructionGroup::System, InstructionGroup::Privileged]
);

r2_insn!(RDTIMEL_W, "rdtimel.w", "RDTIMEL_W", 0xFFFF_FC00, 0x0000_6000);
r2_insn!(RDTIMEH_W, "rdtimeh.w", "RDTIMEH_W", 0xFFFF_FC00, 0x0000_6400);
r2_insn!(CPUCFG, "cpucfg", "CPUCFG", 0xFFFF_FC00, 0x0000_6C00);

// ============================================================================
// Spec table
// ============================================================================
