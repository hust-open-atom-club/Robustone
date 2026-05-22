//! AArch64 generator provenance helpers.

use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};

pub(crate) fn source_hash(path: &Path) -> Result<String, String> {
    let bytes =
        fs::read(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_existing_fixture() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask manifest should have workspace parent");
        let fixture = workspace_root.join("xtask/tests/fixtures/aarch64_tblgen_subset.json");
        let hash = source_hash(&fixture).expect("fixture should hash successfully");

        assert_eq!(hash.len(), 64);
    }
}
