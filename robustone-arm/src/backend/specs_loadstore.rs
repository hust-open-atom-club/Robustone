robustone_isa_macros::define_instructions! {
    arch = Arm; module = loadstore;

    // -------------------------------------------------------------------------
    // Load/Store Register (unsigned immediate) — 64-bit
    // -------------------------------------------------------------------------
    insn LDR_X {
        mnemonic = "ldr";
        opcode_id = "LDR_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xF940_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STR_X {
        mnemonic = "str";
        opcode_id = "STR_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xF900_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Register (unsigned immediate) — 32-bit
    // -------------------------------------------------------------------------
    insn LDR_W {
        mnemonic = "ldr";
        opcode_id = "LDR_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xB940_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STR_W {
        mnemonic = "str";
        opcode_id = "STR_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xB900_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Byte (unsigned immediate)
    // -------------------------------------------------------------------------
    insn LDRB {
        mnemonic = "ldrb";
        opcode_id = "LDRB";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x3940_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STRB {
        mnemonic = "strb";
        opcode_id = "STRB";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x3900_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Halfword (unsigned immediate)
    // -------------------------------------------------------------------------
    insn LDRH {
        mnemonic = "ldrh";
        opcode_id = "LDRH";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x7940_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STRH {
        mnemonic = "strh";
        opcode_id = "STRH";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x7900_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load Signed (unsigned immediate)
    // -------------------------------------------------------------------------
    insn LDRSB_X {
        mnemonic = "ldrsb";
        opcode_id = "LDRSB_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x3980_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDRSB_W {
        mnemonic = "ldrsb";
        opcode_id = "LDRSB_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x39C0_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDRSH_X {
        mnemonic = "ldrsh";
        opcode_id = "LDRSH_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x7980_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDRSH_W {
        mnemonic = "ldrsh";
        opcode_id = "LDRSH_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x79C0_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDRSW {
        mnemonic = "ldrsw";
        opcode_id = "LDRSW";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xB980_0000);
        format = &LDR_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load Literal (PC-relative)
    // -------------------------------------------------------------------------
    insn LDR_LIT_W {
        mnemonic = "ldr";
        opcode_id = "LDR_LIT_W";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x1800_0000);
        format = &LDR_LIT;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_LIT_X {
        mnemonic = "ldr";
        opcode_id = "LDR_LIT_X";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x5800_0000);
        format = &LDR_LIT;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDRSW_LIT {
        mnemonic = "ldrsw";
        opcode_id = "LDRSW_LIT";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x9800_0000);
        format = &LDR_LIT;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Pair
    // -------------------------------------------------------------------------
    insn LDP_X {
        mnemonic = "ldp";
        opcode_id = "LDP_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xA940_0000);
        format = &LDP;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm7, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STP_X {
        mnemonic = "stp";
        opcode_id = "STP_X";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xA900_0000);
        format = &LDP;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm7, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDP_W {
        mnemonic = "ldp";
        opcode_id = "LDP_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x2940_0000);
        format = &LDP;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm7, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STP_W {
        mnemonic = "stp";
        opcode_id = "STP_W";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x2900_0000);
        format = &LDP;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm7, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Exclusive
    // -------------------------------------------------------------------------
    insn STXR_W {
        mnemonic = "stxr";
        opcode_id = "STXR_W";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0x88000000);
        format = &LDR_EXCL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rs, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STXR_X {
        mnemonic = "stxr";
        opcode_id = "STXR_X";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0xC8000000);
        format = &LDR_EXCL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rs, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDXR_W {
        mnemonic = "ldxr";
        opcode_id = "LDXR_W";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0x88400000);
        format = &LDR_EXCL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDXR_X {
        mnemonic = "ldxr";
        opcode_id = "LDXR_X";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0xC8400000);
        format = &LDR_EXCL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STXP_W {
        mnemonic = "stxp";
        opcode_id = "STXP_W";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0x88200000);
        format = &LDR_EXCL_PAIR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rs, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STXP_X {
        mnemonic = "stxp";
        opcode_id = "STXP_X";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0xC8200000);
        format = &LDR_EXCL_PAIR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rs, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDXP_W {
        mnemonic = "ldxp";
        opcode_id = "LDXP_W";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0x88600000);
        format = &LDR_EXCL_PAIR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDXP_X {
        mnemonic = "ldxp";
        opcode_id = "LDXP_X";
        pattern = robustone_isa::mask_value!(0xFFE00000, 0xC8600000);
        format = &LDR_EXCL_PAIR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt2, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Register (post-indexed)
    // -------------------------------------------------------------------------
    insn STR_POST_W {
        mnemonic = "str";
        opcode_id = "STR_POST_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8000400);
        format = &LDR_POST;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_POST_W {
        mnemonic = "ldr";
        opcode_id = "LDR_POST_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8400400);
        format = &LDR_POST;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STR_POST_X {
        mnemonic = "str";
        opcode_id = "STR_POST_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8000400);
        format = &LDR_POST;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_POST_X {
        mnemonic = "ldr";
        opcode_id = "LDR_POST_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8400400);
        format = &LDR_POST;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Register (pre-indexed)
    // -------------------------------------------------------------------------
    insn STR_PRE_W {
        mnemonic = "str";
        opcode_id = "STR_PRE_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8000C00);
        format = &LDR_PRE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_PRE_W {
        mnemonic = "ldr";
        opcode_id = "LDR_PRE_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8400C00);
        format = &LDR_PRE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STR_PRE_X {
        mnemonic = "str";
        opcode_id = "STR_PRE_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8000C00);
        format = &LDR_PRE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDR_PRE_X {
        mnemonic = "ldr";
        opcode_id = "LDR_PRE_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8400C00);
        format = &LDR_PRE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Load/Store Register (unscaled immediate)
    // -------------------------------------------------------------------------
    insn STUR_W {
        mnemonic = "stur";
        opcode_id = "STUR_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8000000);
        format = &LDR_UNSCALED;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDUR_W {
        mnemonic = "ldur";
        opcode_id = "LDUR_W";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xB8400000);
        format = &LDR_UNSCALED;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn STUR_X {
        mnemonic = "stur";
        opcode_id = "STUR_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8000000);
        format = &LDR_UNSCALED;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }
    insn LDUR_X {
        mnemonic = "ldur";
        opcode_id = "LDUR_X";
        pattern = robustone_isa::mask_value!(0xFFE00C00, 0xF8400000);
        format = &LDR_UNSCALED;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm9, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::Absolute),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Memory];
        manual = "ARM ARM";
    }

}
