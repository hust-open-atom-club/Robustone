//! LoongArch LA64 disassembly module for Robustone.
//!
//! Provides instruction decoding for LoongArch LA64 targets.

pub mod decoder;
pub mod render;

use decoder::LoongArchDecoder;
use robustone_core::{
    Instruction, common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    types::error::DisasmError,
};

/// Architecture handler implementation for LoongArch LA64 targets.
pub struct LoongArchHandler {
    decoder: LoongArchDecoder,
}

impl LoongArchHandler {
    /// Creates a new handler.
    pub fn new() -> Self {
        Self {
            decoder: LoongArchDecoder::new(),
        }
    }
}

impl Default for LoongArchHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for LoongArchHandler {
    fn set_detail(&mut self, _detail: bool) {}

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
        let (mnemonic, operands) = render::render_loongarch_text_parts(
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
        "loongarch"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "loongarch" | "loongarch64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_decode() {
        let handler = LoongArchHandler::new();
        let (instr, size) = handler
            .disassemble(&[0x00, 0x00, 0x40, 0x03], "loongarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "nop");
    }

    #[test]
    fn test_addi_w_decode() {
        let handler = LoongArchHandler::new();
        // addi.w $a0, $a1, 4 => 0x028a4004
        let bytes = [0x04, 0x40, 0x8a, 0x02];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "addi.w");
        assert_eq!(instr.operands, "$a0, $a1, 4");
    }

    #[test]
    fn test_add_w_decode() {
        let handler = LoongArchHandler::new();
        // add.w $a0, $a1, $a2 => 0x001098a4
        let bytes = [0xa4, 0x98, 0x10, 0x00];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "add.w");
        assert_eq!(instr.operands, "$a0, $a1, $a2");
    }

    #[test]
    fn test_or_decode() {
        let handler = LoongArchHandler::new();
        // or $a0, $a1, $a2 => 0x001498a4
        let bytes = [0xa4, 0x98, 0x14, 0x00];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "or");
        assert_eq!(instr.operands, "$a0, $a1, $a2");
    }
}
