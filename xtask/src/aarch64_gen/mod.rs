//! AArch64 generator command skeleton.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[allow(dead_code)]
mod emit;
#[allow(dead_code)]
mod lowering;
#[allow(dead_code)]
mod model;
#[allow(dead_code)]
mod normalize;
#[allow(dead_code)]
mod provenance;
#[allow(dead_code)]
mod raw;
#[allow(dead_code)]
mod report;
#[allow(dead_code)]
mod schema;
#[allow(dead_code)]
mod tblgen_json;

const STABLE_AARCH64_TABLEGEN_SOURCE: &str =
    "third_party/llvm-project/llvm/lib/Target/AArch64/AArch64.td.json";

#[derive(Debug, PartialEq, Eq)]
struct GenArgs {
    llvm_project: PathBuf,
    out_dir: PathBuf,
    artifact_dir: PathBuf,
    check: bool,
}

fn parse_args(args: &[String], check: bool) -> Result<GenArgs, String> {
    let mut llvm_project = PathBuf::from("third_party/llvm-project");
    let mut out_dir = PathBuf::from("robustone-arm/src/backend/generated");
    let mut artifact_dir = PathBuf::from("target/aarch64-gen");

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--llvm-project" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "missing value for --llvm-project".to_string())?;
                llvm_project = PathBuf::from(value);
            }
            "--out-dir" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "missing value for --out-dir".to_string())?;
                out_dir = PathBuf::from(value);
            }
            "--artifact-dir" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "missing value for --artifact-dir".to_string())?;
                artifact_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                return Err(
                    "usage: cargo xtask aarch64-gen [--llvm-project <path>] [--out-dir <path>] [--artifact-dir <path>]"
                        .to_string(),
                );
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(GenArgs {
        llvm_project,
        out_dir,
        artifact_dir,
        check,
    })
}

/// Generate AArch64 specs from TableGen JSON.
pub fn generate(args: &[String]) -> ExitCode {
    match run(args, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("aarch64-gen: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Check generated AArch64 specs without writing changes.
pub fn check(args: &[String]) -> ExitCode {
    match run(args, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("aarch64-gen-check: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String], check: bool) -> Result<(), String> {
    let args = parse_args(args, check)?;
    let workspace_root = find_workspace_root()?;
    let llvm_project = resolve_path(&workspace_root, &args.llvm_project);
    let out_dir = resolve_path(&workspace_root, &args.out_dir);
    let artifact_dir = resolve_path(&workspace_root, &args.artifact_dir);
    let json_path = llvm_project.join("llvm/lib/Target/AArch64/AArch64.td.json");
    let json = fs::read_to_string(&json_path)
        .map_err(|err| format!("failed to read {}: {err}", json_path.display()))?;
    let records = tblgen_json::parse_records(&json)?;
    let raw_db = raw::parse_raw_tablegen_db(&json)?;
    let normalized_catalog = normalize::normalize_records(&raw_db.instructions);
    let schema_buckets = schema::build_schema_buckets(&normalized_catalog);
    let (lowered_catalog, lowering_report) =
        lowering::LoweringRegistry::default().lower_catalog(&normalized_catalog);
    let report = report::CoverageReport::from_records(&records);
    let metadata = build_header_metadata(&json_path, &args)?;
    let files = emit::emit_specs_with_metadata(&records, &metadata)?;
    let expected_names: std::collections::BTreeSet<_> = files
        .iter()
        .map(|(filename, _)| filename.as_str())
        .collect();

    if !args.check {
        fs::create_dir_all(&out_dir)
            .map_err(|err| format!("failed to create {}: {err}", out_dir.display()))?;
        remove_stale_generated_files(&out_dir, &expected_names)?;
        let artifact_inputs = ArtifactInputs {
            metadata: &metadata,
            records: &records,
            report: &report,
            normalized_catalog: &normalized_catalog,
            schema_buckets: &schema_buckets,
            lowered_catalog: &lowered_catalog,
            lowering_report: &lowering_report,
        };
        write_artifacts(&artifact_dir, artifact_inputs)?;
    }

    let mut mismatches = Vec::new();
    for (filename, content) in files {
        let path = out_dir.join(&filename);
        if args.check {
            let existing = fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            if existing != content {
                mismatches.push(path.display().to_string());
            }
        } else {
            write_atomically(&path, &content)?;
        }
    }

    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "generated AArch64 specs are out of date: {}",
            mismatches.join(", ")
        ))
    }
}

fn build_header_metadata(json_path: &Path, args: &GenArgs) -> Result<emit::HeaderMetadata, String> {
    Ok(emit::HeaderMetadata {
        source: STABLE_AARCH64_TABLEGEN_SOURCE.to_string(),
        source_hash: provenance::source_hash(json_path)?,
        command: generation_command(args),
    })
}

fn generation_command(args: &GenArgs) -> String {
    format!(
        "cargo xtask aarch64-gen --llvm-project {} --out-dir {} --artifact-dir {}",
        args.llvm_project.display(),
        args.out_dir.display(),
        args.artifact_dir.display()
    )
}

struct ArtifactInputs<'a> {
    metadata: &'a emit::HeaderMetadata,
    records: &'a [model::InstructionRecord],
    report: &'a report::CoverageReport,
    normalized_catalog: &'a model::NormalizedCatalog,
    schema_buckets: &'a model::SchemaBuckets,
    lowered_catalog: &'a model::LoweredCatalog,
    lowering_report: &'a model::LoweringReport,
}

fn write_artifacts(artifact_dir: &Path, inputs: ArtifactInputs<'_>) -> Result<(), String> {
    fs::create_dir_all(artifact_dir)
        .map_err(|err| format!("failed to create {}: {err}", artifact_dir.display()))?;

    let catalog_json = serde_json::to_string_pretty(inputs.records)
        .map_err(|err| format!("failed to serialize catalog.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("catalog.json"),
        &format!("{catalog_json}\n"),
    )?;

    let report_json = serde_json::to_string_pretty(inputs.report)
        .map_err(|err| format!("failed to serialize report.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("report.json"),
        &format!("{report_json}\n"),
    )?;
    write_atomically(
        &artifact_dir.join("report.md"),
        &inputs.report.to_markdown(),
    )?;

    let normalized_json = serde_json::to_string_pretty(inputs.normalized_catalog)
        .map_err(|err| format!("failed to serialize normalized_catalog.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("normalized_catalog.json"),
        &format!("{normalized_json}\n"),
    )?;

    let schema_json = serde_json::to_string_pretty(inputs.schema_buckets)
        .map_err(|err| format!("failed to serialize schema_buckets.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("schema_buckets.json"),
        &format!("{schema_json}\n"),
    )?;

    let lowered_catalog_json = serde_json::to_string_pretty(inputs.lowered_catalog)
        .map_err(|err| format!("failed to serialize lowered_catalog.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("lowered_catalog.json"),
        &format!("{lowered_catalog_json}\n"),
    )?;

    let lowering_json = serde_json::to_string_pretty(inputs.lowering_report)
        .map_err(|err| format!("failed to serialize lowering_report.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("lowering_report.json"),
        &format!("{lowering_json}\n"),
    )?;

    let provenance_json = serde_json::json!({
        "source": inputs.metadata.source,
        "source_hash": inputs.metadata.source_hash,
        "command": inputs.metadata.command,
    });
    let provenance_json = serde_json::to_string_pretty(&provenance_json)
        .map_err(|err| format!("failed to serialize provenance.json: {err}"))?;
    write_atomically(
        &artifact_dir.join("provenance.json"),
        &format!("{provenance_json}\n"),
    )?;

    Ok(())
}

fn resolve_path(workspace_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_root.join(path)
    }
}

fn find_workspace_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|err| format!("current dir: {err}"))?;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            let content = fs::read_to_string(&manifest)
                .map_err(|err| format!("failed to read {}: {err}", manifest.display()))?;
            if content.contains("[workspace]") {
                return Ok(dir);
            }
        }
        if !dir.pop() {
            return Err("could not find workspace root".to_string());
        }
    }
}

fn remove_stale_generated_files(
    out_dir: &Path,
    expected_names: &std::collections::BTreeSet<&str>,
) -> Result<(), String> {
    if !out_dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(out_dir)
        .map_err(|err| format!("failed to read {}: {err}", out_dir.display()))?
    {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if expected_names.contains(name) {
            continue;
        }
        let content = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        if content.contains("@generated by xtask aarch64-gen") {
            fs::remove_file(&path)
                .map_err(|err| format!("failed to remove stale {}: {err}", path.display()))?;
        }
    }
    Ok(())
}

fn write_atomically(path: &Path, content: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("path has no parent: {}", path.display()))?;
    let tmp = parent.join(format!(
        ".{}.tmp",
        path.file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("path has invalid file name: {}", path.display()))?
    ));
    {
        let mut file = File::create(&tmp)
            .map_err(|err| format!("failed to create {}: {err}", tmp.display()))?;
        file.write_all(content.as_bytes())
            .map_err(|err| format!("failed to write {}: {err}", tmp.display()))?;
        file.sync_all()
            .map_err(|err| format!("failed to sync {}: {err}", tmp.display()))?;
    }
    fs::rename(&tmp, path).map_err(|err| {
        let _ = fs::remove_file(&tmp);
        format!(
            "failed to rename {} to {}: {err}",
            tmp.display(),
            path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(args: &[&str]) -> Vec<String> {
        args.iter().map(|arg| (*arg).to_string()).collect()
    }

    #[test]
    fn parse_args_accepts_explicit_paths_for_generate() {
        let args = parse_args(
            &strings(&[
                "--llvm-project",
                "third_party/llvm-project",
                "--out-dir",
                "robustone-arm/src/backend/generated",
            ]),
            false,
        )
        .expect("args should parse");

        assert_eq!(args.llvm_project, PathBuf::from("third_party/llvm-project"));
        assert_eq!(
            args.out_dir,
            PathBuf::from("robustone-arm/src/backend/generated")
        );
        assert_eq!(args.artifact_dir, PathBuf::from("target/aarch64-gen"));
        assert!(!args.check);
    }

    #[test]
    fn parse_args_accepts_explicit_artifact_dir() {
        let args = parse_args(
            &strings(&[
                "--llvm-project",
                "third_party/llvm-project",
                "--out-dir",
                "robustone-arm/src/backend/generated",
                "--artifact-dir",
                "custom/aarch64-artifacts",
            ]),
            false,
        )
        .expect("args should parse");

        assert_eq!(args.artifact_dir, PathBuf::from("custom/aarch64-artifacts"));
    }

    #[test]
    fn generation_command_includes_stable_artifact_dir_metadata() {
        let args = GenArgs {
            llvm_project: PathBuf::from("custom/llvm-project"),
            out_dir: PathBuf::from("custom/generated"),
            artifact_dir: PathBuf::from("custom/artifacts"),
            check: false,
        };

        assert_eq!(
            generation_command(&args),
            "cargo xtask aarch64-gen --llvm-project custom/llvm-project --out-dir custom/generated --artifact-dir custom/artifacts"
        );
    }

    #[test]
    fn parse_args_sets_check_mode() {
        let args = parse_args(
            &strings(&[
                "--llvm-project",
                "third_party/llvm-project",
                "--out-dir",
                "robustone-arm/src/backend/generated",
            ]),
            true,
        )
        .expect("args should parse");

        assert!(args.check);
    }

    #[test]
    fn builds_header_metadata_from_actual_json_path_hash_and_command() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask manifest should have workspace parent");
        let json_path = workspace_root.join("xtask/tests/fixtures/aarch64_tblgen_subset.json");
        let args = GenArgs {
            llvm_project: PathBuf::from("custom/llvm-project"),
            out_dir: PathBuf::from("custom/generated"),
            artifact_dir: PathBuf::from("custom/artifacts"),
            check: false,
        };

        let metadata = build_header_metadata(&json_path, &args).expect("metadata should build");

        assert_eq!(
            metadata.source,
            "third_party/llvm-project/llvm/lib/Target/AArch64/AArch64.td.json"
        );
        assert_eq!(
            metadata.source_hash,
            provenance::source_hash(&json_path).expect("fixture should hash")
        );
        assert_eq!(
            metadata.command,
            "cargo xtask aarch64-gen --llvm-project custom/llvm-project --out-dir custom/generated --artifact-dir custom/artifacts"
        );
    }

    #[test]
    fn remove_stale_generated_files_deletes_current_generated_headers() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let stale_path = temp.path().join("simd_fp.rs");
        fs::write(
            &stale_path,
            "// Generated AArch64 instruction specs.\n//\n// @generated by xtask aarch64-gen. Do not edit by hand.\n",
        )
        .expect("stale generated file should be written");
        let expected_names = std::collections::BTreeSet::from(["mod.rs"]);

        remove_stale_generated_files(temp.path(), &expected_names)
            .expect("stale generated files should be removed");

        assert!(!stale_path.exists());
    }

    #[test]
    fn write_artifacts_emits_catalog_report_json_markdown_and_lowered_catalog_sidecars() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let raw_db = raw::parse_raw_tablegen_db(json).expect("raw fixture should parse");
        let normalized_catalog = normalize::normalize_records(&raw_db.instructions);
        let schema_buckets = schema::build_schema_buckets(&normalized_catalog);
        let (lowered_catalog, lowering_report) =
            lowering::LoweringRegistry::default().lower_catalog(&normalized_catalog);
        let report = report::CoverageReport::from_records(&records);
        let artifact_dir = std::env::temp_dir().join(format!(
            "robustone-aarch64-gen-artifacts-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("unnamed")
        ));
        if artifact_dir.exists() {
            fs::remove_dir_all(&artifact_dir).expect("stale temp artifact dir should be removable");
        }

        let metadata = emit::HeaderMetadata {
            source: STABLE_AARCH64_TABLEGEN_SOURCE.to_string(),
            source_hash: "test-hash".to_string(),
            command: "cargo xtask aarch64-gen --llvm-project custom/llvm-project --out-dir custom/generated --artifact-dir custom/artifacts".to_string(),
        };
        let inputs = ArtifactInputs {
            metadata: &metadata,
            records: &records,
            report: &report,
            normalized_catalog: &normalized_catalog,
            schema_buckets: &schema_buckets,
            lowered_catalog: &lowered_catalog,
            lowering_report: &lowering_report,
        };
        write_artifacts(&artifact_dir, inputs).expect("artifacts should write");

        let catalog =
            fs::read_to_string(artifact_dir.join("catalog.json")).expect("catalog exists");
        let report_json =
            fs::read_to_string(artifact_dir.join("report.json")).expect("report exists");
        let report_md =
            fs::read_to_string(artifact_dir.join("report.md")).expect("markdown exists");
        let normalized_json = fs::read_to_string(artifact_dir.join("normalized_catalog.json"))
            .expect("normalized catalog exists");
        let schema_json = fs::read_to_string(artifact_dir.join("schema_buckets.json"))
            .expect("schema buckets exists");
        let lowered_catalog_json = fs::read_to_string(artifact_dir.join("lowered_catalog.json"))
            .expect("lowered catalog exists");
        let lowering_json = fs::read_to_string(artifact_dir.join("lowering_report.json"))
            .expect("lowering report exists");
        let provenance_json = fs::read_to_string(artifact_dir.join("provenance.json"))
            .expect("provenance sidecar exists");
        fs::remove_dir_all(&artifact_dir).expect("temp artifact dir should be removable");

        let normalized_value: serde_json::Value = serde_json::from_str(&normalized_json)
            .expect("normalized catalog should be valid JSON");
        let schema_value: serde_json::Value =
            serde_json::from_str(&schema_json).expect("schema buckets should be valid JSON");
        let lowered_catalog_value: serde_json::Value = serde_json::from_str(&lowered_catalog_json)
            .expect("lowered catalog should be valid JSON");
        let lowering_value: serde_json::Value =
            serde_json::from_str(&lowering_json).expect("lowering report should be valid JSON");

        let provenance_value: serde_json::Value = serde_json::from_str(&provenance_json)
            .expect("provenance sidecar should be valid JSON");

        assert_eq!(
            provenance_value["source"],
            "third_party/llvm-project/llvm/lib/Target/AArch64/AArch64.td.json"
        );
        assert_eq!(
            provenance_value["command"],
            "cargo xtask aarch64-gen --llvm-project custom/llvm-project --out-dir custom/generated --artifact-dir custom/artifacts"
        );
        assert!(catalog.contains("\n  {"));
        assert!(catalog.contains("\"llvm_name\": \"ADDWri\""));
        assert!(report_json.contains("\"total_records\": 9"));
        assert!(report_json.contains("\"by_family\""));
        assert!(report_md.contains("# AArch64 TableGen Coverage Report"));
        assert!(report_md.contains("Total records: 9"));
        assert_eq!(
            normalized_value["instructions"]
                .as_array()
                .expect("instructions array")
                .len(),
            9
        );
        assert_eq!(normalized_value["instructions"][0]["stage"], "normalized");
        assert!(
            schema_value["buckets"]
                .as_array()
                .expect("buckets array")
                .len()
                > 1
        );
        assert!(schema_json.contains("\"schema_id\""));
        assert!(
            lowered_catalog_value["opcode_ids"]
                .as_array()
                .expect("lowered opcode id array")
                .is_empty()
        );
        assert!(
            lowered_catalog_value["probes"]
                .as_array()
                .expect("lowered probe array")
                .is_empty()
        );
        assert_eq!(lowering_value["lowered_records"], 0);
        assert_eq!(lowering_value["blocked_records"], 9);
        assert!(
            lowering_value["blocked_buckets"]
                .as_array()
                .expect("blocked buckets array")
                .iter()
                .any(|bucket| bucket["reason"] == "unsupported_lowering")
        );
    }

    #[test]
    fn parse_args_rejects_missing_value() {
        let err = parse_args(&strings(&["--llvm-project"]), false).expect_err("missing value");

        assert_eq!(err, "missing value for --llvm-project");
    }

    #[test]
    fn parse_args_rejects_unknown_argument() {
        let err = parse_args(&strings(&["--bogus"]), false).expect_err("unknown argument");

        assert_eq!(err, "unknown argument: --bogus");
    }
}
