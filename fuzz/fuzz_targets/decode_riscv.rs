#![no_main]

use libfuzzer_sys::fuzz_target;
use robustone_core::ArchitectureDispatcher;
use robustone_riscv::RiscVHandler;

fuzz_target!(|data: &[u8]| {
    let mut dispatcher = ArchitectureDispatcher::new();
    dispatcher.register(Box::new(RiscVHandler::new()));
    let _ = dispatcher.decode_instruction(data, "riscv32", 0);
    let _ = dispatcher.decode_instruction(data, "riscv64", 0);
});
