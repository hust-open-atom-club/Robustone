//! Branch instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::decoder_generated;
use crate::extensions::{InstructionFamily, build_decoded_instruction};

/// Branch family dispatching to the generated exact-word decoder.
pub struct BranchFamily;

fn is_branch(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "b" | "bceqz"
            | "bcnez"
            | "beq"
            | "beqz"
            | "bge"
            | "bgeu"
            | "bl"
            | "blt"
            | "bltu"
            | "bne"
            | "bnez"
            | "jirl"
    )
}

impl InstructionFamily for BranchFamily {
    fn try_decode(&self, word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>> {
        match decoder_generated::decode_loongarch_word(word) {
            Ok((mnemonic, operands, size)) => {
                if is_branch(mnemonic) {
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
        "branch"
    }
}
