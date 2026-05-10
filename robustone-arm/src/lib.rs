//! ARM (AArch64) disassembly module for Robustone.

pub mod arch;
pub mod decoder;
pub mod extensions;
pub mod printer;
pub mod render;
pub mod shared;
pub mod types;

pub use arch::AArch64Handler;

#[cfg(test)]
mod tests {
    use super::*;
    use robustone_core::traits::ArchitectureHandler;

    fn disasm(bytes: &[u8]) -> (String, String) {
        let handler = AArch64Handler::new();
        let (instr, _) = handler.disassemble(bytes, "aarch64", 0).unwrap();
        (instr.mnemonic, instr.operands)
    }

    #[test]
    fn test_nop() {
        let (mnemonic, ops) = disasm(&[0x1F, 0x20, 0x03, 0xD5]);
        assert_eq!(mnemonic, "nop");
        assert_eq!(ops, "");
    }

    #[test]
    fn test_add_imm() {
        // add x0, x1, #2  (Data Processing — Immediate, add/sub)
        let (mnemonic, ops) = disasm(&[0x20, 0x08, 0x00, 0x91]);
        assert_eq!(mnemonic, "add");
        assert_eq!(ops, "x0, x1, #2");
    }

    #[test]
    fn test_sub_imm() {
        // sub x0, x1, #2  (Data Processing — Immediate, add/sub)
        let (mnemonic, ops) = disasm(&[0x20, 0x08, 0x00, 0xD1]);
        assert_eq!(mnemonic, "sub");
        assert_eq!(ops, "x0, x1, #2");
    }

    #[test]
    fn test_cmp_alias() {
        // cmp x1, #2 = subs xzr, x1, #2  (Data Processing — Immediate, add/sub)
        let (mnemonic, ops) = disasm(&[0x3F, 0x08, 0x00, 0xF1]);
        assert_eq!(mnemonic, "cmp");
        assert_eq!(ops, "x1, #2");
    }

    #[test]
    fn test_movz() {
        // movz x0, #0x1234  (Data Processing — Immediate, move wide)
        let (mnemonic, ops) = disasm(&[0x80, 0x46, 0x82, 0xD2]);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, "x0, #0x1234");
    }

    #[test]
    fn test_ret() {
        let (mnemonic, ops) = disasm(&[0xC0, 0x03, 0x5F, 0xD6]);
        assert_eq!(mnemonic, "ret");
        assert_eq!(ops, "");
    }

    #[test]
    fn test_b_cond() {
        let (mnemonic, ops) = disasm(&[0x00, 0x04, 0x00, 0x54]);
        assert_eq!(mnemonic, "b.eq");
        assert!(ops.contains("0x"));
    }

    #[test]
    fn test_cbz() {
        let (mnemonic, ops) = disasm(&[0x00, 0x00, 0x5F, 0xB4]);
        assert_eq!(mnemonic, "cbz");
        assert!(ops.starts_with("x0"));
    }

    #[test]
    fn test_mov_alias_orr() {
        // mov x0, x2 = orr x0, xzr, x2
        let (mnemonic, ops) = disasm(&[0x40, 0x00, 0x1F, 0xAA]);
        assert_eq!(mnemonic, "mov");
        assert_eq!(ops, "x0, x2");
    }

    #[test]
    fn test_neg_alias() {
        // neg x0, x1 = sub x0, xzr, x1 (shifted register)
        let (mnemonic, ops) = disasm(&[0xE0, 0x03, 0x01, 0xCB]);
        assert_eq!(mnemonic, "neg");
        assert_eq!(ops, "x0, x1");
    }

    #[test]
    fn test_str_post_index() {
        // str w0, [x1], #0  (post-indexed store)
        let (mnemonic, ops) = disasm(&[0x20, 0x04, 0x00, 0xb8]);
        assert_eq!(mnemonic, "str");
        assert_eq!(ops, "w0, [x1], #0");
    }

    #[test]
    fn test_stxr() {
        // stxr w0, w0, [x1]  (single exclusive store, word)
        let (mnemonic, ops) = disasm(&[0x20, 0x00, 0x00, 0x88]);
        assert_eq!(mnemonic, "stxr");
        assert_eq!(ops, "w0, w0, [x1]");
    }

    #[test]
    fn test_stxp() {
        // stxp w0, w0, w0, [x1]  (pair exclusive store, word)
        let (mnemonic, ops) = disasm(&[0x20, 0x00, 0x20, 0x88]);
        assert_eq!(mnemonic, "stxp");
        assert_eq!(ops, "w0, w0, w0, [x1]");
    }

    #[test]
    fn test_stp() {
        // stp x0, x0, [x1]  (pair store, 64-bit)
        let (mnemonic, ops) = disasm(&[0x20, 0x00, 0x00, 0xa9]);
        assert_eq!(mnemonic, "stp");
        assert_eq!(ops, "x0, x0, [x1]");
    }

    #[test]
    fn test_ldr_literal() {
        // ldr w0, 4  (load literal, PC+4)
        let (mnemonic, ops) = disasm(&[0x20, 0x00, 0x00, 0x18]);
        assert_eq!(mnemonic, "ldr");
        assert_eq!(ops, "w0, 4");
    }

    #[test]
    fn test_str_unsigned_imm() {
        // str w0, [x1, #4]  (unsigned immediate)
        let (mnemonic, ops) = disasm(&[0x20, 0x04, 0x00, 0xb9]);
        assert_eq!(mnemonic, "str");
        assert_eq!(ops, "w0, [x1, #4]");
    }

    #[test]
    fn test_str_register_offset() {
        // str x0, [x1, x2]  (register offset)
        let (mnemonic, ops) = disasm(&[0x20, 0x68, 0x22, 0xf8]);
        assert_eq!(mnemonic, "str");
        assert_eq!(ops, "x0, [x1, x2]");
    }

    #[test]
    fn test_stur() {
        // stur w0, [x1, #0xf]  (unscaled immediate)
        let (mnemonic, ops) = disasm(&[0x20, 0xf0, 0x00, 0xb8]);
        assert_eq!(mnemonic, "stur");
        assert_eq!(ops, "w0, [x1, #0xf]");
    }

    #[test]
    fn test_stur_zero_offset() {
        // stur w0, [x1]  (unscaled immediate, offset=0)
        let (mnemonic, ops) = disasm(&[0x20, 0x00, 0x00, 0xb8]);
        assert_eq!(mnemonic, "stur");
        assert_eq!(ops, "w0, [x1]");
    }

    #[test]
    fn test_ldp_post_index() {
        // ldp x0, x0, [x2], #0x10  (pair post-indexed)
        let (mnemonic, ops) = disasm(&[0x40, 0x00, 0xc1, 0xa8]);
        assert_eq!(mnemonic, "ldp");
        assert_eq!(ops, "x0, x0, [x2], #0x10");
    }

    #[test]
    fn test_ldp_pre_index() {
        // ldp x0, x0, [x2, #16]!  (pair pre-indexed)
        let (mnemonic, ops) = disasm(&[0x40, 0x00, 0xc1, 0xa9]);
        assert_eq!(mnemonic, "ldp");
        assert_eq!(ops, "x0, x0, [x2, #0x10]!");
    }

    // Stage 3: Scalar FP
    #[test]
    fn test_fadd_s() {
        // fadd s1, s2, s3
        let (mnemonic, ops) = disasm(&[0x41, 0x28, 0x23, 0x1e]);
        assert_eq!(mnemonic, "fadd");
        assert_eq!(ops, "s1, s2, s3");
    }

    #[test]
    fn test_fsub_s() {
        // fsub s1, s2, s3
        let (mnemonic, ops) = disasm(&[0x41, 0x38, 0x23, 0x1e]);
        assert_eq!(mnemonic, "fsub");
        assert_eq!(ops, "s1, s2, s3");
    }

    #[test]
    fn test_fmul_s() {
        // fmul s1, s2, s3
        let (mnemonic, ops) = disasm(&[0x41, 0x08, 0x23, 0x1e]);
        assert_eq!(mnemonic, "fmul");
        assert_eq!(ops, "s1, s2, s3");
    }

    #[test]
    fn test_fdiv_s() {
        // fdiv s1, s2, s3
        let (mnemonic, ops) = disasm(&[0x41, 0x18, 0x23, 0x1e]);
        assert_eq!(mnemonic, "fdiv");
        assert_eq!(ops, "s1, s2, s3");
    }

    #[test]
    fn test_fmadd_s() {
        // fmadd s1, s2, s3, s4
        let (mnemonic, ops) = disasm(&[0x41, 0x10, 0x03, 0x1f]);
        assert_eq!(mnemonic, "fmadd");
        assert_eq!(ops, "s1, s2, s3, s4");
    }

    #[test]
    fn test_fmov_reg_s() {
        // fmov s1, s2
        let (mnemonic, ops) = disasm(&[0x41, 0x00, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fmov");
        assert_eq!(ops, "s1, s2");
    }

    #[test]
    fn test_fcmp_s() {
        // fcmp s1, s3
        let (mnemonic, ops) = disasm(&[0x20, 0x20, 0x03, 0x1e]);
        assert_eq!(mnemonic, "fcmp");
        assert_eq!(ops, "s1, s3");
    }

    #[test]
    fn test_fcmp_zero() {
        // fcmp s1, #0.0
        let (mnemonic, ops) = disasm(&[0x20, 0x20, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fcmp");
        assert_eq!(ops, "s1, #0.00000000");
    }

    #[test]
    fn test_fabs_s() {
        // fabs s1, s2
        let (mnemonic, ops) = disasm(&[0x41, 0x04, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fabs");
        assert_eq!(ops, "s1, s2");
    }

    #[test]
    fn test_fneg_s() {
        // fneg s1, s2
        let (mnemonic, ops) = disasm(&[0x41, 0x08, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fneg");
        assert_eq!(ops, "s1, s2");
    }

    #[test]
    fn test_fsqrt_s() {
        // fsqrt s1, s2
        let (mnemonic, ops) = disasm(&[0x41, 0x0c, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fsqrt");
        assert_eq!(ops, "s1, s2");
    }

    #[test]
    fn test_fmov_imm_s() {
        // fmov s1, #0.125
        let (mnemonic, ops) = disasm(&[0x01, 0x30, 0x00, 0x1e]);
        assert_eq!(mnemonic, "fmov");
        assert!(ops.starts_with("s1, #"));
    }

    #[test]
    fn test_fcvt_sd() {
        // fcvt s1, d2
        let (mnemonic, ops) = disasm(&[0x41, 0x08, 0x01, 0x1e]);
        assert_eq!(mnemonic, "fcvt");
        assert_eq!(ops, "s1, d2");
    }

    #[test]
    fn test_scvtf_s() {
        // scvtf s1, w2
        let (mnemonic, ops) = disasm(&[0x41, 0x00, 0x02, 0x1e]);
        assert_eq!(mnemonic, "scvtf");
        assert_eq!(ops, "s1, w2");
    }

    #[test]
    fn test_frinta_s() {
        // frinta s1, s2
        let (mnemonic, ops) = disasm(&[0x41, 0x30, 0x00, 0x1e]);
        assert_eq!(mnemonic, "frinta");
        assert_eq!(ops, "s1, s2");
    }

    #[test]
    fn test_fcsel_s() {
        // fcsel s1, s2, s3, eq
        let (mnemonic, ops) = disasm(&[0x41, 0x0c, 0x03, 0x1e]);
        assert_eq!(mnemonic, "fcsel");
        assert_eq!(ops, "s1, s2, s3, eq");
    }

    // SIMD/FP Loads/Stores — Stage 3A

    #[test]
    fn test_ld1_structure_one_reg() {
        // ld1 {v0.8b}, [x1]
        let (mnemonic, ops) = disasm(&[0x20, 0x70, 0x40, 0x0c]);
        assert_eq!(mnemonic, "ld1");
        assert_eq!(ops, "{ v0.8b }, [x1]");
    }

    #[test]
    fn test_st1_structure_one_reg() {
        // st1 {v0.8b}, [x1]
        let (mnemonic, ops) = disasm(&[0x20, 0x70, 0x00, 0x0c]);
        assert_eq!(mnemonic, "st1");
        assert_eq!(ops, "{ v0.8b }, [x1]");
    }

    #[test]
    fn test_ld1_structure_four_reg() {
        // ld1 {v0.2s, v1.2s, v2.2s, v3.2s}, [x1]
        let (mnemonic, ops) = disasm(&[0x20, 0x28, 0x40, 0x0c]);
        assert_eq!(mnemonic, "ld1");
        assert_eq!(ops, "{ v0.2s, v1.2s, v2.2s, v3.2s }, [x1]");
    }

    #[test]
    fn test_ld1_structure_16b() {
        // ld1 {v0.16b}, [x1]
        let (mnemonic, ops) = disasm(&[0x20, 0x70, 0x40, 0x4c]);
        assert_eq!(mnemonic, "ld1");
        assert_eq!(ops, "{ v0.16b }, [x1]");
    }

    #[test]
    fn test_ldr_literal_fp() {
        // ldr s0, 8
        let (mnemonic, ops) = disasm(&[0x40, 0x00, 0x00, 0x1c]);
        assert_eq!(mnemonic, "ldr");
        assert_eq!(ops, "s0, 8");
    }

    #[test]
    fn test_str_fp_unsigned_imm() {
        // str s7, [sp, #4]
        let (mnemonic, ops) = disasm(&[0xe7, 0x07, 0x00, 0xbd]);
        assert_eq!(mnemonic, "str");
        assert_eq!(ops, "s7, [sp, #4]");
    }

    #[test]
    fn test_ld2_structure() {
        // ld2 {v4.8b, v5.8b}, [x19]
        let (mnemonic, ops) = disasm(&[0x64, 0x82, 0x40, 0x0c]);
        assert_eq!(mnemonic, "ld2");
        assert_eq!(ops, "{ v4.8b, v5.8b }, [x19]");
    }

    #[test]
    fn test_ld3_structure() {
        // ld3 {v4.8b, v5.8b, v6.8b}, [x19]
        let (mnemonic, ops) = disasm(&[0x64, 0x42, 0x40, 0x0c]);
        assert_eq!(mnemonic, "ld3");
        assert_eq!(ops, "{ v4.8b, v5.8b, v6.8b }, [x19]");
    }

    #[test]
    fn test_ld4_structure() {
        // ld4 {v4.8b, v5.8b, v6.8b, v7.8b}, [x19]
        let (mnemonic, ops) = disasm(&[0x64, 0x02, 0x40, 0x0c]);
        assert_eq!(mnemonic, "ld4");
        assert_eq!(ops, "{ v4.8b, v5.8b, v6.8b, v7.8b }, [x19]");
    }

    #[test]
    fn test_st2_structure() {
        // st2 {v4.8b, v5.8b}, [x19]
        let (mnemonic, ops) = disasm(&[0x64, 0x82, 0x00, 0x0c]);
        assert_eq!(mnemonic, "st2");
        assert_eq!(ops, "{ v4.8b, v5.8b }, [x19]");
    }

    #[test]
    fn test_ldr_fp_d_unsigned_imm() {
        // ldr d8, [sp, #8]
        let (mnemonic, ops) = disasm(&[0xe8, 0x07, 0x40, 0xfd]);
        assert_eq!(mnemonic, "ldr");
        assert_eq!(ops, "d8, [sp, #8]");
    }

    #[test]
    fn test_ldp_fp_q() {
        // ldp q2, q3, [x0, #32]
        let (mnemonic, ops) = disasm(&[0x02, 0x0c, 0x41, 0xad]);
        assert_eq!(mnemonic, "ldp");
        assert_eq!(ops, "q2, q3, [x0, #0x20]");
    }

    #[test]
    fn test_ldp_fp_s() {
        // ldp s1, s0, [x2]
        let (mnemonic, ops) = disasm(&[0x41, 0x00, 0x40, 0x2d]);
        assert_eq!(mnemonic, "ldp");
        assert!(ops.starts_with("s1, s0"));
    }

    #[test]
    fn test_ldp_fp_d() {
        // ldp d1, d2, [x2]
        let (mnemonic, ops) = disasm(&[0x41, 0x08, 0x40, 0x6d]);
        assert_eq!(mnemonic, "ldp");
        assert!(ops.starts_with("d1, d2"));
    }

    // -----------------------------------------------------------------------
    // SIMD indexed element — Stage 3C
    // -----------------------------------------------------------------------

    #[test]
    fn test_mla_indexed_s() {
        // mla v0.2s, v1.2s, v2.s[2]
        let (mnemonic, ops) = disasm(&[0x20, 0x08, 0x82, 0x2f]);
        assert_eq!(mnemonic, "mla");
        assert_eq!(ops, "v0.2s, v1.2s, v2.s[2]");
    }

    #[test]
    fn test_mla_indexed_h() {
        // mla v0.8h, v1.8h, v2.h[7]
        let (mnemonic, ops) = disasm(&[0x20, 0x08, 0x72, 0x6f]);
        assert_eq!(mnemonic, "mla");
        assert_eq!(ops, "v0.8h, v1.8h, v2.h[7]");
    }

    #[test]
    fn test_fmla_indexed_s() {
        // fmla v0.2s, v1.2s, v2.s[2]
        let (mnemonic, ops) = disasm(&[0x20, 0x18, 0x82, 0x0f]);
        assert_eq!(mnemonic, "fmla");
        assert_eq!(ops, "v0.2s, v1.2s, v2.s[2]");
    }

    #[test]
    fn test_fmla_indexed_d() {
        // fmla v0.2d, v1.2d, v2.d[1]
        let (mnemonic, ops) = disasm(&[0x20, 0x18, 0xc2, 0x4f]);
        assert_eq!(mnemonic, "fmla");
        assert_eq!(ops, "v0.2d, v1.2d, v2.d[1]");
    }

    #[test]
    fn test_sqdmull_indexed() {
        // sqdmull v0.4s, v1.4h, v2.h[2]
        let (mnemonic, ops) = disasm(&[0x20, 0xb0, 0x62, 0x0f]);
        assert_eq!(mnemonic, "sqdmull");
        assert_eq!(ops, "v0.4s, v1.4h, v2.h[2]");
    }

    #[test]
    fn test_sqdmull2_indexed() {
        // sqdmull2 v0.4s, v1.8h, v2.h[2]
        let (mnemonic, ops) = disasm(&[0x20, 0xb0, 0x62, 0x4f]);
        assert_eq!(mnemonic, "sqdmull2");
        assert_eq!(ops, "v0.4s, v1.8h, v2.h[2]");
    }

    #[test]
    fn test_sqdmulh_indexed() {
        // sqdmulh v0.4h, v1.4h, v2.h[2]
        let (mnemonic, ops) = disasm(&[0x20, 0xc0, 0x62, 0x0f]);
        assert_eq!(mnemonic, "sqdmulh");
        assert_eq!(ops, "v0.4h, v1.4h, v2.h[2]");
    }

    #[test]
    fn test_sqrdmulh_indexed() {
        // sqrdmulh v0.4h, v1.4h, v2.h[2]
        let (mnemonic, ops) = disasm(&[0x20, 0xc0, 0x62, 0x2f]);
        assert_eq!(mnemonic, "sqrdmulh");
        assert_eq!(ops, "v0.4h, v1.4h, v2.h[2]");
    }

    #[test]
    fn test_fmlal_indexed() {
        // fmlal v0.2s, v1.2h, v2.h[4]
        let (mnemonic, ops) = disasm(&[0x20, 0x08, 0x82, 0x0f]);
        assert_eq!(mnemonic, "fmlal");
        assert_eq!(ops, "v0.2s, v1.2h, v2.h[4]");
    }

    #[test]
    fn test_fmlal2_indexed() {
        // fmlal2 v0.2s, v1.2h, v2.h[4]
        let (mnemonic, ops) = disasm(&[0x20, 0x88, 0x82, 0x2f]);
        assert_eq!(mnemonic, "fmlal2");
        assert_eq!(ops, "v0.2s, v1.2h, v2.h[4]");
    }

    #[test]
    fn test_fmlsl_indexed() {
        // fmlsl v0.2s, v1.2h, v2.h[4]
        let (mnemonic, ops) = disasm(&[0x20, 0x48, 0x82, 0x0f]);
        assert_eq!(mnemonic, "fmlsl");
        assert_eq!(ops, "v0.2s, v1.2h, v2.h[4]");
    }

    #[test]
    fn test_fmlsl2_indexed() {
        // fmlsl2 v0.2s, v1.2h, v2.h[4]
        let (mnemonic, ops) = disasm(&[0x20, 0xc8, 0x82, 0x2f]);
        assert_eq!(mnemonic, "fmlsl2");
        assert_eq!(ops, "v0.2s, v1.2h, v2.h[4]");
    }

    // -----------------------------------------------------------------------
    // SIMD modified immediate — Stage 3C
    // -----------------------------------------------------------------------

    #[test]
    fn test_movi_8b() {
        // movi v0.8b, #0
        let (mnemonic, ops) = disasm(&[0x00, 0xe4, 0x00, 0x0f]);
        assert_eq!(mnemonic, "movi");
        assert_eq!(ops, "v0.8b, #0");
    }

    #[test]
    fn test_movi_2s() {
        // movi v0.2s, #1
        let (mnemonic, ops) = disasm(&[0x20, 0x04, 0x00, 0x0f]);
        assert_eq!(mnemonic, "movi");
        assert_eq!(ops, "v0.2s, #1");
    }

    #[test]
    fn test_movi_4s_lsl() {
        // movi v0.4s, #1, lsl #8
        let (mnemonic, ops) = disasm(&[0x20, 0x24, 0x00, 0x4f]);
        assert_eq!(mnemonic, "movi");
        assert_eq!(ops, "v0.4s, #1, lsl #8");
    }

    #[test]
    fn test_movi_2d() {
        // movi v0.2d, #0xff00ff00ff00ff00
        let (mnemonic, ops) = disasm(&[0x40, 0xe5, 0x05, 0x6f]);
        assert_eq!(mnemonic, "movi");
        assert_eq!(ops, "v0.2d, #0xff00ff00ff00ff00");
    }

    #[test]
    fn test_movi_d_scalar() {
        // movi d0, #0xff00ff00ff00ff00
        let (mnemonic, ops) = disasm(&[0x40, 0xe5, 0x05, 0x2f]);
        assert_eq!(mnemonic, "movi");
        assert_eq!(ops, "d0, #0xff00ff00ff00ff00");
    }

    #[test]
    fn test_mvni_2s() {
        // mvni v0.2s, #1
        let (mnemonic, ops) = disasm(&[0x20, 0x04, 0x00, 0x2f]);
        assert_eq!(mnemonic, "mvni");
        assert_eq!(ops, "v0.2s, #1");
    }

    #[test]
    fn test_orr_2s() {
        // orr v0.2s, #1
        let (mnemonic, ops) = disasm(&[0x20, 0x14, 0x00, 0x0f]);
        assert_eq!(mnemonic, "orr");
        assert_eq!(ops, "v0.2s, #1");
    }

    #[test]
    fn test_bic_2s() {
        // bic v0.2s, #1
        let (mnemonic, ops) = disasm(&[0x20, 0x14, 0x00, 0x2f]);
        assert_eq!(mnemonic, "bic");
        assert_eq!(ops, "v0.2s, #1");
    }

    #[test]
    fn test_fmov_vector_imm_s() {
        // fmov v1.2s, #1.00000000
        let (mnemonic, ops) = disasm(&[0x01, 0xf6, 0x03, 0x0f]);
        assert_eq!(mnemonic, "fmov");
        assert_eq!(ops, "v1.2s, #1.00000000");
    }

    #[test]
    fn test_fmov_vector_imm_d() {
        // fmov v31.2d, #1.00000000
        let (mnemonic, ops) = disasm(&[0x1f, 0xf6, 0x03, 0x6f]);
        assert_eq!(mnemonic, "fmov");
        assert_eq!(ops, "v31.2d, #1.00000000");
    }
}

