#![forbid(unsafe_code)]

//! Minimal RISC-V backend using the shared ISA framework.
//!
//! This is a shadow backend that demonstrates the framework for RISC-V.
//! Full migration (replacing the legacy extension-based decoder) is tracked
//! under AC-9.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec,
    ModeSet, RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVMode {
    RV32,
    RV64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RiscVFeature: u16 {
        const I = 1 << 0;
        const M = 1 << 1;
        const A = 1 << 2;
        const F = 1 << 3;
        const D = 1 << 4;
        const C = 1 << 5;
        const G = Self::I.bits() | Self::M.bits() | Self::A.bits() | Self::F.bits() | Self::D.bits();
        const CF = Self::C.bits() | Self::F.bits();
    }
}

impl FeatureSet for RiscVFeature {
    fn empty() -> Self {
        Self::empty()
    }
    fn all_supported_for_tests() -> Self {
        Self::G | Self::C
    }
    fn contains(self, required: Self) -> bool {
        self.bits() & required.bits() == required.bits()
    }
}

impl RiscVFeature {
    /// Build feature set from legacy `Extensions` configuration.
    pub fn from_extensions(extensions: &crate::extensions::Extensions) -> Self {
        use crate::extensions::standard::Standard;
        let mut features = Self::empty();
        if extensions.standard.contains(Standard::I) {
            features |= Self::I;
        }
        if extensions.standard.contains(Standard::M) {
            features |= Self::M;
        }
        if extensions.standard.contains(Standard::A) {
            features |= Self::A;
        }
        if extensions.standard.contains(Standard::F) {
            features |= Self::F;
        }
        if extensions.standard.contains(Standard::D) {
            features |= Self::D;
        }
        if extensions.standard.contains(Standard::C) {
            features |= Self::C;
        }
        features
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVField {
    Rd,
    Rs1,
    Rs2,
    Funct3,
    Funct7,
    Opcode,
    Imm12,
    Imm12S,
    Imm12B,
    Imm20U,
    Imm20J,
    // Compressed fields
    Rs2C,     // rs2 in CR format (bits 2-6)
    RdPrime,  // rd' in CA/CL format (bits 2-4, actual reg = +8)
    Rs2Prime, // rs2' in CA format (bits 2-4, actual reg = +8)
    Rs1Prime, // rs1' in CL format (bits 7-9, actual reg = +8)
    Imm6,     // 6-bit CI immediate (bit12 << 5 | bits2-6)
    ImmCL,    // CL-format immediate for c.ld: {bits[6:5], bits[12:10]} << 3
    ImmCLW,   // CL-format immediate for c.flw: {bit[5], bits[12:10], bit[6], 0}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVRegisterClass {
    Gpr,
    Fpr,
    GprPrime, // compressed 3-bit register (actual reg = raw + 8)
    FprPrime, // compressed 3-bit float register (actual reg = raw + 8)
}

robustone_isa::format_specs! {
    format R_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        rs2: robustone_isa::field("rs2", 20, 5, RiscVField::Rs2),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
        funct7: robustone_isa::field("funct7", 25, 7, RiscVField::Funct7),
    }
    format I_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        imm12: robustone_isa::field("imm12", 20, 12, RiscVField::Imm12),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
    }
    format S_TYPE[RiscVField] {
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        rs2: robustone_isa::field("rs2", 20, 5, RiscVField::Rs2),
        imm12s: robustone_isa::field("imm12s", 0, 12, RiscVField::Imm12S),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
    }
    format B_TYPE[RiscVField] {
        rs1: robustone_isa::field("rs1", 15, 5, RiscVField::Rs1),
        rs2: robustone_isa::field("rs2", 20, 5, RiscVField::Rs2),
        imm12b: robustone_isa::field("imm12b", 0, 12, RiscVField::Imm12B),
        funct3: robustone_isa::field("funct3", 12, 3, RiscVField::Funct3),
    }
    format U_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        imm20u: robustone_isa::field("imm20u", 0, 20, RiscVField::Imm20U),
    }
    format J_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        imm20j: robustone_isa::field("imm20j", 0, 20, RiscVField::Imm20J),
    }
    format CI_TYPE[RiscVField] {
        rd: robustone_isa::field("rd", 7, 5, RiscVField::Rd),
        imm6: robustone_isa::field("imm6", 2, 6, RiscVField::Imm6),
    }
    format CR_TYPE[RiscVField] {
        rs1: robustone_isa::field("rs1", 7, 5, RiscVField::Rs1),
        rs2: robustone_isa::field("rs2", 2, 5, RiscVField::Rs2C),
    }
    format CA_TYPE[RiscVField] {
        rd_prime: robustone_isa::field("rd_prime", 7, 3, RiscVField::RdPrime),
        rs2_prime: robustone_isa::field("rs2_prime", 2, 3, RiscVField::Rs2Prime),
    }
    format CL_TYPE[RiscVField] {
        rd_prime: robustone_isa::field("rd_prime", 2, 3, RiscVField::RdPrime),
        rs1_prime: robustone_isa::field("rs1_prime", 7, 3, RiscVField::Rs1Prime),
        imm_cl: robustone_isa::field("imm_cl", 5, 5, RiscVField::ImmCL),
        imm_clw: robustone_isa::field("imm_clw", 5, 5, RiscVField::ImmCLW),
    }
}

robustone_isa::isa_specs! {
    backend = RiscVBackend;
    spec ADD {
        mnemonic = "add";
        opcode_id = "ADD";
        pattern = robustone_isa::mask_value!(0xFE00_007F, 0x0000_0033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec ADDI {
        mnemonic = "addi";
        opcode_id = "ADDI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0013);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec LW {
        mnemonic = "lw";
        opcode_id = "LW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2003);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec SW {
        mnemonic = "sw";
        opcode_id = "SW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2023);
        format = &S_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec BEQ {
        mnemonic = "beq";
        opcode_id = "BEQ";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0063);
        format = &B_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12B, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 12, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec BNE {
        mnemonic = "bne";
        opcode_id = "BNE";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_1063);
        format = &B_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12B, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 12, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec JAL {
        mnemonic = "jal";
        opcode_id = "JAL";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_006F);
        format = &J_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20J, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec JALR {
        mnemonic = "jalr";
        opcode_id = "JALR";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0067);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec LUI {
        mnemonic = "lui";
        opcode_id = "LUI";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_0037);
        format = &U_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec AUIPC {
        mnemonic = "auipc";
        opcode_id = "AUIPC";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_0017);
        format = &U_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 12 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec MUL {
        mnemonic = "mul";
        opcode_id = "MUL";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0200_0033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::M;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec DIV {
        mnemonic = "div";
        opcode_id = "DIV";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0200_4033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::M;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec REM {
        mnemonic = "rem";
        opcode_id = "REM";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0200_6033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::M;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec LR_W {
        mnemonic = "lr.w";
        opcode_id = "LR_W";
        pattern = robustone_isa::mask_value!(0xF9F0_707F, 0x1000_202F);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem!(RiscVRegisterClass::Gpr, RiscVField::Rs1, 0),
        ];
        features = RiscVFeature::A;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Atomic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec SC_W {
        mnemonic = "sc.w";
        opcode_id = "SC_W";
        pattern = robustone_isa::mask_value!(0xF800_707F, 0x1800_202F);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem!(RiscVRegisterClass::Gpr, RiscVField::Rs1, 0),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::A;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Atomic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FLW {
        mnemonic = "flw";
        opcode_id = "FLW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2007);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FSW {
        mnemonic = "fsw";
        opcode_id = "FSW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2027);
        format = &S_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FADD_S {
        mnemonic = "fadd.s";
        opcode_id = "FADD_S";
        pattern = robustone_isa::mask_value!(0xFE00_007F, 0x0000_0053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FSUB_S {
        mnemonic = "fsub.s";
        opcode_id = "FSUB_S";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0800_0053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FMUL_S {
        mnemonic = "fmul.s";
        opcode_id = "FMUL_S";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x1000_0053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec FDIV_S {
        mnemonic = "fdiv.s";
        opcode_id = "FDIV_S";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x1800_0053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec ORI {
        mnemonic = "ori";
        opcode_id = "ORI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_6013);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec CSRRS {
        mnemonic = "csrrs";
        opcode_id = "CSRRS";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2073);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    // Compressed specs
    spec C_ADDIW {
        mnemonic = "c.addiw";
        opcode_id = "C_ADDIW";
        pattern = robustone_isa::mask_value!(0xE003, 0x2001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_LUI {
        mnemonic = "c.lui";
        opcode_id = "C_LUI";
        pattern = robustone_isa::mask_value!(0xE003, 0x6001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_JR {
        mnemonic = "c.jr";
        opcode_id = "C_JR";
        pattern = robustone_isa::mask_value!(0xF003, 0x8002);
        format = &CR_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_SUBW {
        mnemonic = "c.subw";
        opcode_id = "C_SUBW";
        pattern = robustone_isa::mask_value!(0xFC63, 0x9C01);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_ADDW {
        mnemonic = "c.addw";
        opcode_id = "C_ADDW";
        pattern = robustone_isa::mask_value!(0xFC63, 0x9C21);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec MULW {
        mnemonic = "mulw";
        opcode_id = "MULW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0200_003B);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::M;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec AMOADD_W {
        mnemonic = "amoadd.w";
        opcode_id = "AMOADD_W";
        pattern = robustone_isa::mask_value!(0xF800_707F, 0x0000_202F);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem!(RiscVRegisterClass::Gpr, RiscVField::Rs1, 0),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::A;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Atomic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec LD {
        mnemonic = "ld";
        opcode_id = "LD";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_3003);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec LR_D {
        mnemonic = "lr.d";
        opcode_id = "LR_D";
        pattern = robustone_isa::mask_value!(0xF9F0_707F, 0x1000_302F);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem!(RiscVRegisterClass::Gpr, RiscVField::Rs1, 0),
        ];
        features = RiscVFeature::A;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Atomic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_ADDI {
        mnemonic = "c.addi";
        opcode_id = "C_ADDI";
        pattern = robustone_isa::mask_value!(0xE003, 0x0001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_JAL {
        mnemonic = "c.jal";
        opcode_id = "C_JAL";
        pattern = robustone_isa::mask_value!(0xE003, 0x2001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_LD {
        mnemonic = "c.ld";
        opcode_id = "C_LD";
        pattern = robustone_isa::mask_value!(0xE003, 0x6000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    spec C_FLW {
        mnemonic = "c.flw";
        opcode_id = "C_FLW";
        pattern = robustone_isa::mask_value!(0xE003, 0x6000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}

pub static RISCV_SPECS: &[InstructionSpec<RiscVBackend>] = &[
    ADD, ADDI, ORI, LW, SW, BEQ, BNE, JAL, JALR, LUI, AUIPC, MUL, MULW, DIV, REM, LR_W, SC_W,
    AMOADD_W, LD, LR_D, FLW, FSW, FADD_S, FSUB_S, FMUL_S, FDIV_S, CSRRS, C_ADDI, C_ADDIW, C_JAL,
    C_LUI, C_LD, C_FLW, C_JR, C_SUBW, C_ADDW,
];

pub struct RiscVBackend;

impl ArchitectureBackend for RiscVBackend {
    type Word = u32;
    type Mode = RiscVMode;
    type Feature = RiscVFeature;
    type Field = RiscVField;
    type RegisterClass = RiscVRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Riscv
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() >= 2 && (bytes[0] & 0x3) != 0x3 {
            let word = (bytes[0] as u32) | ((bytes[1] as u32) << 8);
            Ok(InstructionRead {
                raw: word,
                length: 2,
            })
        } else if bytes.len() >= 4 {
            let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            Ok(InstructionRead {
                raw: word,
                length: 4,
            })
        } else {
            Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("riscv".to_string()),
                "incomplete instruction",
            ))
        }
    }

    fn lookup(
        word: Self::Word,
        profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        let is_compressed = (word & 0x3) != 0x3;

        // First pass: exact match including mode and features.
        let exact = RISCV_SPECS.iter().find(|spec| {
            (word & spec.pattern.mask) == spec.pattern.value
                && spec.modes.matches(profile.mode)
                && profile.features.contains(spec.features)
        });
        if exact.is_some() {
            return exact;
        }

        if is_compressed {
            // If there is a spec that matches encoding + mode (ignoring features),
            // the encoding is defined for this mode but the feature is missing.
            // Legacy decoder returns InvalidEncoding for such compressed cases
            // (e.g. c.flw without F).
            let mode_match = RISCV_SPECS.iter().find(|spec| {
                (word & spec.pattern.mask) == spec.pattern.value && spec.modes.matches(profile.mode)
            });
            if mode_match.is_some() {
                return None;
            }
            // No mode-matching spec: try to find a mode-mismatched spec so
            // decode_one can reject with UnsupportedMode (e.g. c.subw on RV32).
            return RISCV_SPECS.iter().find(|spec| {
                (word & spec.pattern.mask) == spec.pattern.value
                    && profile.features.contains(spec.features)
            });
        }

        // Second pass for standard instructions: match encoding only.
        // This lets RV32 match RV64-only specs (e.g. ld, mulw) so decode_one
        // can reject with UnsupportedMode, and I-only match A/M/F specs so
        // decode_one can reject with UnsupportedExtension.
        RISCV_SPECS
            .iter()
            .find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }

    fn lower_register(
        class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        match class {
            RiscVRegisterClass::GprPrime | RiscVRegisterClass::FprPrime => {
                RegisterId::riscv(raw + 8)
            }
            _ => RegisterId::riscv(raw),
        }
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        RenderPolicy::new(
            robustone_isa::RenderDialect::Canonical,
            robustone_isa::AliasPolicy::None,
        )
    }

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> u32 {
        match field {
            RiscVField::Imm12S => {
                let imm115 = (word >> 25) & 0x7F;
                let imm40 = (word >> 7) & 0x1F;
                (imm115 << 5) | imm40
            }
            RiscVField::Imm12B => {
                let imm12 = (word >> 31) & 1;
                let imm105 = (word >> 25) & 0x3F;
                let imm41 = (word >> 8) & 0xF;
                let imm11 = (word >> 7) & 1;
                (imm12 << 11) | (imm11 << 10) | (imm105 << 4) | imm41
            }
            RiscVField::Imm20U => (word >> 12) & 0xFFFFF,
            RiscVField::Imm20J => {
                let imm20 = (word >> 31) & 1;
                let imm101 = (word >> 21) & 0x3FF;
                let imm11 = (word >> 20) & 1;
                let imm1912 = (word >> 12) & 0xFF;
                (imm20 << 19) | (imm1912 << 11) | (imm11 << 10) | imm101
            }
            RiscVField::Imm6 => {
                let high = (word >> 12) & 1;
                let low = (word >> 2) & 0x1F;
                (high << 5) | low
            }
            RiscVField::ImmCL => {
                // CL-format immediate for c.ld: {bits[6:5], bits[12:10]} << 3
                let low = (word >> 5) & 0x3;
                let high = (word >> 8) & 0x1C;
                (low | high) << 3
            }
            RiscVField::ImmCLW => {
                // CL-format immediate for c.flw: {bit[5], bits[12:10], bit[6], 0}
                let bit5 = (word >> 5) & 1;
                let bits12_10 = (word >> 10) & 0x7;
                let bit6 = (word >> 6) & 1;
                (bit5 << 6) | (bits12_10 << 3) | (bit6 << 2)
            }
            _ => {
                for f in format.fields {
                    if f.field_type == field {
                        let mask = ((1u64 << f.length) - 1) as u32;
                        return (word >> f.start) & mask;
                    }
                }
                0
            }
        }
    }
}

pub type RiscVIsaDecoder = robustone_isa::Decoder<RiscVBackend>;
