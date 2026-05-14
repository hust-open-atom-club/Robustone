//! AArch64 SIMD/FP data processing instructions.
//!
//! Covers scalar FP data processing (op1[4] = 1) and Advanced SIMD (vector)
//! data processing (op1[4] = 0) within the SIMD/FP major opcode group
//! (op0 = 0x7 / 0xF).

use crate::extensions::DecodeResult;
use crate::shared::encoding::*;
use crate::shared::registers::{arrangement_suffix, fp_simd_reg_name, FpRegSize};
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
        scalar::decode_scalar_fp(word)
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
            return vector::decode_simd_modified_imm(word);
        } else {
            return vector::decode_simd_shift_imm(word);
        }
    }

    // Indexed Element: b24=1, b10=0
    if b24 && !b10 {
        return vector::decode_simd_indexed_element(word);
    }

    // Three Same (integer + FP32/64): b21=1, b10=1
    if b21 && b10 {
        return vector::decode_simd_three_same(word);
    }

    // b21=0, b10=1: either FP16 Three Same or Copy
    if !b21 && b10 {
        if b22 {
            return vector::decode_simd_fp16_three_same(word);
        } else {
            return vector::decode_simd_copy(word);
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
            return vector::decode_crypto_aes(word);
        }

        // Across Lanes: specific opcodes
        if let Some(result) = vector::try_decode_simd_across_lanes(word, op5_16) {
            return Ok(result);
        }

        // CRITICAL FIX: Check bit 11 to distinguish Three Different from Two-reg Misc.
        // bit 11 = 0: Three Different (opcode in bits 15:12, 4 bits)
        // bit 11 = 1: Two-register Misc (opcode in bits 16:12, 5 bits)
        if !b11 {
            return vector::decode_simd_three_different(word);
        }

        // Two-register Misc (b11=1)
        return vector::decode_simd_two_reg_misc(word, op5_16);
    }

    // b21=0, b10=0: Permute / Extract / Table
    if !b21 && !b10 {
        return vector::decode_simd_permute_table(word);
    }

    Err(DisasmError::decode_failure(
        DecodeErrorKind::UnimplementedInstruction,
        Some("aarch64".to_string()),
        "Unrecognized Advanced SIMD encoding",
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
mod scalar;
mod vector;


// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn vec_reg_operand(reg: u8, size: u8, q: bool) -> Operand {
    let suffix = arrangement_suffix(size, q).unwrap_or("?");
    Operand::Text {
        value: format!("v{}{}", reg, suffix),
    }
}

/// Create a register operand for an FP/SIMD register.
fn fp_reg_operand(reg: u8, size: FpRegSize) -> Operand {
    Operand::Text {
        value: fp_simd_reg_name(reg, size).unwrap_or("?").to_string(),
    }
}

/// Create a vector register operand with arrangement suffix.

///
/// Returns (element_char, index, arrangement_size) where arrangement_size
/// is 0=B, 1=H, 2=S, 3=D for use with `arrangement_suffix`.

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
