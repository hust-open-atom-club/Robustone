robustone_isa_macros::define_instructions! {
    arch = RiscV; module = a;
    insn LR_W {
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
        effect = Memory;
    }
    insn SC_W {
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
        effect = Memory;
    }
    insn AMOADD_W {
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
        effect = Memory;
    }
    insn LR_D {
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
        effect = Memory;
    }
}
