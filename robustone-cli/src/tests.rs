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
    assert!(!cli.json);
}

#[test]
fn test_architecture_spec_rejects_incompatible_modifier() {
    assert!(ArchitectureSpec::parse("riscv32+intel").is_err());
    assert!(ArchitectureSpec::parse("arm+thumb").is_ok());
    assert!(ArchitectureSpec::parse("x86+intel").is_ok());
}

#[test]
fn test_architecture_spec_accepts_endianness_modifiers() {
    let arm_be = ArchitectureSpec::parse("arm+be").expect("arm+be should parse");
    assert_eq!(arm_be.arch, Architecture::ArmBE);
    assert_eq!(arm_be.mode, 0x100);

    let arm_le = ArchitectureSpec::parse("armbe+le").expect("armbe+le should parse");
    assert_eq!(arm_le.arch, Architecture::ArmLE);
    assert_eq!(arm_le.mode, 0x0);

    let mips_le = ArchitectureSpec::parse("mips+little").expect("mips+little should parse");
    assert_eq!(mips_le.arch, Architecture::MipsEL);
    assert_eq!(mips_le.mode, 0x0);

    let ppc_be = ArchitectureSpec::parse("ppc+be").expect("ppc+be should parse");
    assert_eq!(ppc_be.arch, Architecture::PowerPC32BE);
    assert_eq!(ppc_be.mode, 0x100);

    let arm_flip = ArchitectureSpec::parse("armle+be").expect("armle+be should parse");
    assert_eq!(arm_flip.arch, Architecture::ArmBE);
    assert_eq!(arm_flip.mode, 0x100);

    let sparc64_le = ArchitectureSpec::parse("sparc64+le").expect("sparc64+le should parse");
    assert_eq!(sparc64_le.arch, Architecture::Sparc64);
    assert_eq!(sparc64_le.mode, 0x0);

    let sparc64_be = ArchitectureSpec::parse("sparc64+be").expect("sparc64+be should parse");
    assert_eq!(sparc64_be.arch, Architecture::Sparc64);
    assert_eq!(sparc64_be.mode, 0x100);
}

#[test]
fn test_architecture_spec_accepts_cstool_style_modifier_sets() {
    assert!(ArchitectureSpec::parse("arm+noregname").is_ok());
    assert!(ArchitectureSpec::parse("mips+nodollar").is_ok());
    assert!(ArchitectureSpec::parse("ppc+percentage").is_ok());
    assert!(ArchitectureSpec::parse("x86+noregname").is_err());
}

#[test]
fn test_architecture_spec_accepts_riscv_capstone_modifiers() {
    assert!(ArchitectureSpec::parse("riscv64+a+fd").is_ok());
    assert!(ArchitectureSpec::parse("riscv32+noalias").is_ok());
    assert!(ArchitectureSpec::parse("riscv32+noaliascompressed").is_ok());
    assert!(ArchitectureSpec::parse("riscv32+bitmanip").is_err());
    assert!(ArchitectureSpec::parse("riscv32+intel").is_err());
}

#[test]
fn test_process_input_honors_riscv_extension_modifiers() {
    let args = vec!["robustone", "riscv64+a", "d3027300"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let error = process_input(&config).expect_err("missing F/D should fail");

    assert_eq!(error.stable_kind(), "unsupported_extension");

    let args = vec!["robustone", "riscv64+a+fd", "d3027300"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let result = process_input(&config).expect("explicit F/D should decode");

    assert_eq!(result.instructions[0].mnemonic, "fadd.s");
}

#[test]
fn test_noalias_modifier_disables_riscv_alias_rendering() {
    let args = vec!["robustone", "riscv32+noalias", "93001000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let result = process_input(&config).expect("disassembly should succeed");
    let formatter = DisassemblyFormatter::new(config.output_config());
    let output = formatter.format(&result);

    assert!(output.contains("addi\tx1, x0, 1"));
    assert!(!output.contains("li\tra"));
}

#[test]
fn test_config_preserves_input_byte_order() {
    let args = vec!["robustone", "riscv32", "93001000"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");

    assert_eq!(config.hex_bytes, vec![0x93, 0x00, 0x10, 0x00]);
}

#[test]
fn test_config_accepts_prefixed_multi_token_hex_input() {
    let args = vec!["robustone", "riscv32", "0x93 0x00 0x10 0x00"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    let result = process_input(&config).expect("disassembly should succeed");

    assert_eq!(config.hex_bytes, vec![0x93, 0x00, 0x10, 0x00]);
    assert_eq!(result.instructions[0].mnemonic, "li");
    assert_eq!(result.instructions[0].operands, "ra, 1");
}

#[test]
fn test_config_rejects_odd_length_hex_instead_of_truncating() {
    let args = vec!["robustone", "riscv32", "9300100"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config_result = DisasmConfig::config_from_cli(&cli);

    assert!(config_result.is_err());
}

#[test]
fn test_config_accepts_output_flags() {
    let alias_args = vec!["robustone", "-a", "riscv32", "93001000"];
    let alias_cli = Cli::try_parse_from(alias_args).expect("CLI arguments should parse");
    let alias_result = DisasmConfig::config_from_cli(&alias_cli);
    assert!(alias_result.is_ok());

    let unsigned_args = vec!["robustone", "-u", "riscv32", "130101ff"];
    let unsigned_cli = Cli::try_parse_from(unsigned_args).expect("CLI arguments should parse");
    let unsigned_config =
        DisasmConfig::config_from_cli(&unsigned_cli).expect("configuration should be valid");
    let result = process_input(&unsigned_config).expect("disassembly should succeed");
    let formatter = DisassemblyFormatter::new(OutputConfig::from_display_options(
        &unsigned_config.display_options,
    ));
    let output = formatter.format(&result);

    assert!(output.contains("0xfffffff0"));
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
fn test_config_accepts_odd_length_hex_addresses() {
    let args = vec!["robustone", "riscv32", "93001000", "0x1"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    assert_eq!(config.start_address, 0x1);

    let args = vec!["robustone", "riscv32", "93001000", "0x100"];
    let cli = Cli::try_parse_from(args).expect("CLI arguments should parse");
    let config = DisasmConfig::config_from_cli(&cli).expect("configuration should be valid");
    assert_eq!(config.start_address, 0x100);
}

#[test]
fn test_architecture_helpers_still_work() {
    assert!(Architecture::parse("riscv32").is_ok());
    assert!(Architecture::parse("x86").is_ok());
    assert_eq!(Architecture::Riscv32.name(), "riscv32");
}
