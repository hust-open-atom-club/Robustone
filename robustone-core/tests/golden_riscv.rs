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
    render_hint_mnemonic: String,
    hidden_operands: Vec<usize>,
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

#[test]
fn test_addi_li_golden_fixture() {
    let case = load_case("addi_li.json");
    let dispatcher = ArchitectureDispatcher::new();

    let compatibility = dispatcher.disassemble(&case.hex, case.arch.clone());
    assert_eq!(compatibility.mnemonic, case.expected_capstone.mnemonic);
    assert_eq!(compatibility.operands, case.expected_capstone.operands);

    let bytes = hex::decode(&case.hex).expect("hex fixture should decode");
    let (decoded, size) = dispatcher
        .decode_instruction(&bytes, &case.arch, 0)
        .expect("low-level decode should succeed");

    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, case.expected_ir.mnemonic);
    assert_eq!(
        decoded.render_hints.capstone_mnemonic.as_deref(),
        Some(case.expected_ir.render_hint_mnemonic.as_str())
    );
    assert_eq!(
        decoded.render_hints.capstone_hidden_operands,
        case.expected_ir.hidden_operands
    );
}
