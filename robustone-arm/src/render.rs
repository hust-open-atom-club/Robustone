//! AArch64 instruction text rendering with Capstone compatibility.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

/// Render an AArch64 decoded instruction into mnemonic and operand text.
pub fn render_aarch64_text_parts(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    _alias_regs: bool,
    capstone_aliases: bool,
    _compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String) {
    let use_capstone_aliases = capstone_aliases;

    let mnemonic = if matches!(profile, TextRenderProfile::Canonical) || !use_capstone_aliases {
        instruction.mnemonic.clone()
    } else {
        instruction
            .render_hints
            .capstone_mnemonic
            .clone()
            .unwrap_or_else(|| instruction.mnemonic.clone())
    };

    let hidden_operands =
        if matches!(profile, TextRenderProfile::Canonical) || !use_capstone_aliases {
            &[][..]
        } else {
            instruction.render_hints.capstone_hidden_operands.as_slice()
        };

    let visible_operands: Vec<_> = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(index, _)| !hidden_operands.contains(index))
        .map(|(_, op)| op)
        .collect();

    // B.cond mnemonic is already set via capstone_mnemonic in render hints.
    // No additional processing needed here.

    let operands = format_operands(
        &mnemonic,
        &visible_operands,
        unsigned_immediate,
        instruction,
    );
    (mnemonic, operands)
}

fn format_operands(
    mnemonic: &str,
    operands: &[&Operand],
    unsigned_imm: bool,
    instruction: &DecodedInstruction,
) -> String {
    operands
        .iter()
        .map(|op| format_operand(mnemonic, op, unsigned_imm, instruction))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_operand(
    _mnemonic: &str,
    operand: &Operand,
    unsigned_imm: bool,
    instruction: &DecodedInstruction,
) -> String {
    match operand {
        Operand::Register { register } => {
            use crate::shared::registers::gpr_name;
            // Determine register size from mnemonic context or raw bytes
            let is_32bit = is_32bit_mnemonic(_mnemonic, instruction);
            let context = reg_context_for_mnemonic(_mnemonic);
            gpr_name(register.id as u8, is_32bit, context).to_string()
        }
        Operand::Immediate { value } => {
            if unsigned_imm && *value < 0 {
                format!("#0x{:x}", (*value as u64))
            } else if *value >= 0 && *value < 10 {
                format!("#{value}")
            } else {
                format!("#0x{value:x}")
            }
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            use crate::shared::registers::gpr_name;
            let base_name = base
                .map(|b| {
                    gpr_name(
                        b.id as u8,
                        false,
                        crate::shared::registers::RegContext::LoadStore,
                    )
                })
                .unwrap_or("sp");
            if *displacement == 0 {
                format!("[{base_name}]")
            } else {
                format!("[{base_name}, #0x{displacement:x}]")
            }
        }
    }
}

fn is_32bit_mnemonic(mnemonic: &str, instruction: &DecodedInstruction) -> bool {
    // Check if the original encoding used W registers.
    // For cbz/cbnz/tbz/tbnz, the sf bit (bit 31) determines register size.
    if matches!(mnemonic, "cbz" | "cbnz" | "tbz" | "tbnz") && instruction.raw_bytes.len() >= 4 {
        let word = u32::from_le_bytes([
            instruction.raw_bytes[0],
            instruction.raw_bytes[1],
            instruction.raw_bytes[2],
            instruction.raw_bytes[3],
        ]);
        return ((word >> 31) & 1) == 0; // sf = 0 means 32-bit
    }
    mnemonic.starts_with("w")
}

fn reg_context_for_mnemonic(mnemonic: &str) -> crate::shared::registers::RegContext {
    use crate::shared::registers::RegContext;
    match mnemonic {
        "add" | "sub" | "adds" | "subs" => RegContext::AddSub,
        "ldr" | "str" | "ldp" | "stp" => RegContext::LoadStore,
        "br" | "blr" | "ret" => RegContext::Branch,
        _ => RegContext::DataProc,
    }
}
