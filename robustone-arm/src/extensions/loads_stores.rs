//! AArch64 Loads and Stores instructions.

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::registers::{gpr_name, reg_operand, RegContext};
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

/// Decode Loads and Stores instructions (op0 = 0x4, 0x6, 0xC, 0xE).
///
/// Classification per ARM ARM Table C4-4:
/// - op0=0x4 (0100, V=0): GPR loads/stores, lower half
/// - op0=0x6 (0110, V=1): SIMD/FP loads/stores, lower half — Stage 3
/// - op0=0xC (1100, V=0): GPR loads/stores, upper half
/// - op0=0xE (1110, V=1): SIMD/FP loads/stores, upper half — Stage 3
///
/// Within each op0, bit 29 and bit 24 (and sometimes bit 21) determine the sub-class.
pub fn decode_loads_stores(word: u32, addr: u64) -> DecodeResult {
    let op0_val = op0(word);
    let b29 = bit(word, 29);
    let b24 = bit(word, 24);

    match op0_val {
        0x4 => {
            // GPR loads/stores, lower half (V=0)
            if b29 == 0 {
                if b24 == 0 {
                    if bit(word, 23) == 0 {
                        // bit 21=0: single exclusive; bit 21=1: pair exclusive
                        decode_load_store_exclusive(word)
                    } else {
                        // LDLLR/STLLR etc. — ARMv8.1, not stage 2
                        Err(DisasmError::decode_failure(
                            DecodeErrorKind::UnimplementedInstruction,
                            Some("aarch64".to_string()),
                            "LL/SC acquire/release not in stage 2",
                        ))
                    }
                } else {
                    // Invalid encoding for this region
                    Err(DisasmError::decode_failure(
                        DecodeErrorKind::InvalidEncoding,
                        Some("aarch64".to_string()),
                        "invalid load/store encoding",
                    ))
                }
            } else {
                // b29=1: pair instructions
                // bit24=0,bit23=0 → no-allocate (STNP/LDNP)
                // bit24=0,bit23=1 → pair post-indexed (STP/LDP)
                // bit24=1,bit23=0 → pair signed offset (STP/LDP)
                // bit24=1,bit23=1 → pair pre-indexed (STP/LDP)
                if b24 == 0 && bit(word, 23) == 0 {
                    decode_load_store_pair_no_allocate(word)
                } else {
                    decode_load_store_pair(word)
                }
            }
        }
        0x6 => {
            // SIMD/FP loads/stores, lower half — Stage 3
            Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "SIMD/FP loads/stores not in stage 2",
            ))
        }
        0xC => {
            // GPR loads/stores, upper half (V=0)
            if b29 == 0 {
                if b24 == 0 {
                    decode_load_literal(word, addr)
                } else {
                    // RCpc (STLUR/LDLUR) — ARMv8.3, not stage 2
                    Err(DisasmError::decode_failure(
                        DecodeErrorKind::UnimplementedInstruction,
                        Some("aarch64".to_string()),
                        "RCpc loads/stores not in stage 2",
                    ))
                }
            } else {
                // b29=1
                if b24 == 0 {
                    if bit(word, 21) == 0 {
                        decode_load_store_register_immediate(word)
                    } else {
                        // bit21=1: register offset (bits 11:10=10) or atomic (bits 11:10=00)
                        if bits(word, 11, 10) == 0b10 {
                            decode_load_store_register_offset(word)
                        } else {
                            Err(DisasmError::decode_failure(
                                DecodeErrorKind::UnimplementedInstruction,
                                Some("aarch64".to_string()),
                                "atomic memory operations not in stage 2",
                            ))
                        }
                    }
                } else {
                    decode_load_store_register_unsigned_imm(word)
                }
            }
        }
        0xE => {
            // SIMD/FP loads/stores, upper half — Stage 3
            Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "SIMD/FP loads/stores not in stage 2",
            ))
        }
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// Load/store exclusive
// ---------------------------------------------------------------------------

fn decode_load_store_exclusive(word: u32) -> DecodeResult {
    let size = bits(word, 31, 30);
    let l = bit(word, 22);
    let o0 = bit(word, 15);
    let rs = bits(word, 20, 16) as u8;
    let rt2 = bits(word, 14, 10) as u8;
    let rn_val = rn(word);
    let rt_val = rt(word);

    // o0=1: not in stage 2 (atomic memory operations)
    if o0 == 1 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "atomic memory operations not in stage 2",
        ));
    }

    // bit 21 = 0: single register exclusive
    // bit 21 = 1: pair register exclusive
    if bit(word, 21) == 0 {
        // Single register exclusive
        let mnemonic = match (size, l) {
            (0b00, 0) => Mnemonic::Stxrb,
            (0b00, 1) => Mnemonic::Ldxrb,
            (0b01, 0) => Mnemonic::Stxrh,
            (0b01, 1) => Mnemonic::Ldxrh,
            (0b10, 0) | (0b11, 0) => Mnemonic::Stxr,
            (0b10, 1) | (0b11, 1) => Mnemonic::Ldxr,
            _ => unreachable!(),
        };

        let is_store = l == 0;
        if is_store {
            // STXR Ws, Rt, [Xn]
            // Ws is always a 32-bit register regardless of size
            let ws_text = gpr_name(rs, true, RegContext::DataProc);
            Ok((
                mnemonic,
                vec![
                    Operand::Text {
                        value: ws_text.to_string(),
                    },
                    reg_operand(rt_val),
                    addr_text(rn_val, 0, false, false, None, None),
                ],
            ))
        } else {
            // LDXR Rt, [Xn]
            Ok((
                mnemonic,
                vec![
                    reg_operand(rt_val),
                    addr_text(rn_val, 0, false, false, None, None),
                ],
            ))
        }
    } else {
        // Pair register exclusive
        let mnemonic = match (size, l) {
            (0b00, 0) | (0b01, 0) | (0b10, 0) | (0b11, 0) => Mnemonic::Stxp,
            (0b00, 1) | (0b01, 1) | (0b10, 1) | (0b11, 1) => Mnemonic::Ldxp,
            _ => unreachable!(),
        };

        let is_store = l == 0;
        if is_store {
            // STXP Ws, Rt, Rt2, [Xn]
            // Ws is always a 32-bit register regardless of size
            let ws_text = gpr_name(rs, true, RegContext::DataProc);
            Ok((
                mnemonic,
                vec![
                    Operand::Text {
                        value: ws_text.to_string(),
                    },
                    reg_operand(rt_val),
                    reg_operand(rt2),
                    addr_text(rn_val, 0, false, false, None, None),
                ],
            ))
        } else {
            // LDXP Rt, Rt2, [Xn]
            Ok((
                mnemonic,
                vec![
                    reg_operand(rt_val),
                    reg_operand(rt2),
                    addr_text(rn_val, 0, false, false, None, None),
                ],
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Load literal
// ---------------------------------------------------------------------------

fn decode_load_literal(word: u32, addr: u64) -> DecodeResult {
    let size = bits(word, 31, 30);
    let v = v_bit(word);
    let _opc = bits(word, 23, 22);
    let imm19 = ((word >> 5) & 0x7FFFF) as i64;
    let imm = imm19 << 2;
    let imm = if (imm & (1 << 20)) != 0 {
        imm | !((1 << 21) - 1)
    } else {
        imm
    };
    let target = (addr as i64).wrapping_add(imm);
    let rt_val = rt(word);

    if v == 1 {
        // SIMD/FP literal load — Stage 3
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "SIMD/FP load literal not in stage 2",
        ));
    }

    // Capstone is more permissive than the ARM ARM for load literal.
    // It decodes all size/opc combinations, mapping by size only:
    // - size=0b00: LDR (32-bit)
    // - size=0b01: LDR (64-bit)
    // - size=0b10: LDRSW
    // - size=0b11: PRFM (not in stage 2)
    let mnemonic = match size {
        0b00 | 0b01 => Mnemonic::Ldr,
        0b10 => Mnemonic::Ldrsw,
        0b11 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "PRFM not in stage 2",
            ));
        }
        _ => unreachable!(),
    };

    // Capstone formatting for load literal targets:
    // - 0..9: decimal
    // - >= 10 or negative: hex with 0x prefix
    let target_text = if (0..10).contains(&target) {
        format!("{target}")
    } else if target >= 0 {
        format!("0x{target:x}")
    } else {
        format!("0x{:x}", target as u64)
    };

    Ok((
        mnemonic,
        vec![reg_operand(rt_val), Operand::Text {
            value: target_text,
        }],
    ))
}

// ---------------------------------------------------------------------------
// Load/store pair (no-allocate)
// ---------------------------------------------------------------------------

fn decode_load_store_pair_no_allocate(word: u32) -> DecodeResult {
    let opc = bits(word, 31, 30);
    let l = bit(word, 22);
    let imm7 = ((word >> 15) & 0x7F) as i64;
    let rt_val = rt(word);
    let rt2_val = bits(word, 14, 10) as u8;
    let rn_val = rn(word);

    // opc=0b00: 32-bit (W), opc=0b01: LDPSW/STGP (not stage 2), opc=0b10: 64-bit (X), opc=0b11: 128-bit
    let (_is_32bit, scale) = match opc {
        0b00 => (true, 2),
        0b10 => (false, 3),
        0b01 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "STGP/LDPSW no-allocate not in stage 2",
            ));
        }
        0b11 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "128-bit loads/stores not in stage 2",
            ));
        }
        _ => unreachable!(),
    };

    let imm = if (imm7 & 0x40) != 0 {
        (imm7 | !0x7F) << scale
    } else {
        imm7 << scale
    };

    let mnemonic = if l == 1 { Mnemonic::Ldnp } else { Mnemonic::Stnp };

    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            reg_operand(rt2_val),
            addr_text(rn_val, imm, false, false, None, None),
        ],
    ))
}

// ---------------------------------------------------------------------------
// Load/store pair (post-index, signed offset, pre-index)
// ---------------------------------------------------------------------------

fn decode_load_store_pair(word: u32) -> DecodeResult {
    let opc = bits(word, 31, 30);
    let l = bit(word, 22);
    let imm7 = ((word >> 15) & 0x7F) as i64;
    let rt_val = rt(word);
    let rt2_val = bits(word, 14, 10) as u8;
    let rn_val = rn(word);
    let index_mode = bits(word, 24, 23); // 01=post-index, 10=signed offset, 11=pre-index

    // opc=0b00: 32-bit (W), opc=0b01: LDPSW/STGP, opc=0b10: 64-bit (X), opc=0b11: 128-bit
    let (scale, mnemonic) = match (opc, l) {
        (0b00, 0) => (2, Mnemonic::Stp),
        (0b00, 1) => (2, Mnemonic::Ldp),
        (0b01, 0) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "STGP not in stage 2",
            ));
        }
        (0b01, 1) => (2, Mnemonic::Ldpsw),
        (0b10, 0) => (3, Mnemonic::Stp),
        (0b10, 1) => (3, Mnemonic::Ldp),
        (0b11, _) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "128-bit loads/stores not in stage 2",
            ));
        }
        _ => unreachable!(),
    };

    let imm = if (imm7 & 0x40) != 0 {
        (imm7 | !0x7F) << scale
    } else {
        imm7 << scale
    };

    let is_post_index = index_mode == 0b01;
    let is_pre_index = index_mode == 0b11;

    if is_post_index {
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                reg_operand(rt2_val),
                addr_text(rn_val, 0, false, false, None, None),
                Operand::Immediate { value: imm },
            ],
        ))
    } else if is_pre_index {
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                reg_operand(rt2_val),
                addr_text(rn_val, imm, true, false, None, None),
            ],
        ))
    } else {
        // Signed offset
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                reg_operand(rt2_val),
                addr_text(rn_val, imm, false, false, None, None),
            ],
        ))
    }
}

// ---------------------------------------------------------------------------
// Load/store register (immediate: unscaled, post-indexed, pre-indexed)
// ---------------------------------------------------------------------------

fn decode_load_store_register_immediate(word: u32) -> DecodeResult {
    let size = bits(word, 31, 30);
    let v = v_bit(word);
    let opc = bits(word, 23, 22);
    let imm9 = ((word >> 12) & 0x1FF) as i64;
    let imm = if (imm9 & 0x100) != 0 {
        imm9 | !0x1FF
    } else {
        imm9
    };
    let rn_val = rn(word);
    let rt_val = rt(word);
    let index_mode = bits(word, 11, 10); // 00=unscaled, 01=post, 10=unprivileged, 11=pre

    if v == 1 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "SIMD/FP load/store immediate not in stage 2",
        ));
    }

    // Skip unprivileged (LDTR/STTR) for stage 2
    if index_mode == 0b10 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "unprivileged load/store not in stage 2",
        ));
    }

    let is_pre_index = index_mode == 0b11;
    let is_post_index = index_mode == 0b01;
    let is_unscaled = index_mode == 0b00;

    let mnemonic = if is_unscaled {
        decode_ls_unscaled_mnemonic(size, opc)?
    } else {
        decode_ls_mnemonic(size, opc)?
    };

    if is_post_index {
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                addr_text(rn_val, 0, false, false, None, None),
                Operand::Immediate { value: imm },
            ],
        ))
    } else if is_pre_index {
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                addr_text(rn_val, imm, true, false, None, None),
            ],
        ))
    } else {
        // Unscaled immediate (LDUR/STUR)
        // Capstone renders LDUR even when offset is 0, but for LDR/STR with
        // unsigned immediate and offset 0, it renders as [xn] without offset.
        // For unscaled, Capstone always uses LDUR/STUR.
        Ok((
            mnemonic,
            vec![
                reg_operand(rt_val),
                addr_text(rn_val, imm, false, false, None, None),
            ],
        ))
    }
}

// ---------------------------------------------------------------------------
// Load/store register (unsigned immediate)
// ---------------------------------------------------------------------------

fn decode_load_store_register_unsigned_imm(word: u32) -> DecodeResult {
    let size = bits(word, 31, 30);
    let v = v_bit(word);
    let opc = bits(word, 23, 22);
    let imm12 = ((word >> 10) & 0xFFF) as i64;
    let rn_val = rn(word);
    let rt_val = rt(word);

    if v == 1 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "SIMD/FP load/store unsigned immediate not in stage 2",
        ));
    }

    let mnemonic = decode_ls_mnemonic(size, opc)?;
    let scale = match size {
        0b00 => 0, // byte
        0b01 => 1, // halfword
        0b10 => 2, // word
        0b11 => 3, // doubleword
        _ => unreachable!(),
    };
    let imm = imm12 << scale;

    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            addr_text(rn_val, imm, false, false, None, None),
        ],
    ))
}

// ---------------------------------------------------------------------------
// Load/store register (register offset)
// ---------------------------------------------------------------------------

fn decode_load_store_register_offset(word: u32) -> DecodeResult {
    let size = bits(word, 31, 30);
    let v = v_bit(word);
    let opc = bits(word, 23, 22);
    let rm_val = rm(word);
    let option = bits(word, 15, 13);
    let s = bit(word, 12);
    let rn_val = rn(word);
    let rt_val = rt(word);

    if v == 1 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "SIMD/FP load/store register offset not in stage 2",
        ));
    }

    let mnemonic = decode_ls_mnemonic(size, opc)?;

    // Determine extend/shift based on option and S
    let extend = decode_reg_offset_extend(option, s, size);

    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            addr_text(rn_val, 0, false, false, Some(rm_val), extend.as_deref()),
        ],
    ))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Decode the mnemonic for load/store register instructions based on size and opc.
fn decode_ls_mnemonic(size: u32, opc: u32) -> Result<Mnemonic, DisasmError> {
    match (size, opc) {
        // Stores
        (0b00, 0b00) => Ok(Mnemonic::Strb),
        (0b01, 0b00) => Ok(Mnemonic::Strh),
        (0b10, 0b00) => Ok(Mnemonic::Str),
        (0b11, 0b00) => Ok(Mnemonic::Str),
        // Loads
        (0b00, 0b01) => Ok(Mnemonic::Ldrb),
        (0b01, 0b01) => Ok(Mnemonic::Ldrh),
        (0b10, 0b01) => Ok(Mnemonic::Ldr),
        (0b11, 0b01) => Ok(Mnemonic::Ldr),
        // Load signed (32-bit result for byte/half)
        (0b00, 0b10) => Ok(Mnemonic::Ldrsb),
        (0b01, 0b10) => Ok(Mnemonic::Ldrsh),
        (0b10, 0b10) => Ok(Mnemonic::Ldrsw),
        (0b11, 0b10) => Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved load/store encoding",
        )),
        // Load signed (64-bit result for byte/half)
        (0b00, 0b11) => Ok(Mnemonic::Ldrsb),
        (0b01, 0b11) => Ok(Mnemonic::Ldrsh),
        (0b10, 0b11) | (0b11, 0b11) => Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved load/store encoding",
        )),
        _ => unreachable!(),
    }
}

/// Decode the mnemonic for unscaled load/store register instructions (LDUR/STUR).
fn decode_ls_unscaled_mnemonic(size: u32, opc: u32) -> Result<Mnemonic, DisasmError> {
    match (size, opc) {
        // Stores
        (0b00, 0b00) => Ok(Mnemonic::Sturb),
        (0b01, 0b00) => Ok(Mnemonic::Sturh),
        (0b10, 0b00) => Ok(Mnemonic::Stur),
        (0b11, 0b00) => Ok(Mnemonic::Stur),
        // Loads
        (0b00, 0b01) => Ok(Mnemonic::Ldurb),
        (0b01, 0b01) => Ok(Mnemonic::Ldurh),
        (0b10, 0b01) => Ok(Mnemonic::Ldur),
        (0b11, 0b01) => Ok(Mnemonic::Ldur),
        // Load signed (32-bit result for byte/half)
        (0b00, 0b10) => Ok(Mnemonic::Ldursb),
        (0b01, 0b10) => Ok(Mnemonic::Ldursh),
        (0b10, 0b10) => Ok(Mnemonic::Ldursw),
        (0b11, 0b10) => Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved unscaled load/store encoding",
        )),
        // Load signed (64-bit result for byte/half)
        (0b00, 0b11) => Ok(Mnemonic::Ldursb),
        (0b01, 0b11) => Ok(Mnemonic::Ldursh),
        (0b10, 0b11) | (0b11, 0b11) => Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved unscaled load/store encoding",
        )),
        _ => unreachable!(),
    }
}

/// Decode register offset extend/shift string.
fn decode_reg_offset_extend(option: u32, s: u8, size: u32) -> Option<String> {
    // option[1:0] determines extend type:
    // 00: UXTB/UXTH/UXTW/UXTX (but for 64-bit it's UXTX always)
    // 10: SXTB/SXTH/SXTW/SXTX
    // For offset mode, option[2] determines whether it's shifted (S=1)

    let extend_type = match option & 0b11 {
        0b00 => {
            // UXTX for 64-bit register offset, UXTW for 32-bit
            if option == 0b011 {
                "uxtx"
            } else if option == 0b001 {
                "uxtw"
            } else {
                "uxtb"
            }
        }
        0b10 => {
            // SXTX for 64-bit register offset, SXTW for 32-bit
            if option == 0b111 {
                "sxtx"
            } else if option == 0b101 {
                "sxtw"
            } else {
                "sxtb"
            }
        }
        0b01 => "uxtw",
        0b11 => "sxtw",
        _ => unreachable!(),
    };

    // S=1: shift amount = log2(size) for LSL/SXTX/UXTX
    // For non-shifted, no shift amount shown
    if s == 1 {
        let shift = match size {
            0b00 => 0,
            0b01 => 1,
            0b10 => 2,
            0b11 => 3,
            _ => unreachable!(),
        };
        if shift > 0 {
            Some(format!("{extend_type} #{shift}"))
        } else {
            Some(extend_type.to_string())
        }
    } else {
        // For option=011 (UXTX) or 111 (SXTX) with S=0, Capstone doesn't show extend
        if (option == 0b011 || option == 0b111) && s == 0 {
            None
        } else {
            Some(extend_type.to_string())
        }
    }
}

/// Format an immediate value in Capstone style for memory operands.
/// -9..9 (excluding 0 in offset mode): decimal
/// Others: hex with 0x prefix
fn fmt_imm_capstone(value: i64) -> String {
    if value == 0 {
        "#0".to_string()
    } else if (1..10).contains(&value) || (-9..=-1).contains(&value) {
        format!("#{value}")
    } else if value > 0 {
        format!("#0x{value:x}")
    } else {
        format!("#-0x{}", (-value) as u64)
    }
}

/// Build an address text operand.
fn addr_text(
    base: u8,
    displacement: i64,
    pre_index: bool,
    _is_32bit: bool,
    reg_offset: Option<u8>,
    extend: Option<&str>,
) -> Operand {
    let base_name = gpr_name(base, false, RegContext::LoadStore);

    let text = if let Some(reg_off) = reg_offset {
        let offset_name = gpr_name(reg_off, false, RegContext::DataProc);
        if let Some(ext) = extend {
            format!("[{base_name}, {offset_name}, {ext}]")
        } else {
            format!("[{base_name}, {offset_name}]")
        }
    } else if pre_index {
        // Capstone always renders the displacement for pre-indexed, even when 0.
        format!("[{base_name}, {}]!", fmt_imm_capstone(displacement))
    } else if displacement == 0 {
        format!("[{base_name}]")
    } else {
        format!("[{base_name}, {}]", fmt_imm_capstone(displacement))
    };

    Operand::Text { value: text }
}
