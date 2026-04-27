//! LoongArch instruction pretty-printer.
//!
//! Provides Capstone-compatible and canonical text formatting,
//! mirroring the architecture of `robustone-riscv/src/printer.rs`.

use robustone_core::Instruction;
use robustone_core::ir::{DecodedInstruction, TextRenderProfile};

/// Text formatting profiles for the LoongArch printer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchTextProfile {
    Capstone,
    Canonical,
    VerboseDebug,
}

/// Pretty-printer for LoongArch instructions.
pub struct LoongArchPrinter {
    alias_regs: bool,
    capstone_aliases: bool,
    compressed_aliases: bool,
    unsigned_immediate: bool,
    profile: LoongArchTextProfile,
}

impl LoongArchPrinter {
    fn text_render_profile(&self) -> TextRenderProfile {
        match self.profile {
            LoongArchTextProfile::Capstone => TextRenderProfile::Capstone,
            LoongArchTextProfile::Canonical => TextRenderProfile::Canonical,
            LoongArchTextProfile::VerboseDebug => TextRenderProfile::VerboseDebug,
        }
    }

    /// Creates a printer with default formatting behaviour.
    pub fn new() -> Self {
        Self {
            alias_regs: true,
            capstone_aliases: true,
            compressed_aliases: true,
            unsigned_immediate: false,
            profile: LoongArchTextProfile::Capstone,
        }
    }

    /// Enables or disables register alias printing.
    pub fn with_alias_regs(mut self, alias_regs: bool) -> Self {
        self.alias_regs = alias_regs;
        self
    }

    pub fn with_capstone_aliases(mut self, capstone_aliases: bool) -> Self {
        self.capstone_aliases = capstone_aliases;
        self
    }

    pub fn with_compressed_aliases(mut self, compressed_aliases: bool) -> Self {
        self.compressed_aliases = compressed_aliases;
        self
    }

    pub fn with_unsigned_immediate(mut self, unsigned_immediate: bool) -> Self {
        self.unsigned_immediate = unsigned_immediate;
        self
    }

    pub fn with_profile(mut self, profile: LoongArchTextProfile) -> Self {
        self.profile = profile;
        self
    }

    /// Render a decoded instruction into mnemonic and operand strings.
    pub fn render(&self, instruction: &DecodedInstruction) -> (String, String) {
        crate::render::render_loongarch_text_parts(
            instruction,
            self.text_render_profile(),
            self.alias_regs,
            self.capstone_aliases,
            self.compressed_aliases,
            self.unsigned_immediate,
        )
    }
}

impl Default for LoongArchPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: render an `Instruction` with the default Capstone profile.
pub fn render_instruction(instr: &Instruction) -> (String, String) {
    let printer = LoongArchPrinter::new();
    if let Some(ref decoded) = instr.decoded {
        printer.render(decoded)
    } else {
        (instr.mnemonic.clone(), instr.operands.clone())
    }
}
