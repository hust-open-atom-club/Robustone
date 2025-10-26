#!/usr/bin/env python3
"""
RISC-V instruction compatibility test with cstool
Tests that robustone produces exactly the same output as cstool
"""

import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import List, Tuple

# Configuration
ROOT = Path(__file__).resolve().parents[2]
ROBUSTONE_BIN = ROOT / "robustone" / "target" / "debug" / "robustone"
CSTOOL_BIN = ROOT / "third_party" / "capstone" / "cstool" / "cstool"
TEST_FILE = Path(__file__).with_name("verified_instructions.txt")

@dataclass
class TestCase:
    hex_input: str
    expected_cstool: str
    note: str


def run_command(cmd: List[str]) -> Tuple[int, str, str]:
    """Run a command and return (returncode, stdout, stderr)."""
    try:
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        return result.returncode, result.stdout.strip(), result.stderr.strip()
    except Exception as exc:  # pragma: no cover - best effort logging
        return 1, "", str(exc)


def parse_expected(raw: str) -> Tuple[str, str]:
    """Split the expected section into (cstool_output, optional_note)."""
    if not raw:
        return "", ""
    parts = raw.split('|', 1)
    expected = parts[0].strip()
    note = parts[1].strip() if len(parts) == 2 else ""
    return expected, note


def normalize_output(output: str) -> str:
    """Collapse whitespace so formatting differences do not flag drift."""
    return " ".join(output.split())

def build_robustone_if_needed():
    """Build robustone if it doesn't exist."""
    if not ROBUSTONE_BIN.exists():
        print("Building robustone...")
        subprocess.run([
            "cargo", "build", "--manifest-path",
            str(ROOT / "robustone" / "Cargo.toml"), "--bin", "robustone"
        ], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def main():
    """Run tests comparing robustone with cstool."""

    if not CSTOOL_BIN.exists():
        print(f"ERROR: cstool not found at {CSTOOL_BIN}")
        return 1

    build_robustone_if_needed()

    # Load test cases
    test_cases: List[TestCase] = []
    with open(TEST_FILE, 'r') as f:
        for line in f:
            line = line.strip()
            if '#' in line and not line.startswith('#'):
                hex_input, raw_expected = [segment.strip() for segment in line.split('#', 1)]
                if hex_input:
                    expected_cstool, note = parse_expected(raw_expected)
                    test_cases.append(TestCase(hex_input, expected_cstool, note))

    if not test_cases:
        print(f"No test cases found in {TEST_FILE}")
        return 1

    print(f"Testing {len(test_cases)} instructions against cstool output...")
    print("=" * 60)

    matches = 0
    mismatches = []
    command_failures = []
    doc_mismatches = []

    for i, case in enumerate(test_cases, 1):
        hex_input = case.hex_input
        # Test robustone
        rob_code, robustone_output, robustone_err = run_command([str(ROBUSTONE_BIN), "riscv32", hex_input])

        # Test cstool again to get current output
        cstool_code, current_cstool_output, cstool_err = run_command([str(CSTOOL_BIN), "riscv32", hex_input])

        if cstool_code != 0 or rob_code != 0:
            command_failures.append((hex_input, rob_code, robustone_err, cstool_code, cstool_err))
            print(f"✗ {i:3d}/{len(test_cases)} {hex_input} (command failure)")
            continue

        if (
            case.expected_cstool
            and normalize_output(case.expected_cstool)
            != normalize_output(current_cstool_output)
        ):
            doc_mismatches.append((hex_input, case.expected_cstool, current_cstool_output))

        # Check if robustone matches cstool
        if robustone_output == current_cstool_output:
            matches += 1
            print(f"✓ {i:3d}/{len(test_cases)} {hex_input}")
        else:
            mismatches.append((hex_input, case.expected_cstool, robustone_output, current_cstool_output, case.note))
            print(f"✗ {i:3d}/{len(test_cases)} {hex_input}")

    print("=" * 60)
    print(f"Results: {matches}/{len(test_cases)} matches ({matches/len(test_cases)*100:.1f}%)")
    print(f"Mismatches: {len(mismatches)}")

    if command_failures:
        print(f"Command failures: {len(command_failures)}")

    if doc_mismatches:
        print(f"Documentation drift: {len(doc_mismatches)}")

    if mismatches:
        print("\nFirst 10 mismatches:")
        for i, (hex_input, expected, robustone_out, cstool_out, note) in enumerate(mismatches[:10]):
            print(f"\n{i+1}. {hex_input}")
            if expected:
                print(f"   Expected: {expected}")
            if note:
                print(f"   Note:     {note}")
            print(f"   Cstool:   {cstool_out}")
            print(f"   Robustone: {robustone_out}")

    if doc_mismatches:
        print("\nDocumentation mismatches (cstool output diverged from stored value):")
        for hex_input, recorded, current in doc_mismatches[:10]:
            print(f"  {hex_input}")
            print(f"    Stored:  {recorded}")
            print(f"    Current: {current}")

    if command_failures:
        print("\nCommand failures (non-zero exit codes):")
        for hex_input, rob_code, rob_err, cs_code, cs_err in command_failures[:10]:
            print(f"  {hex_input}")
            print(f"    robustone -> code {rob_code}, stderr: {rob_err or '<no stderr>'}")
            print(f"    cstool    -> code {cs_code}, stderr: {cs_err or '<no stderr>'}")

    success = (
        matches == len(test_cases)
        and not command_failures
        and not doc_mismatches
    )

    return 0 if success else 1

if __name__ == "__main__":
    sys.exit(main())