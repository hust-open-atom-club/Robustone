use robustone_riscv::backend;
use robustone_riscv::backend::RiscVBackend;

macro_rules! validate_extension {
    ($mod:ident) => {
        assert!(
            robustone_isa::validate_no_overlaps(backend::$mod::SPECS).is_ok(),
            "{} specs must not have overlapping patterns",
            stringify!($mod)
        );
        let result = robustone_isa::check_spec_table(backend::$mod::SPECS);
        if let Err(ref e) = result {
            eprintln!("{} check_spec_table error: {}", e, stringify!($mod));
        }
        assert!(
            result.is_ok(),
            "{} spec table must pass all validation checks",
            stringify!($mod)
        );
    };
}

#[test]
fn test_specs_no_overlaps() {
    validate_extension!(specs_i);
    validate_extension!(specs_m);
    validate_extension!(specs_a);
    validate_extension!(specs_f);
    validate_extension!(specs_d);
    validate_extension!(specs_c);
    validate_extension!(specs_system);
    validate_extension!(specs_thead);
}

#[test]
fn test_specs_full_validation() {
    // Each extension's spec table is validated within test_specs_no_overlaps.
    // This test verifies no cross-extension spec name collisions.
    let all_names: Vec<&str> = [
        backend::specs_i::SPECS,
        backend::specs_m::SPECS,
        backend::specs_a::SPECS,
        backend::specs_f::SPECS,
        backend::specs_d::SPECS,
        backend::specs_c::SPECS,
        backend::specs_system::SPECS,
        backend::specs_thead::SPECS,
    ]
    .iter()
    .flat_map(|s| s.iter())
    .map(|s| s.mnemonic())
    .collect();
    let mut unique = std::collections::BTreeSet::new();
    for name in &all_names {
        if !unique.insert(*name) {
            panic!("duplicate mnemonic across extensions: {}", name);
        }
    }
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
