use crate::cli::error::{ParseError, ValidationError};
use crate::cli::utils::validate_architecture;
use clap::Parser;
use std::str::FromStr;

/// Robustone - Capstone-compatible disassembly engine CLI tool (cstool style)
#[derive(Parser, Debug)]
#[command(name = "robustone")]
#[command(about = "Robustone - Capstone-compatible disassembly engine CLI tool")]
#[command(version = "0.1.0")]
#[command(disable_version_flag = true)]
pub struct Cli {
    /// 目标架构和模式 (例如: riscv32, riscv64, arm, x86+intel)
    #[arg(help = "Target architecture with optional modes (e.g., riscv32, arm+thumb, x86+intel)")]
    #[arg(value_parser = validate_architecture)]
    pub arch_mode: Option<String>,

    /// 要反汇编的十六进制机器码 (例如: "00100093")
    #[arg(help = "Hexadecimal machine code to disassemble")]
    pub hex_code: Option<String>,

    /// 起始地址 (十六进制格式，默认: 0)
    #[arg(help = "Start address in hex format (default: 0)")]
    pub address: Option<String>,

    /// -d: 显示指令的详细信息
    #[arg(short = 'd', long = "detailed")]
    #[arg(help = "Show detailed instruction information")]
    pub detailed: bool,

    /// -a: 使用Capstone寄存器别名
    #[arg(short = 'a', long = "alias-regs")]
    #[arg(help = "Use Capstone register aliases instead of LLVM names")]
    pub alias_regs: bool,

    /// -r: 显示真实指令的详细信息（包括别名）
    #[arg(short = 'r', long = "real-detail")]
    #[arg(help = "Show detailed real instruction information (including aliases)")]
    pub real_detail: bool,

    /// -s: 在SKIPDATA模式下解码（跳过无法识别的数据）
    #[arg(short = 's', long = "skip-data")]
    #[arg(help = "Decode in SKIPDATA mode (skip unrecognizable data)")]
    pub skip_data: bool,

    /// -u: 以无符号格式显示立即数
    #[arg(short = 'u', long = "unsigned-immediate")]
    #[arg(help = "Display immediates in unsigned format")]
    pub unsigned_immediate: bool,

    /// -v: 显示版本和构建信息
    #[arg(short = 'v', long = "version")]
    #[arg(help = "Show version and build information")]
    pub version: bool,
}
