use std::process::Command;

#[test]
fn test_json_mode_wraps_cli_validation_failures() {
    let output = Command::new(env!("CARGO_BIN_EXE_robustone"))
        .args(["--json", "riscv32", "xyz"])
        .output()
        .expect("robustone binary should run");

    assert!(!output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout should contain JSON");

    assert_eq!(parsed["errors"][0]["kind"], "validation_error");
    assert_eq!(parsed["bytes_processed"], 0);
}

#[test]
fn test_json_mode_wraps_clap_parse_failures() {
    let output = Command::new(env!("CARGO_BIN_EXE_robustone"))
        .args(["--json", "-z"])
        .output()
        .expect("robustone binary should run");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout should contain JSON");

    assert_eq!(parsed["errors"][0]["kind"], "invalid_command");
    assert_eq!(parsed["bytes_processed"], 0);
}
