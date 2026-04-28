//! Vector instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::extensions::InstructionFamily;

/// Vector instruction family (handled by pattern table).
pub struct VectorFamily;

impl InstructionFamily for VectorFamily {
    fn try_decode(
        &self,
        _word: u32,
        _addr: u64,
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        None
    }

    fn name(&self) -> &'static str {
        "vector"
    }
}
