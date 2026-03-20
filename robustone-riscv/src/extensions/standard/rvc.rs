//! RVC (Compressed Instructions) Extension
//!
//! This module implements the RISC-V compressed instruction extension (C extension),
//! which provides 16-bit compressed versions of common instructions to improve code density.

use super::Standard;
use crate::decoder::{Xlen, build_riscv_decoded_instruction};
use crate::extensions::{Extensions, InstructionExtension};
use crate::shared::{
    encoding::convenience as encoding_conv, operands::convenience, registers::RegisterManager,
};
use crate::types::*;
use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

/// RVC Compressed Instructions Extension
pub struct Rvc {
    register_manager: RegisterManager,
}

impl Rvc {
    /// Create a new RVC extension instance.
    pub fn new() -> Self {
        Self {
            register_manager: RegisterManager::new(),
        }
    }

    fn decode_c_addi4spn(&self, rdp: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.addi4spn",
            RiscVInstructionFormat::CIW,
            2,
            vec![
                convenience::register(rdp + 8, Access::write()),
                convenience::register(2, Access::read()),
                convenience::immediate(imm_val),
            ],
        ))
    }

    fn decode_c_addi16sp(&self, rd: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = encoding_conv::sign_extend_16(imm, 10);
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.addi16sp",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm_val),
            ],
        ))
    }

    fn decode_c_lui(&self, rd: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "lui",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::write()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_add(&self, rd: u8, rs2: u8) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.add",
            RiscVInstructionFormat::CR,
            2,
            vec![
                convenience::register(rd, Access::read_write()),
                convenience::register(rs2, Access::read()),
            ],
        ))
    }

    fn decode_c_mv(&self, rd: u8, rs2: u8) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.mv",
            RiscVInstructionFormat::CR,
            2,
            vec![
                convenience::register(rd, Access::write()),
                convenience::register(rs2, Access::read()),
            ],
        ))
    }

    fn decode_c_jr(&self, rd: u8) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.jr",
            RiscVInstructionFormat::CR,
            2,
            vec![convenience::register(rd, Access::read())],
        )
        .with_capstone_alias("jr", Vec::new()))
    }

    fn decode_c_jalr(&self, rd: u8) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.jalr",
            RiscVInstructionFormat::CR,
            2,
            vec![convenience::register(rd, Access::read())],
        )
        .with_capstone_alias("jalr", Vec::new()))
    }

    fn decode_c_lw(&self, rd: u8, rs1: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.lw",
            RiscVInstructionFormat::CL,
            2,
            vec![
                convenience::register(rd + 8, Access::write()),
                convenience::memory(rs1 + 8, imm_val),
            ],
        ))
    }

    fn decode_c_sw(&self, rs2: u8, rs1: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.sw",
            RiscVInstructionFormat::CS,
            2,
            vec![
                convenience::register(rs2 + 8, Access::read()),
                convenience::memory(rs1 + 8, imm_val),
            ],
        ))
    }

    fn decode_c_lwsp(&self, rd: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.lwsp",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::write()),
                convenience::memory(2, imm_val),
            ],
        ))
    }

    fn decode_c_swsp(&self, rs2: u8, imm: u16) -> Result<DecodedInstruction, DisasmError> {
        let imm_val = imm as i64;
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.swsp",
            RiscVInstructionFormat::CSS,
            2,
            vec![
                convenience::register(rs2, Access::read()),
                convenience::memory(2, imm_val),
            ],
        ))
    }

    fn decode_c_addi(&self, rd: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.addi",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_li(&self, rd: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.li",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::write()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_alu(
        &self,
        funct6: u8,
        rd: u8,
        rs2: u8,
        funct2: u8,
    ) -> Result<DecodedInstruction, DisasmError> {
        let mnemonic = match (funct6 & 0b11, funct2) {
            (0b00, 0b00) => "c.srli",
            (0b01, 0b00) => "c.srai",
            (0b10, 0b00) => "c.andi",
            (0b11, 0b00) => "c.sub",
            (0b11, 0b01) => "c.xor",
            (0b11, 0b10) => "c.or",
            (0b11, 0b11) => "c.and",
            _ => {
                return Err(DisasmError::DecodingError(
                    "Invalid C.ALU encoding".to_string(),
                ));
            }
        };

        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::CA,
            2,
            vec![
                convenience::register(rd + 8, Access::read_write()),
                convenience::register(rs2 + 8, Access::read()),
            ],
        ))
    }

    fn decode_c_j(&self, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        Ok(build_riscv_decoded_instruction(
            "c.j",
            RiscVInstructionFormat::CJ,
            2,
            vec![convenience::immediate(imm)],
        ))
    }

    fn decode_c_jal(&self, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        Ok(build_riscv_decoded_instruction(
            "c.jal",
            RiscVInstructionFormat::CJ,
            2,
            vec![convenience::immediate(imm)],
        ))
    }

    fn decode_c_beqz(&self, rs1: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.beqz",
            RiscVInstructionFormat::CB,
            2,
            vec![
                convenience::register(rs1 + 8, Access::read()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_bnez(&self, rs1: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.bnez",
            RiscVInstructionFormat::CB,
            2,
            vec![
                convenience::register(rs1 + 8, Access::read()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_slli(&self, rd: u8, imm: i64) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        Ok(build_riscv_decoded_instruction(
            "c.slli",
            RiscVInstructionFormat::CI,
            2,
            vec![
                convenience::register(rd, Access::read_write()),
                convenience::immediate(imm),
            ],
        ))
    }

    fn decode_c_unimp(&self) -> Result<DecodedInstruction, DisasmError> {
        Ok(build_riscv_decoded_instruction(
            "c.unimp",
            RiscVInstructionFormat::CI,
            2,
            vec![],
        ))
    }

    fn decode_c_ebreak(&self) -> Result<DecodedInstruction, DisasmError> {
        Ok(build_riscv_decoded_instruction(
            "ebreak",
            RiscVInstructionFormat::CR,
            2,
            vec![],
        ))
    }

    fn decode_c_unknown(&self, instruction: u16) -> Result<DecodedInstruction, DisasmError> {
        Err(DisasmError::decode_failure(
            crate::types::error::DecodeErrorKind::InvalidEncoding,
            Some("riscv".to_string()),
            format!("unrecognized compressed instruction 0x{instruction:04x}"),
        ))
    }

    fn decode_c_unimplemented(
        &self,
        mnemonic: &str,
        detail: &str,
    ) -> Result<DecodedInstruction, DisasmError> {
        Err(DisasmError::decode_failure(
            crate::types::error::DecodeErrorKind::UnimplementedInstruction,
            Some("riscv".to_string()),
            format!(
                "{mnemonic} is a legal compressed instruction but is not implemented: {detail}"
            ),
        ))
    }
}

impl InstructionExtension for Rvc {
    fn name(&self) -> &'static str {
        "C"
    }

    fn is_enabled(&self, extensions: &Extensions) -> bool {
        // C extension bit (bit 5)
        extensions.standard.contains(Standard::C)
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
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        // RVC extension only handles compressed instructions
        None
    }

    fn try_decode_compressed(
        &self,
        instruction: u16,
        opcode: u8,
        funct3: u8,
        xlen: Xlen,
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
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
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
            (0b00, 0b011) if xlen == Xlen::X64 => Some(self.decode_c_unimplemented(
                "c.ld",
                "compressed RV64 load-double path is not implemented",
            )),
            (0b00, 0b010) => Some(self.decode_c_lw(rdp, rs1p, uimm_cl)),
            (0b00, 0b110) => Some(self.decode_c_sw(rs2p, rs1p, uimm_cs)),
            (0b00, 0b111) if xlen == Xlen::X64 => Some(self.decode_c_unimplemented(
                "c.sd",
                "compressed RV64 store-double path is not implemented",
            )),

            // C1 opcode (quarters 1)
            (0b01, 0b000) => Some(self.decode_c_addi(rd_full, imm_ci)),
            (0b01, 0b001) => Some(self.decode_c_jal(imm_cj)),
            (0b01, 0b010) => Some(self.decode_c_li(rd_full, imm_ci)),
            (0b01, 0b011) => {
                if rd_full == 2 {
                    let imm_val = (((instruction >> 12) & 0x1) << 9)
                        | (((instruction >> 3) & 0x3) << 7)
                        | (((instruction >> 5) & 0x1) << 6)
                        | (((instruction >> 2) & 0x3) << 4)
                        | (((instruction >> 6) & 0x1) << 5);
                    Some(self.decode_c_addi16sp(rd_full, imm_val))
                } else if rd_full != 0 && imm_ci != 0 {
                    Some(self.decode_c_lui(rd_full, imm_ci))
                } else {
                    Some(self.decode_c_unknown(instruction))
                }
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
            (0b10, 0b011) if xlen == Xlen::X64 => Some(self.decode_c_unimplemented(
                "c.ldsp",
                "compressed RV64 stack load-double path is not implemented",
            )),
            (0b10, 0b100) => {
                let bit12 = ((instruction >> 12) & 0x1) as u8;
                match (bit12, rd_full, rs2_full) {
                    (0, rd, 0) if rd != 0 => Some(self.decode_c_jr(rd)),
                    (0, rd, rs2) if rd != 0 && rs2 != 0 => Some(self.decode_c_mv(rd, rs2)),
                    (1, 0, 0) => Some(self.decode_c_ebreak()),
                    (1, rd, 0) if rd != 0 => Some(self.decode_c_jalr(rd)),
                    (1, rd, rs2) if rd != 0 && rs2 != 0 => Some(self.decode_c_add(rd, rs2)),
                    _ => Some(self.decode_c_unknown(instruction)),
                }
            }
            (0b10, 0b110) => Some(self.decode_c_swsp(rs2_full, uimm_css)),
            (0b10, 0b111) if xlen == Xlen::X64 => Some(self.decode_c_unimplemented(
                "c.sdsp",
                "compressed RV64 stack store-double path is not implemented",
            )),

            _ => Some(self.decode_c_unknown(instruction)),
        }
    }
}

impl Default for Rvc {
    fn default() -> Self {
        Self::new()
    }
}
