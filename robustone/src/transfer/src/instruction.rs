//! 指令相关的核心数据结构

use crate::riscv::types::RiscVOperand;

/// 反汇编结果
#[derive(Debug, Clone)]
pub struct Instruction {
    /// 指令地址
    pub address: u64,
    /// 指令字节
    pub bytes: Vec<u8>,
    /// 指令助记符
    pub mnemonic: String,
    /// 操作数字符串
    pub operands: String,
    /// 指令大小
    pub size: usize,
    /// 详细信息
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

/// 指令详细信息
#[derive(Debug, Clone)]
pub struct InstructionDetail {
    /// 操作数列表
    pub operands: Vec<RiscVOperand>,
    /// 读取的寄存器
    pub regs_read: Vec<u32>,
    /// 写入的寄存器
    pub regs_write: Vec<u32>,
    /// 指令分组
    pub groups: Vec<String>,
}

impl Default for InstructionDetail {
    fn default() -> Self {
        Self {
            operands: Vec::new(),
            regs_read: Vec::new(),
            regs_write: Vec::new(),
            groups: Vec::new(),
        }
    }
}