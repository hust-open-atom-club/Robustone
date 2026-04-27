//! LoongArch instruction set extension families.
//!
//! Organizes decode logic into modular instruction families,
//! mirroring the proven architecture of `robustone-riscv/src/extensions/`.

use robustone_core::ir::DecodedInstruction;
use robustone_core::types::error::DisasmError;

pub mod base;

/// Common trait implemented by every instruction family.
pub trait InstructionFamily: Sync {
    /// Attempt to decode `word` at `addr`.
    ///
    /// Returns `Some(Ok(...))` when the family successfully decodes the
    /// instruction, `Some(Err(...))` on a recoverable decode error, and
    /// `None` when the word does not belong to this family.
    fn try_decode(&self, word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>>;

    /// Human-readable family name (used for diagnostics).
    fn name(&self) -> &'static str;
}

/// Create the default ordered list of families used by the decoder.
pub fn create_families() -> Vec<Box<dyn InstructionFamily>> {
    vec![Box::new(base::BaseFamily)]
}
