use crate::utils::validate_architecture;
use clap::Parser;

/// Robustone - Capstone-compatible disassembly engine CLI tool (cstool style)
#[derive(Parser, Debug)]
#[command(name = "robustone")]
#[command(about = "Robustone - Capstone-compatible disassembly engine CLI tool")]
#[command(version = "0.1.0")]
#[command(disable_version_flag = true)]
pub struct Cli {
    /// Target architecture plus optional mode modifiers (e.g., `riscv32`, `arm+thumb`, `x86+intel`).
    #[arg(help = "Target architecture with optional modes (e.g., riscv32, arm+thumb, x86+intel)")]
    #[arg(value_parser = validate_architecture)]
    pub arch_mode: Option<String>,

    /// Hexadecimal machine code to disassemble (for example `"00100093"`).
    #[arg(help = "Hexadecimal machine code to disassemble")]
    pub hex_code: Option<String>,

    /// Starting address in hexadecimal notation (defaults to zero when omitted).
    #[arg(help = "Start address in hex format (default: 0)")]
    pub address: Option<String>,

    /// `-d`: emit detailed instruction metadata alongside the mnemonic.
    #[arg(short = 'd', long = "detailed")]
    #[arg(help = "Show detailed instruction information")]
    pub detailed: bool,

    /// `-a`: render register names using Capstone's alias list.
    #[arg(short = 'a', long = "alias-regs")]
    #[arg(help = "Use Capstone register aliases instead of LLVM names")]
    pub alias_regs: bool,

    /// `-r`: request detailed instruction information including alias resolutions.
    #[arg(short = 'r', long = "real-detail")]
    #[arg(help = "Show detailed real instruction information (including aliases)")]
    pub real_detail: bool,

    /// `-s`: enable SKIPDATA mode to step past undecodable bytes.
    #[arg(short = 's', long = "skip-data")]
    #[arg(help = "Decode in SKIPDATA mode (skip unrecognizable data)")]
    pub skip_data: bool,

    /// `-u`: format immediates as unsigned values.
    #[arg(short = 'u', long = "unsigned-immediate")]
    #[arg(help = "Display immediates in unsigned format")]
    pub unsigned_immediate: bool,

    /// `-v`: print version and build metadata instead of disassembling input.
    #[arg(short = 'v', long = "version")]
    #[arg(help = "Show version and build information")]
    pub version: bool,
}
