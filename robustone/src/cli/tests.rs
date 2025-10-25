use crate::cli::error::{CliError, ValidationError};
use crate::cli::{Architecture, ArchitectureSpec, Cli, DisasmConfig};
use clap::Parser;

#[test]
fn test_cli_basic_parsing() {
    // 测试基本的CLI解析
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    assert_eq!(cli.arch_mode.as_deref().unwrap(), "riscv32");
    assert_eq!(cli.hex_code.as_deref().unwrap(), "00100093");
    // 地址为可选参数，未提供时应为None（后续从Cli创建配置时默认使用0）
    assert_eq!(cli.address.as_deref().unwrap_or(""), "");
    assert!(!cli.detailed);
    assert!(!cli.version);
}

#[test]
fn test_cli_with_options() {
    // 测试带选项的CLI解析
    // -a: 使用别名寄存器（遵循cstool风格）
    let args = vec![
        "robustone",
        "-d",
        "-a",
        "riscv32+intel",
        "00100093",
        "80000000",
    ];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    assert_eq!(cli.arch_mode.as_deref().unwrap(), "riscv32+intel");
    assert_eq!(cli.hex_code.as_deref().unwrap(), "00100093");
    assert_eq!(cli.address.as_deref().unwrap_or(""), "80000000");
    assert!(cli.detailed);
    assert!(cli.alias_regs);
}

#[test]
fn test_cli_version_option() {
    // 测试版本选项
    let args = vec!["robustone", "-v"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    assert!(cli.version);
}

#[test]
fn test_architecture_parsing() {
    // 测试基础架构解析
    assert!(Architecture::parse("riscv32").is_ok());
    assert!(Architecture::parse("riscv64").is_ok());
    assert!(Architecture::parse("arm").is_ok());
    assert!(Architecture::parse("x86").is_ok());
    assert!(Architecture::parse("x86-64").is_ok());

    // 测试别名
    assert!(Architecture::parse("x86_64").is_ok());
    assert!(Architecture::parse("x64").is_ok());
    assert!(Architecture::parse("ppc").is_ok());
    assert!(Architecture::parse("s390x").is_ok());

    // 测试无效架构
    assert!(Architecture::parse("invalid").is_err());
}

#[test]
fn test_architecture_spec_parsing() {
    // 测试基础架构规范
    let spec = ArchitectureSpec::parse("riscv32").unwrap();
    assert!(matches!(spec.arch, Architecture::Riscv32));

    // 测试带修饰符的架构规范
    let spec = ArchitectureSpec::parse("riscv32+intel").unwrap();
    assert!(matches!(spec.arch, Architecture::Riscv32));
    assert!(spec.options.contains(&"intel".to_string()));

    let spec = ArchitectureSpec::parse("arm+thumb").unwrap();
    assert!(matches!(spec.arch, Architecture::Arm));
    assert!(spec.options.contains(&"thumb".to_string()));

    // 测试无效架构规范
    assert!(ArchitectureSpec::parse("invalid_arch").is_err());
    assert!(ArchitectureSpec::parse("riscv32+invalid_mod").is_err());
}

#[test]
fn test_hex_validation() {
    use crate::cli::parse_hex_code;

    // 测试有效十六进制代码
    assert!(parse_hex_code("00100093").is_ok());
    assert!(parse_hex_code("ff010113").is_ok());
    assert!(parse_hex_code("1a2b3c4d").is_ok());

    // 测试带非十六进制字符的输入
    assert!(parse_hex_code("00 10 00 93").is_ok()); // 应该过滤空格
    assert!(parse_hex_code("0x00100093").is_ok()); // 应该过滤0x前缀

    // 测试无效十六进制代码
    assert!(matches!(
        parse_hex_code(""),
        Err(ValidationError::EmptyHexCode)
    ));
    assert!(matches!(
        parse_hex_code("123"),
        Err(ValidationError::OddHexLength)
    ));
    assert!(matches!(
        parse_hex_code("xyz"),
        Err(ValidationError::InvalidHexChar(_))
    ));
}

#[test]
fn test_address_validation() {
    use crate::cli::parse_address;

    // 测试有效地址
    assert_eq!(parse_address("0").unwrap(), 0);
    assert_eq!(parse_address("80000000").unwrap(), 0x80000000);
    assert_eq!(parse_address("0x80000000").unwrap(), 0x80000000);
    assert_eq!(parse_address("0X80000000").unwrap(), 0x80000000);

    // 测试无效地址
    assert!(matches!(
        parse_address(""),
        Err(ValidationError::EmptyAddress)
    ));
    assert!(matches!(
        parse_address("xyz"),
        Err(ValidationError::InvalidAddressFormat)
    ));
    assert!(matches!(
        parse_address("0x"),
        Err(ValidationError::InvalidAddressFormat)
    ));
}

#[test]
fn test_config_creation() {
    // 测试从CLI创建配置
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

    assert!(matches!(config.arch_spec.arch, Architecture::Riscv32));
    assert!(config.hex_words == vec!["0x00100093"]);
    assert_eq!(config.start_address, 0);
    assert!(!config.detailed);
}

#[test]
fn test_config_with_complex_options() {
    // 测试复杂配置创建
    let args = vec![
        "robustone",
        "-d",
        "-u",
        "riscv32+intel",
        "ff010113",
        "80000000",
    ];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

    assert!(matches!(config.arch_spec.arch, Architecture::Riscv32));
    assert!(config.arch_spec.options.contains(&"intel".to_string()));
    assert_eq!(config.hex_words, vec!["0xff010113", "80000000"]);
    assert_eq!(config.start_address, 0x80000000);
    assert!(config.detailed);
    assert!(config.unsigned_immediate);
}

#[test]
fn test_config_missing_required_args() {
    // 测试缺少必需参数的错误处理
    let args = vec!["robustone"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");
    // 由于缺少必要参数，创建配置应失败
    let cfg = DisasmConfig::config_from_cli(&cli);
    assert!(cfg.is_err());
}

#[test]
fn test_error_handling() {
    use crate::cli::error::Result;

    // 测试错误类型转换
    let result: Result<()> = Err(CliError::InvalidHex("Invalid hex code".to_string()));
    assert!(result.is_err());

    let result: Result<()> = Err(CliError::Architecture("Unknown architecture".to_string()));
    assert!(result.is_err());

    let result: Result<()> = Err(CliError::InvalidAddress("Invalid address".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_architecture_categories() {
    // 测试架构分类
    assert_eq!(Architecture::Riscv32.category(), "RISC-V");
    assert_eq!(Architecture::Riscv64.category(), "RISC-V");
    assert_eq!(Architecture::Arm.category(), "ARM");
    assert_eq!(Architecture::X86_32.category(), "x86");
    assert_eq!(Architecture::X86_64.category(), "x86");
    assert_eq!(Architecture::Mips.category(), "MIPS");
    assert_eq!(Architecture::PowerPC32.category(), "PowerPC");
}

#[test]
fn test_architecture_names() {
    // 测试架构名称
    assert_eq!(Architecture::Riscv32.name(), "riscv32");
    assert_eq!(Architecture::Riscv64.name(), "riscv64");
    assert_eq!(Architecture::Arm.name(), "arm");
    assert_eq!(Architecture::X86_32.name(), "x32");
    assert_eq!(Architecture::X86_64.name(), "x64");
}

#[test]
fn test_architecture_implementation_status() {
    // 测试实现状态
    assert!(Architecture::Riscv32.is_implemented());
    assert!(Architecture::Riscv64.is_implemented());
    assert!(!Architecture::Arm.is_implemented());
    assert!(!Architecture::X86_32.is_implemented());
}

#[test]
fn test_all_architectures() {
    // 测试架构列表
    let archs = Architecture::all_architectures();

    // 确保包含基本架构
    assert!(archs.iter().any(|a| matches!(a, Architecture::Riscv32)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::Riscv64)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::Arm)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::X86_32)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::X86_64)));

    // 确保数量合理（大于10）
    assert!(archs.len() > 10);
}

#[test]
fn test_disasm_config_debug() {
    // 测试配置的Debug输出
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");
    let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

    // 测试Debug实现
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("DisasmConfig"));
    assert!(debug_str.contains("riscv32"));
}

#[test]
fn test_architecture_spec_debug() {
    // 测试架构规范的Debug输出
    let spec = ArchitectureSpec::parse("riscv32+intel").unwrap();

    let debug_str = format!("{:?}", spec);
    assert!(debug_str.contains("ArchitectureSpec"));
    assert!(debug_str.contains("riscv32"));
}

#[test]
fn test_error_display() {
    // 测试错误显示
    let error = CliError::InvalidHex("Invalid hex".to_string());
    let display_str = format!("{}", error);
    assert!(display_str.contains("Invalid hex"));

    let error = CliError::Architecture("Unknown arch".to_string());
    let display_str = format!("{}", error);
    assert!(display_str.contains("Architecture error"));

    let error = CliError::MissingArgument("arg".to_string());
    let display_str = format!("{}", error);
    assert!(display_str.contains("Missing required argument"));
}

#[cfg(test)]
mod integration_tests {
    use crate::cli::disasm::process_input;

    use super::*;

    #[test]
    fn test_full_cli_workflow() {
        // 测试完整的CLI工作流程
        let args = vec!["robustone", "riscv32", "00100093"];
        let cli = Cli::try_parse_from(args).expect("Should parse successfully");

        // 测试配置创建
        let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

        // 测试反汇编（这会调用实际的disasm模块）
        let result = process_input(&config);
        assert!(result.is_ok());

        let disasm_result = result.unwrap();
        assert!(!disasm_result.instructions.is_empty());
    }

    #[test]
    fn test_error_scenarios() {
        // 测试各种错误场景

        // 无效架构
        let args = vec!["robustone", "invalid_arch", "00100093"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_err());

        // 无效十六进制
        let args = vec!["robustone", "riscv32", "invalid_hex"];
        let cli = Cli::try_parse_from(args).expect("Should parse successfully");
        let config_result = DisasmConfig::config_from_cli(&cli);
        assert!(config_result.is_err());
    }
}
