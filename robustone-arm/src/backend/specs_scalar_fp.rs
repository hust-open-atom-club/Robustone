robustone_isa_macros::define_instructions! {
    arch = Arm; module = scalar_fp;

    // -------------------------------------------------------------------------
    // FP 2-source data-processing
    // mask = 0xFF20FC00: bits31:24=0x1E, bit21=1, bits15:10=opcode6
    // -------------------------------------------------------------------------
    insn FADD {
        mnemonic = "fadd";
        opcode_id = "FADD";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_2800);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FSUB {
        mnemonic = "fsub";
        opcode_id = "FSUB";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_3800);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMUL {
        mnemonic = "fmul";
        opcode_id = "FMUL";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_0800);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FDIV {
        mnemonic = "fdiv";
        opcode_id = "FDIV";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_1800);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMAX {
        mnemonic = "fmax";
        opcode_id = "FMAX";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_2000);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMIN {
        mnemonic = "fmin";
        opcode_id = "FMIN";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_2400);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMAXNM {
        mnemonic = "fmaxnm";
        opcode_id = "FMAXNM";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_4000);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMINNM {
        mnemonic = "fminnm";
        opcode_id = "FMINNM";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_4400);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FNMUL {
        mnemonic = "fnmul";
        opcode_id = "FNMUL";
        pattern = robustone_isa::mask_value!(0xFF20_FC00, 0x1E20_4800);
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FP Compare (must appear before 1-source since they share opcode6 space)
    // mask = 0xFF3FFC1F: bits31:24=0x1E, bit21=0, bits18:16=000, bits15:10=opcode6, rd=0
    // -------------------------------------------------------------------------
    insn FCMP {
        mnemonic = "fcmp";
        opcode_id = "FCMP";
        pattern = robustone_isa::mask_value!(0xFF20_FC1F, 0x1E00_2000);
        priority = 3;
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FCMPE {
        mnemonic = "fcmpe";
        opcode_id = "FCMPE";
        pattern = robustone_isa::mask_value!(0xFF20_FC1F, 0x1E00_2400);
        priority = 3;
        format = &FP_2SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FP 1-source data-processing
    // mask = 0xFF27FC00: bits31:24=0x1E, bit21=0, bits18:16=000, bits15:10=opcode6
    // -------------------------------------------------------------------------
    insn FMOV {
        mnemonic = "fmov";
        opcode_id = "FMOV";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_0000);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FABS {
        mnemonic = "fabs";
        opcode_id = "FABS";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_0400);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FNEG {
        mnemonic = "fneg";
        opcode_id = "FNEG";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_0800);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FSQRT {
        mnemonic = "fsqrt";
        opcode_id = "FSQRT";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_0C00);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTN {
        mnemonic = "frintn";
        opcode_id = "FRINTN";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_2000);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTP {
        mnemonic = "frintp";
        opcode_id = "FRINTP";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_2400);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTM {
        mnemonic = "frintm";
        opcode_id = "FRINTM";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_2800);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTZ {
        mnemonic = "frintz";
        opcode_id = "FRINTZ";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_2C00);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTA {
        mnemonic = "frinta";
        opcode_id = "FRINTA";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_3000);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTX {
        mnemonic = "frintx";
        opcode_id = "FRINTX";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_3400);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FRINTI {
        mnemonic = "frinti";
        opcode_id = "FRINTI";
        pattern = robustone_isa::mask_value!(0xFF27_FC00, 0x1E00_3800);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FP Conditional Select (after FSQRT since they share opcode6=0x03)
    // -------------------------------------------------------------------------
    insn FCSEL {
        mnemonic = "fcsel";
        opcode_id = "FCSEL";
        pattern = robustone_isa::mask_value!(0xFF20_0C00, 0x1E00_0C00);
        priority = 0;
        format = &FP_CONDSEL;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FP 3-source (FMADD, FMSUB)
    // -------------------------------------------------------------------------
    insn FMADD {
        mnemonic = "fmadd";
        opcode_id = "FMADD";
        pattern = robustone_isa::mask_value!(0xFF00_8000, 0x1F00_0000);
        format = &FP_3SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Ra, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
    insn FMSUB {
        mnemonic = "fmsub";
        opcode_id = "FMSUB";
        pattern = robustone_isa::mask_value!(0xFF00_8000, 0x1F00_8000);
        format = &FP_3SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rm, robustone_isa::Access::Read),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Ra, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // FP Conversion (FCVT between FP sizes)
    // -------------------------------------------------------------------------
    insn FCVT_FP {
        mnemonic = "fcvt";
        opcode_id = "FCVT_FP";
        pattern = robustone_isa::mask_value!(0xFF27_F000, 0x1E01_0000);
        priority = 1;
        format = &FP_1SOURCE;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(ArmRegisterClass::Vec, ArmField::Rn, robustone_isa::Access::Read),
        ];
        modes = ModeSet::All;
        features = ArmFeature::FP;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "ARM ARM";
    }
}
