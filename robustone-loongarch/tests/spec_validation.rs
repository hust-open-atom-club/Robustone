//! Spec validation tests for LoongArch backend.

use robustone_loongarch::backend::LOONGARCH_BASE_SPECS;

#[test]
fn test_specs_no_overlaps() {
    robustone_isa::validate_no_overlaps(LOONGARCH_BASE_SPECS).unwrap();
}

#[test]
fn test_specs_full_validation() {
    robustone_isa::check_spec_table(LOONGARCH_BASE_SPECS).unwrap();
}
