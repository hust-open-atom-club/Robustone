robustone_isa_macros::define_instructions! {
    arch = RiscV; module = i;
    insn ADD {
        mnemonic = "add";
        opcode_id = "ADD";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_0033);
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
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read),
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
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Write),
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
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            ),
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
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            ),
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
            robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 20 },
                    robustone_isa::ImmComposePart { src_start: 21, src_length: 10, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 20, src_length: 1, dst_start: 11 },
                    robustone_isa::ImmComposePart { src_start: 12, src_length: 8, dst_start: 12 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 21 },
                kind = robustone_isa::ImmediateKind::PcRelative
            ),
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
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtend { bits: 20 }, robustone_isa::ImmediateKind::Absolute),
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
            robustone_isa::imm!(RiscVField::Imm20U, robustone_isa::ImmediateTransform::SignExtend { bits: 20 }, robustone_isa::ImmediateKind::PcRelative),
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
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn SUB {
        mnemonic = "sub";
        opcode_id = "SUB";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_0033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLL {
        mnemonic = "sll";
        opcode_id = "SLL";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_1033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLT {
        mnemonic = "slt";
        opcode_id = "SLT";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_2033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLTU {
        mnemonic = "sltu";
        opcode_id = "SLTU";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_3033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn XOR {
        mnemonic = "xor";
        opcode_id = "XOR";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_4033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRL {
        mnemonic = "srl";
        opcode_id = "SRL";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_5033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRA {
        mnemonic = "sra";
        opcode_id = "SRA";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_5033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn OR {
        mnemonic = "or";
        opcode_id = "OR";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_6033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn AND {
        mnemonic = "and";
        opcode_id = "AND";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_7033);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLTI {
        mnemonic = "slti";
        opcode_id = "SLTI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLTIU {
        mnemonic = "sltiu";
        opcode_id = "SLTIU";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_3013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn XORI {
        mnemonic = "xori";
        opcode_id = "XORI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_4013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn ANDI {
        mnemonic = "andi";
        opcode_id = "ANDI";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_7013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Logical];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLLI {
        mnemonic = "slli";
        opcode_id = "SLLI";
        pattern = robustone_isa::mask_value!(0xFC00_707F, 0x0000_1013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRLI {
        mnemonic = "srli";
        opcode_id = "SRLI";
        pattern = robustone_isa::mask_value!(0xFC00_707F, 0x0000_5013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRAI {
        mnemonic = "srai";
        opcode_id = "SRAI";
        pattern = robustone_isa::mask_value!(0xFC00_707F, 0x4000_5013);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn LB {
        mnemonic = "lb";
        opcode_id = "LB";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0003);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn LH {
        mnemonic = "lh";
        opcode_id = "LH";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_1003);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn LBU {
        mnemonic = "lbu";
        opcode_id = "LBU";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_4003);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn LHU {
        mnemonic = "lhu";
        opcode_id = "LHU";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_5003);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn SB {
        mnemonic = "sb";
        opcode_id = "SB";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_0023);
        format = &S_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Write)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn SH {
        mnemonic = "sh";
        opcode_id = "SH";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_1023);
        format = &S_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Write)];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn BLT {
        mnemonic = "blt";
        opcode_id = "BLT";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_4063);
        format = &B_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            )];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn BGE {
        mnemonic = "bge";
        opcode_id = "BGE";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_5063);
        format = &B_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            )];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn BLTU {
        mnemonic = "bltu";
        opcode_id = "BLTU";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_6063);
        format = &B_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            )];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn BGEU {
        mnemonic = "bgeu";
        opcode_id = "BGEU";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_7063);
        format = &B_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read), robustone_isa::imm_compose!(
                parts = [
                    robustone_isa::ImmComposePart { src_start: 31, src_length: 1, dst_start: 12 },
                    robustone_isa::ImmComposePart { src_start: 25, src_length: 6, dst_start: 5 },
                    robustone_isa::ImmComposePart { src_start: 8, src_length: 4, dst_start: 1 },
                    robustone_isa::ImmComposePart { src_start: 7, src_length: 1, dst_start: 11 },
                ],
                transform = robustone_isa::ImmediateTransform::SignExtend { bits: 13 },
                kind = robustone_isa::ImmediateKind::PcRelative
            )];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn ECALL {
        mnemonic = "ecall";
        opcode_id = "ECALL";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0000_0073);
        format = &I_TYPE;
        operands = &[];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn EBREAK {
        mnemonic = "ebreak";
        opcode_id = "EBREAK";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0010_0073);
        format = &I_TYPE;
        operands = &[];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FENCE {
        mnemonic = "fence";
        opcode_id = "FENCE";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_000F);
        format = &I_TYPE;
        operands = &[];
        features = RiscVFeature::I;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::System, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn ADDIW {
        mnemonic = "addiw";
        opcode_id = "ADDIW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_001B);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLLIW {
        mnemonic = "slliw";
        opcode_id = "SLLIW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_101B);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRLIW {
        mnemonic = "srliw";
        opcode_id = "SRLIW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_501B);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRAIW {
        mnemonic = "sraiw";
        opcode_id = "SRAIW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_501B);
        format = &I_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::imm!(RiscVField::Shamtw, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn ADDW {
        mnemonic = "addw";
        opcode_id = "ADDW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_003B);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SUBW {
        mnemonic = "subw";
        opcode_id = "SUBW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_003B);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SLLW {
        mnemonic = "sllw";
        opcode_id = "SLLW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_103B);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRLW {
        mnemonic = "srlw";
        opcode_id = "SRLW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x0000_503B);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn SRAW {
        mnemonic = "sraw";
        opcode_id = "SRAW";
        pattern = robustone_isa::mask_value!(0xFE00_707F, 0x4000_503B);
        format = &R_TYPE;
        operands = &[robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read), robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2, robustone_isa::Access::Read)];
        features = RiscVFeature::I;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Integer, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}
