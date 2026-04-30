//! Shared decoder utilities for RISC-V instruction decoding.

use crate::ir::{
    ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints,
};
use crate::types::{RiscVInstructionFormat, RiscVOperand, RiscVOperandValue};

/// Build a `DecodedInstruction` from raw operands and metadata.
pub fn build_riscv_decoded_instruction(
    mnemonic: impl Into<String>,
    _format: RiscVInstructionFormat,
    size: usize,
    operands_detail: Vec<RiscVOperand>,
) -> DecodedInstruction {
    let mnemonic = mnemonic.into();
    let mut registers_read = Vec::new();
    let mut registers_written = Vec::new();
    let operands = operands_detail
        .iter()
        .map(|operand| match &operand.value {
            RiscVOperandValue::Register(reg) => {
                let register = RegisterId::riscv(*reg);
                if operand.access.read {
                    registers_read.push(register);
                }
                if operand.access.write {
                    registers_written.push(register);
                }
                Operand::Register { register }
            }
            RiscVOperandValue::Immediate(value) => Operand::Immediate { value: *value },
            RiscVOperandValue::RoundingMode(rm) => Operand::Text {
                value: rounding_mode_name(*rm).to_string(),
            },
            RiscVOperandValue::Memory(memory) => {
                let base = Some(RegisterId::riscv(memory.base));
                if let Some(base_register) = base {
                    registers_read.push(base_register);
                }
                Operand::Memory {
                    base,
                    displacement: memory.disp,
                }
            }
        })
        .collect();

    let (implicit_registers_read, implicit_registers_written) = infer_implicit_registers(&mnemonic);

    DecodedInstruction {
        architecture: ArchitectureId::Riscv,
        address: 0,
        mode: String::new(),
        mnemonic: mnemonic.clone(),
        opcode_id: Some(mnemonic.clone()),
        size,
        raw_bytes: Vec::new(),
        operands,
        registers_read,
        registers_written,
        implicit_registers_read,
        implicit_registers_written,
        groups: infer_groups(&mnemonic),
        status: DecodeStatus::Success,
        render_hints: RenderHints::default(),
    }
}

fn rounding_mode_name(rm: u8) -> &'static str {
    match rm {
        0 => "rne",
        1 => "rtz",
        2 => "rdn",
        3 => "rup",
        4 => "rmm",
        5 => "reserved5",
        6 => "reserved6",
        7 => "dyn",
        _ => "unknown",
    }
}

// LEGACY: Phase 5 will migrate all mnemonic-based group inference to spec-level
// InstructionGroup/EffectSpec classification. Do NOT add new mnemonic patterns here.
/// Infer instruction groups from mnemonic.
pub fn infer_groups(mnemonic: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let is_atomic =
        mnemonic.starts_with("amo") || mnemonic.starts_with("lr.") || mnemonic.starts_with("sc.");

    if mnemonic.starts_with("c.") {
        groups.push("compressed".to_string());
    }

    if mnemonic.starts_with('b') || matches!(mnemonic, "c.beqz" | "c.bnez") {
        groups.push("branch".to_string());
    }
    if mnemonic.contains("jal") || matches!(mnemonic, "j" | "c.j" | "c.jal" | "c.jr" | "c.jalr") {
        groups.push("control_flow".to_string());
    }
    if matches!(
        mnemonic,
        "lb" | "lh" | "lw" | "ld" | "lbu" | "lhu" | "lwu" | "flw" | "fld" | "c.lw" | "c.lwsp"
    ) {
        groups.push("load".to_string());
    }
    if mnemonic.starts_with("prefetch.") {
        groups.push("load".to_string());
    }
    if matches!(
        mnemonic,
        "sb" | "sh" | "sw" | "sd" | "fsw" | "fsd" | "c.sw" | "c.swsp"
    ) {
        groups.push("store".to_string());
    }
    if is_atomic {
        groups.push("atomic".to_string());
    }
    let has_fp_suffix = !is_atomic
        && (mnemonic.ends_with(".s")
            || mnemonic.ends_with(".d")
            || mnemonic.contains(".s.")
            || mnemonic.contains(".d."));

    if (mnemonic.starts_with('f') && !matches!(mnemonic, "fence" | "fence.i")) || has_fp_suffix {
        groups.push("floating_point".to_string());
    }
    if mnemonic.starts_with("fcvt") || mnemonic.starts_with("fmv") || mnemonic.starts_with("fclass")
    {
        groups.push("conversion".to_string());
    }
    if mnemonic.starts_with("feq") || mnemonic.starts_with("flt") || mnemonic.starts_with("fle") {
        groups.push("compare".to_string());
    }
    if mnemonic.starts_with("csr")
        || matches!(
            mnemonic,
            "ecall" | "ebreak" | "uret" | "sret" | "mret" | "wfi" | "sfence.vma"
        )
    {
        groups.push("system".to_string());
    }
    if groups.is_empty() {
        groups.push("arithmetic".to_string());
    }

    groups
}

/// Infer implicit registers read/written for special instructions.
pub fn infer_implicit_registers(mnemonic: &str) -> (Vec<RegisterId>, Vec<RegisterId>) {
    match mnemonic {
        "c.jal" | "c.jalr" => (Vec::new(), vec![RegisterId::riscv(1)]),
        _ => (Vec::new(), Vec::new()),
    }
}
