//! Zicntr Extension
//! 
//! This module implements the RISC-V Zicntr extension, which provides access to machine counters:
//! - cycle: Processor cycle counter
//! - time: Real-time counter
//! - instret: Instructions-retired counter
//! 
//! Note: Zicntr requires Zicsr to be implemented as well.

use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    InstructionFormatter, OperandFactory, RegisterNameProvider,
    formatting::DefaultInstructionFormatter, operands::DefaultOperandFactory,
    registers::RegisterManager,
};
use super::super::types::*;
use super::InstructionExtension;
use crate::error::DisasmError;
use crate::riscv::extensions::extension_masks;

/// Zicntr Extension
pub struct ZicntrExtension {
    operand_factory: DefaultOperandFactory,
    formatter: DefaultInstructionFormatter,
    register_manager: RegisterManager,
}

impl ZicntrExtension {
    /// Create a new Zicntr extension instance.
    pub fn new() -> Self {
        Self {
            operand_factory: DefaultOperandFactory::new(),
            formatter: DefaultInstructionFormatter::new(),
            register_manager: RegisterManager::new(),
        }
    }

    // Counter CSR registers
    pub const CSR_CYCLE: u32 = 0xC00;
    pub const CSR_TIME: u32 = 0xC01;
    pub const CSR_INSTRET: u32 = 0xC02;
    pub const CSR_CYCLEH: u32 = 0xC80;
    pub const CSR_TIMEH: u32 = 0xC81;
    pub const CSR_INSTRETH: u32 = 0xC82;

    // Opcode constants
    const OPCODE_SYSTEM: u32 = 0b111_0011;

    // funct3 constants for system instructions
    const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
    const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
    const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
    const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
    const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
    const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

    // Check if the CSR address corresponds to a counter register
    fn is_counter_csr(&self, csr: u32) -> bool {
        matches!(csr, Self::CSR_CYCLE
            | Self::CSR_TIME
            | Self::CSR_INSTRET
            | Self::CSR_CYCLEH
            | Self::CSR_TIMEH
            | Self::CSR_INSTRETH)
    }

    // Get the name of the counter register based on its address
    fn get_counter_name(&self, csr: u32) -> String {
        match csr {
            Self::CSR_CYCLE => "cycle".to_string(),
            Self::CSR_TIME => "time".to_string(),
            Self::CSR_INSTRET => "instret".to_string(),
            Self::CSR_CYCLEH => "cycleh".to_string(),
            Self::CSR_TIMEH => "timeh".to_string(),
            Self::CSR_INSTRETH => "instreth".to_string(),
            _ => format!("0x{csr:03x}"),
        }
    }

    // Decode counter CSR instructions with register operand
    fn decode_counter_instruction(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        csr: u32,
        _xlen: Xlen,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_name = self.get_counter_name(csr);

        // Handle pseudo-instructions: csrr, csrc, csrw
        let (final_mnemonic, operands, operands_detail) = if rs1 == 0 {
            let pseudo_mnemonic = match mnemonic {
                "csrrs" => "csrr",
                "csrrc" => "csrc",
                "csrrw" => "csrw",
                _ => mnemonic,
            };
            let ops = format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                csr_name
            );
            let ops_detail = vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr as i64),
            ];
            (pseudo_mnemonic, ops, ops_detail)
        } else {
            let ops = format!(
                "{}, {}, {}",
                self.register_manager.int_register_name(rd),
                csr_name,
                self.register_manager.int_register_name(rs1)
            );
            let ops_detail = vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr as i64),
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
            ];
            (mnemonic, ops, ops_detail)
        };

        Ok(self.formatter.create_decoded_instruction(
            final_mnemonic,
            operands,
            RiscVInstructionFormat::I,
            4,
            operands_detail,
        ))
    }

    // Decode counter CSR instructions with immediate operand
    fn decode_counter_instruction_imm(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
        csr: u32,
        _xlen: Xlen,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_name = self.get_counter_name(csr);

        // Handle pseudo-instructions for immediate versions
        let (final_mnemonic, operands, operands_detail) = if imm == 0 {
            let pseudo_mnemonic = match mnemonic {
                "csrrsi" => "csrri",
                "csrrci" => "csrci",
                "csrrwi" => "csrwi",
                _ => mnemonic,
            };
            let ops = format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                csr_name
            );
            let ops_detail = vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr as i64),
            ];
            (pseudo_mnemonic, ops, ops_detail)
        } else {
            let ops = format!(
                "{}, {}, {}",
                self.register_manager.int_register_name(rd),
                csr_name,
                imm
            );
            let ops_detail = vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr as i64),
                self.operand_factory.make_immediate_operand(imm),
            ];
            (mnemonic, ops, ops_detail)
        };

        Ok(self.formatter.create_decoded_instruction(
            final_mnemonic,
            operands,
            RiscVInstructionFormat::I,
            4,
            operands_detail,
        ))
    }
}

impl InstructionExtension for ZicntrExtension {
    fn try_decode_standard(
        &self,
        opcode: u32,
        funct3: u8,
        _funct7: u8,
        rd: u8,
        rs1: u8,
        _rs2: u8,
        funct12: u32,
        _imm_i: i64,
        _imm_s: i64,
        _imm_b: i64,
        _imm_u: i64,
        _imm_j: i64,
        xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        // Check if this is a CSR instruction and if the CSR is a counter register
        if opcode == Self::OPCODE_SYSTEM && self.is_counter_csr(funct12) {
            match funct3 {
                Self::FUNCT3_SYSTEM_CSRRW => {
                    Some(self.decode_counter_instruction("csrrw", rd, rs1, funct12, xlen))
                }
                Self::FUNCT3_SYSTEM_CSRRS => {
                    Some(self.decode_counter_instruction("csrrs", rd, rs1, funct12, xlen))
                }
                Self::FUNCT3_SYSTEM_CSRRC => {
                    Some(self.decode_counter_instruction("csrrc", rd, rs1, funct12, xlen))
                }
                Self::FUNCT3_SYSTEM_CSRRWI => Some(
                    self.decode_counter_instruction_imm("csrrwi", rd, rs1 as i64, funct12, xlen),
                ),
                Self::FUNCT3_SYSTEM_CSRRSI => Some(
                    self.decode_counter_instruction_imm("csrrsi", rd, rs1 as i64, funct12, xlen),
                ),
                Self::FUNCT3_SYSTEM_CSRRCI => Some(
                    self.decode_counter_instruction_imm("csrrci", rd, rs1 as i64, funct12, xlen),
                ),
                _ => None,
            }
        } else {
            None
        }
    }

    fn try_decode_compressed(
        &self,
        _instruction: u16,
        _opcode: u8,
        _funct3: u8,
        _xlen: Xlen,
        _rd_full: u8,
        _rs1_full: u8,
        _rs2_full: u8,
        _rdp: u8,
        _rs1p: u8,
        _rs2p: u8,
        _nzuimm_ciw: u16,
        _uimm_cl: u16,
        _uimm_cs: u16,
        _imm_ci: i64,
        _imm_cj: i64,
        _imm_cb: i64,
        _uimm_css: u16,
        _uimm_clsp: u16,
        _uimm_fldsp: u16,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        // Zicntr extension doesn't handle compressed instructions
        None
    }

    fn name(&self) -> &'static str {
        "Zicntr"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        extensions & extension_masks::ZICNTR != 0
    }
}

impl Default for ZicntrExtension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zicntr_extension_creation() {
        let extension = ZicntrExtension::new();
        assert_eq!(extension.name(), "Zicntr");
        assert!(extension.is_enabled(extension_masks::ZICNTR));
        assert!(!extension.is_enabled(0));
    }

    #[test]
    fn test_zicntr_instruction_decoding() {
        let extension = ZicntrExtension::new();

        // Test CSRRS x1, cycle, x0
        let result = extension.try_decode_standard(
            0b1110011, // opcode
            0b010,     // funct3
            0,         // funct7
            1,         // rd
            0,         // rs1
            0,         // rs2
            0xC00,     // funct12
            0,         // imm_i
            0,         // imm_s
            0,         // imm_b
            0,         // imm_u
            0,         // imm_j
            Xlen::X32,
        );

        assert!(result.is_some());
        let instruction = result.unwrap().unwrap();
        assert_eq!(instruction.mnemonic, "csrr");
    }

    #[test]
    fn test_zicntr_counter_registers() {
        let extension = ZicntrExtension::new();
        assert!(extension.is_counter_csr(0xC00)); // cycle
        assert!(extension.is_counter_csr(0xC01)); // time
        assert!(extension.is_counter_csr(0xC02)); // instret
        assert!(extension.is_counter_csr(0xC80)); // cycleh
        assert!(extension.is_counter_csr(0xC81)); // timeh
        assert!(extension.is_counter_csr(0xC82)); // instreth
        assert!(!extension.is_counter_csr(0x000)); // not a counter register
    }

    #[test]
    fn test_zicntr_counter_names() {
        let extension = ZicntrExtension::new();
        assert_eq!(extension.get_counter_name(0xC00), "cycle");
        assert_eq!(extension.get_counter_name(0xC01), "time");
        assert_eq!(extension.get_counter_name(0xC02), "instret");
        assert_eq!(extension.get_counter_name(0xC80), "cycleh");
        assert_eq!(extension.get_counter_name(0xC81), "timeh");
        assert_eq!(extension.get_counter_name(0xC82), "instreth");
    }
}