//! RISC-V架构反汇编模块
//!
//! 提供RISC-V架构的指令反汇编功能，支持：
//! - RISC-V 32位和64位
//! - 压缩指令(RVC)和标准指令
//! - 基础指令集扩展 (I, M, A, F, D, C)

pub mod types;
pub mod decoder;
pub mod printer;

use crate::{ArchitectureHandler, Instruction, DisasmError};
use types::*;
use decoder::{RiscVDecoder, Xlen};

/// RISC-V架构处理器
pub struct RiscVHandler {
    /// RISC-V指令解码器
    decoder: RiscVDecoder,
}

impl RiscVHandler {
    /// 创建新的RISC-V处理器 (默认64位)
    pub fn new() -> Self {
        // 默认支持 I 基础扩展，64位模式
        let extensions = 0b001; // I扩展
        Self {
            decoder: RiscVDecoder::new(Xlen::X64, extensions),
        }
    }

    /// 创建32位RISC-V处理器
    pub fn rv32() -> Self {
        Self {
            decoder: RiscVDecoder::rv32(),
        }
    }

    /// 创建64位RISC-V处理器
    pub fn rv64() -> Self {
        Self {
            decoder: RiscVDecoder::rv64(),
        }
    }

    /// 创建支持特定扩展的RISC-V处理器
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
    fn disassemble(
        &self,
        bytes: &[u8],
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        // 使用RISC-V解码器解码指令
        let decoded = self.decoder.decode(bytes, addr)?;

        // 构建指令详细信息
        let instruction_detail = crate::InstructionDetail {
            operands: decoded.operands_detail.clone(),
            regs_read: decoded.operands_detail
                .iter()
                .filter(|op| matches!(op.op_type, RiscVOperandType::Register) && op.access.read)
                .map(|op| match op.value {
                    RiscVOperandValue::Register(reg) => reg,
                    _ => 0,
                })
                .collect(),
            regs_write: decoded.operands_detail
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