use crate::arch::ArchitectureSpec;
use crate::command::Cli;

/// Fully validated configuration derived from CLI arguments.
#[derive(Clone)]
pub struct DisasmConfig {
    pub arch_spec: ArchitectureSpec,
    pub hex_words: Vec<String>,
    pub start_address: u64,
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub skip_data: bool,
    pub unsigned_immediate: bool,
}

impl std::fmt::Debug for DisasmConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DisasmConfig {{ arch_spec: {:?}, hex_words: {:?}, start_address: 0x{:x}, detailed: {}, alias_regs: {}, real_detail: {}, skip_data: {}, unsigned_immediate: {} }}",
            self.arch_spec,
            self.hex_words,
            self.start_address,
            self.detailed,
            self.alias_regs,
            self.real_detail,
            self.skip_data,
            self.unsigned_immediate
        )
    }
}

impl DisasmConfig {
    /// Builds a configuration from CLI input and performs full validation.
    pub fn config_from_cli(cli: &Cli) -> crate::error::Result<Self> {
        use crate::error::CliError;

        // Ensure required fields are present.
        let arch_mode = cli
            .arch_mode
            .as_ref()
            .ok_or_else(|| CliError::MissingArgument("arch_mode".to_string()))?;

        let hex_code = cli
            .hex_code
            .as_ref()
            .ok_or_else(|| CliError::MissingArgument("hex_code".to_string()))?;

        // Parse the architecture specification and expand mode modifiers.
        let arch_spec = ArchitectureSpec::parse(arch_mode)
            .map_err(|e| CliError::Architecture(e.to_string()))?;

        let hex_words = crate::utils::parse_hex_code(hex_code)
            .map_err(|e| CliError::InvalidHex(e.to_string()))?;

        // Parse the starting address, defaulting to zero when omitted.
        let address_str = cli.address.as_deref().unwrap_or("0");
        let start_address = crate::utils::parse_address(address_str)
            .map_err(|e| CliError::InvalidAddress(e.to_string()))?;

        Ok(DisasmConfig {
            arch_spec,
            hex_words,
            start_address,
            detailed: cli.detailed,
            alias_regs: cli.alias_regs,
            real_detail: cli.real_detail,
            skip_data: cli.skip_data,
            unsigned_immediate: cli.unsigned_immediate,
        })
    }
}
