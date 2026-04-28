//! AArch64 instruction field extraction helpers.

/// Extract the top-level opcode class (op0 = bits[28:25]).
pub fn extract_op0(word: u32) -> u8 {
    ((word >> 25) & 0xF) as u8
}

/// Extract Rd (destination register) = bits[4:0].
pub fn extract_rd(word: u32) -> u32 {
    word & 0x1F
}

/// Extract Rn (first source register) = bits[9:5].
pub fn extract_rn(word: u32) -> u32 {
    (word >> 5) & 0x1F
}

/// Extract Rm (second source register) = bits[20:16].
pub fn extract_rm(word: u32) -> u32 {
    (word >> 16) & 0x1F
}

/// Extract 12-bit immediate = bits[21:10].
pub fn extract_imm12(word: u32) -> u32 {
    (word >> 10) & 0xFFF
}

/// Extract 16-bit immediate = bits[20:5].
pub fn extract_imm16(word: u32) -> u32 {
    (word >> 5) & 0xFFFF
}

/// Extract 26-bit immediate = bits[25:0].
pub fn extract_imm26(word: u32) -> u32 {
    word & 0x3FFFFFF
}

/// Extract condition code = bits[15:12].
pub fn extract_cond(word: u32) -> u8 {
    ((word >> 12) & 0xF) as u8
}

/// Extract shift type = bits[23:22].
pub fn extract_shift(word: u32) -> u8 {
    ((word >> 22) & 0x3) as u8
}

/// Extract 6-bit immediate (for shifted register) = bits[15:10].
pub fn extract_imm6(word: u32) -> u32 {
    (word >> 10) & 0x3F
}

/// Check if the instruction is 64-bit (sf = bit[31]).
pub fn is_64bit(word: u32) -> bool {
    (word >> 31) & 1 == 1
}

/// Extract bit[30] (op / S bit in many encodings).
pub fn extract_op_bit30(word: u32) -> bool {
    ((word >> 30) & 1) == 1
}

/// Extract bits[30:29] (opc in logical immediate / move wide).
pub fn extract_opc(word: u32) -> u8 {
    ((word >> 29) & 0x3) as u8
}
