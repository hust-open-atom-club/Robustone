//! Refactored RISC-V decoder with extension-based architecture.
//!
//! This decoder uses a modular approach where each RISC-V extension (I, M, A, F, D, C)
//! is implemented as a separate module, making the codebase more maintainable
//! and easier to extend with new instructions.

use super::extensions::standard::Standard;
use super::extensions::{Extensions, InstructionExtension, create_extensions};
use super::types::*;
use robustone_core::common::ArchitectureProfile;
use robustone_core::ir::{
    ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints,
};
use robustone_core::types::error::DisasmError;
use robustone_core::utils::Endianness;

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
    extensions: Extensions,
    extension_handlers: Vec<Box<dyn InstructionExtension>>,
}

impl RiscVDecoder {
    /// Construct a decoder with the provided XLEN and extension bitmask.
    pub fn new(xlen: Xlen, extensions: Extensions) -> Self {
        let extension_handlers = create_extensions(xlen);
        Self {
            xlen,
            extensions,
            extension_handlers,
        }
    }

    /// Create a decoder with full RV32GC support.
    pub fn rv32gc() -> Self {
        Self::new(Xlen::X32, Extensions::rv32gc())
    }

    /// Create a decoder with full RV64GC support.
    pub fn rv64gc() -> Self {
        Self::new(Xlen::X64, Extensions::rv64gc())
    }

    /// Build a decoder from an explicit architecture profile.
    pub fn from_profile(profile: &ArchitectureProfile) -> Result<Self, DisasmError> {
        if profile.endianness != Endianness::Little {
            return Err(DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::UnsupportedMode,
                Some(profile.mode_name.to_string()),
                "big-endian RISC-V profiles are not implemented",
            ));
        }

        let (expected_arch, expected_width, xlen) = match &profile.architecture {
            crate::architecture::Architecture::RiscV32 => ("riscv32", 32, Xlen::X32),
            crate::architecture::Architecture::RiscV64 => ("riscv64", 64, Xlen::X64),
            other => {
                return Err(DisasmError::UnsupportedArchitecture(
                    other.as_str().to_string(),
                ));
            }
        };

        if profile.mode_name != expected_arch || profile.bit_width != expected_width {
            return Err(DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::UnsupportedMode,
                Some(profile.mode_name.to_string()),
                format!(
                    "profile mismatch: architecture={} bit_width={} mode_name={}",
                    expected_arch, profile.bit_width, profile.mode_name
                ),
            ));
        }

        let extensions = Extensions::from_enabled_extensions(&profile.enabled_extensions)?;
        Ok(Self::new(xlen, extensions))
    }

    /// Decode a single instruction located at `address`.
    pub fn decode(
        &self,
        bytes: &[u8],
        address: u64,
    ) -> Result<RiscVDecodedInstruction, DisasmError> {
        if bytes.is_empty() {
            return Err(DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::NeedMoreBytes,
                Some(self.mode_name().to_string()),
                "no bytes provided",
            ));
        }

        // Decoding priority:
        // 1. If at least two bytes are available and the low bits are not `0b11`, decode as
        //    a compressed instruction.
        // 2. Otherwise attempt a standard 32-bit instruction.
        if bytes.len() >= 2 && (bytes[0] & 0x3) != 0x3 {
            // Compressed encoding (two low bits are not `0b11`).
            if !self.extensions.standard.contains(Standard::C) {
                return Err(DisasmError::decode_failure(
                    crate::types::error::DecodeErrorKind::UnsupportedExtension,
                    Some(self.mode_name().to_string()),
                    "compressed instruction requires C extension",
                ));
            }
            self.decode_compressed_instruction(bytes, address)
        } else if bytes.len() >= 4 {
            // Standard instruction (low bits equal `0b11`) or fallback when compression fails.
            self.decode_standard_instruction(bytes, address)
        } else {
            Err(DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::NeedMoreBytes,
                Some(self.mode_name().to_string()),
                "incomplete instruction",
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

        if let Some(required_extension) = self.required_standard_extension(opcode, funct3, funct7) {
            return Err(DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::UnsupportedExtension,
                Some(self.mode_name().to_string()),
                format!("instruction requires {required_extension} extension"),
            ));
        }

        // Try each enabled extension in order
        for extension in &self.extension_handlers {
            if !extension.is_enabled(&self.extensions) {
                continue;
            }

            if let Some(result) = extension.try_decode_standard(
                opcode, funct3, funct7, rd, rs1, rs2, funct12, imm_i, imm_s, imm_b, imm_u, imm_j,
                self.xlen,
            ) {
                return result.map_err(|error| self.normalize_extension_error(error));
            }
        }

        // No extension could decode this instruction
        self.decode_unknown_instruction(instruction)
    }

    /// Decode directly into the shared IR with the provided mode / address context.
    pub fn decode_ir(
        &self,
        bytes: &[u8],
        arch_name: &str,
        address: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        let decoded = self.decode(bytes, address)?;
        let raw_bytes = bytes[..decoded.size].to_vec();
        Ok(decoded.to_ir(arch_name, address, raw_bytes))
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

        if !self.extensions.standard.contains(Standard::C) {
            eprintln!("Warning: Decoding compressed instruction while C extension is disabled");
        }

        // Try each enabled extension for compressed instructions
        for extension in &self.extension_handlers {
            if !extension.is_enabled(&self.extensions) {
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
                return result.map_err(|error| self.normalize_extension_error(error));
            }
        }

        // No extension could decode this compressed instruction
        self.decode_c_unknown(instruction)
    }

    fn mode_name(&self) -> &'static str {
        match self.xlen {
            Xlen::X32 => "riscv32",
            Xlen::X64 => "riscv64",
        }
    }

    fn required_standard_extension(
        &self,
        opcode: u32,
        funct3: u8,
        funct7: u8,
    ) -> Option<&'static str> {
        if opcode == 0b010_1111 && !self.extensions.standard.contains(Standard::A) {
            return Some("A");
        }

        if matches!(
            opcode,
            0b000_0111 | 0b010_0111 | 0b100_0011 | 0b100_0111 | 0b100_1011 | 0b100_1111
        ) {
            let extension = match funct3 {
                0b010 => "F",
                0b011 => "D",
                _ => "F",
            };
            let required = if extension == "D" {
                self.extensions.standard.contains(Standard::D)
            } else {
                self.extensions.standard.contains(Standard::F)
            };
            if !required {
                return Some(extension);
            }
        }

        if opcode == 0b101_0011 {
            let fmt = funct7 & 0b11;
            let required = match fmt {
                0b00 => (!self.extensions.standard.contains(Standard::F)).then_some("F"),
                0b01 => (!self.extensions.standard.contains(Standard::D)).then_some("D"),
                _ => None,
            };
            if required.is_some() {
                return required;
            }
        }

        if matches!(opcode, 0b011_0011 | 0b011_1011)
            && funct7 == 0b000_0001
            && !self.extensions.standard.contains(Standard::M)
        {
            return Some("M");
        }

        None
    }

    fn normalize_extension_error(&self, error: DisasmError) -> DisasmError {
        match error {
            DisasmError::DecodingError(detail) => DisasmError::decode_failure(
                crate::types::error::DecodeErrorKind::InvalidEncoding,
                Some(self.mode_name().to_string()),
                detail,
            ),
            other => other,
        }
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
        Err(DisasmError::decode_failure(
            crate::types::error::DecodeErrorKind::InvalidEncoding,
            Some(self.mode_name().to_string()),
            format!("unrecognized standard instruction 0x{instruction:08x}"),
        ))
    }

    fn decode_c_unknown(&self, instruction: u16) -> Result<RiscVDecodedInstruction, DisasmError> {
        Err(DisasmError::decode_failure(
            crate::types::error::DecodeErrorKind::InvalidEncoding,
            Some(self.mode_name().to_string()),
            format!("unrecognized compressed instruction 0x{instruction:04x}"),
        ))
    }
}

/// Rendering hints used by the RISC-V text formatter.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RiscVRenderHints {
    pub capstone_mnemonic: Option<String>,
    pub hidden_operands: Vec<usize>,
}

/// Structured RISC-V decode payload emitted by extension decoders before any
/// user-visible text rendering happens.
#[derive(Debug, Clone)]
pub struct RiscVDecodedInstruction {
    /// Canonical mnemonic / opcode identity for the instruction.
    pub mnemonic: String,
    /// Instruction format discriminator.
    pub format: RiscVInstructionFormat,
    /// Size of the instruction in bytes.
    pub size: usize,
    /// Structured operand details for downstream consumption.
    pub operands_detail: Vec<RiscVOperand>,
    /// Display hints for Capstone-like formatting.
    pub render_hints: RiscVRenderHints,
}

impl RiscVDecodedInstruction {
    /// Create a structured decode result with canonical mnemonic and operands.
    pub fn new(
        mnemonic: impl Into<String>,
        format: RiscVInstructionFormat,
        size: usize,
        operands_detail: Vec<RiscVOperand>,
    ) -> Self {
        Self {
            mnemonic: mnemonic.into(),
            format,
            size,
            operands_detail,
            render_hints: Default::default(),
        }
    }

    /// Mark this decoded instruction as using a Capstone-compatible alias for
    /// outward text rendering.
    pub fn with_capstone_alias(
        mut self,
        capstone_mnemonic: impl Into<String>,
        hidden_operands: Vec<usize>,
    ) -> Self {
        self.render_hints.capstone_mnemonic = Some(capstone_mnemonic.into());
        self.render_hints.hidden_operands = hidden_operands;
        self
    }

    /// Hide the specified operands in the Capstone-compatible outward view.
    pub fn with_hidden_operands(mut self, hidden_operands: Vec<usize>) -> Self {
        self.render_hints.hidden_operands = hidden_operands;
        self
    }

    /// Convert the RISC-V decode result into the shared IR.
    pub fn to_ir(&self, arch_name: &str, address: u64, raw_bytes: Vec<u8>) -> DecodedInstruction {
        let mut registers_read = Vec::new();
        let mut registers_written = Vec::new();
        let operands = self
            .operands_detail
            .iter()
            .map(|operand| match &operand.value {
                RiscVOperandValue::Register(reg) => {
                    let register = RegisterId::riscv(*reg);
                    if operand.access.read {
                        registers_read.push(register);
                    }
                    if operand.access.write {
                        registers_written.push(register);
                    }
                    Operand::Register { register }
                }
                RiscVOperandValue::Immediate(value) => Operand::Immediate { value: *value },
                RiscVOperandValue::RoundingMode(rm) => Operand::Text {
                    value: rounding_mode_name(*rm).to_string(),
                },
                RiscVOperandValue::Memory(memory) => {
                    let base = Some(RegisterId::riscv(memory.base));
                    if let Some(base_register) = base {
                        registers_read.push(base_register);
                    }
                    Operand::Memory {
                        base,
                        displacement: memory.disp,
                    }
                }
            })
            .collect();

        let (implicit_registers_read, implicit_registers_written) =
            infer_implicit_registers(&self.mnemonic);

        DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address,
            mode: arch_name.to_string(),
            mnemonic: self.mnemonic.clone(),
            opcode_id: Some(self.mnemonic.clone()),
            size: self.size,
            raw_bytes,
            operands,
            registers_read,
            registers_written,
            implicit_registers_read,
            implicit_registers_written,
            groups: infer_groups(&self.mnemonic),
            status: DecodeStatus::Success,
            render_hints: RenderHints {
                capstone_mnemonic: self.render_hints.capstone_mnemonic.clone(),
                capstone_hidden_operands: self.render_hints.hidden_operands.clone(),
            },
        }
    }
}

fn infer_groups(mnemonic: &str) -> Vec<String> {
    let mut groups = Vec::new();

    if mnemonic.starts_with("c.") {
        groups.push("compressed".to_string());
    }

    if mnemonic.starts_with('b') || matches!(mnemonic, "c.beqz" | "c.bnez") {
        groups.push("branch".to_string());
    }
    if mnemonic.contains("jal") || matches!(mnemonic, "j" | "c.j" | "c.jal" | "c.jr" | "c.jalr") {
        groups.push("control_flow".to_string());
    }
    if matches!(
        mnemonic,
        "lb" | "lh" | "lw" | "ld" | "lbu" | "lhu" | "lwu" | "flw" | "fld" | "c.lw" | "c.lwsp"
    ) {
        groups.push("load".to_string());
    }
    if matches!(
        mnemonic,
        "sb" | "sh" | "sw" | "sd" | "fsw" | "fsd" | "c.sw" | "c.swsp"
    ) {
        groups.push("store".to_string());
    }
    if mnemonic.starts_with("amo") || mnemonic.starts_with("lr.") || mnemonic.starts_with("sc.") {
        groups.push("atomic".to_string());
    }
    if mnemonic.starts_with('f') || mnemonic.contains(".s") || mnemonic.contains(".d") {
        groups.push("floating_point".to_string());
    }
    if mnemonic.starts_with("fcvt") || mnemonic.starts_with("fmv") || mnemonic.starts_with("fclass")
    {
        groups.push("conversion".to_string());
    }
    if mnemonic.starts_with("feq") || mnemonic.starts_with("flt") || mnemonic.starts_with("fle") {
        groups.push("compare".to_string());
    }
    if mnemonic.starts_with("csr") || mnemonic == "ecall" || mnemonic == "ebreak" {
        groups.push("system".to_string());
    }
    if groups.is_empty() {
        groups.push("arithmetic".to_string());
    }

    groups
}

fn infer_implicit_registers(mnemonic: &str) -> (Vec<RegisterId>, Vec<RegisterId>) {
    match mnemonic {
        "c.jal" | "c.jalr" => (Vec::new(), vec![RegisterId::riscv(1)]),
        _ => (Vec::new(), Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refactored_decoder_creation() {
        let decoder = RiscVDecoder::rv32gc();
        assert_eq!(decoder.xlen, Xlen::X32);
        assert!(decoder.extensions.standard.contains(Standard::I));

        let decoder = RiscVDecoder::rv64gc();
        assert_eq!(decoder.xlen, Xlen::X64);
        assert!(decoder.extensions.standard.contains(Standard::I));

        let decoder = RiscVDecoder::rv64gc();
        assert_eq!(decoder.xlen, Xlen::X64);
        assert!(decoder.extensions.standard.contains(Standard::G));
        assert!(decoder.extensions.standard.contains(Standard::C));
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
