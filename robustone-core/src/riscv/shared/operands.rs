//! Operand creation and formatting utilities for RISC-V instruction decoding.
//!
//! Provides centralized operand creation and formatting functionality used across
//! all RISC-V extensions to eliminate code duplication.

use super::super::types::*;

/// Trait for creating RISC-V operands in extensions.
pub trait OperandFactory {
    /// Create a register operand with the specified access pattern.
    fn make_register_operand(&self, reg: u8, access: Access) -> RiscVOperand;

    /// Create an immediate operand.
    fn make_immediate_operand(&self, imm: i64) -> RiscVOperand;

    /// Create a memory operand with base register and displacement.
    fn make_memory_operand(&self, base: u8, disp: i64) -> RiscVOperand;

    /// Create a memory operand with explicit base and displacement.
    fn make_explicit_memory_operand(&self, memory: RiscVMemoryOperand) -> RiscVOperand;
}

/// Trait for formatting operands for display.
pub trait OperandFormatter {
    /// Format an immediate value for display.
    fn format_immediate(&self, value: i64) -> String;

    /// Format a CSR address for display.
    fn format_csr(&self, csr: i64) -> String;

    /// Format a memory operand for display (offset(base)).
    fn format_memory_operand(&self, offset: i64, base_reg: &str) -> String;
}

/// Default implementation of operand factory.
pub struct DefaultOperandFactory;

impl OperandFactory for DefaultOperandFactory {
    fn make_register_operand(&self, reg: u8, access: Access) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Register,
            access,
            value: RiscVOperandValue::Register(reg as u32),
        }
    }

    fn make_immediate_operand(&self, imm: i64) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Immediate,
            access: Access::read(),
            value: RiscVOperandValue::Immediate(imm),
        }
    }

    fn make_memory_operand(&self, base: u8, disp: i64) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Memory,
            access: Access::read(),
            value: RiscVOperandValue::Memory(RiscVMemoryOperand {
                base: base as u32,
                disp,
            }),
        }
    }

    fn make_explicit_memory_operand(&self, memory: RiscVMemoryOperand) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Memory,
            access: Access::read(),
            value: RiscVOperandValue::Memory(memory),
        }
    }
}

impl OperandFormatter for DefaultOperandFactory {
    fn format_immediate(&self, value: i64) -> String {
        if value == 0 {
            return "0".to_string();
        }

        let abs = value.abs();
        let use_hex = abs >= 16;

        if use_hex {
            if value < 0 {
                format!("-0x{abs:x}")
            } else {
                format!("0x{abs:x}")
            }
        } else if value < 0 {
            format!("-{abs}")
        } else {
            format!("{value}")
        }
    }

    fn format_csr(&self, csr: i64) -> String {
        let csr_id = csr as u16;
        if let Some(name) = csr_name_lookup(csr_id) {
            name.to_string()
        } else {
            format!("0x{csr:x}")
        }
    }

    fn format_memory_operand(&self, offset: i64, base_reg: &str) -> String {
        let offset_str = self.format_immediate(offset);
        format!("{offset_str}({base_reg})")
    }
}

impl DefaultOperandFactory {
    /// Create a new default operand factory.
    pub const fn new() -> Self {
        Self
    }

    /// Get the global default operand factory instance.
    pub const fn instance() -> &'static Self {
        &DefaultOperandFactory
    }

    /// Create a register operand (convenience method).
    pub fn register(reg: u8, access: Access) -> RiscVOperand {
        Self::instance().make_register_operand(reg, access)
    }

    /// Create an immediate operand (convenience method).
    pub fn immediate(imm: i64) -> RiscVOperand {
        Self::instance().make_immediate_operand(imm)
    }

    /// Create a memory operand (convenience method).
    pub fn memory(base: u8, disp: i64) -> RiscVOperand {
        Self::instance().make_memory_operand(base, disp)
    }
}

impl Default for DefaultOperandFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Combined operand factory and formatter for convenience.
pub struct OperandBuilder {
    factory: DefaultOperandFactory,
}

impl OperandBuilder {
    /// Create a new operand builder.
    pub const fn new() -> Self {
        Self {
            factory: DefaultOperandFactory,
        }
    }

    /// Get the global operand builder instance.
    pub fn instance() -> Self {
        Self::new()
    }

    /// Format instruction operands for common instruction formats.
    pub fn format_r_type(&self, _mnemonic: &str, rd: u8, rs1: u8, rs2: u8) -> String {
        format!(
            "{}, {}, {}",
            super::registers::get_register_name(rd),
            super::registers::get_register_name(rs1),
            super::registers::get_register_name(rs2)
        )
    }

    pub fn format_i_type(&self, _mnemonic: &str, rd: u8, rs1: u8, imm: i64) -> String {
        format!(
            "{}, {}, {}",
            super::registers::get_register_name(rd),
            super::registers::get_register_name(rs1),
            self.factory.format_immediate(imm)
        )
    }

    pub fn format_s_type(&self, _mnemonic: &str, rs2: u8, rs1: u8, imm: i64) -> String {
        format!(
            "{}, {}({})",
            super::registers::get_register_name(rs2),
            self.factory.format_immediate(imm),
            super::registers::get_register_name(rs1)
        )
    }

    pub fn format_b_type(&self, mnemonic: &str, rs1: u8, rs2: u8, imm: i64) -> String {
        let offset_str = self.factory.format_immediate(imm);
        if (mnemonic == "beqz" || mnemonic == "bnez") && rs2 == 0 {
            format!(
                "{}, {}",
                super::registers::get_register_name(rs1),
                offset_str
            )
        } else {
            format!(
                "{}, {}, {}",
                super::registers::get_register_name(rs1),
                super::registers::get_register_name(rs2),
                offset_str
            )
        }
    }

    pub fn format_u_type(&self, _mnemonic: &str, rd: u8, imm: i64) -> String {
        let imm_val = imm >> 12;
        let imm_str = if imm_val == 0 {
            "0".to_string()
        } else {
            format!("0x{imm_val:x}")
        };
        format!("{}, {}", super::registers::get_register_name(rd), imm_str)
    }

    pub fn format_j_type(&self, mnemonic: &str, rd: u8, imm: i64) -> String {
        let imm_str = self.factory.format_immediate(imm);
        match (mnemonic, rd) {
            ("j", _) => imm_str,
            ("jal", 1) => imm_str,
            _ => format!("{}, {}", super::registers::get_register_name(rd), imm_str),
        }
    }

    pub fn format_load_type(
        &self,
        _mnemonic: &str,
        rd: u8,
        rs1: u8,
        imm: i64,
        is_fp: bool,
    ) -> String {
        let rd_name = if is_fp {
            super::registers::get_fp_register_name(rd)
        } else {
            super::registers::get_register_name(rd)
        };
        format!(
            "{}, {}({})",
            rd_name,
            self.factory.format_immediate(imm),
            super::registers::get_register_name(rs1)
        )
    }

    pub fn format_store_type(
        &self,
        _mnemonic: &str,
        rs2: u8,
        rs1: u8,
        imm: i64,
        is_fp: bool,
    ) -> String {
        let rs2_name = if is_fp {
            super::registers::get_fp_register_name(rs2)
        } else {
            super::registers::get_register_name(rs2)
        };
        format!(
            "{}, {}({})",
            rs2_name,
            self.factory.format_immediate(imm),
            super::registers::get_register_name(rs1)
        )
    }
}

impl Default for OperandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// CSR name lookup function.
fn csr_name_lookup(csr: u16) -> Option<&'static str> {
    match csr {
        0x000 => Some("ustatus"),
        0x001 => Some("fflags"),
        0x002 => Some("frm"),
        0x003 => Some("fcsr"),
        0x100 => Some("sstatus"),
        0x102 => Some("sedeleg"),
        0x103 => Some("sideleg"),
        0x104 => Some("sie"),
        0x105 => Some("stvec"),
        0x106 => Some("scounteren"),
        0x140 => Some("sscratch"),
        0x141 => Some("sepc"),
        0x142 => Some("scause"),
        0x143 => Some("stval"),
        0x144 => Some("sip"),
        0x180 => Some("satp"),
        0x300 => Some("mstatus"),
        0x301 => Some("misa"),
        0x302 => Some("medeleg"),
        0x303 => Some("mideleg"),
        0x304 => Some("mie"),
        0x305 => Some("mtvec"),
        0x306 => Some("mcounteren"),
        0x320 => Some("mcountinhibit"),
        0x321 => Some("mhpmevent3"),
        0x340 => Some("mscratch"),
        0x341 => Some("mepc"),
        0x342 => Some("mcause"),
        0x343 => Some("mtval"),
        0x344 => Some("mip"),
        0x34A => Some("mtinst"),
        0x34B => Some("mtval2"),
        0x7A0 => Some("tselect"),
        0x7A1 => Some("tdata1"),
        0x7A2 => Some("tdata2"),
        0x7A3 => Some("tdata3"),
        0x7B0 => Some("dcsr"),
        0x7B1 => Some("dpc"),
        0x7B2 => Some("dscratch0"),
        0x7B3 => Some("dscratch1"),
        0xC00 => Some("cycle"),
        0xC01 => Some("time"),
        0xC02 => Some("instret"),
        0xC80 => Some("cycleh"),
        0xC81 => Some("timeh"),
        0xC82 => Some("instreth"),
        _ => None,
    }
}

/// Convenience functions for operand creation.
pub mod convenience {
    use super::*;

    /// Create a register operand.
    pub fn register(reg: u8, access: Access) -> RiscVOperand {
        DefaultOperandFactory::register(reg, access)
    }

    /// Create an immediate operand.
    pub fn immediate(imm: i64) -> RiscVOperand {
        DefaultOperandFactory::immediate(imm)
    }

    /// Create a memory operand.
    pub fn memory(base: u8, disp: i64) -> RiscVOperand {
        DefaultOperandFactory::memory(base, disp)
    }

    /// Format an immediate value.
    pub fn format_immediate(value: i64) -> String {
        DefaultOperandFactory::instance().format_immediate(value)
    }

    /// Format a CSR address.
    pub fn format_csr(csr: i64) -> String {
        DefaultOperandFactory::instance().format_csr(csr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_factory() {
        let factory = DefaultOperandFactory::new();

        let reg_op = factory.make_register_operand(1, Access::read());
        assert!(matches!(reg_op.op_type, RiscVOperandType::Register));
        assert_eq!(reg_op.access, Access::read());

        let imm_op = factory.make_immediate_operand(42);
        assert!(matches!(imm_op.op_type, RiscVOperandType::Immediate));
        assert_eq!(imm_op.access, Access::read());

        let mem_op = factory.make_memory_operand(2, 8);
        assert!(matches!(mem_op.op_type, RiscVOperandType::Memory));
        assert_eq!(mem_op.access, Access::read());
    }

    #[test]
    fn test_operand_formatter() {
        let formatter = DefaultOperandFactory::new();

        assert_eq!(formatter.format_immediate(0), "0");
        assert_eq!(formatter.format_immediate(10), "10");
        assert_eq!(formatter.format_immediate(16), "0x10");
        assert_eq!(formatter.format_immediate(-5), "-5");
        assert_eq!(formatter.format_immediate(-16), "-0x10");

        assert_eq!(formatter.format_csr(0x001), "fflags");
        assert_eq!(formatter.format_csr(0x100), "sstatus");
        assert_eq!(formatter.format_csr(0x999), "0x999");

        assert_eq!(formatter.format_memory_operand(8, "sp"), "8(sp)");
        assert_eq!(formatter.format_memory_operand(-4, "fp"), "-4(fp)");
    }

    #[test]
    fn test_operand_builder() {
        let builder = OperandBuilder::new();

        assert_eq!(builder.format_r_type("add", 1, 2, 3), "ra, sp, gp");
        assert_eq!(builder.format_i_type("addi", 1, 2, 10), "ra, sp, 10");
        assert_eq!(builder.format_s_type("sw", 3, 4, 16), "gp, 0x10(tp)");
        assert_eq!(builder.format_u_type("lui", 5, 0x1000), "t0, 0x1");
        assert_eq!(builder.format_j_type("jal", 1, 100), "0x64");
    }

    #[test]
    fn test_convenience_functions() {
        let reg_op = convenience::register(1, Access::write());
        assert!(matches!(reg_op.op_type, RiscVOperandType::Register));

        let imm_op = convenience::immediate(42);
        assert!(matches!(imm_op.op_type, RiscVOperandType::Immediate));

        let mem_op = convenience::memory(2, 8);
        assert!(matches!(mem_op.op_type, RiscVOperandType::Memory));

        assert_eq!(convenience::format_immediate(16), "0x10");
        assert_eq!(convenience::format_csr(0x001), "fflags");
    }
}
