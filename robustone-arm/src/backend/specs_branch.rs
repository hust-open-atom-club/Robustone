robustone_isa_macros::define_instructions! {
    arch = Arm; module = branch;

    // -------------------------------------------------------------------------
    // Unconditional Branch (immediate)
    // -------------------------------------------------------------------------
    insn B {
        mnemonic = "b";
        opcode_id = "B";
        pattern = robustone_isa::mask_value!(0xFC00_0000, 0x1400_0000);
        format = &B_UNCOND;
        operands = &[
            robustone_isa::imm!(ArmField::Imm26, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
    insn BL {
        mnemonic = "bl";
        opcode_id = "BL";
        pattern = robustone_isa::mask_value!(0xFC00_0000, 0x9400_0000);
        format = &B_UNCOND;
        operands = &[
            robustone_isa::imm!(ArmField::Imm26, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Conditional Branch (immediate)
    // -------------------------------------------------------------------------
    insn B_COND {
        mnemonic = "b.cond";
        opcode_id = "B_COND";
        pattern = robustone_isa::mask_value!(0xFF00_0010, 0x5400_0000);
        format = &FMT_B_COND;
        operands = &[
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Compare and Branch (immediate)
    // -------------------------------------------------------------------------
    insn CBZ {
        mnemonic = "cbz";
        opcode_id = "CBZ";
        pattern = robustone_isa::mask_value!(0x7F00_0000, 0x3400_0000);
        format = &FMT_CBZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
    insn CBNZ {
        mnemonic = "cbnz";
        opcode_id = "CBNZ";
        pattern = robustone_isa::mask_value!(0x7F00_0000, 0x3500_0000);
        format = &FMT_CBZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }

    // -------------------------------------------------------------------------
    // Test and Branch (immediate)
    // -------------------------------------------------------------------------
    insn TBZ {
        mnemonic = "tbz";
        opcode_id = "TBZ";
        pattern = robustone_isa::mask_value!(0x7F00_0000, 0x3600_0000);
        format = &FMT_TBZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 24, src_length: 5, dst_start: 0 },
                    robustone_isa::ImmComposePart { src_start: 19, src_length: 1, dst_start: 5 },
                ],
                transform = robustone_isa::ImmediateTransform::None,
                kind = robustone_isa::ImmediateKind::Unsigned
            ),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
    insn TBNZ {
        mnemonic = "tbnz";
        opcode_id = "TBNZ";
        pattern = robustone_isa::mask_value!(0x7F00_0000, 0x3700_0000);
        format = &FMT_TBZ;
        operands = &[
            robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Read),
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 24, src_length: 5, dst_start: 0 },
                    robustone_isa::ImmComposePart { src_start: 19, src_length: 1, dst_start: 5 },
                ],
                transform = robustone_isa::ImmediateTransform::None,
                kind = robustone_isa::ImmediateKind::Unsigned
            ),
            robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative),
        ];
        modes = ModeSet::All;
        features = ArmFeature::BASE;
        groups = &[robustone_isa::InstructionGroup::Branch];
        manual = "ARM ARM";
    }
}
