#![no_main]

use libfuzzer_sys::fuzz_target;
use robustone_core::ArchitectureDispatcher;
use robustone_loongarch::LoongArchHandler;

fuzz_target!(|data: &[u8]| {
    let mut dispatcher = ArchitectureDispatcher::new();
    dispatcher.register(Box::new(LoongArchHandler::new()));
    let _ = dispatcher.decode_instruction(data, "loongarch64", 0);
});
