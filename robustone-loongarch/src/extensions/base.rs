//! Base instruction family for LoongArch.
//!
//! Covers integer ALU, bitwise, privileged, and other non-specialized
//! instructions decoded from the auto-generated Capstone tables.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::extensions::InstructionFamily;

/// Base family dispatching to the generated exact-word decoder.
pub struct BaseFamily;

impl InstructionFamily for BaseFamily {
    fn try_decode(
        &self,
        _word: u32,
        _addr: u64,
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        None
    }

    fn name(&self) -> &'static str {
        "base"
    }
}
