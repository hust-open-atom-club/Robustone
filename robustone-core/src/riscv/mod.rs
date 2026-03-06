//! RISC-V disassembly module.
//!
//! Provides instruction decoding for RISC-V, including:
//! - 32-bit and 64-bit profiles
//! - Standard and compressed (RVC) encodings
//! - Core ISA extensions (I, M, A, F, D, C)
//!
//! This module implements the generic `Architecture` trait for RISC-V
//! and provides both modern architecture-aware interfaces and legacy
//! compatibility interfaces.

pub mod arch;
pub mod decoder;
pub mod extensions;
pub mod printer;
pub mod shared;
pub mod types;

use crate::{
    traits::ArchitectureHandler, types::error::DisasmError, types::instruction::Instruction,
};
use arch::RiscVInstructionDetail;
use decoder::{RiscVDecoder, Xlen};
use extensions::Extensions;
use types::*;

/// Architecture handler implementation for RISC-V targets.
pub struct RiscVHandler {
    rv32_decoder: RiscVDecoder,
    rv64_decoder: RiscVDecoder,
}

impl RiscVHandler {
    /// Creates a new handler with both RV32GC and RV64GC decoders.
    pub fn new() -> Self {
        Self {
            rv32_decoder: RiscVDecoder::rv32gc(),
            rv64_decoder: RiscVDecoder::rv64gc(),
        }
    }

    /// Creates a handler targeting RV32GC.
    pub fn rv32() -> Self {
        Self::new()
    }

    /// Creates a handler targeting RV64GC.
    pub fn rv64() -> Self {
        Self::new()
    }

    /// Creates a handler with custom XLEN and extension flags.
    pub fn with_extensions(xlen: Xlen, extensions: Extensions) -> Self {
        match xlen {
            Xlen::X32 => Self {
                rv32_decoder: RiscVDecoder::new(Xlen::X32, extensions),
                rv64_decoder: RiscVDecoder::rv64gc(),
            },
            Xlen::X64 => Self {
                rv32_decoder: RiscVDecoder::rv32gc(),
                rv64_decoder: RiscVDecoder::new(Xlen::X64, extensions),
            },
        }
    }

    fn decoder_for_arch(&self, arch_name: &str) -> Result<&RiscVDecoder, DisasmError> {
        match arch_name {
            "riscv32" => Ok(&self.rv32_decoder),
            "riscv64" | "riscv" => Ok(&self.rv64_decoder),
            _ => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
        }
    }
}

impl Default for RiscVHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for RiscVHandler {
    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let decoder = self.decoder_for_arch(arch_name)?;

        // Decode the instruction with the dedicated RISC-V decoder.
        let decoded = decoder.decode(bytes, addr)?;

        // Create simple instruction detail with register information
        let mut riscv_detail = RiscVInstructionDetail::new();

        // Track register usage from operands
        for operand in &decoded.operands_detail {
            if matches!(operand.op_type, RiscVOperandType::Register) {
                if operand.access.read
                    && let RiscVOperandValue::Register(reg) = operand.value
                {
                    riscv_detail = riscv_detail.reads_register(reg);
                }
                if operand.access.write
                    && let RiscVOperandValue::Register(reg) = operand.value
                {
                    riscv_detail = riscv_detail.writes_register(reg);
                }
            }
        }

        Ok((
            Instruction::with_detail(
                addr,
                bytes[..decoded.size].to_vec(),
                decoded.mnemonic,
                decoded.operands,
                Box::new(riscv_detail),
            ),
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
