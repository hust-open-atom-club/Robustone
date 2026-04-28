//! Test harness that drives Robustone against Capstone YAML oracles.

use std::fs;
use std::path::Path;

use robustone_core::ArchitectureDispatcher;
use robustone_core::types::Instruction;

use crate::yaml::{CapstoneYaml, ExpectedInsn, TestCase};

/// Maps a Capstone arch/options pair to a Robustone architecture name.
///
/// Returns `None` if the combination is not yet supported by the harness.
pub fn map_arch_mode(arch: &str, options: &[String]) -> Option<&'static str> {
    match arch {
        "CS_ARCH_LOONGARCH" => {
            if options.contains(&"CS_MODE_LOONGARCH64".to_string()) {
                Some("loongarch64")
            } else {
                // LA32 not yet supported by the new backend harness.
                None
            }
        }
        _ => None,
    }
}

/// Runs a single YAML test case through Robustone.
///
/// On mismatch returns a descriptive error string; on success returns `Ok(())`.
pub fn run_test_case(dispatcher: &ArchitectureDispatcher, case: &TestCase) -> Result<(), String> {
    let arch_name = map_arch_mode(&case.input.arch, &case.input.options).ok_or_else(|| {
        format!(
            "unsupported arch/options: {} / {:?}",
            case.input.arch, case.input.options
        )
    })?;

    let bytes = &case.input.bytes;
    let (instruction, _size) = dispatcher
        .disassemble_bytes(bytes, arch_name, 0x1000)
        .map_err(|e| format!("disassembly error: {:?}", e))?;

    let expected = case
        .expected
        .insns
        .first()
        .ok_or("expected.insns is empty")?;

    compare_instruction(&instruction, expected)
}

/// Compare a decoded [`Instruction`] against an [`ExpectedInsn`].
pub fn compare_instruction(
    instruction: &Instruction,
    expected: &ExpectedInsn,
) -> Result<(), String> {
    let actual = format_instruction(instruction);
    let expected_trimmed = expected.asm_text.trim();

    if actual == expected_trimmed {
        Ok(())
    } else {
        Err(format!(
            "mismatch: expected \"{}\", got \"{}\"",
            expected_trimmed, actual
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

/// Parse a single YAML file and return all test cases.
pub fn parse_yaml_file(path: &Path) -> Result<Vec<TestCase>, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read {:?}: {}", path, e))?;
    let yaml: CapstoneYaml =
        serde_yaml::from_str(&content).map_err(|e| format!("failed to parse {:?}: {}", path, e))?;
    Ok(yaml.test_cases)
}

/// Run every test case in a single YAML file.
///
/// Returns a vector where each element corresponds to the test case at the
/// same index. `Ok(())` means the case passed; `Err(String)` carries the
/// failure reason.
pub fn run_yaml_file(dispatcher: &ArchitectureDispatcher, path: &Path) -> Vec<Result<(), String>> {
    let cases = match parse_yaml_file(path) {
        Ok(c) => c,
        Err(e) => return vec![Err(e)],
    };
    cases
        .iter()
        .map(|case| run_test_case(dispatcher, case))
        .collect()
}

/// Recursively discover all `.yaml` files under `dir`.
pub fn discover_yaml_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("failed to read dir {:?}: {}", dir, e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(discover_yaml_files(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            files.push(path);
        }
    }
    Ok(files)
}

/// Alias for the per-case result used by [`run_yaml_dir`].
pub type YamlCaseResult = (std::path::PathBuf, usize, Result<(), String>);

/// Run every YAML file discovered under `dir`.
///
/// Returns a flat list of `(file_path, case_index, result)` tuples.
pub fn run_yaml_dir(
    dispatcher: &ArchitectureDispatcher,
    dir: &Path,
) -> Result<Vec<YamlCaseResult>, String> {
    let files = discover_yaml_files(dir)?;
    let mut results = Vec::new();
    for file in files {
        let cases = run_yaml_file(dispatcher, &file);
        for (idx, res) in cases.into_iter().enumerate() {
            results.push((file.clone(), idx, res));
        }
    }
    Ok(results)
}
