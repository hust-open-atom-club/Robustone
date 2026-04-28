//! Memory instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::extensions::InstructionFamily;

/// Memory instruction family (handled by pattern table).
pub struct MemoryFamily;

impl InstructionFamily for MemoryFamily {
    fn try_decode(
        &self,
        _word: u32,
        _addr: u64,
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        None
    }

    fn name(&self) -> &'static str {
        "memory"
    }
}
