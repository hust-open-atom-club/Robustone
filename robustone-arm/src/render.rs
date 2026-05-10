//! AArch64 instruction text rendering with Capstone compatibility.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

/// Options controlling AArch64 text rendering behavior.
pub struct RenderOptions {
    /// Use register aliases (e.g., `fp` for `x29`).
    pub alias_regs: bool,
    /// Prefer Capstone-specific mnemonic aliases.
    pub capstone_aliases: bool,
    /// Use compressed mnemonic aliases where available.
    pub compressed_aliases: bool,
    /// Render immediates as unsigned hex.
    pub unsigned_immediate: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            alias_regs: true,
            capstone_aliases: true,
            compressed_aliases: true,
            unsigned_immediate: false,
        }
    }
}

/// Render an AArch64 decoded instruction into mnemonic and operand text.
pub fn render_aarch64_text_parts(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    options: RenderOptions,
) -> (String, String) {
    let use_capstone_aliases = options.capstone_aliases;

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
        options.unsigned_immediate,
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
            gpr_name(register.id as u8, is_32bit, context)
                .unwrap_or("?")
                .to_string()
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
                .and_then(|b| {
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
    if instruction.raw_bytes.len() < 4 {
        return mnemonic.starts_with("w");
    }
    let word = u32::from_le_bytes([
        instruction.raw_bytes[0],
        instruction.raw_bytes[1],
        instruction.raw_bytes[2],
        instruction.raw_bytes[3],
    ]);

    // For cbz/cbnz/tbz/tbnz, the sf bit (bit 31) determines register size.
    if matches!(mnemonic, "cbz" | "cbnz" | "tbz" | "tbnz") {
        return ((word >> 31) & 1) == 0; // sf = 0 means 32-bit
    }

    // LDRSW always uses X register (64-bit result)
    if mnemonic == "ldrsw" {
        return false;
    }

    // For load/store register instructions, size field (bits 31:30) determines width.
    if matches!(
        mnemonic,
        "ldr" | "str" | "ldrb" | "strb" | "ldrh" | "strh"
            | "ldrsb" | "ldrsh"
            | "ldur" | "stur" | "ldurb" | "sturb" | "ldurh" | "sturh"
            | "ldursb" | "ldursh" | "ldursw"
            | "ldxr" | "stxr" | "ldxrb" | "stxrb" | "ldxrh" | "stxrh"
            | "ldxp" | "stxp"
    ) {
        let size = (word >> 30) & 0b11;
        let opc = (word >> 22) & 0b11;
        // Detect load literal: bit29=0, bit24=0, op0=0xC (bit28=1, bit27=1, bit26=0, bit25=0)
        let op0_val = (word >> 25) & 0xF;
        let is_load_literal = op0_val == 0xC && ((word >> 29) & 1) == 0 && ((word >> 24) & 1) == 0;
        if is_load_literal {
            // Load literal: size=0b00 → W, size=0b01 → X, size=0b10 → LDRSW (handled above)
            return size == 0b00;
        }
        // 64-bit cases: size=0b11 (doubleword) OR sign-extended 64-bit loads
        if size == 0b11 && matches!(opc, 0b00 | 0b01) {
            return false; // X register
        }
        if matches!(size, 0b00 | 0b01) && opc == 0b11 {
            return false; // LDRSB/LDRSH 64-bit result → X register
        }
        return true; // Everything else is W register
    }

    // For load/store pair, opc (bits 31:30) determines width.
    if matches!(mnemonic, "ldp" | "stp" | "ldnp" | "stnp" | "ldpsw") {
        let opc = (word >> 30) & 0b11;
        return opc != 0b10; // opc=0b10 → X registers; otherwise W
    }

    mnemonic.starts_with("w")
}

fn reg_context_for_mnemonic(mnemonic: &str) -> crate::shared::registers::RegContext {
    use crate::shared::registers::RegContext;
    match mnemonic {
        "add" | "sub" | "adds" | "subs" => RegContext::AddSub,
        "ldr" | "str" | "ldrb" | "strb" | "ldrh" | "strh"
        | "ldrsb" | "ldrsh" | "ldrsw"
        | "ldur" | "stur" | "ldurb" | "sturb" | "ldurh" | "sturh"
        | "ldursb" | "ldursh" | "ldursw"
        | "ldp" | "stp" | "ldnp" | "stnp" | "ldpsw"
        | "ldxr" | "stxr" | "ldxrb" | "stxrb" | "ldxrh" | "stxrh"
        | "ldxp" | "stxp" => RegContext::LoadStore,
        "br" | "blr" | "ret" => RegContext::Branch,
        _ => RegContext::DataProc,
    }
}

/// Wrapper matching the core `RenderFn` signature for use as a function pointer.
/// Delegates to [`render_aarch64_text_parts`] with per-flag `RenderOptions`.
pub fn render_aarch64_text_parts_fn(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    alias_regs: bool,
    capstone_aliases: bool,
    compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String) {
    render_aarch64_text_parts(
        instruction,
        profile,
        RenderOptions {
            alias_regs,
            capstone_aliases,
            compressed_aliases,
            unsigned_immediate,
        },
    )
}
