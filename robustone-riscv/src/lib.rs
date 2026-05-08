#![forbid(unsafe_code)]

//! RISC-V disassembly module.
//!
//! Provides instruction decoding for RISC-V, including:
//! - 32-bit and 64-bit profiles
//! - Standard and compressed (RVC) encodings
//! - Core ISA extensions (I, M, A, F, D, C)
//!
//! This module implements the generic `Architecture` trait for RISC-V
//! and provides both modern architecture-aware interfaces and legacy
//! compatibility interfaces.

pub mod aliases;
pub mod arch;
pub mod backend;
// Legacy decoder modules removed — shared backend is now the sole decode path.
pub mod printer;
pub mod render;
pub mod shared;
pub mod types;

pub mod architecture {
    pub use robustone_core::architecture::*;
}

pub mod common {
    pub use robustone_core::common::*;
}

pub mod ir {
    pub use robustone_core::ir::*;
}

pub mod utils {
    pub use robustone_core::utils::*;
}

pub mod riscv {
    pub use crate::arch;
    pub use crate::printer;
    pub use crate::shared;
    pub use crate::types;
}

pub use robustone_core::Instruction;

use arch::RiscVInstructionDetail;
use backend::Xlen;
use robustone_core::{
    common::ArchitectureProfile,
    ir::{DecodedInstruction, TextRenderProfile},
    traits::ArchitectureHandler,
    traits::instruction::Detail,
    types::error::DisasmError,
};
use robustone_isa::DecodeProfile;

/// Architecture handler implementation for RISC-V targets.
pub struct RiscVHandler {
    rv32_features: crate::backend::RiscVFeature,
    rv64_features: crate::backend::RiscVFeature,
    configured_xlen: Option<Xlen>,
    detail: bool,
}

impl RiscVHandler {
    /// Creates a new handler with both RV32GC and RV64GC profiles.
    pub fn new() -> Self {
        Self {
            rv32_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            rv64_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            configured_xlen: None,
            detail: true,
        }
    }

    /// Creates a handler targeting RV32GC.
    pub fn rv32() -> Self {
        Self {
            rv32_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            rv64_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            configured_xlen: Some(Xlen::X32),
            detail: true,
        }
    }

    /// Creates a handler targeting RV64GC.
    pub fn rv64() -> Self {
        Self {
            rv32_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            rv64_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
            configured_xlen: Some(Xlen::X64),
            detail: true,
        }
    }

    /// Creates a handler with custom XLEN and extension flags.
    pub fn with_extensions(xlen: Xlen, extensions: &[&str]) -> Self {
        let features = crate::backend::RiscVFeature::from_extension_names(extensions);
        match xlen {
            Xlen::X32 => Self {
                rv32_features: features,
                rv64_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
                configured_xlen: Some(Xlen::X32),
                detail: true,
            },
            Xlen::X64 => Self {
                rv32_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
                rv64_features: features,
                configured_xlen: Some(Xlen::X64),
                detail: true,
            },
        }
    }

    fn features_for_arch(
        &self,
        arch_name: &str,
    ) -> Result<(Xlen, crate::backend::RiscVFeature), DisasmError> {
        match (self.configured_xlen, arch_name) {
            (Some(Xlen::X32), "riscv32") => Ok((Xlen::X32, self.rv32_features)),
            (Some(Xlen::X64), "riscv64" | "riscv") => Ok((Xlen::X64, self.rv64_features)),
            (Some(_), _) => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
            (None, "riscv32") => Ok((Xlen::X32, self.rv32_features)),
            (None, "riscv64" | "riscv") => Ok((Xlen::X64, self.rv64_features)),
            _ => Err(DisasmError::UnsupportedArchitecture(arch_name.to_string())),
        }
    }

    pub fn from_profile(profile: &ArchitectureProfile) -> Result<Self, DisasmError> {
        let exts: Vec<&str> = profile.enabled_extensions.to_vec();
        // D extension requires F extension.
        if exts.contains(&"D") && !exts.contains(&"F") {
            return Err(robustone_core::types::error::DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::UnsupportedExtension,
                Some(profile.mode_name.to_string()),
                "D extension requires F extension".to_string(),
            ));
        }
        let features = crate::backend::RiscVFeature::from_extension_names(
            profile.enabled_extensions.as_slice(),
        );
        match (&profile.architecture, profile.mode_name) {
            (crate::architecture::Architecture::RiscV32, "riscv64") => {
                return Err(robustone_core::types::error::DisasmError::decode_failure(
                    robustone_core::types::error::DecodeErrorKind::UnsupportedMode,
                    Some(profile.mode_name.to_string()),
                    "RV32 profile cannot use riscv64 mode".to_string(),
                ));
            }
            (crate::architecture::Architecture::RiscV64, "riscv32") => {
                return Err(robustone_core::types::error::DisasmError::decode_failure(
                    robustone_core::types::error::DecodeErrorKind::UnsupportedMode,
                    Some(profile.mode_name.to_string()),
                    "RV64 profile cannot use riscv32 mode".to_string(),
                ));
            }
            _ => {}
        }
        match &profile.architecture {
            crate::architecture::Architecture::RiscV32 => Ok(Self {
                rv32_features: features,
                rv64_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
                configured_xlen: Some(Xlen::X32),
                detail: true,
            }),
            crate::architecture::Architecture::RiscV64 => Ok(Self {
                rv32_features: crate::backend::RiscVFeature::G | crate::backend::RiscVFeature::C,
                rv64_features: features,
                configured_xlen: Some(Xlen::X64),
                detail: true,
            }),
            other => Err(DisasmError::UnsupportedArchitecture(
                other.as_str().to_string(),
            )),
        }
    }
}

impl Default for RiscVHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for RiscVHandler {
    fn set_detail(&mut self, detail: bool) {
        self.detail = detail;
    }

    fn renderer(&self) -> Option<&dyn robustone_core::renderer::Renderer> {
        Some(&crate::render::RiscVRenderer)
    }

    fn decode_instruction(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let (xlen, features) = self.features_for_arch(arch_name)?;
        let mode = match xlen {
            Xlen::X32 => crate::backend::RiscVMode::RV32,
            Xlen::X64 => crate::backend::RiscVMode::RV64,
        };

        // Compressed instructions require the C extension globally.
        if bytes.len() >= 2
            && (bytes[0] & 0x3) != 0x3
            && !features.contains(crate::backend::RiscVFeature::C)
        {
            return Err(robustone_core::types::error::DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::UnsupportedExtension,
                Some(arch_name.to_string()),
                "compressed instruction requires C extension".to_string(),
            ));
        }

        let profile = DecodeProfile {
            mode,
            features,
            render_dialect: robustone_isa::RenderDialect::Canonical,
            alias_policy: robustone_isa::AliasPolicy::None,
        };

        let mut decoded = match robustone_isa::decode_one::<crate::backend::RiscVBackend>(
            bytes, addr, &profile,
        ) {
            Ok(d) => d,
            Err(robustone_core::types::error::DisasmError::DecodeFailure {
                kind,
                architecture: _,
                detail,
            }) => {
                return Err(robustone_core::types::error::DisasmError::DecodeFailure {
                    kind,
                    architecture: Some(arch_name.to_string()),
                    detail,
                });
            }
            Err(e) => return Err(e),
        };
        // Compressed encoding constraints (rd != x0, rs1 != x0) are now
        // validated in decode_one() via EncodingConstraint on each spec.
        decoded.mode = arch_name.to_string();
        crate::aliases::apply_riscv_aliases(&mut decoded);
        let size = decoded.size;
        Ok((decoded, size))
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        let handler = Self::from_profile(profile)?;
        handler.decode_instruction(bytes, profile.mode_name, addr)
    }

    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let (ir, _size) = self.decode_instruction(bytes, arch_name, addr)?;
        let (mnemonic, operands) = crate::render::render_riscv_text_parts(
            &ir,
            TextRenderProfile::Compat,
            true,
            true,
            true,
            false,
        );

        let detail: Option<Box<dyn Detail>> = if self.detail {
            let mut riscv_detail = RiscVInstructionDetail::new();
            for register in ir
                .registers_read
                .iter()
                .chain(ir.implicit_registers_read.iter())
            {
                if !riscv_detail.regs_read.contains(&register.id) {
                    riscv_detail = riscv_detail.reads_register(register.id);
                }
            }
            for register in ir
                .registers_written
                .iter()
                .chain(ir.implicit_registers_written.iter())
            {
                if !riscv_detail.regs_write.contains(&register.id) {
                    riscv_detail = riscv_detail.writes_register(register.id);
                }
            }
            Some(Box::new(riscv_detail))
        } else {
            None
        };

        let size = ir.size;
        let instruction = Instruction::from_decoded(ir, mnemonic, operands, detail);
        Ok((instruction, size))
    }

    fn disassemble_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let mut handler = Self::from_profile(profile)?;
        handler.set_detail(self.detail);
        handler.disassemble(bytes, profile.mode_name, addr)
    }

    fn name(&self) -> &'static str {
        "riscv"
    }

    fn supports(&self, arch_name: &str) -> bool {
        match self.configured_xlen {
            Some(Xlen::X32) => matches!(arch_name, "riscv32"),
            Some(Xlen::X64) => matches!(arch_name, "riscv64" | "riscv"),
            None => matches!(arch_name, "riscv32" | "riscv64" | "riscv"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::types::{Access, RiscVRegister};

    #[test]
    fn test_riscv_handler_creation() {
        let handler = RiscVHandler::new();
        assert_eq!(handler.name(), "riscv");
        assert!(handler.supports("riscv32"));
        assert!(handler.supports("riscv64"));
        assert!(handler.supports("riscv"));
        assert!(!handler.supports("arm"));
    }

    #[test]
    fn test_with_extensions_limits_supported_architectures() {
        let handler = RiscVHandler::with_extensions(Xlen::X32, &["i", "m", "a", "f", "d", "c"]);
        assert!(handler.supports("riscv32"));
        assert!(!handler.supports("riscv64"));
        assert!(!handler.supports("riscv"));

        let error = handler
            .disassemble(&[0x83, 0x30, 0x00, 0x00], "riscv64", 0)
            .expect_err("RV32-only handler should reject riscv64 requests");
        assert!(matches!(error, DisasmError::UnsupportedArchitecture(_)));
    }

    #[test]
    fn test_riscv_register_names() {
        assert_eq!(RiscVRegister::X0.name(), "zero");
        assert_eq!(RiscVRegister::X1.name(), "ra");
        assert_eq!(RiscVRegister::X2.name(), "sp");
        assert_eq!(RiscVRegister::X5.name(), "t0");
        assert_eq!(RiscVRegister::X10.name(), "a0");
    }

    #[test]
    fn test_riscv_register_from_id() {
        assert_eq!(RiscVRegister::from_id(0), RiscVRegister::X0);
        assert_eq!(RiscVRegister::from_id(1), RiscVRegister::X1);
        assert_eq!(RiscVRegister::from_id(32), RiscVRegister::F0_32);
        assert_eq!(RiscVRegister::from_id(64), RiscVRegister::F0_64);
        assert_eq!(RiscVRegister::from_id(95), RiscVRegister::F31_64);
        assert_eq!(RiscVRegister::from_id(100), RiscVRegister::Invalid);
    }

    #[test]
    fn test_access_types() {
        let read_access = Access::read();
        assert!(read_access.read && !read_access.write);

        let write_access = Access::write();
        assert!(!write_access.read && write_access.write);

        let rw_access = Access::read_write();
        assert!(rw_access.read && rw_access.write);

        let none_access = Access::none();
        assert!(!none_access.read && !none_access.write);
    }

    #[test]
    fn test_atomic_doubleword_is_not_tagged_as_floating_point() {
        let handler = RiscVHandler::rv64();
        let (decoded, _) = handler
            .decode_instruction(&[0x2f, 0xb4, 0x02, 0x12], "riscv64", 0)
            .expect("lr.d should decode");

        assert!(
            decoded
                .groups
                .contains(&robustone_core::ir::InstructionGroup::Atomic)
        );
        assert!(
            !decoded
                .groups
                .contains(&robustone_core::ir::InstructionGroup::Float)
        );
    }

    #[test]
    fn test_disassemble_merges_implicit_register_writes_into_detail() {
        let handler = RiscVHandler::rv32();
        let (instruction, size) = handler
            .disassemble(&[0x85, 0x20], "riscv32", 0)
            .expect("c.jal should decode");

        assert_eq!(size, 2);
        assert_eq!(instruction.mnemonic, "jal");

        let detail = instruction.detail.expect("detail should be populated");
        assert_eq!(detail.registers_written(), &[1]);
    }

    #[test]
    fn test_thead_mveqz_decode() {
        let handler = RiscVHandler::with_extensions(Xlen::X32, &["I", "THEAD"]);
        // th.mveqz x1, x2, x3 → funct7=0x20, funct3=0x1, opcode=0x0B
        // word = (0x20 << 25) | (3 << 20) | (2 << 15) | (1 << 7) | (0x1 << 12) | 0x0B
        let word: u32 = 0x4000_0000 | (3 << 20) | (2 << 15) | (1 << 7) | (0x1 << 12) | 0x0B;
        let bytes = word.to_le_bytes();
        let (decoded, size) = handler
            .decode_instruction(&bytes, "riscv32", 0x1000)
            .expect("th.mveqz should decode");
        assert_eq!(size, 4);
        assert_eq!(decoded.mnemonic, "th.mveqz");
        assert_eq!(decoded.registers_written.len(), 1);
        assert_eq!(decoded.registers_read.len(), 2);
    }

    #[test]
    fn test_thead_mvnez_decode() {
        let handler = RiscVHandler::with_extensions(Xlen::X64, &["I", "THEAD"]);
        // th.mvnez x4, x5, x6 → funct7=0x21, funct3=0x1, opcode=0x0B
        let word: u32 = 0x4200_0000 | (6 << 20) | (5 << 15) | (4 << 7) | (0x1 << 12) | 0x0B;
        let bytes = word.to_le_bytes();
        let (decoded, size) = handler
            .decode_instruction(&bytes, "riscv64", 0x2000)
            .expect("th.mvnez should decode");
        assert_eq!(size, 4);
        assert_eq!(decoded.mnemonic, "th.mvnez");
        assert_eq!(decoded.registers_written.len(), 1);
        assert_eq!(decoded.registers_read.len(), 2);
    }

    #[test]
    fn test_thead_requires_feature() {
        let handler = RiscVHandler::rv32();
        let word: u32 = 0x4000_0000 | (3 << 20) | (2 << 15) | (1 << 7) | (0x1 << 12) | 0x0B;
        let bytes = word.to_le_bytes();
        let result = handler.decode_instruction(&bytes, "riscv32", 0x1000);
        assert!(
            result.is_err(),
            "th.mveqz should fail without THEAD feature"
        );
    }

    #[test]
    fn beq_b_type_compose_positive_offset() {
        let handler = RiscVHandler::rv64();
        // beq x0, x0, +0x100: offs=0x80, B-type compose
        // imm[12]=0, imm[10:5]=0x10 (bits 30:25), imm[4:1]=0 (bits 11:8), imm[11]=0 (bit 7)
        // word: opcode=0x63, funct3=0x0, rs1=0, rs2=0
        let offset: u32 = 0x80;
        let imm12 = (offset >> 12) & 1;
        let imm105 = (offset >> 5) & 0x3F;
        let imm41 = (offset >> 1) & 0xF;
        let imm11 = (offset >> 11) & 1;
        let word: u32 = ((imm12 << 31) | (imm105 << 25)) | (imm41 << 8) | (imm11 << 7) | 0x63;
        let bytes = word.to_le_bytes();
        let (decoded, _) = handler
            .decode_instruction(&bytes, "riscv64", 0x1000)
            .expect("beq should decode");
        assert_eq!(decoded.mnemonic, "beq");
        match &decoded.operands[2] {
            robustone_core::ir::Operand::Immediate { value, .. } => {
                assert_eq!(*value, 0x80); // raw composed immediate value (no shift)
            }
            other => panic!("expected immediate operand, got {:?}", other),
        }
    }

    #[test]
    fn jal_j_type_compose_offset() {
        let handler = RiscVHandler::rv64();
        // jal x1, -4: offs=-2, J-type compose
        // imm[20]=1, imm[10:1]=0x3FF (all ones), imm[11]=1, imm[19:12]=0xFF (all ones)
        let word: u32 = (1u32 << 31) | (0x3FF << 21) | (1 << 20) | (0xFF << 12) | (1 << 7) | 0x6F;
        let bytes = word.to_le_bytes();
        let (decoded, _) = handler
            .decode_instruction(&bytes, "riscv64", 0x1000)
            .expect("jal should decode");
        assert_eq!(decoded.mnemonic, "jal");
        match &decoded.operands[1] {
            robustone_core::ir::Operand::Immediate { value, .. } => {
                assert_eq!(*value, -2); // raw composed immediate value (no shift)
            }
            other => panic!("expected immediate operand, got {:?}", other),
        }
    }
}

// Register the RISC-V handler with the global inventory.
inventory::submit! {
    robustone_core::traits::HandlerFactory::new(|| Box::new(RiscVHandler::new()))
}
