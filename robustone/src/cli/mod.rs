pub mod arch;
pub mod command;
pub mod config;
pub mod disasm;
pub mod error;
pub mod executor;
pub mod utils;
pub mod version_info;

// Re-export commonly used types to preserve `crate::cli::{...}` compatibility.
pub use arch::{Architecture, ArchitectureSpec};
pub use command::Cli;
pub use config::DisasmConfig;

// Compatibility shim: keep legacy helpers that older crates expect from `crate::cli`.
use crate::cli::error::ValidationError;

/// Parse hexadecimal machine code into a byte buffer.
/// Preserved for backwards compatibility with earlier call sites.
pub fn parse_hex_code(input: &str) -> std::result::Result<Vec<u8>, ValidationError> {
    let words = crate::cli::utils::parse::parse_hex_code(input)?;
    crate::cli::utils::parse::hex_words_to_bytes(&words)
}

/// Parse a hexadecimal address string (legacy entry point).
pub fn parse_address(input: &str) -> std::result::Result<u64, ValidationError> {
    crate::cli::utils::parse_address(input)
}

/// Validate an architecture string (legacy entry point).
pub fn validate_architecture(arch_str: &str) -> Result<String, String> {
    crate::cli::utils::validate_architecture(arch_str)
}

#[cfg(test)]
mod tests;
