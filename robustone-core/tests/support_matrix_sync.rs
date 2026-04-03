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

    for capability in all_architecture_capabilities() {
        let row = format!(
            "| `{}` | `{}` | {} | {} | {} | {} |",
            capability.canonical_name,
            capability.category,
            yes_no(capability.parse_supported),
            yes_no(capability.decode_supported),
            yes_no(capability.detail_supported),
            yes_no(capability.json_supported),
        );
        assert!(
            content.contains(&row),
            "support matrix row missing for {}",
            capability.canonical_name
        );
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
