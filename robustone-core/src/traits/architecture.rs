//! Architecture handler trait definition.
//!
//! This module defines the core trait that all architecture-specific
//! disassemblers must implement.

use crate::types::error::DisasmError;
use crate::types::instruction::Instruction;

/// Trait that all architecture-specific disassemblers must implement.
///
/// This trait provides a unified interface for disassembling instructions
/// across different architectures. Each architecture handler is responsible
/// for parsing the raw byte representation of instructions and converting
/// them into a standardized `Instruction` format.
///
/// # Thread Safety
///
/// All implementations must be thread-safe (`Sync`) because handlers may
/// be shared across multiple threads in the dispatcher.
///
/// # Required Methods
///
/// * `disassemble`: Core method that performs the actual disassembly
/// * `name`: Returns the canonical name of the architecture
/// * `supports`: Checks if the handler supports a given architecture name
///
/// # Example Implementation
///
/// ```rust
/// use robustone_core::prelude::*;
///
/// pub struct MyArchitectureHandler;
///
/// impl ArchitectureHandler for MyArchitectureHandler {
///     fn disassemble(&self, bytes: &[u8], addr: u64) -> Result<(Instruction, usize), DisasmError> {
///         // Architecture-specific disassembly logic here
///         todo!("Implement actual disassembly logic")
///     }
///
///     fn name(&self) -> &'static str {
///         "myarch"
///     }
///
///     fn supports(&self, arch_name: &str) -> bool {
///         arch_name == "myarch" || arch_name == "myarch32" || arch_name == "myarch64"
///     }
/// }
/// ```
pub trait ArchitectureHandler: Sync {
    /// Disassembles a single instruction from the provided bytes.
    ///
    /// This is the core method that performs the actual disassembly work.
    /// Implementations should parse the instruction bytes and return both
    /// the decoded instruction and the number of bytes consumed.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw instruction bytes to decode
    /// * `addr` - The address where these bytes would be located in memory
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - The decoded `Instruction`
    /// - The number of bytes consumed from the input
    ///
    /// # Errors
    ///
    /// Returns a `DisasmError` if:
    /// - The bytes cannot be decoded as a valid instruction
    /// - The input is malformed or incomplete
    /// - An architecture-specific error occurs
    fn disassemble(&self, bytes: &[u8], addr: u64) -> Result<(Instruction, usize), DisasmError>;

    /// Returns the canonical name of this architecture.
    ///
    /// This should return the primary, canonical name for the architecture.
    /// For example, "riscv", "arm", "x86", etc. This name is used for
    /// identification and logging purposes.
    ///
    /// # Returns
    ///
    /// A string slice containing the canonical architecture name.
    fn name(&self) -> &'static str;

    /// Checks whether this handler supports the given architecture name.
    ///
    /// This method allows a single handler to support multiple variations
    /// of the same architecture. For example, an ARM handler might support
    /// "arm", "arm32", "armv7", "thumb", etc.
    ///
    /// The exact matching behavior (including case sensitivity) is
    /// implementation-defined; callers should pass canonical architecture
    /// names (typically lowercase) as expected by each handler.
    ///
    /// # Arguments
    ///
    /// * `arch_name` - The architecture name to check
    ///
    /// # Returns
    ///
    /// `true` if this handler can disassemble for the given architecture,
    /// `false` otherwise.
    fn supports(&self, arch_name: &str) -> bool;
}
