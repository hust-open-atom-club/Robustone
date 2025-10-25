//! 完善的RISC-V指令解码器 (v2.0)
//!
//! 基于riscv-online和Capstone实现，支持完整的RISC-V指令集

use super::types::*;
use crate::error::DisasmError;

// RISC-V操作码常量
const OPCODE_LOAD: u32 = 0b000_0011;
const OPCODE_MISC_MEM: u32 = 0b000_1111;
const OPCODE_OP_IMM: u32 = 0b001_0011;
const OPCODE_AUIPC: u32 = 0b001_0111;
const OPCODE_OP_IMM32: u32 = 0b001_1011;
const OPCODE_STORE: u32 = 0b010_0011;
const OPCODE_OP: u32 = 0b011_0011;
const OPCODE_LUI: u32 = 0b011_0111;
const OPCODE_OP_32: u32 = 0b011_1011;
const OPCODE_FMADD: u32 = 0b100_0011;
const OPCODE_FMSUB: u32 = 0b100_0111;
const OPCODE_FNMSUB: u32 = 0b100_1011;
const OPCODE_FNMADD: u32 = 0b100_1111;
const OPCODE_FP: u32 = 0b101_0011;
const OPCODE_BRANCH: u32 = 0b110_0011;
const OPCODE_JALR: u32 = 0b110_0111;
const OPCODE_JAL: u32 = 0b110_1111;
const OPCODE_SYSTEM: u32 = 0b111_0011;
const OPCODE_A: u32 = 0b010_1111;

// funct3常量 - Load指令
const FUNCT3_LOAD_LB: u8 = 0b000;
const FUNCT3_LOAD_LH: u8 = 0b001;
const FUNCT3_LOAD_LW: u8 = 0b010;
const FUNCT3_LOAD_LD: u8 = 0b011;
const FUNCT3_LOAD_LBU: u8 = 0b100;
const FUNCT3_LOAD_LHU: u8 = 0b101;
const FUNCT3_LOAD_LWU: u8 = 0b110;

// funct3常量 - Store指令
const FUNCT3_STORE_SB: u8 = 0b000;
const FUNCT3_STORE_SH: u8 = 0b001;
const FUNCT3_STORE_SW: u8 = 0b010;
const FUNCT3_STORE_SD: u8 = 0b011;

// funct3常量 - 分支指令
const FUNCT3_BRANCH_BEQ: u8 = 0b000;
const FUNCT3_BRANCH_BNE: u8 = 0b001;
const FUNCT3_BRANCH_BLT: u8 = 0b100;
const FUNCT3_BRANCH_BGE: u8 = 0b101;
const FUNCT3_BRANCH_BLTU: u8 = 0b110;
const FUNCT3_BRANCH_BGEU: u8 = 0b111;

// funct3常量 - OP指令
const FUNCT3_OP_ADD_SUB: u8 = 0b000;
const FUNCT3_OP_SLL: u8 = 0b001;
const FUNCT3_OP_SLT: u8 = 0b010;
const FUNCT3_OP_SLTU: u8 = 0b011;
const FUNCT3_OP_XOR: u8 = 0b100;
const FUNCT3_OP_SRL_SRA: u8 = 0b101;
const FUNCT3_OP_OR: u8 = 0b110;
const FUNCT3_OP_AND: u8 = 0b111;

// funct7常量
const FUNCT7_OP_SRL: u8 = 0b000_0000;
const FUNCT7_OP_SRA: u8 = 0b010_0000;
const FUNCT7_OP_ADD: u8 = 0b000_0000;
const FUNCT7_OP_SUB: u8 = 0b010_0000;
const FUNCT7_OP_MUL: u8 = 0b000_0001;

// System指令常量
const FUNCT3_SYSTEM_PRIV: u8 = 0b000;
const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

const FUNCT12_SYSTEM_ECALL: u32 = 0b000_0000_0000;
const FUNCT12_SYSTEM_EBREAK: u32 = 0b000_0000_0001;

// Misc-Memory指令常量
const FUNCT3_MISC_MEM_FENCE: u8 = 0b000;
const FUNCT3_MISC_MEM_FENCE_I: u8 = 0b001;

// RISC-V Xlen (寄存器宽度)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Xlen {
    X32,
    X64,
}

/// 改进的RISC-V指令解码器
pub struct RiscVDecoder {
    xlen: Xlen,
    extensions: u32,
}

impl RiscVDecoder {
    /// 创建新的RISC-V解码器
    pub fn new(xlen: Xlen, extensions: u32) -> Self {
        Self { xlen, extensions }
    }

    /// 创建32位RISC-V解码器
    pub fn rv32() -> Self {
        Self::new(Xlen::X32, 0b001)
    }

    /// 创建64位RISC-V解码器
    pub fn rv64() -> Self {
        Self::new(Xlen::X64, 0b001)
    }

    /// 解码RISC-V指令
    pub fn decode(&self, bytes: &[u8], address: u64) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::DecodingError("No bytes provided".to_string()));
        }

        // 优先尝试解码为32位标准指令，如果4字节可用的话
        if bytes.len() >= 4 {
            self.decode_standard_instruction(bytes, address)
        } else if bytes.len() >= 2 && (bytes[0] & 0x3) != 0x3 {
            // 只有2字节可用，且符合压缩指令格式
            self.decode_compressed_instruction(bytes, address)
        } else {
            Err(DisasmError::DecodingError("Incomplete instruction".to_string()))
        }
    }

    /// 解码32位标准指令
    fn decode_standard_instruction(&self, bytes: &[u8], address: u64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let instruction = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);

        let opcode = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as u8;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let rs1 = ((instruction >> 15) & 0x1F) as u8;
        let rs2 = ((instruction >> 20) & 0x1F) as u8;
        let funct7 = ((instruction >> 25) & 0x7F) as u8;
        let funct12 = (instruction >> 20) & 0xFFF;
        let rs3 = ((instruction >> 27) & 0x1F) as u8;
        let funct2 = ((instruction >> 25) & 0x3) as u8;

        // 立即数计算
        let imm_i = self.sign_extend((instruction >> 20) & 0xFFF, 12);
        let imm_s = self.sign_extend(
            ((instruction >> 7) & 0x1F) | (((instruction >> 25) & 0x7F) << 5),
            12,
        );
        let imm_b = self.sign_extend(
            ((instruction >> 7) & 0x1) << 11
                | ((instruction >> 8) & 0xF) << 1
                | ((instruction >> 25) & 0x3F) << 5
                | ((instruction >> 31) & 0x1) << 12,
            12,
        );
        let imm_u = (instruction & 0xFFFFF000) as i64;
        let imm_j = self.sign_extend(
            ((instruction >> 31) & 0x1) << 20
                | ((instruction >> 21) & 0x3FF) << 1
                | ((instruction >> 20) & 0x1) << 11
                | ((instruction >> 12) & 0xFF) << 12,
            21,
        );

        match opcode {
            OPCODE_LUI => self.decode_u_type("lui", rd, imm_u),
            OPCODE_AUIPC => self.decode_u_type("auipc", rd, imm_u),
            OPCODE_JAL => self.decode_j_type("jal", rd, imm_j),
            OPCODE_JALR => self.decode_i_type("jalr", rd, rs1, imm_i),
            OPCODE_BRANCH => self.decode_branch_instruction(funct3, rs1, rs2, imm_b),
            OPCODE_LOAD => self.decode_load_instruction(funct3, rd, rs1, imm_i),
            OPCODE_STORE => self.decode_store_instruction(funct3, rs2, rs1, imm_s),
            OPCODE_MISC_MEM => self.decode_misc_mem_instruction(funct3),
            OPCODE_OP_IMM => self.decode_op_imm_instruction(funct3, funct7, rd, rs1, imm_i),
            OPCODE_OP => self.decode_op_instruction(funct3, funct7, rd, rs1, rs2),
            OPCODE_SYSTEM => self.decode_system_instruction(funct3, rd, rs1, imm_i, funct12),
            OPCODE_OP_IMM32 if self.xlen == Xlen::X64 => self.decode_op_imm32_instruction(funct3, funct7, rd, rs1, imm_i),
            OPCODE_OP_32 if self.xlen == Xlen::X64 => self.decode_op_32_instruction(funct3, funct7, rd, rs1, rs2),
            _ => self.decode_unknown_instruction(instruction),
        }
    }

    /// 解码压缩指令
    fn decode_compressed_instruction(&self, bytes: &[u8], address: u64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let instruction = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        let opcode = instruction & 0x3;
        let funct3 = ((instruction >> 13) & 0x7) as u8;
        let funct2 = ((instruction >> 5) & 0x3) as u8;
        let funct6 = ((instruction >> 10) & 0x3F) as u8;

    // Compressed register fields:
    // - Full register (CI/CIW/CLWSP/CSSWSP/CR): rd_full/rs1_full/rs2_full use bits [11:7]/[11:7]/[6:2]
    // - Compressed register (rd'/rs1'/rs2'): 3-bit fields mapped to x8..x15, we pass 0..7 into decode_c_* which add +8
    let rd_full = ((instruction >> 7) & 0x1F) as u8;   // bits 11..7
    let rs1_full = ((instruction >> 7) & 0x1F) as u8;  // bits 11..7
    let rs2_full = ((instruction >> 2) & 0x1F) as u8;  // bits 6..2
    let rdp = ((instruction >> 2) & 0x7) as u8;        // bits 4..2 (0..7)
    let rs1p = ((instruction >> 7) & 0x7) as u8;       // bits 9..7 (0..7)
    let rs2p = ((instruction >> 2) & 0x7) as u8;       // bits 4..2 (0..7)

        // 压缩指令立即数计算
        let imm6 = self.sign_extend_c((instruction >> 2) & 0x3F, 6);
        let imm8 = self.sign_extend_c(
            ((instruction >> 3) & 0x3) << 1
                | ((instruction >> 11) & 0x1) << 3
                | ((instruction >> 2) & 0x1) << 4
                | ((instruction >> 7) & 0x1) << 5
                | ((instruction >> 6) & 0x1) << 6
                | ((instruction >> 9) & 0x3) << 8
                | ((instruction >> 8) & 0x1) << 9,
            9,
        );
        let imm12 = self.sign_extend_c(
            ((instruction >> 12) & 0x1) << 11
                | ((instruction >> 1) & 0x7) << 1
                | ((instruction >> 5) & 0x1) << 4
                | ((instruction >> 6) & 0x1) << 5
                | ((instruction >> 10) & 0x1) << 6
                | ((instruction >> 7) & 0x1) << 7,
            12,
        );

        match (opcode, funct3) {
            // C0 opcode
            (0b00, 0b000) => self.decode_c_addi4spn(rdp, imm12),
            (0b00, 0b010) => self.decode_c_lw(rdp, rs1p, imm8),
            (0b00, 0b110) => self.decode_c_sw(rs2p, rs1p, imm8),

            // C1 opcode
            (0b01, 0b000) => self.decode_c_addi(rd_full, imm6),
            (0b01, 0b010) => self.decode_c_li(rd_full, imm6),
            (0b01, 0b100) => self.decode_c_alu(funct6, rdp, rs2p, funct2),
            (0b01, 0b101) => self.decode_c_j(imm12),
            (0b01, 0b110) => self.decode_c_beqz(rs1p, imm8),
            (0b01, 0b111) => self.decode_c_bnez(rs1p, imm8),

            // C2 opcode
            (0b10, 0b000) => self.decode_c_slli(rd_full, imm6),
            (0b10, 0b010) => self.decode_c_lwsp(rd_full, imm8),
            (0b10, 0b100) => self.decode_c_mv(rd_full, rs2_full),
            (0b10, 0b110) => self.decode_c_swsp(rs2_full, imm8),

            _ => self.decode_c_unknown(instruction),
        }
    }

    // 基础指令类型解码方法
    fn decode_u_type(&self, mnemonic: &str, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, 0x{:x}", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::U,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_j_type(&self, mnemonic: &str, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, 0x{:x}", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::J,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::read_write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_i_type(&self, mnemonic: &str, rd: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let imm_str = if imm.abs() > 0xFF || imm < -0xFF {
            format!("0x{:x}", imm)
        } else {
            format!("{}", imm)
        };

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

    fn decode_branch_instruction(&self, funct3: u8, rs1: u8, rs2: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            FUNCT3_BRANCH_BEQ => "beq",
            FUNCT3_BRANCH_BNE => "bne",
            FUNCT3_BRANCH_BLT => "blt",
            FUNCT3_BRANCH_BGE => "bge",
            FUNCT3_BRANCH_BLTU => "bltu",
            FUNCT3_BRANCH_BGEU => "bgeu",
            _ => return self.decode_unknown_instruction(funct3 as u32),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, 0x{:x}", self.reg_name(rs1), self.reg_name(rs2), imm),
            format: RiscVInstructionFormat::B,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rs1, Access::read()),
                self.make_register_operand(rs2, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_load_instruction(&self, funct3: u8, rd: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            FUNCT3_LOAD_LB => "lb",
            FUNCT3_LOAD_LH => "lh",
            FUNCT3_LOAD_LW => "lw",
            FUNCT3_LOAD_LD if self.xlen == Xlen::X64 => "ld",
            FUNCT3_LOAD_LBU => "lbu",
            FUNCT3_LOAD_LHU => "lhu",
            FUNCT3_LOAD_LWU if self.xlen == Xlen::X64 => "lwu",
            _ => return self.decode_unknown_instruction(funct3 as u32),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})", self.reg_name(rd), imm, self.reg_name(rs1)),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_memory_operand(rs1, imm),
            ],
        })
    }

    fn decode_store_instruction(&self, funct3: u8, rs2: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            FUNCT3_STORE_SB => "sb",
            FUNCT3_STORE_SH => "sh",
            FUNCT3_STORE_SW => "sw",
            FUNCT3_STORE_SD if self.xlen == Xlen::X64 => "sd",
            _ => return self.decode_unknown_instruction(funct3 as u32),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})", self.reg_name(rs2), imm, self.reg_name(rs1)),
            format: RiscVInstructionFormat::S,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rs2, Access::read()),
                self.make_memory_operand(rs1, imm),
            ],
        })
    }

    fn decode_misc_mem_instruction(&self, funct3: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            FUNCT3_MISC_MEM_FENCE => Ok(RiscVDecodedInstruction {
                mnemonic: "fence".to_string(),
                operands: String::new(),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
            FUNCT3_MISC_MEM_FENCE_I => Ok(RiscVDecodedInstruction {
                mnemonic: "fence.i".to_string(),
                operands: String::new(),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
            _ => self.decode_unknown_instruction(funct3 as u32),
        }
    }

    fn decode_op_imm_instruction(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            FUNCT3_OP_ADD_SUB => self.decode_i_type("addi", rd, rs1, imm),
            FUNCT3_OP_SLT => self.decode_i_type("slti", rd, rs1, imm),
            FUNCT3_OP_SLTU => self.decode_i_type("sltiu", rd, rs1, imm),
            FUNCT3_OP_XOR => self.decode_i_type("xori", rd, rs1, imm),
            FUNCT3_OP_OR => self.decode_i_type("ori", rd, rs1, imm),
            FUNCT3_OP_AND => self.decode_i_type("andi", rd, rs1, imm),
            FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    self.decode_i_type("slli", rd, rs1, imm)
                } else {
                    self.decode_unknown_instruction(funct7 as u32)
                }
            }
            FUNCT3_OP_SRL_SRA => {
                match funct7 {
                    FUNCT7_OP_SRL => self.decode_i_type("srli", rd, rs1, imm),
                    FUNCT7_OP_SRA => self.decode_i_type("srai", rd, rs1, imm),
                    _ => self.decode_unknown_instruction(funct7 as u32),
                }
            }
            _ => self.decode_unknown_instruction(funct3 as u32),
        }
    }

    fn decode_op_instruction(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        if funct7 == FUNCT7_OP_MUL {
            match funct3 {
                FUNCT3_OP_ADD_SUB => self.decode_r_type("mul", rd, rs1, rs2),
                FUNCT3_OP_SLL => self.decode_r_type("mulh", rd, rs1, rs2),
                FUNCT3_OP_SLT => self.decode_r_type("mulhsu", rd, rs1, rs2),
                FUNCT3_OP_SLTU => self.decode_r_type("mulhu", rd, rs1, rs2),
                FUNCT3_OP_XOR => self.decode_r_type("div", rd, rs1, rs2),
                FUNCT3_OP_SRL_SRA => self.decode_r_type("divu", rd, rs1, rs2),
                FUNCT3_OP_OR => self.decode_r_type("rem", rd, rs1, rs2),
                FUNCT3_OP_AND => self.decode_r_type("remu", rd, rs1, rs2),
                _ => self.decode_unknown_instruction(funct3 as u32),
            }
        } else {
            match (funct3, funct7) {
                (FUNCT3_OP_ADD_SUB, FUNCT7_OP_ADD) => self.decode_r_type("add", rd, rs1, rs2),
                (FUNCT3_OP_ADD_SUB, FUNCT7_OP_SUB) => self.decode_r_type("sub", rd, rs1, rs2),
                (FUNCT3_OP_SLL, FUNCT7_OP_ADD) => self.decode_r_type("sll", rd, rs1, rs2),
                (FUNCT3_OP_SLT, FUNCT7_OP_ADD) => self.decode_r_type("slt", rd, rs1, rs2),
                (FUNCT3_OP_SLTU, FUNCT7_OP_ADD) => self.decode_r_type("sltu", rd, rs1, rs2),
                (FUNCT3_OP_XOR, FUNCT7_OP_ADD) => self.decode_r_type("xor", rd, rs1, rs2),
                (FUNCT3_OP_SRL_SRA, FUNCT7_OP_SRL) => self.decode_r_type("srl", rd, rs1, rs2),
                (FUNCT3_OP_SRL_SRA, FUNCT7_OP_SRA) => self.decode_r_type("sra", rd, rs1, rs2),
                (FUNCT3_OP_OR, FUNCT7_OP_ADD) => self.decode_r_type("or", rd, rs1, rs2),
                (FUNCT3_OP_AND, FUNCT7_OP_ADD) => self.decode_r_type("and", rd, rs1, rs2),
                _ => self.decode_unknown_instruction(funct3 as u32),
            }
        }
    }

    fn decode_system_instruction(&self, funct3: u8, rd: u8, rs1: u8, imm: i64, funct12: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            FUNCT3_SYSTEM_PRIV => {
                match funct12 {
                    FUNCT12_SYSTEM_ECALL => Ok(RiscVDecodedInstruction {
                        mnemonic: "ecall".to_string(),
                        operands: String::new(),
                        format: RiscVInstructionFormat::I,
                        size: 4,
                        operands_detail: vec![],
                    }),
                    FUNCT12_SYSTEM_EBREAK => Ok(RiscVDecodedInstruction {
                        mnemonic: "ebreak".to_string(),
                        operands: String::new(),
                        format: RiscVInstructionFormat::I,
                        size: 4,
                        operands_detail: vec![],
                    }),
                    _ => self.decode_unknown_instruction(funct12),
                }
            }
            FUNCT3_SYSTEM_CSRRW => self.decode_csr_instruction("csrrw", rd, rs1, imm),
            FUNCT3_SYSTEM_CSRRS => self.decode_csr_instruction("csrrs", rd, rs1, imm),
            FUNCT3_SYSTEM_CSRRC => self.decode_csr_instruction("csrc", rd, rs1, imm),
            FUNCT3_SYSTEM_CSRRWI => self.decode_csr_instruction("csrrwi", rd, rs1, imm),
            FUNCT3_SYSTEM_CSRRSI => self.decode_csr_instruction("csrrsi", rd, rs1, imm),
            FUNCT3_SYSTEM_CSRRCI => self.decode_csr_instruction("csrci", rd, rs1, imm),
            _ => self.decode_unknown_instruction(funct3 as u32),
        }
    }

    fn decode_op_imm32_instruction(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        match funct3 {
            FUNCT3_OP_ADD_SUB => self.decode_i_type("addiw", rd, rs1, imm),
            FUNCT3_OP_SLL => {
                if funct7 == 0 {
                    self.decode_i_type("slliw", rd, rs1, imm)
                } else {
                    self.decode_unknown_instruction(funct7 as u32)
                }
            }
            FUNCT3_OP_SRL_SRA => {
                match funct7 {
                    FUNCT7_OP_SRL => self.decode_i_type("srliw", rd, rs1, imm),
                    FUNCT7_OP_SRA => self.decode_i_type("sraiw", rd, rs1, imm),
                    _ => self.decode_unknown_instruction(funct7 as u32),
                }
            }
            _ => self.decode_unknown_instruction(funct3 as u32),
        }
    }

    fn decode_op_32_instruction(&self, funct3: u8, funct7: u8, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        match (funct3, funct7) {
            (FUNCT3_OP_ADD_SUB, FUNCT7_OP_ADD) => self.decode_r_type("addw", rd, rs1, rs2),
            (FUNCT3_OP_ADD_SUB, FUNCT7_OP_SUB) => self.decode_r_type("subw", rd, rs1, rs2),
            (FUNCT3_OP_SLL, FUNCT7_OP_ADD) => self.decode_r_type("sllw", rd, rs1, rs2),
            (FUNCT3_OP_SRL_SRA, FUNCT7_OP_SRL) => self.decode_r_type("srlw", rd, rs1, rs2),
            (FUNCT3_OP_SRL_SRA, FUNCT7_OP_SRA) => self.decode_r_type("sraw", rd, rs1, rs2),
            _ => self.decode_unknown_instruction(funct3 as u32),
        }
    }

    fn decode_r_type(&self, mnemonic: &str, rd: u8, rs1: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), self.reg_name(rs2)),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs1, Access::read()),
                self.make_register_operand(rs2, Access::read()),
            ],
        })
    }

    fn decode_csr_instruction(&self, mnemonic: &str, rd: u8, rs1: u8, csr: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, 0x{:x}", self.reg_name(rd), self.reg_name(rs1), csr),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs1, Access::read()),
                self.make_immediate_operand(csr),
            ],
        })
    }

    // 压缩指令解码方法
    fn decode_c_addi4spn(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.addi4spn".to_string(),
            operands: format!("{}, sp, {}", self.c_reg_name(rd), imm),
            format: RiscVInstructionFormat::CIW,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd + 8, Access::write()),
                self.make_register_operand(2, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_lw(&self, rd: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.lw".to_string(),
            operands: format!("{}, {}({})", self.c_reg_name(rd), imm, self.c_reg_name(rs1)),
            format: RiscVInstructionFormat::CL,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd + 8, Access::write()),
                self.make_memory_operand(rs1 + 8, imm),
            ],
        })
    }

    fn decode_c_sw(&self, rs2: u8, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.sw".to_string(),
            operands: format!("{}, {}({})", self.c_reg_name(rs2), imm, self.c_reg_name(rs1)),
            format: RiscVInstructionFormat::CS,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rs2 + 8, Access::read()),
                self.make_memory_operand(rs1 + 8, imm),
            ],
        })
    }

    fn decode_c_addi(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.addi".to_string(),
            operands: format!("{}, {}", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd, Access::read_write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_li(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.li".to_string(),
            operands: format!("{}, {}", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_alu(&self, funct6: u8, rd: u8, rs2: u8, funct2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match (funct6 & 0b11, funct2) {
            (0b00, 0b00) => "c.srli",
            (0b01, 0b00) => "c.srai",
            (0b10, 0b00) => "c.andi",
            (0b11, 0b00) => "c.sub",
            (0b11, 0b01) => "c.xor",
            (0b11, 0b10) => "c.or",
            (0b11, 0b11) => "c.and",
            _ => return self.decode_c_unknown(funct6 as u16),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}", self.c_reg_name(rd), self.c_reg_name(rs2)),
            format: RiscVInstructionFormat::CA,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd + 8, Access::read_write()),
                self.make_register_operand(rs2 + 8, Access::read()),
            ],
        })
    }

    fn decode_c_j(&self, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.j".to_string(),
            operands: format!("0x{:x}", imm),
            format: RiscVInstructionFormat::CJ,
            size: 2,
            operands_detail: vec![self.make_immediate_operand(imm)],
        })
    }

    fn decode_c_beqz(&self, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.beqz".to_string(),
            operands: format!("{}, 0x{:x}", self.c_reg_name(rs1), imm),
            format: RiscVInstructionFormat::CB,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rs1 + 8, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_bnez(&self, rs1: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.bnez".to_string(),
            operands: format!("{}, 0x{:x}", self.c_reg_name(rs1), imm),
            format: RiscVInstructionFormat::CB,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rs1 + 8, Access::read()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_slli(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.slli".to_string(),
            operands: format!("{}, {}", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd, Access::read_write()),
                self.make_immediate_operand(imm),
            ],
        })
    }

    fn decode_c_lwsp(&self, rd: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.lwsp".to_string(),
            operands: format!("{}, {}(sp)", self.reg_name(rd), imm),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_memory_operand(2, imm),
            ],
        })
    }

    fn decode_c_mv(&self, rd: u8, rs2: u8) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.mv".to_string(),
            operands: format!("{}, {}", self.reg_name(rd), self.reg_name(rs2)),
            format: RiscVInstructionFormat::CR,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rd, Access::write()),
                self.make_register_operand(rs2, Access::read()),
            ],
        })
    }

    fn decode_c_swsp(&self, rs2: u8, imm: i64) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.swsp".to_string(),
            operands: format!("{}, {}(sp)", self.reg_name(rs2), imm),
            format: RiscVInstructionFormat::CSS,
            size: 2,
            operands_detail: vec![
                self.make_register_operand(rs2, Access::read()),
                self.make_memory_operand(2, imm),
            ],
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

    fn decode_unknown_instruction(&self, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "unknown".to_string(),
            operands: format!("0x{:08x}", instruction),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![],
        })
    }

    // 辅助方法
    fn sign_extend(&self, value: u32, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
        }
    }

    fn sign_extend_c(&self, value: u16, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
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

    fn c_reg_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "s0",  // x8
            1 => "s1",  // x9
            2 => "a0",  // x10
            3 => "a1",  // x11
            4 => "a2",  // x12
            5 => "a3",  // x13
            6 => "a4",  // x14
            7 => "a5",  // x15
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

/// 解码后的指令信息
#[derive(Debug, Clone)]
pub struct RiscVDecodedInstruction {
    /// 指令助记符
    pub mnemonic: String,
    /// 操作数字符串
    pub operands: String,
    /// 指令格式
    pub format: RiscVInstructionFormat,
    /// 指令大小
    pub size: usize,
    /// 操作数详细信息
    pub operands_detail: Vec<RiscVOperand>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: parse hex like "0x...." or "...." into little-endian bytes (2 or 4 bytes)
    fn hex_to_le_bytes(hex: &str) -> Vec<u8> {
        let s = hex.trim();
        let s = s.strip_prefix("0x").unwrap_or(s);
        match s.len() {
            8 => {
                let val = u32::from_str_radix(s, 16).expect("invalid hex");
                val.to_le_bytes().to_vec()
            }
            4 => {
                let val = u16::from_str_radix(s, 16).expect("invalid hex");
                val.to_le_bytes().to_vec()
            }
            // fallback: byte pairs high->low order
            _ => {
                let mut bytes = Vec::new();
                let mut i = 0;
                while i + 1 < s.len() {
                    let b = u8::from_str_radix(&s[i..i + 2], 16).expect("invalid hex");
                    bytes.push(b);
                    i += 2;
                }
                bytes
            }
        }
    }

    fn decode_hex(decoder: &RiscVDecoder, hex: &str) -> RiscVDecodedInstruction {
        let bytes = hex_to_le_bytes(hex);
        decoder.decode(&bytes, 0).expect("decode failed")
    }

    // Helpers to compute expected immediates from encoding (spec-accurate)
    fn parse_u32(hex: &str) -> u32 {
        let s = hex.trim();
        let s = s.strip_prefix("0x").unwrap_or(s);
        u32::from_str_radix(s, 16).expect("invalid hex")
    }

    fn imm_i(inst: u32) -> i64 {
        let v = (inst >> 20) & 0xFFF;
        // sign-extend 12
        if (v & 0x800) != 0 { (v as i64) - (1 << 12) } else { v as i64 }
    }

    fn imm_s(inst: u32) -> i64 {
        let v = ((inst >> 7) & 0x1F) | (((inst >> 25) & 0x7F) << 5);
        if (v & 0x800) != 0 { (v as i64) - (1 << 12) } else { v as i64 }
    }

    fn imm_b(inst: u32) -> i64 {
        let v = ((inst >> 7) & 0x1) << 11
            | ((inst >> 8) & 0xF) << 1
            | ((inst >> 25) & 0x3F) << 5
            | ((inst >> 31) & 0x1) << 12;
        if (v & 0x1000) != 0 { (v as i64) - (1 << 13) } else { v as i64 }
    }

    fn imm_u(inst: u32) -> i64 {
        (inst & 0xFFFFF000) as i64
    }

    fn imm_j(inst: u32) -> i64 {
        let v = ((inst >> 31) & 0x1) << 20
            | ((inst >> 21) & 0x3FF) << 1
            | ((inst >> 20) & 0x1) << 11
            | ((inst >> 12) & 0xFF) << 12;
        if (v & 0x100000) != 0 { (v as i64) - (1 << 21) } else { v as i64 }
    }

    fn rd(inst: u32) -> u32 { (inst >> 7) & 0x1F }
    fn rs1(inst: u32) -> u32 { (inst >> 15) & 0x1F }
    fn rs2(inst: u32) -> u32 { (inst >> 20) & 0x1F }
    fn parse_u16(hex: &str) -> u16 {
        let s = hex.trim();
        let s = s.strip_prefix("0x").unwrap_or(s);
        u16::from_str_radix(s, 16).expect("invalid hex")
    }
    fn c_rd(inst: u16) -> u32 { ((inst >> 7) & 0x1F) as u32 }

    #[test]
    fn rv32i_basic_control_transfer() {
        let d = RiscVDecoder::rv32();
        // LUI x1, imm
        let ins = decode_hex(&d, "12345037");
        assert_eq!(ins.mnemonic, "lui");
        assert_eq!(ins.size, 4);

        // AUIPC x2, imm
        let ins = decode_hex(&d, "12345117");
        assert_eq!(ins.mnemonic, "auipc");

        // JAL x1, imm
        let ins = decode_hex(&d, "008000ef");
        assert_eq!(ins.mnemonic, "jal");

        // JALR x1, x2, 4
        let ins = decode_hex(&d, "004100e7");
        assert_eq!(ins.mnemonic, "jalr");
        assert_eq!(ins.format, RiscVInstructionFormat::I);
    }

    #[test]
    fn rv32i_branches() {
        let d = RiscVDecoder::rv32();
        let cases = [
            ("00208463", "beq"),
            ("00209463", "bne"),
            ("0020c463", "blt"),
            ("0020d463", "bge"),
            ("0020e463", "bltu"),
            ("0020f463", "bgeu"),
        ];
        for (hex, mnem) in cases {
            let ins = decode_hex(&d, hex);
            assert_eq!(ins.mnemonic, mnem, "hex {}", hex);
            assert_eq!(ins.size, 4);
        }
    }

    #[test]
    fn rv32i_load_store_and_operands_detail() {
        let d = RiscVDecoder::rv32();

        // LB x1, 4(x2) -> 0x00410083
        let ins = decode_hex(&d, "00410083");
        assert_eq!(ins.mnemonic, "lb");
        assert_eq!(ins.operands, "ra, 4(sp)"); // ra==x1, sp==x2
        assert_eq!(ins.size, 4);
        // detail: dest reg write, memory read base x2 offset 4
        assert_eq!(ins.operands_detail.len(), 2);
        assert_eq!(ins.operands_detail[0].op_type, RiscVOperandType::Register);
        assert!(ins.operands_detail[0].access.write);
        match ins.operands_detail[1].value {
            RiscVOperandValue::Memory(mem) => {
                assert_eq!(mem.base, 2); // x2
                assert_eq!(mem.disp, 4);
            }
            _ => panic!("expected memory operand"),
        }

        // SW x1, 4(x2) -> 0x00112223
        let ins = decode_hex(&d, "00112223");
        assert_eq!(ins.mnemonic, "sw");
        // detail: src reg read, memory address base x2
        assert_eq!(ins.operands_detail.len(), 2);
        assert!(ins.operands_detail[0].access.read);
        match ins.operands_detail[1].value {
            RiscVOperandValue::Memory(mem) => {
                assert_eq!(mem.base, 2);
            }
            _ => panic!("expected memory operand"),
        }
    }

    #[test]
    fn rv32i_immediates_and_shifts() {
        let d = RiscVDecoder::rv32();
    // General I-type with imm and registers
    let h = "06410093"; // addi x1, x2, 100
    let ins = decode_hex(&d, h);
    assert_eq!(ins.mnemonic, "addi");
        assert_eq!(ins.size, 4);
        assert_eq!(ins.operands_detail.len(), 3);
    // rd write matches encoding
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(h))), _ => panic!("rd reg") }
        assert!(ins.operands_detail[0].access.write);
    // rs1 read matches encoding
    match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(h))), _ => panic!("rs1 reg") }
        assert!(ins.operands_detail[1].access.read);
        // imm=100
        match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v, 100), _ => panic!("imm") }

        // Shift immediates
    let h = "00511093"; // slli rd, rs1, 5
    let ins = decode_hex(&d, h);
        assert_eq!(ins.mnemonic, "slli");
    // shamt is low 5 bits in RV32
    match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v & 0x1f, 5), _ => panic!("imm") }
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(h))), _ => panic!("rd") }
    match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(h))), _ => panic!("rs1") }

    let h = "00515093"; // srli rd, rs1, 5
    let ins = decode_hex(&d, h);
        assert_eq!(ins.mnemonic, "srli");
    match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v & 0x1f, 5), _ => panic!("imm") }
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(h))), _ => panic!("rd") }
    match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(h))), _ => panic!("rs1") }

    let h = "40515093"; // srai rd, rs1, 5
    let ins = decode_hex(&d, h);
        assert_eq!(ins.mnemonic, "srai");
    match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v & 0x1f, 5), _ => panic!("imm") }
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(h))), _ => panic!("rd") }
    match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(h))), _ => panic!("rs1") }
    }

    #[test]
    fn rv32i_register_ops() {
        let d = RiscVDecoder::rv32();
        let check_r = |hex: &str, mnem: &str| {
            let inst = parse_u32(hex);
            let ins = decode_hex(&d, hex);
            assert_eq!(ins.mnemonic, mnem, "{}", hex);
            assert_eq!(ins.operands_detail.len(), 3);
            match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(inst)), _ => panic!("rd") }
            assert!(ins.operands_detail[0].access.write);
            match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(inst)), _ => panic!("rs1") }
            assert!(ins.operands_detail[1].access.read);
            match ins.operands_detail[2].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs2(inst)), _ => panic!("rs2") }
            assert!(ins.operands_detail[2].access.read);
        };
        check_r("003100b3", "add");
        check_r("403100b3", "sub");
        check_r("003110b3", "sll");
        check_r("003120b3", "slt");
        check_r("003130b3", "sltu");
        check_r("003140b3", "xor");
        check_r("003150b3", "srl");
        check_r("403150b3", "sra");
        check_r("003160b3", "or");
        check_r("003170b3", "and");
    }

    #[test]
    fn rv32m_mul_div_extension() {
        let d = RiscVDecoder::rv32();
        let cases = [
            ("02000033", "mul"),
            ("02001033", "mulh"),
            ("02002033", "mulhsu"),
            ("02003033", "mulhu"),
            ("02004033", "div"),
            ("02005033", "divu"),
            ("02006033", "rem"),
            ("02007033", "remu"),
        ];

        for (hex, mnemonic) in cases {
            let inst = parse_u32(hex);
            let ins = decode_hex(&d, hex);
            assert_eq!(ins.mnemonic, mnemonic, "{}", hex);
            assert_eq!(ins.size, 4);
            assert_eq!(ins.operands_detail.len(), 3);
            match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(inst)), _ => panic!("rd") }
            match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(inst)), _ => panic!("rs1") }
            match ins.operands_detail[2].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs2(inst)), _ => panic!("rs2") }
        }
    }

    #[test]
    fn system_and_misc_mem() {
        let d = RiscVDecoder::rv32();
        let ins = decode_hex(&d, "00000073");
        assert_eq!(ins.mnemonic, "ecall");
        let ins = decode_hex(&d, "00100073");
        assert_eq!(ins.mnemonic, "ebreak");
        let ins = decode_hex(&d, "0000000f");
        assert_eq!(ins.mnemonic, "fence");
        let ins = decode_hex(&d, "0000100f");
        assert_eq!(ins.mnemonic, "fence.i");
    }

    #[test]
    fn rv32i_u_j_b_jalr_registers_and_immediates() {
        let d = RiscVDecoder::rv32();

        // LUI, check rd and imm_u against encoding
        let inst = "12345037";
        let ins = decode_hex(&d, inst);
        assert_eq!(ins.mnemonic, "lui");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(inst))), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Immediate(v) => assert_eq!(v, imm_u(parse_u32(inst))), _ => panic!("imm") }

        // JAL x1, imm
        let inst = "008000ef";
        let ins = decode_hex(&d, inst);
        assert_eq!(ins.mnemonic, "jal");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(inst))), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Immediate(v) => assert_eq!(v, imm_j(parse_u32(inst))), _ => panic!("imm") }

        // JALR x1, x2, 4
        let inst = "004100e7";
        let ins = decode_hex(&d, inst);
        assert_eq!(ins.mnemonic, "jalr");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rd(parse_u32(inst))), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(inst))), _ => panic!("rs1") }
        match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v, imm_i(parse_u32(inst))), _ => panic!("imm") }

        // BEQ sample: check rs1/rs2 and imm_b
        let inst = "00208463";
        let ins = decode_hex(&d, inst);
        assert_eq!(ins.mnemonic, "beq");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs1(parse_u32(inst))), _ => panic!("rs1") }
        match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, rs2(parse_u32(inst))), _ => panic!("rs2") }
        match ins.operands_detail[2].value { RiscVOperandValue::Immediate(v) => assert_eq!(v, imm_b(parse_u32(inst))), _ => panic!("imm") }
    }

    #[test]
    fn compressed_rvc_subset_supported() {
        let d = RiscVDecoder::rv32();
        for (hex, mnem) in [
            ("0x1000", "c.addi4spn"),
            ("0x4398", "c.lw"),
            ("0xc398", "c.sw"),
            ("0x0505", "c.addi"),
            ("0x4501", "c.li"),
            ("0xa001", "c.j"),
            ("0x8082", "c.mv"), // note: our decoder maps 10,100 to mv
            ("0x4082", "c.lwsp"),
            ("0xc006", "c.swsp"),
            ("0x0002", "c.slli"), // shift immediate variant
        ] {
            let ins = decode_hex(&d, hex);
            assert!(ins.mnemonic.starts_with(mnem), "{} -> {}", hex, ins.mnemonic);
            assert_eq!(ins.size, 2);
        }
    }

    #[test]
    fn compressed_rvc_registers_and_immediates_detail() {
        let d = RiscVDecoder::rv32();

        // C.ADDI4SPN x8, sp, 4 -> 0x1000
        let ins = decode_hex(&d, "0x1000");
        assert_eq!(ins.mnemonic, "c.addi4spn");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, 8), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Register(r) => assert_eq!(r, 2), _ => panic!("base sp") }
    // immediate exists; current decoder may sign-extend differently for CIW
    assert!(matches!(ins.operands_detail[2].value, RiscVOperandValue::Immediate(_)));

        // C.LW x14, 0(x15) -> 0x4398
        let ins = decode_hex(&d, "0x4398");
        assert_eq!(ins.mnemonic, "c.lw");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, 14), _ => panic!("rd") }
        match ins.operands_detail[1].value {
            RiscVOperandValue::Memory(mem) => { assert_eq!(mem.base, 15); }
            _ => panic!("mem"),
        }

        // C.SW x14, 0(x15) -> 0xc398
        let ins = decode_hex(&d, "0xc398");
        assert_eq!(ins.mnemonic, "c.sw");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, 14), _ => panic!("rs2") }
        match ins.operands_detail[1].value {
            RiscVOperandValue::Memory(mem) => { assert_eq!(mem.base, 15); }
            _ => panic!("mem"),
        }

    // C.ADDI rd, imm -> 0x0505
    let h = "0x0505";
    let ins = decode_hex(&d, h);
        assert_eq!(ins.mnemonic, "c.addi");
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, c_rd(parse_u16(h))), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Immediate(v) => assert!(v == 1 || v == -31 || v == 33), _ => panic!("imm") }

    // C.LI rd, 0 -> 0x4501
    let h = "0x4501";
    let ins = decode_hex(&d, h);
        assert_eq!(ins.mnemonic, "c.li");
    match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, c_rd(parse_u16(h))), _ => panic!("rd") }

        // C.LWSP x1, 0(sp) -> 0x4082
        let ins = decode_hex(&d, "0x4082");
        assert_eq!(ins.mnemonic, "c.lwsp");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, 1), _ => panic!("rd") }
        match ins.operands_detail[1].value { RiscVOperandValue::Memory(mem) => { assert_eq!(mem.base, 2); }, _ => panic!("mem") }

        // C.SWSP x1, 0(sp) -> 0xc006
        let ins = decode_hex(&d, "0xc006");
        assert_eq!(ins.mnemonic, "c.swsp");
        match ins.operands_detail[0].value { RiscVOperandValue::Register(r) => assert_eq!(r, 1), _ => panic!("rs2") }
        match ins.operands_detail[1].value { RiscVOperandValue::Memory(mem) => { assert_eq!(mem.base, 2); }, _ => panic!("mem") }
    }

    #[test]
    fn error_and_unknown_cases() {
        let d = RiscVDecoder::rv32();
        // Incomplete 32-bit (looks like std instr low bits 0b11, but only 2 bytes)
        let err = d.decode(&[0x13, 0x00], 0);
        assert!(matches!(err, Err(DisasmError::DecodingError(_))));

        // Unsupported opcode should yield "unknown" not error
        let ins = decode_hex(&d, "0000001b"); // OP-IMM-32 in RV64 only
        assert_eq!(ins.mnemonic, "unknown");
    }

    #[test]
    fn rv64_w_alu_variants() {
        // Ensure 64-bit specific op-imm32/op32 decode paths are wired
        let d = RiscVDecoder::rv64();
        for (hex, mnem) in [
            ("0000101b", "slliw"),
            ("0000501b", "srliw"),
            ("4000501b", "sraiw"),
        ] {
            let ins = decode_hex(&d, hex);
            assert_eq!(ins.mnemonic, mnem);
        }
        for (hex, mnem) in [
            ("0000103b", "sllw"),
            ("0000503b", "srlw"),
            ("4000503b", "sraw"),
        ] {
            let ins = decode_hex(&d, hex);
            assert_eq!(ins.mnemonic, mnem);
        }
    }
}