//! 命令执行器模块
//!
//! 这个模块负责命令分发和执行逻辑。

use crate::cli::disasm::print_instructions;
use crate::cli::disasm::process_input;
use crate::cli::error::CliError::Disassembly;
use crate::cli::error::Result;
use crate::cli::version_info::print_version_info;
use crate::cli::Cli;
use crate::cli::DisasmConfig;

use clap::Parser;

/// 主要的CLI运行函数，使用安全的错误处理
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        // 打印版本与支持信息
        print_version_info();
        return Ok(());
    }

    // 构建反汇编配置并执行
    let config = DisasmConfig::config_from_cli(&cli)?;
    let result = process_input(&config).map_err(|e| Disassembly(e.to_string()))?;
    print_instructions(&result, &config);
    Ok(())
}
