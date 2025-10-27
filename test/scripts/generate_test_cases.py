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
            "000000b3", "00000033", "00000037", "00000117", "fe010113",
            "000003b7", "00000397", "00000437", "000004b7",
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
