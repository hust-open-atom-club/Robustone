//! AArch64 System instructions (placeholder for stage 1).
//!
//! NOP, SVC, barriers are handled in `branch.rs` because they share the
//! same `op0=0x2` encoding space. This module is reserved for future
//! MSR/MRS system register moves and AT/DC/IC cache maintenance.

use crate::extensions::DecodeResult;
use robustone_core::types::error::DisasmError;

/// Decode system register instructions (MSR, MRS).
/// Stage 1: returns unimplemented.
pub fn decode_system_register(_word: u32) -> DecodeResult {
    Err(DisasmError::decode_failure(
        robustone_core::types::error::DecodeErrorKind::UnimplementedInstruction,
        Some("aarch64".to_string()),
        "System register moves not in stage 1",
    ))
}
