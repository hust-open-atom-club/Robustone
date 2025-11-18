use robustone::*;

fn main() {
    let dispatcher = ArchitectureDispatcher::new();

    let test_instructions = vec![
        "00000093", // addi ra, zero, 0
        "00000193", // addi gp, zero, 0
        "00000293", // addi t0, zero, 0
        "00000313", // addi t1, zero, 0
        "00000513", // addi a0, zero, 0
        "00052503", // lw a0, 0(zero)
        "13000513", // addi a0, zero, 304
        "00000013", // addi zero, zero, 0
        "00008067", // jalr zero, 0(ra)
        "00100093", // addi ra, zero, 1
        "4101",     // c.addi4spn
    ];

    for hex in test_instructions {
        println!("Testing: {hex}");
        let instruction = dispatcher.disassemble(hex, "riscv32".to_string());
        println!(
            "  Result: {} {}",
            instruction.mnemonic, instruction.operands
        );
        println!("  Bytes: {:02x?}", instruction.bytes);
        println!("  Size: {}", instruction.size);

        // Debug: calculate the instruction value and opcode
        if instruction.bytes.len() == 4 {
            let instr_value = (instruction.bytes[0] as u32)
                | ((instruction.bytes[1] as u32) << 8)
                | ((instruction.bytes[2] as u32) << 16)
                | ((instruction.bytes[3] as u32) << 24);
            let opcode = instr_value & 0x7F;
            println!("  Instruction value: 0x{instr_value:08x}");
            println!("  Opcode: 0x{opcode:02x}");

            // Decode fields for I-type instructions
            if opcode == 0x03 {
                // load instructions
                let rd = (instr_value >> 7) & 0x1F;
                let funct3 = (instr_value >> 12) & 0x7;
                let rs1 = (instr_value >> 15) & 0x1F;
                let imm = (instr_value >> 20) & 0xFFF;
                println!("  I-type: rd={rd}, funct3={funct3}, rs1={rs1}, imm={imm}");
            }
        }
        println!();
    }
}
