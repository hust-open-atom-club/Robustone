//! Scalar FP data-processing instructions.

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::operands;
use crate::shared::registers::{ftype_to_size, FpRegSize};
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

pub fn decode_scalar_fp(word: u32) -> DecodeResult {
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
            return super::vector::decode_crypto_sha2(word);
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
        return super::vector::decode_crypto_sha3(word);
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
            "Invalid ftype for FP 2-source",
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
                format!("Unimplemented FP 2-source opcode 0b{opcode:06b}"),
            ))
        }
    };

    Ok((
        mnemonic,
        vec![
            super::fp_reg_operand(rd_val, size),
            super::fp_reg_operand(rn_val, size),
            super::fp_reg_operand(rm_val, size),
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
            "Invalid ftype for FP 3-source",
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
            super::fp_reg_operand(rd_val, size),
            super::fp_reg_operand(rn_val, size),
            super::fp_reg_operand(rm_val, size),
            super::fp_reg_operand(ra_val, size),
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
            "Invalid ftype for FP 1-source",
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
                format!("Unimplemented FP 1-source opcode 0b{opcode:06b}"),
            ))
        }
    };

    Ok((
        mnemonic,
        vec![super::fp_reg_operand(rd_val, size), super::fp_reg_operand(rn_val, size)],
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
            "Invalid ftype for FP immediate",
        )
    })?;

    let rd_val = rd(word);
    let imm8 = bits(word, 20, 13) as u8;
    let imm_text = decode_fp_imm8(imm8, size);

    Ok((
        Mnemonic::Fmov,
        vec![
            super::fp_reg_operand(rd_val, size),
            Operand::Text { value: imm_text },
        ],
    ))
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
            "Invalid ftype for FP compare",
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
            super::fp_reg_operand(rn_val, size),
            Operand::Text {
                value: "#0.00000000".to_string(),
            },
        ]
    } else {
        vec![super::fp_reg_operand(rn_val, size), super::fp_reg_operand(rm_val, size)]
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
            "Invalid ftype for FP conditional select",
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
                "Invalid condition code",
            )
        })?
        .as_str();

    Ok((
        Mnemonic::Fcsel,
        vec![
            super::fp_reg_operand(rd_val, size),
            super::fp_reg_operand(rn_val, size),
            super::fp_reg_operand(rm_val, size),
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
            "Invalid ftype for FP conversion",
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
                        "Invalid FCVT source size",
                    ))
                }
            };
            Ok((
                Mnemonic::Fcvt,
                vec![
                    super::fp_reg_operand(rd_val, size),
                    super::fp_reg_operand(rn_val, src_size),
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
                    super::fp_reg_operand(rd_val, size),
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
                    super::fp_reg_operand(rn_val, size),
                ],
            ))
        }
        _ => Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            format!("Unimplemented FP conversion opcode 0b{opcode:03b}"),
        )),
    }
}

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

    // Reference renders with 8 decimal places for S/D, fewer for H.
    match size {
        FpRegSize::H => format!("#{:.4}", value),
        _ => format!("#{:.8}", value),
    }
}

