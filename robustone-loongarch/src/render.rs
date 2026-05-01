//! LoongArch instruction text rendering.
//!
//! Provides reference-compatible and canonical text rendering for LoongArch
//! decoded instructions. This module was extracted from robustone-core so
//! that architecture-specific formatting lives in the architecture crate.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

use crate::shared::registers::RegisterManager;

/// Helper: format a raw register id with or without ABI aliases.
fn format_register(id: u32, alias_regs: bool) -> String {
    let mgr = RegisterManager::instance();
    if alias_regs {
        mgr.format_raw_id(id).to_string()
    } else {
        mgr.format_raw_id_unaliased(id).to_string()
    }
}

// LEGACY: Phase 3 will replace immediate masks with per-instruction spec fields.
// immediate_mask_for_mnemonic() below still uses starts_with/ends_with patterns
// that must be migrated to OperandSpec::Immediate::unsigned_mask.

// LEGACY: Phase 3 will migrate remaining starts_with/ends_with patterns to per-instruction
// OperandSpec::Immediate::unsigned_mask. Branch masks now use InstructionGroup/EffectSpec.
/// Return the expected raw bit-mask for the instruction's immediate field.
fn immediate_mask_for_mnemonic(instruction: &DecodedInstruction) -> u64 {
    let mnemonic = &instruction.mnemonic;
    let is_branch = instruction.groups.iter().any(|g| g == "branch");

    match mnemonic.as_str() {
        // 28-bit PC-relative offsets
        "b" | "bl" => 0xFFFFFFF,
        // 20-bit
        m if m.starts_with("lu12i") => 0xFFFFF,
        m if m.starts_with("pcaddi") => 0xFFFFF,
        m if m.starts_with("pcaddu12i") => 0xFFFFF,
        m if m.starts_with("pcalau12i") => 0xFFFFF,
        // 16-bit branch / jirl offsets
        _ if (is_branch && mnemonic != "b" && mnemonic != "bl") || mnemonic == "jirl" => 0xFFFF,
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
    compat_aliases: bool,
    // LoongArch has no compressed instruction encoding, so this flag is
    // intentionally unused. It is kept in the signature to match the
    // `RenderFn` type expected by `DecodedInstruction`.
    _compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String) {
    let use_compat_aliases = compat_aliases && !matches!(profile, TextRenderProfile::Canonical);

    let mnemonic = if use_compat_aliases {
        instruction
            .render_hints
            .compat_mnemonic
            .clone()
            .unwrap_or_else(|| instruction.mnemonic.clone())
    } else {
        instruction.mnemonic.clone()
    };

    let hidden_operands = if use_compat_aliases {
        instruction.render_hints.compat_hidden_operands.as_slice()
    } else {
        &[][..]
    };

    let mut visible_operands = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(index, _)| !hidden_operands.contains(index))
        .collect::<Vec<_>>();

    // Invtlb operand reorder: imm, rj, rk (replaces handler patch).
    // Reorder operands [2, 0, 1] for INVLTLB.
    if instruction.opcode_id.as_deref() == Some("INVTLB") {
        let mut reordered: Vec<(usize, &Operand)> = Vec::new();
        for &idx in &[2usize, 0, 1] {
            if let Some(item) = visible_operands.iter().find(|(i, _)| *i == idx) {
                reordered.push(*item);
            }
        }
        if reordered.len() == visible_operands.len() {
            visible_operands = reordered;
        }
    }

    // Deduplicate equal register operands (replaces CSR/vector handler patches).
    let mut dedup_indices: Vec<usize> = Vec::new();
    for i in 0..visible_operands.len() {
        for j in (i + 1)..visible_operands.len() {
            if let (
                (_, Operand::Register { register: ra }),
                (_, Operand::Register { register: rb }),
            ) = (&visible_operands[i], &visible_operands[j])
                && ra.id == rb.id
            {
                dedup_indices.push(visible_operands[j].0);
            }
        }
    }
    visible_operands.retain(|(idx, _)| !dedup_indices.contains(idx));

    // PC-relative detection via InstructionGroup::Branch (replaces PC_RELATIVE_MNEMONICS list).
    // jirl is excluded because its offset is added to rj, not to the PC.
    let is_branch = instruction.groups.iter().any(|g| g == "branch");
    let is_pc_relative = is_branch || instruction.mnemonic == "jirl";
    let pc = instruction.address as i64;
    let imm_mask = immediate_mask_for_mnemonic(instruction);

    let mut operands = visible_operands
        .iter()
        .enumerate()
        .map(|(i, (_, operand))| {
            // For PC-relative instructions, the decoder adds the PC to the last immediate operand
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

    // LEGACY: Phase 3 will replace mnemonic-based vector register alias detection
    // with register bank metadata on OperandSpec. LSX uses $vr, LASX uses $xr.
    // LSX instructions are in the Vector group but not the SIMD/float groups.
    if alias_regs
        && instruction.groups.iter().any(|g| g == "vector")
        && !instruction.mnemonic.contains("xv")
    {
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
        Operand::Register { register } => format_register(register.id, alias_regs),
        Operand::Immediate { value } => {
            format_loongarch_immediate(*value, unsigned_immediate, imm_mask)
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory {
            base: Some(base),
            displacement,
        } => format!(
            "{}({})",
            format_loongarch_immediate(*displacement, unsigned_immediate, imm_mask),
            format_register(base.id, alias_regs)
        ),
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

use robustone_core::render::RenderOptions;
use robustone_core::renderer::Renderer;

/// LoongArch-specific instruction renderer.
pub struct LoongArchRenderer;

impl Renderer for LoongArchRenderer {
    fn render(&self, instruction: &DecodedInstruction, options: RenderOptions) -> (String, String) {
        render_loongarch_text_parts(
            instruction,
            options.text_profile,
            options.alias_regs,
            options.compat_aliases,
            options.compressed_aliases,
            options.unsigned_immediate,
        )
    }
}
