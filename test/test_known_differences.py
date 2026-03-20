import tempfile
import textwrap
import unittest
from pathlib import Path
import sys

TEST_ROOT = Path(__file__).parent
sys.path.insert(0, str(TEST_ROOT))
sys.path.insert(0, str(TEST_ROOT / "core"))

# pylint: disable=wrong-import-position
from comparator import ComparisonResult, TestCaseResult
from test_runner import TestRunner, _parse_known_differences_fallback


class KnownDifferenceTests(unittest.TestCase):
    def test_active_known_difference_is_honored(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            whitelist = repo_root / "tests" / "differential"
            whitelist.mkdir(parents=True, exist_ok=True)
            (whitelist / "known-differences.toml").write_text(
                textwrap.dedent(
                    """
                    [[difference]]
                    arch = "riscv32"
                    hex = "deadbeef"
                    surface = "text"
                    reason = "accepted parity gap"
                    active = true
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            runner = TestRunner(repo_root=repo_root)
            result = TestCaseResult(
                hex_input="deadbeef",
                result=ComparisonResult.MISMATCH,
                expected_output="expected",
                robustone_output="robustone",
                cstool_output="cstool",
                note="",
            )

            updated = runner.apply_known_difference("riscv32", result)
            self.assertEqual(updated.result, ComparisonResult.MATCH)
            self.assertIn("known-difference", updated.note)

    def test_documentation_drift_is_not_masked(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            whitelist = repo_root / "tests" / "differential"
            whitelist.mkdir(parents=True, exist_ok=True)
            (whitelist / "known-differences.toml").write_text(
                textwrap.dedent(
                    """
                    [[difference]]
                    arch = "riscv32"
                    hex = "deadbeef"
                    surface = "text"
                    reason = "accepted parity gap"
                    active = true
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            runner = TestRunner(repo_root=repo_root)
            result = TestCaseResult(
                hex_input="deadbeef",
                result=ComparisonResult.DOCUMENTATION_DRIFT,
                expected_output="expected",
                robustone_output="robustone",
                cstool_output="cstool",
                note="",
            )

            updated = runner.apply_known_difference("riscv32", result)
            self.assertEqual(updated.result, ComparisonResult.DOCUMENTATION_DRIFT)

    def test_command_failure_is_not_masked(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            whitelist = repo_root / "tests" / "differential"
            whitelist.mkdir(parents=True, exist_ok=True)
            (whitelist / "known-differences.toml").write_text(
                textwrap.dedent(
                    """
                    [[difference]]
                    arch = "riscv32"
                    hex = "deadbeef"
                    surface = "text"
                    reason = "accepted parity gap"
                    active = true
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            runner = TestRunner(repo_root=repo_root)
            result = TestCaseResult(
                hex_input="deadbeef",
                result=ComparisonResult.COMMAND_FAILURE,
                expected_output="expected",
                robustone_output="robustone",
                cstool_output="cstool",
                note="",
            )

            updated = runner.apply_known_difference("riscv32", result)
            self.assertEqual(updated.result, ComparisonResult.COMMAND_FAILURE)

    def test_fallback_known_difference_parser_accepts_surface_entries(self):
        data = _parse_known_differences_fallback(
            textwrap.dedent(
                """
                [[difference]]
                arch = "riscv32"
                hex = "deadbeef"
                surface = "semantic_detail"
                reason = "accepted semantic gap"
                active = true
                """
            )
        )

        self.assertEqual(len(data["difference"]), 1)
        self.assertEqual(data["difference"][0]["surface"], "semantic_detail")
        self.assertTrue(data["difference"][0]["active"])


if __name__ == "__main__":
    unittest.main()
