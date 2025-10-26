use crate::error::{CliError, Result};
use crate::utils::validate_architecture_legacy as validate_architecture;
use clap::Parser;

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
        help = "Target architecture with optional modes (e.g., riscv32, arm+thumb, x86+intel)",
        long_help = "Specify the target architecture and optional mode modifiers.\n\
Examples: riscv32, riscv64, arm+thumb, arm+v8, x86+intel, x86+att"
    )]
    #[arg(value_parser = validate_architecture)]
    pub arch_mode: Option<String>,

    /// Hexadecimal machine code to disassemble (for example `"00100093"`).
    #[arg(
        help = "Hexadecimal machine code to disassemble",
        long_help = "Provide the machine code as a hexadecimal string to be disassembled.\n\
Example: \"00100093\" for RISC-V addi instruction"
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
        help = "Use Capstone register aliases instead of LLVM names",
        long_help = "Display register names using Capstone's alias naming convention rather than LLVM-style names"
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
        help = "Display immediates in unsigned format",
        long_help = "Display immediate values as unsigned integers instead of signed values"
    )]
    pub unsigned_immediate: bool,

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
            version: self.version,
        })
    }

    /// Validate hexadecimal code input.
    fn validate_hex_code(&self) -> Result<Option<Vec<u8>>> {
        match &self.hex_code {
            Some(code) => {
                let clean_code = code.trim().replace("0x", "").replace("0X", "");
                if clean_code.is_empty() {
                    return Err(CliError::validation("hex_code", "Empty hex code provided"));
                }
                if clean_code.len() % 2 != 0 {
                    return Err(CliError::validation(
                        "hex_code",
                        "Hex code must have even number of characters",
                    ));
                }

                hex::decode(&clean_code)
                    .map(Some)
                    .map_err(|e| CliError::validation("hex_code", format!("Invalid hex code: {e}")))
            }
            None => Ok(None),
        }
    }

    /// Validate address input.
    fn validate_address(&self) -> Result<Option<u64>> {
        match &self.address {
            Some(addr) => {
                let clean_addr = addr.trim().replace("0x", "").replace("0X", "");
                if clean_addr.is_empty() {
                    return Err(CliError::validation("address", "Empty address provided"));
                }

                u64::from_str_radix(&clean_addr, 16).map(Some).map_err(|_| {
                    CliError::validation("address", "Invalid hexadecimal address format")
                })
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
    pub hex_code: Option<Vec<u8>>,
    pub address: Option<u64>,
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub skip_data: bool,
    pub unsigned_immediate: bool,
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
}
