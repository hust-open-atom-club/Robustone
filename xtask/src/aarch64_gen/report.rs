//! AArch64 TableGen coverage reporting.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::aarch64_gen::model::{InstructionFamily, InstructionRecord, SkipReason};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CoverageReport {
    pub total_records: usize,
    pub active_records: usize,
    pub skipped_records: usize,
    pub by_family: BTreeMap<String, FamilyCoverage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FamilyCoverage {
    pub family: String,
    pub total_records: usize,
    pub active_records: usize,
    pub skipped_records: usize,
    skip_reasons: BTreeMap<SkipReason, usize>,
}

impl CoverageReport {
    pub(crate) fn from_records(records: &[InstructionRecord]) -> Self {
        let mut report = Self {
            total_records: 0,
            active_records: 0,
            skipped_records: 0,
            by_family: BTreeMap::new(),
        };

        for record in records {
            report.total_records += 1;
            if record.active {
                report.active_records += 1;
            } else {
                report.skipped_records += 1;
            }

            let family = family_name(&record.family).to_string();
            report
                .by_family
                .entry(family.clone())
                .or_insert_with(|| FamilyCoverage::new(family))
                .record(record);
        }

        report
    }

    pub(crate) fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# AArch64 TableGen Coverage Report\n\n");
        markdown.push_str(&format!("Total records: {}\n", self.total_records));
        markdown.push_str(&format!("Active records: {}\n", self.active_records));
        markdown.push_str(&format!("Skipped records: {}\n\n", self.skipped_records));
        markdown.push_str("| Family | Total | Active | Skipped | Pseudo | Alias | Missing encoding | Unknown operand | Unsupported operand schema | Malformed operand | Unsupported feature | Unsupported namespace | Ambiguous encoding |\n");
        markdown.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");

        for family in self.by_family.values() {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                family.family,
                family.total_records,
                family.active_records,
                family.skipped_records,
                family.skip_count(SkipReason::Pseudo),
                family.skip_count(SkipReason::Alias),
                family.skip_count(SkipReason::MissingEncoding),
                family.skip_count(SkipReason::UnknownOperand),
                family.skip_count(SkipReason::UnsupportedOperandSchema),
                family.skip_count(SkipReason::MalformedOperand),
                family.skip_count(SkipReason::UnsupportedFeature),
                family.skip_count(SkipReason::UnsupportedNamespace),
                family.skip_count(SkipReason::AmbiguousEncoding),
            ));
        }

        markdown
    }
}

impl FamilyCoverage {
    fn new(family: String) -> Self {
        Self {
            family,
            total_records: 0,
            active_records: 0,
            skipped_records: 0,
            skip_reasons: BTreeMap::new(),
        }
    }

    fn record(&mut self, record: &InstructionRecord) {
        self.total_records += 1;
        if record.active {
            self.active_records += 1;
        } else {
            self.skipped_records += 1;
        }
        if let Some(reason) = &record.skip_reason {
            *self.skip_reasons.entry(reason.clone()).or_default() += 1;
        }
    }

    pub(crate) fn skip_count(&self, reason: SkipReason) -> usize {
        self.skip_reasons.get(&reason).copied().unwrap_or(0)
    }
}

fn family_name(family: &InstructionFamily) -> &'static str {
    match family {
        InstructionFamily::Branch => "branch",
        InstructionFamily::DataProcessing => "data_processing",
        InstructionFamily::LoadStore => "load_store",
        InstructionFamily::SimdFp => "simd_fp",
        InstructionFamily::System => "system",
        InstructionFamily::Sve => "sve",
        InstructionFamily::Sme => "sme",
        InstructionFamily::Crypto => "crypto",
        InstructionFamily::MemTag => "mem_tag",
        InstructionFamily::Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64_gen::model::SkipReason;
    use crate::aarch64_gen::tblgen_json;

    #[test]
    fn aggregates_totals_families_and_skip_reasons_from_fixture() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");

        let report = CoverageReport::from_records(&records);

        assert_eq!(report.total_records, 9);
        assert_eq!(report.active_records, 5);
        assert_eq!(report.skipped_records, 4);

        let branch = &report.by_family["branch"];
        assert_eq!(branch.total_records, 2);
        assert_eq!(branch.active_records, 1);
        assert_eq!(branch.skipped_records, 1);
        assert_eq!(branch.skip_count(SkipReason::Pseudo), 1);

        let data_processing = &report.by_family["data_processing"];
        assert_eq!(data_processing.total_records, 3);
        assert_eq!(data_processing.active_records, 2);
        assert_eq!(data_processing.skipped_records, 1);
        assert_eq!(data_processing.skip_count(SkipReason::Alias), 1);
        assert_eq!(data_processing.skip_count(SkipReason::UnknownOperand), 0);

        let unknown = &report.by_family["unknown"];
        assert_eq!(unknown.total_records, 2);
        assert_eq!(unknown.active_records, 0);
        assert_eq!(unknown.skipped_records, 2);
        assert_eq!(unknown.skip_count(SkipReason::UnknownOperand), 1);
        assert_eq!(unknown.skip_count(SkipReason::UnsupportedNamespace), 1);
    }

    #[test]
    fn coverage_report_serializes_with_stable_family_keys() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");

        let serialized = serde_json::to_value(CoverageReport::from_records(&records))
            .expect("coverage report should serialize");

        assert_eq!(serialized["total_records"], 9);
        assert_eq!(serialized["active_records"], 5);
        assert_eq!(serialized["skipped_records"], 4);
        assert_eq!(serialized["by_family"]["branch"]["total_records"], 2);
        assert_eq!(serialized["by_family"]["branch"]["active_records"], 1);
        assert_eq!(serialized["by_family"]["branch"]["skipped_records"], 1);
        assert!(serialized.get("families").is_none());
    }

    #[test]
    fn markdown_summary_contains_totals_header_and_family_rows() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");

        let markdown = CoverageReport::from_records(&records).to_markdown();

        assert!(markdown.contains("Total records: 9"));
        assert!(markdown.contains("Active records: 5"));
        assert!(markdown.contains("Skipped records: 4"));
        assert!(markdown.contains("| Family | Total | Active | Skipped | Pseudo | Alias | Missing encoding | Unknown operand | Unsupported operand schema | Malformed operand | Unsupported feature | Unsupported namespace | Ambiguous encoding |"));
        assert!(markdown.contains("| branch | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |"));
        assert!(
            markdown
                .contains("| data_processing | 3 | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |")
        );
        assert!(markdown.contains("| simd_fp | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |"));
        assert!(markdown.contains("| unknown | 2 | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 |"));
    }
}
