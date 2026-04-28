//! ISA-level decode framework shared across architecture backends.
//!
//! This crate provides the `ArchitectureBackend` trait and the generic
//! `decode_one()` pipeline. Architecture-specific crates (RISC-V, LoongArch)
//! implement `ArchitectureBackend` and use the shared pipeline to produce
//! `DecodedInstruction` IR.

use robustone_core::ir::{ArchitectureId, DecodedInstruction, Operand, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

// ============================================================================
// Core trait
// ============================================================================

/// Trait implemented by every architecture backend.
///
/// The associated types allow each ISA to define its own word size,
/// mode/feature enums, field layout, and register classes while sharing
/// the same decode pipeline via `decode_one()`.
pub trait ArchitectureBackend: Sized + Sync + 'static {
    /// Instruction word type (e.g. `u32` for RISC-V and LoongArch).
    type Word: Copy + Eq + core::fmt::Debug + Into<u64> + 'static;

    /// Architecture mode (e.g. RV32/RV64, LA32/LA64).
    type Mode: Copy + Eq + core::fmt::Debug + 'static;

    /// Feature bitflags (e.g. I/M/A/F/D/C, BASE/FLOAT64/PRIVILEGED).
    type Feature: FeatureSet;

    /// Field identifier used by format specs.
    type Field: Copy + Eq + core::fmt::Debug + 'static;

    /// Register class (e.g. X, F, Gpr, Fpr).
    type RegisterClass: Copy + Eq + core::fmt::Debug + 'static;

    /// Return the architecture identifier used in the shared IR.
    fn architecture_id() -> ArchitectureId;

    /// Read the next instruction from raw bytes.
    ///
    /// Returns `InstructionRead` containing the word value and the number
    /// of bytes consumed. For variable-length ISAs (e.g. RISC-V) the
    /// length field distinguishes compressed vs standard instructions.
    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError>;

    /// Look up an instruction spec for the given word.
    ///
    /// Returns `None` when the encoding is not defined in the spec table.
    fn lookup(
        word: Self::Word,
        profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>>;

    /// Lower a raw register number to a shared `RegisterId`.
    fn lower_register(
        class: Self::RegisterClass,
        raw: u32,
        profile: &DecodeProfile<Self>,
    ) -> RegisterId;

    /// Return the render policy for this profile.
    fn render_policy(profile: &DecodeProfile<Self>) -> RenderPolicy<Self>;

    /// Extract a field value from the raw instruction word.
    ///
    /// The default implementation requires `Self::Word: Into<u32>` and
    /// performs a straightforward bit slice extraction. Backends may
    /// override this for custom word types.
    fn extract_field(word: Self::Word, format: &FormatSpec<Self::Field>, field: Self::Field)
    -> u32;
}

// ============================================================================
// Instruction read result
// ============================================================================

/// Result of `ArchitectureBackend::read_instruction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionRead<W> {
    pub raw: W,
    pub length: u8,
}

// ============================================================================
// Feature set abstraction
// ============================================================================

/// Trait for backend feature bitflags.
///
/// Implementors are typically `bitflags!` structs.
pub trait FeatureSet: Copy + Eq + core::fmt::Debug + 'static {
    /// Empty feature set.
    fn empty() -> Self;
    /// All supported features (used for Capstone compatibility testing).
    fn all_supported_for_tests() -> Self;
    /// Check whether `self` contains all features in `required`.
    fn contains(self, required: Self) -> bool;
}

// ============================================================================
// Decode profile
// ============================================================================

/// Profile that controls decode behaviour for a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeProfile<B: ArchitectureBackend> {
    pub mode: B::Mode,
    pub features: B::Feature,
    pub render_dialect: RenderDialect,
    pub alias_policy: AliasPolicy,
}

/// Text rendering dialect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderDialect {
    #[default]
    Canonical,
    Assembler,
}

/// Alias policy applied during rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AliasPolicy {
    #[default]
    None,
    PreferPseudo,
}

// ============================================================================
// Instruction specification
// ============================================================================

/// Static specification of a single instruction.
#[derive(Debug)]
pub struct InstructionSpec<B: ArchitectureBackend + 'static> {
    pub mnemonic: &'static str,
    pub opcode_id: &'static str,
    pub pattern: EncodingPattern<B::Word>,
    pub format: &'static FormatSpec<B::Field>,
    pub operands: &'static [OperandSpec<B>],
    pub features: B::Feature,
    pub modes: ModeSet<B::Mode>,
    pub groups: &'static [InstructionGroup],
    pub manual_ref: Option<&'static str>,
}

/// Mask/value pattern for matching an instruction word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodingPattern<W> {
    pub mask: W,
    pub value: W,
}

/// Set of supported modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSet<M: Copy + Eq + 'static> {
    All,
    Only(&'static [M]),
}

/// Instruction functional groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionGroup {
    Integer,
    Arithmetic,
    Logical,
    Shift,
    Branch,
    Jump,
    Memory,
    Atomic,
    Float,
    Privileged,
    Barrier,
    System,
    Vector,
    BitManipulation,
}

// ============================================================================
// Format specification
// ============================================================================

/// Static specification of an instruction format (field layout).
#[derive(Debug)]
pub struct FormatSpec<F: Copy + Eq + 'static> {
    pub name: &'static str,
    pub fields: &'static [FieldSpec<F>],
}

/// A single field within a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSpec<F: Copy + Eq + 'static> {
    pub name: &'static str,
    pub start: u8,
    pub length: u8,
    pub field_type: F,
}

/// Helper to define a field spec.
pub const fn field<F: Copy + Eq + 'static>(
    name: &'static str,
    start: u8,
    length: u8,
    field_type: F,
) -> FieldSpec<F> {
    FieldSpec {
        name,
        start,
        length,
        field_type,
    }
}

// ============================================================================
// Operand specification
// ============================================================================

/// Static specification of a single operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandSpec<B: ArchitectureBackend + 'static> {
    Register {
        class: B::RegisterClass,
        field: B::Field,
        access: Access,
    },
    Immediate {
        field: B::Field,
        transform: ImmediateTransform,
        kind: ImmediateKind,
    },
}

/// Operand access direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
    ReadWrite,
}

/// Immediate value transformation applied after extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmediateTransform {
    None,
    SignExtend { bits: u8 },
    SignExtendThenShift { bits: u8, shift: u8 },
    ZeroExtend { bits: u8 },
}

/// Semantic kind of an immediate operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmediateKind {
    Absolute,
    PcRelative,
    Unsigned,
}

// ============================================================================
// Render policy
// ============================================================================

/// Per-backend render policy (dialect + alias + register name mapping).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderPolicy<B: ArchitectureBackend> {
    pub dialect: RenderDialect,
    pub alias_policy: AliasPolicy,
    pub _phantom: core::marker::PhantomData<B>,
}

impl<B: ArchitectureBackend> RenderPolicy<B> {
    pub const fn new(dialect: RenderDialect, alias_policy: AliasPolicy) -> Self {
        Self {
            dialect,
            alias_policy,
            _phantom: core::marker::PhantomData,
        }
    }
}

// ============================================================================
// Generic decode pipeline
// ============================================================================

/// Decode a single instruction using the generic pipeline.
///
/// The pipeline steps are:
/// 1. `read_instruction` – extract word and length from bytes.
/// 2. `lookup` – find matching `InstructionSpec`.
/// 3. Check mode / feature compatibility.
/// 4. Extract fields from the word using the format spec.
/// 5. Lower operands (registers, immediates).
/// 6. Infer register read/write lists from operand specs.
/// 7. Build `DecodedInstruction`.
pub fn decode_one<B: ArchitectureBackend>(
    bytes: &[u8],
    addr: u64,
    profile: &DecodeProfile<B>,
) -> Result<DecodedInstruction, DisasmError> {
    // Step 1: read instruction
    let instr = B::read_instruction(bytes)?;

    // Step 2: lookup spec
    let spec = B::lookup(instr.raw, profile).ok_or_else(|| {
        DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some(format!("{:?}", B::architecture_id())),
            format!("unknown encoding: raw={:?}", instr.raw),
        )
    })?;

    // Step 3: mode check
    if !spec.modes.matches(profile.mode) {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnsupportedMode,
            Some(format!("{:?}", B::architecture_id())),
            format!(
                "mode {:?} not supported for {}",
                profile.mode, spec.mnemonic
            ),
        ));
    }

    // Step 4: feature check
    if !profile.features.contains(spec.features) {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnsupportedExtension,
            Some(format!("{:?}", B::architecture_id())),
            format!(
                "features {:?} required for {} not enabled",
                spec.features, spec.mnemonic
            ),
        ));
    }

    // Step 5: extract fields and lower operands
    let mut operands = Vec::with_capacity(spec.operands.len());
    let mut regs_read = Vec::new();
    let mut regs_written = Vec::new();

    for op_spec in spec.operands {
        let operand = lower_operand::<B>(instr.raw, op_spec, profile, spec.format);
        operands.push(operand);

        // Infer register access
        if let OperandSpec::Register {
            class,
            field,
            access,
        } = op_spec
        {
            let raw = B::extract_field(instr.raw, spec.format, *field);
            let reg = B::lower_register(*class, raw, profile);
            match *access {
                Access::Read => regs_read.push(reg),
                Access::Write => regs_written.push(reg),
                Access::ReadWrite => {
                    regs_read.push(reg);
                    regs_written.push(reg);
                }
            }
        }
    }

    // Step 6: build DecodedInstruction
    let decoded = DecodedInstruction {
        architecture: B::architecture_id(),
        address: addr,
        mode: format!("{:?}", profile.mode),
        mnemonic: spec.mnemonic.to_string(),
        opcode_id: Some(spec.opcode_id.to_string()),
        size: instr.length as usize,
        raw_bytes: bytes[..instr.length as usize].to_vec(),
        operands,
        registers_read: regs_read,
        registers_written: regs_written,
        implicit_registers_read: Vec::new(),
        implicit_registers_written: Vec::new(),
        groups: spec.groups.iter().map(|g| format!("{:?}", g)).collect(),
        status: robustone_core::ir::DecodeStatus::Success,
        render_hints: robustone_core::ir::RenderHints::default(),
    };

    Ok(decoded)
}

/// Lower a single operand from the raw word.
fn lower_operand<B: ArchitectureBackend>(
    word: B::Word,
    spec: &OperandSpec<B>,
    _profile: &DecodeProfile<B>,
    format: &FormatSpec<B::Field>,
) -> Operand {
    match spec {
        OperandSpec::Register {
            class: _,
            field,
            access: _,
        } => {
            let raw = B::extract_field(word, format, *field);
            Operand::Register {
                register: RegisterId {
                    architecture: B::architecture_id(),
                    id: raw,
                },
            }
        }
        OperandSpec::Immediate {
            field,
            transform,
            kind,
        } => {
            let raw = B::extract_field(word, format, *field);
            let value = apply_transform(raw, *transform);
            match *kind {
                ImmediateKind::Absolute | ImmediateKind::PcRelative | ImmediateKind::Unsigned => {
                    Operand::Immediate { value }
                }
            }
        }
    }
}

/// Apply an immediate transformation.
fn apply_transform(raw: u32, transform: ImmediateTransform) -> i64 {
    match transform {
        ImmediateTransform::None => raw as i64,
        ImmediateTransform::SignExtend { bits } => {
            let mask = 1u32 << (bits - 1);
            if raw & mask != 0 {
                // Sign-extend
                let sign_extended = raw | !((1u32 << bits) - 1);
                sign_extended as i32 as i64
            } else {
                raw as i64
            }
        }
        ImmediateTransform::SignExtendThenShift { bits, shift } => {
            let mask = 1u32 << (bits - 1);
            let extended = if raw & mask != 0 {
                raw | !((1u32 << bits) - 1)
            } else {
                raw
            };
            ((extended as i32) << shift) as i64
        }
        ImmediateTransform::ZeroExtend { bits: _ } => raw as i64,
    }
}

// ============================================================================
// Helpers
// ============================================================================

impl<M: Copy + Eq + 'static> ModeSet<M> {
    pub fn matches(&self, mode: M) -> bool {
        match self {
            ModeSet::All => true,
            ModeSet::Only(modes) => modes.contains(&mode),
        }
    }
}

impl<W: Copy + Eq + core::fmt::Debug + 'static> EncodingPattern<W> {
    pub const fn new(mask: W, value: W) -> Self {
        Self { mask, value }
    }
}

// ============================================================================
// Mock backend for testing the framework
// ============================================================================

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;

    #[test]
    fn mock_decode_add() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };

        // add: pattern mask=0xFF00_0000, value=0x0100_0000
        // raw bytes for word = 0x0100_0000
        let bytes = &[0x00, 0x00, 0x00, 0x01];
        let result = decode_one::<MockBackend>(bytes, 0x1000, &profile);
        assert!(result.is_ok(), "{:?}", result);

        let insn = result.unwrap();
        assert_eq!(insn.mnemonic, "add");
        assert_eq!(insn.size, 4);
        assert_eq!(insn.opcode_id, Some("ADD".to_string()));
        assert_eq!(insn.registers_read.len(), 2);
        assert_eq!(insn.registers_written.len(), 1);
    }

    #[test]
    fn mock_invalid_encoding_returns_error() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };

        // Unknown encoding
        let bytes = &[0xFF, 0xFF, 0xFF, 0xFF];
        let result = decode_one::<MockBackend>(bytes, 0x1000, &profile);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.stable_kind().contains("invalid_encoding"),
            "got {}",
            err.stable_kind()
        );
    }

    #[test]
    fn mock_feature_gating_rejects_disabled() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::empty(),
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };

        let bytes = &[0x00, 0x00, 0x00, 0x01];
        let result = decode_one::<MockBackend>(bytes, 0x1000, &profile);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.stable_kind().contains("unsupported_extension"),
            "got {}",
            err.stable_kind()
        );
    }
}
