//! Command executor module.
//!
//! This module wires together argument parsing, configuration building,
//! and the actual disassembly pipeline exposed through the CLI.

use crate::command::Cli;
use crate::config::{DisasmConfig, OutputConfig};
use crate::disasm::{DisassemblyEngine, DisassemblyFormatter};
use crate::error::{CliError, Result};
use crate::version_info::print_version_info;

use clap::Parser;

/// High-level application executor that orchestrates the entire CLI workflow.
pub struct CliExecutor {
    engine: DisassemblyEngine,
}

impl CliExecutor {
    /// Create a new CLI executor.
    pub fn new() -> Self {
        Self {
            engine: DisassemblyEngine::new(),
        }
    }

    /// Execute the CLI workflow.
    pub fn run(&self) -> Result<()> {
        let cli = Cli::parse();
        self.execute_cli(cli)
    }

    /// Execute the workflow with the provided CLI arguments.
    fn execute_cli(&self, cli: Cli) -> Result<()> {
        // Handle version display request
        if cli.should_show_version() {
            print_version_info();
            return Ok(());
        }

        // Validate and process the command-line arguments
        let validated_config = cli.validate()?;

        // Create disassembly configuration
        let disasm_config = DisasmConfig::from_validated_config(validated_config)?;

        // Execute the appropriate action
        if cli.has_disassembly_input() {
            self.execute_disassembly(&disasm_config)
        } else {
            Err(CliError::MissingArgument("hex_code".to_string()))
        }
    }

    /// Execute the disassembly pipeline.
    fn execute_disassembly(&self, config: &DisasmConfig) -> Result<()> {
        // Validate the configuration for disassembly
        config.validate_for_disassembly()?;

        // Perform the disassembly
        let result = self
            .engine
            .disassemble(config)
            .map_err(|e| CliError::Disassembly(e.to_string()))?;

        // Format and output the results
        let output_config = OutputConfig::from_display_options(&config.display_options);
        let formatter = DisassemblyFormatter::new(output_config);

        formatter.print(&result);

        // Print summary if there were errors in skip-data mode
        if !result.is_successful() {
            eprintln!(
                "Warning: {} errors encountered during disassembly",
                result.error_count()
            );
        }

        Ok(())
    }

    /// Execute disassembly with custom output formatting.
    pub fn execute_disassembly_with_formatter(
        &self,
        config: &DisasmConfig,
        formatter: DisassemblyFormatter,
    ) -> Result<()> {
        config.validate_for_disassembly()?;

        let result = self
            .engine
            .disassemble(config)
            .map_err(|e| CliError::Disassembly(e.to_string()))?;

        formatter.print(&result);
        Ok(())
    }
}

impl Default for CliExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for backward compatibility.
/// Top-level CLI entry point with structured error handling.
pub fn run() -> Result<()> {
    let executor = CliExecutor::new();
    executor.run()
}

/// Advanced execution modes for specific use cases.
impl CliExecutor {
    /// Execute disassembly and return the result as a string instead of printing.
    pub fn execute_to_string(&self, config: &DisasmConfig) -> Result<String> {
        config.validate_for_disassembly()?;

        let result = self
            .engine
            .disassemble(config)
            .map_err(|e| CliError::Disassembly(e.to_string()))?;

        let output_config = OutputConfig::from_display_options(&config.display_options);
        let formatter = DisassemblyFormatter::new(output_config);

        Ok(formatter.format(&result))
    }

    /// Execute disassembly with minimal output (mnemonics only).
    pub fn execute_minimal(&self, config: &DisasmConfig) -> Result<String> {
        config.validate_for_disassembly()?;

        let result = self
            .engine
            .disassemble(config)
            .map_err(|e| CliError::Disassembly(e.to_string()))?;

        let output_config = OutputConfig::minimal();
        let formatter = DisassemblyFormatter::new(output_config);

        Ok(formatter.format(&result))
    }

    /// Validate CLI arguments without executing disassembly.
    pub fn validate_only(&self) -> Result<()> {
        let cli = Cli::parse();
        cli.validate()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let _executor = CliExecutor::new();
        // Basic test that executor can be created
        assert!(true);
    }

    #[test]
    fn test_executor_default() {
        let _executor = CliExecutor::default();
        // Basic test that default executor works
        assert!(true);
    }
}
