robustone_isa_macros::define_instructions! {
    arch = RiscV; module = system;
    insn CSRRS {
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
        effect = Privileged;
    }
    insn MRET {
        mnemonic = "mret";
        opcode_id = "MRET";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x3020_0073);
        format = &I_TYPE;
        operands = &[];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "RISC-V Privileged ISA";
        effect = Return;
    }
}
