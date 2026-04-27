//! LoongArch LA64 decoder for Robustone.
//!
//! Delegates to instruction families defined in `extensions/`.

use robustone_core::{
    ir::DecodedInstruction,
    types::error::{DecodeErrorKind, DisasmError},
};

use crate::extensions::create_families;

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
        let families = create_families();
        for family in &families {
            if let Some(result) = family.try_decode(word, addr) {
                let mut decoded = result?;
                decoded.raw_bytes = bytes[..decoded.size].to_vec();
                decoded.render = Some(crate::render::render_loongarch_text_parts);
                return Ok(decoded);
            }
        }

        Err(DisasmError::DecodeFailure {
            kind: DecodeErrorKind::InvalidEncoding,
            architecture: Some("loongarch64".to_string()),
            detail: format!("unrecognized LoongArch encoding 0x{word:08x}"),
        })
    }
}
