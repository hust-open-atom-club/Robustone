//! Normalized AArch64 instruction catalog stage.

use crate::aarch64_gen::model::{
    InstructionRecord, NormalizationDiagnostic, NormalizationStage, NormalizedCatalog,
    NormalizedInstruction, NormalizedOperand, SkipReason,
};

pub(crate) fn normalize_records(records: &[InstructionRecord]) -> NormalizedCatalog {
    NormalizedCatalog {
        instructions: records.iter().map(normalize_record).collect(),
    }
}

fn normalize_record(record: &InstructionRecord) -> NormalizedInstruction {
    let diagnostic = record.skip_reason.as_ref().map(normalization_diagnostic);
    NormalizedInstruction {
        llvm_name: record.llvm_name.clone(),
        opcode_id: record.opcode_id.clone(),
        mnemonic: record.mnemonic.clone(),
        family: record.family.clone(),
        groups: record.groups.clone(),
        operands: record
            .operands
            .iter()
            .map(|operand| NormalizedOperand {
                name: operand.name.clone(),
                kind: operand.kind.clone(),
                role: operand.role.clone(),
                optional: operand.optional,
                raw_class: operand.raw_class.clone(),
            })
            .collect(),
        encoding_mask: record.encoding_mask,
        encoding_value: record.encoding_value,
        features: record.features.clone(),
        stage: if diagnostic.is_some() {
            NormalizationStage::Blocked
        } else {
            NormalizationStage::Normalized
        },
        diagnostic,
    }
}

fn normalization_diagnostic(reason: &SkipReason) -> NormalizationDiagnostic {
    match reason {
        SkipReason::Pseudo => NormalizationDiagnostic::Pseudo,
        SkipReason::Alias => NormalizationDiagnostic::Alias,
        SkipReason::MissingEncoding => NormalizationDiagnostic::MissingEncoding,
        SkipReason::UnknownOperand => NormalizationDiagnostic::UnknownOperand,
        SkipReason::UnsupportedOperandSchema => NormalizationDiagnostic::UnsupportedOperandSchema,
        SkipReason::MalformedOperand => NormalizationDiagnostic::MalformedOperand,
        SkipReason::UnsupportedFeature => NormalizationDiagnostic::UnsupportedFeature,
        SkipReason::UnsupportedNamespace => NormalizationDiagnostic::UnsupportedNamespace,
        SkipReason::AmbiguousEncoding => NormalizationDiagnostic::AmbiguousEncoding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64_gen::tblgen_json;

    #[test]
    fn normalizes_instruction_records_into_fact_catalog() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");

        let catalog = normalize_records(&records);

        assert_eq!(catalog.instructions.len(), 9);
        let add = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.opcode_id == "ADDWRI")
            .expect("ADDWRI should be normalized");
        assert_eq!(add.mnemonic, "add");
        assert_eq!(add.encoding_mask, Some(0xffc0_0000));
        assert_eq!(add.encoding_value, Some(0x1100_0000));
        assert_eq!(add.stage, NormalizationStage::Normalized);
        assert_eq!(add.diagnostic, None);
        assert_eq!(add.operands.len(), 3);
    }

    #[test]
    fn blocked_records_keep_stage_diagnostics() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");

        let catalog = normalize_records(&records);

        let pseudo = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.diagnostic == Some(NormalizationDiagnostic::Pseudo))
            .expect("fixture should contain pseudo diagnostic");
        assert_eq!(pseudo.stage, NormalizationStage::Blocked);
    }
}
