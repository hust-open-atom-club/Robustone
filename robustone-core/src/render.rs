use crate::ir::{DecodedInstruction, TextRenderProfile};
use crate::types::instruction::Instruction;
use serde::Serialize;

/// Core-owned rendered instruction payload for text/JSON surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderedInstruction {
    pub address: u64,
    pub mnemonic: String,
    pub operands: String,
    pub size: usize,
    pub bytes: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<DecodedInstruction>,
}

impl RenderedInstruction {
    pub fn from_instruction(instruction: &Instruction, profile: TextRenderProfile) -> Self {
        let (mnemonic, operands) = instruction.rendered_text_parts(profile);
        Self {
            address: instruction.address,
            mnemonic,
            operands,
            size: instruction.size,
            bytes: instruction.bytes.clone(),
            decoded: instruction.decoded.clone(),
        }
    }
}

/// Core-owned rendered error payload for JSON/reporting surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderedIssue {
    pub kind: String,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_offset: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub raw_bytes: Vec<u8>,
}

/// Core-owned rendered disassembly envelope for batch JSON output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderedDisassembly {
    pub architecture: String,
    pub start_address: u64,
    pub bytes_processed: usize,
    pub errors: Vec<RenderedIssue>,
    pub instructions: Vec<RenderedInstruction>,
}
