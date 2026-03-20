"""
Output comparison functionality for the test framework.
"""

from dataclasses import dataclass, field
from enum import Enum
import json
import re
from typing import Any, Dict, List

from utils import normalize_output


class ComparisonResult(Enum):
    """Result of output comparison."""

    MATCH = "match"
    MISMATCH = "mismatch"
    COMMAND_FAILURE = "command_failure"
    DOCUMENTATION_DRIFT = "documentation_drift"


class ComparisonSurface(Enum):
    """Independent comparison surfaces within the differential harness."""

    TEXT = "text"
    SEMANTIC_DETAIL = "semantic_detail"


@dataclass
class SurfaceComparison:
    """Comparison result for one explicit surface."""

    surface: ComparisonSurface
    matched: bool
    robustone_value: str
    cstool_value: str


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
    surface_results: List[SurfaceComparison] = field(default_factory=list)


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


_RISCV_GPR_NAMES = [
    "zero",
    "ra",
    "sp",
    "gp",
    "tp",
    "t0",
    "t1",
    "t2",
    "s0",
    "s1",
    "a0",
    "a1",
    "a2",
    "a3",
    "a4",
    "a5",
    "a6",
    "a7",
    "s2",
    "s3",
    "s4",
    "s5",
    "s6",
    "s7",
    "s8",
    "s9",
    "s10",
    "s11",
    "t3",
    "t4",
    "t5",
    "t6",
]

_RISCV_FPR_NAMES = [
    "ft0",
    "ft1",
    "ft2",
    "ft3",
    "ft4",
    "ft5",
    "ft6",
    "ft7",
    "fs0",
    "fs1",
    "fa0",
    "fa1",
    "fa2",
    "fa3",
    "fa4",
    "fa5",
    "fa6",
    "fa7",
    "fs2",
    "fs3",
    "fs4",
    "fs5",
    "fs6",
    "fs7",
    "fs8",
    "fs9",
    "fs10",
    "fs11",
    "ft8",
    "ft9",
    "ft10",
    "ft11",
]

_RISCV_REGISTER_IDS: Dict[str, int] = {"fp": 8}
_RISCV_REGISTER_IDS.update({f"x{idx}": idx for idx in range(32)})
_RISCV_REGISTER_IDS.update({f"f{idx}": 32 + idx for idx in range(32)})
_RISCV_REGISTER_IDS.update({name: idx for idx, name in enumerate(_RISCV_GPR_NAMES)})
_RISCV_REGISTER_IDS.update(
    {name: 32 + idx for idx, name in enumerate(_RISCV_FPR_NAMES)}
)

_RISCV_CSR_IDS = {
    "sstatus": 0x100,
    "satp": 0x180,
    "mtvec": 0x305,
    "mcause": 0x342,
    "cycle": 0xC00,
}


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

            return robustone_out == cstool_out

        return normalize_output(robustone_out) == normalize_output(cstool_out)

    def compare_text_surface(
        self, robustone_out: str, cstool_out: str
    ) -> SurfaceComparison:
        """Compare the human-readable text surface."""
        return SurfaceComparison(
            surface=ComparisonSurface.TEXT,
            matched=self.compare_outputs(robustone_out, cstool_out),
            robustone_value=robustone_out,
            cstool_value=cstool_out,
        )

    def compare_semantic_detail_surface(
        self, robustone_json: str, cstool_detail: str
    ) -> SurfaceComparison:
        """Compare the detail/semantic surface."""
        robustone_record = self._normalize_robustone_semantic_output(robustone_json)
        cstool_record = self._normalize_cstool_semantic_output(cstool_detail)

        return SurfaceComparison(
            surface=ComparisonSurface.SEMANTIC_DETAIL,
            matched=robustone_record == cstool_record,
            robustone_value=json.dumps(robustone_record, sort_keys=True),
            cstool_value=json.dumps(cstool_record, sort_keys=True),
        )

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

    def classify_result(
        self,
        expected: str,
        cstool_out: str,
        surface_results: List[SurfaceComparison],
        command_failed: bool,
    ) -> ComparisonResult:
        """
        Classify the result of a test case.

        Args:
            expected: Expected output from documentation
            cstool_out: Text output from cstool
            surface_results: Per-surface comparison results
            command_failed: Whether any surface command failed

        Returns:
            ComparisonResult classification
        """
        if command_failed:
            return ComparisonResult.COMMAND_FAILURE

        if expected and self.check_documentation_drift(expected, cstool_out):
            return ComparisonResult.DOCUMENTATION_DRIFT

        if all(surface.matched for surface in surface_results):
            return ComparisonResult.MATCH

        return ComparisonResult.MISMATCH

    def create_test_result(
        self,
        hex_input: str,
        expected: str,
        robustone_out: str,
        cstool_out: str,
        note: str,
        robustone_exit_code: int = 0,
        cstool_exit_code: int = 0,
        robustone_stderr: str = "",
        cstool_stderr: str = "",
        execution_time_ms: int = 0,
        robustone_semantic_out: str = "",
        cstool_semantic_out: str = "",
        robustone_semantic_exit_code: int = 0,
        cstool_semantic_exit_code: int = 0,
        robustone_semantic_stderr: str = "",
        cstool_semantic_stderr: str = "",
    ) -> TestCaseResult:
        """
        Create a TestCaseResult from test execution data.
        """
        surface_results = [self.compare_text_surface(robustone_out, cstool_out)]
        command_failed = any(
            code != 0
            for code in (
                robustone_exit_code,
                cstool_exit_code,
                robustone_semantic_exit_code,
                cstool_semantic_exit_code,
            )
        )

        semantic_parse_error = ""
        if not command_failed:
            try:
                surface_results.append(
                    self.compare_semantic_detail_surface(
                        robustone_semantic_out, cstool_semantic_out
                    )
                )
            except ValueError as exc:
                semantic_parse_error = f"semantic_detail parse failed: {exc}"
                command_failed = True

        combined_robustone_stderr = self._combine_streams(
            robustone_stderr, robustone_semantic_stderr, "semantic_detail"
        )
        combined_cstool_stderr = self._combine_streams(
            cstool_stderr, cstool_semantic_stderr, "semantic_detail"
        )
        if semantic_parse_error:
            combined_robustone_stderr = self._append_message(
                combined_robustone_stderr, semantic_parse_error
            )

        result = self.classify_result(
            expected=expected,
            cstool_out=cstool_out,
            surface_results=surface_results,
            command_failed=command_failed,
        )

        return TestCaseResult(
            hex_input=hex_input,
            result=result,
            expected_output=expected,
            robustone_output=robustone_out,
            cstool_output=cstool_out,
            note=note,
            robustone_exit_code=robustone_exit_code or robustone_semantic_exit_code,
            cstool_exit_code=cstool_exit_code or cstool_semantic_exit_code,
            robustone_stderr=combined_robustone_stderr,
            cstool_stderr=combined_cstool_stderr,
            execution_time_ms=execution_time_ms,
            surface_results=surface_results,
        )

    def generate_summary(
        self, arch_name: str, results: List[TestCaseResult], total_time_ms: int = 0
    ) -> ArchTestSummary:
        """
        Generate a summary of test results.
        """
        total_cases = len(results)
        matches = sum(1 for r in results if r.result == ComparisonResult.MATCH)
        mismatches = sum(1 for r in results if r.result == ComparisonResult.MISMATCH)
        command_failures = sum(
            1 for r in results if r.result == ComparisonResult.COMMAND_FAILURE
        )
        documentation_drifts = sum(
            1 for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT
        )

        return ArchTestSummary(
            arch_name=arch_name,
            total_cases=total_cases,
            matches=matches,
            mismatches=mismatches,
            command_failures=command_failures,
            documentation_drifts=documentation_drifts,
            execution_time_ms=total_time_ms,
            results=results,
        )

    def get_failed_results(
        self, results: List[TestCaseResult], include_drift: bool = True
    ) -> List[TestCaseResult]:
        """
        Get list of failed test results.
        """
        failed = [
            r
            for r in results
            if r.result in [ComparisonResult.MISMATCH, ComparisonResult.COMMAND_FAILURE]
        ]
        if include_drift:
            failed.extend(
                [r for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT]
            )
        return failed

    def format_result_detailed(self, result: TestCaseResult) -> str:
        """
        Format a test result for detailed display.
        """
        lines = [f"Instruction: {result.hex_input}"]

        if result.expected_output:
            lines.append(f"  Expected:   {result.expected_output}")

        lines.append(f"  Robustone:  {result.robustone_output}")
        lines.append(f"  Cstool:     {result.cstool_output}")

        for surface in result.surface_results:
            if surface.matched:
                continue
            lines.append(f"  Surface {surface.surface.value}: mismatch")
            lines.append(f"    Robustone: {surface.robustone_value}")
            lines.append(f"    Cstool:    {surface.cstool_value}")

        if result.note:
            lines.append(f"  Note:       {result.note}")

        if result.robustone_stderr:
            lines.append(f"  Robustone stderr: {result.robustone_stderr}")

        if result.cstool_stderr:
            lines.append(f"  Cstool stderr:    {result.cstool_stderr}")

        if result.execution_time_ms > 0:
            lines.append(f"  Execution time: {result.execution_time_ms}ms")

        return "\n".join(lines)

    def _normalize_robustone_semantic_output(self, output: str) -> Dict[str, Any]:
        try:
            payload = json.loads(output)
        except json.JSONDecodeError as exc:
            raise ValueError(f"invalid robustone JSON: {exc}") from exc

        instructions = payload.get("instructions")
        if not isinstance(instructions, list) or len(instructions) != 1:
            raise ValueError(
                "robustone semantic surface must contain exactly one instruction"
            )

        instruction = instructions[0]
        decoded = instruction.get("decoded")
        if not isinstance(decoded, dict):
            raise ValueError("robustone semantic surface is missing decoded IR")

        operands = []
        for operand in decoded.get("operands", []):
            normalized = self._normalize_robustone_operand(operand)
            if normalized is not None:
                operands.append(normalized)

        return {
            "opcode_id": self._normalize_opcode_name(
                decoded.get("opcode_id")
                or decoded.get("mnemonic")
                or instruction.get("mnemonic")
            ),
            "operands": operands,
            "registers_read": [
                self._normalize_robustone_register(register)
                for register in decoded.get("registers_read", [])
            ],
            "registers_written": [
                self._normalize_robustone_register(register)
                for register in decoded.get("registers_written", [])
            ],
        }

    def _normalize_cstool_semantic_output(  # pylint: disable=too-many-branches
        self, output: str
    ) -> Dict[str, Any]:
        lines = [line.strip() for line in output.splitlines() if line.strip()]
        id_line = next((line for line in lines if line.startswith("ID:")), "")
        id_match = re.search(r"\(([^)]+)\)", id_line)
        if not id_match:
            raise ValueError("cstool semantic surface is missing opcode ID")

        operands: Dict[int, Dict[str, Any]] = {}
        for line in lines:
            operand_match = re.match(
                r"operands\[(\d+)\]\.type: ([A-Z]+)(?: = (.+))?$", line
            )
            if operand_match:
                index = int(operand_match.group(1))
                operand_type = operand_match.group(2)
                operand_value = operand_match.group(3)
                if operand_type == "REG":
                    operands[index] = {
                        "kind": "register",
                        "register_id": self._riscv_register_id(operand_value or ""),
                        "access": "",
                    }
                elif operand_type == "IMM":
                    operands[index] = {
                        "kind": "immediate",
                        "value": self._parse_numeric_value(operand_value or "0"),
                        "access": "",
                    }
                elif operand_type == "MEM":
                    operands[index] = {
                        "kind": "memory",
                        "base": None,
                        "displacement": 0,
                        "access": "",
                    }
                elif operand_type == "CSR":
                    operands[index] = {
                        "kind": "immediate",
                        "value": self._riscv_csr_id(operand_value or ""),
                        "access": "",
                    }
                else:
                    raise ValueError(
                        f"unsupported cstool operand type `{operand_type}`"
                    )
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.mem\.base: REG = (.+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(
                    index,
                    {"kind": "memory", "base": None, "displacement": 0, "access": ""},
                )["base"] = self._riscv_register_id(operand_match.group(2))
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.mem\.disp: (.+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(
                    index,
                    {"kind": "memory", "base": None, "displacement": 0, "access": ""},
                )["displacement"] = self._parse_numeric_value(operand_match.group(2))
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.access: ([A-Z| ]+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(index, {"kind": "unknown", "access": ""})[
                    "access"
                ] = operand_match.group(2).replace(" ", "")

        ordered_operands = []
        registers_read: List[int] = []
        registers_written: List[int] = []

        for index in sorted(operands):
            operand = operands[index]
            normalized_operand = self._normalize_cstool_operand(operand)
            ordered_operands.append(normalized_operand)

            access = operand.get("access", "")
            if normalized_operand["kind"] == "register":
                register_id = normalized_operand["register_id"]
                if "READ" in access:
                    registers_read.append(register_id)
                if "WRITE" in access:
                    registers_written.append(register_id)
            elif normalized_operand["kind"] == "memory":
                base = normalized_operand["base"]
                if base is not None:
                    registers_read.append(base)

        return {
            "opcode_id": self._normalize_opcode_name(id_match.group(1)),
            "operands": ordered_operands,
            "registers_read": registers_read,
            "registers_written": registers_written,
        }

    def _normalize_robustone_register(self, register: Dict[str, Any]) -> int:
        if register.get("architecture") != "riscv":
            raise ValueError(f"unexpected register architecture `{register}`")
        return int(register["id"])

    def _normalize_robustone_operand(
        self, operand: Dict[str, Any]
    ) -> Dict[str, Any] | None:
        kind = operand.get("kind")
        if kind == "register":
            return {
                "kind": "register",
                "register_id": self._normalize_robustone_register(operand["register"]),
            }
        if kind == "immediate":
            return {"kind": "immediate", "value": int(operand["value"])}
        if kind == "memory":
            base = operand.get("base")
            return {
                "kind": "memory",
                "base": (
                    None if base is None else self._normalize_robustone_register(base)
                ),
                "displacement": int(operand["displacement"]),
            }
        if kind == "text":
            return None

        raise ValueError(f"unsupported robustone operand kind `{kind}`")

    def _normalize_cstool_operand(self, operand: Dict[str, Any]) -> Dict[str, Any]:
        kind = operand["kind"]
        if kind == "register":
            return {"kind": "register", "register_id": operand["register_id"]}
        if kind == "immediate":
            return {"kind": "immediate", "value": operand["value"]}
        if kind == "memory":
            return {
                "kind": "memory",
                "base": operand["base"],
                "displacement": operand["displacement"],
            }

        raise ValueError(f"unsupported normalized cstool operand `{kind}`")

    def _normalize_opcode_name(self, opcode_id: Any) -> str:
        if opcode_id is None:
            raise ValueError("missing opcode identifier")
        return str(opcode_id).strip().lower().replace("_", ".")

    def _riscv_register_id(self, register_name: str) -> int:
        name = register_name.strip().lower()
        if name not in _RISCV_REGISTER_IDS:
            raise ValueError(f"unknown RISC-V register `{register_name}`")
        return _RISCV_REGISTER_IDS[name]

    def _riscv_csr_id(self, csr_name: str) -> int:
        name = csr_name.strip().lower()
        if name.startswith("0x"):
            return int(name, 16)
        if name not in _RISCV_CSR_IDS:
            raise ValueError(f"unknown RISC-V CSR `{csr_name}`")
        return _RISCV_CSR_IDS[name]

    def _parse_numeric_value(self, value: str) -> int:
        text = value.strip().lower()
        if not text:
            raise ValueError("empty numeric value")

        negative = text.startswith("-")
        if negative:
            text = text[1:]

        parsed = int(text, 16 if text.startswith("0x") else 10)
        if text.startswith("0x") and not negative and parsed >= (1 << 63):
            parsed -= 1 << 64

        return -parsed if negative else parsed

    def _combine_streams(self, primary: str, secondary: str, label: str) -> str:
        if not secondary:
            return primary
        secondary_block = f"{label}: {secondary}"
        return (
            primary
            if secondary_block in primary
            else self._append_message(primary, secondary_block)
        )

    def _append_message(self, current: str, message: str) -> str:
        if not current:
            return message
        if message in current:
            return current
        return f"{current} | {message}"
