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
}
