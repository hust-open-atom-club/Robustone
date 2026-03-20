//! Architecture-agnostic decoded-instruction IR.
//!
//! This module provides the structured representation that decode backends
//! should populate before any display-oriented formatting happens.

use crate::riscv::printer::{RiscVPrinter, RiscVTextProfile};
use serde::Serialize;

/// Architectures that can currently populate the shared IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchitectureId {
    Riscv,
}

/// Machine-readable decode status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DecodeStatus {
    Success,
    NeedMoreBytes,
    InvalidEncoding,
    UnsupportedExtension,
    Unimplemented,
}

/// Text output profiles derived from the shared IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRenderProfile {
    Capstone,
    Canonical,
    VerboseDebug,
}

/// Shared register identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RegisterId {
    pub architecture: ArchitectureId,
    pub id: u32,
}

impl RegisterId {
    /// Create a register identifier for the RISC-V backend.
    pub const fn riscv(id: u32) -> Self {
        Self {
            architecture: ArchitectureId::Riscv,
            id,
        }
    }
}

/// Shared operand representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Operand {
    Register {
        register: RegisterId,
    },
    Immediate {
        value: i64,
    },
    Text {
        value: String,
    },
    Memory {
        base: Option<RegisterId>,
        displacement: i64,
    },
}

/// Display-oriented rendering hints derived from the structured decode result.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct RenderHints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capstone_mnemonic: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capstone_hidden_operands: Vec<usize>,
}

/// Shared decoded instruction payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DecodedInstruction {
    pub architecture: ArchitectureId,
    pub address: u64,
    pub mode: String,
    pub mnemonic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opcode_id: Option<String>,
    pub size: usize,
    pub raw_bytes: Vec<u8>,
    pub operands: Vec<Operand>,
    pub registers_read: Vec<RegisterId>,
    pub registers_written: Vec<RegisterId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicit_registers_read: Vec<RegisterId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicit_registers_written: Vec<RegisterId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    pub status: DecodeStatus,
    #[serde(default)]
    pub render_hints: RenderHints,
}

impl DecodedInstruction {
    /// Fill in decode context that is only known at the final call site.
    pub fn with_context(
        mut self,
        mode: impl Into<String>,
        address: u64,
        raw_bytes: Vec<u8>,
    ) -> Self {
        self.mode = mode.into();
        self.address = address;
        self.raw_bytes = raw_bytes;
        self
    }

    /// Set a Capstone-facing alias mnemonic and optional hidden operands.
    pub fn with_capstone_alias(
        mut self,
        capstone_mnemonic: impl Into<String>,
        hidden_operands: Vec<usize>,
    ) -> Self {
        self.render_hints.capstone_mnemonic = Some(capstone_mnemonic.into());
        self.render_hints.capstone_hidden_operands = hidden_operands;
        self
    }

    /// Hide the specified operands in the Capstone-facing outward view.
    pub fn with_hidden_operands(mut self, hidden_operands: Vec<usize>) -> Self {
        self.render_hints.capstone_hidden_operands = hidden_operands;
        self
    }

    /// Render the instruction into mnemonic / operands text using the shared IR.
    pub fn render_text_parts(&self, profile: TextRenderProfile) -> (String, String) {
        match self.architecture {
            ArchitectureId::Riscv => {
                let printer = RiscVPrinter::new().with_profile(match profile {
                    TextRenderProfile::Capstone => RiscVTextProfile::Capstone,
                    TextRenderProfile::Canonical => RiscVTextProfile::Canonical,
                    TextRenderProfile::VerboseDebug => RiscVTextProfile::VerboseDebug,
                });
                printer.render_ir_parts(self)
            }
        }
    }

    /// Render the instruction using the Capstone-compatible text profile.
    pub fn render_capstone_text_parts(&self) -> (String, String) {
        self.render_text_parts(TextRenderProfile::Capstone)
    }

    /// Render the instruction using the canonical text profile.
    pub fn render_canonical_text_parts(&self) -> (String, String) {
        self.render_text_parts(TextRenderProfile::Canonical)
    }

    /// Serialize the decoded instruction as pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
