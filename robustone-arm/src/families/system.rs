//! System instructions: HINT, NOP, etc.

use robustone_core::{
    ir::Operand,
    types::error::{DecodeErrorKind, DisasmError},
};

/// Decode system instructions.
pub fn decode_system(word: u32) -> Result<(&'static str, Vec<Operand>), DisasmError> {
    // HINT: 11010101000 | op2(3) | CRm(4) | op1(3) | op0(2) | Rt(5)
    // NOP is HINT #0: 0xD503201F
    if word == 0xD503201F {
        // Capstone identifies this as "hint" with operand 0.
        return Ok(("hint", vec![Operand::Immediate { value: 0 }]));
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("aarch64".to_string()),
        detail: format!("unrecognized system encoding 0x{word:08x}"),
    })
}
