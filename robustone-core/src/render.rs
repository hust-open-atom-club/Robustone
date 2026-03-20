use crate::ir::{ArchitectureId, DecodedInstruction, TextRenderProfile};
use crate::riscv::printer::{RiscVPrinter, RiscVTextProfile};
use crate::types::instruction::Instruction;
use serde::Serialize;

/// Render options shared between text and JSON surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderOptions {
    pub text_profile: TextRenderProfile,
    pub alias_regs: bool,
    pub unsigned_immediate: bool,
}

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
    pub fn from_instruction(instruction: &Instruction, options: RenderOptions) -> Self {
        let (mnemonic, operands) = render_instruction_text(instruction, options);
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

pub fn render_instruction_text(
    instruction: &Instruction,
    options: RenderOptions,
) -> (String, String) {
    if let Some(decoded) = &instruction.decoded
        && decoded.architecture == ArchitectureId::Riscv
    {
        let profile = match options.text_profile {
            TextRenderProfile::Capstone => RiscVTextProfile::Capstone,
            TextRenderProfile::Canonical => RiscVTextProfile::Canonical,
            TextRenderProfile::VerboseDebug => RiscVTextProfile::VerboseDebug,
        };
        let alias_regs =
            options.alias_regs || !matches!(options.text_profile, TextRenderProfile::Canonical);
        let printer = RiscVPrinter::new()
            .with_profile(profile)
            .with_alias_regs(alias_regs)
            .with_unsigned_immediate(options.unsigned_immediate);
        return printer.render_ir_parts(decoded);
    }

    instruction.rendered_text_parts(options.text_profile)
}

pub fn render_disassembly(
    architecture: String,
    start_address: u64,
    bytes_processed: usize,
    errors: Vec<RenderedIssue>,
    instructions: &[Instruction],
    options: RenderOptions,
) -> RenderedDisassembly {
    let instructions = instructions
        .iter()
        .map(|instruction| RenderedInstruction::from_instruction(instruction, options))
        .collect();

    RenderedDisassembly {
        architecture,
        start_address,
        bytes_processed,
        errors,
        instructions,
    }
}
