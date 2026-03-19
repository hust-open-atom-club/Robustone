//! Architecture-agnostic decoded-instruction IR.
//!
//! This module provides the structured representation that decode backends
//! should populate before any display-oriented formatting happens.

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
    Register { register: RegisterId },
    Immediate { value: i64 },
    Text { value: String },
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
    /// Serialize the decoded instruction as pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
