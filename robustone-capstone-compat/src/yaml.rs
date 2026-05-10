//! Data structures for Capstone YAML test files.

use serde::Deserialize;

/// Top-level structure of a Capstone YAML test file.
#[derive(Debug, Deserialize)]
pub struct CapstoneYaml {
    pub test_cases: Vec<TestCase>,
}

/// A single test case within a YAML file.
#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub input: Input,
    pub expected: Expected,
}

/// Input specification: raw bytes, architecture, and mode flags.
#[derive(Debug, Deserialize)]
pub struct Input {
    pub bytes: Vec<u8>,
    pub arch: String,
    #[serde(default)]
    pub options: Vec<String>,
}

/// Expected output: list of decoded instructions.
#[derive(Debug, Deserialize)]
pub struct Expected {
    pub insns: Vec<ExpectedInsn>,
}

/// A single expected instruction.
#[derive(Debug, Deserialize)]
pub struct ExpectedInsn {
    pub asm_text: String,
}
