//! RV32I/RV64I Base Integer Instruction Set Extension
//!
//! This module implements the RISC-V base integer instruction set (I extension),
//! which includes all the fundamental integer operations, control flow instructions,
//! memory operations, and system instructions that form the core of RISC-V.

use super::super::decoder::{RiscVDecodedInstruction, Xlen};
use super::super::shared::{
    InstructionFormatter, OperandFactory, RegisterNameProvider,
    encoding::ShamtExtractor,
    formatting::DefaultInstructionFormatter,
    operands::{DefaultOperandFactory, OperandBuilder, OperandFormatter},
    registers::RegisterManager,
};
use super::super::types::*;
use super::InstructionExtension;
use crate::error::DisasmError;
use crate::riscv::extensions::extension_masks;

/// RV32I/RV64I Base Integer Extension
pub struct RviExtension {
    operand_factory: DefaultOperandFactory,
    formatter: DefaultInstructionFormatter,
    register_manager: RegisterManager,
    operand_builder: OperandBuilder,
}

impl RviExtension {
    /// Create a new RV32I/RV64I extension instance.
    pub fn new() -> Self {
        Self {
            operand_factory: DefaultOperandFactory::new(),
            formatter: DefaultInstructionFormatter::new(),
            register_manager: RegisterManager::new(),
            operand_builder: OperandBuilder::new(),
        }
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

    // Instruction format decoding methods using shared utilities
    fn decode_u_type(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_u_type(mnemonic, rd, imm);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::write()),
            self.operand_factory.make_immediate_operand(imm >> 12),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::U,
            4,
            operands_detail,
        ))
    }

    fn decode_j_type(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_j_type(mnemonic, rd, imm);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::read_write()),
            self.operand_factory.make_immediate_operand(imm),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::J,
            4,
            operands_detail,
        ))
    }

    fn decode_i_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_i_type(mnemonic, rd, rs1, imm);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::write()),
            self.operand_factory
                .make_register_operand(rs1, Access::read()),
            self.operand_factory.make_immediate_operand(imm),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::I,
            4,
            operands_detail,
        ))
    }

    fn decode_r_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_r_type(mnemonic, rd, rs1, rs2);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::write()),
            self.operand_factory
                .make_register_operand(rs1, Access::read()),
            self.operand_factory
                .make_register_operand(rs2, Access::read()),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::R,
            4,
            operands_detail,
        ))
    }

    fn decode_s_type(
        &self,
        mnemonic: &str,
        rs2: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_s_type(mnemonic, rs2, rs1, imm);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rs2, Access::read()),
            self.operand_factory.make_memory_operand(rs1, imm),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::S,
            4,
            operands_detail,
        ))
    }

    fn decode_b_type(
        &self,
        mnemonic: &str,
        rs1: u8,
        rs2: u8,
        imm: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let operands = self.operand_builder.format_b_type(mnemonic, rs1, rs2, imm);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rs1, Access::read()),
            self.operand_factory
                .make_register_operand(rs2, Access::read()),
            self.operand_factory.make_immediate_operand(imm),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::B,
            4,
            operands_detail,
        ))
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

    fn decode_jalr(
        &self,
        rd: u8,
        rs1: u8,
        imm_i: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        self.decode_i_type("jalr", rd, rs1, imm_i)
    }

    fn decode_branch(
        &self,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm_b: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
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
            _ => Err(DisasmError::DecodingError(
                "Invalid branch funct3".to_string(),
            )),
        }
    }

    fn decode_load(
        &self,
        funct3: u8,
        rd: u8,
        rs1: u8,
        imm_i: i64,
        xlen: Xlen,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_LOAD_LB => "lb",
            Self::FUNCT3_LOAD_LH => "lh",
            Self::FUNCT3_LOAD_LW => "lw",
            Self::FUNCT3_LOAD_LD if xlen == Xlen::X64 => "ld",
            Self::FUNCT3_LOAD_LBU => "lbu",
            Self::FUNCT3_LOAD_LHU => "lhu",
            Self::FUNCT3_LOAD_LWU if xlen == Xlen::X64 => "lwu",
            _ => {
                return Err(DisasmError::DecodingError(
                    "Invalid load funct3".to_string(),
                ));
            }
        };

        let operands = self
            .operand_builder
            .format_load_type(mnemonic, rd, rs1, imm_i, false);
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::write()),
            self.operand_factory.make_memory_operand(rs1, imm_i),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::I,
            4,
            operands_detail,
        ))
    }

    fn decode_store(
        &self,
        funct3: u8,
        rs2: u8,
        rs1: u8,
        imm_s: i64,
        xlen: Xlen,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_STORE_SB => "sb",
            Self::FUNCT3_STORE_SH => "sh",
            Self::FUNCT3_STORE_SW => "sw",
            Self::FUNCT3_STORE_SD if xlen == Xlen::X64 => "sd",
            _ => {
                return Err(DisasmError::DecodingError(
                    "Invalid store funct3".to_string(),
                ));
            }
        };
        self.decode_s_type(mnemonic, rs2, rs1, imm_s)
    }

    fn decode_op_imm(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        imm_i: i64,
        xlen: Xlen,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_i_type("addi", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLT => self.decode_i_type("slti", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLTU => self.decode_i_type("sltiu", rd, rs1, imm_i),
            Self::FUNCT3_OP_XOR => self.decode_i_type("xori", rd, rs1, imm_i),
            Self::FUNCT3_OP_OR => self.decode_i_type("ori", rd, rs1, imm_i),
            Self::FUNCT3_OP_AND => self.decode_i_type("andi", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    let shamt = ShamtExtractor::extract_shamt(imm_i, xlen);
                    self.decode_i_type("slli", rd, rs1, shamt)
                } else {
                    Err(DisasmError::DecodingError(
                        "Invalid slli funct7".to_string(),
                    ))
                }
            }
            Self::FUNCT3_OP_SRL_SRA => match funct7 {
                Self::FUNCT7_OP_SRL => {
                    let shamt = ShamtExtractor::extract_shamt(imm_i, xlen);
                    self.decode_i_type("srli", rd, rs1, shamt)
                }
                Self::FUNCT7_OP_SRA => {
                    let shamt = ShamtExtractor::extract_shamt(imm_i, xlen);
                    self.decode_i_type("srai", rd, rs1, shamt)
                }
                _ => Err(DisasmError::DecodingError(
                    "Invalid shift funct7".to_string(),
                )),
            },
            _ => Err(DisasmError::DecodingError(
                "Invalid op-imm funct3".to_string(),
            )),
        }
    }

    fn decode_op(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        // Skip M-extension instructions (funct7 == 0b0000001)
        if funct7 == Self::FUNCT7_OP_MUL {
            return Err(DisasmError::DecodingError(
                "M-extension instruction".to_string(),
            ));
        }

        match (funct3, funct7) {
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_ADD) => {
                self.decode_r_type("add", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_SUB) => {
                self.decode_r_type("sub", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_SLL, Self::FUNCT7_OP_ADD) => self.decode_r_type("sll", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLT, Self::FUNCT7_OP_ADD) => self.decode_r_type("slt", rd, rs1, rs2),
            (Self::FUNCT3_OP_SLTU, Self::FUNCT7_OP_ADD) => self.decode_r_type("sltu", rd, rs1, rs2),
            (Self::FUNCT3_OP_XOR, Self::FUNCT7_OP_ADD) => self.decode_r_type("xor", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRL) => {
                self.decode_r_type("srl", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRA) => {
                self.decode_r_type("sra", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_OR, Self::FUNCT7_OP_ADD) => self.decode_r_type("or", rd, rs1, rs2),
            (Self::FUNCT3_OP_AND, Self::FUNCT7_OP_ADD) => self.decode_r_type("and", rd, rs1, rs2),
            _ => Err(DisasmError::DecodingError(
                "Invalid op instruction encoding".to_string(),
            )),
        }
    }

    fn decode_op_imm_32(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        imm_i: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_i_type("addiw", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    self.decode_i_type("slliw", rd, rs1, imm_i)
                } else {
                    Err(DisasmError::DecodingError(
                        "Invalid slliw funct7".to_string(),
                    ))
                }
            }
            Self::FUNCT3_OP_SRL_SRA => match funct7 {
                Self::FUNCT7_OP_SRL => self.decode_i_type("srliw", rd, rs1, imm_i),
                Self::FUNCT7_OP_SRA => self.decode_i_type("sraiw", rd, rs1, imm_i),
                _ => Err(DisasmError::DecodingError(
                    "Invalid 32-bit shift funct7".to_string(),
                )),
            },
            _ => Err(DisasmError::DecodingError(
                "Invalid op-imm32 funct3".to_string(),
            )),
        }
    }

    fn decode_op_32(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        match (funct3, funct7) {
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_ADD) => {
                self.decode_r_type("addw", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_ADD_SUB, Self::FUNCT7_OP_SUB) => {
                self.decode_r_type("subw", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_SLL, Self::FUNCT7_OP_ADD) => self.decode_r_type("sllw", rd, rs1, rs2),
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRL) => {
                self.decode_r_type("srlw", rd, rs1, rs2)
            }
            (Self::FUNCT3_OP_SRL_SRA, Self::FUNCT7_OP_SRA) => {
                self.decode_r_type("sraw", rd, rs1, rs2)
            }
            _ => Err(DisasmError::DecodingError(
                "Invalid op-32 instruction encoding".to_string(),
            )),
        }
    }

    fn decode_misc_mem(&self, funct3: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_MISC_MEM_FENCE => {
                Ok(DefaultInstructionFormatter::simple_instruction("fence", ""))
            }
            Self::FUNCT3_MISC_MEM_FENCE_I => Ok(DefaultInstructionFormatter::simple_instruction(
                "fence.i", "",
            )),
            _ => Err(DisasmError::DecodingError(
                "Invalid misc mem funct3".to_string(),
            )),
        }
    }

    fn decode_system(
        &self,
        funct3: u8,
        rd: u8,
        rs1: u8,
        _imm_i: i64,
        funct12: u32,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_SYSTEM_PRIV => match funct12 {
                Self::FUNCT12_SYSTEM_ECALL => {
                    Ok(DefaultInstructionFormatter::simple_instruction("ecall", ""))
                }
                Self::FUNCT12_SYSTEM_EBREAK => Ok(DefaultInstructionFormatter::simple_instruction(
                    "ebreak", "",
                )),
                _ => self.decode_csr_instruction("csrrw", rd, rs1, funct12 as i64),
            },
            Self::FUNCT3_SYSTEM_CSRRW => {
                self.decode_csr_instruction("csrrw", rd, rs1, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRS => {
                self.decode_csr_instruction("csrrs", rd, rs1, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRC => {
                self.decode_csr_instruction("csrrc", rd, rs1, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRWI => {
                self.decode_csr_instruction_imm("csrrwi", rd, rs1 as i64, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRSI => {
                self.decode_csr_instruction_imm("csrrsi", rd, rs1 as i64, funct12 as i64)
            }
            Self::FUNCT3_SYSTEM_CSRRCI => {
                self.decode_csr_instruction_imm("csrrci", rd, rs1 as i64, funct12 as i64)
            }
            _ => Err(DisasmError::DecodingError(
                "Invalid system funct3".to_string(),
            )),
        }
    }

    fn decode_csr_instruction(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        csr: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.operand_factory.format_csr(csr);

        // Handle pseudo-instructions: csrr, csrc, csrw
        // csrrs with rs1=0 → csrr
        // csrrc with rs1=0 → csrc
        // csrrw with rs1=0 → csrw
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
                self.operand_factory.make_immediate_operand(csr),
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
                self.operand_factory.make_immediate_operand(csr),
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

    fn decode_csr_instruction_imm(
        &self,
        mnemonic: &str,
        rd: u8,
        zimm: i64,
        csr: i64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let csr_str = self.operand_factory.format_csr(csr);
        let operands = format!(
            "{}, {}, {}",
            self.register_manager.int_register_name(rd),
            csr_str,
            zimm
        );
        let operands_detail = vec![
            self.operand_factory
                .make_register_operand(rd, Access::write()),
            self.operand_factory.make_immediate_operand(csr),
            self.operand_factory.make_immediate_operand(zimm),
        ];
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            operands,
            RiscVInstructionFormat::I,
            4,
            operands_detail,
        ))
    }
}

impl InstructionExtension for RviExtension {
    fn name(&self) -> &'static str {
        "I"
    }

    fn is_enabled(&self, extensions: u32) -> bool {
        // I extension is always enabled (bit 0)
        extensions & extension_masks::I != 0
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
            Self::OPCODE_OP => match self.decode_op(funct3, funct7, rd, rs1, rs2) {
                Ok(inst) => Some(Ok(inst)),
                Err(DisasmError::DecodingError(msg)) if msg.contains("M-extension") => None,
                Err(e) => Some(Err(e)),
            },
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

impl Default for RviExtension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rvi_extension_creation() {
        let extension = RviExtension::new();
        assert_eq!(extension.name(), "I");
        assert!(extension.is_enabled(0b001));
        assert!(!extension.is_enabled(0b010));
    }

    #[test]
    fn test_rvi_instruction_decoding() {
        let extension = RviExtension::new();

        // Test ADDI x1, x2, 10 -> 0x00000513
        let result = extension.try_decode_standard(
            0b0010011, // opcode
            0b000,     // funct3
            0b0000000, // funct7
            1,         // rd
            2,         // rs1
            10,        // rs2
            0,         // funct12
            10,        // imm_i
            0,         // imm_s
            0,         // imm_b
            0,         // imm_u
            0,         // imm_j
            Xlen::X32,
        );

        assert!(result.is_some());
        let instruction = result.unwrap().unwrap();
        assert_eq!(instruction.mnemonic, "addi");
    }

    #[test]
    fn test_rvi_compressed_instructions() {
        let extension = RviExtension::new();

        // RVI extension shouldn't handle compressed instructions
        let result = extension.try_decode_compressed(
            0x0001,
            0b01,
            0b000,
            Xlen::X32,
            1,
            2,
            3,
            0,
            1,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        );
        assert!(result.is_none());
    }
}
