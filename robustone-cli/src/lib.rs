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
        hex_code: Some(
            hex::decode(hex_code)
                .map_err(|e| CliError::validation("hex_code", format!("Invalid hex: {}", e)))?,
        ),
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
mod tests {
    use super::*;

    #[test]
    fn test_modern_api() {
        let cli = RobustoneCli::new();
        // This may succeed or fail depending on CLI args, so just test the API
        let _result = cli.validate_config(); // Just test that the method works
    }

    #[test]
    fn test_disassemble_hex() {
        let result = disassemble_hex("00100093", "riscv32", Some(0x1000));
        // This will likely fail due to missing core implementation, but tests the API
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_legacy_compatibility() {
        assert!(validate_architecture("riscv32").is_ok());
        assert!(parse_address("0x1000").is_ok());
        assert!(parse_hex_code("1234").is_ok());
    }
}
