//! x86/x64 instruction text rendering.

use robustone_core::ir::{DecodedInstruction, TextRenderProfile};

/// Render an x86 decoded instruction into mnemonic and operand text.
pub fn render_x86_text_parts(
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
        .map(format_x86_operand)
        .collect::<Vec<_>>()
        .join(", ");
    (instruction.mnemonic.clone(), operands)
}

fn format_x86_operand(operand: &robustone_core::ir::Operand) -> String {
    use robustone_core::ir::Operand;
    match operand {
        Operand::Register { register } => x86_register_name(register.id),
        Operand::Immediate { value } => format!("0x{value:x}"),
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            if let Some(base) = base {
                format!(
                    "[{}{}]",
                    x86_register_name(base.id),
                    format_disp(*displacement)
                )
            } else {
                format!("[{}]", displacement)
            }
        }
    }
}

fn format_disp(disp: i64) -> String {
    if disp == 0 {
        String::new()
    } else if disp < 0 {
        format!(" - 0x{:x}", disp.abs())
    } else {
        format!(" + 0x{:x}", disp)
    }
}

fn x86_register_name(id: u32) -> String {
    match id {
        0 => "eax",
        1 => "ecx",
        2 => "edx",
        3 => "ebx",
        4 => "esp",
        5 => "ebp",
        6 => "esi",
        7 => "edi",
        _ => "unknown",
    }
    .to_string()
}
