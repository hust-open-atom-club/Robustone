robustone_isa_macros::define_instructions! {
    arch = RiscV; module = f;
    insn FLW {
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
        effect = Memory;
    }
    insn FSW {
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
        effect = Memory;
    }
    insn FADD_S {
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
    insn FSUB_S {
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
    insn FMUL_S {
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
    insn FDIV_S {
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
}
