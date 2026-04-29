use robustone_arm::backend::ARM_SPECS;

#[test]
fn test_specs_no_overlaps() {
    assert!(
        robustone_isa::validate_no_overlaps(ARM_SPECS).is_ok(),
        "ARM specs must not have overlapping patterns"
    );
}

#[test]
fn test_specs_full_validation() {
    assert!(
        robustone_isa::check_spec_table(ARM_SPECS).is_ok(),
        "ARM spec table must pass all validation checks"
    );
}
