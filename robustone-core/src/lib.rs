#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

//! Robustone – Core disassembly engine with multi-architecture support.
//!
//! This crate provides a flexible, extensible disassembly framework inspired by
//! reference decoder design but implemented in pure Rust. The architecture is designed
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
//! 3. Register the handler in an `ArchitectureDispatcher`
//!
//! # Example
//!
//! ```rust,ignore
//! // Use the meta-crate helper for a ready-to-use dispatcher:
//! use robustone::dispatcher;
//!
//! let dispatcher = dispatcher();
//! match dispatcher.disassemble_bytes(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0x1000) {
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
//!
//! `ArchitectureDispatcher::new()` and `ArchitectureDispatcher::default()` return an
//! empty dispatcher with no handlers registered. You must call `register()` to add
//! architecture backends before disassembling.

extern crate alloc;

pub mod architecture;
pub mod common;
pub mod decode_config;
pub mod ir;
pub mod render;
pub mod renderer;
pub mod traits;
pub mod types;
pub mod utils;

/// Robustone prelude.
///
/// Re-exports frequently used types and traits for convenient importing.
/// This module provides access to the most common functionality needed for
/// using the disassembly engine.
pub mod prelude {
    pub use crate::architecture::{
        Architecture, ArchitectureCapability, all_architecture_capabilities,
        canonical_architecture_name, is_address_aligned, lookup_architecture_capability,
    };
    pub use crate::common::ArchitectureProfile;
    pub use crate::decode_config::{
        Arch, DecodeConfig, DecodeConfigError, DetailLevel, FeatureSet, Mode, parse_decode_config,
    };
    pub use crate::ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId};
    pub use crate::render::{
        RenderOptions, RenderedDisassembly, RenderedInstruction, RenderedIssue, render_disassembly,
        render_instruction_text,
    };
    pub use crate::traits::{ArchitectureHandler, BasicInstructionDetail, Detail};
    pub use crate::types::{DisasmError, Instruction};
    pub use crate::utils::{Endianness, HexParser};
}

pub use architecture::{
    ArchitectureCapability, all_architecture_capabilities, canonical_architecture_name,
    lookup_architecture_capability,
};
pub use decode_config::{
    Arch, DecodeConfig, DecodeConfigError, DetailLevel, FeatureSet, Mode, parse_decode_config,
};
pub use ir::DecodedInstruction;
pub use render::{
    RenderOptions, RenderedDisassembly, RenderedInstruction, RenderedIssue, render_disassembly,
    render_instruction_text,
};
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
    /// Creates a new empty dispatcher.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            hex_parser: HexParser::new(),
        }
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

    /// Sets the detail flag on all registered handlers.
    ///
    /// This mirrors the reference decoder `CS_OPT_DETAIL` option. When `false`, handlers
    /// may skip expensive detail construction and return `Instruction` objects
    /// with `detail` set to `None`.
    pub fn set_detail(&mut self, detail: bool) {
        for handler in &mut self.handlers {
            handler.set_detail(detail);
        }
    }

    /// Legacy convenience method for disassembling a hex string.
    ///
    /// **Deprecated:** This is a compatibility shim for demos/REPL only.
    /// It silently swallows parse and decode errors, returning an `unknown`
    /// instruction instead of a `Result`. For production analysis tooling,
    /// use [`Self::disassemble_bytes`] which returns `Result`.
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
    #[deprecated(
        since = "0.0.1",
        note = "Use disassemble_bytes() which returns Result instead of swallowing errors"
    )]
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
                    decoded: None,
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
                    decoded: None,
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
    /// let dispatcher = ArchitectureDispatcher::default();
    /// let bytes = [0x93, 0x00, 0x10, 0x00]; // addi ra, zero, 1
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
                return handler.disassemble(bytes, arch, address);
            }
        }

        // No handler found for this architecture
        Err(DisasmError::UnsupportedArchitecture(arch.to_string()))
    }

    /// Decode raw instruction bytes into the shared IR.
    pub fn decode_instruction(
        &self,
        bytes: &[u8],
        arch: &str,
        address: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        for handler in &self.handlers {
            if handler.supports(arch) {
                return handler.decode_instruction(bytes, arch, address);
            }
        }

        Err(DisasmError::UnsupportedArchitecture(arch.to_string()))
    }

    /// Decode bytes using a strongly-typed `DecodeConfig`.
    ///
    /// This is the preferred entry point for new code. The old
    /// `decode_instruction(&self, bytes, arch: &str, address)` is retained as
    /// a deprecated compatibility shim.
    pub fn decode_instruction_with_config(
        &self,
        bytes: &[u8],
        config: &crate::decode_config::DecodeConfig,
        address: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        config
            .validate()
            .map_err(|e| DisasmError::Configuration(e.to_string()))?;
        let profile = crate::common::ArchitectureProfile::from(config);
        for handler in &self.handlers {
            if handler.supports(profile.mode_name) {
                return handler.decode_instruction_with_profile(bytes, &profile, address);
            }
        }
        Err(DisasmError::UnsupportedArchitecture(
            profile.mode_name.to_string(),
        ))
    }

    /// Disassemble bytes using a strongly-typed `DecodeConfig`.
    pub fn disassemble_bytes_with_config(
        &self,
        bytes: &[u8],
        config: &crate::decode_config::DecodeConfig,
        address: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        config
            .validate()
            .map_err(|e| DisasmError::Configuration(e.to_string()))?;
        let profile = crate::common::ArchitectureProfile::from(config);
        for handler in &self.handlers {
            if handler.supports(profile.mode_name) {
                return handler.disassemble_with_profile(bytes, &profile, address);
            }
        }
        Err(DisasmError::UnsupportedArchitecture(
            profile.mode_name.to_string(),
        ))
    }

    /// Decode bytes using an explicit architecture profile.
    pub fn decode_with_profile(
        &self,
        bytes: &[u8],
        profile: &crate::common::ArchitectureProfile,
        address: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        for handler in &self.handlers {
            if handler.supports(profile.mode_name) {
                return handler.decode_instruction_with_profile(bytes, profile, address);
            }
        }

        Err(DisasmError::UnsupportedArchitecture(
            profile.architecture.as_str().to_string(),
        ))
    }

    /// Disassemble bytes using an explicit architecture profile.
    pub fn disassemble_with_profile(
        &self,
        bytes: &[u8],
        profile: &crate::common::ArchitectureProfile,
        address: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        for handler in &self.handlers {
            if handler.supports(profile.mode_name) {
                return handler.disassemble_with_profile(bytes, profile, address);
            }
        }

        Err(DisasmError::UnsupportedArchitecture(
            profile.architecture.as_str().to_string(),
        ))
    }

    /// Returns the renderer for a given architecture, if available.
    ///
    /// Falls back to the generic renderer if the handler does not provide
    /// a dedicated one.
    pub fn get_renderer(&self, arch: &str) -> &dyn crate::renderer::Renderer {
        for handler in &self.handlers {
            if handler.supports(arch) {
                return handler
                    .renderer()
                    .unwrap_or(&crate::renderer::GenericRenderer);
            }
        }
        &crate::renderer::GenericRenderer
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
    /// let dispatcher = ArchitectureDispatcher::default();
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
    /// let dispatcher = ArchitectureDispatcher::default();
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
    /// Creates a dispatcher pre-populated with all handlers registered via
    /// `inventory::submit!` in the backend crates.
    fn default() -> Self {
        let mut dispatcher = Self::new();
        for factory in inventory::iter::<crate::traits::HandlerFactory> {
            dispatcher.register((factory.factory)());
        }
        dispatcher
    }
}

#[cfg(test)]
mod tests {
    use crate::ArchitectureDispatcher;
    use crate::traits::ArchitectureHandler;

    struct MockHandler;

    impl ArchitectureHandler for MockHandler {
        fn name(&self) -> &'static str {
            "mock"
        }

        fn supports(&self, _arch: &str) -> bool {
            false
        }

        fn decode_instruction(
            &self,
            _bytes: &[u8],
            _arch_name: &str,
            _address: u64,
        ) -> Result<(crate::DecodedInstruction, usize), crate::DisasmError> {
            Err(crate::DisasmError::UnsupportedArchitecture(
                "mock".to_string(),
            ))
        }

        fn disassemble(
            &self,
            _bytes: &[u8],
            _arch_name: &str,
            _address: u64,
        ) -> Result<(crate::Instruction, usize), crate::DisasmError> {
            Err(crate::DisasmError::UnsupportedArchitecture(
                "mock".to_string(),
            ))
        }
    }

    #[test]
    fn dispatcher_starts_empty_and_accepts_registered_handlers() {
        let empty = ArchitectureDispatcher::new();
        assert!(empty.supported_architectures().is_empty());

        let mut dispatcher = ArchitectureDispatcher::new();
        dispatcher.register(Box::new(MockHandler));
        assert!(!dispatcher.supported_architectures().is_empty());
        assert!(dispatcher.supported_architectures().contains(&"mock"));
    }

    #[test]
    fn dispatcher_reports_unsupported_architecture_for_unknown_mode() {
        let dispatcher = ArchitectureDispatcher::new();
        let error = dispatcher
            .decode_instruction(&[0x00, 0x00, 0x00, 0x00], "nonexistent", 0)
            .expect_err("unknown architecture should fail");
        assert_eq!(error.stable_kind(), "unsupported_architecture");
    }

    #[test]
    fn hex_parser_accepts_bare_and_prefixed_hex() {
        let parser = crate::utils::HexParser::new();

        let bytes = parser.parse("deadbeef", None).unwrap();
        assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);

        let bytes = parser.parse("0x1234", None).unwrap();
        assert_eq!(bytes, vec![0x12, 0x34]);
    }
}
