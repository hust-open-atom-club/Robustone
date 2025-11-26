//! RISC-V instruction set extensions module.
//!
//! This module contains the implementation of various RISC-V instruction set
//! extensions, organized into separate modules for better maintainability.

use super::decoder::{RiscVDecodedInstruction, Xlen};
use crate::error::DisasmError;

// Submodules grouping standard and custom-specific extensions.
pub mod standard;
pub mod thead;

use standard::Standard;
use thead::THead;

/// Aggregated extension configuration passed to RISC-V extension handlers.
pub struct Extensions {
    pub(crate) standard: Standard,
    pub(crate) thead: THead,
}

impl Extensions {
    /// Convenience configuration for RV32GC profile with all standard and
    /// no T-Head custom extensions enabled.
    pub fn rv32gc() -> Self {
        Self {
            standard: Standard::G | Standard::C,
            thead: THead::empty(),
        }
    }

    /// Convenience configuration for RV64GC profile with all standard and
    /// no T-Head custom extensions enabled.
    pub fn rv64gc() -> Self {
        Self {
            standard: Standard::G | Standard::C,
            thead: THead::empty(),
        }
    }

    /// Enables all available T-Head custom extensions on this configuration.
    pub fn thead(mut self) -> Self {
        self.thead |= THead::all();
        self
    }
}

/// Trait that all instruction set extensions must implement.
#[allow(clippy::too_many_arguments)]
pub trait InstructionExtension: Sync {
    /// Try to decode a standard 32-bit instruction.
    ///
    /// Returns `Some(Ok(instruction))` if this extension can decode the instruction,
    /// `Some(Err(error))` if decoding fails within this extension,
    /// or `None` if this extension doesn't handle the instruction.
    fn try_decode_standard(
        &self,
        opcode: u32,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
        funct12: u32,
        imm_i: i64,
        imm_s: i64,
        imm_b: i64,
        imm_u: i64,
        imm_j: i64,
        xlen: Xlen,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>>;

    /// Try to decode a compressed 16-bit instruction.
    ///
    /// Returns `Some(Ok(instruction))` if this extension can decode the compressed instruction,
    /// `Some(Err(error))` if decoding fails within this extension,
    /// or `None` if this extension doesn't handle the instruction.
    fn try_decode_compressed(
        &self,
        instruction: u16,
        opcode: u8,
        funct3: u8,
        xlen: Xlen,
        // Compressed instruction fields
        rd_full: u8,
        rs1_full: u8,
        rs2_full: u8,
        rdp: u8,
        rs1p: u8,
        rs2p: u8,
        // Compressed immediates
        nzuimm_ciw: u16,
        uimm_cl: u16,
        uimm_cs: u16,
        imm_ci: i64,
        imm_cj: i64,
        imm_cb: i64,
        uimm_css: u16,
        uimm_clsp: u16,
        uimm_fldsp: u16,
    ) -> Option<Result<RiscVDecodedInstruction, DisasmError>>;

    /// Get the name of this extension.
    fn name(&self) -> &'static str;

    /// Check if this extension is enabled for the given configuration.
    fn is_enabled(&self, extensions: &Extensions) -> bool;
}

/// Create all available standard RISC-V extensions.
pub fn create_extensions() -> Vec<Box<dyn InstructionExtension>> {
    vec![
        Box::new(standard::Rvi::new()),
        Box::new(standard::Rva::new()),
        Box::new(standard::Rvm::new()),
        Box::new(standard::Rvf::new()),
        Box::new(standard::Rvd::new()),
        Box::new(standard::Rvc::new()),
        Box::new(thead::CMov::new()),
    ]
}
