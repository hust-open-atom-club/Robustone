//! RISC-V data types used by the transfer crate.
//!
//! These structures mirror Capstone's RISC-V bindings to ease interoperability.

use crate::riscv::shared::registers::{get_fp_register_name, get_register_name};

/// Kinds of operands that can appear in a RISC-V instruction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RiscVOperandType {
    /// Placeholder for invalid operands.
    #[default]
    Invalid,
    /// Register operand.
    Register,
    /// Immediate operand.
    Immediate,
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RiscVOperand {
    /// Operand classification.
    pub op_type: RiscVOperandType,
    /// Access behaviour for the operand.
    pub access: Access,
    /// Encoded operand payload.
    pub value: RiscVOperandValue,
}

/// Concrete value carried by an operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVOperandValue {
    /// Register identifier.
    Register(u32),
    /// Immediate literal.
    Immediate(i64),
    /// Memory addressing mode.
    Memory(RiscVMemoryOperand),
}

impl Default for RiscVOperandValue {
    fn default() -> Self {
        Self::Immediate(0)
    }
}

/// Register access flags (mirrors `cs_ac_type`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

/// Instruction-level detail (mirrors `cs_riscv`).
#[derive(Debug, Default, Clone)]
pub struct RiscVInstructionDetail {
    /// Whether an effective address is required.
    pub need_effective_addr: bool,
    /// Number of populated operands.
    pub op_count: u8,
    /// Operand list (up to eight entries).
    pub operands: [RiscVOperand; 8],
    /// Registers read by the instruction.
    pub regs_read: Vec<u32>,
    /// Registers written by the instruction.
    pub regs_write: Vec<u32>,
    /// Instruction group tags.
    pub groups: Vec<String>,
}

/// Comprehensive RISC-V register enumeration (compatible with `riscv_reg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

impl From<u32> for RiscVRegister {
    /// Converts a raw register ID (x0=0, x1=1, …, x31=31) into the enum representation.
    fn from(value: u32) -> Self {
        match value {
            0..=31 => unsafe { std::mem::transmute(value as u8 + 1) },
            _ => RiscVRegister::Invalid,
        }
    }
}

impl RiscVRegister {
    /// Returns the canonical register name.
    pub fn name(self) -> &'static str {
        let reg = self as u8;
        if reg >= 1 && reg <= 32 {
            get_register_name(reg - 1)
        } else if reg >= 33 && reg <= 64 {
            get_fp_register_name(reg - 33)
        } else {
            "invalid"
        }
    }

    /// Converts a raw register ID (x0=0, x1=1, …, x31=31) into the enum representation.
    pub fn from_id(id: u32) -> Self {
        Self::from(id)
    }
}

/// Supported RISC-V ISA extensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiscVExtension {
    /// Base integer ISA.
    I,
    /// Multiply/divide extension.
    M,
    /// Atomic operations extension.
    A,
    /// Single-precision floating-point extension.
    F,
    /// Double-precision floating-point extension.
    D,
    /// Compressed instruction extension.
    C,
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
