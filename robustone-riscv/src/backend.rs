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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVRegisterClass {
    Gpr,
    Fpr,
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
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
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
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
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
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::A;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Atomic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}

pub static RISCV_SPECS: &[InstructionSpec<RiscVBackend>] = &[
    ADD, ADDI, LW, SW, BEQ, BNE, JAL, JALR, LUI, AUIPC, MUL, DIV, REM, LR_W, SC_W,
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
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        RISCV_SPECS
            .iter()
            .find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        RegisterId::riscv(raw)
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
