"""
Main test runner for the Robustone test framework.
"""

import time
from pathlib import Path
from typing import List, Optional

from .arch_config import ArchConfig, validate_config
from .comparator import OutputComparator, TestCaseResult, ArchTestSummary
from .utils import run_command, parse_test_case, find_repo_root


class TestRunner:
    """Main test runner for comparing robustone and cstool outputs."""

    def __init__(
        self,
        repo_root: Optional[Path] = None,
        comparator: Optional[OutputComparator] = None,
    ):
        """
        Initialize the test runner.

        Args:
            repo_root: Path to repository root (auto-detected if None)
            comparator: Output comparator instance (default created if None)
        """
        self.repo_root = repo_root or find_repo_root()
        self.comparator = comparator or OutputComparator()
        self.robustone_bin = self.repo_root / "target" / "debug" / "robustone"
        self.cstool_bin = (
            self.repo_root / "third_party" / "capstone" / "cstool" / "cstool"
        )

    def ensure_binaries(self, verbose: bool = False) -> None:
        """
        Ensure that required binaries are available.

        Args:
            verbose: Whether to print build progress

        Raises:
            RuntimeError: If binaries cannot be built or found
        """
        # Build robustone if needed
        if verbose:
            print("Building robustone...")
        build_cmd = [
            "cargo",
            "build",
            "--manifest-path",
            str(self.repo_root / "robustone" / "Cargo.toml"),
            "--bin",
            "robustone",
        ]
        code, _, err = run_command(build_cmd)
        if code != 0:
            raise RuntimeError(f"Failed to build robustone: {err}")

        # Check cstool binary
        if not self.cstool_bin.exists():
            # Try to build using build script
            build_script = self.repo_root / "test" / "scripts" / "build_cstool.sh"
            if build_script.exists():
                if verbose:
                    print("Building cstool...")
                code, _, err = run_command(
                    [
                        "bash",
                        str(build_script),
                        str(self.repo_root / "third_party" / "capstone"),
                    ]
                )
                if code != 0:
                    raise RuntimeError(f"Failed to build cstool: {err}")
            else:
                raise RuntimeError(f"cstool not found at {self.cstool_bin}")

    def run_test_case(
        self,
        config: ArchConfig,
        hex_input: str,
        expected: str,
        note: str,
        verbose: bool = False,
    ) -> TestCaseResult:
        """
        Run a single test case.

        Args:
            config: Architecture configuration
            hex_input: Hexadecimal input instruction
            expected: Expected output from documentation
            note: Optional note
            verbose: Whether to print detailed progress

        Returns:
            TestCaseResult
        """
        start_time = time.time()

        # Build commands
        robustone_cmd = [
            str(self.robustone_bin),
            "--detailed",
            config.robustone_arch,
            hex_input,
        ] + config.robustone_flags

        if verbose:
            print(f"Running Command: {robustone_cmd}")

        cstool_cmd = [
            str(self.cstool_bin),
            config.cstool_arch,
            hex_input,
        ] + config.cstool_flags

        # Execute commands
        rob_code, rob_out, rob_err = run_command(robustone_cmd)
        cs_code, cs_out, cs_err = run_command(cstool_cmd)

        if verbose:
            print(f"Running Result: {rob_out}")
        execution_time = int((time.time() - start_time) * 1000)

        # Create result
        return self.comparator.create_test_result(
            hex_input=hex_input,
            expected=expected,
            robustone_out=rob_out,
            cstool_out=cs_out,
            note=note,
            robustone_exit_code=rob_code,
            cstool_exit_code=cs_code,
            robustone_stderr=rob_err,
            cstool_stderr=cs_err,
            execution_time_ms=execution_time,
        )

    def run_arch_tests(
        self,
        config: ArchConfig,
        limit: Optional[int] = None,
        verbose: bool = False,
        fail_fast: bool = False,
    ) -> ArchTestSummary:
        """
        Run all tests for a specific architecture.

        Args:
            config: Architecture configuration
            limit: Optional limit on number of test cases to run
            verbose: Whether to print detailed progress
            fail_fast: Stop on first failure

        Returns:
            ArchTestSummary with all results
        """
        # Validate configuration
        issues = validate_config(config)
        if issues:
            raise ValueError(
                f"Invalid configuration for {config.name}: {'; '.join(issues)}"
            )

        # Load test cases
        test_cases = self._load_test_cases(config.cases_file)
        if limit is not None:
            test_cases = test_cases[:limit]

        if not test_cases:
            if verbose:
                print(f"Warning: No test cases found in {config.cases_file}")
            # Return empty summary instead of raising error
            return self.comparator.generate_summary(config.name, [], 0)

        if verbose:
            print(f"Running {len(test_cases)} test cases for {config.name}...")
            try:
                print(f"Test file: {config.cases_file.relative_to(self.repo_root)}")
            except ValueError:
                print(f"Test file: {config.cases_file}")

        start_time = time.time()
        results: List[TestCaseResult] = []

        for i, (hex_input, expected, note) in enumerate(test_cases, start=1):
            if verbose:
                print(f"[{i:3d}/{len(test_cases)}] Testing {hex_input}")

            result = self.run_test_case(config, hex_input, expected, note, verbose)
            results.append(result)

            # Print immediate result
            if result.result.value == "match":
                if verbose:
                    print(f"  ✓ {hex_input}")
            else:
                print(f"  ✗ {hex_input} ({result.result.value})")
                if fail_fast:
                    break

        total_time = int((time.time() - start_time) * 1000)
        return self.comparator.generate_summary(config.name, results, total_time)

    def _load_test_cases(self, cases_file: Path) -> List[tuple]:
        """
        Load test cases from a file.

        Args:
            cases_file: Path to test cases file

        Returns:
            List of (hex_input, expected, note) tuples
        """
        test_cases = []
        with cases_file.open("r", encoding="utf-8") as f:
            for _, line in enumerate(f, start=1):
                hex_input, expected, note = parse_test_case(line)
                if hex_input:  # Skip empty lines and comments
                    test_cases.append((hex_input, expected, note))
        return test_cases

    def print_summary(
        self,
        summary: ArchTestSummary,
        show_failures: int = 10,
        show_details: bool = False,
    ) -> None:
        """
        Print test summary to stdout.

        Args:
            summary: Test summary to print
            show_failures: Number of failures to show in detail
            show_details: Whether to show detailed failure information
        """
        print(f"\n{'='*60}")
        print(f"Results for {summary.arch_name}:")
        print(f"{'='*60}")
        print(f"Total cases:     {summary.total_cases}")
        success_rate = (
            (summary.matches / summary.total_cases * 100)
            if summary.total_cases > 0
            else 0.0
        )
        print(f"Matches:         {summary.matches} ({success_rate:.1f}%)")
        print(f"Mismatches:      {summary.mismatches}")
        print(f"Command failures: {summary.command_failures}")
        print(f"Documentation drift: {summary.documentation_drifts}")
        print(f"Execution time:  {summary.execution_time_ms}ms")

        failed_results = self.comparator.get_failed_results(summary.results)
        if failed_results:
            print(
                f"\nFailures (showing first {min(show_failures, len(failed_results))}):"
            )
            print("-" * 60)

            for i, result in enumerate(failed_results[:show_failures], start=1):
                print(f"\n{i}. {result.hex_input} ({result.result.value})")
                if show_details:
                    print(self.comparator.format_result_detailed(result))
                else:
                    if result.expected_output:
                        print(f"   Expected: {result.expected_output}")
                    if result.note:
                        print(f"   Note:     {result.note}")
                    print(f"   Robustone: {result.robustone_output}")
                    print(f"   Cstool:    {result.cstool_output}")

        if len(failed_results) > show_failures:
            print(f"\n... and {len(failed_results) - show_failures} more failures")

        success_rate = (
            (summary.matches / summary.total_cases * 100)
            if summary.total_cases > 0
            else 0.0
        )
        print(f"\nOverall success rate: {success_rate:.1f}%")
        if success_rate == 100.0 and summary.total_cases > 0:
            print("🎉 All tests passed!")
