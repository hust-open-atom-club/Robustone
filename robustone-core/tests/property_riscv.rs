use proptest::prelude::*;
use robustone_core::common::ArchitectureProfile;
use robustone_core::ir::{
    ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints,
    TextRenderProfile,
};
use robustone_core::{
    ArchitectureDispatcher, DisasmError, Instruction, RenderOptions, render_instruction_text,
};
use std::panic::AssertUnwindSafe;

fn sign_extend(value: u32, bits: u8) -> i64 {
    let sign_bit = 1u32 << (bits - 1);
    if (value & sign_bit) != 0 {
        (value as i64) - (1i64 << bits)
    } else {
        value as i64
    }
}

fn encode_addi(rd: u8, rs1: u8, imm12: u16) -> [u8; 4] {
    let instruction =
        ((imm12 as u32) << 20) | ((rs1 as u32) << 15) | ((rd as u32) << 7) | 0b0010011;
    instruction.to_le_bytes()
}

fn encode_c_addi(rd: u8, imm6: u8) -> [u8; 2] {
    let instruction = ((u16::from((imm6 >> 5) & 0x1)) << 12)
        | ((rd as u16) << 7)
        | ((u16::from(imm6 & 0x1f)) << 2)
        | 0b01;
    instruction.to_le_bytes()
}

fn collect_register_ids(decoded: &DecodedInstruction) -> Vec<u32> {
    let mut ids = Vec::new();

    for operand in &decoded.operands {
        match operand {
            Operand::Register { register } => ids.push(register.id),
            Operand::Memory { base, .. } => {
                if let Some(register) = base {
                    ids.push(register.id);
                }
            }
            Operand::Immediate { .. } | Operand::Text { .. } => {}
        }
    }

    ids.extend(decoded.registers_read.iter().map(|register| register.id));
    ids.extend(decoded.registers_written.iter().map(|register| register.id));
    ids.extend(
        decoded
            .implicit_registers_read
            .iter()
            .map(|register| register.id),
    );
    ids.extend(
        decoded
            .implicit_registers_written
            .iter()
            .map(|register| register.id),
    );

    ids
}

fn render_options(profile: TextRenderProfile) -> RenderOptions {
    RenderOptions {
        text_profile: profile,
        alias_regs: false,
        unsigned_immediate: false,
    }
}

prop_compose! {
    fn decoded_instruction_strategy()(
        mnemonic in prop::sample::select(vec![
            "addi".to_string(),
            "beq".to_string(),
            "c.addi".to_string(),
            "lw".to_string(),
            "sw".to_string(),
            "fadd.s".to_string(),
            "csrrw".to_string(),
        ]),
        mode in prop::sample::select(vec!["riscv32".to_string(), "riscv64".to_string()]),
        raw_bytes in prop::collection::vec(any::<u8>(), 0..8),
        operands in prop::collection::vec(operand_strategy(), 0..5),
        registers in (
            prop::collection::vec((0u32..64).prop_map(RegisterId::riscv), 0..4),
            prop::collection::vec((0u32..64).prop_map(RegisterId::riscv), 0..4),
            prop::collection::vec((0u32..64).prop_map(RegisterId::riscv), 0..2),
            prop::collection::vec((0u32..64).prop_map(RegisterId::riscv), 0..2),
        ),
        groups in prop::collection::vec(
            prop::sample::select(vec![
                "arithmetic".to_string(),
                "branch".to_string(),
                "load".to_string(),
                "floating_point".to_string(),
            ]),
            0..3,
        ),
        capstone_mnemonic in prop::option::of(prop::sample::select(vec![
            "li".to_string(),
            "beqz".to_string(),
            "j".to_string(),
        ])),
        capstone_hidden_operands in prop::collection::vec(0usize..5, 0..2),
        status in prop::sample::select(vec![
            DecodeStatus::Success,
            DecodeStatus::InvalidEncoding,
            DecodeStatus::UnsupportedExtension,
            DecodeStatus::Unimplemented,
        ]),
        size in prop_oneof![Just(2usize), Just(4usize)],
    ) -> DecodedInstruction {
        let (
            registers_read,
            registers_written,
            implicit_registers_read,
            implicit_registers_written,
        ) = registers;

        DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode,
            mnemonic: mnemonic.clone(),
            opcode_id: Some(mnemonic),
            size,
            raw_bytes,
            operands,
            registers_read,
            registers_written,
            implicit_registers_read,
            implicit_registers_written,
            groups,
            status,
            render_hints: RenderHints {
                capstone_mnemonic,
                capstone_hidden_operands,
            },
        }
    }
}

fn operand_strategy() -> impl Strategy<Value = Operand> {
    let register_strategy = (0u32..64).prop_map(RegisterId::riscv);
    prop_oneof![
        register_strategy
            .clone()
            .prop_map(|register| Operand::Register { register }),
        any::<i64>().prop_map(|value| Operand::Immediate { value }),
        (prop::option::of(register_strategy.clone()), any::<i64>())
            .prop_map(|(base, displacement)| Operand::Memory { base, displacement }),
        prop::sample::select(vec![
            "rne".to_string(),
            "rtz".to_string(),
            "csr".to_string(),
        ])
        .prop_map(|value| Operand::Text { value }),
    ]
}

proptest! {
    #[test]
    fn test_decode_instruction_never_panics_on_even_length_inputs(
        bytes in prop::collection::vec(any::<u8>(), 2..8)
    ) {
        prop_assume!(bytes.len() % 2 == 0);

        let dispatcher = ArchitectureDispatcher::new();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            dispatcher.decode_instruction(&bytes, "riscv32", 0)
        }));

        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_short_inputs_report_structured_failures(
        bytes in prop::collection::vec(any::<u8>(), 1..4)
    ) {
        let dispatcher = ArchitectureDispatcher::new();
        let result = dispatcher.decode_instruction(&bytes, "riscv32", 0);

        if bytes.len() < 2 {
            prop_assert!(result.is_err());
            if let Err(error) = result {
                let is_structured = matches!(error, DisasmError::DecodeFailure { .. });
                prop_assert!(is_structured);
            }
        }
    }

    #[test]
    fn test_successful_compressed_decodes_have_size_two(bytes in any::<[u8; 2]>()) {
        prop_assume!((bytes[0] & 0x3) != 0x3);

        let dispatcher = ArchitectureDispatcher::new();
        if let Ok((decoded, size)) = dispatcher.decode_instruction(&bytes, "riscv32", 0) {
            prop_assert_eq!(size, 2);
            prop_assert_eq!(decoded.size, 2);
        }
    }

    #[test]
    fn test_successful_standard_decodes_have_size_four(bytes in any::<[u8; 4]>()) {
        prop_assume!((bytes[0] & 0x3) == 0x3);

        let dispatcher = ArchitectureDispatcher::new();
        if let Ok((decoded, size)) = dispatcher.decode_instruction(&bytes, "riscv32", 0) {
            prop_assert_eq!(size, 4);
            prop_assert_eq!(decoded.size, 4);
        }
    }

    #[test]
    fn test_addi_sign_extension_matches_imm12(
        rd in 0u8..32,
        rs1 in 0u8..32,
        imm12 in 0u16..4096,
    ) {
        let dispatcher = ArchitectureDispatcher::new();
        let bytes = encode_addi(rd, rs1, imm12);
        let (decoded, size) = dispatcher
            .decode_instruction(&bytes, "riscv32", 0)
            .expect("encoded addi should decode");

        let immediate = decoded
            .operands
            .iter()
            .find_map(|operand| match operand {
                Operand::Immediate { value } => Some(*value),
                _ => None,
            })
            .expect("addi should contain an immediate operand");

        prop_assert_eq!(size, 4);
        prop_assert_eq!(immediate, sign_extend(imm12 as u32, 12));
    }

    #[test]
    fn test_c_addi_sign_extension_matches_imm6(
        rd in 1u8..32,
        imm6 in 0u8..64,
    ) {
        prop_assume!(imm6 != 0);

        let dispatcher = ArchitectureDispatcher::new();
        let bytes = encode_c_addi(rd, imm6);
        let (decoded, size) = dispatcher
            .decode_instruction(&bytes, "riscv32", 0)
            .expect("encoded c.addi should decode");

        let immediate = decoded
            .operands
            .iter()
            .find_map(|operand| match operand {
                Operand::Immediate { value } => Some(*value),
                _ => None,
            })
            .expect("c.addi should contain an immediate operand");

        prop_assert_eq!(size, 2);
        prop_assert_eq!(decoded.size, 2);
        prop_assert_eq!(immediate, sign_extend(u32::from(imm6), 6));
    }

    #[test]
    fn test_decoded_register_ids_stay_within_riscv_range(
        bytes in prop::collection::vec(any::<u8>(), 2..8),
        arch in prop::sample::select(vec!["riscv32", "riscv64"]),
    ) {
        prop_assume!(bytes.len() % 2 == 0);

        let dispatcher = ArchitectureDispatcher::new();
        if let Ok((decoded, _)) = dispatcher.decode_instruction(&bytes, arch, 0) {
            for register_id in collect_register_ids(&decoded) {
                prop_assert!(register_id < 64);
            }
        }
    }

    #[test]
    fn test_formatter_never_panics_on_arbitrary_decoded_instructions(
        decoded in decoded_instruction_strategy()
    ) {
        let instruction = Instruction::from_decoded(decoded, "legacy".to_string(), "legacy".to_string(), None);
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _ = render_instruction_text(&instruction, render_options(TextRenderProfile::Capstone));
            let _ = render_instruction_text(&instruction, render_options(TextRenderProfile::Canonical));
            let _ = render_instruction_text(&instruction, render_options(TextRenderProfile::VerboseDebug));
        }));

        prop_assert!(result.is_ok());
    }
}

#[test]
fn test_profile_without_c_extension_rejects_compressed_decode() {
    let dispatcher = ArchitectureDispatcher::new();
    let profile = ArchitectureProfile::riscv(
        robustone_core::architecture::Architecture::RiscV32,
        "riscv32",
        32,
        vec!["I"],
    );

    let error = dispatcher
        .decode_with_profile(&[0x05, 0x68], &profile, 0)
        .expect_err("compressed instruction should require the C extension");

    assert!(matches!(
        error,
        DisasmError::DecodeFailure {
            kind: robustone_core::types::error::DecodeErrorKind::UnsupportedExtension,
            ..
        }
    ));
}
