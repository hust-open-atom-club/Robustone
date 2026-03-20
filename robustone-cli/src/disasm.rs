use crate::config::{DisasmConfig, OutputConfig};
use robustone_core::{ArchitectureDispatcher, DisasmError, Instruction};
use robustone_riscv::RiscVHandler;
use serde_json::json;

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
/// Result of a disassembly operation with additional metadata.
#[derive(Debug)]
pub struct DisassemblyResult {
    pub instructions: Vec<Instruction>,
    pub start_address: u64,
    pub architecture: String,
    pub bytes_processed: usize,
    pub errors: Vec<String>,
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
    pub fn add_error(&mut self, error: String) {
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
                        result.add_error(err.to_string());
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

        if result.instructions.is_empty() {
            return output;
        }

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

        // Print errors if any occurred
        for error in &result.errors {
            output.push_str(&format!("; Error: {error}\n"));
        }

        output
    }

    /// Format the disassembly result as structured JSON.
    pub fn format_json(&self, result: &DisassemblyResult) -> String {
        let instructions = result
            .instructions
            .iter()
            .map(|instruction| {
                json!({
                    "address": instruction.address,
                    "mnemonic": instruction.mnemonic,
                    "operands": instruction.operands,
                    "size": instruction.size,
                    "bytes": instruction.bytes,
                    "decoded": instruction.decoded,
                })
            })
            .collect::<Vec<_>>();

        serde_json::to_string_pretty(&json!({
            "architecture": result.architecture,
            "start_address": result.start_address,
            "bytes_processed": result.bytes_processed,
            "errors": result.errors,
            "instructions": instructions,
        }))
        .expect("JSON serialization should not fail")
    }

    /// Format a single instruction.
    fn format_instruction(&self, instr: &Instruction, hex_width: usize) -> String {
        let address_str = format!("{:x}", instr.address);

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
            if instr.operands.is_empty() {
                format!("{address_str}  {bytes_str}  {}", instr.mnemonic)
            } else {
                format!(
                    "{address_str}  {bytes_str}  {}\t{}",
                    instr.mnemonic, instr.operands
                )
            }
        } else if instr.operands.is_empty() {
            format!("{address_str}    {}", instr.mnemonic)
        } else {
            format!("{address_str}    {}\t{}", instr.mnemonic, instr.operands)
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
}

fn format_register_name(architecture_name: &str, reg_id: u32) -> String {
    match architecture_name {
        "riscv" => robustone_core::riscv::types::RiscVRegister::from_id(reg_id)
            .name()
            .to_string(),
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

        result.add_error("test error".to_string());
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
            show_hex: false,
            show_detail_sections: false,
            json: true,
        });
        let parsed: Value = serde_json::from_str(&formatter.format(&result)).unwrap();

        assert_eq!(parsed["architecture"], "riscv32");
        assert_eq!(parsed["instructions"][0]["mnemonic"], "li");
        assert_eq!(parsed["instructions"][0]["decoded"]["mnemonic"], "addi");
    }
}
