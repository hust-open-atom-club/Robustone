#![forbid(unsafe_code)]

//! ARM (AArch64) disassembly module for Robustone.
//!
//! Declarative spec-driven backend using the `robustone-isa` framework.
//! Defines instruction specs, formats, registers, and alias resolution
//! for AArch64 base integer, branch, load/store, scalar FP, system,
//! and Advanced SIMD vector instructions.

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
    /// Create a new AArch64 architecture handler.
    pub fn new() -> Self {
        Self
    }

    fn default_features() -> crate::backend::ArmFeature {
        crate::backend::ArmFeature::BASE
            | crate::backend::ArmFeature::SIMD
            | crate::backend::ArmFeature::FP
            | crate::backend::ArmFeature::CRYPTO
            | crate::backend::ArmFeature::ATOMICS
            | crate::backend::ArmFeature::SYSTEM
            | crate::backend::ArmFeature::SVE
            | crate::backend::ArmFeature::SME
            | crate::backend::ArmFeature::RCPC
            | crate::backend::ArmFeature::PAUTH
            | crate::backend::ArmFeature::BTI
            | crate::backend::ArmFeature::MTE
            | crate::backend::ArmFeature::BF16
            | crate::backend::ArmFeature::CSSC
            | crate::backend::ArmFeature::FP8DOT2
            | crate::backend::ArmFeature::FAMINMAX
            | crate::backend::ArmFeature::LUT
            | crate::backend::ArmFeature::FP8
            | crate::backend::ArmFeature::FP8FMA
    }

    fn features_from_profile(profile: &ArchitectureProfile) -> crate::backend::ArmFeature {
        if profile.enabled_extensions.is_empty() {
            return Self::default_features();
        }

        let mut features = crate::backend::ArmFeature::BASE;
        for extension in &profile.enabled_extensions {
            features |= match extension.to_ascii_uppercase().as_str() {
                "BASE" => crate::backend::ArmFeature::BASE,
                "FP" => crate::backend::ArmFeature::FP,
                "SIMD" | "ASIMD" | "NEON" => crate::backend::ArmFeature::SIMD,
                "CRYPTO" => crate::backend::ArmFeature::CRYPTO,
                "ATOMICS" | "LSE" => crate::backend::ArmFeature::ATOMICS,
                "SYSTEM" => crate::backend::ArmFeature::SYSTEM,
                "SVE" => crate::backend::ArmFeature::SVE,
                "SME" => crate::backend::ArmFeature::SME,
                "RCPC" => crate::backend::ArmFeature::RCPC,
                "PAUTH" => crate::backend::ArmFeature::PAUTH,
                "BTI" => crate::backend::ArmFeature::BTI,
                "MTE" => crate::backend::ArmFeature::MTE,
                "BF16" => crate::backend::ArmFeature::BF16,
                "CSSC" | "FEAT_CSSC" => crate::backend::ArmFeature::CSSC,
                "FP8DOT2" | "FEAT_FP8DOT2" => crate::backend::ArmFeature::FP8DOT2,
                "FAMINMAX" | "FEAT_FAMINMAX" => crate::backend::ArmFeature::FAMINMAX,
                "LUT" | "FEAT_LUT" => crate::backend::ArmFeature::LUT,
                "FP8" | "FEAT_FP8" => crate::backend::ArmFeature::FP8,
                "FP8FMA" | "FEAT_FP8FMA" => crate::backend::ArmFeature::FP8FMA,
                _ => crate::backend::ArmFeature::empty(),
            };
        }
        features
    }

    fn decode_with_features(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
        features: crate::backend::ArmFeature,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        if !self.supports(arch_name) {
            return Err(DisasmError::UnsupportedArchitecture(arch_name.to_string()));
        }
        let profile = DecodeProfile {
            mode: crate::backend::ArmMode::AArch64,
            features,
            render_dialect: robustone_isa::RenderDialect::Canonical,
            alias_policy: robustone_isa::AliasPolicy::None,
        };
        let decoded =
            robustone_isa::decode_one::<crate::backend::ArmBackend>(bytes, addr, &profile)?;
        let size = decoded.size;
        Ok((decoded, size))
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
        self.decode_with_features(bytes, arch_name, addr, Self::default_features())
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        self.decode_with_features(
            bytes,
            profile.mode_name,
            addr,
            Self::features_from_profile(profile),
        )
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
        let (decoded, size) = self.decode_instruction_with_profile(bytes, profile, addr)?;
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

    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "aarch64" | "arm64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disasm_simd(bytes: &[u8]) -> (String, String) {
        let profile = DecodeProfile {
            mode: crate::backend::ArmMode::AArch64,
            features: crate::backend::ArmFeature::BASE
                | crate::backend::ArmFeature::SIMD
                | crate::backend::ArmFeature::FP
                | crate::backend::ArmFeature::CRYPTO
                | crate::backend::ArmFeature::ATOMICS
                | crate::backend::ArmFeature::SYSTEM
                | crate::backend::ArmFeature::SVE
                | crate::backend::ArmFeature::SME
                | crate::backend::ArmFeature::RCPC
                | crate::backend::ArmFeature::PAUTH
                | crate::backend::ArmFeature::BTI
                | crate::backend::ArmFeature::MTE
                | crate::backend::ArmFeature::BF16
                | crate::backend::ArmFeature::FP8DOT2
                | crate::backend::ArmFeature::FAMINMAX
                | crate::backend::ArmFeature::LUT
                | crate::backend::ArmFeature::FP8,
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
    fn test_aarch64_handler_identity_matches_decode_target() {
        let handler = ArmHandler::new();

        assert_eq!(handler.name(), "aarch64");
        assert!(handler.supports("aarch64"));
        assert!(handler.supports("arm64"));
        assert!(!handler.supports("arm"));
    }

    #[test]
    fn test_aarch64_decode_reports_aarch64_ir_identity() {
        let handler = ArmHandler::new();
        let (decoded, size) = handler
            .decode_instruction(&[0x1F, 0x20, 0x03, 0xD5], "aarch64", 0)
            .expect("AArch64 NOP should decode");

        assert_eq!(size, 4);
        assert_eq!(decoded.architecture.as_str(), "aarch64");
        assert_ne!(decoded.architecture.as_str(), "arm");
    }

    #[test]
    fn test_aarch64_register_metadata_reports_aarch64_ir_identity() {
        let handler = ArmHandler::new();
        let (decoded, size) = handler
            .decode_instruction(&[0x20, 0x08, 0x00, 0x91], "aarch64", 0)
            .expect("AArch64 ADD should decode with explicit register operands");

        assert_eq!(size, 4);
        assert_eq!(decoded.mnemonic, "add");

        assert!(decoded.operands.iter().all(|operand| match operand {
            robustone_core::ir::Operand::Register { register } => {
                register.architecture.as_str() == "aarch64"
            }
            _ => true,
        }));
        assert!(
            decoded
                .registers_read
                .iter()
                .all(|register| register.architecture.as_str() == "aarch64")
        );
        assert!(
            decoded
                .registers_written
                .iter()
                .all(|register| register.architecture.as_str() == "aarch64")
        );

        let serialized = decoded
            .to_json_pretty()
            .expect("decoded IR should serialize");
        assert_eq!(
            serialized.matches("\"architecture\": \"aarch64\"").count(),
            5
        );
        assert!(!serialized.contains("\"architecture\": \"arm\""));
    }

    #[test]
    fn test_aarch64_register_banks_cover_predicate_zvec_and_system_ids() {
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::Pred, 0).unwrap(),
            128
        );
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::Pred, 15).unwrap(),
            143
        );
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::ZVec, 0).unwrap(),
            160
        );
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::ZVec, 31).unwrap(),
            191
        );
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::Sys, 0).unwrap(),
            512
        );
        assert_eq!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::Sys, 4095).unwrap(),
            4607
        );
        assert!(
            crate::backend::lower_register(crate::backend::ArmRegisterClass::Sys, 4096).is_err()
        );
    }

    #[test]
    fn test_aarch64_feature_flags_include_extension_set() {
        let all_default_features = crate::backend::ArmFeature::BASE
            | crate::backend::ArmFeature::FP
            | crate::backend::ArmFeature::SIMD
            | crate::backend::ArmFeature::CRYPTO
            | crate::backend::ArmFeature::ATOMICS
            | crate::backend::ArmFeature::SYSTEM
            | crate::backend::ArmFeature::SVE
            | crate::backend::ArmFeature::SME
            | crate::backend::ArmFeature::RCPC
            | crate::backend::ArmFeature::PAUTH
            | crate::backend::ArmFeature::BTI
            | crate::backend::ArmFeature::MTE
            | crate::backend::ArmFeature::BF16
            | crate::backend::ArmFeature::CSSC
            | crate::backend::ArmFeature::FP8DOT2
            | crate::backend::ArmFeature::FAMINMAX
            | crate::backend::ArmFeature::LUT
            | crate::backend::ArmFeature::FP8
            | crate::backend::ArmFeature::FP8FMA;

        assert!(all_default_features.contains(crate::backend::ArmFeature::BF16));
        assert!(all_default_features.contains(crate::backend::ArmFeature::CSSC));
        assert!(all_default_features.contains(crate::backend::ArmFeature::FP8DOT2));
        assert!(all_default_features.contains(crate::backend::ArmFeature::FAMINMAX));
        assert!(all_default_features.contains(crate::backend::ArmFeature::LUT));
        assert!(all_default_features.contains(crate::backend::ArmFeature::FP8));
        assert!(all_default_features.contains(crate::backend::ArmFeature::FP8FMA));
        assert!(all_default_features.contains(crate::backend::ArmFeature::SYSTEM));
        assert_eq!(
            all_default_features.bits(),
            crate::backend::ArmFeature::BASE.bits()
                | crate::backend::ArmFeature::FP.bits()
                | crate::backend::ArmFeature::SIMD.bits()
                | crate::backend::ArmFeature::CRYPTO.bits()
                | crate::backend::ArmFeature::ATOMICS.bits()
                | crate::backend::ArmFeature::SYSTEM.bits()
                | crate::backend::ArmFeature::SVE.bits()
                | crate::backend::ArmFeature::SME.bits()
                | crate::backend::ArmFeature::RCPC.bits()
                | crate::backend::ArmFeature::PAUTH.bits()
                | crate::backend::ArmFeature::BTI.bits()
                | crate::backend::ArmFeature::MTE.bits()
                | crate::backend::ArmFeature::BF16.bits()
                | crate::backend::ArmFeature::CSSC.bits()
                | crate::backend::ArmFeature::FP8DOT2.bits()
                | crate::backend::ArmFeature::FAMINMAX.bits()
                | crate::backend::ArmFeature::LUT.bits()
                | crate::backend::ArmFeature::FP8.bits()
                | crate::backend::ArmFeature::FP8FMA.bits()
        );
    }

    #[test]
    fn test_aarch64_default_decode_keeps_broad_feature_set() {
        let handler = ArmHandler::new();
        let (instr, size) = handler
            .disassemble(&[0x00, 0x28, 0x20, 0x1e], "aarch64", 0)
            .expect("default decode should enable scalar FP instructions");

        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "fadd");
    }

    #[test]
    fn test_aarch64_default_config_keeps_broad_feature_set() {
        let mut dispatcher = robustone_core::ArchitectureDispatcher::new();
        dispatcher.register(Box::new(ArmHandler::new()));
        let config = robustone_core::DecodeConfig::new(robustone_core::Mode::AArch64);

        let (instr, size) = dispatcher
            .disassemble_bytes_with_config(&[0x00, 0x28, 0x20, 0x1e], &config, 0)
            .expect("empty DecodeConfig features should use default AArch64 features");

        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "fadd");
    }

    #[test]
    fn test_aarch64_profile_rejects_disabled_fp_extension() {
        let handler = ArmHandler::new();
        let profile = ArchitectureProfile {
            architecture: robustone_core::architecture::Architecture::AArch64,
            mode_name: "aarch64",
            bit_width: 64,
            endianness: robustone_core::utils::Endianness::Little,
            enabled_extensions: vec!["BASE"],
        };

        let error = handler
            .decode_instruction_with_profile(&[0x00, 0x28, 0x20, 0x1e], &profile, 0)
            .expect_err("scalar FP instruction should require the FP extension");

        assert_eq!(error.stable_kind(), "unsupported_extension");
    }

    #[test]
    fn test_aarch64_disassemble_profile_rejects_disabled_fp_extension() {
        let handler = ArmHandler::new();
        let profile = ArchitectureProfile {
            architecture: robustone_core::architecture::Architecture::AArch64,
            mode_name: "aarch64",
            bit_width: 64,
            endianness: robustone_core::utils::Endianness::Little,
            enabled_extensions: vec!["BASE"],
        };

        let error = handler
            .disassemble_with_profile(&[0x00, 0x28, 0x20, 0x1e], &profile, 0)
            .expect_err("profile disassembly should enforce disabled FP extension");

        assert_eq!(error.stable_kind(), "unsupported_extension");
    }

    #[test]
    fn test_aarch64_profile_allows_enabled_fp_extension() {
        let handler = ArmHandler::new();
        let profile = ArchitectureProfile {
            architecture: robustone_core::architecture::Architecture::AArch64,
            mode_name: "aarch64",
            bit_width: 64,
            endianness: robustone_core::utils::Endianness::Little,
            enabled_extensions: vec!["BASE", "FP"],
        };

        let (decoded, size) = handler
            .decode_instruction_with_profile(&[0x00, 0x28, 0x20, 0x1e], &profile, 0)
            .expect("profile with FP should decode scalar FP instruction");

        assert_eq!(size, 4);
        assert_eq!(decoded.mnemonic, "fadd");
    }

    #[test]
    fn test_aarch64_profiles_treat_base_as_implicit() {
        let handler = ArmHandler::new();
        let profile = ArchitectureProfile {
            architecture: robustone_core::architecture::Architecture::AArch64,
            mode_name: "aarch64",
            bit_width: 64,
            endianness: robustone_core::utils::Endianness::Little,
            enabled_extensions: vec!["FP"],
        };

        let (decoded, size) = handler
            .decode_instruction_with_profile(&[0x1F, 0x20, 0x03, 0xD5], &profile, 0)
            .expect("AArch64 BASE is implicit for explicit extension profiles");

        assert_eq!(size, 4);
        assert_eq!(decoded.mnemonic, "nop");
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
        assert_eq!(operands, "v0.8b, v1.8b, v2.8b, #3");
    }

    #[test]
    fn test_ext_16b() {
        // ext v0.16b, v1.16b, v2.16b, #3 => bytes [0x20, 0x18, 0x02, 0x6e]
        let (mnemonic, operands) = disasm_simd(&[0x20, 0x18, 0x02, 0x6e]);
        assert_eq!(mnemonic, "ext");
        assert_eq!(operands, "v0.16b, v1.16b, v2.16b, #3");
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
