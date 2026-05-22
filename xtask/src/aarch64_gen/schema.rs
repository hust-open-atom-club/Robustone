//! AArch64 normalized operand schema analysis.

use std::collections::{BTreeMap, BTreeSet};

use crate::aarch64_gen::model::{
    InstructionFamily, NormalizationDiagnostic, NormalizedCatalog, NormalizedInstruction,
    SchemaBucket, SchemaBuckets, SchemaId,
};

pub(crate) fn schema_id_for(instruction: &NormalizedInstruction) -> SchemaId {
    let parts = instruction
        .operands
        .iter()
        .map(|operand| {
            format!(
                "{}:{}:{}:{}",
                serde_json::to_string(&operand.kind).expect("operand kind should serialize"),
                serde_json::to_string(&operand.role).expect("operand role should serialize"),
                operand.optional,
                operand.raw_class.to_ascii_lowercase()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    SchemaId(format!("schema:{parts}"))
}

pub(crate) fn build_schema_buckets(catalog: &NormalizedCatalog) -> SchemaBuckets {
    let mut buckets: BTreeMap<SchemaId, BucketBuilder> = BTreeMap::new();

    for instruction in &catalog.instructions {
        let schema_id = schema_id_for(instruction);
        buckets
            .entry(schema_id.clone())
            .or_insert_with(|| BucketBuilder::new(schema_id))
            .push(instruction);
    }

    SchemaBuckets {
        buckets: buckets.into_values().map(BucketBuilder::finish).collect(),
    }
}

struct BucketBuilder {
    schema_id: SchemaId,
    count: usize,
    families: BTreeSet<InstructionFamily>,
    examples: Vec<String>,
    blocked_diagnostics: BTreeSet<NormalizationDiagnostic>,
}

impl BucketBuilder {
    fn new(schema_id: SchemaId) -> Self {
        Self {
            schema_id,
            count: 0,
            families: BTreeSet::new(),
            examples: Vec::new(),
            blocked_diagnostics: BTreeSet::new(),
        }
    }

    fn push(&mut self, instruction: &NormalizedInstruction) {
        self.count += 1;
        self.families.insert(instruction.family.clone());
        if self.examples.len() < 5 {
            self.examples.push(instruction.opcode_id.clone());
        }
        if let Some(diagnostic) = &instruction.diagnostic {
            self.blocked_diagnostics.insert(diagnostic.clone());
        }
    }

    fn finish(self) -> SchemaBucket {
        SchemaBucket {
            schema_id: self.schema_id,
            count: self.count,
            families: self.families.into_iter().collect(),
            examples: self.examples,
            blocked_diagnostics: self.blocked_diagnostics.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64_gen::{normalize, tblgen_json};

    #[test]
    fn assigns_same_schema_to_equivalent_operand_sequences() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);

        let first = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.opcode_id == "PSEUDORET")
            .expect("PSEUDORET should exist");
        let second = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.opcode_id == "BADNAMESPACE")
            .expect("BADNAMESPACE should exist");

        assert_eq!(schema_id_for(first), schema_id_for(second));
    }

    #[test]
    fn schema_buckets_include_counts_families_examples_and_blockers() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);

        let buckets = build_schema_buckets(&catalog);

        assert!(buckets.buckets.iter().any(|bucket| {
            bucket.count >= 2
                && bucket.examples.contains(&"PSEUDORET".to_string())
                && bucket.examples.contains(&"BADNAMESPACE".to_string())
                && bucket.families.contains(&InstructionFamily::Branch)
                && bucket.families.contains(&InstructionFamily::Unknown)
        }));
        assert!(
            buckets
                .buckets
                .iter()
                .any(|bucket| !bucket.blocked_diagnostics.is_empty())
        );
    }
}
