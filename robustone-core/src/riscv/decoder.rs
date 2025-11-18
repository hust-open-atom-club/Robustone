//! Refactored RISC-V decoder with extension-based architecture.
//!
//! This decoder uses a modular approach where each RISC-V extension (I, M, A, F, D, C)
//! is implemented as a separate module, making the codebase more maintainable
//! and easier to extend with new instructions.

use super::extensions::{InstructionExtension, create_extensions, extension_masks};
use super::types::*;
use crate::error::DisasmError;

/// RISC-V XLEN (register width) indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Xlen {
    X32,
    X64,
    // TODO support for RISC-V RV128 (`X128`) architecture
}

/// Refactored RISC-V instruction decoder using extension modules.
pub struct RiscVDecoder {
    xlen: Xlen,
    extensions: u32,
    extension_handlers: Vec<Box<dyn InstructionExtension>>,
}

impl RiscVDecoder {
    /// Construct a decoder with the provided XLEN and extension bitmask.
    pub fn new(xlen: Xlen, extensions: u32) -> Self {
        let extension_handlers = create_extensions();
        Self {
            xlen,
            extensions,
            extension_handlers,
        }
    }

    /// Create a decoder with full RV32GC support.
    pub fn rv32gc() -> Self {
        Self::new(
            Xlen::X32,
            extension_masks::I
                | extension_masks::M
                | extension_masks::A
                | extension_masks::F
                | extension_masks::C
                | extension_masks::ZICSR
                | extension_masks::ZICNTR,
        )
    }

    /// Create a decoder with full RV64GC support.
    pub fn rv64gc() -> Self {
        Self::new(
            Xlen::X64,
            extension_masks::I
                | extension_masks::M
                | extension_masks::A
                | extension_masks::F
                | extension_masks::D
                | extension_masks::C
                | extension_masks::ZICSR
                | extension_masks::ZICNTR,
        )
    }

    /// Decode a single instruction located at `address`.
    pub fn decode(
        &self,
        bytes: &[u8],
        address: u64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::DecodingError("No bytes provided".to_string()));
        }

        // Decoding priority:
        // 1. If at least two bytes are available and the low bits are not `0b11`, decode as
        //    a compressed instruction.
        // 2. Otherwise attempt a standard 32-bit instruction.
        if bytes.len() >= 2 && (bytes[0] & 0x3) != 0x3 {
            // Compressed encoding (two low bits are not `0b11`).
            self.decode_compressed_instruction(bytes, address)
        } else if bytes.len() >= 4 {
            // Standard instruction (low bits equal `0b11`) or fallback when compression fails.
            self.decode_standard_instruction(bytes, address)
        } else {
            Err(DisasmError::DecodingError(
                "Incomplete instruction".to_string(),
            ))
        }
    }

    /// Decode a 32-bit standard instruction using extension modules.
    fn decode_standard_instruction(
        &self,
        bytes: &[u8],
        _address: u64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        let instruction = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);

        let opcode = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as u8;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let rs1 = ((instruction >> 15) & 0x1F) as u8;
        let rs2 = ((instruction >> 20) & 0x1F) as u8;
        let funct7 = ((instruction >> 25) & 0x7F) as u8;
        let funct12 = (instruction >> 20) & 0xFFF;
        let _rs3 = ((instruction >> 27) & 0x1F) as u8;

        // Immediate value extraction across instruction formats.
        let imm_i = self.sign_extend((instruction >> 20) & 0xFFF, 12);
        let imm_s = self.sign_extend(
            ((instruction >> 7) & 0x1F) | (((instruction >> 25) & 0x7F) << 5),
            12,
        );
        let imm_b = self.sign_extend(
            ((instruction >> 7) & 0x1) << 11
                | ((instruction >> 8) & 0xF) << 1
                | ((instruction >> 25) & 0x3F) << 5
                | ((instruction >> 31) & 0x1) << 12,
            13,
        );
        let imm_u = (instruction & 0xFFFFF000) as i64; // U-type: bits[31:12], sign-extend to i64
        let imm_j = self.sign_extend(
            ((instruction >> 31) & 0x1) << 20
                | ((instruction >> 21) & 0x3FF) << 1
                | ((instruction >> 20) & 0x1) << 11
                | ((instruction >> 12) & 0xFF) << 12,
            21,
        );

        // Try each enabled extension in order
        for extension in &self.extension_handlers {
            if !extension.is_enabled(self.extensions) {
                continue;
            }

            if let Some(result) = extension.try_decode_standard(
                opcode, funct3, funct7, rd, rs1, rs2, funct12, imm_i, imm_s, imm_b, imm_u, imm_j,
                self.xlen,
            ) {
                return result;
            }
        }

        // No extension could decode this instruction
        self.decode_unknown_instruction(instruction)
    }

    /// Decode a 16-bit compressed instruction using extension modules.
    fn decode_compressed_instruction(
        &self,
        bytes: &[u8],
        _address: u64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        // cstool compatibility: interpret bytes in reverse order for 16-bit instructions
        let instruction = ((bytes[1] as u16) << 8) | (bytes[0] as u16);
        let opcode = instruction & 0x03;
        let funct3 = ((instruction >> 13) & 0x7) as u8;

        // Compressed register fields:
        let rd_full = ((instruction >> 7) & 0x1F) as u8; // bits 11..7
        let rs1_full = ((instruction >> 7) & 0x1F) as u8; // bits 11..7
        let rs2_full = ((instruction >> 2) & 0x1F) as u8; // bits 6..2
        let rdp = ((instruction >> 2) & 0x7) as u8; // bits 4..2 (0..7)
        let rs1p = ((instruction >> 7) & 0x7) as u8; // bits 9..7 (0..7)
        let rs2p = ((instruction >> 2) & 0x7) as u8; // bits 4..2 (0..7)

        // Decode immediate fields for each compressed encoding shape.
        // CIW format for c.addi4spn: nzuimm[5:4|3:2|6|7] (bits[12:5] of instruction)
        let nzuimm_ciw = ((instruction >> 5) & 0x1) << 4
            | ((instruction >> 6) & 0x1) << 5
            | ((instruction >> 7) & 0x1) << 6
            | ((instruction >> 8) & 0x1) << 7
            | ((instruction >> 9) & 0x1) << 8
            | ((instruction >> 10) & 0x1) << 9
            | ((instruction >> 12) & 0x1) << 5;

        // CL format for c.lw/c.flw: uimm[5:3|6|2|7]
        let uimm_cl = ((instruction >> 5) & 0x3) << 6
            | ((instruction >> 10) & 0x1) << 5
            | ((instruction >> 6) & 0x1) << 2
            | ((instruction >> 12) & 0x1) << 3;

        // CS format for c.sw/c.fsw: same as CL
        let uimm_cs = uimm_cl;

        // CI format for c.addi/c.li/c.jal/c.slli: imm[5] | imm[4:0]
        let imm_ci = self.sign_extend_c(
            ((instruction >> 12) & 0x1) << 5 | ((instruction >> 2) & 0x1F),
            6,
        );

        // CJ format for c.j/c.jal: imm[11|4|9:8|10|6|7|3:1|5]
        let imm_cj = self.sign_extend_c(
            ((instruction >> 12) & 0x1) << 11
                | ((instruction >> 8) & 0x3) << 9   // bits 9:8 from instruction[9:8]
                | ((instruction >> 10) & 0x1) << 8  // bit 8 from instruction[10]
                | ((instruction >> 6) & 0x1) << 7   // bit 7 from instruction[6]
                | ((instruction >> 7) & 0x1) << 6   // bit 6 from instruction[7]
                | ((instruction >> 11) & 0x1) << 5  // bit 5 from instruction[11]
                | ((instruction >> 3) & 0x7) << 1   // bits 3:1 from instruction[3:1]
                | ((instruction >> 2) & 0x1) << 4, // bit 4 from instruction[2]
            12,
        );

        // CB format for c.beqz/c.bnez: imm[8|4:3] | imm[7:6] | imm[2:1] | imm[5]
        let imm_cb = self.sign_extend_c(
            ((instruction >> 12) & 0x1) << 8
                | ((instruction >> 10) & 0x3) << 3
                | ((instruction >> 5) & 0x3) << 6
                | ((instruction >> 3) & 0x3) << 1
                | ((instruction >> 2) & 0x1) << 5,
            9,
        );

        // CSS format for c.swsp: uimm[5:2|6:7]
        let uimm_css = ((instruction >> 7) & 0x3) << 2 | ((instruction >> 9) & 0x3) << 6;

        // CL format for c.lwsp: uimm[5:3|2|6]
        let uimm_clsp = ((instruction >> 7) & 0x7) << 3
            | ((instruction >> 6) & 0x1) << 2
            | ((instruction >> 12) & 0x1) << 6;

        // CI format for c.fldsp: uimm[5:3|2|4|6|8:7] (RISC-V spec)
        let uimm_fldsp = ((instruction >> 7) & 0x7) << 3  // imm[5:3] from rd[2:0]
            | ((instruction >> 5) & 0x1) << 2          // imm[2] from instruction[5]
            | ((instruction >> 12) & 0x1) << 4         // imm[4] from instruction[12]
            | ((instruction >> 6) & 0x1) << 6          // imm[6] from instruction[6]
            | ((instruction >> 9) & 0x3) << 7; // imm[8:7] from instruction[9:8]

        if self.extensions & extension_masks::C == 0 {
            eprintln!("Warning: Decoding compressed instruction while C extension is disabled");
        }

        // Try each enabled extension for compressed instructions
        for extension in &self.extension_handlers {
            if !extension.is_enabled(self.extensions) {
                continue;
            }

            if let Some(result) = extension.try_decode_compressed(
                instruction,
                opcode as u8,
                funct3,
                self.xlen,
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
            ) {
                return result;
            }
        }

        // No extension could decode this compressed instruction
        self.decode_c_unknown(instruction)
    }

    // Helper methods
    fn sign_extend(&self, value: u32, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
        }
    }

    fn sign_extend_c(&self, value: u16, bits: u8) -> i64 {
        let sign_bit = 1 << (bits - 1);
        if (value & sign_bit) != 0 {
            (value as i64) - (1 << bits)
        } else {
            value as i64
        }
    }

    fn decode_unknown_instruction(
        &self,
        instruction: u32,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "unknown".to_string(),
            operands: format!("0x{instruction:08x}"),
            format: RiscVInstructionFormat::I,
            size: 4,
            operands_detail: vec![],
        })
    }

    fn decode_c_unknown(&self, instruction: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        Ok(RiscVDecodedInstruction {
            mnemonic: "c.unknown".to_string(),
            operands: format!("0x{instruction:04x}"),
            format: RiscVInstructionFormat::CI,
            size: 2,
            operands_detail: vec![],
        })
    }
}

/// Fully decoded instruction payload tailored for the CLI output.
#[derive(Debug, Clone)]
pub struct RiscVDecodedInstruction {
    /// Instruction mnemonic.
    pub mnemonic: String,
    /// Formatted operand string.
    pub operands: String,
    /// Instruction format discriminator.
    pub format: RiscVInstructionFormat,
    /// Size of the instruction in bytes.
    pub size: usize,
    /// Structured operand details for downstream consumption.
    pub operands_detail: Vec<RiscVOperand>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refactored_decoder_creation() {
        let decoder = RiscVDecoder::rv32gc();
        assert_eq!(decoder.xlen, Xlen::X32);
        assert!(decoder.extensions & extension_masks::I != 0);

        let decoder = RiscVDecoder::rv64gc();
        assert_eq!(decoder.xlen, Xlen::X64);
        assert!(decoder.extensions & extension_masks::I != 0);

        let decoder = RiscVDecoder::rv64gc();
        assert_eq!(decoder.xlen, Xlen::X64);
        assert!(decoder.extensions & extension_masks::I != 0);
        assert!(decoder.extensions & extension_masks::M != 0);
        assert!(decoder.extensions & extension_masks::A != 0);
        assert!(decoder.extensions & extension_masks::F != 0);
        assert!(decoder.extensions & extension_masks::D != 0);
        assert!(decoder.extensions & extension_masks::C != 0);
    }

    #[test]
    fn test_basic_instruction_decoding() {
        let decoder = RiscVDecoder::rv32gc();

        // Test ADDI x1, x2, 100
        // ADDI format: imm[11:0] | rs1[4:0] | funct3[2:0] | rd[4:0] | opcode[6:0]
        // imm = 100, rs1 = x2 (2), rd = x1 (1), funct3 = 0b000, opcode = 0b0010011
        let instruction = ((100u32 << 20) | (2u32 << 15)) | (1u32 << 7) | 0b0010011;
        let bytes = instruction.to_le_bytes();

        println!("Testing ADDI instruction: 0x{instruction:08x}");
        println!("Bytes: {bytes:?}");

        let result = decoder.decode(&bytes, 0);
        if let Err(e) = &result {
            println!("Decoding error: {e:?}");
        }
        assert!(result.is_ok(), "Failed to decode instruction: {result:?}");

        let instr = result.unwrap();
        assert_eq!(instr.mnemonic, "addi");
        assert_eq!(instr.size, 4);
    }

    #[test]
    fn test_compressed_instruction_decoding() {
        let decoder = RiscVDecoder::rv32gc();

        // Test C.ADDI x1, 1 -> 0x0505
        let bytes = [0x05, 0x05];
        let result = decoder.decode(&bytes, 0);
        assert!(result.is_ok());

        let instr = result.unwrap();
        assert_eq!(instr.mnemonic, "c.addi");
        assert_eq!(instr.size, 2);
    }
}
