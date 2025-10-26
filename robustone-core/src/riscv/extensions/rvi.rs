//! RV32I/RV64I Base Integer Instruction Set Extension
//!
//! This module implements the RISC-V base integer instruction set (I extension),
//! which includes all the fundamental integer operations, control flow instructions,
//! memory operations, and system instructions that form the core of RISC-V.

use super::InstructionExtension;
use super::super::types::*;
use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use crate::error::DisasmError;

/// RV32I/RV64I Base Integer Extension
pub struct RviExtension;

impl RviExtension {
    /// Create a new RV32I/RV64I extension instance.
    pub const fn new() -> Self {
        Self
    }

    // Opcode constants for base integer instructions
    const OPCODE_LUI: u32 = 0b011_0111;
    const OPCODE_AUIPC: u32 = 0b001_0111;
    const OPCODE_JAL: u32 = 0b110_1111;
    const OPCODE_JALR: u32 = 0b110_0111;
    const OPCODE_BRANCH: u32 = 0b110_0011;
    const OPCODE_LOAD: u32 = 0b000_0011;
    const OPCODE_STORE: u32 = 0b010_0011;
    const OPCODE_OP_IMM: u32 = 0b001_0011;
    const OPCODE_OP: u32 = 0b011_0011;
    const OPCODE_OP_IMM_32: u32 = 0b001_1011;
    const OPCODE_OP_32: u32 = 0b011_1011;
    const OPCODE_MISC_MEM: u32 = 0b000_1111;
    const OPCODE_SYSTEM: u32 = 0b111_0011;

    // funct3 selector values
    const FUNCT3_LOAD_LB: u8 = 0b000;
    const FUNCT3_LOAD_LH: u8 = 0b001;
    const FUNCT3_LOAD_LW: u8 = 0b010;
    const FUNCT3_LOAD_LD: u8 = 0b011;
    const FUNCT3_LOAD_LBU: u8 = 0b100;
    const FUNCT3_LOAD_LHU: u8 = 0b101;
    const FUNCT3_LOAD_LWU: u8 = 0b110;

    const FUNCT3_STORE_SB: u8 = 0b000;
    const FUNCT3_STORE_SH: u8 = 0b001;
    const FUNCT3_STORE_SW: u8 = 0b010;
    const FUNCT3_STORE_SD: u8 = 0b011;

    const FUNCT3_BRANCH_BEQ: u8 = 0b000;
    const FUNCT3_BRANCH_BNE: u8 = 0b001;
    const FUNCT3_BRANCH_BLT: u8 = 0b100;
    const FUNCT3_BRANCH_BGE: u8 = 0b101;
    const FUNCT3_BRANCH_BLTU: u8 = 0b110;
    const FUNCT3_BRANCH_BGEU: u8 = 0b111;

    const FUNCT3_OP_ADD_SUB: u8 = 0b000;
    const FUNCT3_OP_SLL: u8 = 0b001;
    const FUNCT3_OP_SLT: u8 = 0b010;
    const FUNCT3_OP_SLTU: u8 = 0b011;
    const FUNCT3_OP_XOR: u8 = 0b100;
    const FUNCT3_OP_SRL_SRA: u8 = 0b101;
    const FUNCT3_OP_OR: u8 = 0b110;
    const FUNCT3_OP_AND: u8 = 0b111;

    const FUNCT7_OP_SRL: u8 = 0b000_0000;
    const FUNCT7_OP_SRA: u8 = 0b010_0000;
    const FUNCT7_OP_ADD: u8 = 0b000_0000;
    const FUNCT7_OP_SUB: u8 = 0b010_0000;
    const FUNCT7_OP_MUL: u8 = 0b000_0001; // Handled by RVM extension

    const FUNCT3_SYSTEM_PRIV: u8 = 0b000;
    const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
    const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
    const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
    const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
    const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
    const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

    const FUNCT12_SYSTEM_ECALL: u32 = 0b000_0000_0000;
    const FUNCT12_SYSTEM_EBREAK: u32 = 0b000_0000_0001;

    const FUNCT3_MISC_MEM_FENCE: u8 = 0b000;
    const FUNCT3_MISC_MEM_FENCE_I: u8 = 0b001;

    // Instruction format decoding methods
    fn decode_u_type(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_val = imm >> 12;
        let imm_str = if imm_val == 0 {
            "0".to_string()
        } else {
            format!("0x{:x}", imm_val)
        };
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}", self.reg_name(rd), imm_str),
            format: RiscVInstructionFormat::U,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_immediate_operand(imm >> 12),
            ],
        })
    }

    fn decode_j_type(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_str = self.format_imm(imm);
        let operands = match (mnemonic, rd) {
            ("j", _) => imm_str.clone(),
            ("jal", 1) => imm_str.clone(),
            _ => format!("{}, {}", self.reg_name(rd), imm_str),
        };
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands,
            format: RiscVInstructionFormat::J,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::read_write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_i_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_str = self.format_imm(imm);
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs1, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

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
                self.reg_name(rd),
                self.reg_name(rs1),
                self.reg_name(rs2)
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

    fn decode_s_type(
        &self,
        mnemonic: &str,
        rs2: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let offset = self.format_imm(imm);
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})", self.reg_name(rs2), offset, self.reg_name(rs1)),
            format: RiscVInstructionFormat::S,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rs2, Access::read()),
                self.make_memory_operand(rs1, imm),
            ],
        })
    }

    fn decode_b_type(
        &self,
        mnemonic: &str,
        rs1: u8,
        rs2: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let offset = self.format_imm(imm);
        let operands = match (mnemonic, rs2) {
            ("beqz", _) => format!("{}, {}", self.reg_name(rs1), offset),
            ("bnez", _) => format!("{}, {}", self.reg_name(rs1), offset),
            _ => format!("{}, {}, {}", self.reg_name(rs1), self.reg_name(rs2), offset),
        };
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands,
            format: RiscVInstructionFormat::B,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rs1, Access::read()),
                self.make_register_operand(rs2, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    // Specific instruction type decoders
    fn decode_auipc(&self, rd: u8, imm_u: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        self.decode_u_type("auipc", rd, imm_u)
    }

    fn decode_lui(&self, rd: u8, imm_u: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        self.decode_u_type("lui", rd, imm_u)
    }

    fn decode_jal(&self, rd: u8, imm_j: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = if rd == 0 { "j" } else { "jal" };
        self.decode_j_type(mnemonic, rd, imm_j)
    }

    fn decode_jalr(&self, rd: u8, rs1: u8, imm_i: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        self.decode_i_type("jalr", rd, rs1, imm_i)
    }

    fn decode_branch(&self, funct3: u8, rs1: u8, rs2: u8, imm_b: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_BRANCH_BEQ => {
                let mnemonic = if rs2 == 0 { "beqz" } else { "beq" };
                self.decode_b_type(mnemonic, rs1, rs2, imm_b)
            }
            Self::FUNCT3_BRANCH_BNE => {
                let mnemonic = if rs2 == 0 { "bnez" } else { "bne" };
                self.decode_b_type(mnemonic, rs1, rs2, imm_b)
            }
            Self::FUNCT3_BRANCH_BLT => self.decode_b_type("blt", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BGE => self.decode_b_type("bge", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BLTU => self.decode_b_type("bltu", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BGEU => self.decode_b_type("bgeu", rs1, rs2, imm_b),
            _ => Err(DisasmError::DecodingError("Invalid branch funct3".to_string())),
        }
    }

    fn decode_load(&self, funct3: u8, rd: u8, rs1: u8, imm_i: i64, xlen: Xlen) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_LOAD_LB => "lb",
            Self::FUNCT3_LOAD_LH => "lh",
            Self::FUNCT3_LOAD_LW => "lw",
            Self::FUNCT3_LOAD_LD if xlen == Xlen::X64 => "ld",
            Self::FUNCT3_LOAD_LBU => "lbu",
            Self::FUNCT3_LOAD_LHU => "lhu",
            Self::FUNCT3_LOAD_LWU if xlen == Xlen::X64 => "lwu",
            _ => return Err(DisasmError::DecodingError("Invalid load funct3".to_string())),
        };

        let offset = self.format_imm(imm_i);
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})", self.reg_name(rd), offset, self.reg_name(rs1)),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_memory_operand(rs1, imm_i),
            ],
        })
    }

    fn decode_store(&self, funct3: u8, rs2: u8, rs1: u8, imm_s: i64, xlen: Xlen) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_STORE_SB => "sb",
            Self::FUNCT3_STORE_SH => "sh",
            Self::FUNCT3_STORE_SW => "sw",
            Self::FUNCT3_STORE_SD if xlen == Xlen::X64 => "sd",
            _ => return Err(DisasmError::DecodingError("Invalid store funct3".to_string())),
        };
        self.decode_s_type(mnemonic, rs2, rs1, imm_s)
    }

    fn decode_op_imm(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, imm_i: i64, xlen: Xlen) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_i_type("addi", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLT => self.decode_i_type("slti", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLTU => self.decode_i_type("sltiu", rd, rs1, imm_i),
            Self::FUNCT3_OP_XOR => self.decode_i_type("xori", rd, rs1, imm_i),
            Self::FUNCT3_OP_OR => self.decode_i_type("ori", rd, rs1, imm_i),
            Self::FUNCT3_OP_AND => self.decode_i_type("andi", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    let shamt = self.extract_shamt(imm_i, xlen);
                    self.decode_i_type("slli", rd, rs1, shamt)
                } else {
                    Err(DisasmError::DecodingError("Invalid slli funct7".to_string()))
                }
            }
            Self::FUNCT3_OP_SRL_SRA => match funct7 {
                Self::FUNCT7_OP_SRL => {
                    let shamt = self.extract_shamt(imm_i, xlen);
                    self.decode_i_type("srli", rd, rs1, shamt)
                }
                Self::FUNCT7_OP_SRA => {
                    let shamt = self.extract_shamt(imm_i, xlen);
                    self.decode_i_type("srai", rd, rs1, shamt)
                }
                _ => Err(DisasmError::DecodingError("Invalid shift funct7".to_string())),
            },
            _ => Err(DisasmError::DecodingError("Invalid op-imm funct3".to_string())),
        }
    }

    fn decode_op(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        // Skip M-extension instructions (funct7 == 0b0000001)
        if funct7 == Self::FUNCT7_OP_MUL {
            return Err(DisasmError::DecodingError("M-extension instruction".to_string()));
        }

        match (funct3, funct7) {
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_ADD) => self.decode_r_type("add", rd, rs1, rs2),
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_SUB) => self.decode_r_type("sub", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLL, Self::FUNCT7_OP_ADD) => self.decode_r_type("sll", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLT, Self::FUNCT7_OP_ADD) => self.decode_r_type("slt", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLTU, Self::FUNCT7_OP_ADD) => self.decode_r_type("sltu", rd, rs1, rs2),
            (Self::FUNCT3_OP_XOR, Self::FUNCT7_OP_ADD) => self.decode_r_type("xor", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRL) => self.decode_r_type("srl", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRA) => self.decode_r_type("sra", rd, rs1, rs2),
            (Self::FUNCT3_OP_OR, Self::FUNCT7_OP_ADD) => self.decode_r_type("or", rd, rs1, rs2),
            (Self::FUNCT3_OP_AND, Self::FUNCT7_OP_ADD) => self.decode_r_type("and", rd, rs1, rs2),
            _ => Err(DisasmError::DecodingError("Invalid op instruction encoding".to_string())),
        }
    }

    fn decode_op_imm_32(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, imm_i: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_i_type("addiw", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    self.decode_i_type("slliw", rd, rs1, imm_i)
                } else {
                    Err(DisasmError::DecodingError("Invalid slliw funct7".to_string()))
                }
            }
            Self::FUNCT3_OP_SRL_SRA => match funct7 {
                Self::FUNCT7_OP_SRL => self.decode_i_type("srliw", rd, rs1, imm_i),
                Self::FUNCT7_OP_SRA => self.decode_i_type("sraiw", rd, rs1, imm_i),
                _ => Err(DisasmError::DecodingError("Invalid 32-bit shift funct7".to_string())),
            },
            _ => Err(DisasmError::DecodingError("Invalid op-imm32 funct3".to_string())),
        }
    }

    fn decode_op_32(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match (funct3, funct7) {
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_ADD) => self.decode_r_type("addw", rd, rs1, rs2),
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_SUB) => self.decode_r_type("subw", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLL, Self::FUNCT7_OP_ADD) => self.decode_r_type("sllw", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRL) => self.decode_r_type("srlw", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRA) => self.decode_r_type("sraw", rd, rs1, rs2),
            _ => Err(DisasmError::DecodingError("Invalid op-32 instruction encoding".to_string())),
        }
    }

    fn decode_misc_mem(&self, funct3: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_MISC_MEM_FENCE => Ok(RiscVDecodedInstruction {
                mnemonic: "fence".to_string(),
                operands: String::new(),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
            Self::FUNCT3_MISC_MEM_FENCE_I => Ok(RiscVDecodedInstruction {
                mnemonic: "fence.i".to_string(),
                operands: String::new(),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
            _ => Err(DisasmError::DecodingError("Invalid misc mem funct3".to_string())),
        }
    }

    fn decode_system(&self, funct3: u8, rd: u8, rs1: u8, _imm_i: i64, funct12: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_SYSTEM_PRIV => match funct12 {
                Self::FUNCT12_SYSTEM_ECALL => Ok(RiscVDecodedInstruction {
                    mnemonic: "ecall".to_string(),
                    operands: String::new(),
                    format: RiscVInstructionFormat::I,
                    size: 4,
                    operands_detail: vec![],
                }),
                Self::FUNCT12_SYSTEM_EBREAK => Ok(RiscVDecodedInstruction {
                    mnemonic: "ebreak".to_string(),
                    operands: String::new(),
                    format: RiscVInstructionFormat::I,
                    size: 4,
                    operands_detail: vec![],
                }),
                _ => self.decode_csr_instruction("csrrw", rd, rs1, funct12 as i64),
            },
            Self::FUNCT3_SYSTEM_CSRRW => self.decode_csr_instruction("csrrw", rd, rs1, funct12 as i64),
            Self::FUNCT3_SYSTEM_CSRRS => self.decode_csr_instruction("csrrs", rd, rs1, funct12 as i64),
            Self::FUNCT3_SYSTEM_CSRRC => self.decode_csr_instruction("csrrc", rd, rs1, funct12 as i64),
            Self::FUNCT3_SYSTEM_CSRRWI => {
                self.decode_csr_instruction_imm("csrrwi", rd, rs1 as i64, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRSI => {
                self.decode_csr_instruction_imm("csrrsi", rd, rs1 as i64, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRCI => {
                self.decode_csr_instruction_imm("csrrci", rd, rs1 as i64, funct12 as i64)
            }
            _ => Err(DisasmError::DecodingError("Invalid system funct3".to_string())),
        }
    }

    fn decode_csr_instruction(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        csr: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.format_csr(csr);
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}", self.reg_name(rd), csr_str, self.reg_name(rs1)),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_immediate_operand(csr),
                self.make_register_operand(rs1, Access::read()),
            ],
        })
    }

    fn decode_csr_instruction_imm(
        &self,
        mnemonic: &str,
        rd: u8,
        zimm: i64,
        csr: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.format_csr(csr);
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}", self.reg_name(rd), csr_str, zimm),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_immediate_operand(csr),
                self.make_immediate_operand(zimm),
            ],
        })
    }

    // Utility methods
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

    fn format_csr(&self, csr: i64) -> String {
        let csr_id = csr as u16;
        if let Some(name) = self.csr_name(csr_id) {
            name.to_string()
        } else {
            format!("0x{:x}", csr)
        }
    }

    fn csr_name(&self, csr: u16) -> Option<&'static str> {
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

    fn extract_shamt(&self, imm: i64, xlen: Xlen) -> i64 {
        let mask = match xlen {
            Xlen::X64 => 0x3f,
            Xlen::X32 => 0x1f,
        } as u64;
        (imm as u64 & mask) as i64
    }

    fn make_register_operand(&self, reg: u8, access: Access) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Register,
            access,
            value: RiscVOperandValue::Register(reg as u32),
        }
    }

    fn make_immediate_operand(&self, imm: i64) -> RiscVOperand {
        RiscVOperand {
            op_type: RiscVOperandType::Immediate,
            access: Access::read(),
            value: RiscVOperandValue::Immediate(imm),
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

impl InstructionExtension for RviExtension {
    fn name(&self) -> &'static str {
        "I"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // I extension is always enabled (bit 0)
        extensions & 0b001 != 0
    }

    fn try_decode_standard(
        &self,
        opcode: u32,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
        funct12: u32,
        imm_i: i64,
        imm_s: i64,
        imm_b: i64,
        imm_u: i64,
        imm_j: i64,
        xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>> {
        match opcode {
            Self::OPCODE_LUI => Some(self.decode_lui(rd, imm_u)),
            Self::OPCODE_AUIPC => Some(self.decode_auipc(rd, imm_u)),
            Self::OPCODE_JAL => Some(self.decode_jal(rd, imm_j)),
            Self::OPCODE_JALR => Some(self.decode_jalr(rd, rs1, imm_i)),
            Self::OPCODE_BRANCH => Some(self.decode_branch(funct3, rs1, rs2, imm_b)),
            Self::OPCODE_LOAD => Some(self.decode_load(funct3, rd, rs1, imm_i, xlen)),
            Self::OPCODE_STORE => Some(self.decode_store(funct3, rs2, rs1, imm_s, xlen)),
            Self::OPCODE_MISC_MEM => Some(self.decode_misc_mem(funct3)),
            Self::OPCODE_OP_IMM => Some(self.decode_op_imm(funct3, funct7, rd, rs1, imm_i, xlen)),
            Self::OPCODE_OP => {
                match self.decode_op(funct3, funct7, rd, rs1, rs2) {
                    Ok(inst) => Some(Ok(inst)),
                    Err(DisasmError::DecodingError(msg)) if msg.contains("M-extension") => None,
                    Err(e) => Some(Err(e)),
                }
            }
            Self::OPCODE_OP_IMM_32 if xlen == Xlen::X64 => {
                Some(self.decode_op_imm_32(funct3, funct7, rd, rs1, imm_i))
            }
            Self::OPCODE_OP_32 if xlen == Xlen::X64 => {
                Some(self.decode_op_32(funct3, funct7, rd, rs1, rs2))
            }
            Self::OPCODE_SYSTEM => Some(self.decode_system(funct3, rd, rs1, imm_i, funct12)),
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
        // RV32I/RV64I extension doesn't handle compressed instructions
        None
    }
}