//! Branch instruction family for LoongArch.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

use crate::extensions::InstructionFamily;

/// Branch instruction family (handled by pattern table).
pub struct BranchFamily;

impl InstructionFamily for BranchFamily {
    fn try_decode(
        &self,
        _word: u32,
        _addr: u64,
    ) -> Option<Result<DecodedInstruction, DisasmError>> {
        None
    }

    fn name(&self) -> &'static str {
        "branch"
    }
}
