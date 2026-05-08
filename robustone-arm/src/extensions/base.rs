//! AArch64 Data Processing instructions.
//!
//! Covers:
//! - Data Processing — Immediate (op0 = 0x0, 0x1)
//! - Data Processing — Register (op0 = 0x4, 0x8, 0x9)

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::operands;
use crate::shared::registers::reg_operand;
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

// ---------------------------------------------------------------------------
// Data Processing — Immediate
// ---------------------------------------------------------------------------

pub fn decode_data_proc_imm(word: u32, addr: u64) -> DecodeResult {
    match op1(word) {
        0b00 => decode_pc_rel_addressing(word, addr),
        0b01 => decode_add_sub_imm(word),
        0b10 => decode_logical_imm(word),
        0b11 => decode_move_wide(word),
        _ => unreachable!(),
    }
}

/// PC-relative addressing: ADR, ADRP.
fn decode_pc_rel_addressing(word: u32, addr: u64) -> DecodeResult {
    let rd_val = rd(word);
    let is_adrp = bit(word, 31) == 1;
    let imm = if is_adrp {
        decode_adrp_imm(word)
    } else {
        decode_adr_imm(word)
    };
    let target = (addr as i64).wrapping_add(imm);
    let mnemonic = if is_adrp { Mnemonic::Adrp } else { Mnemonic::Adr };
    Ok((
        mnemonic,
        vec![
            reg_operand(rd_val),
            operands::label(target as u64),
        ],
    ))
}

/// Add/subtract (immediate): ADD, ADDS, SUB, SUBS, CMP, CMN.
fn decode_add_sub_imm(word: u32) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    let is_sub = bit(word, 30) == 1;
    let set_flags = s_flag(word) == 1;
    let (imm, _shift) = decode_imm12(word);
    let rd_val = rd(word);
    let rn_val = rn(word);

    // CMP = SUBS (with XZR as destination)
    // CMN = ADDS (with XZR as destination)
    let mnemonic = if rd_val == 31 && set_flags {
        if is_sub { Mnemonic::Cmp } else { Mnemonic::Cmn }
    } else {
        match (is_sub, set_flags) {
            (false, false) => Mnemonic::Add,
            (false, true) => Mnemonic::Adds,
            (true, false) => Mnemonic::Sub,
            (true, true) => Mnemonic::Subs,
        }
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        Operand::Immediate { value: imm },
    ];

    // For CMP/CMN, omit the destination (XZR)
    if mnemonic == Mnemonic::Cmp || mnemonic == Mnemonic::Cmn {
        ops.remove(0);
    }

    Ok((mnemonic, ops))
}

/// Logical (immediate): AND, ORR, EOR, ANDS, TST.
fn decode_logical_imm(word: u32) -> DecodeResult {
    let is_32bit = sf(word) == 0;
    let opc_val = opc(word);
    let n = n_bit(word);
    let immr_val = immr(word);
    let imms_val = imms(word);
    let rd_val = rd(word);
    let rn_val = rn(word);

    let _bitmask = decode_bitmask_imm(n, immr_val, imms_val, !is_32bit)
        .ok_or_else(|| DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved bitmask immediate encoding",
        ))?;

    let mnemonic = match opc_val {
        0b00 => {
            if rd_val == 31 {
                Mnemonic::Tst
            } else {
                Mnemonic::And
            }
        }
        0b01 => Mnemonic::Orr,
        0b10 => Mnemonic::Eor,
        0b11 => Mnemonic::Ands,
        _ => unreachable!(),
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        // TODO: render bitmask as hex immediate
        Operand::Immediate { value: _bitmask as i64 },
    ];

    if mnemonic == Mnemonic::Tst {
        ops.remove(0); // Omit XZR destination
    }

    Ok((mnemonic, ops))
}

/// Move wide (immediate): MOVZ, MOVN, MOVK.
fn decode_move_wide(word: u32) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    let opc_val = opc(word);
    let (imm, shift) = decode_imm16_hw(word);
    let rd_val = rd(word);

    let mnemonic = match opc_val {
        0b00 => Mnemonic::Movn,
        0b10 => Mnemonic::Movz,
        0b11 => Mnemonic::Movk,
        0b01 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "reserved move wide opc=01",
            ));
        }
        _ => unreachable!(),
    };

    let mut ops = vec![
        reg_operand(rd_val),
        Operand::Immediate { value: imm },
    ];

    if shift > 0 {
        ops.push(Operand::Text {
            value: format!("lsl #{shift}"),
        });
    }

    Ok((mnemonic, ops))
}

// ---------------------------------------------------------------------------
// Data Processing — Register
// ---------------------------------------------------------------------------

pub fn decode_data_proc_reg(word: u32, _addr: u64) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    match op1(word) {
        0b00 => decode_logical_shifted_reg(word),
        0b01 => decode_add_sub_shifted_reg(word),
        0b10 => {
            if bit(word, 30) == 0 {
                decode_add_sub_extended_reg(word)
            } else {
                decode_add_sub_with_carry(word)
            }
        }
        0b11 => decode_conditional_select(word),
        _ => unreachable!(),
    }
}

/// Logical (shifted register): AND, BIC, ORR, ORN, EOR, EON, ANDS, BICS.
fn decode_logical_shifted_reg(word: u32) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    let opc_val = opc(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let shift_val = shift(word);
    let imm6 = bits(word, 15, 10) as u8;
    let n_bit_val = bit(word, 21);

    let mnemonic = match (opc_val, n_bit_val) {
        (0b00, 0) => Mnemonic::And,
        (0b00, 1) => {
            // BIC — Stage 1 skip, return unimplemented
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "BIC not in stage 1",
            ));
        }
        (0b01, 0) => Mnemonic::Orr,
        (0b01, 1) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "ORN not in stage 1",
            ));
        }
        (0b10, 0) => Mnemonic::Eor,
        (0b10, 1) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "EON not in stage 1",
            ));
        }
        (0b11, 0) => Mnemonic::Ands,
        (0b11, 1) => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "BICS not in stage 1",
            ));
        }
        _ => unreachable!(),
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        reg_operand(rm_val),
    ];

    if imm6 != 0 {
        let shift_type = ShiftType::from_bits(shift_val)
            .map(|s| s.as_str())
            .unwrap_or("lsl");
        ops.push(Operand::Text {
            value: format!("{shift_type} #{imm6}"),
        });
    }

    Ok((mnemonic, ops))
}

/// Add/subtract (shifted register): ADD, ADDS, SUB, SUBS.
fn decode_add_sub_shifted_reg(word: u32) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    let is_sub = bit(word, 30) == 1;
    let set_flags = s_flag(word) == 1;
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let shift_val = shift(word);
    let imm6 = bits(word, 15, 10) as u8;

    let mnemonic = match (is_sub, set_flags) {
        (false, false) => Mnemonic::Add,
        (false, true) => Mnemonic::Adds,
        (true, false) => Mnemonic::Sub,
        (true, true) => Mnemonic::Subs,
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        reg_operand(rm_val),
    ];

    if imm6 != 0 {
        let shift_type = ShiftType::from_bits(shift_val)
            .map(|s| s.as_str())
            .unwrap_or("lsl");
        ops.push(Operand::Text {
            value: format!("{shift_type} #{imm6}"),
        });
    }

    Ok((mnemonic, ops))
}

/// Add/subtract (extended register): ADD, ADDS, SUB, SUBS.
fn decode_add_sub_extended_reg(word: u32) -> DecodeResult {
    let is_sub = bit(word, 30) == 1;
    let set_flags = s_flag(word) == 1;
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let option = bits(word, 15, 13) as u8;
    let imm3 = bits(word, 12, 10) as u8;

    let mnemonic = match (is_sub, set_flags) {
        (false, false) => Mnemonic::Add,
        (false, true) => Mnemonic::Adds,
        (true, false) => Mnemonic::Sub,
        (true, true) => Mnemonic::Subs,
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        reg_operand(rm_val),
    ];

    if let Some(extend) = ExtendType::from_bits(option) {
        let extend_str = extend.as_str();
        if imm3 == 0 {
            ops.push(Operand::Text {
                value: extend_str.to_string(),
            });
        } else {
            ops.push(Operand::Text {
                value: format!("{extend_str} #{imm3}"),
            });
        }
    }

    Ok((mnemonic, ops))
}

/// Add/subtract (with carry): ADC, ADCS, SBC, SBCS.
fn decode_add_sub_with_carry(_word: u32) -> DecodeResult {
    // Stage 1: return unimplemented
    Err(DisasmError::decode_failure(
        DecodeErrorKind::UnimplementedInstruction,
        Some("aarch64".to_string()),
        "add/sub with carry not in stage 1",
    ))
}

/// Conditional select: CSEL, CSINC, CSINV, CSNEG, CSET, CSETM, CINC, CINV, CNEG.
fn decode_conditional_select(word: u32) -> DecodeResult {
    let _is_32bit = sf(word) == 0;
    let op = bit(word, 30);
    let s = s_flag(word);
    let rd_val = rd(word);
    let rn_val = rn(word);
    let rm_val = rm(word);
    let cond_val = cond(word);
    let cond = ConditionCode::from_bits(cond_val)
        .ok_or_else(|| DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid condition code",
        ))?;

    if s == 1 {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "reserved S=1 in conditional select",
        ));
    }

    // Detect aliases
    let mnemonic = if rn_val == 31 && rm_val == 31 {
        // CSET / CSETM
        match (op, bit(word, 10)) {
            (0, 0) => Mnemonic::Cset,
            (0, 1) => Mnemonic::Csetm,
            (1, 0) => Mnemonic::Cinc, // Actually CSINC with ZR/ZR
            (1, 1) => Mnemonic::Cinv,
            _ => unreachable!(),
        }
    } else if rm_val == 31 {
        // CINC / CINV / CNEG
        match (op, bit(word, 10)) {
            (0, 0) => Mnemonic::Csel,
            (0, 1) => Mnemonic::Csinv,
            (1, 0) => Mnemonic::Csinc,
            (1, 1) => Mnemonic::Csneg,
            _ => unreachable!(),
        }
    } else {
        match (op, bit(word, 10)) {
            (0, 0) => Mnemonic::Csel,
            (0, 1) => Mnemonic::Csinv,
            (1, 0) => Mnemonic::Csinc,
            (1, 1) => Mnemonic::Csneg,
            _ => unreachable!(),
        }
    };

    let mut ops = vec![
        reg_operand(rd_val),
        reg_operand(rn_val),
        reg_operand(rm_val),
        Operand::Text { value: cond.as_str().to_string() },
    ];

    // For aliases, simplify operands
    if mnemonic == Mnemonic::Cset || mnemonic == Mnemonic::Csetm {
        ops.truncate(2); // Keep only Rd and condition
    } else if mnemonic == Mnemonic::Cinc || mnemonic == Mnemonic::Cinv || mnemonic == Mnemonic::Cneg {
        ops.remove(2); // Remove ZR Rm
    }

    Ok((mnemonic, ops))
}
