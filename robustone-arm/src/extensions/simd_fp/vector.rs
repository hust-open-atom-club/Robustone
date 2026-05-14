//! Advanced SIMD vector data-processing instructions.

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::registers::{arrangement_suffix, FpRegSize};
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};


// ---------------------------------------------------------------------------
// Advanced SIMD sub-decoders
// ---------------------------------------------------------------------------

/// Decode SIMD Three Same (integer and FP32/64 vector).
///
/// Encoding: b24=0, b21=1, b10=1.
/// Fields: U=bit(29), size=bits(23:22), opcode=bits(15:11), Q=bit(30).
pub fn decode_simd_three_same(word: u32) -> DecodeResult {
    let u = u_bit(word);
    let size = simd_size(word);
    let opcode = bits(word, 15, 11) as u8;
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);

    // Integer opcodes (0-23)
    if opcode <= 23 {
        let mnemonic = match (u, opcode) {
            // Opcode 0: shadd / uhadd
            (false, 0) => Mnemonic::Shadd,
            (true, 0) => Mnemonic::Uhadd,

            // Opcode 1: sqadd / uqadd
            (false, 1) => Mnemonic::Sqadd,
            (true, 1) => Mnemonic::Uqadd,

            // Opcode 2: srhadd / urhadd
            (false, 2) => Mnemonic::Srhadd,
            (true, 2) => Mnemonic::Urhadd,

            // Opcode 3: size-dependent bitwise
            (false, 3) => match size {
                0b00 => Mnemonic::And,
                0b01 => Mnemonic::Bic,
                0b10 => Mnemonic::Orr,
                0b11 => Mnemonic::Orn,
                _ => unreachable!(),
            },
            (true, 3) => match size {
                0b00 => Mnemonic::Eor,
                0b01 => Mnemonic::Bsl,
                0b10 => Mnemonic::Bit,
                0b11 => Mnemonic::Bif,
                _ => unreachable!(),
            },
            // Opcode 4: shsub / uhsub
            (false, 4) => Mnemonic::Shsub,
            (true, 4) => Mnemonic::Uhsub,

            // Opcode 5: sqsub / uqsub
            (false, 5) => Mnemonic::Sqsub,
            (true, 5) => Mnemonic::Uqsub,

            // Opcode 6: cmgt / cmhi
            (false, 6) => Mnemonic::Cmgt,
            (true, 6) => Mnemonic::Cmhi,

            // Opcode 7: cmge / cmhs
            (false, 7) => Mnemonic::Cmge,
            (true, 7) => Mnemonic::Cmhs,

            // Opcode 8: sshl / ushl
            (false, 8) => Mnemonic::Sshl,
            (true, 8) => Mnemonic::Ushl,

            // Opcode 9: sqshl / uqshl
            (false, 9) => Mnemonic::Sqshl,
            (true, 9) => Mnemonic::Uqshl,

            // Opcode 10: srshl / urshl
            (false, 10) => Mnemonic::Srshl,
            (true, 10) => Mnemonic::Urshl,

            // Opcode 11: sqrshl / uqrshl
            (false, 11) => Mnemonic::Sqrshl,
            (true, 11) => Mnemonic::Uqrshl,

            // Opcode 12: smax / umax
            (false, 12) => Mnemonic::Smax,
            (true, 12) => Mnemonic::Umax,

            // Opcode 13: smin / umin
            (false, 13) => Mnemonic::Smin,
            (true, 13) => Mnemonic::Umin,

            // Opcode 14: sabd / uabd
            (false, 14) => Mnemonic::Sabd,
            (true, 14) => Mnemonic::Uabd,

            // Opcode 15: saba / uaba
            (false, 15) => Mnemonic::Saba,
            (true, 15) => Mnemonic::Uaba,

            // Opcode 16: add / sub
            (false, 16) => Mnemonic::Add,
            (true, 16) => Mnemonic::Sub,

            // Opcode 17: cmtst / cmeq
            (false, 17) => Mnemonic::Cmtst,
            (true, 17) => Mnemonic::Cmeq,

            // Opcode 18: mla / mls
            (false, 18) => Mnemonic::Mla,
            (true, 18) => Mnemonic::Mls,

            // Opcode 19: mul / pmul (only size=00)
            (false, 19) => Mnemonic::Mul,
            (true, 19) => {
                if size == 0b00 {
                    Mnemonic::Pmul
                } else {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::UnimplementedInstruction,
                        Some("aarch64".to_string()),
                        format!("pmul requires size=00, got {size:02b}"),
                    ));
                }
            }
            // Opcode 20: smaxp / umaxp
            (false, 20) => Mnemonic::Smaxp,
            (true, 20) => Mnemonic::Umaxp,

            // Opcode 21: sminp / uminp
            (false, 21) => Mnemonic::Sminp,
            (true, 21) => Mnemonic::Uminp,

            // Opcode 22: sqdmulh (only size=01,10) / sqrdmulh (only size=01,10)
            (false, 22) => {
                if size == 0b01 || size == 0b10 {
                    Mnemonic::Sqdmulh
                } else {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::UnimplementedInstruction,
                        Some("aarch64".to_string()),
                        format!("sqdmulh requires size=01 or 10, got {size:02b}"),
                    ));
                }
            }
            (true, 22) => {
                if size == 0b01 || size == 0b10 {
                    Mnemonic::Sqrdmulh
                } else {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::UnimplementedInstruction,
                        Some("aarch64".to_string()),
                        format!("sqrdmulh requires size=01 or 10, got {size:02b}"),
                    ));
                }
            }
            // Opcode 23: addp / faddp (fp)
            (false, 23) => Mnemonic::Addp,
            (true, 23) => Mnemonic::Faddp,
            _ => {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::UnimplementedInstruction,
                    Some("aarch64".to_string()),
                    format!("Unimplemented Three Same opcode {opcode} U={u}"),
                ))
            }
        };

        // For bitwise opcodes (opcode 3), the arrangement is always .8b/.16b
        // because these operate on the entire vector as bytes.
        let arr_size = if opcode == 3 { 0b00 } else { size };

        return Ok((
            mnemonic,
            vec![
                super::vec_reg_operand(rd_val, arr_size, q),
                super::vec_reg_operand(rn_val, arr_size, q),
                super::vec_reg_operand(rm_val, arr_size, q),
            ],
        ));
    }

    // FP opcodes 24-31
    // size field behavior: bit23 selects operation, bit22 selects precision
    let mnemonic = match (u, opcode, size) {
        // size=00 (FP32)
        (false, 24, 0b00) => Mnemonic::Fmaxnm,
        (true, 24, 0b00) => Mnemonic::Fmaxnmp,
        (false, 25, 0b00) => Mnemonic::Fmla,
        (true, 25, 0b00) => Mnemonic::Fnmla,
        (false, 26, 0b00) => Mnemonic::Fadd,
        (true, 26, 0b00) => Mnemonic::Faddp,
        (false, 27, 0b00) => Mnemonic::Fmulx,
        (true, 27, 0b00) => Mnemonic::Fmul,
        (false, 28, 0b00) => Mnemonic::Fcmeq,
        (true, 28, 0b00) => Mnemonic::Fcmge,
        (false, 29, 0b00) => Mnemonic::Fmlal,
        (true, 29, 0b00) => Mnemonic::Facge,
        (false, 30, 0b00) => Mnemonic::Fmax,
        (true, 30, 0b00) => Mnemonic::Fmaxp,
        (false, 31, 0b00) => Mnemonic::Frecps,
        (true, 31, 0b00) => Mnemonic::Fdiv,

        // size=01 (FP64, Q=1 only for .2d)
        (false, 24, 0b01) => Mnemonic::Fmaxnm,
        (true, 24, 0b01) => Mnemonic::Fmaxnmp,
        (false, 25, 0b01) => Mnemonic::Fmla,
        (true, 25, 0b01) => Mnemonic::Fnmla,
        (false, 26, 0b01) => Mnemonic::Fadd,
        (true, 26, 0b01) => Mnemonic::Faddp,
        (false, 27, 0b01) => Mnemonic::Fmulx,
        (true, 27, 0b01) => Mnemonic::Fmul,
        (false, 28, 0b01) => Mnemonic::Fcmeq,
        (true, 28, 0b01) => Mnemonic::Fcmge,
        (false, 29, 0b01) => Mnemonic::Fmlal,
        (true, 29, 0b01) => Mnemonic::Facge,
        (false, 30, 0b01) => Mnemonic::Fmax,
        (true, 30, 0b01) => Mnemonic::Fmaxp,
        (false, 31, 0b01) => Mnemonic::Frecps,
        (true, 31, 0b01) => Mnemonic::Fdiv,

        // size=10 (FP32 "inverse")
        (false, 24, 0b10) => Mnemonic::Fminnm,
        (true, 24, 0b10) => Mnemonic::Fminnmp,
        (false, 25, 0b10) => Mnemonic::Fmls,
        (true, 25, 0b10) => Mnemonic::Fnmls,
        (false, 26, 0b10) => Mnemonic::Fsub,
        (true, 26, 0b10) => Mnemonic::Fabd,
        (false, 27, 0b10) => Mnemonic::Famax,
        (true, 27, 0b10) => Mnemonic::Famin,
        (false, 28, 0b10) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "Reserved FP Three Same opcode 28 size=10 U=0".to_string(),
            ))
        }
        (true, 28, 0b10) => Mnemonic::Fcmgt,
        (false, 29, 0b10) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "Reserved FP Three Same opcode 29 size=10 U=0".to_string(),
            ))
        }
        (true, 29, 0b10) => Mnemonic::Facgt,
        (false, 30, 0b10) => Mnemonic::Fmin,
        (true, 30, 0b10) => Mnemonic::Fminp,
        (false, 31, 0b10) => Mnemonic::Frsqrts,
        (true, 31, 0b10) => Mnemonic::Fscale,

        // size=11 (FP64 "inverse")
        (false, 24, 0b11) => Mnemonic::Fminnm,
        (true, 24, 0b11) => Mnemonic::Fminnmp,
        (false, 25, 0b11) => Mnemonic::Fmls,
        (true, 25, 0b11) => Mnemonic::Fnmls,
        (false, 26, 0b11) => Mnemonic::Fsub,
        (true, 26, 0b11) => Mnemonic::Fabd,
        (false, 27, 0b11) => Mnemonic::Famax,
        (true, 27, 0b11) => Mnemonic::Famin,
        (false, 28, 0b11) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "Reserved FP Three Same opcode 28 size=11 U=0".to_string(),
            ))
        }
        (true, 28, 0b11) => Mnemonic::Fcmgt,
        (false, 29, 0b11) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "Reserved FP Three Same opcode 29 size=11 U=0".to_string(),
            ))
        }
        (true, 29, 0b11) => Mnemonic::Facgt,
        (false, 30, 0b11) => Mnemonic::Fmin,
        (true, 30, 0b11) => Mnemonic::Fminp,
        (false, 31, 0b11) => Mnemonic::Frsqrts,
        (true, 31, 0b11) => Mnemonic::Fscale,

        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented Three Same opcode {opcode} U={u} size={size:02b}"),
            ))
        }
    };

    // For FP vector instructions, the arrangement depends on the element size.
    // FP32: size=00 or 10 -> arrangement .2s (Q=0) or .4s (Q=1)
    // FP64: size=01 or 11 -> arrangement .2d (Q=1 only)
    let fp_size = match size {
        0b00 | 0b10 => 0b10, // FP32 uses .2s/.4s arrangement
        0b01 | 0b11 => 0b11, // FP64 uses .2d arrangement
        _ => unreachable!(),
    };

    Ok((
        mnemonic,
        vec![
            super::vec_reg_operand(rd_val, fp_size, q),
            super::vec_reg_operand(rn_val, fp_size, q),
            super::vec_reg_operand(rm_val, fp_size, q),
        ],
    ))
}

/// Decode SIMD FP16 Three Same.
///
/// Encoding: b24=0, b21=0, b10=1, b22=1.
/// FP16 element size is implied; arrangement is .4h/.8h from Q.
/// Valid size values: size=01 (bit23=0) and size=11 (bit23=1).
pub fn decode_simd_fp16_three_same(word: u32) -> DecodeResult {
    let u = u_bit(word);
    let opcode = bits(word, 15, 11) as u8;
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);


    // FP16 uses size=01 or size=11 for arrangement (.4h / .8h)
    // Only implement common opcodes 0-7 for now.
    let mnemonic = match (u, opcode) {
        // Opcode 0: fmaxnm / fminnm
        (false, 0) => Mnemonic::Fmaxnm,
        (true, 0) => Mnemonic::Fminnm,

        // Opcode 1: fmla / fmls
        (false, 1) => Mnemonic::Fmla,
        (true, 1) => Mnemonic::Fmls,

        // Opcode 2: fadd / fsub
        (false, 2) => Mnemonic::Fadd,
        (true, 2) => Mnemonic::Fsub,

        // Opcode 3: fmulx / famax
        (false, 3) => Mnemonic::Fmulx,
        (true, 3) => Mnemonic::Famax,

        // Opcode 4: fcmeq (U=0 only)
        (false, 4) => Mnemonic::Fcmeq,

        // Opcode 6: fmax / fmin
        (false, 6) => Mnemonic::Fmax,
        (true, 6) => Mnemonic::Fmin,

        // Opcode 7: frecps / frsqrts
        (false, 7) => Mnemonic::Frecps,
        (true, 7) => Mnemonic::Frsqrts,
        // Opcodes 8-23: mostly reserved for FP16
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented FP16 Three Same opcode {opcode} U={u}"),
            ))
        }
    };

    // FP16 arrangement: size=01 or 11 both map to .4h (Q=0) or .8h (Q=1)
    let fp16_size = 0b01;

    Ok((
        mnemonic,
        vec![
            super::vec_reg_operand(rd_val, fp16_size, q),
            super::vec_reg_operand(rn_val, fp16_size, q),
            super::vec_reg_operand(rm_val, fp16_size, q),
        ],
    ))
}

/// Decode SIMD Two-register Miscellaneous.
///
/// Encoding: b24=0, b21=1, b10=0, b11=1.
/// Fields: U=bit(29), size=bits(23:22), opcode=bits(16:12), Q=bit(30).
pub fn decode_simd_two_reg_misc(word: u32, _op5_16: u8) -> DecodeResult {
    let u = u_bit(word);
    let size = simd_size(word);
    let opcode = bits(word, 16, 12) as u8;
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);

    // Dispatch by size first, then (U, opcode)
    let mnemonic = match size {
        // size=00: byte operations and FP conversions
        0b00 => match (u, opcode) {
            // U=0
            (false, 0) => Mnemonic::Rev64,
            (false, 1) => Mnemonic::Rev16,
            (false, 2) => Mnemonic::Saddlp,
            (false, 3) => Mnemonic::Suqadd,
            (false, 4) => Mnemonic::Cls,
            (false, 5) => Mnemonic::Cnt,
            (false, 6) => Mnemonic::Sadalp,
            (false, 7) => Mnemonic::Sqabs,
            (false, 8) => Mnemonic::Cmgt,
            (false, 9) => Mnemonic::Cmeq,
            (false, 10) => Mnemonic::Cmlt,
            (false, 11) => Mnemonic::Abs,
            (false, 18) => Mnemonic::Xtn,
            (false, 20) => Mnemonic::Sqxtn,
            (false, 22) => Mnemonic::Fcvtn,
            (false, 23) => Mnemonic::Fcvtl,
            (false, 24) => Mnemonic::Frintn,
            (false, 25) => Mnemonic::Frintm,
            (false, 26) => Mnemonic::Fcvtns,
            (false, 27) => Mnemonic::Fcvtms,
            (false, 28) => Mnemonic::Fcvtas,
            (false, 29) => Mnemonic::Scvtf,
            // U=1
            (true, 0) => Mnemonic::Rev32,
            (true, 2) => Mnemonic::Uaddlp,
            (true, 3) => Mnemonic::Usqadd,
            (true, 4) => Mnemonic::Clz,
            (true, 5) => Mnemonic::Not,
            (true, 6) => Mnemonic::Uadalp,
            (true, 7) => Mnemonic::Sqneg,
            (true, 8) => Mnemonic::Cmge,
            (true, 9) => Mnemonic::Cmle,
            (true, 11) => Mnemonic::Neg,
            (true, 18) => Mnemonic::Sqxtun,
            (true, 19) => Mnemonic::Shll,
            (true, 20) => Mnemonic::Uqxtn,
            (true, 23) => Mnemonic::Fcvtxn,
            (true, 24) => Mnemonic::Frinta,
            (true, 25) => Mnemonic::Frintx,
            (true, 26) => Mnemonic::Fcvtnu,
            (true, 27) => Mnemonic::Fcvtmu,
            (true, 28) => Mnemonic::Fcvtau,
            (true, 29) => Mnemonic::Ucvtf,
            _ => {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::UnimplementedInstruction,
                    Some("aarch64".to_string()),
                    format!("Unimplemented Two-reg Misc opcode {opcode} U={u} size=00"),
                ))
            }
        },

        // size=01: halfword operations and across-lanes
        0b01 => match (u, opcode) {
            (false, 3) => Mnemonic::Saddlv,
            (false, 10) => Mnemonic::Smaxv,
            (false, 12) => Mnemonic::Fmaxnmv,
            (false, 15) => Mnemonic::Fmaxv,
            (false, 26) => Mnemonic::Sminv,
            (false, 27) => Mnemonic::Addv,
            (true, 3) => Mnemonic::Uaddlv,
            (true, 10) => Mnemonic::Umaxv,
            (true, 26) => Mnemonic::Uminv,
            _ => {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::UnimplementedInstruction,
                    Some("aarch64".to_string()),
                    format!("Unimplemented Two-reg Misc opcode {opcode} U={u} size=01"),
                ))
            }
        },

        // size=10 and size=11: mostly reserved/invalid for the listed opcodes
        0b10 | 0b11 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented Two-reg Misc opcode {opcode} U={u} size={size:02b}"),
            ))
        }

        _ => unreachable!(),
    };

    // For narrow/widen ops, arrangement differs between Rd and Rn.
    // For now, use same arrangement for both (fix when tests reveal issues).
    Ok((
        mnemonic,
        vec![
            super::vec_reg_operand(rd_val, size, q),
            super::vec_reg_operand(rn_val, size, q),
        ],
    ))
}

// ---------------------------------------------------------------------------
// Stub sub-decoders (Tasks #35-#37)
// ---------------------------------------------------------------------------

/// Decode element size and index from imm5.
///
/// ARM encodes element size in the lowest set bit of imm5:
/// - bit0=1: B element, index in bits[4:1]
/// - bit1=1: H element, index in bits[4:2]
/// - bit2=1: S element, index in bits[4:3]
/// - bit3=1: D element, index in bit[4]
pub fn decode_simd_copy(word: u32) -> DecodeResult {
    let op = bit(word, 29);
    let q = q_bit(word);
    let imm5 = bits(word, 20, 16) as u8;
    let imm4 = bits(word, 14, 11) as u8;
    let rd_val = rd(word);
    let rn_val = rn(word);

    let (elem_char, index, arr_size) = decode_imm5(imm5).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            format!("Invalid imm5 encoding 0b{imm5:05b} in SIMD copy"),
        )
    })?;

    let arr_suffix = arrangement_suffix(arr_size, q).unwrap_or("?");
    let vec_elem = format!("v{}.{elem_char}[{index}]", rn_val);

    // Advanced SIMD copy is discriminated by Q, op, and imm4.
    // See ARM ARM Table C4-300.
    match (q, op, imm4) {
        // DUP (element): duplicate vector element to all lanes
        (_, false, 0b0000) => Ok((
            Mnemonic::Dup,
            vec![
                Operand::Text {
                    value: format!("v{}{}", rd_val, arr_suffix),
                },
                Operand::Text { value: vec_elem },
            ],
        )),
        // DUP (general): duplicate general register to all lanes
        (_, false, 0b0001) => {
            let src_reg = match elem_char {
                'd' => format!("x{}", rn_val),
                _ => format!("w{}", rn_val),
            };
            Ok((
                Mnemonic::Dup,
                vec![
                    Operand::Text {
                        value: format!("v{}{}", rd_val, arr_suffix),
                    },
                    Operand::Text { value: src_reg },
                ],
            ))
        }
        // INS (general): insert general register into vector element
        // Reference renders as `mov` alias.
        (true, false, 0b0011) => {
            let dest_elem = format!("v{}.{elem_char}[{index}]", rd_val);
            let src_reg = match elem_char {
                'd' => format!("x{}", rn_val),
                _ => format!("w{}", rn_val),
            };
            Ok((
                Mnemonic::Mov,
                vec![
                    Operand::Text { value: dest_elem },
                    Operand::Text { value: src_reg },
                ],
            ))
        }
        // SMOV: move vector element to general register (signed)
        (false, false, 0b0101) | (true, false, 0b0101) => {
            let dest_reg = if !q {
                format!("w{}", rd_val)
            } else {
                format!("x{}", rd_val)
            };
            Ok((
                Mnemonic::Smov,
                vec![
                    Operand::Text { value: dest_reg },
                    Operand::Text { value: vec_elem },
                ],
            ))
        }
        // UMOV: move vector element to general register (unsigned)
        // Reference uses `mov` alias for S and D elements.
        (false, false, 0b0111) | (true, false, 0b0111) => {
            let dest_reg = if !q {
                format!("w{}", rd_val)
            } else {
                format!("x{}", rd_val)
            };
            let mnemonic = match elem_char {
                's' | 'd' => Mnemonic::Mov,
                _ => Mnemonic::Umov,
            };
            Ok((
                mnemonic,
                vec![
                    Operand::Text { value: dest_reg },
                    Operand::Text { value: vec_elem },
                ],
            ))
        }
        // INS (element): insert vector element into vector element
        // Reference renders as `mov` alias.
        // Q=1, op=1 with any imm4. Source index is imm4 shifted by element size.
        (true, true, _) => {
            let dest_elem = format!("v{}.{elem_char}[{index}]", rd_val);
            let src_index = imm4 >> arr_size;
            let src_elem = format!("v{}.{elem_char}[{src_index}]", rn_val);
            Ok((
                Mnemonic::Mov,
                vec![
                    Operand::Text { value: dest_elem },
                    Operand::Text { value: src_elem },
                ],
            ))
        }
        _ => Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            format!("Unimplemented SIMD copy Q={q} op={op} imm4=0b{imm4:04b}"),
        )),
    }
}

/// Decode SIMD modified immediate instructions.
///
/// Encoding: b24=1, b10=1, immh=0000.
/// Fields: op=bit(29), cmode=bits(15:12), Q=bit(30), o2=bit(11),
///         imm8={bit(18),bit(17),bit(16),bit(9),bit(8),bit(7),bit(6),bit(5)},
///         Rd=bits(4:0).
pub fn decode_simd_modified_imm(word: u32) -> DecodeResult {
    let op = bit(word, 29);
    let q = q_bit(word);
    let cmode = bits(word, 15, 12) as u8;
    let o2 = bit(word, 11);
    let rd_val = rd(word);

    if o2 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "Reserved o2=1 in SIMD modified immediate",
        ));
    }

    // Extract imm8 = {a,b,c,d,e,f,g,h}
    let a = bit(word, 18);
    let b = bit(word, 17);
    let c = bit(word, 16);
    let d = bit(word, 9);
    let e = bit(word, 8);
    let f = bit(word, 7);
    let g = bit(word, 6);
    let h = bit(word, 5);
    let imm8 = (u8::from(a) << 7)
        | (u8::from(b) << 6)
        | (u8::from(c) << 5)
        | (u8::from(d) << 4)
        | (u8::from(e) << 3)
        | (u8::from(f) << 2)
        | (u8::from(g) << 1)
        | u8::from(h);

    // Determine operation and operands from (op, cmode)
    match (op, cmode) {
        // MOVI / MVNI / ORR / BIC — 32-bit element, no shift
        (false, 0b0000) | (false, 0b0010) | (false, 0b0100) | (false, 0b0110) => {
            let shift = ((cmode >> 1) & 0b11) * 8;
            let arr = if !q { ".2s" } else { ".4s" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Movi, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        (true, 0b0000) | (true, 0b0010) | (true, 0b0100) | (true, 0b0110) => {
            let shift = ((cmode >> 1) & 0b11) * 8;
            let arr = if !q { ".2s" } else { ".4s" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Mvni, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        // ORR / BIC — 32-bit element, same shifts
        (false, 0b0001) | (false, 0b0011) | (false, 0b0101) | (false, 0b0111) => {
            let shift = ((cmode >> 1) & 0b11) * 8;
            let arr = if !q { ".2s" } else { ".4s" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Orr, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        (true, 0b0001) | (true, 0b0011) | (true, 0b0101) | (true, 0b0111) => {
            let shift = ((cmode >> 1) & 0b11) * 8;
            let arr = if !q { ".2s" } else { ".4s" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Bic, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        // MOVI / MVNI — 16-bit element
        (false, 0b1000) | (false, 0b1010) => {
            let shift = if (cmode & 0b10) != 0 { 8 } else { 0 };
            let arr = if !q { ".4h" } else { ".8h" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Movi, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        (true, 0b1000) | (true, 0b1010) => {
            let shift = if (cmode & 0b10) != 0 { 8 } else { 0 };
            let arr = if !q { ".4h" } else { ".8h" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Mvni, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        // ORR / BIC — 16-bit element
        (false, 0b1001) | (false, 0b1011) => {
            let shift = if (cmode & 0b10) != 0 { 8 } else { 0 };
            let arr = if !q { ".4h" } else { ".8h" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Orr, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        (true, 0b1001) | (true, 0b1011) => {
            let shift = if (cmode & 0b10) != 0 { 8 } else { 0 };
            let arr = if !q { ".4h" } else { ".8h" };
            let imm_text = if shift == 0 {
                format!("#{}", imm8)
            } else {
                format!("#{}, lsl #{}", imm8, shift)
            };
            Ok((Mnemonic::Bic, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: imm_text },
            ]))
        }
        // MOVI / MVNI — MSL forms
        (false, 0b1100) => {
            let arr = if !q { ".2s" } else { ".4s" };
            Ok((Mnemonic::Movi, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{}, msl #8", imm8) },
            ]))
        }
        (false, 0b1101) => {
            let arr = if !q { ".2s" } else { ".4s" };
            Ok((Mnemonic::Movi, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{}, msl #16", imm8) },
            ]))
        }
        (true, 0b1100) => {
            let arr = if !q { ".2s" } else { ".4s" };
            Ok((Mnemonic::Mvni, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{}, msl #8", imm8) },
            ]))
        }
        (true, 0b1101) => {
            let arr = if !q { ".2s" } else { ".4s" };
            Ok((Mnemonic::Mvni, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{}, msl #16", imm8) },
            ]))
        }
        // MOVI — 8-bit element
        (false, 0b1110) => {
            let arr = if !q { ".8b" } else { ".16b" };
            Ok((Mnemonic::Movi, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{}", imm8) },
            ]))
        }
        // MOVI — 64-bit immediate
        (true, 0b1110) => {
            // Construct 64-bit immediate: each bit of imm8 replicated to a byte
            let mut imm64: u64 = 0;
            for i in 0..8 {
                let bit = (imm8 >> (7 - i)) & 1;
                let byte_val = if bit != 0 { 0xFFu64 } else { 0x00u64 };
                imm64 = (imm64 << 8) | byte_val;
            }
            if !q {
                // Scalar: movi dN, #imm64
                Ok((Mnemonic::Movi, vec![
                    Operand::Text { value: format!("d{}", rd_val) },
                    Operand::Text { value: format!("#0x{:016x}", imm64) },
                ]))
            } else {
                // Vector: movi vN.2d, #imm64
                Ok((Mnemonic::Movi, vec![
                    Operand::Text { value: format!("v{}.2d", rd_val) },
                    Operand::Text { value: format!("#0x{:016x}", imm64) },
                ]))
            }
        }
        // FMOV — floating-point immediate
        (false, 0b1111) => {
            // Single-precision: imm32 = a : ~b : bbbbb : c : d : e : f : g : h : 0... (19 zeros)
            let imm32: u32 = ((a as u32) << 31)
                | ((((!b) as u32) & 1) << 30)
                | ((b as u32) << 29)
                | ((b as u32) << 28)
                | ((b as u32) << 27)
                | ((b as u32) << 26)
                | ((b as u32) << 25)
                | ((c as u32) << 24)
                | ((d as u32) << 23)
                | ((e as u32) << 22)
                | ((f as u32) << 21)
                | ((g as u32) << 20)
                | ((h as u32) << 19);
            let fp_val = f32::from_bits(imm32);
            let arr = if !q { ".2s" } else { ".4s" };
            Ok((Mnemonic::Fmov, vec![
                Operand::Text { value: format!("v{}{}", rd_val, arr) },
                Operand::Text { value: format!("#{:.8}", fp_val) },
            ]))
        }
        (true, 0b1111) => {
            if !q {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::InvalidEncoding,
                    Some("aarch64".to_string()),
                    "Reserved FMOV double-precision Q=0",
                ));
            }
            // Double-precision: imm64 = a : ~b : bbbbbbbb : c : d : e : f : g : h : 0... (42 zeros)
            let imm64: u64 = ((a as u64) << 63)
                | ((((!b) as u64) & 1) << 62)
                | ((b as u64) << 61)
                | ((b as u64) << 60)
                | ((b as u64) << 59)
                | ((b as u64) << 58)
                | ((b as u64) << 57)
                | ((b as u64) << 56)
                | ((b as u64) << 55)
                | ((b as u64) << 54)
                | ((c as u64) << 53)
                | ((d as u64) << 52)
                | ((e as u64) << 51)
                | ((f as u64) << 50)
                | ((g as u64) << 49)
                | ((h as u64) << 48);
            let fp_val = f64::from_bits(imm64);
            Ok((Mnemonic::Fmov, vec![
                Operand::Text { value: format!("v{}.2d", rd_val) },
                Operand::Text { value: format!("#{:.8}", fp_val) },
            ]))
        }
        _ => Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            format!("Unimplemented SIMD modified immediate op={op} cmode={cmode:04b}"),
        )),
    }
}

/// Decode SIMD shift-by-immediate instructions.
///
/// Encoding: b24=1, b10=1, b23:21≠000.
/// Fields: U=bit(29), immh=bits(22:19), immb=bits(18:16), opcode=bits(15:11), Q=bit(30).
pub fn decode_simd_shift_imm(word: u32) -> DecodeResult {
    let u = u_bit(word);
    let q = q_bit(word);
    let immh = bits(word, 22, 19) as u8;
    let immb = bits(word, 18, 16) as u8;
    let opcode = bits(word, 15, 11) as u8;
    let rd_val = rd(word);
    let rn_val = rn(word);

    if immh == 0 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "Invalid immh=0000 in SIMD shift immediate",
        ));
    }

    // Determine element size from highest set bit in immh (4-bit field)
    let highest_set = 3u8.saturating_sub((immh.leading_zeros() as u8).saturating_sub(4));
    let esize = 8u16 << highest_set; // 8, 16, 32, 64
    let immh_immb = ((immh as u16) << 3) | (immb as u16); // 7-bit value

    // Size index for arrangement_suffix: 0=B, 1=H, 2=S, 3=D
    let size = highest_set;

    // Decode mnemonic and compute shift amount
    let (mnemonic, shift, is_narrowing, is_long) = match opcode {
        0b00000 => {
            // SSHR (U=0) / USHR (U=1)
            let m = if !u { Mnemonic::Sshr } else { Mnemonic::Ushr };
            let s = (2 * esize) - immh_immb;
            (m, s, false, false)
        }
        0b00010 => {
            // SSRA (U=0) / USRA (U=1)
            let m = if !u { Mnemonic::Ssra } else { Mnemonic::Usra };
            let s = (2 * esize) - immh_immb;
            (m, s, false, false)
        }
        0b00100 => {
            // SRSHR (U=0) / URSHR (U=1)
            let m = if !u { Mnemonic::Srshr } else { Mnemonic::Urshr };
            let s = (2 * esize) - immh_immb;
            (m, s, false, false)
        }
        0b00110 => {
            // SRSRA (U=0) / URSRA (U=1)
            let m = if !u { Mnemonic::Srsra } else { Mnemonic::Ursra };
            let s = (2 * esize) - immh_immb;
            (m, s, false, false)
        }
        0b01000 => {
            // SRI (U=1 only)
            if !u {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::InvalidEncoding,
                    Some("aarch64".to_string()),
                    "Reserved SIMD shift immediate opcode 01000 U=0",
                ));
            }
            let s = (2 * esize) - immh_immb;
            (Mnemonic::Sri, s, false, false)
        }
        0b01010 => {
            // SHL (U=0) / SLI (U=1)
            let m = if !u { Mnemonic::Shl } else { Mnemonic::Sli };
            let s = immh_immb - esize;
            (m, s, false, false)
        }
        0b01100 => {
            // SQSHLU (U=1 only)
            if !u {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::InvalidEncoding,
                    Some("aarch64".to_string()),
                    "Reserved SIMD shift immediate opcode 01100 U=0",
                ));
            }
            let s = immh_immb - esize;
            (Mnemonic::Sqshlu, s, false, false)
        }
        0b01110 => {
            // SQSHL (imm) (U=0) / UQSHL (imm) (U=1)
            let m = if !u { Mnemonic::Sqshl } else { Mnemonic::Uqshl };
            let s = immh_immb - esize;
            (m, s, false, false)
        }
        0b10000 => {
            // SHRN/SHRN2 (U=0) / SQSHRUN/SQSHRUN2 (U=1)
            // For narrowing, immh encodes dest element size
            let s = (2 * esize) - immh_immb;
            if !u {
                (Mnemonic::Shrn, s, true, false)
            } else {
                (Mnemonic::Sqshrun, s, true, false)
            }
        }
        0b10001 => {
            // RSHRN/RSHRN2 (U=0) / SQRSHRUN/SQRSHRUN2 (U=1)
            let s = (2 * esize) - immh_immb;
            if !u {
                (Mnemonic::Rshrn, s, true, false)
            } else {
                (Mnemonic::Sqrshrun, s, true, false)
            }
        }
        0b10010 => {
            // SQSHRN/SQSHRN2 (U=0) / UQSHRN/UQSHRN2 (U=1)
            let s = (2 * esize) - immh_immb;
            if !u {
                (Mnemonic::Sqshrn, s, true, false)
            } else {
                (Mnemonic::Uqshrn, s, true, false)
            }
        }
        0b10011 => {
            // SQRSHRN/SQRSHRN2 (U=0) / UQRSHRN/UQRSHRN2 (U=1)
            let s = (2 * esize) - immh_immb;
            if !u {
                (Mnemonic::Sqrshrn, s, true, false)
            } else {
                (Mnemonic::Uqrshrn, s, true, false)
            }
        }
        0b10100 => {
            // SSHLL/SSHLL2 (U=0) / USHLL/USHLL2 (U=1)
            let m = if !u { Mnemonic::Sshll } else { Mnemonic::Ushll };
            let s = immh_immb - esize;
            (m, s, false, true)
        }
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented SIMD shift immediate opcode {opcode:05b} U={u}"),
            ))
        }
    };

    // Build operands
    let dest_arr = if is_narrowing {
        // Narrowing: immh encodes dest element size, src is double
        arrangement_suffix(size, q).unwrap_or("?")
    } else if is_long {
        // Widening: dest is always 128-bit, element is double source
        arrangement_suffix(size + 1, true).unwrap_or("?")
    } else {
        // Same-size: dest arrangement matches source
        arrangement_suffix(size, q).unwrap_or("?")
    };

    let src_arr = if is_narrowing {
        // Narrowing: source element is double dest, always 128-bit
        arrangement_suffix(size + 1, true).unwrap_or("?")
    } else if is_long {
        // Widening: source arrangement uses Q as encoded
        arrangement_suffix(size, q).unwrap_or("?")
    } else {
        // Same-size
        arrangement_suffix(size, q).unwrap_or("?")
    };

    // Map mnemonic string back to enum for narrowing/long Q=1 variants
    let final_mnemonic = if is_narrowing && q {
        match mnemonic {
            Mnemonic::Shrn => Mnemonic::Shrn2,
            Mnemonic::Sqshrun => Mnemonic::Sqshrun2,
            Mnemonic::Rshrn => Mnemonic::Rshrn2,
            Mnemonic::Sqrshrun => Mnemonic::Sqrshrun2,
            Mnemonic::Sqshrn => Mnemonic::Sqshrn2,
            Mnemonic::Uqshrn => Mnemonic::Uqshrn2,
            Mnemonic::Sqrshrn => Mnemonic::Sqrshrn2,
            Mnemonic::Uqrshrn => Mnemonic::Uqrshrn2,
            _ => mnemonic,
        }
    } else if is_long && q {
        match mnemonic {
            Mnemonic::Sshll => Mnemonic::Sshll2,
            Mnemonic::Ushll => Mnemonic::Ushll2,
            _ => mnemonic,
        }
    } else {
        mnemonic
    };

    Ok((
        final_mnemonic,
        vec![
            Operand::Text {
                value: format!("v{}{}", rd_val, dest_arr),
            },
            Operand::Text {
                value: format!("v{}{}", rn_val, src_arr),
            },
            Operand::Immediate {
                value: i64::from(shift),
            },
        ],
    ))
}

/// Decode Advanced SIMD vector x indexed element instructions.
///
/// Encoding: b24=1, b10=0.
/// Fields: U=bit(29), size=bits(23:22), opcode=bits(15:12), Q=bit(30),
///         H=bit(11), L=bit(21), M=bit(10), Rd=bits(4:0), Rn=bits(9:5).
pub fn decode_simd_indexed_element(word: u32) -> DecodeResult {
    let u = u_bit(word);
    let q = q_bit(word);
    let size = simd_size(word);
    let opcode = bits(word, 15, 12) as u8;
    let h = bit(word, 11);
    let l = bit(word, 21);
    let m = bit(word, 10);
    let rd_val = rd(word);
    let rn_val = rn(word);

    // Determine mnemonic from (opcode, U, size)
    // ARMv8.2-FP16 fmlal/fmlal2/fmlsl/fmlsl2 use size=10 and overlap with
    // integer opcodes when the element size is 32-bit encoded.
    let (mnemonic, is_fp, is_long, is_fp16_fml) = match (opcode, u, size) {
        // ARMv8.2-FP16 multiply-accumulate (size=10 only)
        (0b0000, false, 0b10) => (Mnemonic::Fmlal, false, false, true),
        (0b0000, true, _) => (Mnemonic::Mla, false, false, false),
        (0b0100, false, 0b10) => (Mnemonic::Fmlsl, false, false, true),
        (0b0100, true, _) => (Mnemonic::Mls, false, false, false),
        (0b1000, true, 0b10) => (Mnemonic::Fmlal2, false, false, true),
        (0b1000, true, _) => (Mnemonic::Mul, false, false, false),
        (0b1100, true, 0b10) => (Mnemonic::Fmlsl2, false, false, true),
        // Integer long multiply-accumulate
        (0b0010, false, _) => (Mnemonic::Smlal, false, true, false),
        (0b0010, true, _) => (Mnemonic::Umlal, false, true, false),
        (0b0110, false, _) => (Mnemonic::Smlsl, false, true, false),
        (0b0110, true, _) => (Mnemonic::Umlsl, false, true, false),
        (0b1010, false, _) => (Mnemonic::Smull, false, true, false),
        (0b1010, true, _) => (Mnemonic::Umull, false, true, false),
        // Integer saturating long multiply
        (0b0011, false, _) => (Mnemonic::Sqdmlal, false, true, false),
        (0b0111, false, _) => (Mnemonic::Sqdmlsl, false, true, false),
        (0b1011, false, _) => (Mnemonic::Sqdmull, false, true, false),
        // Integer saturating multiply
        (0b1100, false, _) => (Mnemonic::Sqdmulh, false, false, false),
        (0b1100, true, _) => (Mnemonic::Sqrdmulh, false, false, false),
        // FP multiply-accumulate
        (0b0001, false, _) => (Mnemonic::Fmla, true, false, false),
        (0b0101, false, _) => (Mnemonic::Fmls, true, false, false),
        // FP multiply
        (0b1001, false, _) => (Mnemonic::Fmul, true, false, false),
        (0b1001, true, _) => (Mnemonic::Fmulx, true, false, false),
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!(
                    "Unimplemented SIMD indexed element opcode={opcode:04b} U={u} size={size}"
                ),
            ))
        }
    };

    // Decode Rm and element index based on size
    // ARMv8.2-FP16 fmlal/fmlal2/fmlsl/fmlsl2 encode size=10 but operate on
    // halfword elements, so use the halfword (size=01) decoding path.
    let (rm_val, index) = if is_fp16_fml {
        // FP16 indexed element: Rm = bits(19:16), index = {H, L, bit20}
        let rm = bits(word, 19, 16) as u8;
        let bit20 = bit(word, 20);
        let idx = (u8::from(h) << 2) | (u8::from(l) << 1) | u8::from(bit20);
        (rm, idx)
    } else {
        match size {
            0b00 => {
                // Byte: Rm = bits(19:16), index = {H, L, M, bit20}
                let rm = bits(word, 19, 16) as u8;
                let bit20 = bit(word, 20);
                let idx = (u8::from(h) << 3) | (u8::from(l) << 2) | (u8::from(m) << 1) | u8::from(bit20);
                (rm, idx)
            }
            0b01 => {
                // Halfword: Rm = bits(19:16), index = {H, L, bit20}
                let rm = bits(word, 19, 16) as u8;
                let bit20 = bit(word, 20);
                let idx = (u8::from(h) << 2) | (u8::from(l) << 1) | u8::from(bit20);
                (rm, idx)
            }
            0b10 => {
                // Word: Rm = bits(20:16), index = {H, L}
                let rm = bits(word, 20, 16) as u8;
                let idx = (u8::from(h) << 1) | u8::from(l);
                (rm, idx)
            }
            0b11 => {
                // Doubleword: Rm = bits(20:16), index = {H}
                let rm = bits(word, 20, 16) as u8;
                let idx = u8::from(h);
                (rm, idx)
            }
            _ => unreachable!(),
        }
    };

    // Determine arrangement suffixes and element character
    let (dest_arr, src_arr, elem_char) = if is_fp16_fml {
        // ARMv8.2-FP16 fmlal/fmlal2/fmlsl/fmlsl2:
        // dest is .2s/.4s (single), src is .2h/.4h (half), element is h
        (
            arrangement_suffix(0b10, q).unwrap_or("?"), // .2s or .4s
            if !q { ".2h" } else { ".4h" },
            'h',
        )
    } else if is_fp {
        // FP: size=00 is FP16, which uses 'h' arrangement
        let arr_size = if size == 0b00 { 0b01 } else { size };
        (
            arrangement_suffix(arr_size, q).unwrap_or("?"),
            arrangement_suffix(arr_size, q).unwrap_or("?"),
            match size {
                0b00 => 'h',
                0b10 => 's',
                0b11 => 'd',
                _ => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        "Invalid FP size for indexed element",
                    ))
                }
            },
        )
    } else if is_long {
        // Long multiply: dest is wider, src is normal size
        let dest_size = size + 1;
        (
            arrangement_suffix(dest_size, true).unwrap_or("?"), // Always 128-bit result
            arrangement_suffix(size, q).unwrap_or("?"),
            match size {
                0b01 => 'h',
                0b10 => 's',
                _ => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        "Invalid size for long multiply indexed element",
                    ))
                }
            },
        )
    } else {
        // Integer same-width
        (
            arrangement_suffix(size, q).unwrap_or("?"),
            arrangement_suffix(size, q).unwrap_or("?"),
            match size {
                0b00 => 'b',
                0b01 => 'h',
                0b10 => 's',
                0b11 => 'd',
                _ => unreachable!(),
            },
        )
    };

    // Handle "2" suffix for long multiply when Q=1
    let final_mnemonic = if is_long && q {
        match mnemonic {
            Mnemonic::Smlal => Mnemonic::Smlal2,
            Mnemonic::Umlal => Mnemonic::Umlal2,
            Mnemonic::Smlsl => Mnemonic::Smlsl2,
            Mnemonic::Umlsl => Mnemonic::Umlsl2,
            Mnemonic::Smull => Mnemonic::Smull2,
            Mnemonic::Umull => Mnemonic::Umull2,
            Mnemonic::Sqdmlal => Mnemonic::Sqdmlal2,
            Mnemonic::Sqdmlsl => Mnemonic::Sqdmlsl2,
            Mnemonic::Sqdmull => Mnemonic::Sqdmull2,
            _ => mnemonic,
        }
    } else {
        mnemonic
    };

    Ok((
        final_mnemonic,
        vec![
            Operand::Text {
                value: format!("v{}{}", rd_val, dest_arr),
            },
            Operand::Text {
                value: format!("v{}{}", rn_val, src_arr),
            },
            Operand::Text {
                value: format!("v{}.{elem_char}[{index}]", rm_val),
            },
        ],
    ))
}

/// Decode Cryptographic AES instructions.
///
/// Encoding: op0=0xE/F, bit28=0, b21=1, b10=0, b11=1, size=00, bits(20:16)=0b01000.
pub fn decode_crypto_aes(word: u32) -> DecodeResult {
    let q = q_bit(word);
    let opcode = bits(word, 16, 12) as u8;
    let rd_val = rd(word);
    let rn_val = rn(word);

    let mnemonic = match opcode {
        4 => Mnemonic::Aese,
        5 => Mnemonic::Aesd,
        6 => Mnemonic::Aesmc,
        7 => Mnemonic::Aesimc,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented AES opcode {opcode}"),
            ))
        }
    };

    // AES operates on .16b (Q=1) or .8b (Q=0)
    let arr = if q { ".16b" } else { ".8b" };

    Ok((
        mnemonic,
        vec![
            Operand::Text {
                value: format!("v{}{}", rd_val, arr),
            },
            Operand::Text {
                value: format!("v{}{}", rn_val, arr),
            },
        ],
    ))
}

/// Decode Cryptographic two-register SHA instructions.
///
/// These encode in the scalar FP space (0x5E prefix) with bits(20:16)=0b01000.
/// Opcode is in bits(9:8) (with bit 11 = 1 as the group indicator).
/// Rn is in bits(7:5) for these instructions.
pub fn decode_crypto_sha2(word: u32) -> DecodeResult {
    let rd_val = rd(word);
    let rn_val = bits(word, 7, 5) as u8;
    let opcode = bits(word, 9, 8) as u8;

    let mnemonic = match opcode {
        0b00 => Mnemonic::Sha1h,
        0b01 => Mnemonic::Sha1su1,
        0b10 => Mnemonic::Sha256su0,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented SHA-2 opcode 0b{opcode:02b}"),
            ))
        }
    };

    match mnemonic {
        Mnemonic::Sha1h => {
            // sha1h uses scalar S registers
            Ok((
                mnemonic,
                vec![
                    super::fp_reg_operand(rd_val, FpRegSize::S),
                    super::fp_reg_operand(rn_val, FpRegSize::S),
                ],
            ))
        }
        _ => {
            // sha1su1, sha256su0 use vector .4s (Q=1 for crypto space)
            Ok((
                mnemonic,
                vec![
                    super::vec_reg_operand(rd_val, 0b10, true), // .4s
                    super::vec_reg_operand(rn_val, 0b10, true),
                ],
            ))
        }
    }
}

/// Decode Cryptographic three-register SHA instructions.
///
/// These encode in the scalar FP space (0x5E prefix) with bits(20:16)=0b00000.
/// Opcode is in bits(10:8) (with bit 11 = 0 as the group indicator).
/// Rm is in bits(23:20) (the high nibble of byte2).
pub fn decode_crypto_sha3(word: u32) -> DecodeResult {
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let opcode = bits(word, 15, 12) as u8;

    let mnemonic = match opcode {
        0b000 => Mnemonic::Sha1c,
        0b001 => Mnemonic::Sha1p,
        0b010 => Mnemonic::Sha1m,
        0b011 => Mnemonic::Sha1su0,
        0b100 => Mnemonic::Sha256h,
        0b101 => Mnemonic::Sha256h2,
        0b110 => Mnemonic::Sha256su1,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented SHA-3 opcode 0b{opcode:03b}"),
            ))
        }
    };

    // Build operands based on instruction type
    let ops = match mnemonic {
        Mnemonic::Sha1c | Mnemonic::Sha1p | Mnemonic::Sha1m => {
            // sha1c  Qd, Sn, Vm.4s
            vec![
                Operand::Text {
                    value: format!("q{}", rd_val),
                },
                super::fp_reg_operand(rn_val, FpRegSize::S),
                super::vec_reg_operand(rm_val, 0b10, true), // .4s
            ]
        }
        Mnemonic::Sha256h | Mnemonic::Sha256h2 => {
            // sha256h  Qd, Qn, Vm.4s
            vec![
                Operand::Text {
                    value: format!("q{}", rd_val),
                },
                Operand::Text {
                    value: format!("q{}", rn_val),
                },
                super::vec_reg_operand(rm_val, 0b10, true), // .4s
            ]
        }
        _ => {
            // sha1su0, sha256su1: Vd.4s, Vn.4s, Vm.4s
            vec![
                super::vec_reg_operand(rd_val, 0b10, true),
                super::vec_reg_operand(rn_val, 0b10, true),
                super::vec_reg_operand(rm_val, 0b10, true),
            ]
        }
    };

    Ok((mnemonic, ops))
}

pub fn try_decode_simd_across_lanes(word: u32, op5_16: u8) -> Option<(Mnemonic, Vec<Operand>)> {
    let u = s_flag(word);
    let size = simd_size(word);
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let b20 = bit(word, 20);

    // Across Lanes instructions all have b20=1 in the b21=1, b10=0, b11=1 branch.
    // Some Two-reg Misc instructions also have b20=1, so we need to filter carefully.
    if !b20 {
        return None;
    }

    // Mnemonic mapping based on op5_16 and U
    let mnemonic = match op5_16 {
        0b00011 => {
            // saddlv (U=0) / uaddlv (U=1)
            if !u { Mnemonic::Saddlv } else { Mnemonic::Uaddlv }
        }
        0b01010 => {
            // smaxv (U=0) / umaxv (U=1)
            if !u { Mnemonic::Smaxv } else { Mnemonic::Umaxv }
        }
        0b01100 => {
            // fmaxnmv (U=0) / fminnmv (U=1) — Across Lanes
            // size=11 → fcmge/fcmgt (Two-reg Misc)
            if size == 0b11 {
                return None;
            }
            if !u { Mnemonic::Fmaxnmv } else { Mnemonic::Fminnmv }
        }
        0b01101 => {
            // fcmeq/fcmle (Two-reg Misc) — no Across Lanes at this opcode
            return None;
        }
        0b01110 => {
            // fcmlt (Two-reg Misc) — no Across Lanes at this opcode
            return None;
        }
        0b01111 => {
            // fmaxv (U=0) / fminv (U=1) — Across Lanes
            // size=11 → fabs/fneg (Two-reg Misc)
            if size == 0b11 {
                return None;
            }
            if !u { Mnemonic::Fmaxv } else { Mnemonic::Fminv }
        }
        0b11010 => {
            // sminv (U=0) / uminv (U=1) — Across Lanes, size must be 00
            if size != 0b00 {
                return None; // fcvtns/fcvtnu/fcvtps/fcvtpu (Two-reg Misc, size=01/10)
            }
            if !u { Mnemonic::Sminv } else { Mnemonic::Uminv }
        }
        0b11011 if !u && size != 0b11 => {
            // addv (U=0, size=00/01/10) — Across Lanes
            Mnemonic::Addv
        }
        0b11100 | 0b11101 | 0b11111 => {
            // frinta/frinti/fcvtas/frecpe/fsqrt (Two-reg Misc)
            return None;
        }
        _ => return None,
    };

    // Destination register: scalar element size
    let dest_size = match mnemonic {
        Mnemonic::Saddlv | Mnemonic::Uaddlv => {
            // Destination element is wider than source
            match size {
                0b00 => FpRegSize::H,
                0b01 => FpRegSize::S,
                0b10 => FpRegSize::D,
                _ => return None,
            }
        }
        Mnemonic::Fmaxnmv | Mnemonic::Fminnmv | Mnemonic::Fmaxv | Mnemonic::Fminv => {
            // FP across-lanes: dest size determined by size (00=H, 01=S, 10=D)
            match size {
                0b00 => FpRegSize::H,
                0b01 => FpRegSize::S,
                0b10 => FpRegSize::D,
                _ => return None,
            }
        }
        _ => {
            // Integer across-lanes: dest size equals source element size
            match size {
                0b00 => FpRegSize::B,
                0b01 => FpRegSize::H,
                0b10 => FpRegSize::S,
                _ => return None,
            }
        }
    };

    let dest = super::fp_reg_operand(rd_val, dest_size);

    // FP across-lanes use size+1 for vector arrangement (00→H/.4h, 01→S/.2s, 10→D/.1d)
    // Integer across-lanes use size directly.
    let src_size = match mnemonic {
        Mnemonic::Fmaxnmv | Mnemonic::Fminnmv | Mnemonic::Fmaxv | Mnemonic::Fminv => {
            size + 1
        }
        _ => size,
    };
    let src = super::vec_reg_operand(rn_val, src_size, q);

    Some((mnemonic, vec![dest, src]))
}

pub fn decode_simd_three_different(word: u32) -> DecodeResult {
    let u = bit(word, 29);
    let size = simd_size(word);
    let opcode = bits(word, 15, 12) as u8;
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);

    // Helper to build vector register text operand
    let vec = |reg: u8, sz: u8, qbit: bool| -> Operand {
        Operand::Text {
            value: format!("v{}{}", reg, arrangement_suffix(sz, qbit).unwrap_or("?")),
        }
    };

    // Classify the instruction by opcode
    match opcode {
        // Widen: ADD Long, SUB Long, MUL Long, etc.
        0b0000 | 0b0010 | 0b0101 | 0b0111 | 0b1000 | 0b1010 | 0b1100 | 0b1001 | 0b1011 | 0b1101 => {
            let base_mnemonic = match opcode {
                0b0000 => if !u { Mnemonic::Saddl } else { Mnemonic::Uaddl },
                0b0010 => if !u { Mnemonic::Ssubl } else { Mnemonic::Usubl },
                0b0101 => if !u { Mnemonic::Sabal } else { Mnemonic::Uabal },
                0b0111 => if !u { Mnemonic::Sabdl } else { Mnemonic::Uabdl },
                0b1000 => if !u { Mnemonic::Smlal } else { Mnemonic::Umlal },
                0b1010 => if !u { Mnemonic::Smlsl } else { Mnemonic::Umlsl },
                0b1100 => if !u { Mnemonic::Smull } else { Mnemonic::Umull },
                0b1001 => Mnemonic::Sqdmlal,
                0b1011 => Mnemonic::Sqdmlsl,
                0b1101 => Mnemonic::Sqdmull,
                _ => unreachable!(),
            };

            // "2" suffix when Q=1 for long/widen operations
            let mnemonic = if q {
                match base_mnemonic {
                    Mnemonic::Saddl => Mnemonic::Saddl2,
                    Mnemonic::Uaddl => Mnemonic::Uaddl2,
                    Mnemonic::Ssubl => Mnemonic::Ssubl2,
                    Mnemonic::Usubl => Mnemonic::Usubl2,
                    Mnemonic::Sabal => Mnemonic::Sabal2,
                    Mnemonic::Uabal => Mnemonic::Uabal2,
                    Mnemonic::Sabdl => Mnemonic::Sabdl2,
                    Mnemonic::Uabdl => Mnemonic::Uabdl2,
                    Mnemonic::Smlal => Mnemonic::Smlal2,
                    Mnemonic::Umlal => Mnemonic::Umlal2,
                    Mnemonic::Smlsl => Mnemonic::Smlsl2,
                    Mnemonic::Umlsl => Mnemonic::Umlsl2,
                    Mnemonic::Smull => Mnemonic::Smull2,
                    Mnemonic::Umull => Mnemonic::Umull2,
                    Mnemonic::Sqdmlal => Mnemonic::Sqdmlal2,
                    Mnemonic::Sqdmlsl => Mnemonic::Sqdmlsl2,
                    Mnemonic::Sqdmull => Mnemonic::Sqdmull2,
                    _ => base_mnemonic,
                }
            } else {
                base_mnemonic
            };

            // Widen: dest uses size+1 with Q=1 (128-bit), sources use size with actual Q
            let dest = vec(rd_val, size + 1, true);
            let src1 = vec(rn_val, size, q);
            let src2 = vec(rm_val, size, q);
            Ok((mnemonic, vec![dest, src1, src2]))
        }

        // Widening add/sub (saddw, ssubw, uaddw, usubw)
        0b0001 | 0b0011 => {
            let base_mnemonic = match opcode {
                0b0001 => if !u { Mnemonic::Saddw } else { Mnemonic::Uaddw },
                0b0011 => if !u { Mnemonic::Ssubw } else { Mnemonic::Usubw },
                _ => unreachable!(),
            };

            let mnemonic = if q {
                match base_mnemonic {
                    Mnemonic::Saddw => Mnemonic::Saddw2,
                    Mnemonic::Uaddw => Mnemonic::Uaddw2,
                    Mnemonic::Ssubw => Mnemonic::Ssubw2,
                    Mnemonic::Usubw => Mnemonic::Usubw2,
                    _ => base_mnemonic,
                }
            } else {
                base_mnemonic
            };

            // saddw: dest and first src use size+1 with Q=1, second src uses size with actual Q
            let dest = vec(rd_val, size + 1, true);
            let src1 = vec(rn_val, size + 1, true);
            let src2 = vec(rm_val, size, q);
            Ok((mnemonic, vec![dest, src1, src2]))
        }

        // Narrow (addhn, raddhn, subhn, rsubhn)
        0b0100 | 0b0110 => {
            let base_mnemonic = match opcode {
                0b0100 => if !u { Mnemonic::Addhn } else { Mnemonic::Raddhn },
                0b0110 => if !u { Mnemonic::Subhn } else { Mnemonic::Rsubhn },
                _ => unreachable!(),
            };

            let mnemonic = if q {
                match base_mnemonic {
                    Mnemonic::Addhn => Mnemonic::Addhn2,
                    Mnemonic::Raddhn => Mnemonic::Raddhn2,
                    Mnemonic::Subhn => Mnemonic::Subhn2,
                    Mnemonic::Rsubhn => Mnemonic::Rsubhn2,
                    _ => base_mnemonic,
                }
            } else {
                base_mnemonic
            };

            // Narrow: dest uses size with actual Q, sources use size+1 with Q=1
            let dest = vec(rd_val, size, q);
            let src1 = vec(rn_val, size + 1, true);
            let src2 = vec(rm_val, size + 1, true);
            Ok((mnemonic, vec![dest, src1, src2]))
        }

        // PMULL / PMULL2
        0b1110 => {
            if u {
                return Err(DisasmError::decode_failure(
                    DecodeErrorKind::InvalidEncoding,
                    Some("aarch64".to_string()),
                    "Pmull with U=1 is unallocated".to_string(),
                ));
            }
            let (dest_arr, src_arr) = match size {
                0b00 => (".8h", if !q { ".8b" } else { ".16b" }),
                0b11 => (".1q", if !q { ".1d" } else { ".2d" }),
                _ => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        format!("Pmull with size={} is unallocated", size),
                    ));
                }
            };
            let mnemonic = if !q { Mnemonic::Pmull } else { Mnemonic::Pmull2 };
            Ok((
                mnemonic,
                vec![
                    Operand::Text {
                        value: format!("v{}{}", rd_val, dest_arr),
                    },
                    Operand::Text {
                        value: format!("v{}{}", rn_val, src_arr),
                    },
                    Operand::Text {
                        value: format!("v{}{}", rm_val, src_arr),
                    },
                ],
            ))
        }

        _ => Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            format!("Unimplemented SIMD Three Different opcode 0b{:04b}", opcode),
        )),
    }
}

pub fn decode_simd_permute_table(word: u32) -> DecodeResult {
    let op = bit(word, 29);
    let q = q_bit(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);

    if op {
        // Advanced SIMD extract: EXT
        // op2 (bits22:21) must be 00; imm4 in bits14:11
        let op2 = bits(word, 22, 21) as u8;
        if op2 != 0b00 {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                format!("Unallocated EXT op2={op2}"),
            ));
        }
        let imm4 = bits(word, 14, 11) as u8;
        let arr = if !q { ".8b" } else { ".16b" };
        return Ok((
            Mnemonic::Ext,
            vec![
                Operand::Text {
                    value: format!("v{}{}", rd_val, arr),
                },
                Operand::Text {
                    value: format!("v{}{}", rn_val, arr),
                },
                Operand::Text {
                    value: format!("v{}{}", rm_val, arr),
                },
                Operand::Immediate {
                    value: i64::from(imm4),
                },
            ],
        ));
    }

    // op=0: either table lookup or permute
    let b11 = bit(word, 11);

    if !b11 {
        // Advanced SIMD table lookup: TBL / TBX
        let len = bits(word, 14, 13) as u8;
        let tbl_op = bit(word, 12);
        let mnemonic = if !tbl_op { Mnemonic::Tbl } else { Mnemonic::Tbx };

        // Build register list: { vRn.16b, vRn+1.16b, ... }
        let table_regs: Vec<String> = (0..=len)
            .map(|i| {
                let reg = (rn_val + i) & 0x1F;
                format!("v{}.16b", reg)
            })
            .collect();
        let table_list = format!("{{ {} }}", table_regs.join(", "));

        let dest_arr = if !q { ".8b" } else { ".16b" };
        let idx_arr = if !q { ".8b" } else { ".16b" };

        return Ok((
            mnemonic,
            vec![
                Operand::Text {
                    value: format!("v{}{}", rd_val, dest_arr),
                },
                Operand::Text { value: table_list },
                Operand::Text {
                    value: format!("v{}{}", rm_val, idx_arr),
                },
            ],
        ));
    }

    // Advanced SIMD permute: UZP1, TRN1, ZIP1, UZP2, TRN2, ZIP2
    let size = simd_size(word);
    let opcode = bits(word, 14, 12) as u8;
    let arr_suffix = arrangement_suffix(size, q).unwrap_or("?");

    let mnemonic = match opcode {
        0b001 => Mnemonic::Uzp1,
        0b010 => Mnemonic::Trn1,
        0b011 => Mnemonic::Zip1,
        0b101 => Mnemonic::Uzp2,
        0b110 => Mnemonic::Trn2,
        0b111 => Mnemonic::Zip2,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("Unimplemented SIMD permute opcode {opcode}"),
            ))
        }
    };

    Ok((
        mnemonic,
        vec![
            Operand::Text {
                value: format!("v{}{}", rd_val, arr_suffix),
            },
            Operand::Text {
                value: format!("v{}{}", rn_val, arr_suffix),
            },
            Operand::Text {
                value: format!("v{}{}", rm_val, arr_suffix),
            },
        ],
    ))
}



fn decode_imm5(imm5: u8) -> Option<(char, u8, u8)> {
    if imm5 & 0b00001 != 0 {
        Some(('b', (imm5 >> 1) & 0xF, 0b00))
    } else if imm5 & 0b00010 != 0 {
        Some(('h', (imm5 >> 2) & 0x7, 0b01))
    } else if imm5 & 0b00100 != 0 {
        Some(('s', (imm5 >> 3) & 0x3, 0b10))
    } else if imm5 & 0b01000 != 0 {
        Some(('d', (imm5 >> 4) & 0x1, 0b11))
    } else {
        None
    }
}

