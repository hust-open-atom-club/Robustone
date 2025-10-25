// 与 CLI 解耦：arch 不依赖 cli 类型

use core::str;

/// 寄存器/内存访问权限（用于详细信息）
#[derive(Debug, Clone, Copy)]
pub struct Access {
    pub read: bool,
    pub write: bool,
}

impl Access {
    pub fn read() -> Self { Self { read: true, write: false } }
    pub fn write() -> Self { Self { read: false, write: true } }
    pub fn read_write() -> Self { Self { read: true, write: true } }
}

/// 操作数（用于详细信息）
#[derive(Debug, Clone)]
pub enum Operand {
    Register { reg: u32, access: Access },
    Immediate { value: u64, access: Access },
    Memory { base: Option<u32>, disp: i64, access: Access },
}
#[derive(Debug, Clone)]
pub struct InstructionDetail {
    pub operands: Vec<Operand>,
    pub regs_read: Vec<u32>,
    pub regs_write: Vec<u32>,
    pub groups: Vec<String>,
}

/// 反汇编结果
#[derive(Debug, Clone)]
pub struct Instruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
    pub size: usize,
    pub detail: Option<InstructionDetail>,
}


/// 反汇编错误
#[derive(Debug)]
pub enum DisasmError {
    UnsupportedArchitecture(String),
    DecodingError(String),
    InvalidHexCode(String),
    InvalidAddress(String),
}

impl std::fmt::Display for DisasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DisasmError::UnsupportedArchitecture(arch) => {
                write!(f, "ERROR: Unsupported architecture: {}", arch)
            }
            DisasmError::DecodingError(msg) => {
                write!(f, "ERROR: Decoding failed: {}", msg)
            }
            DisasmError::InvalidHexCode(msg) => {
                write!(f, "ERROR: invalid assembly code: {}", msg)
            }
            DisasmError::InvalidAddress(msg) => {
                write!(f, "ERROR: invalid address argument: {}", msg)
            }
        }
    }
}

impl std::error::Error for DisasmError {}

/// 反汇编选项（从上层转换而来，避免直接依赖 CLI 配置）
#[derive(Debug, Clone, Copy, Default)]
pub struct DisasmOptions {
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub skip_data: bool,
    pub unsigned_immediate: bool,
}

/// 架构处理接口
pub trait ArchitectureHandler: Sync {
    /// 仅反汇编一条指令，返回(指令, 消耗的字节数)
    fn disassemble(
        &self,
        bytes: &[u8],
        addr: u64,
        opts: &DisasmOptions,
    ) -> Result<(Instruction, usize), DisasmError>;

    /// 获取架构名称
    fn name(&self) -> &'static str;

    /// 检查是否支持该架构（用字符串匹配，避免对上层的依赖）
    fn supports(&self, arch_name: &str) -> bool;
}

/// 架构分发器
pub struct ArchitectureDispatcher {
    handlers: Vec<Box<dyn ArchitectureHandler>>,
}

impl ArchitectureDispatcher {
    /// 创建新的分发器并注册所有架构处理器
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn ArchitectureHandler>> = Vec::new();

        // 注册RISC-V处理器
        handlers.push(Box::new(RiscvHandler::new()));

    // 其他架构处理器占位：后续按需注册

        Self { handlers }
    }

    pub fn disassemble(&self, hex: &str) -> Instruction {
        // 解析 0x 前缀并转换为字节
        let s = hex.trim().to_lowercase();
        let no_prefix = if s.starts_with("0x") { &s[2..] } else { &s };
        let mut bytes = Vec::new();
        let mut i = 0;
        while i + 1 < no_prefix.len() {
            let b = u8::from_str_radix(&no_prefix[i..i + 2], 16).unwrap_or(0);
            bytes.push(b);
            i += 2;
        }
        Instruction {
            address: 0,
            bytes,
            mnemonic: "".to_string(),
            operands: "".to_string(),
            size: s.len() / 2,
            detail: None,
        }
    }

    /// 获取所有支持的架构
    pub fn supported_architectures(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|h| h.name()).collect()
    }
}

impl Default for ArchitectureDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// ============= RISC-V架构处理器 =============

/// RISC-V架构处理器
pub struct RiscvHandler;

impl RiscvHandler {
    pub fn new() -> Self { Self }

    fn is_compressed_instruction(&self, byte1: u8, _byte2: u8) -> bool {
        (byte1 & 0x3) != 0x3
    }

    fn decode_compressed_riscv(
        &self,
        bytes: &[u8],
        opts: &DisasmOptions,
    ) -> (String, String, Option<InstructionDetail>) {
        let instruction = (bytes[0] as u16) | ((bytes[1] as u16) << 8);

        match instruction & 0xE003 {
            0x6000 => {
                let imm = ((instruction >> 7) & 0x3F) * 4;
                let operands_str = format!("s0, sp, 0x{:x}", imm);
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![
                            Operand::Register {
                                reg: 8,
                                access: Access::write(),
                            }, // s0
                            Operand::Register {
                                reg: 2,
                                access: Access::read(),
                            }, // sp
                            Operand::Immediate {
                                value: imm as u64,
                                access: Access::read(),
                            },
                        ],
                        regs_read: vec![2],  // sp
                        regs_write: vec![8], // s0
                        groups: vec!["hasStdExtC".to_string()],
                    })
                } else {
                    None
                };
                ("c.addi4spn".to_string(), operands_str, detail)
            }
            0x4000 | 0x8000 => {
                let imm = ((instruction as i16) << 7) >> 7;
                let operands_str = format!("t1, 0x{:x}", imm);
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![
                            Operand::Register {
                                reg: 6,
                                access: Access::write(),
                            }, // t1
                            Operand::Immediate {
                                value: imm as u64,
                                access: Access::read(),
                            },
                        ],
                        regs_read: vec![],
                        regs_write: vec![6], // t1
                        groups: vec!["hasStdExtC".to_string()],
                    })
                } else {
                    None
                };
                ("c.li".to_string(), operands_str, detail)
            }
            _ => {
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![],
                        regs_read: vec![],
                        regs_write: vec![],
                        groups: vec!["hasStdExtC".to_string()],
                    })
                } else {
                    None
                };
                ("c.unimp".to_string(), "".to_string(), detail)
            }
        }
    }

    fn decode_riscv32_instruction(
        &self,
        bytes: &[u8],
        opts: &DisasmOptions,
    ) -> (String, String, Option<InstructionDetail>) {
        let instruction = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);

        let opcode = instruction & 0x7F;
        let rd = (instruction >> 7) & 0x1F;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = (instruction >> 15) & 0x1F;

        match opcode {
            0x13 => {
                let imm = ((instruction as i32) << 20) >> 20;
                let imm_str = if opts.unsigned_immediate {
                    format!("0x{:x}", imm as u32)
                } else {
                    format!("0x{:x}", imm)
                };

                match funct3 {
                    0x0 => {
                        let operands_str =
                            format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str);
                        let detail = if opts.detailed {
                            Some(InstructionDetail {
                                operands: vec![
                                    Operand::Register {
                                        reg: rd,
                                        access: Access::write(),
                                    },
                                    Operand::Register {
                                        reg: rs1,
                                        access: Access::read(),
                                    },
                                    Operand::Immediate {
                                        value: imm as u64,
                                        access: Access::read(),
                                    },
                                ],
                                regs_read: vec![rs1],
                                regs_write: vec![rd],
                                groups: vec![],
                            })
                        } else {
                            None
                        };
                        ("addi".to_string(), operands_str, detail)
                    }
                    0x4 => {
                        let operands_str =
                            format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str);
                        let detail = if opts.detailed {
                            Some(InstructionDetail {
                                operands: vec![
                                    Operand::Register {
                                        reg: rd,
                                        access: Access::write(),
                                    },
                                    Operand::Register {
                                        reg: rs1,
                                        access: Access::read(),
                                    },
                                    Operand::Immediate {
                                        value: imm as u64,
                                        access: Access::read(),
                                    },
                                ],
                                regs_read: vec![rs1],
                                regs_write: vec![rd],
                                groups: vec![],
                            })
                        } else {
                            None
                        };
                        ("andi".to_string(), operands_str, detail)
                    }
                    0x6 => {
                        let operands_str =
                            format!("{}, {}, {}", self.reg_name(rd), self.reg_name(rs1), imm_str);
                        let detail = if opts.detailed {
                            Some(InstructionDetail {
                                operands: vec![
                                    Operand::Register {
                                        reg: rd,
                                        access: Access::write(),
                                    },
                                    Operand::Register {
                                        reg: rs1,
                                        access: Access::read(),
                                    },
                                    Operand::Immediate {
                                        value: imm as u64,
                                        access: Access::read(),
                                    },
                                ],
                                regs_read: vec![rs1],
                                regs_write: vec![rd],
                                groups: vec![],
                            })
                        } else {
                            None
                        };
                        ("ori".to_string(), operands_str, detail)
                    }
                    0x1 => {
                        let shamt = rs1; // shift amount is in rs1 field
                        let operands_str = format!(
                            "{}, {}, 0x{:x}",
                            self.reg_name(rd),
                            self.reg_name(rs1),
                            shamt
                        );
                        let detail = if opts.detailed {
                            Some(InstructionDetail {
                                operands: vec![
                                    Operand::Register {
                                        reg: rd,
                                        access: Access::write(),
                                    },
                                    Operand::Register {
                                        reg: rs1,
                                        access: Access::read(),
                                    },
                                    Operand::Immediate {
                                        value: shamt as u64,
                                        access: Access::read(),
                                    },
                                ],
                                regs_read: vec![rs1],
                                regs_write: vec![rd],
                                groups: vec![],
                            })
                        } else {
                            None
                        };
                        ("slli".to_string(), operands_str, detail)
                    }
                    _ => {
                        let operands_str = format!("op=0x{:x}, funct3=0x{:x}", opcode, funct3);
                        let detail = if opts.detailed {
                            Some(InstructionDetail {
                                operands: vec![],
                                regs_read: vec![],
                                regs_write: vec![],
                                groups: vec![],
                            })
                        } else {
                            None
                        };
                        ("unknown_i".to_string(), operands_str, detail)
                    }
                }
            }
            0x17 => {
                let imm = ((instruction as i32) << 12) >> 12;
                let operands_str = format!("{}, 0x{:x}", self.reg_name(rd), imm << 12);
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![
                            Operand::Register {
                                reg: rd,
                                access: Access::write(),
                            },
                            Operand::Immediate {
                                value: (imm << 12) as u64,
                                access: Access::read(),
                            },
                        ],
                        regs_read: vec![],
                        regs_write: vec![rd],
                        groups: vec![],
                    })
                } else {
                    None
                };
                ("auipc".to_string(), operands_str, detail)
            }
            0x6F => {
                let imm = ((instruction & 0x80000000) as i32)
                    | (((instruction & 0x7FE00000) >> 20) as i32) << 11
                    | (((instruction & 0x00100000) >> 9) as i32) << 4
                    | ((instruction & 0x000FF000) as i32);
                let operands_str = format!("{}, 0x{:x}", self.reg_name(rd), imm);
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![
                            Operand::Register {
                                reg: rd,
                                access: Access::read_write(),
                            }, // jal reads and writes rd
                            Operand::Immediate {
                                value: imm as u64,
                                access: Access::read(),
                            },
                        ],
                        regs_read: vec![],
                        regs_write: vec![rd],
                        groups: vec![],
                    })
                } else {
                    None
                };
                ("jal".to_string(), operands_str, detail)
            }
            0x67 => {
                let imm = ((instruction as i32) << 20) >> 20;
                let operands_str =
                    format!("{}, 0x{:x}({})", self.reg_name(rd), imm, self.reg_name(rs1));
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![
                            Operand::Register {
                                reg: rd,
                                access: Access::read_write(),
                            }, // jalr reads and writes rd
                            Operand::Memory {
                                base: Some(rs1),
                                disp: imm as i64,
                                access: Access::read(),
                            },
                        ],
                        regs_read: vec![rs1],
                        regs_write: vec![rd],
                        groups: vec![],
                    })
                } else {
                    None
                };
                ("jalr".to_string(), operands_str, detail)
            }
            _ => {
                let operands_str = format!("0x{:08x}", instruction);
                let detail = if opts.detailed {
                    Some(InstructionDetail {
                        operands: vec![],
                        regs_read: vec![],
                        regs_write: vec![],
                        groups: vec![],
                    })
                } else {
                    None
                };
                ("unknown".to_string(), operands_str, detail)
            }
        }
    }

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

    fn disassemble_one_internal(
        &self,
        bytes: &[u8],
        addr: u64,
        opts: &DisasmOptions,
    ) -> Result<(Instruction, usize), DisasmError> {
        if bytes.len() >= 2 && self.is_compressed_instruction(bytes[0], bytes[1]) {
            let (mnemonic, operands, detail) = self.decode_compressed_riscv(&bytes[0..2], opts);
            Ok((
                Instruction {
                    address: addr,
                    bytes: bytes[0..2].to_vec(),
                    mnemonic,
                    operands,
                    size: 2,
                    detail,
                },
                2,
            ))
        } else if bytes.len() >= 4 {
            let (mnemonic, operands, detail) = self.decode_riscv32_instruction(&bytes[0..4], opts);
            Ok((
                Instruction {
                    address: addr,
                    bytes: bytes[0..4].to_vec(),
                    mnemonic,
                    operands,
                    size: 4,
                    detail,
                },
                4,
            ))
        } else {
            Err(DisasmError::DecodingError("incomplete instruction".to_string()))
        }
    }
}

impl ArchitectureHandler for RiscvHandler {
    fn disassemble(
        &self,
        bytes: &[u8],
        addr: u64,
        opts: &DisasmOptions,
    ) -> Result<(Instruction, usize), DisasmError> {
        self.disassemble_one_internal(bytes, addr, opts)
    }

    fn name(&self) -> &'static str {
        "riscv"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "riscv32" | "riscv64")
    }
}