//! LoongArch instruction text rendering.

use robustone_core::ir::{DecodedInstruction, TextRenderProfile};

/// Render a LoongArch decoded instruction into mnemonic and operand text.
pub fn render_loongarch_text_parts(
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
        .map(format_loongarch_operand)
        .collect::<Vec<_>>()
        .join(", ");
    (instruction.mnemonic.clone(), operands)
}

fn format_loongarch_operand(operand: &robustone_core::ir::Operand) -> String {
    use robustone_core::ir::Operand;
    match operand {
        Operand::Register { register } => loongarch_register_name(register.id),
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
                format!("{}({})", displacement, loongarch_register_name(base.id))
            } else {
                displacement.to_string()
            }
        }
    }
}

fn loongarch_register_name(id: u32) -> String {
    match id {
        0 => "$zero".to_string(),
        1 => "$ra".to_string(),
        2 => "$tp".to_string(),
        3 => "$sp".to_string(),
        4..=11 => format!("$a{}", id - 4),
        12..=19 => format!("$t{}", id - 12),
        20 => "$s0".to_string(),
        21 => "$s1".to_string(),
        22..=27 => format!("$s{}", id - 20),
        28 => "$t8".to_string(),
        29 => "$t9".to_string(),
        30 => "$s8".to_string(),
        31 => "$s9".to_string(),
        _ => format!("$r{id}"),
    }
}
