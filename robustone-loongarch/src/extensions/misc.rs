//! Misc instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::decoder_generated;
use crate::extensions::{InstructionFamily, build_decoded_instruction};

/// Misc family dispatching to the generated exact-word decoder.
pub struct MiscFamily;

fn is_misc(mnemonic: &str) -> bool {
    matches!(mnemonic, "dbar" | "ibar")
}

impl InstructionFamily for MiscFamily {
    fn try_decode(&self, word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>> {
        match decoder_generated::decode_loongarch_word(word) {
            Ok((mnemonic, operands, size)) => {
                if is_misc(mnemonic) {
                    Some(Ok(build_decoded_instruction(
                        mnemonic, operands, size, word, addr,
                    )))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn name(&self) -> &'static str {
        "misc"
    }
}
