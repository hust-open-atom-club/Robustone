//! Instruction type definition.

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
        }
    }

    pub fn is_unknown(&self) -> bool {
        self.mnemonic == "unknown"
    }

    pub fn assembly_line(&self) -> String {
        format!(
            "0x{:08x}: {:<7} {}",
            self.address, self.mnemonic, self.operands
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        let instruction =
            Instruction::new(0x1000, vec![0x90, 0x90], "nop".to_string(), String::new());

        assert_eq!(instruction.address, 0x1000);
        assert_eq!(instruction.mnemonic, "nop");
        assert_eq!(instruction.size, 2);
        assert!(instruction.detail.is_none());
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
}
