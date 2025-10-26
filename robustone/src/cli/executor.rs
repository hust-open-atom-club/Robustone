//! Command executor module.
//!
//! This module wires together argument parsing, configuration building,
//! and the actual disassembly pipeline exposed through the CLI.

use crate::cli::Cli;
use crate::cli::DisasmConfig;
use crate::cli::disasm::print_instructions;
use crate::cli::disasm::process_input;
use crate::cli::error::CliError::Disassembly;
use crate::cli::error::Result;
use crate::cli::version_info::print_version_info;

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
