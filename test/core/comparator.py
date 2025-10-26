"""
Output comparison functionality for the test framework.
"""

from dataclasses import dataclass
from enum import Enum
from typing import List, Tuple, Optional
from .utils import normalize_output


class ComparisonResult(Enum):
    """Result of output comparison."""
    MATCH = "match"
    MISMATCH = "mismatch"
    COMMAND_FAILURE = "command_failure"
    DOCUMENTATION_DRIFT = "documentation_drift"


@dataclass
class TestCaseResult:
    """Result of a single test case."""
    hex_input: str
    result: ComparisonResult
    expected_output: str
    robustone_output: str
    cstool_output: str
    note: str
    robustone_exit_code: int = 0
    cstool_exit_code: int = 0
    robustone_stderr: str = ""
    cstool_stderr: str = ""
    execution_time_ms: int = 0


@dataclass
class ArchTestSummary:
    """Summary of test results for an architecture."""
    arch_name: str
    total_cases: int
    matches: int
    mismatches: int
    command_failures: int
    documentation_drifts: int
    execution_time_ms: int
    results: List[TestCaseResult]


class OutputComparator:
    """Handles comparison of robustone and cstool outputs."""

    def __init__(self, strict_match: bool = True, ignore_whitespace: bool = True):
        """
        Initialize the comparator.

        Args:
            strict_match: If True, require exact string match
            ignore_whitespace: If True, normalize whitespace before comparison
        """
        self.strict_match = strict_match
        self.ignore_whitespace = ignore_whitespace

    def compare_outputs(self, robustone_out: str, cstool_out: str) -> bool:
        """
        Compare two output strings.

        Args:
            robustone_out: Output from robustone
            cstool_out: Output from cstool

        Returns:
            True if outputs match according to comparison rules
        """
        if self.strict_match:
            if self.ignore_whitespace:
                return normalize_output(robustone_out) == normalize_output(cstool_out)
            else:
                return robustone_out == cstool_out
        else:
            # Non-strict comparison can be customized here
            return normalize_output(robustone_out) == normalize_output(cstool_out)

    def check_documentation_drift(self, expected: str, actual: str) -> bool:
        """
        Check if documented expected output differs from actual cstool output.

        Args:
            expected: Expected output from documentation
            actual: Actual output from cstool

        Returns:
            True if there is documentation drift
        """
        if not expected:
            return False
        return normalize_output(expected) != normalize_output(actual)

    def classify_result(self, expected: str, robustone_out: str, cstool_out: str,
                       robustone_exit_code: int, cstool_exit_code: int) -> ComparisonResult:
        """
        Classify the result of a test case.

        Args:
            expected: Expected output from documentation
            robustone_out: Output from robustone
            cstool_out: Output from cstool
            robustone_exit_code: Exit code from robustone
            cstool_exit_code: Exit code from cstool

        Returns:
            ComparisonResult classification
        """
        if robustone_exit_code != 0 or cstool_exit_code != 0:
            return ComparisonResult.COMMAND_FAILURE

        if expected and self.check_documentation_drift(expected, cstool_out):
            return ComparisonResult.DOCUMENTATION_DRIFT

        if self.compare_outputs(robustone_out, cstool_out):
            return ComparisonResult.MATCH
        else:
            return ComparisonResult.MISMATCH

    def create_test_result(self, hex_input: str, expected: str, robustone_out: str,
                          cstool_out: str, note: str, robustone_exit_code: int = 0,
                          cstool_exit_code: int = 0, robustone_stderr: str = "",
                          cstool_stderr: str = "", execution_time_ms: int = 0) -> TestCaseResult:
        """
        Create a TestCaseResult from test execution data.

        Args:
            hex_input: Hexadecimal input instruction
            expected: Expected output
            robustone_out: Output from robustone
            cstool_out: Output from cstool
            note: Optional note
            robustone_exit_code: Exit code from robustone
            cstool_exit_code: Exit code from cstool
            robustone_stderr: Stderr from robustone
            cstool_stderr: Stderr from cstool
            execution_time_ms: Execution time in milliseconds

        Returns:
            TestCaseResult object
        """
        result = self.classify_result(
            expected, robustone_out, cstool_out,
            robustone_exit_code, cstool_exit_code
        )

        return TestCaseResult(
            hex_input=hex_input,
            result=result,
            expected_output=expected,
            robustone_output=robustone_out,
            cstool_output=cstool_out,
            note=note,
            robustone_exit_code=robustone_exit_code,
            cstool_exit_code=cstool_exit_code,
            robustone_stderr=robustone_stderr,
            cstool_stderr=cstool_stderr,
            execution_time_ms=execution_time_ms
        )

    def generate_summary(self, arch_name: str, results: List[TestCaseResult],
                        total_time_ms: int = 0) -> ArchTestSummary:
        """
        Generate a summary of test results.

        Args:
            arch_name: Name of the architecture
            results: List of test case results
            total_time_ms: Total execution time in milliseconds

        Returns:
            ArchTestSummary object
        """
        total_cases = len(results)
        matches = sum(1 for r in results if r.result == ComparisonResult.MATCH)
        mismatches = sum(1 for r in results if r.result == ComparisonResult.MISMATCH)
        command_failures = sum(1 for r in results if r.result == ComparisonResult.COMMAND_FAILURE)
        documentation_drifts = sum(1 for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT)

        return ArchTestSummary(
            arch_name=arch_name,
            total_cases=total_cases,
            matches=matches,
            mismatches=mismatches,
            command_failures=command_failures,
            documentation_drifts=documentation_drifts,
            execution_time_ms=total_time_ms,
            results=results
        )

    def get_failed_results(self, results: List[TestCaseResult],
                          include_drift: bool = True) -> List[TestCaseResult]:
        """
        Get list of failed test results.

        Args:
            results: List of test case results
            include_drift: Whether to include documentation drift as failure

        Returns:
            List of failed test case results
        """
        failed = [r for r in results if r.result in [
            ComparisonResult.MISMATCH,
            ComparisonResult.COMMAND_FAILURE
        ]]
        if include_drift:
            failed.extend([r for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT])
        return failed

    def format_result_detailed(self, result: TestCaseResult) -> str:
        """
        Format a test result for detailed display.

        Args:
            result: TestCaseResult to format

        Returns:
            Formatted string
        """
        lines = [f"Instruction: {result.hex_input}"]

        if result.expected_output:
            lines.append(f"  Expected:   {result.expected_output}")

        lines.append(f"  Robustone:  {result.robustone_output}")
        lines.append(f"  Cstool:     {result.cstool_output}")

        if result.note:
            lines.append(f"  Note:       {result.note}")

        if result.robustone_stderr:
            lines.append(f"  Robustone stderr: {result.robustone_stderr}")

        if result.cstool_stderr:
            lines.append(f"  Cstool stderr:    {result.cstool_stderr}")

        if result.execution_time_ms > 0:
            lines.append(f"  Execution time: {result.execution_time_ms}ms")

        return "\n".join(lines)