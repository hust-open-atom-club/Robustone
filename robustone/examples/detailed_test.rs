use robustone::*;

fn main() {
    let dispatcher = ArchitectureDispatcher::new();

    // Test the ADDI instruction - this should produce detailed operand information
    let instruction = dispatcher.disassemble("13000513", "riscv32".to_string());

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
        ("97000000", "auipc zero, 0x97000"),
        ("33000000", "add zero, zero, t0"),
        ("13000513", "addi t0, t0, 5"),
    ];

    for (hex, _expected) in test_instructions {
        let instr = dispatcher.disassemble(hex, "riscv32".to_string());
        println!("{} -> {} {}", hex, instr.mnemonic, instr.operands);
        if instr.mnemonic != "unknown" && instr.mnemonic != "c.unimp" {
            println!("  ✓ Success");
        } else {
            println!("  ✗ Failed");
        }
    }
}
