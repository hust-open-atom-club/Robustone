use std::fs;
use std::process::Command;

use tempfile::TempDir;

#[test]
fn aarch64_gen_writes_specs_catalog_and_reports() {
    let temp = TempDir::new().expect("tempdir should be created");
    let llvm_project = temp.path().join("llvm-project");
    let aarch64_dir = llvm_project.join("llvm/lib/Target/AArch64");
    fs::create_dir_all(&aarch64_dir).expect("llvm fixture dir should be created");
    fs::write(
        aarch64_dir.join("AArch64.td.json"),
        include_str!("fixtures/aarch64_tblgen_subset.json"),
    )
    .expect("fixture should be written");

    let out_dir = temp.path().join("generated");
    let artifact_dir = temp.path().join("artifacts");

    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("aarch64-gen")
        .arg("--llvm-project")
        .arg(&llvm_project)
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--artifact-dir")
        .arg(&artifact_dir)
        .output()
        .expect("xtask should run");

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let base_integer = fs::read_to_string(out_dir.join("base_integer.rs"))
        .expect("base_integer.rs should be written");
    assert!(base_integer.contains("// Input hash:"));
    assert!(!base_integer.to_lowercase().contains("llvm"));

    let provenance = fs::read_to_string(artifact_dir.join("provenance.json"))
        .expect("provenance should be written");
    assert!(provenance.contains("llvm/lib/Target/AArch64/AArch64.td.json"));

    let catalog =
        fs::read_to_string(artifact_dir.join("catalog.json")).expect("catalog should be written");
    assert!(catalog.contains("\"llvm_name\": \"PseudoRET\""));
    assert!(catalog.contains("\"skip_reason\": \"pseudo\""));

    let report =
        fs::read_to_string(artifact_dir.join("report.md")).expect("report should be written");
    assert!(report.contains("# AArch64 TableGen Coverage Report"));
    assert!(report.contains("| branch | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |"));
}

#[test]
fn aarch64_gen_check_detects_drift() {
    let temp = TempDir::new().expect("tempdir should be created");
    let llvm_project = temp.path().join("llvm-project");
    let aarch64_dir = llvm_project.join("llvm/lib/Target/AArch64");
    fs::create_dir_all(&aarch64_dir).expect("llvm fixture dir should be created");
    fs::write(
        aarch64_dir.join("AArch64.td.json"),
        include_str!("fixtures/aarch64_tblgen_subset.json"),
    )
    .expect("fixture should be written");

    let out_dir = temp.path().join("generated");
    let artifact_dir = temp.path().join("artifacts");

    let generate = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("aarch64-gen")
        .arg("--llvm-project")
        .arg(&llvm_project)
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--artifact-dir")
        .arg(&artifact_dir)
        .output()
        .expect("xtask generate should run");
    assert!(
        generate.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&generate.stdout),
        String::from_utf8_lossy(&generate.stderr)
    );

    fs::write(out_dir.join("base_integer.rs"), "drift\n")
        .expect("generated file should be changed");

    let check = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("aarch64-gen-check")
        .arg("--llvm-project")
        .arg(&llvm_project)
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--artifact-dir")
        .arg(&artifact_dir)
        .output()
        .expect("xtask check should run");

    assert!(!check.status.success());
    let stderr = String::from_utf8_lossy(&check.stderr);
    assert!(stderr.contains("generated AArch64 specs are out of date"));
    assert!(stderr.contains("base_integer.rs"));
}
