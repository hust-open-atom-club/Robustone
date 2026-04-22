use robustone::*;

fn main() {
    let dispatcher = dispatcher();

    let test_instructions = vec![
        "93000000", // addi ra, zero, 0
        "93010000", // addi gp, zero, 0
        "93020000", // addi t0, zero, 0
        "13030000", // addi t1, zero, 0
        "13050000", // addi a0, zero, 0
        "03250500", "130101ff", "13000000", // addi zero, zero, 0
        "67800000", "93001000", // addi ra, zero, 1
        "4101",     // c.addi4spn
    ];

    for hex in test_instructions {
        println!("Testing: {hex}");
        let parser = robustone::utils::HexParser::new();
        let bytes = parser.parse(hex, None).unwrap();
        let (instruction, _) = dispatcher.disassemble_bytes(&bytes, "riscv32", 0).unwrap();
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
