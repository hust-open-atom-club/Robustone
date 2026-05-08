robustone_isa_macros::define_instructions! {
    arch = RiscV; module = d;
    insn FMADD_D {
        mnemonic = "fmadd.d";
        opcode_id = "FMADD_D";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x02000043);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FADD_D {
        mnemonic = "fadd.d";
        opcode_id = "FADD_D";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x02000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSQRT_D {
        mnemonic = "fsqrt.d";
        opcode_id = "FSQRT_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x5A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMIN_D {
        mnemonic = "fmin.d";
        opcode_id = "FMIN_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x2A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FEQ_D {
        mnemonic = "feq.d";
        opcode_id = "FEQ_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA2002053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_D {
        mnemonic = "fcvt.s.d";
        opcode_id = "FCVT_S_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x40100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}
