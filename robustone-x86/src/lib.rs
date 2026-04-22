//! x86/x64 disassembly module for Robustone.
//!
//! Provides instruction decoding for x86 and x86-64 targets.

pub mod decoder;
pub mod render;

use decoder::{X86Decoder, X86Mode};
use robustone_core::{
    Instruction, common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    types::error::DisasmError,
};

/// Architecture handler implementation for x86/x64 targets.
pub struct X86Handler {
    x86_decoder: X86Decoder,
    x64_decoder: X86Decoder,
}

impl X86Handler {
    /// Creates a new handler with both x86 and x64 decoders.
    pub fn new() -> Self {
        Self {
            x86_decoder: X86Decoder::new(X86Mode::X86),
            x64_decoder: X86Decoder::new(X86Mode::X64),
        }
    }

    fn decoder_for_arch(&self, arch_name: &str) -> Result<&X86Decoder, DisasmError> {
        match arch_name {
            "x86" | "x32" | "i386" => Ok(&self.x86_decoder),
            "x64" | "x86_64" | "amd64" => Ok(&self.x64_decoder),
            _ => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
        }
    }
}

impl Default for X86Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for X86Handler {
    fn set_detail(&mut self, _detail: bool) {}

    fn decode_instruction(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let decoder = self.decoder_for_arch(arch_name)?;
        let decoded = decoder.decode(bytes, arch_name, addr)?;
        let size = decoded.size;
        Ok((decoded, size))
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        self.decode_instruction(bytes, profile.mode_name, addr)
    }

    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let (decoded, size) = self.decode_instruction(bytes, arch_name, addr)?;
        let (mnemonic, operands) = render::render_x86_text_parts(
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
        "x86"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(
            arch_name,
            "x86" | "x32" | "i386" | "x64" | "x86_64" | "amd64"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_decode() {
        let handler = X86Handler::new();
        let (instr, size) = handler.disassemble(&[0x90], "x86", 0).unwrap();
        assert_eq!(size, 1);
        assert_eq!(instr.mnemonic, "nop");
    }

    #[test]
    fn test_push_pop_decode() {
        let handler = X86Handler::new();
        let (instr, size) = handler.disassemble(&[0x50], "x86", 0).unwrap();
        assert_eq!(size, 1);
        assert_eq!(instr.mnemonic, "push");
        assert_eq!(instr.operands, "eax");

        let (instr, size) = handler.disassemble(&[0x58], "x86", 0).unwrap();
        assert_eq!(size, 1);
        assert_eq!(instr.mnemonic, "pop");
        assert_eq!(instr.operands, "eax");
    }

    #[test]
    fn test_mov_imm_decode() {
        let handler = X86Handler::new();
        let (instr, size) = handler
            .disassemble(&[0xB8, 0x78, 0x56, 0x34, 0x12], "x86", 0)
            .unwrap();
        assert_eq!(size, 5);
        assert_eq!(instr.mnemonic, "mov");
        assert_eq!(instr.operands, "eax, 0x12345678");
    }
}
