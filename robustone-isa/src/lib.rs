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
    pub priority: u16,
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

impl InstructionGroup {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstructionGroup::Integer => "integer",
            InstructionGroup::Arithmetic => "arithmetic",
            InstructionGroup::Logical => "logical",
            InstructionGroup::Shift => "shift",
            InstructionGroup::Branch => "branch",
            InstructionGroup::Jump => "jump",
            InstructionGroup::Memory => "memory",
            InstructionGroup::Atomic => "atomic",
            InstructionGroup::Float => "floating_point",
            InstructionGroup::Privileged => "privileged",
            InstructionGroup::Barrier => "barrier",
            InstructionGroup::System => "system",
            InstructionGroup::Vector => "vector",
            InstructionGroup::BitManipulation => "bit_manipulation",
        }
    }
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
    Text {
        field: B::Field,
        transform: ImmediateTransform,
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
        groups: spec.groups.iter().map(|g| g.as_str().to_string()).collect(),
        status: robustone_core::ir::DecodeStatus::Success,
        render_hints: robustone_core::ir::RenderHints::default(),
    };

    Ok(decoded)
}

/// Lower a single operand from the raw word.
fn lower_operand<B: ArchitectureBackend>(
    word: B::Word,
    spec: &OperandSpec<B>,
    profile: &DecodeProfile<B>,
    format: &FormatSpec<B::Field>,
) -> Operand {
    match spec {
        OperandSpec::Register {
            class,
            field,
            access: _,
        } => {
            let raw = B::extract_field(word, format, *field);
            let reg = B::lower_register(*class, raw, profile);
            Operand::Register { register: reg }
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
        OperandSpec::Text { field, transform } => {
            let raw = B::extract_field(word, format, *field);
            let value = apply_transform(raw, *transform);
            Operand::Text {
                value: value.to_string(),
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
        $(pub static $name: $crate::FormatSpec<$field_ty> = $crate::FormatSpec {
            name: stringify!($name),
            fields: &[$($field_expr),*],
        };)*
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
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec {
            mnemonic: $mnemonic,
            opcode_id: $opcode_id,
            pattern: $pattern,
            format: $format,
            operands: $operands,
            features: $features,
            modes: $modes,
            groups: $groups,
            manual_ref: Some($manual),
            priority: 0,
        };)*
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
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec {
            mnemonic: $mnemonic,
            opcode_id: $opcode_id,
            pattern: $pattern,
            format: $format,
            operands: $operands,
            features: $features,
            modes: $modes,
            groups: $groups,
            manual_ref: None,
            priority: 0,
        };)*
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
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec {
            mnemonic: $mnemonic,
            opcode_id: $opcode_id,
            pattern: $pattern,
            format: $format,
            operands: $operands,
            features: $features,
            modes: $modes,
            groups: $groups,
            manual_ref: Some($manual),
            priority: $priority,
        };)*
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
        $(pub static $name: $crate::InstructionSpec<$backend> = $crate::InstructionSpec {
            mnemonic: $mnemonic,
            opcode_id: $opcode_id,
            pattern: $pattern,
            format: $format,
            operands: $operands,
            features: $features,
            modes: $modes,
            groups: $groups,
            manual_ref: None,
            priority: $priority,
        };)*
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
}
