//! External test adapter trait for compatibility testing.

use std::path::Path;

use robustone_isa::{ArchitectureBackend, DecodeProfile};
use robustone_loongarch::backend::LoongArchBackend;

use crate::yaml::{CapstoneYaml, TestCase};

/// Errors encountered during compatibility testing.
#[derive(Debug)]
pub enum CompatError {
    Io(String),
    Parse(String),
    Config(String),
}

impl std::fmt::Display for CompatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompatError::Io(s) => write!(f, "io error: {}", s),
            CompatError::Parse(s) => write!(f, "parse error: {}", s),
            CompatError::Config(s) => write!(f, "config error: {}", s),
        }
    }
}

impl std::error::Error for CompatError {}

/// Expected detail information from an external test fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedDetail {
    pub mnemonic: String,
    pub operand_count: usize,
    pub groups: Vec<String>,
}

/// Adapter that bridges an external test format (Capstone YAML, LLVM MC,
/// binutils objdump) to the shared compatibility harness.
pub trait CapstoneArchAdapter<B: ArchitectureBackend> {
    type Fixture;

    /// Architecture name for decoder dispatch (e.g. "loongarch64", "riscv64").
    fn arch_name() -> &'static str;

    /// Map Capstone arch/options to a Robustone architecture name.
    fn map_arch_mode(arch: &str, options: &[String]) -> Option<&'static str>;

    /// Return the directory within `third_party/capstone/tests/MC/` where
    /// YAML test files for this architecture are stored.
    fn yaml_test_dir() -> &'static str;

    /// Load test fixtures from a file or directory.
    fn load_fixtures(path: &Path) -> Result<Vec<Self::Fixture>, CompatError>;

    /// Return the raw input bytes for a fixture.
    fn input_bytes(fixture: &Self::Fixture) -> &[u8];

    /// Return the expected disassembly text, if any.
    fn expected_text(fixture: &Self::Fixture) -> Option<&str>;

    /// Return the expected detail information, if any.
    fn expected_detail(fixture: &Self::Fixture) -> Option<ExpectedDetail>;

    /// Return the decode profile that should be used for this fixture.
    fn profile_for_fixture(fixture: &Self::Fixture) -> DecodeProfile<B>;

    /// Normalize expected text before comparison.
    fn normalize_expected(text: &str) -> String;

    /// Normalize actual text before comparison.
    fn normalize_actual(text: &str) -> String;
}

/// Concrete adapter for Capstone YAML test files targeting LoongArch.
pub struct CapstoneLoongArchYaml;

impl CapstoneArchAdapter<LoongArchBackend> for CapstoneLoongArchYaml {
    type Fixture = TestCase;

    fn arch_name() -> &'static str {
        "loongarch64"
    }

    fn map_arch_mode(arch: &str, options: &[String]) -> Option<&'static str> {
        match arch {
            "CS_ARCH_LOONGARCH" => {
                if options.contains(&"CS_MODE_LOONGARCH64".to_string()) {
                    Some("loongarch64")
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn yaml_test_dir() -> &'static str {
        "LoongArch"
    }

    fn load_fixtures(path: &Path) -> Result<Vec<Self::Fixture>, CompatError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CompatError::Io(format!("failed to read {:?}: {}", path, e)))?;
        let yaml: CapstoneYaml = serde_yaml::from_str(&content)
            .map_err(|e| CompatError::Parse(format!("failed to parse {:?}: {}", path, e)))?;
        Ok(yaml.test_cases)
    }

    fn input_bytes(fixture: &Self::Fixture) -> &[u8] {
        &fixture.input.bytes
    }

    fn expected_text(fixture: &Self::Fixture) -> Option<&str> {
        fixture.expected.insns.first().map(|i| i.asm_text.trim())
    }

    fn expected_detail(_fixture: &Self::Fixture) -> Option<ExpectedDetail> {
        // Capstone YAML does not carry detailed operand-level metadata in the format we parse.
        None
    }

    fn profile_for_fixture(_fixture: &Self::Fixture) -> DecodeProfile<LoongArchBackend> {
        LoongArchBackend::capstone_test_la64()
    }

    fn normalize_expected(text: &str) -> String {
        text.to_string()
    }

    fn normalize_actual(text: &str) -> String {
        text.to_string()
    }
}
