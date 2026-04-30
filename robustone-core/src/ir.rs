//! Architecture-agnostic decoded-instruction IR.
//!
//! This module provides the structured representation that decode backends
//! should populate before any display-oriented formatting happens.

use serde::Serialize;

/// Architectures that can currently populate the shared IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureId {
    Riscv,
    Arm,
    X86,
    LoongArch,
    /// Architecture identifier for dynamically added or experimental backends.
    Other(&'static str),
}

impl ArchitectureId {
    pub fn as_str(&self) -> &str {
        match self {
            ArchitectureId::Riscv => "riscv",
            ArchitectureId::Arm => "arm",
            ArchitectureId::X86 => "x86",
            ArchitectureId::LoongArch => "loongarch",
            ArchitectureId::Other(name) => name,
        }
    }
}

impl Serialize for ArchitectureId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
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
    Compat,
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
    /// Create a register identifier with an arbitrary architecture.
    pub const fn new(architecture: ArchitectureId, id: u32) -> Self {
        Self { architecture, id }
    }

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

    /// Create a register identifier for the ARM backend.
    pub const fn arm(id: u32) -> Self {
        Self {
            architecture: ArchitectureId::Arm,
            id,
        }
    }

    /// Create a register identifier for the x86 backend.
    pub const fn x86(id: u32) -> Self {
        Self {
            architecture: ArchitectureId::X86,
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
    pub compat_mnemonic: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compat_hidden_operands: Vec<usize>,
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

    /// Set a decoder-facing alias mnemonic and optional hidden operands.
    pub fn with_compat_alias(
        mut self,
        compat_mnemonic: impl Into<String>,
        hidden_operands: Vec<usize>,
    ) -> Self {
        self.render_hints.compat_mnemonic = Some(compat_mnemonic.into());
        self.render_hints.compat_hidden_operands = hidden_operands;
        self
    }

    /// Hide the specified operands in the decoder-facing outward view.
    pub fn with_hidden_operands(mut self, hidden_operands: Vec<usize>) -> Self {
        self.render_hints.compat_hidden_operands = hidden_operands;
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
        _profile: TextRenderProfile,
        _alias_regs: bool,
        _compat_aliases: bool,
        _compressed_aliases: bool,
        _unsigned_immediate: bool,
    ) -> (String, String) {
        // Generic fallback for architectures without a custom renderer.
        let operands = self
            .operands
            .iter()
            .map(format_generic_operand)
            .collect::<Vec<_>>()
            .join(", ");
        (self.mnemonic.clone(), operands)
    }

    /// Render the instruction using the decoder-compatible text profile.
    pub fn render_compat_text_parts(&self) -> (String, String) {
        self.render_text_parts(TextRenderProfile::Compat)
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
        ArchitectureId::Other(name) => name,
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
        let (mnemonic, operands) = instruction.render_compat_text_parts();
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
        let (_, operands) = instruction.render_compat_text_parts();
        assert_eq!(operands, "riscv:5, 8(riscv:2)");
    }

    #[test]
    fn generic_renderer_uses_stored_mnemonic() {
        let instruction = sample_instruction("c.addi", vec![]);
        let (mnemonic, _) = instruction.render_compat_text_parts();
        assert_eq!(mnemonic, "c.addi");
    }

    #[test]
    fn compat_hidden_operands_are_ignored_by_generic_renderer() {
        let mut instruction = sample_instruction(
            "jal",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Immediate { value: 0x1000 },
            ],
        );
        instruction.render_hints.compat_hidden_operands = vec![0];
        let (_, operands) = instruction.render_compat_text_parts();
        // Generic renderer does not apply hidden operands
        assert_eq!(operands, "riscv:1, 4096");
    }

    #[test]
    fn render_hints_compat_mnemonic_is_ignored_by_generic_renderer() {
        let mut instruction = sample_instruction(
            "addi",
            vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Immediate { value: 1 },
            ],
        );
        instruction.render_hints.compat_mnemonic = Some("li".to_string());
        let (mnemonic, _) = instruction.render_compat_text_parts();
        // Generic renderer does not apply compat mnemonic aliases
        assert_eq!(mnemonic, "addi");
    }
}
