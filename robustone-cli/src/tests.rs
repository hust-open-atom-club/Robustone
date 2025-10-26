use crate::error::{CliError, ValidationError};
use crate::{Architecture, ArchitectureSpec, Cli, DisasmConfig};
use clap::Parser;

#[test]
fn test_cli_basic_parsing() {
    // Verify that the minimal CLI invocation parses successfully.
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    assert_eq!(cli.arch_mode.as_deref().unwrap(), "riscv32");
    assert_eq!(cli.hex_code.as_deref().unwrap(), "00100093");
    // Address is optional; when omitted it should remain `None` (defaulting to zero later).
    assert_eq!(cli.address.as_deref().unwrap_or(""), "");
    assert!(!cli.detailed);
    assert!(!cli.version);
}

#[test]
fn test_cli_with_options() {
    // Ensure options and modifiers are forwarded correctly (e.g. alias register flag).
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
    // The version flag should be recognised without additional arguments.
    let args = vec!["robustone", "-v"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    assert!(cli.version);
}

#[test]
fn test_architecture_parsing() {
    // Baseline architecture parsing should accept supported names.
    assert!(Architecture::parse("riscv32").is_ok());
    assert!(Architecture::parse("riscv64").is_ok());
    assert!(Architecture::parse("arm").is_ok());
    assert!(Architecture::parse("x86").is_ok());
    assert!(Architecture::parse("x86-64").is_ok());

    // Aliases should resolve to the same architectures.
    assert!(Architecture::parse("x86_64").is_ok());
    assert!(Architecture::parse("x64").is_ok());
    assert!(Architecture::parse("ppc").is_ok());
    assert!(Architecture::parse("s390x").is_ok());

    // Invalid inputs should be rejected.
    assert!(Architecture::parse("invalid").is_err());
}

#[test]
fn test_architecture_spec_parsing() {
    // Base architecture specifications should parse without modifiers.
    let spec = ArchitectureSpec::parse("riscv32").unwrap();
    assert!(matches!(spec.arch, Architecture::Riscv32));

    // Specifications with modifiers should populate the option list.
    let spec = ArchitectureSpec::parse("riscv32+intel").unwrap();
    assert!(matches!(spec.arch, Architecture::Riscv32));
    assert!(spec.options.contains(&"intel".to_string()));

    let spec = ArchitectureSpec::parse("arm+thumb").unwrap();
    assert!(matches!(spec.arch, Architecture::Arm));
    assert!(spec.options.contains(&"thumb".to_string()));

    // Invalid modifiers or architectures should fail fast.
    assert!(ArchitectureSpec::parse("invalid_arch").is_err());
    assert!(ArchitectureSpec::parse("riscv32+invalid_mod").is_err());
}

#[test]
fn test_hex_validation() {
    use crate::parse_hex_code;

    // Valid hexadecimal payloads should parse successfully.
    assert!(parse_hex_code("00100093").is_ok());
    assert!(parse_hex_code("ff010113").is_ok());
    assert!(parse_hex_code("1a2b3c4d").is_ok());

    // Inputs with whitespace or prefixes are normalised automatically.
    assert!(parse_hex_code("00 10 00 93").is_ok());
    assert!(parse_hex_code("0x00100093").is_ok());

    // Malformed input should surface validation errors.
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
    use crate::parse_address;

    // Accept canonical and prefixed hexadecimal addresses.
    assert_eq!(parse_address("0").unwrap(), 0);
    assert_eq!(parse_address("80000000").unwrap(), 0x80000000);
    assert_eq!(parse_address("0x80000000").unwrap(), 0x80000000);
    assert_eq!(parse_address("0X80000000").unwrap(), 0x80000000);

    // Reject empty and malformed addresses.
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
    // Building a configuration from minimal arguments should succeed.
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");

    let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

    assert!(matches!(config.arch_spec.arch, Architecture::Riscv32));
    assert_eq!(config.hex_words, vec!["0x00100093"]);
    assert_eq!(config.start_address, 0);
    assert!(!config.detailed);
}

#[test]
fn test_config_with_complex_options() {
    // Complex combinations of flags and modifiers should be preserved.
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
    assert_eq!(config.hex_words, vec!["0xff010113"]);
    assert_eq!(config.start_address, 0x80000000);
    assert!(config.detailed);
    assert!(config.unsigned_immediate);
}

#[test]
fn test_config_missing_required_args() {
    // Missing required arguments should return a descriptive error.
    let args = vec!["robustone"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");
    // Configuration creation must fail when required arguments are absent.
    let cfg = DisasmConfig::config_from_cli(&cli);
    assert!(cfg.is_err());
}

#[test]
fn test_error_handling() {
    use crate::error::Result;

    // Ensure canonical error variants propagate through the Result alias.
    let result: Result<()> = Err(CliError::InvalidHex("Invalid hex code".to_string()));
    assert!(result.is_err());

    let result: Result<()> = Err(CliError::Architecture("Unknown architecture".to_string()));
    assert!(result.is_err());

    let result: Result<()> = Err(CliError::InvalidAddress("Invalid address".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_architecture_categories() {
    // Category helper should map each architecture into the expected group.
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
    // Name helper should return canonical identifiers.
    assert_eq!(Architecture::Riscv32.name(), "riscv32");
    assert_eq!(Architecture::Riscv64.name(), "riscv64");
    assert_eq!(Architecture::Arm.name(), "arm");
    assert_eq!(Architecture::X86_32.name(), "x32");
    assert_eq!(Architecture::X86_64.name(), "x64");
}

#[test]
fn test_architecture_implementation_status() {
    // Implementation status should distinguish supported and pending targets.
    assert!(Architecture::Riscv32.is_implemented());
    assert!(Architecture::Riscv64.is_implemented());
    assert!(!Architecture::Arm.is_implemented());
    assert!(!Architecture::X86_32.is_implemented());
}

#[test]
fn test_all_architectures() {
    // The global architecture registry should list representative entries.
    let archs = Architecture::all_architectures();

    // Confirm that key architectures are always present.
    assert!(archs.iter().any(|a| matches!(a, Architecture::Riscv32)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::Riscv64)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::Arm)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::X86_32)));
    assert!(archs.iter().any(|a| matches!(a, Architecture::X86_64)));

    // Ensure the registry contains a sufficiently broad selection.
    assert!(archs.len() > 10);
}

#[test]
fn test_disasm_config_debug() {
    // The Debug implementation should include salient configuration details.
    let args = vec!["robustone", "riscv32", "00100093"];
    let cli = Cli::try_parse_from(args).expect("Should parse successfully");
    let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

    // Rendered string should mention the type name and selected architecture.
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("DisasmConfig"));
    assert!(debug_str.contains("riscv32"));
}

#[test]
fn test_architecture_spec_debug() {
    // Debug output for architecture specs should embed the base name.
    let spec = ArchitectureSpec::parse("riscv32+intel").unwrap();

    let debug_str = format!("{:?}", spec);
    assert!(debug_str.contains("ArchitectureSpec"));
    assert!(debug_str.contains("riscv32"));
}

#[test]
fn test_error_display() {
    // Human-readable formatting should embed variant context.
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
    use crate::disasm::process_input;

    use super::*;

    #[test]
    fn test_full_cli_workflow() {
        // The full CLI workflow should disassemble a simple program without errors.
        let args = vec!["robustone", "riscv32", "00100093"];
        let cli = Cli::try_parse_from(args).expect("Should parse successfully");

        // Building the configuration should succeed.
        let config = DisasmConfig::config_from_cli(&cli).expect("Should create config");

        // Disassembly should return at least one decoded instruction.
        let result = process_input(&config);
        assert!(result.is_ok());

        let disasm_result = result.unwrap();
        assert!(!disasm_result.instructions.is_empty());
    }

    #[test]
    fn test_error_scenarios() {
        // Validate representative error scenarios.

        // Invalid architectures should be rejected during argument parsing.
        let args = vec!["robustone", "invalid_arch", "00100093"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_err());

        // Invalid hex input should be caught during configuration creation.
        let args = vec!["robustone", "riscv32", "invalid_hex"];
        let cli = Cli::try_parse_from(args).expect("Should parse successfully");
        let config_result = DisasmConfig::config_from_cli(&cli);
        assert!(config_result.is_err());
    }
}
