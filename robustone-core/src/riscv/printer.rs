//! RISC-V instruction formatting helpers.
//!
//! Inspired by Capstone's printer to maintain compatible output formatting.

use super::types::*;
use crate::Instruction;

/// Pretty-printer for RISC-V instructions.
pub struct RiscVPrinter {
    /// Whether register aliases should be printed instead of canonical names.
    alias_regs: bool,
    /// Whether immediates should be rendered as unsigned values when possible.
    unsigned_immediate: bool,
}

impl RiscVPrinter {
    /// Creates a printer with default formatting behaviour.
    pub fn new() -> Self {
        Self {
            alias_regs: false,
            unsigned_immediate: false,
        }
    }

    /// Enables or disables register alias printing.
    pub fn with_alias_regs(mut self, alias_regs: bool) -> Self {
        self.alias_regs = alias_regs;
        self
    }

    /// Enables or disables unsigned immediate formatting.
    pub fn with_unsigned_immediate(mut self, unsigned_immediate: bool) -> Self {
        self.unsigned_immediate = unsigned_immediate;
        self
    }

    /// Formats an immediate according to the active configuration.
    fn format_immediate(&self, imm: i64) -> String {
        if imm > 0xFF {
            format!("0x{imm:x}")
        } else if imm >= 0 {
            format!("{imm}")
        } else if imm < -0xFF {
            format!("-0x{imm:x}")
        } else {
            format!("{imm}")
        }
    }

    /// Formats a register operand.
    fn format_register(&self, reg_id: u32) -> String {
        let reg = RiscVRegister::from_id(reg_id);
        if self.alias_regs {
            reg.name().to_string()
        } else {
            // Use the x0-x31 naming scheme when aliases are disabled.
            if reg_id <= 31 {
                format!("x{reg_id}")
            } else {
                reg.name().to_string()
            }
        }
    }

    /// Formats a memory operand using `offset(base)` syntax.
    fn format_memory_operand(&self, base: u32, disp: i64) -> String {
        if disp == 0 {
            format!("({})", self.format_register(base))
        } else if disp > 0 {
            format!(
                "{}({})",
                self.format_immediate(disp),
                self.format_register(base)
            )
        } else {
            format!(
                "-{}({})",
                self.format_immediate(-disp),
                self.format_register(base)
            )
        }
    }

    /// Formats a single operand into its textual form.
    pub fn format_operand(&self, operand: &RiscVOperand) -> String {
        match &operand.value {
            RiscVOperandValue::Register(reg_id) => self.format_register(*reg_id),
            RiscVOperandValue::Immediate(imm) => self.format_immediate(*imm),
            RiscVOperandValue::Memory(mem) => self.format_memory_operand(mem.base, mem.disp),
        }
    }

    /// Formats a sequence of operands into a comma-separated string.
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

    /// Renders the instruction mnemonic and operand list.
    pub fn print_basic(&self, instruction: &Instruction) -> String {
        format!("{} {}", instruction.mnemonic, instruction.operands)
    }

    /// Renders the detailed instruction representation including metadata.
    pub fn print_detailed(&self, instruction: &Instruction) -> String {
        let mut result = Vec::new();

        // Basic summary line.
        result.push(format!(
            "0x{:016x}: {} {}",
            instruction.address,
            hex::encode(&instruction.bytes),
            self.print_basic(instruction)
        ));

        // Emit detailed sections when available.
        if let Some(detail) = &instruction.detail {
            // Group identifiers (if present).
            if !detail.groups.is_empty() {
                result.push(format!("\tGroups: {}", detail.groups.join(", ")));
            }

            // Operand breakdown.
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
                            format!(
                                "{} (base={}, disp={})",
                                self.format_memory_operand(mem.base, mem.disp),
                                mem.base,
                                mem.disp
                            )
                        }
                    };

                    result.push(format!(
                        "\toperands[{i}].type: {operand_type_str} = {value_str}",
                    ));
                    result.push(format!("\toperands[{i}].access: {access_str}"));
                }
            }

            // Register access lists.
            if !detail.regs_read.is_empty() {
                let regs_read: Vec<String> = detail
                    .regs_read
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters read: {}", regs_read.join(", ")));
            }

            if !detail.regs_write.is_empty() {
                let regs_write: Vec<String> = detail
                    .regs_write
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters modified: {}", regs_write.join(", ")));
            }
        }

        result.join("\n")
    }

    /// Formats instruction bytes as a hex string and pads to the requested width.
    pub fn print_hex_bytes(&self, instruction: &Instruction, align_width: usize) -> String {
        let hex_str = hex::encode(&instruction.bytes);
        let padding = if hex_str.len() < align_width {
            " ".repeat(align_width - hex_str.len())
        } else {
            String::new()
        };
        format!("{hex_str}{padding}")
    }
}

impl Default for RiscVPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common RISC-V printing scenarios.
pub mod format {
    use super::*;

    /// Returns the basic printable form for a single instruction.
    pub fn basic_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_basic(instruction)
    }

    /// Returns the fully detailed printable form for a single instruction.
    pub fn detailed_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_detailed(instruction)
    }

    /// Renders a list of instructions using either basic or detailed mode.
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

    /// Formats a list of operands using default printer settings.
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

        // Positive values
        assert_eq!(printer.format_immediate(10), "10");
        assert_eq!(printer.format_immediate(0x1000), "0x1000");

        // Negative values
        assert_eq!(printer.format_immediate(-10), "-10");
        assert_eq!(printer.format_immediate(-0x1000), "-0x1000");

        // Zero
        assert_eq!(printer.format_immediate(0), "0");
    }

    #[test]
    fn test_format_register() {
        let printer = RiscVPrinter::new();

        // Canonical register formatting
        assert_eq!(printer.format_register(0), "x0");
        assert_eq!(printer.format_register(1), "x1");
        assert_eq!(printer.format_register(10), "x10");

        // Alias-based formatting
        let printer_with_alias = printer.with_alias_regs(true);
        assert_eq!(printer_with_alias.format_register(0), "zero");
        assert_eq!(printer_with_alias.format_register(1), "ra");
        assert_eq!(printer_with_alias.format_register(10), "a0");
    }

    #[test]
    fn test_format_memory_operand() {
        let printer = RiscVPrinter::new().with_alias_regs(true);

        // Base register only
        assert_eq!(printer.format_memory_operand(2, 0), "(sp)");

        // Positive offset
        assert_eq!(printer.format_memory_operand(10, 100), "100(a0)");

        // Negative offset
        assert_eq!(printer.format_memory_operand(10, -100), "-100(a0)");
    }

    #[test]
    fn test_format_operand() {
        let printer = RiscVPrinter::new().with_alias_regs(true);

        // Register operand
        let reg_op = RiscVOperand {
            op_type: RiscVOperandType::Register,
            access: Access::read(),
            value: RiscVOperandValue::Register(10),
        };
        assert_eq!(printer.format_operand(&reg_op), "a0");

        // Immediate operand
        let imm_op = RiscVOperand {
            op_type: RiscVOperandType::Immediate,
            access: Access::read(),
            value: RiscVOperandValue::Immediate(42),
        };
        assert_eq!(printer.format_operand(&imm_op), "42");

        // Memory operand
        let mem_op = RiscVOperand {
            op_type: RiscVOperandType::Memory,
            access: Access::read(),
            value: RiscVOperandValue::Memory(RiscVMemoryOperand { base: 2, disp: 100 }),
        };
        assert_eq!(printer.format_operand(&mem_op), "100(sp)");
    }
}
