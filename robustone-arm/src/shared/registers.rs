//! AArch64 register name resolution.
//!
//! Handles the XZR/SP aliasing for R31 and 32-bit register naming.

use robustone_core::ir::{ArchitectureId, Operand, RegisterId};

/// Register class context: determines whether R31 is ZR or SP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegContext {
    /// Data processing (most ops): R31 = ZR.
    DataProc,
    /// Add/subtract with SP: R31 = SP.
    AddSub,
    /// Load/store base register: R31 = SP.
    LoadStore,
    /// Branch target: R31 = XZR (for `br xzr`).
    Branch,
}

/// Get the name of a general-purpose register.
/// `is_32bit` selects W* vs X* naming.
/// `context` determines whether R31 is ZR/SP or WZR/WSP.
pub fn gpr_name(reg: u8, is_32bit: bool, context: RegContext) -> &'static str {
    if reg >= 31 {
        return match (is_32bit, context) {
            (false, RegContext::LoadStore) | (false, RegContext::AddSub) => "sp",
            (false, _) => "xzr",
            (true, RegContext::LoadStore) | (true, RegContext::AddSub) => "wsp",
            (true, _) => "wzr",
        };
    }

    if is_32bit {
        match reg {
            0 => "w0", 1 => "w1", 2 => "w2", 3 => "w3",
            4 => "w4", 5 => "w5", 6 => "w6", 7 => "w7",
            8 => "w8", 9 => "w9", 10 => "w10", 11 => "w11",
            12 => "w12", 13 => "w13", 14 => "w14", 15 => "w15",
            16 => "w16", 17 => "w17", 18 => "w18", 19 => "w19",
            20 => "w20", 21 => "w21", 22 => "w22", 23 => "w23",
            24 => "w24", 25 => "w25", 26 => "w26", 27 => "w27",
            28 => "w28", 29 => "w29", 30 => "w30",
            _ => unreachable!(),
        }
    } else {
        match reg {
            0 => "x0", 1 => "x1", 2 => "x2", 3 => "x3",
            4 => "x4", 5 => "x5", 6 => "x6", 7 => "x7",
            8 => "x8", 9 => "x9", 10 => "x10", 11 => "x11",
            12 => "x12", 13 => "x13", 14 => "x14", 15 => "x15",
            16 => "x16", 17 => "x17", 18 => "x18", 19 => "x19",
            20 => "x20", 21 => "x21", 22 => "x22", 23 => "x23",
            24 => "x24", 25 => "x25", 26 => "x26", 27 => "x27",
            28 => "x28", 29 => "x29", 30 => "x30",
            _ => unreachable!(),
        }
    }
}

/// Create a RegisterId operand for AArch64.
pub fn reg_id(reg: u8) -> RegisterId {
    RegisterId {
        architecture: ArchitectureId::Arm,
        id: reg as u32,
    }
}

/// Create a register Operand.
pub fn reg_operand(reg: u8) -> Operand {
    Operand::Register {
        register: reg_id(reg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpr_names() {
        assert_eq!(gpr_name(0, false, RegContext::DataProc), "x0");
        assert_eq!(gpr_name(0, true, RegContext::DataProc), "w0");
        assert_eq!(gpr_name(30, false, RegContext::DataProc), "x30");
        assert_eq!(gpr_name(31, false, RegContext::DataProc), "xzr");
        assert_eq!(gpr_name(31, false, RegContext::LoadStore), "sp");
        assert_eq!(gpr_name(31, true, RegContext::DataProc), "wzr");
        assert_eq!(gpr_name(31, true, RegContext::LoadStore), "wsp");
    }

    #[test]
    fn test_reg_operand() {
        let op = reg_operand(5);
        match op {
            Operand::Register { register } => {
                assert_eq!(register.id, 5);
                assert_eq!(register.architecture, ArchitectureId::Arm);
            }
            _ => panic!("expected Register operand"),
        }
    }
}
