//! XTheadCondMov (Conditional Move) Extension
//!
//! This module implements the XuanTie conditional move extension (XTheadCondMov),
//! which provides instructions to conditionally move register values based on
//! whether another register is zero or non-zero.

use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    operands::convenience,
    registers::{RegisterManager, RegisterNameProvider},
};
use super::super::types::*;
use super::{Extensions, InstructionExtension};
use crate::error::DisasmError;

/// XTheadCondMov Conditional Move Extension
pub struct XTheadCondMovExtension {
    register_manager: RegisterManager,
}

impl XTheadCondMovExtension {
    /// Create a new XTheadCondMov extension instance.
    pub fn new() -> Self {
        Self {
            register_manager: RegisterManager::new(),
        }
    }

    // XTheadCondMov encoding constants
    const OPCODE: u32 = 0x0B; // custom-0
    const FUNCT3: u8 = 0x1; // Arithmetic
    const FUNCT5: u8 = 0x08; // XTheadCondMov identifier

    // funct2 values to distinguish instructions
    const FUNCT2_MVEQZ: u8 = 0x00; // Move if equal to zero
    const FUNCT2_MVNEZ: u8 = 0x01; // Move if not equal to zero

    /// Decode an R-type conditional move instruction.
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
                convenience::register(rd, Access::write()),
                convenience::register(rs1, Access::read()),
                convenience::register(rs2, Access::read()),
            ],
        })
    }
}

impl InstructionExtension for XTheadCondMovExtension {
    fn name(&self) -> &'static str {
        "XTheadCondMov"
    }

    fn is_enabled(&self, extensions: Extensions) -> bool {
        // XTheadCondMov extension bit
        extensions.contains(Extensions::XTHEADCONDMOV)
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
        // Check if opcode matches XTheadCondMov custom-0 space
        if opcode != Self::OPCODE {
            return None;
        }

        // Check if funct3 matches Arithmetic encoding
        if funct3 != Self::FUNCT3 {
            return None;
        }

        // Extract funct5 (bits[6:2]) and funct2 (bits[1:0]) from funct7
        let funct5 = (funct7 >> 2) & 0x1F;
        let funct2 = funct7 & 0x3;

        // Check if funct5 matches XTheadCondMov identifier
        if funct5 != Self::FUNCT5 {
            return None;
        }

        // Decode based on funct2 to distinguish between mveqz and mvnez
        match funct2 {
            Self::FUNCT2_MVEQZ => Some(self.decode_r_type("th.mveqz", rd, rs1, rs2)),
            Self::FUNCT2_MVNEZ => Some(self.decode_r_type("th.mvnez", rd, rs1, rs2)),
            _ => Some(Err(DisasmError::DecodingError(
                "Invalid XTheadCondMov funct2".to_string(),
            ))),
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
        // XTheadCondMov extension does not provide compressed instruction variants
        None
    }
}

impl Default for XTheadCondMovExtension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mveqz_decoding() {
        let ext = XTheadCondMovExtension::new();

        // th.mveqz ra, sp, gp (x1, x2, x3)
        // opcode=0x0B, funct3=0x1, funct7=(0x08<<2)|0x00=0x20, rd=1, rs1=2, rs2=3
        let result = ext.try_decode_standard(0x0B, 0x1, 0x20, 1, 2, 3, 0, 0, 0, 0, 0, 0, Xlen::X32);

        assert!(result.is_some());
        let instr = result.unwrap().unwrap();
        assert_eq!(instr.mnemonic, "th.mveqz");
        assert_eq!(instr.operands, "ra, sp, gp");
        assert_eq!(instr.size, 4);
    }

    #[test]
    fn test_mvnez_decoding() {
        let ext = XTheadCondMovExtension::new();

        // th.mvnez x1, x2, x3
        // opcode=0x0B, funct3=0x1, funct7=(0x08<<2)|0x01=0x21, rd=1, rs1=2, rs2=3
        let result = ext.try_decode_standard(0x0B, 0x1, 0x21, 1, 2, 3, 0, 0, 0, 0, 0, 0, Xlen::X32);

        assert!(result.is_some());
        let instr = result.unwrap().unwrap();
        assert_eq!(instr.mnemonic, "th.mvnez");
        assert_eq!(instr.operands, "ra, sp, gp");
        assert_eq!(instr.size, 4);
    }

    #[test]
    fn test_non_matching_opcode() {
        let ext = XTheadCondMovExtension::new();

        // Different opcode should not match
        let result = ext.try_decode_standard(0x33, 0x1, 0x20, 1, 2, 3, 0, 0, 0, 0, 0, 0, Xlen::X32);

        assert!(result.is_none());
    }
}
