robustone_isa_macros::define_instructions! {
    arch = Arm; module = vector;

    // -------------------------------------------------------------------------
    // Advanced SIMD Three Same — Integer arithmetic
    // mask = 0xBF20_FC00: bit31=0, bit24=0, bits28:25=0111, bit21=1, bit10=1
    // -------------------------------------------------------------------------
    insn VEC_ADD {
        mnemonic = "add";
        opcode_id = "VEC_ADD";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x0E20_8400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SUB {
        mnemonic = "sub";
        opcode_id = "VEC_SUB";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x2E20_8400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MUL {
        mnemonic = "mul";
        opcode_id = "VEC_MUL";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x0E20_9C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MLA {
        mnemonic = "mla";
        opcode_id = "VEC_MLA";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x0E20_9400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MLS {
        mnemonic = "mls";
        opcode_id = "VEC_MLS";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x2E20_9400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMAX {
        mnemonic = "smax";
        opcode_id = "VEC_SMAX";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x0E20_6400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMAX {
        mnemonic = "umax";
        opcode_id = "VEC_UMAX";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x2E20_6400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMIN {
        mnemonic = "smin";
        opcode_id = "VEC_SMIN";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x0E20_6C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMIN {
        mnemonic = "umin";
        opcode_id = "VEC_UMIN";
        pattern = robustone_isa::mask_value!(0xBF20_FC00, 0x2E20_6C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Three Same — Logical
    // mask = 0xBFE0_FC00: bit31=0, bit24=0, bits28:25=0111, bits23:21=xxx, bit10=1
    // -------------------------------------------------------------------------
    insn VEC_AND {
        mnemonic = "and";
        opcode_id = "VEC_AND";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x0E20_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_ORR {
        mnemonic = "orr";
        opcode_id = "VEC_ORR";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x0EA0_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_BIC {
        mnemonic = "bic";
        opcode_id = "VEC_BIC";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x0E60_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_ORN {
        mnemonic = "orn";
        opcode_id = "VEC_ORN";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x0EE0_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_EOR {
        mnemonic = "eor";
        opcode_id = "VEC_EOR";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x2E20_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_BIT {
        mnemonic = "bit";
        opcode_id = "VEC_BIT";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x2EA0_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_BSL {
        mnemonic = "bsl";
        opcode_id = "VEC_BSL";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x2E60_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_BIF {
        mnemonic = "bif";
        opcode_id = "VEC_BIF";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x2EE0_1C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Three Same — FP vector (non-FP16: S/D)
    // mask = 0xBFA0_FC00: bit31=0, bit24=0, bits28:25=0111, bit21=1, bit10=1
    // -------------------------------------------------------------------------
    insn VEC_FADD {
        mnemonic = "fadd";
        opcode_id = "VEC_FADD";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0E20_D400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FSUB {
        mnemonic = "fsub";
        opcode_id = "VEC_FSUB";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0EA0_D400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMUL {
        mnemonic = "fmul";
        opcode_id = "VEC_FMUL";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x2E20_DC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FDIV {
        mnemonic = "fdiv";
        opcode_id = "VEC_FDIV";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x2EA0_FC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMAX {
        mnemonic = "fmax";
        opcode_id = "VEC_FMAX";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0E20_F400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMIN {
        mnemonic = "fmin";
        opcode_id = "VEC_FMIN";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0EA0_F400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMAXNM {
        mnemonic = "fmaxnm";
        opcode_id = "VEC_FMAXNM";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0E20_C400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMINNM {
        mnemonic = "fminnm";
        opcode_id = "VEC_FMINNM";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0EA0_C400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMLA {
        mnemonic = "fmla";
        opcode_id = "VEC_FMLA";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0E20_CC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMLS {
        mnemonic = "fmls";
        opcode_id = "VEC_FMLS";
        pattern = robustone_isa::mask_value!(0xBFA0_FC00, 0x0EA0_CC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Two-register Miscellaneous
    // mask = 0xBF3F_F400: bit31=0, bit24=0, bits28:25=0111, bit21=1,
    //                     bits20:16=opcode, bits15:12=opcode_ext, bit10=0
    // -------------------------------------------------------------------------
    insn VEC_REV64 {
        mnemonic = "rev64";
        opcode_id = "VEC_REV64";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_0000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_REV32 {
        mnemonic = "rev32";
        opcode_id = "VEC_REV32";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x2E20_0000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_REV16 {
        mnemonic = "rev16";
        opcode_id = "VEC_REV16";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_1000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SADDLP {
        mnemonic = "saddlp";
        opcode_id = "VEC_SADDLP";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_2000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UADDLP {
        mnemonic = "uaddlp";
        opcode_id = "VEC_UADDLP";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x2E20_2000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_CLS {
        mnemonic = "cls";
        opcode_id = "VEC_CLS";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_4000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_CLZ {
        mnemonic = "clz";
        opcode_id = "VEC_CLZ";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x2E20_4000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_CNT {
        mnemonic = "cnt";
        opcode_id = "VEC_CNT";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_5000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MVN {
        mnemonic = "mvn";
        opcode_id = "VEC_MVN";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x2E20_5000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_ABS {
        mnemonic = "abs";
        opcode_id = "VEC_ABS";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x0E20_B000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_NEG {
        mnemonic = "neg";
        opcode_id = "VEC_NEG";
        pattern = robustone_isa::mask_value!(0xBF3F_F400, 0x2E20_B000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SABD {
        mnemonic = "sabd";
        opcode_id = "VEC_SABD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E207400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UABD {
        mnemonic = "uabd";
        opcode_id = "VEC_UABD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E207400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SABA {
        mnemonic = "saba";
        opcode_id = "VEC_SABA";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E207C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UABA {
        mnemonic = "uaba";
        opcode_id = "VEC_UABA";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E207C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SHADD {
        mnemonic = "shadd";
        opcode_id = "VEC_SHADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E200400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UHADD {
        mnemonic = "uhadd";
        opcode_id = "VEC_UHADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E200400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SRHADD {
        mnemonic = "srhadd";
        opcode_id = "VEC_SRHADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E201400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_URHADD {
        mnemonic = "urhadd";
        opcode_id = "VEC_URHADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E201400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SHSUB {
        mnemonic = "shsub";
        opcode_id = "VEC_SHSUB";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E202400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UHSUB {
        mnemonic = "uhsub";
        opcode_id = "VEC_UHSUB";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E202400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQADD {
        mnemonic = "sqadd";
        opcode_id = "VEC_SQADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E200C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UQADD {
        mnemonic = "uqadd";
        opcode_id = "VEC_UQADD";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E200C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQSUB {
        mnemonic = "sqsub";
        opcode_id = "VEC_SQSUB";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E202C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UQSUB {
        mnemonic = "uqsub";
        opcode_id = "VEC_UQSUB";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E202C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SSHL {
        mnemonic = "sshl";
        opcode_id = "VEC_SSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E204400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_USHL {
        mnemonic = "ushl";
        opcode_id = "VEC_USHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E204400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQSHL {
        mnemonic = "sqshl";
        opcode_id = "VEC_SQSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E204C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UQSHL {
        mnemonic = "uqshl";
        opcode_id = "VEC_UQSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E204C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SRSHL {
        mnemonic = "srshl";
        opcode_id = "VEC_SRSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E205400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_URSHL {
        mnemonic = "urshl";
        opcode_id = "VEC_URSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E205400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQRSHL {
        mnemonic = "sqrshl";
        opcode_id = "VEC_SQRSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E205C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UQRSHL {
        mnemonic = "uqrshl";
        opcode_id = "VEC_UQRSHL";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E205C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SMAXP {
        mnemonic = "smaxp";
        opcode_id = "VEC_SMAXP";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E20A400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UMAXP {
        mnemonic = "umaxp";
        opcode_id = "VEC_UMAXP";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E20A400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SMINP {
        mnemonic = "sminp";
        opcode_id = "VEC_SMINP";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E20AC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_UMINP {
        mnemonic = "uminp";
        opcode_id = "VEC_UMINP";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E20AC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQDMULH {
        mnemonic = "sqdmulh";
        opcode_id = "VEC_SQDMULH";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E20B400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_SQRDMULH {
        mnemonic = "sqrdmulh";
        opcode_id = "VEC_SQRDMULH";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E20B400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_CMGE_REG {
        mnemonic = "cmge";
        opcode_id = "VEC_CMGE_REG";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E203C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_CMGT_REG {
        mnemonic = "cmgt";
        opcode_id = "VEC_CMGT_REG";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E203400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_CMHI {
        mnemonic = "cmhi";
        opcode_id = "VEC_CMHI";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E203400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_CMHS {
        mnemonic = "cmhs";
        opcode_id = "VEC_CMHS";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x2E203C00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_ADDP {
        mnemonic = "addp";
        opcode_id = "VEC_ADDP";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E20BC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
insn VEC_FADDP {
        mnemonic = "faddp";
        opcode_id = "VEC_FADDP";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2E20D400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMAXP {
        mnemonic = "fmaxp";
        opcode_id = "VEC_FMAXP";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2E20F400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMINP {
        mnemonic = "fminp";
        opcode_id = "VEC_FMINP";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2EA0F400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMAXNMP {
        mnemonic = "fmaxnmp";
        opcode_id = "VEC_FMAXNMP";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2E20C400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMINNMP {
        mnemonic = "fminnmp";
        opcode_id = "VEC_FMINNMP";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2EA0C400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FCMEQ_REG {
        mnemonic = "fcmeq";
        opcode_id = "VEC_FCMEQ_REG";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x0E20E400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FCMGE_REG {
        mnemonic = "fcmge";
        opcode_id = "VEC_FCMGE_REG";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2E20E400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FCMGT_REG {
        mnemonic = "fcmgt";
        opcode_id = "VEC_FCMGT_REG";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2EA0E400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FRECPS {
        mnemonic = "frecps";
        opcode_id = "VEC_FRECPS";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x0E20FC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FRSQRTS {
        mnemonic = "frsqrts";
        opcode_id = "VEC_FRSQRTS";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x0EA0FC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMULX {
        mnemonic = "fmulx";
        opcode_id = "VEC_FMULX";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x0E20DC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FABD {
        mnemonic = "fabd";
        opcode_id = "VEC_FABD";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2EA0D400);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FACGE {
        mnemonic = "facge";
        opcode_id = "VEC_FACGE";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2E20EC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FACGT {
        mnemonic = "facgt";
        opcode_id = "VEC_FACGT";
        pattern = robustone_isa::mask_value!(0xBFA0FC00, 0x2EA0EC00);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FCVTN {
        mnemonic = "fcvtn";
        opcode_id = "VEC_FCVTN";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E216000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FSQRT {
        mnemonic = "fsqrt";
        opcode_id = "VEC_FSQRT";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E21F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FRSQRTE {
        mnemonic = "frsqrte";
        opcode_id = "VEC_FRSQRTE";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E21D000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FCMLE_ZERO {
        mnemonic = "fcmle";
        opcode_id = "VEC_FCMLE_ZERO";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E20D000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMAXV_H {
        mnemonic = "fmaxv";
        opcode_id = "VEC_FMAXV_H";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x0E30F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMAXV_S {
        mnemonic = "fmaxv";
        opcode_id = "VEC_FMAXV_S";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x2E30F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMINV_H {
        mnemonic = "fminv";
        opcode_id = "VEC_FMINV_H";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x0EB0F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_FMINV_S {
        mnemonic = "fminv";
        opcode_id = "VEC_FMINV_S";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x2EB0F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
insn VEC_UADDW {
        mnemonic = "uaddw";
        opcode_id = "VEC_UADDW";
        pattern = robustone_isa::mask_value!(0xBF20F400, 0x2E201000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    // -------------------------------------------------------------------------
    // Advanced SIMD Across Lanes (integer)
    // -------------------------------------------------------------------------
    insn VEC_SADDLV {
        mnemonic = "saddlv";
        opcode_id = "VEC_SADDLV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E303000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UADDLV {
        mnemonic = "uaddlv";
        opcode_id = "VEC_UADDLV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E303000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMAXV {
        mnemonic = "smaxv";
        opcode_id = "VEC_SMAXV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E30A000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMAXV {
        mnemonic = "umaxv";
        opcode_id = "VEC_UMAXV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E30A000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMINV {
        mnemonic = "sminv";
        opcode_id = "VEC_SMINV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E31A000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMINV {
        mnemonic = "uminv";
        opcode_id = "VEC_UMINV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E31A000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_ADDV {
        mnemonic = "addv";
        opcode_id = "VEC_ADDV";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E31B000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    // -------------------------------------------------------------------------
    // Advanced SIMD Across Lanes (FP)
    // -------------------------------------------------------------------------
    insn VEC_FMAXNMV {
        mnemonic = "fmaxnmv";
        opcode_id = "VEC_FMAXNMV";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x2E30C000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMINNMV {
        mnemonic = "fminnmv";
        opcode_id = "VEC_FMINNMV";
        pattern = robustone_isa::mask_value!(0xBFBFF400, 0x2EB0C000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    // -------------------------------------------------------------------------
    // Advanced SIMD Two-register Miscellaneous (FP)
    // -------------------------------------------------------------------------
    insn VEC_FABS {
        mnemonic = "fabs";
        opcode_id = "VEC_FABS";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E20F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FNEG {
        mnemonic = "fneg";
        opcode_id = "VEC_FNEG";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E20F000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FRECPE {
        mnemonic = "frecpe";
        opcode_id = "VEC_FRECPE";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E21D000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FCMEQ_ZERO {
        mnemonic = "fcmeq";
        opcode_id = "VEC_FCMEQ_ZERO";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E20D000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FCMGT_ZERO {
        mnemonic = "fcmgt";
        opcode_id = "VEC_FCMGT_ZERO";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E20C000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FCMGE_ZERO {
        mnemonic = "fcmge";
        opcode_id = "VEC_FCMGE_ZERO";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x2E20C000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FCMLT_ZERO {
        mnemonic = "fcmlt";
        opcode_id = "VEC_FCMLT_ZERO";
        pattern = robustone_isa::mask_value!(0xBF3FF400, 0x0E20E000);
        format = &VEC_TWO_REG;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Cryptographic AES — two-register, always .16b
    // mask = 0xFFFF_F400: fixed byte3=0x4E, byte2=0x28, bits15:12+bit10 fixed
    // -------------------------------------------------------------------------
    insn VEC_AESE {
        mnemonic = "aese";
        opcode_id = "VEC_AESE";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x4E284000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_AESD {
        mnemonic = "aesd";
        opcode_id = "VEC_AESD";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x4E285000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_AESMC {
        mnemonic = "aesmc";
        opcode_id = "VEC_AESMC";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x4E286000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_AESIMC {
        mnemonic = "aesimc";
        opcode_id = "VEC_AESIMC";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x4E287000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Cryptographic SHA — three-register
    // mask = 0xFFE0_FC00: fixed byte3=0x5E, byte2 bits23:21=000, bits15:10 fixed
    // -------------------------------------------------------------------------
    insn VEC_SHA1C {
        mnemonic = "sha1c";
        opcode_id = "VEC_SHA1C";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E000000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA1P {
        mnemonic = "sha1p";
        opcode_id = "VEC_SHA1P";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E001000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA1M {
        mnemonic = "sha1m";
        opcode_id = "VEC_SHA1M";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E002000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA1SU0 {
        mnemonic = "sha1su0";
        opcode_id = "VEC_SHA1SU0";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E003000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA256H {
        mnemonic = "sha256h";
        opcode_id = "VEC_SHA256H";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E004000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA256H2 {
        mnemonic = "sha256h2";
        opcode_id = "VEC_SHA256H2";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E005000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA256SU1 {
        mnemonic = "sha256su1";
        opcode_id = "VEC_SHA256SU1";
        pattern = robustone_isa::mask_value!(0xFFE0FC00, 0x5E006000);
        format = &VEC_CRYPTO3;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Cryptographic SHA — two-register (vector)
    // mask = 0xFFFF_F400: fixed byte3=0x5E, byte2=0x28, bits15:12+bit10 fixed
    // -------------------------------------------------------------------------
    insn VEC_SHA1SU1 {
        mnemonic = "sha1su1";
        opcode_id = "VEC_SHA1SU1";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x5E281000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHA256SU0 {
        mnemonic = "sha256su0";
        opcode_id = "VEC_SHA256SU0";
        pattern = robustone_isa::mask_value!(0xFFFFF400, 0x5E282000);
        format = &VEC_CRYPTO2;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::CRYPTO;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Table Lookup
    // mask = 0xBFE0_FC00: fixed byte3=0x0E/0x4E, byte2 bits23:21=000, byte1 bits15:10 fixed
    // -------------------------------------------------------------------------
    insn VEC_TBL1 {
        mnemonic = "tbl";
        opcode_id = "VEC_TBL1";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E000000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBX1 {
        mnemonic = "tbx";
        opcode_id = "VEC_TBX1";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E001000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBL2 {
        mnemonic = "tbl";
        opcode_id = "VEC_TBL2";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E002000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBX2 {
        mnemonic = "tbx";
        opcode_id = "VEC_TBX2";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E003000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBL3 {
        mnemonic = "tbl";
        opcode_id = "VEC_TBL3";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E004000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBX3 {
        mnemonic = "tbx";
        opcode_id = "VEC_TBX3";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E005000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBL4 {
        mnemonic = "tbl";
        opcode_id = "VEC_TBL4";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E006000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_TBX4 {
        mnemonic = "tbx";
        opcode_id = "VEC_TBX4";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E007000);
        format = &VEC_TABLE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Copy/Extract — DUP, INS, EXT, SMOV, UMOV
    // -------------------------------------------------------------------------
    insn VEC_DUP_ELEMENT {
        mnemonic = "dup";
        opcode_id = "VEC_DUP_ELEMENT";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E000400);
        format = &VEC_COPY;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_DUP_GENERAL {
        mnemonic = "dup";
        opcode_id = "VEC_DUP_GENERAL";
        pattern = robustone_isa::mask_value!(0xBFF0FC00, 0x0E000C00);
        format = &VEC_COPY;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_INS_GENERAL {
        mnemonic = "mov";
        opcode_id = "VEC_INS_GENERAL";
        pattern = robustone_isa::mask_value!(0xBFE0FC00, 0x0E001C00);
        format = &VEC_COPY;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMOV {
        mnemonic = "smov";
        opcode_id = "VEC_SMOV";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E002C00);
        format = &VEC_COPY;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMOV {
        mnemonic = "umov";
        opcode_id = "VEC_UMOV";
        pattern = robustone_isa::mask_value!(0xBF20FC00, 0x0E003C00);
        format = &VEC_COPY;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_INS_ELEMENT {
        mnemonic = "mov";
        opcode_id = "VEC_INS_ELEMENT";
        pattern = robustone_isa::mask_value!(0xFF208400, 0x6E000400);
        format = &VEC_COPY_INS;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_EXT {
        mnemonic = "ext";
        opcode_id = "VEC_EXT";
        pattern = robustone_isa::mask_value!(0xBF208400, 0x2E000000);
        format = &VEC_EXT_FMT;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Indexed Element
    // mask = 0xBF00_F400: bit31=0, bits28:24=01111, bit10=0
    // U=1: mla, mls, umlal, umlsl, fmulx, umull
    // U=0: fmla, fmls, smlal, sqdmlal, smlsl, sqdmlsl, mul, fmul, smull,
    //       sqdmull, sqdmulh, sqrdmulh
    // -------------------------------------------------------------------------
    insn VEC_MLA_INDEXED {
        mnemonic = "mla";
        opcode_id = "VEC_MLA_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F000000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MLS_INDEXED {
        mnemonic = "mls";
        opcode_id = "VEC_MLS_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F004000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_FMLA_INDEXED {
        mnemonic = "fmla";
        opcode_id = "VEC_FMLA_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F001000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMLS_INDEXED {
        mnemonic = "fmls";
        opcode_id = "VEC_FMLS_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F005000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_SMLAL_INDEXED {
        mnemonic = "smlal";
        opcode_id = "VEC_SMLAL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F002000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMLAL_INDEXED {
        mnemonic = "umlal";
        opcode_id = "VEC_UMLAL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F002000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SQDMLAL_INDEXED {
        mnemonic = "sqdmlal";
        opcode_id = "VEC_SQDMLAL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F003000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SMLSL_INDEXED {
        mnemonic = "smlsl";
        opcode_id = "VEC_SMLSL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F006000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMLSL_INDEXED {
        mnemonic = "umlsl";
        opcode_id = "VEC_UMLSL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F006000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SQDMLSL_INDEXED {
        mnemonic = "sqdmlsl";
        opcode_id = "VEC_SQDMLSL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F007000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_MUL_INDEXED {
        mnemonic = "mul";
        opcode_id = "VEC_MUL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F008000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_FMUL_INDEXED {
        mnemonic = "fmul";
        opcode_id = "VEC_FMUL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F009000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_FMULX_INDEXED {
        mnemonic = "fmulx";
        opcode_id = "VEC_FMULX_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F009000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn VEC_SMULL_INDEXED {
        mnemonic = "smull";
        opcode_id = "VEC_SMULL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F00A000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_UMULL_INDEXED {
        mnemonic = "umull";
        opcode_id = "VEC_UMULL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x2F00A000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SQDMULL_INDEXED {
        mnemonic = "sqdmull";
        opcode_id = "VEC_SQDMULL_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F00B000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SQDMULH_INDEXED {
        mnemonic = "sqdmulh";
        opcode_id = "VEC_SQDMULH_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F00C000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SQRDMULH_INDEXED {
        mnemonic = "sqrdmulh";
        opcode_id = "VEC_SQRDMULH_INDEXED";
        pattern = robustone_isa::mask_value!(0xBF00F400, 0x0F00D000);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Modified Immediate
    // mask = 0x9F98_0400: bit31=0, bits28:24=01111, bit23=0, bit20=0, bit19=0, bit10=1
    // bit20=0 and bit19=0 distinguish modified immediate from shift immediate.
    // -------------------------------------------------------------------------
    insn VEC_MODIFIED_IMM {
        mnemonic = "movi";
        opcode_id = "VEC_MODIFIED_IMM";
        pattern = robustone_isa::mask_value!(0x9F98_0400, 0x0F00_0400);
        format = &FMT_VEC_MODIFIED_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Vector Shift Immediate — Group A (bit19=1)
    // mask = 0xBF88_0400: bit31=0, bits28:24=01111, bit23=0, bit19=1, bit10=1
    // Covers: sshr, ushr, ssra, usra, srshr, urshr, srsra, ursra, sri, usri,
    //         shl, sqshl, sshll, ushll, shrn, rshrn, sqshrn, uqshrn, sqrshrn,
    //         uqrshrn, sqshrun, sqrshrun, and their "2" variants.
    // -------------------------------------------------------------------------
    insn VEC_SHIFT_IMM_A {
        mnemonic = "sshr";
        opcode_id = "VEC_SHIFT_IMM_A";
        pattern = robustone_isa::mask_value!(0xBF88_0400, 0x0F08_0400);
        format = &VEC_SHIFT_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHIFT_IMM_A_U1 {
        mnemonic = "ushr";
        opcode_id = "VEC_SHIFT_IMM_A_U1";
        pattern = robustone_isa::mask_value!(0xBF88_0400, 0x2F08_0400);
        format = &VEC_SHIFT_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Vector Shift Immediate — Group B (bit20=1, bit19=0)
    // mask = 0xBF98_0400: bit31=0, bits28:24=01111, bit23=0, bit20=1, bit19=0, bit10=1
    // Covers: sqshlu and other shifts where immh has bit20=1.
    // -------------------------------------------------------------------------
    insn VEC_SHIFT_IMM_B {
        mnemonic = "sqshlu";
        opcode_id = "VEC_SHIFT_IMM_B";
        pattern = robustone_isa::mask_value!(0xBF98_0400, 0x0F10_0400);
        format = &VEC_SHIFT_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn VEC_SHIFT_IMM_B_U1 {
        mnemonic = "sqshlu";
        opcode_id = "VEC_SHIFT_IMM_B_U1";
        pattern = robustone_isa::mask_value!(0xBF98_0400, 0x2F10_0400);
        format = &VEC_SHIFT_IMM;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Advanced SIMD Load/Store
    // bits 30:24 = 0001100 (AdvSIMD load/store category)
    // -------------------------------------------------------------------------
    insn SIMD_LS_LOAD {
        mnemonic = "ld1";
        opcode_id = "SIMD_LS_LOAD";
        pattern = robustone_isa::mask_value!(0xBFE0_0000, 0x0C40_0000);
        format = &SIMD_LS;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rt, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
    insn SIMD_LS_STORE {
        mnemonic = "st1";
        opcode_id = "SIMD_LS_STORE";
        pattern = robustone_isa::mask_value!(0xBFE0_0000, 0x0C00_0000);
        format = &SIMD_LS;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FMLAL2 (vector, indexed element) — FP16 widening multiply-add long
    // bits 31:24 = 0 Q 1 0 1 1 1 1, bits 23:21 = 1 0 0, bits 15:12 = 1 0 0 0
    // -------------------------------------------------------------------------
    insn VEC_FMLAL2_INDEXED {
        mnemonic = "fmlal2";
        opcode_id = "VEC_FMLAL2_INDEXED";
        pattern = robustone_isa::mask_value!(0xBFE0_FC00, 0x2F80_8800);
        format = &VEC_THREE_SAME;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::SIMD;
        groups = &[robustone_isa::InstructionGroup::Vector];
        manual = "ARM ARM";
    }
}
