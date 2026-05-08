//! AArch64 Branches, Exception Generating, and System instructions.

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::operands;
use crate::shared::registers::reg_operand;
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

pub fn decode_branch_system(word: u32, addr: u64) -> DecodeResult {
    let op0_val = bits(word, 30, 29);
    let op1_val = bits(word, 28, 25);
    let _op2_val = bits(word, 24, 22);

    // Conditional branch, Exception generating, System
    if op0_val == 0b00 {
        match op1_val {
            0b0100 => decode_conditional_branch(word, addr),
            0b0101 => decode_exception_generating(word),
            0b0110 | 0b0111 => decode_system(word),
            _ => Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "unrecognized branch/system encoding",
            )),
        }
    } else if op0_val == 0b01 {
        // Compare & branch, Test & branch
        match op1_val {
            0b0100 | 0b0101 => decode_compare_branch(word, addr),
            0b0110 | 0b0111 => decode_test_branch(word, addr),
            _ => Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "unrecognized compare/test branch encoding",
            )),
        }
    } else if op0_val == 0b10 {
        // Unconditional branch (register)
        decode_unconditional_branch_reg(word)
    } else if op0_val == 0b11 {
        // Unconditional branch (immediate)
        decode_unconditional_branch_imm(word, addr)
    } else {
        unreachable!()
    }
}

/// Conditional branch: B.cond.
fn decode_conditional_branch(word: u32, addr: u64) -> DecodeResult {
    let cond_val = cond(word);
    let imm = decode_bcond_imm(word);
    let target = (addr as i64).wrapping_add(imm) as u64;
    let cond = ConditionCode::from_bits(cond_val)
        .ok_or_else(|| DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid condition code",
        ))?;

    Ok((
        Mnemonic::BCond,
        vec![
            Operand::Text { value: cond.as_str_capstone().to_string() },
            operands::label(target),
        ],
    ))
}

/// Exception generating: SVC, HVC, SMC, BRK, DCPS1/2/3, HLT.
fn decode_exception_generating(word: u32) -> DecodeResult {
    let opc = bits(word, 23, 21);
    let imm16 = bits(word, 20, 5) as i64;

    let mnemonic = match opc {
        0b000 => Mnemonic::Svc,
        0b001 => Mnemonic::Hvc,
        0b010 => Mnemonic::Smc,
        0b100 => Mnemonic::Brk,
        0b101..=0b111 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "DCPS/HLT not in stage 1",
            ));
        }
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "reserved exception generating encoding",
            ));
        }
    };

    Ok((
        mnemonic,
        vec![Operand::Immediate { value: imm16 }],
    ))
}

/// System instructions: NOP, YIELD, WFE, WFI, SEV, SEVL, ISB, DSB, DMB, MSR, MRS.
fn decode_system(word: u32) -> DecodeResult {
    let l = bit(word, 21);
    let op0 = bits(word, 19, 16);
    let _op1 = bits(word, 15, 12);
    let crn = bits(word, 11, 8);
    let crm = bits(word, 7, 4);
    let op2 = bits(word, 3, 1);
    let _rt = bits(word, 4, 0);

    if l == 0 {
        // MSR (system register move) or hints/barriers
        if op0 == 0b0100 {
            match crn {
                0b0010 => decode_hints(crm),
                0b0011 => decode_barriers(crm, op2),
                _ => Err(DisasmError::decode_failure(
                    DecodeErrorKind::UnimplementedInstruction,
                    Some("aarch64".to_string()),
                    "MSR system register not in stage 1",
                )),
            }
        } else {
            Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "system instruction not in stage 1",
            ))
        }
    } else {
        // MRS
        Err(DisasmError::decode_failure(
            DecodeErrorKind::UnimplementedInstruction,
            Some("aarch64".to_string()),
            "MRS not in stage 1",
        ))
    }
}

fn decode_hints(crm: u32) -> DecodeResult {
    let mnemonic = match crm {
        0b0000 => Mnemonic::Nop,
        0b0001 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "YIELD not in stage 1",
            ));
        }
        0b0010 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "WFE not in stage 1",
            ));
        }
        0b0011 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "WFI not in stage 1",
            ));
        }
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "advanced hints not in stage 1",
            ));
        }
    };
    Ok((mnemonic, vec![]))
}

fn decode_barriers(crm: u32, op2: u32) -> DecodeResult {
    let mnemonic = match (crm, op2) {
        (0b0010, 0b001) => {
            // CLREX — not in Mnemonic enum for stage 1
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "CLREX not in stage 1",
            ));
        }
        (_, 0b011) => Mnemonic::Dsb,
        (_, 0b101) => Mnemonic::Dmb,
        (_, 0b110) => Mnemonic::Isb,
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "unrecognized barrier encoding",
            ));
        }
    };
    Ok((mnemonic, vec![]))
}

/// Unconditional branch (register): BR, BLR, RET.
fn decode_unconditional_branch_reg(word: u32) -> DecodeResult {
    let op = bits(word, 22, 21);
    let rn_val = rn(word);

    let mnemonic = match op {
        0b00 => Mnemonic::Br,
        0b01 => Mnemonic::Blr,
        0b10 => Mnemonic::Ret,
        0b11 => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "ERET/DRPS not in stage 1",
            ));
        }
        _ => unreachable!(),
    };

    if mnemonic == Mnemonic::Ret && rn_val == 30 {
        Ok((mnemonic, vec![]))
    } else {
        Ok((mnemonic, vec![reg_operand(rn_val)]))
    }
}

/// Unconditional branch (immediate): B, BL.
fn decode_unconditional_branch_imm(word: u32, addr: u64) -> DecodeResult {
    let is_bl = bit(word, 31) == 1;
    let imm = decode_b_imm(word);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_bl { Mnemonic::Bl } else { Mnemonic::B };
    Ok((mnemonic, vec![operands::label(target)]))
}

/// Compare & branch: CBZ, CBNZ.
fn decode_compare_branch(word: u32, addr: u64) -> DecodeResult {
    let _is_32bit = bit(word, 31) == 0;
    let is_cbnz = bit(word, 24) == 1;
    let imm = decode_cbz_imm(word);
    let rt_val = rt(word);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_cbnz { Mnemonic::Cbnz } else { Mnemonic::Cbz };
    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            operands::label(target),
        ],
    ))
}

/// Test & branch: TBZ, TBNZ.
fn decode_test_branch(word: u32, addr: u64) -> DecodeResult {
    let is_tbnz = bit(word, 24) == 1;
    let imm = decode_cbz_imm(word);
    let rt_val = rt(word);
    let b40 = bits(word, 23, 19) as u8;
    let b5 = bit(word, 31);
    let bit_pos = b40 | (b5 << 5);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_tbnz { Mnemonic::Tbnz } else { Mnemonic::Tbz };
    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            Operand::Immediate { value: bit_pos as i64 },
            operands::label(target),
        ],
    ))
}
