//! Architecture utilities for multi-architecture support.
//!
//! This module provides utility functions for working with different
//! instruction set architectures in a consistent way.

/// Utility functions for working with architectures.
pub struct ArchitectureUtils;

impl ArchitectureUtils {
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
    /// A normalized architecture name, or the original name if no pattern matches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use robustone_core::prelude::*;
    /// assert_eq!(ArchitectureUtils::normalize_name("RISCV32"), "riscv32");
    /// assert_eq!(ArchitectureUtils::normalize_name("x86-64"), "x86_64");
    /// assert_eq!(ArchitectureUtils::normalize_name("armv7"), "arm");
    /// ```
    pub fn normalize_name(name: &str) -> String {
        let normalized = name.to_lowercase();

        match normalized.as_str() {
            // RISC-V variants
            n if n.starts_with("riscv") => {
                if n.contains("e") {
                    "riscv32e".to_string()
                } else if n.contains("32") {
                    "riscv32".to_string()
                } else if n.contains("64") {
                    "riscv64".to_string()
                } else {
                    "riscv".to_string()
                }
            }

            // x86 variants
            n if n.starts_with("x86") || n.starts_with("i386") || n.starts_with("amd64") => {
                if n.contains("64") {
                    "x86_64".to_string()
                } else {
                    "x86".to_string()
                }
            }

            // ARM variants
            n if n.starts_with("arm") => {
                if n.contains("64") || n.starts_with("aarch64") {
                    "aarch64".to_string()
                } else {
                    "arm".to_string()
                }
            }

            // Return normalized version for unknown architectures
            _ => normalized,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_utils_normalize_name() {
        assert_eq!(ArchitectureUtils::normalize_name("RISCV32"), "riscv32");
        assert_eq!(ArchitectureUtils::normalize_name("riscv64"), "riscv64");
        assert_eq!(ArchitectureUtils::normalize_name("RISCV32E"), "riscv32e");
        assert_eq!(ArchitectureUtils::normalize_name("x86-64"), "x86_64");
        assert_eq!(ArchitectureUtils::normalize_name("AMD64"), "x86_64");
        assert_eq!(ArchitectureUtils::normalize_name("aarch64"), "aarch64");
        assert_eq!(ArchitectureUtils::normalize_name("ARMV7"), "arm");
        assert_eq!(ArchitectureUtils::normalize_name("unknown"), "unknown");
    }

    #[test]
    fn test_architecture_utils_alignment() {
        assert!(ArchitectureUtils::is_address_aligned(0x1000, 4));
        assert!(ArchitectureUtils::is_address_aligned(0x1004, 4));
        assert!(!ArchitectureUtils::is_address_aligned(0x1002, 4));
        assert!(ArchitectureUtils::is_address_aligned(0x1000, 8));
        assert!(!ArchitectureUtils::is_address_aligned(0x1004, 8));
    }
}
