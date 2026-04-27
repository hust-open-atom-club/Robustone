//! Register management utilities for LoongArch instruction decoding.
//!
//! Provides centralized register name lookup and management functionality
//! used across all LoongArch instruction families.

use crate::types::LoongArchRegister;

/// Trait for providing register names in LoongArch extensions.
pub trait RegisterNameProvider {
    /// Get the name of a general-purpose register (r0-r31).
    fn gpr_name(&self, reg: u8) -> &'static str;

    /// Get the name of a floating-point register (f0-f31).
    fn fpr_name(&self, reg: u8) -> &'static str;

    /// Get the name of a vector register (xr0-xr31).
    fn vector_name(&self, reg: u8) -> &'static str;

    /// Get the name of a condition flag register (fcc0-fcc7).
    fn fcc_name(&self, reg: u8) -> &'static str;

    /// Get the name of an SCR (scr0-scr3).
    fn scr_name(&self, reg: u8) -> &'static str;
}

/// Centralized register manager providing name lookup for all LoongArch registers.
pub struct RegisterManager;

impl RegisterNameProvider for RegisterManager {
    fn gpr_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "$zero",
            1 => "$ra",
            2 => "$tp",
            3 => "$sp",
            4 => "$a0",
            5 => "$a1",
            6 => "$a2",
            7 => "$a3",
            8 => "$a4",
            9 => "$a5",
            10 => "$a6",
            11 => "$a7",
            12 => "$t0",
            13 => "$t1",
            14 => "$t2",
            15 => "$t3",
            16 => "$t4",
            17 => "$t5",
            18 => "$t6",
            19 => "$t7",
            20 => "$t8",
            21 => "$r21",
            22 => "$fp",
            23 => "$s0",
            24 => "$s1",
            25 => "$s2",
            26 => "$s3",
            27 => "$s4",
            28 => "$s5",
            29 => "$s6",
            30 => "$s7",
            31 => "$s8",
            _ => "$invalid",
        }
    }

    fn fpr_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "$fa0",
            1 => "$fa1",
            2 => "$fa2",
            3 => "$fa3",
            4 => "$fa4",
            5 => "$fa5",
            6 => "$fa6",
            7 => "$fa7",
            8 => "$ft0",
            9 => "$ft1",
            10 => "$ft2",
            11 => "$ft3",
            12 => "$ft4",
            13 => "$ft5",
            14 => "$ft6",
            15 => "$ft7",
            16 => "$ft8",
            17 => "$ft9",
            18 => "$ft10",
            19 => "$ft11",
            20 => "$ft12",
            21 => "$ft13",
            22 => "$ft14",
            23 => "$ft15",
            24 => "$fs0",
            25 => "$fs1",
            26 => "$fs2",
            27 => "$fs3",
            28 => "$fs4",
            29 => "$fs5",
            30 => "$fs6",
            31 => "$fs7",
            _ => "$invalid",
        }
    }

    fn vector_name(&self, reg: u8) -> &'static str {
        match reg {
            0..=31 => {
                // Use static array for vector names to avoid runtime format
                static NAMES: &[&str] = &[
                    "$xr0", "$xr1", "$xr2", "$xr3", "$xr4", "$xr5", "$xr6", "$xr7", "$xr8", "$xr9",
                    "$xr10", "$xr11", "$xr12", "$xr13", "$xr14", "$xr15", "$xr16", "$xr17",
                    "$xr18", "$xr19", "$xr20", "$xr21", "$xr22", "$xr23", "$xr24", "$xr25",
                    "$xr26", "$xr27", "$xr28", "$xr29", "$xr30", "$xr31",
                ];
                NAMES[reg as usize]
            }
            _ => "$invalid",
        }
    }

    fn fcc_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "$fcc0",
            1 => "$fcc1",
            2 => "$fcc2",
            3 => "$fcc3",
            4 => "$fcc4",
            5 => "$fcc5",
            6 => "$fcc6",
            7 => "$fcc7",
            _ => "$invalid",
        }
    }

    fn scr_name(&self, reg: u8) -> &'static str {
        match reg {
            0 => "$scr0",
            1 => "$scr1",
            2 => "$scr2",
            3 => "$scr3",
            _ => "$invalid",
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

    /// Format a register for display given its class and raw id.
    pub fn format_register(&self, reg: LoongArchRegister) -> &'static str {
        reg.name()
    }

    /// Format a raw register id for display, inferring the class from the id range.
    pub fn format_raw_id(&self, id: u32) -> &'static str {
        LoongArchRegister::from_id(id).name()
    }
}

impl Default for RegisterManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: get the GPR name for a raw 5-bit id.
pub fn gpr_name(reg: u8) -> &'static str {
    RegisterManager::instance().gpr_name(reg)
}

/// Convenience function: get the FPR name for a raw 5-bit id.
pub fn fpr_name(reg: u8) -> &'static str {
    RegisterManager::instance().fpr_name(reg)
}

/// Convenience function: get the vector register name for a raw 5-bit id.
pub fn vector_name(reg: u8) -> &'static str {
    RegisterManager::instance().vector_name(reg)
}

/// Convenience function: get the FCC name for a raw 3-bit id.
pub fn fcc_name(reg: u8) -> &'static str {
    RegisterManager::instance().fcc_name(reg)
}

/// Convenience function: get the SCR name for a raw 2-bit id.
pub fn scr_name(reg: u8) -> &'static str {
    RegisterManager::instance().scr_name(reg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpr_names() {
        let mgr = RegisterManager::new();
        assert_eq!(mgr.gpr_name(0), "$zero");
        assert_eq!(mgr.gpr_name(1), "$ra");
        assert_eq!(mgr.gpr_name(3), "$sp");
        assert_eq!(mgr.gpr_name(4), "$a0");
        assert_eq!(mgr.gpr_name(11), "$a7");
        assert_eq!(mgr.gpr_name(12), "$t0");
        assert_eq!(mgr.gpr_name(20), "$t8");
        assert_eq!(mgr.gpr_name(21), "$r21");
        assert_eq!(mgr.gpr_name(22), "$fp");
        assert_eq!(mgr.gpr_name(31), "$s8");
        assert_eq!(mgr.gpr_name(32), "$invalid");
    }

    #[test]
    fn test_fpr_names() {
        let mgr = RegisterManager::new();
        assert_eq!(mgr.fpr_name(0), "$fa0");
        assert_eq!(mgr.fpr_name(7), "$fa7");
        assert_eq!(mgr.fpr_name(8), "$ft0");
        assert_eq!(mgr.fpr_name(24), "$fs0");
        assert_eq!(mgr.fpr_name(31), "$fs7");
        assert_eq!(mgr.fpr_name(32), "$invalid");
    }

    #[test]
    fn test_vector_names() {
        let mgr = RegisterManager::new();
        assert_eq!(mgr.vector_name(0), "$xr0");
        assert_eq!(mgr.vector_name(31), "$xr31");
        assert_eq!(mgr.vector_name(32), "$invalid");
    }

    #[test]
    fn test_fcc_names() {
        let mgr = RegisterManager::new();
        assert_eq!(mgr.fcc_name(0), "$fcc0");
        assert_eq!(mgr.fcc_name(7), "$fcc7");
        assert_eq!(mgr.fcc_name(8), "$invalid");
    }

    #[test]
    fn test_scr_names() {
        let mgr = RegisterManager::new();
        assert_eq!(mgr.scr_name(0), "$scr0");
        assert_eq!(mgr.scr_name(3), "$scr3");
        assert_eq!(mgr.scr_name(4), "$invalid");
    }

    #[test]
    fn test_register_enum_roundtrip() {
        for id in 0..=107u32 {
            let reg = LoongArchRegister::from_id(id);
            // Every valid id should map to a named register
            assert_ne!(reg, LoongArchRegister::Invalid, "id {} should be valid", id);
        }
        assert_eq!(LoongArchRegister::from_id(200), LoongArchRegister::Invalid);
    }
}
