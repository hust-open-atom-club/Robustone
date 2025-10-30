#!/usr/bin/env python3
"""
Generate test cases by running cstool on various instructions.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import List, Dict


def find_cstool(repo_root: Path) -> Path:
    """Find the cstool binary."""
    cstool = repo_root / "third_party" / "capstone" / "cstool" / "cstool"
    if not cstool.exists():
        raise FileNotFoundError(f"cstool not found at {cstool}")
    return cstool


def run_cstool(cstool_bin: Path, arch: str, instruction: str) -> str:
    """Run cstool and return its output."""
    try:
        result = subprocess.run(
            [str(cstool_bin), arch, instruction],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running cstool for {instruction}: {e.stderr}", file=sys.stderr)
        return ""


def generate_test_cases(cstool_bin: Path, arch: str, instructions: List[str],
                       output_file: Path) -> None:
    """Generate test cases file from instructions."""
    lines = [f"# Generated test cases for {arch}", "# Format: <hex_bytes> [| <cstool_output>] [| <note>]", ""]

    for instruction in instructions:
        output = run_cstool(cstool_bin, arch, instruction)
        if output:
            lines.append(f"{instruction}  # {output}")
        else:
            lines.append(f"{instruction}  # Failed to get cstool output")

    lines.append("")
    output_file.write_text("\n".join(lines), encoding="utf-8")
    print(f"Generated {len(instructions)} test cases in {output_file}")


def load_instruction_set(arch: str) -> List[str]:
    """Load instruction sets for different architectures."""
    # Basic instruction sets - this can be expanded
    instruction_sets = {
        "riscv32": [
            # Arithmetic instructions
            "00000097", "00000113", "00000193", "00000213", "00000293", "00000313",
            "00000393", "00000413", "00000493", "00000513", "00000593", "00000613",
            # Memory instructions
            "00002303", "00002383", "00002823", "00002c23", "00002023",
            # Branch instructions
            "0000a063", "0000b063", "0000c063", "0000d063", "0000e063", "0000f063",
            # Jump instructions
            "0000006f", "000000ef", "00000067", "000001e7",
            # System instructions
            "00000073", "00100073", "00200073", "00300073",
            # More complex instructions
            # "000000b3", "00000033", "00000037", "00000117", "ff010113",
        ],
        "riscv64": [
            # 64-bit specific instructions
            "37340000", "97820000", "ef008000", "eff01fff",
            "e7004500", "e700c0ff", "63054100", "e39d61fe",
            "63ca9300", "6353b500", "6365d600", "6376f700",
            "03881800", "03994900", "03aa6a00", "03cb2b01",
            "03dc8c01", "2386ad03", "239ace03", "238fef01",
            "9300e000", "13a10101", "13b2027d", "13c303dd",
            "13e4c412", "13f5850c", "1396e601", "13d79701",
            "13d8f840", "33894901", "b30a7b41", "33acac01",
            "b33dde01", "33d26240", "b3439400", "33e5c500",
            "b376f700", "b3543901", "b3503100", "339f0f00",
            "731504b0", "f3560010", "33057b03", "b3459c03",
            "3366bd03", "2fa40210", "af236518", "2f272f01",
            "43f02018", "d3727300", "53f40458", "5385c528",
            "532edea1", "d38405f0", "530605e0", "537500c0",
            "d3f005d0", "d31508e0", "87aa7500", "27276601",
            "43f0201a", "d3727302", "53f4045a", "5385c52a",
            "532edea3",
        ],
        "arm": [
            # ARM Thumb instructions (16-bit)
            "0000", "4700", "1800", "1c40", "4018", "4001", "4280", "d100",
            # ARM instructions (32-bit)
            "e1a00000", "e0810000", "e0800001", "e1500000", "e3500000",
        ]
    }

    return instruction_sets.get(arch, [])


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Generate test cases for architecture")
    parser.add_argument("--arch", required=True, help="Architecture name")
    parser.add_argument("--instructions", nargs="*", help="Specific instructions to test")
    parser.add_argument("--output", help="Output file (default: test_cases.txt)")
    parser.add_argument("--cstool", help="Path to cstool binary")

    args = parser.parse_args()

    # Setup paths
    repo_root = Path(__file__).parent.parent.parent
    cstool_bin = Path(args.cstool) if args.cstool else find_cstool(repo_root)

    # Get instructions
    if args.instructions:
        instructions = args.instructions
    else:
        instructions = load_instruction_set(args.arch)
        if not instructions:
            print(f"No instruction set defined for {args.arch}", file=sys.stderr)
            print("Provide specific instructions with --instructions", file=sys.stderr)
            return 1

    # Setup output file
    if args.output:
        output_file = Path(args.output)
    else:
        arch_dir = repo_root / "test" / "architectures" / args.arch
        arch_dir.mkdir(parents=True, exist_ok=True)
        output_file = arch_dir / "test_cases.txt"

    # Generate test cases
    try:
        generate_test_cases(cstool_bin, args.arch, instructions, output_file)
        return 0
    except Exception as e:
        print(f"Error generating test cases: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
