#!/usr/bin/env python3
"""Compare Robustone disassembly output with Capstone's cstool for RV32 instructions."""
from __future__ import annotations

import re
import shlex
import string
import subprocess
import sys
from dataclasses import dataclass
from typing import Optional
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROBUSTONE_MANIFEST = ROOT / "robustone" / "Cargo.toml"
ROBUSTONE_BIN = ROOT / "robustone" / "target" / "debug" / "robustone"
INSTRUCTION_FILE = Path(__file__).with_name("instructions.txt")


@dataclass
class CompareConfig:
    cstool_bin: Path


COMPARE_CONFIG: Optional[CompareConfig] = None

HEX_CHARS = set(string.hexdigits)
BYTE_TOKEN_RE = re.compile(r"^[0-9a-f]{1,2}$")


def fail(msg: str) -> None:
    print(msg, file=sys.stderr)
    sys.exit(1)


def ensure_binaries(cstool_bin: Path) -> None:
    if not cstool_bin.exists():
        fail(f"cstool binary not found at {cstool_bin}. Did you run build_cstool.sh?")
    if not ROBUSTONE_BIN.exists():
        print("Building robustone CLI binary...", file=sys.stderr)
        subprocess.run(
            [
                "cargo",
                "build",
                "--manifest-path",
                str(ROBUSTONE_MANIFEST),
                "--bin",
                "robustone",
            ],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )


def hex_to_le_bytes(hex_word: str) -> str:
    raw = hex_word.lower().strip()
    if raw.startswith("0x"):
        raw = raw[2:]
    if not raw:
        fail(f"Empty hex word in test cases: {hex_word!r}")
    if len(raw) % 2:
        raw = "0" + raw
    if any(ch not in HEX_CHARS for ch in raw):
        fail(f"Invalid hex digits in test case: {hex_word}")
    byte_pairs = [raw[i : i + 2] for i in range(0, len(raw), 2)]
    byte_pairs.reverse()
    return " ".join(byte_pairs)


def run_command(cmd: list[str]) -> str:
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if proc.returncode != 0:
        fail(
            "Command failed ({}):\n{}".format(
                " ".join(shlex.quote(arg) for arg in cmd), proc.stderr.strip()
            )
        )
    return proc.stdout.strip()


def parse_int_token(token: str) -> int:
    token = token.strip().lower()
    if not token:
        raise ValueError("empty integer token")
    sign = -1 if token.startswith("-") else 1
    if token[0] in "+-":
        token = token[1:]
    if token.startswith("0x"):
        value = int(token, 16)
    else:
        value = int(token, 10)
    return sign * value


def canonical_token(token: str) -> str:
    token = token.strip().lower()
    if not token:
        return ""
    mem_match = re.fullmatch(r"([+-]?(?:0x)?[0-9a-f]+)\(([^)]+)\)", token)
    if mem_match:
        imm_token, base = mem_match.groups()
        imm_val = parse_int_token(imm_token)
        base_canon = canonical_token(base)
        if not base_canon.startswith("sym:"):
            base_canon = f"sym:{base_canon}"
        return f"mem:{imm_val}:{base_canon}"
    try:
        value = parse_int_token(token)
    except ValueError:
        return f"sym:{token}"
    return f"imm:{value}"


def normalize_operands(operands: str) -> list[str]:
    if not operands:
        return []
    cleaned = re.sub(r"\s+", " ", operands.strip().lower())
    raw_tokens = [tok for tok in re.split(r"[\s,]+", cleaned) if tok]
    return [canonical_token(tok) for tok in raw_tokens]


def parse_disasm_line(output: str, source: str) -> tuple[str, list[str]]:
    for line in output.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        parts = stripped.split()
        idx = 0
        while idx < len(parts) and BYTE_TOKEN_RE.fullmatch(parts[idx].lower().strip(",:")):
            idx += 1
        if idx >= len(parts):
            continue
        mnemonic = parts[idx].lower()
        operands = " ".join(parts[idx + 1 :])
        return mnemonic, normalize_operands(operands)
    fail(f"{source} produced no parseable output:\n{output}")


def parse_cstool(output: str) -> tuple[str, list[str]]:
    return parse_disasm_line(output, "cstool")


def parse_robustone(output: str) -> tuple[str, list[str]]:
    return parse_disasm_line(output, "robustone")


@dataclass
class ComparisonResult:
    hex_word: str
    cstool_mnemonic: str
    cstool_operands: list[str]
    robustone_mnemonic: str
    robustone_operands: list[str]

    def matches(self) -> bool:
        return (
            self.cstool_mnemonic == self.robustone_mnemonic
            and self.cstool_operands == self.robustone_operands
        )


def compare_instruction(hex_word: str) -> ComparisonResult:
    little_endian_bytes = hex_to_le_bytes(hex_word)
    if COMPARE_CONFIG is None:
        raise RuntimeError("Comparison configuration has not been initialised")
    cstool_out = run_command([str(COMPARE_CONFIG.cstool_bin), "riscv32", little_endian_bytes])
    robustone_out = run_command([
        str(ROBUSTONE_BIN),
        "riscv32",
        little_endian_bytes,
    ])
    c_mnem, c_ops = parse_cstool(cstool_out)
    r_mnem, r_ops = parse_robustone(robustone_out)
    return ComparisonResult(hex_word, c_mnem, c_ops, r_mnem, r_ops)


def load_instructions() -> list[str]:
    if not INSTRUCTION_FILE.exists():
        fail(f"Instruction file not found: {INSTRUCTION_FILE}")
    lines = []
    for raw in INSTRUCTION_FILE.read_text().splitlines():
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        lines.append(stripped)
    if not lines:
        fail("Instruction list is empty")
    return lines


def main() -> int:
    capstone_arg = sys.argv[1] if len(sys.argv) > 1 else str(ROOT / "third_party" / "capstone")
    capstone_dir = Path(capstone_arg).resolve()
    cstool_bin = capstone_dir / "cstool" / "cstool"
    ensure_binaries(cstool_bin)

    global COMPARE_CONFIG
    COMPARE_CONFIG = CompareConfig(cstool_bin=cstool_bin)
    mismatches: list[ComparisonResult] = []
    for hex_word in load_instructions():
        result = compare_instruction(hex_word)
        if not result.matches():
            mismatches.append(result)
    if mismatches:
        print("RISC-V disassembly comparison mismatches detected:")
        for res in mismatches:
            print(f"  {res.hex_word}:")
            print(
                f"    cstool   -> {res.cstool_mnemonic} {' '.join(res.cstool_operands)}"
            )
            print(
                f"    robustone -> {res.robustone_mnemonic} {' '.join(res.robustone_operands)}"
            )
        return 1
    print(
        f"All {len(load_instructions())} RISC-V instructions matched between cstool and robustone."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
