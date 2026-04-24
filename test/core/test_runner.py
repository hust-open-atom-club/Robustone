"""
Main test runner for the Robustone test framework.
"""

import ast
import os
import sys
import time
from datetime import date
from pathlib import Path
from typing import List, Optional

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - exercised on Python < 3.11
    try:
        import tomli as tomllib  # type: ignore[assignment]
    except ModuleNotFoundError:  # pragma: no cover - exercised when tomli is absent
        tomllib = None  # type: ignore[assignment]

PROJECT_ROOT = os.path.dirname(os.path.abspath(__file__))
sys.path.append(PROJECT_ROOT)

# pylint: disable=wrong-import-position
try:
    from .arch_config import ArchConfig, validate_config
    from .comparator import (
        OutputComparator,
        TestCaseResult,
        ArchTestSummary,
        ComparisonResult,
        ComparisonSurface,
    )
    from .utils import run_command, parse_test_case, find_repo_root
    from .yaml_loader import load_yaml_test_cases
except ImportError:  # pragma: no cover - script-mode fallback
    from arch_config import ArchConfig, validate_config
    from comparator import (
        OutputComparator,
        TestCaseResult,
        ArchTestSummary,
        ComparisonResult,
        ComparisonSurface,
    )
    from utils import run_command, parse_test_case, find_repo_root
    from yaml_loader import load_yaml_test_cases

# pylint: enable=wrong-import-position


def _parse_known_differences_fallback(text: str) -> dict:
    """Parse the tiny known-differences TOML subset without third-party deps."""
    differences = []
    current = None

    for raw_line in text.splitlines():
        line = raw_line.split("#", 1)[0].strip()
        if not line:
            continue

        if line == "[[difference]]":
            if current:
                differences.append(current)
            current = {}
            continue

        if current is None:
            continue

        key, separator, value = line.partition("=")
        if not separator:
            raise ValueError(f"Invalid known-differences entry: {raw_line}")

        key = key.strip()
        value = value.strip()
        if value.lower() in {"true", "false"}:
            parsed = value.lower() == "true"
        else:
            parsed = ast.literal_eval(value)
        current[key] = parsed

    if current:
        differences.append(current)

    return {"difference": differences}


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
        self.known_differences = self._load_known_differences()

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

        # Allow per-test-case cstool arch override from note (YAML loader embeds it)
        cstool_arch = config.cstool_arch
        if note and "cstool_arch=" in note:
            for part in note.split(";"):
                part = part.strip()
                if part.startswith("cstool_arch="):
                    cstool_arch = part[len("cstool_arch=") :].strip()
                    break

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
            cstool_arch,
            hex_input,
        ] + config.cstool_flags
        semantic_robustone_cmd = [
            str(self.robustone_bin),
            "--json",
            "--detailed",
            "--real-detail",
            config.robustone_arch,
            hex_input,
        ] + config.robustone_flags
        semantic_cstool_cmd = [
            str(self.cstool_bin),
            "-d",
            "-r",
            cstool_arch,
            hex_input,
        ] + config.cstool_flags

        # Execute commands
        rob_code, rob_out, rob_err = run_command(robustone_cmd)
        cs_code, cs_out, cs_err = run_command(cstool_cmd)
        rob_sem_code, rob_sem_out, rob_sem_err = run_command(semantic_robustone_cmd)
        cs_sem_code, cs_sem_out, cs_sem_err = run_command(semantic_cstool_cmd)

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
            robustone_semantic_out=rob_sem_out,
            cstool_semantic_out=cs_sem_out,
            robustone_semantic_exit_code=rob_sem_code,
            cstool_semantic_exit_code=cs_sem_code,
            robustone_semantic_stderr=rob_sem_err,
            cstool_semantic_stderr=cs_sem_err,
        )

    def _load_known_differences(self) -> dict:
        """Load active known-difference entries keyed by (arch, hex_input, surface)."""
        path = self.repo_root / "tests" / "differential" / "known-differences.toml"
        if not path.exists():
            return {}

        if tomllib is not None:
            with path.open("rb") as handle:
                data = tomllib.load(handle)
        else:
            data = _parse_known_differences_fallback(path.read_text(encoding="utf-8"))

        differences = {}
        for entry in data.get("difference", []):
            if not entry.get("active", False):
                continue
            self._validate_active_known_difference(entry)
            arch = str(entry.get("arch", "")).strip()
            hex_input = str(entry.get("hex", "")).strip().lower()
            surface = str(entry.get("surface", "")).strip().lower()
            reason = str(entry.get("reason", "")).strip()
            key = (arch, hex_input, surface)
            if key in differences:
                raise ValueError(
                    "Duplicate active known-difference entry for "
                    f"arch={arch}, hex={hex_input}, surface={surface}"
                )
            differences[key] = reason
        return differences

    def _validate_active_known_difference(self, entry: dict) -> None:
        """Reject malformed or stale active known-difference entries."""
        required_fields = {
            "arch",
            "hex",
            "surface",
            "reason",
            "owner",
            "expires_on",
            "active",
        }
        allowed_fields = required_fields

        unknown_fields = set(entry.keys()) - allowed_fields
        if unknown_fields:
            raise ValueError(
                "Unknown field(s) in active known-difference entry: "
                + ", ".join(sorted(unknown_fields))
            )

        for field in required_fields:
            value = entry.get(field)
            if field == "active":
                continue
            if value is None or not str(value).strip():
                raise ValueError(
                    f"Active known-difference entry is missing required field '{field}'"
                )

        arch = str(entry["arch"]).strip()
        hex_input = str(entry["hex"]).strip().lower()
        surface = str(entry["surface"]).strip().lower()
        expires_on = str(entry["expires_on"]).strip()

        if surface not in {candidate.value for candidate in ComparisonSurface}:
            raise ValueError(f"Unsupported known-difference surface: {surface}")

        if not all(character in "0123456789abcdef" for character in hex_input):
            raise ValueError(
                "Known-difference hex field must be normalized lowercase hexadecimal"
            )

        if not arch:
            raise ValueError("Known-difference arch field must not be empty")

        expiry_date = date.fromisoformat(expires_on)
        if date.today() > expiry_date:
            raise ValueError(
                "Known-difference entry expired on "
                f"{expires_on} for arch={arch}, hex={hex_input}, surface={surface}"
            )

    def apply_known_difference(
        self, arch_name: str, result: TestCaseResult
    ) -> TestCaseResult:
        """Downgrade expected divergences according to the active whitelist."""
        if result.result != ComparisonResult.MISMATCH:
            return result

        if not result.surface_results:
            reason = self.known_differences.get(
                (arch_name, result.hex_input.lower(), ComparisonSurface.TEXT.value)
            )
            if not reason:
                return result
            note = f"known-difference[{ComparisonSurface.TEXT.value}]: {reason}"
            result.note = f"{result.note} | {note}" if result.note else note
            result.result = ComparisonResult.MATCH
            return result

        allowlisted_surfaces = []
        for surface_result in result.surface_results:
            if surface_result.matched:
                continue
            reason = self.known_differences.get(
                (arch_name, result.hex_input.lower(), surface_result.surface.value)
            )
            if not reason:
                continue
            surface_result.matched = True
            allowlisted_surfaces.append((surface_result.surface.value, reason))

        if not allowlisted_surfaces:
            return result

        notes = [
            f"known-difference[{surface}]: {reason}"
            for surface, reason in allowlisted_surfaces
        ]
        suffix = " | ".join(notes)
        result.note = f"{result.note} | {suffix}" if result.note else suffix
        if all(surface.matched for surface in result.surface_results):
            result.result = ComparisonResult.MATCH
        return result

    def run_arch_tests(  # pylint: disable=too-many-branches
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
        test_cases = self._load_test_cases(config)
        if limit is not None:
            test_cases = test_cases[:limit]

        if not test_cases:
            if verbose:
                source = config.yaml_source or config.cases_file
                print(f"Warning: No test cases found in {source}")
            # Return empty summary instead of raising error
            return self.comparator.generate_summary(config.name, [], 0)

        if verbose:
            print(f"Running {len(test_cases)} test cases for {config.name}...")
            source = config.yaml_source or config.cases_file
            if source is not None:
                try:
                    print(f"Test file: {Path(source).relative_to(self.repo_root)}")
                except ValueError:
                    print(f"Test file: {source}")

        start_time = time.time()
        results: List[TestCaseResult] = []

        for i, (hex_input, expected, note) in enumerate(test_cases, start=1):
            if verbose:
                print(f"[{i:3d}/{len(test_cases)}] Testing {hex_input}")

            result = self.run_test_case(config, hex_input, expected, note, verbose)
            result = self.apply_known_difference(config.name, result)
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

    def _load_test_cases(self, config: ArchConfig) -> List[tuple]:
        """
        Load test cases from the configured source.

        Supports both legacy text files and Capstone YAML sources.

        Args:
            config: Architecture configuration

        Returns:
            List of (hex_input, expected, note) tuples
        """
        # YAML source takes precedence if configured
        if config.yaml_source is not None:
            return list(
                load_yaml_test_cases(
                    config.yaml_source,
                    yaml_filter=config.yaml_filter,
                )
            )

        # Fall back to legacy text format
        test_cases = []
        with config.cases_file.open("r", encoding="utf-8") as f:
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
