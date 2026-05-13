#![forbid(unsafe_code)]

//! ARM (AArch64) disassembly module for Robustone.
//!
//! Experimental legacy backend. This crate is currently a framework placeholder
//! and is not yet migrated to the robustone-isa declarative backend model.
//! Uses the unified `decode_one` pipeline via `ArmBackend` for smoke testing.

pub mod backend;
pub mod render;

use robustone_core::{
    Instruction, common::ArchitectureProfile, ir::DecodedInstruction, traits::ArchitectureHandler,
    types::error::DisasmError,
};
use robustone_isa::DecodeProfile;

/// Architecture handler implementation for ARM AArch64 targets.
pub struct ArmHandler;

impl ArmHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArmHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for ArmHandler {
    fn set_detail(&mut self, _detail: bool) {}

    fn renderer(&self) -> Option<&dyn robustone_core::renderer::Renderer> {
        Some(&crate::render::AArch64Renderer)
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
        let profile = DecodeProfile {
            mode: crate::backend::ArmMode::AArch64,
            features: crate::backend::ArmFeature::BASE,
            render_dialect: robustone_isa::RenderDialect::Canonical,
            alias_policy: robustone_isa::AliasPolicy::None,
        };
        let decoded =
            robustone_isa::decode_one::<crate::backend::ArmBackend>(bytes, addr, &profile)?;
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
            robustone_core::ir::TextRenderProfile::Compat,
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
        "arm"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "arm" | "aarch64" | "arm64")
    }
}

// LEGACY: decoder module is kept as a stub. All decoding now routes through
// the unified ArmBackend + decode_one pipeline. The file is being retained
// until Phase 6's full AArch64 migration is complete.
pub mod decoder {}

#[cfg(test)]
mod tests {
    use super::*;

    fn disasm_simd(bytes: &[u8]) -> (String, String) {
        let profile = DecodeProfile {
            mode: crate::backend::ArmMode::AArch64,
            features: crate::backend::ArmFeature::BASE
                | crate::backend::ArmFeature::SIMD
                | crate::backend::ArmFeature::FP,
            render_dialect: robustone_isa::RenderDialect::Canonical,
            alias_policy: robustone_isa::AliasPolicy::None,
        };
        let decoded =
            robustone_isa::decode_one::<crate::backend::ArmBackend>(bytes, 0, &profile).unwrap();
        let (mnemonic, operands) = render::render_aarch64_text_parts(
            &decoded,
            robustone_core::ir::TextRenderProfile::Compat,
            true,
            true,
            true,
            false,
        );
        (mnemonic, operands)
    }

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
        assert_eq!(instr.operands, "x0, x1, 2");
    }

    #[test]
    fn test_movz_decode() {
        let handler = ArmHandler::new();
        // mov x0, #0x1234  => 0xD2824680
        let (instr, size) = handler
            .disassemble(&[0x80, 0x46, 0x82, 0xD2], "aarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "movz");
        assert_eq!(instr.operands, "x0, 0x1234");
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

    // -------------------------------------------------------------------------
    // Advanced SIMD Copy/Extract
    // -------------------------------------------------------------------------

    #[test]
    fn test_dup_element_8b() {
        // dup v1.8b, v2.b[2] => bytes [0x41, 0x04, 0x05, 0x0e]
        let (mnemonic, operands) = disasm_simd(&[0x41, 0x04, 0x05, 0x0e]);
        assert_eq!(mnemonic, "dup");
        assert_eq!(operands, "v1.8b, v2.b[2]");
    }

    #[test]
    fn test_dup_element_16b() {
        // dup v1.16b, v2.b[2] => bytes [0x41, 0x04, 0x05, 0x4e]
        let (mnemonic, operands) = disasm_simd(&[0x41, 0x04, 0x05, 0x4e]);
        assert_eq!(mnemonic, "dup");
        assert_eq!(operands, "v1.16b, v2.b[2]");
    }

    #[test]
    fn test_dup_general_8b() {
        // dup v1.8b, w1 => bytes [0x21, 0x0c, 0x01, 0x0e]
        let (mnemonic, operands) = disasm_simd(&[0x21, 0x0c, 0x01, 0x0e]);
        assert_eq!(mnemonic, "dup");
        assert_eq!(operands, "v1.8b, w1");
    }

    #[test]
    fn test_smov_w_b() {
        // smov w1, v0.b[15] => bytes [0x01, 0x2c, 0x1f, 0x0e]
        let (mnemonic, operands) = disasm_simd(&[0x01, 0x2c, 0x1f, 0x0e]);
        assert_eq!(mnemonic, "smov");
        assert_eq!(operands, "w1, v0.b[15]");
    }

    #[test]
    fn test_smov_x_h() {
        // smov x14, v6.h[4] => bytes [0xce, 0x2c, 0x12, 0x4e]
        let (mnemonic, operands) = disasm_simd(&[0xce, 0x2c, 0x12, 0x4e]);
        assert_eq!(mnemonic, "smov");
        assert_eq!(operands, "x14, v6.h[4]");
    }

    #[test]
    fn test_umov_alias_mov_w_s() {
        // mov w20, v9.s[2] => bytes [0x34, 0x3d, 0x14, 0x0e]
        let (mnemonic, operands) = disasm_simd(&[0x34, 0x3d, 0x14, 0x0e]);
        assert_eq!(mnemonic, "mov");
        assert_eq!(operands, "w20, v9.s[2]");
    }

    #[test]
    fn test_umov_x_d() {
        // mov x7, v18.d[1] => bytes [0x47, 0x3e, 0x18, 0x4e]
        let (mnemonic, operands) = disasm_simd(&[0x47, 0x3e, 0x18, 0x4e]);
        assert_eq!(mnemonic, "mov");
        assert_eq!(operands, "x7, v18.d[1]");
    }

    #[test]
    fn test_ins_element_b() {
        // mov v1.b[14], v3.b[6] => bytes [0x61, 0x34, 0x1d, 0x6e]
        let (mnemonic, operands) = disasm_simd(&[0x61, 0x34, 0x1d, 0x6e]);
        assert_eq!(mnemonic, "mov");
        assert_eq!(operands, "v1.b[14], v3.b[6]");
    }

    #[test]
    fn test_ext_8b() {
        // ext v0.8b, v1.8b, v2.8b, #3 => bytes [0x20, 0x18, 0x02, 0x2e]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x18, 0x02, 0x2e]);
        assert_eq!(mnemonic, "ext");
        assert_eq!(operands, "v0.8b, v1.8b, v2.8b, 3");
    }

    #[test]
    fn test_ext_16b() {
        // ext v0.16b, v1.16b, v2.16b, #3 => bytes [0x20, 0x18, 0x02, 0x6e]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x18, 0x02, 0x6e]);
        assert_eq!(mnemonic, "ext");
        assert_eq!(operands, "v0.16b, v1.16b, v2.16b, 3");
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Vector Shift Immediate
    // -------------------------------------------------------------------------

    #[test]
    fn test_sshr_8b() {
        // sshr v0.8b, v1.8b, #3 => bytes [0x20, 0x04, 0x0d, 0x0f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x04, 0x0d, 0x0f]);
        assert_eq!(mnemonic, "sshr");
        assert_eq!(operands, "v0.8b, v1.8b, #3");
    }

    #[test]
    fn test_sshr_16b() {
        // sshr v0.16b, v1.16b, #3 => bytes [0x20, 0x04, 0x0d, 0x4f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x04, 0x0d, 0x4f]);
        assert_eq!(mnemonic, "sshr");
        assert_eq!(operands, "v0.16b, v1.16b, #3");
    }

    #[test]
    fn test_ushr_8b() {
        // ushr v0.8b, v1.8b, #3 => bytes [0x20, 0x04, 0x0d, 0x2f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x04, 0x0d, 0x2f]);
        assert_eq!(mnemonic, "ushr");
        assert_eq!(operands, "v0.8b, v1.8b, #3");
    }

    #[test]
    fn test_sqshl_8b() {
        // sqshl v0.8b, v1.8b, #3 => bytes [0x20, 0x74, 0x0b, 0x0f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x74, 0x0b, 0x0f]);
        assert_eq!(mnemonic, "sqshl");
        assert_eq!(operands, "v0.8b, v1.8b, #3");
    }

    #[test]
    fn test_sqshlu_8b() {
        // sqshlu v0.8b, v1.8b, #3 => bytes [0x20, 0x64, 0x0b, 0x2f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x64, 0x0b, 0x2f]);
        assert_eq!(mnemonic, "sqshlu");
        assert_eq!(operands, "v0.8b, v1.8b, #3");
    }

    #[test]
    fn test_shrn_8b() {
        // shrn v0.8b, v1.8h, #3 => bytes [0x20, 0x84, 0x0d, 0x0f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x84, 0x0d, 0x0f]);
        assert_eq!(mnemonic, "shrn");
        assert_eq!(operands, "v0.8b, v1.8h, #3");
    }

    #[test]
    fn test_sshll_8b() {
        // sshll v0.8h, v1.8b, #3 => bytes [0x20, 0xa4, 0x0b, 0x0f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0xa4, 0x0b, 0x0f]);
        assert_eq!(mnemonic, "sshll");
        assert_eq!(operands, "v0.8h, v1.8b, #3");
    }

    #[test]
    fn test_sshll2_16b() {
        // sshll2 v0.8h, v1.16b, #3 => bytes [0x20, 0xa4, 0x0b, 0x4f]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0xa4, 0x0b, 0x4f]);
        assert_eq!(mnemonic, "sshll2");
        assert_eq!(operands, "v0.8h, v1.16b, #3");
    }
}

// Register the ARM handler with the global inventory.
inventory::submit! {
    robustone_core::traits::HandlerFactory::new(|| Box::new(ArmHandler::new()))
}
