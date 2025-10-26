#!/usr/bin/env python3
"""
Robustone Test Framework - Main Entry Point

A comprehensive testing framework for comparing Robustone CLI output with
Capstone's cstool reference implementation.

Usage:
  python3 test/run_tests.py --all
  python3 test/run_tests.py --arch riscv32 --limit 20
  python3 test/run_tests.py --list
  python3 test/run_tests.py --init new_arch
"""

import argparse
import sys
from pathlib import Path
from typing import List, Optional

from core.test_runner import TestRunner
from core.arch_config import discover_arch_configs, create_sample_config
from core.comparator import OutputComparator


def list_architectures(test_root: Path) -> None:
    """List all available architectures."""
    archs = discover_arch_configs(test_root)
    if not archs:
        print("No architecture configurations found.")
        print("Create one with: python3 test/run_tests.py --init <arch_name>")
        return

    print("Available architectures:")
    print("-" * 40)
    for name, config in sorted(archs.items()):
        cases_count = _count_test_cases(config.cases_file)
        print(f"  {name:<15} ({cases_count:3d} cases) - {config.description}")


def _count_test_cases(cases_file: Path) -> int:
    """Count test cases in a file."""
    if not cases_file.exists():
        return 0
    count = 0
    with cases_file.open("r") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "#" in line:
                count += 1
    return count


def init_architecture(arch_name: str, test_root: Path) -> None:
    """Initialize a new architecture configuration."""
    arch_dir = test_root / "architectures" / arch_name

    if arch_dir.exists():
        print(f"Architecture '{arch_name}' already exists at {arch_dir}")
        return

    try:
        config_path = create_sample_config(arch_name, arch_dir)
        print(f"Created architecture configuration:")
        print(f"  Config: {config_path}")
        print(f"  Cases:  {config_path.parent / 'test_cases.txt'}")
        print(f"\nNext steps:")
        print(f"  1. Add test cases to {config_path.parent / 'test_cases.txt'}")
        print(f"  2. Run tests with: python3 test/run_tests.py --arch {arch_name}")
    except Exception as e:
        print(f"Failed to create architecture: {e}")
        sys.exit(1)


def run_tests(args: argparse.Namespace) -> int:
    """Run tests based on command line arguments."""
    # Setup
    test_root = Path(__file__).parent
    runner = TestRunner()

    # Discover architectures
    archs = discover_arch_configs(test_root)
    if not archs:
        print("No architecture configurations found under test/architectures/")
        print("Create one with: python3 test/run_tests.py --init <arch_name>")
        return 2

    # Select architectures to test
    if args.all or not args.arch:
        selected_archs = list(archs.keys())
    else:
        selected_archs = []
        for name in args.arch:
            if name not in archs:
                print(f"Unknown architecture: '{name}'")
                print(f"Available: {', '.join(sorted(archs.keys()))}")
                return 2
            selected_archs.append(name)

    if args.verbose:
        print(f"Selected architectures: {', '.join(selected_archs)}")

    # Setup comparator
    comparator = OutputComparator(
        strict_match=not args.loose_match,
        ignore_whitespace=args.ignore_whitespace
    )
    runner.comparator = comparator

    # Ensure binaries are available
    try:
        runner.ensure_binaries(verbose=args.verbose)
    except Exception as e:
        print(f"Failed to prepare binaries: {e}")
        return 1

    # Run tests
    overall_rc = 0
    all_summaries = []

    for arch_name in selected_archs:
        config = archs[arch_name]

        print(f"\n{'='*60}")
        print(f"Testing architecture: {arch_name}")
        print(f"{'='*60}")

        try:
            summary = runner.run_arch_tests(
                config=config,
                limit=args.limit,
                verbose=args.verbose,
                fail_fast=args.fail_fast
            )
            all_summaries.append(summary)
            runner.print_summary(
                summary,
                show_failures=args.show_failures,
                show_details=args.show_details
            )

            # Determine if this architecture passed
            if summary.mismatches > 0 or summary.command_failures > 0:
                overall_rc |= 1

        except Exception as e:
            print(f"Error testing {arch_name}: {e}")
            overall_rc |= 1

    # Print overall summary
    if len(selected_archs) > 1:
        print(f"\n{'='*60}")
        print("Overall Summary")
        print(f"{'='*60}")

        total_cases = sum(s.total_cases for s in all_summaries)
        total_matches = sum(s.matches for s in all_summaries)
        total_mismatches = sum(s.mismatches for s in all_summaries)
        total_failures = sum(s.command_failures for s in all_summaries)
        total_drifts = sum(s.documentation_drifts for s in all_summaries)
        total_time = sum(s.execution_time_ms for s in all_summaries)

        print(f"Architectures tested: {len(selected_archs)}")
        print(f"Total test cases:    {total_cases}")
        total_success_rate = (total_matches/total_cases*100) if total_cases > 0 else 0.0
        print(f"Total matches:       {total_matches} ({total_success_rate:.1f}%)")
        print(f"Total mismatches:    {total_mismatches}")
        print(f"Total failures:      {total_failures}")
        print(f"Total drifts:        {total_drifts}")
        print(f"Total time:          {total_time}ms")

    return overall_rc


def main(argv: Optional[List[str]] = None) -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Robustone Test Framework - Compare with cstool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 test/run_tests.py --all                    # Test all architectures
  python3 test/run_tests.py --arch riscv32          # Test specific architecture
  python3 test/run_tests.py --arch riscv32 --limit 20  # Limit test cases
  python3 test/run_tests.py --list                  # List available architectures
  python3 test/run_tests.py --init new_arch         # Create new architecture config
        """
    )

    # Action selection
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--all", action="store_true", help="Test all architectures")
    group.add_argument("--arch", action="append", metavar="NAME", help="Architecture(s) to test")
    group.add_argument("--list", action="store_true", help="List available architectures")
    group.add_argument("--init", metavar="NAME", help="Initialize new architecture")

    # Test configuration
    parser.add_argument("--limit", type=int, help="Limit number of test cases per architecture")
    parser.add_argument("--fail-fast", action="store_true", help="Stop on first failure")
    parser.add_argument("--show-failures", type=int, default=10, help="Number of failures to display")
    parser.add_argument("--show-details", action="store_true", help="Show detailed failure information")

    # Comparison options
    parser.add_argument("--loose-match", action="store_true", help="Use loose output matching")
    parser.add_argument("--ignore-whitespace", action="store_true", default=True,
                       help="Ignore whitespace differences (default: True)")

    # General options
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")

    args = parser.parse_args(argv)

    # Handle special actions
    test_root = Path(__file__).parent
    if args.list:
        list_architectures(test_root)
        return 0
    elif args.init:
        init_architecture(args.init, test_root)
        return 0

    # Default to --all if no architecture specified
    if not args.all and not args.arch:
        args.all = True

    return run_tests(args)


if __name__ == "__main__":
    sys.exit(main())