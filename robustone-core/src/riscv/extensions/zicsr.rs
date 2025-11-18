//! Zicsr Extension
//!
//! This module implements the RISC-V Zicsr extension, which provides access to Control and Status Registers (CSRs).

use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    InstructionFormatter, OperandFactory, RegisterNameProvider,
    formatting::DefaultInstructionFormatter,
    operands::{DefaultOperandFactory, OperandBuilder},
    registers::RegisterManager,
};
use super::super::types::*;
use super::InstructionExtension;
use crate::error::DisasmError;
use crate::riscv::extensions::extension_masks;

/// Zicsr Extension
pub struct ZicsrExtension {
    operand_factory: DefaultOperandFactory,
    formatter: DefaultInstructionFormatter,
    register_manager: RegisterManager,
    operand_builder: OperandBuilder,
}

impl ZicsrExtension {
    /// Create a new Zicsr extension instance.
    pub fn new() -> Self {
        Self {
            operand_factory: DefaultOperandFactory::new(),
            formatter: DefaultInstructionFormatter::new(),
            register_manager: RegisterManager::new(),
            operand_builder: OperandBuilder::new(),
        }
    }

    // Zicsr specific CSR registers
    // Note: These are just some common CSRs from Zicsr
    pub const CSR_STATUS: u32 = 0x000; // Machine Status Register
    pub const CSR_IE: u32 = 0x004; // Machine Interrupt Enable Register
    pub const CSR_TVAL: u32 = 0x003; // Machine Trap Value
    pub const CSR_TIP: u32 = 0x005; // Machine Interrupt Pending Register
    pub const CSR_TMIP: u32 = 0x707; // Machine Timer Interrupt Pending
    pub const CSR_TMEIE: u32 = 0x009; // Machine Timer External Interrupt Enable

    // Opcode constants
    const OPCODE_SYSTEM: u32 = 0b111_0011;

    // funct3 constants for system instructions
    const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
    const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
    const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
    const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
    const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
    const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

    // Zicsr specific CSR registers
    fn is_zicsr_csr(&self, csr: u32) -> bool {
        // Zicsr provides access to various CSRs
        // This is a simplified check - in reality, Zicsr allows access to all CSRs
        // that are supported by the implementation
        (csr >= 0x000 && csr <= 0x7FF) // Standard CSR address space
    }

    // Format CSR name based on address
    fn format_csr_name(&self, csr: u32) -> String {
        match csr {
            // Common CSR names
            Self::CSR_STATUS => match csr >> 8 {
                0 => "mstatus".to_string(),
                1 => "sstatus".to_string(),
                2 => "hstatus".to_string(),
                _ => format!("0x{:03x}", csr),
            },
            Self::CSR_IE => match csr >> 8 {
                0 => "mie".to_string(),
                1 => "sie".to_string(),
                2 => "hie".to_string(),
                _ => format!("0x{:03x}", csr),
            },
            Self::CSR_TVAL => match csr >> 8 {
                0 => "mtval".to_string(),
                1 => "stval".to_string(),
                2 => "htval".to_string(),
                _ => format!("0x{:03x}", csr),
            },
            Self::CSR_TIP => match csr >> 8 {
                0 => "mip".to_string(),
                1 => "sip".to_string(),
                2 => "hip".to_string(),
                _ => format!("0x{:03x}", csr),
            },
            Self::CSR_TMIP => "mtip".to_string(),
            Self::CSR_TMEIE => "mtie".to_string(),
            // Zicsr specific CSRs
            0xC00 => "cycle".to_string(),
            0xC01 => "time".to_string(),
            0xC02 => "instret".to_string(),
            0xC80 => "cycleh".to_string(),
            0xC81 => "timeh".to_string(),
            0xC82 => "instreth".to_string(),
            _ => format!("0x{:03x}", csr),
        }
    }

    // Decode CSR instructions with register operand
    fn decode_csr_instruction(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        csr: u32,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.format_csr_name(csr);

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
                csr_str
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
                csr_str,
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

    // Decode CSR instructions with immediate operand
    fn decode_csr_instruction_imm(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
        csr: u32,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.format_csr_name(csr);

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
                csr_str
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
                csr_str,
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

impl InstructionExtension for ZicsrExtension {
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
        _xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        // Check if this is a CSR instruction
        if opcode == Self::OPCODE_SYSTEM {
            match funct3 {
                Self::FUNCT3_SYSTEM_CSRRW => {
                    Some(self.decode_csr_instruction("csrrw", rd, rs1, funct12))
                }
                Self::FUNCT3_SYSTEM_CSRRS => {
                    Some(self.decode_csr_instruction("csrrs", rd, rs1, funct12))
                }
                Self::FUNCT3_SYSTEM_CSRRC => {
                    Some(self.decode_csr_instruction("csrrc", rd, rs1, funct12))
                }
                Self::FUNCT3_SYSTEM_CSRRWI => {
                    Some(self.decode_csr_instruction_imm("csrrwi", rd, rs1 as i64, funct12))
                }
                Self::FUNCT3_SYSTEM_CSRRSI => {
                    Some(self.decode_csr_instruction_imm("csrrsi", rd, rs1 as i64, funct12))
                }
                Self::FUNCT3_SYSTEM_CSRRCI => {
                    Some(self.decode_csr_instruction_imm("csrrci", rd, rs1 as i64, funct12))
                }
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
        // Zicsr extension doesn't handle compressed instructions
        None
    }

    fn name(&self) -> &'static str {
        "Zicsr"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        extensions & extension_masks::ZICSR != 0
    }
}

impl Default for ZicsrExtension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zicsr_extension_creation() {
        let extension = ZicsrExtension::new();
        assert_eq!(extension.name(), "Zicsr");
        assert!(extension.is_enabled(extension_masks::ZICSR));
        assert!(!extension.is_enabled(0));
    }

    #[test]
    fn test_zicsr_instruction_decoding() {
        let extension = ZicsrExtension::new();

        // Test CSRRW x1, cycle, x2 (assumes cycle is 0xC00)
        let result = extension.try_decode_standard(
            0b1110011, // opcode (SYSTEM)
            0b001,     // funct3 (CSRRW)
            0,         // funct7
            1,         // rd
            2,         // rs1
            0,         // rs2
            0xC00,     // funct12 (cycle CSR)
            0,         // imm_i
            0,         // imm_s
            0,         // imm_b
            0,         // imm_u
            0,         // imm_j
            Xlen::X32,
        );

        assert!(result.is_some());
        let instruction = result.unwrap().unwrap();
        assert_eq!(instruction.mnemonic, "csrrw");
    }

    #[test]
    fn test_zicsr_csr_formatting() {
        let extension = ZicsrExtension::new();
        assert_eq!(extension.format_csr_name(0xC00), "cycle");
        assert_eq!(extension.format_csr_name(0xC01), "time");
        assert_eq!(extension.format_csr_name(0xC02), "instret");
    }
}
