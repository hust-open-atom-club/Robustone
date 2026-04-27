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
    Arm,
    X86,
    LoongArch,
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

/// Function signature for architecture-specific instruction text rendering.
/// Per-architecture crates provide an implementation and attach it to
/// `DecodedInstruction::render` so that `robustone-core` remains free of
/// architecture-specific formatting code.
pub type RenderFn = fn(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    alias_regs: bool,
    capstone_aliases: bool,
    compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String);

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

    /// Create a register identifier for the LoongArch backend.
    pub const fn loongarch(id: u32) -> Self {
        Self {
            architecture: ArchitectureId::LoongArch,
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
#[allow(unpredictable_function_pointer_comparisons)]
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
    /// Optional architecture-specific renderer. Set by architecture crates
    /// (e.g. `robustone-riscv`) so that text rendering can happen outside
    /// `robustone-core`.
    #[serde(skip)]
    pub render: Option<RenderFn>,
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
        self.render_text_parts_with_options(
            profile,
            !matches!(profile, TextRenderProfile::Canonical),
            !matches!(profile, TextRenderProfile::Canonical),
            !matches!(profile, TextRenderProfile::Canonical),
            false,
        )
    }

    pub fn render_text_parts_with_options(
        &self,
        profile: TextRenderProfile,
        alias_regs: bool,
        capstone_aliases: bool,
        compressed_aliases: bool,
        unsigned_immediate: bool,
    ) -> (String, String) {
        if let Some(render) = self.render {
            return render(
                self,
                profile,
                alias_regs,
                capstone_aliases,
                compressed_aliases,
                unsigned_immediate,
            );
        }
        // Generic fallback for architectures without a custom renderer.
        let operands = self
            .operands
            .iter()
            .map(format_generic_operand)
            .collect::<Vec<_>>()
            .join(", ");
        (self.mnemonic.clone(), operands)
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

fn format_generic_operand(operand: &Operand) -> String {
    let arch_str = |arch: ArchitectureId| match arch {
        ArchitectureId::Riscv => "riscv",
        ArchitectureId::Arm => "arm",
        ArchitectureId::X86 => "x86",
        ArchitectureId::LoongArch => "loongarch",
    };
    match operand {
        Operand::Register { register } => {
            format!("{}:{}", arch_str(register.architecture), register.id)
        }
        Operand::Immediate { value } => value.to_string(),
        Operand::Text { value } => value.clone(),
        Operand::Memory {
            base: Some(base),
            displacement,
        } => {
            format!(
                "{}({}:{})",
                displacement,
                arch_str(base.architecture),
                base.id
            )
        }
        Operand::Memory {
            base: None,
            displacement,
        } => displacement.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_instruction(mnemonic: &str, operands: Vec<Operand>) -> DecodedInstruction {
        DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: "riscv32".to_string(),
            mnemonic: mnemonic.to_string(),
            opcode_id: Some(mnemonic.to_string()),
            size: 4,
            raw_bytes: vec![0; 4],
            operands,
            registers_read: Vec::new(),
            registers_written: Vec::new(),
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: Vec::new(),
            status: DecodeStatus::Success,
            render_hints: RenderHints::default(),
            render: None,
        }
    }

    #[test]
    fn generic_renderer_formats_operands() {
        let instruction = sample_instruction(
            "addi",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Register {
                    register: RegisterId::riscv(2),
                },
                Operand::Immediate { value: 42 },
            ],
        );
        let (mnemonic, operands) = instruction.render_capstone_text_parts();
        assert_eq!(mnemonic, "addi");
        assert_eq!(operands, "riscv:1, riscv:2, 42");
    }

    #[test]
    fn generic_renderer_formats_memory() {
        let instruction = sample_instruction(
            "lw",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(5),
                },
                Operand::Memory {
                    base: Some(RegisterId::riscv(2)),
                    displacement: 8,
                },
            ],
        );
        let (_, operands) = instruction.render_capstone_text_parts();
        assert_eq!(operands, "riscv:5, 8(riscv:2)");
    }

    #[test]
    fn generic_renderer_uses_stored_mnemonic() {
        let instruction = sample_instruction("c.addi", vec![]);
        let (mnemonic, _) = instruction.render_capstone_text_parts();
        assert_eq!(mnemonic, "c.addi");
    }

    #[test]
    fn capstone_hidden_operands_are_ignored_by_generic_renderer() {
        let mut instruction = sample_instruction(
            "jal",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Immediate { value: 0x1000 },
            ],
        );
        instruction.render_hints.capstone_hidden_operands = vec![0];
        let (_, operands) = instruction.render_capstone_text_parts();
        // Generic renderer does not apply hidden operands
        assert_eq!(operands, "riscv:1, 4096");
    }

    #[test]
    fn render_hints_capstone_mnemonic_is_ignored_by_generic_renderer() {
        let mut instruction = sample_instruction(
            "addi",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Immediate { value: 1 },
            ],
        );
        instruction.render_hints.capstone_mnemonic = Some("li".to_string());
        let (mnemonic, _) = instruction.render_capstone_text_parts();
        // Generic renderer does not apply capstone mnemonic aliases
        assert_eq!(mnemonic, "addi");
    }
}
