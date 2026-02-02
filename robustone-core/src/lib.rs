//! Robustone – Core disassembly engine with multi-architecture support.
//!
//! This crate provides a flexible, extensible disassembly framework inspired by
//! Capstone's design but implemented in pure Rust. The architecture is designed
//! to support multiple instruction set architectures through a unified interface.
//!
//! # Architecture Overview
//!
//! The disassembly engine is built around several key abstractions:
//!
//! - **ArchitectureHandler**: Trait that all architecture-specific disassemblers must implement
//! - **ArchitectureDispatcher**: Runtime dispatcher that selects the appropriate handler
//! - **Utils**: Common utilities for hex parsing, endianness handling, etc.
//!
//! # Adding New Architectures
//!
//! To add support for a new architecture:
//!
//! 1. Create a new module in `src/` (e.g., `src/arm/`)
//! 2. Implement the `ArchitectureHandler` trait for your architecture
//! 3. Register the handler in `ArchitectureDispatcher::new()`
//!
//! # Example
//!
//! ```rust
//! use robustone_core::prelude::*;
//! use robustone_core::ArchitectureDispatcher;
//!
//! let dispatcher = ArchitectureDispatcher::new();
//! match dispatcher.disassemble_bytes(&[0x93, 0x01, 0x00, 0x00], "riscv32", 0x1000) {
//!     Ok((instruction, size)) => {
//!         println!("Instruction: {} {}", instruction.mnemonic, instruction.operands);
//!     }
//!     Err(DisasmError::UnsupportedArchitecture(arch)) => {
//!         eprintln!("Architecture '{}' not supported", arch);
//!     }
//!     Err(e) => {
//!         eprintln!("Disassembly error: {:?}", e);
//!     }
//! }
//! ```

pub mod architecture;
pub mod riscv;
pub mod traits;
pub mod types;
pub mod utils;

/// Robustone prelude.
///
/// Re-exports frequently used types and traits for convenient importing.
/// This module provides access to the most common functionality needed for
/// using the disassembly engine.
pub mod prelude {
    pub use crate::architecture::{Architecture, is_address_aligned};
    pub use crate::traits::{ArchitectureHandler, BasicInstructionDetail, Detail};
    pub use crate::types::{DisasmError, Instruction};
    pub use crate::utils::{Endianness, HexParser};
}

pub use traits::ArchitectureHandler;
pub use traits::instruction::Detail;
pub use types::error::DisasmError;
pub use types::instruction::Instruction;

use crate::utils::HexParser;

/// Runtime dispatcher that selects the appropriate architecture handler.
///
/// The dispatcher maintains a registry of architecture handlers and provides
/// a unified interface for disassembling instructions across different
/// architectures. It handles the complexity of selecting the correct handler
/// based on the architecture name.
///
/// # Thread Safety
///
/// The dispatcher is thread-safe and can be shared across multiple threads
/// since all handlers are required to implement `Sync`.
pub struct ArchitectureDispatcher {
    handlers: Vec<Box<dyn ArchitectureHandler>>,
    hex_parser: HexParser,
}

impl ArchitectureDispatcher {
    /// Creates a new dispatcher and registers all available architecture handlers.
    ///
    /// This method automatically registers handlers for all supported architectures.
    /// Currently supported architectures:
    ///
    /// - RISC-V (32-bit and 64-bit)
    ///
    /// # Returns
    ///
    /// A new `ArchitectureDispatcher` instance with all available handlers registered.
    pub fn new() -> Self {
        let mut dispatcher = Self {
            handlers: Vec::new(),
            hex_parser: HexParser::new(),
        };

        dispatcher.register(Box::new(riscv::RiscVHandler::new()));

        dispatcher
    }

    /// Registers an architecture handler with the dispatcher.
    ///
    /// This method allows adding custom architecture handlers at runtime.
    /// The handler will be added to the end of the handler list.
    ///
    /// # Arguments
    ///
    /// * `handler` - A boxed architecture handler to register
    pub fn register(&mut self, handler: Box<dyn ArchitectureHandler>) {
        self.handlers.push(handler);
    }

    /// Legacy convenience method for disassembling a hex string.
    ///
    /// This method provides backwards compatibility with the original API.
    /// It parses a hexadecimal string and attempts to disassemble it using
    /// the specified architecture.
    ///
    /// # Arguments
    ///
    /// * `hex` - Hexadecimal string representation of instruction bytes
    /// * `arch` - Target architecture name (e.g., "riscv32", "arm", "x86")
    ///
    /// # Returns
    ///
    /// Returns the decoded `Instruction`. If disassembly fails, returns
    /// an "unknown" instruction with the original bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::ArchitectureDispatcher;
    /// let dispatcher = ArchitectureDispatcher::new();
    /// let instruction = dispatcher.disassemble("13000513", "riscv32".to_string());
    /// println!("Instruction: {} {}", instruction.mnemonic, instruction.operands);
    /// ```
    pub fn disassemble(&self, hex: &str, arch: String) -> Instruction {
        // Use the improved hex parser with architecture-specific handling
        let bytes = match self.hex_parser.parse_for_architecture(hex, &arch) {
            Ok(bytes) => bytes,
            Err(_) => {
                // If parsing fails, create a minimal unknown instruction
                return Instruction {
                    address: 0,
                    bytes: vec![],
                    mnemonic: "unknown".to_string(),
                    operands: format!("(parse error: {hex})"),
                    size: 0,
                    detail: None,
                };
            }
        };

        // Attempt to disassemble the parsed bytes
        match self.disassemble_bytes(&bytes, &arch, 0) {
            Ok((instruction, _)) => instruction,
            Err(_) => {
                // Create an unknown instruction with the parsed bytes
                let size = bytes.len();
                Instruction {
                    address: 0,
                    bytes,
                    mnemonic: "unknown".to_string(),
                    operands: format!("0x{}", hex.trim_start_matches("0x")),
                    size,
                    detail: None,
                }
            }
        }
    }

    /// Disassembles raw instruction bytes using the specified architecture.
    ///
    /// This is the primary method for disassembling raw binary data. It
    /// automatically selects the appropriate handler based on the architecture
    /// name and delegates the disassembly work to that handler.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw instruction bytes to decode
    /// * `arch` - Target architecture name
    /// * `address` - Memory address where these bytes would be located
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - The decoded `Instruction`
    /// - The number of bytes consumed from the input
    ///
    /// # Errors
    ///
    /// Returns `DisasmError::UnsupportedArchitecture` if no handler supports
    /// the specified architecture. Propagates any architecture-specific errors
    /// that occur during disassembly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::ArchitectureDispatcher;
    /// let dispatcher = ArchitectureDispatcher::new();
    /// let bytes = [0x13, 0x05, 0x00, 0x00]; // addi a0, zero, 0
    /// match dispatcher.disassemble_bytes(&bytes, "riscv32", 0x1000) {
    ///     Ok((instruction, size)) => {
    ///         println!("Instruction: {} {}", instruction.mnemonic, instruction.operands);
    ///         println!("Size: {} bytes", size);
    ///     }
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    pub fn disassemble_bytes(
        &self,
        bytes: &[u8],
        arch: &str,
        address: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        // Find the first handler that supports this architecture
        for handler in &self.handlers {
            if handler.supports(arch) {
                return handler.disassemble(bytes, address);
            }
        }

        // No handler found for this architecture
        Err(DisasmError::UnsupportedArchitecture(arch.to_string()))
    }

    /// Returns a list of all registered architecture names.
    ///
    /// This method returns the canonical names of all architectures that
    /// have been registered with this dispatcher. The names can be used
    /// to check what architectures are available or for UI display purposes.
    ///
    /// # Returns
    ///
    /// A vector of string slices containing the canonical architecture names.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::ArchitectureDispatcher;
    /// let dispatcher = ArchitectureDispatcher::new();
    /// let archs = dispatcher.supported_architectures();
    /// for arch in archs {
    ///     println!("Supported architecture: {}", arch);
    /// }
    /// ```
    pub fn supported_architectures(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|h| h.name()).collect()
    }

    /// Checks if a specific architecture is supported.
    ///
    /// This is a convenience method that can be used to check if the
    /// dispatcher can handle a particular architecture before attempting
    /// disassembly.
    ///
    /// # Arguments
    ///
    /// * `arch_name` - The architecture name to check
    ///
    /// # Returns
    ///
    /// `true` if the architecture is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::ArchitectureDispatcher;
    /// let dispatcher = ArchitectureDispatcher::new();
    /// if dispatcher.supports_architecture("riscv32") {
    ///     println!("RISC-V 32-bit is supported!");
    /// }
    /// ```
    pub fn supports_architecture(&self, arch_name: &str) -> bool {
        self.handlers.iter().any(|h| h.supports(arch_name))
    }

    /// Gets the handler for a specific architecture, if available.
    ///
    /// This method provides direct access to the underlying architecture
    /// handler, which can be useful for advanced use cases or testing.
    ///
    /// # Arguments
    ///
    /// * `arch_name` - The architecture name
    ///
    /// # Returns
    ///
    /// An optional reference to the architecture handler, or `None` if
    /// no handler supports the specified architecture.
    ///
    /// # Note
    ///
    /// This is primarily intended for internal use and testing. Most users
    /// should prefer the `disassemble` and `disassemble_bytes` methods.
    pub fn get_handler(&self, arch_name: &str) -> Option<&dyn ArchitectureHandler> {
        self.handlers
            .iter()
            .find(|h| h.supports(arch_name))
            .map(|h| h.as_ref())
    }
}

impl Default for ArchitectureDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_dispatcher_creation() {
        let dispatcher = ArchitectureDispatcher::new();
        let archs = dispatcher.supported_architectures();

        // Ensure RISC-V is present
        assert!(archs.contains(&"riscv"));
    }

    #[test]
    fn test_hex_parsing() {
        let dispatcher = ArchitectureDispatcher::new();

        // Hex parsing should succeed for bare strings.
        let instruction = dispatcher.disassemble("deadbeef", "unknown".to_string());
        assert_eq!(instruction.mnemonic, "unknown");
        assert_eq!(instruction.bytes, vec![0xef, 0xbe, 0xad, 0xde]);
        assert_eq!(instruction.size, 4);

        // Hex parsing should also accept a `0x` prefix.
        let instruction = dispatcher.disassemble("0x1234", "unknown".to_string());
        assert_eq!(instruction.bytes, vec![0x34, 0x12]);
        assert_eq!(instruction.size, 2);
    }
}
