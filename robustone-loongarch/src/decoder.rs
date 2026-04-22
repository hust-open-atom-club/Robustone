//! Minimal LoongArch LA64 decoder for Robustone.
//!
//! Handles a small set of common LoongArch instructions.

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// Minimal LoongArch decoder.
pub struct LoongArchDecoder;

impl Default for LoongArchDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl LoongArchDecoder {
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
                architecture: Some("loongarch64".to_string()),
                detail: "need 4 bytes for LoongArch".to_string(),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let (mnemonic, operands, size) = decode_loongarch_word(word)?;

        Ok(DecodedInstruction {
            architecture: ArchitectureId::LoongArch,
            address: addr,
            mode: "loongarch64".to_string(),
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
            render: Some(crate::render::render_loongarch_text_parts),
        })
    }
}

fn decode_loongarch_word(word: u32) -> Result<(&'static str, Vec<Operand>, usize), DisasmError> {
    // NOP: addi.w $zero, $zero, 0 => 0x03400000
    if word == 0x03400000 {
        return Ok(("nop", vec![], 4));
    }

    // ADDI.W: opcode=0000001010, Rj(5), Rd(5), si12(12)
    // Pattern: 0x02xxxxxx where top 10 bits are 0000001010
    if (word & 0xFFC00000) == 0x02800000 {
        let rd = (word >> 12) & 0x1F;
        let rj = (word >> 17) & 0x1F;
        let simm12 = sign_extend_12(word & 0xFFF) as i64;
        return Ok((
            "addi.w",
            vec![
                Operand::Register {
                    register: loongarch_reg(rd),
                },
                Operand::Register {
                    register: loongarch_reg(rj),
                },
                Operand::Immediate { value: simm12 },
            ],
            4,
        ));
    }

    // ADD.W: opcode=00000000000100001, Rk(5), Rj(5), Rd(5)
    // Pattern: 0x0010xxxx where we check top 17 bits
    if (word & 0xFFFF0000) == 0x00100000 {
        let rd = word & 0x1F;
        let rj = (word >> 5) & 0x1F;
        let rk = (word >> 10) & 0x1F;
        return Ok((
            "add.w",
            vec![
                Operand::Register {
                    register: loongarch_reg(rd),
                },
                Operand::Register {
                    register: loongarch_reg(rj),
                },
                Operand::Register {
                    register: loongarch_reg(rk),
                },
            ],
            4,
        ));
    }

    // OR: opcode=00000000000101001, Rk(5), Rj(5), Rd(5)
    if (word & 0xFFFF0000) == 0x00140000 {
        let rd = word & 0x1F;
        let rj = (word >> 5) & 0x1F;
        let rk = (word >> 10) & 0x1F;
        return Ok((
            "or",
            vec![
                Operand::Register {
                    register: loongarch_reg(rd),
                },
                Operand::Register {
                    register: loongarch_reg(rj),
                },
                Operand::Register {
                    register: loongarch_reg(rk),
                },
            ],
            4,
        ));
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("loongarch64".to_string()),
        detail: format!("unrecognized LoongArch encoding 0x{word:08x}"),
    })
}

fn sign_extend_12(val: u32) -> i32 {
    if val & 0x800 != 0 {
        (val | 0xFFFFF000) as i32
    } else {
        val as i32
    }
}

fn loongarch_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: ArchitectureId::LoongArch,
        id,
    }
}
