//! Encoding and decoding utilities for RISC-V instructions.
//!
//! Provides centralized functionality for sign extension, immediate extraction,
//! and instruction encoding patterns used across all RISC-V extensions.

/// Trait for sign extension operations.
pub trait SignExtender {
    /// Sign extend a value with the specified bit width.
    fn sign_extend(&self, value: u32, bits: u8) -> i64;

    /// Sign extend a 16-bit value with the specified bit width.
    fn sign_extend_16(&self, value: u16, bits: u8) -> i64;

    /// Extract a signed immediate value from an instruction field.
    fn extract_signed_imm(&self, instruction: u32, start: u8, width: u8) -> i64;
}

/// Trait for instruction decoding operations.
pub trait InstructionDecoder {
    /// Extract common RISC-V instruction fields.
    fn extract_fields(&self, instruction: u32) -> InstructionFields;

    /// Extract R-type instruction fields.
    fn extract_r_type(&self, instruction: u32) -> RTypeFields;

    /// Extract I-type instruction fields.
    fn extract_i_type(&self, instruction: u32) -> ITypeFields;

    /// Extract S-type instruction fields.
    fn extract_s_type(&self, instruction: u32) -> STypeFields;

    /// Extract B-type instruction fields.
    fn extract_b_type(&self, instruction: u32) -> BTypeFields;

    /// Extract U-type instruction fields.
    fn extract_u_type(&self, instruction: u32) -> UTypeFields;

    /// Extract J-type instruction fields.
    fn extract_j_type(&self, instruction: u32) -> JTypeFields;

    /// Extract compressed instruction fields.
    fn extract_compressed_fields(&self, instruction: u16) -> CompressedFields;
}

/// Common instruction fields extracted from a 32-bit instruction.
#[derive(Debug, Clone)]
pub struct InstructionFields {
    pub opcode: u32,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
    pub funct12: u32,
}

/// R-type instruction fields (register-register operations).
#[derive(Debug, Clone)]
pub struct RTypeFields {
    pub opcode: u32,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

/// I-type instruction fields (register-immediate operations).
#[derive(Debug, Clone)]
pub struct ITypeFields {
    pub opcode: u32,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i64,
}

/// S-type instruction fields (store operations).
#[derive(Debug, Clone)]
pub struct STypeFields {
    pub opcode: u32,
    pub imm: i64,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
}

/// B-type instruction fields (branch operations).
#[derive(Debug, Clone)]
pub struct BTypeFields {
    pub opcode: u32,
    pub imm: i64,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
}

/// U-type instruction fields (upper immediate operations).
#[derive(Debug, Clone)]
pub struct UTypeFields {
    pub opcode: u32,
    pub rd: u8,
    pub imm: i64,
}

/// J-type instruction fields (jump operations).
#[derive(Debug, Clone)]
pub struct JTypeFields {
    pub opcode: u32,
    pub rd: u8,
    pub imm: i64,
}

/// Compressed instruction fields.
#[derive(Debug, Clone)]
pub struct CompressedFields {
    pub opcode: u8,
    pub funct3: u8,
    pub rd_full: u8,
    pub rs1_full: u8,
    pub rs2_full: u8,
    pub rdp: u8,
    pub rs1p: u8,
    pub rs2p: u8,
    pub nzuimm_ciw: u16,
    pub uimm_cl: u16,
    pub uimm_cs: u16,
    pub imm_ci: i64,
    pub imm_cj: i64,
    pub imm_cb: i64,
    pub uimm_css: u16,
    pub uimm_clsp: u16,
    pub uimm_fldsp: u16,
}

/// Default implementation of sign extender.
pub struct DefaultSignExtender;

impl SignExtender for DefaultSignExtender {
    fn sign_extend(&self, value: u32, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
        }
    }

    fn sign_extend_16(&self, value: u16, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
        }
    }

    fn extract_signed_imm(&self, instruction: u32, start: u8, width: u8) -> i64 {
        let mask = (1u32 << width) - 1;
        let value = (instruction >> start) & mask;
        self.sign_extend(value, width)
    }
}

impl InstructionDecoder for DefaultSignExtender {
    fn extract_fields(&self, instruction: u32) -> InstructionFields {
        InstructionFields {
            opcode: instruction & 0x7F,
            rd: ((instruction >> 7) & 0x1F) as u8,
            funct3: ((instruction >> 12) & 0x7) as u8,
            rs1: ((instruction >> 15) & 0x1F) as u8,
            rs2: ((instruction >> 20) & 0x1F) as u8,
            funct7: ((instruction >> 25) & 0x7F) as u8,
            funct12: (instruction >> 20) & 0xFFF,
        }
    }

    fn extract_r_type(&self, instruction: u32) -> RTypeFields {
        let fields = self.extract_fields(instruction);
        RTypeFields {
            opcode: fields.opcode,
            rd: fields.rd,
            funct3: fields.funct3,
            rs1: fields.rs1,
            rs2: fields.rs2,
            funct7: fields.funct7,
        }
    }

    fn extract_i_type(&self, instruction: u32) -> ITypeFields {
        let fields = self.extract_fields(instruction);
        ITypeFields {
            opcode: fields.opcode,
            rd: fields.rd,
            funct3: fields.funct3,
            rs1: fields.rs1,
            imm: self.sign_extend((instruction >> 20) & 0xFFF, 12),
        }
    }

    fn extract_s_type(&self, instruction: u32) -> STypeFields {
        let fields = self.extract_fields(instruction);
        let imm = self.sign_extend(
            ((instruction >> 7) & 0x1F) | (((instruction >> 25) & 0x7F) << 5),
            12,
        );
        STypeFields {
            opcode: fields.opcode,
            imm,
            funct3: fields.funct3,
            rs1: fields.rs1,
            rs2: fields.rs2,
        }
    }

    fn extract_b_type(&self, instruction: u32) -> BTypeFields {
        let fields = self.extract_fields(instruction);
        let imm = self.sign_extend(
            ((instruction >> 7) & 0x1) << 11
                | ((instruction >> 8) & 0xF) << 1
                | ((instruction >> 25) & 0x3F) << 5
                | ((instruction >> 31) & 0x1) << 12,
            13,
        );
        BTypeFields {
            opcode: fields.opcode,
            imm,
            funct3: fields.funct3,
            rs1: fields.rs1,
            rs2: fields.rs2,
        }
    }

    fn extract_u_type(&self, instruction: u32) -> UTypeFields {
        let fields = self.extract_fields(instruction);
        UTypeFields {
            opcode: fields.opcode,
            rd: fields.rd,
            imm: (instruction & 0xFFFFF000) as i64,
        }
    }

    fn extract_j_type(&self, instruction: u32) -> JTypeFields {
        let fields = self.extract_fields(instruction);
        let imm = self.sign_extend(
            ((instruction >> 31) & 0x1) << 20
                | ((instruction >> 21) & 0x3FF) << 1
                | ((instruction >> 20) & 0x1) << 11
                | ((instruction >> 12) & 0xFF) << 12,
            21,
        );
        JTypeFields {
            opcode: fields.opcode,
            rd: fields.rd,
            imm,
        }
    }

    fn extract_compressed_fields(&self, instruction: u16) -> CompressedFields {
        let opcode = instruction & 0x3;
        let funct3 = ((instruction >> 13) & 0x7) as u8;
        let rd_full = ((instruction >> 7) & 0x1F) as u8;
        let rs1_full = ((instruction >> 7) & 0x1F) as u8;
        let rs2_full = ((instruction >> 2) & 0x1F) as u8;
        let rdp = ((instruction >> 2) & 0x7) as u8;
        let rs1p = ((instruction >> 7) & 0x7) as u8;
        let rs2p = ((instruction >> 2) & 0x7) as u8;

        // Extract immediate fields for compressed formats
        let nzuimm_ciw = ((instruction >> 5) & 0x1) << 4
            | ((instruction >> 6) & 0x1) << 5
            | ((instruction >> 7) & 0x1) << 6
            | ((instruction >> 8) & 0x1) << 7
            | ((instruction >> 9) & 0x1) << 8
            | ((instruction >> 10) & 0x1) << 9
            | ((instruction >> 12) & 0x1) << 5;

        let uimm_cl = ((instruction >> 5) & 0x3) << 6
            | ((instruction >> 10) & 0x1) << 5
            | ((instruction >> 6) & 0x1) << 2
            | ((instruction >> 12) & 0x1) << 3;

        let uimm_cs = uimm_cl;

        let imm_ci = self.sign_extend_16(
            ((instruction >> 12) & 0x1) << 5 | ((instruction >> 2) & 0x1F),
            6,
        );

        let imm_cj = self.sign_extend_16(
            ((instruction >> 12) & 0x1) << 11
                | ((instruction >> 8) & 0x3) << 9
                | ((instruction >> 10) & 0x1) << 8
                | ((instruction >> 6) & 0x1) << 7
                | ((instruction >> 7) & 0x1) << 6
                | ((instruction >> 11) & 0x1) << 5
                | ((instruction >> 3) & 0x7) << 1
                | ((instruction >> 2) & 0x1) << 4,
            12,
        );

        let imm_cb = self.sign_extend_16(
            ((instruction >> 12) & 0x1) << 8
                | ((instruction >> 10) & 0x3) << 3
                | ((instruction >> 5) & 0x3) << 6
                | ((instruction >> 3) & 0x3) << 1
                | ((instruction >> 2) & 0x1) << 5,
            9,
        );

        let uimm_css = ((instruction >> 7) & 0x3) << 2 | ((instruction >> 9) & 0x3) << 6;

        let uimm_clsp = ((instruction >> 7) & 0x7) << 3
            | ((instruction >> 6) & 0x1) << 2
            | ((instruction >> 12) & 0x1) << 6;

        let uimm_fldsp = ((instruction >> 7) & 0x7) << 3
            | ((instruction >> 5) & 0x1) << 2
            | ((instruction >> 12) & 0x1) << 4
            | ((instruction >> 6) & 0x1) << 6
            | ((instruction >> 9) & 0x3) << 7;

        CompressedFields {
            opcode: opcode as u8,
            funct3,
            rd_full,
            rs1_full,
            rs2_full,
            rdp,
            rs1p,
            rs2p,
            nzuimm_ciw,
            uimm_cl,
            uimm_cs,
            imm_ci,
            imm_cj,
            imm_cb,
            uimm_css,
            uimm_clsp,
            uimm_fldsp,
        }
    }
}

impl DefaultSignExtender {
    /// Create a new default sign extender.
    pub const fn new() -> Self {
        Self
    }

    /// Get the global default sign extender instance.
    pub const fn instance() -> &'static Self {
        &DefaultSignExtender
    }
}

impl Default for DefaultSignExtender {
    fn default() -> Self {
        Self::new()
    }
}

/// Shamt (shift amount) extraction utilities.
pub struct ShamtExtractor;

impl ShamtExtractor {
    /// Extract shift amount for standard instructions.
    pub fn extract_shamt(imm: i64, xlen: super::super::decoder::Xlen) -> i64 {
        let mask = match xlen {
            super::super::decoder::Xlen::X64 => 0x3f,
            super::super::decoder::Xlen::X32 => 0x1f,
        } as u64;
        (imm as u64 & mask) as i64
    }

    /// Extract shift amount for compressed instructions.
    pub fn extract_shamt_c(imm: i64, xlen: super::super::decoder::Xlen) -> i64 {
        Self::extract_shamt(imm, xlen)
    }

    /// Validate shift amount for the given XLEN.
    pub fn is_valid_shamt(shamt: i64, xlen: super::super::decoder::Xlen) -> bool {
        let max_bits = match xlen {
            super::super::decoder::Xlen::X64 => 6,
            super::super::decoder::Xlen::X32 => 5,
        };
        shamt >= 0 && shamt < (1 << max_bits)
    }
}

/// Convenience functions for encoding operations.
pub mod convenience {
    use super::*;

    /// Sign extend a value with the specified bit width.
    pub fn sign_extend(value: u32, bits: u8) -> i64 {
        DefaultSignExtender::instance().sign_extend(value, bits)
    }

    /// Sign extend a 16-bit value with the specified bit width.
    pub fn sign_extend_16(value: u16, bits: u8) -> i64 {
        DefaultSignExtender::instance().sign_extend_16(value, bits)
    }

    /// Extract instruction fields from a 32-bit instruction.
    pub fn extract_fields(instruction: u32) -> InstructionFields {
        DefaultSignExtender::instance().extract_fields(instruction)
    }

    /// Extract R-type fields.
    pub fn extract_r_type(instruction: u32) -> RTypeFields {
        DefaultSignExtender::instance().extract_r_type(instruction)
    }

    /// Extract I-type fields.
    pub fn extract_i_type(instruction: u32) -> ITypeFields {
        DefaultSignExtender::instance().extract_i_type(instruction)
    }

    /// Extract S-type fields.
    pub fn extract_s_type(instruction: u32) -> STypeFields {
        DefaultSignExtender::instance().extract_s_type(instruction)
    }

    /// Extract B-type fields.
    pub fn extract_b_type(instruction: u32) -> BTypeFields {
        DefaultSignExtender::instance().extract_b_type(instruction)
    }

    /// Extract U-type fields.
    pub fn extract_u_type(instruction: u32) -> UTypeFields {
        DefaultSignExtender::instance().extract_u_type(instruction)
    }

    /// Extract J-type fields.
    pub fn extract_j_type(instruction: u32) -> JTypeFields {
        DefaultSignExtender::instance().extract_j_type(instruction)
    }

    /// Extract compressed instruction fields.
    pub fn extract_compressed_fields(instruction: u16) -> CompressedFields {
        DefaultSignExtender::instance().extract_compressed_fields(instruction)
    }

    /// Extract and validate shift amount.
    pub fn extract_shamt(imm: i64, xlen: crate::riscv::decoder::Xlen) -> i64 {
        ShamtExtractor::extract_shamt(imm, xlen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::decoder::Xlen;

    #[test]
    fn test_sign_extender() {
        let extender = DefaultSignExtender::new();

        // Test positive values
        assert_eq!(extender.sign_extend(0x123, 12), 0x123);
        assert_eq!(extender.sign_extend(0x7FF, 12), 0x7FF);

        // Test negative values
        assert_eq!(extender.sign_extend(0x800, 12), -2048);
        assert_eq!(extender.sign_extend(0xFFF, 12), -1);

        // Test 16-bit extension
        assert_eq!(extender.sign_extend_16(0x1234, 16), 0x1234);
        assert_eq!(extender.sign_extend_16(0x8000, 16), -32768);
    }

    #[test]
    fn test_instruction_decoder() {
        let decoder = DefaultSignExtender::new();

        // Test R-type: add x1, x2, x3
        let instruction = 0b0000000_00011_00010_000_00001_0110011;
        let r_fields = decoder.extract_r_type(instruction);
        assert_eq!(r_fields.opcode, 0b0110011);
        assert_eq!(r_fields.rd, 1);
        assert_eq!(r_fields.rs1, 2);
        assert_eq!(r_fields.rs2, 3);
        assert_eq!(r_fields.funct3, 0b000);
        assert_eq!(r_fields.funct7, 0b0000000);

        // Test I-type: addi x1, x2, 5
        let instruction = 0b000000000101_00010_000_00001_0010011;
        let i_fields = decoder.extract_i_type(instruction);
        assert_eq!(i_fields.opcode, 0b0010011);
        assert_eq!(i_fields.rd, 1);
        assert_eq!(i_fields.rs1, 2);
        assert_eq!(i_fields.funct3, 0b000);
        assert_eq!(i_fields.imm, 5);

        // Test U-type: lui x5, 0x10
        let instruction = 0b0001_0000_0000_0000_0000_00101_0110111;
        let u_fields = decoder.extract_u_type(instruction);
        assert_eq!(u_fields.opcode, 0b0110111);
        assert_eq!(u_fields.rd, 5);
        assert_eq!(u_fields.imm, 268435456);
    }

    #[test]
    fn test_shamt_extractor() {
        // Test RV32
        assert_eq!(ShamtExtractor::extract_shamt(0x1F, Xlen::X32), 31);
        assert_eq!(ShamtExtractor::extract_shamt(0x1F, Xlen::X64), 31);

        // Test RV64
        assert_eq!(ShamtExtractor::extract_shamt(0x3F, Xlen::X64), 63);
        assert_eq!(ShamtExtractor::extract_shamt(0x3F, Xlen::X32), 31);

        // Test validation
        assert!(ShamtExtractor::is_valid_shamt(31, Xlen::X32));
        assert!(!ShamtExtractor::is_valid_shamt(32, Xlen::X32));
        assert!(ShamtExtractor::is_valid_shamt(63, Xlen::X64));
        assert!(!ShamtExtractor::is_valid_shamt(64, Xlen::X64));
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(convenience::sign_extend(0x800, 12), -2048);
        assert_eq!(convenience::sign_extend_16(0x8000, 16), -32768);

        let instruction = 0b0000000_00011_00010_000_00001_0110011;
        let fields = convenience::extract_fields(instruction);
        assert_eq!(fields.rd, 1);
        assert_eq!(fields.rs1, 2);
        assert_eq!(fields.rs2, 3);

        let r_fields = convenience::extract_r_type(instruction);
        assert_eq!(r_fields.rd, 1);
        assert_eq!(r_fields.rs1, 2);
        assert_eq!(r_fields.rs2, 3);

        assert_eq!(convenience::extract_shamt(31, Xlen::X32), 31);
    }
}
