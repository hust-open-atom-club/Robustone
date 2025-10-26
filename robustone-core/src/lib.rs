//! Transfer crate – core of the disassembly engine.
//!
//! Rust-based engine inspired by Capstone’s design, with multi-architecture support.

pub mod error;
pub mod instruction;

/// Robustone prelude.
///
/// Re-export frequently used types.
pub mod prelude {
    pub use crate::error::DisasmError;
    pub use crate::instruction::{Instruction, InstructionDetail};
}

use crate::error::DisasmError;
use crate::instruction::Instruction;
use core::str;

/// Trait implemented by architecture-specific disassemblers.
pub trait ArchitectureHandler: Sync {
    /// Disassembles a single instruction and returns the decoded instruction plus the byte count consumed.
    fn disassemble(&self, bytes: &[u8], addr: u64) -> Result<(Instruction, usize), DisasmError>;

    /// Returns the canonical architecture name.
    fn name(&self) -> &'static str;

    /// Checks whether the handler supports the requested architecture name without leaking dependencies upward.
    fn supports(&self, arch_name: &str) -> bool;
}

/// Dispatcher that selects the correct architecture handler at runtime.
pub struct ArchitectureDispatcher {
    handlers: Vec<Box<dyn ArchitectureHandler>>,
}

impl ArchitectureDispatcher {
    /// Creates a dispatcher instance and registers all available handlers.
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn ArchitectureHandler>> = Vec::new();

        // Register the RISC-V handler when the feature is enabled.
        #[cfg(feature = "riscv")]
        {
            use crate::riscv::RiscVHandler;
            handlers.push(Box::new(RiscVHandler::new()));
        }

        Self { handlers }
    }

    /// Legacy helper that accepts a hex string and returns the decoded instruction.
    pub fn disassemble(&self, hex: &str, arch: String) -> Instruction {
        let s = hex.trim().to_lowercase();
        let no_prefix = if s.starts_with("0x") { &s[2..] } else { &s };

        let mut bytes: Vec<u8> = Vec::new();
        let mut i = 0;
        while i + 1 < no_prefix.len() {
            let b = u8::from_str_radix(&no_prefix[i..i + 2], 16).unwrap_or(0);
            bytes.push(b);
            i += 2;
        }

        if arch.starts_with("riscv") {
            bytes.reverse();
        }

        if let Ok((instruction, _)) = self.disassemble_bytes(&bytes, &arch, 0) {
            instruction
        } else {
            let size = bytes.len();
            Instruction {
                address: 0,
                bytes,
                mnemonic: "unknown".to_string(),
                operands: "".to_string(),
                size,
                detail: None,
            }
        }
    }

    pub fn disassemble_bytes(
        &self,
        bytes: &[u8],
        arch: &str,
        address: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        for handler in &self.handlers {
            if handler.supports(arch) {
                return handler.disassemble(bytes, address);
            }
        }

        Err(DisasmError::UnsupportedArchitecture(arch.to_string()))
    }

    /// Return all registered architectures.
    pub fn supported_architectures(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|h| h.name()).collect()
    }
}

impl Default for ArchitectureDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// Default RISC-V module inclusion when the feature flag is enabled.
#[cfg(feature = "riscv")]
pub mod riscv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_dispatcher_creation() {
        let dispatcher = ArchitectureDispatcher::new();
        let archs = dispatcher.supported_architectures();

        // Ensure RISC-V is present whenever the feature flag is enabled.
        #[cfg(feature = "riscv")]
        assert!(archs.contains(&"riscv"));
    }

    #[test]
    fn test_hex_parsing() {
        let dispatcher = ArchitectureDispatcher::new();

        // Hex parsing should succeed for bare strings.
        let instruction = dispatcher.disassemble("deadbeef", "unknown".to_string());
        assert_eq!(instruction.mnemonic, "unknown");
        assert_eq!(instruction.bytes, vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(instruction.size, 4);

        // Hex parsing should also accept a `0x` prefix.
        let instruction = dispatcher.disassemble("0x1234", "unknown".to_string());
        assert_eq!(instruction.bytes, vec![0x12, 0x34]);
        assert_eq!(instruction.size, 2);
    }
}
