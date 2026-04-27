//! LoongArch data types used by the architecture crate.
//!
//! These structures define register enumerations, instruction formats,
//! and operand types specific to the LoongArch LA64 ISA.

/// Kinds of operands that can appear in a LoongArch instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchOperandType {
    /// Placeholder for invalid operands.
    Invalid,
    /// Register operand.
    Register,
    /// Immediate operand.
    Immediate,
    /// Memory operand (base + displacement).
    Memory,
    /// Condition flag register (FCC).
    ConditionFlag,
    /// SCR (state control register).
    Scr,
}

/// Concrete value carried by a LoongArch operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoongArchOperandValue {
    /// General-purpose or floating-point register identifier.
    Register(LoongArchRegister),
    /// Immediate literal.
    Immediate(i64),
    /// Memory addressing mode.
    Memory(LoongArchMemoryOperand),
    /// Condition flag register.
    ConditionFlag(u8),
    /// SCR register.
    Scr(u8),
}

/// Memory operand descriptor (matches LoongArch load/store syntax).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoongArchMemoryOperand {
    /// Base register (always GPR).
    pub base: u8,
    /// Displacement relative to the base register.
    pub disp: i64,
}

/// Fully described operand (aligned with internal IR).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoongArchOperand {
    /// Operand classification.
    pub op_type: LoongArchOperandType,
    /// Encoded operand payload.
    pub value: LoongArchOperandValue,
}

/// Comprehensive LoongArch register enumeration.
///
/// IDs are laid out in a single u32 space for compatibility with `RegisterId`:
/// - GPR:  0..=31
/// - FPR:  32..=63
/// - Vector (LSX/LASX XR): 64..=95
/// - FCC:  96..=103
/// - SCR:  104..=107
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchRegister {
    Invalid = 255,

    // General-purpose registers (GPR)
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    R16 = 16,
    R17 = 17,
    R18 = 18,
    R19 = 19,
    R20 = 20,
    R21 = 21,
    R22 = 22,
    R23 = 23,
    R24 = 24,
    R25 = 25,
    R26 = 26,
    R27 = 27,
    R28 = 28,
    R29 = 29,
    R30 = 30,
    R31 = 31,

    // Floating-point registers (FPR)
    F0 = 32,
    F1 = 33,
    F2 = 34,
    F3 = 35,
    F4 = 36,
    F5 = 37,
    F6 = 38,
    F7 = 39,
    F8 = 40,
    F9 = 41,
    F10 = 42,
    F11 = 43,
    F12 = 44,
    F13 = 45,
    F14 = 46,
    F15 = 47,
    F16 = 48,
    F17 = 49,
    F18 = 50,
    F19 = 51,
    F20 = 52,
    F21 = 53,
    F22 = 54,
    F23 = 55,
    F24 = 56,
    F25 = 57,
    F26 = 58,
    F27 = 59,
    F28 = 60,
    F29 = 61,
    F30 = 62,
    F31 = 63,

    // Vector registers (LSX/LASX XR)
    Xr0 = 64,
    Xr1 = 65,
    Xr2 = 66,
    Xr3 = 67,
    Xr4 = 68,
    Xr5 = 69,
    Xr6 = 70,
    Xr7 = 71,
    Xr8 = 72,
    Xr9 = 73,
    Xr10 = 74,
    Xr11 = 75,
    Xr12 = 76,
    Xr13 = 77,
    Xr14 = 78,
    Xr15 = 79,
    Xr16 = 80,
    Xr17 = 81,
    Xr18 = 82,
    Xr19 = 83,
    Xr20 = 84,
    Xr21 = 85,
    Xr22 = 86,
    Xr23 = 87,
    Xr24 = 88,
    Xr25 = 89,
    Xr26 = 90,
    Xr27 = 91,
    Xr28 = 92,
    Xr29 = 93,
    Xr30 = 94,
    Xr31 = 95,

    // Floating-point condition flags (FCC)
    Fcc0 = 96,
    Fcc1 = 97,
    Fcc2 = 98,
    Fcc3 = 99,
    Fcc4 = 100,
    Fcc5 = 101,
    Fcc6 = 102,
    Fcc7 = 103,

    // State control registers (SCR)
    Scr0 = 104,
    Scr1 = 105,
    Scr2 = 106,
    Scr3 = 107,

    // Floating-point control and status registers (FCSR)
    Fcsr0 = 108,
    Fcsr1 = 109,
    Fcsr2 = 110,
    Fcsr3 = 111,
}

impl LoongArchRegister {
    /// Returns the canonical register name as used by Capstone.
    pub fn name(self) -> &'static str {
        match self {
            LoongArchRegister::Invalid => "invalid",

            // GPR canonical names (ABI aliases)
            LoongArchRegister::R0 => "$zero",
            LoongArchRegister::R1 => "$ra",
            LoongArchRegister::R2 => "$tp",
            LoongArchRegister::R3 => "$sp",
            LoongArchRegister::R4 => "$a0",
            LoongArchRegister::R5 => "$a1",
            LoongArchRegister::R6 => "$a2",
            LoongArchRegister::R7 => "$a3",
            LoongArchRegister::R8 => "$a4",
            LoongArchRegister::R9 => "$a5",
            LoongArchRegister::R10 => "$a6",
            LoongArchRegister::R11 => "$a7",
            LoongArchRegister::R12 => "$t0",
            LoongArchRegister::R13 => "$t1",
            LoongArchRegister::R14 => "$t2",
            LoongArchRegister::R15 => "$t3",
            LoongArchRegister::R16 => "$t4",
            LoongArchRegister::R17 => "$t5",
            LoongArchRegister::R18 => "$t6",
            LoongArchRegister::R19 => "$t7",
            LoongArchRegister::R20 => "$t8",
            LoongArchRegister::R21 => "$r21",
            LoongArchRegister::R22 => "$fp",
            LoongArchRegister::R23 => "$s0",
            LoongArchRegister::R24 => "$s1",
            LoongArchRegister::R25 => "$s2",
            LoongArchRegister::R26 => "$s3",
            LoongArchRegister::R27 => "$s4",
            LoongArchRegister::R28 => "$s5",
            LoongArchRegister::R29 => "$s6",
            LoongArchRegister::R30 => "$s7",
            LoongArchRegister::R31 => "$s8",

            // FPR names (Capstone aliases)
            LoongArchRegister::F0 => "$fa0",
            LoongArchRegister::F1 => "$fa1",
            LoongArchRegister::F2 => "$fa2",
            LoongArchRegister::F3 => "$fa3",
            LoongArchRegister::F4 => "$fa4",
            LoongArchRegister::F5 => "$fa5",
            LoongArchRegister::F6 => "$fa6",
            LoongArchRegister::F7 => "$fa7",
            LoongArchRegister::F8 => "$ft0",
            LoongArchRegister::F9 => "$ft1",
            LoongArchRegister::F10 => "$ft2",
            LoongArchRegister::F11 => "$ft3",
            LoongArchRegister::F12 => "$ft4",
            LoongArchRegister::F13 => "$ft5",
            LoongArchRegister::F14 => "$ft6",
            LoongArchRegister::F15 => "$ft7",
            LoongArchRegister::F16 => "$ft8",
            LoongArchRegister::F17 => "$ft9",
            LoongArchRegister::F18 => "$ft10",
            LoongArchRegister::F19 => "$ft11",
            LoongArchRegister::F20 => "$ft12",
            LoongArchRegister::F21 => "$ft13",
            LoongArchRegister::F22 => "$ft14",
            LoongArchRegister::F23 => "$ft15",
            LoongArchRegister::F24 => "$fs0",
            LoongArchRegister::F25 => "$fs1",
            LoongArchRegister::F26 => "$fs2",
            LoongArchRegister::F27 => "$fs3",
            LoongArchRegister::F28 => "$fs4",
            LoongArchRegister::F29 => "$fs5",
            LoongArchRegister::F30 => "$fs6",
            LoongArchRegister::F31 => "$fs7",

            // Vector register names
            LoongArchRegister::Xr0 => "$xr0",
            LoongArchRegister::Xr1 => "$xr1",
            LoongArchRegister::Xr2 => "$xr2",
            LoongArchRegister::Xr3 => "$xr3",
            LoongArchRegister::Xr4 => "$xr4",
            LoongArchRegister::Xr5 => "$xr5",
            LoongArchRegister::Xr6 => "$xr6",
            LoongArchRegister::Xr7 => "$xr7",
            LoongArchRegister::Xr8 => "$xr8",
            LoongArchRegister::Xr9 => "$xr9",
            LoongArchRegister::Xr10 => "$xr10",
            LoongArchRegister::Xr11 => "$xr11",
            LoongArchRegister::Xr12 => "$xr12",
            LoongArchRegister::Xr13 => "$xr13",
            LoongArchRegister::Xr14 => "$xr14",
            LoongArchRegister::Xr15 => "$xr15",
            LoongArchRegister::Xr16 => "$xr16",
            LoongArchRegister::Xr17 => "$xr17",
            LoongArchRegister::Xr18 => "$xr18",
            LoongArchRegister::Xr19 => "$xr19",
            LoongArchRegister::Xr20 => "$xr20",
            LoongArchRegister::Xr21 => "$xr21",
            LoongArchRegister::Xr22 => "$xr22",
            LoongArchRegister::Xr23 => "$xr23",
            LoongArchRegister::Xr24 => "$xr24",
            LoongArchRegister::Xr25 => "$xr25",
            LoongArchRegister::Xr26 => "$xr26",
            LoongArchRegister::Xr27 => "$xr27",
            LoongArchRegister::Xr28 => "$xr28",
            LoongArchRegister::Xr29 => "$xr29",
            LoongArchRegister::Xr30 => "$xr30",
            LoongArchRegister::Xr31 => "$xr31",

            // FCC names
            LoongArchRegister::Fcc0 => "$fcc0",
            LoongArchRegister::Fcc1 => "$fcc1",
            LoongArchRegister::Fcc2 => "$fcc2",
            LoongArchRegister::Fcc3 => "$fcc3",
            LoongArchRegister::Fcc4 => "$fcc4",
            LoongArchRegister::Fcc5 => "$fcc5",
            LoongArchRegister::Fcc6 => "$fcc6",
            LoongArchRegister::Fcc7 => "$fcc7",

            // SCR names
            LoongArchRegister::Scr0 => "$scr0",
            LoongArchRegister::Scr1 => "$scr1",
            LoongArchRegister::Scr2 => "$scr2",
            LoongArchRegister::Scr3 => "$scr3",

            // FCSR names
            LoongArchRegister::Fcsr0 => "$fcsr0",
            LoongArchRegister::Fcsr1 => "$fcsr1",
            LoongArchRegister::Fcsr2 => "$fcsr2",
            LoongArchRegister::Fcsr3 => "$fcsr3",
        }
    }

    /// Converts a raw register ID into the enum representation.
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => LoongArchRegister::R0,
            1 => LoongArchRegister::R1,
            2 => LoongArchRegister::R2,
            3 => LoongArchRegister::R3,
            4 => LoongArchRegister::R4,
            5 => LoongArchRegister::R5,
            6 => LoongArchRegister::R6,
            7 => LoongArchRegister::R7,
            8 => LoongArchRegister::R8,
            9 => LoongArchRegister::R9,
            10 => LoongArchRegister::R10,
            11 => LoongArchRegister::R11,
            12 => LoongArchRegister::R12,
            13 => LoongArchRegister::R13,
            14 => LoongArchRegister::R14,
            15 => LoongArchRegister::R15,
            16 => LoongArchRegister::R16,
            17 => LoongArchRegister::R17,
            18 => LoongArchRegister::R18,
            19 => LoongArchRegister::R19,
            20 => LoongArchRegister::R20,
            21 => LoongArchRegister::R21,
            22 => LoongArchRegister::R22,
            23 => LoongArchRegister::R23,
            24 => LoongArchRegister::R24,
            25 => LoongArchRegister::R25,
            26 => LoongArchRegister::R26,
            27 => LoongArchRegister::R27,
            28 => LoongArchRegister::R28,
            29 => LoongArchRegister::R29,
            30 => LoongArchRegister::R30,
            31 => LoongArchRegister::R31,
            32 => LoongArchRegister::F0,
            33 => LoongArchRegister::F1,
            34 => LoongArchRegister::F2,
            35 => LoongArchRegister::F3,
            36 => LoongArchRegister::F4,
            37 => LoongArchRegister::F5,
            38 => LoongArchRegister::F6,
            39 => LoongArchRegister::F7,
            40 => LoongArchRegister::F8,
            41 => LoongArchRegister::F9,
            42 => LoongArchRegister::F10,
            43 => LoongArchRegister::F11,
            44 => LoongArchRegister::F12,
            45 => LoongArchRegister::F13,
            46 => LoongArchRegister::F14,
            47 => LoongArchRegister::F15,
            48 => LoongArchRegister::F16,
            49 => LoongArchRegister::F17,
            50 => LoongArchRegister::F18,
            51 => LoongArchRegister::F19,
            52 => LoongArchRegister::F20,
            53 => LoongArchRegister::F21,
            54 => LoongArchRegister::F22,
            55 => LoongArchRegister::F23,
            56 => LoongArchRegister::F24,
            57 => LoongArchRegister::F25,
            58 => LoongArchRegister::F26,
            59 => LoongArchRegister::F27,
            60 => LoongArchRegister::F28,
            61 => LoongArchRegister::F29,
            62 => LoongArchRegister::F30,
            63 => LoongArchRegister::F31,
            64 => LoongArchRegister::Xr0,
            65 => LoongArchRegister::Xr1,
            66 => LoongArchRegister::Xr2,
            67 => LoongArchRegister::Xr3,
            68 => LoongArchRegister::Xr4,
            69 => LoongArchRegister::Xr5,
            70 => LoongArchRegister::Xr6,
            71 => LoongArchRegister::Xr7,
            72 => LoongArchRegister::Xr8,
            73 => LoongArchRegister::Xr9,
            74 => LoongArchRegister::Xr10,
            75 => LoongArchRegister::Xr11,
            76 => LoongArchRegister::Xr12,
            77 => LoongArchRegister::Xr13,
            78 => LoongArchRegister::Xr14,
            79 => LoongArchRegister::Xr15,
            80 => LoongArchRegister::Xr16,
            81 => LoongArchRegister::Xr17,
            82 => LoongArchRegister::Xr18,
            83 => LoongArchRegister::Xr19,
            84 => LoongArchRegister::Xr20,
            85 => LoongArchRegister::Xr21,
            86 => LoongArchRegister::Xr22,
            87 => LoongArchRegister::Xr23,
            88 => LoongArchRegister::Xr24,
            89 => LoongArchRegister::Xr25,
            90 => LoongArchRegister::Xr26,
            91 => LoongArchRegister::Xr27,
            92 => LoongArchRegister::Xr28,
            93 => LoongArchRegister::Xr29,
            94 => LoongArchRegister::Xr30,
            95 => LoongArchRegister::Xr31,
            96 => LoongArchRegister::Fcc0,
            97 => LoongArchRegister::Fcc1,
            98 => LoongArchRegister::Fcc2,
            99 => LoongArchRegister::Fcc3,
            100 => LoongArchRegister::Fcc4,
            101 => LoongArchRegister::Fcc5,
            102 => LoongArchRegister::Fcc6,
            103 => LoongArchRegister::Fcc7,
            104 => LoongArchRegister::Scr0,
            105 => LoongArchRegister::Scr1,
            106 => LoongArchRegister::Scr2,
            107 => LoongArchRegister::Scr3,
            108 => LoongArchRegister::Fcsr0,
            109 => LoongArchRegister::Fcsr1,
            110 => LoongArchRegister::Fcsr2,
            111 => LoongArchRegister::Fcsr3,
            _ => LoongArchRegister::Invalid,
        }
    }
}

/// Instruction encoding formats available in LoongArch.
///
/// These are *logical* formats describing the operand layout,
/// not the exact wire encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchInstructionFormat {
    /// Three GPR registers: Rd, Rj, Rk.
    ThreeReg,
    /// Two GPR registers + immediate: Rd, Rj, imm.
    TwoRegImm,
    /// Two GPR registers (memory): Rd, Rj, offset.
    TwoRegMem,
    /// Branch: Rj, Rd, offset.
    Branch,
    /// Jump / indirect branch: Rj, offset.
    Jump,
    /// Single register + immediate: Rd, imm.
    OneRegImm,
    /// Two GPR registers: Rj, Rd.
    TwoReg,
    /// Single register only: Rd.
    OneReg,
    /// No operands.
    None,
    /// Three vector registers: Xd, Xj, Xk.
    VectorThreeReg,
    /// Two vector registers + immediate: Xd, Xj, imm.
    VectorTwoRegImm,
    /// Three FP registers: Fa, Fj, Fk.
    FloatThreeReg,
    /// Two FP registers + immediate: Fa, Fj, imm.
    FloatTwoRegImm,
    /// Two FP registers: Fa, Fj.
    FloatTwoReg,
    /// One FP register: Fa.
    FloatOneReg,
    /// GPR + FCC: Rd, Fcc.
    RegConditionFlag,
    /// SCR operation: Scr, Rj.
    ScrReg,
    /// Barrier instruction with optional operand.
    Barrier,
}

/// Threshold for formatting immediates in hex vs decimal.
/// Matches Capstone's default behavior (values > 9 use hex).
pub const HEX_THRESHOLD: i64 = 9;
