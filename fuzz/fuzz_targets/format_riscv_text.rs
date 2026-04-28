#![no_main]

use libfuzzer_sys::fuzz_target;
use robustone_core::ir::{
    ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints,
    TextRenderProfile,
};
use robustone_core::{
    Instruction, RenderOptions, render_disassembly, render_instruction_text,
};

fn next_byte(data: &[u8], cursor: &mut usize) -> u8 {
    if data.is_empty() {
        return 0;
    }
    let value = data[*cursor % data.len()];
    *cursor += 1;
    value
}

fn next_register(data: &[u8], cursor: &mut usize) -> RegisterId {
    RegisterId::riscv(u32::from(next_byte(data, cursor) % 64))
}

fn next_i64(data: &[u8], cursor: &mut usize) -> i64 {
    let mut bytes = [0u8; 8];
    for byte in &mut bytes {
        *byte = next_byte(data, cursor);
    }
    i64::from_le_bytes(bytes)
}

fn next_operand(data: &[u8], cursor: &mut usize) -> Operand {
    match next_byte(data, cursor) % 4 {
        0 => Operand::Register {
            register: next_register(data, cursor),
        },
        1 => Operand::Immediate {
            value: next_i64(data, cursor),
        },
        2 => Operand::Memory {
            base: Some(next_register(data, cursor)),
            displacement: next_i64(data, cursor),
        },
        _ => Operand::Text {
            value: match next_byte(data, cursor) % 3 {
                0 => "rne".to_string(),
                1 => "csr".to_string(),
                _ => "custom".to_string(),
            },
        },
    }
}

fn build_instruction(data: &[u8]) -> Instruction {
    let mut cursor = 0usize;
    let mnemonic = match next_byte(data, &mut cursor) % 7 {
        0 => "addi",
        1 => "beq",
        2 => "c.addi",
        3 => "lw",
        4 => "sw",
        5 => "fadd.s",
        _ => "csrrw",
    }
    .to_string();
    let operand_count = usize::from(next_byte(data, &mut cursor) % 5);
    let size = if next_byte(data, &mut cursor) & 1 == 0 { 2 } else { 4 };
    let mut operands = Vec::new();
    for _ in 0..operand_count {
        operands.push(next_operand(data, &mut cursor));
    }

    let decoded = DecodedInstruction {
        architecture: ArchitectureId::Riscv,
        address: 0,
        mode: if next_byte(data, &mut cursor) & 1 == 0 {
            "riscv32".to_string()
        } else {
            "riscv64".to_string()
        },
        mnemonic: mnemonic.clone(),
        opcode_id: Some(mnemonic.clone()),
        size,
        raw_bytes: data.iter().copied().take(size).collect(),
        operands,
        registers_read: vec![next_register(data, &mut cursor)],
        registers_written: vec![next_register(data, &mut cursor)],
        implicit_registers_read: Vec::new(),
        implicit_registers_written: Vec::new(),
        groups: vec!["arithmetic".to_string()],
        status: match next_byte(data, &mut cursor) % 4 {
            0 => DecodeStatus::Success,
            1 => DecodeStatus::InvalidEncoding,
            2 => DecodeStatus::UnsupportedExtension,
            _ => DecodeStatus::Unimplemented,
        },
        render_hints: RenderHints {
            capstone_mnemonic: if next_byte(data, &mut cursor) & 1 == 0 {
                Some("li".to_string())
            } else {
                None
            },
            capstone_hidden_operands: vec![usize::from(next_byte(data, &mut cursor) % 4)],
        },
    };

    Instruction::from_decoded(decoded, "legacy".to_string(), "legacy".to_string(), None)
}

fn options(profile: TextRenderProfile) -> RenderOptions {
    RenderOptions {
        text_profile: profile,
        alias_regs: false,
        unsigned_immediate: false,
    }
}

fuzz_target!(|data: &[u8]| {
    let instruction = build_instruction(data);

    let _ = render_instruction_text(&instruction, options(TextRenderProfile::Capstone));
    let _ = render_instruction_text(&instruction, options(TextRenderProfile::Canonical));
    let _ = render_instruction_text(&instruction, options(TextRenderProfile::VerboseDebug));
    let _ = render_disassembly(
        "riscv32".to_string(),
        0,
        instruction.size,
        Vec::new(),
        &[instruction],
        options(TextRenderProfile::Capstone),
    );
});
