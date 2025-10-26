//! RISC-V disassembly module.
//!
//! Provides instruction decoding for RISC-V, including:
//! - 32-bit and 64-bit profiles
//! - Standard and compressed (RVC) encodings
//! - Core ISA extensions (I, M, A, F, D, C)

pub mod decoder;
pub mod printer;
pub mod types;
pub mod extensions;
pub mod shared;

use crate::{
    ArchitectureHandler,
    error::DisasmError,
    instruction::{Instruction, InstructionDetail},
};
use decoder::{RiscVDecoder, Xlen};
use types::*;

/// Architecture handler implementation for RISC-V targets.
pub struct RiscVHandler {
    /// Decoder used to translate raw bytes into structured instructions.
    decoder: RiscVDecoder,
}

impl RiscVHandler {
    /// Creates a new handler configured for 64-bit RISC-V with the base extension set.
    pub fn new() -> Self {
        // Default to the base I extension in 64-bit mode.
        let extensions = 0b001; // I extension
        Self {
            decoder: RiscVDecoder::new(Xlen::X64, extensions),
        }
    }

    /// Creates a handler targeting RV32.
    pub fn rv32() -> Self {
        Self {
            decoder: RiscVDecoder::rv32(),
        }
    }

    /// Creates a handler targeting RV64.
    pub fn rv64() -> Self {
        Self {
            decoder: RiscVDecoder::rv64(),
        }
    }

    /// Creates a handler with custom XLEN and extension flags.
    pub fn with_extensions(xlen: Xlen, extensions: u32) -> Self {
        Self {
            decoder: RiscVDecoder::new(xlen, extensions),
        }
    }
}

impl Default for RiscVHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for RiscVHandler {
    fn disassemble(&self, bytes: &[u8], addr: u64) -> Result<(Instruction, usize), DisasmError> {
        // Decode the instruction with the dedicated RISC-V decoder.
        let decoded = self.decoder.decode(bytes, addr)?;

        // Assemble the high-level instruction detail payload.
        let instruction_detail = InstructionDetail {
            operands: decoded.operands_detail.clone(),
            regs_read: decoded
                .operands_detail
                .iter()
                .filter(|op| matches!(op.op_type, RiscVOperandType::Register) && op.access.read)
                .map(|op| match op.value {
                    RiscVOperandValue::Register(reg) => reg,
                    _ => 0,
                })
                .collect(),
            regs_write: decoded
                .operands_detail
                .iter()
                .filter(|op| matches!(op.op_type, RiscVOperandType::Register) && op.access.write)
                .map(|op| match op.value {
                    RiscVOperandValue::Register(reg) => reg,
                    _ => 0,
                })
                .collect(),
            groups: vec!["riscv".to_string()],
        };

        Ok((
            Instruction {
                address: addr,
                bytes: bytes[..decoded.size].to_vec(),
                mnemonic: decoded.mnemonic,
                operands: decoded.operands,
                size: decoded.size,
                detail: Some(instruction_detail),
            },
            decoded.size,
        ))
    }

    fn name(&self) -> &'static str {
        "riscv"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "riscv32" | "riscv64" | "riscv")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_riscv_handler_creation() {
        let handler = RiscVHandler::new();
        assert_eq!(handler.name(), "riscv");
        assert!(handler.supports("riscv32"));
        assert!(handler.supports("riscv64"));
        assert!(handler.supports("riscv"));
        assert!(!handler.supports("arm"));
    }

    #[test]
    fn test_riscv_register_names() {
        assert_eq!(RiscVRegister::X0.name(), "zero");
        assert_eq!(RiscVRegister::X1.name(), "ra");
        assert_eq!(RiscVRegister::X2.name(), "sp");
        assert_eq!(RiscVRegister::X5.name(), "t0");
        assert_eq!(RiscVRegister::X10.name(), "a0");
    }

    #[test]
    fn test_riscv_register_from_id() {
        assert_eq!(RiscVRegister::from_id(0), RiscVRegister::X0);
        assert_eq!(RiscVRegister::from_id(1), RiscVRegister::X1);
        assert_eq!(RiscVRegister::from_id(32), RiscVRegister::Invalid);
        assert_eq!(RiscVRegister::from_id(100), RiscVRegister::Invalid);
    }

    #[test]
    fn test_access_types() {
        let read_access = Access::read();
        assert!(read_access.read && !read_access.write);

        let write_access = Access::write();
        assert!(!write_access.read && write_access.write);

        let rw_access = Access::read_write();
        assert!(rw_access.read && rw_access.write);

        let none_access = Access::none();
        assert!(!none_access.read && !none_access.write);
    }
}
