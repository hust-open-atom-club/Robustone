#!/usr/bin/env python3
"""
Infer LoongArch instruction formats from YAML test cases.

Outputs a TSV of: mnemonic, format, opcode_mask, opcode_pattern, sample_word, sample_asm
"""

import sys
import yaml
from pathlib import Path
from collections import defaultdict

# Define standard LoongArch formats with their opcode bit positions.
# Each format specifies which bits are considered "opcode" (fixed for all instances of the mnemonic)
# and which bits are operands (variable).
FORMATS = [
    # name, opcode_start, opcode_end (inclusive, 0-indexed from LSB), description
    ("3R", 15, 31, "3-register: opcode[31:15], rk[14:10], rj[9:5], rd[4:0]"),
    ("2RI12", 22, 31, "2-reg+imm12: opcode[31:22], rj[21:17], rd[16:12], si12[11:0]"),
    ("2RI14", 22, 31, "2-reg+imm14: opcode[31:22], rj[21:17], rd[16:12], si14[13:0]"),
    ("2RI16", 22, 31, "2-reg+imm16: opcode[31:22], rj[21:17], rd[16:12], si16[15:0]?"),
    ("1RI21", 26, 31, "1-reg+imm21: opcode[31:26], rd[4:0], imm21[25:5]?"),  # beqz/bnez
    ("I26", 26, 31, "26-bit imm: opcode[31:26], imm26[25:0]"),  # b/bl
    ("B", 26, 31, "branch: opcode[31:26], offset[25:10], rj[9:5], rd[4:0]"),
    ("JIRL", 26, 31, "jirl: opcode[31:26], imm16[25:10], rj[9:5], rd[4:0]"),
    ("ATOMIC", 24, 31, "atomic: opcode[31:24], ..."),
    ("BARRIER", 15, 31, "barrier: opcode[31:15], hint[14:0]"),
    ("1RI20", 25, 31, "1-reg+imm20: opcode[31:25], si20[24:5], rd[4:0]"),  # lu12i.w etc
    ("FCMP", 22, 31, "fcmp: opcode[31:22], ..."),
    ("F3R", 15, 31, "FP 3-reg: opcode[31:15], fk[14:10], fj[9:5], fd[4:0]"),
    ("F2RI8", 22, 31, "FP 2-reg+imm8: opcode[31:22], fj[21:17], fd[16:12], imm8[11:4]?"),
    ("V3R", 15, 31, "Vector 3-reg: opcode[31:15], xk[14:10], xj[9:5], xd[4:0]"),
    ("V2RI8", 22, 31, "Vector 2-reg+imm8: opcode[31:22], xj[21:17], xd[16:12], imm8[11:4]?"),
    ("V2RI5", 22, 31, "Vector 2-reg+imm5: opcode[31:22], xj[21:17], xd[16:12], imm5[11:7]?"),
    ("V1RI5", 22, 31, "Vector 1-reg+imm5: opcode[31:22], xd[16:12], imm5[11:7]?"),
    ("SCR", 15, 31, "SCR: opcode[31:15], ..."),
    ("PRIV", 22, 31, "privilege: opcode[31:22], ..."),
    ("UNKNOWN", 0, 31, "full word match (fallback)"),
]


def extract_opcode(word, start, end):
    """Extract opcode bits from word."""
    mask = ((1 << (end - start + 1)) - 1) << start
    return word & mask


def infer_format(words):
    """Find the best format for a set of instruction words."""
    if not words:
        return "UNKNOWN", 0, 0

    for fmt_name, start, end, desc in FORMATS:
        if fmt_name == "UNKNOWN":
            continue
        opcodes = [extract_opcode(w, start, end) for w in words]
        if len(set(opcodes)) == 1:
            # All instances have the same opcode for this format
            mask = ((1 << (end - start + 1)) - 1) << start
            pattern = opcodes[0]
            return fmt_name, mask, pattern

    # Fallback: full word match
    if len(words) == 1:
        return "UNKNOWN", 0xFFFFFFFF, words[0]
    else:
        # Find common prefix
        common = words[0]
        diff = 0
        for w in words[1:]:
            common &= w
            diff |= (w ^ words[0])
        if diff == 0:
            return "UNKNOWN", 0xFFFFFFFF, words[0]
        # Use highest bits that are common
        highest_diff = 32 - diff.bit_length()
        mask = ((1 << highest_diff) - 1) << (32 - highest_diff)
        pattern = common & mask
        return "UNKNOWN", mask, pattern


def parse_operands(asm_text):
    """Parse operands from asm_text."""
    parts = asm_text.split(None, 1)
    mnemonic = parts[0]
    if len(parts) == 1:
        return mnemonic, []
    rest = parts[1]
    operands = [op.strip() for op in rest.split(",")]
    return mnemonic, operands


def operand_type(op):
    """Classify an operand."""
    op = op.strip()
    if op.startswith("$"):
        if op.startswith("$xr"):
            return "XR"
        elif op.startswith("$f") and not op.startswith("$fcc"):
            return "FR"
        elif op.startswith("$fcc"):
            return "FCC"
        elif op.startswith("$scr"):
            return "SCR"
        else:
            return "GR"
    else:
        # Try to parse as integer (hex or decimal)
        try:
            if op.startswith("0x") or op.startswith("0X"):
                int(op, 16)
            else:
                int(op)
            return "IMM"
        except ValueError:
            return "OTHER"


def main():
    yaml_dir = Path("third_party/capstone/tests/MC/LoongArch")
    if not yaml_dir.exists():
        print(f"Error: {yaml_dir} not found", file=sys.stderr)
        sys.exit(1)

    mnemonic_words = defaultdict(list)
    mnemonic_samples = {}

    for yaml_file in sorted(yaml_dir.rglob("*.yaml")):
        try:
            with yaml_file.open("r", encoding="utf-8") as fh:
                data = yaml.safe_load(fh)
        except yaml.YAMLError:
            continue
        if not isinstance(data, dict):
            continue
        for test_case in data.get("test_cases", []):
            inp = test_case.get("input", {})
            expected = test_case.get("expected", {})
            insns = expected.get("insns", [])
            if not insns:
                continue
            byte_list = inp.get("bytes", [])
            if len(byte_list) != 4:
                continue
            word = int.from_bytes(byte_list, byteorder="little")
            asm_text = insns[0].get("asm_text", "")
            mnemonic, operands = parse_operands(asm_text)
            mnemonic_words[mnemonic].append(word)
            if mnemonic not in mnemonic_samples:
                mnemonic_samples[mnemonic] = (word, asm_text, operands)

    print("mnemonic\tformat\tmask_hex\tpattern_hex\tsample_word\toperand_types\tcount")
    for mnemonic in sorted(mnemonic_words.keys()):
        words = mnemonic_words[mnemonic]
        fmt, mask, pattern = infer_format(words)
        word, asm_text, operands = mnemonic_samples[mnemonic]
        op_types = "/".join(operand_type(op) for op in operands)
        print(f"{mnemonic}\t{fmt}\t0x{mask:08X}\t0x{pattern:08X}\t0x{word:08X}\t{op_types}\t{len(words)}")


if __name__ == "__main__":
    main()
