//! Table-driven decode patterns for RISC-V base integer instructions.
//!
//! This module provides a static pattern table for RV32I/RV64I base integer
//! instructions. Each pattern matches via `(word & mask) == value`. When a
//! pattern matches, the decoder builds the `DecodedInstruction` directly from
//! the pattern's operand layout without consulting extension match trees.

use crate::decoder::Xlen;
use crate::shared::encoding::convenience as bits;
use crate::types::*;

use robustone_core::ir::{DecodedInstruction, Operand, RenderHints};
use robustone_core::types::error::DisasmError;

/// Which XLEN variants a pattern applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XlenMask {
    /// Valid for both RV32 and RV64.
    Both,
    /// RV32 only.
    X32,
    /// RV64 only.
    X64,
}

impl XlenMask {
    /// Check whether this pattern is compatible with the given XLEN.
    pub fn matches(self, xlen: Xlen) -> bool {
        match self {
            XlenMask::Both => true,
            XlenMask::X32 => xlen == Xlen::X32,
            XlenMask::X64 => xlen == Xlen::X64,
        }
    }
}

/// Describes how operands are laid out for a given instruction format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandLayout {
    /// R-type: rd, rs1, rs2
    R,
    /// I-type: rd, rs1, imm12
    I,
    /// I-type shift: rd, rs1, shamt (funct7 distinguishes slli/srli/srai)
    IShift,
    /// S-type: rs2, rs1, imm12 (memory store)
    S,
    /// B-type: rs1, rs2, imm13
    B,
    /// U-type: rd, imm20
    U,
    /// J-type: rd, imm21
    J,
    /// System with register source: rd, csr, rs1
    System,
    /// System with immediate source: rd, csr, uimm5
    SystemImm,
    /// Fence: pred, succ
    Fence,
    /// ECALL/EBREAK: no operands
    EcallEbreak,
}

/// A single decode pattern for a RISC-V instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiscVPattern {
    /// Bit mask of fields that must match.
    pub mask: u32,
    /// Expected value after masking.
    pub value: u32,
    /// Which XLEN this pattern applies to.
    pub xlen: XlenMask,
    /// Instruction format (for operand extraction).
    pub format: RiscVInstructionFormat,
    /// Canonical mnemonic.
    pub mnemonic: &'static str,
    /// How operands are encoded in the instruction word.
    pub operand_layout: OperandLayout,
}

/// Build a `DecodedInstruction` from a matched pattern and raw instruction word.
pub fn decode_from_pattern(
    word: u32,
    pattern: &RiscVPattern,
    xlen: Xlen,
) -> Result<DecodedInstruction, DisasmError> {
    if !pattern.xlen.matches(xlen) {
        return Err(crate::types::error::DisasmError::decode_failure(
            crate::types::error::DecodeErrorKind::UnsupportedMode,
            None::<String>,
            format!("{} is not valid for {:?}", pattern.mnemonic, xlen),
        ));
    }

    let operands = match pattern.operand_layout {
        OperandLayout::R => {
            let f = bits::extract_fields(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(f.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(f.rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(f.rs2 as u32),
                },
            ]
        }
        OperandLayout::I => {
            let i = bits::extract_i_type(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(i.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(i.rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(i.imm),
                },
            ]
        }
        OperandLayout::IShift => {
            let i = bits::extract_i_type(word);
            let shamt = bits::extract_shamt(i.imm, xlen);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(i.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(i.rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(shamt),
                },
            ]
        }
        OperandLayout::S => {
            let s = bits::extract_s_type(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(s.rs2 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Memory,
                    access: Access::read(),
                    value: RiscVOperandValue::Memory(RiscVMemoryOperand {
                        base: s.rs1 as u32,
                        disp: s.imm,
                    }),
                },
            ]
        }
        OperandLayout::B => {
            let b = bits::extract_b_type(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(b.rs1 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(b.rs2 as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(b.imm),
                },
            ]
        }
        OperandLayout::U => {
            let u = bits::extract_u_type(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(u.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(u.imm),
                },
            ]
        }
        OperandLayout::J => {
            let j = bits::extract_j_type(word);
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(j.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(j.imm),
                },
            ]
        }
        OperandLayout::System => {
            let i = bits::extract_i_type(word);
            let csr = (word >> 20) & 0xFFF;
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(i.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(csr as i64),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::read(),
                    value: RiscVOperandValue::Register(i.rs1 as u32),
                },
            ]
        }
        OperandLayout::SystemImm => {
            let i = bits::extract_i_type(word);
            let csr = (word >> 20) & 0xFFF;
            let uimm = i.rs1 as i64;
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Register,
                    access: Access::write(),
                    value: RiscVOperandValue::Register(i.rd as u32),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(csr as i64),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(uimm),
                },
            ]
        }
        OperandLayout::Fence => {
            let imm = bits::extract_i_type(word).imm as u16;
            let pred = ((imm >> 4) & 0xF) as i64;
            let succ = (imm & 0xF) as i64;
            vec![
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(pred),
                },
                RiscVOperand {
                    op_type: RiscVOperandType::Immediate,
                    access: Access::none(),
                    value: RiscVOperandValue::Immediate(succ),
                },
            ]
        }
        OperandLayout::EcallEbreak => vec![],
    };

    let mut decoded = crate::decoder::build_riscv_decoded_instruction(
        pattern.mnemonic,
        pattern.format,
        4,
        operands,
    );

    // Apply reference-compatible aliases based on operand values.
    match pattern.mnemonic {
        "addi" => {
            if let [_, Operand::Register { register: rs1 }, _] = decoded.operands.as_slice()
                && rs1.id == 0
                && decoded
                    .registers_written
                    .first()
                    .map(|r| r.id != 0)
                    .unwrap_or(false)
            {
                decoded.render_hints.compat_mnemonic = Some("li".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "ori" => {
            let rd = ((word >> 7) & 0x1F) as u8;
            if rd == 0 {
                let (rs1, imm) = if let [
                    _,
                    Operand::Register { register: rs1 },
                    Operand::Immediate { value: imm },
                ] = decoded.operands.as_slice()
                {
                    (Some(*rs1), *imm)
                } else {
                    (None, 0)
                };
                if let Some(rs1) = rs1 {
                    let prefetch = match imm {
                        0 => Some("prefetch.i"),
                        1 => Some("prefetch.r"),
                        2 => Some("prefetch.t"),
                        3 => Some("prefetch.w"),
                        _ => None,
                    };
                    if let Some(pf) = prefetch {
                        decoded.mnemonic = pf.to_string();
                        decoded.operands = vec![Operand::Memory {
                            base: Some(rs1),
                            displacement: 0,
                        }];
                        decoded.registers_read = vec![rs1];
                        decoded.groups = vec!["load".to_string()];
                        decoded.render_hints = RenderHints::default();
                    }
                }
            }
        }
        "jal" => {
            if let Some(rd) = decoded.registers_written.first()
                && rd.id == 0
            {
                decoded.render_hints.compat_mnemonic = Some("j".to_string());
                decoded.render_hints.compat_hidden_operands = vec![0];
            } else if let Some(rd) = decoded.registers_written.first()
                && rd.id == 1
            {
                decoded.render_hints.compat_hidden_operands = vec![0];
            }
        }
        "jalr" => {
            if let Some(rd) = decoded.registers_written.first()
                && rd.id == 1
            {
                decoded.render_hints.compat_hidden_operands = vec![0];
            }
        }
        "beq" => {
            if let [_, _, Operand::Register { register: rs2 }] = decoded.operands.as_slice()
                && rs2.id == 0
            {
                decoded.render_hints.compat_mnemonic = Some("beqz".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "bne" => {
            if let [_, _, Operand::Register { register: rs2 }] = decoded.operands.as_slice()
                && rs2.id == 0
            {
                decoded.render_hints.compat_mnemonic = Some("bnez".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "csrrs" | "csrrw" | "csrrc" => {
            if let [
                _,
                Operand::Immediate { value: csr },
                Operand::Register { register: rs1 },
            ] = decoded.operands.as_slice()
            {
                let (alias, hidden) = match (
                    pattern.mnemonic,
                    decoded.registers_written.first(),
                    rs1.id,
                    *csr as u16,
                ) {
                    ("csrrs", _, 0, 0xC00) => (Some("rdcycle"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC01) => (Some("rdtime"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC02) => (Some("rdinstret"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC80) => (Some("rdcycleh"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC81) => (Some("rdtimeh"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC82) => (Some("rdinstreth"), vec![1, 2]),
                    ("csrrs", Some(_), 0, _) => (Some("csrr"), vec![2]),
                    ("csrrw", _, _, _) if decoded.registers_written.is_empty() => {
                        (Some("csrw"), vec![0])
                    }
                    ("csrrs", _, _, _) if decoded.registers_written.is_empty() => {
                        (Some("csrs"), vec![0])
                    }
                    ("csrrc", _, _, _) if decoded.registers_written.is_empty() => {
                        (Some("csrc"), vec![0])
                    }
                    _ => (None, Vec::new()),
                };
                if let Some(a) = alias {
                    decoded.render_hints.compat_mnemonic = Some(a.to_string());
                    decoded.render_hints.compat_hidden_operands = hidden;
                }
            }
        }
        "csrrwi" | "csrrsi" | "csrrci" => {
            if decoded.registers_written.is_empty() {
                let alias = match pattern.mnemonic {
                    "csrrwi" => Some("csrwi"),
                    "csrrsi" => Some("csrsi"),
                    "csrrci" => Some("csrci"),
                    _ => None,
                };
                if let Some(a) = alias {
                    decoded.render_hints.compat_mnemonic = Some(a.to_string());
                    decoded.render_hints.compat_hidden_operands = vec![0];
                }
            }
        }
        _ => {}
    }

    Ok(decoded)
}

/// Attempt to decode a 32-bit standard instruction using the pattern table.
///
/// Returns `Some(Ok(decoded))` on match, `None` if no pattern matched.
pub fn try_decode_from_patterns(
    word: u32,
    xlen: Xlen,
) -> Option<Result<DecodedInstruction, DisasmError>> {
    for pattern in RISCV_BASE_PATTERNS.iter() {
        if (word & pattern.mask) == pattern.value {
            return Some(decode_from_pattern(word, pattern, xlen));
        }
    }
    None
}

/// Static pattern table for RV32I/RV64I base integer instructions.
pub const RISCV_BASE_PATTERNS: &[RiscVPattern] = &[
    // OP-IMM
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_0013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "addi",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_2013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "slti",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_3013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "sltiu",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_4013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "xori",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_6013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "ori",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_7013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "andi",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_1013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "slli",
        operand_layout: OperandLayout::IShift,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_5013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "srli",
        operand_layout: OperandLayout::IShift,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_5013,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "srai",
        operand_layout: OperandLayout::IShift,
    },
    // OP-IMM-32 (RV64 only)
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_001B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "addiw",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_101B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "slliw",
        operand_layout: OperandLayout::IShift,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_501B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "srliw",
        operand_layout: OperandLayout::IShift,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_501B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "sraiw",
        operand_layout: OperandLayout::IShift,
    },
    // OP
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_0033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "add",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_0033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "sub",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_1033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "sll",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_2033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "slt",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_3033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "sltu",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_4033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "xor",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_5033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "srl",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_5033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "sra",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_6033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "or",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_7033,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::R,
        mnemonic: "and",
        operand_layout: OperandLayout::R,
    },
    // OP-32 (RV64 only)
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_003B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::R,
        mnemonic: "addw",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_003B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::R,
        mnemonic: "subw",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_103B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::R,
        mnemonic: "sllw",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x0000_503B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::R,
        mnemonic: "srlw",
        operand_layout: OperandLayout::R,
    },
    RiscVPattern {
        mask: 0xFE00_707F,
        value: 0x4000_503B,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::R,
        mnemonic: "sraw",
        operand_layout: OperandLayout::R,
    },
    // LOAD
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_0003,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "lb",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_1003,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "lh",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_2003,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "lw",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_4003,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "lbu",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_5003,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "lhu",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_3003,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "ld",
        operand_layout: OperandLayout::I,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_6003,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::I,
        mnemonic: "lwu",
        operand_layout: OperandLayout::I,
    },
    // STORE
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_0023,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::S,
        mnemonic: "sb",
        operand_layout: OperandLayout::S,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_1023,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::S,
        mnemonic: "sh",
        operand_layout: OperandLayout::S,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_2023,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::S,
        mnemonic: "sw",
        operand_layout: OperandLayout::S,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_3023,
        xlen: XlenMask::X64,
        format: RiscVInstructionFormat::S,
        mnemonic: "sd",
        operand_layout: OperandLayout::S,
    },
    // BRANCH
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_0063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "beq",
        operand_layout: OperandLayout::B,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_1063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "bne",
        operand_layout: OperandLayout::B,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_4063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "blt",
        operand_layout: OperandLayout::B,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_5063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "bge",
        operand_layout: OperandLayout::B,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_6063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "bltu",
        operand_layout: OperandLayout::B,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_7063,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::B,
        mnemonic: "bgeu",
        operand_layout: OperandLayout::B,
    },
    // JAL / JALR
    RiscVPattern {
        mask: 0x0000_007F,
        value: 0x0000_006F,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::J,
        mnemonic: "jal",
        operand_layout: OperandLayout::J,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_0067,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "jalr",
        operand_layout: OperandLayout::I,
    },
    // U-type
    RiscVPattern {
        mask: 0x0000_007F,
        value: 0x0000_0037,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::U,
        mnemonic: "lui",
        operand_layout: OperandLayout::U,
    },
    RiscVPattern {
        mask: 0x0000_007F,
        value: 0x0000_0017,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::U,
        mnemonic: "auipc",
        operand_layout: OperandLayout::U,
    },
    // SYSTEM
    RiscVPattern {
        mask: 0xFFF0_707F,
        value: 0x0000_0073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "ecall",
        operand_layout: OperandLayout::EcallEbreak,
    },
    RiscVPattern {
        mask: 0xFFF0_707F,
        value: 0x0010_0073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "ebreak",
        operand_layout: OperandLayout::EcallEbreak,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_1073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrw",
        operand_layout: OperandLayout::System,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_2073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrs",
        operand_layout: OperandLayout::System,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_3073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrc",
        operand_layout: OperandLayout::System,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_5073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrwi",
        operand_layout: OperandLayout::SystemImm,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_6073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrsi",
        operand_layout: OperandLayout::SystemImm,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_7073,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "csrrci",
        operand_layout: OperandLayout::SystemImm,
    },
    // MISC-MEM
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_000F,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "fence",
        operand_layout: OperandLayout::Fence,
    },
    RiscVPattern {
        mask: 0x0000_707F,
        value: 0x0000_100F,
        xlen: XlenMask::Both,
        format: RiscVInstructionFormat::I,
        mnemonic: "fence.i",
        operand_layout: OperandLayout::I,
    },
];
