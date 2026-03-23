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
        match self.architecture {
            ArchitectureId::Riscv => render_riscv_text_parts(
                self,
                profile,
                alias_regs,
                capstone_aliases,
                compressed_aliases,
                unsigned_immediate,
            ),
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

fn render_riscv_text_parts(
    instruction: &DecodedInstruction,
    profile: TextRenderProfile,
    alias_regs: bool,
    capstone_aliases: bool,
    compressed_aliases: bool,
    unsigned_immediate: bool,
) -> (String, String) {
    let use_capstone_aliases =
        capstone_aliases && (compressed_aliases || !instruction.mnemonic.starts_with("c."));

    let mnemonic = if matches!(profile, TextRenderProfile::Canonical) || !use_capstone_aliases {
        instruction.mnemonic.clone()
    } else {
        instruction
            .render_hints
            .capstone_mnemonic
            .clone()
            .unwrap_or_else(|| instruction.mnemonic.clone())
    };

    let hidden_operands =
        if matches!(profile, TextRenderProfile::Canonical) || !use_capstone_aliases {
            &[][..]
        } else {
            instruction.render_hints.capstone_hidden_operands.as_slice()
        };

    let visible_operands = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(index, _)| !hidden_operands.contains(index))
        .collect::<Vec<_>>();

    if mnemonic == "jalr" {
        return (
            mnemonic,
            format_riscv_jalr_operands(
                &visible_operands,
                &instruction.mode,
                alias_regs,
                unsigned_immediate,
            ),
        );
    }

    if mnemonic.starts_with("sc.") || mnemonic.starts_with("amo") {
        return (
            mnemonic,
            format_riscv_atomic_operands(
                &visible_operands,
                &instruction.mode,
                alias_regs,
                unsigned_immediate,
            ),
        );
    }

    let last_visible_index = visible_operands.last().map(|(index, _)| *index);
    let operands = visible_operands
        .iter()
        .map(|(index, operand)| {
            format_riscv_operand(
                &mnemonic,
                *index,
                operand,
                &instruction.mode,
                alias_regs,
                unsigned_immediate,
                last_visible_index,
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    (mnemonic, operands)
}

fn format_riscv_jalr_operands(
    operands: &[(usize, &Operand)],
    mode: &str,
    alias_regs: bool,
    unsigned_immediate: bool,
) -> String {
    let mut visible = operands.iter().map(|(_, operand)| *operand);
    match (visible.next(), visible.next(), visible.next()) {
        (
            Some(Operand::Register { register: rd }),
            Some(Operand::Register { register: rs1 }),
            Some(Operand::Immediate { value }),
        ) => format!(
            "{}, {}({})",
            format_riscv_register(rd.id, alias_regs),
            format_riscv_immediate(*value, mode, unsigned_immediate),
            format_riscv_register(rs1.id, alias_regs)
        ),
        (Some(Operand::Register { register: rs1 }), Some(Operand::Immediate { value }), None) => {
            format!(
                "{}({})",
                format_riscv_immediate(*value, mode, unsigned_immediate),
                format_riscv_register(rs1.id, alias_regs)
            )
        }
        _ => operands
            .iter()
            .map(|(_, operand)| {
                format_riscv_basic_operand(operand, mode, alias_regs, false, unsigned_immediate)
            })
            .collect::<Vec<_>>()
            .join(", "),
    }
}

fn format_riscv_atomic_operands(
    operands: &[(usize, &Operand)],
    mode: &str,
    alias_regs: bool,
    unsigned_immediate: bool,
) -> String {
    let mut rendered = Vec::new();
    let mut memory = None;

    for (_, operand) in operands {
        match operand {
            Operand::Memory {
                base: Some(base),
                displacement,
            } if *displacement == 0 => {
                memory = Some(format!("({})", format_riscv_register(base.id, alias_regs)));
            }
            Operand::Memory { .. } => {
                memory = Some(format_riscv_basic_operand(
                    operand,
                    mode,
                    alias_regs,
                    true,
                    unsigned_immediate,
                ));
            }
            _ => rendered.push(format_riscv_basic_operand(
                operand,
                mode,
                alias_regs,
                true,
                unsigned_immediate,
            )),
        }
    }

    if let Some(memory) = memory {
        rendered.push(memory);
    }

    rendered.join(", ")
}

fn format_riscv_operand(
    mnemonic: &str,
    index: usize,
    operand: &Operand,
    mode: &str,
    alias_regs: bool,
    unsigned_immediate: bool,
    last_visible_index: Option<usize>,
) -> String {
    match operand {
        Operand::Immediate { value } if is_riscv_csr_operand(mnemonic, index) => {
            csr_name_lookup(*value as u16)
                .map(str::to_string)
                .unwrap_or_else(|| format_riscv_immediate(*value, "", unsigned_immediate))
        }
        Operand::Immediate { value }
            if last_visible_index == Some(index) && is_riscv_control_flow_mnemonic(mnemonic) =>
        {
            format_riscv_control_immediate(*value, mode, unsigned_immediate)
        }
        Operand::Memory {
            base: Some(base),
            displacement,
        } if *displacement == 0 && is_riscv_atomic_memory_mnemonic(mnemonic) => {
            format!("({})", format_riscv_register(base.id, alias_regs))
        }
        _ => format_riscv_basic_operand(operand, mode, alias_regs, true, unsigned_immediate),
    }
}

fn format_riscv_basic_operand(
    operand: &Operand,
    mode: &str,
    alias_regs: bool,
    allow_control_hex: bool,
    unsigned_immediate: bool,
) -> String {
    match operand {
        Operand::Register { register } => format_riscv_register(register.id, alias_regs),
        Operand::Immediate { value } => {
            if allow_control_hex {
                format_riscv_immediate(*value, mode, unsigned_immediate)
            } else {
                format_riscv_control_immediate(*value, mode, unsigned_immediate)
            }
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            let displacement = format_riscv_immediate(*displacement, mode, unsigned_immediate);
            if let Some(base) = base {
                format!(
                    "{}({})",
                    displacement,
                    format_riscv_register(base.id, alias_regs)
                )
            } else {
                displacement
            }
        }
    }
}

fn format_riscv_register(register_id: u32, alias_regs: bool) -> String {
    if !alias_regs {
        return match register_id {
            0..=31 => format!("x{register_id}"),
            32..=63 => format!("f{}", register_id - 32),
            _ => format!("r{register_id}"),
        };
    }

    match register_id {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0",
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        32 => "ft0",
        33 => "ft1",
        34 => "ft2",
        35 => "ft3",
        36 => "ft4",
        37 => "ft5",
        38 => "ft6",
        39 => "ft7",
        40 => "fs0",
        41 => "fs1",
        42 => "fa0",
        43 => "fa1",
        44 => "fa2",
        45 => "fa3",
        46 => "fa4",
        47 => "fa5",
        48 => "fa6",
        49 => "fa7",
        50 => "fs2",
        51 => "fs3",
        52 => "fs4",
        53 => "fs5",
        54 => "fs6",
        55 => "fs7",
        56 => "fs8",
        57 => "fs9",
        58 => "fs10",
        59 => "fs11",
        60 => "ft8",
        61 => "ft9",
        62 => "ft10",
        63 => "ft11",
        _ => return format!("r{register_id}"),
    }
    .to_string()
}

fn format_riscv_immediate(value: i64, mode: &str, unsigned_immediate: bool) -> String {
    if unsigned_immediate && value < 0 {
        return format_riscv_unsigned_immediate(value, mode);
    }

    if value == 0 {
        return "0".to_string();
    }

    let abs = value.abs();
    let use_hex = abs > 9;
    if use_hex {
        if value < 0 {
            format!("-0x{abs:x}")
        } else {
            format!("0x{abs:x}")
        }
    } else {
        value.to_string()
    }
}

fn format_riscv_control_immediate(value: i64, mode: &str, unsigned_immediate: bool) -> String {
    if unsigned_immediate && value < 0 {
        return format_riscv_unsigned_immediate(value, mode);
    }
    if value >= 0 {
        return format_riscv_immediate(value, mode, unsigned_immediate);
    }

    format_riscv_unsigned_immediate(value, mode)
}

fn format_riscv_unsigned_immediate(value: i64, mode: &str) -> String {
    match mode {
        "riscv32" => format!("0x{:x}", value as u32),
        _ => format!("0x{:x}", value as u64),
    }
}

fn is_riscv_control_flow_mnemonic(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "j" | "jal"
            | "jalr"
            | "beq"
            | "bne"
            | "blt"
            | "bge"
            | "bltu"
            | "bgeu"
            | "beqz"
            | "bnez"
            | "c.j"
            | "c.jal"
            | "c.beqz"
            | "c.bnez"
    )
}

fn is_riscv_csr_operand(mnemonic: &str, index: usize) -> bool {
    matches!(
        mnemonic,
        "csrrw" | "csrrs" | "csrrc" | "csrrwi" | "csrrsi" | "csrrci" | "csrr" | "csrc" | "csrw"
    ) && index == 1
}

fn is_riscv_atomic_memory_mnemonic(mnemonic: &str) -> bool {
    mnemonic.starts_with("lr.") || mnemonic.starts_with("sc.") || mnemonic.starts_with("amo")
}

fn csr_name_lookup(csr: u16) -> Option<&'static str> {
    match csr {
        0x100 => Some("sstatus"),
        0x105 => Some("stvec"),
        0x106 => Some("scounteren"),
        0x143 => Some("stval"),
        0x180 => Some("satp"),
        0x305 => Some("mtvec"),
        0x342 => Some("mcause"),
        0xb00 => Some("mcycle"),
        0xc00 => Some("cycle"),
        _ => None,
    }
}
