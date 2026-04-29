//! LoongArch instruction patterns derived from Capstone decoder tables.
//!
//! This file is auto-generated from Capstone's spec-driven decoder tree.
//! It replaces the exact-word match with proper mask/value patterns.

#![allow(clippy::all)]
#![allow(unused_mut, unused_variables, dead_code)]

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// A single LoongArch instruction pattern.
pub struct LoongArchPattern {
    pub mask: u32,
    pub value: u32,
    pub mnemonic: &'static str,
    pub layout: u16,
}

fn extract_case_0(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x3) as u32 + 104),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_1(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x3) as u32 + 104),
    });
    ops
}

fn extract_case_2(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_3(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Immediate {
        value: ((word >> 5) & 0x7) as i64,
    });
    ops
}

fn extract_case_4(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_5(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_6(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops
}

fn extract_case_7(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_8(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: (((word >> 15) & 0x3) + 1) as i64,
    });
    ops
}

fn extract_case_9(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 15) & 0x3) as i64,
    });
    ops
}

fn extract_case_10(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 15) & 0x7) as i64,
    });
    ops
}

fn extract_case_11(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_12(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0x1F, 5),
    });
    ops
}

fn extract_case_13(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0x7FFF) as i64,
    });
    ops
}

fn extract_case_14(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_15(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_16(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0xF) as i64,
    });
    ops
}

fn extract_case_17(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0xF) as i64,
    });
    ops
}

fn extract_case_18(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_19(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_20(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_21(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_22(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_23(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_24(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_25(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 5) & 0x1F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_26(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_27(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 16) & 0x1F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_28(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 16) & 0x1F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_29(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 16) & 0x3F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_30(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 16) & 0x3F) as i64,
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_31(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_32(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_33(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_34(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_35(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_36(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_37(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_38(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_39(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_40(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 108),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_41(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 108),
    });
    ops
}

fn extract_case_42(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_43(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 96),
    });
    ops
}

fn extract_case_44(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_45(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 96),
    });
    ops
}

fn extract_case_46(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_47(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_48(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_49(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFFF) as i64,
    });
    ops
}

fn extract_case_50(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3FFF) as i64,
    });
    ops
}

fn extract_case_51(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3FFF) as i64,
    });
    ops
}

fn extract_case_52(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3FFF) as i64,
    });
    ops
}

fn extract_case_53(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0x1F) as i64,
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_54(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_55(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_56(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0x1F) as i64,
    });
    ops
}

fn extract_case_57(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 15) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_58(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 15) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_59(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 15) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_60(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 15) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_61(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x7) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_62(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x7) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops
}

fn extract_case_63(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_64(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_65(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 15) & 0x7) as u32 + 96),
    });
    ops
}

fn extract_case_66(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFFF, 16),
    });
    ops
}

fn extract_case_67(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 5) & 0xFFFFF, 20),
    });
    ops
}

fn extract_case_68(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 5) & 0xFFFFF, 20),
    });
    ops
}

fn extract_case_69(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x3FFF) << 2, 16),
    });
    ops
}

fn extract_case_70(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x3FFF) << 2, 16),
    });
    ops
}

fn extract_case_71(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_72(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_73(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_74(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFFF, 12),
    });
    ops
}

fn extract_case_75(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x1FF) << 3, 12),
    });
    ops
}

fn extract_case_76(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x3FF) << 2, 12),
    });
    ops
}

fn extract_case_77(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x7FF) << 1, 12),
    });
    ops
}

fn extract_case_78(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 3, 11),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x1) as i64,
    });
    ops
}

fn extract_case_79(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 2, 10),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x3) as i64,
    });
    ops
}

fn extract_case_80(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 1, 9),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x7) as i64,
    });
    ops
}

fn extract_case_81(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFF, 8),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0xF) as i64,
    });
    ops
}

fn extract_case_82(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x1FF) << 3, 12),
    });
    ops
}

fn extract_case_83(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x3FF) << 2, 12),
    });
    ops
}

fn extract_case_84(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0x7FF) << 1, 12),
    });
    ops
}

fn extract_case_85(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 3, 11),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x3) as i64,
    });
    ops
}

fn extract_case_86(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 2, 10),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x7) as i64,
    });
    ops
}

fn extract_case_87(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFF) << 1, 9),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0xF) as i64,
    });
    ops
}

fn extract_case_88(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0xFF, 8),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 18) & 0x1F) as i64,
    });
    ops
}

fn extract_case_89(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Immediate {
        value: ((word >> 0) & 0x1F) as i64,
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_90(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_91(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 32),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_92(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_93(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_94(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_95(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_96(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_97(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    // TODO: unknown field at 0:5
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 23),
    });
    ops
}

fn extract_case_98(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x7) as u32 + 96),
    });
    // TODO: unknown field at 0:5
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 23),
    });
    ops
}

fn extract_case_99(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    // TODO: unknown field at 0:5
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 23),
    });
    ops
}

fn extract_case_100(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 18),
    });
    ops
}

fn extract_case_101(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    // TODO: unknown field at 0:10
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 28),
    });
    ops
}

fn extract_case_102(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: sign_extend(((word >> 10) & 0xFFFF) << 2, 18),
    });
    ops
}

fn extract_case_103(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_104(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_105(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0x1F, 5),
    });
    ops
}

fn extract_case_106(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_107(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_108(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_109(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x7) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_110(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_111(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_112(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_113(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_114(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_115(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_116(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_117(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1) as i64,
    });
    ops
}

fn extract_case_118(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_119(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_120(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_121(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1) as i64,
    });
    ops
}

fn extract_case_122(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_123(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1) as i64,
    });
    ops
}

fn extract_case_124(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_125(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_126(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7F) as i64,
    });
    ops
}

fn extract_case_127(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_128(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_129(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 5) & 0x1FFF, 13),
    });
    ops
}

fn extract_case_130(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_131(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 10) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_132(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 10) & 0x1F, 5),
    });
    ops
}

fn extract_case_133(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_134(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1F) as i64,
    });
    ops
}

fn extract_case_135(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_136(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x7) as u32 + 96),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops
}

fn extract_case_137(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops
}

fn extract_case_138(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_139(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_140(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_141(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_142(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_143(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_144(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 0),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_145(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_146(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x1) as i64,
    });
    ops
}

fn extract_case_147(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7) as i64,
    });
    ops
}

fn extract_case_148(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3) as i64,
    });
    ops
}

fn extract_case_149(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xF) as i64,
    });
    ops
}

fn extract_case_150(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x3F) as i64,
    });
    ops
}

fn extract_case_151(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0x7F) as i64,
    });
    ops
}

fn extract_case_152(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_153(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 5) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: ((word >> 10) & 0xFF) as i64,
    });
    ops
}

fn extract_case_154(word: u32) -> Vec<Operand> {
    let mut ops = Vec::new();
    ops.push(Operand::Register {
        register: RegisterId::loongarch(((word >> 0) & 0x1F) as u32 + 64),
    });
    ops.push(Operand::Immediate {
        value: sign_extend((word >> 5) & 0x1FFF, 13),
    });
    ops
}

/// Static pattern table for LoongArch instructions.
pub const LOONGARCH_PATTERNS: &[LoongArchPattern] = &[
    LoongArchPattern {
        mask: 0xFFFFFC1C,
        value: 0x00000800,
        mnemonic: "movgr2scr",
        layout: 0,
    },
    LoongArchPattern {
        mask: 0xFFFFFF80,
        value: 0x00000C00,
        mnemonic: "movscr2gr",
        layout: 1,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00001000,
        mnemonic: "clo.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00001400,
        mnemonic: "clz.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00001800,
        mnemonic: "cto.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00001C00,
        mnemonic: "ctz.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00002000,
        mnemonic: "clo.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00002400,
        mnemonic: "clz.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00002800,
        mnemonic: "cto.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00002C00,
        mnemonic: "ctz.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00003000,
        mnemonic: "revb.2h",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00003400,
        mnemonic: "revb.4h",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00003800,
        mnemonic: "revb.2w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00003C00,
        mnemonic: "revb.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00004000,
        mnemonic: "revh.2w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00004400,
        mnemonic: "revh.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00004800,
        mnemonic: "bitrev.4b",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00004C00,
        mnemonic: "bitrev.8b",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00005000,
        mnemonic: "bitrev.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00005400,
        mnemonic: "bitrev.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00005800,
        mnemonic: "ext.w.h",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00005C00,
        mnemonic: "ext.w.b",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00006000,
        mnemonic: "rdtimel.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00006400,
        mnemonic: "rdtimeh.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00006800,
        mnemonic: "rdtime.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00006C00,
        mnemonic: "cpucfg",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFF1F,
        value: 0x00007000,
        mnemonic: "x86mttop",
        layout: 3,
    },
    LoongArchPattern {
        mask: 0xFFFFFFE0,
        value: 0x00007400,
        mnemonic: "x86mftop",
        layout: 4,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00007800,
        mnemonic: "setx86loope",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x00007C00,
        mnemonic: "setx86loopne",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008000,
        mnemonic: "x86inc.b",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008001,
        mnemonic: "x86inc.h",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008002,
        mnemonic: "x86inc.w",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008003,
        mnemonic: "x86inc.d",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008004,
        mnemonic: "x86dec.b",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008005,
        mnemonic: "x86dec.h",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008006,
        mnemonic: "x86dec.w",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFC1F,
        value: 0x00008007,
        mnemonic: "x86dec.d",
        layout: 5,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x00008008,
        mnemonic: "x86settm",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x00008028,
        mnemonic: "x86clrtm",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x00008009,
        mnemonic: "x86inctop",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x00008029,
        mnemonic: "x86dectop",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00010000,
        mnemonic: "asrtle.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00018000,
        mnemonic: "asrtgt.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x00040000,
        mnemonic: "alsl.w",
        layout: 8,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x00060000,
        mnemonic: "alsl.wu",
        layout: 8,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x00080000,
        mnemonic: "bytepick.w",
        layout: 9,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x000C0000,
        mnemonic: "bytepick.d",
        layout: 10,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00100000,
        mnemonic: "add.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00108000,
        mnemonic: "add.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00110000,
        mnemonic: "sub.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00118000,
        mnemonic: "sub.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00120000,
        mnemonic: "slt",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00128000,
        mnemonic: "sltu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00130000,
        mnemonic: "maskeqz",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00138000,
        mnemonic: "masknez",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00140000,
        mnemonic: "nor",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00148000,
        mnemonic: "and",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00150000,
        mnemonic: "or",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00158000,
        mnemonic: "xor",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00160000,
        mnemonic: "orn",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00168000,
        mnemonic: "andn",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00170000,
        mnemonic: "sll.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00178000,
        mnemonic: "srl.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00180000,
        mnemonic: "sra.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00188000,
        mnemonic: "sll.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00190000,
        mnemonic: "srl.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00198000,
        mnemonic: "sra.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001A0000,
        mnemonic: "rotr.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001A8000,
        mnemonic: "rotr.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001B0000,
        mnemonic: "rotr.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001B8000,
        mnemonic: "rotr.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001C0000,
        mnemonic: "mul.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001C8000,
        mnemonic: "mulh.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001D0000,
        mnemonic: "mulh.wu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001D8000,
        mnemonic: "mul.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001E0000,
        mnemonic: "mulh.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001E8000,
        mnemonic: "mulh.du",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001F0000,
        mnemonic: "mulw.d.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x001F8000,
        mnemonic: "mulw.d.wu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00200000,
        mnemonic: "div.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00208000,
        mnemonic: "mod.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00210000,
        mnemonic: "div.wu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00218000,
        mnemonic: "mod.wu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00220000,
        mnemonic: "div.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00228000,
        mnemonic: "mod.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00230000,
        mnemonic: "div.du",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00238000,
        mnemonic: "mod.du",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00240000,
        mnemonic: "crc.w.b.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00248000,
        mnemonic: "crc.w.h.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00250000,
        mnemonic: "crc.w.w.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00258000,
        mnemonic: "crc.w.d.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00260000,
        mnemonic: "crcc.w.b.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00268000,
        mnemonic: "crcc.w.h.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00270000,
        mnemonic: "crcc.w.w.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00278000,
        mnemonic: "crcc.w.d.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00290000,
        mnemonic: "addu12i.w",
        layout: 12,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00298000,
        mnemonic: "addu12i.d",
        layout: 12,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x002A0000,
        mnemonic: "break",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x002A8000,
        mnemonic: "dbcl",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x002B0000,
        mnemonic: "syscall",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x002B8000,
        mnemonic: "hvcl",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x002C0000,
        mnemonic: "alsl.d",
        layout: 8,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00300000,
        mnemonic: "adc.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00308000,
        mnemonic: "adc.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00310000,
        mnemonic: "adc.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00318000,
        mnemonic: "adc.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00320000,
        mnemonic: "sbc.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00328000,
        mnemonic: "sbc.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00330000,
        mnemonic: "sbc.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00338000,
        mnemonic: "sbc.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00340000,
        mnemonic: "rcr.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00348000,
        mnemonic: "rcr.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00350000,
        mnemonic: "rcr.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00358000,
        mnemonic: "rcr.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x00364000,
        mnemonic: "armmove",
        layout: 14,
    },
    LoongArchPattern {
        mask: 0xFFFFC3E0,
        value: 0x00368000,
        mnemonic: "setx86j",
        layout: 15,
    },
    LoongArchPattern {
        mask: 0xFFFFC3E0,
        value: 0x0036C000,
        mnemonic: "setarmj",
        layout: 15,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00370010,
        mnemonic: "armadd.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00378010,
        mnemonic: "armsub.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00380010,
        mnemonic: "armadc.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00388010,
        mnemonic: "armsbc.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00390010,
        mnemonic: "armand.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x00398010,
        mnemonic: "armor.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003A0010,
        mnemonic: "armxor.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003A8010,
        mnemonic: "armsll.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003B0010,
        mnemonic: "armsrl.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003B8010,
        mnemonic: "armsra.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003C0010,
        mnemonic: "armrotr.w",
        layout: 16,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003C8010,
        mnemonic: "armslli.w",
        layout: 17,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003D0010,
        mnemonic: "armsrli.w",
        layout: 17,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003D8010,
        mnemonic: "armsrai.w",
        layout: 17,
    },
    LoongArchPattern {
        mask: 0xFFFF8010,
        value: 0x003E0010,
        mnemonic: "armrotri.w",
        layout: 17,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8000,
        mnemonic: "x86mul.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8001,
        mnemonic: "x86mul.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8002,
        mnemonic: "x86mul.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8003,
        mnemonic: "x86mul.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8004,
        mnemonic: "x86mul.bu",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8005,
        mnemonic: "x86mul.hu",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8006,
        mnemonic: "x86mul.wu",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003E8007,
        mnemonic: "x86mul.du",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0000,
        mnemonic: "x86add.wu",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0001,
        mnemonic: "x86add.du",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0002,
        mnemonic: "x86sub.wu",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0003,
        mnemonic: "x86sub.du",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0004,
        mnemonic: "x86add.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0005,
        mnemonic: "x86add.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0006,
        mnemonic: "x86add.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0007,
        mnemonic: "x86add.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0008,
        mnemonic: "x86sub.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0009,
        mnemonic: "x86sub.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000A,
        mnemonic: "x86sub.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000B,
        mnemonic: "x86sub.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000C,
        mnemonic: "x86adc.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000D,
        mnemonic: "x86adc.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000E,
        mnemonic: "x86adc.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F000F,
        mnemonic: "x86adc.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0010,
        mnemonic: "x86sbc.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0011,
        mnemonic: "x86sbc.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0012,
        mnemonic: "x86sbc.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0013,
        mnemonic: "x86sbc.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0014,
        mnemonic: "x86sll.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0015,
        mnemonic: "x86sll.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0016,
        mnemonic: "x86sll.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0017,
        mnemonic: "x86sll.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0018,
        mnemonic: "x86srl.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F0019,
        mnemonic: "x86srl.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001A,
        mnemonic: "x86srl.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001B,
        mnemonic: "x86srl.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001C,
        mnemonic: "x86sra.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001D,
        mnemonic: "x86sra.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001E,
        mnemonic: "x86sra.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F001F,
        mnemonic: "x86sra.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8000,
        mnemonic: "x86rotr.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8001,
        mnemonic: "x86rotr.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8002,
        mnemonic: "x86rotr.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8003,
        mnemonic: "x86rotr.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8004,
        mnemonic: "x86rotl.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8005,
        mnemonic: "x86rotl.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8006,
        mnemonic: "x86rotl.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8007,
        mnemonic: "x86rotl.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8008,
        mnemonic: "x86rcr.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8009,
        mnemonic: "x86rcr.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800A,
        mnemonic: "x86rcr.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800B,
        mnemonic: "x86rcr.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800C,
        mnemonic: "x86rcl.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800D,
        mnemonic: "x86rcl.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800E,
        mnemonic: "x86rcl.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F800F,
        mnemonic: "x86rcl.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8010,
        mnemonic: "x86and.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8011,
        mnemonic: "x86and.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8012,
        mnemonic: "x86and.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8013,
        mnemonic: "x86and.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8014,
        mnemonic: "x86or.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8015,
        mnemonic: "x86or.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8016,
        mnemonic: "x86or.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8017,
        mnemonic: "x86or.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8018,
        mnemonic: "x86xor.b",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F8019,
        mnemonic: "x86xor.h",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F801A,
        mnemonic: "x86xor.w",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x003F801B,
        mnemonic: "x86xor.d",
        layout: 7,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x003FC01C,
        mnemonic: "armnot.w",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x003FC01D,
        mnemonic: "armmov.w",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x003FC01E,
        mnemonic: "armmov.d",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x003FC01F,
        mnemonic: "armrrx.w",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00408000,
        mnemonic: "slli.w",
        layout: 19,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x00410000,
        mnemonic: "slli.d",
        layout: 20,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00448000,
        mnemonic: "srli.w",
        layout: 19,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x00450000,
        mnemonic: "srli.d",
        layout: 20,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00488000,
        mnemonic: "srai.w",
        layout: 19,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x00490000,
        mnemonic: "srai.d",
        layout: 20,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x004C2000,
        mnemonic: "rotri.b",
        layout: 21,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x004C4000,
        mnemonic: "rotri.h",
        layout: 14,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x004C8000,
        mnemonic: "rotri.w",
        layout: 19,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x004D0000,
        mnemonic: "rotri.d",
        layout: 20,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x00502000,
        mnemonic: "rcri.b",
        layout: 21,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x00504000,
        mnemonic: "rcri.h",
        layout: 14,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x00508000,
        mnemonic: "rcri.w",
        layout: 19,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x00510000,
        mnemonic: "rcri.d",
        layout: 20,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542000,
        mnemonic: "x86slli.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544001,
        mnemonic: "x86slli.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00548002,
        mnemonic: "x86slli.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x00550003,
        mnemonic: "x86slli.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542004,
        mnemonic: "x86srli.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544005,
        mnemonic: "x86srli.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00548006,
        mnemonic: "x86srli.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x00550007,
        mnemonic: "x86srli.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542008,
        mnemonic: "x86srai.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544009,
        mnemonic: "x86srai.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x0054800A,
        mnemonic: "x86srai.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x0055000B,
        mnemonic: "x86srai.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x0054200C,
        mnemonic: "x86rotri.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x0054400D,
        mnemonic: "x86rotri.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x0054800E,
        mnemonic: "x86rotri.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x0055000F,
        mnemonic: "x86rotri.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542010,
        mnemonic: "x86rcri.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544011,
        mnemonic: "x86rcri.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00548012,
        mnemonic: "x86rcri.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x00550013,
        mnemonic: "x86rcri.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542014,
        mnemonic: "x86rotli.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544015,
        mnemonic: "x86rotli.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x00548016,
        mnemonic: "x86rotli.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x00550017,
        mnemonic: "x86rotli.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFFE01F,
        value: 0x00542018,
        mnemonic: "x86rcli.b",
        layout: 22,
    },
    LoongArchPattern {
        mask: 0xFFFFC01F,
        value: 0x00544019,
        mnemonic: "x86rcli.h",
        layout: 18,
    },
    LoongArchPattern {
        mask: 0xFFFF801F,
        value: 0x0054801A,
        mnemonic: "x86rcli.w",
        layout: 23,
    },
    LoongArchPattern {
        mask: 0xFFFF001F,
        value: 0x0055001B,
        mnemonic: "x86rcli.d",
        layout: 24,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x00580000,
        mnemonic: "x86settag",
        layout: 25,
    },
    LoongArchPattern {
        mask: 0xFFFC03E0,
        value: 0x005C0000,
        mnemonic: "x86mfflag",
        layout: 26,
    },
    LoongArchPattern {
        mask: 0xFFFC03E0,
        value: 0x005C0020,
        mnemonic: "x86mtflag",
        layout: 26,
    },
    LoongArchPattern {
        mask: 0xFFFC03E0,
        value: 0x005C0040,
        mnemonic: "armmfflag",
        layout: 26,
    },
    LoongArchPattern {
        mask: 0xFFFC03E0,
        value: 0x005C0060,
        mnemonic: "armmtflag",
        layout: 26,
    },
    LoongArchPattern {
        mask: 0xFFE08000,
        value: 0x00600000,
        mnemonic: "bstrins.w",
        layout: 27,
    },
    LoongArchPattern {
        mask: 0xFFE08000,
        value: 0x00608000,
        mnemonic: "bstrpick.w",
        layout: 28,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x00800000,
        mnemonic: "bstrins.d",
        layout: 29,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x00C00000,
        mnemonic: "bstrpick.d",
        layout: 30,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01008000,
        mnemonic: "fadd.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01010000,
        mnemonic: "fadd.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01028000,
        mnemonic: "fsub.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01030000,
        mnemonic: "fsub.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01048000,
        mnemonic: "fmul.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01050000,
        mnemonic: "fmul.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01068000,
        mnemonic: "fdiv.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01070000,
        mnemonic: "fdiv.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01088000,
        mnemonic: "fmax.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01090000,
        mnemonic: "fmax.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010A8000,
        mnemonic: "fmin.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010B0000,
        mnemonic: "fmin.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010C8000,
        mnemonic: "fmaxa.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010D0000,
        mnemonic: "fmaxa.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010E8000,
        mnemonic: "fmina.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x010F0000,
        mnemonic: "fmina.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01108000,
        mnemonic: "fscaleb.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01110000,
        mnemonic: "fscaleb.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01128000,
        mnemonic: "fcopysign.s",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01130000,
        mnemonic: "fcopysign.d",
        layout: 32,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01140400,
        mnemonic: "fabs.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01140800,
        mnemonic: "fabs.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01141400,
        mnemonic: "fneg.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01141800,
        mnemonic: "fneg.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01142400,
        mnemonic: "flogb.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01142800,
        mnemonic: "flogb.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01143400,
        mnemonic: "fclass.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01143800,
        mnemonic: "fclass.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01144400,
        mnemonic: "fsqrt.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01144800,
        mnemonic: "fsqrt.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01145400,
        mnemonic: "frecip.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01145800,
        mnemonic: "frecip.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01146400,
        mnemonic: "frsqrt.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01146800,
        mnemonic: "frsqrt.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01147400,
        mnemonic: "frecipe.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01147800,
        mnemonic: "frecipe.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01148400,
        mnemonic: "frsqrte.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01148800,
        mnemonic: "frsqrte.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01149400,
        mnemonic: "fmov.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01149800,
        mnemonic: "fmov.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114A400,
        mnemonic: "movgr2fr.w",
        layout: 35,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114A800,
        mnemonic: "movgr2fr.d",
        layout: 36,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114AC00,
        mnemonic: "movgr2frh.w",
        layout: 37,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114B400,
        mnemonic: "movfr2gr.s",
        layout: 38,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114B800,
        mnemonic: "movfr2gr.d",
        layout: 39,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114BC00,
        mnemonic: "movfrh2gr.s",
        layout: 39,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114C000,
        mnemonic: "movgr2fcsr",
        layout: 40,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114C800,
        mnemonic: "movfcsr2gr",
        layout: 41,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114D000,
        mnemonic: "movfr2cf.xs",
        layout: 42,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114D400,
        mnemonic: "movcf2fr.xs",
        layout: 43,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114D800,
        mnemonic: "movgr2cf",
        layout: 44,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114DC00,
        mnemonic: "movcf2gr",
        layout: 45,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114E000,
        mnemonic: "fcvt.ld.d",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x0114E400,
        mnemonic: "fcvt.ud.d",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x01150000,
        mnemonic: "fcvt.d.ld",
        layout: 31,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01191800,
        mnemonic: "fcvt.s.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x01192400,
        mnemonic: "fcvt.d.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A0400,
        mnemonic: "ftintrm.w.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A0800,
        mnemonic: "ftintrm.w.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A2400,
        mnemonic: "ftintrm.l.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A2800,
        mnemonic: "ftintrm.l.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A4400,
        mnemonic: "ftintrp.w.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A4800,
        mnemonic: "ftintrp.w.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A6400,
        mnemonic: "ftintrp.l.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A6800,
        mnemonic: "ftintrp.l.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A8400,
        mnemonic: "ftintrz.w.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011A8800,
        mnemonic: "ftintrz.w.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AA400,
        mnemonic: "ftintrz.l.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AA800,
        mnemonic: "ftintrz.l.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AC400,
        mnemonic: "ftintrne.w.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AC800,
        mnemonic: "ftintrne.w.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AE400,
        mnemonic: "ftintrne.l.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011AE800,
        mnemonic: "ftintrne.l.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011B0400,
        mnemonic: "ftint.w.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011B0800,
        mnemonic: "ftint.w.d",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011B2400,
        mnemonic: "ftint.l.s",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011B2800,
        mnemonic: "ftint.l.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011D1000,
        mnemonic: "ffint.s.w",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011D1800,
        mnemonic: "ffint.s.l",
        layout: 46,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011D2000,
        mnemonic: "ffint.d.w",
        layout: 47,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011D2800,
        mnemonic: "ffint.d.l",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011E4400,
        mnemonic: "frint.s",
        layout: 33,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x011E4800,
        mnemonic: "frint.d",
        layout: 34,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x02000000,
        mnemonic: "slti",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x02400000,
        mnemonic: "sltui",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x02800000,
        mnemonic: "addi.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x02C00000,
        mnemonic: "addi.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x03000000,
        mnemonic: "lu52i.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x03400000,
        mnemonic: "andi",
        layout: 49,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x03800000,
        mnemonic: "ori",
        layout: 49,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x03C00000,
        mnemonic: "xori",
        layout: 49,
    },
    LoongArchPattern {
        mask: 0xFF0003E0,
        value: 0x04000000,
        mnemonic: "csrrd",
        layout: 50,
    },
    LoongArchPattern {
        mask: 0xFF0003E0,
        value: 0x04000020,
        mnemonic: "csrwr",
        layout: 51,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x04000000,
        mnemonic: "csrxchg",
        layout: 52,
    },
    LoongArchPattern {
        mask: 0xFF0003E0,
        value: 0x05000000,
        mnemonic: "gcsrrd",
        layout: 50,
    },
    LoongArchPattern {
        mask: 0xFF0003E0,
        value: 0x05000020,
        mnemonic: "gcsrwr",
        layout: 51,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x05000000,
        mnemonic: "gcsrxchg",
        layout: 52,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x06000000,
        mnemonic: "cacop",
        layout: 53,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x06400000,
        mnemonic: "lddir",
        layout: 54,
    },
    LoongArchPattern {
        mask: 0xFFFC001F,
        value: 0x06440000,
        mnemonic: "ldpte",
        layout: 55,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06480000,
        mnemonic: "iocsrrd.b",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06480400,
        mnemonic: "iocsrrd.h",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06480800,
        mnemonic: "iocsrrd.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06480C00,
        mnemonic: "iocsrrd.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06481000,
        mnemonic: "iocsrwr.b",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06481400,
        mnemonic: "iocsrwr.h",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06481800,
        mnemonic: "iocsrwr.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x06481C00,
        mnemonic: "iocsrwr.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06482000,
        mnemonic: "tlbclr",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06482400,
        mnemonic: "tlbflush",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06482401,
        mnemonic: "gtlbflush",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06482800,
        mnemonic: "tlbsrch",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06482C00,
        mnemonic: "tlbrd",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06483000,
        mnemonic: "tlbwr",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06483400,
        mnemonic: "tlbfill",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFFFFFF,
        value: 0x06483800,
        mnemonic: "ertn",
        layout: 6,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x06488000,
        mnemonic: "idle",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x06498000,
        mnemonic: "invtlb",
        layout: 56,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08100000,
        mnemonic: "fmadd.s",
        layout: 57,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08200000,
        mnemonic: "fmadd.d",
        layout: 58,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08500000,
        mnemonic: "fmsub.s",
        layout: 57,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08600000,
        mnemonic: "fmsub.d",
        layout: 58,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08900000,
        mnemonic: "fnmadd.s",
        layout: 57,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08A00000,
        mnemonic: "fnmadd.d",
        layout: 58,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08D00000,
        mnemonic: "fnmsub.s",
        layout: 57,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x08E00000,
        mnemonic: "fnmsub.d",
        layout: 58,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09100000,
        mnemonic: "vfmadd.s",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09200000,
        mnemonic: "vfmadd.d",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09500000,
        mnemonic: "vfmsub.s",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09600000,
        mnemonic: "vfmsub.d",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09900000,
        mnemonic: "vfnmadd.s",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09A00000,
        mnemonic: "vfnmadd.d",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09D00000,
        mnemonic: "vfnmsub.s",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x09E00000,
        mnemonic: "vfnmsub.d",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0A100000,
        mnemonic: "xvfmadd.s",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0A200000,
        mnemonic: "xvfmadd.d",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0A500000,
        mnemonic: "xvfmsub.s",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0A600000,
        mnemonic: "xvfmsub.d",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0A900000,
        mnemonic: "xvfnmadd.s",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0AA00000,
        mnemonic: "xvfnmadd.d",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0AD00000,
        mnemonic: "xvfnmsub.s",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0AE00000,
        mnemonic: "xvfnmsub.d",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C100000,
        mnemonic: "fcmp.caf.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C108000,
        mnemonic: "fcmp.saf.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C110000,
        mnemonic: "fcmp.clt.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C118000,
        mnemonic: "fcmp.slt.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C120000,
        mnemonic: "fcmp.ceq.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C128000,
        mnemonic: "fcmp.seq.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C130000,
        mnemonic: "fcmp.cle.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C138000,
        mnemonic: "fcmp.sle.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C140000,
        mnemonic: "fcmp.cun.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C148000,
        mnemonic: "fcmp.sun.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C150000,
        mnemonic: "fcmp.cult.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C158000,
        mnemonic: "fcmp.sult.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C160000,
        mnemonic: "fcmp.cueq.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C168000,
        mnemonic: "fcmp.sueq.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C170000,
        mnemonic: "fcmp.cule.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C178000,
        mnemonic: "fcmp.sule.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C180000,
        mnemonic: "fcmp.cne.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C188000,
        mnemonic: "fcmp.sne.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C1A0000,
        mnemonic: "fcmp.cor.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C1A8000,
        mnemonic: "fcmp.sor.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C1C0000,
        mnemonic: "fcmp.cune.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C1C8000,
        mnemonic: "fcmp.sune.s",
        layout: 61,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C200000,
        mnemonic: "fcmp.caf.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C208000,
        mnemonic: "fcmp.saf.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C210000,
        mnemonic: "fcmp.clt.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C218000,
        mnemonic: "fcmp.slt.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C220000,
        mnemonic: "fcmp.ceq.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C228000,
        mnemonic: "fcmp.seq.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C230000,
        mnemonic: "fcmp.cle.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C238000,
        mnemonic: "fcmp.sle.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C240000,
        mnemonic: "fcmp.cun.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C248000,
        mnemonic: "fcmp.sun.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C250000,
        mnemonic: "fcmp.cult.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C258000,
        mnemonic: "fcmp.sult.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C260000,
        mnemonic: "fcmp.cueq.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C268000,
        mnemonic: "fcmp.sueq.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C270000,
        mnemonic: "fcmp.cule.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C278000,
        mnemonic: "fcmp.sule.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C280000,
        mnemonic: "fcmp.cne.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C288000,
        mnemonic: "fcmp.sne.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C2A0000,
        mnemonic: "fcmp.cor.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C2A8000,
        mnemonic: "fcmp.sor.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C2C0000,
        mnemonic: "fcmp.cune.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8018,
        value: 0x0C2C8000,
        mnemonic: "fcmp.sune.d",
        layout: 62,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C500000,
        mnemonic: "vfcmp.caf.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C508000,
        mnemonic: "vfcmp.saf.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C510000,
        mnemonic: "vfcmp.clt.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C518000,
        mnemonic: "vfcmp.slt.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C520000,
        mnemonic: "vfcmp.ceq.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C528000,
        mnemonic: "vfcmp.seq.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C530000,
        mnemonic: "vfcmp.cle.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C538000,
        mnemonic: "vfcmp.sle.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C540000,
        mnemonic: "vfcmp.cun.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C548000,
        mnemonic: "vfcmp.sun.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C550000,
        mnemonic: "vfcmp.cult.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C558000,
        mnemonic: "vfcmp.sult.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C560000,
        mnemonic: "vfcmp.cueq.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C568000,
        mnemonic: "vfcmp.sueq.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C570000,
        mnemonic: "vfcmp.cule.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C578000,
        mnemonic: "vfcmp.sule.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C580000,
        mnemonic: "vfcmp.cne.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C588000,
        mnemonic: "vfcmp.sne.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C5A0000,
        mnemonic: "vfcmp.cor.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C5A8000,
        mnemonic: "vfcmp.sor.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C5C0000,
        mnemonic: "vfcmp.cune.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C5C8000,
        mnemonic: "vfcmp.sune.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C600000,
        mnemonic: "vfcmp.caf.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C608000,
        mnemonic: "vfcmp.saf.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C610000,
        mnemonic: "vfcmp.clt.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C618000,
        mnemonic: "vfcmp.slt.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C620000,
        mnemonic: "vfcmp.ceq.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C628000,
        mnemonic: "vfcmp.seq.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C630000,
        mnemonic: "vfcmp.cle.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C638000,
        mnemonic: "vfcmp.sle.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C640000,
        mnemonic: "vfcmp.cun.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C648000,
        mnemonic: "vfcmp.sun.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C650000,
        mnemonic: "vfcmp.cult.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C658000,
        mnemonic: "vfcmp.sult.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C660000,
        mnemonic: "vfcmp.cueq.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C668000,
        mnemonic: "vfcmp.sueq.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C670000,
        mnemonic: "vfcmp.cule.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C678000,
        mnemonic: "vfcmp.sule.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C680000,
        mnemonic: "vfcmp.cne.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C688000,
        mnemonic: "vfcmp.sne.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C6A0000,
        mnemonic: "vfcmp.cor.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C6A8000,
        mnemonic: "vfcmp.sor.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C6C0000,
        mnemonic: "vfcmp.cune.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C6C8000,
        mnemonic: "vfcmp.sune.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C900000,
        mnemonic: "xvfcmp.caf.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C908000,
        mnemonic: "xvfcmp.saf.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C910000,
        mnemonic: "xvfcmp.clt.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C918000,
        mnemonic: "xvfcmp.slt.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C920000,
        mnemonic: "xvfcmp.ceq.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C928000,
        mnemonic: "xvfcmp.seq.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C930000,
        mnemonic: "xvfcmp.cle.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C938000,
        mnemonic: "xvfcmp.sle.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C940000,
        mnemonic: "xvfcmp.cun.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C948000,
        mnemonic: "xvfcmp.sun.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C950000,
        mnemonic: "xvfcmp.cult.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C958000,
        mnemonic: "xvfcmp.sult.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C960000,
        mnemonic: "xvfcmp.cueq.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C968000,
        mnemonic: "xvfcmp.sueq.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C970000,
        mnemonic: "xvfcmp.cule.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C978000,
        mnemonic: "xvfcmp.sule.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C980000,
        mnemonic: "xvfcmp.cne.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C988000,
        mnemonic: "xvfcmp.sne.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C9A0000,
        mnemonic: "xvfcmp.cor.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C9A8000,
        mnemonic: "xvfcmp.sor.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C9C0000,
        mnemonic: "xvfcmp.cune.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0C9C8000,
        mnemonic: "xvfcmp.sune.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA00000,
        mnemonic: "xvfcmp.caf.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA08000,
        mnemonic: "xvfcmp.saf.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA10000,
        mnemonic: "xvfcmp.clt.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA18000,
        mnemonic: "xvfcmp.slt.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA20000,
        mnemonic: "xvfcmp.ceq.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA28000,
        mnemonic: "xvfcmp.seq.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA30000,
        mnemonic: "xvfcmp.cle.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA38000,
        mnemonic: "xvfcmp.sle.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA40000,
        mnemonic: "xvfcmp.cun.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA48000,
        mnemonic: "xvfcmp.sun.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA50000,
        mnemonic: "xvfcmp.cult.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA58000,
        mnemonic: "xvfcmp.sult.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA60000,
        mnemonic: "xvfcmp.cueq.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA68000,
        mnemonic: "xvfcmp.sueq.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA70000,
        mnemonic: "xvfcmp.cule.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA78000,
        mnemonic: "xvfcmp.sule.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA80000,
        mnemonic: "xvfcmp.cne.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CA88000,
        mnemonic: "xvfcmp.sne.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CAA0000,
        mnemonic: "xvfcmp.cor.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CAA8000,
        mnemonic: "xvfcmp.sor.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CAC0000,
        mnemonic: "xvfcmp.cune.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x0CAC8000,
        mnemonic: "xvfcmp.sune.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x0D000000,
        mnemonic: "fsel.xs",
        layout: 65,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0D100000,
        mnemonic: "vbitsel.v",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0D200000,
        mnemonic: "xvbitsel.v",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0D500000,
        mnemonic: "vshuf.b",
        layout: 59,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x0D600000,
        mnemonic: "xvshuf.b",
        layout: 60,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x10000000,
        mnemonic: "addu16i.d",
        layout: 66,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x14000000,
        mnemonic: "lu12i.w",
        layout: 67,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x16000000,
        mnemonic: "lu32i.d",
        layout: 68,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x18000000,
        mnemonic: "pcaddi",
        layout: 67,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x1A000000,
        mnemonic: "pcalau12i",
        layout: 67,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x1C000000,
        mnemonic: "pcaddu12i",
        layout: 67,
    },
    LoongArchPattern {
        mask: 0xFE000000,
        value: 0x1E000000,
        mnemonic: "pcaddu18i",
        layout: 67,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x20000000,
        mnemonic: "ll.w",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x21000000,
        mnemonic: "sc.w",
        layout: 70,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x22000000,
        mnemonic: "ll.d",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x23000000,
        mnemonic: "sc.d",
        layout: 70,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x24000000,
        mnemonic: "ldptr.w",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x25000000,
        mnemonic: "stptr.w",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x26000000,
        mnemonic: "ldptr.d",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFF000000,
        value: 0x27000000,
        mnemonic: "stptr.d",
        layout: 69,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x28000000,
        mnemonic: "ld.b",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x28400000,
        mnemonic: "ld.h",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x28800000,
        mnemonic: "ld.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x28C00000,
        mnemonic: "ld.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x29000000,
        mnemonic: "st.b",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x29400000,
        mnemonic: "st.h",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x29800000,
        mnemonic: "st.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x29C00000,
        mnemonic: "st.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2A000000,
        mnemonic: "ld.bu",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2A400000,
        mnemonic: "ld.hu",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2A800000,
        mnemonic: "ld.wu",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2AC00000,
        mnemonic: "preld",
        layout: 53,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2B000000,
        mnemonic: "fld.s",
        layout: 71,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2B400000,
        mnemonic: "fst.s",
        layout: 71,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2B800000,
        mnemonic: "fld.d",
        layout: 72,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2BC00000,
        mnemonic: "fst.d",
        layout: 72,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2C000000,
        mnemonic: "vld",
        layout: 73,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2C400000,
        mnemonic: "vst",
        layout: 73,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2C800000,
        mnemonic: "xvld",
        layout: 74,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2CC00000,
        mnemonic: "xvst",
        layout: 74,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2E000000,
        mnemonic: "ldl.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2E400000,
        mnemonic: "ldr.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2E800000,
        mnemonic: "ldl.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2EC00000,
        mnemonic: "ldr.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2F000000,
        mnemonic: "stl.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2F400000,
        mnemonic: "str.w",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2F800000,
        mnemonic: "stl.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x2FC00000,
        mnemonic: "str.d",
        layout: 48,
    },
    LoongArchPattern {
        mask: 0xFFF80000,
        value: 0x30100000,
        mnemonic: "vldrepl.d",
        layout: 75,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x30200000,
        mnemonic: "vldrepl.w",
        layout: 76,
    },
    LoongArchPattern {
        mask: 0xFFE00000,
        value: 0x30400000,
        mnemonic: "vldrepl.h",
        layout: 77,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x30800000,
        mnemonic: "vldrepl.b",
        layout: 73,
    },
    LoongArchPattern {
        mask: 0xFFF80000,
        value: 0x31100000,
        mnemonic: "vstelm.d",
        layout: 78,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x31200000,
        mnemonic: "vstelm.w",
        layout: 79,
    },
    LoongArchPattern {
        mask: 0xFFE00000,
        value: 0x31400000,
        mnemonic: "vstelm.h",
        layout: 80,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x31800000,
        mnemonic: "vstelm.b",
        layout: 81,
    },
    LoongArchPattern {
        mask: 0xFFF80000,
        value: 0x32100000,
        mnemonic: "xvldrepl.d",
        layout: 82,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x32200000,
        mnemonic: "xvldrepl.w",
        layout: 83,
    },
    LoongArchPattern {
        mask: 0xFFE00000,
        value: 0x32400000,
        mnemonic: "xvldrepl.h",
        layout: 84,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x32800000,
        mnemonic: "xvldrepl.b",
        layout: 74,
    },
    LoongArchPattern {
        mask: 0xFFF00000,
        value: 0x33100000,
        mnemonic: "xvstelm.d",
        layout: 85,
    },
    LoongArchPattern {
        mask: 0xFFE00000,
        value: 0x33200000,
        mnemonic: "xvstelm.w",
        layout: 86,
    },
    LoongArchPattern {
        mask: 0xFFC00000,
        value: 0x33400000,
        mnemonic: "xvstelm.h",
        layout: 87,
    },
    LoongArchPattern {
        mask: 0xFF800000,
        value: 0x33800000,
        mnemonic: "xvstelm.b",
        layout: 88,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38000000,
        mnemonic: "ldx.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38040000,
        mnemonic: "ldx.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38080000,
        mnemonic: "ldx.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x380C0000,
        mnemonic: "ldx.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38100000,
        mnemonic: "stx.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38140000,
        mnemonic: "stx.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38180000,
        mnemonic: "stx.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x381C0000,
        mnemonic: "stx.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38200000,
        mnemonic: "ldx.bu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38240000,
        mnemonic: "ldx.hu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38280000,
        mnemonic: "ldx.wu",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x382C0000,
        mnemonic: "preldx",
        layout: 89,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38300000,
        mnemonic: "fldx.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38340000,
        mnemonic: "fldx.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38380000,
        mnemonic: "fstx.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x383C0000,
        mnemonic: "fstx.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38400000,
        mnemonic: "vldx",
        layout: 92,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38440000,
        mnemonic: "vstx",
        layout: 92,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38480000,
        mnemonic: "xvldx",
        layout: 93,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x384C0000,
        mnemonic: "xvstx",
        layout: 93,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38570000,
        mnemonic: "sc.q",
        layout: 94,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x38578000,
        mnemonic: "llacq.w",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x38578400,
        mnemonic: "screl.w",
        layout: 95,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x38578800,
        mnemonic: "llacq.d",
        layout: 2,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x38578C00,
        mnemonic: "screl.d",
        layout: 95,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38580000,
        mnemonic: "amcas.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38588000,
        mnemonic: "amcas.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38590000,
        mnemonic: "amcas.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38598000,
        mnemonic: "amcas.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385A0000,
        mnemonic: "amcas..db.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385A8000,
        mnemonic: "amcas..db.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385B0000,
        mnemonic: "amcas..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385B8000,
        mnemonic: "amcas..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385C0000,
        mnemonic: "amswap.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385C8000,
        mnemonic: "amswap.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385D0000,
        mnemonic: "amadd.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385D8000,
        mnemonic: "amadd.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385E0000,
        mnemonic: "amswap..db.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385E8000,
        mnemonic: "amswap..db.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385F0000,
        mnemonic: "amadd..db.b",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x385F8000,
        mnemonic: "amadd..db.h",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38600000,
        mnemonic: "amswap.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38608000,
        mnemonic: "amswap.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38610000,
        mnemonic: "amadd.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38618000,
        mnemonic: "amadd.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38620000,
        mnemonic: "amand.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38628000,
        mnemonic: "amand.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38630000,
        mnemonic: "amor.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38638000,
        mnemonic: "amor.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38640000,
        mnemonic: "amxor.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38648000,
        mnemonic: "amxor.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38650000,
        mnemonic: "ammax.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38658000,
        mnemonic: "ammax.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38660000,
        mnemonic: "ammin.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38668000,
        mnemonic: "ammin.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38670000,
        mnemonic: "ammax.wu",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38678000,
        mnemonic: "ammax.du",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38680000,
        mnemonic: "ammin.wu",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38688000,
        mnemonic: "ammin.du",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38690000,
        mnemonic: "amswap..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38698000,
        mnemonic: "amswap..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386A0000,
        mnemonic: "amadd..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386A8000,
        mnemonic: "amadd..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386B0000,
        mnemonic: "amand..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386B8000,
        mnemonic: "amand..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386C0000,
        mnemonic: "amor..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386C8000,
        mnemonic: "amor..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386D0000,
        mnemonic: "amxor..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386D8000,
        mnemonic: "amxor..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386E0000,
        mnemonic: "ammax..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386E8000,
        mnemonic: "ammax..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386F0000,
        mnemonic: "ammin..db.w",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x386F8000,
        mnemonic: "ammin..db.d",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38700000,
        mnemonic: "ammax..db.wu",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38708000,
        mnemonic: "ammax..db.du",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38710000,
        mnemonic: "ammin..db.wu",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38718000,
        mnemonic: "ammin..db.du",
        layout: 96,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38720000,
        mnemonic: "dbar",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38728000,
        mnemonic: "ibar",
        layout: 13,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38740000,
        mnemonic: "fldgt.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38748000,
        mnemonic: "fldgt.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38750000,
        mnemonic: "fldle.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38758000,
        mnemonic: "fldle.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38760000,
        mnemonic: "fstgt.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38768000,
        mnemonic: "fstgt.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38770000,
        mnemonic: "fstle.s",
        layout: 90,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38778000,
        mnemonic: "fstle.d",
        layout: 91,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38780000,
        mnemonic: "ldgt.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38788000,
        mnemonic: "ldgt.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38790000,
        mnemonic: "ldgt.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x38798000,
        mnemonic: "ldgt.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387A0000,
        mnemonic: "ldle.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387A8000,
        mnemonic: "ldle.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387B0000,
        mnemonic: "ldle.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387B8000,
        mnemonic: "ldle.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387C0000,
        mnemonic: "stgt.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387C8000,
        mnemonic: "stgt.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387D0000,
        mnemonic: "stgt.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387D8000,
        mnemonic: "stgt.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387E0000,
        mnemonic: "stle.b",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387E8000,
        mnemonic: "stle.h",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387F0000,
        mnemonic: "stle.w",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x387F8000,
        mnemonic: "stle.d",
        layout: 11,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x40000000,
        mnemonic: "beqz",
        layout: 97,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x44000000,
        mnemonic: "bnez",
        layout: 97,
    },
    LoongArchPattern {
        mask: 0xFC000300,
        value: 0x48000000,
        mnemonic: "bceqz",
        layout: 98,
    },
    LoongArchPattern {
        mask: 0xFC000300,
        value: 0x48000100,
        mnemonic: "bcnez",
        layout: 98,
    },
    LoongArchPattern {
        mask: 0xFC0003E0,
        value: 0x48000200,
        mnemonic: "jiscr0",
        layout: 99,
    },
    LoongArchPattern {
        mask: 0xFC0003E0,
        value: 0x48000300,
        mnemonic: "jiscr1",
        layout: 99,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x4C000000,
        mnemonic: "jirl",
        layout: 100,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x50000000,
        mnemonic: "b",
        layout: 101,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x54000000,
        mnemonic: "bl",
        layout: 101,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x58000000,
        mnemonic: "beq",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x5C000000,
        mnemonic: "bne",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x60000000,
        mnemonic: "blt",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x64000000,
        mnemonic: "bge",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x68000000,
        mnemonic: "bltu",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFC000000,
        value: 0x6C000000,
        mnemonic: "bgeu",
        layout: 102,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70000000,
        mnemonic: "vseq.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70008000,
        mnemonic: "vseq.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70010000,
        mnemonic: "vseq.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70018000,
        mnemonic: "vseq.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70020000,
        mnemonic: "vsle.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70028000,
        mnemonic: "vsle.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70030000,
        mnemonic: "vsle.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70038000,
        mnemonic: "vsle.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70040000,
        mnemonic: "vsle.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70048000,
        mnemonic: "vsle.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70050000,
        mnemonic: "vsle.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70058000,
        mnemonic: "vsle.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70060000,
        mnemonic: "vslt.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70068000,
        mnemonic: "vslt.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70070000,
        mnemonic: "vslt.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70078000,
        mnemonic: "vslt.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70080000,
        mnemonic: "vslt.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70088000,
        mnemonic: "vslt.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70090000,
        mnemonic: "vslt.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70098000,
        mnemonic: "vslt.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700A0000,
        mnemonic: "vadd.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700A8000,
        mnemonic: "vadd.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700B0000,
        mnemonic: "vadd.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700B8000,
        mnemonic: "vadd.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700C0000,
        mnemonic: "vsub.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700C8000,
        mnemonic: "vsub.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700D0000,
        mnemonic: "vsub.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x700D8000,
        mnemonic: "vsub.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x701E0000,
        mnemonic: "vaddwev.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x701E8000,
        mnemonic: "vaddwev.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x701F0000,
        mnemonic: "vaddwev.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x701F8000,
        mnemonic: "vaddwev.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70200000,
        mnemonic: "vsubwev.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70208000,
        mnemonic: "vsubwev.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70210000,
        mnemonic: "vsubwev.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70218000,
        mnemonic: "vsubwev.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70220000,
        mnemonic: "vaddwod.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70228000,
        mnemonic: "vaddwod.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70230000,
        mnemonic: "vaddwod.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70238000,
        mnemonic: "vaddwod.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70240000,
        mnemonic: "vsubwod.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70248000,
        mnemonic: "vsubwod.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70250000,
        mnemonic: "vsubwod.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70258000,
        mnemonic: "vsubwod.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x702E0000,
        mnemonic: "vaddwev.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x702E8000,
        mnemonic: "vaddwev.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x702F0000,
        mnemonic: "vaddwev.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x702F8000,
        mnemonic: "vaddwev.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70300000,
        mnemonic: "vsubwev.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70308000,
        mnemonic: "vsubwev.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70310000,
        mnemonic: "vsubwev.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70318000,
        mnemonic: "vsubwev.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70320000,
        mnemonic: "vaddwod.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70328000,
        mnemonic: "vaddwod.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70330000,
        mnemonic: "vaddwod.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70338000,
        mnemonic: "vaddwod.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70340000,
        mnemonic: "vsubwod.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70348000,
        mnemonic: "vsubwod.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70350000,
        mnemonic: "vsubwod.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70358000,
        mnemonic: "vsubwod.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x703E0000,
        mnemonic: "vaddwev.h.bu.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x703E8000,
        mnemonic: "vaddwev.w.hu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x703F0000,
        mnemonic: "vaddwev.d.wu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x703F8000,
        mnemonic: "vaddwev.q.du.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70400000,
        mnemonic: "vaddwod.h.bu.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70408000,
        mnemonic: "vaddwod.w.hu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70410000,
        mnemonic: "vaddwod.d.wu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70418000,
        mnemonic: "vaddwod.q.du.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70460000,
        mnemonic: "vsadd.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70468000,
        mnemonic: "vsadd.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70470000,
        mnemonic: "vsadd.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70478000,
        mnemonic: "vsadd.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70480000,
        mnemonic: "vssub.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70488000,
        mnemonic: "vssub.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70490000,
        mnemonic: "vssub.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70498000,
        mnemonic: "vssub.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704A0000,
        mnemonic: "vsadd.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704A8000,
        mnemonic: "vsadd.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704B0000,
        mnemonic: "vsadd.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704B8000,
        mnemonic: "vsadd.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704C0000,
        mnemonic: "vssub.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704C8000,
        mnemonic: "vssub.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704D0000,
        mnemonic: "vssub.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x704D8000,
        mnemonic: "vssub.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70540000,
        mnemonic: "vhaddw.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70548000,
        mnemonic: "vhaddw.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70550000,
        mnemonic: "vhaddw.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70558000,
        mnemonic: "vhaddw.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70560000,
        mnemonic: "vhsubw.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70568000,
        mnemonic: "vhsubw.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70570000,
        mnemonic: "vhsubw.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70578000,
        mnemonic: "vhsubw.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70580000,
        mnemonic: "vhaddw.hu.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70588000,
        mnemonic: "vhaddw.wu.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70590000,
        mnemonic: "vhaddw.du.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70598000,
        mnemonic: "vhaddw.qu.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705A0000,
        mnemonic: "vhsubw.hu.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705A8000,
        mnemonic: "vhsubw.wu.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705B0000,
        mnemonic: "vhsubw.du.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705B8000,
        mnemonic: "vhsubw.qu.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705C0000,
        mnemonic: "vadda.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705C8000,
        mnemonic: "vadda.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705D0000,
        mnemonic: "vadda.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x705D8000,
        mnemonic: "vadda.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70600000,
        mnemonic: "vabsd.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70608000,
        mnemonic: "vabsd.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70610000,
        mnemonic: "vabsd.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70618000,
        mnemonic: "vabsd.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70620000,
        mnemonic: "vabsd.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70628000,
        mnemonic: "vabsd.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70630000,
        mnemonic: "vabsd.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70638000,
        mnemonic: "vabsd.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70640000,
        mnemonic: "vavg.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70648000,
        mnemonic: "vavg.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70650000,
        mnemonic: "vavg.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70658000,
        mnemonic: "vavg.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70660000,
        mnemonic: "vavg.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70668000,
        mnemonic: "vavg.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70670000,
        mnemonic: "vavg.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70678000,
        mnemonic: "vavg.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70680000,
        mnemonic: "vavgr.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70688000,
        mnemonic: "vavgr.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70690000,
        mnemonic: "vavgr.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70698000,
        mnemonic: "vavgr.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x706A0000,
        mnemonic: "vavgr.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x706A8000,
        mnemonic: "vavgr.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x706B0000,
        mnemonic: "vavgr.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x706B8000,
        mnemonic: "vavgr.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70700000,
        mnemonic: "vmax.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70708000,
        mnemonic: "vmax.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70710000,
        mnemonic: "vmax.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70718000,
        mnemonic: "vmax.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70720000,
        mnemonic: "vmin.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70728000,
        mnemonic: "vmin.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70730000,
        mnemonic: "vmin.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70738000,
        mnemonic: "vmin.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70740000,
        mnemonic: "vmax.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70748000,
        mnemonic: "vmax.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70750000,
        mnemonic: "vmax.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70758000,
        mnemonic: "vmax.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70760000,
        mnemonic: "vmin.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70768000,
        mnemonic: "vmin.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70770000,
        mnemonic: "vmin.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70778000,
        mnemonic: "vmin.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70840000,
        mnemonic: "vmul.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70848000,
        mnemonic: "vmul.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70850000,
        mnemonic: "vmul.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70858000,
        mnemonic: "vmul.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70860000,
        mnemonic: "vmuh.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70868000,
        mnemonic: "vmuh.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70870000,
        mnemonic: "vmuh.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70878000,
        mnemonic: "vmuh.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70880000,
        mnemonic: "vmuh.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70888000,
        mnemonic: "vmuh.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70890000,
        mnemonic: "vmuh.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70898000,
        mnemonic: "vmuh.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70900000,
        mnemonic: "vmulwev.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70908000,
        mnemonic: "vmulwev.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70910000,
        mnemonic: "vmulwev.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70918000,
        mnemonic: "vmulwev.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70920000,
        mnemonic: "vmulwod.h.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70928000,
        mnemonic: "vmulwod.w.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70930000,
        mnemonic: "vmulwod.d.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70938000,
        mnemonic: "vmulwod.q.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70980000,
        mnemonic: "vmulwev.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70988000,
        mnemonic: "vmulwev.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70990000,
        mnemonic: "vmulwev.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70998000,
        mnemonic: "vmulwev.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x709A0000,
        mnemonic: "vmulwod.h.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x709A8000,
        mnemonic: "vmulwod.w.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x709B0000,
        mnemonic: "vmulwod.d.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x709B8000,
        mnemonic: "vmulwod.q.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A00000,
        mnemonic: "vmulwev.h.bu.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A08000,
        mnemonic: "vmulwev.w.hu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A10000,
        mnemonic: "vmulwev.d.wu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A18000,
        mnemonic: "vmulwev.q.du.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A20000,
        mnemonic: "vmulwod.h.bu.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A28000,
        mnemonic: "vmulwod.w.hu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A30000,
        mnemonic: "vmulwod.d.wu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A38000,
        mnemonic: "vmulwod.q.du.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A80000,
        mnemonic: "vmadd.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A88000,
        mnemonic: "vmadd.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A90000,
        mnemonic: "vmadd.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70A98000,
        mnemonic: "vmadd.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AA0000,
        mnemonic: "vmsub.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AA8000,
        mnemonic: "vmsub.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AB0000,
        mnemonic: "vmsub.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AB8000,
        mnemonic: "vmsub.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AC0000,
        mnemonic: "vmaddwev.h.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AC8000,
        mnemonic: "vmaddwev.w.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AD0000,
        mnemonic: "vmaddwev.d.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AD8000,
        mnemonic: "vmaddwev.q.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AE0000,
        mnemonic: "vmaddwod.h.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AE8000,
        mnemonic: "vmaddwod.w.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AF0000,
        mnemonic: "vmaddwod.d.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70AF8000,
        mnemonic: "vmaddwod.q.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B40000,
        mnemonic: "vmaddwev.h.bu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B48000,
        mnemonic: "vmaddwev.w.hu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B50000,
        mnemonic: "vmaddwev.d.wu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B58000,
        mnemonic: "vmaddwev.q.du",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B60000,
        mnemonic: "vmaddwod.h.bu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B68000,
        mnemonic: "vmaddwod.w.hu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B70000,
        mnemonic: "vmaddwod.d.wu",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70B78000,
        mnemonic: "vmaddwod.q.du",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BC0000,
        mnemonic: "vmaddwev.h.bu.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BC8000,
        mnemonic: "vmaddwev.w.hu.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BD0000,
        mnemonic: "vmaddwev.d.wu.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BD8000,
        mnemonic: "vmaddwev.q.du.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BE0000,
        mnemonic: "vmaddwod.h.bu.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BE8000,
        mnemonic: "vmaddwod.w.hu.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BF0000,
        mnemonic: "vmaddwod.d.wu.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70BF8000,
        mnemonic: "vmaddwod.q.du.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E00000,
        mnemonic: "vdiv.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E08000,
        mnemonic: "vdiv.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E10000,
        mnemonic: "vdiv.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E18000,
        mnemonic: "vdiv.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E20000,
        mnemonic: "vmod.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E28000,
        mnemonic: "vmod.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E30000,
        mnemonic: "vmod.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E38000,
        mnemonic: "vmod.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E40000,
        mnemonic: "vdiv.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E48000,
        mnemonic: "vdiv.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E50000,
        mnemonic: "vdiv.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E58000,
        mnemonic: "vdiv.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E60000,
        mnemonic: "vmod.bu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E68000,
        mnemonic: "vmod.hu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E70000,
        mnemonic: "vmod.wu",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E78000,
        mnemonic: "vmod.du",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E80000,
        mnemonic: "vsll.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E88000,
        mnemonic: "vsll.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E90000,
        mnemonic: "vsll.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70E98000,
        mnemonic: "vsll.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EA0000,
        mnemonic: "vsrl.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EA8000,
        mnemonic: "vsrl.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EB0000,
        mnemonic: "vsrl.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EB8000,
        mnemonic: "vsrl.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EC0000,
        mnemonic: "vsra.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EC8000,
        mnemonic: "vsra.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70ED0000,
        mnemonic: "vsra.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70ED8000,
        mnemonic: "vsra.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EE0000,
        mnemonic: "vrotr.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EE8000,
        mnemonic: "vrotr.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EF0000,
        mnemonic: "vrotr.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70EF8000,
        mnemonic: "vrotr.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F00000,
        mnemonic: "vsrlr.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F08000,
        mnemonic: "vsrlr.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F10000,
        mnemonic: "vsrlr.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F18000,
        mnemonic: "vsrlr.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F20000,
        mnemonic: "vsrar.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F28000,
        mnemonic: "vsrar.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F30000,
        mnemonic: "vsrar.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F38000,
        mnemonic: "vsrar.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F48000,
        mnemonic: "vsrln.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F50000,
        mnemonic: "vsrln.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F58000,
        mnemonic: "vsrln.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F68000,
        mnemonic: "vsran.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F70000,
        mnemonic: "vsran.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F78000,
        mnemonic: "vsran.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F88000,
        mnemonic: "vsrlrn.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F90000,
        mnemonic: "vsrlrn.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70F98000,
        mnemonic: "vsrlrn.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FA8000,
        mnemonic: "vsrarn.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FB0000,
        mnemonic: "vsrarn.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FB8000,
        mnemonic: "vsrarn.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FC8000,
        mnemonic: "vssrln.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FD0000,
        mnemonic: "vssrln.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FD8000,
        mnemonic: "vssrln.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FE8000,
        mnemonic: "vssran.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FF0000,
        mnemonic: "vssran.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x70FF8000,
        mnemonic: "vssran.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71008000,
        mnemonic: "vssrlrn.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71010000,
        mnemonic: "vssrlrn.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71018000,
        mnemonic: "vssrlrn.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71028000,
        mnemonic: "vssrarn.b.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71030000,
        mnemonic: "vssrarn.h.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71038000,
        mnemonic: "vssrarn.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71048000,
        mnemonic: "vssrln.bu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71050000,
        mnemonic: "vssrln.hu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71058000,
        mnemonic: "vssrln.wu.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71068000,
        mnemonic: "vssran.bu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71070000,
        mnemonic: "vssran.hu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71078000,
        mnemonic: "vssran.wu.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71088000,
        mnemonic: "vssrlrn.bu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71090000,
        mnemonic: "vssrlrn.hu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71098000,
        mnemonic: "vssrlrn.wu.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710A8000,
        mnemonic: "vssrarn.bu.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710B0000,
        mnemonic: "vssrarn.hu.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710B8000,
        mnemonic: "vssrarn.wu.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710C0000,
        mnemonic: "vbitclr.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710C8000,
        mnemonic: "vbitclr.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710D0000,
        mnemonic: "vbitclr.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710D8000,
        mnemonic: "vbitclr.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710E0000,
        mnemonic: "vbitset.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710E8000,
        mnemonic: "vbitset.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710F0000,
        mnemonic: "vbitset.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x710F8000,
        mnemonic: "vbitset.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71100000,
        mnemonic: "vbitrev.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71108000,
        mnemonic: "vbitrev.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71110000,
        mnemonic: "vbitrev.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71118000,
        mnemonic: "vbitrev.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71160000,
        mnemonic: "vpackev.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71168000,
        mnemonic: "vpackev.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71170000,
        mnemonic: "vpackev.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71178000,
        mnemonic: "vpackev.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71180000,
        mnemonic: "vpackod.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71188000,
        mnemonic: "vpackod.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71190000,
        mnemonic: "vpackod.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71198000,
        mnemonic: "vpackod.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711A0000,
        mnemonic: "vilvl.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711A8000,
        mnemonic: "vilvl.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711B0000,
        mnemonic: "vilvl.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711B8000,
        mnemonic: "vilvl.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711C0000,
        mnemonic: "vilvh.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711C8000,
        mnemonic: "vilvh.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711D0000,
        mnemonic: "vilvh.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711D8000,
        mnemonic: "vilvh.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711E0000,
        mnemonic: "vpickev.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711E8000,
        mnemonic: "vpickev.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711F0000,
        mnemonic: "vpickev.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x711F8000,
        mnemonic: "vpickev.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71200000,
        mnemonic: "vpickod.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71208000,
        mnemonic: "vpickod.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71210000,
        mnemonic: "vpickod.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71218000,
        mnemonic: "vpickod.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71220000,
        mnemonic: "vreplve.b",
        layout: 104,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71228000,
        mnemonic: "vreplve.h",
        layout: 104,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71230000,
        mnemonic: "vreplve.w",
        layout: 104,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71238000,
        mnemonic: "vreplve.d",
        layout: 104,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71260000,
        mnemonic: "vand.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71268000,
        mnemonic: "vor.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71270000,
        mnemonic: "vxor.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71278000,
        mnemonic: "vnor.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71280000,
        mnemonic: "vandn.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71288000,
        mnemonic: "vorn.v",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712B0000,
        mnemonic: "vfrstp.b",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712B8000,
        mnemonic: "vfrstp.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712D0000,
        mnemonic: "vadd.q",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712D8000,
        mnemonic: "vsub.q",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712E0000,
        mnemonic: "vsigncov.b",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712E8000,
        mnemonic: "vsigncov.h",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712F0000,
        mnemonic: "vsigncov.w",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x712F8000,
        mnemonic: "vsigncov.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71308000,
        mnemonic: "vfadd.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71310000,
        mnemonic: "vfadd.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71328000,
        mnemonic: "vfsub.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71330000,
        mnemonic: "vfsub.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71388000,
        mnemonic: "vfmul.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71390000,
        mnemonic: "vfmul.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713A8000,
        mnemonic: "vfdiv.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713B0000,
        mnemonic: "vfdiv.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713C8000,
        mnemonic: "vfmax.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713D0000,
        mnemonic: "vfmax.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713E8000,
        mnemonic: "vfmin.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x713F0000,
        mnemonic: "vfmin.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71408000,
        mnemonic: "vfmaxa.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71410000,
        mnemonic: "vfmaxa.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71428000,
        mnemonic: "vfmina.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71430000,
        mnemonic: "vfmina.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71460000,
        mnemonic: "vfcvt.h.s",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71468000,
        mnemonic: "vfcvt.s.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71480000,
        mnemonic: "vffint.s.l",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x71498000,
        mnemonic: "vftint.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x714A0000,
        mnemonic: "vftintrm.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x714A8000,
        mnemonic: "vftintrp.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x714B0000,
        mnemonic: "vftintrz.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x714B8000,
        mnemonic: "vftintrne.w.d",
        layout: 63,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x717A8000,
        mnemonic: "vshuf.h",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x717B0000,
        mnemonic: "vshuf.w",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x717B8000,
        mnemonic: "vshuf.d",
        layout: 103,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72800000,
        mnemonic: "vseqi.b",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72808000,
        mnemonic: "vseqi.h",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72810000,
        mnemonic: "vseqi.w",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72818000,
        mnemonic: "vseqi.d",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72820000,
        mnemonic: "vslei.b",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72828000,
        mnemonic: "vslei.h",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72830000,
        mnemonic: "vslei.w",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72838000,
        mnemonic: "vslei.d",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72840000,
        mnemonic: "vslei.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72848000,
        mnemonic: "vslei.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72850000,
        mnemonic: "vslei.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72858000,
        mnemonic: "vslei.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72860000,
        mnemonic: "vslti.b",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72868000,
        mnemonic: "vslti.h",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72870000,
        mnemonic: "vslti.w",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72878000,
        mnemonic: "vslti.d",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72880000,
        mnemonic: "vslti.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72888000,
        mnemonic: "vslti.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72890000,
        mnemonic: "vslti.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72898000,
        mnemonic: "vslti.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728A0000,
        mnemonic: "vaddi.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728A8000,
        mnemonic: "vaddi.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728B0000,
        mnemonic: "vaddi.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728B8000,
        mnemonic: "vaddi.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728C0000,
        mnemonic: "vsubi.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728C8000,
        mnemonic: "vsubi.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728D0000,
        mnemonic: "vsubi.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728D8000,
        mnemonic: "vsubi.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728E0000,
        mnemonic: "vbsll.v",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x728E8000,
        mnemonic: "vbsrl.v",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72900000,
        mnemonic: "vmaxi.b",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72908000,
        mnemonic: "vmaxi.h",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72910000,
        mnemonic: "vmaxi.w",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72918000,
        mnemonic: "vmaxi.d",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72920000,
        mnemonic: "vmini.b",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72928000,
        mnemonic: "vmini.h",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72930000,
        mnemonic: "vmini.w",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72938000,
        mnemonic: "vmini.d",
        layout: 105,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72940000,
        mnemonic: "vmaxi.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72948000,
        mnemonic: "vmaxi.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72950000,
        mnemonic: "vmaxi.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72958000,
        mnemonic: "vmaxi.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72960000,
        mnemonic: "vmini.bu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72968000,
        mnemonic: "vmini.hu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72970000,
        mnemonic: "vmini.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72978000,
        mnemonic: "vmini.du",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x729A0000,
        mnemonic: "vfrstpi.b",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x729A8000,
        mnemonic: "vfrstpi.h",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C0000,
        mnemonic: "vclo.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C0400,
        mnemonic: "vclo.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C0800,
        mnemonic: "vclo.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C0C00,
        mnemonic: "vclo.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C1000,
        mnemonic: "vclz.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C1400,
        mnemonic: "vclz.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C1800,
        mnemonic: "vclz.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C1C00,
        mnemonic: "vclz.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C2000,
        mnemonic: "vpcnt.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C2400,
        mnemonic: "vpcnt.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C2800,
        mnemonic: "vpcnt.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C2C00,
        mnemonic: "vpcnt.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C3000,
        mnemonic: "vneg.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C3400,
        mnemonic: "vneg.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C3800,
        mnemonic: "vneg.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C3C00,
        mnemonic: "vneg.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C4000,
        mnemonic: "vmskltz.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C4400,
        mnemonic: "vmskltz.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C4800,
        mnemonic: "vmskltz.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C4C00,
        mnemonic: "vmskltz.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C5000,
        mnemonic: "vmskgez.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729C6000,
        mnemonic: "vmsknz.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729C9800,
        mnemonic: "vseteqz.v",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729C9C00,
        mnemonic: "vsetnez.v",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CA000,
        mnemonic: "vsetanyeqz.b",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CA400,
        mnemonic: "vsetanyeqz.h",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CA800,
        mnemonic: "vsetanyeqz.w",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CAC00,
        mnemonic: "vsetanyeqz.d",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CB000,
        mnemonic: "vsetallnez.b",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CB400,
        mnemonic: "vsetallnez.h",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CB800,
        mnemonic: "vsetallnez.w",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x729CBC00,
        mnemonic: "vsetallnez.d",
        layout: 109,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CC400,
        mnemonic: "vflogb.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CC800,
        mnemonic: "vflogb.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CD400,
        mnemonic: "vfclass.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CD800,
        mnemonic: "vfclass.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CE400,
        mnemonic: "vfsqrt.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CE800,
        mnemonic: "vfsqrt.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CF400,
        mnemonic: "vfrecip.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729CF800,
        mnemonic: "vfrecip.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D0400,
        mnemonic: "vfrsqrt.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D0800,
        mnemonic: "vfrsqrt.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D1400,
        mnemonic: "vfrecipe.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D1800,
        mnemonic: "vfrecipe.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D2400,
        mnemonic: "vfrsqrte.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D2800,
        mnemonic: "vfrsqrte.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D3400,
        mnemonic: "vfrint.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D3800,
        mnemonic: "vfrint.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D4400,
        mnemonic: "vfrintrm.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D4800,
        mnemonic: "vfrintrm.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D5400,
        mnemonic: "vfrintrp.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D5800,
        mnemonic: "vfrintrp.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D6400,
        mnemonic: "vfrintrz.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D6800,
        mnemonic: "vfrintrz.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D7400,
        mnemonic: "vfrintrne.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729D7800,
        mnemonic: "vfrintrne.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729DE800,
        mnemonic: "vfcvtl.s.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729DEC00,
        mnemonic: "vfcvth.s.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729DF000,
        mnemonic: "vfcvtl.d.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729DF400,
        mnemonic: "vfcvth.d.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E0000,
        mnemonic: "vffint.s.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E0400,
        mnemonic: "vffint.s.wu",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E0800,
        mnemonic: "vffint.d.l",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E0C00,
        mnemonic: "vffint.d.lu",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E1000,
        mnemonic: "vffintl.d.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E1400,
        mnemonic: "vffinth.d.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E3000,
        mnemonic: "vftint.w.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E3400,
        mnemonic: "vftint.l.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E3800,
        mnemonic: "vftintrm.w.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E3C00,
        mnemonic: "vftintrm.l.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E4000,
        mnemonic: "vftintrp.w.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E4400,
        mnemonic: "vftintrp.l.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E4800,
        mnemonic: "vftintrz.w.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E4C00,
        mnemonic: "vftintrz.l.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E5000,
        mnemonic: "vftintrne.w.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E5400,
        mnemonic: "vftintrne.l.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E5800,
        mnemonic: "vftint.wu.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E5C00,
        mnemonic: "vftint.lu.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E7000,
        mnemonic: "vftintrz.wu.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E7400,
        mnemonic: "vftintrz.lu.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E8000,
        mnemonic: "vftintl.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E8400,
        mnemonic: "vftinth.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E8800,
        mnemonic: "vftintrml.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E8C00,
        mnemonic: "vftintrmh.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E9000,
        mnemonic: "vftintrpl.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E9400,
        mnemonic: "vftintrph.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E9800,
        mnemonic: "vftintrzl.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729E9C00,
        mnemonic: "vftintrzh.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EA000,
        mnemonic: "vftintrnel.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EA400,
        mnemonic: "vftintrneh.l.s",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EE000,
        mnemonic: "vexth.h.b",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EE400,
        mnemonic: "vexth.w.h",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EE800,
        mnemonic: "vexth.d.w",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EEC00,
        mnemonic: "vexth.q.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EF000,
        mnemonic: "vexth.hu.bu",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EF400,
        mnemonic: "vexth.wu.hu",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EF800,
        mnemonic: "vexth.du.wu",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729EFC00,
        mnemonic: "vexth.qu.du",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729F0000,
        mnemonic: "vreplgr2vr.b",
        layout: 110,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729F0400,
        mnemonic: "vreplgr2vr.h",
        layout: 110,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729F0800,
        mnemonic: "vreplgr2vr.w",
        layout: 110,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x729F0C00,
        mnemonic: "vreplgr2vr.d",
        layout: 110,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72A02000,
        mnemonic: "vrotri.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72A04000,
        mnemonic: "vrotri.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72A08000,
        mnemonic: "vrotri.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x72A10000,
        mnemonic: "vrotri.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72A42000,
        mnemonic: "vsrlri.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72A44000,
        mnemonic: "vsrlri.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72A48000,
        mnemonic: "vsrlri.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x72A50000,
        mnemonic: "vsrlri.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72A82000,
        mnemonic: "vsrari.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72A84000,
        mnemonic: "vsrari.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x72A88000,
        mnemonic: "vsrari.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x72A90000,
        mnemonic: "vsrari.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72EB8000,
        mnemonic: "vinsgr2vr.b",
        layout: 114,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72EBC000,
        mnemonic: "vinsgr2vr.h",
        layout: 115,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x72EBE000,
        mnemonic: "vinsgr2vr.w",
        layout: 116,
    },
    LoongArchPattern {
        mask: 0xFFFFF800,
        value: 0x72EBF000,
        mnemonic: "vinsgr2vr.d",
        layout: 117,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72EF8000,
        mnemonic: "vpickve2gr.b",
        layout: 118,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72EFC000,
        mnemonic: "vpickve2gr.h",
        layout: 119,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x72EFE000,
        mnemonic: "vpickve2gr.w",
        layout: 120,
    },
    LoongArchPattern {
        mask: 0xFFFFF800,
        value: 0x72EFF000,
        mnemonic: "vpickve2gr.d",
        layout: 121,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72F38000,
        mnemonic: "vpickve2gr.bu",
        layout: 118,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72F3C000,
        mnemonic: "vpickve2gr.hu",
        layout: 119,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x72F3E000,
        mnemonic: "vpickve2gr.wu",
        layout: 120,
    },
    LoongArchPattern {
        mask: 0xFFFFF800,
        value: 0x72F3F000,
        mnemonic: "vpickve2gr.du",
        layout: 121,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x72F78000,
        mnemonic: "vreplvei.b",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x72F7C000,
        mnemonic: "vreplvei.h",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x72F7E000,
        mnemonic: "vreplvei.w",
        layout: 122,
    },
    LoongArchPattern {
        mask: 0xFFFFF800,
        value: 0x72F7F000,
        mnemonic: "vreplvei.d",
        layout: 123,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73082000,
        mnemonic: "vsllwil.h.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73084000,
        mnemonic: "vsllwil.w.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73088000,
        mnemonic: "vsllwil.d.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x73090000,
        mnemonic: "vextl.q.d",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x730C2000,
        mnemonic: "vsllwil.hu.bu",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x730C4000,
        mnemonic: "vsllwil.wu.hu",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x730C8000,
        mnemonic: "vsllwil.du.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x730D0000,
        mnemonic: "vextl.qu.du",
        layout: 108,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73102000,
        mnemonic: "vbitclri.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73104000,
        mnemonic: "vbitclri.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73108000,
        mnemonic: "vbitclri.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73110000,
        mnemonic: "vbitclri.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73142000,
        mnemonic: "vbitseti.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73144000,
        mnemonic: "vbitseti.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73148000,
        mnemonic: "vbitseti.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73150000,
        mnemonic: "vbitseti.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73182000,
        mnemonic: "vbitrevi.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73184000,
        mnemonic: "vbitrevi.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73188000,
        mnemonic: "vbitrevi.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73190000,
        mnemonic: "vbitrevi.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73242000,
        mnemonic: "vsat.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73244000,
        mnemonic: "vsat.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73248000,
        mnemonic: "vsat.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73250000,
        mnemonic: "vsat.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73282000,
        mnemonic: "vsat.bu",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73284000,
        mnemonic: "vsat.hu",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73288000,
        mnemonic: "vsat.wu",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73290000,
        mnemonic: "vsat.du",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x732C2000,
        mnemonic: "vslli.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x732C4000,
        mnemonic: "vslli.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x732C8000,
        mnemonic: "vslli.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x732D0000,
        mnemonic: "vslli.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73302000,
        mnemonic: "vsrli.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73304000,
        mnemonic: "vsrli.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73308000,
        mnemonic: "vsrli.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73310000,
        mnemonic: "vsrli.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x73342000,
        mnemonic: "vsrai.b",
        layout: 111,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73344000,
        mnemonic: "vsrai.h",
        layout: 112,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73348000,
        mnemonic: "vsrai.w",
        layout: 106,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73350000,
        mnemonic: "vsrai.d",
        layout: 113,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73404000,
        mnemonic: "vsrlni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73408000,
        mnemonic: "vsrlni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73410000,
        mnemonic: "vsrlni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73420000,
        mnemonic: "vsrlni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73444000,
        mnemonic: "vsrlrni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73448000,
        mnemonic: "vsrlrni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73450000,
        mnemonic: "vsrlrni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73460000,
        mnemonic: "vsrlrni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73484000,
        mnemonic: "vssrlni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73488000,
        mnemonic: "vssrlni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73490000,
        mnemonic: "vssrlni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x734A0000,
        mnemonic: "vssrlni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x734C4000,
        mnemonic: "vssrlni.bu.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x734C8000,
        mnemonic: "vssrlni.hu.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x734D0000,
        mnemonic: "vssrlni.wu.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x734E0000,
        mnemonic: "vssrlni.du.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73504000,
        mnemonic: "vssrlrni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73508000,
        mnemonic: "vssrlrni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73510000,
        mnemonic: "vssrlrni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73520000,
        mnemonic: "vssrlrni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73544000,
        mnemonic: "vssrlrni.bu.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73548000,
        mnemonic: "vssrlrni.hu.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73550000,
        mnemonic: "vssrlrni.wu.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73560000,
        mnemonic: "vssrlrni.du.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73584000,
        mnemonic: "vsrani.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73588000,
        mnemonic: "vsrani.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73590000,
        mnemonic: "vsrani.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x735A0000,
        mnemonic: "vsrani.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x735C4000,
        mnemonic: "vsrarni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x735C8000,
        mnemonic: "vsrarni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x735D0000,
        mnemonic: "vsrarni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x735E0000,
        mnemonic: "vsrarni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73604000,
        mnemonic: "vssrani.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73608000,
        mnemonic: "vssrani.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73610000,
        mnemonic: "vssrani.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73620000,
        mnemonic: "vssrani.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73644000,
        mnemonic: "vssrani.bu.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73648000,
        mnemonic: "vssrani.hu.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73650000,
        mnemonic: "vssrani.wu.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x73660000,
        mnemonic: "vssrani.du.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x73684000,
        mnemonic: "vssrarni.b.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x73688000,
        mnemonic: "vssrarni.h.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x73690000,
        mnemonic: "vssrarni.w.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x736A0000,
        mnemonic: "vssrarni.d.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x736C4000,
        mnemonic: "vssrarni.bu.h",
        layout: 124,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x736C8000,
        mnemonic: "vssrarni.hu.w",
        layout: 107,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x736D0000,
        mnemonic: "vssrarni.wu.d",
        layout: 125,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x736E0000,
        mnemonic: "vssrarni.du.q",
        layout: 126,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73800000,
        mnemonic: "vextrins.d",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73840000,
        mnemonic: "vextrins.w",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73880000,
        mnemonic: "vextrins.h",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x738C0000,
        mnemonic: "vextrins.b",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73900000,
        mnemonic: "vshuf4i.b",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73940000,
        mnemonic: "vshuf4i.h",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73980000,
        mnemonic: "vshuf4i.w",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x739C0000,
        mnemonic: "vshuf4i.d",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73C40000,
        mnemonic: "vbitseli.b",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73D00000,
        mnemonic: "vandi.b",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73D40000,
        mnemonic: "vori.b",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73D80000,
        mnemonic: "vxori.b",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73DC0000,
        mnemonic: "vnori.b",
        layout: 128,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73E00000,
        mnemonic: "vldi",
        layout: 129,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x73E40000,
        mnemonic: "vpermi.w",
        layout: 127,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74000000,
        mnemonic: "xvseq.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74008000,
        mnemonic: "xvseq.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74010000,
        mnemonic: "xvseq.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74018000,
        mnemonic: "xvseq.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74020000,
        mnemonic: "xvsle.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74028000,
        mnemonic: "xvsle.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74030000,
        mnemonic: "xvsle.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74038000,
        mnemonic: "xvsle.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74040000,
        mnemonic: "xvsle.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74048000,
        mnemonic: "xvsle.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74050000,
        mnemonic: "xvsle.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74058000,
        mnemonic: "xvsle.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74060000,
        mnemonic: "xvslt.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74068000,
        mnemonic: "xvslt.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74070000,
        mnemonic: "xvslt.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74078000,
        mnemonic: "xvslt.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74080000,
        mnemonic: "xvslt.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74088000,
        mnemonic: "xvslt.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74090000,
        mnemonic: "xvslt.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74098000,
        mnemonic: "xvslt.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740A0000,
        mnemonic: "xvadd.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740A8000,
        mnemonic: "xvadd.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740B0000,
        mnemonic: "xvadd.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740B8000,
        mnemonic: "xvadd.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740C0000,
        mnemonic: "xvsub.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740C8000,
        mnemonic: "xvsub.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740D0000,
        mnemonic: "xvsub.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x740D8000,
        mnemonic: "xvsub.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x741E0000,
        mnemonic: "xvaddwev.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x741E8000,
        mnemonic: "xvaddwev.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x741F0000,
        mnemonic: "xvaddwev.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x741F8000,
        mnemonic: "xvaddwev.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74200000,
        mnemonic: "xvsubwev.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74208000,
        mnemonic: "xvsubwev.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74210000,
        mnemonic: "xvsubwev.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74218000,
        mnemonic: "xvsubwev.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74220000,
        mnemonic: "xvaddwod.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74228000,
        mnemonic: "xvaddwod.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74230000,
        mnemonic: "xvaddwod.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74238000,
        mnemonic: "xvaddwod.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74240000,
        mnemonic: "xvsubwod.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74248000,
        mnemonic: "xvsubwod.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74250000,
        mnemonic: "xvsubwod.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74258000,
        mnemonic: "xvsubwod.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x742E0000,
        mnemonic: "xvaddwev.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x742E8000,
        mnemonic: "xvaddwev.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x742F0000,
        mnemonic: "xvaddwev.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x742F8000,
        mnemonic: "xvaddwev.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74300000,
        mnemonic: "xvsubwev.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74308000,
        mnemonic: "xvsubwev.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74310000,
        mnemonic: "xvsubwev.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74318000,
        mnemonic: "xvsubwev.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74320000,
        mnemonic: "xvaddwod.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74328000,
        mnemonic: "xvaddwod.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74330000,
        mnemonic: "xvaddwod.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74338000,
        mnemonic: "xvaddwod.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74340000,
        mnemonic: "xvsubwod.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74348000,
        mnemonic: "xvsubwod.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74350000,
        mnemonic: "xvsubwod.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74358000,
        mnemonic: "xvsubwod.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x743E0000,
        mnemonic: "xvaddwev.h.bu.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x743E8000,
        mnemonic: "xvaddwev.w.hu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x743F0000,
        mnemonic: "xvaddwev.d.wu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x743F8000,
        mnemonic: "xvaddwev.q.du.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74400000,
        mnemonic: "xvaddwod.h.bu.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74408000,
        mnemonic: "xvaddwod.w.hu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74410000,
        mnemonic: "xvaddwod.d.wu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74418000,
        mnemonic: "xvaddwod.q.du.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74460000,
        mnemonic: "xvsadd.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74468000,
        mnemonic: "xvsadd.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74470000,
        mnemonic: "xvsadd.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74478000,
        mnemonic: "xvsadd.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74480000,
        mnemonic: "xvssub.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74488000,
        mnemonic: "xvssub.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74490000,
        mnemonic: "xvssub.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74498000,
        mnemonic: "xvssub.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744A0000,
        mnemonic: "xvsadd.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744A8000,
        mnemonic: "xvsadd.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744B0000,
        mnemonic: "xvsadd.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744B8000,
        mnemonic: "xvsadd.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744C0000,
        mnemonic: "xvssub.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744C8000,
        mnemonic: "xvssub.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744D0000,
        mnemonic: "xvssub.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x744D8000,
        mnemonic: "xvssub.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74540000,
        mnemonic: "xvhaddw.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74548000,
        mnemonic: "xvhaddw.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74550000,
        mnemonic: "xvhaddw.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74558000,
        mnemonic: "xvhaddw.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74560000,
        mnemonic: "xvhsubw.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74568000,
        mnemonic: "xvhsubw.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74570000,
        mnemonic: "xvhsubw.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74578000,
        mnemonic: "xvhsubw.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74580000,
        mnemonic: "xvhaddw.hu.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74588000,
        mnemonic: "xvhaddw.wu.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74590000,
        mnemonic: "xvhaddw.du.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74598000,
        mnemonic: "xvhaddw.qu.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745A0000,
        mnemonic: "xvhsubw.hu.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745A8000,
        mnemonic: "xvhsubw.wu.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745B0000,
        mnemonic: "xvhsubw.du.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745B8000,
        mnemonic: "xvhsubw.qu.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745C0000,
        mnemonic: "xvadda.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745C8000,
        mnemonic: "xvadda.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745D0000,
        mnemonic: "xvadda.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x745D8000,
        mnemonic: "xvadda.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74600000,
        mnemonic: "xvabsd.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74608000,
        mnemonic: "xvabsd.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74610000,
        mnemonic: "xvabsd.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74618000,
        mnemonic: "xvabsd.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74620000,
        mnemonic: "xvabsd.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74628000,
        mnemonic: "xvabsd.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74630000,
        mnemonic: "xvabsd.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74638000,
        mnemonic: "xvabsd.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74640000,
        mnemonic: "xvavg.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74648000,
        mnemonic: "xvavg.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74650000,
        mnemonic: "xvavg.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74658000,
        mnemonic: "xvavg.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74660000,
        mnemonic: "xvavg.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74668000,
        mnemonic: "xvavg.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74670000,
        mnemonic: "xvavg.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74678000,
        mnemonic: "xvavg.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74680000,
        mnemonic: "xvavgr.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74688000,
        mnemonic: "xvavgr.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74690000,
        mnemonic: "xvavgr.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74698000,
        mnemonic: "xvavgr.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x746A0000,
        mnemonic: "xvavgr.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x746A8000,
        mnemonic: "xvavgr.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x746B0000,
        mnemonic: "xvavgr.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x746B8000,
        mnemonic: "xvavgr.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74700000,
        mnemonic: "xvmax.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74708000,
        mnemonic: "xvmax.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74710000,
        mnemonic: "xvmax.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74718000,
        mnemonic: "xvmax.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74720000,
        mnemonic: "xvmin.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74728000,
        mnemonic: "xvmin.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74730000,
        mnemonic: "xvmin.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74738000,
        mnemonic: "xvmin.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74740000,
        mnemonic: "xvmax.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74748000,
        mnemonic: "xvmax.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74750000,
        mnemonic: "xvmax.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74758000,
        mnemonic: "xvmax.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74760000,
        mnemonic: "xvmin.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74768000,
        mnemonic: "xvmin.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74770000,
        mnemonic: "xvmin.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74778000,
        mnemonic: "xvmin.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74840000,
        mnemonic: "xvmul.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74848000,
        mnemonic: "xvmul.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74850000,
        mnemonic: "xvmul.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74858000,
        mnemonic: "xvmul.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74860000,
        mnemonic: "xvmuh.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74868000,
        mnemonic: "xvmuh.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74870000,
        mnemonic: "xvmuh.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74878000,
        mnemonic: "xvmuh.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74880000,
        mnemonic: "xvmuh.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74888000,
        mnemonic: "xvmuh.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74890000,
        mnemonic: "xvmuh.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74898000,
        mnemonic: "xvmuh.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74900000,
        mnemonic: "xvmulwev.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74908000,
        mnemonic: "xvmulwev.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74910000,
        mnemonic: "xvmulwev.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74918000,
        mnemonic: "xvmulwev.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74920000,
        mnemonic: "xvmulwod.h.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74928000,
        mnemonic: "xvmulwod.w.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74930000,
        mnemonic: "xvmulwod.d.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74938000,
        mnemonic: "xvmulwod.q.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74980000,
        mnemonic: "xvmulwev.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74988000,
        mnemonic: "xvmulwev.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74990000,
        mnemonic: "xvmulwev.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74998000,
        mnemonic: "xvmulwev.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x749A0000,
        mnemonic: "xvmulwod.h.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x749A8000,
        mnemonic: "xvmulwod.w.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x749B0000,
        mnemonic: "xvmulwod.d.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x749B8000,
        mnemonic: "xvmulwod.q.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A00000,
        mnemonic: "xvmulwev.h.bu.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A08000,
        mnemonic: "xvmulwev.w.hu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A10000,
        mnemonic: "xvmulwev.d.wu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A18000,
        mnemonic: "xvmulwev.q.du.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A20000,
        mnemonic: "xvmulwod.h.bu.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A28000,
        mnemonic: "xvmulwod.w.hu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A30000,
        mnemonic: "xvmulwod.d.wu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A38000,
        mnemonic: "xvmulwod.q.du.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A80000,
        mnemonic: "xvmadd.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A88000,
        mnemonic: "xvmadd.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A90000,
        mnemonic: "xvmadd.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74A98000,
        mnemonic: "xvmadd.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AA0000,
        mnemonic: "xvmsub.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AA8000,
        mnemonic: "xvmsub.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AB0000,
        mnemonic: "xvmsub.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AB8000,
        mnemonic: "xvmsub.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AC0000,
        mnemonic: "xvmaddwev.h.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AC8000,
        mnemonic: "xvmaddwev.w.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AD0000,
        mnemonic: "xvmaddwod.h.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AD8000,
        mnemonic: "xvmaddwev.q.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AE0000,
        mnemonic: "xvmaddwod.h.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AE8000,
        mnemonic: "xvmaddwod.w.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AF0000,
        mnemonic: "xvmaddwod.d.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74AF8000,
        mnemonic: "xvmaddwod.q.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B40000,
        mnemonic: "xvmaddwev.h.bu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B48000,
        mnemonic: "xvmaddwev.w.hu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B50000,
        mnemonic: "xvmaddwev.d.wu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B58000,
        mnemonic: "xvmaddwod.d.wu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B60000,
        mnemonic: "xvmaddwod.h.bu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B68000,
        mnemonic: "xvmaddwod.w.hu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B70000,
        mnemonic: "xvmaddwod.d.wu",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74B78000,
        mnemonic: "xvmaddwod.q.du",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BC0000,
        mnemonic: "xvmaddwev.h.bu.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BC8000,
        mnemonic: "xvmaddwev.w.hu.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BD0000,
        mnemonic: "xvmaddwev.d.wu.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BD8000,
        mnemonic: "xvmaddwev.q.du.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BE0000,
        mnemonic: "xvmaddwod.h.bu.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BE8000,
        mnemonic: "xvmaddwod.w.hu.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BF0000,
        mnemonic: "xvmaddwod.d.wu.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74BF8000,
        mnemonic: "xvmaddwod.q.du.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E00000,
        mnemonic: "xvdiv.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E08000,
        mnemonic: "xvdiv.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E10000,
        mnemonic: "xvdiv.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E18000,
        mnemonic: "xvdiv.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E20000,
        mnemonic: "xvmod.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E28000,
        mnemonic: "xvmod.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E30000,
        mnemonic: "xvmod.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E38000,
        mnemonic: "xvmod.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E40000,
        mnemonic: "xvdiv.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E48000,
        mnemonic: "xvdiv.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E50000,
        mnemonic: "xvdiv.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E58000,
        mnemonic: "xvdiv.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E60000,
        mnemonic: "xvmod.bu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E68000,
        mnemonic: "xvmod.hu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E70000,
        mnemonic: "xvmod.wu",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E78000,
        mnemonic: "xvmod.du",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E80000,
        mnemonic: "xvsll.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E88000,
        mnemonic: "xvsll.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E90000,
        mnemonic: "xvsll.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74E98000,
        mnemonic: "xvsll.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EA0000,
        mnemonic: "xvsrl.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EA8000,
        mnemonic: "xvsrl.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EB0000,
        mnemonic: "xvsrl.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EB8000,
        mnemonic: "xvsrl.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EC0000,
        mnemonic: "xvsra.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EC8000,
        mnemonic: "xvsra.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74ED0000,
        mnemonic: "xvsra.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74ED8000,
        mnemonic: "xvsra.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EE0000,
        mnemonic: "xvrotr.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EE8000,
        mnemonic: "xvrotr.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EF0000,
        mnemonic: "xvrotr.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74EF8000,
        mnemonic: "xvrotr.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F00000,
        mnemonic: "xvsrlr.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F08000,
        mnemonic: "xvsrlr.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F10000,
        mnemonic: "xvsrlr.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F18000,
        mnemonic: "xvsrlr.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F20000,
        mnemonic: "xvsrar.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F28000,
        mnemonic: "xvsrar.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F30000,
        mnemonic: "xvsrar.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F38000,
        mnemonic: "xvsrar.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F48000,
        mnemonic: "xvsrln.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F50000,
        mnemonic: "xvsrln.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F58000,
        mnemonic: "xvsrln.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F68000,
        mnemonic: "xvsran.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F70000,
        mnemonic: "xvsran.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F78000,
        mnemonic: "xvsran.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F88000,
        mnemonic: "xvsrlrn.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F90000,
        mnemonic: "xvsrlrn.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74F98000,
        mnemonic: "xvsrlrn.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FA8000,
        mnemonic: "xvsrarn.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FB0000,
        mnemonic: "xvsrarn.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FB8000,
        mnemonic: "xvsrarn.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FC8000,
        mnemonic: "xvssrln.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FD0000,
        mnemonic: "xvssrln.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FD8000,
        mnemonic: "xvssrln.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FE8000,
        mnemonic: "xvssran.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FF0000,
        mnemonic: "xvssran.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x74FF8000,
        mnemonic: "xvssran.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75008000,
        mnemonic: "xvssrlrn.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75010000,
        mnemonic: "xvssrlrn.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75018000,
        mnemonic: "xvssrlrn.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75028000,
        mnemonic: "xvssrarn.b.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75030000,
        mnemonic: "xvssrarn.h.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75038000,
        mnemonic: "xvssrarn.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75048000,
        mnemonic: "xvssrln.bu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75050000,
        mnemonic: "xvssrln.hu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75058000,
        mnemonic: "xvssrln.wu.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75068000,
        mnemonic: "xvssran.bu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75070000,
        mnemonic: "xvssran.hu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75078000,
        mnemonic: "xvssran.wu.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75088000,
        mnemonic: "xvssrlrn.bu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75090000,
        mnemonic: "xvssrlrn.hu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75098000,
        mnemonic: "xvssrlrn.wu.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750A8000,
        mnemonic: "xvssrarn.bu.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750B0000,
        mnemonic: "xvssrarn.hu.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750B8000,
        mnemonic: "xvssrarn.wu.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750C0000,
        mnemonic: "xvbitclr.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750C8000,
        mnemonic: "xvbitclr.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750D0000,
        mnemonic: "xvbitclr.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750D8000,
        mnemonic: "xvbitclr.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750E0000,
        mnemonic: "xvbitset.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750E8000,
        mnemonic: "xvbitset.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750F0000,
        mnemonic: "xvbitset.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x750F8000,
        mnemonic: "xvbitset.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75100000,
        mnemonic: "xvbitrev.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75108000,
        mnemonic: "xvbitrev.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75110000,
        mnemonic: "xvbitrev.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75118000,
        mnemonic: "xvbitrev.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75160000,
        mnemonic: "xvpackev.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75168000,
        mnemonic: "xvpackev.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75170000,
        mnemonic: "xvpackev.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75178000,
        mnemonic: "xvpackev.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75180000,
        mnemonic: "xvpackod.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75188000,
        mnemonic: "xvpackod.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75190000,
        mnemonic: "xvpackod.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75198000,
        mnemonic: "xvpackod.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751A0000,
        mnemonic: "xvilvl.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751A8000,
        mnemonic: "xvilvl.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751B0000,
        mnemonic: "xvilvl.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751B8000,
        mnemonic: "xvilvl.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751C0000,
        mnemonic: "xvilvh.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751C8000,
        mnemonic: "xvilvh.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751D0000,
        mnemonic: "xvilvh.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751D8000,
        mnemonic: "xvilvh.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751E0000,
        mnemonic: "xvpickev.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751E8000,
        mnemonic: "xvpickev.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751F0000,
        mnemonic: "xvpickev.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x751F8000,
        mnemonic: "xvpickev.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75200000,
        mnemonic: "xvpickod.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75208000,
        mnemonic: "xvpickod.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75210000,
        mnemonic: "xvpickod.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75218000,
        mnemonic: "xvpickod.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75220000,
        mnemonic: "xvreplve.b",
        layout: 131,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75228000,
        mnemonic: "xvreplve.h",
        layout: 131,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75230000,
        mnemonic: "xvreplve.w",
        layout: 131,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75238000,
        mnemonic: "xvreplve.d",
        layout: 131,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75260000,
        mnemonic: "xvandn.v",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75268000,
        mnemonic: "xvorn.v",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75270000,
        mnemonic: "xvfrstp.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75278000,
        mnemonic: "xvfrstp.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75280000,
        mnemonic: "xvadd.q",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75288000,
        mnemonic: "xvsub.q",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752B0000,
        mnemonic: "xvsigncov.b",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752B8000,
        mnemonic: "xvfrstp.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752D0000,
        mnemonic: "xvadd.q",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752D8000,
        mnemonic: "xvsub.q",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752E0000,
        mnemonic: "xvsigncov.b",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752E8000,
        mnemonic: "xvsigncov.h",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752F0000,
        mnemonic: "xvsigncov.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x752F8000,
        mnemonic: "xvsigncov.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75308000,
        mnemonic: "xvfadd.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75310000,
        mnemonic: "xvfadd.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75328000,
        mnemonic: "xvfsub.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75330000,
        mnemonic: "xvfsub.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75388000,
        mnemonic: "xvfmul.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75390000,
        mnemonic: "xvfmul.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753A8000,
        mnemonic: "xvfdiv.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753B0000,
        mnemonic: "xvfdiv.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753C8000,
        mnemonic: "xvfmax.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753D0000,
        mnemonic: "xvfmax.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753E8000,
        mnemonic: "xvfmin.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x753F0000,
        mnemonic: "xvfmin.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75408000,
        mnemonic: "xvfmaxa.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75410000,
        mnemonic: "xvfmaxa.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75428000,
        mnemonic: "xvfmina.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75430000,
        mnemonic: "xvfmina.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75460000,
        mnemonic: "xvfcvt.h.s",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75468000,
        mnemonic: "xvfcvt.s.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75480000,
        mnemonic: "xvffint.s.l",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x75498000,
        mnemonic: "xvftint.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x754A0000,
        mnemonic: "xvftintrm.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x754A8000,
        mnemonic: "xvftintrp.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x754B0000,
        mnemonic: "xvftintrz.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x754B8000,
        mnemonic: "xvftintrne.w.d",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x757A8000,
        mnemonic: "xvshuf.h",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x757B0000,
        mnemonic: "xvshuf.w",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x757B8000,
        mnemonic: "xvshuf.d",
        layout: 130,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x757D0000,
        mnemonic: "xvperm.w",
        layout: 64,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76800000,
        mnemonic: "xvseqi.b",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76808000,
        mnemonic: "xvseqi.h",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76810000,
        mnemonic: "xvseqi.w",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76818000,
        mnemonic: "xvseqi.d",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76820000,
        mnemonic: "xvslei.b",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76828000,
        mnemonic: "xvslei.wu",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76830000,
        mnemonic: "xvslei.w",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76838000,
        mnemonic: "xvslei.d",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76840000,
        mnemonic: "xvslei.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76848000,
        mnemonic: "xvslei.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76850000,
        mnemonic: "xvslei.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76858000,
        mnemonic: "xvslei.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76860000,
        mnemonic: "xvslti.b",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76868000,
        mnemonic: "xvslti.h",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76870000,
        mnemonic: "xvaddi.bu",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76878000,
        mnemonic: "xvslti.d",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76880000,
        mnemonic: "xvslti.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76888000,
        mnemonic: "xvslti.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76890000,
        mnemonic: "xvslti.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76898000,
        mnemonic: "xvslti.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768A0000,
        mnemonic: "xvaddi.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768A8000,
        mnemonic: "xvaddi.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768B0000,
        mnemonic: "xvaddi.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768B8000,
        mnemonic: "xvaddi.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768C0000,
        mnemonic: "xvsubi.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768C8000,
        mnemonic: "xvsubi.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768D0000,
        mnemonic: "xvsubi.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768D8000,
        mnemonic: "xvsubi.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768E0000,
        mnemonic: "xvbsll.v",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x768E8000,
        mnemonic: "xvbsrl.v",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76900000,
        mnemonic: "xvmaxi.b",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76908000,
        mnemonic: "xvmaxi.h",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76910000,
        mnemonic: "xvmaxi.wu",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76918000,
        mnemonic: "xvmaxi.d",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76920000,
        mnemonic: "xvmini.b",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76928000,
        mnemonic: "xvmini.h",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76930000,
        mnemonic: "xvmini.w",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76938000,
        mnemonic: "xvmini.d",
        layout: 132,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76940000,
        mnemonic: "xvmaxi.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76948000,
        mnemonic: "xvmaxi.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76950000,
        mnemonic: "xvmaxi.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76958000,
        mnemonic: "xvmaxi.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76960000,
        mnemonic: "xvmini.bu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76968000,
        mnemonic: "xvmini.hu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76970000,
        mnemonic: "xvmini.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76978000,
        mnemonic: "xvmini.du",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x769A0000,
        mnemonic: "xvfrstpi.b",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x769A8000,
        mnemonic: "xvfrstpi.h",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C0000,
        mnemonic: "xvclo.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C0400,
        mnemonic: "xvclo.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C0800,
        mnemonic: "xvneg.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C0C00,
        mnemonic: "xvclo.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C1000,
        mnemonic: "xvclz.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C1400,
        mnemonic: "xvclz.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C1800,
        mnemonic: "xvclz.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C1C00,
        mnemonic: "xvclz.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C2000,
        mnemonic: "xvpcnt.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C2400,
        mnemonic: "xvpcnt.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C2800,
        mnemonic: "xvpcnt.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C2C00,
        mnemonic: "xvpcnt.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C3000,
        mnemonic: "xvneg.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C3400,
        mnemonic: "xvneg.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C3800,
        mnemonic: "xvneg.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C3C00,
        mnemonic: "xvneg.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C4000,
        mnemonic: "xvmskltz.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C4400,
        mnemonic: "xvmskltz.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C4800,
        mnemonic: "xvsetanyeqz.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C4C00,
        mnemonic: "xvmskltz.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C5000,
        mnemonic: "xvmskgez.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769C6000,
        mnemonic: "xvmsknz.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769C9800,
        mnemonic: "xvseteqz.v",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769C9C00,
        mnemonic: "xvsetnez.v",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CA000,
        mnemonic: "xvflogb.s",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CA400,
        mnemonic: "xvsetanyeqz.h",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CA800,
        mnemonic: "xvsetanyeqz.w",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CAC00,
        mnemonic: "xvsetanyeqz.d",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CB000,
        mnemonic: "xvsetallnez.b",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CB400,
        mnemonic: "xvsetallnez.h",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CB800,
        mnemonic: "xvsetallnez.w",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC18,
        value: 0x769CBC00,
        mnemonic: "xvsetallnez.d",
        layout: 136,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CC400,
        mnemonic: "xvflogb.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CC800,
        mnemonic: "xvflogb.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CD400,
        mnemonic: "xvfclass.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CD800,
        mnemonic: "xvfclass.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CE400,
        mnemonic: "xvfsqrt.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CE800,
        mnemonic: "xvfrintrp.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CF400,
        mnemonic: "xvfrecip.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769CF800,
        mnemonic: "xvfrecip.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D0400,
        mnemonic: "xvfrsqrt.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D0800,
        mnemonic: "xvfrsqrt.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D1400,
        mnemonic: "xvfrecipe.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D1800,
        mnemonic: "xvfrecipe.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D2400,
        mnemonic: "xvfrsqrte.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D2800,
        mnemonic: "xvfrsqrte.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D3400,
        mnemonic: "xvfrint.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D3800,
        mnemonic: "xvfrint.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D4400,
        mnemonic: "xvfrintrm.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D4800,
        mnemonic: "xvfrintrm.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D5400,
        mnemonic: "xvfrintrp.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D5800,
        mnemonic: "xvfrintrp.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D6400,
        mnemonic: "xvfrintrz.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D6800,
        mnemonic: "xvfrintrz.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D7400,
        mnemonic: "xvftintrm.l.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769D7800,
        mnemonic: "xvfrintrne.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769DE800,
        mnemonic: "xvfcvtl.s.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769DEC00,
        mnemonic: "xvfcvth.s.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769DF000,
        mnemonic: "xvfcvtl.d.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769DF400,
        mnemonic: "xvfcvth.d.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E0000,
        mnemonic: "xvffint.s.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E0400,
        mnemonic: "xvffint.s.wu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E0800,
        mnemonic: "xvffint.d.l",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E0C00,
        mnemonic: "xvffint.d.lu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E1000,
        mnemonic: "xvftinth.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E1400,
        mnemonic: "xvffinth.d.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E3000,
        mnemonic: "xvftint.w.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E3400,
        mnemonic: "xvftint.l.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E3800,
        mnemonic: "xvftintrm.w.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E3C00,
        mnemonic: "xvftintrm.l.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E4000,
        mnemonic: "xvftintrp.w.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E4400,
        mnemonic: "xvftintrp.l.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E4800,
        mnemonic: "xvftintrz.w.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E4C00,
        mnemonic: "xvexth.w.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E5000,
        mnemonic: "xvftintrne.w.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E5400,
        mnemonic: "xvftintrne.l.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E5800,
        mnemonic: "xvftint.wu.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E5C00,
        mnemonic: "xvftint.lu.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E7000,
        mnemonic: "xvftintrz.wu.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E7400,
        mnemonic: "xvftintrz.lu.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E8000,
        mnemonic: "xvftintl.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E8400,
        mnemonic: "xvftinth.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E8800,
        mnemonic: "xvreplgr2vr.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E8C00,
        mnemonic: "xvftintrmh.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E9000,
        mnemonic: "xvftintrpl.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E9400,
        mnemonic: "xvftintrph.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E9800,
        mnemonic: "xvftintrzl.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769E9C00,
        mnemonic: "xvftintrzh.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EA000,
        mnemonic: "xvftintrnel.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EA400,
        mnemonic: "xvftintrneh.l.s",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EE000,
        mnemonic: "xvexth.h.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EE400,
        mnemonic: "vext2xv.wu.hu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EE800,
        mnemonic: "xvexth.d.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EEC00,
        mnemonic: "xvexth.q.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EF000,
        mnemonic: "xvexth.hu.bu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EF400,
        mnemonic: "xvexth.wu.hu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EF800,
        mnemonic: "xvexth.du.wu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769EFC00,
        mnemonic: "xvexth.qu.du",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F0000,
        mnemonic: "xvreplgr2vr.b",
        layout: 137,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F0400,
        mnemonic: "xvreplgr2vr.h",
        layout: 137,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F0800,
        mnemonic: "xvreplgr2vr.w",
        layout: 137,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F0C00,
        mnemonic: "xvreplgr2vr.d",
        layout: 137,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F1000,
        mnemonic: "vext2xv.h.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F1400,
        mnemonic: "vext2xv.w.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F1800,
        mnemonic: "vext2xv.d.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F1C00,
        mnemonic: "vext2xv.w.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F2000,
        mnemonic: "vext2xv.d.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F2400,
        mnemonic: "vext2xv.d.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F2800,
        mnemonic: "vext2xv.hu.bu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F2C00,
        mnemonic: "vext2xv.wu.bu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F3000,
        mnemonic: "vext2xv.du.bu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F3400,
        mnemonic: "vext2xv.wu.hu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F3800,
        mnemonic: "vext2xv.du.hu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x769F3C00,
        mnemonic: "vext2xv.du.wu",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x769F8000,
        mnemonic: "xvhseli.d",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76A02000,
        mnemonic: "xvrotri.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x76A04000,
        mnemonic: "xvrotri.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76A08000,
        mnemonic: "xvrotri.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x76A10000,
        mnemonic: "xvrotri.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76A42000,
        mnemonic: "xvsrlri.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x76A44000,
        mnemonic: "xvsrlri.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76A48000,
        mnemonic: "xvsrlri.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x76A50000,
        mnemonic: "xvsrlri.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76A82000,
        mnemonic: "xvsrari.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x76A84000,
        mnemonic: "xvsrari.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x76A88000,
        mnemonic: "xvsrari.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x76A90000,
        mnemonic: "xvsrari.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76EBC000,
        mnemonic: "xvinsgr2vr.w",
        layout: 141,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x76EBE000,
        mnemonic: "xvinsgr2vr.d",
        layout: 142,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76EFC000,
        mnemonic: "xvpickve2gr.w",
        layout: 143,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x76EFE000,
        mnemonic: "xvpickve2gr.d",
        layout: 144,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76F3C000,
        mnemonic: "xvpickve2gr.wu",
        layout: 143,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x76F3E000,
        mnemonic: "xvpickve2gr.du",
        layout: 144,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x76F78000,
        mnemonic: "xvrepl128vei.b",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76F7C000,
        mnemonic: "xvrepl128vei.h",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x76F7E000,
        mnemonic: "xvrepl128vei.w",
        layout: 145,
    },
    LoongArchPattern {
        mask: 0xFFFFF800,
        value: 0x76F7F000,
        mnemonic: "xvrepl128vei.d",
        layout: 146,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x76FFC000,
        mnemonic: "xvinsve0.w",
        layout: 147,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x76FFE000,
        mnemonic: "xvinsve0.d",
        layout: 148,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x7703C000,
        mnemonic: "xvpickve.w",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFF000,
        value: 0x7703E000,
        mnemonic: "xvpickve.d",
        layout: 145,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x77070000,
        mnemonic: "xvreplve0.b",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x77078000,
        mnemonic: "xvreplve0.h",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x7707C000,
        mnemonic: "xvreplve0.w",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x7707E000,
        mnemonic: "xvreplve0.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x7707F000,
        mnemonic: "xvreplve0.q",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77082000,
        mnemonic: "xvsllwil.h.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77084000,
        mnemonic: "xvsllwil.w.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77088000,
        mnemonic: "xvsllwil.d.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x77090000,
        mnemonic: "xvextl.q.d",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x770C2000,
        mnemonic: "xvsllwil.hu.bu",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x770C4000,
        mnemonic: "xvsllwil.wu.hu",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x770C8000,
        mnemonic: "xvsllwil.du.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFFFC00,
        value: 0x770D0000,
        mnemonic: "xvextl.qu.du",
        layout: 135,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77102000,
        mnemonic: "xvbitclri.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77104000,
        mnemonic: "xvbitclri.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77108000,
        mnemonic: "xvbitclri.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77110000,
        mnemonic: "xvsat.hu",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77142000,
        mnemonic: "xvbitseti.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77144000,
        mnemonic: "xvbitseti.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77148000,
        mnemonic: "xvbitseti.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77150000,
        mnemonic: "xvbitseti.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77182000,
        mnemonic: "xvbitrevi.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77184000,
        mnemonic: "xvbitrevi.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77188000,
        mnemonic: "xvbitrevi.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77190000,
        mnemonic: "xvbitrevi.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77242000,
        mnemonic: "xvsat.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77244000,
        mnemonic: "xvsat.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77248000,
        mnemonic: "xvsat.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77250000,
        mnemonic: "xvsat.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77282000,
        mnemonic: "xvsat.bu",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77284000,
        mnemonic: "xvsat.hu",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77288000,
        mnemonic: "xvsat.wu",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77290000,
        mnemonic: "xvsat.du",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x772C2000,
        mnemonic: "xvslli.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x772C4000,
        mnemonic: "xvsrlrni.h.w",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x772C8000,
        mnemonic: "xvslli.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x772D0000,
        mnemonic: "xvslli.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77302000,
        mnemonic: "xvsrli.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77304000,
        mnemonic: "xvsrli.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77308000,
        mnemonic: "xvsrli.w",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77310000,
        mnemonic: "xvsrli.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFE000,
        value: 0x77342000,
        mnemonic: "xvsrai.b",
        layout: 138,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77344000,
        mnemonic: "xvsrai.h",
        layout: 139,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77348000,
        mnemonic: "xvssrlni.du.q",
        layout: 133,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77350000,
        mnemonic: "xvsrai.d",
        layout: 140,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77404000,
        mnemonic: "xvsrlni.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77408000,
        mnemonic: "xvsrlni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77410000,
        mnemonic: "xvsrlni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77420000,
        mnemonic: "xvsrlni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77444000,
        mnemonic: "xvsrlrni.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77448000,
        mnemonic: "xvsrlrni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77450000,
        mnemonic: "xvsrlrni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77460000,
        mnemonic: "xvsrlrni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77484000,
        mnemonic: "xvssrlni.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77488000,
        mnemonic: "xvssrlni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77490000,
        mnemonic: "xvssrlni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x774A0000,
        mnemonic: "xvssrlni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x774C4000,
        mnemonic: "xvssrlni.bu.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x774C8000,
        mnemonic: "xvssrlni.hu.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x774D0000,
        mnemonic: "xvssrlni.wu.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x774E0000,
        mnemonic: "xvssrlni.du.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77504000,
        mnemonic: "xvssrani.d.q",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77508000,
        mnemonic: "xvssrlrni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77510000,
        mnemonic: "xvssrlrni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77520000,
        mnemonic: "xvssrlrni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77544000,
        mnemonic: "xvssrlrni.bu.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77548000,
        mnemonic: "xvssrlrni.hu.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77550000,
        mnemonic: "xvssrlrni.wu.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77560000,
        mnemonic: "xvssrlrni.du.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77584000,
        mnemonic: "xvsrani.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77588000,
        mnemonic: "xvsrani.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77590000,
        mnemonic: "xvsrani.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x775A0000,
        mnemonic: "xvsrani.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x775C4000,
        mnemonic: "xvsrarni.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x775C8000,
        mnemonic: "xvsrarni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x775D0000,
        mnemonic: "xvsrarni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x775E0000,
        mnemonic: "xvsrarni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77604000,
        mnemonic: "xvssrani.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77608000,
        mnemonic: "xvssrani.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77610000,
        mnemonic: "xvssrani.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77620000,
        mnemonic: "xvssrani.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77644000,
        mnemonic: "xvssrani.bu.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77648000,
        mnemonic: "xvssrani.hu.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77650000,
        mnemonic: "xvssrani.wu.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x77660000,
        mnemonic: "xvssrani.du.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x77684000,
        mnemonic: "xvssrarni.b.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x77688000,
        mnemonic: "xvssrarni.h.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x77690000,
        mnemonic: "xvssrarni.w.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x776A0000,
        mnemonic: "xvssrarni.d.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFFC000,
        value: 0x776C4000,
        mnemonic: "xvssrarni.bu.h",
        layout: 149,
    },
    LoongArchPattern {
        mask: 0xFFFF8000,
        value: 0x776C8000,
        mnemonic: "xvssrarni.hu.w",
        layout: 134,
    },
    LoongArchPattern {
        mask: 0xFFFF0000,
        value: 0x776D0000,
        mnemonic: "xvssrarni.wu.d",
        layout: 150,
    },
    LoongArchPattern {
        mask: 0xFFFE0000,
        value: 0x776E0000,
        mnemonic: "xvssrarni.du.q",
        layout: 151,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77800000,
        mnemonic: "xvextrins.d",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77840000,
        mnemonic: "xvextrins.w",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77880000,
        mnemonic: "xvextrins.h",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x778C0000,
        mnemonic: "xvextrins.b",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77900000,
        mnemonic: "xvshuf4i.b",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77940000,
        mnemonic: "xvshuf4i.h",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77980000,
        mnemonic: "xvshuf4i.w",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x779C0000,
        mnemonic: "xvshuf4i.d",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77C40000,
        mnemonic: "xvbitseli.b",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77D00000,
        mnemonic: "xvandi.b",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77D40000,
        mnemonic: "xvori.b",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77D80000,
        mnemonic: "xvxori.b",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77DC0000,
        mnemonic: "xvnori.b",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77E00000,
        mnemonic: "xvldi",
        layout: 154,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77E40000,
        mnemonic: "xvpermi.w",
        layout: 152,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77E80000,
        mnemonic: "xvpermi.d",
        layout: 153,
    },
    LoongArchPattern {
        mask: 0xFFFC0000,
        value: 0x77EC0000,
        mnemonic: "xvpermi.q",
        layout: 152,
    },
];

/// Try to decode a LoongArch instruction from a pattern match.
pub fn try_decode_from_patterns(
    word: u32,
    addr: u64,
) -> Option<Result<DecodedInstruction, DisasmError>> {
    for pattern in LOONGARCH_PATTERNS {
        if (word & pattern.mask) == pattern.value {
            return Some(decode_from_pattern(word, pattern, addr));
        }
    }
    None
}

fn decode_from_pattern(
    word: u32,
    pattern: &LoongArchPattern,
    addr: u64,
) -> Result<DecodedInstruction, DisasmError> {
    let operands = match pattern.layout {
        0 => extract_case_0(word),
        1 => extract_case_1(word),
        2 => extract_case_2(word),
        3 => extract_case_3(word),
        4 => extract_case_4(word),
        5 => extract_case_5(word),
        6 => extract_case_6(word),
        7 => extract_case_7(word),
        8 => extract_case_8(word),
        9 => extract_case_9(word),
        10 => extract_case_10(word),
        11 => extract_case_11(word),
        12 => extract_case_12(word),
        13 => extract_case_13(word),
        14 => extract_case_14(word),
        15 => extract_case_15(word),
        16 => extract_case_16(word),
        17 => extract_case_17(word),
        18 => extract_case_18(word),
        19 => extract_case_19(word),
        20 => extract_case_20(word),
        21 => extract_case_21(word),
        22 => extract_case_22(word),
        23 => extract_case_23(word),
        24 => extract_case_24(word),
        25 => extract_case_25(word),
        26 => extract_case_26(word),
        27 => extract_case_27(word),
        28 => extract_case_28(word),
        29 => extract_case_29(word),
        30 => extract_case_30(word),
        31 => extract_case_31(word),
        32 => extract_case_32(word),
        33 => extract_case_33(word),
        34 => extract_case_34(word),
        35 => extract_case_35(word),
        36 => extract_case_36(word),
        37 => extract_case_37(word),
        38 => extract_case_38(word),
        39 => extract_case_39(word),
        40 => extract_case_40(word),
        41 => extract_case_41(word),
        42 => extract_case_42(word),
        43 => extract_case_43(word),
        44 => extract_case_44(word),
        45 => extract_case_45(word),
        46 => extract_case_46(word),
        47 => extract_case_47(word),
        48 => extract_case_48(word),
        49 => extract_case_49(word),
        50 => extract_case_50(word),
        51 => extract_case_51(word),
        52 => extract_case_52(word),
        53 => extract_case_53(word),
        54 => extract_case_54(word),
        55 => extract_case_55(word),
        56 => extract_case_56(word),
        57 => extract_case_57(word),
        58 => extract_case_58(word),
        59 => extract_case_59(word),
        60 => extract_case_60(word),
        61 => extract_case_61(word),
        62 => extract_case_62(word),
        63 => extract_case_63(word),
        64 => extract_case_64(word),
        65 => extract_case_65(word),
        66 => extract_case_66(word),
        67 => extract_case_67(word),
        68 => extract_case_68(word),
        69 => extract_case_69(word),
        70 => extract_case_70(word),
        71 => extract_case_71(word),
        72 => extract_case_72(word),
        73 => extract_case_73(word),
        74 => extract_case_74(word),
        75 => extract_case_75(word),
        76 => extract_case_76(word),
        77 => extract_case_77(word),
        78 => extract_case_78(word),
        79 => extract_case_79(word),
        80 => extract_case_80(word),
        81 => extract_case_81(word),
        82 => extract_case_82(word),
        83 => extract_case_83(word),
        84 => extract_case_84(word),
        85 => extract_case_85(word),
        86 => extract_case_86(word),
        87 => extract_case_87(word),
        88 => extract_case_88(word),
        89 => extract_case_89(word),
        90 => extract_case_90(word),
        91 => extract_case_91(word),
        92 => extract_case_92(word),
        93 => extract_case_93(word),
        94 => extract_case_94(word),
        95 => extract_case_95(word),
        96 => extract_case_96(word),
        97 => extract_case_97(word),
        98 => extract_case_98(word),
        99 => extract_case_99(word),
        100 => extract_case_100(word),
        101 => extract_case_101(word),
        102 => extract_case_102(word),
        103 => extract_case_103(word),
        104 => extract_case_104(word),
        105 => extract_case_105(word),
        106 => extract_case_106(word),
        107 => extract_case_107(word),
        108 => extract_case_108(word),
        109 => extract_case_109(word),
        110 => extract_case_110(word),
        111 => extract_case_111(word),
        112 => extract_case_112(word),
        113 => extract_case_113(word),
        114 => extract_case_114(word),
        115 => extract_case_115(word),
        116 => extract_case_116(word),
        117 => extract_case_117(word),
        118 => extract_case_118(word),
        119 => extract_case_119(word),
        120 => extract_case_120(word),
        121 => extract_case_121(word),
        122 => extract_case_122(word),
        123 => extract_case_123(word),
        124 => extract_case_124(word),
        125 => extract_case_125(word),
        126 => extract_case_126(word),
        127 => extract_case_127(word),
        128 => extract_case_128(word),
        129 => extract_case_129(word),
        130 => extract_case_130(word),
        131 => extract_case_131(word),
        132 => extract_case_132(word),
        133 => extract_case_133(word),
        134 => extract_case_134(word),
        135 => extract_case_135(word),
        136 => extract_case_136(word),
        137 => extract_case_137(word),
        138 => extract_case_138(word),
        139 => extract_case_139(word),
        140 => extract_case_140(word),
        141 => extract_case_141(word),
        142 => extract_case_142(word),
        143 => extract_case_143(word),
        144 => extract_case_144(word),
        145 => extract_case_145(word),
        146 => extract_case_146(word),
        147 => extract_case_147(word),
        148 => extract_case_148(word),
        149 => extract_case_149(word),
        150 => extract_case_150(word),
        151 => extract_case_151(word),
        152 => extract_case_152(word),
        153 => extract_case_153(word),
        154 => extract_case_154(word),
        _ => {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::InvalidEncoding,
                architecture: Some("loongarch64".to_string()),
                detail: format!("unknown layout {} for LoongArch", pattern.layout),
            });
        }
    };

    Ok(DecodedInstruction {
        architecture: ArchitectureId::LoongArch,
        address: addr,
        mode: "LA64".to_string(),
        mnemonic: pattern.mnemonic.to_string(),
        opcode_id: None,
        size: 4,
        raw_bytes: word.to_le_bytes().to_vec(),
        operands,
        registers_read: Vec::new(),
        registers_written: Vec::new(),
        implicit_registers_read: Vec::new(),
        implicit_registers_written: Vec::new(),
        groups: Vec::new(),
        status: DecodeStatus::Success,
        render_hints: RenderHints::default(),
    })
}

fn sign_extend(value: u32, bit_width: u8) -> i64 {
    let mask = 1u32 << (bit_width - 1);
    if value & mask != 0 {
        let sign_extended = value | !((1u32 << bit_width) - 1);
        sign_extended as i32 as i64
    } else {
        value as i64
    }
}
