use robustone_x86::backend::X86_SPECS;

#[test]
fn test_specs_no_overlaps() {
    assert!(
        robustone_isa::validate_no_overlaps(X86_SPECS).is_ok(),
        "x86 specs must not have overlapping patterns"
    );
}

#[test]
fn test_specs_full_validation() {
    assert!(
        robustone_isa::check_spec_table(X86_SPECS).is_ok(),
        "x86 spec table must pass all validation checks"
    );
}
