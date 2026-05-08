// LBT (Binary Translation) instruction specs for LoongArch.

// Case 2
loongarch_insn!(
    SETX86LOOPE,
    "setx86loope",
    "SETX86LOOPE",
    0xffffffff,
    0x000078a4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    SETX86LOOPNE,
    "setx86loopne",
    "SETX86LOOPNE",
    0xffffffff,
    0x00007ca4,
    &FMT_R2,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

// Case 3
loongarch_insn!(
    X86MTTOP,
    "x86mttop",
    "X86MTTOP",
    0xffffff1f,
    0x00007000,
    &FMT_I3,
    &[
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 4
loongarch_insn!(
    X86MFTOP,
    "x86mftop",
    "X86MFTOP",
    0xffffffe0,
    0x00007400,
    &FMT_R1,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
    &[InstructionGroup::Integer]
);

// Case 5
loongarch_insn!(
    X86DEC_B,
    "x86dec.b",
    "X86DEC_B",
    0xfffffc1f,
    0x00008004,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86DEC_D,
    "x86dec.d",
    "X86DEC_D",
    0xfffffc1f,
    0x00008007,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86DEC_H,
    "x86dec.h",
    "X86DEC_H",
    0xfffffc1f,
    0x00008005,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86DEC_W,
    "x86dec.w",
    "X86DEC_W",
    0xfffffc1f,
    0x00008006,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86INC_B,
    "x86inc.b",
    "X86INC_B",
    0xfffffc1f,
    0x00008000,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86INC_D,
    "x86inc.d",
    "X86INC_D",
    0xfffffc1f,
    0x00008003,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86INC_H,
    "x86inc.h",
    "X86INC_H",
    0xfffffc1f,
    0x00008001,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86INC_W,
    "x86inc.w",
    "X86INC_W",
    0xfffffc1f,
    0x00008002,
    &FMT_R1_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

// Case 6
loongarch_insn!(
    X86CLRTM,
    "x86clrtm",
    "X86CLRTM",
    0xffffffff,
    0x00008028,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86DECTOP,
    "x86dectop",
    "X86DECTOP",
    0xffffffff,
    0x00008029,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86INCTOP,
    "x86inctop",
    "X86INCTOP",
    0xffffffff,
    0x00008009,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SETTM,
    "x86settm",
    "X86SETTM",
    0xffffffff,
    0x00008008,
    &FMT_NONE,
    &[],
    &[InstructionGroup::Integer]
);

// Case 7
loongarch_insn!(
    X86ADC_B,
    "x86adc.b",
    "X86ADC_B",
    0xffff801f,
    0x003f000c,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADC_D,
    "x86adc.d",
    "X86ADC_D",
    0xffff801f,
    0x003f000f,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADC_H,
    "x86adc.h",
    "X86ADC_H",
    0xffff801f,
    0x003f000d,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADC_W,
    "x86adc.w",
    "X86ADC_W",
    0xffff801f,
    0x003f000e,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_B,
    "x86add.b",
    "X86ADD_B",
    0xffff801f,
    0x003f0004,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_D,
    "x86add.d",
    "X86ADD_D",
    0xffff801f,
    0x003f0007,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_DU,
    "x86add.du",
    "X86ADD_DU",
    0xffff801f,
    0x003f0001,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_H,
    "x86add.h",
    "X86ADD_H",
    0xffff801f,
    0x003f0005,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_W,
    "x86add.w",
    "X86ADD_W",
    0xffff801f,
    0x003f0006,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ADD_WU,
    "x86add.wu",
    "X86ADD_WU",
    0xffff801f,
    0x003f0000,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86AND_B,
    "x86and.b",
    "X86AND_B",
    0xffff801f,
    0x003f8010,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86AND_D,
    "x86and.d",
    "X86AND_D",
    0xffff801f,
    0x003f8013,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86AND_H,
    "x86and.h",
    "X86AND_H",
    0xffff801f,
    0x003f8011,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86AND_W,
    "x86and.w",
    "X86AND_W",
    0xffff801f,
    0x003f8012,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_B,
    "x86mul.b",
    "X86MUL_B",
    0xffff801f,
    0x003e8000,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_BU,
    "x86mul.bu",
    "X86MUL_BU",
    0xffff801f,
    0x003e8004,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_D,
    "x86mul.d",
    "X86MUL_D",
    0xffff801f,
    0x003e8003,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_DU,
    "x86mul.du",
    "X86MUL_DU",
    0xffff801f,
    0x003e8007,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_H,
    "x86mul.h",
    "X86MUL_H",
    0xffff801f,
    0x003e8001,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_HU,
    "x86mul.hu",
    "X86MUL_HU",
    0xffff801f,
    0x003e8005,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_W,
    "x86mul.w",
    "X86MUL_W",
    0xffff801f,
    0x003e8002,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MUL_WU,
    "x86mul.wu",
    "X86MUL_WU",
    0xffff801f,
    0x003e8006,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86OR_B,
    "x86or.b",
    "X86OR_B",
    0xffff801f,
    0x003f8014,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86OR_D,
    "x86or.d",
    "X86OR_D",
    0xffff801f,
    0x003f8017,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86OR_H,
    "x86or.h",
    "X86OR_H",
    0xffff801f,
    0x003f8015,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86OR_W,
    "x86or.w",
    "X86OR_W",
    0xffff801f,
    0x003f8016,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCL_B,
    "x86rcl.b",
    "X86RCL_B",
    0xffff801f,
    0x003f800c,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCL_D,
    "x86rcl.d",
    "X86RCL_D",
    0xffff801f,
    0x003f800f,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCL_H,
    "x86rcl.h",
    "X86RCL_H",
    0xffff801f,
    0x003f800d,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCL_W,
    "x86rcl.w",
    "X86RCL_W",
    0xffff801f,
    0x003f800e,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCR_B,
    "x86rcr.b",
    "X86RCR_B",
    0xffff801f,
    0x003f8008,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCR_D,
    "x86rcr.d",
    "X86RCR_D",
    0xffff801f,
    0x003f800b,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCR_H,
    "x86rcr.h",
    "X86RCR_H",
    0xffff801f,
    0x003f8009,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCR_W,
    "x86rcr.w",
    "X86RCR_W",
    0xffff801f,
    0x003f800a,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTL_B,
    "x86rotl.b",
    "X86ROTL_B",
    0xffff801f,
    0x003f8004,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTL_D,
    "x86rotl.d",
    "X86ROTL_D",
    0xffff801f,
    0x003f8007,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTL_H,
    "x86rotl.h",
    "X86ROTL_H",
    0xffff801f,
    0x003f8005,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTL_W,
    "x86rotl.w",
    "X86ROTL_W",
    0xffff801f,
    0x003f8006,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTR_B,
    "x86rotr.b",
    "X86ROTR_B",
    0xffff801f,
    0x003f8000,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTR_D,
    "x86rotr.d",
    "X86ROTR_D",
    0xffff801f,
    0x003f8002,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTR_H,
    "x86rotr.h",
    "X86ROTR_H",
    0xffff801f,
    0x003f8001,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTR_W,
    "x86rotr.w",
    "X86ROTR_W",
    0xffff801f,
    0x003f8003,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SBC_B,
    "x86sbc.b",
    "X86SBC_B",
    0xffff801f,
    0x003f0010,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SBC_D,
    "x86sbc.d",
    "X86SBC_D",
    0xffff801f,
    0x003f0013,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SBC_H,
    "x86sbc.h",
    "X86SBC_H",
    0xffff801f,
    0x003f0011,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SBC_W,
    "x86sbc.w",
    "X86SBC_W",
    0xffff801f,
    0x003f0012,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLL_B,
    "x86sll.b",
    "X86SLL_B",
    0xffff801f,
    0x003f0014,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLL_D,
    "x86sll.d",
    "X86SLL_D",
    0xffff801f,
    0x003f0017,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLL_H,
    "x86sll.h",
    "X86SLL_H",
    0xffff801f,
    0x003f0015,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLL_W,
    "x86sll.w",
    "X86SLL_W",
    0xffff801f,
    0x003f0016,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRA_B,
    "x86sra.b",
    "X86SRA_B",
    0xffff801f,
    0x003f001c,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRA_D,
    "x86sra.d",
    "X86SRA_D",
    0xffff801f,
    0x003f001f,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRA_H,
    "x86sra.h",
    "X86SRA_H",
    0xffff801f,
    0x003f001d,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRA_W,
    "x86sra.w",
    "X86SRA_W",
    0xffff801f,
    0x003f001e,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRL_B,
    "x86srl.b",
    "X86SRL_B",
    0xffff801f,
    0x003f0018,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRL_D,
    "x86srl.d",
    "X86SRL_D",
    0xffff801f,
    0x003f001b,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRL_H,
    "x86srl.h",
    "X86SRL_H",
    0xffff801f,
    0x003f0019,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRL_W,
    "x86srl.w",
    "X86SRL_W",
    0xffff801f,
    0x003f001a,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_B,
    "x86sub.b",
    "X86SUB_B",
    0xffff801f,
    0x003f0008,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_D,
    "x86sub.d",
    "X86SUB_D",
    0xffff801f,
    0x003f000b,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_DU,
    "x86sub.du",
    "X86SUB_DU",
    0xffff801f,
    0x003f0003,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_H,
    "x86sub.h",
    "X86SUB_H",
    0xffff801f,
    0x003f0009,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_W,
    "x86sub.w",
    "X86SUB_W",
    0xffff801f,
    0x003f000a,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SUB_WU,
    "x86sub.wu",
    "X86SUB_WU",
    0xffff801f,
    0x003f0002,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86XOR_B,
    "x86xor.b",
    "X86XOR_B",
    0xffff801f,
    0x003f8018,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86XOR_D,
    "x86xor.d",
    "X86XOR_D",
    0xffff801f,
    0x003f801b,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86XOR_H,
    "x86xor.h",
    "X86XOR_H",
    0xffff801f,
    0x003f8019,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86XOR_W,
    "x86xor.w",
    "X86XOR_W",
    0xffff801f,
    0x003f801a,
    &FMT_R2_HI5,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
    &[InstructionGroup::Integer]
);

// Case 14
loongarch_insn!(
    ARMMOVE,
    "armmove",
    "ARMMOVE",
    0xffffc000,
    0x00364000,
    &FMT_R2I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 15
loongarch_insn!(
    SETARMJ,
    "setarmj",
    "SETARMJ",
    0xffffc3e0,
    0x0036c000,
    &FMT_R1I4_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    SETX86J,
    "setx86j",
    "SETX86J",
    0xffffc3e0,
    0x00368000,
    &FMT_R1I4_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 16
loongarch_insn!(
    ARMADC_W,
    "armadc.w",
    "ARMADC_W",
    0xffff8000,
    0x00380000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMADD_W,
    "armadd.w",
    "ARMADD_W",
    0xffff8000,
    0x00370000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMAND_W,
    "armand.w",
    "ARMAND_W",
    0xffff8000,
    0x00390000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMOR_W,
    "armor.w",
    "ARMOR_W",
    0xffff8000,
    0x00398000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMROTR_W,
    "armrotr.w",
    "ARMROTR_W",
    0xffff8000,
    0x003c0000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSBC_W,
    "armsbc.w",
    "ARMSBC_W",
    0xffff8000,
    0x00388000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSLL_W,
    "armsll.w",
    "ARMSLL_W",
    0xffff8000,
    0x003a8000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSRA_W,
    "armsra.w",
    "ARMSRA_W",
    0xffff8000,
    0x003b8000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSRL_W,
    "armsrl.w",
    "ARMSRL_W",
    0xffff8000,
    0x003b0000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSUB_W,
    "armsub.w",
    "ARMSUB_W",
    0xffff8000,
    0x00378000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMXOR_W,
    "armxor.w",
    "ARMXOR_W",
    0xffff8000,
    0x003a0000,
    &FMT_R2RKI4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 17
loongarch_insn!(
    ARMROTRI_W,
    "armrotri.w",
    "ARMROTRI_W",
    0xffff8000,
    0x003e0000,
    &FMT_R2I5I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSLLI_W,
    "armslli.w",
    "ARMSLLI_W",
    0xffff8000,
    0x003c8000,
    &FMT_R2I5I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSRAI_W,
    "armsrai.w",
    "ARMSRAI_W",
    0xffff8000,
    0x003d8000,
    &FMT_R2I5I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMSRLI_W,
    "armsrli.w",
    "ARMSRLI_W",
    0xffff8000,
    0x003d0000,
    &FMT_R2I5I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 18
loongarch_insn!(
    ARMMOV_D,
    "armmov.d",
    "ARMMOV_D",
    0xffffc01f,
    0x003fc01e,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMMOV_W,
    "armmov.w",
    "ARMMOV_W",
    0xffffc01f,
    0x003fc01d,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMNOT_W,
    "armnot.w",
    "ARMNOT_W",
    0xffffc01f,
    0x003fc01c,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMRRX_W,
    "armrrx.w",
    "ARMRRX_W",
    0xffffc01f,
    0x003fc01f,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCLI_H,
    "x86rcli.h",
    "X86RCLI_H",
    0xffffc01f,
    0x00544019,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCRI_H,
    "x86rcri.h",
    "X86RCRI_H",
    0xffffc01f,
    0x00544011,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTLI_H,
    "x86rotli.h",
    "X86ROTLI_H",
    0xffffc01f,
    0x00544015,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTRI_H,
    "x86rotri.h",
    "X86ROTRI_H",
    0xffffc01f,
    0x0054400d,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLLI_H,
    "x86slli.h",
    "X86SLLI_H",
    0xffffc01f,
    0x00544001,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRAI_H,
    "x86srai.h",
    "X86SRAI_H",
    0xffffc01f,
    0x00544009,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRLI_H,
    "x86srli.h",
    "X86SRLI_H",
    0xffffc01f,
    0x00544005,
    &FMT_R1I4,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 22
loongarch_insn!(
    X86RCLI_B,
    "x86rcli.b",
    "X86RCLI_B",
    0xffffe01f,
    0x00542018,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCRI_B,
    "x86rcri.b",
    "X86RCRI_B",
    0xffffe01f,
    0x00542010,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTLI_B,
    "x86rotli.b",
    "X86ROTLI_B",
    0xffffe01f,
    0x00542014,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTRI_B,
    "x86rotri.b",
    "X86ROTRI_B",
    0xffffe01f,
    0x0054200c,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLLI_B,
    "x86slli.b",
    "X86SLLI_B",
    0xffffe01f,
    0x00542000,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRAI_B,
    "x86srai.b",
    "X86SRAI_B",
    0xffffe01f,
    0x00542008,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRLI_B,
    "x86srli.b",
    "X86SRLI_B",
    0xffffe01f,
    0x00542004,
    &FMT_R2I3_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 23
loongarch_insn!(
    X86RCLI_W,
    "x86rcli.w",
    "X86RCLI_W",
    0xffff801f,
    0x0054801a,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCRI_W,
    "x86rcri.w",
    "X86RCRI_W",
    0xffff801f,
    0x00548012,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTLI_W,
    "x86rotli.w",
    "X86ROTLI_W",
    0xffff801f,
    0x00548016,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTRI_W,
    "x86rotri.w",
    "X86ROTRI_W",
    0xffff801f,
    0x0054800e,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLLI_W,
    "x86slli.w",
    "X86SLLI_W",
    0xffff801f,
    0x00548002,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRAI_W,
    "x86srai.w",
    "X86SRAI_W",
    0xffff801f,
    0x0054800a,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRLI_W,
    "x86srli.w",
    "X86SRLI_W",
    0xffff801f,
    0x00548006,
    &FMT_R2I5_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 24
loongarch_insn!(
    X86RCLI_D,
    "x86rcli.d",
    "X86RCLI_D",
    0xffff001f,
    0x0055001b,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86RCRI_D,
    "x86rcri.d",
    "X86RCRI_D",
    0xffff001f,
    0x00550013,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTLI_D,
    "x86rotli.d",
    "X86ROTLI_D",
    0xffff001f,
    0x00550017,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86ROTRI_D,
    "x86rotri.d",
    "X86ROTRI_D",
    0xffff001f,
    0x0055000f,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SLLI_D,
    "x86slli.d",
    "X86SLLI_D",
    0xffff001f,
    0x00550003,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRAI_D,
    "x86srai.d",
    "X86SRAI_D",
    0xffff001f,
    0x0055000b,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86SRLI_D,
    "x86srli.d",
    "X86SRLI_D",
    0xffff001f,
    0x00550007,
    &FMT_R2I6_H,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::imm!(
            LoongArchField::Ui6,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 25
loongarch_insn!(
    X86SETTAG,
    "x86settag",
    "X86SETTAG",
    0xfffc0000,
    0x00580000,
    &FMT_R1I5I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::Ui5,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
        robustone_isa::imm!(
            LoongArchField::I8,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

// Case 26
loongarch_insn!(
    ARMMFFLAG,
    "armmfflag",
    "ARMMFFLAG",
    0xfffc03e0,
    0x005c0040,
    &FMT_R1I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::I8,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    ARMMTFLAG,
    "armmtflag",
    "ARMMTFLAG",
    0xfffc03e0,
    0x005c0060,
    &FMT_R1I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::I8,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MFFLAG,
    "x86mfflag",
    "X86MFFLAG",
    0xfffc03e0,
    0x005c0000,
    &FMT_R1I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::I8,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);

loongarch_insn!(
    X86MTFLAG,
    "x86mtflag",
    "X86MTFLAG",
    0xfffc03e0,
    0x005c0020,
    &FMT_R1I8,
    &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::imm!(
            LoongArchField::I8,
            ImmediateTransform::None,
            ImmediateKind::Absolute
        ),
    ],
    &[InstructionGroup::Integer]
);


pub static LBT_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    SETX86LOOPE,
    SETX86LOOPNE,
    X86MTTOP,
    X86MFTOP,
    X86DEC_B,
    X86DEC_D,
    X86DEC_H,
    X86DEC_W,
    X86INC_B,
    X86INC_D,
    X86INC_H,
    X86INC_W,
    X86CLRTM,
    X86DECTOP,
    X86INCTOP,
    X86SETTM,
    X86ADC_B,
    X86ADC_D,
    X86ADC_H,
    X86ADC_W,
    X86ADD_B,
    X86ADD_D,
    X86ADD_DU,
    X86ADD_H,
    X86ADD_W,
    X86ADD_WU,
    X86AND_B,
    X86AND_D,
    X86AND_H,
    X86AND_W,
    X86MUL_B,
    X86MUL_BU,
    X86MUL_D,
    X86MUL_DU,
    X86MUL_H,
    X86MUL_HU,
    X86MUL_W,
    X86MUL_WU,
    X86OR_B,
    X86OR_D,
    X86OR_H,
    X86OR_W,
    X86RCL_B,
    X86RCL_D,
    X86RCL_H,
    X86RCL_W,
    X86RCR_B,
    X86RCR_D,
    X86RCR_H,
    X86RCR_W,
    X86ROTL_B,
    X86ROTL_D,
    X86ROTL_H,
    X86ROTL_W,
    X86ROTR_B,
    X86ROTR_D,
    X86ROTR_H,
    X86ROTR_W,
    X86SBC_B,
    X86SBC_D,
    X86SBC_H,
    X86SBC_W,
    X86SLL_B,
    X86SLL_D,
    X86SLL_H,
    X86SLL_W,
    X86SRA_B,
    X86SRA_D,
    X86SRA_H,
    X86SRA_W,
    X86SRL_B,
    X86SRL_D,
    X86SRL_H,
    X86SRL_W,
    X86SUB_B,
    X86SUB_D,
    X86SUB_DU,
    X86SUB_H,
    X86SUB_W,
    X86SUB_WU,
    X86XOR_B,
    X86XOR_D,
    X86XOR_H,
    X86XOR_W,
    ARMMOVE,
    SETARMJ,
    SETX86J,
    ARMADC_W,
    ARMADD_W,
    ARMAND_W,
    ARMOR_W,
    ARMROTR_W,
    ARMSBC_W,
    ARMSLL_W,
    ARMSRA_W,
    ARMSRL_W,
    ARMSUB_W,
    ARMXOR_W,
    ARMROTRI_W,
    ARMSLLI_W,
    ARMSRAI_W,
    ARMSRLI_W,
    ARMMOV_D,
    ARMMOV_W,
    ARMNOT_W,
    ARMRRX_W,
    X86RCLI_H,
    X86RCRI_H,
    X86ROTLI_H,
    X86ROTRI_H,
    X86SLLI_H,
    X86SRAI_H,
    X86SRLI_H,
    X86RCLI_B,
    X86RCRI_B,
    X86ROTLI_B,
    X86ROTRI_B,
    X86SLLI_B,
    X86SRAI_B,
    X86SRLI_B,
    X86RCLI_W,
    X86RCRI_W,
    X86ROTLI_W,
    X86ROTRI_W,
    X86SLLI_W,
    X86SRAI_W,
    X86SRLI_W,
    X86RCLI_D,
    X86RCRI_D,
    X86ROTLI_D,
    X86ROTRI_D,
    X86SLLI_D,
    X86SRAI_D,
    X86SRLI_D,
    X86SETTAG,
    ARMMFFLAG,
    ARMMTFLAG,
    X86MFFLAG,
    X86MTFLAG,
];
