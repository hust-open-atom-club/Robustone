use robustone::*;

fn main() {
    let dispatcher = dispatcher();

    println!(
        "Supported architectures: {:?}",
        dispatcher.supported_architectures()
    );

    let parser = robustone::utils::HexParser::new();
    let bytes1 = parser.parse("37010000", None).unwrap();
    let (instruction, _) = dispatcher.disassemble_bytes(&bytes1, "riscv32", 0).unwrap();
    println!("Instruction: {}", instruction.mnemonic);
    println!("Operands: {}", instruction.operands);
    println!("Bytes: {:02x?}", instruction.bytes);

    let bytes2 = parser.parse("b3003100", None).unwrap();
    let (instruction2, _) = dispatcher.disassemble_bytes(&bytes2, "riscv32", 0).unwrap();
    println!("Instruction2: {}", instruction2.mnemonic);
    println!("Operands2: {}", instruction2.operands);
    println!("Bytes2: {:02x?}", instruction2.bytes);
}
