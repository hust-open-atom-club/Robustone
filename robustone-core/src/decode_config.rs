//! Strongly-typed decode configuration.
//!
//! This module replaces string-driven architecture/mode selection with
//! a type-safe `DecodeConfig` that is validated at the CLI boundary and
//! passed through the core decode path.

use crate::common::ArchitectureProfile;
use crate::utils::Endianness;

/// Architecture family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    RiscV,
    X86,
    Arm,
    AArch64,
    LoongArch,
}

impl Arch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Arch::RiscV => "riscv",
            Arch::X86 => "x86",
            Arch::Arm => "arm",
            Arch::AArch64 => "aarch64",
            Arch::LoongArch => "loongarch",
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Architecture-specific mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    // RISC-V
    Rv32,
    Rv64,
    Rv32E,
    // x86
    X16,
    X32,
    X64,
    // ARM
    Arm,
    ArmLE,
    ArmBE,
    Thumb,
    AArch64,
    AArch64BE,
    // LoongArch
    La32,
    La64,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Rv32 => "riscv32",
            Mode::Rv64 => "riscv64",
            Mode::Rv32E => "riscv32e",
            Mode::X16 => "x16",
            Mode::X32 => "x32",
            Mode::X64 => "x64",
            Mode::Arm => "arm",
            Mode::ArmLE => "armle",
            Mode::ArmBE => "armbe",
            Mode::Thumb => "thumb",
            Mode::AArch64 => "aarch64",
            Mode::AArch64BE => "aarch64be",
            Mode::La32 => "loongarch32",
            Mode::La64 => "loongarch64",
        }
    }

    /// Returns the architecture family this mode belongs to.
    pub fn arch(&self) -> Arch {
        match self {
            Mode::Rv32 | Mode::Rv64 | Mode::Rv32E => Arch::RiscV,
            Mode::X16 | Mode::X32 | Mode::X64 => Arch::X86,
            Mode::Arm | Mode::ArmLE | Mode::ArmBE | Mode::Thumb => Arch::Arm,
            Mode::AArch64 | Mode::AArch64BE => Arch::AArch64,
            Mode::La32 | Mode::La64 => Arch::LoongArch,
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Level of detail requested for decode output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailLevel {
    #[default]
    None,
    Basic,
    Full,
}

/// A set of enabled ISA extensions.
///
/// Stored as a list of canonical extension names (e.g. `"I"`, `"M"`, `"C"`).
/// Architecture-specific backends interpret these names according to their
/// own extension model.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FeatureSet {
    extensions: Vec<String>,
}

impl FeatureSet {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn from_extensions(extensions: &[&str]) -> Self {
        Self {
            extensions: extensions.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn push(&mut self, ext: impl Into<String>) {
        self.extensions.push(ext.into());
    }

    pub fn contains(&self, ext: &str) -> bool {
        self.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
    }

    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}

/// Strongly-typed configuration for a decode request.
///
/// `DecodeConfig` is produced at the CLI/API boundary and consumed by the
/// core dispatcher and architecture handlers. No string parsing of
/// architecture or mode should happen past this point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeConfig {
    pub arch: Arch,
    pub mode: Mode,
    pub endianness: Endianness,
    pub features: FeatureSet,
    pub detail: DetailLevel,
}

impl DecodeConfig {
    /// Create a minimal config for the given mode.
    ///
    /// Endianness and features default to architecture-typical values.
    pub fn new(mode: Mode) -> Self {
        let endianness = Self::default_endianness_for_mode(&mode);
        Self {
            arch: mode.arch(),
            mode,
            endianness,
            features: FeatureSet::new(),
            detail: DetailLevel::Full,
        }
    }

    /// Builder-style method to set features.
    pub fn with_features(mut self, features: FeatureSet) -> Self {
        self.features = features;
        self
    }

    /// Builder-style method to set detail level.
    pub fn with_detail(mut self, detail: DetailLevel) -> Self {
        self.detail = detail;
        self
    }

    /// Builder-style method to set endianness.
    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }

    /// Validate that the mode is compatible with the arch.
    pub fn validate(&self) -> Result<(), DecodeConfigError> {
        let expected_arch = self.mode.arch();
        if self.arch != expected_arch {
            return Err(DecodeConfigError::ModeArchMismatch {
                arch: self.arch.to_string(),
                mode: self.mode.to_string(),
            });
        }
        Ok(())
    }

    /// Return the canonical mode name string used by handlers.
    pub fn mode_name(&self) -> &'static str {
        self.mode.as_str()
    }

    fn default_endianness_for_mode(mode: &Mode) -> Endianness {
        match mode {
            Mode::ArmBE | Mode::AArch64BE => Endianness::Big,
            _ => Endianness::Little,
        }
    }
}

/// Errors that can occur when building or validating a `DecodeConfig`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeConfigError {
    ModeArchMismatch { arch: String, mode: String },
    UnknownArchitecture(String),
    UnknownMode(String),
}

impl std::fmt::Display for DecodeConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeConfigError::ModeArchMismatch { arch, mode } => {
                write!(f, "mode '{mode}' is not valid for architecture '{arch}'")
            }
            DecodeConfigError::UnknownArchitecture(s) => {
                write!(f, "unknown architecture: {s}")
            }
            DecodeConfigError::UnknownMode(s) => {
                write!(f, "unknown mode: {s}")
            }
        }
    }
}

impl std::error::Error for DecodeConfigError {}

/// Parse a canonical architecture token into a `DecodeConfig`.
///
/// This is the single point where CLI strings enter the typed core.
/// Every other part of the decode pipeline must use `DecodeConfig`.
pub fn parse_decode_config(token: &str) -> Result<DecodeConfig, DecodeConfigError> {
    let mode = match token.to_ascii_lowercase().as_str() {
        "riscv32" => Mode::Rv32,
        "riscv64" | "riscv" => Mode::Rv64,
        "riscv32e" => Mode::Rv32E,
        "x16" => Mode::X16,
        "x32" | "x86" | "i386" => Mode::X32,
        "x64" | "x86-64" | "x86_64" | "amd64" => Mode::X64,
        "arm" | "armv7" => Mode::Arm,
        "armle" => Mode::ArmLE,
        "armbe" => Mode::ArmBE,
        "thumb" => Mode::Thumb,
        "aarch64" | "arm64" => Mode::AArch64,
        "aarch64be" => Mode::AArch64BE,
        "loongarch" | "loongarch64" => Mode::La64,
        _ => return Err(DecodeConfigError::UnknownArchitecture(token.to_string())),
    };

    Ok(DecodeConfig::new(mode))
}

impl TryFrom<&ArchitectureProfile> for DecodeConfig {
    type Error = DecodeConfigError;

    fn try_from(profile: &ArchitectureProfile) -> Result<Self, Self::Error> {
        let mode = match profile.mode_name {
            "riscv32" => Mode::Rv32,
            "riscv64" => Mode::Rv64,
            "riscv32e" => Mode::Rv32E,
            "x16" => Mode::X16,
            "x32" => Mode::X32,
            "x64" => Mode::X64,
            "arm" => Mode::Arm,
            "armle" => Mode::ArmLE,
            "armbe" => Mode::ArmBE,
            "thumb" => Mode::Thumb,
            "aarch64" => Mode::AArch64,
            "aarch64be" => Mode::AArch64BE,
            "loongarch64" => Mode::La64,
            "loongarch32" => Mode::La32,
            _ => {
                return Err(DecodeConfigError::UnknownMode(
                    profile.mode_name.to_string(),
                ));
            }
        };

        let mut config = DecodeConfig::new(mode);
        config.endianness = profile.endianness;
        config.features = FeatureSet {
            extensions: profile
                .enabled_extensions
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        };
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_riscv64() {
        let config = parse_decode_config("riscv64").unwrap();
        assert_eq!(config.arch, Arch::RiscV);
        assert_eq!(config.mode, Mode::Rv64);
        assert_eq!(config.endianness, Endianness::Little);
    }

    #[test]
    fn test_parse_aarch64be_big_endian() {
        let config = parse_decode_config("aarch64be").unwrap();
        assert_eq!(config.arch, Arch::AArch64);
        assert_eq!(config.mode, Mode::AArch64BE);
        assert_eq!(config.endianness, Endianness::Big);
    }

    #[test]
    fn test_parse_unknown() {
        assert!(parse_decode_config("unknown").is_err());
    }

    #[test]
    fn test_config_validate_ok() {
        let config = DecodeConfig::new(Mode::Rv32);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_feature_set_contains() {
        let fs = FeatureSet::from_extensions(&["I", "M", "C"]);
        assert!(fs.contains("I"));
        assert!(fs.contains("m"));
        assert!(!fs.contains("V"));
    }

    #[test]
    fn test_profile_to_decode_config() {
        let profile = ArchitectureProfile::riscv64gc();
        let config = DecodeConfig::try_from(&profile).unwrap();
        assert_eq!(config.arch, Arch::RiscV);
        assert_eq!(config.mode, Mode::Rv64);
        assert!(config.features.contains("C"));
    }
}
