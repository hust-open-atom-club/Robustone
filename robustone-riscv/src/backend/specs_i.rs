robustone_isa_macros::define_instructions! {
    arch = RiscV; module = i;
    insn ADD {
        mnemonic = "add";
        opcode_id = "ADD";
        pattern = robustone_isa::mask_value!(0xFE00_007F, 0x0000_0033);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn ADDI {
        mnemonic = "addi";
        opcode_id = "ADDI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0013);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn LW {
        mnemonic = "lw";
        opcode_id = "LW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2003);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn SW {
        mnemonic = "sw";
        opcode_id = "SW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2023);
        format = &S_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn BEQ {
        mnemonic = "beq";
        opcode_id = "BEQ";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0063);
        format = &B_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12B, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 12, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn BNE {
        mnemonic = "bne";
        opcode_id = "BNE";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_1063);
        format = &B_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12B, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 12, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn JAL {
        mnemonic = "jal";
        opcode_id = "JAL";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_006F);
        format = &J_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20J, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 1 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Call;
    }
    insn JALR {
        mnemonic = "jalr";
        opcode_id = "JALR";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0067);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Return;
    }
    insn LUI {
        mnemonic = "lui";
        opcode_id = "LUI";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_0037);
        format = &U_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn AUIPC {
        mnemonic = "auipc";
        opcode_id = "AUIPC";
        pattern = robustone_isa::mask_value!(0x0000_007F, 0x0000_0017);
        format = &U_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtendThenShift { bits: 20, shift: 12 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn ORI {
        mnemonic = "ori";
        opcode_id = "ORI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_6013);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn LD {
        mnemonic = "ld";
        opcode_id = "LD";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_3003);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
}
