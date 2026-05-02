robustone_isa_macros::define_instructions! {
    arch = RiscV; module = d;
    insn FCVT_S_D {
        mnemonic = "fcvt.s.d";
        opcode_id = "FCVT_S_D";
        pattern = robustone_isa::mask_value!(0xFFF0_007F, 0x4010_0053);
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
