//! RISC-V instruction set extensions module.
//!
//! This module contains the implementation of various RISC-V instruction set
//! extensions, organized into separate modules for better maintainability.

use super::decoder::{RiscVDecodedInstruction, Xlen};
use crate::error::DisasmError;
use bitflags::bitflags;

bitflags! {
    /// Bitflags representing enabled RISC-V extensions.
    #[derive(Clone, Copy)]
    pub struct Extensions: u32 {
        const G             = Self::I.bits() | Self::M.bits() | Self::A.bits() | Self::F.bits() | Self::D.bits();
        const I             = 1 << 0;
        const M             = 1 << 1;
        const A             = 1 << 2;
        const F             = 1 << 3;
        const D             = 1 << 4;
        const C             = 1 << 5;
        const XTHEADCONDMOV = 1 << 6;
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
    fn is_enabled(&self, extensions: Extensions) -> bool;
}

// Standard RISC-V extension modules
pub mod rva; // RVA - Atomic Instructions
pub mod rvc; // RVC - Compressed Instructions
pub mod rvd; // RVD - Double-Precision Floating-Point
pub mod rvf; // RVF - Single-Precision Floating-Point
pub mod rvi; // RV32I/RV64I - Base Integer Instruction Set
pub mod rvm; // RVM - Multiply and Divide Instructions

// XuanTie vendor extension modules
pub mod xtheadcondmov; // XTheadCondMov - Conditional Move Instructions

use rva::RvaExtension;
use rvc::RvcExtension;
use rvd::RvdExtension;
use rvf::RvfExtension;
use rvi::RviExtension;
use rvm::RvmExtension;
use xtheadcondmov::XTheadCondMovExtension;

/// Create all available standard RISC-V extensions.
pub fn create_extensions() -> Vec<Box<dyn InstructionExtension>> {
    vec![
        Box::new(RviExtension::new()),
        Box::new(RvaExtension::new()),
        Box::new(RvmExtension::new()),
        Box::new(RvfExtension::new()),
        Box::new(RvdExtension::new()),
        Box::new(RvcExtension::new()),
        Box::new(XTheadCondMovExtension::new()),
    ]
}
