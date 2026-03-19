#![no_main]

use libfuzzer_sys::fuzz_target;
use robustone_core::ArchitectureDispatcher;

fuzz_target!(|data: &[u8]| {
    let dispatcher = ArchitectureDispatcher::new();
    let _ = dispatcher.decode_instruction(data, "riscv32", 0);
    let _ = dispatcher.decode_instruction(data, "riscv64", 0);
});
