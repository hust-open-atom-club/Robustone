//! Encoding and decoding utilities for LoongArch instructions.
//!
//! Provides centralized functionality for bit extraction, sign extension,
//! and instruction field helpers used across all LoongArch families.

/// Extract a bit field from a 32-bit word.
///
/// # Arguments
/// * `word` - The instruction word
/// * `start` - The starting bit position (inclusive, 0-indexed from LSB)
/// * `len` - The number of bits to extract
///
/// # Example
/// ```
/// let word = 0b0000_0000_0000_0001_0000_0000_0000_0100u32;
/// assert_eq!(bits(word, 2, 3), 0b001);
/// ```
pub const fn bits(word: u32, start: u8, len: u8) -> u32 {
    (word >> start) & ((1u32 << len) - 1)
}

/// Sign-extend a value with the specified bit width to i64.
pub const fn sign_extend(value: u32, bit_width: u8) -> i64 {
    let mask = 1u32 << (bit_width - 1);
    if value & mask != 0 {
        // Negative: extend with ones
        let sign_extended = value | (!((1u32 << bit_width) - 1));
        sign_extended as i32 as i64
    } else {
        // Positive or zero
        value as i64
    }
}

/// Extract the 5-bit Rd field (bits 4:0).
pub const fn extract_rd(word: u32) -> u8 {
    bits(word, 0, 5) as u8
}

/// Extract the 5-bit Rj field (bits 9:5).
pub const fn extract_rj(word: u32) -> u8 {
    bits(word, 5, 5) as u8
}

/// Extract the 5-bit Rk field (bits 14:10).
pub const fn extract_rk(word: u32) -> u8 {
    bits(word, 10, 5) as u8
}

/// Extract the 5-bit Fa field (bits 4:0) for FP instructions.
pub const fn extract_fa(word: u32) -> u8 {
    bits(word, 0, 5) as u8
}

/// Extract the 5-bit Fj field (bits 9:5) for FP instructions.
pub const fn extract_fj(word: u32) -> u8 {
    bits(word, 5, 5) as u8
}

/// Extract the 5-bit Fk field (bits 14:10) for FP instructions.
pub const fn extract_fk(word: u32) -> u8 {
    bits(word, 10, 5) as u8
}

/// Extract the 3-bit FCC field.
pub const fn extract_fcc(word: u32, start: u8) -> u8 {
    bits(word, start, 3) as u8
}

/// Extract a 12-bit signed immediate.
pub const fn extract_si12(word: u32) -> i64 {
    sign_extend(bits(word, 10, 12), 12)
}

/// Extract a 14-bit signed immediate (used by some branch instructions).
pub const fn extract_si14(word: u32) -> i64 {
    sign_extend(bits(word, 10, 14), 14)
}

/// Extract a 16-bit signed immediate (used by some instructions).
pub const fn extract_si16(word: u32) -> i64 {
    sign_extend(bits(word, 10, 16), 16)
}

/// Extract a 20-bit signed immediate.
pub const fn extract_si20(word: u32) -> i64 {
    sign_extend(bits(word, 5, 20), 20)
}

/// Extract a 26-bit signed immediate (used by branch instructions).
pub const fn extract_si26(word: u32) -> i64 {
    let imm = bits(word, 0, 16) | (bits(word, 16, 10) << 16);
    sign_extend(imm, 26)
}

/// Extract a 5-bit unsigned immediate.
pub const fn extract_ui5(word: u32, start: u8) -> u8 {
    bits(word, start, 5) as u8
}

/// Extract the major opcode (top 6 bits, 31:26).
pub const fn extract_major_opcode(word: u32) -> u8 {
    bits(word, 26, 6) as u8
}

/// Extract a longer opcode field (top 10 bits, 31:22).
pub const fn extract_opcode10(word: u32) -> u16 {
    bits(word, 22, 10) as u16
}

/// Extract a 17-bit opcode field (top 17 bits, 31:15).
pub const fn extract_opcode17(word: u32) -> u32 {
    bits(word, 15, 17)
}

/// Extract a 22-bit opcode field (top 22 bits, 31:10).
pub const fn extract_opcode22(word: u32) -> u32 {
    bits(word, 10, 22)
}

/// Extract a 15-bit opcode field (top 15 bits, 31:17).
pub const fn extract_opcode15(word: u32) -> u32 {
    bits(word, 17, 15)
}

/// Extract an 8-bit opcode field (top 8 bits, 31:24).
pub const fn extract_opcode8(word: u32) -> u8 {
    bits(word, 24, 8) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_extraction() {
        assert_eq!(bits(0b1010_1100, 2, 3), 0b011);
        assert_eq!(bits(0xFFFFFFFF, 0, 5), 0x1F);
        assert_eq!(bits(0xFFFFFFFF, 31, 1), 0x1);
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0, 12), 0);
        assert_eq!(sign_extend(0x7FF, 12), 2047);
        assert_eq!(sign_extend(0x800, 12), -2048);
        assert_eq!(sign_extend(0xFFF, 12), -1);
    }

    #[test]
    fn test_extract_si12() {
        // addi.w $a0, $a1, 4 => opcode 0x0A, si12=4, rj=5, rd=4 => 0x028010A4
        let word = 0x028010A4u32;
        assert_eq!(extract_rd(word), 4); // $a0
        assert_eq!(extract_rj(word), 5); // $a1
        assert_eq!(extract_si12(word), 4);
    }

    #[test]
    fn test_extract_si26() {
        // b 0xf8 => bytes [0x00, 0xf8, 0x00, 0x50]
        // little-endian word: 0x5000f800
        let word = 0x5000f800u32;
        // I26 format: imm26 = bits(0,16) | bits(16,10)<<16
        let imm26 = extract_si26(word);
        assert_eq!(imm26, 0xF800i64 as i64); // raw 26-bit immediate from bits 25:0
    }
}
