//! Architecture utilities for multi-architecture support.
//!
//! This module centralizes accepted architecture tokens and their exposed
//! repository capabilities so CLI parsing, docs, and version output can share
//! the same source of truth.

use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchitectureCapability {
    pub canonical_name: &'static str,
    pub category: &'static str,
    pub aliases: &'static [&'static str],
    pub parse_supported: bool,
    pub decode_supported: bool,
    pub detail_supported: bool,
    pub json_supported: bool,
}

impl ArchitectureCapability {
    pub fn matches(&self, token: &str) -> bool {
        let normalized = normalize_architecture_token(token);
        self.aliases.contains(&normalized.as_str())
    }

    pub fn implementation_status(&self) -> &'static str {
        if self.decode_supported { "✅" } else { "❌" }
    }
}

const RISCV32_ALIASES: &[&str] = &["riscv32"];
const RISCV64_ALIASES: &[&str] = &["riscv64", "riscv"];
const RISCV32E_ALIASES: &[&str] = &["riscv32e"];
const ARM_ALIASES: &[&str] = &["arm", "armv7"];
const ARMLE_ALIASES: &[&str] = &["armle"];
const ARMBE_ALIASES: &[&str] = &["armbe"];
const THUMB_ALIASES: &[&str] = &["thumb"];
const AARCH64_ALIASES: &[&str] = &["aarch64"];
const AARCH64BE_ALIASES: &[&str] = &["aarch64be"];
const X16_ALIASES: &[&str] = &["x16"];
const X32_ALIASES: &[&str] = &["x32", "x86", "i386"];
const X64_ALIASES: &[&str] = &["x64", "x86-64", "x86_64", "amd64"];
const MIPS_ALIASES: &[&str] = &["mips"];
const MIPSEL_ALIASES: &[&str] = &["mipsel"];
const MIPS64_ALIASES: &[&str] = &["mips64"];
const MIPS64EL_ALIASES: &[&str] = &["mips64el"];
const POWERPC32_ALIASES: &[&str] = &["ppc", "powerpc", "ppc32", "powerpc32"];
const POWERPC32BE_ALIASES: &[&str] = &["ppcbe", "powerpcbe", "ppc32be", "powerpc32be"];
const POWERPC64_ALIASES: &[&str] = &["ppc64", "powerpc64"];
const POWERPC64BE_ALIASES: &[&str] = &["ppc64be", "powerpc64be"];
const SPARC_ALIASES: &[&str] = &["sparc"];
const SPARCLE_ALIASES: &[&str] = &["sparcle"];
const SPARC64_ALIASES: &[&str] = &["sparc64"];
const SYSTEMZ_ALIASES: &[&str] = &["systemz", "s390x"];
const XCORE_ALIASES: &[&str] = &["xcore"];
const M68K_ALIASES: &[&str] = &["m68k"];
const TMS320C64X_ALIASES: &[&str] = &["tms320c64x", "c64x"];
const M680X_ALIASES: &[&str] = &["m680x"];
const EVM_ALIASES: &[&str] = &["evm"];
const BPF_ALIASES: &[&str] = &["bpf"];

const ARCHITECTURE_CAPABILITIES: &[ArchitectureCapability] = &[
    ArchitectureCapability {
        canonical_name: "riscv32",
        category: "RISC-V",
        aliases: RISCV32_ALIASES,
        parse_supported: true,
        decode_supported: true,
        detail_supported: true,
        json_supported: true,
    },
    ArchitectureCapability {
        canonical_name: "riscv64",
        category: "RISC-V",
        aliases: RISCV64_ALIASES,
        parse_supported: true,
        decode_supported: true,
        detail_supported: true,
        json_supported: true,
    },
    ArchitectureCapability {
        canonical_name: "riscv32e",
        category: "RISC-V",
        aliases: RISCV32E_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "arm",
        category: "ARM",
        aliases: ARM_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "armle",
        category: "ARM",
        aliases: ARMLE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "armbe",
        category: "ARM",
        aliases: ARMBE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "thumb",
        category: "ARM",
        aliases: THUMB_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "aarch64",
        category: "ARM",
        aliases: AARCH64_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "aarch64be",
        category: "ARM",
        aliases: AARCH64BE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "x16",
        category: "x86",
        aliases: X16_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "x32",
        category: "x86",
        aliases: X32_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "x64",
        category: "x86",
        aliases: X64_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "mips",
        category: "MIPS",
        aliases: MIPS_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "mipsel",
        category: "MIPS",
        aliases: MIPSEL_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "mips64",
        category: "MIPS",
        aliases: MIPS64_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "mips64el",
        category: "MIPS",
        aliases: MIPS64EL_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "powerpc32",
        category: "PowerPC",
        aliases: POWERPC32_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "powerpc32be",
        category: "PowerPC",
        aliases: POWERPC32BE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "powerpc64",
        category: "PowerPC",
        aliases: POWERPC64_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "powerpc64be",
        category: "PowerPC",
        aliases: POWERPC64BE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "sparc",
        category: "SPARC",
        aliases: SPARC_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "sparcle",
        category: "SPARC",
        aliases: SPARCLE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "sparc64",
        category: "SPARC",
        aliases: SPARC64_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "systemz",
        category: "Other",
        aliases: SYSTEMZ_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "xcore",
        category: "Other",
        aliases: XCORE_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "m68k",
        category: "Other",
        aliases: M68K_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "tms320c64x",
        category: "Other",
        aliases: TMS320C64X_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "m680x",
        category: "Other",
        aliases: M680X_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "evm",
        category: "Other",
        aliases: EVM_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
    ArchitectureCapability {
        canonical_name: "bpf",
        category: "Other",
        aliases: BPF_ALIASES,
        parse_supported: true,
        decode_supported: false,
        detail_supported: false,
        json_supported: false,
    },
];

pub fn all_architecture_capabilities() -> &'static [ArchitectureCapability] {
    ARCHITECTURE_CAPABILITIES
}

pub fn lookup_architecture_capability(token: &str) -> Option<&'static ArchitectureCapability> {
    let normalized = normalize_architecture_token(token);
    if normalized.is_empty() {
        return None;
    }

    ARCHITECTURE_CAPABILITIES
        .iter()
        .find(|capability| capability.aliases.contains(&normalized.as_str()))
}

pub fn canonical_architecture_name(token: &str) -> Option<&'static str> {
    lookup_architecture_capability(token).map(|capability| capability.canonical_name)
}

fn normalize_architecture_token(token: &str) -> String {
    token.trim().to_ascii_lowercase()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Architecture {
    RiscV32,
    RiscV64,
    RiscV32E,
    X86,
    X86_64,
    AArch64,
    Arm,
    Unknown,
}

impl Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Architecture {
    /// Performs the conversion from a string slice (`&str`) into `Architecture`.
    ///
    /// This uses the same logic as [`Architecture::parse`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use robustone_core::prelude::Architecture;
    /// let arch: Architecture = "AMD64".into();
    /// assert_eq!(arch, "x86_64");
    /// ```
    fn from(name: &str) -> Self {
        Architecture::parse(name)
    }
}

impl From<String> for Architecture {
    /// Performs the conversion from a owned string (`String`) into `Architecture`.
    ///
    /// This uses the same logic as [`Architecture::parse`].
    fn from(name: String) -> Self {
        Architecture::parse(&name)
    }
}

impl PartialEq<&str> for Architecture {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for Architecture {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Architecture {
    /// Return a static string slice representation of the architecture without allocating.
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::RiscV32 => "riscv32",
            Architecture::RiscV64 => "riscv64",
            Architecture::RiscV32E => "riscv32e",
            Architecture::X86 => "x86",
            Architecture::X86_64 => "x86_64",
            Architecture::AArch64 => "aarch64",
            Architecture::Arm => "arm",
            Architecture::Unknown => "unknown",
        }
    }

    /// Determines the architecture from a name string.
    ///
    /// This utility function attempts to match a provided architecture name
    /// against the canonical capability registry. It's useful for parsing
    /// user input while keeping CLI and documentation contracts aligned.
    ///
    /// # Arguments
    ///
    /// * `name` - The architecture name to parse
    ///
    /// # Returns
    ///
    /// An Architecture enum representing the parsed architecture.
    ///
    /// Returns `Architecture::Unknown` if the input does not match a known architecture.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use robustone_core::prelude::*;
    /// assert_eq!(Architecture::from("RISCV32"), "riscv32");
    /// assert_eq!(Architecture::from("x86-64"), "x86_64");
    /// assert_eq!(Architecture::from("armv7"), "arm");
    /// assert_eq!(Architecture::from("RISCV32"), Architecture::RiscV32);
    /// assert_eq!(Architecture::from("x86-64"), Architecture::X86_64);
    /// assert_eq!(Architecture::from("ARMV7"), Architecture::Arm);
    /// ```
    fn parse(name: impl AsRef<str>) -> Self {
        match canonical_architecture_name(name.as_ref()) {
            Some("riscv32") => Architecture::RiscV32,
            Some("riscv64") => Architecture::RiscV64,
            Some("riscv32e") => Architecture::RiscV32E,
            Some("x16" | "x32") => Architecture::X86,
            Some("x64") => Architecture::X86_64,
            Some("aarch64" | "aarch64be") => Architecture::AArch64,
            Some("arm" | "armle" | "armbe" | "thumb") => Architecture::Arm,
            _ => Architecture::Unknown,
        }
    }
}

/// Validates that a byte sequence is properly aligned for an architecture.
///
/// # Arguments
///
/// * `address` - The memory address to check
/// * `alignment` - The required alignment in bytes
///
/// # Returns
///
/// `true` if the address is properly aligned, `false` otherwise.
pub fn is_address_aligned(address: u64, alignment: usize) -> bool {
    address.is_multiple_of(alignment as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_utils_determine_architecture() {
        assert_eq!(Architecture::from("RISCV32"), Architecture::RiscV32);
        assert_eq!(Architecture::from("riscv64"), Architecture::RiscV64);
        assert_eq!(Architecture::from("RISCV32E"), Architecture::RiscV32E);
        assert_eq!(Architecture::from("x86-64"), Architecture::X86_64);
        assert_eq!(Architecture::from("AMD64"), Architecture::X86_64);
        assert_eq!(Architecture::from("aarch64"), Architecture::AArch64);
        assert_eq!(Architecture::from("ARMV7"), Architecture::Arm);
        assert_eq!(Architecture::from("unknown"), Architecture::Unknown);
    }

    #[test]
    fn test_architecture_utils_determine_architecture_variants() {
        assert_eq!(Architecture::from("riscv32"), Architecture::RiscV32);
        assert_eq!(Architecture::from("riscv64"), Architecture::RiscV64);
        assert_eq!(Architecture::from("riscv32e"), Architecture::RiscV32E);
        assert_eq!(Architecture::from("x86"), Architecture::X86);
        assert_eq!(Architecture::from("x86_64"), Architecture::X86_64);
        assert_eq!(Architecture::from("aarch64"), Architecture::AArch64);
        assert_eq!(Architecture::from("arm"), Architecture::Arm);
        assert_eq!(Architecture::from("mips"), Architecture::Unknown);
    }

    #[test]
    fn test_architecture_utils_normalize_name() {
        assert_eq!(Architecture::from("RISCV32"), "riscv32");
        assert_eq!(Architecture::from("riscv64"), "riscv64");
        assert_eq!(Architecture::from("RISCV32E"), "riscv32e");
        assert_eq!(Architecture::from("x86-64"), "x86_64");
        assert_eq!(Architecture::from("AMD64"), "x86_64");
        assert_eq!(Architecture::from("aarch64"), "aarch64");
        assert_eq!(Architecture::from("ARMV7"), "arm");
        assert_eq!(Architecture::from("unknown"), "unknown");
    }

    #[test]
    fn test_lookup_architecture_capability_handles_aliases() {
        let capability = lookup_architecture_capability("x86-64").expect("alias should resolve");
        assert_eq!(capability.canonical_name, "x64");
        assert!(capability.parse_supported);
        assert!(!capability.decode_supported);

        let capability = lookup_architecture_capability("ppc").expect("alias should resolve");
        assert_eq!(capability.canonical_name, "powerpc32");
        assert_eq!(capability.category, "PowerPC");
    }

    #[test]
    fn test_lookup_architecture_capability_marks_riscv_surfaces() {
        let capability =
            lookup_architecture_capability("riscv32").expect("riscv32 capability should exist");
        assert!(capability.decode_supported);
        assert!(capability.detail_supported);
        assert!(capability.json_supported);

        let capability =
            lookup_architecture_capability("riscv32e").expect("riscv32e capability should exist");
        assert!(capability.parse_supported);
        assert!(!capability.decode_supported);
        assert!(!capability.detail_supported);
        assert!(!capability.json_supported);
    }

    #[test]
    fn test_architecture_utils_alignment() {
        assert!(is_address_aligned(0x1000, 4));
        assert!(is_address_aligned(0x1004, 4));
        assert!(!is_address_aligned(0x1002, 4));
        assert!(is_address_aligned(0x1000, 8));
        assert!(!is_address_aligned(0x1004, 8));
    }
}
