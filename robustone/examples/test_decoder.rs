use robustone::*;

fn main() {
    let dispatcher = ArchitectureDispatcher::new();

    println!(
        "Supported architectures: {:?}",
        dispatcher.supported_architectures()
    );

    // Test AUIPC instruction - 0x97000000 (little-endian bytes)
    let instruction = dispatcher.disassemble("97000000", "riscv32".to_string());
    println!("Instruction: {}", instruction.mnemonic);
    println!("Operands: {}", instruction.operands);
    println!("Bytes: {:02x?}", instruction.bytes);

    // Test ADD instruction - 0x00000033 (little-endian bytes)
    let instruction2 = dispatcher.disassemble("33000000", "riscv32".to_string());
    println!("Instruction2: {}", instruction2.mnemonic);
    println!("Operands2: {}", instruction2.operands);
    println!("Bytes2: {:02x?}", instruction2.bytes);
}
