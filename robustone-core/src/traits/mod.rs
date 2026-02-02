//! Core traits for the Robustone disassembly engine.
//!
//! This module contains the fundamental traits that define the architecture
//! for extensible disassembly support.

pub mod architecture;
pub mod instruction;

pub use architecture::ArchitectureHandler;
pub use instruction::{BasicInstructionDetail, Detail};
