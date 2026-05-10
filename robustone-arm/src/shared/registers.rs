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
            0 => "w0",
            1 => "w1",
            2 => "w2",
            3 => "w3",
            4 => "w4",
            5 => "w5",
            6 => "w6",
            7 => "w7",
            8 => "w8",
            9 => "w9",
            10 => "w10",
            11 => "w11",
            12 => "w12",
            13 => "w13",
            14 => "w14",
            15 => "w15",
            16 => "w16",
            17 => "w17",
            18 => "w18",
            19 => "w19",
            20 => "w20",
            21 => "w21",
            22 => "w22",
            23 => "w23",
            24 => "w24",
            25 => "w25",
            26 => "w26",
            27 => "w27",
            28 => "w28",
            29 => "w29",
            30 => "w30",
            _ => unreachable!(),
        }
    } else {
        match reg {
            0 => "x0",
            1 => "x1",
            2 => "x2",
            3 => "x3",
            4 => "x4",
            5 => "x5",
            6 => "x6",
            7 => "x7",
            8 => "x8",
            9 => "x9",
            10 => "x10",
            11 => "x11",
            12 => "x12",
            13 => "x13",
            14 => "x14",
            15 => "x15",
            16 => "x16",
            17 => "x17",
            18 => "x18",
            19 => "x19",
            20 => "x20",
            21 => "x21",
            22 => "x22",
            23 => "x23",
            24 => "x24",
            25 => "x25",
            26 => "x26",
            27 => "x27",
            28 => "x28",
            29 => "x29",
            30 => "x30",
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

/// FP/SIMD register size class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpRegSize {
    /// Byte (8-bit)
    B,
    /// Half (16-bit)
    H,
    /// Single (32-bit)
    S,
    /// Double (64-bit)
    D,
    /// Quad (128-bit)
    Q,
    /// Vector (used for structure loads/stores, e.g. v0.16b)
    V,
}

/// Get the name of a scalar FP/SIMD register.
pub fn fp_simd_reg_name(reg: u8, size: FpRegSize) -> &'static str {
    match size {
        FpRegSize::B => match reg {
            0 => "b0", 1 => "b1", 2 => "b2", 3 => "b3", 4 => "b4",
            5 => "b5", 6 => "b6", 7 => "b7", 8 => "b8", 9 => "b9",
            10 => "b10", 11 => "b11", 12 => "b12", 13 => "b13",
            14 => "b14", 15 => "b15", 16 => "b16", 17 => "b17",
            18 => "b18", 19 => "b19", 20 => "b20", 21 => "b21",
            22 => "b22", 23 => "b23", 24 => "b24", 25 => "b25",
            26 => "b26", 27 => "b27", 28 => "b28", 29 => "b29",
            30 => "b30", 31 => "b31",
            _ => unreachable!(),
        },
        FpRegSize::H => match reg {
            0 => "h0", 1 => "h1", 2 => "h2", 3 => "h3", 4 => "h4",
            5 => "h5", 6 => "h6", 7 => "h7", 8 => "h8", 9 => "h9",
            10 => "h10", 11 => "h11", 12 => "h12", 13 => "h13",
            14 => "h14", 15 => "h15", 16 => "h16", 17 => "h17",
            18 => "h18", 19 => "h19", 20 => "h20", 21 => "h21",
            22 => "h22", 23 => "h23", 24 => "h24", 25 => "h25",
            26 => "h26", 27 => "h27", 28 => "h28", 29 => "h29",
            30 => "h30", 31 => "h31",
            _ => unreachable!(),
        },
        FpRegSize::S => match reg {
            0 => "s0", 1 => "s1", 2 => "s2", 3 => "s3", 4 => "s4",
            5 => "s5", 6 => "s6", 7 => "s7", 8 => "s8", 9 => "s9",
            10 => "s10", 11 => "s11", 12 => "s12", 13 => "s13",
            14 => "s14", 15 => "s15", 16 => "s16", 17 => "s17",
            18 => "s18", 19 => "s19", 20 => "s20", 21 => "s21",
            22 => "s22", 23 => "s23", 24 => "s24", 25 => "s25",
            26 => "s26", 27 => "s27", 28 => "s28", 29 => "s29",
            30 => "s30", 31 => "s31",
            _ => unreachable!(),
        },
        FpRegSize::D => match reg {
            0 => "d0", 1 => "d1", 2 => "d2", 3 => "d3", 4 => "d4",
            5 => "d5", 6 => "d6", 7 => "d7", 8 => "d8", 9 => "d9",
            10 => "d10", 11 => "d11", 12 => "d12", 13 => "d13",
            14 => "d14", 15 => "d15", 16 => "d16", 17 => "d17",
            18 => "d18", 19 => "d19", 20 => "d20", 21 => "d21",
            22 => "d22", 23 => "d23", 24 => "d24", 25 => "d25",
            26 => "d26", 27 => "d27", 28 => "d28", 29 => "d29",
            30 => "d30", 31 => "d31",
            _ => unreachable!(),
        },
        FpRegSize::Q => match reg {
            0 => "q0", 1 => "q1", 2 => "q2", 3 => "q3", 4 => "q4",
            5 => "q5", 6 => "q6", 7 => "q7", 8 => "q8", 9 => "q9",
            10 => "q10", 11 => "q11", 12 => "q12", 13 => "q13",
            14 => "q14", 15 => "q15", 16 => "q16", 17 => "q17",
            18 => "q18", 19 => "q19", 20 => "q20", 21 => "q21",
            22 => "q22", 23 => "q23", 24 => "q24", 25 => "q25",
            26 => "q26", 27 => "q27", 28 => "q28", 29 => "q29",
            30 => "q30", 31 => "q31",
            _ => unreachable!(),
        },
        FpRegSize::V => match reg {
            0 => "v0", 1 => "v1", 2 => "v2", 3 => "v3", 4 => "v4",
            5 => "v5", 6 => "v6", 7 => "v7", 8 => "v8", 9 => "v9",
            10 => "v10", 11 => "v11", 12 => "v12", 13 => "v13",
            14 => "v14", 15 => "v15", 16 => "v16", 17 => "v17",
            18 => "v18", 19 => "v19", 20 => "v20", 21 => "v21",
            22 => "v22", 23 => "v23", 24 => "v24", 25 => "v25",
            26 => "v26", 27 => "v27", 28 => "v28", 29 => "v29",
            30 => "v30", 31 => "v31",
            _ => unreachable!(),
        },
    }
}

/// Vector arrangement suffix from size:Q bits.
/// Returns e.g. ".8b", ".4h", ".2s", ".1d", ".16b", ".8h", ".4s", ".2d"
pub fn arrangement_suffix(size: u8, q: u8) -> &'static str {
    match (size, q) {
        (0b00, 0) => ".8b",
        (0b00, 1) => ".16b",
        (0b01, 0) => ".4h",
        (0b01, 1) => ".8h",
        (0b10, 0) => ".2s",
        (0b10, 1) => ".4s",
        (0b11, 0) => ".1d",
        (0b11, 1) => ".2d",
        _ => unreachable!(),
    }
}

/// FP register size from `ftype` field (bits 23:22 in scalar FP data processing).
pub fn ftype_to_size(ftype: u8) -> Option<FpRegSize> {
    match ftype {
        0b00 => Some(FpRegSize::S),
        0b01 => Some(FpRegSize::D),
        0b10 => Some(FpRegSize::H),
        0b11 => Some(FpRegSize::B),
        _ => unreachable!(),
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
