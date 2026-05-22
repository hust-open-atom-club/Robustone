robustone_isa_macros::define_instructions! {
    arch = Arm; module = system;

    // -------------------------------------------------------------------------
    // Streaming vector-length arithmetic
    // -------------------------------------------------------------------------
    insn ADDSVL_XXI {
        mnemonic = "addsvl";
        opcode_id = "ADDSVL_XXI";
        pattern = robustone_isa::mask_value!(0xFFE0_F800, 0x0420_5800);
        format = &SVE_RI;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SME;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn ADDSPL_XXI {
        mnemonic = "addspl";
        opcode_id = "ADDSPL_XXI";
        pattern = robustone_isa::mask_value!(0xFFE0_F800, 0x0460_5800);
        format = &SVE_RI;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SME;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Streaming vector-length reads
    // -------------------------------------------------------------------------
    insn RDSVLI_XI {
        mnemonic = "rdsvl";
        opcode_id = "RDSVLI_XI";
        pattern = robustone_isa::mask_value!(0xFFFF_F800, 0x04BF_5800);
        format = &SVE_RI;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SME;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }


    insn SVC {
        mnemonic = "svc";
        opcode_id = "SVC";
        pattern = robustone_isa::mask_value!(0xFFE0_001F, 0xD400_0001);
        format = &EXCEPT;
        operands = &[
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn HVC {
        mnemonic = "hvc";
        opcode_id = "HVC";
        pattern = robustone_isa::mask_value!(0xFFE0_001F, 0xD400_0002);
        format = &EXCEPT;
        operands = &[
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn SMC {
        mnemonic = "smc";
        opcode_id = "SMC";
        pattern = robustone_isa::mask_value!(0xFFE0_001F, 0xD400_0003);
        format = &EXCEPT;
        operands = &[
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn BRK {
        mnemonic = "brk";
        opcode_id = "BRK";
        pattern = robustone_isa::mask_value!(0xFFE0_001F, 0xD420_0000);
        format = &EXCEPT;
        operands = &[
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn HLT {
        mnemonic = "hlt";
        opcode_id = "HLT";
        pattern = robustone_isa::mask_value!(0xFFE0_001F, 0xD440_0000);
        format = &EXCEPT;
        operands = &[
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Barriers
    // -------------------------------------------------------------------------
    insn DSB {
        mnemonic = "dsb";
        opcode_id = "DSB";
        pattern = robustone_isa::mask_value!(0xFFFF_F0FF, 0xD503_309F);
        format = &BARRIER;
        operands = &[
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn DMB {
        mnemonic = "dmb";
        opcode_id = "DMB";
        pattern = robustone_isa::mask_value!(0xFFFF_F0FF, 0xD503_30BF);
        format = &BARRIER;
        operands = &[
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn ISB {
        mnemonic = "isb";
        opcode_id = "ISB";
        pattern = robustone_isa::mask_value!(0xFFFF_F0FF, 0xD503_30DF);
        format = &BARRIER;
        operands = &[
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn CLREX {
        mnemonic = "clrex";
        opcode_id = "CLREX";
        pattern = robustone_isa::mask_value!(0xFFFF_F0FF, 0xD503_305F);
        format = &BARRIER;
        operands = &[
            robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Exception return
    // -------------------------------------------------------------------------
    insn ERET {
        mnemonic = "eret";
        opcode_id = "ERET";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD69F_03E0);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn DRPS {
        mnemonic = "drps";
        opcode_id = "DRPS";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD6BF_03E0);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // HINT variants
    // -------------------------------------------------------------------------
    insn YIELD {
        mnemonic = "yield";
        opcode_id = "YIELD";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD503_203F);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn WFE {
        mnemonic = "wfe";
        opcode_id = "WFE";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD503_205F);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn WFI {
        mnemonic = "wfi";
        opcode_id = "WFI";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD503_207F);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn SEV {
        mnemonic = "sev";
        opcode_id = "SEV";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD503_209F);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
    insn SEVL {
        mnemonic = "sevl";
        opcode_id = "SEVL";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD503_20BF);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "ARM ARM";
    }
}
