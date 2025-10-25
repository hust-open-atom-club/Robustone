//! RISC-V架构处理器实现

use crate::arch::{ArchitectureHandler, DisasmConfig, DisassemblyResult, Instruction, DisasmError};
use crate::cli::Architecture;

/// RISC-V架构处理器
pub struct RiscvHandler {
    /// 支持的架构列表
    supported_archs: [Architecture; 2],
}

impl RiscvHandler {
    /// 创建新的RISC-V处理器
    pub fn new() -> Self {
        Self {
            supported_archs: [
                Architecture::Riscv32,
                Architecture::Riscv64,
            ],
        }
    }

    /// 检查是否为压缩指令 (前2位不为11)
    fn is_compressed_instruction(&self, byte1: u8, _byte2: u8) -> bool {
        (byte1 & 0x3) != 0x3
    }

    /// 解码16位压缩指令
    fn decode_compressed_riscv(&self, bytes: &[u8], _config: &DisasmConfig) -> (String, String) {
        let instruction = (bytes[0] as u16) | ((bytes[1] as u16) << 8);

        // 简单的压缩指令解码示例
        match instruction & 0xE003 {
            0x6000 => ("c.addi4spn".to_string(), format!("s0, sp, 0x{:x}", ((instruction >> 7) & 0x3F) * 4)),
            0x4000 => ("c.li".to_string(), format!("t1, 0x{:x}", ((instruction as i16) << 7) >> 7)),
            0x8000 => ("c.li".to_string(), format!("t1, 0x{:x}", ((instruction as i16) << 7) >> 7)),
            _ => ("c.unimp".to_string(), "".to_string()),
        }
    }

    /// 解码32位标准指令
    fn decode_riscv32_instruction(&self, bytes: &[u8], config: &DisasmConfig) -> (String, String) {
        let instruction = (bytes[0] as u32) |
                         ((bytes[1] as u32) << 8) |
                         ((bytes[2] as u32) << 16) |
                         ((bytes[3] as u32) << 24);

        // 简单的RISC-V指令解码示例
        let opcode = instruction & 0x7F;
        let rd = (instruction >> 7) & 0x1F;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = (instruction >> 15) & 0x1F;
        let _funct7 = (instruction >> 25) & 0x7F;

        match opcode {
            0x13 => {
                // I-type指令 (addi, andi, ori, xori, slti, sltiu)
                let imm = ((instruction as i32) << 20) >> 20;
                let imm_str = if config.unsigned_immediate {
                    format!("0x{:x}", imm as u32)
                } else {
                    format!("0x{:x}", imm)
                };

                match funct3 {
                    0x0 => ("addi".to_string(), format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str)),
                    0x4 => ("andi".to_string(), format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str)),
                    0x6 => ("ori".to_string(), format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str)),
                    0x1 => ("slli".to_string(), format!("{}, {}, 0x{:x}", self.reg_name(rd), self.reg_name(rs1), rs1)),
                    _ => ("unknown_i".to_string(), format!("op=0x{:x}, funct3=0x{:x}", opcode, funct3)),
                }
            }
            0x17 => {
                // auipc指令
                let imm = ((instruction as i32) << 12) >> 12;
                ("auipc".to_string(), format!("{}, 0x{:x}", self.reg_name(rd), imm << 12))
            }
            0x6F => {
                // jal指令 - 使用正确的位操作
                let imm = ((instruction & 0x80000000) as i32) |
                         (((instruction & 0x7FE00000) >> 20) as i32) << 11 |
                         (((instruction & 0x00100000) >> 9) as i32) << 4 |
                         ((instruction & 0x000FF000) as i32);
                ("jal".to_string(), format!("{}, 0x{:x}", self.reg_name(rd), imm))
            }
            0x67 => {
                // jalr指令
                let imm = ((instruction as i32) << 20) >> 20;
                ("jalr".to_string(), format!("{}, 0x{:x}({})", self.reg_name(rd), imm, self.reg_name(rs1)))
            }
            _ => ("unknown".to_string(), format!("0x{:08x}", instruction)),
        }
    }

    /// 寄存器名称转换
    fn reg_name(&self, reg: u32) -> String {
        match reg {
            0 => "zero".to_string(),
            1 => "ra".to_string(),
            2 => "sp".to_string(),
            3 => "gp".to_string(),
            4 => "tp".to_string(),
            5 => "t0".to_string(),
            6 => "t1".to_string(),
            7 => "t2".to_string(),
            8 => "s0".to_string(),
            9 => "s1".to_string(),
            10 => "a0".to_string(),
            11 => "a1".to_string(),
            12 => "a2".to_string(),
            13 => "a3".to_string(),
            14 => "a4".to_string(),
            15 => "a5".to_string(),
            16 => "a6".to_string(),
            17 => "a7".to_string(),
            18 => "s2".to_string(),
            19 => "s3".to_string(),
            20 => "s4".to_string(),
            21 => "s5".to_string(),
            22 => "s6".to_string(),
            23 => "s7".to_string(),
            24 => "s8".to_string(),
            25 => "s9".to_string(),
            26 => "s10".to_string(),
            27 => "s11".to_string(),
            28 => "t3".to_string(),
            29 => "t4".to_string(),
            30 => "t5".to_string(),
            31 => "t6".to_string(),
            _ => format!("x{}", reg),
        }
    }

    /// 反汇编RISC-V指令序列
    fn disassemble_riscv(&self, config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
        let address = config.start_address;
        let mut instructions = Vec::new();
        let mut current_address = address;
        let mut i = 0;

        while i < config.hex_code.len() {
            // RISC-V支持16位和32位指令
            let (size, mnemonic, operands) = if i + 1 < config.hex_code.len() &&
                self.is_compressed_instruction(config.hex_code[i], config.hex_code[i + 1]) {
                // 16位压缩指令
                let instruction_bytes = &config.hex_code[i..i+2];
                let (mnemonic, operands) = self.decode_compressed_riscv(instruction_bytes, config);
                (2, mnemonic, operands)
            } else if i + 3 < config.hex_code.len() {
                // 32位指令
                let instruction_bytes = &config.hex_code[i..i+4];
                let (mnemonic, operands) = self.decode_riscv32_instruction(instruction_bytes, config);
                (4, mnemonic, operands)
            } else {
                return Err(DisasmError::DecodingError("incomplete instruction at end".to_string()));
            };

            instructions.push(Instruction {
                address: current_address,
                bytes: config.hex_code[i..i+size].to_vec(),
                mnemonic,
                operands,
                size,
            });

            current_address += size as u64;
            i += size;
        }

        Ok(DisassemblyResult {
            instructions,
        })
    }
}

impl ArchitectureHandler for RiscvHandler {
    fn disassemble(&self, config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
        // 根据RISC-V架构变体进行分发
        match config.arch_spec.arch {
            Architecture::Riscv32 | Architecture::Riscv64 => {
                self.disassemble_riscv(config)
            }
            _ => Err(DisasmError::UnsupportedArchitecture(config.arch_spec.arch.clone())),
        }
    }

    fn name(&self) -> &'static str {
        "riscv"
    }

    fn supports(&self, arch: &Architecture) -> bool {
        self.supported_archs.contains(arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_riscv_handler_creation() {
        let handler = RiscvHandler::new();
        assert_eq!(handler.name(), "riscv");
        assert!(handler.supports(&Architecture::Riscv32));
        assert!(handler.supports(&Architecture::Riscv64));
        assert!(!handler.supports(&Architecture::Arm));
    }

    #[test]
    fn test_compressed_instruction_detection() {
        let handler = RiscvHandler::new();

        // 压缩指令前2位不为11
        assert!(handler.is_compressed_instruction(0x01, 0x00)); // 0000 0001
        assert!(handler.is_compressed_instruction(0x02, 0x00)); // 0000 0010

        // 标准指令前2位为11
        assert!(!handler.is_compressed_instruction(0xFF, 0x00)); // 1111 1111
        assert!(!handler.is_compressed_instruction(0x93, 0x00)); // 1001 0011
    }
}