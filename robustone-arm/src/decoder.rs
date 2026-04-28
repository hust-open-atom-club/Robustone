//! AArch64 decoder for Robustone.
//!
//! Uses hierarchical match-based decoding following the AArch64 instruction encoding tree.

use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

use crate::encoding;
use crate::families;

/// AArch64 decoder.
pub struct AArch64Decoder;

impl Default for AArch64Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AArch64Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(
        &self,
        bytes: &[u8],
        _mode_name: &str,
        addr: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::NeedMoreBytes,
                architecture: Some("aarch64".to_string()),
                detail: "need 4 bytes for AArch64".to_string(),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let (mnemonic, operands) = decode_aarch64_word(word)?;

        let (
            registers_read,
            registers_written,
            implicit_registers_read,
            implicit_registers_written,
            groups,
        ) = compute_metadata(mnemonic, &operands);

        let render_hints = compute_render_hints(mnemonic, &operands);

        Ok(DecodedInstruction {
            architecture: ArchitectureId::Arm,
            address: addr,
            mode: "aarch64".to_string(),
            mnemonic: mnemonic.to_string(),
            opcode_id: Some(mnemonic.to_string()),
            size: 4,
            raw_bytes: bytes[..4].to_vec(),
            operands,
            registers_read,
            registers_written,
            implicit_registers_read,
            implicit_registers_written,
            groups,
            status: DecodeStatus::Success,
            render_hints,
            render: Some(crate::render::render_aarch64_text_parts),
        })
    }
}

fn decode_aarch64_word(word: u32) -> Result<(&'static str, Vec<Operand>), DisasmError> {
    // Fast-path: check for originally-supported hardcoded instructions first.
    // These have encodings that overlap multiple op0 ranges (e.g., NOP/HINT).
    if let Ok(result) = families::system::decode_system(word) {
        return Ok(result);
    }

    let op0 = encoding::extract_op0(word);

    match op0 {
        // Data Processing - Immediate: 100x
        0b1000 | 0b1001 => families::data_proc_imm::decode_data_processing_immediate(word),
        // Data Processing - Register: 0101, 1101
        0b0101 | 0b1101 => families::data_proc_reg::decode_data_processing_register(word),
        // Branches, Exception Generating and System: 101x
        0b1010 | 0b1011 => families::branches::decode_branches(word),
        _ => Err(DisasmError::DecodeFailure {
            kind: DecodeErrorKind::InvalidEncoding,
            architecture: Some("aarch64".to_string()),
            detail: format!("unrecognized AArch64 encoding 0x{word:08x}"),
        }),
    }
}

/// Compute render hints to match Capstone text output.
fn compute_render_hints(mnemonic: &str, operands: &[Operand]) -> RenderHints {
    match mnemonic {
        "hint" => RenderHints {
            capstone_mnemonic: Some("nop".to_string()),
            capstone_hidden_operands: vec![0],
        },
        "ret" => {
            // Capstone hides the operand only for the default ret (x30).
            // Non-default returns like ret x1 are shown explicitly.
            let is_default = matches!(
                operands.first(),
                Some(Operand::Register { register }) if register.id == 30
            );
            RenderHints {
                capstone_mnemonic: None,
                capstone_hidden_operands: if is_default { vec![0] } else { vec![] },
            }
        }
        _ => RenderHints::default(),
    }
}

/// Compute register read/write metadata and instruction groups from decoded instruction.
#[allow(clippy::type_complexity)]
fn compute_metadata(
    mnemonic: &str,
    operands: &[Operand],
) -> (
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<String>,
) {
    let mut registers_read: Vec<RegisterId> = Vec::new();
    let mut registers_written: Vec<RegisterId> = Vec::new();
    let implicit_registers_read: Vec<RegisterId> = Vec::new();
    let mut implicit_registers_written: Vec<RegisterId> = Vec::new();
    let mut groups: Vec<String> = Vec::new();

    match mnemonic {
        "add" | "adds" | "sub" | "subs" | "orr" | "eor" | "and" => {
            groups.push("arithmetic".to_string());

            if let Some(Operand::Register { register }) = operands.first() {
                registers_written.push(*register);
            }
            // For immediate forms (e.g., ADD imm), Rn=31 is SP (a real read).
            // For register forms, Rn=31 is XZR (can be omitted).
            let is_imm = operands.len() >= 3
                && matches!(operands.get(2), Some(Operand::Immediate { .. }));
            if let Some(Operand::Register { register }) = operands.get(1)
                && (is_imm || register.id != 31)
            {
                registers_read.push(*register);
            }
            if let Some(Operand::Register { register }) = operands.get(2)
                && register.id != 31
            {
                registers_read.push(*register);
            }
        }
        "mov" => {
            groups.push("arithmetic".to_string());
            if let Some(Operand::Register { register }) = operands.first() {
                registers_written.push(*register);
            }
        }
        "csel" => {
            groups.push("conditional".to_string());
            if let Some(Operand::Register { register }) = operands.first() {
                registers_written.push(*register);
            }
            if let Some(Operand::Register { register }) = operands.get(1)
                && register.id != 31
            {
                registers_read.push(*register);
            }
            if let Some(Operand::Register { register }) = operands.get(2)
                && register.id != 31
            {
                registers_read.push(*register);
            }
        }
        "b" => {
            groups.push("branch".to_string());
            groups.push("jump".to_string());
        }
        "bl" => {
            groups.push("branch".to_string());
            groups.push("call".to_string());
            groups.push("jump".to_string());
            implicit_registers_written.push(aarch64_reg(30));
        }
        "br" => {
            groups.push("branch".to_string());
            groups.push("jump".to_string());
            if let Some(Operand::Register { register }) = operands.first() {
                registers_read.push(*register);
            }
        }
        "ret" => {
            groups.push("branch".to_string());
            groups.push("return".to_string());
            groups.push("jump".to_string());
            if let Some(Operand::Register { register }) = operands.first() {
                registers_read.push(*register);
            }
        }
        "hint" | "nop" => {
            groups.push("system".to_string());
        }
        _ => {}
    }

    (
        registers_read,
        registers_written,
        implicit_registers_read,
        implicit_registers_written,
        groups,
    )
}

fn aarch64_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: ArchitectureId::Arm,
        id,
    }
}
