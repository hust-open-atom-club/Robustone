use robustone_arm::{
    ArmHandler,
    backend::{ArmBackend, generated::ALL_GENERATED_SPEC_SLICES},
};
use robustone_core::{
    ir::{ArchitectureId, Operand, RegisterId},
    traits::ArchitectureHandler,
};
use robustone_isa::{DecodeProfile, InstructionSpec};

fn generated_specs() -> Vec<&'static InstructionSpec<ArmBackend>> {
    ALL_GENERATED_SPEC_SLICES
        .iter()
        .flat_map(|slice| slice.iter())
        .collect()
}

fn assert_register_operand(
    operand: &robustone_isa::OperandSpec<ArmBackend>,
    expected_class: robustone_arm::backend::ArmRegisterClass,
    expected_field: robustone_arm::backend::ArmField,
    expected_access: robustone_isa::Access,
) {
    match operand {
        robustone_isa::OperandSpec::Register {
            class,
            field,
            access,
        } => {
            assert_eq!(*class, expected_class);
            assert_eq!(*field, expected_field);
            assert_eq!(*access, expected_access);
        }
        _ => panic!("expected register operand"),
    }
}

#[test]
fn generated_baseline_contains_active_tablegen_modules() {
    assert_eq!(ALL_GENERATED_SPEC_SLICES.len(), 9);
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "ADDWRI")
    );
    assert!(generated_specs().iter().any(|spec| spec.opcode_id() == "B"));
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "LDRWUI")
    );
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "BRK")
    );
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "FADDSRR")
    );
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "AESMCRR")
    );
    assert!(
        generated_specs()
            .iter()
            .any(|spec| spec.opcode_id() == "ADDG")
    );
}

#[test]
fn generated_specs_include_m3_operand_metadata_for_representative_schemas() {
    let specs = generated_specs();
    let add = specs
        .iter()
        .find(|spec| spec.opcode_id() == "ADDWRI")
        .expect("ADDWRI should be generated");
    assert_eq!(add.format().name(), "I_ADD");
    assert_eq!(add.operands().len(), 3);

    let bcc = specs
        .iter()
        .find(|spec| spec.opcode_id() == "BCC")
        .expect("BCC should be generated");
    assert_eq!(bcc.format().name(), "FMT_B_COND");
    assert_eq!(bcc.operands().len(), 2);

    let ldr = specs
        .iter()
        .find(|spec| spec.opcode_id() == "LDRWUI")
        .expect("LDRWUI should be generated");
    assert_eq!(ldr.format().name(), "LDR_IMM");
    assert_eq!(ldr.operands().len(), 3);
}

#[test]
fn generated_baseline_includes_tablegen_compound_register_specs() {
    let specs = generated_specs();

    let addwrx = specs
        .iter()
        .find(|spec| spec.opcode_id() == "ADDWRX")
        .expect("ADDWRX should be generated after compound extended-register support");
    assert_eq!(addwrx.operands().len(), 3);
}

#[test]
fn generated_specs_include_m5_sve_sme_predicate_vector_metadata() {
    let specs = generated_specs();

    assert_eq!(ALL_GENERATED_SPEC_SLICES.len(), 9);
    assert!(specs.iter().any(|spec| {
        spec.features()
            .contains(robustone_arm::backend::ArmFeature::SVE)
    }));
    assert!(specs.iter().any(|spec| {
        spec.features()
            .contains(robustone_arm::backend::ArmFeature::SME)
    }));

    let sve_pred = specs
        .iter()
        .find(|spec| spec.opcode_id() == "CMPEQ_PPZZI_B")
        .expect("SVE predicate/vector immediate spec should be generated");
    assert_eq!(sve_pred.format().name(), "SVE_PRED_ZI");
    assert_eq!(sve_pred.operands().len(), 4);

    let sme_tile = specs
        .iter()
        .find(|spec| spec.opcode_id() == "ADDHA_MPPZ_D")
        .expect("SME tile predicate spec should be generated");
    assert_eq!(sme_tile.format().name(), "SME_TILE_PRED_Z");
    assert_eq!(sme_tile.operands().len(), 4);
}

#[test]
fn generated_sve_zpmz_specs_preserve_predicate_tied_vector_metadata_for_all_widths() {
    let specs = generated_specs();

    for opcode in ["ADD_ZPMZ_B", "ADD_ZPMZ_H", "ADD_ZPMZ_S", "ADD_ZPMZ_D"] {
        let spec = specs
            .iter()
            .find(|spec| spec.opcode_id() == opcode)
            .unwrap_or_else(|| panic!("{opcode} should be generated"));
        assert_eq!(spec.format().name(), "SVE_PRED_Z");
        assert_eq!(spec.operands().len(), 3);
        assert!(
            spec.features()
                .contains(robustone_arm::backend::ArmFeature::SVE)
        );
        assert_eq!(spec.groups(), &[robustone_isa::InstructionGroup::Vector]);
        assert_register_operand(
            &spec.operands()[0],
            robustone_arm::backend::ArmRegisterClass::Pred,
            robustone_arm::backend::ArmField::Cond,
            robustone_isa::Access::Read,
        );
        assert_register_operand(
            &spec.operands()[1],
            robustone_arm::backend::ArmRegisterClass::ZVec,
            robustone_arm::backend::ArmField::Rd,
            robustone_isa::Access::ReadWrite,
        );
        assert_register_operand(
            &spec.operands()[2],
            robustone_arm::backend::ArmRegisterClass::ZVec,
            robustone_arm::backend::ArmField::Rm,
            robustone_isa::Access::Read,
        );
    }
}

#[test]
fn generated_baseline_excludes_fpr_load_store_fallback_specs() {
    let specs = generated_specs();

    assert!(!specs.iter().any(|spec| spec.opcode_id() == "LDRSUI"));
    assert!(!specs.iter().any(|spec| spec.pattern().value == 0xbd40_0000));
}

#[test]
fn base_profile_does_not_decode_deferred_fpr_load_store_fallback() {
    let profile = DecodeProfile {
        mode: robustone_arm::backend::ArmMode::AArch64,
        features: robustone_arm::backend::ArmFeature::BASE,
        render_dialect: robustone_isa::RenderDialect::Canonical,
        alias_policy: robustone_isa::AliasPolicy::None,
    };

    let err = robustone_isa::decode_one::<ArmBackend>(&[0x00, 0x00, 0x40, 0xbd], 0, &profile)
        .expect_err("BASE-only profile must not decode FP LDR fallback");

    assert_eq!(err.stable_kind(), "unsupported_extension");
}

#[test]
fn generated_baseline_does_not_override_existing_manual_decode() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x20, 0xfc, 0xa2, 0x6e], "aarch64", 0)
        .expect("manual SIMD fdiv should still decode with the limited generated baseline");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "fdiv");
    assert_eq!(instruction.operands, "v0.4s, v1.4s, v2.4s");
}

#[test]
fn generated_baseline_does_not_override_manual_branch_operands() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x00, 0x00, 0x00, 0x14], "aarch64", 0)
        .expect("manual B should still decode with operands");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "b");
    assert_eq!(instruction.operands, "0");
}

#[test]
fn generated_baseline_does_not_override_manual_ldr_w_unsigned_immediate_operands() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x20, 0x00, 0x40, 0xb9], "aarch64", 0)
        .expect("manual LDR W unsigned immediate should still decode with operands");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "ldr");
    assert_eq!(instruction.operands, "w0, [x1]");
}

#[test]
fn generated_spec_slices_are_wired_into_backend_lookup() {
    let generated_count: usize = robustone_arm::backend::generated::ALL_GENERATED_SPEC_SLICES
        .iter()
        .map(|slice| slice.len())
        .sum();

    let all_count = robustone_arm::backend::ALL_SPEC_SLICES
        .iter()
        .chain(robustone_arm::backend::generated::ALL_GENERATED_SPEC_SLICES.iter())
        .map(|slice| slice.len())
        .sum::<usize>();

    assert!(generated_count > 0);
    assert!(all_count >= generated_count);
}

#[test]
fn generated_m5_sve_while_compare_text_preserves_predicate_element_and_gpr_width() {
    let handler = ArmHandler::new();

    let (whilege_w, whilege_w_size) = handler
        .disassemble(&[0xef, 0x03, 0x20, 0x25], "aarch64", 0)
        .expect("generated SVE WHILEGE PWW should decode");
    assert_eq!(whilege_w_size, 4);
    assert_eq!(whilege_w.mnemonic, "whilege");
    assert_eq!(whilege_w.operands, "p15.b, wzr, w0");

    let (whilege_w_decoded, whilege_w_decode_size) = handler
        .decode_instruction(&[0xef, 0x03, 0x20, 0x25], "aarch64", 0)
        .expect("generated SVE WHILEGE PWW detail should decode");
    assert_eq!(whilege_w_decode_size, 4);
    assert_eq!(
        whilege_w_decoded.registers_read,
        vec![RegisterId::aarch64(31), RegisterId::aarch64(0)]
    );
    assert_eq!(
        whilege_w_decoded.registers_written,
        vec![RegisterId::aarch64(143)]
    );

    let (whilerw, whilerw_size) = handler
        .disassemble(&[0xdf, 0x33, 0x3e, 0x25], "aarch64", 0)
        .expect("generated SVE2 WHILERW PXX should decode");
    assert_eq!(whilerw_size, 4);
    assert_eq!(whilerw.mnemonic, "whilerw");
    assert_eq!(whilerw.operands, "p15.b, x30, x30");

    let (whilerw_decoded, whilerw_decode_size) = handler
        .decode_instruction(&[0xdf, 0x33, 0x3e, 0x25], "aarch64", 0)
        .expect("generated SVE2 WHILERW PXX detail should decode");
    assert_eq!(whilerw_decode_size, 4);
    assert_eq!(
        whilerw_decoded.registers_read,
        vec![RegisterId::aarch64(30), RegisterId::aarch64(30)]
    );
    assert_eq!(
        whilerw_decoded.registers_written,
        vec![RegisterId::aarch64(143)]
    );

    let (whilegt_x, whilegt_x_size) = handler
        .disassemble(&[0xff, 0x13, 0xe0, 0x25], "aarch64", 0)
        .expect("generated SVE WHILEGT PXX should decode");
    assert_eq!(whilegt_x_size, 4);
    assert_eq!(whilegt_x.mnemonic, "whilegt");
    assert_eq!(whilegt_x.operands, "p15.d, xzr, x0");

    let (whilegt_x_decoded, whilegt_x_decode_size) = handler
        .decode_instruction(&[0xff, 0x13, 0xe0, 0x25], "aarch64", 0)
        .expect("generated SVE WHILEGT PXX detail should decode");
    assert_eq!(whilegt_x_decode_size, 4);
    assert_eq!(
        whilegt_x_decoded.registers_read,
        vec![RegisterId::aarch64(31), RegisterId::aarch64(0)]
    );
    assert_eq!(
        whilegt_x_decoded.registers_written,
        vec![RegisterId::aarch64(143)]
    );
}

#[test]
fn generated_m5_sve_predicated_vector_text_preserves_arrangement_and_predicate_mode() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x00, 0xa0, 0x16, 0x04], "aarch64", 0)
        .expect("generated SVE ABS should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "abs");
    assert_eq!(instruction.operands, "z0.b, p0/m, z0.b");
}

#[test]
fn generated_m5_sme_tile_predicate_text_preserves_tile_and_predicate_modes() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x00, 0x00, 0x90, 0xc0], "aarch64", 0)
        .expect("generated SME ADDHA should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "addha");
    assert_eq!(instruction.operands, "za0.s, p0/m, p0/m, z0.s");
}

#[test]
fn generated_m5_sve_binary_vector_text_preserves_source_register_and_arrangement() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0xb7, 0x01, 0xe8, 0x04], "aarch64", 0)
        .expect("generated SVE ADD should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "add");
    assert_eq!(instruction.operands, "z23.d, z13.d, z8.d");
}

#[test]
fn generated_m5_sve_predicated_binary_text_preserves_tied_destination() {
    let handler = ArmHandler::new();

    for (bytes, expected) in [
        ([0xb7, 0x0d, 0x00, 0x04], "z23.b, p3/m, z23.b, z13.b"),
        ([0xb7, 0x0d, 0x40, 0x04], "z23.h, p3/m, z23.h, z13.h"),
        ([0xb7, 0x0d, 0x80, 0x04], "z23.s, p3/m, z23.s, z13.s"),
        ([0xb7, 0x0d, 0xc0, 0x04], "z23.d, p3/m, z23.d, z13.d"),
    ] {
        let (instruction, size) = handler
            .disassemble(&bytes, "aarch64", 0)
            .expect("generated predicated SVE ADD should decode");

        assert_eq!(size, 4);
        assert_eq!(instruction.mnemonic, "add");
        assert_eq!(instruction.operands, expected);
    }
}

#[test]
fn generated_m5_sve_movprfx_text_preserves_predicate_mode() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0xc4, 0x3c, 0xd0, 0x04], "aarch64", 0)
        .expect("generated predicated SVE MOVPRFX should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "movprfx");
    assert_eq!(instruction.operands, "z4.d, p7/z, z6.d");
}

#[test]
fn generated_m5_sme_double_tile_text_preserves_extended_tile_number() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0x45, 0x55, 0xd0, 0xc0], "aarch64", 0)
        .expect("generated SME ADDHA doubleword should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "addha");
    assert_eq!(instruction.operands, "za5.d, p5/m, p2/m, z10.d");
}

#[test]
fn generated_m5_sme_streaming_vl_add_text_preserves_signed_immediates() {
    let handler = ArmHandler::new();

    let (addspl, addspl_size) = handler
        .disassemble(&[0xf7, 0x5f, 0x68, 0x04], "aarch64", 0)
        .expect("generated SME ADDSPL should decode");
    assert_eq!(addspl_size, 4);
    assert_eq!(addspl.mnemonic, "addspl");
    assert_eq!(addspl.operands, "x23, x8, #-1");

    let (addsvl, addsvl_size) = handler
        .disassemble(&[0x00, 0x5c, 0x20, 0x04], "aarch64", 0)
        .expect("generated SME ADDSVL should decode");
    assert_eq!(addsvl_size, 4);
    assert_eq!(addsvl.mnemonic, "addsvl");
    assert_eq!(addsvl.operands, "x0, x0, #-0x20");
}

#[test]
fn generated_m5_sve_movprfx_text_preserves_byte_arrangement() {
    let handler = ArmHandler::new();
    let (instruction, size) = handler
        .disassemble(&[0xc4, 0x3c, 0x10, 0x04], "aarch64", 0)
        .expect("generated predicated SVE MOVPRFX byte should decode");

    assert_eq!(size, 4);
    assert_eq!(instruction.mnemonic, "movprfx");
    assert_eq!(instruction.operands, "z4.b, p7/z, z6.b");
}

#[test]
fn generated_m5_sve_vl_add_text_preserves_source_register_and_signed_immediate() {
    let handler = ArmHandler::new();

    let (addpl, addpl_size) = handler
        .disassemble(&[0xf7, 0x57, 0x68, 0x04], "aarch64", 0)
        .expect("generated SVE ADDPL should decode");
    assert_eq!(addpl_size, 4);
    assert_eq!(addpl.mnemonic, "addpl");
    assert_eq!(addpl.operands, "x23, x8, #-1");

    let (addvl, addvl_size) = handler
        .disassemble(&[0x15, 0x50, 0x35, 0x04], "aarch64", 0)
        .expect("generated SVE ADDVL should decode");
    assert_eq!(addvl_size, 4);
    assert_eq!(addvl.mnemonic, "addvl");
    assert_eq!(addvl.operands, "x21, x21, #0");
}

#[test]
fn generated_m5_sve_adr_text_preserves_arrangement_and_address_operands() {
    let handler = ArmHandler::new();

    let (plain, plain_size) = handler
        .disassemble(&[0x00, 0xa0, 0xa0, 0x04], "aarch64", 0)
        .expect("generated SVE ADR should decode");
    assert_eq!(plain_size, 4);
    assert_eq!(plain.mnemonic, "adr");
    assert_eq!(plain.operands, "z0.s, [z0.s, z0.s]");

    let (shifted, shifted_size) = handler
        .disassemble(&[0x00, 0xa4, 0xa0, 0x04], "aarch64", 0)
        .expect("generated SVE shifted ADR should decode");
    assert_eq!(shifted_size, 4);
    assert_eq!(shifted.mnemonic, "adr");
    assert_eq!(shifted.operands, "z0.s, [z0.s, z0.s, lsl #1]");
}

#[test]
fn generated_m5_sve_prefetch_text_preserves_prfop_and_address_modes() {
    let handler = ArmHandler::new();

    let (vector_uxtw, vector_uxtw_size) = handler
        .disassemble(&[0x00, 0x00, 0x20, 0x84], "aarch64", 0)
        .expect("generated SVE PRFB vector offset should decode");
    assert_eq!(vector_uxtw_size, 4);
    assert_eq!(vector_uxtw.mnemonic, "prfb");
    assert_eq!(vector_uxtw.operands, "pldl1keep, p0, [x0, z0.s, uxtw]");

    let (vector_uxtw_decoded, vector_uxtw_decode_size) = handler
        .decode_instruction(&[0x00, 0x00, 0x20, 0x84], "aarch64", 0)
        .expect("generated SVE PRFB vector offset detail should decode");
    assert_eq!(vector_uxtw_decode_size, 4);
    assert_eq!(
        vector_uxtw_decoded.registers_read,
        vec![
            RegisterId::aarch64(128),
            RegisterId::aarch64(0),
            RegisterId::aarch64(160)
        ]
    );
    assert_eq!(
        vector_uxtw_decoded.registers_written,
        Vec::<RegisterId>::new()
    );

    let (vector_lsl, vector_lsl_size) = handler
        .disassemble(&[0x00, 0xe0, 0x60, 0xc4], "aarch64", 0)
        .expect("generated SVE PRFD vector lsl offset should decode");
    assert_eq!(vector_lsl_size, 4);
    assert_eq!(vector_lsl.mnemonic, "prfd");
    assert_eq!(vector_lsl.operands, "pldl1keep, p0, [x0, z0.d, lsl #3]");

    let (vector_imm, vector_imm_size) = handler
        .disassemble(&[0xef, 0xff, 0x9f, 0x84], "aarch64", 0)
        .expect("generated SVE PRFH vector immediate should decode");
    assert_eq!(vector_imm_size, 4);
    assert_eq!(vector_imm.mnemonic, "prfh");
    assert_eq!(vector_imm.operands, "#15, p7, [z31.s, #62]");

    let (scalar_imm, scalar_imm_size) = handler
        .disassemble(&[0x05, 0x00, 0xc0, 0x85], "aarch64", 0)
        .expect("generated SVE PRFB scalar base should decode");
    assert_eq!(scalar_imm_size, 4);
    assert_eq!(scalar_imm.mnemonic, "prfb");
    assert_eq!(scalar_imm.operands, "pldl3strm, p0, [x0]");
}

#[test]
fn generated_m5_sve_tuple_memory_text_preserves_vector_list_predicate_and_address_modes() {
    let handler = ArmHandler::new();

    let (ld2b, ld2b_size) = handler
        .disassemble(&[0x00, 0xc0, 0x20, 0xa4], "aarch64", 0)
        .expect("generated SVE LD2B register offset should decode");
    assert_eq!(ld2b_size, 4);
    assert_eq!(ld2b.mnemonic, "ld2b");
    assert_eq!(ld2b.operands, "{ z0.b, z1.b }, p0/z, [x0, x0]");

    let (ld2b_decoded, ld2b_decode_size) = handler
        .decode_instruction(&[0x00, 0xc0, 0x20, 0xa4], "aarch64", 0)
        .expect("generated SVE LD2B register offset detail should decode");
    assert_eq!(ld2b_decode_size, 4);
    assert_eq!(
        ld2b_decoded.registers_read,
        vec![
            RegisterId::aarch64(128),
            RegisterId::aarch64(0),
            RegisterId::aarch64(0)
        ]
    );
    assert_eq!(
        ld2b_decoded.registers_written,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(161)]
    );

    let (ld3h, ld3h_size) = handler
        .disassemble(&[0x00, 0xc0, 0xc0, 0xa4], "aarch64", 0)
        .expect("generated SVE LD3H register offset should decode");
    assert_eq!(ld3h_size, 4);
    assert_eq!(ld3h.mnemonic, "ld3h");
    assert_eq!(ld3h.operands, "{ z0.h - z2.h }, p0/z, [x0, x0, lsl #1]");

    let (ld2b_imm, ld2b_imm_size) = handler
        .disassemble(&[0xb7, 0xed, 0x28, 0xa4], "aarch64", 0)
        .expect("generated SVE LD2B immediate offset should decode");
    assert_eq!(ld2b_imm_size, 4);
    assert_eq!(ld2b_imm.mnemonic, "ld2b");
    assert_eq!(
        ld2b_imm.operands,
        "{ z23.b, z24.b }, p3/z, [x13, #-16, mul vl]"
    );

    let (st3h_imm, st3h_imm_size) = handler
        .disassemble(&[0xb7, 0xed, 0xd8, 0xe4], "aarch64", 0)
        .expect("generated SVE ST3H immediate offset should decode");
    assert_eq!(st3h_imm_size, 4);
    assert_eq!(st3h_imm.mnemonic, "st3h");
    assert_eq!(
        st3h_imm.operands,
        "{ z23.h - z25.h }, p3, [x13, #-24, mul vl]"
    );

    let (st4d, st4d_size) = handler
        .disassemble(&[0x00, 0x60, 0xe0, 0xe5], "aarch64", 0)
        .expect("generated SVE ST4D register offset should decode");
    assert_eq!(st4d_size, 4);
    assert_eq!(st4d.mnemonic, "st4d");
    assert_eq!(st4d.operands, "{ z0.d - z3.d }, p0, [x0, x0, lsl #3]");

    let (st4d_decoded, st4d_decode_size) = handler
        .decode_instruction(&[0x00, 0x60, 0xe0, 0xe5], "aarch64", 0)
        .expect("generated SVE ST4D register offset detail should decode");
    assert_eq!(st4d_decode_size, 4);
    assert_eq!(
        st4d_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(162),
            RegisterId::aarch64(163),
            RegisterId::aarch64(128),
            RegisterId::aarch64(0),
            RegisterId::aarch64(0)
        ]
    );
    assert_eq!(st4d_decoded.registers_written, Vec::<RegisterId>::new());
}

#[test]
fn generated_m5_sve2_binary_vector_text_preserves_arrangement_and_third_source() {
    let handler = ArmHandler::new();

    let (adclb_s, adclb_s_size) = handler
        .disassemble(&[0x20, 0xd0, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADCLB word should decode");
    assert_eq!(adclb_s_size, 4);
    assert_eq!(adclb_s.mnemonic, "adclb");
    assert_eq!(adclb_s.operands, "z0.s, z1.s, z31.s");

    let (adclb_s_decoded, adclb_s_decode_size) = handler
        .decode_instruction(&[0x20, 0xd0, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADCLB word detail should decode");
    assert_eq!(adclb_s_decode_size, 4);
    assert_eq!(
        adclb_s_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        adclb_s_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (adclb_d, adclb_d_size) = handler
        .disassemble(&[0x20, 0xd0, 0x5f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADCLB doubleword should decode");
    assert_eq!(adclb_d_size, 4);
    assert_eq!(adclb_d.mnemonic, "adclb");
    assert_eq!(adclb_d.operands, "z0.d, z1.d, z31.d");

    let (sbclb, sbclb_size) = handler
        .disassemble(&[0x20, 0xd0, 0x9f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SBCLB should decode");
    assert_eq!(sbclb_size, 4);
    assert_eq!(sbclb.mnemonic, "sbclb");
    assert_eq!(sbclb.operands, "z0.s, z1.s, z31.s");

    let (sbclb_decoded, sbclb_decode_size) = handler
        .decode_instruction(&[0x20, 0xd0, 0x9f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SBCLB detail should decode");
    assert_eq!(sbclb_decode_size, 4);
    assert_eq!(
        sbclb_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        sbclb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );
    let (addp, addp_size) = handler
        .disassemble(&[0x20, 0xa0, 0x11, 0x44], "aarch64", 0)
        .expect("generated SVE2 ADDP predicated vector should decode");
    assert_eq!(addp_size, 4);
    assert_eq!(addp.mnemonic, "addp");
    assert_eq!(addp.operands, "z0.b, p0/m, z0.b, z1.b");

    let (addp_decoded, addp_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x11, 0x44], "aarch64", 0)
        .expect("generated SVE2 ADDP predicated vector detail should decode");
    assert_eq!(addp_decode_size, 4);
    assert_eq!(
        addp_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161)
        ]
    );
    assert_eq!(
        addp_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (faddp, faddp_size) = handler
        .disassemble(&[0x20, 0x80, 0x50, 0x64], "aarch64", 0)
        .expect("generated SVE FADDP pairwise should decode");
    assert_eq!(faddp_size, 4);
    assert_eq!(faddp.mnemonic, "faddp");
    assert_eq!(faddp.operands, "z0.h, p0/m, z0.h, z1.h");

    let (fmaxnmp, fmaxnmp_size) = handler
        .disassemble(&[0x20, 0x80, 0x54, 0x64], "aarch64", 0)
        .expect("generated SVE FMAXNMP pairwise should decode");
    assert_eq!(fmaxnmp_size, 4);
    assert_eq!(fmaxnmp.mnemonic, "fmaxnmp");
    assert_eq!(fmaxnmp.operands, "z0.h, p0/m, z0.h, z1.h");

    let (fmlalb, fmlalb_size) = handler
        .disassemble(&[0xdd, 0x83, 0xbf, 0x64], "aarch64", 0)
        .expect("generated SVE2 FMLALB long multiply-add should decode");
    assert_eq!(fmlalb_size, 4);
    assert_eq!(fmlalb.mnemonic, "fmlalb");
    assert_eq!(fmlalb.operands, "z29.s, z30.h, z31.h");

    let (fmlalb_indexed, fmlalb_indexed_size) = handler
        .disassemble(&[0x20, 0x40, 0xa7, 0x64], "aarch64", 0)
        .expect("generated SVE2 FMLALB indexed long multiply-add should decode");
    assert_eq!(fmlalb_indexed_size, 4);
    assert_eq!(fmlalb_indexed.mnemonic, "fmlalb");
    assert_eq!(fmlalb_indexed.operands, "z0.s, z1.h, z7.h[0]");

    let (fmlalb_indexed_max, fmlalb_indexed_max_size) = handler
        .disassemble(&[0xfe, 0x4b, 0xbf, 0x64], "aarch64", 0)
        .expect("generated SVE2 FMLALB indexed max lane should decode");
    assert_eq!(fmlalb_indexed_max_size, 4);
    assert_eq!(fmlalb_indexed_max.mnemonic, "fmlalb");
    assert_eq!(fmlalb_indexed_max.operands, "z30.s, z31.h, z7.h[7]");

    let (fmlalb_indexed_decoded, fmlalb_indexed_decode_size) = handler
        .decode_instruction(&[0x20, 0x40, 0xa7, 0x64], "aarch64", 0)
        .expect("generated SVE2 FMLALB indexed detail should decode");
    assert_eq!(fmlalb_indexed_decode_size, 4);
    assert_eq!(
        fmlalb_indexed_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(167)
        ]
    );
    assert_eq!(
        fmlalb_indexed_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (smlalb_indexed_d, smlalb_indexed_d_size) = handler
        .disassemble(&[0x20, 0x88, 0xef, 0x44], "aarch64", 0)
        .expect("generated SVE2 SMLALB indexed doubleword should decode");
    assert_eq!(smlalb_indexed_d_size, 4);
    assert_eq!(smlalb_indexed_d.mnemonic, "smlalb");
    assert_eq!(smlalb_indexed_d.operands, "z0.d, z1.s, z15.s[1]");

    let (smlalb_indexed_d_decoded, smlalb_indexed_d_decode_size) = handler
        .decode_instruction(&[0x20, 0x88, 0xef, 0x44], "aarch64", 0)
        .expect("generated SVE2 SMLALB indexed doubleword detail should decode");
    assert_eq!(smlalb_indexed_d_decode_size, 4);
    assert_eq!(
        smlalb_indexed_d_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(175)
        ]
    );
    assert_eq!(
        smlalb_indexed_d_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (faddp_decoded, faddp_decode_size) = handler
        .decode_instruction(&[0x20, 0x80, 0x50, 0x64], "aarch64", 0)
        .expect("generated SVE FADDP pairwise detail should decode");
    assert_eq!(faddp_decode_size, 4);
    assert_eq!(
        faddp_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161)
        ]
    );
    assert_eq!(
        faddp_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (fcvtlt, fcvtlt_size) = handler
        .disassemble(&[0x20, 0xa0, 0x89, 0x64], "aarch64", 0)
        .expect("generated SVE FCVTLT should decode");
    assert_eq!(fcvtlt_size, 4);
    assert_eq!(fcvtlt.mnemonic, "fcvtlt");
    assert_eq!(fcvtlt.operands, "z0.s, p0/m, z1.h");

    let (fcvtlt_decoded, fcvtlt_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x89, 0x64], "aarch64", 0)
        .expect("generated SVE FCVTLT detail should decode");
    assert_eq!(fcvtlt_decode_size, 4);
    assert_eq!(
        fcvtlt_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161)
        ]
    );
    assert_eq!(
        fcvtlt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (ext, ext_size) = handler
        .disassemble(&[0x20, 0x00, 0x60, 0x05], "aarch64", 0)
        .expect("generated SVE2 EXT vector-list should decode");
    assert_eq!(ext_size, 4);
    assert_eq!(ext.mnemonic, "ext");
    assert_eq!(ext.operands, "z0.b, { z1.b, z2.b }, #0");

    let (ext_decoded, ext_decode_size) = handler
        .decode_instruction(&[0x20, 0x00, 0x60, 0x05], "aarch64", 0)
        .expect("generated SVE2 EXT vector-list detail should decode");
    assert_eq!(ext_decode_size, 4);
    assert_eq!(
        ext_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        ext_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (ext_max, ext_max_size) = handler
        .disassemble(&[0xdf, 0x1f, 0x7f, 0x05], "aarch64", 0)
        .expect("generated SVE2 EXT max vector-list immediate should decode");
    assert_eq!(ext_max_size, 4);
    assert_eq!(ext_max.mnemonic, "ext");
    assert_eq!(ext_max.operands, "z31.b, { z30.b, z31.b }, #0xff");

    let (nbsl, nbsl_size) = handler
        .disassemble(&[0x40, 0x3c, 0xe1, 0x04], "aarch64", 0)
        .expect("generated SVE2 NBSL should decode");
    assert_eq!(nbsl_size, 4);
    assert_eq!(nbsl.mnemonic, "nbsl");
    assert_eq!(nbsl.operands, "z0.d, z0.d, z1.d, z2.d");

    let (nbsl_decoded, nbsl_decode_size) = handler
        .decode_instruction(&[0x40, 0x3c, 0xe1, 0x04], "aarch64", 0)
        .expect("generated SVE2 NBSL detail should decode");
    assert_eq!(nbsl_decode_size, 4);
    assert_eq!(
        nbsl_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(162)
        ]
    );
    assert_eq!(
        nbsl_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (pmullb, pmullb_size) = handler
        .disassemble(&[0xdd, 0x6b, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 PMULLB quadword should decode");
    assert_eq!(pmullb_size, 4);
    assert_eq!(pmullb.mnemonic, "pmullb");
    assert_eq!(pmullb.operands, "z29.q, z30.d, z31.d");

    let (pmullb_decoded, pmullb_decode_size) = handler
        .decode_instruction(&[0xdd, 0x6b, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 PMULLB quadword detail should decode");
    assert_eq!(pmullb_decode_size, 4);
    assert_eq!(
        pmullb_decoded.registers_read,
        vec![RegisterId::aarch64(190), RegisterId::aarch64(191)]
    );
    assert_eq!(
        pmullb_decoded.registers_written,
        vec![RegisterId::aarch64(189)]
    );

    let (pmullb_h, pmullb_h_size) = handler
        .disassemble(&[0x20, 0x68, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 PMULLB halfword should decode");
    assert_eq!(pmullb_h_size, 4);
    assert_eq!(pmullb_h.mnemonic, "pmullb");
    assert_eq!(pmullb_h.operands, "z0.h, z1.b, z2.b");

    let (pmullb_h_decoded, pmullb_h_decode_size) = handler
        .decode_instruction(&[0x20, 0x68, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 PMULLB halfword detail should decode");
    assert_eq!(pmullb_h_decode_size, 4);
    assert_eq!(
        pmullb_h_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        pmullb_h_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (mla_indexed, mla_indexed_size) = handler
        .disassemble(&[0x20, 0x08, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 MLA indexed halfword should decode");
    assert_eq!(mla_indexed_size, 4);
    assert_eq!(mla_indexed.mnemonic, "mla");
    assert_eq!(mla_indexed.operands, "z0.h, z1.h, z7.h[7]");

    let (mla_indexed_decoded, mla_indexed_decode_size) = handler
        .decode_instruction(&[0x20, 0x08, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 MLA indexed halfword detail should decode");
    assert_eq!(mla_indexed_decode_size, 4);
    assert_eq!(
        mla_indexed_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(167)
        ]
    );
    assert_eq!(
        mla_indexed_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqdmulh_indexed, sqdmulh_indexed_size) = handler
        .disassemble(&[0x20, 0xf0, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed halfword should decode");
    assert_eq!(sqdmulh_indexed_size, 4);
    assert_eq!(sqdmulh_indexed.mnemonic, "sqdmulh");
    assert_eq!(sqdmulh_indexed.operands, "z0.h, z1.h, z7.h[7]");

    let (sqdmulh_indexed_decoded, sqdmulh_indexed_decode_size) = handler
        .decode_instruction(&[0x20, 0xf0, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed halfword detail should decode");
    assert_eq!(sqdmulh_indexed_decode_size, 4);
    assert_eq!(
        sqdmulh_indexed_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(167)]
    );
    assert_eq!(
        sqdmulh_indexed_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqdmulh_indexed_s, sqdmulh_indexed_s_size) = handler
        .disassemble(&[0x20, 0xf0, 0xbf, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed word should decode");
    assert_eq!(sqdmulh_indexed_s_size, 4);
    assert_eq!(sqdmulh_indexed_s.mnemonic, "sqdmulh");
    assert_eq!(sqdmulh_indexed_s.operands, "z0.s, z1.s, z7.s[3]");

    let (sqdmulh_indexed_s_decoded, sqdmulh_indexed_s_decode_size) = handler
        .decode_instruction(&[0x20, 0xf0, 0xbf, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed word detail should decode");
    assert_eq!(sqdmulh_indexed_s_decode_size, 4);
    assert_eq!(
        sqdmulh_indexed_s_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(167)]
    );
    assert_eq!(
        sqdmulh_indexed_s_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqdmulh_indexed_d, sqdmulh_indexed_d_size) = handler
        .disassemble(&[0x20, 0xf0, 0xff, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed doubleword should decode");
    assert_eq!(sqdmulh_indexed_d_size, 4);
    assert_eq!(sqdmulh_indexed_d.mnemonic, "sqdmulh");
    assert_eq!(sqdmulh_indexed_d.operands, "z0.d, z1.d, z15.d[1]");

    let (sqdmulh_indexed_d_decoded, sqdmulh_indexed_d_decode_size) = handler
        .decode_instruction(&[0x20, 0xf0, 0xff, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMULH indexed doubleword detail should decode");
    assert_eq!(sqdmulh_indexed_d_decode_size, 4);
    assert_eq!(
        sqdmulh_indexed_d_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(175)]
    );
    assert_eq!(
        sqdmulh_indexed_d_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (mul_indexed, mul_indexed_size) = handler
        .disassemble(&[0x20, 0xf8, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 MUL indexed halfword should decode");
    assert_eq!(mul_indexed_size, 4);
    assert_eq!(mul_indexed.mnemonic, "mul");
    assert_eq!(mul_indexed.operands, "z0.h, z1.h, z7.h[7]");

    let (mul_indexed_decoded, mul_indexed_decode_size) = handler
        .decode_instruction(&[0x20, 0xf8, 0x7f, 0x44], "aarch64", 0)
        .expect("generated SVE2 MUL indexed halfword detail should decode");
    assert_eq!(mul_indexed_decode_size, 4);
    assert_eq!(
        mul_indexed_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(167)]
    );
    assert_eq!(
        mul_indexed_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (histcnt, histcnt_size) = handler
        .disassemble(&[0x20, 0xc0, 0xa2, 0x45], "aarch64", 0)
        .expect("generated SVE2 HISTCNT should decode");
    assert_eq!(histcnt_size, 4);
    assert_eq!(histcnt.mnemonic, "histcnt");
    assert_eq!(histcnt.operands, "z0.s, p0/z, z1.s, z2.s");

    let (sve_match, sve_match_size) = handler
        .disassemble(&[0x00, 0x80, 0x20, 0x45], "aarch64", 0)
        .expect("generated SVE2 MATCH should decode");
    assert_eq!(sve_match_size, 4);
    assert_eq!(sve_match.mnemonic, "match");
    assert_eq!(sve_match.operands, "p0.b, p0/z, z0.b, z0.b");

    let (sve_match_decoded, sve_match_decode_size) = handler
        .decode_instruction(&[0x00, 0x80, 0x20, 0x45], "aarch64", 0)
        .expect("generated SVE2 MATCH detail should decode");
    assert_eq!(sve_match_decode_size, 4);
    assert_eq!(
        sve_match_decoded.registers_read,
        vec![RegisterId::aarch64(128), RegisterId::aarch64(160)]
    );
    assert_eq!(
        sve_match_decoded.registers_written,
        vec![RegisterId::aarch64(128)]
    );

    let (histcnt_decoded, histcnt_decode_size) = handler
        .decode_instruction(&[0x20, 0xc0, 0xa2, 0x45], "aarch64", 0)
        .expect("generated SVE2 HISTCNT detail should decode");
    assert_eq!(histcnt_decode_size, 4);
    assert_eq!(
        histcnt_decoded.registers_read,
        vec![
            RegisterId::aarch64(128),
            RegisterId::aarch64(161),
            RegisterId::aarch64(162)
        ]
    );
    assert_eq!(
        histcnt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (histseg, histseg_size) = handler
        .disassemble(&[0x20, 0xa0, 0x3f, 0x45], "aarch64", 0)
        .expect("generated SVE2 HISTSEG should decode");
    assert_eq!(histseg_size, 4);
    assert_eq!(histseg.mnemonic, "histseg");
    assert_eq!(histseg.operands, "z0.b, z1.b, z31.b");

    let (histseg_decoded, histseg_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x3f, 0x45], "aarch64", 0)
        .expect("generated SVE2 HISTSEG detail should decode");
    assert_eq!(histseg_decode_size, 4);
    assert_eq!(
        histseg_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(191)]
    );
    assert_eq!(
        histseg_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (rax1, rax1_size) = handler
        .disassemble(&[0x20, 0xf4, 0x3f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SHA3 RAX1 should decode");
    assert_eq!(rax1_size, 4);
    assert_eq!(rax1.mnemonic, "rax1");
    assert_eq!(rax1.operands, "z0.d, z1.d, z31.d");

    let (rax1_decoded, rax1_decode_size) = handler
        .decode_instruction(&[0x20, 0xf4, 0x3f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SHA3 RAX1 detail should decode");
    assert_eq!(rax1_decode_size, 4);
    assert_eq!(
        rax1_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(191)]
    );
    assert_eq!(
        rax1_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqrshrunb, sqrshrunb_size) = handler
        .disassemble(&[0x00, 0x08, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQRSHRUNB narrowing shift should decode");
    assert_eq!(sqrshrunb_size, 4);
    assert_eq!(sqrshrunb.mnemonic, "sqrshrunb");
    assert_eq!(sqrshrunb.operands, "z0.b, z0.h, #1");

    let (sqrshrunb_decoded, sqrshrunb_decode_size) = handler
        .decode_instruction(&[0x00, 0x08, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQRSHRUNB narrowing shift detail should decode");
    assert_eq!(sqrshrunb_decode_size, 4);
    assert_eq!(
        sqrshrunb_decoded.registers_read,
        vec![RegisterId::aarch64(160)]
    );
    assert_eq!(
        sqrshrunb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqxtnb, sqxtnb_size) = handler
        .disassemble(&[0xe0, 0x43, 0x28, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQXTNB extract narrow should decode");
    assert_eq!(sqxtnb_size, 4);
    assert_eq!(sqxtnb.mnemonic, "sqxtnb");
    assert_eq!(sqxtnb.operands, "z0.b, z31.h");

    let (sqxtnb_decoded, sqxtnb_decode_size) = handler
        .decode_instruction(&[0xe0, 0x43, 0x28, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQXTNB extract narrow detail should decode");
    assert_eq!(sqxtnb_decode_size, 4);
    assert_eq!(
        sqxtnb_decoded.registers_read,
        vec![RegisterId::aarch64(191)]
    );
    assert_eq!(
        sqxtnb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqxtunt, sqxtunt_size) = handler
        .disassemble(&[0xe0, 0x57, 0x28, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQXTUNT extract narrow should decode");
    assert_eq!(sqxtunt_size, 4);
    assert_eq!(sqxtunt.mnemonic, "sqxtunt");
    assert_eq!(sqxtunt.operands, "z0.b, z31.h");

    let (sqxtunt_decoded, sqxtunt_decode_size) = handler
        .decode_instruction(&[0xe0, 0x57, 0x28, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQXTUNT extract narrow detail should decode");
    assert_eq!(sqxtunt_decode_size, 4);
    assert_eq!(
        sqxtunt_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(191)]
    );
    assert_eq!(
        sqxtunt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (rshrnb, rshrnb_size) = handler
        .disassemble(&[0x00, 0x18, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 RSHRNB narrowing shift should decode");
    assert_eq!(rshrnb_size, 4);
    assert_eq!(rshrnb.mnemonic, "rshrnb");
    assert_eq!(rshrnb.operands, "z0.b, z0.h, #1");

    let (rshrnb_decoded, rshrnb_decode_size) = handler
        .decode_instruction(&[0x00, 0x18, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 RSHRNB narrowing shift detail should decode");
    assert_eq!(rshrnb_decode_size, 4);
    assert_eq!(
        rshrnb_decoded.registers_read,
        vec![RegisterId::aarch64(160)]
    );
    assert_eq!(
        rshrnb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (rshrnt, rshrnt_size) = handler
        .disassemble(&[0x00, 0x1c, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 RSHRNT narrowing shift should decode");
    assert_eq!(rshrnt_size, 4);
    assert_eq!(rshrnt.mnemonic, "rshrnt");
    assert_eq!(rshrnt.operands, "z0.b, z0.h, #1");

    let (rshrnt_decoded, rshrnt_decode_size) = handler
        .decode_instruction(&[0x00, 0x1c, 0x2f, 0x45], "aarch64", 0)
        .expect("generated SVE2 RSHRNT narrowing shift detail should decode");
    assert_eq!(rshrnt_decode_size, 4);
    assert_eq!(
        rshrnt_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(160)]
    );
    assert_eq!(
        rshrnt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (saba, saba_size) = handler
        .disassemble(&[0x20, 0xf8, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABA should decode");
    assert_eq!(saba_size, 4);
    assert_eq!(saba.mnemonic, "saba");
    assert_eq!(saba.operands, "z0.b, z1.b, z31.b");

    let (saba_decoded, saba_decode_size) = handler
        .decode_instruction(&[0x20, 0xf8, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABA detail should decode");
    assert_eq!(saba_decode_size, 4);
    assert_eq!(
        saba_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        saba_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqrdmlah, sqrdmlah_size) = handler
        .disassemble(&[0x20, 0x70, 0x1f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQRDMLAH should decode");
    assert_eq!(sqrdmlah_size, 4);
    assert_eq!(sqrdmlah.mnemonic, "sqrdmlah");
    assert_eq!(sqrdmlah.operands, "z0.b, z1.b, z31.b");

    let (sqrdmlah_decoded, sqrdmlah_decode_size) = handler
        .decode_instruction(&[0x20, 0x70, 0x1f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQRDMLAH detail should decode");
    assert_eq!(sqrdmlah_decode_size, 4);
    assert_eq!(
        sqrdmlah_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        sqrdmlah_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sabalb, sabalb_size) = handler
        .disassemble(&[0x20, 0xc0, 0x5f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABALB long absolute difference should decode");
    assert_eq!(sabalb_size, 4);
    assert_eq!(sabalb.mnemonic, "sabalb");
    assert_eq!(sabalb.operands, "z0.h, z1.b, z31.b");

    let (sqdmlalb, sqdmlalb_size) = handler
        .disassemble(&[0x20, 0x60, 0x5f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMLALB should decode");
    assert_eq!(sqdmlalb_size, 4);
    assert_eq!(sqdmlalb.mnemonic, "sqdmlalb");
    assert_eq!(sqdmlalb.operands, "z0.h, z1.b, z31.b");

    let (sqdmlalb_decoded, sqdmlalb_decode_size) = handler
        .decode_instruction(&[0x20, 0x60, 0x5f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMLALB detail should decode");
    assert_eq!(sqdmlalb_decode_size, 4);
    assert_eq!(
        sqdmlalb_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        sqdmlalb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqdmlalbt, sqdmlalbt_size) = handler
        .disassemble(&[0x20, 0x08, 0x5f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMLALBT should decode");
    assert_eq!(sqdmlalbt_size, 4);
    assert_eq!(sqdmlalbt.mnemonic, "sqdmlalbt");
    assert_eq!(sqdmlalbt.operands, "z0.h, z1.b, z31.b");

    let (sqdmlalbt_decoded, sqdmlalbt_decode_size) = handler
        .decode_instruction(&[0x20, 0x08, 0x5f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQDMLALBT detail should decode");
    assert_eq!(sqdmlalbt_decode_size, 4);
    assert_eq!(
        sqdmlalbt_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        sqdmlalbt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sabalb_decoded, sabalb_decode_size) = handler
        .decode_instruction(&[0x20, 0xc0, 0x5f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABALB long absolute difference detail should decode");
    assert_eq!(sabalb_decode_size, 4);
    assert_eq!(
        sabalb_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        sabalb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sabdlb, sabdlb_size) = handler
        .disassemble(&[0x20, 0x30, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABDLB long absolute difference should decode");
    assert_eq!(sabdlb_size, 4);
    assert_eq!(sabdlb.mnemonic, "sabdlb");
    assert_eq!(sabdlb.operands, "z0.h, z1.b, z2.b");

    let (sabdlb_decoded, sabdlb_decode_size) = handler
        .decode_instruction(&[0x20, 0x30, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SABDLB long absolute difference detail should decode");
    assert_eq!(sabdlb_decode_size, 4);
    assert_eq!(
        sabdlb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        sabdlb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (saddlb, saddlb_size) = handler
        .disassemble(&[0x20, 0x00, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SADDLB long add should decode");
    assert_eq!(saddlb_size, 4);
    assert_eq!(saddlb.mnemonic, "saddlb");
    assert_eq!(saddlb.operands, "z0.h, z1.b, z2.b");

    let (saddlb_decoded, saddlb_decode_size) = handler
        .decode_instruction(&[0x20, 0x00, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SADDLB long add detail should decode");
    assert_eq!(saddlb_decode_size, 4);
    assert_eq!(
        saddlb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        saddlb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (saddwb, saddwb_size) = handler
        .disassemble(&[0x20, 0x40, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SADDWB wide add should decode");
    assert_eq!(saddwb_size, 4);
    assert_eq!(saddwb.mnemonic, "saddwb");
    assert_eq!(saddwb.operands, "z0.h, z1.h, z2.b");

    let (saddwb_decoded, saddwb_decode_size) = handler
        .decode_instruction(&[0x20, 0x40, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SADDWB wide add detail should decode");
    assert_eq!(saddwb_decode_size, 4);
    assert_eq!(
        saddwb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        saddwb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sadalp, sadalp_size) = handler
        .disassemble(&[0x20, 0xa0, 0x44, 0x44], "aarch64", 0)
        .expect("generated SVE2 SADALP should decode");
    assert_eq!(sadalp_size, 4);
    assert_eq!(sadalp.mnemonic, "sadalp");
    assert_eq!(sadalp.operands, "z0.h, p0/m, z1.b");

    let (sadalp_decoded, sadalp_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x44, 0x44], "aarch64", 0)
        .expect("generated SVE2 SADALP detail should decode");
    assert_eq!(sadalp_decode_size, 4);
    assert_eq!(
        sadalp_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161)
        ]
    );
    assert_eq!(
        sadalp_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (smullb, smullb_size) = handler
        .disassemble(&[0x20, 0x70, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SMULLB long multiply should decode");
    assert_eq!(smullb_size, 4);
    assert_eq!(smullb.mnemonic, "smullb");
    assert_eq!(smullb.operands, "z0.h, z1.b, z2.b");

    let (smullb_decoded, smullb_decode_size) = handler
        .decode_instruction(&[0x20, 0x70, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SMULLB long multiply detail should decode");
    assert_eq!(smullb_decode_size, 4);
    assert_eq!(
        smullb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        smullb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sqdmullb, sqdmullb_size) = handler
        .disassemble(&[0x20, 0x60, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQDMULLB long multiply should decode");
    assert_eq!(sqdmullb_size, 4);
    assert_eq!(sqdmullb.mnemonic, "sqdmullb");
    assert_eq!(sqdmullb.operands, "z0.h, z1.b, z2.b");

    let (sqdmullb_decoded, sqdmullb_decode_size) = handler
        .decode_instruction(&[0x20, 0x60, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQDMULLB long multiply detail should decode");
    assert_eq!(sqdmullb_decode_size, 4);
    assert_eq!(
        sqdmullb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        sqdmullb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (splice, splice_size) = handler
        .disassemble(&[0xdd, 0x9f, 0x2d, 0x05], "aarch64", 0)
        .expect("generated SVE SPLICE should decode");
    assert_eq!(splice_size, 4);
    assert_eq!(splice.mnemonic, "splice");
    assert_eq!(splice.operands, "z29.b, p7, { z30.b, z31.b }");

    let (splice_decoded, splice_decode_size) = handler
        .decode_instruction(&[0xdd, 0x9f, 0x2d, 0x05], "aarch64", 0)
        .expect("generated SVE SPLICE detail should decode");
    assert_eq!(splice_decode_size, 4);
    assert_eq!(
        splice_decoded.registers_read,
        vec![
            RegisterId::aarch64(135),
            RegisterId::aarch64(190),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        splice_decoded.registers_written,
        vec![RegisterId::aarch64(189)]
    );

    let (sli, sli_size) = handler
        .disassemble(&[0x00, 0xf4, 0x08, 0x45], "aarch64", 0)
        .expect("generated SVE2 SLI should decode");
    assert_eq!(sli_size, 4);
    assert_eq!(sli.mnemonic, "sli");
    assert_eq!(sli.operands, "z0.b, z0.b, #0");

    let (sli_decoded, sli_decode_size) = handler
        .decode_instruction(&[0x00, 0xf4, 0x08, 0x45], "aarch64", 0)
        .expect("generated SVE2 SLI detail should decode");
    assert_eq!(sli_decode_size, 4);
    assert_eq!(
        sli_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(160)]
    );
    assert_eq!(
        sli_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (ldnt1b, ldnt1b_size) = handler
        .disassemble(&[0x20, 0xa0, 0x1f, 0x84], "aarch64", 0)
        .expect("generated SVE LDNT1B vector address should decode");
    assert_eq!(ldnt1b_size, 4);
    assert_eq!(ldnt1b.mnemonic, "ldnt1b");
    assert_eq!(ldnt1b.operands, "{ z0.s }, p0/z, [z1.s]");

    let (ldnt1b_decoded, ldnt1b_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x1f, 0x84], "aarch64", 0)
        .expect("generated SVE LDNT1B vector address detail should decode");
    assert_eq!(ldnt1b_decode_size, 4);
    assert_eq!(
        ldnt1b_decoded.registers_read,
        vec![RegisterId::aarch64(128), RegisterId::aarch64(161)]
    );
    assert_eq!(
        ldnt1b_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (stnt1b, stnt1b_size) = handler
        .disassemble(&[0x20, 0x20, 0x5f, 0xe4], "aarch64", 0)
        .expect("generated SVE2 STNT1B vector address should decode");
    assert_eq!(stnt1b_size, 4);
    assert_eq!(stnt1b.mnemonic, "stnt1b");
    assert_eq!(stnt1b.operands, "{ z0.s }, p0, [z1.s]");

    let (stnt1b_decoded, stnt1b_decode_size) = handler
        .decode_instruction(&[0x20, 0x20, 0x5f, 0xe4], "aarch64", 0)
        .expect("generated SVE2 STNT1B vector address detail should decode");
    assert_eq!(stnt1b_decode_size, 4);
    assert_eq!(
        stnt1b_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161),
            RegisterId::aarch64(31)
        ]
    );
    assert_eq!(stnt1b_decoded.registers_written, Vec::<RegisterId>::new());

    let (ldnt1b_indexed, ldnt1b_indexed_size) = handler
        .disassemble(&[0xff, 0xbf, 0x00, 0x84], "aarch64", 0)
        .expect("generated SVE LDNT1B vector address with scalar index should decode");
    assert_eq!(ldnt1b_indexed_size, 4);
    assert_eq!(ldnt1b_indexed.mnemonic, "ldnt1b");
    assert_eq!(ldnt1b_indexed.operands, "{ z31.s }, p7/z, [z31.s, x0]");

    let (ldnt1b_indexed_decoded, ldnt1b_indexed_decode_size) = handler
        .decode_instruction(&[0xff, 0xbf, 0x00, 0x84], "aarch64", 0)
        .expect("generated SVE LDNT1B vector address with scalar index detail should decode");
    assert_eq!(ldnt1b_indexed_decode_size, 4);
    assert_eq!(
        ldnt1b_indexed_decoded.registers_read,
        vec![
            RegisterId::aarch64(135),
            RegisterId::aarch64(191),
            RegisterId::aarch64(0)
        ]
    );
    assert_eq!(
        ldnt1b_indexed_decoded.registers_written,
        vec![RegisterId::aarch64(191)]
    );

    let (tbl2, tbl2_size) = handler
        .disassemble(&[0xbc, 0x2b, 0x3f, 0x05], "aarch64", 0)
        .expect("generated SVE2 TBL two-register table should decode");
    assert_eq!(tbl2_size, 4);
    assert_eq!(tbl2.mnemonic, "tbl");
    assert_eq!(tbl2.operands, "z28.b, { z29.b, z30.b }, z31.b");

    let (tbl2_decoded, tbl2_decode_size) = handler
        .decode_instruction(&[0xbc, 0x2b, 0x3f, 0x05], "aarch64", 0)
        .expect("generated SVE2 TBL two-register table detail should decode");
    assert_eq!(tbl2_decode_size, 4);
    assert_eq!(
        tbl2_decoded.registers_read,
        vec![
            RegisterId::aarch64(189),
            RegisterId::aarch64(190),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        tbl2_decoded.registers_written,
        vec![RegisterId::aarch64(188)]
    );

    let (uadalp, uadalp_size) = handler
        .disassemble(&[0x20, 0xa0, 0x45, 0x44], "aarch64", 0)
        .expect("generated SVE2 UADALP predicated pairwise long accumulate should decode");
    assert_eq!(uadalp_size, 4);
    assert_eq!(uadalp.mnemonic, "uadalp");
    assert_eq!(uadalp.operands, "z0.h, p0/m, z1.b");

    let (uadalp_decoded, uadalp_decode_size) = handler
        .decode_instruction(&[0x20, 0xa0, 0x45, 0x44], "aarch64", 0)
        .expect("generated SVE2 UADALP predicated pairwise long accumulate detail should decode");
    assert_eq!(uadalp_decode_size, 4);
    assert_eq!(
        uadalp_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(128),
            RegisterId::aarch64(161)
        ]
    );
    assert_eq!(
        uadalp_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (usublb, usublb_size) = handler
        .disassemble(&[0x20, 0x18, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 USUBLB long subtract should decode");
    assert_eq!(usublb_size, 4);
    assert_eq!(usublb.mnemonic, "usublb");
    assert_eq!(usublb.operands, "z0.h, z1.b, z2.b");

    let (usublb_decoded, usublb_decode_size) = handler
        .decode_instruction(&[0x20, 0x18, 0x42, 0x45], "aarch64", 0)
        .expect("generated SVE2 USUBLB long subtract detail should decode");
    assert_eq!(usublb_decode_size, 4);
    assert_eq!(
        usublb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(162)]
    );
    assert_eq!(
        usublb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (xar_b, xar_b_size) = handler
        .disassemble(&[0x20, 0x34, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE2 XAR byte rotate should decode");
    assert_eq!(xar_b_size, 4);
    assert_eq!(xar_b.mnemonic, "xar");
    assert_eq!(xar_b.operands, "z0.b, z0.b, z1.b, #1");

    let (xar_b_decoded, xar_b_decode_size) = handler
        .decode_instruction(&[0x20, 0x34, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE2 XAR byte rotate detail should decode");
    assert_eq!(xar_b_decode_size, 4);
    assert_eq!(
        xar_b_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(161)]
    );
    assert_eq!(
        xar_b_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (xar_d, xar_d_size) = handler
        .disassemble(&[0xdf, 0x37, 0xa0, 0x04], "aarch64", 0)
        .expect("generated SVE2 XAR doubleword rotate should decode");
    assert_eq!(xar_d_size, 4);
    assert_eq!(xar_d.mnemonic, "xar");
    assert_eq!(xar_d.operands, "z31.d, z31.d, z30.d, #0x40");

    let (xar_d_decoded, xar_d_decode_size) = handler
        .decode_instruction(&[0xdf, 0x37, 0xa0, 0x04], "aarch64", 0)
        .expect("generated SVE2 XAR doubleword rotate detail should decode");
    assert_eq!(xar_d_decode_size, 4);
    assert_eq!(
        xar_d_decoded.registers_read,
        vec![RegisterId::aarch64(191), RegisterId::aarch64(190)]
    );
    assert_eq!(
        xar_d_decoded.registers_written,
        vec![RegisterId::aarch64(191)]
    );

    let (fmov_pred, fmov_pred_size) = handler
        .disassemble(&[0x00, 0xd8, 0x50, 0x05], "aarch64", 0)
        .expect("generated SVE FCPY predicated FP immediate should decode");
    assert_eq!(fmov_pred_size, 4);
    assert_eq!(fmov_pred.mnemonic, "fmov");
    assert_eq!(fmov_pred.operands, "z0.h, p0/m, #-0.12500000");

    let (fmov_pred_decoded, fmov_pred_decode_size) = handler
        .decode_instruction(&[0x00, 0xd8, 0x50, 0x05], "aarch64", 0)
        .expect("generated SVE FCPY predicated FP immediate detail should decode");
    assert_eq!(fmov_pred_decode_size, 4);
    assert_eq!(
        fmov_pred_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(128)]
    );
    assert_eq!(
        fmov_pred_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (fmov_unpred, fmov_unpred_size) = handler
        .disassemble(&[0x00, 0xd8, 0x79, 0x25], "aarch64", 0)
        .expect("generated SVE FDUP FP immediate should decode");
    assert_eq!(fmov_unpred_size, 4);
    assert_eq!(fmov_unpred.mnemonic, "fmov");
    assert_eq!(fmov_unpred.operands, "z0.h, #-0.12500000");

    let (fmov_unpred_decoded, fmov_unpred_decode_size) = handler
        .decode_instruction(&[0x00, 0xd8, 0x79, 0x25], "aarch64", 0)
        .expect("generated SVE FDUP FP immediate detail should decode");
    assert_eq!(fmov_unpred_decode_size, 4);
    assert_eq!(fmov_unpred_decoded.registers_read, Vec::<RegisterId>::new());
    assert_eq!(
        fmov_unpred_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (addhnb, addhnb_size) = handler
        .disassemble(&[0x20, 0x60, 0x7f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADDHNB byte should decode");
    assert_eq!(addhnb_size, 4);
    assert_eq!(addhnb.mnemonic, "addhnb");
    assert_eq!(addhnb.operands, "z0.b, z1.h, z31.h");

    let (addhnb_decoded, addhnb_decode_size) = handler
        .decode_instruction(&[0x20, 0x60, 0x7f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADDHNB byte detail should decode");
    assert_eq!(addhnb_decode_size, 4);
    assert_eq!(
        addhnb_decoded.registers_read,
        vec![RegisterId::aarch64(161), RegisterId::aarch64(191)]
    );
    assert_eq!(
        addhnb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (addhnt, addhnt_size) = handler
        .disassemble(&[0x20, 0x64, 0x7f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADDHNT byte should decode");
    assert_eq!(addhnt_size, 4);
    assert_eq!(addhnt.mnemonic, "addhnt");
    assert_eq!(addhnt.operands, "z0.b, z1.h, z31.h");

    let (addhnt_decoded, addhnt_decode_size) = handler
        .decode_instruction(&[0x20, 0x64, 0x7f, 0x45], "aarch64", 0)
        .expect("generated SVE2 ADDHNT byte detail should decode");
    assert_eq!(addhnt_decode_size, 4);
    assert_eq!(
        addhnt_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        addhnt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );
    let (aesd, aesd_size) = handler
        .disassemble(&[0xe0, 0xe7, 0x22, 0x45], "aarch64", 0)
        .expect("generated SVE2 AESD should decode");
    assert_eq!(aesd_size, 4);
    assert_eq!(aesd.mnemonic, "aesd");
    assert_eq!(aesd.operands, "z0.b, z0.b, z31.b");

    let (aesmc, aesmc_size) = handler
        .disassemble(&[0x00, 0xe0, 0x20, 0x45], "aarch64", 0)
        .expect("generated SVE2 AESMC should decode");
    assert_eq!(aesmc_size, 4);
    assert_eq!(aesmc.mnemonic, "aesmc");
    assert_eq!(aesmc.operands, "z0.b, z0.b");

    let (eorbt, eorbt_size) = handler
        .disassemble(&[0x20, 0x90, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 EORBT should decode");
    assert_eq!(eorbt_size, 4);
    assert_eq!(eorbt.mnemonic, "eorbt");
    assert_eq!(eorbt.operands, "z0.b, z1.b, z31.b");

    let (eorbt_decoded, eorbt_decode_size) = handler
        .decode_instruction(&[0x20, 0x90, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 EORBT detail should decode");
    assert_eq!(eorbt_decode_size, 4);
    assert_eq!(
        eorbt_decoded.registers_read,
        vec![
            RegisterId::aarch64(160),
            RegisterId::aarch64(161),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        eorbt_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (eortb, eortb_size) = handler
        .disassemble(&[0x20, 0x94, 0x1f, 0x45], "aarch64", 0)
        .expect("generated SVE2 EORTB should decode");
    assert_eq!(eortb_size, 4);
    assert_eq!(eortb.mnemonic, "eortb");
    assert_eq!(eortb.operands, "z0.b, z1.b, z31.b");

    let (cadd, cadd_size) = handler
        .disassemble(&[0xe4, 0xdf, 0xc0, 0x45], "aarch64", 0)
        .expect("generated SVE2 CADD should decode");
    assert_eq!(cadd_size, 4);
    assert_eq!(cadd.mnemonic, "cadd");
    assert_eq!(cadd.operands, "z4.d, z4.d, z31.d, #270");

    let (sqcadd, sqcadd_size) = handler
        .disassemble(&[0xe4, 0xdf, 0xc1, 0x45], "aarch64", 0)
        .expect("generated SVE2 SQCADD should decode");
    assert_eq!(sqcadd_size, 4);
    assert_eq!(sqcadd.mnemonic, "sqcadd");
    assert_eq!(sqcadd.operands, "z4.d, z4.d, z31.d, #270");

    let (cdot_s, cdot_s_size) = handler
        .disassemble(&[0x20, 0x10, 0x9f, 0x44], "aarch64", 0)
        .expect("generated SVE2 CDOT should decode");
    assert_eq!(cdot_s_size, 4);
    assert_eq!(cdot_s.mnemonic, "cdot");
    assert_eq!(cdot_s.operands, "z0.s, z1.b, z31.b, #0");

    let (cdot_d, cdot_d_size) = handler
        .disassemble(&[0x20, 0x1c, 0xdf, 0x44], "aarch64", 0)
        .expect("generated SVE2 CDOT doubleword should decode");
    assert_eq!(cdot_d_size, 4);
    assert_eq!(cdot_d.mnemonic, "cdot");
    assert_eq!(cdot_d.operands, "z0.d, z1.h, z31.h, #270");

    let (cdot_indexed_s, cdot_indexed_s_size) = handler
        .disassemble(&[0x20, 0x40, 0xbf, 0x44], "aarch64", 0)
        .expect("generated SVE2 CDOT indexed word should decode");
    assert_eq!(cdot_indexed_s_size, 4);
    assert_eq!(cdot_indexed_s.mnemonic, "cdot");
    assert_eq!(cdot_indexed_s.operands, "z0.s, z1.b, z7.b[3], #0");

    let (cdot_indexed_d, cdot_indexed_d_size) = handler
        .disassemble(&[0x20, 0x40, 0xff, 0x44], "aarch64", 0)
        .expect("generated SVE2 CDOT indexed doubleword should decode");
    assert_eq!(cdot_indexed_d_size, 4);
    assert_eq!(cdot_indexed_d.mnemonic, "cdot");
    assert_eq!(cdot_indexed_d.operands, "z0.d, z1.h, z15.h[1], #0");

    let (cmla, cmla_size) = handler
        .disassemble(&[0xdd, 0x27, 0x9f, 0x44], "aarch64", 0)
        .expect("generated SVE2 CMLA should decode");
    assert_eq!(cmla_size, 4);
    assert_eq!(cmla.mnemonic, "cmla");
    assert_eq!(cmla.operands, "z29.s, z30.s, z31.s, #90");

    let (sqrdcmlah, sqrdcmlah_size) = handler
        .disassemble(&[0xdd, 0x37, 0x9f, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQRDCMLAH should decode");
    assert_eq!(sqrdcmlah_size, 4);
    assert_eq!(sqrdcmlah.mnemonic, "sqrdcmlah");
    assert_eq!(sqrdcmlah.operands, "z29.s, z30.s, z31.s, #90");

    let (cmla_indexed, cmla_indexed_size) = handler
        .disassemble(&[0x55, 0x65, 0xf5, 0x44], "aarch64", 0)
        .expect("generated SVE2 CMLA indexed word should decode");
    assert_eq!(cmla_indexed_size, 4);
    assert_eq!(cmla_indexed.mnemonic, "cmla");
    assert_eq!(cmla_indexed.operands, "z21.s, z10.s, z5.s[1], #90");

    let (sqrdcmlah_indexed, sqrdcmlah_indexed_size) = handler
        .disassemble(&[0x55, 0x75, 0xf5, 0x44], "aarch64", 0)
        .expect("generated SVE2 SQRDCMLAH indexed word should decode");
    assert_eq!(sqrdcmlah_indexed_size, 4);
    assert_eq!(sqrdcmlah_indexed.mnemonic, "sqrdcmlah");
    assert_eq!(sqrdcmlah_indexed.operands, "z21.s, z10.s, z5.s[1], #90");

    let (fcmla_pred, fcmla_pred_size) = handler
        .disassemble(&[0x00, 0x00, 0x40, 0x64], "aarch64", 0)
        .expect("generated SVE FCMLA predicated halfword should decode");
    assert_eq!(fcmla_pred_size, 4);
    assert_eq!(fcmla_pred.mnemonic, "fcmla");
    assert_eq!(fcmla_pred.operands, "z0.h, p0/m, z0.h, z0.h, #0");

    let (fcmla_indexed, fcmla_indexed_size) = handler
        .disassemble(&[0x55, 0x15, 0xf5, 0x64], "aarch64", 0)
        .expect("generated SVE FCMLA indexed word should decode");
    assert_eq!(fcmla_indexed_size, 4);
    assert_eq!(fcmla_indexed.mnemonic, "fcmla");
    assert_eq!(fcmla_indexed.operands, "z21.s, z10.s, z5.s[1], #90");

    let (sm4e, sm4e_size) = handler
        .disassemble(&[0xe0, 0xe3, 0x23, 0x45], "aarch64", 0)
        .expect("generated SVE2 SM4E should decode");
    assert_eq!(sm4e_size, 4);
    assert_eq!(sm4e.mnemonic, "sm4e");
    assert_eq!(sm4e.operands, "z0.s, z0.s, z31.s");

    let (sm4e_decoded, sm4e_decode_size) = handler
        .decode_instruction(&[0xe0, 0xe3, 0x23, 0x45], "aarch64", 0)
        .expect("generated SVE2 SM4E detail should decode");
    assert_eq!(sm4e_decode_size, 4);
    assert_eq!(
        sm4e_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(191)]
    );
    assert_eq!(
        sm4e_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (bcax, bcax_size) = handler
        .disassemble(&[0xfd, 0x3b, 0x7e, 0x04], "aarch64", 0)
        .expect("generated SVE2 BCAX should decode");
    assert_eq!(bcax_size, 4);
    assert_eq!(bcax.mnemonic, "bcax");
    assert_eq!(bcax.operands, "z29.d, z29.d, z30.d, z31.d");

    let (bcax_decoded, bcax_decode_size) = handler
        .decode_instruction(&[0xfd, 0x3b, 0x7e, 0x04], "aarch64", 0)
        .expect("generated SVE2 BCAX detail should decode");
    assert_eq!(bcax_decode_size, 4);
    assert_eq!(
        bcax_decoded.registers_read,
        vec![
            RegisterId::aarch64(189),
            RegisterId::aarch64(190),
            RegisterId::aarch64(191)
        ]
    );
    assert_eq!(
        bcax_decoded.registers_written,
        vec![RegisterId::aarch64(189)]
    );
}

#[test]
fn generated_m5_sve_exact_fp_immediate_text_preserves_predicate_and_immediate() {
    let handler = ArmHandler::new();

    let (half, half_size) = handler
        .disassemble(&[0x00, 0x80, 0xd8, 0x65], "aarch64", 0)
        .expect("generated SVE FADD half immediate should decode");
    assert_eq!(half_size, 4);
    assert_eq!(half.mnemonic, "fadd");
    assert_eq!(half.operands, "z0.d, p0/m, z0.d, #0.5");

    let (one, one_size) = handler
        .disassemble(&[0x20, 0x80, 0xd8, 0x65], "aarch64", 0)
        .expect("generated SVE FADD one immediate should decode");
    assert_eq!(one_size, 4);
    assert_eq!(one.mnemonic, "fadd");
    assert_eq!(one.operands, "z0.d, p0/m, z0.d, #1.0");

    let (zero, zero_size) = handler
        .disassemble(&[0x00, 0x80, 0xde, 0x65], "aarch64", 0)
        .expect("generated SVE FMAX zero immediate should decode");
    assert_eq!(zero_size, 4);
    assert_eq!(zero.mnemonic, "fmax");
    assert_eq!(zero.operands, "z0.d, p0/m, z0.d, #0.0");
}

#[test]
fn generated_m5_sve_pattern_count_text_preserves_pattern_alias_and_multiplier() {
    let handler = ArmHandler::new();

    let (plain, plain_size) = handler
        .disassemble(&[0xe0, 0xe3, 0x20, 0x04], "aarch64", 0)
        .expect("generated SVE CNTB should decode");
    assert_eq!(plain_size, 4);
    assert_eq!(plain.mnemonic, "cntb");
    assert_eq!(plain.operands, "x0");

    let (multiplied, multiplied_size) = handler
        .disassemble(&[0xe0, 0xe3, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE CNTB with multiplier should decode");
    assert_eq!(multiplied_size, 4);
    assert_eq!(multiplied.mnemonic, "cntb");
    assert_eq!(multiplied.operands, "x0, all, mul #16");

    let (pattern, pattern_size) = handler
        .disassemble(&[0x00, 0xe0, 0x20, 0x04], "aarch64", 0)
        .expect("generated SVE CNTB with pattern should decode");
    assert_eq!(pattern_size, 4);
    assert_eq!(pattern.mnemonic, "cntb");
    assert_eq!(pattern.operands, "x0, pow2");

    let (numeric_pattern, numeric_pattern_size) = handler
        .disassemble(&[0x80, 0xe3, 0x20, 0x04], "aarch64", 0)
        .expect("generated SVE CNTB with numeric pattern should decode");
    assert_eq!(numeric_pattern_size, 4);
    assert_eq!(numeric_pattern.mnemonic, "cntb");
    assert_eq!(numeric_pattern.operands, "x0, #0x1c");

    let (inc, inc_size) = handler
        .disassemble(&[0xe0, 0xe3, 0x30, 0x04], "aarch64", 0)
        .expect("generated SVE INCB should decode");
    assert_eq!(inc_size, 4);
    assert_eq!(inc.mnemonic, "incb");
    assert_eq!(inc.operands, "x0");

    let (inc_decoded, inc_decode_size) = handler
        .decode_instruction(&[0xe0, 0xe3, 0x30, 0x04], "aarch64", 0)
        .expect("generated SVE INCB detail should decode");
    assert_eq!(inc_decode_size, 4);
    assert_eq!(inc_decoded.registers_read, vec![RegisterId::aarch64(0)]);
    assert_eq!(inc_decoded.registers_written, vec![RegisterId::aarch64(0)]);

    let (dec, dec_size) = handler
        .disassemble(&[0xe0, 0xe7, 0x30, 0x04], "aarch64", 0)
        .expect("generated SVE DECB should decode");
    assert_eq!(dec_size, 4);
    assert_eq!(dec.mnemonic, "decb");
    assert_eq!(dec.operands, "x0");

    let (decd, decd_size) = handler
        .disassemble(&[0xe0, 0xe7, 0xf0, 0x04], "aarch64", 0)
        .expect("generated SVE DECD should decode");
    assert_eq!(decd_size, 4);
    assert_eq!(decd.mnemonic, "decd");
    assert_eq!(decd.operands, "x0");

    let (incd_z, incd_z_size) = handler
        .disassemble(&[0xe0, 0xc3, 0xf0, 0x04], "aarch64", 0)
        .expect("generated SVE INCD vector should decode");
    assert_eq!(incd_z_size, 4);
    assert_eq!(incd_z.mnemonic, "incd");
    assert_eq!(incd_z.operands, "z0.d");

    let (inch_z, inch_z_size) = handler
        .disassemble(&[0xe0, 0xc3, 0x70, 0x04], "aarch64", 0)
        .expect("generated SVE INCH vector should decode");
    assert_eq!(inch_z_size, 4);
    assert_eq!(inch_z.mnemonic, "inch");
    assert_eq!(inch_z.operands, "z0.h");

    let (sqdecb_pair, sqdecb_pair_size) = handler
        .disassemble(&[0xe0, 0xfb, 0x20, 0x04], "aarch64", 0)
        .expect("generated SVE SQDECB pair should decode");
    assert_eq!(sqdecb_pair_size, 4);
    assert_eq!(sqdecb_pair.mnemonic, "sqdecb");
    assert_eq!(sqdecb_pair.operands, "x0, w0");

    let (sqdecb_pair_multiplied, sqdecb_pair_multiplied_size) = handler
        .disassemble(&[0xe0, 0xfb, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE SQDECB pair with multiplier should decode");
    assert_eq!(sqdecb_pair_multiplied_size, 4);
    assert_eq!(sqdecb_pair_multiplied.mnemonic, "sqdecb");
    assert_eq!(sqdecb_pair_multiplied.operands, "x0, w0, all, mul #16");

    let (uqdecb_w, uqdecb_w_size) = handler
        .disassemble(&[0xe0, 0xff, 0x20, 0x04], "aarch64", 0)
        .expect("generated SVE UQDECB W should decode");
    assert_eq!(uqdecb_w_size, 4);
    assert_eq!(uqdecb_w.mnemonic, "uqdecb");
    assert_eq!(uqdecb_w.operands, "w0");

    let (uqdecb_x, uqdecb_x_size) = handler
        .disassemble(&[0xe0, 0xff, 0x30, 0x04], "aarch64", 0)
        .expect("generated SVE UQDECB X should decode");
    assert_eq!(uqdecb_x_size, 4);
    assert_eq!(uqdecb_x.mnemonic, "uqdecb");
    assert_eq!(uqdecb_x.operands, "x0");

    let (dec_decoded, dec_decode_size) = handler
        .decode_instruction(&[0xe0, 0xe7, 0x30, 0x04], "aarch64", 0)
        .expect("generated SVE DECB detail should decode");
    assert_eq!(dec_decode_size, 4);
    assert_eq!(dec_decoded.registers_read, vec![RegisterId::aarch64(0)]);
    assert_eq!(dec_decoded.registers_written, vec![RegisterId::aarch64(0)]);
}
#[test]
fn generated_m5_sve_shift_immediate_text_preserves_arrangement_and_shift_amount() {
    let handler = ArmHandler::new();

    let (asr, asr_size) = handler
        .disassemble(&[0x00, 0x90, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE ASR immediate should decode");
    assert_eq!(asr_size, 4);
    assert_eq!(asr.mnemonic, "asr");
    assert_eq!(asr.operands, "z0.b, z0.b, #1");

    let (lsl, lsl_size) = handler
        .disassemble(&[0xff, 0x9f, 0x2f, 0x04], "aarch64", 0)
        .expect("generated SVE LSL immediate should decode");
    assert_eq!(lsl_size, 4);
    assert_eq!(lsl.mnemonic, "lsl");
    assert_eq!(lsl.operands, "z31.b, z31.b, #7");

    let (asrd, asrd_size) = handler
        .disassemble(&[0xe0, 0x81, 0x04, 0x04], "aarch64", 0)
        .expect("generated SVE ASRD immediate should decode");
    assert_eq!(asrd_size, 4);
    assert_eq!(asrd.mnemonic, "asrd");
    assert_eq!(asrd.operands, "z0.b, p0/m, z0.b, #1");

    let (sqshl, sqshl_size) = handler
        .disassemble(&[0x00, 0x81, 0x06, 0x04], "aarch64", 0)
        .expect("generated SVE2 SQSHL predicated immediate should decode");
    assert_eq!(sqshl_size, 4);
    assert_eq!(sqshl.mnemonic, "sqshl");
    assert_eq!(sqshl.operands, "z0.b, p0/m, z0.b, #0");

    let (sqshl_decoded, sqshl_decode_size) = handler
        .decode_instruction(&[0x00, 0x81, 0x06, 0x04], "aarch64", 0)
        .expect("generated SVE2 SQSHL predicated immediate detail should decode");
    assert_eq!(sqshl_decode_size, 4);
    assert_eq!(
        sqshl_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(128)]
    );
    assert_eq!(
        sqshl_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (srshr, srshr_size) = handler
        .disassemble(&[0xe0, 0x81, 0x0c, 0x04], "aarch64", 0)
        .expect("generated SVE2 SRSHR predicated immediate should decode");
    assert_eq!(srshr_size, 4);
    assert_eq!(srshr.mnemonic, "srshr");
    assert_eq!(srshr.operands, "z0.b, p0/m, z0.b, #1");

    let (srshr_decoded, srshr_decode_size) = handler
        .decode_instruction(&[0xe0, 0x81, 0x0c, 0x04], "aarch64", 0)
        .expect("generated SVE2 SRSHR predicated immediate detail should decode");
    assert_eq!(srshr_decode_size, 4);
    assert_eq!(
        srshr_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(128)]
    );
    assert_eq!(
        srshr_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (srsra, srsra_size) = handler
        .disassemble(&[0x00, 0xe8, 0x0f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SRSRA shift accumulate should decode");
    assert_eq!(srsra_size, 4);
    assert_eq!(srsra.mnemonic, "srsra");
    assert_eq!(srsra.operands, "z0.b, z0.b, #1");

    let (srsra_decoded, srsra_decode_size) = handler
        .decode_instruction(&[0x00, 0xe8, 0x0f, 0x45], "aarch64", 0)
        .expect("generated SVE2 SRSRA shift accumulate detail should decode");
    assert_eq!(srsra_decode_size, 4);
    assert_eq!(
        srsra_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(160)]
    );
    assert_eq!(
        srsra_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );

    let (sshllb, sshllb_size) = handler
        .disassemble(&[0x00, 0xa0, 0x08, 0x45], "aarch64", 0)
        .expect("generated SVE2 SSHLLB shift long should decode");
    assert_eq!(sshllb_size, 4);
    assert_eq!(sshllb.mnemonic, "sshllb");
    assert_eq!(sshllb.operands, "z0.h, z0.b, #0");

    let (sshllb_decoded, sshllb_decode_size) = handler
        .decode_instruction(&[0x00, 0xa0, 0x08, 0x45], "aarch64", 0)
        .expect("generated SVE2 SSHLLB shift long detail should decode");
    assert_eq!(sshllb_decode_size, 4);
    assert_eq!(
        sshllb_decoded.registers_read,
        vec![RegisterId::aarch64(160)]
    );
    assert_eq!(
        sshllb_decoded.registers_written,
        vec![RegisterId::aarch64(160)]
    );
}

#[test]
fn generated_m5_sve_predicated_register_shift_text_preserves_source_and_shift_registers() {
    let handler = ArmHandler::new();

    let (asr, asr_size) = handler
        .disassemble(&[0x00, 0x80, 0x10, 0x04], "aarch64", 0)
        .expect("generated SVE ASR register shift should decode");
    assert_eq!(asr_size, 4);
    assert_eq!(asr.mnemonic, "asr");
    assert_eq!(asr.operands, "z0.b, p0/m, z0.b, z0.b");

    let (lsl, lsl_size) = handler
        .disassemble(&[0x00, 0x80, 0x13, 0x04], "aarch64", 0)
        .expect("generated SVE LSL register shift should decode");
    assert_eq!(lsl_size, 4);
    assert_eq!(lsl.mnemonic, "lsl");
    assert_eq!(lsl.operands, "z0.b, p0/m, z0.b, z0.b");

    let (lsr, lsr_size) = handler
        .disassemble(&[0x00, 0x80, 0x11, 0x04], "aarch64", 0)
        .expect("generated SVE LSR register shift should decode");
    assert_eq!(lsr_size, 4);
    assert_eq!(lsr.mnemonic, "lsr");
    assert_eq!(lsr.operands, "z0.b, p0/m, z0.b, z0.b");

    let (asr_wide, asr_wide_size) = handler
        .disassemble(&[0x20, 0x80, 0x18, 0x04], "aarch64", 0)
        .expect("generated SVE wide ASR predicated register shift should decode");
    assert_eq!(asr_wide_size, 4);
    assert_eq!(asr_wide.mnemonic, "asr");
    assert_eq!(asr_wide.operands, "z0.b, p0/m, z0.b, z1.d");

    let (asr_wide_unpred, asr_wide_unpred_size) = handler
        .disassemble(&[0x20, 0x80, 0x22, 0x04], "aarch64", 0)
        .expect("generated SVE wide ASR register shift should decode");
    assert_eq!(asr_wide_unpred_size, 4);
    assert_eq!(asr_wide_unpred.mnemonic, "asr");
    assert_eq!(asr_wide_unpred.operands, "z0.b, z1.b, z2.d");
}

#[test]
fn generated_m5_sme2_fp8_and_faminmax_report_buckets_decode() {
    let handler = ArmHandler::new();

    let (add_vg2, add_vg2_size) = handler
        .disassemble(&[0x00, 0xa3, 0x60, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD VG2 grouped vector should decode");
    assert_eq!(add_vg2_size, 4);
    assert_eq!(add_vg2.mnemonic, "add");
    assert_eq!(add_vg2.operands, "{ z0.h, z1.h }, { z0.h, z1.h }, z0.h");

    let (add_za_vg2, add_za_vg2_size) = handler
        .disassemble(&[0x55, 0x5d, 0xa0, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD ZA VG2 grouped vector should decode");
    assert_eq!(add_za_vg2_size, 4);
    assert_eq!(add_za_vg2.mnemonic, "add");
    assert_eq!(add_za_vg2.operands, "za.s[w10, 5, vgx2], { z10.s, z11.s }");

    let (bfadd_za_vg2, bfadd_za_vg2_size) = handler
        .disassemble(&[0x00, 0x1c, 0xe4, 0xc1], "aarch64", 0)
        .expect("generated SME2 BFADD ZA VG2 grouped vector should decode");
    assert_eq!(bfadd_za_vg2_size, 4);
    assert_eq!(bfadd_za_vg2.mnemonic, "bfadd");
    assert_eq!(bfadd_za_vg2.operands, "za.h[w8, 0, vgx2], { z0.h, z1.h }");

    let (fadd_za_vg4, fadd_za_vg4_size) = handler
        .disassemble(&[0x00, 0x1c, 0xa1, 0xc1], "aarch64", 0)
        .expect("generated SME2 FADD ZA VG4 grouped vector should decode");
    assert_eq!(fadd_za_vg4_size, 4);
    assert_eq!(fadd_za_vg4.mnemonic, "fadd");
    assert_eq!(fadd_za_vg4.operands, "za.s[w8, 0, vgx4], { z0.s - z3.s }");

    let (add_za_vg2_r, add_za_vg2_r_size) = handler
        .disassemble(&[0x10, 0x18, 0x20, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD ZA VG2 grouped vector and scalar vector should decode");
    assert_eq!(add_za_vg2_r_size, 4);
    assert_eq!(add_za_vg2_r.mnemonic, "add");
    assert_eq!(
        add_za_vg2_r.operands,
        "za.s[w8, 0, vgx2], { z0.s, z1.s }, z0.s"
    );

    let (add_za_vg2_group_rhs, add_za_vg2_group_rhs_size) = handler
        .disassemble(&[0x10, 0x18, 0xa0, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD ZA VG2 grouped RHS should decode");
    assert_eq!(add_za_vg2_group_rhs_size, 4);
    assert_eq!(add_za_vg2_group_rhs.mnemonic, "add");
    assert_eq!(
        add_za_vg2_group_rhs.operands,
        "za.s[w8, 0, vgx2], { z0.s, z1.s }, { z0.s, z1.s }"
    );

    let (add_za_vg4_group_rhs, add_za_vg4_group_rhs_size) = handler
        .disassemble(&[0x10, 0x18, 0xa1, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD ZA VG4 grouped RHS should decode");
    assert_eq!(add_za_vg4_group_rhs_size, 4);
    assert_eq!(add_za_vg4_group_rhs.mnemonic, "add");
    assert_eq!(
        add_za_vg4_group_rhs.operands,
        "za.s[w8, 0, vgx4], { z0.s - z3.s }, { z0.s - z3.s }"
    );

    let (add_za_vg2_r_odd, add_za_vg2_r_odd_size) = handler
        .disassemble(&[0xb7, 0x79, 0x28, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD ZA VG2 odd grouped vector should decode");
    assert_eq!(add_za_vg2_r_odd_size, 4);
    assert_eq!(add_za_vg2_r_odd.mnemonic, "add");
    assert_eq!(
        add_za_vg2_r_odd.operands,
        "za.s[w11, 7, vgx2], { z13.s, z14.s }, z8.s"
    );

    let (fdot_vg2_group, fdot_vg2_group_size) = handler
        .disassemble(&[0x30, 0x10, 0xa0, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FDOT VG2 byte-to-single grouped RHS should decode");
    assert_eq!(fdot_vg2_group_size, 4);
    assert_eq!(fdot_vg2_group.mnemonic, "fdot");
    assert_eq!(
        fdot_vg2_group.operands,
        "za.s[w8, 0, vgx2], { z0.b, z1.b }, { z0.b, z1.b }"
    );

    let (fdot_vg4, fdot_vg4_size) = handler
        .disassemble(&[0x18, 0x10, 0x30, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FDOT VG4 byte-to-single scalar RHS should decode");
    assert_eq!(fdot_vg4_size, 4);
    assert_eq!(fdot_vg4.mnemonic, "fdot");
    assert_eq!(
        fdot_vg4.operands,
        "za.s[w8, 0, vgx4], { z0.b - z3.b }, z0.b"
    );

    let (fdot_vg4_wrap, fdot_vg4_wrap_size) = handler
        .disassemble(&[0xef, 0x73, 0x3f, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FDOT VG4 wrap scalar RHS should decode");
    assert_eq!(fdot_vg4_wrap_size, 4);
    assert_eq!(fdot_vg4_wrap.mnemonic, "fdot");
    assert_eq!(
        fdot_vg4_wrap.operands,
        "za.h[w11, 7, vgx4], { z31.b, z0.b, z1.b, z2.b }, z15.b"
    );

    let (fvdot, fvdot_size) = handler
        .disassemble(&[0x20, 0x10, 0xd0, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FVDOT VG2 indexed vector should decode");
    assert_eq!(fvdot_size, 4);
    assert_eq!(fvdot.mnemonic, "fvdot");
    assert_eq!(fvdot.operands, "za.h[w8, 0, vgx2], { z0.b, z1.b }, z0.b[0]");

    let (fvdotb, fvdotb_size) = handler
        .disassemble(&[0x00, 0x08, 0xd0, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FVDOTB VG4 indexed vector should decode");
    assert_eq!(fvdotb_size, 4);
    assert_eq!(fvdotb.mnemonic, "fvdotb");
    assert_eq!(
        fvdotb.operands,
        "za.s[w8, 0, vgx4], { z0.b, z1.b }, z0.b[0]"
    );

    let (fvdott, fvdott_size) = handler
        .disassemble(&[0x10, 0x08, 0xd0, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FVDOTT VG4 indexed vector should decode");
    assert_eq!(fvdott_size, 4);
    assert_eq!(fvdott.mnemonic, "fvdott");
    assert_eq!(
        fvdott.operands,
        "za.s[w8, 0, vgx4], { z0.b, z1.b }, z0.b[0]"
    );

    let (fdot_vg2, fdot_vg2_size) = handler
        .disassemble(&[0x08, 0x10, 0x20, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FDOT VG2 grouped vector should decode");
    assert_eq!(fdot_vg2_size, 4);
    assert_eq!(fdot_vg2.mnemonic, "fdot");
    assert_eq!(fdot_vg2.operands, "za.h[w8, 0, vgx2], { z0.b, z1.b }, z0.b");

    let (fdot_vg2_btos, fdot_vg2_btos_size) = handler
        .disassemble(&[0x18, 0x10, 0x20, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FDOT VG2 byte-to-single grouped vector should decode");
    assert_eq!(fdot_vg2_btos_size, 4);
    assert_eq!(fdot_vg2_btos.mnemonic, "fdot");
    assert_eq!(
        fdot_vg2_btos.operands,
        "za.s[w8, 0, vgx2], { z0.b, z1.b }, z0.b"
    );

    let (add_vg2_decoded, add_vg2_decode_size) = handler
        .decode_instruction(&[0x00, 0xa3, 0x60, 0xc1], "aarch64", 0)
        .expect("generated SME2 ADD VG2 grouped vector detail should decode");
    assert_eq!(add_vg2_decode_size, 4);
    assert_eq!(
        add_vg2_decoded.registers_read,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(161)]
    );
    assert_eq!(
        add_vg2_decoded.registers_written,
        vec![RegisterId::aarch64(160), RegisterId::aarch64(161)]
    );

    let (famin, famin_size) = handler
        .disassemble(&[0x20, 0x80, 0x4f, 0x65], "aarch64", 0)
        .expect("generated SVE FAMIN should decode");
    assert_eq!(famin_size, 4);
    assert_eq!(famin.mnemonic, "famin");
    assert_eq!(famin.operands, "z0.h, p0/m, z0.h, z1.h");

    let (f1cvt_sve, f1cvt_sve_size) = handler
        .disassemble(&[0x00, 0x30, 0x08, 0x65], "aarch64", 0)
        .expect("generated SVE FP8 F1CVT should decode");
    assert_eq!(f1cvt_sve_size, 4);
    assert_eq!(f1cvt_sve.mnemonic, "f1cvt");
    assert_eq!(f1cvt_sve.operands, "z0.h, z0.b");

    let (f1cvt_sme, f1cvt_sme_size) = handler
        .disassemble(&[0x00, 0xe0, 0x26, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 F1CVT grouped vector should decode");
    assert_eq!(f1cvt_sme_size, 4);
    assert_eq!(f1cvt_sme.mnemonic, "f1cvt");
    assert_eq!(f1cvt_sme.operands, "{ z0.h, z1.h }, z0.b");

    let (fcvt_range4, fcvt_range4_size) = handler
        .disassemble(&[0x00, 0xe0, 0x34, 0xc1], "aarch64", 0)
        .expect("generated SME2 FP8 FCVT range-4 vector should decode");
    assert_eq!(fcvt_range4_size, 4);
    assert_eq!(fcvt_range4.mnemonic, "fcvt");
    assert_eq!(fcvt_range4.operands, "z0.b, { z0.s - z3.s }");
}

#[test]
fn generated_m5_sme_mopa_text_preserves_tile_predicates_and_half_vectors() {
    let handler = ArmHandler::new();

    let (bfmopa, bfmopa_size) = handler
        .disassemble(&[0x41, 0x55, 0x95, 0x81], "aarch64", 0)
        .expect("generated SME BFMOPA should decode");
    assert_eq!(bfmopa_size, 4);
    assert_eq!(bfmopa.mnemonic, "bfmopa");
    assert_eq!(bfmopa.operands, "za1.s, p5/m, p2/m, z10.h, z21.h");

    let (fmopa, fmopa_size) = handler
        .disassemble(&[0x41, 0x55, 0xb5, 0x81], "aarch64", 0)
        .expect("generated SME FMOPA should decode");
    assert_eq!(fmopa_size, 4);
    assert_eq!(fmopa.mnemonic, "fmopa");
    assert_eq!(fmopa.operands, "za1.s, p5/m, p2/m, z10.h, z21.h");
}

#[test]
fn generated_m5_sme_mops_text_preserves_tile_predicates_and_half_vectors() {
    let handler = ArmHandler::new();

    let (bfmops, bfmops_size) = handler
        .disassemble(&[0x51, 0x55, 0x95, 0x81], "aarch64", 0)
        .expect("generated SME BFMOPS should decode");
    assert_eq!(bfmops_size, 4);
    assert_eq!(bfmops.mnemonic, "bfmops");
    assert_eq!(bfmops.operands, "za1.s, p5/m, p2/m, z10.h, z21.h");

    let (fmops, fmops_size) = handler
        .disassemble(&[0x51, 0x55, 0xb5, 0x81], "aarch64", 0)
        .expect("generated SME FMOPS should decode");
    assert_eq!(fmops_size, 4);
    assert_eq!(fmops.mnemonic, "fmops");
    assert_eq!(fmops.operands, "za1.s, p5/m, p2/m, z10.h, z21.h");
}

#[test]
fn generated_m5_sve_immediate_arithmetic_text_preserves_arrangement_and_shift() {
    let handler = ArmHandler::new();

    let (plain, plain_size) = handler
        .disassemble(&[0x00, 0xc0, 0x20, 0x25], "aarch64", 0)
        .expect("generated SVE ADD immediate should decode");
    assert_eq!(plain_size, 4);
    assert_eq!(plain.mnemonic, "add");
    assert_eq!(plain.operands, "z0.b, z0.b, #0");

    let (shifted, shifted_size) = handler
        .disassemble(&[0x00, 0xe0, 0x60, 0x25], "aarch64", 0)
        .expect("generated SVE ADD shifted immediate should decode");
    assert_eq!(shifted_size, 4);
    assert_eq!(shifted.mnemonic, "add");
    assert_eq!(shifted.operands, "z0.h, z0.h, #0, lsl #8");
}

#[test]
fn generated_m5_sme_tile_vector_load_store_text_preserves_tile_index_predicate_and_address() {
    let handler = ArmHandler::new();

    let (ld1b, ld1b_size) = handler
        .disassemble(&[0x00, 0x00, 0x00, 0xe0], "aarch64", 0)
        .expect("generated SME LD1B tile-vector should decode");
    assert_eq!(ld1b_size, 4);
    assert_eq!(ld1b.mnemonic, "ld1b");
    assert_eq!(ld1b.operands, "{za0h.b[w12, 0]}, p0/z, [x0, x0]");

    let (sp, sp_size) = handler
        .disassemble(&[0xef, 0x7f, 0x1f, 0xe0], "aarch64", 0)
        .expect("generated SME LD1B tile-vector SP form should decode");
    assert_eq!(sp_size, 4);
    assert_eq!(sp.mnemonic, "ld1b");
    assert_eq!(sp.operands, "{za0h.b[w15, 0xf]}, p7/z, [sp]");

    let (st1b, st1b_size) = handler
        .disassemble(&[0x45, 0x55, 0x35, 0xe0], "aarch64", 0)
        .expect("generated SME ST1B tile-vector should decode");
    assert_eq!(st1b_size, 4);
    assert_eq!(st1b.mnemonic, "st1b");
    assert_eq!(st1b.operands, "{za0h.b[w14, 5]}, p5, [x10, x21]");

    let (ld1h, ld1h_size) = handler
        .disassemble(&[0x4d, 0x55, 0x55, 0xe0], "aarch64", 0)
        .expect("generated SME LD1H tile-vector should decode");
    assert_eq!(ld1h_size, 4);
    assert_eq!(ld1h.mnemonic, "ld1h");
    assert_eq!(ld1h.operands, "{za1h.h[w14, 5]}, p5/z, [x10, x21, lsl #1]");

    let (ld1w, ld1w_size) = handler
        .disassemble(&[0x45, 0x55, 0x95, 0xe0], "aarch64", 0)
        .expect("generated SME LD1W tile-vector should decode");
    assert_eq!(ld1w_size, 4);
    assert_eq!(ld1w.mnemonic, "ld1w");
    assert_eq!(ld1w.operands, "{za1h.s[w14, 1]}, p5/z, [x10, x21, lsl #2]");

    let (ld1d, ld1d_size) = handler
        .disassemble(&[0x4b, 0x55, 0xd5, 0xe0], "aarch64", 0)
        .expect("generated SME LD1D tile-vector should decode");
    assert_eq!(ld1d_size, 4);
    assert_eq!(ld1d.mnemonic, "ld1d");
    assert_eq!(ld1d.operands, "{za5h.d[w14, 1]}, p5/z, [x10, x21, lsl #3]");

    let (ld1q, ld1q_size) = handler
        .disassemble(&[0x45, 0x55, 0xd5, 0xe1], "aarch64", 0)
        .expect("generated SME LD1Q tile-vector should decode");
    assert_eq!(ld1q_size, 4);
    assert_eq!(ld1q.mnemonic, "ld1q");
    assert_eq!(ld1q.operands, "{za5h.q[w14, 0]}, p5/z, [x10, x21, lsl #4]");

    let (st1h, st1h_size) = handler
        .disassemble(&[0x4d, 0x55, 0x75, 0xe0], "aarch64", 0)
        .expect("generated SME ST1H tile-vector should decode");
    assert_eq!(st1h_size, 4);
    assert_eq!(st1h.mnemonic, "st1h");
    assert_eq!(st1h.operands, "{za1h.h[w14, 5]}, p5, [x10, x21, lsl #1]");

    let (st1w, st1w_size) = handler
        .disassemble(&[0x45, 0x55, 0xb5, 0xe0], "aarch64", 0)
        .expect("generated SME ST1W tile-vector should decode");
    assert_eq!(st1w_size, 4);
    assert_eq!(st1w.mnemonic, "st1w");
    assert_eq!(st1w.operands, "{za1h.s[w14, 1]}, p5, [x10, x21, lsl #2]");

    let (st1d, st1d_size) = handler
        .disassemble(&[0x4b, 0x55, 0xf5, 0xe0], "aarch64", 0)
        .expect("generated SME ST1D tile-vector should decode");
    assert_eq!(st1d_size, 4);
    assert_eq!(st1d.mnemonic, "st1d");
    assert_eq!(st1d.operands, "{za5h.d[w14, 1]}, p5, [x10, x21, lsl #3]");

    let (st1q, st1q_size) = handler
        .disassemble(&[0x45, 0x55, 0xf5, 0xe1], "aarch64", 0)
        .expect("generated SME ST1Q tile-vector should decode");
    assert_eq!(st1q_size, 4);
    assert_eq!(st1q.mnemonic, "st1q");
    assert_eq!(st1q.operands, "{za5h.q[w14, 0]}, p5, [x10, x21, lsl #4]");
}

#[test]
fn generated_m5_sme_psel_text_preserves_predicate_tile_index_operands() {
    let handler = ArmHandler::new();

    let (psel_b, psel_b_size) = handler
        .disassemble(&[0x45, 0x55, 0x75, 0x25], "aarch64", 0)
        .expect("generated SME PSEL byte should decode");
    assert_eq!(psel_b_size, 4);
    assert_eq!(psel_b.mnemonic, "psel");
    assert_eq!(psel_b.operands, "p5, p5, p10.b[w13, 6]");

    let (psel_h, psel_h_size) = handler
        .disassemble(&[0xa7, 0x6d, 0x68, 0x25], "aarch64", 0)
        .expect("generated SME PSEL halfword should decode");
    assert_eq!(psel_h_size, 4);
    assert_eq!(psel_h.mnemonic, "psel");
    assert_eq!(psel_h.operands, "p7, p11, p13.h[w12, 2]");

    let (psel_s, psel_s_size) = handler
        .disassemble(&[0xef, 0x7d, 0xf3, 0x25], "aarch64", 0)
        .expect("generated SME PSEL word should decode");
    assert_eq!(psel_s_size, 4);
    assert_eq!(psel_s.mnemonic, "psel");
    assert_eq!(psel_s.operands, "p15, p15, p15.s[w15, 3]");

    let (psel_d, psel_d_size) = handler
        .disassemble(&[0xef, 0x7d, 0xe3, 0x25], "aarch64", 0)
        .expect("generated SME PSEL doubleword should decode");
    assert_eq!(psel_d_size, 4);
    assert_eq!(psel_d.mnemonic, "psel");
    assert_eq!(psel_d.operands, "p15, p15, p15.d[w15, 1]");
}

#[test]
fn generated_m5_sme_mova_text_preserves_tile_vector_index_predicate_and_arrangement() {
    let handler = ArmHandler::new();

    let (extract_b, extract_b_size) = handler
        .disassemble(&[0x55, 0x55, 0x02, 0xc0], "aarch64", 0)
        .expect("generated SME MOVA tile-to-vector byte should decode");
    assert_eq!(extract_b_size, 4);
    assert_eq!(extract_b.mnemonic, "mov");
    assert_eq!(extract_b.operands, "z21.b, p5/m, za0h.b[w14, 0xa]");

    let (extract_q, extract_q_size) = handler
        .disassemble(&[0x87, 0xa9, 0xc3, 0xc0], "aarch64", 0)
        .expect("generated SME MOVA tile-to-vector quadword should decode");
    assert_eq!(extract_q_size, 4);
    assert_eq!(extract_q.mnemonic, "mov");
    assert_eq!(extract_q.operands, "z7.q, p2/m, za12v.q[w13, 0]");

    let (insert_b, insert_b_size) = handler
        .disassemble(&[0x45, 0x55, 0x00, 0xc0], "aarch64", 0)
        .expect("generated SME MOVA vector-to-tile byte should decode");
    assert_eq!(insert_b_size, 4);
    assert_eq!(insert_b.mnemonic, "mov");
    assert_eq!(insert_b.operands, "za0h.b[w14, 5], p5/m, z10.b");

    let (insert_q, insert_q_size) = handler
        .disassemble(&[0x87, 0xa9, 0xc1, 0xc0], "aarch64", 0)
        .expect("generated SME MOVA vector-to-tile quadword should decode");
    assert_eq!(insert_q_size, 4);
    assert_eq!(insert_q.mnemonic, "mov");
    assert_eq!(insert_q.operands, "za7v.q[w13, 0], p2/m, z12.q");
}

#[test]
fn generated_m5_sme_za_spill_fill_text_preserves_index_and_vector_length_offset() {
    let handler = ArmHandler::new();

    let (ldr_plain, ldr_plain_size) = handler
        .disassemble(&[0x00, 0x00, 0x00, 0xe1], "aarch64", 0)
        .expect("generated SME LDR ZA plain should decode");
    assert_eq!(ldr_plain_size, 4);
    assert_eq!(ldr_plain.mnemonic, "ldr");
    assert_eq!(ldr_plain.operands, "za[w12, 0], [x0]");

    let (ldr_offset, ldr_offset_size) = handler
        .disassemble(&[0xef, 0x63, 0x00, 0xe1], "aarch64", 0)
        .expect("generated SME LDR ZA offset should decode");
    assert_eq!(ldr_offset_size, 4);
    assert_eq!(ldr_offset.mnemonic, "ldr");
    assert_eq!(ldr_offset.operands, "za[w15, 0xf], [sp, #0xf, mul vl]");

    let (str_offset, str_offset_size) = handler
        .disassemble(&[0x45, 0x41, 0x20, 0xe1], "aarch64", 0)
        .expect("generated SME STR ZA offset should decode");
    assert_eq!(str_offset_size, 4);
    assert_eq!(str_offset.mnemonic, "str");
    assert_eq!(str_offset.operands, "za[w14, 5], [x10, #5, mul vl]");
}

#[test]
fn generated_m5_sme_zero_text_preserves_matrix_tile_list() {
    let handler = ArmHandler::new();

    let (empty, empty_size) = handler
        .disassemble(&[0x00, 0x00, 0x08, 0xc0], "aarch64", 0)
        .expect("generated SME ZERO empty tile list should decode");
    assert_eq!(empty_size, 4);
    assert_eq!(empty.mnemonic, "zero");
    assert_eq!(empty.operands, "{}");

    let (all, all_size) = handler
        .disassemble(&[0xff, 0x00, 0x08, 0xc0], "aarch64", 0)
        .expect("generated SME ZERO full tile list should decode");
    assert_eq!(all_size, 4);
    assert_eq!(all.mnemonic, "zero");
    assert_eq!(all.operands, "{za}");

    let (half, half_size) = handler
        .disassemble(&[0x55, 0x00, 0x08, 0xc0], "aarch64", 0)
        .expect("generated SME ZERO half tile list should decode");
    assert_eq!(half_size, 4);
    assert_eq!(half.mnemonic, "zero");
    assert_eq!(half.operands, "{za0.h}");

    let (double, double_size) = handler
        .disassemble(&[0xb7, 0x00, 0x08, 0xc0], "aarch64", 0)
        .expect("generated SME ZERO double tile list should decode");
    assert_eq!(double_size, 4);
    assert_eq!(double.mnemonic, "zero");
    assert_eq!(
        double.operands,
        "{za0.d, za1.d, za2.d, za4.d, za5.d, za7.d}"
    );

    let (single_s, single_s_size) = handler
        .disassemble(&[0x33, 0x00, 0x08, 0xc0], "aarch64", 0)
        .expect("generated SME ZERO s tile list should decode");
    assert_eq!(single_s_size, 4);
    assert_eq!(single_s.mnemonic, "zero");
    assert_eq!(single_s.operands, "{za0.s,za1.s}");
}

#[test]
fn generated_m5_sme_streaming_vl_add_keeps_structured_operands_for_semantics() {
    let handler = ArmHandler::new();

    let (decoded, size) = handler
        .decode_instruction(&[0xf7, 0x5f, 0x68, 0x04], "aarch64", 0)
        .expect("generated SME ADDSPL should decode");
    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, "addspl");
    assert_eq!(
        decoded.operands,
        vec![
            Operand::Register {
                register: RegisterId {
                    architecture: ArchitectureId::AArch64,
                    id: 23,
                },
            },
            Operand::Register {
                register: RegisterId {
                    architecture: ArchitectureId::AArch64,
                    id: 8,
                },
            },
            Operand::Immediate {
                value: -1,
                unsigned_mask: 0,
            },
        ]
    );
    assert_eq!(decoded.registers_read, vec![RegisterId::aarch64(8)]);
    assert_eq!(decoded.registers_written, vec![RegisterId::aarch64(23)]);
}

#[test]
fn generated_m5_sme_state_access_text_preserves_rdsvl_and_streaming_aliases() {
    let handler = ArmHandler::new();

    let (rdsvl_zero, rdsvl_zero_size) = handler
        .disassemble(&[0x00, 0x58, 0xbf, 0x04], "aarch64", 0)
        .expect("SME RDSVL zero immediate should decode");
    assert_eq!(rdsvl_zero_size, 4);
    assert_eq!(rdsvl_zero.mnemonic, "rdsvl");
    assert_eq!(rdsvl_zero.operands, "x0, #0");

    let (rdsvl_neg, rdsvl_neg_size) = handler
        .disassemble(&[0xff, 0x5f, 0xbf, 0x04], "aarch64", 0)
        .expect("SME RDSVL negative immediate should decode");
    assert_eq!(rdsvl_neg_size, 4);
    assert_eq!(rdsvl_neg.mnemonic, "rdsvl");
    assert_eq!(rdsvl_neg.operands, "xzr, #-1");

    let (smstart, smstart_size) = handler
        .disassemble(&[0x7f, 0x47, 0x03, 0xd5], "aarch64", 0)
        .expect("SME SMSTART should decode");
    assert_eq!(smstart_size, 4);
    assert_eq!(smstart.mnemonic, "smstart");
    assert_eq!(smstart.operands, "");

    let (smstart_za, smstart_za_size) = handler
        .disassemble(&[0x7f, 0x45, 0x03, 0xd5], "aarch64", 0)
        .expect("SME SMSTART ZA should decode");
    assert_eq!(smstart_za_size, 4);
    assert_eq!(smstart_za.mnemonic, "smstart");
    assert_eq!(smstart_za.operands, "za");

    let (smstop_sm, smstop_sm_size) = handler
        .disassemble(&[0x7f, 0x42, 0x03, 0xd5], "aarch64", 0)
        .expect("SME SMSTOP SM should decode");
    assert_eq!(smstop_sm_size, 4);
    assert_eq!(smstop_sm.mnemonic, "smstop");
    assert_eq!(smstop_sm.operands, "sm");
}

#[test]
fn generated_specs_decode_prefetch_literal_and_lse_cas_parity_cases() {
    let handler = ArmHandler::new();

    let (cas, cas_size) = handler
        .disassemble(&[0x20, 0x7c, 0xa2, 0x88], "aarch64", 0)
        .expect("generated LSE CAS should decode");
    assert_eq!(cas_size, 4);
    assert_eq!(cas.mnemonic, "cas");
    assert_eq!(cas.operands, "w2, w0, [x1]");

    let (prfm, prfm_size) = handler
        .disassemble(&[0x00, 0x00, 0x80, 0xd8], "aarch64", 0)
        .expect("generated PRFM literal should decode");
    assert_eq!(prfm_size, 4);
    assert_eq!(prfm.mnemonic, "prfm");
    assert_eq!(prfm.operands, "pldl1keep, 0xfffffffffff00000");
}
