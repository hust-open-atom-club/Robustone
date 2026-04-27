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
    alias_regs: bool,
    capstone_aliases: bool,
    // LoongArch has no compressed instruction encoding, so this flag is
    // intentionally unused. It is kept in the signature to match the
    // `RenderFn` type expected by `DecodedInstruction`.
    _compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String) {
    let use_capstone_aliases = capstone_aliases && !matches!(profile, TextRenderProfile::Canonical);

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

    let mut operands = visible_operands
        .iter()
        .enumerate()
        .map(|(i, (_, operand))| {
            // For branch instructions, Capstone adds the PC to the last immediate operand
            if is_branch
                && i == visible_operands.len() - 1
                && let Operand::Immediate { value } = operand
            {
                return format_loongarch_immediate(value + pc, unsigned_immediate);
            }
            format_loongarch_operand(operand, alias_regs, unsigned_immediate)
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Capstone uses $vr for LSX (128-bit) vector registers and $xr for LASX (256-bit).
    // LSX instructions start with 'v' but do not contain "xv"; LASX instructions contain "xv".
    // Only apply the alias when register aliasing is enabled.
    if alias_regs && mnemonic.starts_with('v') && !mnemonic.contains("xv") {
        operands = operands.replace("$xr", "$vr");
    }

    (mnemonic, operands)
}

fn format_loongarch_operand(
    operand: &Operand,
    alias_regs: bool,
    unsigned_immediate: bool,
) -> String {
    match operand {
        Operand::Register { register } => {
            if alias_regs {
                RegisterManager::instance()
                    .format_raw_id(register.id)
                    .to_string()
            } else {
                RegisterManager::instance()
                    .format_raw_id_unaliased(register.id)
                    .to_string()
            }
        }
        Operand::Immediate { value } => format_loongarch_immediate(*value, unsigned_immediate),
        Operand::Text { value } => value.clone(),
        Operand::Memory {
            base: Some(base),
            displacement,
        } => {
            let base_name = if alias_regs {
                RegisterManager::instance().format_raw_id(base.id)
            } else {
                RegisterManager::instance().format_raw_id_unaliased(base.id)
            };
            format!(
                "{}({})",
                format_loongarch_immediate(*displacement, unsigned_immediate),
                base_name
            )
        }
        Operand::Memory {
            base: None,
            displacement,
        } => format_loongarch_immediate(*displacement, unsigned_immediate),
    }
}

fn format_loongarch_immediate(value: i64, unsigned_immediate: bool) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let (display_value, is_negative) = if unsigned_immediate && value < 0 {
        // Sign-extended negative immediates encode small unsigned constants.
        // Most LoongArch immediates are ≤16 bits, so mask to 16 bits to
        // avoid showing misleading 64-bit sign-extension artifacts.
        ((value as u64) & 0xFFFF, false)
    } else {
        (value.unsigned_abs(), value < 0)
    };
    let use_hex = display_value > 9;
    if use_hex {
        if is_negative {
            format!("-0x{display_value:x}")
        } else {
            format!("0x{display_value:x}")
        }
    } else if is_negative {
        format!("-{display_value}")
    } else {
        format!("{display_value}")
    }
}
