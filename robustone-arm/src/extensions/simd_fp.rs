//! AArch64 SIMD/FP data processing instructions.
//!
//! Covers scalar FP data processing (op1[4] = 1) and Advanced SIMD (vector)
//! data processing (op1[4] = 0) within the SIMD/FP major opcode group
//! (op0 = 0x7 / 0xF).

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::operands;
use crate::shared::registers::{arrangement_suffix, fp_simd_reg_name, ftype_to_size, FpRegSize};
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

// ---------------------------------------------------------------------------
// Top-level dispatch
// ---------------------------------------------------------------------------

/// Decode a SIMD/FP instruction.
///
/// Called from `decode_by_op0` for op0 = 0x7 / 0xF.
/// Uses `op1` (bits 28:24) to separate scalar FP from Advanced SIMD.
pub fn decode_simd_fp(word: u32, _addr: u64) -> DecodeResult {
    let op1 = op1_5bit(word);
    if (op1 & 0x10) != 0 {
        // Scalar FP data processing (op1[4] = 1)
        decode_scalar_fp(word)
    } else {
        // Advanced SIMD — Stage 3C
        decode_advanced_simd(word)
    }
}

// ---------------------------------------------------------------------------
// Advanced SIMD dispatch
// ---------------------------------------------------------------------------

/// Decode Advanced SIMD (vector) data-processing instructions.
///
/// Classification follows ARM ARM Table C4-300 for op0 = 0x7/0xF, bit28=0:
fn decode_advanced_simd(word: u32) -> DecodeResult {
    let b24 = bit(word, 24);
    let b21 = bit(word, 21);
    let b10 = bit(word, 10);
    let b22 = bit(word, 22);
    let immh = bits(word, 22, 19) as u8;

    // Modified Immediate: b24=1, b10=1, immh=0000
    // Shift Immediate: b24=1, b10=1, immh≠0000
    if b24 && b10 {
        if immh == 0b0000 {
            return decode_simd_modified_imm(word);
        } else {
            return decode_simd_shift_imm(word);
        }
    }

    // Indexed Element: b24=1, b10=0
    if b24 && !b10 {
        return decode_simd_indexed_element(word);
    }

    // Three Same (integer + FP32/64): b21=1, b10=1
    if b21 && b10 {
        return decode_simd_three_same(word);
    }

    // b21=0, b10=1: either FP16 Three Same or Copy
    if !b21 && b10 {
        if b22 {
            return decode_simd_fp16_three_same(word);
        } else {
            return decode_simd_copy(word);
        }
    }

    // b21=1, b10=0: Three Different / Across Lanes / AES / Two-reg Misc
    if b21 && !b10 {
        let b11 = bit(word, 11);
        let op5_16 = bits(word, 16, 12) as u8;
        let size = simd_size(word);
        let bits_20_16 = bits(word, 20, 16) as u8;

        // Crypto AES: op5_16 in {4,5,6,7}, size=0, bits20:16=8
        if (4..=7).contains(&op5_16) && size == 0 && bits_20_16 == 0b01000 {
            return decode_crypto_aes(word);
        }

        // Across Lanes: specific opcodes
        if let Some(result) = try_decode_simd_across_lanes(word, op5_16) {
            return Ok(result);
        }

        // CRITICAL FIX: Check bit 11 to distinguish Three Different from Two-reg Misc.
        // bit 11 = 0: Three Different (opcode in bits 15:12, 4 bits)
        // bit 11 = 1: Two-register Misc (opcode in bits 16:12, 5 bits)
        if !b11 {
            return decode_simd_three_different(word);
        }

        // Two-register Misc (b11=1)
        return decode_simd_two_reg_misc(word, op5_16);
    }

    // b21=0, b10=0: Permute / Extract / Table
    if !b21 && !b10 {
        return decode_simd_permute_table(word);
    }

    Err(DisasmError::decode_failure(
        DecodeErrorKind::UnimplementedInstruction,
        Some("aarch64".to_string()),
        "unrecognized Advanced SIMD encoding",
    ))
}

// ---------------------------------------------------------------------------
// Scalar FP dispatch
// ---------------------------------------------------------------------------

/// Decode scalar FP data-processing instructions.
///
/// Classification follows the AArch64 encoding structure:
/// - bit24=1 → 3-source (FMADD, FMSUB, FNMADD, FNMSUB)
/// - bit21=1 → 2-source (FADD, FSUB, FMUL, FDIV, etc.)
/// - b24=0, b21=0 → mixed group distinguished by opcode6 and opcode3
fn decode_scalar_fp(word: u32) -> DecodeResult {
    let m = bit(word, 31);
    let b24 = bit(word, 24);
    let b21 = bit(word, 21);
    let opcode6 = bits(word, 15, 10) as u8;
    let opcode3 = bits(word, 18, 16) as u8;
    let rd_val = rd(word);

    if m {
        // M=1: conversions from 64-bit integer (UCVTF, SCVTF with Xn)
        return decode_fp_conversion(word);
    }

    if b24 {
        // 3-source (FMADD, FMSUB, FNMADD, FNMSUB)
        return decode_fp_3source(word, opcode6);
    }

    if b21 {
        // 2-source data-processing OR cryptographic two-register SHA
        let bits_20_16 = bits(word, 20, 16) as u8;
        let bit30 = bit(word, 30);

        // Cryptographic two-register SHA uses bit30=1 (0x5E prefix) with bits(20:16)=0b01000
        if bit30 && bits_20_16 == 0b01000 {
            return decode_crypto_sha2(word);
        }

        return decode_fp_2source(word, opcode6);
    }

    // b24=0, b21=0: mixed group (1-source, conversions, compares, conditional selects,
    // immediates, AND cryptographic three-register SHA)
    //
    // Cryptographic three-register SHA encodes in the scalar FP space with bit30=1
    // (0x5E prefix), bits(23:21)=000, bit11=0, and bits(15:12) <= 6.
    let bit30 = bit(word, 30);
    let b23_21 = bits(word, 23, 21) as u8;
    let bits_15_12 = bits(word, 15, 12) as u8;
    let b11 = bit(word, 11);
    if bit30 && b23_21 == 0b000 && !b11 && bits_15_12 <= 0b0110 {
        return decode_crypto_sha3(word);
    }

    // Note: opcode6 may include bits from imm8 (for FMOV imm) or Rm (for FCSEL),
    // so we also check bits(12:10) where needed.
    let bits_12_10 = bits(word, 12, 10) as u8;
    let rm_val = rm(word);
    let rn_val = rn(word);
    match opcode6 {
        // FMOV (immediate): bits 12:10 = 100 and Rn = 0 (reserved field)
        _ if bits_12_10 == 0b100 && rn_val == 0 => decode_fp_immediate(word),
        0b001000 | 0b001001 if rd_val == 0 => decode_fp_compare(word),
        0b000011 if rm_val != 0 => decode_fp_conditional_select(word),
        _ => {
            // 1-source or conversion
            if opcode3 == 0b000 {
                decode_fp_1source(word, opcode6)
            } else {
                decode_fp_conversion(word)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 2-source FP
// ---------------------------------------------------------------------------

/// Decode 2-source FP instructions.
fn decode_fp_2source(word: u32, opcode: u8) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP 2-source",
        )
    })?;

    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);

    let mnemonic = match opcode {
        0b001010 => Mnemonic::Fadd,
        0b001110 => Mnemonic::Fsub,
        0b000010 => Mnemonic::Fmul,
        0b000110 => Mnemonic::Fdiv,
        0b001000 => Mnemonic::Fmax,
        0b001001 => Mnemonic::Fmin,
        0b010000 => Mnemonic::Fmaxnm,
        0b010001 => Mnemonic::Fminnm,
        0b010010 => Mnemonic::Fnmul,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("unimplemented FP 2-source opcode 0b{opcode:06b}"),
            ))
        }
    };

    Ok((
        mnemonic,
        vec![
            fp_reg_operand(rd_val, size),
            fp_reg_operand(rn_val, size),
            fp_reg_operand(rm_val, size),
        ],
    ))
}

// ---------------------------------------------------------------------------
// 3-source FP
// ---------------------------------------------------------------------------

/// Decode 3-source FP instructions (FMADD, FMSUB, FNMADD, FNMSUB).
fn decode_fp_3source(word: u32, _opcode: u8) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP 3-source",
        )
    })?;

    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let ra_val = ra(word);

    // o0 = bit 15 distinguishes fmadd (0) from fmsub (1)
    let o0 = bit(word, 15);
    // N = bit 31 distinguishes fnmadd/fnmsub (N=1) from fmadd/fmsub (N=0)
    // But M is also bit 31. For scalar 3-source, M=0 always.
    // FNMADD/FNMSUB use a different encoding pattern; for now we only
    // decode the two most common: FMADD and FMSUB.
    let mnemonic = if !o0 {
        Mnemonic::Fmadd
    } else {
        Mnemonic::Fmsub
    };

    Ok((
        mnemonic,
        vec![
            fp_reg_operand(rd_val, size),
            fp_reg_operand(rn_val, size),
            fp_reg_operand(rm_val, size),
            fp_reg_operand(ra_val, size),
        ],
    ))
}

// ---------------------------------------------------------------------------
// 1-source FP
// ---------------------------------------------------------------------------

/// Decode 1-source FP instructions.
fn decode_fp_1source(word: u32, opcode: u8) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP 1-source",
        )
    })?;

    let rd_val = rd(word);
    let rn_val = rn(word);

    let mnemonic = match opcode {
        0b000000 => Mnemonic::Fmov,
        0b000001 => Mnemonic::Fabs,
        0b000010 => Mnemonic::Fneg,
        0b000011 => Mnemonic::Fsqrt,
        0b001000 => Mnemonic::Frintn,
        0b001001 => Mnemonic::Frintp,
        0b001010 => Mnemonic::Frintm,
        0b001011 => Mnemonic::Frintz,
        0b001100 => Mnemonic::Frinta,
        0b001101 => Mnemonic::Frintx,
        0b001110 => Mnemonic::Frinti,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("unimplemented FP 1-source opcode 0b{opcode:06b}"),
            ))
        }
    };

    Ok((
        mnemonic,
        vec![fp_reg_operand(rd_val, size), fp_reg_operand(rn_val, size)],
    ))
}

// ---------------------------------------------------------------------------
// FP immediate
// ---------------------------------------------------------------------------

/// Decode FP immediate instructions.
///
/// Currently implements `FMOV (immediate)`; all others are unimplemented.
fn decode_fp_immediate(word: u32) -> DecodeResult {
    let m = bit(word, 31);


    if m {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "FP immediate with M=1 is reserved",
        ));
    }

    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP immediate",
        )
    })?;

    let rd_val = rd(word);
    let imm8 = bits(word, 20, 13) as u8;
    let imm_text = decode_fp_imm8(imm8, size);

    Ok((
        Mnemonic::Fmov,
        vec![
            fp_reg_operand(rd_val, size),
            Operand::Text { value: imm_text },
        ],
    ))
}

/// Decode an 8-bit FP immediate into a Capstone-compatible text operand.
///
/// The imm8 encodes a small floating-point constant.  For simplicity we
/// decode the common cases and fall back to a generic representation.
fn decode_fp_imm8(imm8: u8, size: FpRegSize) -> String {
    // imm8 layout: [7]=sign, [6:4]=exponent, [3:0]=fraction
    let sign = (imm8 >> 7) & 1;
    let exp = (imm8 >> 4) & 0x7;
    let frac = (imm8 & 0xF) as u32;

    // See ARM ARM "Floating-point modified immediate":
    // value = (-1)^sign * 2^exp * (1.fraction)
    // where exp is biased by 3 (i.e. real exponent = exp - 3).
    let mantissa = 16.0 + f64::from(frac); // 1.fraction in 4-bit form
    let real_exp = f64::from(exp) - 3.0;
    let mut value = mantissa * f64::powf(2.0, real_exp - 4.0); // -4 because frac is 4 bits

    if sign != 0 {
        value = -value;
    }

    // Capstone renders with 8 decimal places for S/D, fewer for H.
    match size {
        FpRegSize::H => format!("#{:.4}", value),
        _ => format!("#{:.8}", value),
    }
}

// ---------------------------------------------------------------------------
// FP compare
// ---------------------------------------------------------------------------

/// Decode FP compare instructions (FCMP, FCMPE).
fn decode_fp_compare(word: u32) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP compare",
        )
    })?;

    let rn_val = rn(word);
    let rm_val = rm(word);
    let op = bit(word, 3);
    let e = bit(word, 4);

    let mnemonic = if e { Mnemonic::Fcmpe } else { Mnemonic::Fcmp };

    // When Rm = 0 and bit 3 (op) = 0, the instruction compares against #0.0
    let ops = if rm_val == 0 && !op {
        vec![
            fp_reg_operand(rn_val, size),
            Operand::Text {
                value: "#0.00000000".to_string(),
            },
        ]
    } else {
        vec![fp_reg_operand(rn_val, size), fp_reg_operand(rm_val, size)]
    };

    Ok((mnemonic, ops))
}

// ---------------------------------------------------------------------------
// FP conditional select
// ---------------------------------------------------------------------------

/// Decode FP conditional select (FCSEL).
fn decode_fp_conditional_select(word: u32) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP conditional select",
        )
    })?;

    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    // FCSEL encodes the condition in bits 15:12, not bits 3:0.
    let cond_val = opcode_4bit(word);
    let cond_str = ConditionCode::from_bits(cond_val)
        .ok_or_else(|| {
            DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "invalid condition code",
            )
        })?
        .as_str();

    Ok((
        Mnemonic::Fcsel,
        vec![
            fp_reg_operand(rd_val, size),
            fp_reg_operand(rn_val, size),
            fp_reg_operand(rm_val, size),
            operands::text(cond_str),
        ],
    ))
}

// ---------------------------------------------------------------------------
// FP conversion
// ---------------------------------------------------------------------------

/// Decode FP conversion instructions.
///
/// Covers FCVT between FP sizes, SCVTF/UCVTF from integer to FP, and
/// FCVT* to integer (rendered as `fcvt` with size suffixes on operands).
fn decode_fp_conversion(word: u32) -> DecodeResult {
    let ftype_val = ftype(word);
    let size = ftype_to_size(ftype_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid ftype for FP conversion",
        )
    })?;

    let rd_val = rd(word);
    let rn_val = rn(word);
    let opcode = bits(word, 18, 16) as u8;
    let rmode = bits(word, 15, 14) as u8;
    let opc = bits(word, 13, 10) as u8;

    // Distinguish by opcode (bits 18:16):
    // 0b000 = FCVT (general, i.e. FP <-> integer)
    // 0b001 = FCVT (between FP precisions)
    // 0b010 = SCVTF / UCVTF (integer -> FP)
    // 0b011 = SCVTF / UCVTF (integer -> FP, M=1 for 64-bit)
    match opcode {
        0b001 => {
            // FCVT between FP precisions
            // opc encodes the source size
            let src_size = match opc {
                0b0000 => FpRegSize::H,
                0b0001 => FpRegSize::S,
                0b0010 => FpRegSize::D,
                0b0011 => FpRegSize::B,
                _ => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        "invalid FCVT source size",
                    ))
                }
            };
            Ok((
                Mnemonic::Fcvt,
                vec![
                    fp_reg_operand(rd_val, size),
                    fp_reg_operand(rn_val, src_size),
                ],
            ))
        }
        0b010 | 0b011 => {
            // SCVTF / UCVTF
            // sf (bit 31) determines integer register size: 0 = W, 1 = X
            let is_32bit = sf(word).is_w();
            let is_unsigned = (opc & 1) != 0;
            let mnemonic = if is_unsigned {
                Mnemonic::Ucvtf
            } else {
                Mnemonic::Scvtf
            };
            let int_reg = if is_32bit {
                format!("w{}", rn_val)
            } else {
                format!("x{}", rn_val)
            };
            Ok((
                mnemonic,
                vec![
                    fp_reg_operand(rd_val, size),
                    Operand::Text { value: int_reg },
                ],
            ))
        }
        0b000 => {
            // FCVT to integer (general) — rendered as fcvt with rounding mode
            let _ = (rmode, opc); // used for rounding mode / signed / unsigned
            // For simplicity, render as `fcvt` with integer destination
            let is_32bit = sf(word).is_w();
            let int_reg = if is_32bit {
                format!("w{}", rd_val)
            } else {
                format!("x{}", rd_val)
            };
            Ok((
                Mnemonic::Fcvt,
                vec![
                    Operand::Text { value: int_reg },
                    fp_reg_operand(rn_val, size),
                ],
            ))
        }
        _ => Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            format!("unimplemented FP conversion opcode 0b{opcode:03b}"),
        )),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a register operand for an FP/SIMD register.
fn fp_reg_operand(reg: u8, size: FpRegSize) -> Operand {
    Operand::Text {
        value: fp_simd_reg_name(reg, size).unwrap_or("?").to_string(),
    }
}

/// Create a vector register operand with arrangement suffix.
fn vec_reg_operand(reg: u8, size: u8, q: bool) -> Operand {
    let suffix = arrangement_suffix(size, q).unwrap_or("?");
    Operand::Text {
        value: format!("v{}{}", reg, suffix),
    }
}

// ---------------------------------------------------------------------------
// Advanced SIMD sub-decoders
// ---------------------------------------------------------------------------

/// Decode SIMD Three Same (integer and FP32/64 vector).
///
/// Encoding: b24=0, b21=1, b10=1.
/// Fields: U=bit(29), size=bits(23:22), opcode=bits(15:11), Q=bit(30).
fn decode_simd_three_same(word: u32) -> DecodeResult {
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
                    format!("unimplemented Three Same opcode {opcode} U={u}"),
                ))
            }
        };

        // For bitwise opcodes (opcode 3), the arrangement is always .8b/.16b
        // because these operate on the entire vector as bytes.
        let arr_size = if opcode == 3 { 0b00 } else { size };

        return Ok((
            mnemonic,
            vec![
                vec_reg_operand(rd_val, arr_size, q),
                vec_reg_operand(rn_val, arr_size, q),
                vec_reg_operand(rm_val, arr_size, q),
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
                "reserved FP Three Same opcode 28 size=10 U=0".to_string(),
            ))
        }
        (true, 28, 0b10) => Mnemonic::Fcmgt,
        (false, 29, 0b10) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "reserved FP Three Same opcode 29 size=10 U=0".to_string(),
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
                "reserved FP Three Same opcode 28 size=11 U=0".to_string(),
            ))
        }
        (true, 28, 0b11) => Mnemonic::Fcmgt,
        (false, 29, 0b11) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "reserved FP Three Same opcode 29 size=11 U=0".to_string(),
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
                format!("unimplemented Three Same opcode {opcode} U={u} size={size:02b}"),
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
            vec_reg_operand(rd_val, fp_size, q),
            vec_reg_operand(rn_val, fp_size, q),
            vec_reg_operand(rm_val, fp_size, q),
        ],
    ))
}

/// Decode SIMD FP16 Three Same.
///
/// Encoding: b24=0, b21=0, b10=1, b22=1.
/// FP16 element size is implied; arrangement is .4h/.8h from Q.
/// Valid size values: size=01 (bit23=0) and size=11 (bit23=1).
fn decode_simd_fp16_three_same(word: u32) -> DecodeResult {
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
                format!("unimplemented FP16 Three Same opcode {opcode} U={u}"),
            ))
        }
    };

    // FP16 arrangement: size=01 or 11 both map to .4h (Q=0) or .8h (Q=1)
    let fp16_size = 0b01;

    Ok((
        mnemonic,
        vec![
            vec_reg_operand(rd_val, fp16_size, q),
            vec_reg_operand(rn_val, fp16_size, q),
            vec_reg_operand(rm_val, fp16_size, q),
        ],
    ))
}

/// Decode SIMD Two-register Miscellaneous.
///
/// Encoding: b24=0, b21=1, b10=0, b11=1.
/// Fields: U=bit(29), size=bits(23:22), opcode=bits(16:12), Q=bit(30).
fn decode_simd_two_reg_misc(word: u32, _op5_16: u8) -> DecodeResult {
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
                    format!("unimplemented Two-reg Misc opcode {opcode} U={u} size=00"),
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
                    format!("unimplemented Two-reg Misc opcode {opcode} U={u} size=01"),
                ))
            }
        },

        // size=10 and size=11: mostly reserved/invalid for the listed opcodes
        0b10 | 0b11 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                format!("unimplemented Two-reg Misc opcode {opcode} U={u} size={size:02b}"),
            ))
        }

        _ => unreachable!(),
    };

    // For narrow/widen ops, arrangement differs between Rd and Rn.
    // For now, use same arrangement for both (fix when tests reveal issues).
    Ok((
        mnemonic,
        vec![
            vec_reg_operand(rd_val, size, q),
            vec_reg_operand(rn_val, size, q),
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
///
/// Returns (element_char, index, arrangement_size) where arrangement_size
/// is 0=B, 1=H, 2=S, 3=D for use with `arrangement_suffix`.
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

fn decode_simd_copy(word: u32) -> DecodeResult {
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
            format!("invalid imm5 encoding 0b{imm5:05b} in SIMD copy"),
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
        // Capstone renders as `mov` alias.
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
        // Capstone uses `mov` alias for S and D elements.
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
        // Capstone renders as `mov` alias.
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
            format!("unimplemented SIMD copy Q={q} op={op} imm4=0b{imm4:04b}"),
        )),
    }
}

/// Decode SIMD modified immediate instructions.
///
/// Encoding: b24=1, b10=1, immh=0000.
/// Fields: op=bit(29), cmode=bits(15:12), Q=bit(30), o2=bit(11),
///         imm8={bit(18),bit(17),bit(16),bit(9),bit(8),bit(7),bit(6),bit(5)},
///         Rd=bits(4:0).
fn decode_simd_modified_imm(word: u32) -> DecodeResult {
    let op = bit(word, 29);
    let q = q_bit(word);
    let cmode = bits(word, 15, 12) as u8;
    let o2 = bit(word, 11);
    let rd_val = rd(word);

    if o2 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved o2=1 in SIMD modified immediate",
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
                    "reserved FMOV double-precision Q=0",
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
            format!("unimplemented SIMD modified immediate op={op} cmode={cmode:04b}"),
        )),
    }
}

/// Decode SIMD shift-by-immediate instructions.
///
/// Encoding: b24=1, b10=1, b23:21≠000.
/// Fields: U=bit(29), immh=bits(22:19), immb=bits(18:16), opcode=bits(15:11), Q=bit(30).
fn decode_simd_shift_imm(word: u32) -> DecodeResult {
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
            "invalid immh=0000 in SIMD shift immediate",
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
                    "reserved SIMD shift immediate opcode 01000 U=0",
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
                    "reserved SIMD shift immediate opcode 01100 U=0",
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
                format!("unimplemented SIMD shift immediate opcode {opcode:05b} U={u}"),
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
fn decode_simd_indexed_element(word: u32) -> DecodeResult {
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
                    "unimplemented SIMD indexed element opcode={opcode:04b} U={u} size={size}"
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
                        "invalid FP size for indexed element",
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
                        "invalid size for long multiply indexed element",
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
fn decode_crypto_aes(word: u32) -> DecodeResult {
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
                format!("unimplemented AES opcode {opcode}"),
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
fn decode_crypto_sha2(word: u32) -> DecodeResult {
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
                format!("unimplemented SHA-2 opcode 0b{opcode:02b}"),
            ))
        }
    };

    match mnemonic {
        Mnemonic::Sha1h => {
            // sha1h uses scalar S registers
            Ok((
                mnemonic,
                vec![
                    fp_reg_operand(rd_val, FpRegSize::S),
                    fp_reg_operand(rn_val, FpRegSize::S),
                ],
            ))
        }
        _ => {
            // sha1su1, sha256su0 use vector .4s (Q=1 for crypto space)
            Ok((
                mnemonic,
                vec![
                    vec_reg_operand(rd_val, 0b10, true), // .4s
                    vec_reg_operand(rn_val, 0b10, true),
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
fn decode_crypto_sha3(word: u32) -> DecodeResult {
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
                format!("unimplemented SHA-3 opcode 0b{opcode:03b}"),
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
                fp_reg_operand(rn_val, FpRegSize::S),
                vec_reg_operand(rm_val, 0b10, true), // .4s
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
                vec_reg_operand(rm_val, 0b10, true), // .4s
            ]
        }
        _ => {
            // sha1su0, sha256su1: Vd.4s, Vn.4s, Vm.4s
            vec![
                vec_reg_operand(rd_val, 0b10, true),
                vec_reg_operand(rn_val, 0b10, true),
                vec_reg_operand(rm_val, 0b10, true),
            ]
        }
    };

    Ok((mnemonic, ops))
}

fn try_decode_simd_across_lanes(word: u32, op5_16: u8) -> Option<(Mnemonic, Vec<Operand>)> {
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

    let dest = fp_reg_operand(rd_val, dest_size);

    // FP across-lanes use size+1 for vector arrangement (00→H/.4h, 01→S/.2s, 10→D/.1d)
    // Integer across-lanes use size directly.
    let src_size = match mnemonic {
        Mnemonic::Fmaxnmv | Mnemonic::Fminnmv | Mnemonic::Fmaxv | Mnemonic::Fminv => {
            size + 1
        }
        _ => size,
    };
    let src = vec_reg_operand(rn_val, src_size, q);

    Some((mnemonic, vec![dest, src]))
}

fn decode_simd_three_different(word: u32) -> DecodeResult {
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
                    "pmull with U=1 is unallocated".to_string(),
                ));
            }
            let (dest_arr, src_arr) = match size {
                0b00 => (".8h", if !q { ".8b" } else { ".16b" }),
                0b11 => (".1q", if !q { ".1d" } else { ".2d" }),
                _ => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        format!("pmull with size={} is unallocated", size),
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
            format!("unimplemented SIMD Three Different opcode 0b{:04b}", opcode),
        )),
    }
}

fn decode_simd_permute_table(word: u32) -> DecodeResult {
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
                format!("unallocated EXT op2={op2}"),
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
                format!("unimplemented SIMD permute opcode {opcode}"),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn disasm_fp_word(word: u32) -> (String, Vec<String>) {
        let bytes = word.to_le_bytes();
        let (mnemonic, operands) = decode_simd_fp(
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            0,
        )
        .unwrap();
        let op_texts: Vec<String> = operands
            .iter()
            .map(|op| match op {
                Operand::Text { value } => value.clone(),
                Operand::Register { register } => format!("r{}", register.id),
                Operand::Immediate { value } => format!("#{}", value),
                Operand::Memory { .. } => "mem".to_string(),
            })
            .collect();
        (mnemonic.as_str().to_string(), op_texts)
    }

    // 2-source FP
    #[test]
    fn test_fadd_s() {
        let word = 0x1E222820; // fadd s0, s1, s2
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fadd");
        assert_eq!(ops, vec!["s0", "s1", "s2"]);
    }

    #[test]
    fn test_fsub_d() {
        let word = 0x1E653883; // fsub d3, d4, d5
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fsub");
        assert_eq!(ops, vec!["d3", "d4", "d5"]);
    }

    #[test]
    fn test_fmul_s() {
        let word = 0x1E2808E6; // fmul s6, s7, s8
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmul");
        assert_eq!(ops, vec!["s6", "s7", "s8"]);
    }

    #[test]
    fn test_fdiv_h() {
        let word = 0x1EAB1949; // fdiv h9, h10, h11
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fdiv");
        assert_eq!(ops, vec!["h9", "h10", "h11"]);
    }

    // 3-source FP
    #[test]
    fn test_fmadd_s() {
        let word = 0x1F020C20; // fmadd s0, s1, s2, s3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmadd");
        assert_eq!(ops, vec!["s0", "s1", "s2", "s3"]);
    }

    #[test]
    fn test_fmsub_d() {
        let word = 0x1F459C83; // fmsub d3, d4, d5, d7
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmsub");
        assert_eq!(ops, vec!["d3", "d4", "d5", "d7"]);
    }

    // 1-source FP
    #[test]
    fn test_fabs_s() {
        let word = 0x1E000420; // fabs s0, s1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fabs");
        assert_eq!(ops, vec!["s0", "s1"]);
    }

    #[test]
    fn test_fneg_d() {
        let word = 0x1E400862; // fneg d2, d3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fneg");
        assert_eq!(ops, vec!["d2", "d3"]);
    }

    #[test]
    fn test_fsqrt_s() {
        let word = 0x1E000CA4; // fsqrt s4, s5
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fsqrt");
        assert_eq!(ops, vec!["s4", "s5"]);
    }

    #[test]
    fn test_fmov_reg() {
        let word = 0x1E0000E6; // fmov s6, s7
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmov");
        assert_eq!(ops, vec!["s6", "s7"]);
    }

    // FP immediate
    #[test]
    fn test_fmov_imm() {
        let word = 0x1E0E1000; // fmov s0, #1.0 (imm8=0x70)
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmov");
        assert_eq!(ops[0], "s0");
        assert!(ops[1].starts_with("#"));
    }

    // FP compare
    #[test]
    fn test_fcmp_s() {
        let word = 0x1E012000; // fcmp s0, s1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fcmp");
        assert_eq!(ops.len(), 2);
    }

    // FP conditional select
    #[test]
    fn test_fcsel_s() {
        let word = 0x1E020C20; // fcsel s0, s1, s2, eq
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fcsel");
        assert_eq!(ops[0], "s0");
        assert_eq!(ops[1], "s1");
        assert_eq!(ops[2], "s2");
        assert_eq!(ops[3], "eq");
    }

    // FP conversion
    #[test]
    fn test_fcvt_sd() {
        let word = 0x1E010820; // fcvt s0, d1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fcvt");
        assert_eq!(ops[0], "s0");
        assert_eq!(ops[1], "d1");
    }

    #[test]
    fn test_scvtf_ws() {
        let word = 0x1E020020; // scvtf s0, w1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "scvtf");
        assert_eq!(ops[0], "s0");
        assert_eq!(ops[1], "w1");
    }

    #[test]
    fn test_ucvtf_xd() {
        let word = 0x9E430420; // ucvtf d0, x1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ucvtf");
        assert_eq!(ops[0], "d0");
        assert_eq!(ops[1], "x1");
    }

    // Stage 3C: Advanced SIMD Three Same
    #[test]
    fn test_vadd_8b() {
        let word = 0x0E228420; // add v0.8b, v1.8b, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "add");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "v2.8b"]);
    }

    #[test]
    fn test_vadd_16b() {
        let word = 0x4E228420; // add v0.16b, v1.16b, v2.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "add");
        assert_eq!(ops, vec!["v0.16b", "v1.16b", "v2.16b"]);
    }

    #[test]
    fn test_vsub_4s() {
        // sub v0.4s, v1.4s, v2.4s: U=1, opcode=16, size=10, Q=1
        // byte3=0x4E: Q=1, U=0... wait, U=1 means byte3=0x6E
        // Actually sub is U=1, opcode=16. word = 0x6EA28420
        let word = 0x6EA28420; // sub v0.4s, v1.4s, v2.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sub");
        assert_eq!(ops, vec!["v0.4s", "v1.4s", "v2.4s"]);
    }

    #[test]
    fn test_vand_8b() {
        // and v0.8b, v1.8b, v2.8b: U=0, opcode=3, size=00
        // word = 0x0E221C20
        let word = 0x0E221C20; // and v0.8b, v1.8b, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "and");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "v2.8b"]);
    }

    #[test]
    fn test_vorr_8b() {
        // orr v0.8b, v1.8b, v2.8b: U=0, opcode=3, size=10
        // word = 0x0EA21C20
        let word = 0x0EA21C20; // orr v0.8b, v1.8b, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "orr");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "v2.8b"]);
    }

    #[test]
    fn test_vfadd_2s() {
        // fadd v0.2s, v1.2s, v2.2s: FP opcode 26, size=00 (FP32), U=0, Q=0
        // word = 0x0E22D420
        let word = 0x0E22D420; // fadd v0.2s, v1.2s, v2.2s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fadd");
        assert_eq!(ops, vec!["v0.2s", "v1.2s", "v2.2s"]);
    }

    #[test]
    fn test_vfadd_4s() {
        // fadd v0.4s, v1.4s, v2.4s: FP opcode 26, size=00 (FP32), U=0, Q=1
        // word = 0x4E22D420
        let word = 0x4E22D420; // fadd v0.4s, v1.4s, v2.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fadd");
        assert_eq!(ops, vec!["v0.4s", "v1.4s", "v2.4s"]);
    }

    #[test]
    fn test_vfadd_2d() {
        // fadd v0.2d, v1.2d, v2.2d: FP opcode 26, size=01 (FP64), U=0, Q=1
        // word = 0x4E62D420
        let word = 0x4E62D420; // fadd v0.2d, v1.2d, v2.2d
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fadd");
        assert_eq!(ops, vec!["v0.2d", "v1.2d", "v2.2d"]);
    }

    // Stage 3C: Advanced SIMD Two-register Misc
    #[test]
    fn test_vrev64_16b() {
        // rev64 v0.16b, v1.16b: U=0, opcode=0, size=00, Q=1, b21=1, b10=0, b11=1
        // word layout: Q=1, U=0, size=00, b21=1, opcode=00000, b11=1, Rn=1, Rd=0
        // bits 31:0 = 0_1_0_01101_00000_1_00001_00000 = 0x4E200820
        let word = 0x4E200820; // rev64 v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "rev64");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    #[test]
    fn test_vcls_16b() {
        // cls v0.16b, v1.16b: U=0, opcode=4, size=00, Q=1, b21=1, b10=0, b11=1
        // word layout: Q=1, U=0, size=00, b21=1, opcode=00100, b11=1, Rn=1, Rd=0
        // bits 31:0 = 0_1_0_01101_00100_1_00001_00000 = 0x4E204820
        let word = 0x4E204820; // cls v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "cls");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    #[test]
    fn test_vcnt_8b() {
        // cnt v0.8b, v1.8b: U=0, opcode=5, size=00, Q=0, b21=1, b10=0, b11=1
        // word layout: Q=0, U=0, size=00, b21=1, opcode=00101, b11=1, Rn=1, Rd=0
        // bits 31:0 = 0_0_0_01101_00101_1_00001_00000 = 0x0E205820
        let word = 0x0E205820; // cnt v0.8b, v1.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "cnt");
        assert_eq!(ops, vec!["v0.8b", "v1.8b"]);
    }

    #[test]
    fn test_vnot_16b() {
        // not v0.16b, v1.16b: U=1, opcode=5, size=00, Q=1, b21=1, b10=0, b11=1
        // word layout: Q=1, U=1, size=00, b21=1, opcode=00101, b11=1, Rn=1, Rd=0
        // bits 31:0 = 0_1_1_01101_00101_1_00001_00000 = 0x6E205820
        let word = 0x6E205820; // not v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "not");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    // Cryptographic AES
    #[test]
    fn test_aese() {
        let word = 0x4E284820; // aese v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "aese");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    #[test]
    fn test_aesd() {
        let word = 0x4E285820; // aesd v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "aesd");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    #[test]
    fn test_aesmc() {
        let word = 0x4E286820; // aesmc v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "aesmc");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    #[test]
    fn test_aesimc() {
        let word = 0x4E287820; // aesimc v0.16b, v1.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "aesimc");
        assert_eq!(ops, vec!["v0.16b", "v1.16b"]);
    }

    // Cryptographic SHA
    #[test]
    fn test_sha1h() {
        let word = 0x5E280820; // sha1h s0, s1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha1h");
        assert_eq!(ops, vec!["s0", "s1"]);
    }

    #[test]
    fn test_sha1su1() {
        let word = 0x5E280920; // sha1su1 v0.4s, v1.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha1su1");
        assert_eq!(ops, vec!["v0.4s", "v1.4s"]);
    }

    #[test]
    fn test_sha256su0() {
        let word = 0x5E280A20; // sha256su0 v0.4s, v1.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha256su0");
        assert_eq!(ops, vec!["v0.4s", "v1.4s"]);
    }

    #[test]
    fn test_sha1c() {
        let word = 0x5E020020; // sha1c q0, s1, v2.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha1c");
        assert_eq!(ops, vec!["q0", "s1", "v2.4s"]);
    }

    #[test]
    fn test_sha256h() {
        let word = 0x5E024020; // sha256h q0, q1, v2.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha256h");
        assert_eq!(ops, vec!["q0", "q1", "v2.4s"]);
    }

    #[test]
    fn test_sha256su1() {
        let word = 0x5E026020; // sha256su1 v0.4s, v1.4s, v2.4s
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sha256su1");
        assert_eq!(ops, vec!["v0.4s", "v1.4s", "v2.4s"]);
    }

    // Across Lanes
    #[test]
    fn test_saddlv() {
        let word = 0x0E303820; // saddlv h0, v1.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "saddlv");
        assert_eq!(ops, vec!["h0", "v1.8b"]);
    }

    #[test]
    fn test_addv() {
        let word = 0x0E31B820; // addv b0, v1.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "addv");
        assert_eq!(ops, vec!["b0", "v1.8b"]);
    }

    #[test]
    fn test_smaxv() {
        let word = 0x0E30A820; // smaxv b0, v1.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "smaxv");
        assert_eq!(ops, vec!["b0", "v1.8b"]);
    }

    #[test]
    fn test_fmaxv() {
        let word = 0x0E30F820; // fmaxv h0, v1.4h
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "fmaxv");
        assert_eq!(ops, vec!["h0", "v1.4h"]);
    }

    // SIMD Copy/Extract
    #[test]
    fn test_dup_element() {
        let word = 0x0E050441; // dup v1.8b, v2.b[2]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "dup");
        assert_eq!(ops, vec!["v1.8b", "v2.b[2]"]);
    }

    #[test]
    fn test_dup_element_q1() {
        let word = 0x4E050441; // dup v1.16b, v2.b[2]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "dup");
        assert_eq!(ops, vec!["v1.16b", "v2.b[2]"]);
    }

    #[test]
    fn test_dup_general() {
        let word = 0x0E010C21; // dup v1.8b, w1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "dup");
        assert_eq!(ops, vec!["v1.8b", "w1"]);
    }

    #[test]
    fn test_dup_general_d() {
        let word = 0x4E080C05; // dup v5.2d, x0
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "dup");
        assert_eq!(ops, vec!["v5.2d", "x0"]);
    }

    #[test]
    fn test_smov_w() {
        let word = 0x0E1F2C01; // smov w1, v0.b[15]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "smov");
        assert_eq!(ops, vec!["w1", "v0.b[15]"]);
    }

    #[test]
    fn test_smov_x() {
        let word = 0x4E1F2C01; // smov x1, v0.b[15]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "smov");
        assert_eq!(ops, vec!["x1", "v0.b[15]"]);
    }

    #[test]
    fn test_umov_b() {
        let word = 0x0E1F3C01; // umov w1, v0.b[15]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "umov");
        assert_eq!(ops, vec!["w1", "v0.b[15]"]);
    }

    #[test]
    fn test_mov_umov_alias_s() {
        let word = 0x0E143D34; // mov w20, v9.s[2]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["w20", "v9.s[2]"]);
    }

    #[test]
    fn test_mov_umov_alias_d() {
        let word = 0x4E183E47; // mov x7, v18.d[1]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["x7", "v18.d[1]"]);
    }

    #[test]
    fn test_ins_general() {
        let word = 0x4E051C22; // mov v2.b[2], w1
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["v2.b[2]", "w1"]);
    }

    #[test]
    fn test_ins_general_d() {
        let word = 0x4E181CE1; // mov v1.d[1], x7
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["v1.d[1]", "x7"]);
    }

    #[test]
    fn test_ins_element() {
        let word = 0x6E1D3461; // mov v1.b[14], v3.b[6]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["v1.b[14]", "v3.b[6]"]);
    }

    #[test]
    fn test_ins_element_h() {
        let word = 0x6E1E54E6; // mov v6.h[7], v7.h[5]
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, vec!["v6.h[7]", "v7.h[5]"]);
    }

    // SIMD Permute / Extract / Table
    #[test]
    fn test_ext() {
        let word = 0x2E021820; // ext v0.8b, v1.8b, v2.8b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ext");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "v2.8b", "#3"]);
    }

    #[test]
    fn test_ext_q1() {
        let word = 0x6E021820; // ext v0.16b, v1.16b, v2.16b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ext");
        assert_eq!(ops, vec!["v0.16b", "v1.16b", "v2.16b", "#3"]);
    }

    #[test]
    fn test_tbl_1reg() {
        let word = 0x0E020020; // tbl v0.8b, { v1.16b }, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "tbl");
        assert_eq!(ops, vec!["v0.8b", "{ v1.16b }", "v2.8b"]);
    }

    #[test]
    fn test_tbl_4reg() {
        let word = 0x0E026020; // tbl v0.8b, { v1.16b, v2.16b, v3.16b, v4.16b }, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "tbl");
        assert_eq!(ops, vec!["v0.8b", "{ v1.16b, v2.16b, v3.16b, v4.16b }", "v2.8b"]);
    }

    #[test]
    fn test_tbx_2reg() {
        let word = 0x0E023020; // tbx v0.8b, { v1.16b, v2.16b }, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "tbx");
        assert_eq!(ops, vec!["v0.8b", "{ v1.16b, v2.16b }", "v2.8b"]);
    }

    #[test]
    fn test_uzp1() {
        let word = 0x0E021820; // uzp1 v0.8b, v1.8b, v2.8b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "uzp1");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "v2.8b"]);
    }

    #[test]
    fn test_zip2() {
        let word = 0x4E027820; // zip2 v0.16b, v1.16b, v2.16b
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "zip2");
        assert_eq!(ops, vec!["v0.16b", "v1.16b", "v2.16b"]);
    }

    // SIMD Shift Immediate
    #[test]
    fn test_sshr_8b() {
        let word = 0x0F0D0420; // sshr v0.8b, v1.8b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sshr");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "#3"]);
    }

    #[test]
    fn test_ushr_16b() {
        let word = 0x6F0D0420; // ushr v0.16b, v1.16b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ushr");
        assert_eq!(ops, vec!["v0.16b", "v1.16b", "#3"]);
    }

    #[test]
    fn test_ssra_4h() {
        let word = 0x0F1D1420; // ssra v0.4h, v1.4h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ssra");
        assert_eq!(ops, vec!["v0.4h", "v1.4h", "#3"]);
    }

    #[test]
    fn test_srshr_2s() {
        let word = 0x0F3D2420; // srshr v0.2s, v1.2s, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "srshr");
        assert_eq!(ops, vec!["v0.2s", "v1.2s", "#3"]);
    }

    #[test]
    fn test_srsra_2d() {
        let word = 0x4F7D3420; // srsra v0.2d, v1.2d, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "srsra");
        assert_eq!(ops, vec!["v0.2d", "v1.2d", "#3"]);
    }

    #[test]
    fn test_sri_8b() {
        let word = 0x2F0D4420; // sri v0.8b, v1.8b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sri");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "#3"]);
    }

    #[test]
    fn test_shl_16b() {
        let word = 0x4F0B5420; // shl v0.16b, v1.16b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "shl");
        assert_eq!(ops, vec!["v0.16b", "v1.16b", "#3"]);
    }

    #[test]
    fn test_sli_4h() {
        let word = 0x2F135420; // sli v0.4h, v1.4h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sli");
        assert_eq!(ops, vec!["v0.4h", "v1.4h", "#3"]);
    }

    #[test]
    fn test_sqshlu_2s() {
        let word = 0x2F236420; // sqshlu v0.2s, v1.2s, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sqshlu");
        assert_eq!(ops, vec!["v0.2s", "v1.2s", "#3"]);
    }

    #[test]
    fn test_sqshl_imm_8b() {
        let word = 0x0F0B7420; // sqshl v0.8b, v1.8b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sqshl");
        assert_eq!(ops, vec!["v0.8b", "v1.8b", "#3"]);
    }

    #[test]
    fn test_uqshl_imm_4h() {
        let word = 0x2F137420; // uqshl v0.4h, v1.4h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "uqshl");
        assert_eq!(ops, vec!["v0.4h", "v1.4h", "#3"]);
    }

    #[test]
    fn test_shrn_8b() {
        let word = 0x0F0D8420; // shrn v0.8b, v1.8h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "shrn");
        assert_eq!(ops, vec!["v0.8b", "v1.8h", "#3"]);
    }

    #[test]
    fn test_shrn2_16b() {
        let word = 0x4F0D8420; // shrn2 v0.16b, v1.8h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "shrn2");
        assert_eq!(ops, vec!["v0.16b", "v1.8h", "#3"]);
    }

    #[test]
    fn test_sqshrun_4h() {
        let word = 0x2F1D8420; // sqshrun v0.4h, v1.4s, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sqshrun");
        assert_eq!(ops, vec!["v0.4h", "v1.4s", "#3"]);
    }

    #[test]
    fn test_rshrn_2s() {
        let word = 0x0F3D8C20; // rshrn v0.2s, v1.2d, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "rshrn");
        assert_eq!(ops, vec!["v0.2s", "v1.2d", "#3"]);
    }

    #[test]
    fn test_sqshrn_8b() {
        let word = 0x0F0D9420; // sqshrn v0.8b, v1.8h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sqshrn");
        assert_eq!(ops, vec!["v0.8b", "v1.8h", "#3"]);
    }

    #[test]
    fn test_uqshrn_4h() {
        let word = 0x2F1D9420; // uqshrn v0.4h, v1.4s, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "uqshrn");
        assert_eq!(ops, vec!["v0.4h", "v1.4s", "#3"]);
    }

    #[test]
    fn test_sqrshrn_2s() {
        let word = 0x0F3D9C20; // sqrshrn v0.2s, v1.2d, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sqrshrn");
        assert_eq!(ops, vec!["v0.2s", "v1.2d", "#3"]);
    }

    #[test]
    fn test_uqrshrn_8b() {
        let word = 0x2F0D9C20; // uqrshrn v0.8b, v1.8h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "uqrshrn");
        assert_eq!(ops, vec!["v0.8b", "v1.8h", "#3"]);
    }

    #[test]
    fn test_sshll_8h() {
        let word = 0x0F0BA420; // sshll v0.8h, v1.8b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sshll");
        assert_eq!(ops, vec!["v0.8h", "v1.8b", "#3"]);
    }

    #[test]
    fn test_sshll2_8h() {
        let word = 0x4F0BA420; // sshll2 v0.8h, v1.16b, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "sshll2");
        assert_eq!(ops, vec!["v0.8h", "v1.16b", "#3"]);
    }

    #[test]
    fn test_ushll_4s() {
        let word = 0x2F13A420; // ushll v0.4s, v1.4h, #3
        let (mnemonic, ops) = disasm_fp_word(word);
        assert_eq!(mnemonic, "ushll");
        assert_eq!(ops, vec!["v0.4s", "v1.4h", "#3"]);
    }
}
