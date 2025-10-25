//! RISC-V指令解码器
//!
//! 基于Capstone解码逻辑实现的RISC-V指令解码器

use super::types::*;
use crate::error::DisasmError;

/// RISC-V指令解码器
pub struct RiscVDecoder {
    /// 支持的扩展
    extensions: u32,
    /// 是否为64位模式
    is_64bit: bool,
}

impl RiscVDecoder {
    /// 创建新的RISC-V解码器
    pub fn new(extensions: u32, is_64bit: bool) -> Self {
        Self {
            extensions,
            is_64bit,
        }
    }

    /// 检查是否为压缩指令 (前2位不为11)
    fn is_compressed_instruction(&self, byte1: u8, _byte2: u8) -> bool {
        (byte1 & 0x3) != 0x3
    }

    /// 解码16位压缩指令
    fn decode_compressed_instruction(&self, bytes: &[u8]) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.len() < 2 {
            return Err(DisasmError::DecodingError("Incomplete compressed instruction".to_string()));
        }

        let instruction = (bytes[0] as u16) | ((bytes[1] as u16) << 8);

        // 基于Capstone的压缩指令解码逻辑
        let opcode = instruction & 0xE003;
        let funct3 = (instruction >> 13) & 0x7;
        let rd = (instruction >> 7) & 0x1F;

        match opcode {
            0x6000 => {
                // C.ADDI4SPN
                let nzuimm = ((instruction >> 5) & 0x3) << 1 |
                             ((instruction >> 6) & 0x1) << 3 |
                             ((instruction >> 11) & 0x1) << 2 |
                             ((instruction >> 12) & 0x1) << 4 |
                             ((instruction >> 2) & 0x7) << 6;

                Ok(RiscVDecodedInstruction {
                    mnemonic: "c.addi4spn".to_string(),
                    operands: format!("s0, sp, {}", nzuimm * 4),
                    format: RiscVInstructionFormat::CIW,
                    size: 2,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::write(),
                            value: RiscVOperandValue::Register(8), // s0
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::read(),
                            value: RiscVOperandValue::Register(2), // sp
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Immediate,
                            access: Access::read(),
                            value: RiscVOperandValue::Immediate((nzuimm * 4) as i64),
                        },
                    ],
                })
            }
            0x4000 | 0x8000 => {
                // C.LI
                let imm = ((instruction as i16) << 7) >> 7;
                Ok(RiscVDecodedInstruction {
                    mnemonic: "c.li".to_string(),
                    operands: format!("t1, {}", imm),
                    format: RiscVInstructionFormat::CI,
                    size: 2,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::write(),
                            value: RiscVOperandValue::Register(6), // t1
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Immediate,
                            access: Access::read(),
                            value: RiscVOperandValue::Immediate(imm as i64),
                        },
                    ],
                })
            }
            0x2000 => {
                // C.SLLI
                if rd == 0 {
                    return Err(DisasmError::DecodingError("Invalid compressed instruction".to_string()));
                }
                let nzuimm = ((instruction >> 7) & 0x1F) |
                             ((instruction & 0x20) << 5);
                Ok(RiscVDecodedInstruction {
                    mnemonic: "c.slli".to_string(),
                    operands: format!("{}, {}", RiscVRegister::from_id(rd as u32).name(), nzuimm),
                    format: RiscVInstructionFormat::CI,
                    size: 2,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::read_write(),
                            value: RiscVOperandValue::Register(rd as u32),
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Immediate,
                            access: Access::read(),
                            value: RiscVOperandValue::Immediate(nzuimm as i64),
                        },
                    ],
                })
            }
            _ => {
                // 其他压缩指令的处理逻辑...
                Ok(RiscVDecodedInstruction {
                    mnemonic: "c.unimp".to_string(),
                    operands: String::new(),
                    format: RiscVInstructionFormat::CI,
                    size: 2,
                    operands_detail: vec![],
                })
            }
        }
    }

    /// 解码32位标准指令
    fn decode_standard_instruction(&self, bytes: &[u8]) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::DecodingError("Incomplete standard instruction".to_string()));
        }

        let instruction = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);

        let opcode = instruction & 0x7F;
        let rd = (instruction >> 7) & 0x1F;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = (instruction >> 15) & 0x1F;
        let rs2 = (instruction >> 20) & 0x1F;
        let funct7 = (instruction >> 25) & 0x7F;

        match opcode {
            0x13 => {
                // I-type指令 (算术和逻辑立即数指令)
                let imm = ((instruction as i32) << 20) >> 20;
                self.decode_i_type_instruction(funct3, rd, rs1, imm, instruction)
            }
            0x33 => {
                // R-type指令 (算术和逻辑寄存器指令)
                self.decode_r_type_instruction(funct3, funct7, rd, rs1, rs2, instruction)
            }
            0x03 => {
                // Load指令
                let imm = ((instruction as i32) << 20) >> 20;
                self.decode_load_instruction(funct3, rd, rs1, imm, instruction)
            }
            0x23 => {
                // Store指令
                let imm = ((instruction as i32) << 20) >> 20;
                self.decode_store_instruction(funct3, rs2, rs1, imm, instruction)
            }
            0x17 => {
                // AUIPC指令
                let imm = ((instruction as i32) << 12) >> 12;
                Ok(RiscVDecodedInstruction {
                    mnemonic: "auipc".to_string(),
                    operands: format!("{}, 0x{:x}", RiscVRegister::from_id(rd as u32).name(), imm << 12),
                    format: RiscVInstructionFormat::U,
                    size: 4,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::write(),
                            value: RiscVOperandValue::Register(rd as u32),
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Immediate,
                            access: Access::read(),
                            value: RiscVOperandValue::Immediate((imm << 12) as i64),
                        },
                    ],
                })
            }
            0x6F => {
                // JAL指令
                let imm = ((instruction & 0x80000000) as i32)
                    | (((instruction & 0x7FE00000) >> 20) as i32) << 11
                    | (((instruction & 0x00100000) >> 9) as i32) << 4
                    | ((instruction & 0x000FF000) as i32);

                Ok(RiscVDecodedInstruction {
                    mnemonic: "jal".to_string(),
                    operands: format!("{}, 0x{:x}", RiscVRegister::from_id(rd as u32).name(), imm),
                    format: RiscVInstructionFormat::J,
                    size: 4,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::read_write(),
                            value: RiscVOperandValue::Register(rd as u32),
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Immediate,
                            access: Access::read(),
                            value: RiscVOperandValue::Immediate(imm as i64),
                        },
                    ],
                })
            }
            0x67 => {
                // JALR指令
                let imm = ((instruction as i32) << 20) >> 20;
                Ok(RiscVDecodedInstruction {
                    mnemonic: "jalr".to_string(),
                    operands: format!("{}, 0x{:x}({})",
                        RiscVRegister::from_id(rd as u32).name(),
                        imm,
                        RiscVRegister::from_id(rs1 as u32).name()),
                    format: RiscVInstructionFormat::I,
                    size: 4,
                    operands_detail: vec![
                        RiscVOperand {
                            op_type: RiscVOperandType::Register,
                            access: Access::read_write(),
                            value: RiscVOperandValue::Register(rd as u32),
                        },
                        RiscVOperand {
                            op_type: RiscVOperandType::Memory,
                            access: Access::read(),
                            value: RiscVOperandValue::Memory(RiscVMemoryOperand {
                                base: rs1 as u32,
                                disp: imm as i64,
                            }),
                        },
                    ],
                })
            }
            0x63 => {
                // B-type分支指令
                let imm = ((instruction & 0x80000000) as i32) >> 19
                    | ((instruction & 0x80) as i32) << 4
                    | (((instruction >> 20) & 0x3E0) as i32)
                    | ((instruction & 0x40) as i32) << 2;
                self.decode_branch_instruction(funct3, rs1, rs2, imm, instruction)
            }
            _ => {
                // 未知指令
                Ok(RiscVDecodedInstruction {
                    mnemonic: "unknown".to_string(),
                    operands: format!("0x{:08x}", instruction),
                    format: RiscVInstructionFormat::I,
                    size: 4,
                    operands_detail: vec![],
                })
            }
        }
    }

    /// 解码I型指令
    fn decode_i_type_instruction(&self, funct3: u32, rd: u32, rs1: u32, imm: i32, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        let (mnemonic, imm_str) = match funct3 {
            0x0 => ("addi", format!("{}", imm)),
            0x1 => ("slli", format!("{}", rs1)), // shamt in rs1 field for shift instructions
            0x2 => ("slti", format!("{}", imm)),
            0x3 => ("sltiu", format!("{}", (imm as u32))),
            0x4 => ("xori", format!("{}", imm)),
            0x5 => {
                let funct7 = (instruction >> 25) & 0x7F;
                let shamt = rs1;
                match funct7 {
                    0x0 => ("srli", format!("{}", shamt)),
                    0x20 => ("srai", format!("{}", shamt)),
                    _ => return Ok(RiscVDecodedInstruction {
                        mnemonic: "unknown_i".to_string(),
                        operands: format!("funct7=0x{:x}, funct3=0x{:x}", funct7, funct3),
                        format: RiscVInstructionFormat::I,
                        size: 4,
                        operands_detail: vec![],
                    }),
                }
            }
            0x6 => ("ori", format!("{}", imm)),
            0x7 => ("andi", format!("{}", imm)),
            _ => return Ok(RiscVDecodedInstruction {
                mnemonic: "unknown_i".to_string(),
                operands: format!("funct3=0x{:x}", funct3),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}",
                RiscVRegister::from_id(rd as u32).name(),
                RiscVRegister::from_id(rs1 as u32).name(),
                imm_str),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::read(),
                    value: RiscVOperandValue::Immediate(imm as i64),
                },
            ],
        })
    }

    /// 解码R型指令
    fn decode_r_type_instruction(&self, funct3: u32, funct7: u32, rd: u32, rs1: u32, rs2: u32, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        let (mnemonic, access) = match (funct3, funct7) {
            (0x0, 0x00) => ("add", Access::read_write()),
            (0x0, 0x20) => ("sub", Access::read_write()),
            (0x1, 0x00) => ("sll", Access::read_write()),
            (0x2, 0x00) => ("slt", Access::read_write()),
            (0x3, 0x00) => ("sltu", Access::read_write()),
            (0x4, 0x00) => ("xor", Access::read_write()),
            (0x5, 0x00) => ("srl", Access::read_write()),
            (0x5, 0x20) => ("sra", Access::read_write()),
            (0x6, 0x00) => ("or", Access::read_write()),
            (0x7, 0x00) => ("and", Access::read_write()),
            _ => return Ok(RiscVDecodedInstruction {
                mnemonic: "unknown_r".to_string(),
                operands: format!("funct3=0x{:x}, funct7=0x{:x}", funct3, funct7),
                format: RiscVInstructionFormat::R,
                size: 4,
                operands_detail: vec![],
            }),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, {}",
                RiscVRegister::from_id(rd as u32).name(),
                RiscVRegister::from_id(rs1 as u32).name(),
                RiscVRegister::from_id(rs2 as u32).name()),
            format: RiscVInstructionFormat::R,
            size: 4,
            operands_detail: vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access,
                    value: RiscVOperandValue::Register(rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs2 as u32),
                },
            ],
        })
    }

    /// 解码Load指令
    fn decode_load_instruction(&self, funct3: u32, rd: u32, rs1: u32, imm: i32, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            0x0 => "lb",
            0x1 => "lh",
            0x2 => "lw",
            0x3 => "ld",
            0x4 => "lbu",
            0x5 => "lhu",
            0x6 => "lwu",
            _ => return Ok(RiscVDecodedInstruction {
                mnemonic: "unknown_load".to_string(),
                operands: format!("funct3=0x{:x}", funct3),
                format: RiscVInstructionFormat::I,
                size: 4,
                operands_detail: vec![],
            }),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})",
                RiscVRegister::from_id(rd as u32).name(),
                imm,
                RiscVRegister::from_id(rs1 as u32).name()),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Memory,
                    access: Access::read(),
                    value: RiscVOperandValue::Memory(RiscVMemoryOperand {
                        base: rs1 as u32,
                        disp: imm as i64,
                    }),
                },
            ],
        })
    }

    /// 解码Store指令
    fn decode_store_instruction(&self, funct3: u32, rs2: u32, rs1: u32, imm: i32, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            0x0 => "sb",
            0x1 => "sh",
            0x2 => "sw",
            0x3 => "sd",
            _ => return Ok(RiscVDecodedInstruction {
                mnemonic: "unknown_store".to_string(),
                operands: format!("funct3=0x{:x}", funct3),
                format: RiscVInstructionFormat::S,
                size: 4,
                operands_detail: vec![],
            }),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}({})",
                RiscVRegister::from_id(rs2 as u32).name(),
                imm,
                RiscVRegister::from_id(rs1 as u32).name()),
            format: RiscVInstructionFormat::S,
            size: 4,
            operands_detail: vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs2 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Memory,
                    access: Access::write(),
                    value: RiscVOperandValue::Memory(RiscVMemoryOperand {
                        base: rs1 as u32,
                        disp: imm as i64,
                    }),
                },
            ],
        })
    }

    /// 解码分支指令
    fn decode_branch_instruction(&self, funct3: u32, rs1: u32, rs2: u32, imm: i32, instruction: u32) -> Result<RiscVDecodedInstruction, DisasmError> {
        let mnemonic = match funct3 {
            0x0 => "beq",
            0x1 => "bne",
            0x4 => "blt",
            0x5 => "bge",
            0x6 => "bltu",
            0x7 => "bgeu",
            _ => return Ok(RiscVDecodedInstruction {
                mnemonic: "unknown_branch".to_string(),
                operands: format!("funct3=0x{:x}", funct3),
                format: RiscVInstructionFormat::B,
                size: 4,
                operands_detail: vec![],
            }),
        };

        Ok(RiscVDecodedInstruction {
            mnemonic: mnemonic.to_string(),
            operands: format!("{}, {}, 0x{:x}",
                RiscVRegister::from_id(rs1 as u32).name(),
                RiscVRegister::from_id(rs2 as u32).name(),
                imm),
            format: RiscVInstructionFormat::B,
            size: 4,
            operands_detail: vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(rs2 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::read(),
                    value: RiscVOperandValue::Immediate(imm as i64),
                },
            ],
        })
    }

    /// 主解码函数
    pub fn decode(&self, bytes: &[u8], address: u64) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::DecodingError("No bytes provided".to_string()));
        }

        // 检查是否为压缩指令
        if bytes.len() >= 2 && self.is_compressed_instruction(bytes[0], bytes[1]) {
            let decoded = self.decode_compressed_instruction(bytes)?;
            Ok(decoded)
        } else if bytes.len() >= 4 {
            let decoded = self.decode_standard_instruction(bytes)?;
            Ok(decoded)
        } else {
            Err(DisasmError::DecodingError("Incomplete instruction".to_string()))
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