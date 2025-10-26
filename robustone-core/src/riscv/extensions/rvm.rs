//! RVM (Multiply and Divide) Extension
//!
//! This module implements the RISC-V multiply and divide extension (M extension),
//! which includes integer multiplication, division, and remainder operations.

use super::InstructionExtension;
use super::super::types::*;
use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    operands::DefaultOperandFactory,
    registers::{RegisterManager, RegisterNameProvider},
    OperandFactory,
};
use crate::error::DisasmError;

/// RVM Multiply and Divide Extension
pub struct RvmExtension {
    operand_factory: DefaultOperandFactory,
    register_manager: RegisterManager,
}

impl RvmExtension {
    /// Create a new RVM extension instance.
    pub fn new() -> Self {
        Self {
            operand_factory: DefaultOperandFactory::new(),
            register_manager: RegisterManager::new(),
        }
    }

    // M-extension opcodes (same as base opcodes, but distinguished by funct7)
    const OPCODE_OP: u32 = 0b011_0011;

    // M-extension funct3 values
    const FUNCT3_OP_ADD_SUB: u8 = 0b000;
    const FUNCT3_OP_SLL: u8 = 0b001;
    const FUNCT3_OP_SLT: u8 = 0b010;
    const FUNCT3_OP_SLTU: u8 = 0b011;
    const FUNCT3_OP_XOR: u8 = 0b100;
    const FUNCT3_OP_SRL_SRA: u8 = 0b101;
    const FUNCT3_OP_OR: u8 = 0b110;
    const FUNCT3_OP_AND: u8 = 0b111;

    // M-extension funct7 values
    const FUNCT7_OP_MUL: u8 = 0b000_0001;

    fn decode_r_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}, {}",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs1),
                self.register_manager.int_register_name(rs2)
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.operand_factory.make_register_operand(rd, Access::write()),
                self.operand_factory.make_register_operand(rs1, Access::read()),
                self.operand_factory.make_register_operand(rs2, Access::read()),
            ],
        })
    }

  
    fn decode_mul(&self, funct3: u8, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_r_type("mul", rd, rs1, rs2),
            Self::FUNCT3_OP_SLL => self.decode_r_type("mulh", rd, rs1, rs2),
            Self::FUNCT3_OP_SLT => self.decode_r_type("mulhsu", rd, rs1, rs2),
            Self::FUNCT3_OP_SLTU => self.decode_r_type("mulhu", rd, rs1, rs2),
            Self::FUNCT3_OP_XOR => self.decode_r_type("div", rd, rs1, rs2),
            Self::FUNCT3_OP_SRL_SRA => self.decode_r_type("divu", rd, rs1, rs2),
            Self::FUNCT3_OP_OR => self.decode_r_type("rem", rd, rs1, rs2),
            Self::FUNCT3_OP_AND => self.decode_r_type("remu", rd, rs1, rs2),
            _ => Err(DisasmError::DecodingError("Invalid M-extension funct3".to_string())),
        }
    }
}

impl InstructionExtension for RvmExtension {
    fn name(&self) -> &'static str {
        "M"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // M extension bit (bit 1)
        extensions & 0b010 != 0
    }

    fn try_decode_standard(
        &self,
        opcode: u32,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
        _funct12: u32,
        _imm_i: i64,
        _imm_s: i64,
        _imm_b: i64,
        _imm_u: i64,
        _imm_j: i64,
        _xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        // Only handle OP opcode with M-extension funct7
        if opcode == Self::OPCODE_OP && funct7 == Self::FUNCT7_OP_MUL {
            Some(self.decode_mul(funct3, rd, rs1, rs2))
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
        // RVM extension doesn't handle compressed instructions
        None
    }
}