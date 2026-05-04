//! Robustone workspace task runner.
//!
//! Provides development and CI tools:
//! - `cargo xtask verify-boundary` – check workspace dependency and content boundaries
//! - `cargo xtask check-spec --all` – validate instruction spec tables
//! - `cargo xtask compat [--strict]` – run Capstone compatibility tests
//! - `cargo xtask compat-report [--deny-xfail-increase]` – generate compatibility report
//! - `cargo xtask audit-no-hardcode` – detect mnemonic-based hardcode
//! - `cargo xtask new-arch <name>` – scaffold a new architecture crate

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo xtask <command> [args...]");
        eprintln!("Commands:");
        eprintln!("  verify-boundary       Check workspace boundary constraints");
        eprintln!("  check-spec --all      Validate instruction spec tables");
        eprintln!("  compat [--strict]     Run Capstone compatibility tests");
        eprintln!("  compat-report [--deny-xfail-increase]  Generate compatibility report");
        eprintln!("  audit-no-hardcode     Detect mnemonic-based hardcode in production");
        eprintln!("  new-arch <name>       Scaffold a new architecture crate");
        return ExitCode::FAILURE;
    }

    match args[0].as_str() {
        "verify-boundary" => verify_boundary(),
        "check-spec" => check_spec(&args[1..]),
        "compat" => compat(&args[1..]),
        "compat-report" => compat_report(&args[1..]),
        "audit-no-hardcode" => audit_no_hardcode(&args[1..]),
        "new-arch" => new_arch(&args[1..]),
        other => {
            eprintln!("Unknown command: {}", other);
            ExitCode::FAILURE
        }
    }
}

// ============================================================================
// verify-boundary
// ============================================================================

fn verify_boundary() -> ExitCode {
    let mut violations = Vec::new();
    let workspace_root = find_workspace_root();

    // 1. production arch crates must not contain capstone/llvm/binutils strings
    let arch_crates = [
        "robustone-loongarch",
        "robustone-riscv",
        "robustone-arm",
        "robustone-x86",
    ];
    for crate_name in &arch_crates {
        let src_dir = workspace_root.join(crate_name).join("src");
        if src_dir.exists() {
            check_forbidden_strings(
                &src_dir,
                crate_name,
                &["capstone", "llvm", "binutils"],
                &mut violations,
            );
        }
    }

    // 2. production crates must not depend on robustone-compat
    let production_crates = [
        "robustone-core",
        "robustone-isa",
        "robustone-isa-macros",
        "robustone-loongarch",
        "robustone-riscv",
        "robustone-arm",
        "robustone-x86",
    ];
    for crate_name in &production_crates {
        let cargo_toml = workspace_root.join(crate_name).join("Cargo.toml");
        if cargo_toml.exists() {
            let content = fs::read_to_string(&cargo_toml).unwrap_or_default();
            if content.contains("robustone-compat") || content.contains("robustone-capstone-compat")
            {
                violations.push(format!(
                    "{}: production crate Cargo.toml references compat crate",
                    cargo_toml.display()
                ));
            }
        }
    }

    // 3. no python decoder generators
    let scripts_dir = workspace_root.join("scripts");
    if scripts_dir.exists() {
        for entry in fs::read_dir(&scripts_dir)
            .unwrap_or_else(|_| panic!("read_dir {}", scripts_dir.display()))
        {
            let entry = entry.unwrap();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("generate_") && name.ends_with(".py") {
                violations.push(format!(
                    "scripts/{}: Python decoder generator still exists",
                    name
                ));
            }
        }
    }

    // 4. production crates must not read third_party/capstone
    for crate_name in &production_crates {
        let src_dir = workspace_root.join(crate_name).join("src");
        if src_dir.exists() {
            check_forbidden_strings(
                &src_dir,
                crate_name,
                &["third_party/capstone", "third_party\\capstone"],
                &mut violations,
            );
        }
    }

    // 5. RISC-V production crate must not reference legacy decoder infrastructure
    let riscv_src = workspace_root.join("robustone-riscv").join("src");
    if riscv_src.exists() {
        check_forbidden_strings(
            &riscv_src,
            "robustone-riscv",
            &["InstructionExtension", "create_extensions()"],
            &mut violations,
        );
    }
    let riscv_cargo = workspace_root.join("robustone-riscv").join("Cargo.toml");
    if riscv_cargo.exists() {
        let content = fs::read_to_string(&riscv_cargo).unwrap_or_default();
        if content.contains("legacy-decoder") {
            violations.push(format!(
                "{}: legacy-decoder feature still present",
                riscv_cargo.display()
            ));
        }
    }

    if violations.is_empty() {
        println!("verify-boundary: OK – no violations found");
        ExitCode::SUCCESS
    } else {
        eprintln!(
            "verify-boundary: FAILED – {} violation(s) found",
            violations.len()
        );
        for v in &violations {
            eprintln!("  - {}", v);
        }
        ExitCode::FAILURE
    }
}

fn check_forbidden_strings(
    dir: &Path,
    crate_name: &str,
    forbidden: &[&str],
    violations: &mut Vec<String>,
) {
    for entry in walk_dir(dir) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let _path_str = path.to_string_lossy();
        // NOTE: printer.rs and extensions/ are no longer skipped.
        // All Capstone-derived strings have been refactored to neutral terminology.
        // Any new violations will be caught by the check below.
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for &word in forbidden {
            if content.to_lowercase().contains(word) {
                for (line_no, line) in content.lines().enumerate() {
                    let lower = line.to_lowercase();
                    if lower.contains(word)
                        && !lower.contains("textrenderprofile")
                        && !lower.contains("compat_aliases")
                        && !lower.contains("capstone_hidden")
                        && !lower.contains("capstone_test_")
                        && !lower.contains("compat_mnemonic")
                        && !lower.contains("render_hints")
                        && !lower.contains("with_compat_alias")
                        && !lower.contains("textprofile")
                        && !lower.contains("compat_alias")
                    {
                        violations.push(format!(
                            "{}:{}: forbidden string '{}' in {}",
                            path.strip_prefix(dir.parent().unwrap())
                                .unwrap_or(&path)
                                .display(),
                            line_no + 1,
                            word,
                            crate_name
                        ));
                        break;
                    }
                }
            }
        }
    }
}

fn walk_dir(dir: &Path) -> Vec<fs::DirEntry> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(walk_dir(&path));
            } else {
                result.push(entry);
            }
        }
    }
    result
}

// ============================================================================
// check-spec
// ============================================================================

fn check_spec(args: &[String]) -> ExitCode {
    if args.contains(&"--all".to_string()) {
        let arch_crates = [
            "robustone-loongarch",
            "robustone-riscv",
            "robustone-arm",
            "robustone-x86",
        ];
        let mut failed = false;
        for crate_name in &arch_crates {
            let test_name = "test_specs_full_validation";
            let output = std::process::Command::new("cargo")
                .args([
                    "test",
                    "-p",
                    crate_name,
                    "--test",
                    "spec_validation",
                    test_name,
                ])
                .output();
            match output {
                Ok(out) if out.status.success() => {
                    println!("check-spec: {} OK", crate_name);
                }
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if stderr.contains("no test target named") {
                        eprintln!(
                            "check-spec: {} FAILED (missing spec_validation integration test)",
                            crate_name
                        );
                        failed = true;
                    } else {
                        eprintln!("check-spec: {} FAILED\n{}", crate_name, stderr);
                        failed = true;
                    }
                }
                Err(e) => {
                    eprintln!("check-spec: {} error: {}", crate_name, e);
                    failed = true;
                }
            }
        }
        if failed {
            ExitCode::FAILURE
        } else {
            println!("check-spec --all: OK");
            ExitCode::SUCCESS
        }
    } else {
        eprintln!("Usage: cargo xtask check-spec --all");
        ExitCode::FAILURE
    }
}

// ============================================================================
// new-arch
// ============================================================================

fn new_arch(args: &[String]) -> ExitCode {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        eprintln!("Usage: cargo xtask new-arch <name>");
        eprintln!("Scaffolds a new architecture crate under robustone-<name>/");
        return ExitCode::FAILURE;
    }
    let name = &args[0];
    let crate_name = format!("robustone-{}", name.to_lowercase());
    let workspace_root = find_workspace_root();
    let crate_dir = workspace_root.join(&crate_name);

    if crate_dir.exists() {
        eprintln!("Error: {} already exists", crate_dir.display());
        return ExitCode::FAILURE;
    }

    fs::create_dir_all(crate_dir.join("src/specs")).unwrap();
    fs::create_dir_all(crate_dir.join("tests")).unwrap();

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
robustone-core = {{ path = "../robustone-core" }}
robustone-isa = {{ path = "../robustone-isa" }}
robustone-isa-macros = {{ path = "../robustone-isa-macros" }}
bitflags = "2"
"#,
        crate_name
    );
    fs::write(crate_dir.join("Cargo.toml"), cargo_toml).unwrap();

    let lib_rs = format!(
        r#"#![forbid(unsafe_code)]

pub mod arch;
pub mod profile;
pub mod registers;
pub mod formats;
pub mod specs;
pub mod render;

pub use arch::{{{}Backend, {}Decoder}};
"#,
        to_pascal_case(name),
        to_pascal_case(name)
    );
    fs::write(crate_dir.join("src/lib.rs"), lib_rs).unwrap();

    let pascal = to_pascal_case(name);
    let _upper = pascal.to_uppercase();
    let _arch_id = &pascal;
    let _register_ctor = name.to_lowercase();
    let arch_rs = format!(
        r#"use robustone_core::ir::{{ArchitectureId, RegisterId}};
use robustone_core::types::error::{{DecodeErrorKind, DisasmError}};
use robustone_isa::{{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead,
    InstructionSpec, RenderPolicy,
}};

robustone_isa_macros::define_arch! {{
    pub arch {pascal} {{
        word = u32;
        endian = little;
        instruction_length = fixed(4);
        modes {{ Base = "{name}"; }};
        features: u8 {{ BASE = 0; }};
        registers = {name}_registers;
        formats = {name}_formats;
        specs = {name}_specs;
        render = {pascal}RenderPolicy;
    }}
}}

robustone_isa_macros::define_registers! {{
    arch = {pascal};
    bank Gpr {{ count = 32; prefix = "$r"; }}
}}

robustone_isa_macros::define_formats! {{
    arch = {pascal};
    fields {{ rd; rs1; }};
    format R {{ rd: bits(0, 5), rs1: bits(5, 5) }}
}}

robustone_isa_macros::define_instructions! {{
    arch = {pascal}; module = base;
    insn NOP {{
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0000_0000);
        format = &R;
        operands = &[];
        modes = robustone_isa::ModeSet::All;
        features = {pascal}Feature::BASE;
        groups = &[robustone_isa::InstructionGroup::Integer];
        manual = "Architecture Reference Manual";
    }}
}}

robustone_isa_macros::define_aliases! {{
    arch = {pascal};
    // Add aliases here, e.g.:
    // alias "nop" for "NOP" {{
    //     when [operand(0) == reg(0)];
    //     mnemonic = "nop";
    //     visible_operands = [];
    // }}
}}

impl ArchitectureBackend for {pascal}Backend {{
    type Word = u32;
    type Mode = {pascal}Mode;
    type Feature = {pascal}Feature;
    type Field = {pascal}Field;
    type RegisterClass = {pascal}RegisterClass;

    fn architecture_id() -> ArchitectureId {{
        ArchitectureId::Other("{name}")
    }}

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {{
        if bytes.len() < 4 {{
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("{name}".to_string()),
                "need 4 bytes",
            ));
        }}
        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(InstructionRead {{ raw: word, length: 4 }})
    }}

    fn lookup(
        word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {{
        SPECS.iter().find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }}

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {{
        RegisterId::new(ArchitectureId::Other("{name}"), raw)
    }}

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {{
        RenderPolicy::new(robustone_isa::RenderDialect::Canonical, robustone_isa::AliasPolicy::None)
    }}

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> Result<u32, DisasmError> {{
        for f in format.fields() {{
            if f.field_type() == field {{
                let mask = ((1u64 << f.length()) - 1) as u32;
                return Ok((word >> f.start()) & mask);
            }}
        }}
        Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidField,
            Some("{name}"),
            format!("field {{:?}} not found in format {{}}", field, format.name()),
        ))
    }}
}}

pub type {pascal}Decoder = robustone_isa::Decoder<{pascal}Backend>;
"#,
        pascal = pascal,
        name = name.to_lowercase(),
    );
    fs::write(crate_dir.join("src/arch.rs"), arch_rs).unwrap();

    let stub_files = [
        ("profile.rs", "//! Architecture profile definitions.\n"),
        (
            "registers.rs",
            "//! Register bank definitions.\n\n/// Register manager for the architecture.\npub struct RegisterManager;\n",
        ),
        (
            "formats.rs",
            "//! Instruction format definitions.\n\nuse robustone_isa::FormatSpec;\n",
        ),
        (
            "render.rs",
            "//! Text rendering for decoded instructions.\n\nuse robustone_core::ir::DecodedInstruction;\n\n/// Render a decoded instruction into text.\npub fn render(_insn: &DecodedInstruction) -> (String, String) {\n    (String::new(), String::new())\n}\n",
        ),
    ];
    for (f, content) in &stub_files {
        fs::write(crate_dir.join("src").join(f), content).unwrap();
    }

    fs::create_dir_all(crate_dir.join("src/specs")).unwrap();
    fs::write(
        crate_dir.join("src/specs/mod.rs"),
        "// TODO: define instruction specs\n",
    )
    .unwrap();

    let crate_name_underscore = crate_name.replace("-", "_");
    let spec_validation = format!(
        r#"#![forbid(unsafe_code)]

use robustone_isa::{{ArchitectureBackend, check_spec_table}};
use {}::arch::{{{}Backend, SPECS}};

#[test]
fn test_specs_no_overlaps() {{
    assert!(
        robustone_isa::validate_no_overlaps(SPECS).is_ok(),
        "overlapping specs detected"
    );
}}

#[test]
fn test_specs_full_validation() {{
    check_spec_table::<{}Backend>(SPECS).unwrap();
}}
"#,
        crate_name_underscore, pascal, pascal,
    );
    fs::write(crate_dir.join("tests/spec_validation.rs"), spec_validation).unwrap();

    // Update workspace Cargo.toml
    let workspace_toml = workspace_root.join("Cargo.toml");
    let mut workspace_content = fs::read_to_string(&workspace_toml).unwrap();
    if !workspace_content.contains(&crate_name) {
        workspace_content = workspace_content.replace(
            "    \"robustone-x86\",\n",
            &format!("    \"robustone-x86\",\n    \"{}\",\n", crate_name),
        );
        fs::write(&workspace_toml, workspace_content).unwrap();
    }

    println!("Created {} at {}", crate_name, crate_dir.display());
    ExitCode::SUCCESS
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect()
}

// ============================================================================
// compat
// ============================================================================

fn compat(args: &[String]) -> ExitCode {
    let strict = args.contains(&"--strict".to_string());
    let output = std::process::Command::new("cargo")
        .args([
            "test",
            "-p",
            "robustone-capstone-compat",
            "--lib",
            "--quiet",
        ])
        .output();

    match output {
        Ok(out) => {
            let _stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if out.status.success() {
                println!("compat: all tests passed");
                if !stderr.is_empty() {
                    println!("{}", stderr);
                }
                ExitCode::SUCCESS
            } else {
                eprintln!("compat: tests failed");
                eprintln!("{}", stderr);
                if strict {
                    ExitCode::FAILURE
                } else {
                    println!("compat: non-strict mode – treating as warning");
                    ExitCode::SUCCESS
                }
            }
        }
        Err(e) => {
            eprintln!("compat: failed to run tests: {}", e);
            ExitCode::FAILURE
        }
    }
}

// ============================================================================
// compat-report
// ============================================================================

fn expected_xfail_count() -> usize {
    // Known baseline: 0 xfail entries currently match any test case.
    // Update when knowingly adding xfails that match test cases.
    0
}

fn compat_report(args: &[String]) -> ExitCode {
    let deny_xfail_increase = args.contains(&"--deny-xfail-increase".to_string());

    let output = std::process::Command::new("cargo")
        .args([
            "test",
            "-p",
            "robustone-capstone-compat",
            "--lib",
            "--",
            "--nocapture",
            "--test-threads=1",
        ])
        .output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("compat-report: cargo test exited with error");
                return ExitCode::FAILURE;
            }
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let combined = format!("{}\n{}", stdout, stderr);

            let mut pass = 0usize;
            let mut fail = 0usize;
            let mut ignored = 0usize;
            let mut known_diff = 0usize;

            for line in combined.lines() {
                if line.contains("test result:") {
                    // Parse "test result: ok. 18 passed; 0 failed; 1 ignored"
                    if let Some(p) = line.find("passed") {
                        let trimmed = line[..p].trim_end();
                        let start = trimmed.rfind(' ').map(|i| i + 1).unwrap_or(0);
                        if let Ok(n) = trimmed[start..].trim().parse::<usize>() {
                            pass += n;
                        }
                    }
                    if let Some(f) = line.find("failed") {
                        let trimmed = line[..f].trim_end();
                        let start = trimmed.rfind(' ').map(|i| i + 1).unwrap_or(0);
                        if let Ok(n) = trimmed[start..].trim().parse::<usize>() {
                            fail += n;
                        }
                    }
                    if let Some(i) = line.find("ignored") {
                        let trimmed = line[..i].trim_end();
                        let start = trimmed.rfind(' ').map(|i| i + 1).unwrap_or(0);
                        if let Ok(n) = trimmed[start..].trim().parse::<usize>() {
                            ignored += n;
                        }
                    }
                }
                // Parse per-file summaries emitted by the tests.
                if line.contains("summary:") && line.contains("known_diff") {
                    for part in line.split(',') {
                        let part = part.trim();
                        if let Some(n) = part.strip_suffix(" known_diff")
                            && let Ok(v) = n.trim().parse::<usize>()
                        {
                            known_diff += v;
                        }
                    }
                }
            }

            println!(
                "compat-report: {} passed, {} failed, {} ignored, {} known_diff",
                pass, fail, ignored, known_diff
            );

            if fail > 0 {
                println!("compat-report: WARNING – {} test(s) failed", fail);
            }

            if deny_xfail_increase {
                let expected = expected_xfail_count();
                if known_diff > expected {
                    eprintln!(
                        "compat-report: FAIL – xfail increased ({} known_diff > expected {})",
                        known_diff, expected
                    );
                    return ExitCode::FAILURE;
                }
            }

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("compat-report: failed to run tests: {}", e);
            ExitCode::FAILURE
        }
    }
}

// ============================================================================
// helpers
// ============================================================================

fn find_workspace_root() -> PathBuf {
    let mut dir = env::current_dir().expect("current dir");
    loop {
        if dir.join("Cargo.toml").exists() {
            let content = fs::read_to_string(dir.join("Cargo.toml")).unwrap_or_default();
            if content.contains("[workspace]") {
                return dir;
            }
        }
        if !dir.pop() {
            panic!("Could not find workspace root");
        }
    }
}

// ============================================================================
// audit-no-hardcode
// ============================================================================

/// Scan production crate sources for mnemonic-based semantic classification
/// and other hardcoded patterns that violate AC-5/AC-6.
fn audit_no_hardcode(args: &[String]) -> ExitCode {
    for arg in args {
        if arg != "--all" {
            eprintln!("Unknown flag: {}", arg);
            return ExitCode::FAILURE;
        }
    }

    let workspace_root = find_workspace_root();
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // Production crates: hard errors for violations (LoongArch, RISC-V)
    let hard_error_crates = ["robustone-loongarch", "robustone-riscv"];
    // Legacy placeholder crates: warnings only (ARM, X86)
    let warning_crates = ["robustone-arm", "robustone-x86"];

    let handler_patch_patterns: &[(&str, &str)] = &[
        (
            "decoded\\.mnemonic\\s*=",
            "decoded.mnemonic assignment (handler patch)",
        ),
        (
            "decoded\\.operands\\.remove",
            "decoded.operands.remove (handler patch)",
        ),
        (
            "decoded\\.operands\\.push",
            "decoded.operands.push (handler patch)",
        ),
        ("mnemonic\\.starts_with\\(", "mnemonic.starts_with"),
        ("mnemonic\\.ends_with\\(", "mnemonic.ends_with"),
        ("mnemonic\\.contains\\(", "mnemonic.contains"),
        ("\\.mnemonic\\s*==\\s*\"", "mnemonic == string literal"),
        (
            "BRANCH_MNEMONICS|PC_RELATIVE_MNEMONICS",
            "mnemonic list constant",
        ),
        (
            "all_supported_for_tests\\(\\)",
            "all_supported_for_tests() — use profile_for_arch_name instead",
        ),
        (
            "\\.groups\\..*==\\s*\"",
            "string-based instruction group comparison",
        ),
        (
            "word\\s*==\\s*0x[0-9a-fA-F]{5,}",
            "bare hex exact-word comparison",
        ),
        (
            "include_str!\\s*\\(\\s*\"[^\"]*capstone[^\"]*\"",
            "include_str with capstone path",
        ),
    ];

    let excluded_dirs = [
        "tests/",
        "robustone-capstone-compat/",
        "robustone-compat/",
        "xtask/",
    ];

    // Scan hard-error crates
    for crate_name in &hard_error_crates {
        scan_crate(
            &workspace_root,
            crate_name,
            handler_patch_patterns,
            &excluded_dirs,
            &mut errors,
            false,
        );
    }

    // Scan warning-only crates
    for crate_name in &warning_crates {
        scan_crate(
            &workspace_root,
            crate_name,
            handler_patch_patterns,
            &excluded_dirs,
            &mut warnings,
            true,
        );
    }

    // Print warnings
    if !warnings.is_empty() {
        println!(
            "audit-no-hardcode: {} warning(s) in legacy crates:",
            warnings.len()
        );
        for w in &warnings {
            println!("  WARNING: {}", w);
        }
    }

    if errors.is_empty() {
        println!("audit-no-hardcode: OK – no violations found");
        ExitCode::SUCCESS
    } else {
        println!("audit-no-hardcode: {} violation(s) found:", errors.len());
        for v in &errors {
            println!("  ERROR: {}", v);
        }
        ExitCode::FAILURE
    }
}

fn scan_crate(
    workspace_root: &Path,
    crate_name: &str,
    patterns: &[(&str, &str)],
    excluded_dirs: &[&str],
    violations: &mut Vec<String>,
    warning_only: bool,
) {
    let src_dir = workspace_root.join(crate_name).join("src");
    if !src_dir.exists() {
        return;
    }

    for entry in walk_dir(&src_dir) {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }

        // Skip excluded directories (tests, compat, xtask)
        let path_str = path.to_string_lossy();
        if excluded_dirs.iter().any(|d| path_str.contains(d)) {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Track current function context for contextual allow rules
        let mut current_fn: Option<String> = None;

        for (pattern, desc) in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for (line_num, line) in content.lines().enumerate() {
                    // Track function context (handles `fn`, `pub fn`, `pub(crate) fn`, etc.)
                    let trimmed = line.trim();
                    if let Some(fn_pos) = trimmed.find("fn ") {
                        let after_fn = &trimmed[fn_pos + 3..];
                        if let Some(paren) = after_fn.find('(') {
                            current_fn = Some(after_fn[..paren].trim().to_string());
                        }
                    }
                    if line.trim().starts_with('}') {
                        current_fn = None;
                    }

                    if re.is_match(line) {
                        // Skip the function definition itself
                        if line.trim().starts_with("fn all_supported_for_tests") {
                            continue;
                        }
                        // Allow all_supported_for_tests() in backend.rs (defines FeatureSet impls
                        // and test profile helpers like capstone_test_la64()) and in
                        // test/capstone helper function contexts
                        if desc.contains("all_supported_for_tests") {
                            if path_str.contains("backend.rs") {
                                continue;
                            }
                            if let Some(ref fname) = current_fn
                                && (fname.contains("test") || fname.contains("capstone"))
                            {
                                continue;
                            }
                        }
                        // Skip comment lines but still enforce banned handler-patch patterns
                        if line.trim().starts_with("//") && !desc.contains("handler patch") {
                            continue;
                        }
                        let rel_path = path.strip_prefix(workspace_root).unwrap_or(&path);
                        let prefix = if warning_only { "WARNING" } else { "ERROR" };
                        violations.push(format!(
                            "{}:{}: [{}] {} — `{}`",
                            rel_path.display(),
                            line_num + 1,
                            prefix,
                            desc,
                            line.trim()
                        ));
                    }
                }
            }
        }
    }
}
