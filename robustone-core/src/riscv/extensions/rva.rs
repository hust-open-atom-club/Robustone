//! RVA (Atomic Instructions) Extension
//!
//! This module implements the RISC-V atomic instructions extension (A extension),
//! which provides atomic memory operations for synchronization and concurrency.

use super::InstructionExtension;
use super::super::types::*;
use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    operands::convenience,
    registers::{RegisterManager, RegisterNameProvider},
};
use crate::error::DisasmError;

/// RVA Atomic Instructions Extension
pub struct RvaExtension {
    register_manager: RegisterManager,
}

impl RvaExtension {
    /// Create a new RVA extension instance.
    pub fn new() -> Self {
        Self {
            register_manager: RegisterManager::new(),
        }
    }

    // A-extension opcode
    const OPCODE_A: u32 = 0b010_1111;

    // A-extension funct3 values
    const FUNCT3_AMO_W: u8 = 0b010;
    const FUNCT3_AMO_D: u8 = 0b011;
    const FUNCT3_LR_W: u8 = 0b010;
    const FUNCT3_SC_W: u8 = 0b010;
    const FUNCT3_LR_D: u8 = 0b011;
    const FUNCT3_SC_D: u8 = 0b011;

    // A-extension funct5 values
    const FUNCT5_LR: u8 = 0b00010;
    const FUNCT5_SC: u8 = 0b00011;
    const FUNCT5_AMOSWAP: u8 = 0b00001;
    const FUNCT5_AMOADD: u8 = 0b00000;
    const FUNCT5_AMOXOR: u8 = 0b00100;
    const FUNCT5_AMOAND: u8 = 0b01100;
    const FUNCT5_AMOOR: u8 = 0b01000;
    const FUNCT5_AMOMIN: u8 = 0b10000;
    const FUNCT5_AMOMAX: u8 = 0b10100;
    const FUNCT5_AMOMINU: u8 = 0b11000;
    const FUNCT5_AMOMAXU: u8 = 0b11100;

    fn decode_amo(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}, ({})",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs2),
                self.register_manager.int_register_name(rs1)
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                convenience::register(rd, Access::write()),
                convenience::register(rs2, Access::read()),
                convenience::register(rs1, Access::read()),
            ],
        })
    }

    fn decode_lr_sc(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = if mnemonic == "lr.w" || mnemonic == "lr.d" {
            format!(
                "{}, ({})",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs1)
            )
        } else {
            format!(
                "{}, {}, ({})",
                self.register_manager.int_register_name(rd),
                self.register_manager.int_register_name(rs2),
                self.register_manager.int_register_name(rs1)
            )
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands,
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                convenience::register(rd, Access::write()),
                convenience::register(rs2, Access::read()),
                convenience::register(rs1, Access::read()),
            ],
        })
    }
}

impl InstructionExtension for RvaExtension {
    fn name(&self) -> &'static str {
        "A"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // A extension bit (bit 2)
        extensions & 0b100 != 0
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
        xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        if opcode != Self::OPCODE_A {
            return None;
        }

        let funct5 = (funct7 >> 2) & 0b11111;
        let funct2 = funct7 & 0b11;

        match (funct3, funct5, funct2) {
            // Load-Reserved/Store-Conditional instructions
            (Self::FUNCT3_LR_W, Self::FUNCT5_LR, 0b00) => {
                Some(self.decode_lr_sc("lr.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_SC_W, Self::FUNCT5_SC, 0b00) => {
                Some(self.decode_lr_sc("sc.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_LR_D, Self::FUNCT5_LR, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_lr_sc("lr.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_SC_D, Self::FUNCT5_SC, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_lr_sc("sc.d", rd, rs1, rs2))
            }

            // Atomic Memory Operation instructions
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOSWAP, 0b00) => {
                Some(self.decode_amo("amoswap.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOADD, 0b00) => {
                Some(self.decode_amo("amoadd.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOXOR, 0b00) => {
                Some(self.decode_amo("amoxor.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOAND, 0b00) => {
                Some(self.decode_amo("amoand.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOOR, 0b00) => {
                Some(self.decode_amo("amoor.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOMIN, 0b00) => {
                Some(self.decode_amo("amomin.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOMAX, 0b00) => {
                Some(self.decode_amo("amomax.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOMINU, 0b00) => {
                Some(self.decode_amo("amominu.w", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_W, Self::FUNCT5_AMOMAXU, 0b00) => {
                Some(self.decode_amo("amomaxu.w", rd, rs1, rs2))
            }

            // 64-bit AMO instructions
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOSWAP, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amoswap.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOADD, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amoadd.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOXOR, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amoxor.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOAND, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amoand.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOOR, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amoor.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOMIN, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amomin.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOMAX, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amomax.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOMINU, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amominu.d", rd, rs1, rs2))
            }
            (Self::FUNCT3_AMO_D, Self::FUNCT5_AMOMAXU, 0b01) if xlen == Xlen::X64 => {
                Some(self.decode_amo("amomaxu.d", rd, rs1, rs2))
            }

            _ => Some(Err(DisasmError::DecodingError("Invalid A-extension encoding".to_string()))),
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
        // RVA extension doesn't handle compressed instructions
        None
    }
}