use robustone::*;

fn main() {
    let dispatcher = dispatcher();

    println!(
        "Supported architectures: {:?}",
        dispatcher.supported_architectures()
    );

    let instruction = dispatcher.disassemble("37010000", "riscv32".to_string());
    println!("Instruction: {}", instruction.mnemonic);
    println!("Operands: {}", instruction.operands);
    println!("Bytes: {:02x?}", instruction.bytes);

    let instruction2 = dispatcher.disassemble("b3003100", "riscv32".to_string());
    println!("Instruction2: {}", instruction2.mnemonic);
    println!("Operands2: {}", instruction2.operands);
    println!("Bytes2: {:02x?}", instruction2.bytes);
}
