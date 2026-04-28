//! Operand creation and formatting utilities for LoongArch instruction decoding.
//!
//! Provides centralized operand creation and formatting functionality used across
//! all LoongArch instruction families to eliminate code duplication.

use crate::types::{
    HEX_THRESHOLD, LoongArchMemoryOperand, LoongArchOperand, LoongArchOperandType,
    LoongArchOperandValue,
};

/// Trait for formatting operands for display.
pub trait OperandFormatter {
    /// Format an immediate value for display.
    fn format_immediate(&self, value: i64) -> String;

    /// Format a memory operand for display (offset(base)).
    fn format_memory_operand(&self, offset: i64, base_reg: &str) -> String;

    /// Format a control-flow offset (branch/jump target).
    fn format_control_offset(&self, value: i64) -> String;
}

/// Default implementation of operand formatter.
pub struct DefaultOperandFormatter;

impl DefaultOperandFormatter {
    pub const fn new() -> Self {
        Self
    }

    /// Format an immediate value for display (static convenience).
    pub fn format_imm(value: i64) -> String {
        Self::new().format_immediate(value)
    }
}

impl OperandFormatter for DefaultOperandFormatter {
    fn format_immediate(&self, value: i64) -> String {
        if value == 0 {
            return "0".to_string();
        }

        let abs = value.abs();
        let use_hex = abs > HEX_THRESHOLD;

        if use_hex {
            if value < 0 {
                format!("-0x{abs:x}")
            } else {
                format!("0x{abs:x}")
            }
        } else if value < 0 {
            format!("-{abs}")
        } else {
            format!("{abs}")
        }
    }

    fn format_memory_operand(&self, offset: i64, base_reg: &str) -> String {
        let offset_str = self.format_immediate(offset);
        format!("{offset_str}({base_reg})")
    }

    fn format_control_offset(&self, value: i64) -> String {
        // LoongArch branch offsets in Capstone are displayed as absolute targets.
        // The decoder pre-computes target = addr + sign_extend(offset) * 4,
        // so the value passed here is already the target address.
        if value >= 0 {
            self.format_immediate(value)
        } else {
            format!("0x{:x}", value as u64)
        }
    }
}

impl Default for DefaultOperandFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience module for creating operands.
pub mod convenience {
    use super::*;
    use crate::types::LoongArchRegister;

    /// Create a GPR operand.
    pub fn gpr(reg: u8) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::Register,
            value: LoongArchOperandValue::Register(LoongArchRegister::from_id(reg as u32)),
        }
    }

    /// Create an FPR operand.
    pub fn fpr(reg: u8) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::Register,
            value: LoongArchOperandValue::Register(LoongArchRegister::from_id(32 + reg as u32)),
        }
    }

    /// Create a vector (XR) operand.
    pub fn xr(reg: u8) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::Register,
            value: LoongArchOperandValue::Register(LoongArchRegister::from_id(64 + reg as u32)),
        }
    }

    /// Create an FCC operand.
    pub fn fcc(reg: u8) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::ConditionFlag,
            value: LoongArchOperandValue::ConditionFlag(reg),
        }
    }

    /// Create an immediate operand.
    pub fn imm(value: i64) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::Immediate,
            value: LoongArchOperandValue::Immediate(value),
        }
    }

    /// Create a memory operand (base GPR + displacement).
    pub fn mem(base: u8, disp: i64) -> LoongArchOperand {
        LoongArchOperand {
            op_type: LoongArchOperandType::Memory,
            value: LoongArchOperandValue::Memory(LoongArchMemoryOperand { base, disp }),
        }
    }

    /// Format an immediate value.
    pub fn format_immediate(value: i64) -> String {
        super::DefaultOperandFormatter::format_imm(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_formatting() {
        let fmt = DefaultOperandFormatter::new();
        assert_eq!(fmt.format_immediate(0), "0");
        assert_eq!(fmt.format_immediate(9), "9");
        assert_eq!(fmt.format_immediate(10), "0xa");
        assert_eq!(fmt.format_immediate(-9), "-9");
        assert_eq!(fmt.format_immediate(-10), "-0xa");
        assert_eq!(fmt.format_immediate(255), "0xff");
        assert_eq!(fmt.format_immediate(-255), "-0xff");
    }

    #[test]
    fn test_memory_operand_formatting() {
        let fmt = DefaultOperandFormatter::new();
        assert_eq!(fmt.format_memory_operand(0, "$a0"), "0($a0)");
        assert_eq!(fmt.format_memory_operand(21, "$a4"), "0x15($a4)");
        assert_eq!(fmt.format_memory_operand(-4, "$sp"), "-4($sp)");
    }

    #[test]
    fn test_control_offset_formatting() {
        let fmt = DefaultOperandFormatter::new();
        assert_eq!(fmt.format_control_offset(0xb0), "0xb0");
        assert_eq!(fmt.format_control_offset(4), "4");
    }
}
