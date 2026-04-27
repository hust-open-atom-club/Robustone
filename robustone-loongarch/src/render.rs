//! LoongArch instruction text rendering.
//!
//! Provides Capstone-compatible and canonical text rendering for LoongArch
//! decoded instructions. This module was extracted from robustone-core so
//! that architecture-specific formatting lives in the architecture crate.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

use crate::shared::registers::RegisterManager;

/// Branch instructions that Capstone renders as absolute addresses.
const BRANCH_MNEMONICS: &[&str] = &[
    "b", "bl", "beq", "bne", "blt", "bge", "bltu", "bgeu", "beqz", "bnez", "bceqz", "bcnez",
];

/// Render a LoongArch decoded instruction into mnemonic and operand text.
pub fn render_loongarch_text_parts(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    _alias_regs: bool,
    _capstone_aliases: bool,
    _compressed_aliases: bool,
    _unsigned_immediate: bool,
) -> (String, String) {
    let use_capstone_aliases = !matches!(profile, TextRenderProfile::Canonical);

    let mnemonic = if use_capstone_aliases {
        instruction
            .render_hints
            .capstone_mnemonic
            .clone()
            .unwrap_or_else(|| instruction.mnemonic.clone())
    } else {
        instruction.mnemonic.clone()
    };

    let hidden_operands = if use_capstone_aliases {
        instruction.render_hints.capstone_hidden_operands.as_slice()
    } else {
        &[][..]
    };

    let visible_operands = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(index, _)| !hidden_operands.contains(index))
        .collect::<Vec<_>>();

    let is_branch = BRANCH_MNEMONICS.contains(&mnemonic.as_str());
    let pc = instruction.address as i64;

    let operands = visible_operands
        .iter()
        .enumerate()
        .map(|(i, (_, operand))| {
            // For branch instructions, Capstone adds the PC to the last immediate operand
            if is_branch
                && i == visible_operands.len() - 1
                && let Operand::Immediate { value } = operand
            {
                return format_loongarch_immediate(value + pc);
            }
            format_loongarch_operand(operand)
        })
        .collect::<Vec<_>>()
        .join(", ");

    (mnemonic, operands)
}

fn format_loongarch_operand(operand: &Operand) -> String {
    match operand {
        Operand::Register { register } => RegisterManager::instance()
            .format_raw_id(register.id)
            .to_string(),
        Operand::Immediate { value } => format_loongarch_immediate(*value),
        Operand::Text { value } => value.clone(),
        Operand::Memory {
            base: Some(base),
            displacement,
        } => {
            format!(
                "{}({})",
                format_loongarch_immediate(*displacement),
                RegisterManager::instance().format_raw_id(base.id)
            )
        }
        Operand::Memory {
            base: None,
            displacement,
        } => format_loongarch_immediate(*displacement),
    }
}

fn format_loongarch_immediate(value: i64) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let abs = value.abs();
    let use_hex = abs > 9;
    if use_hex {
        if value < 0 {
            format!("-0x{abs:x}")
        } else {
            format!("0x{abs:x}")
        }
    } else if value < 0 {
        format!("-{abs}")
    } else {
        format!("{abs}")
    }
}
