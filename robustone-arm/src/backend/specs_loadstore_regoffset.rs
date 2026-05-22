robustone_isa_macros::define_instructions! {
    arch = Arm; module = loadstore_regoffset;

    // -------------------------------------------------------------------------
    // Load/Store Register (register offset)
    //
    // These four specs intentionally overlap in the common mask region
    // (bit 31=1, bit 29:27=111, bit 26:23=0000, bit 21=1, bit 11:10=10).
    // They are distinguished by bit 30 (size: 0=W, 1=X) and bit 22 (L: 0=STR, 1=LDR).
    // Priority ordering ensures the correct spec wins in arm_lookup.
    // -------------------------------------------------------------------------
    insn STR_REG_W {
        mnemonic = "str";
        opcode_id = "STR_REG_W";
        pattern = robustone_isa::mask_value!(0xB8200800, 0xB8200800);
        format = &LDR_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_REG_W {
        mnemonic = "ldr";
        opcode_id = "LDR_REG_W";
        pattern = robustone_isa::mask_value!(0xB8600800, 0xB8600800);
        priority = 1;
        format = &LDR_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STR_REG_X {
        mnemonic = "str";
        opcode_id = "STR_REG_X";
        pattern = robustone_isa::mask_value!(0xF8200800, 0xF8200800);
        priority = 2;
        format = &LDR_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_REG_X {
        mnemonic = "ldr";
        opcode_id = "LDR_REG_X";
        pattern = robustone_isa::mask_value!(0xF8600800, 0xF8600800);
        priority = 3;
        format = &LDR_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
}
