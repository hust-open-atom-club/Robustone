//! Base instruction family for LoongArch.
//!
//! Covers all integer, floating-point, vector, and privileged
//! instructions decoded from the auto-generated Capstone tables.

use robustone_core::ir::{ArchitectureId, DecodeStatus, DecodedInstruction, RenderHints};
use robustone_core::types::error::DisasmError;

use crate::decoder_generated;
use crate::extensions::InstructionFamily;

/// Base family dispatching to the generated exact-word decoder.
pub struct BaseFamily;

impl InstructionFamily for BaseFamily {
    fn try_decode(&self, word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>> {
        match decoder_generated::decode_loongarch_word(word) {
            Ok((mnemonic, operands, size)) => {
                let mut render_hints = RenderHints::default();
                match mnemonic {
                    "nop" => {
                        render_hints.capstone_hidden_operands = vec![0, 1, 2];
                    }
                    "move" => {
                        render_hints.capstone_hidden_operands = vec![2];
                    }
                    _ => {}
                }

                // Capstone renders invtlb as "invtlb imm, Rj, Rk" but the
                // generated decoder produces operands in wire order "Rk, Rj, imm".
                let operands: Vec<_> = if mnemonic == "invtlb" && operands.len() == 3 {
                    vec![
                        operands[2].clone(),
                        operands[1].clone(),
                        operands[0].clone(),
                    ]
                } else {
                    operands
                };

                Some(Ok(DecodedInstruction {
                    architecture: ArchitectureId::LoongArch,
                    address: addr,
                    mode: "loongarch64".to_string(),
                    mnemonic: mnemonic.to_string(),
                    opcode_id: Some(mnemonic.to_string()),
                    size,
                    raw_bytes: word.to_le_bytes().to_vec(),
                    operands,
                    registers_read: Vec::new(),
                    registers_written: Vec::new(),
                    implicit_registers_read: Vec::new(),
                    implicit_registers_written: Vec::new(),
                    groups: Vec::new(),
                    status: DecodeStatus::Success,
                    render_hints,
                    render: Some(crate::render::render_loongarch_text_parts),
                }))
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn name(&self) -> &'static str {
        "base"
    }
}
