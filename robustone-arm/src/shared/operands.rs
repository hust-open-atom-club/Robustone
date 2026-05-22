//! Operand construction helpers for AArch64 instructions.

use robustone_core::ir::Operand;

/// Create an immediate operand.
pub fn imm(value: i64) -> Operand {
    Operand::Immediate { value }
}

/// Create a text operand (for condition codes, shift types, etc.).
pub fn text(value: impl Into<String>) -> Operand {
    Operand::Text {
        value: value.into(),
    }
}

/// Create a memory operand with base register and displacement.
pub fn mem_base_displ(base: Option<u8>, displacement: i64) -> Operand {
    use crate::shared::registers::reg_id;
    Operand::Memory {
        base: base.map(reg_id),
        displacement,
    }
}

/// Create a PC-relative address operand.
pub fn pcrel_addr(offset: i64) -> Operand {
    if offset == 0 {
        Operand::Text {
            value: "0".to_string(),
        }
    } else {
        Operand::Text {
            value: format!("0x{offset:x}"),
        }
    }
}

/// Create a label operand for branch targets.
pub fn label(addr: u64) -> Operand {
    if addr == 0 {
        Operand::Text {
            value: "0".to_string(),
        }
    } else {
        Operand::Text {
            value: format!("0x{addr:x}"),
        }
    }
}
