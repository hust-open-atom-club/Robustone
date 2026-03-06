//! Shared utilities for RISC-V instruction decoding.
//!
//! This module contains common utilities and shared functionality used across
//! all RISC-V extensions to eliminate code duplication and maintain consistency.

pub mod encoding;
pub mod formatting;
pub mod operands;
pub mod registers;

// Re-export commonly used items for convenience
pub use encoding::{InstructionDecoder, SignExtender};
pub use formatting::{ImmediateFormatter, InstructionFormatter};
pub use operands::{OperandFactory, OperandFormatter};
pub use registers::{RegisterManager, RegisterNameProvider};
