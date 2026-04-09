//! RV32I/RV64I Base Integer Instruction Set Extension
//!
//! This module implements the RISC-V base integer instruction set (I extension),
//! which includes all the fundamental integer operations, control flow instructions,
//! memory operations, and system instructions that form the core of RISC-V.

use super::Standard;
use crate::ir::DecodedInstruction;
use crate::riscv::decoder::Xlen;
use crate::riscv::extensions::{
    Extensions, InstructionExtension, invalid_encoding, unsupported_mode,
};
use crate::riscv::shared::{
    InstructionFormatter, OperandFactory, encoding::ShamtExtractor,
    formatting::DefaultInstructionFormatter, operands::DefaultOperandFactory,
    registers::RegisterManager,
};
use crate::riscv::types::*;
use crate::types::error::DisasmError;

/// RV32I/RV64I Base Integer Extension
pub struct Rvi {
    operand_factory: DefaultOperandFactory,
    formatter: DefaultInstructionFormatter,
    register_manager: RegisterManager,
}

impl Rvi {
    /// Create a new RV32I/RV64I extension instance.
    pub fn new() -> Self {
        Self {
            operand_factory: DefaultOperandFactory::new(),
            formatter: DefaultInstructionFormatter::new(),
            register_manager: RegisterManager::new(),
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
    ) -> Result<DecodedInstruction, DisasmError> {
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::U,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(imm >> 12),
            ],
        ))
    }

    fn decode_j_type(
        &self,
        mnemonic: &str,
        rd: u8,
        imm: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::J,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(imm),
            ],
        ))
    }

    fn decode_i_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if mnemonic == "ori"
            && rd == 0
            && let Some(prefetch_mnemonic) = prefetch_mnemonic(imm)
        {
            return Ok(self.formatter.create_decoded_instruction(
                prefetch_mnemonic,
                RiscVInstructionFormat::I,
                4,
                vec![self.operand_factory.make_memory_operand(rs1, 0)],
            ));
        }

        let instruction = self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::I,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
                self.operand_factory.make_immediate_operand(imm),
            ],
        );

        if mnemonic == "addi" && rs1 == 0 && rd != 0 {
            Ok(instruction.with_capstone_alias("li", vec![1]))
        } else {
            Ok(instruction)
        }
    }

    fn decode_r_type(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<DecodedInstruction, DisasmError> {
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::R,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
                self.operand_factory
                    .make_register_operand(rs2, Access::read()),
            ],
        ))
    }

    fn decode_s_type(
        &self,
        mnemonic: &str,
        rs2: u8,
        rs1: u8,
        imm: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::S,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rs2, Access::read()),
                self.operand_factory.make_memory_operand(rs1, imm),
            ],
        ))
    }

    fn decode_b_type(
        &self,
        mnemonic: &str,
        rs1: u8,
        rs2: u8,
        imm: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        let instruction = self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::B,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
                self.operand_factory
                    .make_register_operand(rs2, Access::read()),
                self.operand_factory.make_immediate_operand(imm),
            ],
        );

        match mnemonic {
            "beq" if rs2 == 0 => Ok(instruction.with_capstone_alias("beqz", vec![1])),
            "bne" if rs2 == 0 => Ok(instruction.with_capstone_alias("bnez", vec![1])),
            _ => Ok(instruction),
        }
    }

    // Specific instruction type decoders
    fn decode_auipc(&self, rd: u8, imm_u: i64) -> Result<DecodedInstruction, DisasmError> {
        self.decode_u_type("auipc", rd, imm_u)
    }

    fn decode_lui(&self, rd: u8, imm_u: i64) -> Result<DecodedInstruction, DisasmError> {
        self.decode_u_type("lui", rd, imm_u)
    }

    fn decode_jal(
        &self,
        rd: u8,
        imm_j: i64,
        _xlen: Xlen,
    ) -> Result<DecodedInstruction, DisasmError> {
        let instruction = self.decode_j_type("jal", rd, imm_j)?;
        match rd {
            0 => Ok(instruction.with_capstone_alias("j", vec![0])),
            1 => Ok(instruction.with_hidden_operands(vec![0])),
            _ => Ok(instruction),
        }
    }

    fn decode_jalr(&self, rd: u8, rs1: u8, imm_i: i64) -> Result<DecodedInstruction, DisasmError> {
        let instruction = self.formatter.create_decoded_instruction(
            "jalr",
            RiscVInstructionFormat::I,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
                self.operand_factory.make_immediate_operand(imm_i),
            ],
        );

        if rd == 1 {
            Ok(instruction.with_hidden_operands(vec![0]))
        } else {
            Ok(instruction)
        }
    }

    fn decode_branch(
        &self,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm_b: i64,
        _xlen: Xlen,
    ) -> Result<DecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_BRANCH_BEQ => self.decode_b_type("beq", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BNE => self.decode_b_type("bne", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BLT => self.decode_b_type("blt", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BGE => self.decode_b_type("bge", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BLTU => self.decode_b_type("bltu", rs1, rs2, imm_b),
            Self::FUNCT3_BRANCH_BGEU => self.decode_b_type("bgeu", rs1, rs2, imm_b),
            _ => Err(invalid_encoding("invalid branch funct3")),
        }
    }

    fn decode_load(
        &self,
        funct3: u8,
        rd: u8,
        rs1: u8,
        imm_i: i64,
        xlen: Xlen,
    ) -> Result<DecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_LOAD_LB => "lb",
            Self::FUNCT3_LOAD_LH => "lh",
            Self::FUNCT3_LOAD_LW => "lw",
            Self::FUNCT3_LOAD_LD if xlen == Xlen::X64 => "ld",
            Self::FUNCT3_LOAD_LD => return Err(unsupported_mode("ld requires RV64")),
            Self::FUNCT3_LOAD_LBU => "lbu",
            Self::FUNCT3_LOAD_LHU => "lhu",
            Self::FUNCT3_LOAD_LWU if xlen == Xlen::X64 => "lwu",
            Self::FUNCT3_LOAD_LWU => return Err(unsupported_mode("lwu requires RV64")),
            _ => return Err(invalid_encoding("invalid load funct3")),
        };

        Ok(self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::I,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_memory_operand(rs1, imm_i),
            ],
        ))
    }

    fn decode_store(
        &self,
        funct3: u8,
        rs2: u8,
        rs1: u8,
        imm_s: i64,
        xlen: Xlen,
    ) -> Result<DecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            Self::FUNCT3_STORE_SB => "sb",
            Self::FUNCT3_STORE_SH => "sh",
            Self::FUNCT3_STORE_SW => "sw",
            Self::FUNCT3_STORE_SD if xlen == Xlen::X64 => "sd",
            Self::FUNCT3_STORE_SD => return Err(unsupported_mode("sd requires RV64")),
            _ => return Err(invalid_encoding("invalid store funct3")),
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
    ) -> Result<DecodedInstruction, DisasmError> {
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
                    Err(invalid_encoding("invalid slli funct7"))
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
                _ => Err(invalid_encoding("invalid shift funct7")),
            },
            _ => Err(invalid_encoding("invalid op-imm funct3")),
        }
    }

    fn decode_op(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<DecodedInstruction, DisasmError> {
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
            _ => Err(invalid_encoding("invalid op instruction encoding")),
        }
    }

    fn decode_op_imm_32(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        imm_i: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_OP_ADD_SUB => self.decode_i_type("addiw", rd, rs1, imm_i),
            Self::FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    self.decode_i_type("slliw", rd, rs1, imm_i)
                } else {
                    Err(invalid_encoding("invalid slliw funct7"))
                }
            }
            Self::FUNCT3_OP_SRL_SRA => match funct7 {
                Self::FUNCT7_OP_SRL => self.decode_i_type("srliw", rd, rs1, imm_i),
                Self::FUNCT7_OP_SRA => self.decode_i_type("sraiw", rd, rs1, imm_i),
                _ => Err(invalid_encoding("invalid 32-bit shift funct7")),
            },
            _ => Err(invalid_encoding("invalid op-imm32 funct3")),
        }
    }

    fn decode_op_32(
        &self,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<DecodedInstruction, DisasmError> {
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
            _ => Err(invalid_encoding("invalid op-32 instruction encoding")),
        }
    }

    fn decode_misc_mem(&self, funct3: u8, imm_i: i64) -> Result<DecodedInstruction, DisasmError> {
        match funct3 {
            Self::FUNCT3_MISC_MEM_FENCE => {
                let imm_bits = imm_i as u16;
                let predecessor = ((imm_bits >> 4) & 0xf) as i64;
                let successor = (imm_bits & 0xf) as i64;
                Ok(self
                    .formatter
                    .create_decoded_instruction(
                        "fence",
                        RiscVInstructionFormat::I,
                        4,
                        vec![
                            self.operand_factory.make_immediate_operand(predecessor),
                            self.operand_factory.make_immediate_operand(successor),
                        ],
                    )
                    .with_hidden_operands(vec![0, 1]))
            }
            Self::FUNCT3_MISC_MEM_FENCE_I => Ok(DefaultInstructionFormatter::simple_instruction(
                "fence.i", "",
            )),
            _ => Err(invalid_encoding("invalid misc mem funct3")),
        }
    }

    fn decode_system(
        &self,
        funct3: u8,
        rd: u8,
        rs1: u8,
        _imm_i: i64,
        funct12: u32,
    ) -> Result<DecodedInstruction, DisasmError> {
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
            _ => Err(invalid_encoding("invalid system funct3")),
        }
    }

    fn decode_csr_instruction(
        &self,
        mnemonic: &str,
        rd: u8,
        rs1: u8,
        csr: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        let (capstone_alias, hidden_operands) = csr_capstone_alias(mnemonic, rd, rs1, csr as u16);

        let instruction = self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::I,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr),
                self.operand_factory
                    .make_register_operand(rs1, Access::read()),
            ],
        );

        if let Some(capstone_alias) = capstone_alias {
            Ok(instruction.with_capstone_alias(capstone_alias, hidden_operands))
        } else {
            Ok(instruction)
        }
    }

    fn decode_csr_instruction_imm(
        &self,
        mnemonic: &str,
        rd: u8,
        zimm: i64,
        csr: i64,
    ) -> Result<DecodedInstruction, DisasmError> {
        let _ = &self.register_manager;
        let instruction = self.formatter.create_decoded_instruction(
            mnemonic,
            RiscVInstructionFormat::I,
            4,
            vec![
                self.operand_factory
                    .make_register_operand(rd, Access::write()),
                self.operand_factory.make_immediate_operand(csr),
                self.operand_factory.make_immediate_operand(zimm),
            ],
        );

        let capstone_alias = if rd == 0 {
            match mnemonic {
                "csrrwi" => Some("csrwi"),
                "csrrsi" => Some("csrsi"),
                "csrrci" => Some("csrci"),
                _ => None,
            }
        } else {
            None
        };

        if let Some(capstone_alias) = capstone_alias {
            Ok(instruction.with_capstone_alias(capstone_alias, vec![0]))
        } else {
            Ok(instruction)
        }
    }
}

fn prefetch_mnemonic(imm: i64) -> Option<&'static str> {
    match imm {
        0 => Some("prefetch.i"),
        1 => Some("prefetch.r"),
        3 => Some("prefetch.w"),
        _ => None,
    }
}

fn csr_capstone_alias(
    mnemonic: &str,
    rd: u8,
    rs1: u8,
    csr: u16,
) -> (Option<&'static str>, Vec<usize>) {
    match (mnemonic, rd, rs1, csr) {
        ("csrrs", _, 0, 0xC00) => (Some("rdcycle"), vec![1, 2]),
        ("csrrs", _, 0, 0xC01) => (Some("rdtime"), vec![1, 2]),
        ("csrrs", _, 0, 0xC02) => (Some("rdinstret"), vec![1, 2]),
        ("csrrs", _, 0, 0xC80) => (Some("rdcycleh"), vec![1, 2]),
        ("csrrs", _, 0, 0xC81) => (Some("rdtimeh"), vec![1, 2]),
        ("csrrs", _, 0, 0xC82) => (Some("rdinstreth"), vec![1, 2]),
        ("csrrs", _, 0, _) => (Some("csrr"), vec![2]),
        ("csrrw", 0, _, _) => (Some("csrw"), vec![0]),
        ("csrrs", 0, _, _) => (Some("csrs"), vec![0]),
        ("csrrc", 0, _, _) => (Some("csrc"), vec![0]),
        _ => (None, Vec::new()),
    }
}

impl InstructionExtension for Rvi {
    fn name(&self) -> &'static str {
        "I"
    }

    fn is_enabled(&self, extensions: &Extensions) -> bool {
        // I extension is always enabled (bit 0)
        extensions.standard.contains(Standard::I)
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
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        match opcode {
            Self::OPCODE_LUI => Some(self.decode_lui(rd, imm_u)),
            Self::OPCODE_AUIPC => Some(self.decode_auipc(rd, imm_u)),
            Self::OPCODE_JAL => Some(self.decode_jal(rd, imm_j, xlen)),
            Self::OPCODE_JALR => Some(self.decode_jalr(rd, rs1, imm_i)),
            Self::OPCODE_BRANCH => Some(self.decode_branch(funct3, rs1, rs2, imm_b, xlen)),
            Self::OPCODE_LOAD => Some(self.decode_load(funct3, rd, rs1, imm_i, xlen)),
            Self::OPCODE_STORE => Some(self.decode_store(funct3, rs2, rs1, imm_s, xlen)),
            Self::OPCODE_MISC_MEM => Some(self.decode_misc_mem(funct3, imm_i)),
            Self::OPCODE_OP_IMM => Some(self.decode_op_imm(funct3, funct7, rd, rs1, imm_i, xlen)),
            Self::OPCODE_OP if funct7 == Self::FUNCT7_OP_MUL => None,
            Self::OPCODE_OP => Some(self.decode_op(funct3, funct7, rd, rs1, rs2)),
            Self::OPCODE_OP_IMM_32 if xlen == Xlen::X64 => {
                Some(self.decode_op_imm_32(funct3, funct7, rd, rs1, imm_i))
            }
            Self::OPCODE_OP_IMM_32 => Some(Err(unsupported_mode("OP-IMM-32 requires RV64"))),
            Self::OPCODE_OP_32 if xlen != Xlen::X64 => {
                Some(Err(unsupported_mode("OP-32 requires RV64")))
            }
            Self::OPCODE_OP_32 if xlen == Xlen::X64 && funct7 == Self::FUNCT7_OP_MUL => None,
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
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        // RV32I/RV64I extension doesn't handle compressed instructions
        None
    }
}

impl Default for Rvi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::extensions::thead::THead;
    #[test]
    fn test_rvi_extension_creation() {
        let extension = Rvi::new();
        assert_eq!(extension.name(), "I");
        let exts_i = Extensions {
            standard: Standard::I,
            thead: THead::empty(),
        };
        let exts_m = Extensions {
            standard: Standard::M,
            thead: THead::empty(),
        };
        assert!(extension.is_enabled(&exts_i));
        assert!(!extension.is_enabled(&exts_m));
    }

    #[test]
    fn test_rvi_instruction_decoding() {
        let extension = Rvi::new();

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
        let extension = Rvi::new();

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
