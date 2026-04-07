use std::str::FromStr;

use crate::error::ParseError;
use robustone_core::common::ArchitectureProfile;
use robustone_core::{
    ArchitectureCapability, all_architecture_capabilities, lookup_architecture_capability,
};

const MODE_BIG_ENDIAN: u32 = 0x100;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Architecture {
    capability: &'static ArchitectureCapability,
}

impl Architecture {
    fn new(capability: &'static ArchitectureCapability) -> Self {
        Self { capability }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        lookup_architecture_capability(input)
            .map(Self::new)
            .ok_or_else(|| {
                format!(
                    "Invalid <arch+mode>: {input}. Supported: {}",
                    supported_architecture_list()
                )
            })
    }

    pub fn name(&self) -> &'static str {
        self.capability.canonical_name
    }

    pub fn is_implemented(&self) -> bool {
        self.capability.decode_supported
    }

    pub fn implementation_status(&self) -> &'static str {
        self.capability.implementation_status()
    }

    pub fn category(&self) -> &'static str {
        self.capability.category
    }

    pub fn all_architectures() -> Vec<Self> {
        all_architecture_capabilities()
            .iter()
            .map(Self::new)
            .collect()
    }

    fn supports_modifier(&self, modifier: &str) -> bool {
        match self.name() {
            "riscv32" | "riscv64" | "riscv32e" => is_supported_riscv_modifier(modifier),
            "arm" | "armle" | "armbe" => {
                matches!(
                    modifier,
                    "thumb" | "m" | "v8" | "noregname" | "regalias" | "be" | "le"
                )
            }
            "thumb" => matches!(modifier, "m" | "v8" | "noregname" | "regalias"),
            "aarch64" | "aarch64be" => {
                matches!(modifier, "apple" | "noregname" | "regalias" | "be" | "le")
            }
            "x16" | "x32" | "x64" => matches!(modifier, "att" | "intel" | "masm" | "nasm"),
            "mips" | "mipsel" | "mips64" | "mips64el" => {
                matches!(
                    modifier,
                    "nofloat" | "ptr64" | "noregname" | "nodollar" | "be" | "le"
                )
            }
            "powerpc32" | "powerpc32be" | "powerpc64" | "powerpc64be" => {
                matches!(
                    modifier,
                    "aix"
                        | "booke"
                        | "maix"
                        | "msync"
                        | "qpx"
                        | "ps"
                        | "spe"
                        | "noregname"
                        | "percentage"
                        | "be"
                        | "le"
                )
            }
            "sparc" | "sparcle" | "sparc64" => matches!(modifier, "v9" | "be" | "le"),
            _ => false,
        }
    }

    fn apply_endianness_modifier(&self, modifier: &str) -> std::result::Result<Self, ParseError> {
        let canonical_name = match (self.name(), modifier) {
            ("arm" | "armbe" | "armle", "be") => "armbe",
            ("arm" | "armbe" | "armle", "le") => "armle",
            ("aarch64" | "aarch64be", "be") => "aarch64be",
            ("aarch64" | "aarch64be", "le") => "aarch64",
            ("mips", "be") | ("mips64", "be") | ("mipsel", "le") | ("mips64el", "le") => {
                self.name()
            }
            ("mips", "le") => "mipsel",
            ("mips64", "le") => "mips64el",
            ("mipsel", "be") => "mips",
            ("mips64el", "be") => "mips64",
            ("powerpc32" | "powerpc32be", "be") => "powerpc32be",
            ("powerpc32" | "powerpc32be", "le") => "powerpc32",
            ("powerpc64" | "powerpc64be", "be") => "powerpc64be",
            ("powerpc64" | "powerpc64be", "le") => "powerpc64",
            ("sparc" | "sparcle", "be") => "sparc",
            ("sparc" | "sparcle", "le") => "sparcle",
            ("sparc64", "be" | "le") => "sparc64",
            _ => return Err(ParseError::UnknownOption(modifier.to_string())),
        };

        lookup_architecture_capability(canonical_name)
            .map(Self::new)
            .ok_or_else(|| ParseError::UnknownArchitecture(canonical_name.to_string()))
    }

    pub fn default_mode(&self) -> u32 {
        0x0
    }
}

impl std::fmt::Debug for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Architecture").field(&self.name()).finish()
    }
}

impl FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Architecture specification holding the resolved architecture, mode flags, and modifiers.
#[derive(Clone)]
pub struct ArchitectureSpec {
    pub arch: Architecture,
    pub mode: u32,            // Capstone mode bitmask
    pub options: Vec<String>, // Architecture-specific option modifiers
}

impl ArchitectureSpec {
    /// Parses an architecture string, supporting `+`-separated modifiers.
    pub fn parse(input: &str) -> std::result::Result<Self, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let parts: Vec<&str> = input.split('+').collect();
        if parts.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let mut arch = Architecture::from_str(parts[0])
            .map_err(|_| ParseError::UnknownArchitecture(parts[0].to_string()))?;

        let mut mode = arch.default_mode();
        let mut options = Vec::new();

        for modifier in &parts[1..] {
            let canonical_modifier = normalize_modifier(modifier);
            if !arch.supports_modifier(&canonical_modifier) {
                return Err(ParseError::UnknownOption(modifier.to_string()));
            }

            if matches!(canonical_modifier.as_str(), "be" | "le") {
                arch = arch.apply_endianness_modifier(&canonical_modifier)?;
                mode = arch.default_mode() | endianness_mode_bits(&canonical_modifier);
                continue;
            }

            options.push(canonical_modifier);
        }

        Ok(ArchitectureSpec {
            arch,
            mode,
            options,
        })
    }

    pub fn riscv_profile(&self) -> Option<ArchitectureProfile> {
        let mut profile = match self.arch.name() {
            "riscv32" => ArchitectureProfile::riscv32gc(),
            "riscv64" => ArchitectureProfile::riscv64gc(),
            _ => return None,
        };

        let mut uses_explicit_profile = false;

        if self.has_option("a") {
            uses_explicit_profile = true;
        }
        if self.has_option("c") {
            uses_explicit_profile = true;
        }
        if self.has_option("fd") {
            uses_explicit_profile = true;
        }

        if uses_explicit_profile {
            profile.enabled_extensions.sort_unstable();
            profile.enabled_extensions.dedup();
            Some(profile)
        } else {
            None
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        self.options.iter().any(|candidate| candidate == option)
    }
}

impl std::fmt::Debug for ArchitectureSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch_name = self.arch.name();
        if self.options.is_empty() {
            write!(f, "ArchitectureSpec({arch_name})")
        } else {
            let options_str = self.options.join("+");
            write!(f, "ArchitectureSpec({arch_name}+{options_str})")
        }
    }
}

fn supported_architecture_list() -> String {
    all_architecture_capabilities()
        .iter()
        .map(|capability| capability.canonical_name)
        .collect::<Vec<_>>()
        .join(", ")
}

fn endianness_mode_bits(modifier: &str) -> u32 {
    match modifier {
        "be" => MODE_BIG_ENDIAN,
        _ => 0,
    }
}

fn is_supported_riscv_modifier(modifier: &str) -> bool {
    matches!(modifier, "a" | "c" | "fd" | "noalias" | "noaliascompressed")
}

fn normalize_modifier(modifier: &str) -> String {
    match modifier.to_lowercase().as_str() {
        "at&t" => "att".to_string(),
        "micro" => "m".to_string(),
        "big" => "be".to_string(),
        "little" => "le".to_string(),
        canonical => canonical.to_string(),
    }
}
