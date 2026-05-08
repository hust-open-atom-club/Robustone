# AArch64 Backend Design Document

**Date**: 2026-05-08
**Author**: Robustone Team
**Status**: Draft

## 1. Background & Goals

### 1.1 Context

The upstream `hust-open-atom-club/Robustone` repository has undergone a major refactor. The local fork previously had a `feature/aarch64-stage1` branch with 151 commits implementing a baseline AArch64 decoder. This branch has been archived as `feature/aarch64-stage1-backup`. The goal is to reimplement the AArch64 backend from scratch based on the latest upstream architecture, following patterns established by the RISC-V backend (hand-written decoder, extension-based organization).

### 1.2 Goals

- Implement a **complete AArch64 disassembly backend** for Robustone, covering:
  - Base integer instructions (Data Processing, Branches, Loads/Stores)
  - System / Exception instructions
  - Scalar Floating-Point
  - NEON / Advanced SIMD
  - SVE / SVE2 (if Capstone supports it)
- Achieve **parity with Capstone's `cstool`** output for all supported instructions.
- Follow existing Rust patterns: `ArchitectureHandler` trait, `DecodedInstruction` IR, two-tier API.
- Use **hand-written decoder** with extension-based module organization (reference: `robustone-riscv`).

### 1.3 Non-Goals

- AArch32 / ARMv7 support (out of scope; focus on AArch64 only).
- Instruction execution / emulation.
- Auto-generated decoder from YAML (reference: `robustone-loongarch` uses this, but we choose hand-written for flexibility).

---

## 2. Architecture Overview

### 2.1 High-Level Flow

```
Bytes (4-byte aligned)
  │
  ▼
┌─────────────────────┐
│ AArch64Decoder      │  ──►  Top-level decode, extracts op0[28:25]
│   (decoder.rs)      │       and dispatches to extension modules
└─────────────────────┘
  │
  ▼
┌─────────────────────┐
│ Extension Modules   │  ──►  Per-family decode functions return
│ (extensions/*.rs)   │       (Mnemonic, Vec<Operand>)
└─────────────────────┘
  │
  ▼
┌─────────────────────┐
│ Metadata & Render   │  ──►  compute_metadata() determines read/write
│   (render.rs)       │       registers, groups. render_aarch64() produces
│                     │       mnemonic + operand text.
└─────────────────────┘
  │
  ▼
DecodedInstruction ──► Text output via render_fn
```

### 2.2 Crate Structure

```
robustone-arm/src/
├── lib.rs              # Module exports, ArmHandler impl
├── arch.rs             # ArchitectureHandler trait glue
├── decoder.rs          # Top-level AArch64Decoder, op0 dispatch
├── types.rs            # Mnemonic enum, ConditionCode, ShiftType,
│                       # ExtendType, immediate decoders
├── render.rs           # Text rendering (mnemonic + operands)
├── printer.rs          # Print helper for test/debug
├── extensions/
│   ├── mod.rs          # Extension registry, decode dispatch table
│   ├── base.rs         # Data Processing — Immediate & Register
│   ├── branch.rs       # Branches, Exception Generating, Hints
│   ├── memory.rs       # Loads and Stores
│   ├── system.rs       # System instructions (MSR/MRS, barriers, etc.)
│   ├── float.rs        # Scalar Floating-Point
│   ├── simd.rs         # NEON / Advanced SIMD
│   └── sve.rs          # SVE / SVE2 (optional, stage 4)
└── shared/
    ├── mod.rs
    ├── encoding.rs     # Bit-field extraction helpers
    ├── operands.rs     # Operand construction (reg, imm, mem, etc.)
    ├── registers.rs    # AArch64 register definitions + aliases
    └── formatting.rs   # Text formatting helpers (hex, shifts, etc.)
```

---

## 3. Decode Strategy

### 3.1 AArch64 Encoding Hierarchy

AArch64 instructions are fixed 32-bit. The top-level classification uses `op0` (bits 28:25), with bit 31 providing secondary discrimination:

| op0  | bit31=0                              | bit31=1                              | Module            |
|------|--------------------------------------|--------------------------------------|-------------------|
| 0x0  | UNALLOCATED                          | Data Processing — Immediate          | `base.rs`         |
| 0x1  | Data Processing — Immediate          | Data Processing — Immediate          | `base.rs`         |
| 0x2  | Branches, Exception, System          | Branches, Exception, System          | `branch.rs`       |
| 0x3  | Loads and Stores                     | Loads and Stores                     | `memory.rs`       |
| 0x4  | Data Processing — Register           | Data Processing — Register           | `base.rs`         |
| 0x5  | Data Processing — SIMD/FP            | Data Processing — SIMD/FP            | `float.rs/simd.rs`|
| 0x6  | Data Processing — SIMD/FP            | Data Processing — SIMD/FP            | `float.rs/simd.rs`|
| 0x7  | Data Processing — SIMD/FP / SVE      | Data Processing — SIMD/FP / SVE      | `float.rs/simd.rs`/`sve.rs` |
| 0x8  | Data Processing — Register           | Data Processing — Register           | `base.rs`         |
| 0x9  | Data Processing — Register           | Data Processing — Register           | `base.rs`         |
| 0xA  | Branches, Exception, System          | Branches, Exception, System          | `branch.rs`       |
| 0xB  | Branches, Exception, System          | Branches, Exception, System          | `branch.rs`       |
| 0xC  | Loads and Stores (SIMD&FP)           | Loads and Stores (SIMD&FP)           | `memory.rs`       |
| 0xD  | Loads and Stores (SIMD&FP)           | Loads and Stores (SIMD&FP)           | `memory.rs`       |
| 0xE  | Data Processing — SIMD/FP            | Data Processing — SIMD/FP            | `float.rs/simd.rs`|
| 0xF  | Data Processing — SIMD/FP / SVE      | Data Processing — SIMD/FP / SVE / UNALLOCATED | `float.rs/simd.rs`/`sve.rs` |

> Note: Within each op0 class, sub-fields (`op1`, `op2`, `op3`, etc.) further discriminate the exact instruction. The decoder uses a hierarchical match tree: `op0` → `op1`/`op2` → field extraction → mnemonic + operands.

### 3.2 Decoder Function Signature

Each extension module exposes decode functions with a consistent signature:

```rust
pub fn decode(word: u32, addr: u64) -> Result<(Mnemonic, Vec<Operand>), DisasmError>
```

The top-level `decoder.rs` maps `op0` to the appropriate module function.

### 3.3 Immediate Decoding

AArch64 has several immediate encodings. A dedicated module in `types.rs` handles:

- **Unsigned 12-bit immediate** (`imm12`) — used in `add/sub` (with optional shift by 12)
- **16-bit shifted immediate** (`hw:imm16`) — used in `movz/movn/movk`
- **Bitmask immediate** (`N:immr:imms`) — used in logical instructions (`and`, `orr`, `eor`)
- **PC-relative immediate** (19-bit for `b.cond`, 26-bit for `b`/`bl`, 21-bit for `adr`/`adrp`)
- **Signed 9-bit offset** — used in load/store with pre/post-index
- **Scaled 12-bit offset** — used in load/store (unsigned offset)
- **Shift/Extend immediate** — used in register-shifted operations

---

## 4. Register Model

### 4.1 General-Purpose Registers

| Register | 64-bit | 32-bit | Notes |
|----------|--------|--------|-------|
| R0–R30   | X0–X30 | W0–W30 | Integer GPRs |
| R31      | XZR/SP | WZR/WSP | Context-dependent: XZR for most data processing, SP for loads/stores and `add/sub` with SP |

The decoder must determine whether R31 is ZR or SP based on the instruction family and specific encoding fields. This is handled in `shared/registers.rs`.

### 4.2 SIMD & FP Registers

| Register | 128-bit | 64-bit | 32-bit | 16-bit | 8-bit |
|----------|---------|--------|--------|--------|-------|
| V0–V31   | Q0–Q31  | D0–D31 | S0–S31 | H0–H31 | B0–B31 |

### 4.3 SVE Predicate Registers

| Register | Description |
|----------|-------------|
| P0–P15   | Scalable predicate registers (SVE) |

### 4.4 Special Registers

- **SP / WSP** — Stack pointer (only in 64-bit / 32-bit forms)
- **XZR / WZR** — Zero register
- **PC** — Program counter (not directly accessible as operand in most instructions)
- **PSTATE** — Processor state (N, Z, C, V flags)

---

## 5. Instruction Families (Detailed)

### 5.1 Data Processing — Immediate (`op0 = 0x1`)

Subdivided by `op1` (bits 24:23):

| op1 | Family                     | Examples            |
|-----|----------------------------|---------------------|
| 00  | PC-rel. addressing         | `adr`, `adrp`       |
| 01  | Add/subtract (immediate)   | `add`, `sub`, `cmp` |
| 10  | Logical (immediate)        | `and`, `orr`, `eor` |
| 11  | Move (wide immediate)      | `movz`, `movn`, `movk` |

### 5.2 Data Processing — Register (`op0 = 0x4, 0x8, 0x9`)

Subdivided by `op1` (bits 24:23) and `op2` (bits 21:16):

| op1 | op2 range   | Family                           | Examples                |
|-----|-------------|----------------------------------|-------------------------|
| 00  | 0bxxxx00    | Logical (shifted register)       | `and`, `orr`, `eor`     |
| 00  | 0bxxxx01    | Bit-field operations             | `bfm`, `sbfm`, `ubfm`   |
| 00  | 0bxxxx10    | Extract                          | `extr`                  |
| 01  | —           | Add/subtract (shifted register)  | `add`, `sub`            |
| 10  | —           | Add/subtract (extended register) | `add`, `sub`            |
| 10  | —           | Add/subtract (with carry)        | `adc`, `sbc`            |
| 11  | —           | Conditional select               | `csel`, `csinc`, `csneg`|
| 11  | —           | Conditional compare              | `ccmp`, `ccmn`          |
| 11  | —           | Data-processing (3-source)       | `madd`, `msub`, `smulh` |

### 5.3 Branches, Exception Generating, System (`op0 = 0x2, 0xA, 0xB`)

| Category               | Examples                         |
|------------------------|----------------------------------|
| Unconditional branch   | `b`, `bl`                        |
| Compare & branch       | `cbz`, `cbnz`                    |
| Test & branch          | `tbz`, `tbnz`                    |
| Conditional branch     | `b.eq`, `b.ne`, ...              |
| Exception generating   | `svc`, `hvc`, `smc`, `brk`       |
| Hints                  | `nop`, `yield`, `wfe`, `wfi`     |
| Barriers               | `isb`, `dsb`, `dmb`              |
| System register move   | `msr`, `mrs`                     |

### 5.4 Loads and Stores (`op0 = 0x3, 0xC, 0xD`)

| Category               | Examples                         |
|------------------------|----------------------------------|
| Load/store (unsigned)  | `ldr`, `str`, `ldrb`, `strb`     |
| Load/store (pre-index) | `ldr`, `str` with `!`            |
| Load/store (post-index)| `ldr`, `str` with implicit writeback |
| Load/store pair        | `ldp`, `stp`                     |
| Load literal           | `ldr` (PC-relative)              |
| Load/store (SIMD&FP)   | `ldr q0`, `str d1`, etc.         |
| Atomic memory ops      | `ldxr`, `stxr`, `ldadd`, `cas`   |

---

## 6. Condition Codes

AArch64 uses 4-bit condition codes for `b.cond` and conditional data processing:

| Code | Mnemonic | Meaning                          |
|------|----------|----------------------------------|
| 0000 | EQ       | Equal (Z == 1)                   |
| 0001 | NE       | Not equal (Z == 0)               |
| 0010 | CS/HS    | Carry set / Unsigned higher or same |
| 0011 | CC/LO    | Carry clear / Unsigned lower     |
| 0100 | MI       | Minus / Negative (N == 1)        |
| 0101 | PL       | Plus / Positive or zero (N == 0) |
| 0110 | VS       | Overflow (V == 1)                |
| 0111 | VC       | No overflow (V == 0)             |
| 1000 | HI       | Unsigned higher                  |
| 1001 | LS       | Unsigned lower or same           |
| 1010 | GE       | Signed greater or equal          |
| 1011 | LT       | Signed less than                 |
| 1100 | GT       | Signed greater than              |
| 1101 | LE       | Signed less than or equal        |
| 1110 | AL       | Always (unconditional)           |
| 1111 | NV       | Always (unconditional, deprecated) |

---

## 7. Testing Strategy

Following the project's **ISA Triple Gate** (from CONTRIBUTING.md):

1. **Parity Coverage**: Add test cases to `test/architectures/aarch64/test_cases.txt`
2. **Rust Coverage**: Add golden tests in `tests/golden/`, property tests in `tests/property/`
3. **Known Differences**: Document intentional divergences in `tests/differential/known-differences.toml`

### 7.1 Test Organization

```
test/architectures/aarch64/
├── config.json           # Capstone invocation config
└── test_cases.txt        # Hex bytes + expected output

tests/golden/aarch64/
└── *.json                # Per-instruction-family golden fixtures
```

### 7.2 Coverage Targets

| Stage | Instruction Families | Min Parity Cases | Min Rust Tests |
|-------|----------------------|------------------|----------------|
| 1     | Base integer         | 50               | 20             |
| 2     | Memory + Atomics     | 50               | 20             |
| 3     | FP + SIMD            | 100              | 30             |
| 4     | SVE                  | TBD              | TBD            |

---

## 8. Compatibility & Capstone Parity

### 8.1 Text Rendering Conventions

Output must match Capstone's `cstool` format:

- **Register naming**: Lowercase (`x0`, `w1`, `sp`, `xzr`, `v0`, `d1`)
- **Immediate formatting**: Hex with `0x` prefix (`#0x10`), decimal otherwise
- **Shift amounts**: `#<amount>` (e.g., `lsl #12`)
- **Memory operands**: `[x0, #0x8]`, `[x0, x1, lsl #3]!`
- **Condition suffixes**: `.eq`, `.ne`, etc. attached to `b` and conditional selects
- **Zero register**: Rendered as `xzr` / `wzr`, never as `x31` / `w31`
- **SP**: Rendered as `sp` / `wsp` when appropriate

### 8.2 Known Differences (to be documented)

- Capstone may use `mov` as an alias for `orr` (with XZR) — we follow the ARM ARM canonical form.
- Capstone may render `cmp` as `subs` (with XZR destination) — we use `cmp` as the canonical mnemonic.
- SVE support depends on Capstone version (v6.0.0+).

---

## 9. Phase Plan Summary

| Phase | Scope                              | Key Deliverables                                      | Est. Size |
|-------|------------------------------------|------------------------------------------------------|-----------|
| 1     | Base integer (Data Proc + Branch) | `extensions/base.rs`, `extensions/branch.rs`, tests  | ~3K LOC   |
| 2     | Memory + System                    | `extensions/memory.rs`, `extensions/system.rs`, tests| ~2.5K LOC |
| 3     | FP + SIMD                          | `extensions/float.rs`, `extensions/simd.rs`, tests   | ~4K LOC   |
| 4     | SVE / SVE2                         | `extensions/sve.rs`, tests                           | ~2K LOC   |

**Total estimated size**: ~11.5K lines (comparable to `robustone-riscv` at ~7.5K + growth for SIMD/SVE).

---

## 10. Design Decisions (Resolved)

1. **Capstone-style alias expansion**: **Implement**. The decoder will emit canonical ARM ARM mnemonics internally, but the renderer will perform alias expansion to match Capstone output:
   - `orr Rd, XZR, Rm` → `mov Rd, Rm`
   - `subs Rn, Rm, #0` → `cmp Rn, Rm`
   - `sub Rd, XZR, Rm` → `neg Rd, Rm`
   - (And other standard aliases per ARM ARM section C1.2.4)

2. **SVE predicate register rendering**: Confirmed — Capstone uses `p0.b`, `p1.h`, `p2.s`, `p3.d` format with element size suffix.

3. **ArchitectureProfile extension gating**: **Support**. AArch64 `ArchitectureProfile` will accept extension flags similar to RISC-V:
   - `"base"` (default): Base integer only
   - `"+fp"`: Scalar floating-point
   - `"+simd"` or `"+neon"`: Advanced SIMD
   - `"+sve"`: Scalable Vector Extension
   - `"+sve2"`: SVE2
   - Instructions requiring an un-enabled extension will return `DisasmError::UnsupportedExtension`.
