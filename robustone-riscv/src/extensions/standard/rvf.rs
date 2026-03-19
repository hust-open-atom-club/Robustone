//! RVF (Single-Precision Floating-Point) Extension
//!
//! This module implements the RISC-V single-precision floating-point extension (F extension),
//! which provides IEEE 754 single-precision floating-point operations.

use super::Standard;
use crate::decoder::{RiscVDecodedInstruction, Xlen};
use crate::extensions::{Extensions, InstructionExtension};
use crate::shared::{
    operands::convenience,
    registers::{RegisterManager, RegisterNameProvider},
};
use crate::types::*;
use robustone_core::types::error::DisasmError;

/// RVF Single-Precision Floating-Point Extension
pub struct Rvf {
    register_manager: RegisterManager,
}

impl Rvf {
    /// Create a new RVF extension instance.
    pub fn new() -> Self {
        Self {
            register_manager: RegisterManager::new(),
        }
    }

    fn reg_operand(&self, reg: u8, access: Access, is_fp: bool) -> RiscVOperand {
        if is_fp {
            convenience::fp_register(reg, access)
        } else {
            convenience::register(reg, access)
        }
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

    fn decode_load_fp(
        &self,
        rd: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "flw".to_string(),
            operands: format!(
                "{}, {}({})",
                self.register_manager.fp_register_name(rd),
                convenience::format_immediate(imm),
                self.register_manager.int_register_name(rs1)
            ),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), true),
                convenience::memory(rs1, imm),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_store_fp(
        &self,
        rs2: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "fsw".to_string(),
            operands: format!(
                "{}, {}({})",
                self.register_manager.fp_register_name(rs2),
                convenience::format_immediate(imm),
                self.register_manager.int_register_name(rs1)
            ),
            format: RiscVInstructionFormat::S,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rs2, Access::read(), true),
                convenience::memory(rs1, imm),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
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
                self.register_manager.fp_register_name(rd),
                self.register_manager.fp_register_name(rs1),
                self.register_manager.fp_register_name(rs2)
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), true),
                self.reg_operand(rs1, Access::read(), true),
                self.reg_operand(rs2, Access::read(), true),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_fp_r_type_with_rm(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}, {}, {}",
                self.register_manager.fp_register_name(rd),
                self.register_manager.fp_register_name(rs1),
                self.register_manager.fp_register_name(rs2),
                rounding_mode_name(rm),
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), true),
                self.reg_operand(rs1, Access::read(), true),
                self.reg_operand(rs2, Access::read(), true),
                convenience::rounding_mode(rm),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
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
                self.register_manager.fp_register_name(rd),
                self.register_manager.fp_register_name(rs1),
                self.register_manager.fp_register_name(rs2),
                self.register_manager.fp_register_name(rs3)
            ),
            format: RiscVInstructionFormat::R4,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), true),
                self.reg_operand(rs1, Access::read(), true),
                self.reg_operand(rs2, Access::read(), true),
                self.reg_operand(rs3, Access::read(), true),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_fp_int_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        _rs2: u8,
        rd_is_fp: bool,
        rs1_is_fp: bool,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let rd_name = if rd_is_fp {
            self.register_manager.fp_register_name(rd)
        } else {
            self.register_manager.int_register_name(rd)
        };
        let rs1_name = if rs1_is_fp {
            self.register_manager.fp_register_name(rs1)
        } else {
            self.register_manager.int_register_name(rs1)
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{rd_name}, {rs1_name}"),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), rd_is_fp),
                self.reg_operand(rs1, Access::read(), rs1_is_fp),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_fp_int_type_with_rm(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rd_is_fp: bool,
        rs1_is_fp: bool,
        rm: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let rd_name = if rd_is_fp {
            self.register_manager.fp_register_name(rd)
        } else {
            self.register_manager.int_register_name(rd)
        };
        let rs1_name = if rs1_is_fp {
            self.register_manager.fp_register_name(rs1)
        } else {
            self.register_manager.int_register_name(rs1)
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{rd_name}, {rs1_name}, {}", rounding_mode_name(rm)),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), rd_is_fp),
                self.reg_operand(rs1, Access::read(), rs1_is_fp),
                convenience::rounding_mode(rm),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_fp_unary_type_with_rm(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rm: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!(
                "{}, {}, {}",
                self.register_manager.fp_register_name(rd),
                self.register_manager.fp_register_name(rs1),
                rounding_mode_name(rm),
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), true),
                self.reg_operand(rs1, Access::read(), true),
                convenience::rounding_mode(rm),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }

    fn decode_fp_compare_type(
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
                self.register_manager.fp_register_name(rs1),
                self.register_manager.fp_register_name(rs2)
            ),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.reg_operand(rd, Access::write(), false),
                self.reg_operand(rs1, Access::read(), true),
                self.reg_operand(rs2, Access::read(), true),
            ],
            canonical_mnemonic: None,
            render_hints: Default::default(),
        })
    }
}

impl InstructionExtension for Rvf {
    fn name(&self) -> &'static str {
        "F"
    }

    fn is_enabled(&self, extensions: &Extensions) -> bool {
        // F extension bit (bit 3)
        extensions.standard.contains(Standard::F)
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
        xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        match opcode {
            Self::OPCODE_LOAD_FP if funct3 == Self::FUNCT3_LOAD_FLW => {
                Some(self.decode_load_fp(rd, rs1, imm_i))
            }
            Self::OPCODE_STORE_FP if funct3 == Self::FUNCT3_STORE_FSW => {
                Some(self.decode_store_fp(rs2, rs1, imm_s))
            }
            Self::OPCODE_FMADD => {
                if (funct7 & 0b11) != 0b00 {
                    return None;
                }
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fmadd.s", rd, rs1, rs2, rs3))
            }
            Self::OPCODE_FMSUB => {
                if (funct7 & 0b11) != 0b00 {
                    return None;
                }
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fmsub.s", rd, rs1, rs2, rs3))
            }
            Self::OPCODE_FNMSUB => {
                if (funct7 & 0b11) != 0b00 {
                    return None;
                }
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fnmsub.s", rd, rs1, rs2, rs3))
            }
            Self::OPCODE_FNMADD => {
                if (funct7 & 0b11) != 0b00 {
                    return None;
                }
                let rs3 = (funct7 >> 2) & 0x1F;
                Some(self.decode_fp_r4_type("fnmadd.s", rd, rs1, rs2, rs3))
            }
            Self::OPCODE_FP => {
                let funct5 = funct7 >> 2;
                let fmt = funct7 & 0b11;

                if fmt != 0b00 {
                    // Only single-precision (fmt=00), let RVD extension handle fmt=01
                    return None;
                }

                match (funct5, funct3) {
                    (0b00000, rm) => {
                        Some(self.decode_fp_r_type_with_rm("fadd.s", rd, rs1, rs2, rm))
                    }
                    (0b00001, rm) => {
                        Some(self.decode_fp_r_type_with_rm("fsub.s", rd, rs1, rs2, rm))
                    }
                    (0b00010, rm) => {
                        Some(self.decode_fp_r_type_with_rm("fmul.s", rd, rs1, rs2, rm))
                    }
                    (0b00011, rm) => {
                        Some(self.decode_fp_r_type_with_rm("fdiv.s", rd, rs1, rs2, rm))
                    }
                    (0b01011, rm) => {
                        Some(self.decode_fp_unary_type_with_rm("fsqrt.s", rd, rs1, rm))
                    }
                    (0b00100, 0b000) => Some(self.decode_fp_r_type("fsgnj.s", rd, rs1, rs2)),
                    (0b00100, 0b001) => Some(self.decode_fp_r_type("fsgnjn.s", rd, rs1, rs2)),
                    (0b00100, 0b010) => Some(self.decode_fp_r_type("fsgnjx.s", rd, rs1, rs2)),
                    (0b00101, 0b000) => Some(self.decode_fp_r_type("fmin.s", rd, rs1, rs2)),
                    (0b00101, 0b001) => Some(self.decode_fp_r_type("fmax.s", rd, rs1, rs2)),
                    (0b11000, rm) => match rs2 {
                        0 => Some(
                            self.decode_fp_int_type_with_rm("fcvt.w.s", rd, rs1, false, true, rm),
                        ),
                        1 => Some(self.decode_fp_int_type_with_rm(
                            "fcvt.wu.s",
                            rd,
                            rs1,
                            false,
                            true,
                            rm,
                        )),
                        2 => {
                            if xlen == Xlen::X64 {
                                Some(self.decode_fp_int_type_with_rm(
                                    "fcvt.l.s", rd, rs1, false, true, rm,
                                ))
                            } else {
                                Some(Err(DisasmError::DecodingError(
                                    "fcvt.l.s requires RV64".to_string(),
                                )))
                            }
                        }
                        3 => {
                            if xlen == Xlen::X64 {
                                Some(self.decode_fp_int_type_with_rm(
                                    "fcvt.lu.s",
                                    rd,
                                    rs1,
                                    false,
                                    true,
                                    rm,
                                ))
                            } else {
                                Some(Err(DisasmError::DecodingError(
                                    "fcvt.lu.s requires RV64".to_string(),
                                )))
                            }
                        }
                        _ => Some(Err(DisasmError::DecodingError(
                            "Invalid F-extension integer conversion".to_string(),
                        ))),
                    },
                    (0b11100, 0b000) => {
                        Some(self.decode_fp_int_type("fmv.x.w", rd, rs1, rs2, false, true))
                    } // rs2 ignored
                    (0b10100, 0b010) => Some(self.decode_fp_compare_type("feq.s", rd, rs1, rs2)),
                    (0b10100, 0b001) => Some(self.decode_fp_compare_type("flt.s", rd, rs1, rs2)),
                    (0b10100, 0b000) => Some(self.decode_fp_compare_type("fle.s", rd, rs1, rs2)),
                    (0b11100, 0b001) => {
                        Some(self.decode_fp_int_type("fclass.s", rd, rs1, rs2, false, true))
                    } // rs2 ignored
                    (0b11010, rm) => match rs2 {
                        0 => Some(
                            self.decode_fp_int_type_with_rm("fcvt.s.w", rd, rs1, true, false, rm),
                        ),
                        1 => Some(self.decode_fp_int_type_with_rm(
                            "fcvt.s.wu",
                            rd,
                            rs1,
                            true,
                            false,
                            rm,
                        )),
                        2 => {
                            if xlen == Xlen::X64 {
                                Some(self.decode_fp_int_type_with_rm(
                                    "fcvt.s.l", rd, rs1, true, false, rm,
                                ))
                            } else {
                                Some(Err(DisasmError::DecodingError(
                                    "fcvt.s.l requires RV64".to_string(),
                                )))
                            }
                        }
                        3 => {
                            if xlen == Xlen::X64 {
                                Some(self.decode_fp_int_type_with_rm(
                                    "fcvt.s.lu",
                                    rd,
                                    rs1,
                                    true,
                                    false,
                                    rm,
                                ))
                            } else {
                                Some(Err(DisasmError::DecodingError(
                                    "fcvt.s.lu requires RV64".to_string(),
                                )))
                            }
                        }
                        _ => Some(Err(DisasmError::DecodingError(
                            "Invalid F-extension floating conversion".to_string(),
                        ))),
                    },
                    (0b11110, 0b000) => {
                        Some(self.decode_fp_int_type("fmv.w.x", rd, rs1, rs2, true, false))
                    } // rs2 ignored
                    _ => Some(Err(DisasmError::DecodingError(
                        "Invalid F-extension encoding".to_string(),
                    ))),
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

impl Default for Rvf {
    fn default() -> Self {
        Self::new()
    }
}

fn rounding_mode_name(rm: u8) -> &'static str {
    match rm {
        0b000 => "rne",
        0b001 => "rtz",
        0b010 => "rdn",
        0b011 => "rup",
        0b100 => "rmm",
        0b111 => "dyn",
        _ => "invalid",
    }
}
