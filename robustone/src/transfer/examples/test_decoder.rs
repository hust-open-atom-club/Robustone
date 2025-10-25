use transfer::*;

fn main() {
    let dispatcher = ArchitectureDispatcher::new();

    println!(
        "Supported architectures: {:?}",
        dispatcher.supported_architectures()
    );

    // 测试AUIPC指令 - 0x97000000 (小端序字节)
    let instruction = dispatcher.disassemble("97000000", "riscv32".to_string());
    println!("Instruction: {}", instruction.mnemonic);
    println!("Operands: {}", instruction.operands);
    println!("Bytes: {:02x?}", instruction.bytes);

    // 测试ADD指令 - 0x00000033 (小端序字节)
    let instruction2 = dispatcher.disassemble("33000000", "riscv32".to_string());
    println!("Instruction2: {}", instruction2.mnemonic);
    println!("Operands2: {}", instruction2.operands);
    println!("Bytes2: {:02x?}", instruction2.bytes);
}
