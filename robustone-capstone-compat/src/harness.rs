//! Test harness that drives Robustone against Capstone YAML oracles.

use std::fs;
use std::path::Path;

use crate::adapter::CapstoneArchAdapter;
use crate::xfail::XfailRegistry;
use crate::yaml::{ExpectedInsn, TestCase};
use robustone_core::ArchitectureDispatcher;
use robustone_core::types::Instruction;
use robustone_isa::ArchitectureBackend;

/// Result of running a single compatibility test case.
#[derive(Debug)]
pub enum TestResult {
    Pass,
    /// Known expected failure (xfail). Carries reason and detail message.
    Xfail(String, String),
    /// Unsupported architecture or mode.
    Unsupported(String),
    /// Unexpected mismatch.
    Fail(String),
}

/// Runs a single YAML test case through Robustone.
///
/// Returns a [`TestResult`] describing the outcome.
pub fn run_test_case<A: CapstoneArchAdapter<B, Fixture = TestCase>, B: ArchitectureBackend>(
    dispatcher: &ArchitectureDispatcher,
    case: &TestCase,
    xfail: &XfailRegistry,
) -> TestResult {
    let arch_name = match A::map_arch_mode(&case.input.arch, &case.input.options) {
        Some(name) => name,
        None => {
            return TestResult::Unsupported(format!(
                "unsupported arch/options: {} / {:?}",
                case.input.arch, case.input.options
            ));
        }
    };

    let bytes = &case.input.bytes;
    let (instruction, _size) = match dispatcher.disassemble_bytes(bytes, arch_name, 0x0) {
        Ok(res) => res,
        Err(e) => return TestResult::Fail(format!("disassembly error: {:?}", e)),
    };

    let expected = match case.expected.insns.first() {
        Some(e) => e,
        None => return TestResult::Fail("expected.insns is empty".into()),
    };

    match compare_instruction::<A, B>(&instruction, expected) {
        Ok(()) => TestResult::Pass,
        Err(msg) => {
            if let Some(entry) = xfail.match_error(&msg) {
                TestResult::Xfail(entry.reason.clone(), msg)
            } else {
                TestResult::Fail(msg)
            }
        }
    }
}

/// Compare a decoded [`Instruction`] against an [`ExpectedInsn`].
pub fn compare_instruction<
    A: CapstoneArchAdapter<B, Fixture = TestCase>,
    B: ArchitectureBackend,
>(
    instruction: &Instruction,
    expected: &ExpectedInsn,
) -> Result<(), String> {
    let actual = format_instruction(instruction);
    let expected_text = expected.asm_text.trim();

    let normalized_actual = A::normalize_actual(&actual);
    let normalized_expected = A::normalize_expected(expected_text);

    if normalized_actual == normalized_expected {
        Ok(())
    } else {
        Err(format!(
            "mismatch: expected \"{}\" got \"{}\"",
            normalized_expected, normalized_actual
        ))
    }
}

/// Format a Robustone [`Instruction`] the same way Capstone YAML records it.
fn format_instruction(instruction: &Instruction) -> String {
    let mut s = instruction.mnemonic.clone();
    if !instruction.operands.is_empty() {
        s.push(' ');
        s.push_str(&instruction.operands);
    }
    s
}

/// Count results by category.
pub fn count_results(results: &[TestResult]) -> (usize, usize, usize, usize) {
    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut xfail = 0usize;
    let mut unsupported = 0usize;
    for r in results {
        match r {
            TestResult::Pass => pass += 1,
            TestResult::Xfail(_, _) => xfail += 1,
            TestResult::Unsupported(_) => unsupported += 1,
            TestResult::Fail(_) => fail += 1,
        }
    }
    (pass, fail, xfail, unsupported)
}

/// Run every test case in a single YAML file.
///
/// Returns a vector where each element corresponds to the test case at the
/// same index.
pub fn run_yaml_file<A: CapstoneArchAdapter<B, Fixture = TestCase>, B: ArchitectureBackend>(
    dispatcher: &ArchitectureDispatcher,
    path: &Path,
    xfail: &XfailRegistry,
) -> Vec<TestResult> {
    let cases = match A::load_fixtures(path) {
        Ok(c) => c,
        Err(e) => {
            return vec![TestResult::Fail(format!(
                "failed to load {:?}: {}",
                path, e
            ))];
        }
    };
    cases
        .iter()
        .map(|case| run_test_case::<A, B>(dispatcher, case, xfail))
        .collect()
}

/// Recursively discover all `.yaml` files under `dir`.
pub fn discover_yaml_files_recursive(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("failed to read dir {:?}: {}", dir, e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(discover_yaml_files_recursive(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            files.push(path);
        }
    }
    Ok(files)
}

/// Alias for the per-case result used by [`run_yaml_dir`].
pub type YamlCaseResult = (std::path::PathBuf, usize, TestResult);

/// Run every YAML file discovered under `dir`.
///
/// Returns a flat list of `(file_path, case_index, result)` tuples.
pub fn run_yaml_dir<A: CapstoneArchAdapter<B, Fixture = TestCase>, B: ArchitectureBackend>(
    dispatcher: &ArchitectureDispatcher,
    dir: &Path,
    xfail: &XfailRegistry,
) -> Result<Vec<YamlCaseResult>, String> {
    let files = discover_yaml_files_recursive(dir)?;
    let mut results = Vec::new();
    for file in files {
        let cases = run_yaml_file::<A, B>(dispatcher, &file, xfail);
        for (idx, res) in cases.into_iter().enumerate() {
            results.push((file.clone(), idx, res));
        }
    }
    Ok(results)
}
