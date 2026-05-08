//! AArch64 instruction debug printer.

use robustone_core::ir::DecodedInstruction;

/// Print a decoded AArch64 instruction for debugging.
pub fn print_instruction(instr: &DecodedInstruction) {
    println!(
        "{:08x}  {}  {}",
        instr.address,
        instr.mnemonic,
        instr
            .operands
            .iter()
            .map(|op| format!("{:?}", op))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
