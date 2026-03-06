use crate::arch::{Architecture, ArchitectureSpec};
use crate::command::Cli;
use crate::config::{DisasmConfig, OutputConfig};
use crate::disasm::{DisassemblyFormatter, process_input};
use clap::Parser;

#[test]
fn test_cli_basic_parsing() {
    let args = vec!["robustone", "riscv32", "93001000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");

    assert_eq!(cli.arch_mode.as_deref(), Some("riscv32"));
    assert_eq!(cli.hex_code.as_deref(), Some("93001000"));
    assert_eq!(cli.address, None);
    assert!(!cli.detailed);
}

#[test]
fn test_architecture_spec_rejects_incompatible_modifier() {
    assert!(ArchitectureSpec::parse("riscv32+intel").is_err());
    assert!(ArchitectureSpec::parse("arm+thumb").is_ok());
    assert!(ArchitectureSpec::parse("x86+intel").is_ok());
}

#[test]
fn test_config_preserves_input_byte_order() {
    let args = vec!["robustone", "riscv32", "93001000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");

    assert_eq!(config.hex_bytes, vec![0x93, 0x00, 0x10, 0x00]);
}

#[test]
fn test_config_rejects_odd_length_hex_instead_of_truncating() {
    let args = vec!["robustone", "riscv32", "9300100"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config_result = DisasmConfig::config_from_cli(&cli);

    assert!(config_result.is_err());
}

#[test]
fn test_config_rejects_unsupported_output_flags() {
    let alias_args = vec!["robustone", "-a", "riscv32", "93001000"];
    let alias_cli = Cli::try_parse_from(alias_args).expect("CLI arguments should parse");
    let alias_result = DisasmConfig::config_from_cli(&alias_cli);
    assert!(alias_result.is_err());

    let unsigned_args = vec!["robustone", "-u", "riscv32", "93001000"];
    let unsigned_cli = Cli::try_parse_from(unsigned_args).expect("CLI arguments should parse");
    let unsigned_result = DisasmConfig::config_from_cli(&unsigned_cli);
    assert!(unsigned_result.is_err());
}

#[test]
fn test_process_input_decodes_expected_instruction() {
    let args = vec!["robustone", "riscv32", "93001000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let result = process_input(&config).expect("disassembly should succeed");

    assert_eq!(result.instructions.len(), 1);
    assert_eq!(result.instructions[0].mnemonic, "li");
    assert_eq!(result.instructions[0].operands, "ra, 1");
}

#[test]
fn test_real_detail_output_uses_instruction_addresses() {
    let args = vec!["robustone", "-r", "riscv32", "93001000", "1000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let result = process_input(&config).expect("disassembly should succeed");
    let formatter =
        DisassemblyFormatter::new(OutputConfig::from_display_options(&config.display_options));
    let output = formatter.format(&result);

    assert!(output.contains("1000"));
    assert!(output.contains("Registers written"));
}

#[test]
fn test_architecture_helpers_still_work() {
    assert!(Architecture::parse("riscv32").is_ok());
    assert!(Architecture::parse("x86").is_ok());
    assert_eq!(Architecture::Riscv32.name(), "riscv32");
}
