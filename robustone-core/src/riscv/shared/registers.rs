//! Register management utilities for RISC-V instruction decoding.
//!
//! Provides centralized register name lookup and management functionality
//! used across all RISC-V extensions.

use super::super::types::*;

/// Trait for providing register names in RISC-V extensions.
pub trait RegisterNameProvider {
    /// Get the name of an integer register (x0-x31).
    fn int_register_name(&self, reg: u8) -> &'static str;

    /// Get the name of a floating-point register (f0-f31).
    fn fp_register_name(&self, reg: u8) -> &'static str;

    /// Get the name of a compressed register (x8-x15).
    fn compressed_register_name(&self, reg: u8) -> &'static str;
}

/// Centralized register manager providing name lookup for all RISC-V registers.
pub struct RegisterManager;

impl RegisterNameProvider for RegisterManager {
    fn int_register_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "invalid",
        }
    }

    fn fp_register_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "ft0",
            1 => "ft1",
            2 => "ft2",
            3 => "ft3",
            4 => "ft4",
            5 => "ft5",
            6 => "ft6",
            7 => "ft7",
            8 => "fs0",
            9 => "fs1",
            10 => "fa0",
            11 => "fa1",
            12 => "fa2",
            13 => "fa3",
            14 => "fa4",
            15 => "fa5",
            16 => "fa6",
            17 => "fa7",
            18 => "fs2",
            19 => "fs3",
            20 => "fs4",
            21 => "fs5",
            22 => "fs6",
            23 => "fs7",
            24 => "fs8",
            25 => "fs9",
            26 => "fs10",
            27 => "fs11",
            28 => "ft8",
            29 => "ft9",
            30 => "ft10",
            31 => "ft11",
            _ => "invalid",
        }
    }

    fn compressed_register_name(&self, reg: u8) -> &'static str {
        if reg <= 7 {
            self.int_register_name(reg + 8)
        } else {
            "invalid"
        }
    }
}

impl RegisterManager {
    /// Create a new register manager.
    pub const fn new() -> Self {
        Self
    }

    /// Get the global register manager instance.
    pub const fn instance() -> &'static Self {
        &RegisterManager
    }

    /// Check if a register number is valid for integer registers.
    pub const fn is_valid_int_register(reg: u8) -> bool {
        reg <= 31
    }

    /// Check if a register number is valid for floating-point registers.
    pub const fn is_valid_fp_register(reg: u8) -> bool {
        reg <= 31
    }

    /// Check if a register number is valid for compressed registers (x8-x15).
    pub const fn is_valid_compressed_register(reg: u8) -> bool {
        reg <= 7
    }
}

/// Default implementation for register name lookup.
pub fn get_register_name(reg: u8) -> &'static str {
    let manager = RegisterManager::instance();
    manager.int_register_name(reg)
}

/// Default implementation for floating-point register name lookup.
pub fn get_fp_register_name(reg: u8) -> &'static str {
    let manager = RegisterManager::instance();
    manager.fp_register_name(reg)
}

/// Default implementation for compressed register name lookup.
pub fn get_compressed_register_name(reg: u8) -> &'static str {
    let manager = RegisterManager::instance();
    manager.compressed_register_name(reg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_register_names() {
        let manager = RegisterManager::new();
        assert_eq!(manager.int_register_name(0), "zero");
        assert_eq!(manager.int_register_name(1), "ra");
        assert_eq!(manager.int_register_name(2), "sp");
        assert_eq!(manager.int_register_name(31), "t6");
        assert_eq!(manager.int_register_name(32), "invalid");
    }

    #[test]
    fn test_fp_register_names() {
        let manager = RegisterManager::new();
        assert_eq!(manager.fp_register_name(0), "ft0");
        assert_eq!(manager.fp_register_name(8), "fs0");
        assert_eq!(manager.fp_register_name(10), "fa0");
        assert_eq!(manager.fp_register_name(31), "ft11");
        assert_eq!(manager.fp_register_name(32), "invalid");
    }

    #[test]
    fn test_compressed_register_names() {
        let manager = RegisterManager::new();
        assert_eq!(manager.compressed_register_name(0), "s0");
        assert_eq!(manager.compressed_register_name(7), "a5");
        assert_eq!(manager.compressed_register_name(8), "invalid");
    }

    #[test]
    fn test_register_validation() {
        assert!(RegisterManager::is_valid_int_register(31));
        assert!(!RegisterManager::is_valid_int_register(32));

        assert!(RegisterManager::is_valid_fp_register(31));
        assert!(!RegisterManager::is_valid_fp_register(32));

        assert!(RegisterManager::is_valid_compressed_register(7));
        assert!(!RegisterManager::is_valid_compressed_register(8));
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(get_register_name(0), "zero");
        assert_eq!(get_register_name(1), "ra");

        assert_eq!(get_fp_register_name(0), "ft0");
        assert_eq!(get_fp_register_name(8), "fs0");

        assert_eq!(get_compressed_register_name(0), "s0");
        assert_eq!(get_compressed_register_name(7), "a5");
    }
}