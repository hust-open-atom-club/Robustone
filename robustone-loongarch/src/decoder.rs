//! LoongArch LA64 decoder for Robustone.
//!
//! Uses a spec-driven mask/value pattern table derived from Capstone decoder trees.

use robustone_core::{
    ir::{ArchitectureId, DecodedInstruction, Operand},
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
        match crate::patterns::try_decode_from_patterns(word, addr) {
            Some(Ok(mut decoded)) => {
                decoded.raw_bytes = bytes[..decoded.size].to_vec();

                // Capstone aliases: treat certain instruction+operand combos
                // as pseudo-instructions.
                match decoded.mnemonic.as_str() {
                    "andi" if decoded.operands.len() == 3 => {
                        if let (
                            Operand::Register { register: rd },
                            Operand::Register { register: rj },
                            Operand::Immediate { value: 0 },
                        ) = (
                            &decoded.operands[0],
                            &decoded.operands[1],
                            &decoded.operands[2],
                        ) && rd.architecture == ArchitectureId::LoongArch
                            && rd.id == 0
                            && rj.id == 0
                        {
                            decoded.mnemonic = "nop".to_string();
                            decoded.operands.clear();
                        }
                    }
                    "or" if decoded.operands.len() == 3 => {
                        if let Operand::Register { register: rk } = &decoded.operands[2]
                            && rk.architecture == ArchitectureId::LoongArch
                            && rk.id == 0
                        {
                            decoded.mnemonic = "move".to_string();
                            decoded.operands.pop();
                        }
                    }
                    _ => {}
                }

                Ok(decoded)
            }
            Some(Err(e)) => Err(e),
            None => Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::InvalidEncoding,
                architecture: Some("loongarch64".to_string()),
                detail: format!("unrecognized LoongArch encoding 0x{word:08x}"),
            }),
        }
    }
}
