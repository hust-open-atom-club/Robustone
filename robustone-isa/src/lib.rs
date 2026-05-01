#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

//! ISA-level decode framework shared across architecture backends.
//!
//! This crate provides the `ArchitectureBackend` trait and the generic
//! `decode_one()` pipeline. Architecture-specific crates (RISC-V, LoongArch)
//! implement `ArchitectureBackend` and use the shared pipeline to produce
//! `DecodedInstruction` IR.

extern crate alloc;

use robustone_core::ir::{ArchitectureId, DecodedInstruction, Operand, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};

// ============================================================================
// Generic decoder type
// ============================================================================

/// Architecture-specific decoder produced by the shared framework.
///
/// Architecture crates re-export this as their public decoder type:
/// ```ignore
/// pub type LoongArchDecoder = robustone_isa::Decoder<LoongArchBackend>;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Decoder<B: ArchitectureBackend> {
    _phantom: core::marker::PhantomData<B>,
}

impl<B: ArchitectureBackend> Decoder<B> {
    /// Create a new decoder instance.
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    /// Decode a single instruction.
    pub fn decode(
        &self,
        bytes: &[u8],
        addr: u64,
        profile: &DecodeProfile<B>,
    ) -> Result<DecodedInstruction, DisasmError> {
        decode_one::<B>(bytes, addr, profile)
    }
}

impl<B: ArchitectureBackend> Default for Decoder<B> {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// Returns `Err` when the field is not found in the format or when
    /// the word does not contain valid data for the field.
    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> Result<u32, DisasmError>;

    /// Apply architecture-specific aliases to a decoded instruction.
    ///
    /// The default implementation is a no-op. Backends with alias
    /// rules (declared via `define_aliases!`) override this to set
    /// `render_hints.compat_mnemonic` and hidden operands.
    fn apply_aliases(_decoded: &mut DecodedInstruction) {}
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
    /// All supported features (used for compatibility testing).
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
///
/// Fields are `pub(crate)` — construction is only possible via
/// `InstructionSpec::new()` (called by proc-macros). Downstream
/// crates use the public accessor methods for lookup and decode.
#[derive(Debug)]
pub struct InstructionSpec<B: ArchitectureBackend + 'static> {
    pub(crate) mnemonic: &'static str,
    pub(crate) opcode_id: &'static str,
    pub(crate) pattern: EncodingPattern<B::Word>,
    pub(crate) format: &'static FormatSpec<B::Field>,
    pub(crate) operands: &'static [OperandSpec<B>],
    pub(crate) features: B::Feature,
    pub(crate) modes: ModeSet<B::Mode>,
    pub(crate) groups: &'static [InstructionGroup],
    pub(crate) effect: Option<EffectSpec>,
    pub(crate) manual_ref: Option<&'static str>,
    pub(crate) priority: u16,
}

impl<B: ArchitectureBackend + 'static> InstructionSpec<B> {
    /// Construct a new `InstructionSpec`.
    ///
    /// This is the canonical constructor used by proc-macros.
    /// Architecture crates should use `define_instructions!` rather
    /// than calling this directly.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        mnemonic: &'static str,
        opcode_id: &'static str,
        pattern: EncodingPattern<B::Word>,
        format: &'static FormatSpec<B::Field>,
        operands: &'static [OperandSpec<B>],
        features: B::Feature,
        modes: ModeSet<B::Mode>,
        groups: &'static [InstructionGroup],
        effect: Option<EffectSpec>,
        manual_ref: Option<&'static str>,
        priority: u16,
    ) -> Self {
        Self {
            mnemonic,
            opcode_id,
            pattern,
            format,
            operands,
            features,
            modes,
            groups,
            effect,
            manual_ref,
            priority,
        }
    }

    /// Instruction mnemonic (e.g. "add", "lw").
    pub fn mnemonic(&self) -> &'static str {
        self.mnemonic
    }

    /// Stable opcode identifier used for cross-referencing.
    pub fn opcode_id(&self) -> &'static str {
        self.opcode_id
    }

    /// Mask/value pattern for matching instruction words.
    pub fn pattern(&self) -> &EncodingPattern<B::Word> {
        &self.pattern
    }

    /// Format specification defining the field layout.
    pub fn format(&self) -> &FormatSpec<B::Field> {
        self.format
    }

    /// Operand specifications (registers, immediates, etc.).
    pub fn operands(&self) -> &[OperandSpec<B>] {
        self.operands
    }

    /// Required feature set for this instruction.
    pub fn features(&self) -> B::Feature {
        self.features
    }

    /// Supported modes for this instruction.
    pub fn modes(&self) -> &ModeSet<B::Mode> {
        &self.modes
    }

    /// Functional groups this instruction belongs to.
    pub fn groups(&self) -> &[InstructionGroup] {
        self.groups
    }

    /// Semantic effect classification.
    pub fn effect(&self) -> Option<EffectSpec> {
        self.effect
    }

    /// Optional reference to the architecture manual.
    pub fn manual_ref(&self) -> Option<&'static str> {
        self.manual_ref
    }

    /// Priority for disambiguation when multiple patterns match.
    pub fn priority(&self) -> u16 {
        self.priority
    }
}

impl<B: ArchitectureBackend + 'static> Clone for InstructionSpec<B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: ArchitectureBackend + 'static> Copy for InstructionSpec<B> {}

/// Mask/value pattern for matching an instruction word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodingPattern<W> {
    pub mask: W,
    pub value: W,
}

// ============================================================================
// Phase 1: Core abstractions (T1.1–T1.3)
// ============================================================================

/// Trait for instruction encoding tokens.
///
/// Represents the raw encoding of an instruction before decoding.
/// Different ISAs use different encoding schemes:
/// - Fixed-width 32-bit words (RISC-V, LoongArch, AArch64)
/// - Variable-length prefix-based (x86)
/// - Bytecode (WebAssembly)
pub trait EncodingToken: Copy + Eq + core::fmt::Debug + 'static {
    /// The fixed-width word type for bit-field extraction.
    /// For variable-length encodings, this is the primary opcode word.
    type Word: Copy + Eq + core::fmt::Debug + Into<u64>;

    /// Total size of the encoded instruction in bytes.
    fn size(&self) -> u8;

    /// Extract a bit field from the primary word.
    fn extract(&self, start: u8, length: u8) -> u32;
}

/// Fixed 32-bit instruction word encoding.
/// Used by RISC-V, LoongArch, AArch64, and ARM (Thumb-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedWord32(pub u32);

impl EncodingToken for FixedWord32 {
    type Word = u32;

    fn size(&self) -> u8 {
        4
    }

    fn extract(&self, start: u8, length: u8) -> u32 {
        if length == 0 || start >= 32 {
            return 0;
        }
        let effective_len = length.min(32 - start);
        let mask = if effective_len >= 32 {
            0xFFFF_FFFF
        } else {
            (1u32 << effective_len) - 1
        };
        (self.0 >> start) & mask
    }
}

/// x86 variable-length instruction encoding.
///
/// Placeholder for the full prefix + opcode + ModRM + SIB + disp + imm
/// encoding used by x86/x86-64. Phase 6 will flesh out the complete
/// structure; for now this stub proves the `EncodingToken` trait is
/// generic enough to handle non-fixed-width encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86Encoding {
    /// Total instruction size in bytes.
    pub size: u8,
    /// Primary opcode byte.
    pub opcode: u8,
    /// ModRM byte (if present).
    pub modrm: Option<u8>,
}

impl EncodingToken for X86Encoding {
    type Word = u8;

    fn size(&self) -> u8 {
        self.size
    }

    fn extract(&self, start: u8, length: u8) -> u32 {
        if length == 0 || start >= 8 {
            return 0;
        }
        let effective_len = length.min(8 - start);
        let mask = if effective_len >= 8 {
            0xFF
        } else {
            (1u8 << effective_len) - 1
        };
        ((self.opcode >> start) & mask) as u32
    }
}

/// WebAssembly bytecode instruction encoding.
///
/// WebAssembly uses a variable-length LEB128-based encoding for its
/// bytecode instructions. This stub proves the `EncodingToken` trait
/// handles fundamentally different encoding schemes beyond machine-code
/// ISAs. Phase 6+ will flesh out the complete structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmEncoding {
    /// Total instruction size in bytes (including LEB128 operands).
    pub size: u8,
    /// Primary opcode byte.
    pub opcode: u8,
}

impl EncodingToken for WasmEncoding {
    type Word = u8;

    fn size(&self) -> u8 {
        self.size
    }

    fn extract(&self, start: u8, length: u8) -> u32 {
        if length == 0 || start >= 8 {
            return 0;
        }
        let effective_len = length.min(8 - start);
        let mask = if effective_len >= 8 {
            0xFF
        } else {
            (1u8 << effective_len) - 1
        };
        ((self.opcode >> start) & mask) as u32
    }
}

/// A single part of a composed immediate.
///
/// Maps a contiguous source bit field to a destination position
/// in the final immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmComposePart {
    /// Source bit field start position in the instruction word.
    pub src_start: u8,
    /// Source bit field length.
    pub src_length: u8,
    /// Destination bit position in the composed immediate.
    pub dst_start: u8,
}

/// Expression describing how an immediate value is constructed from
/// one or more bit fields in the instruction encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmExpr {
    /// Single contiguous bit field with optional transform.
    Simple {
        field_start: u8,
        field_length: u8,
        transform: ImmediateTransform,
    },
    /// Compose an immediate from multiple non-contiguous bit fields.
    ///
    /// Used for:
    /// - RISC-V B-type: imm[12|10:5|4:1|11] from bits [31],[30:25],[11:8],[7]
    /// - RISC-V J-type: imm[20|10:1|11|19:12] from bits [31],[30:21],[20],[19:12]
    /// - RISC-V S-type: imm[11:5|4:0] from bits [31:25],[11:7]
    /// - LoongArch I26: disp[15:0|25:16] from bits [25:10],[9:0]
    Compose {
        parts: &'static [ImmComposePart],
        transform: ImmediateTransform,
    },
}

impl ImmExpr {
    /// Evaluate this expression against a raw instruction word.
    pub fn evaluate(&self, word: u32) -> i64 {
        match *self {
            ImmExpr::Simple {
                field_start,
                field_length,
                transform,
            } => {
                let raw = if field_length >= 32 {
                    word
                } else {
                    let mask = (1u32 << field_length) - 1;
                    (word >> field_start) & mask
                };
                apply_transform(raw, transform)
            }
            ImmExpr::Compose { parts, transform } => {
                let mut composed: u32 = 0;
                for part in parts {
                    let raw = if part.src_length >= 32 {
                        word
                    } else {
                        let mask = (1u32 << part.src_length) - 1;
                        (word >> part.src_start) & mask
                    };
                    composed |= raw << part.dst_start;
                }
                apply_transform(composed, transform)
            }
        }
    }
}

/// Memory expression describing a memory operand's addressing mode.
///
/// Covers all主流架构 memory operand forms:
/// - LoongArch: base + disp
/// - RISC-V: base + imm
/// - AArch64: base + imm, pre-indexed, post-indexed
/// - x86: segment:base + index*scale + disp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemExpr<B: ArchitectureBackend + 'static> {
    /// Base register field.
    pub base: Option<B::Field>,
    /// Index register field (for scaled indexed addressing).
    pub index: Option<B::Field>,
    /// Scale factor (1, 2, 4, 8).
    pub scale: u8,
    /// Displacement (immediate offset).
    pub displacement: i64,
    /// Segment register field (x86-specific).
    pub segment: Option<B::Field>,
    /// Whether this is a pre-indexed access (AArch64).
    pub pre_indexed: bool,
    /// Whether this is a post-indexed access (AArch64).
    pub post_indexed: bool,
}

/// Semantic roles a register encoding can take.
///
/// Some ISAs use the same register encoding for different semantic
/// roles depending on instruction context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterRoleKind {
    /// Register encoding is treated as the stack pointer.
    StackPointer,
    /// Register encoding is treated as the zero register.
    ZeroRegister,
    /// Register encoding is treated as the link register.
    LinkRegister,
}

/// Constraint on instruction encoding legality beyond pattern matching.
///
/// Some instructions have additional constraints that cannot be expressed
/// by mask/value patterns alone (e.g., register must be non-zero,
/// certain feature combinations are illegal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingConstraint<B: ArchitectureBackend + 'static> {
    /// A specific register field must not be zero.
    ///
    /// Example: RISC-V `c.jr` requires rs1 ≠ x0.
    RegisterNotZero { field: B::Field },
    /// A specific register field must equal a given value.
    ///
    /// Example: Some pseudo-instructions encode an implicit register.
    RegisterEquals { field: B::Field, value: u32 },
    /// A specific mode is required.
    ///
    /// Example: Compressed instructions only valid in specific modes.
    ModeRequired(B::Mode),
    /// A specific feature combination is required.
    ///
    /// Example: D-extension requires F-extension.
    FeatureRequired(B::Feature),
    /// Two register fields must not alias (must be different registers).
    ///
    /// Example: AArch64 load-pair with identical destination registers is UNPREDICTABLE.
    RegistersDistinct {
        field_a: B::Field,
        field_b: B::Field,
    },
    /// Two register fields must alias (must be the same register).
    ///
    /// Example: Some vector instructions require Rd == Rm.
    RegistersAlias {
        field_a: B::Field,
        field_b: B::Field,
    },
    /// One feature requires another feature to also be present.
    ///
    /// Example: RISC-V D-extension (double-precision float) requires
    /// F-extension (single-precision float).
    FeatureDepends {
        feature: B::Feature,
        requires: B::Feature,
    },
    /// A register field must encode a specific semantic role in context.
    ///
    /// Example: AArch64 register 31 encodes SP in load/store addressing
    /// but XZR in data-processing instructions.
    RegisterRole {
        field: B::Field,
        role: RegisterRoleKind,
    },
}

// ============================================================================
// Phase 1 T1.4: EffectSpec + InstructionView
// ============================================================================

// Re-export EffectSpec from robustone_core so downstream crates have a single import path.
pub use robustone_core::ir::EffectSpec;

/// Static specification of a register bank.
///
/// Describes a named group of registers (e.g. GPR, FPR, vector) within
/// an architecture. Used by `define_registers!` to generate register
/// metadata and lowering logic.
#[derive(Debug, Clone, Copy)]
pub struct RegisterBankSpec<B: ArchitectureBackend + 'static> {
    /// Human-readable name (e.g. "Gpr", "Float").
    pub name: &'static str,
    /// Number of registers in the bank.
    pub count: u32,
    /// Register class this bank maps to.
    pub class: B::RegisterClass,
    /// Optional prefix for textual rendering (e.g. "$r" → "$r1").
    pub prefix: Option<&'static str>,
    /// Named aliases for specific register indices (e.g. 0 → "$zero").
    pub aliases: &'static [(&'static str, u32)],
}

/// Typed view over a decoded instruction's fields.
///
/// Provides safe, type-checked access to instruction operands
/// without exposing the raw `DecodedInstruction` internals.
#[derive(Debug, Clone)]
pub struct InstructionView<'a> {
    decoded: &'a DecodedInstruction,
}

impl<'a> InstructionView<'a> {
    /// Create a new view over a decoded instruction.
    pub fn new(decoded: &'a DecodedInstruction) -> Self {
        Self { decoded }
    }

    /// The instruction mnemonic.
    pub fn mnemonic(&self) -> &str {
        &self.decoded.mnemonic
    }

    /// The instruction size in bytes.
    pub fn size(&self) -> usize {
        self.decoded.size
    }

    /// The instruction address.
    pub fn address(&self) -> u64 {
        self.decoded.address
    }

    /// Iterate over operands.
    pub fn operands(&self) -> impl Iterator<Item = &'a Operand> {
        self.decoded.operands.iter()
    }

    /// Number of operands.
    pub fn operand_count(&self) -> usize {
        self.decoded.operands.len()
    }

    /// Get the operand at the given index.
    pub fn operand(&self, index: usize) -> Option<&'a Operand> {
        self.decoded.operands.get(index)
    }

    /// Registers read by this instruction.
    pub fn registers_read(&self) -> &'a [RegisterId] {
        &self.decoded.registers_read
    }

    /// Registers written by this instruction.
    pub fn registers_written(&self) -> &'a [RegisterId] {
        &self.decoded.registers_written
    }

    /// Instruction functional groups.
    pub fn groups(&self) -> impl Iterator<Item = &'a str> {
        self.decoded.groups.iter().map(|s| s.as_str())
    }

    /// Check if the instruction belongs to a specific group.
    pub fn is_in_group(&self, group: &str) -> bool {
        self.decoded.groups.iter().any(|g| g == group)
    }

    /// Semantic effect of this instruction (branch, call, return, etc.).
    pub fn effect(&self) -> Option<EffectSpec> {
        self.decoded.effect
    }
}

// ============================================================================
// Set of supported modes.
// ============================================================================

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
    Compressed,
}

impl InstructionGroup {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstructionGroup::Integer => "integer",
            InstructionGroup::Arithmetic => "arithmetic",
            InstructionGroup::Logical => "logical",
            InstructionGroup::Shift => "shift",
            InstructionGroup::Branch => "branch",
            InstructionGroup::Jump => "control_flow",
            InstructionGroup::Memory => "memory",
            InstructionGroup::Atomic => "atomic",
            InstructionGroup::Float => "floating_point",
            InstructionGroup::Privileged => "privileged",
            InstructionGroup::Barrier => "barrier",
            InstructionGroup::System => "system",
            InstructionGroup::Vector => "vector",
            InstructionGroup::BitManipulation => "bit_manipulation",
            InstructionGroup::Compressed => "compressed",
        }
    }
}

// ============================================================================
// Format specification
// ============================================================================

/// Static specification of an instruction format (field layout).
///
/// Fields are `pub(crate)` — construction is via `FormatSpec::new()`
/// or the `format_specs!` / `define_formats!` macros.
#[derive(Debug)]
pub struct FormatSpec<F: Copy + Eq + 'static> {
    pub(crate) name: &'static str,
    pub(crate) fields: &'static [FieldSpec<F>],
}

impl<F: Copy + Eq + 'static> FormatSpec<F> {
    /// Construct a new format specification.
    pub const fn new(name: &'static str, fields: &'static [FieldSpec<F>]) -> Self {
        Self { name, fields }
    }

    /// Format name (e.g. "R", "R3", "I26").
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Field specifications defining the bit layout.
    pub fn fields(&self) -> &[FieldSpec<F>] {
        self.fields
    }
}

/// A single field within a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSpec<F: Copy + Eq + 'static> {
    pub(crate) name: &'static str,
    pub(crate) start: u8,
    pub(crate) length: u8,
    pub(crate) field_type: F,
}

impl<F: Copy + Eq + 'static> FieldSpec<F> {
    /// Construct a new field specification.
    pub const fn new(name: &'static str, start: u8, length: u8, field_type: F) -> Self {
        Self {
            name,
            start,
            length,
            field_type,
        }
    }

    /// Field name (e.g. "rd", "rs1", "imm12").
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Starting bit position.
    pub fn start(&self) -> u8 {
        self.start
    }
    /// Bit length.
    pub fn length(&self) -> u8 {
        self.length
    }
    /// Field type discriminator.
    pub fn field_type(&self) -> F {
        self.field_type
    }
}

/// Helper to define a field spec.
pub const fn field<F: Copy + Eq + 'static>(
    name: &'static str,
    start: u8,
    length: u8,
    field_type: F,
) -> FieldSpec<F> {
    FieldSpec::new(name, start, length, field_type)
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
    Text {
        field: B::Field,
        transform: ImmediateTransform,
    },
    Memory {
        base_class: B::RegisterClass,
        base_field: B::Field,
        displacement: i64,
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
    AddConst { value: i64 },
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
            Some(B::architecture_id().as_str().to_string()),
            format!("unknown encoding: raw={:?}", instr.raw),
        )
    })?;

    // Step 3: mode check
    if !spec.modes.matches(profile.mode) {
        return Err(DisasmError::decode_failure(
            DecodeErrorKind::UnsupportedMode,
            Some(B::architecture_id().as_str().to_string()),
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
            Some(B::architecture_id().as_str().to_string()),
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
        let operand = lower_operand::<B>(instr.raw, op_spec, profile, spec.format)?;
        operands.push(operand);

        // Infer register access
        if let OperandSpec::Register {
            class,
            field,
            access,
        } = op_spec
        {
            let raw = B::extract_field(instr.raw, spec.format, *field)?;
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
    let mut decoded = DecodedInstruction {
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
        groups: spec.groups.iter().map(|g| g.as_str().to_string()).collect(),
        effect: spec.effect,
        status: robustone_core::ir::DecodeStatus::Success,
        render_hints: robustone_core::ir::RenderHints::default(),
    };

    // Step 7: apply architecture-specific aliases
    B::apply_aliases(&mut decoded);

    Ok(decoded)
}

/// Lower a single operand from the raw word.
fn lower_operand<B: ArchitectureBackend>(
    word: B::Word,
    spec: &OperandSpec<B>,
    profile: &DecodeProfile<B>,
    format: &FormatSpec<B::Field>,
) -> Result<Operand, DisasmError> {
    match spec {
        OperandSpec::Register {
            class,
            field,
            access: _,
        } => {
            let raw = B::extract_field(word, format, *field)?;
            let reg = B::lower_register(*class, raw, profile);
            Ok(Operand::Register { register: reg })
        }
        OperandSpec::Immediate {
            field,
            transform,
            kind,
        } => {
            let raw = B::extract_field(word, format, *field)?;
            let value = apply_transform(raw, *transform);
            Ok(match *kind {
                ImmediateKind::Absolute | ImmediateKind::PcRelative | ImmediateKind::Unsigned => {
                    Operand::Immediate { value }
                }
            })
        }
        OperandSpec::Text { field, transform } => {
            let raw = B::extract_field(word, format, *field)?;
            let value = apply_transform(raw, *transform);
            Ok(Operand::Text {
                value: value.to_string(),
            })
        }
        OperandSpec::Memory {
            base_class,
            base_field,
            displacement,
        } => {
            let raw = B::extract_field(word, format, *base_field)?;
            let reg = B::lower_register(*base_class, raw, profile);
            Ok(Operand::Memory {
                base: Some(reg),
                displacement: *displacement,
            })
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
        ImmediateTransform::AddConst { value } => raw as i64 + value,
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

    /// Check whether this mode set and another have no modes in common.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        match (self, other) {
            (ModeSet::All, _) | (_, ModeSet::All) => false,
            (ModeSet::Only(a), ModeSet::Only(b)) => a.iter().all(|x| !b.contains(x)),
        }
    }
}

impl<W: Copy + Eq + core::fmt::Debug + Into<u64> + 'static> EncodingPattern<W> {
    pub const fn new(mask: W, value: W) -> Self {
        Self { mask, value }
    }

    /// Check whether this pattern overlaps with another.
    ///
    /// Two patterns overlap if there exists at least one word that matches both.
    pub fn overlaps(&self, other: &Self) -> bool {
        let self_mask: u64 = self.mask.into();
        let self_value: u64 = self.value.into();
        let other_mask: u64 = other.mask.into();
        let other_value: u64 = other.value.into();
        let common_mask = self_mask & other_mask;
        ((self_value ^ other_value) & common_mask) == 0
    }
}

/// Validate that no patterns in the spec table overlap.
///
/// Returns `Ok(())` if all patterns are disjoint. Otherwise returns an error
/// describing the first overlap found.
///
/// This should be called in debug builds or tests.
pub fn validate_no_overlaps<B: ArchitectureBackend>(
    specs: &[InstructionSpec<B>],
) -> Result<(), String> {
    for i in 0..specs.len() {
        for j in (i + 1)..specs.len() {
            if specs[i].pattern.overlaps(&specs[j].pattern) {
                // Overlaps with different priorities are allowed: the higher-priority
                // (more specific) spec is tried first by lookup.
                if specs[i].priority != specs[j].priority {
                    continue;
                }
                // Overlaps with disjoint mode sets are allowed: they can never
                // match the same profile (e.g. c.addiw on RV64 vs c.jal on RV32).
                if specs[i].modes.is_disjoint(&specs[j].modes) {
                    continue;
                }
                return Err(format!(
                    "pattern overlap detected between '{}' (mask={:?}, value={:?}) \
                     and '{}' (mask={:?}, value={:?})",
                    specs[i].mnemonic,
                    specs[i].pattern.mask,
                    specs[i].pattern.value,
                    specs[j].mnemonic,
                    specs[j].pattern.mask,
                    specs[j].pattern.value,
                ));
            }
        }
    }
    Ok(())
}

/// Validate a spec table for overlap, opcode_id uniqueness, and manual_ref presence.
pub fn check_spec_table<B: ArchitectureBackend>(
    specs: &[InstructionSpec<B>],
) -> Result<(), String> {
    // 1. Check for overlaps
    validate_no_overlaps(specs)?;

    // 2. Check opcode_id uniqueness
    let mut seen = alloc::collections::BTreeMap::new();
    for spec in specs {
        if let Some(prev) = seen.insert(spec.opcode_id, spec.mnemonic) {
            return Err(format!(
                "duplicate opcode_id '{}' used by '{}' and '{}'",
                spec.opcode_id, prev, spec.mnemonic
            ));
        }
    }

    // 3. Check manual_ref presence
    let missing_manual: Vec<_> = specs
        .iter()
        .filter(|s| s.manual_ref.is_none())
        .map(|s| s.mnemonic)
        .collect();
    if !missing_manual.is_empty() {
        return Err(format!(
            "check-spec: {} specs missing manual_ref: {:?}",
            missing_manual.len(),
            missing_manual
        ));
    }

    // 4. Check group self-consistency (every spec must belong to at least one group)
    let missing_groups: Vec<_> = specs
        .iter()
        .filter(|s| s.groups.is_empty())
        .map(|s| s.mnemonic)
        .collect();
    if !missing_groups.is_empty() {
        return Err(format!(
            "{} specs missing groups: {:?}",
            missing_groups.len(),
            missing_groups
        ));
    }

    // 5. Check that every operand field exists in the format
    for spec in specs {
        for op in spec.operands {
            let field = match op {
                OperandSpec::Register { field, .. } => *field,
                OperandSpec::Immediate { field, .. } => *field,
                OperandSpec::Text { field, .. } => *field,
                OperandSpec::Memory { base_field, .. } => *base_field,
            };
            let found = spec.format.fields.iter().any(|f| f.field_type == field);
            if !found {
                return Err(format!(
                    "spec '{}' operand references unknown field in format '{}'",
                    spec.mnemonic, spec.format.name
                ));
            }
        }
    }

    // 6. Check that no operand field is fully covered by the pattern mask
    for spec in specs {
        let mask: u64 = spec.pattern.mask.into();
        for op in spec.operands {
            let field = match op {
                OperandSpec::Register { field, .. } => *field,
                OperandSpec::Immediate { field, .. } => *field,
                OperandSpec::Text { field, .. } => *field,
                OperandSpec::Memory { base_field, .. } => *base_field,
            };
            if let Some(field_spec) = spec.format.fields.iter().find(|f| f.field_type == field) {
                let start = field_spec.start as u64;
                let length = field_spec.length as u64;
                if length == 0 || start + length > 64 {
                    continue;
                }
                let field_mask = ((1u64 << length) - 1) << start;
                if (mask & field_mask) == field_mask {
                    return Err(format!(
                        "spec '{}' operand field '{}' is fully covered by pattern mask; \
                         the operand cannot vary",
                        spec.mnemonic, field_spec.name
                    ));
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// Declarative spec macros
// ============================================================================

/// Define one or more static `FormatSpec` instances.
///
/// Example:
/// ```ignore
/// format_specs! {
///     format MOCK_FORMAT_R[MockField] {
///         rd: field("rd", 0, 5, MockField::Rd),
///         rs1: field("rs1", 5, 5, MockField::Rs1),
///     }
/// }
/// ```
#[macro_export]
macro_rules! format_specs {
    (
        $(format $name:ident[$field_ty:ty] {
            $($field_name:ident: $field_expr:expr),* $(,)?
        })*
    ) => {
        $(pub static $name: $crate::FormatSpec<$field_ty> = $crate::FormatSpec::new(
            stringify!($name),
            &[$($field_expr),*],
        );)*
    };
}

/// Define one or more static `InstructionSpec` instances.
///
/// Example:
/// ```ignore
/// isa_specs! {
///     backend = MockBackend;
///     spec ADD {
///         mnemonic = "add";
///         opcode_id = "ADD";
///         pattern = mask_value(0xFF00_0000, 0x0100_0000);
///         format = &MOCK_FORMAT_R;
///         operands = &[reg!(MockRegisterClass::Gpr, MockField::Rd, Access::Write), ...];
///         features = MockFeature::BASE;
///         modes = ModeSet::All;
///         groups = &[InstructionGroup::Integer, InstructionGroup::Arithmetic];
///     }
/// }
/// ```
#[macro_export]
macro_rules! isa_specs {
    (
        backend = $backend:ty;
        $(spec $name:ident {
            mnemonic = $mnemonic:expr;
            opcode_id = $opcode_id:expr;
            pattern = $pattern:expr;
            format = $format:expr;
            operands = $operands:expr;
            features = $features:expr;
            modes = $modes:expr;
            groups = $groups:expr;
            manual = $manual:expr;
        })*
    ) => {
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec::new(
            $mnemonic,
            $opcode_id,
            $pattern,
            $format,
            $operands,
            $features,
            $modes,
            $groups,
            None,
            Some($manual),
            0,
        );)*
    };
    (
        backend = $backend:ty;
        $(spec $name:ident {
            mnemonic = $mnemonic:expr;
            opcode_id = $opcode_id:expr;
            pattern = $pattern:expr;
            format = $format:expr;
            operands = $operands:expr;
            features = $features:expr;
            modes = $modes:expr;
            groups = $groups:expr;
        })*
    ) => {
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec::new(
            $mnemonic,
            $opcode_id,
            $pattern,
            $format,
            $operands,
            $features,
            $modes,
            $groups,
            None,
            None,
            0,
        );)*
    };
    (
        backend = $backend:ty;
        $(spec $name:ident {
            mnemonic = $mnemonic:expr;
            opcode_id = $opcode_id:expr;
            pattern = $pattern:expr;
            format = $format:expr;
            operands = $operands:expr;
            features = $features:expr;
            modes = $modes:expr;
            groups = $groups:expr;
            manual = $manual:expr;
            priority = $priority:expr;
        })*
    ) => {
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec::new(
            $mnemonic,
            $opcode_id,
            $pattern,
            $format,
            $operands,
            $features,
            $modes,
            $groups,
            None,
            Some($manual),
            $priority,
        );)*
    };
    (
        backend = $backend:ty;
        $(spec $name:ident {
            mnemonic = $mnemonic:expr;
            opcode_id = $opcode_id:expr;
            pattern = $pattern:expr;
            format = $format:expr;
            operands = $operands:expr;
            features = $features:expr;
            modes = $modes:expr;
            groups = $groups:expr;
            priority = $priority:expr;
        })*
    ) => {
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec::new(
            $mnemonic,
            $opcode_id,
            $pattern,
            $format,
            $operands,
            $features,
            $modes,
            $groups,
            None,
            None,
            $priority,
        );)*
    };
}

/// Helper to construct a register operand spec.
#[macro_export]
macro_rules! reg {
    ($class:expr, $field:expr, $access:expr) => {
        $crate::OperandSpec::Register {
            class: $class,
            field: $field,
            access: $access,
        }
    };
}

/// Helper to construct an immediate operand spec.
#[macro_export]
macro_rules! imm {
    ($field:expr, $transform:expr, $kind:expr) => {
        $crate::OperandSpec::Immediate {
            field: $field,
            transform: $transform,
            kind: $kind,
        }
    };
}

/// Helper to construct a text operand spec.
#[macro_export]
macro_rules! text {
    ($field:expr, $transform:expr) => {
        $crate::OperandSpec::Text {
            field: $field,
            transform: $transform,
        }
    };
}

/// Helper to construct a memory operand spec.
#[macro_export]
macro_rules! mem {
    ($class:expr, $base_field:expr, $displacement:expr) => {
        $crate::OperandSpec::Memory {
            base_class: $class,
            base_field: $base_field,
            displacement: $displacement,
        }
    };
}

/// Helper to construct an `EncodingPattern` from mask and value.
#[macro_export]
macro_rules! mask_value {
    ($mask:expr, $value:expr) => {
        $crate::EncodingPattern::new($mask, $value)
    };
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

    #[test]
    fn mock_specs_have_no_overlaps() {
        assert!(validate_no_overlaps(mock::MOCK_SPECS).is_ok());
    }

    #[test]
    fn validate_no_overlaps_detects_conflict() {
        use crate::{EncodingPattern, FormatSpec, InstructionSpec, ModeSet};

        let overlapping_specs: &[InstructionSpec<MockBackend>] = &[
            InstructionSpec {
                mnemonic: "inst_a",
                opcode_id: "INST_A",
                pattern: EncodingPattern::new(0xFF00_0000, 0x0100_0000),
                format: &FormatSpec {
                    name: "R",
                    fields: &[],
                },
                operands: &[],
                features: mock::MockFeature::BASE,
                modes: ModeSet::All,
                groups: &[],
                effect: None,
                manual_ref: None,
                priority: 0,
            },
            InstructionSpec {
                mnemonic: "inst_b",
                opcode_id: "INST_B",
                pattern: EncodingPattern::new(0x0F00_0000, 0x0100_0000),
                format: &FormatSpec {
                    name: "R",
                    fields: &[],
                },
                operands: &[],
                features: mock::MockFeature::BASE,
                modes: ModeSet::All,
                groups: &[],
                effect: None,
                manual_ref: None,
                priority: 0,
            },
        ];
        let result = validate_no_overlaps(overlapping_specs);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("inst_a"));
        assert!(msg.contains("inst_b"));
    }

    // ============================================================================
    // T1.4: EffectSpec + InstructionView tests
    // ============================================================================

    #[test]
    fn mock_instruction_view_basic() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };
        let bytes = &[0x00, 0x00, 0x00, 0x01];
        let decoded = decode_one::<MockBackend>(bytes, 0x1000, &profile).unwrap();

        let view = InstructionView::new(&decoded);
        assert_eq!(view.mnemonic(), "add");
        assert_eq!(view.size(), 4);
        assert_eq!(view.address(), 0x1000);
        assert_eq!(view.operand_count(), 3);
        assert!(view.is_in_group("integer"));
        assert!(view.is_in_group("arithmetic"));
        assert!(!view.is_in_group("branch"));
    }

    #[test]
    fn mock_instruction_view_operands() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };
        let bytes = &[0x00, 0x00, 0x00, 0x01];
        let decoded = decode_one::<MockBackend>(bytes, 0x1000, &profile).unwrap();

        let view = InstructionView::new(&decoded);
        let ops: Vec<_> = view.operands().collect();
        assert_eq!(ops.len(), 3);
        assert!(view.registers_written().len() == 1);
        assert!(view.registers_read().len() == 2);
    }

    // ============================================================================
    // T1.4: EffectSpec tests
    // ============================================================================

    #[test]
    fn effect_spec_variants_construct_and_compare() {
        assert_eq!(EffectSpec::Branch, EffectSpec::Branch);
        assert_eq!(EffectSpec::Call, EffectSpec::Call);
        assert_eq!(EffectSpec::Return, EffectSpec::Return);
        assert_eq!(EffectSpec::Barrier, EffectSpec::Barrier);
        assert_eq!(EffectSpec::Trap, EffectSpec::Trap);
        assert_eq!(EffectSpec::Privileged, EffectSpec::Privileged);
        assert_eq!(EffectSpec::Stack, EffectSpec::Stack);
        assert_eq!(EffectSpec::Memory, EffectSpec::Memory);
        assert_eq!(EffectSpec::None, EffectSpec::None);
        assert_ne!(EffectSpec::Branch, EffectSpec::Call);
        assert_ne!(EffectSpec::Return, EffectSpec::Branch);
        assert_ne!(EffectSpec::Memory, EffectSpec::Branch);
        assert_ne!(EffectSpec::None, EffectSpec::Trap);
    }

    #[test]
    fn instruction_view_effect_returns_spec_effect() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };
        let bytes = &[0x00, 0x00, 0x00, 0x01];
        let decoded = decode_one::<MockBackend>(bytes, 0x1000, &profile).unwrap();
        let view = InstructionView::new(&decoded);
        // ADD spec has effect: None (default from isa_specs! macro)
        assert_eq!(view.effect(), None);
    }

    // ============================================================================
    // T1.5: extract_field returns Result
    // ============================================================================

    #[test]
    fn mock_extract_field_returns_result_ok() {
        let profile = DecodeProfile {
            mode: mock::MockMode::Base,
            features: mock::MockFeature::BASE,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        };
        let bytes = &[0x00, 0x00, 0x00, 0x01];
        // decode_one internally calls extract_field which now returns Result
        let result = decode_one::<MockBackend>(bytes, 0x1000, &profile);
        assert!(result.is_ok());
    }

    #[test]
    fn mock_extract_field_returns_result_err_on_unknown_field() {
        // Directly call extract_field with a field not present in the format.
        // MOCK_FORMAT_R has fields Rd(0,5), Rs1(5,5), Rs2(10,5) — no Imm12.
        let word: u32 = 0;
        let result = MockBackend::extract_field(word, &mock::MOCK_FORMAT_R, mock::MockField::Imm12);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.stable_kind().contains("invalid_field"),
            "got {}",
            err.stable_kind()
        );
    }

    // ============================================================================
    // T1.6: EncodingToken tests for all encoding types
    // ============================================================================

    #[test]
    fn encoding_token_fixed_word_32_extract() {
        let token = FixedWord32(0x1234_5678);
        assert_eq!(token.size(), 4);
        // Extract bits [7:0]
        assert_eq!(token.extract(0, 8), 0x78);
        // Extract bits [15:8]
        assert_eq!(token.extract(8, 8), 0x56);
        // Extract bits [31:16]
        assert_eq!(token.extract(16, 16), 0x1234);
        // Edge cases
        assert_eq!(token.extract(0, 0), 0);
        assert_eq!(token.extract(32, 8), 0);
    }

    #[test]
    fn encoding_token_x86_extract() {
        let token = X86Encoding {
            size: 3,
            opcode: 0x8B,
            modrm: Some(0xC3),
        };
        assert_eq!(token.size(), 3);
        // Extract bits [7:0] from opcode
        assert_eq!(token.extract(0, 8), 0x8B);
        // Extract bits [3:0] (lower nibble)
        assert_eq!(token.extract(0, 4), 0x0B);
        // Edge cases
        assert_eq!(token.extract(0, 0), 0);
        assert_eq!(token.extract(8, 8), 0);
    }

    #[test]
    fn encoding_token_wasm_extract() {
        let token = WasmEncoding {
            size: 2,
            opcode: 0x20, // local.get
        };
        assert_eq!(token.size(), 2);
        // Extract bits [7:0] from opcode
        assert_eq!(token.extract(0, 8), 0x20);
        // Extract bits [5:0]
        assert_eq!(token.extract(0, 6), 0x20);
        // Edge cases
        assert_eq!(token.extract(0, 0), 0);
        assert_eq!(token.extract(8, 8), 0);
    }

    // ============================================================================
    // T1.2: ImmExpr::Compose evaluation tests
    // ============================================================================

    #[test]
    fn imm_expr_simple_evaluate() {
        // Word = 0x1234_5678, extract bits [7:0] = 0x78
        let expr = ImmExpr::Simple {
            field_start: 0,
            field_length: 8,
            transform: ImmediateTransform::None,
        };
        assert_eq!(expr.evaluate(0x1234_5678), 0x78);
    }

    #[test]
    fn imm_expr_compose_riscv_b_type() {
        // RISC-V B-type immediate composition:
        // imm[12|10:5|4:1|11] from bits [31],[30:25],[11:8],[7]
        // Test word: 0xFE30_0FE3 (beq x0, x0, -2)
        // imm12 = 1, imm10_5 = 0b111111, imm4_1 = 0b1110, imm11 = 1
        // Composed: 0b1_1_111111_1110 = 0xFFE = -2 (sign-extended 12-bit)
        static PARTS: [ImmComposePart; 4] = [
            ImmComposePart {
                src_start: 31,
                src_length: 1,
                dst_start: 12,
            },
            ImmComposePart {
                src_start: 25,
                src_length: 6,
                dst_start: 5,
            },
            ImmComposePart {
                src_start: 8,
                src_length: 4,
                dst_start: 1,
            },
            ImmComposePart {
                src_start: 7,
                src_length: 1,
                dst_start: 11,
            },
        ];
        let expr = ImmExpr::Compose {
            parts: &PARTS,
            transform: ImmediateTransform::SignExtend { bits: 12 },
        };
        let result = expr.evaluate(0xFE30_0FE3);
        assert_eq!(result, -2);
    }
}
