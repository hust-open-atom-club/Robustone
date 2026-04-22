//! AArch64 instruction text rendering.

use robustone_core::ir::{DecodedInstruction, TextRenderProfile};

/// Render an AArch64 decoded instruction into mnemonic and operand text.
pub fn render_aarch64_text_parts(
    instruction: &DecodedInstruction,
    _profile: TextRenderProfile,
    _alias_regs: bool,
    _capstone_aliases: bool,
    _compressed_aliases: bool,
    _unsigned_immediate: bool,
) -> (String, String) {
    let operands = instruction
        .operands
        .iter()
        .map(format_aarch64_operand)
        .collect::<Vec<_>>()
        .join(", ");
    (instruction.mnemonic.clone(), operands)
}

fn format_aarch64_operand(operand: &robustone_core::ir::Operand) -> String {
    use robustone_core::ir::Operand;
    match operand {
        Operand::Register { register } => aarch64_register_name(register.id),
        Operand::Immediate { value } => {
            if *value >= 0 && *value < 10 {
                value.to_string()
            } else {
                format!("0x{value:x}")
            }
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            if let Some(base) = base {
                format!("[{}, #{}]", aarch64_register_name(base.id), displacement)
            } else {
                format!("[#{}]", displacement)
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
