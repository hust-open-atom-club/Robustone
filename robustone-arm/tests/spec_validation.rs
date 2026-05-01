use robustone_arm::backend::SPECS;

#[test]
fn test_specs_no_overlaps() {
    assert!(
        robustone_isa::validate_no_overlaps(SPECS).is_ok(),
        "ARM specs must not have overlapping patterns"
    );
}

#[test]
fn test_specs_full_validation() {
    assert!(
        robustone_isa::check_spec_table(SPECS).is_ok(),
        "ARM spec table must pass all validation checks"
    );
}
