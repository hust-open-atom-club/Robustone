//! Minimal AArch64 decoder for Robustone.
//!
//! Handles a small set of common AArch64 instructions.

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// Minimal AArch64 decoder.
pub struct AArch64Decoder;

impl Default for AArch64Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AArch64Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(
        &self,
        bytes: &[u8],
        _mode_name: &str,
        addr: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::NeedMoreBytes,
                architecture: Some("aarch64".to_string()),
                detail: "need 4 bytes for AArch64".to_string(),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let (mnemonic, operands, size) = decode_aarch64_word(word)?;

        Ok(DecodedInstruction {
            architecture: ArchitectureId::Arm,
            address: addr,
            mode: "aarch64".to_string(),
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
            render: Some(crate::render::render_aarch64_text_parts),
        })
    }
}

fn decode_aarch64_word(word: u32) -> Result<(&'static str, Vec<Operand>, usize), DisasmError> {
    // NOP: 0xD503201F
    if word == 0xD503201F {
        return Ok(("nop", vec![], 4));
    }

    // ADD (immediate): sf=1, op=0, S=0, shift=00, opcode=100010
    // Pattern: 1 00 10001 << 23 | imm12 << 10 | Rn << 5 | Rd
    if (word & 0xFF000000) == 0x91000000 {
        let rd = word & 0x1F;
        let rn = (word >> 5) & 0x1F;
        let imm12 = ((word >> 10) & 0xFFF) as i64;
        return Ok((
            "add",
            vec![
                Operand::Register {
                    register: aarch64_reg(rd),
                },
                Operand::Register {
                    register: aarch64_reg(rn),
                },
                Operand::Immediate { value: imm12 },
            ],
            4,
        ));
    }

    // MOVZ: 0xD2800000 | (hw << 21) | (imm16 << 5) | Rd
    if (word & 0xFFE00000) == 0xD2800000 {
        let rd = word & 0x1F;
        let imm16 = ((word >> 5) & 0xFFFF) as i64;
        return Ok((
            "mov",
            vec![
                Operand::Register {
                    register: aarch64_reg(rd),
                },
                Operand::Immediate { value: imm16 },
            ],
            4,
        ));
    }

    // RET: 0xD65F03C0
    if word == 0xD65F03C0 {
        return Ok(("ret", vec![], 4));
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("aarch64".to_string()),
        detail: format!("unrecognized AArch64 encoding 0x{word:08x}"),
    })
}

fn aarch64_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: ArchitectureId::Arm,
        id,
    }
}
