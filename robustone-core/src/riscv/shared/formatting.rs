//! Formatting utilities for RISC-V instruction decoding.
//!
//! Provides centralized formatting functionality for instructions, operands,
//! and immediate values used across all RISC-V extensions.

use super::super::decoder::RiscVDecodedInstruction;
use super::super::types::*;

/// Trait for formatting decoded RISC-V instructions.
pub trait InstructionFormatter {
    /// Create a decoded instruction with the given parameters.
    fn create_decoded_instruction(
        &self,
        mnemonic: &str,
        operands: String,
        format: RiscVInstructionFormat,
        size: usize,
        operands_detail: Vec<RiscVOperand>,
    ) -> RiscVDecodedInstruction;

    /// Create a decoded instruction using the operand builder.
    fn create_instruction_from_parts(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: i64,
        format: RiscVInstructionFormat,
        rd_access: Access,
        rs1_access: Access,
        rs2_access: Access,
    ) -> RiscVDecodedInstruction;
}

/// Trait for immediate value formatting with different formats.
pub trait ImmediateFormatter {
    /// Format an immediate value for display.
    fn format_immediate(&self, value: i64) -> String;

    /// Format an immediate value as hex.
    fn format_immediate_hex(&self, value: i64) -> String;

    /// Format an immediate value as decimal.
    fn format_immediate_decimal(&self, value: i64) -> String;

    /// Format an immediate value with automatic format selection.
    fn format_immediate_auto(&self, value: i64) -> String;
}

/// Default implementation of instruction formatter.
pub struct DefaultInstructionFormatter;

impl InstructionFormatter for DefaultInstructionFormatter {
    fn create_decoded_instruction(
        &self,
        mnemonic: &str,
        operands: String,
        format: RiscVInstructionFormat,
        size: usize,
        operands_detail: Vec<RiscVOperand>,
    ) -> RiscVDecodedInstruction {
        RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands,
            format,
            size,
            operands_detail,
        }
    }

    fn create_instruction_from_parts(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: i64,
        format: RiscVInstructionFormat,
        rd_access: Access,
        rs1_access: Access,
        rs2_access: Access,
    ) -> RiscVDecodedInstruction {
        use super::operands::convenience;
        use super::registers::get_register_name;

        let (operands, operands_detail) = match format {
            RiscVInstructionFormat::R => {
                let ops = format!(
                    "{}, {}, {}",
                    get_register_name(rd),
                    get_register_name(rs1),
                    get_register_name(rs2)
                );
                let details = vec![
                    convenience::register(rd, rd_access),
                    convenience::register(rs1, rs1_access),
                    convenience::register(rs2, rs2_access),
                ];
                (ops, details)
            }
            RiscVInstructionFormat::I => {
                let ops = format!(
                    "{}, {}, {}",
                    get_register_name(rd),
                    get_register_name(rs1),
                    self.format_immediate_auto(imm)
                );
                let details = vec![
                    convenience::register(rd, rd_access),
                    convenience::register(rs1, rs1_access),
                    convenience::immediate(imm),
                ];
                (ops, details)
            }
            RiscVInstructionFormat::S => {
                let ops = format!(
                    "{}, {}({})",
                    get_register_name(rs2),
                    self.format_immediate_auto(imm),
                    get_register_name(rs1)
                );
                let details = vec![
                    convenience::register(rs2, rs2_access),
                    convenience::memory(rs1, imm),
                ];
                (ops, details)
            }
            RiscVInstructionFormat::B => {
                let offset_str = self.format_immediate_auto(imm);
                let ops = if rs2 == 0 && (mnemonic == "beqz" || mnemonic == "bnez") {
                    format!("{}, {}", get_register_name(rs1), offset_str)
                } else {
                    format!(
                        "{}, {}, {}",
                        get_register_name(rs1),
                        get_register_name(rs2),
                        offset_str
                    )
                };
                let details = vec![
                    convenience::register(rs1, rs1_access),
                    convenience::register(rs2, rs2_access),
                    convenience::immediate(imm),
                ];
                (ops, details)
            }
            RiscVInstructionFormat::U => {
                let imm_val = imm >> 12;
                let imm_str = if imm_val == 0 {
                    "0".to_string()
                } else {
                    format!("0x{:x}", imm_val)
                };
                let ops = format!("{}, {}", get_register_name(rd), imm_str);
                let details = vec![
                    convenience::register(rd, rd_access),
                    convenience::immediate(imm_val as i64),
                ];
                (ops, details)
            }
            RiscVInstructionFormat::J => {
                let offset_str = self.format_immediate_auto(imm);
                let ops = match (mnemonic, rd) {
                    ("j", _) => offset_str,
                    ("jal", 1) => offset_str,
                    _ => format!("{}, {}", get_register_name(rd), offset_str),
                };
                let details = vec![
                    convenience::register(rd, rd_access),
                    convenience::immediate(imm),
                ];
                (ops, details)
            }
            _ => {
                let ops = format!("unknown format");
                let details = vec![];
                (ops, details)
            }
        };

        self.create_decoded_instruction(mnemonic, operands, format, 4, operands_detail)
    }
}

impl ImmediateFormatter for DefaultInstructionFormatter {
    fn format_immediate(&self, value: i64) -> String {
        self.format_immediate_auto(value)
    }

    fn format_immediate_hex(&self, value: i64) -> String {
        if value < 0 {
            format!("-0x{:x}", -value)
        } else {
            format!("0x{:x}", value)
        }
    }

    fn format_immediate_decimal(&self, value: i64) -> String {
        value.to_string()
    }

    fn format_immediate_auto(&self, value: i64) -> String {
        if value == 0 {
            return "0".to_string();
        }

        let abs = value.abs();
        let use_hex = abs >= 16;

        if use_hex {
            self.format_immediate_hex(value)
        } else {
            self.format_immediate_decimal(value)
        }
    }
}

impl DefaultInstructionFormatter {
    /// Create a new default instruction formatter.
    pub const fn new() -> Self {
        Self
    }

    /// Get the global default instruction formatter instance.
    pub const fn instance() -> &'static Self {
        &DefaultInstructionFormatter
    }

    /// Create a simple decoded instruction with just mnemonic and operands.
    pub fn simple_instruction(mnemonic: &str, operands: &str) -> RiscVDecodedInstruction {
        Self::instance().create_decoded_instruction(
            mnemonic,
            operands.to_string(),
            RiscVInstructionFormat::I,
            4,
            vec![],
        )
    }

    /// Create an unknown instruction placeholder.
    pub fn unknown_instruction(value: u32) -> RiscVDecodedInstruction {
        Self::simple_instruction("unknown", &format!("0x{:08x}", value))
    }

    /// Create an unknown compressed instruction placeholder.
    pub fn unknown_compressed_instruction(value: u16) -> RiscVDecodedInstruction {
        Self::instance().create_decoded_instruction(
            "c.unknown",
            format!("0x{:04x}", value),
            RiscVInstructionFormat::CI,
            2,
            vec![],
        )
    }
}

/// Formatting utilities for different instruction formats.
pub struct InstructionFormatHelper;

impl InstructionFormatHelper {
    /// Format an R-type instruction (register-register).
    pub fn format_r_type(mnemonic: &str, rd: u8, rs1: u8, rs2: u8) -> String {
        use super::registers::get_register_name;
        format!(
            "{}, {}, {}",
            get_register_name(rd),
            get_register_name(rs1),
            get_register_name(rs2)
        )
    }

    /// Format an I-type instruction (register-immediate).
    pub fn format_i_type(mnemonic: &str, rd: u8, rs1: u8, imm: i64) -> String {
        use super::operands::convenience;
        use super::registers::get_register_name;
        format!(
            "{}, {}, {}",
            get_register_name(rd),
            get_register_name(rs1),
            convenience::format_immediate(imm)
        )
    }

    /// Format an S-type instruction (store).
    pub fn format_s_type(mnemonic: &str, rs2: u8, rs1: u8, imm: i64) -> String {
        use super::operands::convenience;
        use super::registers::get_register_name;
        format!(
            "{}, {}({})",
            get_register_name(rs2),
            convenience::format_immediate(imm),
            get_register_name(rs1)
        )
    }

    /// Format a B-type instruction (branch).
    pub fn format_b_type(mnemonic: &str, rs1: u8, rs2: u8, imm: i64) -> String {
        use super::operands::convenience;
        use super::registers::get_register_name;
        let offset_str = convenience::format_immediate(imm);
        if (mnemonic == "beqz" || mnemonic == "bnez") && rs2 == 0 {
            format!("{}, {}", get_register_name(rs1), offset_str)
        } else {
            format!(
                "{}, {}, {}",
                get_register_name(rs1),
                get_register_name(rs2),
                offset_str
            )
        }
    }

    /// Format a U-type instruction (upper immediate).
    pub fn format_u_type(mnemonic: &str, rd: u8, imm: i64) -> String {
        use super::registers::get_register_name;
        let imm_val = imm >> 12;
        let imm_str = if imm_val == 0 {
            "0".to_string()
        } else {
            format!("0x{:x}", imm_val)
        };
        format!("{}, {}", get_register_name(rd), imm_str)
    }

    /// Format a J-type instruction (jump).
    pub fn format_j_type(mnemonic: &str, rd: u8, imm: i64) -> String {
        use super::operands::convenience;
        use super::registers::get_register_name;
        let offset_str = convenience::format_immediate(imm);
        match (mnemonic, rd) {
            ("j", _) => offset_str,
            ("jal", 1) => offset_str,
            _ => format!("{}, {}", get_register_name(rd), offset_str),
        }
    }
}

/// CSR formatting utilities.
pub struct CsrFormatter;

impl CsrFormatter {
    /// Format a CSR address for display.
    pub fn format_csr(csr: i64) -> String {
        let csr_id = csr as u16;
        if let Some(name) = Self::csr_name_lookup(csr_id) {
            name.to_string()
        } else {
            format!("0x{:x}", csr)
        }
    }

    /// Look up CSR name by address.
    pub fn csr_name_lookup(csr: u16) -> Option<&'static str> {
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
}

/// Convenience functions for formatting.
pub mod convenience {
    use super::*;

    /// Format an immediate value using automatic format selection.
    pub fn format_immediate(value: i64) -> String {
        DefaultInstructionFormatter::instance().format_immediate_auto(value)
    }

    /// Format a CSR address.
    pub fn format_csr(csr: i64) -> String {
        CsrFormatter::format_csr(csr)
    }

    /// Format an R-type instruction.
    pub fn format_r_type(mnemonic: &str, rd: u8, rs1: u8, rs2: u8) -> String {
        InstructionFormatHelper::format_r_type(mnemonic, rd, rs1, rs2)
    }

    /// Format an I-type instruction.
    pub fn format_i_type(mnemonic: &str, rd: u8, rs1: u8, imm: i64) -> String {
        InstructionFormatHelper::format_i_type(mnemonic, rd, rs1, imm)
    }

    /// Format an S-type instruction.
    pub fn format_s_type(mnemonic: &str, rs2: u8, rs1: u8, imm: i64) -> String {
        InstructionFormatHelper::format_s_type(mnemonic, rs2, rs1, imm)
    }

    /// Format a B-type instruction.
    pub fn format_b_type(mnemonic: &str, rs1: u8, rs2: u8, imm: i64) -> String {
        InstructionFormatHelper::format_b_type(mnemonic, rs1, rs2, imm)
    }

    /// Format a U-type instruction.
    pub fn format_u_type(mnemonic: &str, rd: u8, imm: i64) -> String {
        InstructionFormatHelper::format_u_type(mnemonic, rd, imm)
    }

    /// Format a J-type instruction.
    pub fn format_j_type(mnemonic: &str, rd: u8, imm: i64) -> String {
        InstructionFormatHelper::format_j_type(mnemonic, rd, imm)
    }

    /// Create a simple decoded instruction.
    pub fn simple_instruction(mnemonic: &str, operands: &str) -> RiscVDecodedInstruction {
        DefaultInstructionFormatter::simple_instruction(mnemonic, operands)
    }

    /// Create an unknown instruction.
    pub fn unknown_instruction(value: u32) -> RiscVDecodedInstruction {
        DefaultInstructionFormatter::unknown_instruction(value)
    }

    /// Create an unknown compressed instruction.
    pub fn unknown_compressed_instruction(value: u16) -> RiscVDecodedInstruction {
        DefaultInstructionFormatter::unknown_compressed_instruction(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_formatter() {
        let formatter = DefaultInstructionFormatter::new();

        let instruction = formatter.create_decoded_instruction(
            "add",
            "x1, x2, x3".to_string(),
            RiscVInstructionFormat::R,
            4,
            vec![],
        );

        assert_eq!(instruction.mnemonic, "add");
        assert_eq!(instruction.operands, "x1, x2, x3");
        assert_eq!(instruction.format, RiscVInstructionFormat::R);
        assert_eq!(instruction.size, 4);
    }

    #[test]
    fn test_immediate_formatter() {
        let formatter = DefaultInstructionFormatter::new();

        assert_eq!(formatter.format_immediate(0), "0");
        assert_eq!(formatter.format_immediate(5), "5");
        assert_eq!(formatter.format_immediate(15), "15");
        assert_eq!(formatter.format_immediate(-5), "-5");
        assert_eq!(formatter.format_immediate(-15), "-15");
    }

    #[test]
    fn test_instruction_format_helper() {
        assert_eq!(
            InstructionFormatHelper::format_r_type("add", 1, 2, 3),
            "ra, sp, gp"
        );
        assert_eq!(
            InstructionFormatHelper::format_i_type("addi", 1, 2, 10),
            "ra, sp, 10"
        );
        assert_eq!(
            InstructionFormatHelper::format_s_type("sw", 3, 4, 16),
            "gp, 0x10(tp)"
        );
        assert_eq!(
            InstructionFormatHelper::format_u_type("lui", 5, 0x1000),
            "t0, 0x1"
        );
        assert_eq!(
            InstructionFormatHelper::format_j_type("jal", 1, 100),
            "0x64"
        );
    }

    #[test]
    fn test_csr_formatter() {
        assert_eq!(CsrFormatter::format_csr(0x001), "fflags");
        assert_eq!(CsrFormatter::format_csr(0x100), "sstatus");
        assert_eq!(CsrFormatter::format_csr(0x999), "0x999");
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(convenience::format_immediate(16), "0x10");
        assert_eq!(convenience::format_csr(0x001), "fflags");
        assert_eq!(convenience::format_r_type("add", 1, 2, 3), "ra, sp, gp");

        let unknown = convenience::unknown_instruction(0x12345678);
        assert_eq!(unknown.mnemonic, "unknown");
        assert_eq!(unknown.operands, "0x12345678");
    }
}
