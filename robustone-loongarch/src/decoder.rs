//! LoongArch LA64 decoder for Robustone.
//!
//! Auto-generated from Capstone YAML test data.

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// LoongArch decoder.
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
        let (mnemonic, operands, size) = crate::decoder_generated::decode_loongarch_word(word)?;

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
