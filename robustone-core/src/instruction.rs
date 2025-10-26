//! Core data structures shared by decoded instructions.

use crate::riscv::types::RiscVOperand;

/// Decoded instruction returned by the disassembler.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Instruction address.
    pub address: u64,
    /// Raw instruction bytes in execution order.
    pub bytes: Vec<u8>,
    /// Instruction mnemonic.
    pub mnemonic: String,
    /// Formatted operand string.
    pub operands: String,
    /// Size of the instruction in bytes.
    pub size: usize,
    /// Optional architecture-specific detail payload.
    pub detail: Option<InstructionDetail>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            address: 0,
            bytes: Vec::new(),
            mnemonic: "unknown".to_string(),
            operands: String::new(),
            size: 0,
            detail: None,
        }
    }
}

/// Expanded instruction metadata for architectures that provide it.
#[derive(Debug, Clone, Default)]
pub struct InstructionDetail {
    /// Normalized operands as structured data.
    pub operands: Vec<RiscVOperand>,
    /// Registers read by the instruction.
    pub regs_read: Vec<u32>,
    /// Registers written by the instruction.
    pub regs_write: Vec<u32>,
    /// Group tags describing semantic categories.
    pub groups: Vec<String>,
}

// Default is derived above
