#![no_main]

use clap::Parser;
use libfuzzer_sys::fuzz_target;
use robustone_cli::{Cli, CliExecutor, DisasmConfig};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };

    let mut args = vec!["robustone".to_string()];
    args.extend(text.split_whitespace().take(8).map(str::to_string));

    let Ok(cli) = Cli::try_parse_from(args) else {
        return;
    };
    if cli.should_show_version() || !cli.has_disassembly_input() {
        return;
    }

    let Ok(config) = DisasmConfig::config_from_cli(&cli) else {
        return;
    };

    let executor = CliExecutor::new();
    let _ = executor.execute_to_string(&config);
});
