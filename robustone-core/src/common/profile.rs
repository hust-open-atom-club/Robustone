//! Architecture profile types shared across decode backends.

use crate::architecture::Architecture;
use crate::utils::Endianness;

/// A concrete architecture configuration used by low-level decode APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchitectureProfile {
    pub architecture: Architecture,
    pub mode_name: &'static str,
    pub bit_width: u8,
    pub endianness: Endianness,
    pub enabled_extensions: Vec<&'static str>,
}

impl ArchitectureProfile {
    /// Create a canonical RV32I profile (base integer only).
    pub fn riscv32i() -> Self {
        Self {
            architecture: Architecture::RiscV32,
            mode_name: "riscv32",
            bit_width: 32,
            endianness: Endianness::Little,
            enabled_extensions: vec!["I"],
        }
    }

    /// Create a canonical RV32E profile (embedded base integer).
    pub fn riscv32e() -> Self {
        Self {
            architecture: Architecture::RiscV32,
            mode_name: "riscv32e",
            bit_width: 32,
            endianness: Endianness::Little,
            enabled_extensions: vec!["I"],
        }
    }

    /// Create a canonical RV64I profile (base integer only).
    pub fn riscv64i() -> Self {
        Self {
            architecture: Architecture::RiscV64,
            mode_name: "riscv64",
            bit_width: 64,
            endianness: Endianness::Little,
            enabled_extensions: vec!["I"],
        }
    }

    /// Create a canonical RV32GC profile.
    pub fn riscv32gc() -> Self {
        Self {
            architecture: Architecture::RiscV32,
            mode_name: "riscv32",
            bit_width: 32,
            endianness: Endianness::Little,
            enabled_extensions: vec!["I", "M", "A", "F", "D", "C"],
        }
    }

    /// Create a canonical RV64GC profile.
    pub fn riscv64gc() -> Self {
        Self {
            architecture: Architecture::RiscV64,
            mode_name: "riscv64",
            bit_width: 64,
            endianness: Endianness::Little,
            enabled_extensions: vec!["I", "M", "A", "F", "D", "C"],
        }
    }

    /// Create a profile with explicit RISC-V configuration.
    pub fn riscv(
        architecture: Architecture,
        mode_name: &'static str,
        bit_width: u8,
        enabled_extensions: Vec<&'static str>,
    ) -> Self {
        Self {
            architecture,
            mode_name,
            bit_width,
            endianness: Endianness::Little,
            enabled_extensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_riscv_profiles() {
        let rv32 = ArchitectureProfile::riscv32gc();
        assert_eq!(rv32.mode_name, "riscv32");
        assert_eq!(rv32.bit_width, 32);
        assert!(rv32.enabled_extensions.contains(&"C"));

        let rv64 = ArchitectureProfile::riscv64gc();
        assert_eq!(rv64.mode_name, "riscv64");
        assert_eq!(rv64.bit_width, 64);
        assert!(rv64.enabled_extensions.contains(&"D"));
    }
}
