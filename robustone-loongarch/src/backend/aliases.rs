//! LoongArch instruction aliases applied post-decode via render hints.
//!
//! These aliases set `render_hints.compat_mnemonic` and hidden operands
//! without mutating the canonical `DecodedInstruction` fields (AC-4 compliant).

robustone_isa_macros::define_aliases! {
    arch = LoongArch;

    alias "nop" for "ANDI" {
        when [operand(0) == reg(0), operand(1) == reg(0), operand(2) == imm(0)];
        mnemonic = "nop";
        visible_operands = [];
    }

    alias "move" for "OR" {
        when [operand(2) == reg(0)];
        mnemonic = "move";
        visible_operands = [0, 1];
    }

}
