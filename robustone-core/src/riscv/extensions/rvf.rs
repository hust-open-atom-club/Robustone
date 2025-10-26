//! RVF (Single-Precision Floating-Point) Extension
//!
//! This module implements the RISC-V single-precision floating-point extension (F extension),
//! which provides IEEE 754 single-precision floating-point operations.

use super::InstructionExtension;
use super::super::types::*;
use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use crate::error::DisasmError;

/// RVF Single-Precision Floating-Point Extension
pub struct RvfExtension;

impl RvfExtension {
    /// Create a new RVF extension instance.
    pub const fn new() -> Self {
        Self
    }

    // F-extension opcodes
    const OPCODE_LOAD_FP: u32 = 0b000_0111;
    const OPCODE_STORE_FP: u32 = 0b010_0111;
    const OPCODE_FMADD: u32 = 0b100_0011;
    const OPCODE_FMSUB: u32 = 0b100_0111;
    const OPCODE_FNMSUB: u32 = 0b100_1011;
    const OPCODE_FNMADD: u32 = 0b100_1111;
    const OPCODE_FP: u32 = 0b101_0011;

    // F-extension funct3 values for loads/stores
    const FUNCT3_LOAD_FLW: u8 = 0b010;
    const FUNCT3_STORE_FSW: u8 = 0b010;

    // F-extension funct7 values for FP operations
    const FUNCT7_FADD_S: u8 = 0b000_0000;
    const FUNCT7_FSUB_S: u8 = 0b000_0100;
    const FUNCT7_FMUL_S: u8 = 0b000_1000;
    const FUNCT7_FDIV_S: u8 = 0b000_1100;
    const FUNCT7_FSQRT_S: u8 = 0b010_1100;
    const FUNCT7_FSGNJ_S: u8 = 0b001_0000;
    const FUNCT7_FMIN_MAX_S: u8 = 0b001_0100;
    const FUNCT7_FCVT_W_S: u8 = 0b110_0000;
    const FUNCT7_FCVT_WU_S: u8 = 0b110_0001;
    const FUNCT7_FMV_X_W: u8 = 0b111_0000;
    const FUNCT7_FEQ_S: u8 = 0b101_0000;
    const FUNCT7_FLT_S: u8 = 0b101_0001;
    const FUNCT7_FLE_S: u8 = 0b101_0010;
    const FUNCT7_FCLASS_S: u8 = 0b111_0000;
    const FUNCT7_FCVT_S_W: u8 = 0b110_1000;
    const FUNCT7_FCVT_S_WU: u8 = 0b110_1001;
    const FUNCT7_FMV_W_X: u8 = 0b111_1000;

    fn decode_load_fp(
        &self,
        rd: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let offset = self.format_imm(imm);
        Ok(RiscVDecodedInstruction {
            mnemonic: "flw".to_string(),
            operands: format!("{}, {}({})", self.f_reg_name(rd), offset, self.reg_name(rs1)),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_memory_operand(rs1, imm),
            ],
        })
    }

    fn decode_store_fp(
        &self,
        rs2: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let offset = self.format_imm(imm);
        Ok(RiscVDecodedInstruction {
            mnemonic: "fsw".to_string(),
            operands: format!("{}, {}({})", self.f_reg_name(rs2), offset, self.reg_name(rs1)),
            format: RiscVInstructionFormat::S,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rs2, Access::read()),
                self.make_memory_operand(rs1, imm),
            ],
        })
    }

    fn decode_fp_r_type(
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
                self.f_reg_name(rd),
                self.f_reg_name(rs1),
                self.f_reg_name(rs2)
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs1, Access::read()),
                self.make_register_operand(rs2, Access::read()),
            ],
        })
    }

    fn decode_fp_r4_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}, {}, {}",
                self.f_reg_name(rd),
                self.f_reg_name(rs1),
                self.f_reg_name(rs2),
                self.f_reg_name(rs3)
            ),
            format: RiscVInstructionFormat::R4,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs1, Access::read()),
                self.make_register_operand(rs2, Access::read()),
                self.make_register_operand(rs3, Access::read()),
            ],
        })
    }

    fn format_imm(&self, value: i64) -> String {
        if value == 0 {
            return "0".to_string();
        }

        let abs = value.abs();
        let use_hex = abs >= 10;

        if use_hex {
            if value < 0 {
                format!("-0x{:x}", abs)
            } else {
                format!("0x{:x}", abs)
            }
        } else if value < 0 {
            format!("-{}", abs)
        } else {
            format!("{}", value)
        }
    }

    fn reg_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "invalid",
        }
    }

    fn f_reg_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "ft0",
            1 => "ft1",
            2 => "ft2",
            3 => "ft3",
            4 => "ft4",
            5 => "ft5",
            6 => "ft6",
            7 => "ft7",
            8 => "fs0",
            9 => "fs1",
            10 => "fa0",
            11 => "fa1",
            12 => "fa2",
            13 => "fa3",
            14 => "fa4",
            15 => "fa5",
            16 => "fa6",
            17 => "fa7",
            18 => "fs2",
            19 => "fs3",
            20 => "fs4",
            21 => "fs5",
            22 => "fs6",
            23 => "fs7",
            24 => "fs8",
            25 => "fs9",
            26 => "fs10",
            27 => "fs11",
            28 => "ft8",
            29 => "ft9",
            30 => "ft10",
            31 => "ft11",
            _ => "invalid",
        }
    }

    fn make_register_operand(&self, reg: u8, access: Access) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Register,
            access,
            value: RiscVOperandValue::Register(reg as u32),
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
}

impl InstructionExtension for RvfExtension {
    fn name(&self) -> &'static str {
        "F"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // F extension bit (bit 3)
        extensions & 0b1000 != 0
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
        imm_i: i64,
        imm_s: i64,
        _imm_b: i64,
        _imm_u: i64,
        _imm_j: i64,
        _xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        match opcode {
            Self::OPCODE_LOAD_FP if funct3 == Self::FUNCT3_LOAD_FLW => {
                Some(self.decode_load_fp(rd, rs1, imm_i))
            }
            Self::OPCODE_STORE_FP if funct3 == Self::FUNCT3_STORE_FSW => {
                Some(self.decode_store_fp(rs2, rs1, imm_s))
            }
            Self::OPCODE_FMADD => {
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fmadd.s", rd, rs1, rs2, rs3 as u8))
            }
            Self::OPCODE_FMSUB => {
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fmsub.s", rd, rs1, rs2, rs3 as u8))
            }
            Self::OPCODE_FNMSUB => {
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fnmsub.s", rd, rs1, rs2, rs3 as u8))
            }
            Self::OPCODE_FNMADD => {
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fnmadd.s", rd, rs1, rs2, rs3 as u8))
            }
            Self::OPCODE_FP => {
                let funct5 = funct7 >> 2;
                let fmt = funct7 & 0b11;

                if fmt != 0b00 { // Only single-precision (fmt=00)
                    return Some(Err(DisasmError::DecodingError("Invalid F-extension fmt".to_string())));
                }

                match (funct5, funct3) {
                    (0b00000, 0b000) => Some(self.decode_fp_r_type("fadd.s", rd, rs1, rs2)),
                    (0b00001, 0b000) => Some(self.decode_fp_r_type("fsub.s", rd, rs1, rs2)),
                    (0b00010, 0b000) => Some(self.decode_fp_r_type("fmul.s", rd, rs1, rs2)),
                    (0b00011, 0b000) => Some(self.decode_fp_r_type("fdiv.s", rd, rs1, rs2)),
                    (0b01011, 0b000) => Some(self.decode_fp_r_type("fsqrt.s", rd, rs1, rs2)), // rs2 ignored
                    (0b00100, 0b000) => Some(self.decode_fp_r_type("fsgnj.s", rd, rs1, rs2)),
                    (0b00100, 0b001) => Some(self.decode_fp_r_type("fsgnjn.s", rd, rs1, rs2)),
                    (0b00100, 0b010) => Some(self.decode_fp_r_type("fsgnjx.s", rd, rs1, rs2)),
                    (0b00101, 0b000) => Some(self.decode_fp_r_type("fmin.s", rd, rs1, rs2)),
                    (0b00101, 0b001) => Some(self.decode_fp_r_type("fmax.s", rd, rs1, rs2)),
                    (0b11000, 0b000) => Some(self.decode_fp_r_type("fcvt.w.s", rd, rs1, rs2)), // rs2 ignored
                    (0b11000, 0b001) => Some(self.decode_fp_r_type("fcvt.wu.s", rd, rs1, rs2)), // rs2 ignored
                    (0b11100, 0b000) => Some(self.decode_fp_r_type("fmv.x.w", rd, rs1, rs2)), // rs2 ignored
                    (0b10100, 0b010) => Some(self.decode_fp_r_type("feq.s", rd, rs1, rs2)),
                    (0b10100, 0b001) => Some(self.decode_fp_r_type("flt.s", rd, rs1, rs2)),
                    (0b10100, 0b000) => Some(self.decode_fp_r_type("fle.s", rd, rs1, rs2)),
                    (0b11100, 0b001) => Some(self.decode_fp_r_type("fclass.s", rd, rs1, rs2)), // rs2 ignored
                    (0b11010, 0b000) => Some(self.decode_fp_r_type("fcvt.s.w", rd, rs1, rs2)), // rs2 ignored
                    (0b11010, 0b001) => Some(self.decode_fp_r_type("fcvt.s.wu", rd, rs1, rs2)), // rs2 ignored
                    (0b11110, 0b000) => Some(self.decode_fp_r_type("fmv.w.x", rd, rs1, rs2)), // rs2 ignored
                    _ => Some(Err(DisasmError::DecodingError("Invalid F-extension encoding".to_string()))),
                }
            }
            _ => None,
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
        // RVF extension doesn't handle compressed instructions in this implementation
        None
    }
}