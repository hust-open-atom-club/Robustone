//! RISC-V指令格式化输出器
//!
//! 基于Capstone RISC-V打印机实现的指令格式化功能

use super::types::*;
use crate::Instruction;

/// RISC-V指令打印机
pub struct RiscVPrinter {
    /// 是否显示详细寄存器信息
    alias_regs: bool,
    /// 是否显示无符号立即数
    unsigned_immediate: bool,
}

impl RiscVPrinter {
    /// 创建新的RISC-V打印机
    pub fn new() -> Self {
        Self {
            alias_regs: false,
            unsigned_immediate: false,
        }
    }

    /// 设置是否使用寄存器别名
    pub fn with_alias_regs(mut self, alias_regs: bool) -> Self {
        self.alias_regs = alias_regs;
        self
    }

    /// 设置是否显示无符号立即数
    pub fn with_unsigned_immediate(mut self, unsigned_immediate: bool) -> Self {
        self.unsigned_immediate = unsigned_immediate;
        self
    }

    /// 格式化立即数
    fn format_immediate(&self, imm: i64) -> String {
        if self.unsigned_immediate {
            if imm >= 0 && imm > 0xFF {
                format!("0x{:x}", imm)
            } else if imm >= 0 {
                format!("{}", imm)
            } else if imm < -0xFF {
                format!("-0x{:x}", -imm)
            } else {
                format!("{}", imm)
            }
        } else {
            if imm >= 0 && imm > 0xFF {
                format!("0x{:x}", imm)
            } else if imm >= 0 {
                format!("{}", imm)
            } else if imm < -0xFF {
                format!("-0x{:x}", -imm)
            } else {
                format!("{}", imm)
            }
        }
    }

    /// 格式化寄存器
    fn format_register(&self, reg_id: u32) -> String {
        let reg = RiscVRegister::from_id(reg_id);
        if self.alias_regs {
            reg.name().to_string()
        } else {
            // 使用x0-x31格式
            if reg_id <= 31 {
                format!("x{}", reg_id)
            } else {
                reg.name().to_string()
            }
        }
    }

    /// 格式化内存操作数
    fn format_memory_operand(&self, base: u32, disp: i64) -> String {
        if disp == 0 {
            format!("({})", self.format_register(base))
        } else if disp > 0 {
            format!("{}({})", self.format_immediate(disp), self.format_register(base))
        } else {
            format!("-{}({})", self.format_immediate(-disp), self.format_register(base))
        }
    }

    /// 格式化操作数
    pub fn format_operand(&self, operand: &RiscVOperand) -> String {
        match &operand.value {
            RiscVOperandValue::Register(reg_id) => {
                self.format_register(*reg_id)
            }
            RiscVOperandValue::Immediate(imm) => {
                self.format_immediate(*imm)
            }
            RiscVOperandValue::Memory(mem) => {
                self.format_memory_operand(mem.base, mem.disp)
            }
        }
    }

    /// 格式化指令操作数字符串
    pub fn format_operands(&self, operands: &[RiscVOperand]) -> String {
        if operands.is_empty() {
            String::new()
        } else {
            operands
                .iter()
                .map(|op| self.format_operand(op))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// 打印指令基本信息
    pub fn print_basic(&self, instruction: &Instruction) -> String {
        format!("{} {}", instruction.mnemonic, instruction.operands)
    }

    /// 打印指令详细信息
    pub fn print_detailed(&self, instruction: &Instruction) -> String {
        let mut result = Vec::new();

        // 基本信息
        result.push(format!(
            "0x{:016x}: {} {}",
            instruction.address,
            hex::encode(&instruction.bytes),
            self.print_basic(instruction)
        ));

        // 如果有详细信息，打印详细内容
        if let Some(detail) = &instruction.detail {
            // ID信息 (如果有的话)
            if !detail.groups.is_empty() {
                result.push(format!("\tGroups: {}", detail.groups.join(", ")));
            }

            // 操作数信息
            if !detail.operands.is_empty() {
                result.push(format!("\tOperand count: {}", detail.operands.len()));
                for (i, operand) in detail.operands.iter().enumerate() {
                    let access_str = match (operand.access.read, operand.access.write) {
                        (true, true) => "READ | WRITE",
                        (true, false) => "READ",
                        (false, true) => "WRITE",
                        (false, false) => "NONE",
                    };

                    let operand_type_str = match operand.op_type {
                        RiscVOperandType::Register => "REG",
                        RiscVOperandType::Immediate => "IMM",
                        RiscVOperandType::Memory => "MEM",
                        RiscVOperandType::Invalid => "INVALID",
                    };

                    let value_str = match &operand.value {
                        RiscVOperandValue::Register(reg) => {
                            format!("{} ({})", self.format_register(*reg), reg)
                        }
                        RiscVOperandValue::Immediate(imm) => {
                            format!("{} ({})", self.format_immediate(*imm), imm)
                        }
                        RiscVOperandValue::Memory(mem) => {
                            format!("{} (base={}, disp={})",
                                self.format_memory_operand(mem.base, mem.disp),
                                mem.base,
                                mem.disp)
                        }
                    };

                    result.push(format!("\toperands[{}].type: {} = {}",
                        i, operand_type_str, value_str));
                    result.push(format!("\toperands[{}].access: {}", i, access_str));
                }
            }

            // 寄存器访问信息
            if !detail.regs_read.is_empty() {
                let regs_read: Vec<String> = detail.regs_read
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters read: {}", regs_read.join(", ")));
            }

            if !detail.regs_write.is_empty() {
                let regs_write: Vec<String> = detail.regs_write
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters modified: {}", regs_write.join(", ")));
            }
        }

        result.join("\n")
    }

    /// 打印指令的十六进制字节
    pub fn print_hex_bytes(&self, instruction: &Instruction, align_width: usize) -> String {
        let hex_str = hex::encode(&instruction.bytes);
        let padding = if hex_str.len() < align_width {
            " ".repeat(align_width - hex_str.len())
        } else {
            String::new()
        };
        format!("{}{}", hex_str, padding)
    }
}

impl Default for RiscVPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// RISC-V指令格式化工具函数
pub mod format {
    use super::*;

    /// 格式化RISC-V指令为基本格式
    pub fn basic_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_basic(instruction)
    }

    /// 格式化RISC-V指令为详细格式
    pub fn detailed_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_detailed(instruction)
    }

    /// 格式化指令列表
    pub fn instruction_list(instructions: &[Instruction], detailed: bool) -> String {
        let printer = RiscVPrinter::new();
        let mut result = Vec::new();

        for instruction in instructions {
            if detailed {
                result.push(printer.print_detailed(instruction));
            } else {
                result.push(format!(
                    "0x{:016x}: {} {}",
                    instruction.address,
                    printer.print_hex_bytes(instruction, 16),
                    printer.print_basic(instruction)
                ));
            }
        }

        result.join("\n")
    }

    /// 格式化操作数列表
    pub fn operands_list(operands: &[RiscVOperand]) -> String {
        let printer = RiscVPrinter::new();
        printer.format_operands(operands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_creation() {
        let printer = RiscVPrinter::new();
        assert!(!printer.alias_regs);
        assert!(!printer.unsigned_immediate);

        let printer = RiscVPrinter::new()
            .with_alias_regs(true)
            .with_unsigned_immediate(true);
        assert!(printer.alias_regs);
        assert!(printer.unsigned_immediate);
    }

    #[test]
    fn test_format_immediate() {
        let printer = RiscVPrinter::new();

        // 正数
        assert_eq!(printer.format_immediate(10), "10");
        assert_eq!(printer.format_immediate(0x1000), "0x1000");

        // 负数
        assert_eq!(printer.format_immediate(-10), "-10");
        assert_eq!(printer.format_immediate(-0x1000), "-0x1000");

        // 零
        assert_eq!(printer.format_immediate(0), "0");
    }

    #[test]
    fn test_format_register() {
        let printer = RiscVPrinter::new();

        // 普通格式
        assert_eq!(printer.format_register(0), "x0");
        assert_eq!(printer.format_register(1), "x1");
        assert_eq!(printer.format_register(10), "x10");

        // 别名格式
        let printer_with_alias = printer.with_alias_regs(true);
        assert_eq!(printer_with_alias.format_register(0), "zero");
        assert_eq!(printer_with_alias.format_register(1), "ra");
        assert_eq!(printer_with_alias.format_register(10), "a0");
    }

    #[test]
    fn test_format_memory_operand() {
        let printer = RiscVPrinter::new();

        // 只有基址寄存器
        assert_eq!(printer.format_memory_operand(2, 0), "(sp)");

        // 正偏移
        assert_eq!(printer.format_memory_operand(10, 100), "100(a0)");

        // 负偏移
        assert_eq!(printer.format_memory_operand(10, -100), "-100(a0)");
    }

    #[test]
    fn test_format_operand() {
        let printer = RiscVPrinter::new();

        // 寄存器操作数
        let reg_op = RiscVOperand {
            op_type: RiscVOperandType::Register,
            access: Access::read(),
            value: RiscVOperandValue::Register(10),
        };
        assert_eq!(printer.format_operand(&reg_op), "x10");

        // 立即数操作数
        let imm_op = RiscVOperand {
            op_type: RiscVOperandType::Immediate,
            access: Access::read(),
            value: RiscVOperandValue::Immediate(42),
        };
        assert_eq!(printer.format_operand(&imm_op), "42");

        // 内存操作数
        let mem_op = RiscVOperand {
            op_type: RiscVOperandType::Memory,
            access: Access::read(),
            value: RiscVOperandValue::Memory(RiscVMemoryOperand { base: 2, disp: 100 }),
        };
        assert_eq!(printer.format_operand(&mem_op), "100(sp)");
    }
}