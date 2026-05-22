//! Bit-field extraction helpers for AArch64 instructions.

/// Register width selector from the `sf` bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegWidth {
    /// 32-bit (W registers).
    W,
    /// 64-bit (X registers).
    X,
}

impl RegWidth {
    /// Returns true if this is the 32-bit width.
    pub fn is_w(self) -> bool {
        matches!(self, RegWidth::W)
    }

    /// Returns true if this is the 64-bit width.
    pub fn is_x(self) -> bool {
        matches!(self, RegWidth::X)
    }
}

/// Extract bits `[high:low]` (inclusive) from a 32-bit word.
pub fn bits(word: u32, high: u8, low: u8) -> u32 {
    debug_assert!(high >= low, "bits: high ({}) must be >= low ({})", high, low);
    debug_assert!(high < 32, "bits: high ({}) must be < 32", high);
    let width = (high - low + 1) as u32;
    let mask = (1u32.checked_shl(width).unwrap_or(0)).wrapping_sub(1);
    (word >> low) & mask
}

/// Extract a single bit.
pub fn bit(word: u32, pos: u8) -> bool {
    ((word >> pos) & 1) != 0
}

/// Extract the `sf` field (bit 31).
pub fn sf(word: u32) -> RegWidth {
    if bit(word, 31) {
        RegWidth::X
    } else {
        RegWidth::W
    }
}

/// Extract the `op0` field (bits 28:25) — top-level instruction class.
pub fn op0(word: u32) -> u8 {
    bits(word, 28, 25) as u8
}

/// Extract the `op1` field (bits 24:23).
pub fn op1(word: u32) -> u8 {
    bits(word, 24, 23) as u8
}

/// Extract the `op1_3bit` field (bits 25:23) for sub-group decode.
pub fn op1_3bit(word: u32) -> u8 {
    bits(word, 25, 23) as u8
}

/// Extract bit 28 (op1 in Data Processing — Register table).
pub fn bit28(word: u32) -> bool {
    bit(word, 28)
}

/// Extract bits 24:21 (op2 in Data Processing — Register table).
pub fn op2_4bit(word: u32) -> u8 {
    bits(word, 24, 21) as u8
}

/// Extract the `op2` field (bits 22:20).
pub fn op2(word: u32) -> u8 {
    bits(word, 22, 20) as u8
}

/// Extract the `op3` field (bits 15:10).
pub fn op3(word: u32) -> u8 {
    bits(word, 15, 10) as u8
}

/// Extract Rd (bits 4:0).
pub fn rd(word: u32) -> u8 {
    bits(word, 4, 0) as u8
}

/// Extract Rn (bits 9:5).
pub fn rn(word: u32) -> u8 {
    bits(word, 9, 5) as u8
}

/// Extract Rm (bits 20:16).
pub fn rm(word: u32) -> u8 {
    bits(word, 20, 16) as u8
}

/// Extract Ra (bits 14:10) for 3-source operations.
pub fn ra(word: u32) -> u8 {
    bits(word, 14, 10) as u8
}

/// Extract the `S` flag (bit 29) — set flags.
pub fn s_flag(word: u32) -> bool {
    bit(word, 29)
}

/// Extract the `shift` field (bits 23:22) for register operations.
pub fn shift(word: u32) -> u8 {
    bits(word, 23, 22) as u8
}

/// Extract the `imm12` field (bits 21:10).
pub fn imm12(word: u32) -> u16 {
    bits(word, 21, 10) as u16
}

/// Extract the `imm16` field (bits 20:5).
pub fn imm16(word: u32) -> u16 {
    bits(word, 20, 5) as u16
}

/// Extract the `hw` field (bits 22:21) for wide immediates.
pub fn hw(word: u32) -> u8 {
    bits(word, 22, 21) as u8
}

/// Extract the `N` bit (bit 22) for logical immediate.
pub fn n_bit(word: u32) -> bool {
    bit(word, 22)
}

/// Extract the `immr` field (bits 21:16) for logical immediate.
pub fn immr(word: u32) -> u8 {
    bits(word, 21, 16) as u8
}

/// Extract the `imms` field (bits 15:10) for logical immediate.
pub fn imms(word: u32) -> u8 {
    bits(word, 15, 10) as u8
}

/// Extract the `cond` field (bits 3:0).
pub fn cond(word: u32) -> u8 {
    bits(word, 3, 0) as u8
}

/// Extract the `opc` field (bits 30:29) for some operations.
pub fn opc(word: u32) -> u8 {
    bits(word, 30, 29) as u8
}

/// Extract the `size` field (bits 31:30) for load/store.
pub fn size(word: u32) -> u8 {
    bits(word, 31, 30) as u8
}

/// Extract the `v` bit (bit 26) for SIMD/FP load/store.
pub fn v_bit(word: u32) -> bool {
    bit(word, 26)
}

/// Extract the `op1` field (bits 28:24) for SIMD/FP data processing.
pub fn op1_5bit(word: u32) -> u8 {
    bits(word, 28, 24) as u8
}

/// Extract the `opcode` field (bits 15:12) for SIMD/FP.
pub fn opcode_4bit(word: u32) -> u8 {
    bits(word, 15, 12) as u8
}

/// Extract the `cmode` field (bits 15:12) for SIMD modified immediate.
pub fn cmode(word: u32) -> u8 {
    bits(word, 15, 12) as u8
}

/// Extract the `ftype` field (bits 23:22) for scalar FP.
pub fn ftype(word: u32) -> u8 {
    bits(word, 23, 22) as u8
}

/// Extract the `size` field (bits 23:22) for SIMD/FP operations.
pub fn simd_size(word: u32) -> u8 {
    bits(word, 23, 22) as u8
}

/// Extract the `Q` bit (bit 30) for SIMD vector width.
pub fn q_bit(word: u32) -> bool {
    bit(word, 30)
}

/// Extract the `U` bit (bit 29) for SIMD/FP operations.
pub fn u_bit(word: u32) -> bool {
    bit(word, 29)
}

/// Extract the `L` bit (bit 22) for SIMD/FP load/store direction.
pub fn l_bit(word: u32) -> bool {
    bit(word, 22)
}

/// Extract the `r` bit (bit 21) for SIMD/FP load/store.
pub fn r_bit(word: u32) -> bool {
    bit(word, 21)
}

/// Extract the `scale` field (bits 15:12) for SIMD/FP load/store.
pub fn scale(word: u32) -> u8 {
    bits(word, 15, 12) as u8
}

/// Extract the `len` field (bits 11:10) for SIMD structure loads/stores.
pub fn len(word: u32) -> u8 {
    bits(word, 11, 10) as u8
}

/// Extract the `opcode` field (bits 15:13) for SIMD structure loads/stores.
pub fn opcode_3bit(word: u32) -> u8 {
    bits(word, 15, 13) as u8
}

/// Extract the `rt` field (bits 4:0) — same as Rd.
pub fn rt(word: u32) -> u8 {
    rd(word)
}

/// Extract the `rn` field (bits 9:5) — same as Rn.
pub fn rn_reg(word: u32) -> u8 {
    rn(word)
}

/// Extract the `nzcv` field (bits 3:0) for conditional compare.
pub fn nzcv(word: u32) -> u8 {
    bits(word, 3, 0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_extraction() {
        let word = 0x91000820; // add x0, x1, #2
        assert!(sf(word).is_x());
        assert_eq!(op0(word), 0b1000);
        assert_eq!(op1(word), 0b10);
        assert_eq!(rd(word), 0);
        assert_eq!(rn(word), 1);
        assert_eq!(imm12(word), 2);
    }

    #[test]
    fn test_single_bit() {
        assert!(bit(0x80000000, 31));
        assert!(!bit(0x00000000, 31));
    }

    #[test]
    fn test_cond_extraction() {
        let word = 0x54000400; // b.eq
        assert_eq!(cond(word), 0);
    }
}
