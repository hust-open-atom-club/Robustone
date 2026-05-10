//! AArch64 instruction decoder for Robustone.
//!
//! Top-level decoder: extracts the 32-bit instruction word, validates size,
//! and dispatches to extension modules via `op0[28:25]`.

use crate::extensions;
use crate::types::{AArch64Extensions, Mnemonic};
use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// AArch64 decoder with extension gating.
pub struct AArch64Decoder {
    extensions: AArch64Extensions,
}

impl Default for AArch64Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AArch64Decoder {
    /// Creates a new decoder with all extensions enabled.
    pub fn new() -> Self {
        Self {
            extensions: AArch64Extensions::all(),
        }
    }

    /// Creates a new decoder with the given extensions.
    pub fn with_extensions(extensions: AArch64Extensions) -> Self {
        Self { extensions }
    }

    /// Decodes a single AArch64 instruction from the given bytes.
    pub fn decode(
        &self,
        bytes: &[u8],
        _mode_name: &str,
        addr: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::NeedMoreBytes,
                architecture: Some("aarch64".to_string()),
                detail: "need 4 bytes for AArch64".to_string(),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let (mnemonic, operands) = extensions::decode_by_op0(word, addr, &self.extensions)?;

        let mnemonic_str = mnemonic.as_str().to_string();

        // Compute metadata
        let (registers_read, registers_written, implicit_regs_read, implicit_regs_written, groups) =
            compute_metadata(mnemonic, &operands);

        // Compute render hints for alias expansion
        let render_hints = compute_render_hints(mnemonic, &operands);

        Ok(DecodedInstruction {
            architecture: ArchitectureId::Arm,
            address: addr,
            mode: "aarch64".to_string(),
            mnemonic: mnemonic_str.clone(),
            opcode_id: Some(mnemonic_str),
            size: 4,
            raw_bytes: bytes[..4].to_vec(),
            operands,
            registers_read,
            registers_written,
            implicit_registers_read: implicit_regs_read,
            implicit_registers_written: implicit_regs_written,
            groups,
            status: DecodeStatus::Success,
            render_hints,
            render: Some(crate::render::render_aarch64_text_parts_fn),
        })
    }
}

type Metadata = (
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<String>,
);

/// Collect register IDs from operand list.
fn operand_regs(operands: &[Operand]) -> Vec<RegisterId> {
    operands
        .iter()
        .filter_map(|op| match op {
            Operand::Register { register } => Some(*register),
            _ => None,
        })
        .collect()
}

/// Compute read/write register metadata and instruction groups.
fn compute_metadata(mnemonic: Mnemonic, operands: &[Operand]) -> Metadata {
    use crate::types::Mnemonic::*;

    let regs = operand_regs(operands);
    let mut read = Vec::new();
    let mut written = Vec::new();
    let mut implicit_read = Vec::new();
    let implicit_written = Vec::new();
    let mut groups = Vec::new();

    match mnemonic {
        // Data processing — register
        Add | Adds | Sub | Subs | And | Orr | Eor | Ands | Lsl | Lsr | Asr | Ror | Movz | Movn
        | Movk | Csel | Csinc | Csinv | Csneg | Madd | Msub | Smaddl | Smsubl | Umaddl | Umsubl
        | Sdiv | Udiv | Neg | Mvn | Mul | Mulh | Abs => {
            meta_data_proc(&regs, &mut read, &mut written, &mut groups);
        }
        // Data processing — compare / test
        Cmp | Cmn | Tst => {
            meta_compare(&regs, &mut read, &mut written, &mut groups);
        }
        // Address generation / move
        Adr | Adrp | Mov => {
            meta_move(&regs, &mut written, &mut groups);
        }
        // Branch — direct
        B | Bl => {
            meta_branch_direct(mnemonic, &mut written, &mut groups);
        }
        // Branch — indirect
        Br | Blr | Ret => {
            meta_branch_indirect(mnemonic, &regs, &mut read, &mut implicit_read, &mut groups);
        }
        // Branch — conditional
        BCond => {
            meta_branch_cond(&mut implicit_read, &mut groups);
        }
        // Branch — compare
        Cbz | Cbnz | Tbz | Tbnz => {
            meta_branch_compare(&regs, &mut read, &mut groups);
        }
        // Conditional select aliases
        Cset | Csetm | Cinc | Cinv | Cneg => {
            meta_conditional(&regs, &mut written, &mut implicit_read, &mut groups);
        }
        // System / exceptions / barriers
        Nop | Svc | Hvc | Smc | Brk | Isb | Dsb | Dmb => {
            meta_system(mnemonic, &mut groups);
        }
        Msr | Mrs => {
            groups.push("system".to_string());
        }
        // Loads
        Ldr | Ldrb | Ldrh | Ldrsb | Ldrsh | Ldrsw | Ldur | Ldurb | Ldurh | Ldursb | Ldursh
        | Ldursw | Ldp | Ldpsw | Ldnp | Ldxr | Ldxrb | Ldxrh | Ldxp => {
            meta_load(mnemonic, &regs, &mut written, &mut groups);
        }
        // Stores
        Str | Strb | Strh | Stur | Sturb | Sturh | Stp | Stnp | Stxr | Stxrb | Stxrh | Stxp => {
            meta_store(mnemonic, &regs, &mut written, &mut groups);
        }
        // SIMD/FP — all other instructions
        _ => {
            groups.push("simd".to_string());
        }
    }

    (read, written, implicit_read, implicit_written, groups)
}

// ---------------------------------------------------------------------------
// Per-category metadata helpers
// ---------------------------------------------------------------------------

fn meta_data_proc(
    regs: &[RegisterId],
    read: &mut Vec<RegisterId>,
    written: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    if !regs.is_empty() {
        written.push(regs[0]);
    }
    for reg in regs.iter().skip(1) {
        read.push(*reg);
    }
    groups.push("data".to_string());
}

fn meta_compare(
    regs: &[RegisterId],
    read: &mut Vec<RegisterId>,
    written: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    for reg in regs {
        read.push(*reg);
    }
    written.push(RegisterId {
        architecture: ArchitectureId::Arm,
        id: 33,
    }); // NZCV
    groups.push("data".to_string());
}

fn meta_move(regs: &[RegisterId], written: &mut Vec<RegisterId>, groups: &mut Vec<String>) {
    if !regs.is_empty() {
        written.push(regs[0]);
    }
    groups.push("data".to_string());
}

fn meta_branch_direct(mnemonic: Mnemonic, written: &mut Vec<RegisterId>, groups: &mut Vec<String>) {
    use crate::types::Mnemonic::*;
    groups.push("branch".to_string());
    if mnemonic == Bl {
        written.push(RegisterId {
            architecture: ArchitectureId::Arm,
            id: 30,
        }); // LR
    }
}

fn meta_branch_indirect(
    mnemonic: Mnemonic,
    regs: &[RegisterId],
    read: &mut Vec<RegisterId>,
    implicit_read: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    use crate::types::Mnemonic::*;
    groups.push("branch".to_string());
    if !regs.is_empty() {
        read.push(regs[0]);
    }
    if mnemonic == Ret {
        implicit_read.push(RegisterId {
            architecture: ArchitectureId::Arm,
            id: 30,
        }); // LR
    }
}

fn meta_branch_cond(implicit_read: &mut Vec<RegisterId>, groups: &mut Vec<String>) {
    groups.push("branch".to_string());
    implicit_read.push(RegisterId {
        architecture: ArchitectureId::Arm,
        id: 33,
    }); // NZCV
}

fn meta_branch_compare(
    regs: &[RegisterId],
    read: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    groups.push("branch".to_string());
    if !regs.is_empty() {
        read.push(regs[0]);
    }
}

fn meta_conditional(
    regs: &[RegisterId],
    written: &mut Vec<RegisterId>,
    implicit_read: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    if !regs.is_empty() {
        written.push(regs[0]);
    }
    implicit_read.push(RegisterId {
        architecture: ArchitectureId::Arm,
        id: 33,
    }); // NZCV
    groups.push("data".to_string());
}

fn meta_system(mnemonic: Mnemonic, groups: &mut Vec<String>) {
    use crate::types::Mnemonic::*;
    if matches!(mnemonic, Svc | Hvc | Smc | Brk) {
        groups.push("interrupt".to_string());
    } else {
        groups.push("system".to_string());
    }
}

fn meta_load(
    mnemonic: Mnemonic,
    regs: &[RegisterId],
    written: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    use crate::types::Mnemonic::*;
    groups.push("load".to_string());
    match mnemonic {
        Ldp | Ldpsw | Ldxp | Ldnp => {
            if regs.len() >= 2 {
                written.push(regs[0]);
                written.push(regs[1]);
            }
        }
        Ldr | Ldrb | Ldrh | Ldrsb | Ldrsh | Ldrsw | Ldur | Ldurb | Ldurh | Ldursb | Ldursh
        | Ldursw | Ldxr | Ldxrb | Ldxrh => {
            if !regs.is_empty() {
                written.push(regs[0]);
            }
        }
        _ => {}
    }
}

fn meta_store(
    mnemonic: Mnemonic,
    regs: &[RegisterId],
    written: &mut Vec<RegisterId>,
    groups: &mut Vec<String>,
) {
    use crate::types::Mnemonic::*;
    groups.push("store".to_string());
    match mnemonic {
        Stxr | Stxrb | Stxrh => {
            if !regs.is_empty() {
                written.push(regs[0]); // Ws (status)
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Render hints
// ---------------------------------------------------------------------------

fn compute_render_hints(mnemonic: Mnemonic, operands: &[Operand]) -> RenderHints {
    use crate::types::Mnemonic::*;

    let mut hints = RenderHints::default();

    // Capstone alias expansion
    let capstone_mnemonic = match mnemonic {
        Orr => {
            // MOV = ORR (immediate) or ORR (register) with XZR
            if is_zero_reg_operand(operands.get(2)) || is_zero_reg_operand(operands.get(1)) {
                Some("mov".to_string())
            } else {
                None
            }
        }
        Sub => {
            // NEG = SUB with XZR as first source
            if is_zero_reg_operand(operands.get(1)) {
                Some("neg".to_string())
            } else {
                None
            }
        }
        Eor => {
            // MVN = EOR with -1 immediate (all ones)
            if is_all_ones_immediate(operands.get(2)) {
                Some("mvn".to_string())
            } else {
                None
            }
        }
        Madd => {
            // MUL = MADD with XZR as accumulator
            if is_zero_reg_operand(operands.get(3)) {
                Some("mul".to_string())
            } else {
                None
            }
        }
        Movz => {
            // MOV = MOVZ with no shift
            if operands.len() == 2 {
                Some("mov".to_string())
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(mnemonic) = capstone_mnemonic {
        hints.capstone_mnemonic = Some(mnemonic.clone());
        // Hide operands that are implied by the alias
        match mnemonic.as_str() {
            "mov" => {
                if let Some(Operand::Register { register }) = operands.get(2)
                    && register.id == 31
                {
                    hints.capstone_hidden_operands.push(2); // Hide XZR
                }
            }
            "neg" => {
                hints.capstone_hidden_operands.push(1); // Hide XZR
            }
            "mvn" => {
                hints.capstone_hidden_operands.push(2); // Hide all-ones imm
            }
            "mul" => {
                hints.capstone_hidden_operands.push(3); // Hide XZR accumulator
            }
            _ => {}
        }
    }

    // For B.cond, set capstone mnemonic to b.<cond> and hide the condition code operand
    if mnemonic == BCond
        && !operands.is_empty()
        && let Some(Operand::Text { value }) = operands.first()
    {
        hints.capstone_mnemonic = Some(format!("b.{}", value));
        hints.capstone_hidden_operands.push(0);
    }

    hints
}

fn is_zero_reg_operand(op: Option<&Operand>) -> bool {
    matches!(op, Some(Operand::Register { register }) if register.id == 31)
}

fn is_all_ones_immediate(op: Option<&Operand>) -> bool {
    matches!(op, Some(Operand::Immediate { value }) if *value == -1)
}
