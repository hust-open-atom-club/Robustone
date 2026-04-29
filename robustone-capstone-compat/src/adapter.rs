//! External test adapter trait for compatibility testing.

use std::path::Path;

use robustone_isa::{ArchitectureBackend, DecodeProfile};

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
pub trait ExternalTestAdapter<B: ArchitectureBackend> {
    type Fixture;

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
