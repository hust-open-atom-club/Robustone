//! Formatting utilities for LoongArch instruction decoding.
//!
//! Provides centralized formatting functionality for instructions, operands,
//! and immediate values used across all LoongArch families.

use crate::shared::operands::DefaultOperandFormatter;
use crate::shared::registers::RegisterManager;
use crate::types::LoongArchInstructionFormat;

/// Format operands according to the instruction format.
pub fn format_operands(
    format: LoongArchInstructionFormat,
    operands: &[crate::types::LoongArchOperand],
) -> String {
    use crate::types::LoongArchOperandType;
    use crate::types::LoongArchOperandValue;

    let formatter = DefaultOperandFormatter::new();
    let mgr = RegisterManager::instance();

    match format {
        LoongArchInstructionFormat::ThreeReg
        | LoongArchInstructionFormat::VectorThreeReg
        | LoongArchInstructionFormat::FloatThreeReg => {
            if operands.len() >= 3 {
                format!(
                    "{}, {}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr),
                    format_operand(&operands[2], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::TwoRegImm
        | LoongArchInstructionFormat::VectorTwoRegImm
        | LoongArchInstructionFormat::FloatTwoRegImm => {
            if operands.len() >= 3 {
                format!(
                    "{}, {}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr),
                    format_operand(&operands[2], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::TwoRegMem => {
            if operands.len() >= 3 {
                // Memory syntax for LoongArch: Rd, Rj, offset
                format!(
                    "{}, {}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr),
                    format_operand(&operands[2], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::Branch => {
            if operands.len() >= 3 {
                // Branch syntax: Rj, Rd, offset
                format!(
                    "{}, {}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr),
                    format_operand(&operands[2], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::Jump => {
            if operands.len() >= 2 {
                format!(
                    "{}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::TwoReg
        | LoongArchInstructionFormat::FloatTwoReg
        | LoongArchInstructionFormat::ScrReg => {
            if operands.len() >= 2 {
                format!(
                    "{}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::OneRegImm => {
            if operands.len() >= 2 {
                format!(
                    "{}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::OneReg | LoongArchInstructionFormat::FloatOneReg => {
            if let Some(op) = operands.first() {
                format_operand(op, &formatter, mgr)
            } else {
                String::new()
            }
        }
        LoongArchInstructionFormat::RegConditionFlag => {
            if operands.len() >= 2 {
                format!(
                    "{}, {}",
                    format_operand(&operands[0], &formatter, mgr),
                    format_operand(&operands[1], &formatter, mgr)
                )
            } else {
                default_format(operands, &formatter, mgr)
            }
        }
        LoongArchInstructionFormat::None | LoongArchInstructionFormat::Barrier => {
            String::new()
        }
    }
}

fn format_operand(
    operand: &crate::types::LoongArchOperand,
    formatter: &DefaultOperandFormatter,
    mgr: &RegisterManager,
) -> String {
    use crate::types::LoongArchOperandType;
    use crate::types::LoongArchOperandValue;

    match (&operand.op_type, &operand.value) {
        (LoongArchOperandType::Register, LoongArchOperandValue::Register(reg)) => {
            mgr.format_register(*reg).to_string()
        }
        (LoongArchOperandType::ConditionFlag, LoongArchOperandValue::ConditionFlag(id)) => {
            mgr.fcc_name(*id).to_string()
        }
        (LoongArchOperandType::Scr, LoongArchOperandValue::Scr(id)) => {
            mgr.scr_name(*id).to_string()
        }
        (LoongArchOperandType::Immediate, LoongArchOperandValue::Immediate(val)) => {
            formatter.format_immediate(*val)
        }
        (
            LoongArchOperandType::Memory,
            LoongArchOperandValue::Memory(crate::types::LoongArchMemoryOperand { base, disp }),
        ) => {
            formatter.format_memory_operand(*disp, mgr.gpr_name(*base))
        }
        _ => "invalid".to_string(),
    }
}

fn default_format(
    operands: &[crate::types::LoongArchOperand],
    formatter: &DefaultOperandFormatter,
    mgr: &RegisterManager,
) -> String {
    operands
        .iter()
        .map(|op| format_operand(op, formatter, mgr))
        .collect::<Vec<_>>()
        .join(", ")
}
