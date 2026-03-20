import tempfile
import textwrap
import unittest
from pathlib import Path
import sys

TEST_ROOT = Path(__file__).parent
sys.path.insert(0, str(TEST_ROOT))
sys.path.insert(0, str(TEST_ROOT / "core"))

# pylint: disable=wrong-import-position
from comparator import ComparisonResult, ComparisonSurface, OutputComparator
from test_runner import TestRunner


ROBUSTONE_ADDI_JSON = """
{
  "instructions": [
    {
      "decoded": {
        "mnemonic": "addi",
        "opcode_id": "addi",
        "operands": [
          { "kind": "register", "register": { "architecture": "riscv", "id": 1 } },
          { "kind": "register", "register": { "architecture": "riscv", "id": 0 } },
          { "kind": "immediate", "value": 1 }
        ],
        "registers_read": [
          { "architecture": "riscv", "id": 0 }
        ],
        "registers_written": [
          { "architecture": "riscv", "id": 1 }
        ]
      }
    }
  ]
}
""".strip()

ROBUSTONE_BAD_JSON = """
{
  "instructions": [
    {
      "decoded": {
        "mnemonic": "addi",
        "opcode_id": "addi",
        "operands": [
          { "kind": "register", "register": { "architecture": "riscv", "id": 1 } },
          { "kind": "register", "register": { "architecture": "riscv", "id": 2 } },
          { "kind": "immediate", "value": 1 }
        ],
        "registers_read": [
          { "architecture": "riscv", "id": 2 }
        ],
        "registers_written": [
          { "architecture": "riscv", "id": 1 }
        ]
      }
    }
  ]
}
""".strip()

CSTOOL_ADDI_DETAIL = """
 0  93 00 10 00  li\t\t\tra, 1
\tID: 40 (addi)
\tIs alias: 0 (invalid) with REAL operand set
\top_count: 3
\t\toperands[0].type: REG = ra
\t\toperands[0].access: WRITE
\t\toperands[1].type: REG = zero
\t\toperands[1].access: READ
\t\toperands[2].type: IMM = 0x1
\t\toperands[2].access: READ
""".strip()


class DifferentialSurfaceTests(unittest.TestCase):
    def test_semantic_detail_surface_can_fail_independently(self):
        comparator = OutputComparator()
        result = comparator.create_test_result(
            hex_input="93001000",
            expected="",
            robustone_out="0  93 00 10 00  li\tra, 1",
            cstool_out="0  93 00 10 00  li\tra, 1",
            note="",
            robustone_semantic_out=ROBUSTONE_BAD_JSON,
            cstool_semantic_out=CSTOOL_ADDI_DETAIL,
        )

        self.assertEqual(result.result, ComparisonResult.MISMATCH)
        self.assertEqual(len(result.surface_results), 2)
        self.assertTrue(result.surface_results[0].matched)
        self.assertFalse(result.surface_results[1].matched)
        self.assertEqual(
            result.surface_results[1].surface, ComparisonSurface.SEMANTIC_DETAIL
        )

    def test_known_difference_only_masks_named_surface(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            whitelist = repo_root / "tests" / "differential"
            whitelist.mkdir(parents=True, exist_ok=True)
            (whitelist / "known-differences.toml").write_text(
                textwrap.dedent(
                    """
                    [[difference]]
                    arch = "riscv32"
                    hex = "93001000"
                    surface = "text"
                    reason = "accepted text alias drift"
                    active = true
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            comparator = OutputComparator()
            result = comparator.create_test_result(
                hex_input="93001000",
                expected="",
                robustone_out="0  93 00 10 00  addi\tx1, x0, 1",
                cstool_out="0  93 00 10 00  li\tra, 1",
                note="",
                robustone_semantic_out=ROBUSTONE_BAD_JSON,
                cstool_semantic_out=CSTOOL_ADDI_DETAIL,
            )

            runner = TestRunner(repo_root=repo_root, comparator=comparator)
            updated = runner.apply_known_difference("riscv32", result)

            self.assertEqual(updated.result, ComparisonResult.MISMATCH)
            text_surface, semantic_surface = updated.surface_results
            self.assertTrue(text_surface.matched)
            self.assertFalse(semantic_surface.matched)

    def test_known_difference_can_mask_only_semantic_surface(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            whitelist = repo_root / "tests" / "differential"
            whitelist.mkdir(parents=True, exist_ok=True)
            (whitelist / "known-differences.toml").write_text(
                textwrap.dedent(
                    """
                    [[difference]]
                    arch = "riscv32"
                    hex = "93001000"
                    surface = "semantic_detail"
                    reason = "accepted semantic gap"
                    active = true
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            comparator = OutputComparator()
            result = comparator.create_test_result(
                hex_input="93001000",
                expected="",
                robustone_out="0  93 00 10 00  li\tra, 1",
                cstool_out="0  93 00 10 00  li\tra, 1",
                note="",
                robustone_semantic_out=ROBUSTONE_BAD_JSON,
                cstool_semantic_out=CSTOOL_ADDI_DETAIL,
            )

            runner = TestRunner(repo_root=repo_root, comparator=comparator)
            updated = runner.apply_known_difference("riscv32", result)

            self.assertEqual(updated.result, ComparisonResult.MATCH)
            self.assertTrue(all(surface.matched for surface in updated.surface_results))


if __name__ == "__main__":
    unittest.main()
