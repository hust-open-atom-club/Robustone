//! Data Processing - Immediate: ADD imm, MOVZ, ORR imm.

use robustone_core::{
    ir::{Operand, RegisterId},
    types::error::{DecodeErrorKind, DisasmError},
};

use crate::encoding;

/// Decode data-processing immediate instructions.
pub fn decode_data_processing_immediate(
    word: u32,
) -> Result<(&'static str, Vec<Operand>), DisasmError> {
    // ADD (immediate): sf=1 | op=0 | S=0 | 10001 | shift | imm12 | Rn | Rd
    // Strict mask to avoid matching sub/adds encodings.
    if (word & 0xFF000000) == 0x91000000 {
        let rd = encoding::extract_rd(word);
        let rn = encoding::extract_rn(word);
        let imm12 = encoding::extract_imm12(word);
        let shift = encoding::extract_shift(word);

        let mut ops = vec![
            Operand::Register {
                register: aarch64_reg(rd),
            },
            Operand::Register {
                register: aarch64_reg(rn),
            },
        ];

        if shift == 0 {
            ops.push(Operand::Immediate {
                value: imm12 as i64,
            });
        } else if shift == 1 {
            ops.push(Operand::Immediate {
                value: (imm12 << 12) as i64,
            });
        } else {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::InvalidEncoding,
                architecture: Some("aarch64".to_string()),
                detail: format!("invalid shift for ADD imm: {shift}"),
            });
        }

        return Ok(("add", ops));
    }

    // Logical immediate: sf | opc(2) | 100100 | N | immr | imms | Rn | Rd
    // ORR (immediate): opc=01
    // Require sf=1 (64-bit).
    if (word & 0x1F800000) == 0x12000000 && ((word >> 31) & 1) == 1 {
        let opc = encoding::extract_opc(word);
        if opc == 0b01 {
            let rd = encoding::extract_rd(word);
            let rn = encoding::extract_rn(word);
            // Decode bitmask immediate (simplified)
            let immr = (word >> 16) & 0x3F;
            let imms = (word >> 10) & 0x3F;
            let _n = (word >> 22) & 1;

            let imm_value = decode_bitmask_imm(_n, immr, imms).ok_or_else(|| DisasmError::DecodeFailure {
                kind: DecodeErrorKind::InvalidEncoding,
                architecture: Some("aarch64".to_string()),
                detail: "invalid bitmask immediate encoding".to_string(),
            })?;

            let mut ops = vec![Operand::Register {
                register: aarch64_reg(rd),
            }];

            // When Rn is XZR, this is the MOV alias: mov Rd, #imm
            let mnemonic = if rn == 31 {
                "mov"
            } else {
                ops.push(Operand::Register {
                    register: aarch64_reg(rn),
                });
                "orr"
            };

            ops.push(Operand::Immediate { value: imm_value as i64 });
            return Ok((mnemonic, ops));
        }
    }

    // Move wide (immediate): sf | opc(2) | 100101 | hw | imm16 | Rd
    // MOVZ: opc=10
    // Require sf=1 (64-bit).
    if (word & 0x1F800000) == 0x12800000 && ((word >> 31) & 1) == 1 {
        let opc = encoding::extract_opc(word);
        if opc == 0b10 {
            let rd = encoding::extract_rd(word);
            let imm16 = encoding::extract_imm16(word);
            let hw = (word >> 21) & 0x3;

            let value = if hw == 0 {
                imm16 as i64
            } else {
                (imm16 as i64) << (hw * 16)
            };

            return Ok((
                "mov",
                vec![
                    Operand::Register {
                        register: aarch64_reg(rd),
                    },
                    Operand::Immediate { value },
                ],
            ));
        }
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("aarch64".to_string()),
        detail: format!("unrecognized DP-imm encoding 0x{word:08x}"),
    })
}

/// Decode a bitmask immediate following the AArch64 algorithm.
///
/// Reference: ARM ARM DDI 0487I.a, section "DecodeBitMasks".
fn decode_bitmask_imm(n: u32, immr: u32, imms: u32) -> Option<u64> {
    // len = HighestSetBit(N:NOT(imms)) for a 7-bit value.
    let not_imms = (!imms) & 0x3F;
    let concat = ((n & 1) << 6) | not_imms;
    let mut len = 0;
    for i in (0..7).rev() {
        if (concat >> i) & 1 == 1 {
            len = i;
            break;
        }
    }
    // N:NOT(imms) all zeros is a reserved encoding.
    if concat == 0 {
        return None; // Invalid encoding per ARM ARM.
    }
    // Element size must be at least 2 bits (len >= 1).
    if len < 1 {
        return None; // Invalid encoding per ARM ARM.
    }

    // Defensive check: N must be 0 when len < 6, and N must be 1 when len == 6.
    if (len < 6 && n != 0) || (len == 6 && n != 1) {
        return None; // Invalid encoding per ARM ARM.
    }

    // size = 2^len
    let size = 1u64 << len;
    let levels = size - 1;

    let s = (imms as u64) & levels;
    let r = (immr as u64) & levels;

    // S == levels is a reserved encoding for every element size.
    if s == levels {
        return None; // Invalid encoding per ARM ARM.
    }

    // d = S + 1 consecutive ones
    let d = s + 1;
    let mut welem = (1u64 << d) - 1;

    // Rotate right by r within the element size
    if r > 0 {
        let size_mask = if size == 64 {
            u64::MAX
        } else {
            (1u64 << size) - 1
        };
        welem = ((welem >> r) | (welem << (size - r))) & size_mask;
    }

    // Replicate to fill 64 bits
    let mut result = 0u64;
    let mut i = 0u64;
    while i < 64 {
        result |= welem << i;
        i += size;
    }

    Some(result)
}

fn aarch64_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: robustone_core::ir::ArchitectureId::Arm,
        id,
    }
}
