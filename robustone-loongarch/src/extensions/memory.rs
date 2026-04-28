//! Memory instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::decoder_generated;
use crate::extensions::{InstructionFamily, build_decoded_instruction};

/// Memory family dispatching to the generated exact-word decoder.
pub struct MemoryFamily;

fn is_memory(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "ld.b"
            | "ld.bu"
            | "ld.h"
            | "ld.hu"
            | "ld.w"
            | "preld"
            | "st.b"
            | "st.h"
            | "st.w"
            // Scalar FP loads / stores
            | "fld.d"
            | "fld.s"
            | "fldx.d"
            | "fldx.s"
            | "fst.d"
            | "fst.s"
            | "fstx.d"
            | "fstx.s"
            | "fldgt.d"
            | "fldgt.s"
            | "fldle.d"
            | "fldle.s"
            | "fstgt.d"
            | "fstgt.s"
            | "fstle.d"
            | "fstle.s"
            // Conditional loads / stores
            | "ldgt.b"
            | "ldgt.d"
            | "ldgt.h"
            | "ldgt.w"
            | "ldle.b"
            | "ldle.d"
            | "ldle.h"
            | "ldle.w"
            | "stgt.b"
            | "stgt.d"
            | "stgt.h"
            | "stgt.w"
            | "stle.b"
            | "stle.d"
            | "stle.h"
            | "stle.w"
            // Release-consistency loads / stores
            | "ldl.d"
            | "ldl.w"
            | "ldr.d"
            | "ldr.w"
            | "stl.d"
            | "stl.w"
            | "str.d"
            | "str.w"
            // LASX vector memory
            | "xvfrstp.b"
            | "xvfrstp.h"
            | "xvfrstpi.b"
            | "xvfrstpi.h"
            | "xvld"
            | "xvldrepl.b"
            | "xvldrepl.d"
            | "xvldrepl.h"
            | "xvldrepl.w"
            | "xvldx"
            | "xvst"
            | "xvstelm.b"
            | "xvstelm.d"
            | "xvstelm.h"
            | "xvstelm.w"
            | "xvstx"
    )
}

impl InstructionFamily for MemoryFamily {
    fn try_decode(&self, word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>> {
        match decoder_generated::decode_loongarch_word(word) {
            Ok((mnemonic, operands, size)) => {
                if is_memory(mnemonic) {
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
        "memory"
    }
}
