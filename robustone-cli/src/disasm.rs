use crate::config::{DisasmConfig, OutputConfig};
use robustone_core::ir::TextRenderProfile;
use robustone_core::{
    ArchitectureDispatcher, DisasmError, Instruction, render_disassembly, render_instruction_text,
};
use robustone_core::{RenderOptions, RenderedIssue};
use robustone_riscv::{RiscVHandler, types::RiscVRegister};
use serde::Serialize;

fn create_dispatcher(arch: &str) -> ArchitectureDispatcher {
    let mut dispatcher = ArchitectureDispatcher::new();
    // Create handler based on architecture
    let handler = match arch {
        "riscv32" => RiscVHandler::rv32(),
        "riscv64" | "riscv" => RiscVHandler::rv64(),
        _ => RiscVHandler::new(),
    };
    dispatcher.register(Box::new(handler));
    dispatcher
}

/// Structured error information captured during disassembly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DisassemblyIssue {
    pub kind: String,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_offset: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub raw_bytes: Vec<u8>,
}

impl DisassemblyIssue {
    /// Create a minimal issue for tests or formatter-only failures.
    pub fn generic(message: impl Into<String>) -> Self {
        Self {
            kind: "error".to_string(),
            operation: "unknown".to_string(),
            message: message.into(),
            architecture: None,
            address: None,
            input_offset: None,
            raw_bytes: Vec::new(),
        }
    }

    /// Capture a structured issue from a core disassembly error.
    pub fn from_core_error(
        error: &DisasmError,
        operation: impl Into<String>,
        architecture: impl Into<String>,
        address: u64,
        input_offset: usize,
        raw_bytes: &[u8],
    ) -> Self {
        Self {
            kind: error.stable_kind().to_string(),
            operation: operation.into(),
            message: error.detail_message(),
            architecture: Some(
                error
                    .architecture_name()
                    .map(str::to_string)
                    .unwrap_or_else(|| architecture.into()),
            ),
            address: Some(address),
            input_offset: Some(input_offset),
            raw_bytes: raw_bytes.iter().take(8).copied().collect(),
        }
    }

    /// Render the issue into the human-readable CLI form.
    pub fn display_message(&self) -> String {
        let mut parts = vec![format!("[{}] {}", self.kind, self.message)];
        if let Some(architecture) = &self.architecture {
            parts.push(format!("arch={architecture}"));
        }
        if let Some(address) = self.address {
            parts.push(format!("addr=0x{address:x}"));
        }
        if let Some(offset) = self.input_offset {
            parts.push(format!("offset={offset}"));
        }
        if !self.raw_bytes.is_empty() {
            parts.push(format!("bytes={}", hex::encode(&self.raw_bytes)));
        }
        parts.join(" | ")
    }

    fn to_rendered_issue(&self) -> RenderedIssue {
        RenderedIssue {
            kind: self.kind.clone(),
            operation: self.operation.clone(),
            message: self.message.clone(),
            architecture: self.architecture.clone(),
            address: self.address,
            input_offset: self.input_offset,
            raw_bytes: self.raw_bytes.clone(),
        }
    }
}
/// Result of a disassembly operation with additional metadata.
#[derive(Debug)]
pub struct DisassemblyResult {
    pub instructions: Vec<Instruction>,
    pub start_address: u64,
    pub architecture: String,
    pub bytes_processed: usize,
    pub errors: Vec<DisassemblyIssue>,
}

impl DisassemblyResult {
    /// Create a new empty disassembly result.
    pub fn new(start_address: u64, architecture: String) -> Self {
        Self {
            instructions: Vec::new(),
            start_address,
            architecture,
            bytes_processed: 0,
            errors: Vec::new(),
        }
    }

    /// Add an instruction to the result.
    pub fn add_instruction(&mut self, instr: Instruction) {
        self.bytes_processed += instr.size;
        self.instructions.push(instr);
    }

    /// Add an error to the result.
    pub fn add_error(&mut self, error: DisassemblyIssue) {
        self.errors.push(error);
    }

    /// Get the number of successfully disassembled instructions.
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    /// Get the number of errors encountered.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Check if the disassembly was completely successful.
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the final address after processing all instructions.
    pub fn final_address(&self) -> u64 {
        self.start_address + self.bytes_processed as u64
    }
}

/// Iterator for DisassemblyResult that consumes the result.
impl IntoIterator for DisassemblyResult {
    type Item = Instruction;
    type IntoIter = std::vec::IntoIter<Instruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}

/// Iterator for DisassemblyResult that yields references.
impl<'a> IntoIterator for &'a DisassemblyResult {
    type Item = &'a Instruction;
    type IntoIter = std::slice::Iter<'a, Instruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.iter()
    }
}

/// High-level disassembly engine that processes byte sequences.
#[derive(Default)]
pub struct DisassemblyEngine {
    dispatcher: ArchitectureDispatcher,
}

impl DisassemblyEngine {
    /// Create a new disassembly engine for the given architecture.
    pub fn new(arch: &str) -> Self {
        Self {
            dispatcher: create_dispatcher(arch),
        }
    }

    /// Create a new engine instance for riscv64 (default).
    pub fn new_engine() -> Self {
        Self::new("riscv64")
    }

    /// Disassemble bytes using the provided configuration.
    pub fn disassemble(&self, config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
        config
            .validate_for_disassembly()
            .map_err(|e| DisasmError::DecodingError(e.to_string()))?;

        let mut result =
            DisassemblyResult::new(config.start_address, config.arch_name().to_string());
        let mut offset = 0;
        let mut current_address = config.start_address;
        let arch_name = config.arch_name();

        while offset < config.hex_bytes.len() {
            let slice = &config.hex_bytes[offset..];

            match self
                .dispatcher
                .disassemble_bytes(slice, arch_name, current_address)
            {
                Ok((instruction, size)) => {
                    if size == 0 {
                        return Err(DisasmError::DecodingError(
                            "Decoder returned zero-length instruction".to_string(),
                        ));
                    }

                    result.add_instruction(instruction);
                    offset += size;
                    current_address = current_address.saturating_add(size as u64);
                }
                Err(err) => {
                    if config.skip_data {
                        // Skip the problematic byte and continue
                        result.add_error(DisassemblyIssue::from_core_error(
                            &err,
                            "decode_instruction",
                            arch_name,
                            current_address,
                            offset,
                            slice,
                        ));
                        offset += 1;
                        current_address = current_address.saturating_add(1);
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Disassemble a single instruction at the given address.
    pub fn disassemble_single(
        &self,
        bytes: &[u8],
        arch_name: &str,
        address: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        self.dispatcher.disassemble_bytes(bytes, arch_name, address)
    }
}

/// Formatter for disassembly output with multiple display modes.
pub struct DisassemblyFormatter {
    output_config: OutputConfig,
}

impl DisassemblyFormatter {
    /// Create a new formatter with the given output configuration.
    pub fn new(output_config: OutputConfig) -> Self {
        Self { output_config }
    }

    /// Format the disassembly result for display.
    pub fn format(&self, result: &DisassemblyResult) -> String {
        if self.output_config.json {
            return self.format_json(result);
        }

        let mut output = String::new();
        if !result.instructions.is_empty() {
            let hex_width = result
                .instructions
                .iter()
                .map(|instruction| instruction.bytes.len().saturating_mul(3).saturating_sub(1))
                .max()
                .unwrap_or(0);

            for instruction in &result.instructions {
                let formatted = self.format_instruction(instruction, hex_width);
                output.push_str(&formatted);
                output.push('\n');
            }
        }

        // Print errors if any occurred
        for error in &result.errors {
            output.push_str(&format!("; Error: {}\n", error.display_message()));
        }

        output
    }

    /// Format the disassembly result as structured JSON.
    pub fn format_json(&self, result: &DisassemblyResult) -> String {
        let errors = result
            .errors
            .iter()
            .map(DisassemblyIssue::to_rendered_issue)
            .collect::<Vec<_>>();

        serde_json::to_string_pretty(&render_disassembly(
            result.architecture.clone(),
            result.start_address,
            result.bytes_processed,
            errors,
            &result.instructions,
            self.render_options(),
        ))
        .expect("JSON serialization should not fail")
    }

    /// Format a single instruction.
    fn format_instruction(&self, instr: &Instruction, hex_width: usize) -> String {
        let address_str = format!("{:x}", instr.address);
        let (mnemonic, operands) = self.render_instruction_text(instr);

        let bytes_str = if self.output_config.show_hex {
            format!(
                "{:>width$}",
                instr
                    .bytes
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                width = hex_width
            )
        } else {
            String::new()
        };

        let mut line = if self.output_config.show_hex {
            if operands.is_empty() {
                format!("{address_str}  {bytes_str}  {mnemonic}")
            } else {
                format!("{address_str}  {bytes_str}  {mnemonic}\t{operands}")
            }
        } else if operands.is_empty() {
            format!("{address_str}    {mnemonic}")
        } else {
            format!("{address_str}    {mnemonic}\t{operands}")
        };

        if self.output_config.show_detail_sections {
            let detail_lines = self.format_detail_sections(instr);
            if !detail_lines.is_empty() {
                line.push('\n');
                line.push_str(&detail_lines.join("\n"));
            }
        }

        line
    }

    fn format_detail_sections(&self, instr: &Instruction) -> Vec<String> {
        let Some(detail) = &instr.detail else {
            return Vec::new();
        };

        let mut detail_lines = Vec::new();
        if matches!(
            self.output_config.text_profile,
            TextRenderProfile::VerboseDebug
        ) && let Some(decoded) = &instr.decoded
        {
            if let Some(opcode_id) = &decoded.opcode_id {
                detail_lines.push(format!("\tOpcode ID: {opcode_id}"));
            }
            if !decoded.groups.is_empty() {
                detail_lines.push(format!("\tGroups: {}", decoded.groups.join(", ")));
            }
            detail_lines.push(format!("\tStatus: {:?}", decoded.status));
        }
        let registers_read = detail.registers_read();
        if !registers_read.is_empty() {
            let registers = registers_read
                .iter()
                .map(|reg_id| format_register_name(detail.architecture_name(), *reg_id))
                .collect::<Vec<_>>()
                .join(", ");
            detail_lines.push(format!("\tRegisters read: {registers}"));
        }

        let registers_written = detail.registers_written();
        if !registers_written.is_empty() {
            let registers = registers_written
                .iter()
                .map(|reg_id| format_register_name(detail.architecture_name(), *reg_id))
                .collect::<Vec<_>>()
                .join(", ");
            detail_lines.push(format!("\tRegisters written: {registers}"));
        }

        detail_lines
    }

    /// Print the disassembly result directly to stdout.
    pub fn print(&self, result: &DisassemblyResult) {
        print!("{}", self.format(result));
    }

    fn render_instruction_text(&self, instr: &Instruction) -> (String, String) {
        render_instruction_text(instr, self.render_options())
    }

    fn render_options(&self) -> RenderOptions {
        RenderOptions {
            text_profile: self.output_config.text_profile,
            alias_regs: self.output_config.alias_regs,
            unsigned_immediate: self.output_config.unsigned_immediate,
        }
    }
}

fn format_register_name(architecture_name: &str, reg_id: u32) -> String {
    match architecture_name {
        "riscv" => RiscVRegister::from_id(reg_id).name().to_string(),
        _ => reg_id.to_string(),
    }
}

/// Convenience functions for backward compatibility.
/// Disassembles the supplied byte tokens using the provided configuration.
pub fn process_input(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    let arch = config.arch_name();
    let engine = DisassemblyEngine::new(arch);
    engine.disassemble(config)
}

/// Backwards-compatible wrapper that keeps the legacy function name.
#[allow(dead_code)]
pub fn disassemble(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    process_input(config)
}

/// Prints the disassembly result in the cstool-compatible layout.
pub fn print_instructions(result: &DisassemblyResult, config: &DisasmConfig) {
    let output_config = OutputConfig::from_display_options(&config.display_options);
    let formatter = DisassemblyFormatter::new(output_config);
    formatter.print(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::ArchitectureSpec;
    use crate::command::DisplayOptions;
    use robustone_core::ir::{ArchitectureId, DecodeStatus, Operand, RegisterId, RenderHints};
    use serde_json::Value;

    #[test]
    fn test_disassembly_engine() {
        let engine = DisassemblyEngine::new("riscv64");
        // The exact number of architectures may vary, so just check it's a reasonable number
        assert!(!engine.dispatcher.supported_architectures().is_empty()); // Basic sanity check
    }

    #[test]
    fn test_disassembly_result() {
        let mut result = DisassemblyResult::new(0x1000, "riscv32".to_string());
        assert_eq!(result.instruction_count(), 0);
        assert!(result.is_successful());

        result.add_error(DisassemblyIssue::generic("test error"));
        assert_eq!(result.error_count(), 1);
        assert!(!result.is_successful());
    }

    #[test]
    fn test_json_formatter_includes_decoded_ir() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x93, 0x00, 0x10, 0x00],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: false,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig {
            text_profile: robustone_core::ir::TextRenderProfile::Capstone,
            alias_regs: false,
            unsigned_immediate: false,
            show_hex: false,
            show_detail_sections: false,
            json: true,
        });
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["architecture"], "riscv32");
        assert_eq!(parsed["instructions"][0]["mnemonic"], "li");
        assert_eq!(parsed["instructions"][0]["decoded"]["mnemonic"], "addi");
    }

    #[test]
    fn test_formatter_prefers_decoded_ir_over_legacy_instruction_text() {
        let decoded = robustone_core::DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: "riscv32".to_string(),
            mnemonic: "addi".to_string(),
            opcode_id: Some("addi".to_string()),
            size: 4,
            raw_bytes: vec![0x93, 0x00, 0x10, 0x00],
            operands: vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Register {
                    register: RegisterId::riscv(0),
                },
                Operand::Immediate { value: 1 },
            ],
            registers_read: vec![RegisterId::riscv(0)],
            registers_written: vec![RegisterId::riscv(1)],
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: vec!["arithmetic".to_string()],
            status: DecodeStatus::Success,
            render_hints: RenderHints {
                capstone_mnemonic: Some("li".to_string()),
                capstone_hidden_operands: vec![1],
            },
        };
        let instruction =
            Instruction::from_decoded(decoded, "legacy".to_string(), "legacy".to_string(), None);
        let result = DisassemblyResult {
            instructions: vec![instruction],
            start_address: 0,
            architecture: "riscv32".to_string(),
            bytes_processed: 4,
            errors: Vec::new(),
        };
        let formatter = DisassemblyFormatter::new(OutputConfig::minimal());
        let output = formatter.format(&result);

        assert!(output.contains("li\t"));
        assert!(output.contains("ra, 1"));
        assert!(!output.contains("legacy"));
    }

    #[test]
    fn test_json_formatter_emits_structured_errors() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0xff, 0xff],
            start_address: 0x40,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: true,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig {
            text_profile: robustone_core::ir::TextRenderProfile::Capstone,
            alias_regs: false,
            unsigned_immediate: false,
            show_hex: false,
            show_detail_sections: false,
            json: true,
        });
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["errors"][0]["kind"], "need_more_bytes");
        assert_eq!(parsed["errors"][0]["architecture"], "riscv32");
        assert_eq!(parsed["errors"][0]["address"], 0x40);
        assert_eq!(parsed["errors"][0]["input_offset"], 0);
        assert_eq!(parsed["errors"][0]["raw_bytes"][0], 0xff);
    }

    #[test]
    fn test_json_formatter_emits_unimplemented_instruction_errors() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv64").unwrap(),
            hex_bytes: vec![0x00, 0x60],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: true,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig {
            text_profile: robustone_core::ir::TextRenderProfile::Capstone,
            alias_regs: false,
            unsigned_immediate: false,
            show_hex: false,
            show_detail_sections: false,
            json: true,
        });
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["errors"][0]["kind"], "unimplemented_instruction");
        assert_eq!(parsed["errors"][0]["architecture"], "riscv64");
    }

    #[test]
    fn test_text_formatter_emits_unsupported_mode_errors() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x83, 0x30, 0x00, 0x00],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: false,
            },
            skip_data: true,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig::minimal());
        let output = formatter.format(&result);

        assert!(output.contains("[unsupported_mode] ld requires RV64"));
        assert!(output.contains("arch=riscv32"));
    }

    #[test]
    fn test_json_formatter_emits_unsupported_mode_errors() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x83, 0x30, 0x00, 0x00],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: true,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig {
            text_profile: robustone_core::ir::TextRenderProfile::Capstone,
            alias_regs: false,
            unsigned_immediate: false,
            show_hex: false,
            show_detail_sections: false,
            json: true,
        });
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["errors"][0]["kind"], "unsupported_mode");
        assert_eq!(parsed["errors"][0]["architecture"], "riscv32");
        assert_eq!(parsed["errors"][0]["message"], "ld requires RV64");
    }

    #[test]
    fn test_canonical_json_profile_uses_canonical_text() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x93, 0x00, 0x10, 0x00],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: true,
            },
            skip_data: false,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter = DisassemblyFormatter::new(OutputConfig::canonical_json());
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["instructions"][0]["mnemonic"], "addi");
        assert_eq!(parsed["instructions"][0]["operands"], "x1, x0, 1");
    }

    #[test]
    fn test_json_formatter_respects_unsigned_immediate_option() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x13, 0x01, 0x01, 0xff],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: true,
                json: true,
            },
            skip_data: false,
        };
        let result = engine.disassemble(&config).unwrap();
        let formatter =
            DisassemblyFormatter::new(OutputConfig::from_display_options(&config.display_options));
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["instructions"][0]["operands"], "sp, sp, 0xfffffff0");
    }

    #[test]
    fn test_text_formatter_emits_structured_error_kinds() {
        let result = DisassemblyResult {
            instructions: Vec::new(),
            start_address: 0,
            architecture: "riscv32".to_string(),
            bytes_processed: 0,
            errors: vec![
                DisassemblyIssue::from_core_error(
                    &DisasmError::decode_failure(
                        robustone_core::types::error::DecodeErrorKind::InvalidEncoding,
                        Some("riscv32".to_string()),
                        "invalid test encoding",
                    ),
                    "decode_instruction",
                    "riscv32",
                    0,
                    0,
                    &[0xff, 0xff, 0xff, 0xff],
                ),
                DisassemblyIssue::from_core_error(
                    &DisasmError::decode_failure(
                        robustone_core::types::error::DecodeErrorKind::UnsupportedExtension,
                        Some("riscv32".to_string()),
                        "instruction requires F extension",
                    ),
                    "decode_instruction",
                    "riscv32",
                    4,
                    4,
                    &[0xd3, 0x02, 0x73, 0x00],
                ),
                DisassemblyIssue::from_core_error(
                    &DisasmError::decode_failure(
                        robustone_core::types::error::DecodeErrorKind::UnimplementedInstruction,
                        Some("riscv64".to_string()),
                        "c.fldsp is a legal compressed instruction but is not implemented",
                    ),
                    "decode_instruction",
                    "riscv64",
                    8,
                    8,
                    &[0x00, 0x60],
                ),
            ],
        };

        let formatter = DisassemblyFormatter::new(OutputConfig::minimal());
        let output = formatter.format(&result);

        assert!(output.contains("[invalid_encoding] invalid test encoding"));
        assert!(output.contains("[unsupported_extension] instruction requires F extension"));
        assert!(output.contains(
            "[unimplemented_instruction] c.fldsp is a legal compressed instruction but is not implemented"
        ));
    }

    #[test]
    fn test_canonical_text_and_json_share_register_rendering() {
        let engine = DisassemblyEngine::new();
        let config = DisasmConfig {
            arch_spec: ArchitectureSpec::parse("riscv32").unwrap(),
            hex_bytes: vec![0x93, 0x00, 0x10, 0x00],
            start_address: 0,
            display_options: DisplayOptions {
                detailed: false,
                alias_regs: false,
                real_detail: false,
                unsigned_immediate: false,
                json: false,
            },
            skip_data: false,
        };
        let result = engine.disassemble(&config).unwrap();

        let text_formatter = DisassemblyFormatter::new(OutputConfig {
            text_profile: robustone_core::ir::TextRenderProfile::Canonical,
            alias_regs: false,
            unsigned_immediate: false,
            show_hex: false,
            show_detail_sections: false,
            json: false,
        });
        let json_formatter = DisassemblyFormatter::new(OutputConfig::canonical_json());

        let text_output = text_formatter.format(&result);
        let parsed: Value = serde_json::from_str(&json_formatter.format(&result)).unwrap();

        assert!(text_output.contains("addi\tx1, x0, 1"));
        assert_eq!(parsed["instructions"][0]["mnemonic"], "addi");
        assert_eq!(parsed["instructions"][0]["operands"], "x1, x0, 1");
    }
}
