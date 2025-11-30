//! Architecture utilities for multi-architecture support.
//!
//! This module provides utility functions for working with different
//! instruction set architectures in a consistent way.

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
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
    /// against known architecture patterns. It's useful for parsing user input.
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
        let normalized = name.as_ref().to_lowercase();

        match normalized.as_str() {
            // RISC-V variants
            n if n.starts_with("riscv") => {
                if n.contains("e") {
                    Architecture::RiscV32E
                } else if n.contains("32") {
                    Architecture::RiscV32
                } else if n.contains("64") {
                    Architecture::RiscV64
                } else {
                    Architecture::RiscV32
                }
            }

            // x86 variants
            n if n.starts_with("x86") || n.starts_with("i386") || n.starts_with("amd64") => {
                if n.contains("64") {
                    Architecture::X86_64
                } else {
                    Architecture::X86
                }
            }

            // ARM variants
            n if n.starts_with("aarch64") => Architecture::AArch64,
            n if n.starts_with("arm") => {
                if n.contains("64") {
                    Architecture::AArch64
                } else {
                    Architecture::Arm
                }
            }

            // Return normalized version for unknown architectures
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
    fn test_architecture_utils_alignment() {
        assert!(is_address_aligned(0x1000, 4));
        assert!(is_address_aligned(0x1004, 4));
        assert!(!is_address_aligned(0x1002, 4));
        assert!(is_address_aligned(0x1000, 8));
        assert!(!is_address_aligned(0x1004, 8));
    }
}
