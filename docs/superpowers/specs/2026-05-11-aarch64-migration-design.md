# AArch64 Backend Migration to robustone-isa Unified Pipeline

## Context

Our hand-written AArch64 decoder (~3000 lines across base, branch, system, loads/stores, scalar FP, and vector SIMD) was built against the pre-unification architecture. Upstream has since moved to a declarative `robustone-isa` spec-driven backend model. The upstream ARM stub currently contains only 4 instructions (NOP, RET, ADD_IMM, MOVZ).

This design documents the migration of our full AArch64 decoder to the new upstream architecture.

---

## Goals

1. Migrate all decoded instruction categories to the `robustone-isa` spec format
2. Maintain decoding parity with our current implementation
3. Preserve Capstone-compatible text rendering
4. Keep decode performance reasonable via hierarchical lookup

---

## Non-Goals

1. Adding new instruction categories beyond what we already decode
2. Changing the Capstone text output format
3. Modifying upstream `robustone-isa` core types (we work within the existing framework)

---

## Architecture

### Directory Structure

```
robustone-arm/src/
├── lib.rs                  # ArmHandler, public API
├── backend/
│   ├── mod.rs              # define_arch!, hierarchical lookup, extract_field
│   ├── specs_base.rs       # Base integer data-processing (~50 specs)
│   ├── specs_branch.rs     # Branches and exceptions (~30 specs)
│   ├── specs_system.rs     # System instructions (~20 specs)
│   ├── specs_loadstore.rs  # Loads and stores (~80 specs)
│   ├── specs_scalar_fp.rs  # Scalar FP (~60 specs)
│   └── specs_vector.rs     # Advanced SIMD vector (~150 specs)
├── render.rs               # AArch64Renderer, handles arrangement suffixes
└── aliases.rs              # define_aliases! macro calls
```

### Backend Definition

```rust
robustone_isa_macros::define_arch! {
    pub arch Arm {
        word = u32;
        endian = little;
        instruction_length = fixed(4);
        modes { AArch64 = "aarch64"; };
        features: u8 { BASE = 0; FP = 1; SIMD = 2; CRYPTO = 3; };
        registers = arm_registers;
        formats = arm_formats;
        specs = arm_specs;
        render = ArmRenderPolicy;
        backend_impl {
            field = ArmField;
            register_class = ArmRegisterClass;
            architecture_id = ArchitectureId::Arm;
            read_instruction = arm_read_instruction;
            lookup = arm_lookup;
            lower_register = arm_lower_register;
            extract_field = arm_extract_field;
        }
    }
}
```

### Hierarchical Lookup

AArch64 instructions are classified by `op0 = bits(28:25)`. The `arm_lookup()` function dispatches to category-specific spec sub-tables:

```rust
fn arm_lookup(word: u32, profile: &DecodeProfile<ArmBackend>) -> Option<&'static InstructionSpec<ArmBackend>> {
    let op0 = bits(word, 28, 25) as u8;
    let table: &[&[InstructionSpec<ArmBackend>]] = match op0 {
        0x0..=0x3   => &[BASE_SPECS, BRANCH_SPECS],
        0x4 | 0x6   => &[LOAD_STORE_SPECS],
        0x5 | 0xD   => &[DATA_IMM_SPECS],
        0x7 | 0xF   => &[SCALAR_FP_SPECS, VECTOR_SPECS],
        _           => return None,
    };
    table.iter()
        .flat_map(|s| s.iter())
        .find(|spec|
            (word & spec.pattern().mask) == spec.pattern().value
            && profile.features.contains(spec.features())
        )
}
```

This preserves AArch64's natural encoding hierarchy while staying compatible with the upstream `lookup` trait signature.

---

## Field and Format Definitions

### ArmField Enum

All bit-field identifiers used by AArch64 formats:

```rust
pub enum ArmField {
    // General-purpose registers
    Rd, Rn, Rm, Ra, Rt, Rt2, Rs,
    // Immediates
    Imm12, Imm16, Imm19, Imm26,
    Immhi, Immlo,           // ADR/ADRP split immediate
    Cond,                   // Condition codes (B.cond)
    Shift, Imm6, Hw,        // Shift amounts and halfword positions
    N, Immr, Imms,          // Bitmask immediate components
    // FP / SIMD fields
    Ftype, Opcode, Size, Q, U, L, M, H, VmIdx,
}
```

### Format Examples

```rust
format R_TYPE {
    rd:  bits(0,  5) as Rd,
    rn:  bits(5,  5) as Rn,
    rm:  bits(16, 5) as Rm,
};

format I_ADD {
    rd:     bits(0,  5) as Rd,
    rn:     bits(5,  5) as Rn,
    imm12:  bits(10, 12) as Imm12,
    shift:  bits(22, 1)  as Shift,
};

format B_COND {
    imm19: bits(5, 19) as Imm19,
    cond:  bits(0, 4)  as Cond,
};

format VEC_THREE_SAME {
    rd:    bits(0,  5) as Rd,
    rn:    bits(5,  5) as Rn,
    rm:    bits(16, 5) as Rm,
    size:  bits(22, 2) as Size,
    q:     bits(30, 1) as Q,
};
```

---

## Vector Register Rendering

Vector registers use standard `RegisterId` operands in the spec system. The `AArch64Renderer` adds arrangement suffixes based on the instruction's `Size` and `Q` field values:

| Size | Q=0 | Q=1 |
|------|-----|-----|
| 00 (B) | `.8b` | `.16b` |
| 01 (H) | `.4h` | `.8h` |
| 10 (S) | `.2s` | `.4s` |
| 11 (D) | `.1d` | `.2d` |

The renderer reads `render_hints` populated during operand lowering to determine the correct suffix. Scalar FP registers (`s0`, `d0`, `h0`) are handled similarly based on `Ftype`.

---

## Complex Immediate Handling

For immediates that cannot be expressed with simple `imm!` or `imm_compose!`, `arm_extract_field` returns the raw bit pattern and the renderer performs the final conversion:

| Field | Encoding | Render Output |
|-------|----------|---------------|
| `Imm12` + `Shift` | 12-bit unsigned + 1-bit `shift` | `#imm` or `#imm, lsl #12` |
| `N` + `Immr` + `Imms` | Bitmask immediate | `#0x...` (decoded by renderer) |
| `Imm8` (FP) | 8-bit FP modified immediate | `#1.0`, `#2.0`, etc. |
| `Imm19`/`Imm26` | PC-relative branch offset | `#0x...` (address calculated by renderer) |

The renderer has access to the full `DecodedInstruction` and can use `opcode_id` to select the appropriate immediate decoding logic.

---

## Alias Definitions

Aliases are defined via the upstream `define_aliases!` macro:

```rust
robustone_isa_macros::define_aliases! {
    arch = Arm;

    alias MOV_ORR {
        base = "ORR_REG";
        condition = operand_eq(2, 31);  // Rm == XZR/WZR
        mnemonic = "mov";
        hidden_operands = [2];
    }

    alias NEG_SUB {
        base = "SUB_REG";
        condition = operand_eq(1, 31);  // Rn == XZR/WZR
        mnemonic = "neg";
        hidden_operands = [1];
    }

    alias CSET_CSEL {
        base = "CSEL";
        condition = operand_eq(1, 31) && operand_eq(2, 31);  // both XZR
        mnemonic = "cset";
        hidden_operands = [1, 2];
    }
}
```

---

## Migration Phases

| Phase | Category | Instructions | Est. Effort |
|-------|----------|--------------|-------------|
| 1 | Base integer (ADD, SUB, AND, ORR, MOVZ, CSEL, etc.) | ~50 | 2-3 days |
| 2 | Branch (B, BL, BR, RET, CBZ, B.cond, TBZ/TBNZ) | ~30 | 2 days |
| 3 | System (NOP, SVC, HVC, barriers, MSR/MRS stub) | ~20 | 1-2 days |
| 4 | Loads/Stores (LDR, STR, LDP, STP, LDXR, STXR, etc.) | ~80 | 3-4 days |
| 5 | Scalar FP (FADD, FSUB, FMUL, FCVT, FMOV, etc.) | ~60 | 3 days |
| 6 | Vector SIMD (VADD, VAND, VSHR, DUP, TBL, crypto, etc.) | ~150 | 5-7 days |
| 7 | Alias system + Renderer + Audit | - | 2-3 days |

**Total: approximately 18-24 days**

Each phase includes:
- Writing spec definitions
- Adding render logic for new operand types
- Porting existing unit tests
- Running parity tests against Capstone

---

## Testing Strategy

1. **Unit tests**: Each spec file has `#[cfg(test)]` tests verifying known instruction encodings decode to the correct mnemonic and operands.
2. **Parity tests**: `test/architectures/aarch64/test_cases.txt` cases must continue to pass.
3. **Golden fixtures**: Update JSON golden files if render output changes.
4. **Differential tests**: Document any intentional divergences in `tests/differential/known-differences.toml`.

---

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Mask/value pattern overlap causing wrong spec match | Validate every pattern against ARM ARM encoding tables; use Capstone YAML tests |
| Performance regression from spec-based decode | Hierarchical lookup limits search space; benchmark against old decoder |
| Vector register suffix rendering incorrect | Test all Size x Q combinations for each vector instruction class |
| Complex immediate decoding diverges from Capstone | Maintain a mapping table of known imm8/imm16 values to Capstone text output |
| Scope creep (trying to decode new instructions) | Strictly limit to instructions already decoded by old implementation |
