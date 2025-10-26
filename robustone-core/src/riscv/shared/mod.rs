//! Shared utilities for RISC-V instruction decoding.
//!
//! This module contains common utilities and shared functionality used across
//! all RISC-V extensions to eliminate code duplication and maintain consistency.

pub mod registers;
pub mod operands;
pub mod formatting;
pub mod encoding;

// Re-export commonly used items for convenience
pub use registers::{RegisterNameProvider, RegisterManager};
pub use operands::{OperandFactory, OperandFormatter};
pub use formatting::{ImmediateFormatter, InstructionFormatter};
pub use encoding::{SignExtender, InstructionDecoder};