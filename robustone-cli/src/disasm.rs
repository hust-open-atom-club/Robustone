use crate::DisasmConfig;
use crate::arch::Architecture;
use robustone_core::{ArchitectureDispatcher, DisasmError, Instruction};

// Shared dispatcher instance reused to avoid repeated initialisation costs.
lazy_static::lazy_static! {
    static ref DISPATCHER: ArchitectureDispatcher = ArchitectureDispatcher::new();
}

#[derive(Debug)]
pub struct DisassemblyResult {
    pub instructions: Vec<Instruction>,
    next_index: usize,
}

impl DisassemblyResult {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            next_index: 0,
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}

impl Iterator for DisassemblyResult {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.instructions.len() {
            None
        } else {
            let item = self.instructions[self.next_index].clone();
            self.next_index += 1;
            Some(item)
        }
    }
}

/// Disassembles the supplied byte tokens using the provided configuration.
pub fn process_input(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    let bytes = hex_words_to_arch_bytes(&config.hex_words, &config.arch_spec.arch)
        .map_err(DisasmError::InvalidHexCode)?;

    let mut result = DisassemblyResult::new();
    let mut offset: usize = 0;
    let mut address = config.start_address;
    let arch_name = config.arch_spec.arch.name().to_string();

    while offset < bytes.len() {
        let slice = &bytes[offset..];
        let (instruction, size) = DISPATCHER.disassemble_bytes(slice, &arch_name, address)?;
        if size == 0 {
            return Err(DisasmError::DecodingError(
                "decoder returned zero-length instruction".to_string(),
            ));
        }

        result.add_instruction(instruction);
        offset += size;
        address = address.saturating_add(size as u64);
    }

    Ok(result)
}

/// Backwards-compatible wrapper that keeps the legacy function name.
#[allow(dead_code)]
pub fn disassemble(config: &DisasmConfig) -> Result<DisassemblyResult, DisasmError> {
    process_input(config)
}

/// Prints the disassembly result in the cstool-compatible layout.
pub fn print_instructions(result: &DisassemblyResult, _config: &DisasmConfig) {
    let mut offset: usize = 0;
    for instr in &result.instructions {
        let bytes_str = instr
            .bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");

        if instr.operands.is_empty() {
            println!("{:>2}  {}  {}", offset, bytes_str, instr.mnemonic);
        } else {
            println!(
                "{:>2}  {}  {}\t{}",
                offset, bytes_str, instr.mnemonic, instr.operands
            );
        }

        offset += instr.size as usize;
    }
}

fn hex_words_to_arch_bytes(words: &[String], _arch: &Architecture) -> Result<Vec<u8>, String> {
    let mut bytes: Vec<u8> = Vec::new();
    for word in words {
        let token = word.trim().to_lowercase();
        if token.is_empty() {
            continue;
        }

        let no_prefix = token.strip_prefix("0x").unwrap_or(&token);
        if no_prefix.is_empty() {
            return Err("empty hex token".into());
        }
        if no_prefix.len() % 2 != 0 {
            return Err(format!("odd-length hex token: {}", word));
        }

        let mut token_bytes: Vec<u8> = Vec::new();
        for i in (0..no_prefix.len()).step_by(2) {
            let byte = u8::from_str_radix(&no_prefix[i..i + 2], 16)
                .map_err(|_| format!("invalid hex byte in token: {}", word))?;
            token_bytes.push(byte);
        }

        // For RISC-V we mirror cstool by treating the hex string as a direct byte sequence in
        // little-endian order (no word swapping). The generic conversion above already behaves
        // that way, so the note is only documentation for future readers.

        bytes.extend(token_bytes);
    }

    Ok(bytes)
}
