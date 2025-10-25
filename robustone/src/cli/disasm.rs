use crate::cli::DisasmConfig;
use transfer::{ArchitectureDispatcher, DisasmError, Instruction};

// 全局架构分发器实例
lazy_static::lazy_static! {
    static ref DISPATCHER: ArchitectureDispatcher = ArchitectureDispatcher::new();
}

#[derive(Debug)]
pub struct DisassemblyResult {
    pub instructions: Vec<Instruction>,
}

impl DisassemblyResult {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}

impl Iterator for DisassemblyResult {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instructions.is_empty() {
            None
        } else {
            Some(self.instructions.remove(0))
        }
    }
}

/// 处理输入内容并执行反汇编
pub fn process_input(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    let mut result = DisassemblyResult::new();
    for word in &config.hex_words {
        let instr = DISPATCHER.disassemble(word);
        result.add_instruction(instr);
    }
    Ok(result)
}

/// 兼容旧接口：按配置整体反汇编
pub fn disassemble(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    process_input(config)
}

/// 打印反汇编结果
pub fn print_instructions(result: &DisassemblyResult, config: &DisasmConfig) {
    for instr in &result.instructions {
        if config.detailed {
            println!(
                "0x{:016x}: {:<20} {:<10} {}",
                instr.address,
                hex::encode(&instr.bytes),
                instr.mnemonic,
                instr.operands
            );
        } else {
            println!(
                "0x{:016x}: {} {}",
                instr.address, instr.mnemonic, instr.operands
            );
        }
    }
}
