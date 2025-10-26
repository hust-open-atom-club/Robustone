//! RVC (Compressed Instructions) Extension
//!
//! This module implements the RISC-V compressed instruction extension (C extension),
//! which provides 16-bit compressed versions of common instructions to improve code density.

use super::InstructionExtension;
use super::super::types::*;
use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    operands::convenience,
    registers::{RegisterManager, RegisterNameProvider},
    encoding::convenience as encoding_conv,
};
use crate::error::DisasmError;

/// RVC Compressed Instructions Extension
pub struct RvcExtension {
    register_manager: RegisterManager,
}

impl RvcExtension {
    /// Create a new RVC extension instance.
    pub fn new() -> Self {
        Self {
            register_manager: RegisterManager::new(),
        }
    }

    fn decode_c_addi4spn(&self, rdp: u8, imm: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.addi4spn".to_string(),
            operands: format!(
                "{}, sp, {}",
                self.register_manager.int_register_name(rdp + 8),
                convenience::format_immediate(imm_val)
            ),
            format: RiscVInstructionFormat::CIW,
            size: 2,
            operands_detail: vec![
                convenience::register(rdp + 8, Access::write()),
                convenience::register(2, Access::read()),
                convenience::immediate(imm_val),
            ],
        })
    }

    fn decode_c_addi16sp(&self, rd: u8, imm: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = encoding_conv::sign_extend_16(imm, 10);
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.addi16sp".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                convenience::format_immediate(imm_val)
            ),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm_val),
            ],
        })
    }

    fn decode_c_add(&self, rd: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.add".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs2)
            ),
            format: RiscVInstructionFormat::CR,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::read_write()),
                convenience::register(rs2, Access::read()),
            ],
        })
    }

    fn decode_c_mv(&self, rd: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.mv".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs2)
            ),
            format: RiscVInstructionFormat::CR,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::write()),
                convenience::register(rs2, Access::read()),
            ],
        })
    }

    fn decode_c_jr(&self, rd: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.jr".to_string(),
            operands: format!("{}", self.register_manager.int_register_name(rd)),
            format: RiscVInstructionFormat::CR,
            size: 2,
            operands_detail: vec![convenience::register(rd, Access::read())],
        })
    }

    fn decode_c_jalr(&self, rd: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.jalr".to_string(),
            operands: format!("{}", self.register_manager.int_register_name(rd)),
            format: RiscVInstructionFormat::CR,
            size: 2,
            operands_detail: vec![convenience::register(rd, Access::read())],
        })
    }

    fn decode_c_lw(
        &self,
        rd: u8,
        rs1: u8,
        imm: u16,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.lw".to_string(),
            operands: format!(
                "{}, {}({})",
                self.register_manager.int_register_name(rd + 8),
                convenience::format_immediate(imm_val),
                self.register_manager.int_register_name(rs1 + 8)
            ),
            format: RiscVInstructionFormat::CL,
            size: 2,
            operands_detail: vec![
                convenience::register(rd + 8, Access::write()),
                convenience::memory(rs1 + 8, imm_val),
            ],
        })
    }

    fn decode_c_sw(
        &self,
        rs2: u8,
        rs1: u8,
        imm: u16,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.sw".to_string(),
            operands: format!(
                "{}, {}({})",
                self.register_manager.int_register_name(rs2 + 8),
                convenience::format_immediate(imm_val),
                self.register_manager.int_register_name(rs1 + 8)
            ),
            format: RiscVInstructionFormat::CS,
            size: 2,
            operands_detail: vec![
                convenience::register(rs2 + 8, Access::read()),
                convenience::memory(rs1 + 8, imm_val),
            ],
        })
    }

    fn decode_c_lwsp(&self, rd: u8, imm: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.lwsp".to_string(),
            operands: format!(
                "{}, {}(sp)",
                self.register_manager.int_register_name(rd),
                convenience::format_immediate(imm_val)
            ),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::write()),
                convenience::memory(2, imm_val),
            ],
        })
    }

    fn decode_c_swsp(&self, rs2: u8, imm: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.swsp".to_string(),
            operands: format!(
                "{}, {}(sp)",
                self.register_manager.int_register_name(rs2),
                convenience::format_immediate(imm_val)
            ),
            format: RiscVInstructionFormat::CSS,
            size: 2,
            operands_detail: vec![
                convenience::register(rs2, Access::read()),
                convenience::memory(2, imm_val),
            ],
        })
    }

    fn decode_c_addi(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.addi".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                convenience::format_immediate(imm)
            ),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm),
            ],
        })
    }

    fn decode_c_li(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.li".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                convenience::format_immediate(imm)
            ),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::write()),
                convenience::immediate(imm),
            ],
        })
    }

    fn decode_c_alu(
        &self,
        funct6: u8,
        rd: u8,
        rs2: u8,
        funct2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match (funct6 & 0b11, funct2) {
            (0b00, 0b00) => "c.srli",
            (0b01, 0b00) => "c.srai",
            (0b10, 0b00) => "c.andi",
            (0b11, 0b00) => "c.sub",
            (0b11, 0b01) => "c.xor",
            (0b11, 0b10) => "c.or",
            (0b11, 0b11) => "c.and",
            _ => return Err(DisasmError::DecodingError("Invalid C.ALU encoding".to_string())),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd + 8),
                self.register_manager.int_register_name(rs2 + 8)
            ),
            format: RiscVInstructionFormat::CA,
            size: 2,
            operands_detail: vec![
                convenience::register(rd + 8, Access::read_write()),
                convenience::register(rs2 + 8, Access::read()),
            ],
        })
    }

    fn decode_c_j(&self, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.j".to_string(),
            operands: convenience::format_immediate(imm),
            format: RiscVInstructionFormat::CJ,
            size: 2,
            operands_detail: vec![convenience::immediate(imm)],
        })
    }

    fn decode_c_jal(&self, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.jal".to_string(),
            operands: convenience::format_immediate(imm),
            format: RiscVInstructionFormat::CJ,
            size: 2,
            operands_detail: vec![convenience::immediate(imm)],
        })
    }

    fn decode_c_beqz(&self, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.beqz".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rs1 + 8),
                convenience::format_immediate(imm)
            ),
            format: RiscVInstructionFormat::CB,
            size: 2,
            operands_detail: vec![
                convenience::register(rs1 + 8, Access::read()),
                convenience::immediate(imm),
            ],
        })
    }

    fn decode_c_bnez(&self, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.bnez".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rs1 + 8),
                convenience::format_immediate(imm)
            ),
            format: RiscVInstructionFormat::CB,
            size: 2,
            operands_detail: vec![
                convenience::register(rs1 + 8, Access::read()),
                convenience::immediate(imm),
            ],
        })
    }

    fn decode_c_slli(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.slli".to_string(),
            operands: format!(
                "{}, {}",
                self.register_manager.int_register_name(rd),
                convenience::format_immediate(imm)
            ),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm),
            ],
        })
    }

    fn decode_c_unimp(&self) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.unimp".to_string(),
            operands: String::new(),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![],
        })
    }

    fn decode_c_unknown(&self, instruction: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.unknown".to_string(),
            operands: format!("0x{:04x}", instruction),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![],
        })
    }
}

impl InstructionExtension for RvcExtension {
    fn name(&self) -> &'static str {
        "C"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // C extension bit (bit 5)
        extensions & 0b100000 != 0
    }

    fn try_decode_standard(
        &self,
        _opcode: u32,
        _funct3: u8,
        _funct7: u8,
        _rd: u8,
        _rs1: u8,
        _rs2: u8,
        _funct12: u32,
        _imm_i: i64,
        _imm_s: i64,
        _imm_b: i64,
        _imm_u: i64,
        _imm_j: i64,
        _xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        // RVC extension only handles compressed instructions
        None
    }

    fn try_decode_compressed(
        &self,
        instruction: u16,
        opcode: u8,
        funct3: u8,
        _xlen: Xlen,
        rd_full: u8,
        _rs1_full: u8,
        rs2_full: u8,
        rdp: u8,
        rs1p: u8,
        rs2p: u8,
        nzuimm_ciw: u16,
        uimm_cl: u16,
        uimm_cs: u16,
        imm_ci: i64,
        imm_cj: i64,
        imm_cb: i64,
        uimm_css: u16,
        uimm_clsp: u16,
        _uimm_fldsp: u16,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        match (opcode, funct3) {
            // C0 opcode (quarters 0)
            (0b00, 0b000) => {
                // c.addi4spn with nzuimm == 0 is illegal, should be c.unimp
                if nzuimm_ciw == 0 {
                    Some(self.decode_c_unimp())
                } else {
                    Some(self.decode_c_addi4spn(rdp, nzuimm_ciw))
                }
            }
            (0b00, 0b010) => Some(self.decode_c_lw(rdp, rs1p, uimm_cl)),
            (0b00, 0b110) => Some(self.decode_c_sw(rs2p, rs1p, uimm_cs)),

            // C1 opcode (quarters 1)
            (0b01, 0b000) => Some(self.decode_c_addi(rd_full, imm_ci)),
            (0b01, 0b001) => Some(self.decode_c_jal(imm_cj)),
            (0b01, 0b010) => Some(self.decode_c_li(rd_full, imm_ci)),
            (0b01, 0b011) => {
                let imm_val = (((instruction >> 12) & 0x1) as u16) << 9
                    | (((instruction >> 3) & 0x3) as u16) << 7
                    | (((instruction >> 5) & 0x1) as u16) << 6
                    | (((instruction >> 2) & 0x3) as u16) << 4
                    | (((instruction >> 6) & 0x1) as u16) << 5;
                Some(self.decode_c_addi16sp(rd_full, imm_val))
            }
            (0b01, 0b100) => {
                let funct6 = ((instruction >> 10) & 0x3F) as u8;
                let funct2 = ((instruction >> 5) & 0x3) as u8;
                Some(self.decode_c_alu(funct6, rdp, rs2p, funct2))
            }
            (0b01, 0b101) => Some(self.decode_c_j(imm_cj)),
            (0b01, 0b110) => Some(self.decode_c_beqz(rs1p, imm_cb)),
            (0b01, 0b111) => Some(self.decode_c_bnez(rs1p, imm_cb)),

            // C2 opcode (quarters 2)
            (0b10, 0b000) => Some(self.decode_c_slli(rd_full, imm_ci)),
            (0b10, 0b010) => Some(self.decode_c_lwsp(rd_full, uimm_clsp)),
            (0b10, 0b100) => Some(self.decode_c_mv(rd_full, rs2_full)),
            (0b10, 0b101) => {
                if rs2_full == 0 {
                    Some(self.decode_c_jr(rd_full))
                } else if rd_full == 0 {
                    Some(self.decode_c_jalr(rd_full))
                } else {
                    Some(self.decode_c_add(rd_full, rs2_full))
                }
            }
            (0b10, 0b110) => Some(self.decode_c_swsp(rs2_full, uimm_css)),

            _ => Some(self.decode_c_unknown(instruction)),
        }
    }
}