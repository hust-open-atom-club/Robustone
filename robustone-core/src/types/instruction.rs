//! Instruction type definition.

use crate::ir::{DecodedInstruction, TextRenderProfile};
use crate::traits::instruction::{BasicInstructionDetail, Detail};

/// Decoded instruction returned by the disassembler.
#[derive(Debug)]
pub struct Instruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
    pub size: usize,
    pub detail: Option<Box<dyn Detail>>,
    pub decoded: Option<DecodedInstruction>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            address: 0,
            bytes: Vec::new(),
            mnemonic: "unknown".to_string(),
            operands: String::new(),
            size: 0,
            detail: None,
            decoded: None,
        }
    }
}

impl Instruction {
    pub fn new(address: u64, bytes: Vec<u8>, mnemonic: String, operands: String) -> Self {
        let size = bytes.len();
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: None,
            decoded: None,
        }
    }

    pub fn with_detail(
        address: u64,
        bytes: Vec<u8>,
        mnemonic: String,
        operands: String,
        detail: Box<dyn Detail>,
    ) -> Self {
        let size = bytes.len();
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: Some(detail),
            decoded: None,
        }
    }

    pub fn with_basic_detail(
        address: u64,
        bytes: Vec<u8>,
        mnemonic: String,
        operands: String,
        architecture: &'static str,
    ) -> Self {
        let size = bytes.len();
        let detail = BasicInstructionDetail::new(architecture);
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: Some(Box::new(detail)),
            decoded: None,
        }
    }

    pub fn unknown(address: u64, bytes: Vec<u8>) -> Self {
        let size = bytes.len();
        let hex_repr = format!("0x{}", hex::encode(&bytes));
        Self {
            address,
            bytes,
            mnemonic: "unknown".to_string(),
            operands: hex_repr,
            size,
            detail: None,
            decoded: None,
        }
    }

    /// Build a compatibility wrapper from a structured decoded instruction.
    pub fn from_decoded(
        decoded: DecodedInstruction,
        mnemonic: String,
        operands: String,
        detail: Option<Box<dyn Detail>>,
    ) -> Self {
        Self {
            address: decoded.address,
            bytes: decoded.raw_bytes.clone(),
            mnemonic,
            operands,
            size: decoded.size,
            detail,
            decoded: Some(decoded),
        }
    }

    pub fn is_unknown(&self) -> bool {
        self.mnemonic == "unknown"
    }

    /// Return text rendered from the shared IR when available, otherwise fall
    /// back to the legacy compatibility fields.
    pub fn rendered_text_parts(&self, profile: TextRenderProfile) -> (String, String) {
        self.decoded
            .as_ref()
            .map(|decoded| decoded.render_text_parts(profile))
            .unwrap_or_else(|| (self.mnemonic.clone(), self.operands.clone()))
    }

    pub fn assembly_line(&self) -> String {
        let (mnemonic, operands) = self.rendered_text_parts(TextRenderProfile::Capstone);
        format!("0x{:08x}: {:<7} {}", self.address, mnemonic, operands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ArchitectureId, DecodeStatus, Operand, RegisterId, RenderHints};

    #[test]
    fn test_instruction_creation() {
        let instruction =
            Instruction::new(0x1000, vec![0x90, 0x90], "nop".to_string(), String::new());

        assert_eq!(instruction.address, 0x1000);
        assert_eq!(instruction.mnemonic, "nop");
        assert_eq!(instruction.size, 2);
        assert!(instruction.detail.is_none());
        assert!(instruction.decoded.is_none());
    }

    #[test]
    fn test_unknown_instruction() {
        let instruction = Instruction::unknown(0x1000, vec![0xFF, 0xFF]);

        assert!(instruction.is_unknown());
        assert_eq!(instruction.mnemonic, "unknown");
        assert_eq!(instruction.operands, "0xffff");
    }

    #[test]
    fn test_assembly_line_formatting() {
        let instruction = Instruction::new(
            0x100,
            vec![0x48, 0x89, 0xD8],
            "mov".to_string(),
            "rax, rbx".to_string(),
        );

        let formatted = instruction.assembly_line();
        assert_eq!(formatted, "0x00000100: mov     rax, rbx");
    }

    #[test]
    fn test_rendered_text_parts_prefer_decoded_ir() {
        let decoded = DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: "riscv32".to_string(),
            mnemonic: "addi".to_string(),
            opcode_id: Some("addi".to_string()),
            size: 4,
            raw_bytes: vec![0x93, 0x00, 0x10, 0x00],
            operands: vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Register {
                    register: RegisterId::riscv(0),
                },
                Operand::Immediate { value: 1 },
            ],
            registers_read: vec![RegisterId::riscv(0)],
            registers_written: vec![RegisterId::riscv(1)],
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: vec!["arithmetic".to_string()],
            status: DecodeStatus::Success,
            render_hints: RenderHints {
                capstone_mnemonic: Some("li".to_string()),
                capstone_hidden_operands: vec![1],
            },
        };
        let instruction =
            Instruction::from_decoded(decoded, "legacy".to_string(), "legacy".to_string(), None);

        let (mnemonic, operands) = instruction.rendered_text_parts(TextRenderProfile::Capstone);
        assert_eq!(mnemonic, "li");
        assert_eq!(operands, "ra, 1");
    }
}
