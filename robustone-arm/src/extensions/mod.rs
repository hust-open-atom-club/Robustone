//! AArch64 instruction set extension modules.
//!
//! Organized by instruction family. Each module exposes decode functions
//! that return `(Mnemonic, Vec<Operand>)` for a given 32-bit word.

use crate::types::{AArch64Extensions, Mnemonic};
use robustone_core::ir::Operand;
use robustone_core::types::error::DisasmError;

pub mod base;
pub mod branch;
pub mod loads_stores;
pub mod simd_fp;
pub mod system;

/// Decode result type alias.
pub type DecodeResult = Result<(Mnemonic, Vec<Operand>), DisasmError>;

/// Decode dispatch table: maps `op0` value to the appropriate decoder function.
///
/// Follows ARM ARM Table C4-1 classification:
/// - `00xx` (0x0-0x3): Unallocated
/// - `100x` (0x8, 0x9): Data Processing — Immediate
/// - `101x` (0xA, 0xB): Branches, Exception Generating and System
/// - `x1x0` (0x4, 0x6, 0xC, 0xE): Loads and Stores
/// - `x101` (0x5, 0xD): Data Processing — Register
/// - `x111` (0x7, 0xF): SIMD/FP
pub fn decode_by_op0(word: u32, addr: u64, _extensions: &AArch64Extensions) -> DecodeResult {
    use crate::shared::encoding::op0;
    match op0(word) {
        0x8 | 0x9 => base::decode_data_proc_imm(word, addr),
        0xA | 0xB => branch::decode_branch_system(word, addr),
        0x5 | 0xD => base::decode_data_proc_reg(word, addr),
        0x4 | 0x6 | 0xC | 0xE => {
            // Loads and stores — Stage 2
            loads_stores::decode_loads_stores(word, addr)
        }
        0x7 | 0xF => {
            // SIMD/FP / Scalar FP — Stage 3
            simd_fp::decode_simd_fp(word, addr)
        }
        0x0..=0x3 => {
            // UNALLOCATED
            Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "unallocated encoding",
            ))
        }
        _ => Err(DisasmError::decode_failure(
            robustone_core::types::error::DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            format!("unrecognized op0=0x{:x}", op0(word)),
        )),
    }
}
