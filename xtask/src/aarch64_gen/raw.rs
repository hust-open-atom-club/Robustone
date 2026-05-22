//! Raw AArch64 TableGen JSON ingest stage.

use crate::aarch64_gen::model::InstructionRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawTableGenDb {
    pub instructions: Vec<InstructionRecord>,
}

pub(crate) fn parse_raw_tablegen_db(json: &str) -> Result<RawTableGenDb, String> {
    let instructions = crate::aarch64_gen::tblgen_json::parse_records(json)?;
    Ok(RawTableGenDb { instructions })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_raw_tablegen_db_without_lowering_decisions() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");

        let db = parse_raw_tablegen_db(json).expect("fixture should parse");

        assert_eq!(db.instructions.len(), 9);
        assert!(
            db.instructions
                .iter()
                .any(|record| record.llvm_name == "ADDWri")
        );
        assert!(
            db.instructions
                .iter()
                .any(|record| record.llvm_name == "Bcc")
        );
    }
}
