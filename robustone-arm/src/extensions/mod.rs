//! AArch64 instruction set extension modules.
//!
//! Organized by instruction family. Each module exposes decode functions
//! that return `(Mnemonic, Vec<Operand>)` for a given 32-bit word.

use crate::types::{AArch64Extensions, Mnemonic};
use robustone_core::ir::Operand;
use robustone_core::types::error::DisasmError;

pub mod base;
pub mod branch;
pub mod system;

/// Decode result type alias.
pub type DecodeResult = Result<(Mnemonic, Vec<Operand>), DisasmError>;

/// Decode dispatch table: maps `op0` value to the appropriate decoder function.
pub fn decode_by_op0(word: u32, addr: u64, _extensions: &AArch64Extensions) -> DecodeResult {
    use crate::shared::encoding::op0;
    match op0(word) {
        0x0 | 0x1 => base::decode_data_proc_imm(word, addr),
        0x2 | 0xA | 0xB => branch::decode_branch_system(word, addr),
        0x4 | 0x8 | 0x9 => base::decode_data_proc_reg(word, addr),
        0x3 | 0xC | 0xD => {
            // Loads and stores — Stage 2
            Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "loads/stores not yet implemented in stage 1",
            ))
        }
        0x5 | 0x6 | 0xE => {
            // SIMD/FP — Stage 3
            Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "SIMD/FP not yet implemented in stage 1",
            ))
        }
        0x7 | 0xF => {
            // SVE / reserved
            Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "SVE not yet implemented",
            ))
        }
        _ => Err(DisasmError::decode_failure(
            robustone_core::types::error::DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            format!("unrecognized op0=0x{:x}", op0(word)),
        )),
    }
}
