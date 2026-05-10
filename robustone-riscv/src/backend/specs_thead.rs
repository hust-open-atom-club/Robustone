robustone_isa_macros::define_instructions! {
    arch = RiscV; module = thead;
    insn TH_MVEQZ {
        mnemonic = "th.mveqz";
        opcode_id = "TH_MVEQZ";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_100B);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::THEAD;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer];
        manual = "XTheadCondMov Specification";
    }
    insn TH_MVNEZ {
        mnemonic = "th.mvnez";
        opcode_id = "TH_MVNEZ";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4200_100B);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::THEAD;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer];
        manual = "XTheadCondMov Specification";
    }
}
