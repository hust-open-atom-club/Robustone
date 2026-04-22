use robustone::*;

fn main() {
    let dispatcher = dispatcher();

    // Test the ADDI instruction - this should produce detailed operand information
    let parser = robustone::utils::HexParser::new();
    let bytes = parser.parse("130101ff", None).unwrap();
    let (instruction, _) = dispatcher.disassemble_bytes(&bytes, "riscv32", 0).unwrap();

    println!("=== Basic Output ===");
    println!("{} {}", instruction.mnemonic, instruction.operands);
    println!("Size: {} bytes", instruction.size);
    println!("Bytes: {:02x?}", instruction.bytes);

    if let Some(detail) = &instruction.detail {
        println!("\n=== Detailed Information ===");
        let regs_read = detail.registers_read();
        let regs_write = detail.registers_written();
        println!("Architecture: {}", detail.architecture_name());
        println!("Registers read: {regs_read:?}");
        println!("Registers written: {regs_write:?}");
    } else {
        println!("\nNo detailed information available");
    }

    // Test additional instructions
    println!("\n=== Testing Multiple Instructions ===");
    let test_instructions = vec![
        ("37010000", "lui sp, 0"),
        ("b3003100", "add ra, sp, gp"),
        ("130101ff", "addi sp, sp, -0x10"),
    ];

    for (hex, _expected) in test_instructions {
        let bytes = parser.parse(hex, None).unwrap();
        let (instr, _) = dispatcher.disassemble_bytes(&bytes, "riscv32", 0).unwrap();
        println!("{} -> {} {}", hex, instr.mnemonic, instr.operands);
        if instr.mnemonic != "unknown" && instr.mnemonic != "c.unimp" {
            println!("  ✓ Success");
        } else {
            println!("  ✗ Failed");
        }
    }
}
