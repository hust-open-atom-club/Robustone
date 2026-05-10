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

    /// Create a canonical RV32E profile (embedded base integer, 16 registers).
    pub fn riscv32e() -> Self {
        Self {
            architecture: Architecture::RiscV32E,
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

impl<'a> From<&'a crate::decode_config::DecodeConfig> for ArchitectureProfile {
    fn from(config: &'a crate::decode_config::DecodeConfig) -> Self {
        use crate::architecture::Architecture;
        let mode_name = config.mode_name();
        let architecture = match config.arch {
            crate::decode_config::Arch::RiscV => match config.mode {
                crate::decode_config::Mode::Rv32E => Architecture::RiscV32E,
                _ if mode_name.starts_with("riscv64") => Architecture::RiscV64,
                _ => Architecture::RiscV32,
            },
            crate::decode_config::Arch::LoongArch => Architecture::LoongArch64,
            crate::decode_config::Arch::Arm => Architecture::Arm,
            crate::decode_config::Arch::AArch64 => Architecture::AArch64,
            crate::decode_config::Arch::X86 => Architecture::X86,
        };
        let bit_width = match config.mode {
            crate::decode_config::Mode::X16 => 16,
            crate::decode_config::Mode::X32
            | crate::decode_config::Mode::Rv32
            | crate::decode_config::Mode::Rv32E
            | crate::decode_config::Mode::Arm
            | crate::decode_config::Mode::ArmLE
            | crate::decode_config::Mode::ArmBE
            | crate::decode_config::Mode::Thumb
            | crate::decode_config::Mode::La32 => 32,
            crate::decode_config::Mode::X64
            | crate::decode_config::Mode::Rv64
            | crate::decode_config::Mode::AArch64
            | crate::decode_config::Mode::AArch64BE
            | crate::decode_config::Mode::La64 => 64,
        };
        let extensions: Vec<&'static str> = config
            .features
            .extensions()
            .iter()
            .map(|s| {
                // Leak the string to get a &'static str for the profile.
                // The profile is consumed immediately in the decode path so this
                // is safe in practice.
                Box::leak(s.clone().into_boxed_str()) as &'static str
            })
            .collect();
        ArchitectureProfile {
            architecture,
            mode_name,
            bit_width,
            endianness: config.endianness,
            enabled_extensions: extensions,
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
