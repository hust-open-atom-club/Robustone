# Robustone Public Contract

> **Status:** Draft for `0.1.0-alpha`
>
> This document defines the stability guarantees that external tools can rely on when integrating with Robustone. Until `1.0.0`, all contracts are subject to change with a minor version bump, but breaking changes will be documented in the changelog.

## 1. JSON Output Schema (Stable Fields)

When `--json` is used, Robustone emits a single JSON object with the following **stable top-level fields**:

| Field | Type | Stability | Description |
|---|---|---|---|
| `architecture` | `string` | **Stable** | Canonical architecture name (e.g., `"riscv32"`, `"riscv64"`). |
| `start_address` | `number` (u64) | **Stable** | Address of the first byte in the input. |
| `bytes_processed` | `number` (usize) | **Stable** | Total bytes consumed (instructions + skipped data). |
| `instructions` | `array` | **Stable** | Ordered list of decoded instructions and data pseudo-instructions. |
| `errors` | `array` | **Stable** | Non-fatal issues encountered during disassembly (empty when `skip_data` is enabled, because skipped bytes become pseudo-instructions instead of errors). |

### 1.1 Instruction Object

Each element in `instructions` is guaranteed to contain:

| Field | Type | Stability | Description |
|---|---|---|---|
| `address` | `number` (u64) | **Stable** | Memory address of this instruction. |
| `mnemonic` | `string` | **Stable** | Display mnemonic. For data pseudo-instructions this is always `".byte"`. |
| `operands` | `string` | **Stable** | Operand text. Empty string if the instruction has no operands. |
| `size` | `number` (usize) | **Stable** | Number of bytes consumed by this item. |
| `bytes` | `array` of `number` (u8) | **Stable** | Raw bytes of this item. |
| `kind` | `string` | **Stable** | `"instruction"` for normal instructions, `"data"` for SKIPDATA pseudo-instructions. |
| `decoded` | `object` or omitted | **Stable** | Structured IR when available (see §1.2). May be omitted for data pseudo-instructions. |

### 1.2 Decoded IR Object (`decoded`)

When present, the following fields are stable:

| Field | Type | Stability | Description |
|---|---|---|---|
| `architecture` | `string` | **Stable** | Backend identifier (e.g., `"riscv"`). |
| `mode` | `string` | **Stable** | Architecture mode token (e.g., `"riscv32"`, `"riscv64"`). |
| `mnemonic` | `string` | **Stable** | Canonical backend mnemonic (may differ from Capstone-facing alias). |
| `opcode_id` | `string` or `null` | **Stable** | Normalized opcode identifier. |
| `size` | `number` (usize) | **Stable** | Instruction size in bytes. |
| `raw_bytes` | `array` of `number` (u8) | **Stable** | Raw bytes of the instruction. |
| `operands` | `array` | **Stable** | Typed operand list (see §1.3). |
| `groups` | `array` of `string` | **Stable** | Semantic groups (e.g., `["arithmetic"]`, `["compressed"]`, `["atomic"]`). |
| `status` | `string` | **Stable** | `"success"` or decode-failure classification. |
| `registers_read` | `array` of `{architecture, id}` | **Stable** | Explicitly read registers. |
| `registers_written` | `array` of `{architecture, id}` | **Stable** | Explicitly written registers. |
| `implicit_registers_read` | `array` of `{architecture, id}` | **Stable** | Implicitly read registers (e.g., `sp` for `c.addi16sp`). |
| `implicit_registers_written` | `array` of `{architecture, id}` | **Stable** | Implicitly written registers. |
| `render_hints` | `object` | **Stable** | Display-oriented hints (`capstone_mnemonic`, `capstone_hidden_operands`). |

### 1.3 Operand Objects

Each operand is a tagged object with `kind`:

- `{ "kind": "register", "register": { "architecture": "riscv", "id": 1 } }`
- `{ "kind": "immediate", "value": -16 }`
- `{ "kind": "text", "value": "rne" }`
- `{ "kind": "memory", "base": { "architecture": "riscv", "id": 2 }, "displacement": 4 }`

The `kind` tag and the field names above are **stable**.

## 2. Error Taxonomy (Stable Identifiers)

All structured errors expose a machine-readable `kind` string. The following identifiers are guaranteed stable:

| Stable `kind` | Meaning |
|---|---|
| `need_more_bytes` | Input was truncated mid-instruction. |
| `invalid_encoding` | Bytes do not form a valid instruction for the selected architecture. |
| `unsupported_extension` | Instruction requires an extension not enabled in the current profile (e.g., compressed instruction without `C`). |
| `unimplemented_instruction` | Valid encoding that the backend does not yet handle. |
| `unsupported_mode` | Instruction valid for a different mode of the same architecture (e.g., RV64-only instruction on RV32). |
| `unsupported_architecture` | No backend registered for the requested architecture token. |
| `decoding_error` | Catch-all for architecture-agnostic decode failures. |
| `invalid_hex_code` | CLI/input layer could not parse the provided hex string. |
| `invalid_address` | CLI/input layer could not parse the provided address. |

## 3. Architecture Profile Modifier Semantics

RISC-V architecture strings accept `+`-separated modifiers. The following guarantees apply:

### 3.1 Baseline Behavior

- `riscv32` and `riscv64` without any extension modifier map to the **default GC profile** (`I`, `M`, `A`, `F`, `D`, `C`). This is the backward-compatible baseline and matches Capstone's default RISC-V behavior.
- `riscv32e` maps to the RV32E base profile (`I` only, with E semantics).

### 3.2 Explicit Extension Modifiers

When any of the following modifiers appear, the profile is built **incrementally from the base integer set** rather than from GC:

| Modifier | Effect | Implied Dependency |
|---|---|---|
| `+m` | Include `M` extension | None |
| `+a` | Include `A` extension | None |
| `+c` | Include `C` extension | None |
| `+f` | Include `F` extension | None |
| `+d` | Include `D` extension | Implies `+f` |
| `+fd` | Include `F` and `D` extensions | None (self-contained) |

Base integer (`I`) and `M` are always included when explicit modifiers are used, because Capstone's RISC-V baseline assumes a multiplier-capable target.

### 3.3 Display Modifiers

The following modifiers affect rendering only and do not change the extension set:

| Modifier | Effect |
|---|---|
| `+noalias` | Disable Capstone-style register aliases (e.g., print `x1` instead of `ra`). |
| `+noaliascompressed` | Disable compressed-instruction aliases only. |

## 4. CLI Flags and Their Decode-Time Effects

| Flag | Decode-Time Effect | Render-Time Effect |
|---|---|---|
| `-d` / `--detailed` | Enables detail generation (`set_detail(true)`). | Shows raw hex bytes. |
| `-r` / `--real-detail` | Enables detail generation (`set_detail(true)`). | Shows raw hex bytes + detail sections. |
| `-s` / `--skip-data` | Enables SKIPDATA mode (emits `.byte` pseudo-instructions on decode failure). | No additional render effect. |
| `--json` | None. | Emits JSON instead of text. |

Detail generation defaults to **off** unless `-d` or `-r` is provided.

## 5. Versioning Policy

Robustone follows a pre-1.0 semantic versioning policy:

- **Patch bumps** (`0.1.x`): Bug fixes, documentation improvements, new test cases. No intentional breaking changes to stable fields.
- **Minor bumps** (`0.x.0`): New features, new stable fields, API additions. May include breaking changes to unstable surfaces with clear migration notes.
- **1.0.0** (future): All fields marked **Stable** in this document become frozen. Breaking changes will only happen in major version bumps.

## 6. Integration Checklist

If you are building a tool on top of Robustone JSON output, you should:

1. Rely on `kind` (`"instruction"` / `"data"`) rather than `mnemonic` to distinguish data pseudo-instructions.
2. Handle `UnsupportedArchitecture` gracefully — the CLI accepts parser-only tokens for discovery, but decode will fail.
3. Use `stable_kind()` rather than parsing error display strings when classifying failures.
4. Pin to a specific minor version in CI until `1.0.0` is reached.
