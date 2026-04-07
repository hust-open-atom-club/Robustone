use std::fs;
use std::path::PathBuf;

use robustone_core::all_architecture_capabilities;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

#[test]
fn support_matrix_tracks_architecture_capabilities() {
    let support_matrix_path = repo_root().join("docs").join("support-matrix.md");
    let content = fs::read_to_string(&support_matrix_path).expect("support matrix should exist");
    let documented_rows = architecture_table_rows(&content);
    let expected_rows: Vec<String> = all_architecture_capabilities()
        .iter()
        .map(|capability| {
            format!(
                "| `{}` | `{}` | {} | {} | {} | {} |",
                capability.canonical_name,
                capability.category,
                yes_no(capability.parse_supported),
                yes_no(capability.decode_supported),
                yes_no(capability.detail_supported),
                yes_no(capability.json_supported),
            )
        })
        .collect();

    assert_eq!(documented_rows, expected_rows);
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn architecture_table_rows(content: &str) -> Vec<String> {
    let mut in_table = false;
    let mut rows = Vec::new();

    for line in content.lines() {
        if line == "## Architecture Capability Registry" {
            in_table = true;
            continue;
        }

        if in_table && line.starts_with("## ") {
            break;
        }

        if in_table && line.starts_with("| `") {
            rows.push(line.to_string());
        }
    }

    rows
}
