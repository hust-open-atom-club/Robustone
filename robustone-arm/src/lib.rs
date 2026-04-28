//! ARM (AArch64) disassembly module for Robustone.
//!
//! Provides instruction decoding for ARM AArch64 targets.

pub mod decoder;
pub mod detail;
pub mod encoding;
pub mod families;
pub mod render;

use decoder::AArch64Decoder;
use detail::ArmInstructionDetail;
use robustone_core::{
    Instruction, common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    types::error::DisasmError,
};

/// Architecture handler implementation for ARM AArch64 targets.
pub struct ArmHandler {
    decoder: AArch64Decoder,
    detail: bool,
}

impl ArmHandler {
    /// Creates a new handler.
    pub fn new() -> Self {
        Self {
            decoder: AArch64Decoder::new(),
            detail: false,
        }
    }
}

impl Default for ArmHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for ArmHandler {
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
        let (mnemonic, operands) = render::render_aarch64_text_parts(
            &decoded,
            robustone_core::ir::TextRenderProfile::Capstone,
            true,
            true,
            true,
            false,
        );

        let detail = if self.detail {
            let mut arm_detail = ArmInstructionDetail::new();
            for register in decoded
                .registers_read
                .iter()
                .chain(decoded.implicit_registers_read.iter())
            {
                arm_detail = arm_detail.reads_register(register.id);
            }
            for register in decoded
                .registers_written
                .iter()
                .chain(decoded.implicit_registers_written.iter())
            {
                arm_detail = arm_detail.writes_register(register.id);
            }
            Some(Box::new(arm_detail) as Box<dyn robustone_core::traits::Detail>)
        } else {
            None
        };

        let instruction = Instruction::from_decoded(decoded, mnemonic, operands, detail);
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
        "arm"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "arm" | "aarch64" | "arm64" | "aarch64be")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_decode() {
        let handler = ArmHandler::new();
        let (instr, size) = handler
            .disassemble(&[0x1F, 0x20, 0x03, 0xD5], "aarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "nop");
    }

    #[test]
    fn test_add_imm_decode() {
        let handler = ArmHandler::new();
        // add x0, x1, #2  => 0x91000820
        let (instr, size) = handler
            .disassemble(&[0x20, 0x08, 0x00, 0x91], "aarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "add");
        assert_eq!(instr.operands, "x0, x1, #2");
    }

    #[test]
    fn test_movz_decode() {
        let handler = ArmHandler::new();
        // mov x0, #0x1234  => 0xD2824680
        let (instr, size) = handler
            .disassemble(&[0x80, 0x46, 0x82, 0xD2], "aarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "mov");
        assert_eq!(instr.operands, "x0, #0x1234");
    }

    #[test]
    fn test_ret_decode() {
        let handler = ArmHandler::new();
        let (instr, size) = handler
            .disassemble(&[0xC0, 0x03, 0x5F, 0xD6], "aarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "ret");
    }
}
