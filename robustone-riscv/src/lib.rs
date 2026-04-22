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

pub mod architecture {
    pub use robustone_core::architecture::*;
}

pub mod common {
    pub use robustone_core::common::*;
}

pub mod ir {
    pub use robustone_core::ir::*;
}

pub mod utils {
    pub use robustone_core::utils::*;
}

pub mod riscv {
    pub use crate::arch;
    pub use crate::decoder;
    pub use crate::extensions;
    pub use crate::printer;
    pub use crate::shared;
    pub use crate::types;
}

pub use robustone_core::Instruction;

use arch::RiscVInstructionDetail;
use decoder::{RiscVDecoder, Xlen};
use extensions::Extensions;
use robustone_core::{
    common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    traits::instruction::Detail, types::error::DisasmError,
};

/// Architecture handler implementation for RISC-V targets.
pub struct RiscVHandler {
    rv32_decoder: RiscVDecoder,
    rv64_decoder: RiscVDecoder,
    configured_xlen: Option<Xlen>,
    detail: bool,
}

impl RiscVHandler {
    /// Creates a new handler with both RV32GC and RV64GC decoders.
    pub fn new() -> Self {
        Self {
            rv32_decoder: RiscVDecoder::rv32gc(),
            rv64_decoder: RiscVDecoder::rv64gc(),
            configured_xlen: None,
            detail: true,
        }
    }

    /// Creates a handler targeting RV32GC.
    pub fn rv32() -> Self {
        Self {
            rv32_decoder: RiscVDecoder::rv32gc(),
            rv64_decoder: RiscVDecoder::rv64gc(),
            configured_xlen: Some(Xlen::X32),
            detail: true,
        }
    }

    /// Creates a handler targeting RV64GC.
    pub fn rv64() -> Self {
        Self {
            rv32_decoder: RiscVDecoder::rv32gc(),
            rv64_decoder: RiscVDecoder::rv64gc(),
            configured_xlen: Some(Xlen::X64),
            detail: true,
        }
    }

    /// Creates a handler with custom XLEN and extension flags.
    pub fn with_extensions(xlen: Xlen, extensions: Extensions) -> Self {
        match xlen {
            Xlen::X32 => Self {
                rv32_decoder: RiscVDecoder::new(Xlen::X32, extensions),
                rv64_decoder: RiscVDecoder::rv64gc(),
                configured_xlen: Some(Xlen::X32),
                detail: true,
            },
            Xlen::X64 => Self {
                rv32_decoder: RiscVDecoder::rv32gc(),
                rv64_decoder: RiscVDecoder::new(Xlen::X64, extensions),
                configured_xlen: Some(Xlen::X64),
                detail: true,
            },
        }
    }

    fn decoder_for_arch(&self, arch_name: &str) -> Result<&RiscVDecoder, DisasmError> {
        match (self.configured_xlen, arch_name) {
            (Some(Xlen::X32), "riscv32") => Ok(&self.rv32_decoder),
            (Some(Xlen::X64), "riscv64" | "riscv") => Ok(&self.rv64_decoder),
            (Some(_), _) => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
            (None, "riscv32") => Ok(&self.rv32_decoder),
            (None, "riscv64" | "riscv") => Ok(&self.rv64_decoder),
            _ => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
        }
    }

    pub fn from_profile(profile: &ArchitectureProfile) -> Result<Self, DisasmError> {
        let decoder = RiscVDecoder::from_profile(profile)?;
        match &profile.architecture {
            crate::architecture::Architecture::RiscV32 => Ok(Self {
                rv32_decoder: decoder,
                rv64_decoder: RiscVDecoder::rv64gc(),
                configured_xlen: Some(Xlen::X32),
                detail: true,
            }),
            crate::architecture::Architecture::RiscV64 => Ok(Self {
                rv32_decoder: RiscVDecoder::rv32gc(),
                rv64_decoder: decoder,
                configured_xlen: Some(Xlen::X64),
                detail: true,
            }),
            other => Err(DisasmError::UnsupportedArchitecture(
                other.as_str().to_string(),
            )),
        }
    }
}

impl Default for RiscVHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for RiscVHandler {
    fn set_detail(&mut self, detail: bool) {
        self.detail = detail;
    }

    fn decode_instruction(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let decoder = self.decoder_for_arch(arch_name)?;
        let decoded = decoder.decode(bytes, arch_name, addr)?;
        let size = decoded.size;
        Ok((decoded, size))
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let handler = Self::from_profile(profile)?;
        handler.decode_instruction(bytes, profile.mode_name, addr)
    }

    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let decoder = self.decoder_for_arch(arch_name)?;
        let ir = decoder.decode(bytes, arch_name, addr)?;
        let (mnemonic, operands) = ir.render_capstone_text_parts();

        let detail: Option<Box<dyn Detail>> = if self.detail {
            let mut riscv_detail = RiscVInstructionDetail::new();
            for register in ir
                .registers_read
                .iter()
                .chain(ir.implicit_registers_read.iter())
            {
                if !riscv_detail.regs_read.contains(&register.id) {
                    riscv_detail = riscv_detail.reads_register(register.id);
                }
            }
            for register in ir
                .registers_written
                .iter()
                .chain(ir.implicit_registers_written.iter())
            {
                if !riscv_detail.regs_write.contains(&register.id) {
                    riscv_detail = riscv_detail.writes_register(register.id);
                }
            }
            Some(Box::new(riscv_detail))
        } else {
            None
        };

        let size = ir.size;
        let instruction = Instruction::from_decoded(ir, mnemonic, operands, detail);
        Ok((instruction, size))
    }

    fn disassemble_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let mut handler = Self::from_profile(profile)?;
        handler.set_detail(self.detail);
        handler.disassemble(bytes, profile.mode_name, addr)
    }

    fn name(&self) -> &'static str {
        "riscv"
    }

    fn supports(&self, arch_name: &str) -> bool {
        match self.configured_xlen {
            Some(Xlen::X32) => matches!(arch_name, "riscv32"),
            Some(Xlen::X64) => matches!(arch_name, "riscv64" | "riscv"),
            None => matches!(arch_name, "riscv32" | "riscv64" | "riscv"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::types::{Access, RiscVRegister};

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
    fn test_with_extensions_limits_supported_architectures() {
        let handler = RiscVHandler::with_extensions(Xlen::X32, Extensions::rv32gc());
        assert!(handler.supports("riscv32"));
        assert!(!handler.supports("riscv64"));
        assert!(!handler.supports("riscv"));

        let error = handler
            .disassemble(&[0x83, 0x30, 0x00, 0x00], "riscv64", 0)
            .expect_err("RV32-only handler should reject riscv64 requests");
        assert!(matches!(error, DisasmError::UnsupportedArchitecture(_)));
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
        assert_eq!(RiscVRegister::from_id(32), RiscVRegister::F0_32);
        assert_eq!(RiscVRegister::from_id(64), RiscVRegister::F0_64);
        assert_eq!(RiscVRegister::from_id(95), RiscVRegister::F31_64);
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

    #[test]
    fn test_atomic_doubleword_is_not_tagged_as_floating_point() {
        let handler = RiscVHandler::rv64();
        let (decoded, _) = handler
            .decode_instruction(&[0x2f, 0xb4, 0x02, 0x12], "riscv64", 0)
            .expect("lr.d should decode");

        assert!(decoded.groups.iter().any(|group| group == "atomic"));
        assert!(!decoded.groups.iter().any(|group| group == "floating_point"));
    }

    #[test]
    fn test_disassemble_merges_implicit_register_writes_into_detail() {
        let handler = RiscVHandler::rv32();
        let (instruction, size) = handler
            .disassemble(&[0x85, 0x20], "riscv32", 0)
            .expect("c.jal should decode");

        assert_eq!(size, 2);
        assert_eq!(instruction.mnemonic, "jal");

        let detail = instruction.detail.expect("detail should be populated");
        assert_eq!(detail.registers_written(), &[1]);
    }
}
