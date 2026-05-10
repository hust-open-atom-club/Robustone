//! AArch64 Branches, Exception Generating, and System instructions.

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::operands;
use crate::shared::registers::reg_operand;
use crate::types::*;
use robustone_core::ir::Operand;
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

pub fn decode_branch_system(word: u32, addr: u64) -> DecodeResult {
    // System instructions: 1101_0101_00xx_xxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0xFFC00000) == 0xD5000000 {
        return decode_system(word);
    }

    // Exception generating: 1101_0100_00xx_xxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0xFFC00000) == 0xD4000000 {
        return decode_exception_generating(word);
    }

    // Unconditional branch immediate: x_00101_xxxxxxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0x7C000000) == 0x14000000 {
        return decode_unconditional_branch_imm(word, addr);
    }

    // Conditional branch: x_01010_xxxxxxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0x7E000000) == 0x54000000 {
        return decode_conditional_branch(word, addr);
    }

    // Compare & branch: x_011010_xxxxxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0x7E000000) == 0x34000000 {
        return decode_compare_branch(word, addr);
    }

    // Test & branch: x_011011_xxxxxxx_xxxx_xxxx_xxxx_xxxx
    if (word & 0x7E000000) == 0x36000000 {
        return decode_test_branch(word, addr);
    }

    // Unconditional branch register: 1101_0110_0xxxxxxxx_xxxx_xxxx_xxxx
    if (word & 0xFF800000) == 0xD6000000 {
        return decode_unconditional_branch_reg(word);
    }

    Err(DisasmError::decode_failure(
        DecodeErrorKind::InvalidEncoding,
        Some("aarch64".to_string()),
        "unrecognized branch/system encoding",
    ))
}

/// Conditional branch: B.cond.
fn decode_conditional_branch(word: u32, addr: u64) -> DecodeResult {
    let cond_val = cond(word);
    let imm = decode_bcond_imm(word);
    let target = (addr as i64).wrapping_add(imm) as u64;
    let cond = ConditionCode::from_bits(cond_val).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("aarch64".to_string()),
            "invalid condition code",
        )
    })?;

    Ok((
        Mnemonic::BCond,
        vec![
            Operand::Text {
                value: cond.as_str_capstone().to_string(),
            },
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

    Ok((mnemonic, vec![Operand::Immediate { value: imm16 }]))
}

/// System instructions: NOP, YIELD, WFE, WFI, SEV, SEVL, ISB, DSB, DMB, MSR, MRS.
fn decode_system(word: u32) -> DecodeResult {
    let l = bit(word, 21);

    let crn = bits(word, 15, 12);
    let crm = bits(word, 11, 8);

    let rt = bits(word, 4, 0);

    if !l {
        // System instructions with CRn determine the category.
        match crn {
            0b0010 => decode_hints(crm),
            0b0011 => decode_barriers(word),
            0b0100 => {
                // MSR (system register move) — stage 1 skip
                Err(DisasmError::decode_failure(
                    DecodeErrorKind::UnimplementedInstruction,
                    Some("aarch64".to_string()),
                    "MSR system register not in stage 1",
                ))
            }
            _ => Err(DisasmError::decode_failure(
                DecodeErrorKind::UnimplementedInstruction,
                Some("aarch64".to_string()),
                "system instruction not in stage 1",
            )),
        }
    } else {
        // MRS — only valid when Rt != 31
        if rt == 31 {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "reserved system encoding",
            ));
        }
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

fn decode_barriers(word: u32) -> DecodeResult {
    // Capstone matches barriers by the full bits 7:0 value.
    // These all require Rt=31 (bits 4:0 = 0b11111).
    let bottom_byte = word & 0xFF;
    let crm = bits(word, 11, 8);

    let (mnemonic, domain) = match bottom_byte {
        0x9F => {
            let domain = dsb_domain(crm);
            (Mnemonic::Dsb, domain)
        }
        0xBF => {
            let domain = dmb_domain(crm);
            (Mnemonic::Dmb, domain)
        }
        0xDF => {
            let domain = isb_domain(crm);
            (Mnemonic::Isb, domain)
        }
        _ => {
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidEncoding,
                Some("aarch64".to_string()),
                "unrecognized barrier encoding",
            ));
        }
    };

    if domain.is_empty() {
        Ok((mnemonic, vec![]))
    } else {
        Ok((mnemonic, vec![Operand::Text {
            value: domain.to_string(),
        }]))
    }
}

fn dsb_domain(crm: u32) -> &'static str {
    match crm {
        0x1 => "oshld",
        0x2 => "oshst",
        0x3 => "osh",
        0x5 => "nshld",
        0x6 => "nshst",
        0x7 => "nsh",
        0x8 => "#8",
        0x9 => "ishld",
        0xA => "ishst",
        0xB => "ish",
        0xD => "ld",
        0xE => "st",
        0xF => "sy",
        _ => "",
    }
}

fn dmb_domain(crm: u32) -> &'static str {
    match crm {
        0x0 => "#0",
        0x1 => "oshld",
        0x2 => "oshst",
        0x3 => "osh",
        0x4 => "#4",
        0x5 => "nshld",
        0x6 => "nshst",
        0x7 => "nsh",
        0x8 => "#8",
        0x9 => "ishld",
        0xA => "ishst",
        0xB => "ish",
        0xC => "#0xc",
        0xD => "ld",
        0xE => "st",
        0xF => "sy",
        _ => "",
    }
}

fn isb_domain(crm: u32) -> &'static str {
    match crm {
        0x0 => "#0",
        _ => "",
    }
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
    let is_bl = bit(word, 31);
    let imm = decode_b_imm(word);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_bl { Mnemonic::Bl } else { Mnemonic::B };
    Ok((mnemonic, vec![operands::label(target)]))
}

/// Compare & branch: CBZ, CBNZ.
fn decode_compare_branch(word: u32, addr: u64) -> DecodeResult {

    let is_cbnz = bit(word, 24);
    let imm = decode_cbz_imm(word);
    let rt_val = rt(word);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_cbnz {
        Mnemonic::Cbnz
    } else {
        Mnemonic::Cbz
    };
    Ok((mnemonic, vec![reg_operand(rt_val), operands::label(target)]))
}

/// Test & branch: TBZ, TBNZ.
fn decode_test_branch(word: u32, addr: u64) -> DecodeResult {
    let is_tbnz = bit(word, 24);
    let imm = decode_cbz_imm(word);
    let rt_val = rt(word);
    let b40 = bits(word, 23, 19) as u8;
    let b5 = bit(word, 31);
    let bit_pos = b40 | (u8::from(b5) << 5);
    let target = (addr as i64).wrapping_add(imm) as u64;

    let mnemonic = if is_tbnz {
        Mnemonic::Tbnz
    } else {
        Mnemonic::Tbz
    };
    Ok((
        mnemonic,
        vec![
            reg_operand(rt_val),
            Operand::Immediate {
                value: bit_pos as i64,
            },
            operands::label(target),
        ],
    ))
}
