use robustone_riscv::backend::{RiscVBackend, SPECS};

#[test]
fn test_specs_no_overlaps() {
    assert!(
        robustone_isa::validate_no_overlaps(SPECS).is_ok(),
        "RISC-V specs must not have overlapping patterns"
    );
}

#[test]
fn test_specs_full_validation() {
    let result = robustone_isa::check_spec_table(SPECS);
    if let Err(ref e) = result {
        eprintln!("check_spec_table error: {}", e);
    }
    assert!(
        result.is_ok(),
        "RISC-V spec table must pass all validation checks"
    );
}

#[test]
fn test_backend_decode_addi() {
    let profile = robustone_isa::DecodeProfile {
        mode: robustone_riscv::backend::RiscVMode::RV32,
        features: robustone_riscv::backend::RiscVFeature::I,
        render_dialect: robustone_isa::RenderDialect::Canonical,
        alias_policy: robustone_isa::AliasPolicy::None,
    };
    // addi x1, x2, 100 -> imm=100, rs1=2, rd=1, funct3=0, opcode=0x13
    let instruction = ((100u32 << 20) | (2u32 << 15)) | (1u32 << 7) | 0b0010011;
    let bytes = instruction.to_le_bytes();
    let decoded = robustone_isa::decode_one::<RiscVBackend>(&bytes, 0x1000, &profile).unwrap();
    assert_eq!(decoded.mnemonic, "addi");
    assert_eq!(decoded.size, 4);
}
