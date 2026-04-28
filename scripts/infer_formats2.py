#!/usr/bin/env python3
"""
Infer LoongArch instruction formats from YAML test cases (improved).

Uses operand types to guide format selection.
"""

import sys
import yaml
from pathlib import Path
from collections import defaultdict


def parse_operands(asm_text):
    parts = asm_text.split(None, 1)
    mnemonic = parts[0]
    if len(parts) == 1:
        return mnemonic, []
    rest = parts[1]
    operands = [op.strip() for op in rest.split(",")]
    return mnemonic, operands


def operand_type(op):
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
        try:
            if op.startswith("0x") or op.startswith("0X"):
                int(op, 16)
            else:
                int(op)
            return "IMM"
        except ValueError:
            return "OTHER"


def get_candidate_formats(op_types):
    """Return candidate formats based on operand types."""
    key = tuple(op_types)
    # Map operand type signatures to likely formats
    candidates = {
        ("GR", "GR", "GR"): [("3R", 15, 31), ("4R", 10, 31), ("F3R", 15, 31), ("V3R", 15, 31)],
        ("GR", "GR", "IMM"): [("2RI12", 22, 31), ("2RI14", 22, 31), ("2RI16", 22, 31), ("B", 26, 31), ("JIRL", 26, 31), ("MEM", 22, 31)],
        ("GR", "IMM"): [("1RI20", 25, 31), ("1RI21", 26, 31), ("2RI12", 22, 31)],
        ("IMM",): [("I26", 26, 31), ("BARRIER", 15, 31), ("1RI20", 25, 31)],
        ("GR", "GR"): [("2R", 15, 31), ("SCR", 15, 31)],
        ("GR",): [("1R", 15, 31)],
        (): [("BARRIER", 15, 31)],
        ("FR", "FR", "FR"): [("F3R", 15, 31)],
        ("FR", "FR", "IMM"): [("F2RI8", 22, 31)],
        ("FR", "FR"): [("F2R", 15, 31)],
        ("FR",): [("F1R", 15, 31)],
        ("XR", "XR", "XR"): [("V3R", 15, 31)],
        ("XR", "XR", "IMM"): [("V2RI8", 22, 31), ("V2RI5", 22, 31)],
        ("XR", "IMM"): [("V1RI5", 22, 31)],
        ("XR",): [("V1R", 15, 31)],
        ("FCC", "IMM"): [("1RI21", 26, 31)],
        ("GR", "FCC"): [("2R", 15, 31)],
        ("GR", "GR", "GR", "IMM"): [("4R", 10, 31), ("3R", 15, 31)],
    }
    return candidates.get(key, [("3R", 15, 31), ("2RI12", 22, 31), ("1RI20", 25, 31), ("B", 26, 31), ("I26", 26, 31), ("UNKNOWN", 0, 31)])


def extract_bits(word, start, end):
    mask = ((1 << (end - start + 1)) - 1) << start
    return word & mask


def infer_format(mnemonic, words, op_types):
    candidates = get_candidate_formats(op_types)

    for fmt_name, start, end in candidates:
        if fmt_name == "UNKNOWN":
            continue
        extracted = [extract_bits(w, start, end) for w in words]
        if len(set(extracted)) == 1:
            mask = ((1 << (end - start + 1)) - 1) << start
            pattern = extracted[0]
            return fmt_name, mask, pattern

    # Fallback
    if len(words) == 1:
        return "UNKNOWN", 0xFFFFFFFF, words[0]
    common = words[0]
    diff = 0
    for w in words[1:]:
        common &= w
        diff |= (w ^ words[0])
    if diff == 0:
        return "UNKNOWN", 0xFFFFFFFF, words[0]
    highest_diff = 32 - diff.bit_length()
    mask = ((1 << highest_diff) - 1) << (32 - highest_diff)
    pattern = common & mask
    return "UNKNOWN", mask, pattern


def main():
    yaml_dir = Path("third_party/capstone/tests/MC/LoongArch")
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

    # Print summary of format distribution
    format_counts = defaultdict(int)
    format_mnemonics = defaultdict(list)

    for mnemonic in sorted(mnemonic_words.keys()):
        words = mnemonic_words[mnemonic]
        word, asm_text, operands = mnemonic_samples[mnemonic]
        op_types = [operand_type(op) for op in operands]
        fmt, mask, pattern = infer_format(mnemonic, words, op_types)
        format_counts[fmt] += 1
        format_mnemonics[fmt].append((mnemonic, mask, pattern, word, asm_text, op_types))

    print("Format distribution:")
    for fmt, count in sorted(format_counts.items(), key=lambda x: -x[1]):
        print(f"  {fmt:12s}: {count} mnemonics")

    print("\n--- Sample mnemonics per format ---")
    for fmt, entries in sorted(format_mnemonics.items()):
        print(f"\n{fmt} ({len(entries)} mnemonics):")
        for mnemonic, mask, pattern, word, asm, op_types in entries[:10]:
            print(f"  {mnemonic:20s} mask=0x{mask:08X} pattern=0x{pattern:08X} word=0x{word:08X} ops={op_types} -> {asm}")
        if len(entries) > 10:
            print(f"  ... and {len(entries)-10} more")


if __name__ == "__main__":
    main()
