use std::fs;
use std::path::PathBuf;

use robustone_core::ArchitectureDispatcher;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoldenCase {
    arch: String,
    hex: String,
    expected_capstone: ExpectedText,
    expected_ir: ExpectedIr,
}

#[derive(Debug, Deserialize)]
struct ExpectedText {
    mnemonic: String,
    operands: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedIr {
    mnemonic: String,
    render_hint_mnemonic: Option<String>,
    hidden_operands: Vec<usize>,
    groups: Vec<String>,
    operand_kinds: Vec<String>,
}

fn load_case(path: &str) -> GoldenCase {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("tests")
        .join("golden")
        .join("riscv")
        .join(path);
    let data = fs::read_to_string(fixture_path).expect("golden fixture should be readable");
    serde_json::from_str(&data).expect("golden fixture should parse")
}

fn assert_case(case: GoldenCase) {
    let dispatcher = ArchitectureDispatcher::new();

    let compatibility = dispatcher.disassemble(&case.hex, case.arch.clone());
    assert_eq!(compatibility.mnemonic, case.expected_capstone.mnemonic);
    assert_eq!(compatibility.operands, case.expected_capstone.operands);

    let bytes = hex::decode(&case.hex).expect("hex fixture should decode");
    let (decoded, size) = dispatcher
        .decode_instruction(&bytes, &case.arch, 0)
        .expect("low-level decode should succeed");

    assert_eq!(size, bytes.len());
    assert_eq!(decoded.mnemonic, case.expected_ir.mnemonic);
    assert_eq!(
        decoded.render_hints.capstone_mnemonic.as_deref(),
        case.expected_ir.render_hint_mnemonic.as_deref()
    );
    assert_eq!(
        decoded.render_hints.capstone_hidden_operands,
        case.expected_ir.hidden_operands
    );
    let (rendered_mnemonic, rendered_operands) = decoded.render_capstone_text_parts();
    assert_eq!(rendered_mnemonic, case.expected_capstone.mnemonic);
    assert_eq!(rendered_operands, case.expected_capstone.operands);
    assert_eq!(decoded.groups, case.expected_ir.groups);
    let operand_kinds = decoded
        .operands
        .iter()
        .map(|operand| match operand {
            robustone_core::ir::Operand::Register { .. } => "register",
            robustone_core::ir::Operand::Immediate { .. } => "immediate",
            robustone_core::ir::Operand::Text { .. } => "text",
            robustone_core::ir::Operand::Memory { .. } => "memory",
        })
        .collect::<Vec<_>>();
    assert_eq!(operand_kinds, case.expected_ir.operand_kinds);
}

#[test]
fn test_addi_li_golden_fixture() {
    assert_case(load_case("addi_li.json"));
}

#[test]
fn test_fadd_s_golden_fixture() {
    assert_case(load_case("fadd_s.json"));
}

#[test]
fn test_c_lui_golden_fixture() {
    assert_case(load_case("c_lui.json"));
}

#[test]
fn test_c_jr_golden_fixture() {
    assert_case(load_case("c_jr.json"));
}

#[test]
fn test_ir_rendering_covers_control_flow_and_atomic_variants() {
    let dispatcher = ArchitectureDispatcher::new();
    let cases = [
        (
            "jalr_hidden_rd",
            "riscv32",
            vec![0xe7, 0x00, 0x01, 0x00],
            ("jalr".to_string(), "0(sp)".to_string()),
            ("jalr".to_string(), "x1, 0(x2)".to_string()),
        ),
        (
            "lr_w",
            "riscv32",
            vec![0xaf, 0x20, 0x01, 0x10],
            ("lr.w".to_string(), "ra, (sp)".to_string()),
            ("lr.w".to_string(), "x1, (x2)".to_string()),
        ),
        (
            "sc_w",
            "riscv32",
            vec![0xaf, 0x20, 0x31, 0x18],
            ("sc.w".to_string(), "ra, gp, (sp)".to_string()),
            ("sc.w".to_string(), "x1, x3, (x2)".to_string()),
        ),
        (
            "amoadd_w",
            "riscv32",
            vec![0xaf, 0x20, 0x31, 0x00],
            ("amoadd.w".to_string(), "ra, gp, (sp)".to_string()),
            ("amoadd.w".to_string(), "x1, x3, (x2)".to_string()),
        ),
    ];

    for (_name, arch, bytes, expected_capstone, expected_canonical) in cases {
        let (decoded, _) = dispatcher
            .decode_instruction(&bytes, arch, 0)
            .expect("decode should succeed");
        assert_eq!(decoded.render_capstone_text_parts(), expected_capstone);
        assert_eq!(decoded.render_canonical_text_parts(), expected_canonical);

        let (instruction, _) = dispatcher
            .disassemble_bytes(&bytes, arch, 0)
            .expect("compatibility disassembly should succeed");
        assert_eq!(
            instruction.rendered_text_parts(robustone_core::ir::TextRenderProfile::Capstone),
            expected_capstone
        );
        assert_eq!(
            instruction.rendered_text_parts(robustone_core::ir::TextRenderProfile::Canonical),
            expected_canonical
        );
    }
}

#[test]
fn test_invalid_compressed_encoding_reports_failure() {
    let dispatcher = ArchitectureDispatcher::new();
    let error = dispatcher
        .decode_instruction(&[0x00, 0x20], "riscv32", 0)
        .expect_err("invalid compressed encoding should fail");

    match error {
        robustone_core::DisasmError::DecodeFailure { kind, .. } => {
            assert_eq!(
                kind,
                robustone_core::types::error::DecodeErrorKind::InvalidEncoding
            );
        }
        other => panic!("expected decode failure, got {other:?}"),
    }
}
