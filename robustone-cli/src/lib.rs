//! Robustone CLI Library
//!
//! A comprehensive command-line interface for the Robustone disassembly engine.
//! This library provides a clean, modern API for disassembling machine code
//! across multiple architectures with extensive configuration options.

pub mod arch;
pub mod command;
pub mod config;
pub mod disasm;
pub mod error;
pub mod executor;
pub mod utils;
pub mod version_info;

// Re-export modern API surface for convenient use
pub use arch::{Architecture, ArchitectureSpec};
pub use command::{Cli, DisplayOptions, ValidatedConfig};
pub use config::{DisasmConfig, OutputConfig};
pub use disasm::{DisassemblyEngine, DisassemblyFormatter, DisassemblyResult};
pub use error::{CliError, ParseError, Result, ValidationError};
pub use executor::CliExecutor;

/// Main library interface for programmatic use.
pub struct RobustoneCli {
    executor: CliExecutor,
}

impl RobustoneCli {
    /// Create a new CLI instance.
    pub fn new() -> Self {
        Self {
            executor: CliExecutor::new(),
        }
    }

    /// Execute the CLI with current command-line arguments.
    pub fn run(&self) -> Result<()> {
        self.executor.run()
    }

    /// Execute with custom configuration.
    pub fn execute_with_config(&self, config: &DisasmConfig) -> Result<String> {
        self.executor.execute_to_string(config)
    }

    /// Execute with minimal output.
    pub fn execute_minimal(&self, config: &DisasmConfig) -> Result<String> {
        self.executor.execute_minimal(config)
    }

    /// Validate configuration without executing.
    pub fn validate_config(&self) -> Result<()> {
        self.executor.validate_only()
    }
}

impl Default for RobustoneCli {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for quick disassembly.
pub fn disassemble_hex(hex_code: &str, architecture: &str, address: Option<u64>) -> Result<String> {
    let cli = RobustoneCli::new();

    let config = DisasmConfig::from_validated_config(ValidatedConfig {
        arch_mode: Some(architecture.to_string()),
        hex_code: Some(hex_code.to_string()),
        address,
        detailed: false,
        alias_regs: false,
        real_detail: false,
        skip_data: false,
        unsigned_immediate: false,
        version: false,
    })?;

    cli.execute_minimal(&config)
}

// Legacy compatibility functions (deprecated - use modern API instead)
#[deprecated(note = "Use utils::parse_hex_to_bytes instead")]
pub fn parse_hex_code(input: &str) -> std::result::Result<Vec<u8>, ValidationError> {
    use std::result::Result::*;
    match utils::parse_hex_code_legacy(input) {
        Ok(words) => match utils::hex_words_to_bytes(&words) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(ValidationError::InvalidHexChar(' ')),
        },
        Err(e) => Err(match e {
            CliError::Validation { message, .. } => {
                ValidationError::InvalidHexChar(message.chars().next().unwrap_or('x'))
            }
            _ => ValidationError::InvalidHexChar('x'),
        }),
    }
}

#[deprecated(note = "Use utils::parse_address instead")]
pub fn parse_address(input: &str) -> std::result::Result<u64, ValidationError> {
    use std::result::Result::*;
    match utils::parse_address_legacy(input) {
        Ok(addr) => Ok(addr),
        Err(_) => Err(ValidationError::InvalidAddressFormat),
    }
}

#[deprecated(note = "Use utils::validate_architecture instead")]
pub fn validate_architecture(arch_str: &str) -> std::result::Result<String, String> {
    utils::validate_architecture_legacy(arch_str).map_err(|e| e.to_string())
}

#[cfg(test)]
mod smoke_tests {
    use super::*;
    use robustone_core::Instruction;

    #[test]
    fn test_default_cli_wrapper() {
        let cli = RobustoneCli::default();
        let config = DisasmConfig::from_validated_config(ValidatedConfig {
            arch_mode: Some("riscv32".to_string()),
            hex_code: Some("93001000".to_string()),
            address: Some(0x1000),
            detailed: false,
            alias_regs: false,
            real_detail: false,
            skip_data: false,
            unsigned_immediate: false,
            version: false,
        })
        .expect("configuration should be valid");

        let output = cli
            .execute_minimal(&config)
            .expect("disassembly should succeed");
        assert!(output.contains("li"));
    }

    #[test]
    fn test_disassemble_hex() {
        let result = disassemble_hex("93001000", "riscv32", Some(0x1000))
            .expect("RISC-V bytes should disassemble");

        assert!(result.contains("li"));
    }

    #[test]
    fn test_legacy_compatibility() {
        assert!(utils::validate_architecture("riscv32").is_ok());
        assert!(utils::parse_address("0x1000").is_ok());
        assert!(utils::parse_hex_code("1234").is_ok());
    }

    #[test]
    fn test_instruction_default_is_available() {
        let instruction = Instruction::default();
        assert_eq!(instruction.mnemonic, "unknown");
    }
}

#[cfg(test)]
mod tests;
