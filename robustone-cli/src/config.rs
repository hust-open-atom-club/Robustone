use crate::arch::ArchitectureSpec;
use crate::command::{DisplayOptions, ValidatedConfig};
use crate::error::{CliError, Result};

/// High-level disassembly configuration that unifies all options.
#[derive(Debug, Clone)]
pub struct DisasmConfig {
    pub arch_spec: ArchitectureSpec,
    pub hex_bytes: Vec<u8>,
    pub start_address: u64,
    pub display_options: DisplayOptions,
    pub skip_data: bool,
}

impl DisasmConfig {
    /// Create a disassembly configuration from validated CLI input.
    pub fn from_validated_config(mut config: ValidatedConfig) -> Result<Self> {
        // Parse and validate architecture specification
        let arch_mode = config.arch_mode.take().ok_or_else(|| {
            CliError::validation("arch_mode", "Architecture specification is required")
        })?;
        let arch_spec = ArchitectureSpec::parse(&arch_mode)
            .map_err(|e| CliError::parse("architecture", e.to_string()))?;

        // Get hex bytes (already validated in command.rs)
        let hex_bytes = config.hex_code.take().ok_or_else(|| {
            CliError::validation("hex_code", "Hexadecimal code is required for disassembly")
        })?;

        Ok(DisasmConfig {
            arch_spec,
            hex_bytes,
            start_address: config.address_or_default(),
            display_options: config.display_options(),
            skip_data: config.skip_data,
        })
    }

    /// Legacy method for backward compatibility.
    /// Builds a configuration from CLI input and performs full validation.
    pub fn config_from_cli(cli: &crate::command::Cli) -> Result<Self> {
        let validated = cli.validate()?;
        Self::from_validated_config(validated)
    }

    /// Get the architecture name as a string.
    pub fn arch_name(&self) -> &str {
        self.arch_spec.arch.name()
    }

    /// Check if detailed output is enabled.
    pub fn is_detailed(&self) -> bool {
        self.display_options.detailed || self.display_options.real_detail
    }

    /// Get the hex code as formatted words for display.
    pub fn hex_words(&self) -> Vec<String> {
        self.hex_bytes
            .chunks(4)
            .map(|chunk| {
                let word = chunk
                    .iter()
                    .rev()
                    .fold(0u32, |acc, &byte| (acc << 8) | byte as u32);
                format!("{word:08x}")
            })
            .collect()
    }

    /// Get the raw hex code as a string.
    pub fn hex_string(&self) -> String {
        hex::encode(&self.hex_bytes)
    }

    /// Get the number of instructions (estimated based on architecture).
    pub fn estimated_instruction_count(&self) -> usize {
        match self.arch_spec.arch.name() {
            "riscv32" | "riscv64" => self.hex_bytes.len() / 4, // RISC-V instructions are 4 bytes
            "arm" | "arm64" => self.hex_bytes.len() / 4, // ARM instructions are typically 4 bytes
            "x86" | "x86_64" => self.hex_bytes.len(),    // x86 has variable instruction length
            _ => self.hex_bytes.len() / 4,               // Default estimate
        }
    }

    /// Validate that the configuration is sufficient for disassembly.
    pub fn validate_for_disassembly(&self) -> Result<()> {
        if self.hex_bytes.is_empty() {
            return Err(CliError::validation(
                "hex_code",
                "No hexadecimal data provided for disassembly",
            ));
        }

        // Architecture-specific validation
        if self.arch_spec.arch.name().starts_with("riscv") && !self.hex_bytes.len().is_multiple_of(2) {
            return Err(CliError::validation(
                "hex_code",
                "RISC-V hex code must have even number of bytes",
            ));
        }

        Ok(())
    }
}

/// Configuration for output formatting and display options.
#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub show_address: bool,
    pub show_hex: bool,
    pub show_bytes: bool,
    pub address_width: usize,
    pub hex_width: usize,
}

impl OutputConfig {
    /// Create output configuration based on display options.
    pub fn from_display_options(display: &DisplayOptions) -> Self {
        Self {
            show_address: true,
            show_hex: display.detailed,
            show_bytes: display.real_detail,
            address_width: 8,
            hex_width: 8,
        }
    }

    /// Create minimal output configuration for brief display.
    pub fn minimal() -> Self {
        Self {
            show_address: false,
            show_hex: false,
            show_bytes: false,
            address_width: 0,
            hex_width: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{DisplayOptions, ValidatedConfig};

    #[test]
    fn test_config_creation() {
        let config = ValidatedConfig {
            arch_mode: Some("riscv32".to_string()),
            hex_code: Some(vec![0x93, 0x00, 0x10, 0x00]),
            address: Some(0x1000),
            detailed: true,
            alias_regs: false,
            real_detail: false,
            skip_data: false,
            unsigned_immediate: false,
            version: false,
        };

        let disasm_config = DisasmConfig::from_validated_config(config).unwrap();
        assert_eq!(disasm_config.arch_name(), "riscv32");
        assert_eq!(disasm_config.start_address, 0x1000);
        assert_eq!(disasm_config.hex_bytes.len(), 4);
    }

    #[test]
    fn test_output_config() {
        let display = DisplayOptions {
            detailed: true,
            alias_regs: false,
            real_detail: false,
            unsigned_immediate: false,
        };

        let output = OutputConfig::from_display_options(&display);
        assert!(output.show_hex);
        assert!(!output.show_bytes);
    }
}
