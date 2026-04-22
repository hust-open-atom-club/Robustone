//! RISC-V data types used by the transfer crate.
//!
//! These structures mirror Capstone's RISC-V bindings to ease interoperability.

pub mod error {
    pub use robustone_core::types::error::*;
}

/// Kinds of operands that can appear in a RISC-V instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVOperandType {
    /// Placeholder for invalid operands.
    Invalid,
    /// Register operand.
    Register,
    /// Immediate operand.
    Immediate,
    /// Rounding mode operand.
    RoundingMode,
    /// Memory operand.
    Memory,
}

/// Memory operand descriptor (matches `RISCV_OP_MEM`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiscVMemoryOperand {
    /// Base register.
    pub base: u32,
    /// Displacement relative to the base register.
    pub disp: i64,
}

/// Fully described operand (aligned with `cs_riscv_op`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiscVOperand {
    /// Operand classification.
    pub op_type: RiscVOperandType,
    /// Access behaviour for the operand.
    pub access: Access,
    /// Encoded operand payload.
    pub value: RiscVOperandValue,
}

/// Concrete value carried by an operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiscVOperandValue {
    /// Register identifier.
    Register(u32),
    /// Immediate literal.
    Immediate(i64),
    /// Floating-point rounding mode selector.
    RoundingMode(u8),
    /// Memory addressing mode.
    Memory(RiscVMemoryOperand),
}

/// Register access flags (mirrors `cs_ac_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Access {
    /// Indicates the register is read.
    pub read: bool,
    /// Indicates the register is written.
    pub write: bool,
}

impl Access {
    pub fn read() -> Self {
        Self {
            read: true,
            write: false,
        }
    }
    pub fn write() -> Self {
        Self {
            read: false,
            write: true,
        }
    }
    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
        }
    }
    pub fn none() -> Self {
        Self {
            read: false,
            write: false,
        }
    }
}

/// Converts a floating-point rounding mode selector into its mnemonic.
pub fn rounding_mode_name(rm: u8) -> &'static str {
    match rm {
        0b000 => "rne",
        0b001 => "rtz",
        0b010 => "rdn",
        0b011 => "rup",
        0b100 => "rmm",
        0b111 => "dyn",
        _ => "invalid",
    }
}

/// Comprehensive RISC-V register enumeration (compatible with `riscv_reg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVRegister {
    Invalid = 0,

    // General-purpose registers
    X0 = 1,   // "zero"
    X1 = 2,   // "ra"
    X2 = 3,   // "sp"
    X3 = 4,   // "gp"
    X4 = 5,   // "tp"
    X5 = 6,   // "t0"
    X6 = 7,   // "t1"
    X7 = 8,   // "t2"
    X8 = 9,   // "s0/fp"
    X9 = 10,  // "s1"
    X10 = 11, // "a0"
    X11 = 12, // "a1"
    X12 = 13, // "a2"
    X13 = 14, // "a3"
    X14 = 15, // "a4"
    X15 = 16, // "a5"
    X16 = 17, // "a6"
    X17 = 18, // "a7"
    X18 = 19, // "s2"
    X19 = 20, // "s3"
    X20 = 21, // "s4"
    X21 = 22, // "s5"
    X22 = 23, // "s6"
    X23 = 24, // "s7"
    X24 = 25, // "s8"
    X25 = 26, // "s9"
    X26 = 27, // "s10"
    X27 = 28, // "s11"
    X28 = 29, // "t3"
    X29 = 30, // "t4"
    X30 = 31, // "t5"
    X31 = 32, // "t6"

    // 32-bit floating-point registers
    F0_32 = 33,
    F1_32 = 34,
    F2_32 = 35,
    F3_32 = 36,
    F4_32 = 37,
    F5_32 = 38,
    F6_32 = 39,
    F7_32 = 40,
    F8_32 = 41,
    F9_32 = 42,
    F10_32 = 43,
    F11_32 = 44,
    F12_32 = 45,
    F13_32 = 46,
    F14_32 = 47,
    F15_32 = 48,
    F16_32 = 49,
    F17_32 = 50,
    F18_32 = 51,
    F19_32 = 52,
    F20_32 = 53,
    F21_32 = 54,
    F22_32 = 55,
    F23_32 = 56,
    F24_32 = 57,
    F25_32 = 58,
    F26_32 = 59,
    F27_32 = 60,
    F28_32 = 61,
    F29_32 = 62,
    F30_32 = 63,
    F31_32 = 64,

    // 64-bit floating-point registers
    F0_64 = 65,
    F1_64 = 66,
    F2_64 = 67,
    F3_64 = 68,
    F4_64 = 69,
    F5_64 = 70,
    F6_64 = 71,
    F7_64 = 72,
    F8_64 = 73,
    F9_64 = 74,
    F10_64 = 75,
    F11_64 = 76,
    F12_64 = 77,
    F13_64 = 78,
    F14_64 = 79,
    F15_64 = 80,
    F16_64 = 81,
    F17_64 = 82,
    F18_64 = 83,
    F19_64 = 84,
    F20_64 = 85,
    F21_64 = 86,
    F22_64 = 87,
    F23_64 = 88,
    F24_64 = 89,
    F25_64 = 90,
    F26_64 = 91,
    F27_64 = 92,
    F28_64 = 93,
    F29_64 = 94,
    F30_64 = 95,
    F31_64 = 96,
}

impl RiscVRegister {
    /// Returns the canonical register name.
    pub fn name(self) -> &'static str {
        match self {
            RiscVRegister::Invalid => "invalid",

            // General-purpose registers
            RiscVRegister::X0 => "zero",
            RiscVRegister::X1 => "ra",
            RiscVRegister::X2 => "sp",
            RiscVRegister::X3 => "gp",
            RiscVRegister::X4 => "tp",
            RiscVRegister::X5 => "t0",
            RiscVRegister::X6 => "t1",
            RiscVRegister::X7 => "t2",
            RiscVRegister::X8 => "s0",
            RiscVRegister::X9 => "s1",
            RiscVRegister::X10 => "a0",
            RiscVRegister::X11 => "a1",
            RiscVRegister::X12 => "a2",
            RiscVRegister::X13 => "a3",
            RiscVRegister::X14 => "a4",
            RiscVRegister::X15 => "a5",
            RiscVRegister::X16 => "a6",
            RiscVRegister::X17 => "a7",
            RiscVRegister::X18 => "s2",
            RiscVRegister::X19 => "s3",
            RiscVRegister::X20 => "s4",
            RiscVRegister::X21 => "s5",
            RiscVRegister::X22 => "s6",
            RiscVRegister::X23 => "s7",
            RiscVRegister::X24 => "s8",
            RiscVRegister::X25 => "s9",
            RiscVRegister::X26 => "s10",
            RiscVRegister::X27 => "s11",
            RiscVRegister::X28 => "t3",
            RiscVRegister::X29 => "t4",
            RiscVRegister::X30 => "t5",
            RiscVRegister::X31 => "t6",

            // 32-bit floating-point aliases
            RiscVRegister::F0_32 => "ft0",
            RiscVRegister::F1_32 => "ft1",
            RiscVRegister::F2_32 => "ft2",
            RiscVRegister::F3_32 => "ft3",
            RiscVRegister::F4_32 => "ft4",
            RiscVRegister::F5_32 => "ft5",
            RiscVRegister::F6_32 => "ft6",
            RiscVRegister::F7_32 => "ft7",
            RiscVRegister::F8_32 => "fs0",
            RiscVRegister::F9_32 => "fs1",
            RiscVRegister::F10_32 => "fa0",
            RiscVRegister::F11_32 => "fa1",
            RiscVRegister::F12_32 => "fa2",
            RiscVRegister::F13_32 => "fa3",
            RiscVRegister::F14_32 => "fa4",
            RiscVRegister::F15_32 => "fa5",
            RiscVRegister::F16_32 => "fa6",
            RiscVRegister::F17_32 => "fa7",
            RiscVRegister::F18_32 => "fs2",
            RiscVRegister::F19_32 => "fs3",
            RiscVRegister::F20_32 => "fs4",
            RiscVRegister::F21_32 => "fs5",
            RiscVRegister::F22_32 => "fs6",
            RiscVRegister::F23_32 => "fs7",
            RiscVRegister::F24_32 => "fs8",
            RiscVRegister::F25_32 => "fs9",
            RiscVRegister::F26_32 => "fs10",
            RiscVRegister::F27_32 => "fs11",
            RiscVRegister::F28_32 => "ft8",
            RiscVRegister::F29_32 => "ft9",
            RiscVRegister::F30_32 => "ft10",
            RiscVRegister::F31_32 => "ft11",

            // 64-bit floating-point aliases
            RiscVRegister::F0_64 => "ft0",
            RiscVRegister::F1_64 => "ft1",
            RiscVRegister::F2_64 => "ft2",
            RiscVRegister::F3_64 => "ft3",
            RiscVRegister::F4_64 => "ft4",
            RiscVRegister::F5_64 => "ft5",
            RiscVRegister::F6_64 => "ft6",
            RiscVRegister::F7_64 => "ft7",
            RiscVRegister::F8_64 => "fs0",
            RiscVRegister::F9_64 => "fs1",
            RiscVRegister::F10_64 => "fa0",
            RiscVRegister::F11_64 => "fa1",
            RiscVRegister::F12_64 => "fa2",
            RiscVRegister::F13_64 => "fa3",
            RiscVRegister::F14_64 => "fa4",
            RiscVRegister::F15_64 => "fa5",
            RiscVRegister::F16_64 => "fa6",
            RiscVRegister::F17_64 => "fa7",
            RiscVRegister::F18_64 => "fs2",
            RiscVRegister::F19_64 => "fs3",
            RiscVRegister::F20_64 => "fs4",
            RiscVRegister::F21_64 => "fs5",
            RiscVRegister::F22_64 => "fs6",
            RiscVRegister::F23_64 => "fs7",
            RiscVRegister::F24_64 => "fs8",
            RiscVRegister::F25_64 => "fs9",
            RiscVRegister::F26_64 => "fs10",
            RiscVRegister::F27_64 => "fs11",
            RiscVRegister::F28_64 => "ft8",
            RiscVRegister::F29_64 => "ft9",
            RiscVRegister::F30_64 => "ft10",
            RiscVRegister::F31_64 => "ft11",
        }
    }

    /// Converts a raw register ID into the enum representation.
    ///
    /// Integer registers use `0..=31`.
    /// Floating-point registers use `32..=63`.
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => RiscVRegister::X0,
            1 => RiscVRegister::X1,
            2 => RiscVRegister::X2,
            3 => RiscVRegister::X3,
            4 => RiscVRegister::X4,
            5 => RiscVRegister::X5,
            6 => RiscVRegister::X6,
            7 => RiscVRegister::X7,
            8 => RiscVRegister::X8,
            9 => RiscVRegister::X9,
            10 => RiscVRegister::X10,
            11 => RiscVRegister::X11,
            12 => RiscVRegister::X12,
            13 => RiscVRegister::X13,
            14 => RiscVRegister::X14,
            15 => RiscVRegister::X15,
            16 => RiscVRegister::X16,
            17 => RiscVRegister::X17,
            18 => RiscVRegister::X18,
            19 => RiscVRegister::X19,
            20 => RiscVRegister::X20,
            21 => RiscVRegister::X21,
            22 => RiscVRegister::X22,
            23 => RiscVRegister::X23,
            24 => RiscVRegister::X24,
            25 => RiscVRegister::X25,
            26 => RiscVRegister::X26,
            27 => RiscVRegister::X27,
            28 => RiscVRegister::X28,
            29 => RiscVRegister::X29,
            30 => RiscVRegister::X30,
            31 => RiscVRegister::X31,
            32 => RiscVRegister::F0_32,
            33 => RiscVRegister::F1_32,
            34 => RiscVRegister::F2_32,
            35 => RiscVRegister::F3_32,
            36 => RiscVRegister::F4_32,
            37 => RiscVRegister::F5_32,
            38 => RiscVRegister::F6_32,
            39 => RiscVRegister::F7_32,
            40 => RiscVRegister::F8_32,
            41 => RiscVRegister::F9_32,
            42 => RiscVRegister::F10_32,
            43 => RiscVRegister::F11_32,
            44 => RiscVRegister::F12_32,
            45 => RiscVRegister::F13_32,
            46 => RiscVRegister::F14_32,
            47 => RiscVRegister::F15_32,
            48 => RiscVRegister::F16_32,
            49 => RiscVRegister::F17_32,
            50 => RiscVRegister::F18_32,
            51 => RiscVRegister::F19_32,
            52 => RiscVRegister::F20_32,
            53 => RiscVRegister::F21_32,
            54 => RiscVRegister::F22_32,
            55 => RiscVRegister::F23_32,
            56 => RiscVRegister::F24_32,
            57 => RiscVRegister::F25_32,
            58 => RiscVRegister::F26_32,
            59 => RiscVRegister::F27_32,
            60 => RiscVRegister::F28_32,
            61 => RiscVRegister::F29_32,
            62 => RiscVRegister::F30_32,
            63 => RiscVRegister::F31_32,
            64 => RiscVRegister::F0_64,
            65 => RiscVRegister::F1_64,
            66 => RiscVRegister::F2_64,
            67 => RiscVRegister::F3_64,
            68 => RiscVRegister::F4_64,
            69 => RiscVRegister::F5_64,
            70 => RiscVRegister::F6_64,
            71 => RiscVRegister::F7_64,
            72 => RiscVRegister::F8_64,
            73 => RiscVRegister::F9_64,
            74 => RiscVRegister::F10_64,
            75 => RiscVRegister::F11_64,
            76 => RiscVRegister::F12_64,
            77 => RiscVRegister::F13_64,
            78 => RiscVRegister::F14_64,
            79 => RiscVRegister::F15_64,
            80 => RiscVRegister::F16_64,
            81 => RiscVRegister::F17_64,
            82 => RiscVRegister::F18_64,
            83 => RiscVRegister::F19_64,
            84 => RiscVRegister::F20_64,
            85 => RiscVRegister::F21_64,
            86 => RiscVRegister::F22_64,
            87 => RiscVRegister::F23_64,
            88 => RiscVRegister::F24_64,
            89 => RiscVRegister::F25_64,
            90 => RiscVRegister::F26_64,
            91 => RiscVRegister::F27_64,
            92 => RiscVRegister::F28_64,
            93 => RiscVRegister::F29_64,
            94 => RiscVRegister::F30_64,
            95 => RiscVRegister::F31_64,
            _ => RiscVRegister::Invalid,
        }
    }
}

/// Instruction encoding formats available in RISC-V.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVInstructionFormat {
    /// R-type (register-register) format.
    R,
    /// I-type (immediate) format.
    I,
    /// S-type (store) format.
    S,
    /// B-type (branch) format.
    B,
    /// U-type (upper immediate) format.
    U,
    /// J-type (jump and link) format.
    J,
    /// R4-type (fused floating multiply-add) format.
    R4,
    /// Compressed instruction formats.
    CR,
    CI,
    CSS,
    CIW,
    CL,
    CS,
    CA,
    CB,
    CJ,
}

pub const HEX_THRESHOLD: i64 = 9;
