#![no_main]

use libfuzzer_sys::fuzz_target;
use robustone_core::utils::{Endianness, HexParser};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };

    let parser = HexParser::new();
    let compat_parser = HexParser::with_endianness(Endianness::Little);

    let _ = parser.parse(text, None);
    let _ = compat_parser.parse(text, Some(Endianness::Little));
    let _ = parser.parse_for_architecture(text, "riscv32");
    let _ = parser.parse_for_architecture(text, "riscv64");
});
