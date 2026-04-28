#!/usr/bin/env python3
"""
Analyze LoongArch YAML test cases to extract opcode patterns and generate
Rust decoder match arms.

Usage:
    python3 scripts/analyze_loongarch_yaml.py
"""

import sys
import yaml
from pathlib import Path
from collections import defaultdict


def main():
    yaml_dir = Path("third_party/capstone/tests/MC/LoongArch")
    if not yaml_dir.exists():
        print(f"Error: {yaml_dir} not found", file=sys.stderr)
        sys.exit(1)

    # Collect all test cases
    cases = []
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
            cases.append((word, asm_text, yaml_file.name))

    print(f"Total cases: {len(cases)}")

    # Group by major opcode (top 6 bits)
    by_major = defaultdict(list)
    for word, asm, fname in cases:
        major = (word >> 26) & 0x3F
        by_major[major].append((word, asm, fname))

    print(f"\nMajor opcode distribution:")
    for major in sorted(by_major.keys()):
        mnemonics = set()
        for word, asm, _ in by_major[major]:
            mnemonic = asm.split()[0]
            mnemonics.add(mnemonic)
        print(f"  0x{major:02X} ({len(by_major[major])} cases, {len(mnemonics)} unique mnemonics)")

    # Group by top 10 bits (opcode10)
    by_opcode10 = defaultdict(list)
    for word, asm, fname in cases:
        opcode10 = (word >> 22) & 0x3FF
        by_opcode10[opcode10].append((word, asm, fname))

    print(f"\nOpcode10 distribution (top 10 bits, showing groups with >5 cases):")
    for opcode10 in sorted(by_opcode10.keys()):
        if len(by_opcode10[opcode10]) > 5:
            mnemonics = set()
            for word, asm, _ in by_opcode10[opcode10]:
                mnemonic = asm.split()[0]
                mnemonics.add(mnemonic)
            print(f"  0x{opcode10:03X} ({len(by_opcode10[opcode10])} cases, {len(mnemonics)} unique)")

    # Group by top 15 bits (opcode15)
    by_opcode15 = defaultdict(list)
    for word, asm, fname in cases:
        opcode15 = (word >> 17) & 0x7FFF
        by_opcode15[opcode15].append((word, asm, fname))

    print(f"\nOpcode15 distribution (top 15 bits, showing groups with >3 cases):")
    for opcode15 in sorted(by_opcode15.keys()):
        if len(by_opcode15[opcode15]) > 3:
            mnemonics = set()
            for word, asm, _ in by_opcode15[opcode15]:
                mnemonic = asm.split()[0]
                mnemonics.add(mnemonic)
            print(f"  0x{opcode15:04X} ({len(by_opcode15[opcode15])} cases, {len(mnemonics)} unique)")

    # Generate a comprehensive mnemonic -> (mask, pattern, format) map
    # by analyzing the first occurrence of each mnemonic
    mnemonic_patterns = {}
    for word, asm, fname in cases:
        mnemonic = asm.split()[0]
        if mnemonic not in mnemonic_patterns:
            mnemonic_patterns[mnemonic] = []
        mnemonic_patterns[mnemonic].append((word, asm))

    print(f"\nTotal unique mnemonics: {len(mnemonic_patterns)}")

    # For each mnemonic, find common bits
    print(f"\nSample patterns:")
    sample_mnemonics = [
        "add.w", "add.d", "sub.w", "sub.d", "and", "or", "xor", "nor",
        "addi.w", "addi.d", "lu12i.w", "pcaddi", "beq", "bne", "blt", "bge",
        "ld.b", "ld.h", "ld.w", "ld.d", "st.b", "st.h", "st.w", "st.d",
        "ll.w", "sc.w", "amswap.w", "amadd.w",
        "fadd.s", "fadd.d", "fld.s", "fst.s",
        "xvadd.b", "xvadd.h", "vld", "vst",
        "dbar", "ibar", "csrrd", "csrwr",
    ]
    for m in sample_mnemonics:
        if m in mnemonic_patterns:
            entries = mnemonic_patterns[m]
            words = [w for w, _ in entries]
            # Find common bits across all instances
            common = words[0]
            for w in words[1:]:
                common &= w
            # Find differing bits
            diff = 0
            for w in words:
                diff |= (w ^ common)
            mask = (~diff) & 0xFFFFFFFF
            print(f"  {m}: mask=0x{mask:08X}, pattern=0x{common:08X}, count={len(words)}")
            if len(words) <= 3:
                for w, asm in entries:
                    print(f"    0x{w:08X} -> {asm}")

    # Try to derive register/imm field patterns for common formats
    print(f"\nField analysis for 3-register format (add.w, add.d, and, or, xor, nor):")
    for m in ["add.w", "add.d", "and", "or", "xor", "nor"]:
        if m in mnemonic_patterns:
            for w, asm in mnemonic_patterns[m][:3]:
                rd = w & 0x1F
                rj = (w >> 5) & 0x1F
                rk = (w >> 10) & 0x1F
                opcode = w >> 15
                print(f"  {m}: word=0x{w:08X} opcode={opcode:04X} rk={rk:2} rj={rj:2} rd={rd:2} -> {asm}")

    print(f"\nField analysis for 2-reg+imm12 format (addi.w, addi.d, andi, ori, xori):")
    for m in ["addi.w", "addi.d", "andi", "ori", "xori"]:
        if m in mnemonic_patterns:
            for w, asm in mnemonic_patterns[m][:3]:
                rd = w & 0x1F
                rj = (w >> 5) & 0x1F
                imm = (w >> 10) & 0xFFF
                opcode = w >> 22
                print(f"  {m}: word=0x{w:08X} opcode={opcode:03X} rj={rj:2} rd={rd:2} imm={imm:3} -> {asm}")

    print(f"\nField analysis for branch format (beq, bne, blt, bge):")
    for m in ["beq", "bne", "blt", "bge"]:
        if m in mnemonic_patterns:
            for w, asm in mnemonic_patterns[m][:3]:
                rd = w & 0x1F
                rj = (w >> 5) & 0x1F
                imm16 = (w >> 10) & 0xFFFF
                opcode = w >> 26
                print(f"  {m}: word=0x{w:08X} opcode={opcode:02X} rj={rj:2} rd={rd:2} imm16={imm16:04X} -> {asm}")

    print(f"\nField analysis for memory load (ld.b, ld.h, ld.w, ld.d):")
    for m in ["ld.b", "ld.h", "ld.w", "ld.d"]:
        if m in mnemonic_patterns:
            for w, asm in mnemonic_patterns[m][:3]:
                rd = w & 0x1F
                rj = (w >> 5) & 0x1F
                imm = (w >> 10) & 0xFFF
                opcode = w >> 22
                print(f"  {m}: word=0x{w:08X} opcode={opcode:03X} rj={rj:2} rd={rd:2} imm={imm:3} -> {asm}")

    # Generate Rust decoder match arms for the most common instructions
    print(f"\n\n=== Generated Rust match arms (sample) ===\n")
    generate_rust_arms(mnemonic_patterns)


def generate_rust_arms(mnemonic_patterns):
    """Generate sample Rust match arms for common instructions."""
    # 3-register ALU (opcode >> 15 is constant for each mnemonic)
    alu_3r = {
        "add.w": 0x0010, "add.d": 0x0011, "sub.w": 0x0012, "sub.d": 0x0013,
        "slt": 0x0014, "sltu": 0x0015, "maskeqz": 0x0016, "masknez": 0x0017,
        "nor": 0x0018, "and": 0x0019, "or": 0x001A, "xor": 0x001B,
        "orn": 0x001C, "andn": 0x001D, "sll.w": 0x001E, "srl.w": 0x001F,
        "sra.w": 0x0020, "sll.d": 0x0021, "srl.d": 0x0022, "sra.d": 0x0023,
    }
    print("// 3-register ALU (match on bits 31:15)")
    for mnemonic, opcode in alu_3r.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:04X} => Self::decode_3reg(word, \"{mnemonic}\"),")

    # 2-reg + imm12 (opcode >> 22 is constant)
    alu_2ri = {
        "slti": 0x002, "sltui": 0x003, "addi.w": 0x00A, "addi.d": 0x00B,
        "lu12i.w": 0x00E, "lu32i.d": 0x00F, "pcaddi": 0x010, "pcalau12i": 0x011,
        "pcaddu12i": 0x012, "pcaddu18i": 0x013,
        "csrrd": 0x018, "csrwr": 0x019, "csrxchg": 0x01A,
        "cacop": 0x01B, "lddir": 0x01C, "ldpte": 0x01D,
        "andi": 0x024, "ori": 0x025, "xori": 0x026,
    }
    print("\n// 2-register + imm12 (match on bits 31:22)")
    for mnemonic, opcode in alu_2ri.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:03X} => Self::decode_2reg_imm(word, \"{mnemonic}\"),")

    # Memory loads (opcode >> 22)
    loads = {
        "ld.b": 0x0A0, "ld.h": 0x0A1, "ld.w": 0x0A2, "ld.d": 0x0A3,
        "st.b": 0x0A4, "st.h": 0x0A5, "st.w": 0x0A6, "st.d": 0x0A7,
        "ld.bu": 0x0A8, "ld.hu": 0x0A9, "ld.wu": 0x0AA,
        "preld": 0x0AC, "preldx": 0x0AD,
        "fld.s": 0x0B0, "fst.s": 0x0B1, "fld.d": 0x0B2, "fst.d": 0x0B3,
    }
    print("\n// Memory operations (match on bits 31:22)")
    for mnemonic, opcode in loads.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:03X} => Self::decode_2reg_mem(word, \"{mnemonic}\"),")

    # Branches (opcode >> 26)
    branches = {
        "beq": 0x14, "bne": 0x15, "blt": 0x16, "bge": 0x17,
        "bltu": 0x18, "bgeu": 0x19,
    }
    print("\n// Branches (match on bits 31:26)")
    for mnemonic, opcode in branches.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:02X} => Self::decode_branch(word, addr, \"{mnemonic}\"),")

    # Jumps (opcode >> 26)
    jumps = {
        "beqz": 0x1A, "bnez": 0x1B, "bceqz": 0x1C, "bcnez": 0x1C,  # bceqz/bcnez share top 6 bits
        "jirl": 0x13, "b": 0x14, "bl": 0x15,
    }
    print("\n// Jumps (match on bits 31:26)")
    for mnemonic, opcode in jumps.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:02X} => Self::decode_jump(word, addr, \"{mnemonic}\"),")

    # Atomics (opcode >> 24)
    atomics = {
        "ll.w": 0x20, "sc.w": 0x21, "ll.d": 0x22, "sc.d": 0x23,
        "amswap.w": 0x24, "amswap.d": 0x25, "amadd.w": 0x26, "amadd.d": 0x27,
        "amand.w": 0x28, "amand.d": 0x29, "amor.w": 0x2A, "amor.d": 0x2B,
        "amxor.w": 0x2C, "amxor.d": 0x2D, "ammax.w": 0x2E, "ammax.d": 0x2F,
        "ammin.w": 0x30, "ammin.d": 0x31, "ammax.wu": 0x32, "ammax.du": 0x33,
        "ammin.wu": 0x34, "ammin.du": 0x35, "amswap.db.w": 0x36, "amswap.db.d": 0x37,
        "amadd.db.w": 0x38, "amadd.db.d": 0x39, "amand.db.w": 0x3A, "amand.db.d": 0x3B,
        "amor.db.w": 0x3C, "amor.db.d": 0x3D, "amxor.db.w": 0x3E, "amxor.db.d": 0x3F,
        "ammax.db.w": 0x40, "ammax.db.d": 0x41, "ammin.db.w": 0x42, "ammin.db.d": 0x43,
        "ammax.db.wu": 0x44, "ammax.db.du": 0x45, "ammin.db.wu": 0x46, "ammin.db.du": 0x47,
    }
    print("\n// Atomics (match on bits 31:24)")
    for mnemonic, opcode in atomics.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:02X} => Self::decode_atomic(word, \"{mnemonic}\"),")

    # Barriers
    barriers = {
        "dbar": 0x1C, "ibar": 0x1D,
    }
    print("\n// Barriers (match on bits 31:15)")
    for mnemonic, opcode in barriers.items():
        if mnemonic in mnemonic_patterns:
            print(f"    0x{opcode:04X} => Self::decode_barrier(word, \"{mnemonic}\"),")


if __name__ == "__main__":
    main()
