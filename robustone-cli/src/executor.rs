//! Command executor module.
//!
//! This module wires together argument parsing, configuration building,
//! and the actual disassembly pipeline exposed through the CLI.

use crate::Cli;
use crate::DisasmConfig;
use crate::disasm::print_instructions;
use crate::disasm::process_input;
use crate::error::CliError::Disassembly;
use crate::error::Result;
use crate::version_info::print_version_info;

use clap::Parser;

/// Top-level CLI entry point with structured error handling.
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        // Print version and supported architecture information on demand.
        print_version_info();
        return Ok(());
    }

    // Build the disassembly configuration and execute the requested action.
    let config = DisasmConfig::config_from_cli(&cli)?;
    let result = process_input(&config).map_err(|e| Disassembly(e.to_string()))?;
    print_instructions(&result, &config);
    Ok(())
}
