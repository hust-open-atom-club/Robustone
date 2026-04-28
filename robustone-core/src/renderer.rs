//! Renderer trait and profiles for architecture-agnostic instruction formatting.
//!
//! This module decouples text rendering from the decode IR (`DecodedInstruction`).
//! Each backend provides its own `Renderer` implementation, and the CLI/core
//! formatting pipeline invokes it after decode is complete.

use crate::ir::DecodedInstruction;
use crate::render::RenderOptions;

/// A rendering profile that controls output style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderProfile {
    /// Canonical mnemonic and register names (no aliases).
    Canonical,
    /// Capstone-compatible output (aliases, hidden operands, etc.).
    #[default]
    CapstoneCompat,
    /// Human-readable debug view.
    Debug,
}

/// Trait for architecture-specific instruction renderers.
///
/// Implementations live in the backend crates (e.g. `robustone-riscv`,
/// `robustone-loongarch`) so that `robustone-core` stays free of
/// architecture-specific formatting code.
pub trait Renderer {
    /// Render the instruction into `(mnemonic, operands)` text parts.
    fn render(&self, instruction: &DecodedInstruction, options: RenderOptions) -> (String, String);
}

/// A generic renderer that works for any architecture without a custom renderer.
///
/// It joins operands with commas and uses the raw mnemonic. This is the
/// fallback used when a backend does not yet provide a dedicated renderer.
pub struct GenericRenderer;

impl Renderer for GenericRenderer {
    fn render(
        &self,
        instruction: &DecodedInstruction,
        _options: RenderOptions,
    ) -> (String, String) {
        let operands = instruction
            .operands
            .iter()
            .map(|op| format!("{op:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        (instruction.mnemonic.clone(), operands)
    }
}
