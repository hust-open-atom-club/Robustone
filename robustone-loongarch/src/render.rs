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

/// Control-flow mnemonics whose last immediate operand is PC-relative.
/// `jirl` is excluded because its offset is added to `rj`, not to the PC.
const PC_RELATIVE_MNEMONICS: &[&str] = &[
    "b", "bl", "beq", "bne", "blt", "bge", "bltu", "bgeu", "beqz", "bnez", "bceqz", "bcnez",
];

/// Return the expected raw bit-mask for the immediate field of `mnemonic`.
///
/// This is used when `unsigned_immediate` is enabled so that sign-extended
/// negative constants are truncated back to their original encoded width
/// instead of being rendered as full 64-bit values.
fn immediate_mask_for_mnemonic(mnemonic: &str) -> u64 {
    match mnemonic {
        // 28-bit PC-relative offsets
        "b" | "bl" => 0xFFFFFFF,
        // 20-bit
        m if m.starts_with("lu12i") => 0xFFFFF,
        m if m.starts_with("pcaddi") => 0xFFFFF,
        m if m.starts_with("pcaddu12i") => 0xFFFFF,
        m if m.starts_with("pcalau12i") => 0xFFFFF,
        // 16-bit branch / jirl offsets
        m if BRANCH_MNEMONICS.contains(&m) && m != "b" && m != "bl" || m == "jirl" => 0xFFFF,
        // 14-bit
        "ll.w" | "llacq.w" | "sc.w" | "screl.w" => 0x3FFF,
        m if m.starts_with("ldl.")
            || m.starts_with("ldr.")
            || m.starts_with("stl.")
            || m.starts_with("str.") =>
        {
            0x3FFF
        }
        // 5-bit unsigned shift / vector immediates
        m if m.starts_with("slli")
            || m.starts_with("srli")
            || m.starts_with("srai")
            || m.starts_with("rotri")
            || m.starts_with("rcri")
            || m.starts_with("xvmaxi")
            || m.starts_with("xvmini")
            || m.starts_with("xvseqi")
            || m.starts_with("xvslei")
            || m.starts_with("xvslli")
            || m.starts_with("xvsrli")
            || m.starts_with("xvsrai")
            || m.starts_with("xvrotri")
            || m.starts_with("xvstelm")
            || m.starts_with("xvfrstpi") =>
        {
            0x1F
        }
        // 4-bit
        m if m.ends_with("replvei.b") => 0xF,
        // 3-bit
        m if m.ends_with("replvei.h") => 0x7,
        // 2-bit
        m if m.ends_with("replvei.w") => 0x3,
        // 1-bit
        m if m.ends_with("replvei.d") => 0x1,
        // 12-bit (default for the vast majority of LoongArch instructions)
        _ => 0xFFF,
    }
}

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

    let is_pc_relative = PC_RELATIVE_MNEMONICS.contains(&mnemonic.as_str());
    let pc = instruction.address as i64;
    let imm_mask = immediate_mask_for_mnemonic(&mnemonic);

    let mut operands = visible_operands
        .iter()
        .enumerate()
        .map(|(i, (_, operand))| {
            // For PC-relative instructions, Capstone adds the PC to the last immediate operand
            if is_pc_relative
                && i == visible_operands.len() - 1
                && let Operand::Immediate { value } = operand
            {
                return format_loongarch_immediate(value + pc, unsigned_immediate, imm_mask);
            }
            format_loongarch_operand(operand, alias_regs, unsigned_immediate, imm_mask)
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
    imm_mask: u64,
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
        Operand::Immediate { value } => {
            format_loongarch_immediate(*value, unsigned_immediate, imm_mask)
        }
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
                format_loongarch_immediate(*displacement, unsigned_immediate, imm_mask),
                base_name
            )
        }
        Operand::Memory {
            base: None,
            displacement,
        } => format_loongarch_immediate(*displacement, unsigned_immediate, imm_mask),
    }
}

fn format_loongarch_immediate(value: i64, unsigned_immediate: bool, imm_mask: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let (display_value, is_negative) = if unsigned_immediate && value < 0 {
        // Truncate the sign-extended value back to its original encoded width
        // so that e.g. a 12-bit -1 renders as 0xfff instead of 0xffffffffffffffff.
        ((value as u64) & imm_mask, false)
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
