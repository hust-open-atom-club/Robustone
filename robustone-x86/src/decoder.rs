//! Minimal x86/x64 decoder for Robustone.
//!
//! Handles a small set of common single-byte and multi-byte instructions.

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// x86 architecture mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Mode {
    X86,
    X64,
}

/// Minimal x86 decoder.
pub struct X86Decoder {
    mode: X86Mode,
}

impl X86Decoder {
    pub fn new(mode: X86Mode) -> Self {
        Self { mode }
    }

    pub fn decode(
        &self,
        bytes: &[u8],
        _mode_name: &str,
        addr: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::NeedMoreBytes,
                architecture: Some("x86".to_string()),
                detail: "empty input".to_string(),
            });
        }

        let opcode = bytes[0];
        let (mnemonic, operands, size) = match opcode {
            // NOP
            0x90 => ("nop", vec![], 1),
            // RET
            0xC3 => ("ret", vec![], 1),
            // PUSH reg (0x50-0x57)
            0x50..=0x57 => {
                let reg = opcode - 0x50;
                (
                    "push",
                    vec![Operand::Register {
                        register: x86_reg(reg),
                    }],
                    1,
                )
            }
            // POP reg (0x58-0x5F)
            0x58..=0x5F => {
                let reg = opcode - 0x58;
                (
                    "pop",
                    vec![Operand::Register {
                        register: x86_reg(reg),
                    }],
                    1,
                )
            }
            // INC r32 (0x40-0x47) — in x64 these are REX prefixes, so treat as unknown for safety
            // MOV r32, imm32 (0xB8-0xBF)
            0xB8..=0xBF => {
                let reg = opcode - 0xB8;
                if bytes.len() < 5 {
                    return Err(DisasmError::DecodeFailure {
                        kind: DecodeErrorKind::NeedMoreBytes,
                        architecture: Some("x86".to_string()),
                        detail: "need 5 bytes for mov imm32".to_string(),
                    });
                }
                let imm = i64::from(u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]));
                (
                    "mov",
                    vec![
                        Operand::Register {
                            register: x86_reg(reg),
                        },
                        Operand::Immediate { value: imm },
                    ],
                    5,
                )
            }
            _ => {
                return Err(DisasmError::DecodeFailure {
                    kind: DecodeErrorKind::InvalidEncoding,
                    architecture: Some("x86".to_string()),
                    detail: format!("unrecognized opcode 0x{opcode:02x}"),
                });
            }
        };

        Ok(DecodedInstruction {
            architecture: ArchitectureId::X86,
            address: addr,
            mode: match self.mode {
                X86Mode::X86 => "x86".to_string(),
                X86Mode::X64 => "x64".to_string(),
            },
            mnemonic: mnemonic.to_string(),
            opcode_id: Some(mnemonic.to_string()),
            size,
            raw_bytes: bytes[..size].to_vec(),
            operands,
            registers_read: Vec::new(),
            registers_written: Vec::new(),
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: Vec::new(),
            status: DecodeStatus::Success,
            render_hints: RenderHints::default(),
            render: Some(crate::render::render_x86_text_parts),
        })
    }
}

fn x86_reg(id: u8) -> RegisterId {
    RegisterId {
        architecture: ArchitectureId::X86,
        id: id as u32,
    }
}
