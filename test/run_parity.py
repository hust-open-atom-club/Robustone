#!/usr/bin/env python3
"""
Generic parity test runner comparing Robustone CLI output with Capstone's cstool.

- Discovers architectures under test/<arch>/config.json
- For each arch, loads cases (hex + optional expected cstool output) and compares
  the current outputs of Robustone and cstool.
 - Returns non-zero when mismatches or command failures occur.

Usage examples:
  python3 test/run_parity.py --all
  python3 test/run_parity.py --arch riscv32 --limit 20
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple


REPO_ROOT = Path(__file__).resolve().parents[1]
TEST_ROOT = REPO_ROOT / "test"
ROBUSTONE_BIN_DEBUG = REPO_ROOT / "robustone" / "target" / "debug" / "robustone"
ROBUSTONE_BIN_RELEASE = REPO_ROOT / "robustone" / "target" / "release" / "robustone"
CSTOOL_BIN = REPO_ROOT / "third_party" / "capstone" / "cstool" / "cstool"


@dataclass
class ArchConfig:
    name: str
    robustone_arch: str
    cstool_arch: str
    cases_file: Path
    robustone_flags: List[str]
    cstool_flags: List[str]
    dir: Path


def _read_json_config(config_path: Path) -> Dict:
    with config_path.open("r", encoding="utf-8") as f:
        return json.load(f)


def discover_arch_configs() -> Dict[str, ArchConfig]:
    archs: Dict[str, ArchConfig] = {}
    for sub in sorted(TEST_ROOT.iterdir()):
        if not sub.is_dir():
            continue
        cfg_path = sub / "config.json"
        if not cfg_path.is_file():
            continue
        raw = _read_json_config(cfg_path)
        name = raw.get("name") or sub.name
        archs[name] = ArchConfig(
            name=name,
            robustone_arch=raw.get("robustone_arch", name),
            cstool_arch=raw.get("cstool_arch", name),
            cases_file=(sub / raw.get("cases_file", "verified_instructions.txt")).resolve(),
            robustone_flags=list(raw.get("robustone_flags", [])),
            cstool_flags=list(raw.get("cstool_flags", [])),
            dir=sub,
        )
    return archs


def normalize_output(s: str) -> str:
    return " ".join(s.split())


def run_command(cmd: List[str]) -> Tuple[int, str, str]:
    try:
        r = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        return r.returncode, r.stdout.strip(), r.stderr.strip()
    except Exception as exc:  # pragma: no cover - best effort logging
        return 1, "", str(exc)


def ensure_cstool(verbose: bool = False) -> None:
    if CSTOOL_BIN.exists() and CSTOOL_BIN.is_file():
        return
    if verbose:
        print("Building cstool via test/build_cstool.sh ...")
    script = TEST_ROOT / "build_cstool.sh"
    subprocess.run(["bash", str(script), str(REPO_ROOT / "third_party" / "capstone")], check=True)


def build_robustone_if_needed(verbose: bool = False) -> Path:
    if ROBUSTONE_BIN_DEBUG.exists():
        return ROBUSTONE_BIN_DEBUG
    if ROBUSTONE_BIN_RELEASE.exists():
        return ROBUSTONE_BIN_RELEASE
    if verbose:
        print("Building robustone (debug)...")
    subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(REPO_ROOT / "robustone" / "Cargo.toml"),
            "--bin",
            "robustone",
        ],
        check=True,
    )
    return ROBUSTONE_BIN_DEBUG


def parse_cases(cases_file: Path) -> List[Tuple[str, str, str]]:
    """Return list of (hex_input, expected_cstool, note)."""
    cases: List[Tuple[str, str, str]] = []
    with cases_file.open("r", encoding="utf-8") as f:
        for raw in f:
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            if "#" not in line:
                # Allow plain hex-only rows as well
                hex_input = line
                expected = ""
                note = ""
            else:
                hex_input, right = [seg.strip() for seg in line.split("#", 1)]
                if "|" in right:
                    expected, note = [seg.strip() for seg in right.split("|", 1)]
                else:
                    expected, note = right.strip(), ""
            if hex_input:
                cases.append((hex_input, expected, note))
    return cases


def run_arch(arch: ArchConfig, robustone_bin: Path, *, limit: Optional[int] = None, verbose: bool = False, show_mismatches: int = 10, fail_fast: bool = False) -> int:
    print(f"==> [{arch.name}] Loading cases from {arch.cases_file.relative_to(REPO_ROOT)}")
    cases = parse_cases(arch.cases_file)
    if limit is not None:
        cases = cases[:limit]
    if not cases:
        print(f"No cases for {arch.name}")
        return 1

    print(f"Running {len(cases)} cases against cstool...")

    matches = 0
    mismatches: List[Tuple[str, str, str, str, str]] = []
    cmd_failures: List[Tuple[str, int, str, int, str]] = []
    doc_drifts: List[Tuple[str, str, str]] = []

    for idx, (hex_input, expected_cstool, note) in enumerate(cases, start=1):
        rob_cmd = [str(robustone_bin), arch.robustone_arch, hex_input, *arch.robustone_flags]
        cs_cmd = [str(CSTOOL_BIN), arch.cstool_arch, hex_input, *arch.cstool_flags]

        rob_code, rob_out, rob_err = run_command(rob_cmd)
        cs_code, cs_out, cs_err = run_command(cs_cmd)

        if rob_code != 0 or cs_code != 0:
            cmd_failures.append((hex_input, rob_code, rob_err, cs_code, cs_err))
            print(f"✗ {idx:3d}/{len(cases)} {hex_input} (command failure)")
            if fail_fast:
                break
            continue

        if expected_cstool and normalize_output(expected_cstool) != normalize_output(cs_out):
            doc_drifts.append((hex_input, expected_cstool, cs_out))

        if rob_out == cs_out:
            matches += 1
            if verbose:
                print(f"✓ {idx:3d}/{len(cases)} {hex_input}")
        else:
            mismatches.append((hex_input, expected_cstool, rob_out, cs_out, note))
            print(f"✗ {idx:3d}/{len(cases)} {hex_input}")
            if fail_fast:
                break

    total = len(cases)
    print("-" * 60)
    print(f"[{arch.name}] Results: {matches}/{total} matches ({(matches/total*100.0):.1f}%)")
    if mismatches:
        print(f"[{arch.name}] Mismatches: {len(mismatches)}")
    if cmd_failures:
        print(f"[{arch.name}] Command failures: {len(cmd_failures)}")
    if doc_drifts:
        print(f"[{arch.name}] Documentation drift: {len(doc_drifts)}")

    if mismatches:
        print("\nFirst mismatches:")
        for i, (hex_input, expected, rob_out, cs_out, note) in enumerate(mismatches[:show_mismatches], start=1):
            print(f"\n{i}. {hex_input}")
            if expected:
                print(f"   Expected:  {expected}")
            if note:
                print(f"   Note:      {note}")
            print(f"   cstool:    {cs_out}")
            print(f"   robustone: {rob_out}")

    if doc_drifts:
        print("\nDocumentation mismatches (stored vs current cstool):")
        for hex_input, recorded, current in doc_drifts[:show_mismatches]:
            print(f"  {hex_input}")
            print(f"    Stored:  {recorded}")
            print(f"    Current: {current}")

    if cmd_failures:
        print("\nCommand failures (non-zero exit codes):")
        for hex_input, rob_code, rob_err, cs_code, cs_err in cmd_failures[:show_mismatches]:
            print(f"  {hex_input}")
            print(f"    robustone -> code {rob_code}, stderr: {rob_err or '<no stderr>'}")
            print(f"    cstool    -> code {cs_code}, stderr: {cs_err or '<no stderr>'}")

    ok = matches == total and not cmd_failures and not doc_drifts
    return 0 if ok else 1


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Robustone vs cstool parity runner")
    group = parser.add_mutually_exclusive_group(required=False)
    group.add_argument("--arch", action="append", help="Architecture name(s) to run (default: all discovered)")
    group.add_argument("--all", action="store_true", help="Run all discovered architectures")
    parser.add_argument("--limit", type=int, default=None, help="Limit number of cases per architecture")
    parser.add_argument("--verbose", action="store_true", help="Verbose per-case success output")
    parser.add_argument("--show-mismatches", type=int, default=10, help="Number of mismatches to print per arch")
    parser.add_argument("--fail-fast", action="store_true", help="Stop on first failure for each arch")
    args = parser.parse_args(argv)

    archs = discover_arch_configs()
    if not archs:
        print("No architecture configs found under test/<arch>/config.json", file=sys.stderr)
        return 2

    selected: List[str]
    if args.all or not args.arch:
        selected = list(archs.keys())
    else:
        selected = []
        for name in args.arch:
            if name not in archs:
                print(f"Unknown arch '{name}'. Known: {', '.join(sorted(archs.keys()))}", file=sys.stderr)
                return 2
            selected.append(name)

    ensure_cstool(verbose=args.verbose)
    robustone_bin = build_robustone_if_needed(verbose=args.verbose)

    overall_rc = 0
    for name in selected:
        print("=" * 60)
        rc = run_arch(
            archs[name],
            robustone_bin,
            limit=args.limit,
            verbose=args.verbose,
            show_mismatches=args.show_mismatches,
            fail_fast=args.fail_fast,
        )
        overall_rc |= rc

    return overall_rc


if __name__ == "__main__":
    sys.exit(main())
