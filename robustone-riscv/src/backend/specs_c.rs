robustone_isa_macros::define_instructions! {
    arch = RiscV; module = c;
    insn C_ADDIW {
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
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_LUI {
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
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_JR {
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
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs1 }];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Return;
    }
    insn C_SUBW {
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
    insn C_ADDW {
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
    insn C_ADDI {
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
    insn C_JAL {
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
        effect = Call;
    }
    insn C_LD {
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
        effect = Memory;
    }
    insn C_FLW {
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
        effect = Memory;
    }
    insn C_FLD {
        mnemonic = "c.fld";
        opcode_id = "C_FLD";
        pattern = robustone_isa::mask_value!(0xE003, 0x2000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SW {
        mnemonic = "c.sw";
        opcode_id = "C_SW";
        pattern = robustone_isa::mask_value!(0xE003, 0xC000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SD {
        mnemonic = "c.sd";
        opcode_id = "C_SD";
        pattern = robustone_isa::mask_value!(0xE003, 0xE000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FSW {
        mnemonic = "c.fsw";
        opcode_id = "C_FSW";
        pattern = robustone_isa::mask_value!(0xE003, 0xE000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FSD {
        mnemonic = "c.fsd";
        opcode_id = "C_FSD";
        pattern = robustone_isa::mask_value!(0xE003, 0xA000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
}
