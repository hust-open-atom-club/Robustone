//! Core types for the Robustone disassembly engine.

pub mod error;
pub mod instruction;

pub use error::DisasmError;
pub use instruction::Instruction;
