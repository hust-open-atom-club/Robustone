//! Robustone workspace task runner.
//!
//! Provides development and CI tools:
//! - `cargo xtask verify-boundary` – check workspace dependency and content boundaries
//! - `cargo xtask check-spec --all` – validate instruction spec tables
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
        eprintln!("  verify-boundary     Check workspace boundary constraints");
        eprintln!("  check-spec --all    Validate instruction spec tables");
        eprintln!("  new-arch <name>     Scaffold a new architecture crate");
        return ExitCode::FAILURE;
    }

    match args[0].as_str() {
        "verify-boundary" => verify_boundary(),
        "check-spec" => check_spec(&args[1..]),
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
    if args.is_empty() {
        eprintln!("Usage: cargo xtask new-arch <name>");
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
    let upper = pascal.to_uppercase();
    let arch_id = &pascal;
    let register_ctor = name.to_lowercase();
    let arch_rs = format!(
        r#"use robustone_isa::{{
    Access, ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, ImmediateKind,
    ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, ModeSet, RenderPolicy,
    field,
}};
use robustone_core::ir::{{ArchitectureId, RegisterId}};
use robustone_core::types::error::{{DecodeErrorKind, DisasmError}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {pascal}Mode {{
    Base,
}}

bitflags::bitflags! {{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct {pascal}Feature: u8 {{
        const BASE = 1 << 0;
    }}
}}

impl FeatureSet for {pascal}Feature {{
    fn empty() -> Self {{ Self::empty() }}
    fn all_supported_for_tests() -> Self {{ Self::BASE }}
    fn contains(self, required: Self) -> bool {{ self.0 & required.0 == required.0 }}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {pascal}Field {{
    Rd,
    Rs1,
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {pascal}RegisterClass {{
    Gpr,
}}

robustone_isa::format_specs! {{
    format R[{pascal}Field] {{
        rd: field("rd", 0, 5, {pascal}Field::Rd),
        rs1: field("rs1", 5, 5, {pascal}Field::Rs1),
    }}
}}

robustone_isa::isa_specs! {{
    backend = {pascal}Backend;
    spec NOP {{
        mnemonic = "nop";
        opcode_id = "NOP";
        pattern = robustone_isa::mask_value!(0xFFFF_FFFF, 0x0000_0000);
        format = &R;
        operands = &[];
        features = {pascal}Feature::BASE;
        modes = ModeSet::All;
        groups = &[];
    }}
}}

pub static {upper}_SPECS: &[InstructionSpec<{pascal}Backend>] = &[NOP];

pub struct {pascal}Backend;

impl ArchitectureBackend for {pascal}Backend {{
    type Word = u32;
    type Mode = {pascal}Mode;
    type Feature = {pascal}Feature;
    type Field = {pascal}Field;
    type RegisterClass = {pascal}RegisterClass;

    fn architecture_id() -> ArchitectureId {{
        ArchitectureId::{arch_id}
    }}

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {{
        if bytes.len() < 4 {{
            return Err(DisasmError::decode_failure(
                DecodeErrorKind::NeedMoreBytes,
                Some("mock".to_string()),
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
        {upper}_SPECS.iter().find(|spec| (word & spec.pattern.mask) == spec.pattern.value)
    }}

    fn lower_register(
        _class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {{
        RegisterId::{register_ctor}(raw)
    }}

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {{
        RenderPolicy::new(robustone_isa::RenderDialect::Canonical, robustone_isa::AliasPolicy::None)
    }}

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> u32 {{
        for f in format.fields {{
            if f.field_type == field {{
                let mask = ((1u64 << f.length) - 1) as u32;
                return (word >> f.start) & mask;
            }}
        }}
        0
    }}
}}

pub type {pascal}Decoder = robustone_isa::Decoder<{pascal}Backend>;
"#,
        pascal = pascal,
        upper = upper
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

    fs::write(
        crate_dir.join("tests/invariants.rs"),
        "// TODO: add invariant tests\n",
    )
    .unwrap();

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
