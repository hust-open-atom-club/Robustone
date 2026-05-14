use robustone_arm::backend::ALL_SPEC_SLICES;

#[test]
fn test_specs_no_overlaps() {
    for specs in ALL_SPEC_SLICES {
        if let Err(e) = robustone_isa::validate_no_overlaps(specs) {
            panic!("ARM specs overlap: {}", e);
        }
    }
}

#[test]
fn test_specs_full_validation() {
    for specs in ALL_SPEC_SLICES {
        if let Err(e) = robustone_isa::check_spec_table(specs) {
            panic!("ARM spec table validation failed: {}", e);
        }
    }
}
