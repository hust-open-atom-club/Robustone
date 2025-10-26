#!/usr/bin/env python3
"""Simple test to verify robustone vs cstool compatibility."""

import subprocess
from pathlib import Path

# Configuration
ROBUSTONE_BIN = Path("robustone/target/debug/robustone")
CSTOOL_BIN = Path("third_party/capstone/cstool/cstool")

def test_instruction(hex_input: str) -> dict:
    """Test a single instruction and return results."""

    # Test robustone
    try:
        robustone_result = subprocess.run(
            [str(ROBUSTONE_BIN), "riscv32", hex_input],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        robustone_output = robustone_result.stdout.strip()
        robustone_success = robustone_result.returncode == 0 and robustone_output
    except Exception as e:
        robustone_output = ""
        robustone_success = False

    # Test cstool
    try:
        cstool_result = subprocess.run(
            [str(CSTOOL_BIN), "riscv32", hex_input],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        cstool_output = cstool_result.stdout.strip()
        cstool_success = cstool_result.returncode == 0 and cstool_output
    except Exception as e:
        cstool_output = ""
        cstool_success = False

    return {
        "input": hex_input,
        "robustone_output": robustone_output,
        "robustone_success": robustone_success,
        "cstool_output": cstool_output,
        "cstool_success": cstool_success,
        "match": robustone_output == cstool_output
    }

def main():
    """Test a few key instructions."""

    test_cases = [
        "37010000",  # lui sp, 0 - we know this works
        "b3003100",  # add ra, sp, gp - we know this works
        "00000093",  # addi zero, zero, 0
        "0000006f",  # j 0
        "ff010113",  # addi sp, sp, -16  # This is a common stack allocation
    ]

    print("RISC-V Instruction Compatibility Test")
    print("=" * 50)

    matches = 0
    total = len(test_cases)

    for i, hex_input in enumerate(test_cases, 1):
        print(f"\nTest {i}/{total}: {hex_input}")
        result = test_instruction(hex_input)

        print(f"  Robustone: {result['robustone_output']}")
        print(f"  Cstool:    {result['cstool_output']}")
        print(f"  Match:     {'✓' if result['match'] else '✗'}")

        if result['match']:
            matches += 1

    print(f"\nSummary: {matches}/{total} tests passed ({matches/total*100:.1f}%)")

if __name__ == "__main__":
    main()