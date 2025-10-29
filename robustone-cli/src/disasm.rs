use crate::config::{DisasmConfig, OutputConfig};
use robustone_core::{ArchitectureDispatcher, error::DisasmError, instruction::Instruction};

// Shared dispatcher instance reused to avoid repeated initialisation costs.
lazy_static::lazy_static! {
    static ref DISPATCHER: ArchitectureDispatcher = ArchitectureDispatcher::new();
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
    /// Create a new disassembly engine.
    pub fn new() -> Self {
        Self {
            dispatcher: ArchitectureDispatcher::new(),
        }
    }

    /// Create a new engine instance (preferred method for thread safety).
    pub fn new_engine() -> Self {
        Self::new()
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
                        current_address += 1;
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
        let mut output = String::new();

        if result.instructions.is_empty() {
            return output;
        }

        let mut current_address = result.start_address;

        for instruction in &result.instructions {
            let formatted = self.format_instruction(instruction, current_address);
            output.push_str(&formatted);
            output.push('\n');
            current_address += instruction.size as u64;
        }

        // Print errors if any occurred
        for error in &result.errors {
            output.push_str(&format!("; Error: {error}\n"));
        }

        output
    }

    /// Format a single instruction.
    fn format_instruction(&self, instr: &Instruction, address: u64) -> String {
        let address_str = format!("{:x}", address,);

        let bytes_str = if self.output_config.show_hex {
            format!(
                "{:>width$}",
                instr
                    .bytes
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                width = self.output_config.hex_width
            )
        } else {
            String::new()
        };

        if instr.operands.is_empty() {
            format!("{}  {}  {}", address_str, bytes_str, instr.mnemonic)
        } else {
            format!(
                "{}  {}  {}\t{}",
                address_str, bytes_str, instr.mnemonic, instr.operands
            )
        }
    }

    /// Print the disassembly result directly to stdout.
    pub fn print(&self, result: &DisassemblyResult) {
        print!("{}", self.format(result));
    }
}

/// Convenience functions for backward compatibility.
/// Disassembles the supplied byte tokens using the provided configuration.
pub fn process_input(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    let engine = DisassemblyEngine::new();
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

    #[test]
    fn test_disassembly_engine() {
        let engine = DisassemblyEngine::new();
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
}
