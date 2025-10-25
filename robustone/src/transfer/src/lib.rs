//! Transfer库 - 反汇编引擎核心库
//!
//! 基于Capstone架构设计的Rust反汇编引擎，支持多架构指令集。

pub mod error;
pub mod instruction;

// 重新导出核心类型
pub use error::DisasmError;
pub use instruction::{Instruction, InstructionDetail};

use core::str;

/// 架构处理接口
pub trait ArchitectureHandler: Sync {
    /// 反汇编一条指令，返回(指令, 消耗的字节数)
    fn disassemble(
        &self,
        bytes: &[u8],
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError>;

    /// 获取架构名称
    fn name(&self) -> &'static str;

    /// 检查是否支持该架构（用字符串匹配，避免对上层的依赖）
    fn supports(&self, arch_name: &str) -> bool;
}

/// 架构分发器
pub struct ArchitectureDispatcher {
    handlers: Vec<Box<dyn ArchitectureHandler>>,
}

impl ArchitectureDispatcher {
    /// 创建新的分发器并注册所有架构处理器
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn ArchitectureHandler>> = Vec::new();

        // 注册RISC-V处理器
        #[cfg(feature = "riscv")]
        {
            use crate::riscv::RiscVHandler;
            handlers.push(Box::new(RiscVHandler::new()));
        }

        Self { handlers }
    }

    /// 反汇编单个指令
    pub fn disassemble(&self, hex: &str, arch: String) -> Instruction {
        // 查找支持该架构的处理器
        for handler in &self.handlers {
            if handler.supports(&arch) {
                // 解析 0x 前缀并转换为字节
                let s = hex.trim().to_lowercase();
                let no_prefix = if s.starts_with("0x") { &s[2..] } else { &s };
                let mut bytes = Vec::new();
                let mut i = 0;
                while i + 1 < no_prefix.len() {
                    let b = u8::from_str_radix(&no_prefix[i..i + 2], 16).unwrap_or(0);
                    bytes.push(b);
                    i += 2;
                }

                // 对于RISC-V架构，十六进制字符串表示指令值，需要转换为小端字节序
                if handler.name() == "riscv" && bytes.len() == 4 {
                    // 将指令值转换为小端字节序
                    let instruction_value = u32::from_str_radix(no_prefix, 16).unwrap_or(0);
                    bytes = instruction_value.to_le_bytes().to_vec();
                } else if handler.name() == "riscv" && bytes.len() == 2 {
                    // 16位指令也需要转换为小端字节序
                    let instruction_value = u16::from_str_radix(no_prefix, 16).unwrap_or(0);
                    bytes = instruction_value.to_le_bytes().to_vec();
                }

    
                // 尝试反汇编
                if let Ok((instruction, _size)) = handler.disassemble(&bytes, 0) {
                    return instruction;
                }
                break;
            }
        }

        // 如果没有找到支持的处理器或反汇编失败，返回基础指令
        let s = hex.trim().to_lowercase();
        let no_prefix = if s.starts_with("0x") { &s[2..] } else { &s };
        let mut bytes = Vec::new();
        let mut i = 0;
        while i + 1 < no_prefix.len() {
            let b = u8::from_str_radix(&no_prefix[i..i + 2], 16).unwrap_or(0);
            bytes.push(b);
            i += 2;
        }

        let size = bytes.len();
        Instruction {
            address: 0,
            bytes,
            mnemonic: "unknown".to_string(),
            operands: "".to_string(),
            size,
            detail: None,
        }
    }

    /// 获取所有支持的架构
    pub fn supported_architectures(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|h| h.name()).collect()
    }
}

impl Default for ArchitectureDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// RISC-V模块 (默认包含)
#[cfg(feature = "riscv")]
pub mod riscv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_dispatcher_creation() {
        let dispatcher = ArchitectureDispatcher::new();
        let archs = dispatcher.supported_architectures();

        // 应该至少包含RISC-V (如果启用)
        #[cfg(feature = "riscv")]
        assert!(archs.contains(&"riscv"));
    }

    #[test]
    fn test_hex_parsing() {
        let dispatcher = ArchitectureDispatcher::new();

        // 测试十六进制解析
        let instruction = dispatcher.disassemble("deadbeef", "unknown".to_string());
        assert_eq!(instruction.mnemonic, "unknown");
        assert_eq!(instruction.bytes, vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(instruction.size, 4);

        // 测试带0x前缀
        let instruction = dispatcher.disassemble("0x1234", "unknown".to_string());
        assert_eq!(instruction.bytes, vec![0x12, 0x34]);
        assert_eq!(instruction.size, 2);
    }
}