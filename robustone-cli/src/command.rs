use crate::error::{CliError, Result};
use crate::utils::validate_architecture_legacy as validate_architecture;
use crate::utils::{parse_address_legacy, parse_hex_code_legacy};
use clap::{CommandFactory, Parser};
use robustone_core::all_architecture_capabilities;

/// Robustone - Capstone-compatible disassembly engine CLI tool (cstool style)
#[derive(Parser, Debug)]
#[command(
    name = "robustone",
    about = "Robustone - Capstone-compatible disassembly engine CLI tool",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    disable_version_flag = true
)]
pub struct Cli {
    /// Target architecture plus optional mode modifiers (e.g., `riscv32`, `arm+thumb`, `x86+intel`).
    #[arg(
        help = "Target architecture with optional mode modifiers",
        long_help = "Specify the target architecture and optional mode modifiers.\n\
See the registry-derived architecture support section in `robustone --help` for the current canonical tokens and parser-only placeholders."
    )]
    #[arg(value_parser = validate_architecture)]
    pub arch_mode: Option<String>,

    /// Hexadecimal machine code bytes to disassemble (for example `"93001000"`).
    #[arg(
        help = "Hexadecimal machine code to disassemble",
        long_help = "Provide the machine code as a hexadecimal string to be disassembled.\n\
Example: \"93001000\" for the RISC-V bytes `addi ra, zero, 1`"
    )]
    #[arg(value_names = ["HEX_CODE"])]
    pub hex_code: Option<String>,

    /// Starting address in hexadecimal notation (defaults to zero when omitted).
    #[arg(
        help = "Start address in hex format (default: 0)",
        long_help = "Specify the starting address for disassembly in hexadecimal format.\n\
If not provided, defaults to 0. Prefix with 0x or use plain hex."
    )]
    #[arg(value_names = ["ADDRESS"])]
    pub address: Option<String>,

    // Display options group
    /// `-d`: emit detailed instruction metadata alongside the mnemonic.
    #[arg(
        short = 'd',
        long = "detailed",
        help = "Show detailed instruction information",
        long_help = "Display additional instruction metadata including operands, encoding format, and size"
    )]
    pub detailed: bool,

    /// `-a`: render register names using Capstone's alias list.
    #[arg(
        short = 'a',
        long = "alias-regs",
        help = "Accept alias-register compatibility flag",
        long_help = "Compatibility-accepted flag for alias-oriented register output. The current RISC-V backend already prints Capstone-style aliases by default, so this switch currently acts as an explicit no-op."
    )]
    pub alias_regs: bool,

    /// `-r`: request detailed instruction information including alias resolutions.
    #[arg(
        short = 'r',
        long = "real-detail",
        help = "Show detailed real instruction information (including aliases)",
        long_help = "Show comprehensive instruction details including register alias mappings and operand details"
    )]
    pub real_detail: bool,

    /// `-u`: format immediates as unsigned values.
    #[arg(
        short = 'u',
        long = "unsigned-immediate",
        help = "Render immediates as unsigned when possible",
        long_help = "Render negative immediates using an unsigned hexadecimal form when the current formatter/profile can preserve that view."
    )]
    pub unsigned_immediate: bool,

    /// Emit structured JSON instead of the human-readable view.
    #[arg(
        long = "json",
        help = "Render structured JSON output",
        long_help = "Render the disassembly result as structured JSON built from the shared decode IR."
    )]
    pub json: bool,

    // Decoding options group
    /// `-s`: enable SKIPDATA mode to step past undecodable bytes.
    #[arg(
        short = 's',
        long = "skip-data",
        help = "Decode in SKIPDATA mode (skip unrecognizable data)",
        long_help = "When encountering unrecognizable bytes, treat them as data and continue decoding the next instruction"
    )]
    pub skip_data: bool,

    // System options group
    /// `-v`: print version and build metadata instead of disassembling input.
    #[arg(
        short = 'v',
        long = "version",
        help = "Show version and build information",
        long_help = "Display version number, build timestamp, and supported architectures"
    )]
    pub version: bool,
}

impl Cli {
    /// Validate the CLI arguments and return a configuration.
    pub fn validate(&self) -> Result<ValidatedConfig> {
        let hex_code = self.validate_hex_code()?;
        let address = self.validate_address()?;

        Ok(ValidatedConfig {
            arch_mode: self.arch_mode.clone(),
            hex_code,
            address,
            detailed: self.detailed,
            alias_regs: self.alias_regs,
            real_detail: self.real_detail,
            skip_data: self.skip_data,
            unsigned_immediate: self.unsigned_immediate,
            json: self.json,
            version: self.version,
        })
    }

    /// Validate hexadecimal code input.
    fn validate_hex_code(&self) -> Result<Option<String>> {
        match &self.hex_code {
            Some(code) => {
                if code.trim().is_empty() {
                    return Err(CliError::validation("hex_code", "Empty hex code provided"));
                }

                parse_hex_code_legacy(code)?;
                Ok(Some(code.trim().to_string()))
            }
            None => Ok(None),
        }
    }

    /// Validate address input.
    fn validate_address(&self) -> Result<Option<u64>> {
        match &self.address {
            Some(addr) => {
                if addr.trim().is_empty() {
                    return Err(CliError::validation("address", "Empty address provided"));
                }

                parse_address_legacy(addr).map(Some)
            }
            None => Ok(None),
        }
    }

    /// Check if version information should be displayed.
    pub fn should_show_version(&self) -> bool {
        self.version
    }

    /// Check if the CLI has valid input for disassembly.
    pub fn has_disassembly_input(&self) -> bool {
        self.hex_code.is_some()
    }
}

/// Validated and processed command-line configuration.
#[derive(Debug, Clone)]
pub struct ValidatedConfig {
    pub arch_mode: Option<String>,
    pub hex_code: Option<String>,
    pub address: Option<u64>,
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub skip_data: bool,
    pub unsigned_immediate: bool,
    pub json: bool,
    pub version: bool,
}

impl ValidatedConfig {
    /// Get the starting address, defaulting to 0 if not provided.
    pub fn address_or_default(&self) -> u64 {
        self.address.unwrap_or(0)
    }

    /// Check if any detailed output mode is enabled.
    pub fn is_detailed_mode(&self) -> bool {
        self.detailed || self.real_detail
    }

    /// Get display options as a unified configuration.
    pub fn display_options(&self) -> DisplayOptions {
        DisplayOptions {
            detailed: self.detailed,
            alias_regs: self.alias_regs,
            real_detail: self.real_detail,
            unsigned_immediate: self.unsigned_immediate,
            json: self.json,
        }
    }
}

/// Unified display options for disassembly output.
#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub unsigned_immediate: bool,
    pub json: bool,
}

pub fn render_help_text() -> String {
    let mut command = Cli::command();
    let mut output = Vec::new();
    command
        .write_long_help(&mut output)
        .expect("help rendering should succeed");

    let mut help = String::from_utf8(output).expect("help should be valid UTF-8");
    help.push_str("\n\nArchitecture Support (shared capability registry):\n");

    let mut current_category = "";
    for capability in all_architecture_capabilities() {
        if capability.category != current_category {
            current_category = capability.category;
            help.push_str(&format!("\n  {}:\n", current_category));
        }

        let support_label = if capability.decode_supported {
            "decode-ready"
        } else {
            "parser-only"
        };
        help.push_str(&format!(
            "    - {} [{}]\n",
            capability.canonical_name, support_label
        ));
    }

    help.push_str(
        "\n  Note: tokens marked parser-only are accepted by the CLI parser, but they currently fail with a configuration error before decode because no backend is implemented yet.\n",
    );
    help
}
