use std::str::FromStr;

use crate::error::ParseError;
use robustone_core::common::ArchitectureProfile;
use robustone_core::{
    all_architecture_capabilities, canonical_architecture_name, lookup_architecture_capability,
};

const MODE_BIG_ENDIAN: u32 = 0x100;

#[derive(Debug, Clone, PartialEq)]
pub enum Architecture {
    // RISC-V variants
    Riscv32,
    Riscv64,
    Riscv32E,

    // 32-bit ARM variants
    Arm,
    ArmLE,
    ArmBE,
    Thumb,

    // 64-bit ARM variants
    Aarch64,
    Aarch64BE,

    // x86 family
    X86_16,
    X86_32,
    X86_64,

    // MIPS family
    Mips,
    MipsEL,
    Mips64,
    MipsEL64,

    // PowerPC family
    PowerPC32,
    PowerPC32BE,
    PowerPC64,
    PowerPC64BE,

    // SPARC family
    Sparc,
    SparcLE,
    Sparc64,

    // Other architectures
    SystemZ,
    Xcore,
    M68k,
    Tms320c64x,
    M680x,
    Evm,
    Bpf,
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

        // Interpret the base architecture token.
        let mut arch = Architecture::from_str(parts[0])
            .map_err(|_| ParseError::UnknownArchitecture(parts[0].to_string()))?;

        let mut mode = arch.default_mode();
        let mut options = Vec::new();

        // Apply modifiers according to Capstone semantics.
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
        let mut profile = match self.arch {
            Architecture::Riscv32 => ArchitectureProfile::riscv32gc(),
            Architecture::Riscv64 => ArchitectureProfile::riscv64gc(),
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
            write!(f, "ArchitectureSpec({arch_name})",)
        } else {
            let options_str = self.options.join("+");
            write!(f, "ArchitectureSpec({arch_name}+{options_str})")
        }
    }
}

impl Architecture {
    fn supports_modifier(&self, modifier: &str) -> bool {
        match self {
            Architecture::Riscv32 | Architecture::Riscv64 | Architecture::Riscv32E => {
                is_supported_riscv_modifier(modifier)
            }
            Architecture::Arm | Architecture::ArmLE | Architecture::ArmBE => {
                matches!(
                    modifier,
                    "thumb" | "m" | "v8" | "noregname" | "regalias" | "be" | "le"
                )
            }
            Architecture::Thumb => matches!(modifier, "m" | "v8" | "noregname" | "regalias"),
            Architecture::Aarch64 | Architecture::Aarch64BE => {
                matches!(modifier, "apple" | "noregname" | "regalias" | "be" | "le")
            }
            Architecture::X86_16 | Architecture::X86_32 | Architecture::X86_64 => {
                matches!(modifier, "att" | "intel" | "masm" | "nasm")
            }
            Architecture::Mips
            | Architecture::MipsEL
            | Architecture::Mips64
            | Architecture::MipsEL64 => {
                matches!(
                    modifier,
                    "nofloat" | "ptr64" | "noregname" | "nodollar" | "be" | "le"
                )
            }
            Architecture::PowerPC32
            | Architecture::PowerPC32BE
            | Architecture::PowerPC64
            | Architecture::PowerPC64BE => {
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
            Architecture::Sparc | Architecture::SparcLE | Architecture::Sparc64 => {
                matches!(modifier, "v9" | "be" | "le")
            }
            _ => false,
        }
    }

    fn apply_endianness_modifier(&self, modifier: &str) -> std::result::Result<Self, ParseError> {
        match (self, modifier) {
            (Architecture::Arm | Architecture::ArmBE | Architecture::ArmLE, "be") => {
                Ok(Architecture::ArmBE)
            }
            (Architecture::Arm | Architecture::ArmBE | Architecture::ArmLE, "le") => {
                Ok(Architecture::ArmLE)
            }
            (Architecture::Aarch64 | Architecture::Aarch64BE, "be") => Ok(Architecture::Aarch64BE),
            (Architecture::Aarch64 | Architecture::Aarch64BE, "le") => Ok(Architecture::Aarch64),
            (Architecture::Mips, "be") | (Architecture::Mips64, "be") => Ok(self.clone()),
            (Architecture::MipsEL, "le") | (Architecture::MipsEL64, "le") => Ok(self.clone()),
            (Architecture::Mips, "le") => Ok(Architecture::MipsEL),
            (Architecture::Mips64, "le") => Ok(Architecture::MipsEL64),
            (Architecture::MipsEL, "be") => Ok(Architecture::Mips),
            (Architecture::MipsEL64, "be") => Ok(Architecture::Mips64),
            (Architecture::PowerPC32 | Architecture::PowerPC32BE, "be") => {
                Ok(Architecture::PowerPC32BE)
            }
            (Architecture::PowerPC32 | Architecture::PowerPC32BE, "le") => {
                Ok(Architecture::PowerPC32)
            }
            (Architecture::PowerPC64 | Architecture::PowerPC64BE, "be") => {
                Ok(Architecture::PowerPC64BE)
            }
            (Architecture::PowerPC64 | Architecture::PowerPC64BE, "le") => {
                Ok(Architecture::PowerPC64)
            }
            (Architecture::Sparc | Architecture::SparcLE, "be") => Ok(Architecture::Sparc),
            (Architecture::Sparc | Architecture::SparcLE, "le") => Ok(Architecture::SparcLE),
            (Architecture::Sparc64, "be" | "le") => Ok(Architecture::Sparc64),
            _ => Err(ParseError::UnknownOption(modifier.to_string())),
        }
    }

    pub fn default_mode(&self) -> u32 {
        match self {
            Architecture::Riscv32 | Architecture::Riscv64 | Architecture::Riscv32E => 0x0,
            Architecture::Arm | Architecture::Aarch64 => 0x0,
            Architecture::X86_16 | Architecture::X86_32 | Architecture::X86_64 => 0x0,
            Architecture::Mips
            | Architecture::MipsEL
            | Architecture::Mips64
            | Architecture::MipsEL64 => 0x0,
            Architecture::PowerPC32
            | Architecture::PowerPC32BE
            | Architecture::PowerPC64
            | Architecture::PowerPC64BE => 0x0,
            Architecture::Sparc | Architecture::Sparc64 => 0x0,
            _ => 0x0,
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        canonical_architecture_name(input)
            .and_then(Self::from_canonical_name)
            .ok_or_else(|| {
                format!(
                    "Invalid <arch+mode>: {input}. Supported: riscv32, riscv64, riscv32e, arm, armle, armbe, thumb, aarch64, aarch64be, x16, x32, x64, mips, mipsel, mips64, mips64el, ppc, ppc32, ppc64, sparc, sparc64, systemz, and others"
                )
            })
    }
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

impl FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Architecture {
    fn from_canonical_name(name: &str) -> Option<Self> {
        match name {
            "riscv32" => Some(Architecture::Riscv32),
            "riscv64" => Some(Architecture::Riscv64),
            "riscv32e" => Some(Architecture::Riscv32E),
            "arm" => Some(Architecture::Arm),
            "armle" => Some(Architecture::ArmLE),
            "armbe" => Some(Architecture::ArmBE),
            "thumb" => Some(Architecture::Thumb),
            "aarch64" => Some(Architecture::Aarch64),
            "aarch64be" => Some(Architecture::Aarch64BE),
            "x16" => Some(Architecture::X86_16),
            "x32" => Some(Architecture::X86_32),
            "x64" => Some(Architecture::X86_64),
            "mips" => Some(Architecture::Mips),
            "mipsel" => Some(Architecture::MipsEL),
            "mips64" => Some(Architecture::Mips64),
            "mips64el" => Some(Architecture::MipsEL64),
            "powerpc32" => Some(Architecture::PowerPC32),
            "powerpc32be" => Some(Architecture::PowerPC32BE),
            "powerpc64" => Some(Architecture::PowerPC64),
            "powerpc64be" => Some(Architecture::PowerPC64BE),
            "sparc" => Some(Architecture::Sparc),
            "sparcle" => Some(Architecture::SparcLE),
            "sparc64" => Some(Architecture::Sparc64),
            "systemz" => Some(Architecture::SystemZ),
            "xcore" => Some(Architecture::Xcore),
            "m68k" => Some(Architecture::M68k),
            "tms320c64x" => Some(Architecture::Tms320c64x),
            "m680x" => Some(Architecture::M680x),
            "evm" => Some(Architecture::Evm),
            "bpf" => Some(Architecture::Bpf),
            _ => None,
        }
    }

    fn capability(&self) -> &'static robustone_core::ArchitectureCapability {
        lookup_architecture_capability(self.name()).expect("canonical architecture must exist")
    }

    pub fn name(&self) -> &'static str {
        match self {
            // RISC-V
            Architecture::Riscv32 => "riscv32",
            Architecture::Riscv64 => "riscv64",
            Architecture::Riscv32E => "riscv32e",

            // ARM
            Architecture::Arm => "arm",
            Architecture::ArmLE => "armle",
            Architecture::ArmBE => "armbe",
            Architecture::Thumb => "thumb",

            // AArch64
            Architecture::Aarch64 => "aarch64",
            Architecture::Aarch64BE => "aarch64be",

            // x86
            Architecture::X86_16 => "x16",
            Architecture::X86_32 => "x32",
            Architecture::X86_64 => "x64",

            // MIPS
            Architecture::Mips => "mips",
            Architecture::MipsEL => "mipsel",
            Architecture::Mips64 => "mips64",
            Architecture::MipsEL64 => "mips64el",

            // PowerPC
            Architecture::PowerPC32 => "powerpc32",
            Architecture::PowerPC32BE => "powerpc32be",
            Architecture::PowerPC64 => "powerpc64",
            Architecture::PowerPC64BE => "powerpc64be",

            // SPARC
            Architecture::Sparc => "sparc",
            Architecture::SparcLE => "sparcle",
            Architecture::Sparc64 => "sparc64",

            // Other architectures
            Architecture::SystemZ => "systemz",
            Architecture::Xcore => "xcore",
            Architecture::M68k => "m68k",
            Architecture::Tms320c64x => "tms320c64x",
            Architecture::M680x => "m680x",
            Architecture::Evm => "evm",
            Architecture::Bpf => "bpf",
        }
    }

    pub fn is_implemented(&self) -> bool {
        self.capability().decode_supported
    }

    pub fn implementation_status(&self) -> &'static str {
        self.capability().implementation_status()
    }

    pub fn category(&self) -> &'static str {
        self.capability().category
    }

    pub fn all_architectures() -> Vec<Self> {
        all_architecture_capabilities()
            .iter()
            .filter_map(|capability| Self::from_canonical_name(capability.canonical_name))
            .collect()
    }
}
