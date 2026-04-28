//! Branch instructions: B, BL, BR, RET.

use robustone_core::{
    ir::{Operand, RegisterId},
    types::error::{DecodeErrorKind, DisasmError},
};

use crate::encoding;

/// Decode branch instructions.
pub fn decode_branches(word: u32) -> Result<(&'static str, Vec<Operand>), DisasmError> {
    // B / BL (unconditional immediate): op(1) | 00101 | imm26
    // The imm26 is a word offset; multiply by 4 to get byte offset.
    if (word & 0xFC000000) == 0x14000000 {
        let imm26 = encoding::extract_imm26(word);
        let target = sign_extend_26(imm26) << 2;
        return Ok(("b", vec![Operand::Immediate { value: target }]));
    }
    if (word & 0xFC000000) == 0x94000000 {
        let imm26 = encoding::extract_imm26(word);
        let target = sign_extend_26(imm26) << 2;
        return Ok(("bl", vec![Operand::Immediate { value: target }]));
    }

    // Branch to register: 1101011 | Z | op1(3) | op2(5) | op3(6) | Rn(5) | Rd(5)
    if (word & 0xFE000000) == 0xD6000000 {
        let op1 = (word >> 21) & 0x7;
        let op2 = (word >> 16) & 0x1F;
        let op3 = (word >> 10) & 0x3F;
        let rn = encoding::extract_rn(word);
        let rd = encoding::extract_rd(word);

        // BR: op1=000, op2=11111, op3=000000, Rd=00000
        if op1 == 0 && op2 == 0x1F && op3 == 0 && rd == 0 {
            return Ok((
                "br",
                vec![Operand::Register {
                    register: aarch64_reg(rn),
                }],
            ));
        }

        // RET: op1=010, op2=11111, op3=000000, Rd=00000
        // Capstone shows "ret" without operands, but includes the target register in detail.
        if op1 == 0b010 && op2 == 0x1F && op3 == 0 && rd == 0 {
            return Ok((
                "ret",
                vec![Operand::Register {
                    register: aarch64_reg(rn),
                }],
            ));
        }
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("aarch64".to_string()),
        detail: format!("unrecognized branch encoding 0x{word:08x}"),
    })
}

fn sign_extend_26(imm26: u32) -> i64 {
    let val = imm26 as i64;
    if val & (1 << 25) != 0 {
        val | !((1 << 26) - 1)
    } else {
        val
    }
}

fn aarch64_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: robustone_core::ir::ArchitectureId::Arm,
        id,
    }
}
