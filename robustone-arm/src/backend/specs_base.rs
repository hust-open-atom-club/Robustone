robustone_isa_macros::define_instructions! {
    arch = Arm; module = base;

    // -------------------------------------------------------------------------
    // System / Misc
    // -------------------------------------------------------------------------
    insn NOP {
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0xD5_03_20_1F);
        format = &R_TYPE;
        operands = &[];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::System];
        effect = None;
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Immediate: Add/Subtract
    // -------------------------------------------------------------------------
    insn ADD_IMM {
        mnemonic = "add";
        opcode_id = "ADD_IMM";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0x9100_0000);
        format = &I_ADD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn SUB_IMM {
        mnemonic = "sub";
        opcode_id = "SUB_IMM";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0xD100_0000);
        format = &I_ADD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn ADDS_IMM {
        mnemonic = "adds";
        opcode_id = "ADDS_IMM";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0xB100_0000);
        format = &I_ADD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn SUBS_IMM {
        mnemonic = "subs";
        opcode_id = "SUBS_IMM";
        pattern = robustone_isa::mask_value!(0xFF00_0000, 0xF100_0000);
        format = &I_ADD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Immediate: Logical
    // -------------------------------------------------------------------------
    insn AND_IMM {
        mnemonic = "and";
        opcode_id = "AND_IMM";
        pattern = robustone_isa::mask_value!(0xFF80_0000, 0x9200_0000);
        format = &I_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn ORR_IMM {
        mnemonic = "orr";
        opcode_id = "ORR_IMM";
        pattern = robustone_isa::mask_value!(0xFF80_0000, 0xB200_0000);
        format = &I_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn EOR_IMM {
        mnemonic = "eor";
        opcode_id = "EOR_IMM";
        pattern = robustone_isa::mask_value!(0xFF80_0000, 0xD200_0000);
        format = &I_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn ANDS_IMM {
        mnemonic = "ands";
        opcode_id = "ANDS_IMM";
        pattern = robustone_isa::mask_value!(0xFF80_0000, 0xF200_0000);
        format = &I_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Immediate: Move Wide
    // -------------------------------------------------------------------------
    insn MOVZ {
        mnemonic = "movz";
        opcode_id = "MOVZ";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xD280_0000);
        format = &I_MOVZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn MOVN {
        mnemonic = "movn";
        opcode_id = "MOVN";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0x9280_0000);
        format = &I_MOVZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn MOVK {
        mnemonic = "movk";
        opcode_id = "MOVK";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xF280_0000);
        format = &I_MOVZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Immediate: Bitfield
    // -------------------------------------------------------------------------
    insn SBFM {
        mnemonic = "sbfm";
        opcode_id = "SBFM";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0x9340_0000);
        format = &I_BITFIELD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Immr, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn BFM {
        mnemonic = "bfm";
        opcode_id = "BFM";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xB340_0000);
        format = &I_BITFIELD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::ReadWrite),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Immr, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn UBFM {
        mnemonic = "ubfm";
        opcode_id = "UBFM";
        pattern = robustone_isa::mask_value!(0xFFC0_0000, 0xD340_0000);
        format = &I_BITFIELD;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Immr, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
            robustone_isa::imm!(ArmField::Imms, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Immediate: PC-relative Address
    // -------------------------------------------------------------------------
    insn ADR {
        mnemonic = "adr";
        opcode_id = "ADR";
        pattern = robustone_isa::mask_value!(0x9F00_0000, 0x1000_0000);
        format = &FMT_ADR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 29, src_length: 2, dst_start: 0 },
                    robustone_isa::ImmComposePart { src_start: 5, src_length: 19, dst_start: 2 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 21 },
                kind = robustone_isa::ImmediateKind::PcRelative
            ),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn ADRP {
        mnemonic = "adrp";
        opcode_id = "ADRP";
        pattern = robustone_isa::mask_value!(0x9F00_0000, 0x9000_0000);
        format = &FMT_ADR;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 29, src_length: 2, dst_start: 0 },
                    robustone_isa::ImmComposePart { src_start: 5, src_length: 19, dst_start: 2 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 21, shift: 12 },
                kind = robustone_isa::ImmediateKind::PcRelative
            ),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Register: Add/Subtract (shifted register)
    // -------------------------------------------------------------------------
    insn ADD_REG {
        mnemonic = "add";
        opcode_id = "ADD_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0x8B00_0000);
        format = &R_DP_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn SUB_REG {
        mnemonic = "sub";
        opcode_id = "SUB_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xCB00_0000);
        format = &R_DP_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn ADDS_REG {
        mnemonic = "adds";
        opcode_id = "ADDS_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xAB00_0000);
        format = &R_DP_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn SUBS_REG {
        mnemonic = "subs";
        opcode_id = "SUBS_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xEB00_0000);
        format = &R_DP_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Register: Logical (shifted register)
    // -------------------------------------------------------------------------
    insn AND_REG {
        mnemonic = "and";
        opcode_id = "AND_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0x8A00_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn BIC_REG {
        mnemonic = "bic";
        opcode_id = "BIC_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0x8A20_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn ORR_REG {
        mnemonic = "orr";
        opcode_id = "ORR_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xAA00_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn ORN_REG {
        mnemonic = "orn";
        opcode_id = "ORN_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xAA20_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn EOR_REG {
        mnemonic = "eor";
        opcode_id = "EOR_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xCA00_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn EON_REG {
        mnemonic = "eon";
        opcode_id = "EON_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xCA20_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn ANDS_REG {
        mnemonic = "ands";
        opcode_id = "ANDS_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xEA00_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }
    insn BICS_REG {
        mnemonic = "bics";
        opcode_id = "BICS_REG";
        pattern = robustone_isa::mask_value!(0xFFE0_0000, 0xEA20_0000);
        format = &R_LOGICAL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Register: Conditional Select
    // -------------------------------------------------------------------------
    insn CSEL {
        mnemonic = "csel";
        opcode_id = "CSEL";
        pattern = robustone_isa::mask_value!(0xFFE0_0C00, 0x9A80_0000);
        format = &R_CONDSEL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn CSINC {
        mnemonic = "csinc";
        opcode_id = "CSINC";
        pattern = robustone_isa::mask_value!(0xFFE0_0C00, 0x9A80_0400);
        format = &R_CONDSEL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn CSINV {
        mnemonic = "csinv";
        opcode_id = "CSINV";
        pattern = robustone_isa::mask_value!(0xFFE0_0C00, 0xBA80_0000);
        format = &R_CONDSEL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn CSNEG {
        mnemonic = "csneg";
        opcode_id = "CSNEG";
        pattern = robustone_isa::mask_value!(0xFFE0_0C00, 0xBA80_0400);
        format = &R_CONDSEL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Register: 3-source (multiply-add / multiply-sub)
    // -------------------------------------------------------------------------
    insn MADD {
        mnemonic = "madd";
        opcode_id = "MADD";
        pattern = robustone_isa::mask_value!(0xFFE0_8000, 0x9B00_0000);
        format = &R_4REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Ra, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn MSUB {
        mnemonic = "msub";
        opcode_id = "MSUB";
        pattern = robustone_isa::mask_value!(0xFFE0_8000, 0x9B00_8000);
        format = &R_4REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Ra, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Data Processing — Register: 2-source (SDIV, UDIV)
    // -------------------------------------------------------------------------
    insn SDIV {
        mnemonic = "sdiv";
        opcode_id = "SDIV";
        pattern = robustone_isa::mask_value!(0xFFE0_FC00, 0x9AC0_0800);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }
    insn UDIV {
        mnemonic = "udiv";
        opcode_id = "UDIV";
        pattern = robustone_isa::mask_value!(0xFFE0_FC00, 0x9AC0_0000);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Branch — Unconditional Branch (Register)
    // -------------------------------------------------------------------------
    insn BR {
        mnemonic = "br";
        opcode_id = "BR";
        pattern = robustone_isa::mask_value!(0xFFFF_FC1F, 0xD61F_0000);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
    insn BLR {
        mnemonic = "blr";
        opcode_id = "BLR";
        pattern = robustone_isa::mask_value!(0xFFFF_FC1F, 0xD63F_0000);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
    insn RET {
        mnemonic = "ret";
        opcode_id = "RET";
        pattern = robustone_isa::mask_value!(0xFFFF_FC1F, 0xD65F_0000);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
}
