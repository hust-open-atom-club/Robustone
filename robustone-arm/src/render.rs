//! AArch64 instruction text rendering.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

/// Render an AArch64 decoded instruction into mnemonic and operand text.
pub fn render_aarch64_text_parts(
    instruction: &DecodedInstruction,
    _profile: TextRenderProfile,
    _alias_regs: bool,
    _capstone_aliases: bool,
    _compressed_aliases: bool,
    _unsigned_immediate: bool,
) -> (String, String) {
    let capstone_mnemonic = instruction
        .render_hints
        .capstone_mnemonic
        .as_ref()
        .unwrap_or(&instruction.mnemonic)
        .clone();

    let hidden: std::collections::HashSet<usize> = instruction
        .render_hints
        .capstone_hidden_operands
        .iter()
        .copied()
        .collect();

    let operands = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(idx, _)| !hidden.contains(idx))
        .map(|(idx, op)| {
            format_aarch64_operand(
                instruction.mnemonic.as_str(),
                idx,
                op,
                &instruction.operands,
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    (capstone_mnemonic, operands)
}

fn format_aarch64_operand(
    mnemonic: &str,
    idx: usize,
    operand: &Operand,
    all_operands: &[Operand],
) -> String {
    match operand {
        Operand::Register { register } => {
            if register.id == 31 {
                // Register 31 is SP only in ADD/SUB immediate instructions.
                // In all other contexts (including CSEL, logical, branch), it is XZR.
                let is_sp = (mnemonic == "add" || mnemonic == "sub")
                    && idx < 2
                    && all_operands.len() >= 3
                    && matches!(all_operands.get(2), Some(Operand::Immediate { .. }));
                if is_sp {
                    "sp".to_string()
                } else {
                    "xzr".to_string()
                }
            } else {
                format!("x{}", register.id)
            }
        }
        Operand::Immediate { value } => {
            // Branch targets are printed without # prefix in Capstone
            if mnemonic == "b" || mnemonic == "bl" {
                format!("{value}")
            } else if *value >= 0 && *value < 10 {
                format!("#{value}")
            } else {
                // AArch64 bitmask immediates are unsigned; render as unsigned hex
                // when the signed i64 representation is negative.
                if *value < 0 {
                    format!("#0x{:x}", *value as u64)
                } else {
                    format!("#0x{value:x}")
                }
            }
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            if let Some(base) = base {
                format!("[{}, #{}]", aarch64_register_name(base.id), displacement)
            } else {
                format!("[#{displacement}]")
            }
        }
    }
}

fn aarch64_register_name(id: u32) -> String {
    match id {
        0..=30 => format!("x{id}"),
        31 => "sp".to_string(),
        _ => format!("r{id}"),
    }
}
