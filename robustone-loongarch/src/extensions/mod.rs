//! LoongArch instruction set extension families.
//!
//! Organizes decode logic into modular instruction families,
//! mirroring the proven architecture of `robustone-riscv/src/extensions/`.

use robustone_core::ir::{ArchitectureId, DecodeStatus, DecodedInstruction, RenderHints};
use robustone_core::types::error::DisasmError;

pub mod atomic;
pub mod base;
pub mod branch;
pub mod float;
pub mod memory;
pub mod misc;
pub mod vector;

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

/// Build a [`DecodedInstruction`] from decoded components.
///
/// This helper is used by every family module to avoid duplicating the
/// boilerplate of constructing the IR structure.
pub(crate) fn build_decoded_instruction(
    mnemonic: &str,
    operands: Vec<robustone_core::ir::Operand>,
    size: usize,
    word: u32,
    addr: u64,
) -> DecodedInstruction {
    DecodedInstruction {
        architecture: ArchitectureId::LoongArch,
        address: addr,
        mode: "loongarch64".to_string(),
        mnemonic: mnemonic.to_string(),
        opcode_id: Some(mnemonic.to_string()),
        size,
        raw_bytes: word.to_le_bytes().to_vec(),
        operands,
        registers_read: Vec::new(),
        registers_written: Vec::new(),
        implicit_registers_read: Vec::new(),
        implicit_registers_written: Vec::new(),
        groups: Vec::new(),
        status: DecodeStatus::Success,
        render_hints: RenderHints::default(),
        render: Some(crate::render::render_loongarch_text_parts),
    }
}

/// Create the default ordered list of families used by the decoder.
///
/// Order matters: more specific families (e.g. branch, atomic) are tried
/// before the catch-all base family so that overlapping patterns resolve
/// deterministically.
pub fn create_families() -> Vec<Box<dyn InstructionFamily>> {
    vec![
        Box::new(branch::BranchFamily),
        Box::new(memory::MemoryFamily),
        Box::new(atomic::AtomicFamily),
        Box::new(float::FloatFamily),
        Box::new(vector::VectorFamily),
        Box::new(misc::MiscFamily),
        Box::new(base::BaseFamily),
    ]
}
