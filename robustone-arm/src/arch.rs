//! AArch64 `ArchitectureHandler` implementation.
//!
//! Bridges the AArch64 decoder to the generic `ArchitectureHandler` trait.

use crate::decoder::AArch64Decoder;
use crate::types::AArch64Extensions;
use robustone_core::{
    common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    types::error::DisasmError, types::instruction::Instruction,
};

/// AArch64 architecture handler with optional detail support.
pub struct AArch64Handler {
    decoder: AArch64Decoder,
    detail: bool,
}

impl AArch64Handler {
    /// Creates a new handler with all extensions enabled.
    pub fn new() -> Self {
        Self {
            decoder: AArch64Decoder::new(),
            detail: false,
        }
    }

    /// Creates a new handler with the given extensions.
    pub fn with_extensions(extensions: AArch64Extensions) -> Self {
        Self {
            decoder: AArch64Decoder::with_extensions(extensions),
            detail: false,
        }
    }
}

impl Default for AArch64Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for AArch64Handler {
    fn set_detail(&mut self, detail: bool) {
        self.detail = detail;
    }

    fn decode_instruction(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        if !self.supports(arch_name) {
            return Err(DisasmError::UnsupportedArchitecture(arch_name.to_string()));
        }
        let decoded = self.decoder.decode(bytes, arch_name, addr)?;
        Ok((decoded, 4))
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let extensions = AArch64Extensions::from_profile(&profile.enabled_extensions[..])
            .unwrap_or(AArch64Extensions::all());
        let decoder = AArch64Decoder::with_extensions(extensions);
        let decoded = decoder.decode(bytes, profile.mode_name, addr)?;
        Ok((decoded, 4))
    }

    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let (decoded, size) = self.decode_instruction(bytes, arch_name, addr)?;
        let (mnemonic, operands) = crate::render::render_aarch64_text_parts(
            &decoded,
            robustone_core::ir::TextRenderProfile::Capstone,
            true,
            true,
            true,
            false,
        );
        let instruction = Instruction::from_decoded(decoded, mnemonic, operands, None);
        Ok((instruction, size))
    }

    fn disassemble_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        self.disassemble(bytes, profile.mode_name, addr)
    }

    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "arm" | "aarch64" | "arm64" | "aarch64be")
    }
}
