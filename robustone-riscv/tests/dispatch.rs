use robustone_core::ArchitectureDispatcher;
use robustone_core::architecture::Architecture;
use robustone_core::common::ArchitectureProfile;
use robustone_riscv::RiscVHandler;

fn dispatcher_with_riscv() -> ArchitectureDispatcher {
    let mut dispatcher = ArchitectureDispatcher::new();
    dispatcher.register(Box::new(RiscVHandler::new()));
    dispatcher
}

#[test]
fn riscv_decode_addi_yields_li_alias() {
    let dispatcher = dispatcher_with_riscv();
    let (decoded, size) = dispatcher
        .decode_instruction(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0)
        .expect("decode should succeed");

    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, "addi");
    assert_eq!(decoded.render_hints.compat_mnemonic.as_deref(), Some("li"));
    assert_eq!(decoded.render_hints.compat_hidden_operands, vec![1]);
}

#[test]
fn riscv_invalid_encoding_returns_structured_error() {
    let dispatcher = dispatcher_with_riscv();
    let error = dispatcher
        .decode_instruction(&[0xff, 0xff, 0xff, 0xff], "riscv32", 0)
        .expect_err("invalid encoding should fail");
    assert_eq!(error.stable_kind(), "invalid_encoding");
}

#[test]
fn riscv_profile_enforces_enabled_extensions() {
    let profile = ArchitectureProfile::riscv(Architecture::RiscV32, "riscv32", 32, vec!["I"]);
    let dispatcher = dispatcher_with_riscv();

    let error = dispatcher
        .decode_with_profile(&[0x05, 0x68], &profile, 0)
        .expect_err("compressed instruction should require C extension");

    assert_eq!(error.stable_kind(), "unsupported_extension");
}

#[test]
fn riscv_profile_rejects_mode_mismatch() {
    let profile = ArchitectureProfile::riscv(Architecture::RiscV32, "riscv64", 32, vec!["I", "C"]);
    let error = match RiscVHandler::from_profile(&profile) {
        Ok(_) => panic!("mismatched profile should fail"),
        Err(error) => error,
    };

    assert_eq!(error.stable_kind(), "unsupported_mode");
}

#[test]
fn riscv_rv64_compressed_ld_decodes() {
    let dispatcher = dispatcher_with_riscv();
    let (decoded, size) = dispatcher
        .decode_instruction(&[0x00, 0x60], "riscv64", 0)
        .expect("RV64 should decode c.ld");
    assert_eq!(size, 2);
    assert_eq!(decoded.mnemonic, "c.ld");
}

#[test]
fn riscv_rv64_only_ld_rejected_on_rv32() {
    let dispatcher = dispatcher_with_riscv();
    let (decoded, size) = dispatcher
        .decode_instruction(&[0x83, 0x30, 0x00, 0x00], "riscv64", 0)
        .expect("RV64 should decode ld");

    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, "ld");

    let error = dispatcher
        .decode_instruction(&[0x83, 0x30, 0x00, 0x00], "riscv32", 0)
        .expect_err("RV32 should reject ld with unsupported_mode");

    assert_eq!(error.stable_kind(), "unsupported_mode");
    assert_eq!(error.architecture_name(), Some("riscv32"));
}

#[test]
fn riscv_profile_rejects_d_without_f() {
    let profile = ArchitectureProfile::riscv(Architecture::RiscV32, "riscv32", 32, vec!["I", "D"]);
    let error = match RiscVHandler::from_profile(&profile) {
        Ok(_) => panic!("D without F should be rejected"),
        Err(error) => error,
    };

    assert_eq!(error.stable_kind(), "unsupported_extension");
}

#[test]
fn riscv_disassemble_with_profile_enforces_extensions() {
    let profile = ArchitectureProfile::riscv(Architecture::RiscV64, "riscv64", 64, vec!["I", "A"]);
    let dispatcher = dispatcher_with_riscv();

    let error = dispatcher
        .disassemble_with_profile(&[0xd3, 0x02, 0x73, 0x00], &profile, 0)
        .expect_err("missing F/D should be reported");

    assert_eq!(error.stable_kind(), "unsupported_extension");
}

#[test]
fn riscv_rv64_only_lr_d_rejected_on_rv32() {
    let dispatcher = dispatcher_with_riscv();
    let (decoded, size) = dispatcher
        .decode_instruction(&[0x2f, 0xb4, 0x02, 0x12], "riscv64", 0)
        .expect("RV64 should decode lr.d");

    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, "lr.d");

    let error = dispatcher
        .decode_instruction(&[0x2f, 0xb4, 0x02, 0x12], "riscv32", 0)
        .expect_err("RV32 should reject lr.d with unsupported_mode");

    assert_eq!(error.stable_kind(), "unsupported_mode");
    assert_eq!(error.architecture_name(), Some("riscv32"));
}

#[test]
fn riscv_rv64_only_compressed_families_rejected_on_rv32() {
    let dispatcher = dispatcher_with_riscv();

    let (decoded, size) = dispatcher
        .decode_instruction(&[0x00, 0x60], "riscv64", 0)
        .expect("RV64 should decode c.ld");
    assert_eq!(size, 2);
    assert_eq!(decoded.mnemonic, "c.ld");

    let (decoded, size) = dispatcher
        .decode_instruction(&[0x00, 0x60], "riscv32", 0)
        .expect("RV32 GC should decode c.flw");
    assert_eq!(size, 2);
    assert_eq!(decoded.mnemonic, "c.flw");

    let profile =
        ArchitectureProfile::riscv(Architecture::RiscV32, "riscv32", 32, vec!["I", "M", "C"]);
    let error = dispatcher
        .decode_with_profile(&[0x00, 0x60], &profile, 0)
        .expect_err("RV32 without F should reject c.flw encoding");
    assert_eq!(error.stable_kind(), "invalid_encoding");
}

#[test]
fn riscv_rv64_only_mulw_decodes_on_rv64_rejected_on_rv32() {
    let dispatcher = dispatcher_with_riscv();
    let (decoded, size) = dispatcher
        .decode_instruction(&[0xbb, 0x00, 0x31, 0x02], "riscv64", 0)
        .expect("RV64 should decode mulw");

    assert_eq!(size, 4);
    assert_eq!(decoded.mnemonic, "mulw");

    let error = dispatcher
        .decode_instruction(&[0xbb, 0x00, 0x31, 0x02], "riscv32", 0)
        .expect_err("RV32 should reject mulw with unsupported_mode");

    assert_eq!(error.stable_kind(), "unsupported_mode");
    assert_eq!(error.architecture_name(), Some("riscv32"));
}

#[test]
fn riscv_rv64c_addiw_decodes_on_rv64_rv32_keeps_c_jal() {
    let dispatcher = dispatcher_with_riscv();
    let (rv64, size) = dispatcher
        .decode_instruction(&[0x85, 0x20], "riscv64", 0)
        .expect("RV64 should decode c.addiw");
    assert_eq!(size, 2);
    assert_eq!(rv64.mnemonic, "c.addiw");

    let (rv32, size) = dispatcher
        .decode_instruction(&[0x85, 0x20], "riscv32", 0)
        .expect("RV32 should still decode c.jal");
    assert_eq!(size, 2);
    assert_eq!(rv32.mnemonic, "c.jal");
}

#[test]
fn riscv_rv64c_word_alu_encodings_mode_sensitive() {
    let dispatcher = dispatcher_with_riscv();
    let cases = [(&[0x05, 0x9c][..], "c.subw"), (&[0x25, 0x9c][..], "c.addw")];

    for (bytes, expected_mnemonic) in cases {
        let (decoded, size) = dispatcher
            .decode_instruction(bytes, "riscv64", 0)
            .expect("RV64 should decode the mode-sensitive compressed form");

        assert_eq!(size, 2);
        assert_eq!(decoded.mnemonic, expected_mnemonic);

        let error = dispatcher
            .decode_instruction(bytes, "riscv32", 0)
            .expect_err("RV32 should reject the RV64C word-ALU form with unsupported_mode");

        assert_eq!(error.stable_kind(), "unsupported_mode");
        assert_eq!(error.architecture_name(), Some("riscv32"));
    }
}

#[test]
fn riscv_profile_matrix_enforces_extension_boundaries() {
    let dispatcher = dispatcher_with_riscv();

    let amoadd_w: &[u8] = &[0xaf, 0x21, 0x52, 0x00];
    let mul: &[u8] = &[0xb3, 0x01, 0x52, 0x02];
    let fadd_s: &[u8] = &[0xd3, 0x00, 0x31, 0x00];
    let c_addi: &[u8] = &[0x05, 0x05];

    // GC profile: everything should decode.
    let gc = ArchitectureProfile::riscv32gc();
    dispatcher
        .decode_with_profile(amoadd_w, &gc, 0)
        .expect("GC should decode A");
    dispatcher
        .decode_with_profile(mul, &gc, 0)
        .expect("GC should decode M");
    dispatcher
        .decode_with_profile(fadd_s, &gc, 0)
        .expect("GC should decode F");
    dispatcher
        .decode_with_profile(c_addi, &gc, 0)
        .expect("GC should decode C");

    // I-only profile: A/M/F/C should all report unsupported_extension.
    let i_only = ArchitectureProfile::riscv(Architecture::RiscV32, "riscv32", 32, vec!["I"]);
    assert_eq!(
        dispatcher
            .decode_with_profile(amoadd_w, &i_only, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );
    assert_eq!(
        dispatcher
            .decode_with_profile(mul, &i_only, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );
    assert_eq!(
        dispatcher
            .decode_with_profile(fadd_s, &i_only, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );
    assert_eq!(
        dispatcher
            .decode_with_profile(c_addi, &i_only, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );

    // I+M+C profile: A and F should report unsupported_extension.
    let imc = ArchitectureProfile::riscv(Architecture::RiscV32, "riscv32", 32, vec!["I", "M", "C"]);
    dispatcher
        .decode_with_profile(mul, &imc, 0)
        .expect("IMC should decode M");
    dispatcher
        .decode_with_profile(c_addi, &imc, 0)
        .expect("IMC should decode C");
    assert_eq!(
        dispatcher
            .decode_with_profile(amoadd_w, &imc, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );
    assert_eq!(
        dispatcher
            .decode_with_profile(fadd_s, &imc, 0)
            .unwrap_err()
            .stable_kind(),
        "unsupported_extension"
    );
}
