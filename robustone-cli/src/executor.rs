//! Command executor module.
//!
//! This module wires together argument parsing, configuration building,
//! and the actual disassembly pipeline exposed through the CLI.

use crate::command::{Cli, DisplayOptions};
use crate::config::{DisasmConfig, OutputConfig};
use crate::disasm::{DisassemblyEngine, DisassemblyFormatter, DisassemblyIssue, DisassemblyResult};
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
            engine: DisassemblyEngine::new("riscv64"),
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
        let validated_config = match cli.validate() {
            Ok(config) => config,
            Err(error) if cli.json => {
                println!(
                    "{}",
                    self.render_cli_error_json(&cli, &error, "validate_cli")
                );
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(error),
        };

        // Create disassembly configuration
        let disasm_config = match DisasmConfig::from_validated_config(validated_config) {
            Ok(config) => config,
            Err(error) if cli.json => {
                println!(
                    "{}",
                    self.render_cli_error_json(&cli, &error, "build_config")
                );
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(error),
        };

        // Execute the appropriate action
        if cli.has_disassembly_input() {
            self.execute_disassembly(&disasm_config)
        } else if cli.json {
            println!(
                "{}",
                self.render_cli_error_json(
                    &cli,
                    &CliError::MissingArgument("hex_code".to_string()),
                    "validate_cli",
                )
            );
            Err(CliError::reported(1))
        } else {
            Err(CliError::MissingArgument("hex_code".to_string()))
        }
    }

    /// Execute the disassembly pipeline.
    fn execute_disassembly(&self, config: &DisasmConfig) -> Result<()> {
        // Validate the configuration for disassembly
        match config.validate_for_disassembly() {
            Ok(()) => {}
            Err(error) if config.display_options.json => {
                println!(
                    "{}",
                    self.render_config_error_json(config, &error, "validate_disassembly_config")
                );
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(error),
        }

        // Create engine with correct architecture
        let arch = config.arch_name();
        let engine = DisassemblyEngine::new(arch);

        // Perform the disassembly
        let result = match self.engine.disassemble(config) {
            Ok(result) => result,
            Err(error) if config.display_options.json => {
                println!("{}", self.render_fatal_json(config, &error));
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(CliError::disassembly(&error)),
        };
        let result = match engine.disassemble(config) {
            Ok(result) => result,
            Err(error) if config.display_options.json => {
                println!("{}", self.render_fatal_json(config, &error));
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(CliError::disassembly(&error)),
        };

        // Format and output the results
        let output_config = config.output_config();
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
        match config.validate_for_disassembly() {
            Ok(()) => {}
            Err(error) if config.display_options.json => {
                println!(
                    "{}",
                    self.render_config_error_json(config, &error, "validate_disassembly_config")
                );
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(error),
        }

        let result = match self.engine.disassemble(config) {
            Ok(result) => result,
            Err(error) if config.display_options.json => {
                println!("{}", self.render_fatal_json(config, &error));
                return Err(CliError::reported(1));
            }
            Err(error) => return Err(CliError::disassembly(&error)),
        };

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
        match config.validate_for_disassembly() {
            Ok(()) => {}
            Err(error) if config.display_options.json => {
                return Ok(self.render_config_error_json(
                    config,
                    &error,
                    "validate_disassembly_config",
                ));
            }
            Err(error) => return Err(error),
        }

        let result = match self.engine.disassemble(config) {
            Ok(result) => result,
            Err(error) if config.display_options.json => {
                return Ok(self.render_fatal_json(config, &error));
            }
            Err(error) => return Err(CliError::disassembly(&error)),
        };

        let output_config = config.output_config();
        let formatter = DisassemblyFormatter::new(output_config);

        Ok(formatter.format(&result))
    }

    /// Execute disassembly with minimal output (mnemonics only).
    pub fn execute_minimal(&self, config: &DisasmConfig) -> Result<String> {
        config.validate_for_disassembly()?;

        let result = self
            .engine
            .disassemble(config)
            .map_err(|error| CliError::disassembly(&error))?;

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

    fn render_fatal_json(
        &self,
        config: &DisasmConfig,
        error: &robustone_core::DisasmError,
    ) -> String {
        self.render_issue_json(
            config.start_address,
            config.arch_name().to_string(),
            config.output_config(),
            DisassemblyIssue::from_core_error(
                error,
                "decode_instruction",
                config.arch_name(),
                config.start_address,
                0,
                &config.hex_bytes,
            ),
        )
    }

    fn render_config_error_json(
        &self,
        config: &DisasmConfig,
        error: &CliError,
        operation: &str,
    ) -> String {
        self.render_issue_json(
            config.start_address,
            config.arch_name().to_string(),
            config.output_config(),
            DisassemblyIssue::from_cli_error(
                error,
                operation,
                Some(config.arch_name().to_string()),
                Some(config.start_address),
            ),
        )
    }

    fn render_cli_error_json(&self, cli: &Cli, error: &CliError, operation: &str) -> String {
        self.render_issue_json(
            0,
            cli.arch_mode
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            OutputConfig::from_display_options(&DisplayOptions {
                detailed: cli.detailed,
                alias_regs: cli.alias_regs,
                real_detail: cli.real_detail,
                unsigned_immediate: cli.unsigned_immediate,
                json: cli.json,
            }),
            DisassemblyIssue::from_cli_error(error, operation, cli.arch_mode.clone(), None),
        )
    }

    fn render_issue_json(
        &self,
        start_address: u64,
        architecture: String,
        output_config: OutputConfig,
        issue: DisassemblyIssue,
    ) -> String {
        let mut result = DisassemblyResult::new(start_address, architecture);
        result.add_error(issue);
        let formatter = DisassemblyFormatter::new(output_config);
        formatter.format(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::ArchitectureSpec;
    use crate::command::DisplayOptions;
    use crate::config::DisasmConfig;
    use serde_json::Value;

    #[test]
    fn test_executor_creation() {
        let _executor = CliExecutor::new();
        // Basic test that executor can be created
    }

    #[test]
    fn test_executor_default() {
        let _executor = CliExecutor::default();
        // Basic test that default executor works
    }

    #[test]
    fn test_execute_to_string_returns_json_for_fatal_decode_errors() {
        let executor = CliExecutor::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0xff, 0xff, 0xff, 0xff],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: false,
        };

        let output = executor
            .execute_to_string(&config)
            .expect("json mode should render fatal errors as JSON");
        let parsed: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["errors"][0]["kind"], "invalid_encoding");
        assert_eq!(parsed["bytes_processed"], 0);
    }

    #[test]
    fn test_execute_to_string_returns_json_for_validation_errors() {
        let executor = CliExecutor::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x93],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: false,
        };

        let output = executor
            .execute_to_string(&config)
            .expect("json mode should render validation errors as JSON");
        let parsed: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["errors"][0]["kind"], "validation_error");
        assert_eq!(parsed["bytes_processed"], 0);
    }
}
