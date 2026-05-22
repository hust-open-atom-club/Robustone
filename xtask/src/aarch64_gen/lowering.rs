//! AArch64 lowering registry diagnostics.

use std::collections::{BTreeMap, BTreeSet};

use crate::aarch64_gen::model::{
    InstructionFamily, LoweredCatalog, LoweredInstructionProbe, LoweredOperandProbe,
    LoweringBlockReason, LoweringBlockedBucket, LoweringReport, NormalizationStage,
    NormalizedCatalog, NormalizedInstruction, NormalizedOperand, OperandKind, OperandRole,
    SchemaId,
};
use crate::aarch64_gen::schema::schema_id_for;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweringRule {
    ExactSchema {
        family: InstructionFamily,
        schema_id: SchemaId,
        format_name: &'static str,
    },
    SveZpmz {
        format_name: &'static str,
    },
}

impl LoweringRule {
    pub(crate) fn exact_schema(
        family: InstructionFamily,
        schema_id: SchemaId,
        format_name: &'static str,
    ) -> Self {
        Self::ExactSchema {
            family,
            schema_id,
            format_name,
        }
    }

    pub(crate) fn sve_zpmz() -> Self {
        Self::SveZpmz {
            format_name: "SVE_PRED_Z",
        }
    }

    fn format_name(&self) -> &'static str {
        match self {
            Self::ExactSchema { format_name, .. } | Self::SveZpmz { format_name } => format_name,
        }
    }

    fn probe_facts(&self, instruction: &NormalizedInstruction) -> Option<RuleProbeFacts> {
        match self {
            Self::ExactSchema {
                family, schema_id, ..
            } => {
                let actual_schema = schema_id_for(instruction);
                if *family == instruction.family && schema_id == &actual_schema {
                    Some(RuleProbeFacts::default())
                } else {
                    None
                }
            }
            Self::SveZpmz { .. } => sve_zpmz_probe_facts(instruction),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RuleProbeFacts {
    element_width_bits: Option<u16>,
    predicate_register_class: Option<String>,
    vector_register_class: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoweringRegistry {
    rules: Vec<LoweringRule>,
}

impl Default for LoweringRegistry {
    fn default() -> Self {
        Self {
            rules: vec![LoweringRule::sve_zpmz()],
        }
    }
}

impl LoweringRegistry {
    pub(crate) fn new(rules: Vec<LoweringRule>) -> Self {
        Self { rules }
    }

    pub(crate) fn lower_catalog(
        &self,
        catalog: &NormalizedCatalog,
    ) -> (LoweredCatalog, LoweringReport) {
        let mut lowered = Vec::new();
        let mut probes = Vec::new();
        let mut blocked: BTreeMap<(SchemaId, LoweringBlockReason), BlockedBuilder> =
            BTreeMap::new();

        for instruction in &catalog.instructions {
            let schema_id = schema_id_for(instruction);
            if instruction.stage != NormalizationStage::Normalized {
                blocked
                    .entry((
                        schema_id.clone(),
                        LoweringBlockReason::BlockedBeforeLowering,
                    ))
                    .or_insert_with(|| {
                        BlockedBuilder::new(schema_id, LoweringBlockReason::BlockedBeforeLowering)
                    })
                    .push(instruction);
                continue;
            }

            if let Some((rule, facts)) = self.matching_rule(instruction) {
                lowered.push(instruction.opcode_id.clone());
                probes.push(LoweredInstructionProbe {
                    opcode_id: instruction.opcode_id.clone(),
                    mnemonic: probe_mnemonic(instruction).to_string(),
                    family: instruction.family.clone(),
                    schema_id,
                    encoding_mask: instruction.encoding_mask,
                    encoding_value: instruction.encoding_value,
                    format_name: rule.format_name().to_string(),
                    element_width_bits: facts.element_width_bits,
                    predicate_register_class: facts.predicate_register_class,
                    vector_register_class: facts.vector_register_class,
                    operands: instruction
                        .operands
                        .iter()
                        .map(|operand| LoweredOperandProbe {
                            kind: operand.kind.clone(),
                            role: operand.role.clone(),
                            raw_class: operand.raw_class.clone(),
                        })
                        .collect(),
                });
            } else {
                blocked
                    .entry((schema_id.clone(), LoweringBlockReason::UnsupportedLowering))
                    .or_insert_with(|| {
                        BlockedBuilder::new(schema_id, LoweringBlockReason::UnsupportedLowering)
                    })
                    .push(instruction);
            }
        }

        let blocked_buckets: Vec<_> = blocked.into_values().map(BlockedBuilder::finish).collect();
        let blocked_records = blocked_buckets.iter().map(|bucket| bucket.count).sum();
        let lowered_records = lowered.len();

        (
            LoweredCatalog {
                opcode_ids: lowered,
                probes,
            },
            LoweringReport {
                lowered_records,
                blocked_records,
                blocked_buckets,
            },
        )
    }

    fn matching_rule(
        &self,
        instruction: &NormalizedInstruction,
    ) -> Option<(&LoweringRule, RuleProbeFacts)> {
        self.rules
            .iter()
            .find_map(|rule| rule.probe_facts(instruction).map(|facts| (rule, facts)))
    }
}

fn probe_mnemonic(instruction: &NormalizedInstruction) -> &str {
    if instruction.opcode_id == "BCC" && instruction.mnemonic == "b.$cond" {
        "b.cond"
    } else {
        &instruction.mnemonic
    }
}

fn sve_zpmz_probe_facts(instruction: &NormalizedInstruction) -> Option<RuleProbeFacts> {
    if instruction.family != InstructionFamily::Sve || instruction.operands.len() != 3 {
        return None;
    }

    let mut predicate = None;
    let mut tied_vector = None;
    let mut source_vector = None;

    for operand in &instruction.operands {
        if is_predicate_read(operand) {
            if predicate.replace(operand).is_some() {
                return None;
            }
        } else if is_zpr_read_write(operand) {
            if tied_vector.replace(operand).is_some() {
                return None;
            }
        } else if is_zpr_read(operand) {
            if source_vector.replace(operand).is_some() {
                return None;
            }
        } else {
            return None;
        }
    }

    let predicate = predicate?;
    let tied_vector = tied_vector?;
    let source_vector = source_vector?;
    let width = sve_zpr_element_width(&tied_vector.raw_class)?;

    if tied_vector.raw_class != source_vector.raw_class {
        return None;
    }

    Some(RuleProbeFacts {
        element_width_bits: Some(width),
        predicate_register_class: Some(predicate.raw_class.clone()),
        vector_register_class: Some(tied_vector.raw_class.clone()),
    })
}

fn is_predicate_read(operand: &NormalizedOperand) -> bool {
    operand.kind == OperandKind::Predicate
        && operand.role == OperandRole::Read
        && operand.raw_class == "PPR3bAny"
}

fn is_zpr_read_write(operand: &NormalizedOperand) -> bool {
    operand.kind == OperandKind::Register
        && operand.role == OperandRole::ReadWrite
        && sve_zpr_element_width(&operand.raw_class).is_some()
}

fn is_zpr_read(operand: &NormalizedOperand) -> bool {
    operand.kind == OperandKind::Register
        && operand.role == OperandRole::Read
        && sve_zpr_element_width(&operand.raw_class).is_some()
}

fn sve_zpr_element_width(raw_class: &str) -> Option<u16> {
    match raw_class {
        "ZPR8" => Some(8),
        "ZPR16" => Some(16),
        "ZPR32" => Some(32),
        "ZPR64" => Some(64),
        _ => None,
    }
}

struct BlockedBuilder {
    schema_id: SchemaId,
    reason: LoweringBlockReason,
    count: usize,
    families: BTreeSet<InstructionFamily>,
    examples: Vec<String>,
}

impl BlockedBuilder {
    fn new(schema_id: SchemaId, reason: LoweringBlockReason) -> Self {
        Self {
            schema_id,
            reason,
            count: 0,
            families: BTreeSet::new(),
            examples: Vec::new(),
        }
    }

    fn push(&mut self, instruction: &NormalizedInstruction) {
        self.count += 1;
        self.families.insert(instruction.family.clone());
        if self.examples.len() < 5 {
            self.examples.push(instruction.opcode_id.clone());
        }
    }

    fn finish(self) -> LoweringBlockedBucket {
        LoweringBlockedBucket {
            schema_id: self.schema_id,
            count: self.count,
            families: self.families.into_iter().collect(),
            examples: self.examples,
            reason: self.reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64_gen::{normalize, tblgen_json};

    #[test]
    fn lowering_rule_activates_all_records_in_matching_schema_and_family() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);
        let add = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.opcode_id == "ADDWRI")
            .expect("ADDWRI should exist");
        let registry = LoweringRegistry::new(vec![LoweringRule::exact_schema(
            InstructionFamily::DataProcessing,
            schema_id_for(add),
            "I_ADD",
        )]);

        let (lowered, report) = registry.lower_catalog(&catalog);

        assert!(lowered.opcode_ids.contains(&"ADDWRI".to_string()));
        assert_eq!(report.lowered_records, 1);
        assert_eq!(report.blocked_records, catalog.instructions.len() - 1);
    }

    #[test]
    fn branch_schema_rule_lowers_bcc_into_probe_with_stable_facts() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);
        let branch = catalog
            .instructions
            .iter()
            .find(|instruction| instruction.opcode_id == "BCC")
            .expect("BCC should exist");
        let branch_schema = schema_id_for(branch);
        let registry = LoweringRegistry::new(vec![LoweringRule::exact_schema(
            InstructionFamily::Branch,
            branch_schema.clone(),
            "FMT_BCC",
        )]);

        let (lowered, report) = registry.lower_catalog(&catalog);

        let probe = lowered
            .probes
            .iter()
            .find(|probe| probe.opcode_id == "BCC")
            .expect("BCC should be lowered into probe catalog");
        assert_eq!(probe.family, InstructionFamily::Branch);
        assert_eq!(probe.schema_id, branch_schema);
        assert_eq!(probe.format_name, "FMT_BCC");
        assert_eq!(probe.mnemonic, "b.cond");
        assert_eq!(probe.element_width_bits, None);
        assert_eq!(probe.predicate_register_class, None);
        assert_eq!(probe.vector_register_class, None);
        assert_eq!(probe.encoding_mask, Some(0xff00_0010));
        assert_eq!(probe.encoding_value, Some(0x5400_0000));
        assert_eq!(report.lowered_records, 1);
        assert!(lowered.opcode_ids.contains(&"BCC".to_string()));
    }

    #[test]
    fn default_registry_lowers_fixture_sve_zpmz_width_buckets_into_probes() {
        let json = include_str!("../../tests/fixtures/aarch64_sve_zpmz_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);
        let registry = LoweringRegistry::default();

        let (lowered, report) = registry.lower_catalog(&catalog);

        for (opcode, raw_class, width, encoding_value) in [
            ("ADD_ZPMZ_B", "ZPR8", 8, 0x0400_0000),
            ("ADD_ZPMZ_H", "ZPR16", 16, 0x0440_0000),
            ("ADD_ZPMZ_S", "ZPR32", 32, 0x0480_0000),
            ("ADD_ZPMZ_D", "ZPR64", 64, 0x04c0_0000),
        ] {
            let instruction = catalog
                .instructions
                .iter()
                .find(|instruction| instruction.opcode_id == opcode)
                .expect("representative SVE ZPmZ instruction should exist");
            let probe = lowered
                .probes
                .iter()
                .find(|probe| probe.opcode_id == opcode)
                .expect("representative SVE ZPmZ instruction should be lowered");

            assert_eq!(probe.mnemonic, "add");
            assert_eq!(probe.family, InstructionFamily::Sve);
            assert_eq!(probe.schema_id, schema_id_for(instruction));
            assert_eq!(probe.format_name, "SVE_PRED_Z");
            assert_eq!(probe.element_width_bits, Some(width));
            assert_eq!(probe.predicate_register_class.as_deref(), Some("PPR3bAny"));
            assert_eq!(probe.vector_register_class.as_deref(), Some(raw_class));
            assert_eq!(probe.encoding_mask, Some(0xffff_e000));
            assert_eq!(probe.encoding_value, Some(encoding_value));
            assert_eq!(probe.operands.len(), 3);
            assert_eq!(probe.operands[0].raw_class, "PPR3bAny");
            assert_eq!(
                probe.operands[0].role,
                crate::aarch64_gen::model::OperandRole::Read
            );
            assert_eq!(probe.operands[1].raw_class, raw_class);
            assert_eq!(
                probe.operands[1].role,
                crate::aarch64_gen::model::OperandRole::ReadWrite
            );
            assert_eq!(probe.operands[2].raw_class, raw_class);
            assert_eq!(
                probe.operands[2].role,
                crate::aarch64_gen::model::OperandRole::Read
            );
            assert!(lowered.opcode_ids.contains(&opcode.to_string()));
        }

        assert!(report.lowered_records >= 4);
    }

    #[test]
    fn default_registry_lowers_sve_zpmz_when_tied_vector_precedes_predicate() {
        let instruction = NormalizedInstruction {
            llvm_name: "ABS_ZPMZ_B".to_string(),
            opcode_id: "ABS_ZPMZ_B".to_string(),
            mnemonic: "abs".to_string(),
            family: InstructionFamily::Sve,
            groups: vec![],
            operands: vec![
                NormalizedOperand {
                    name: "_Zdn".to_string(),
                    kind: OperandKind::Register,
                    role: OperandRole::ReadWrite,
                    optional: false,
                    raw_class: "ZPR8".to_string(),
                },
                NormalizedOperand {
                    name: "Pg".to_string(),
                    kind: OperandKind::Predicate,
                    role: OperandRole::Read,
                    optional: false,
                    raw_class: "PPR3bAny".to_string(),
                },
                NormalizedOperand {
                    name: "Zm".to_string(),
                    kind: OperandKind::Register,
                    role: OperandRole::Read,
                    optional: false,
                    raw_class: "ZPR8".to_string(),
                },
            ],
            encoding_mask: Some(0xffff_e000),
            encoding_value: Some(0x0418_0000),
            features: vec!["sve".to_string()],
            stage: NormalizationStage::Normalized,
            diagnostic: None,
        };
        let schema_id = schema_id_for(&instruction);
        let catalog = NormalizedCatalog {
            instructions: vec![instruction],
        };
        let registry = LoweringRegistry::default();

        let (lowered, report) = registry.lower_catalog(&catalog);

        assert_eq!(report.lowered_records, 1);
        assert_eq!(report.blocked_records, 0);
        assert_eq!(lowered.opcode_ids, vec!["ABS_ZPMZ_B".to_string()]);
        let probe = lowered
            .probes
            .iter()
            .find(|probe| probe.opcode_id == "ABS_ZPMZ_B")
            .expect("ABS_ZPMZ_B should be lowered");
        assert_eq!(probe.mnemonic, "abs");
        assert_eq!(probe.family, InstructionFamily::Sve);
        assert_eq!(probe.schema_id, schema_id);
        assert_eq!(probe.format_name, "SVE_PRED_Z");
        assert_eq!(probe.element_width_bits, Some(8));
        assert_eq!(probe.predicate_register_class.as_deref(), Some("PPR3bAny"));
        assert_eq!(probe.vector_register_class.as_deref(), Some("ZPR8"));
        assert_eq!(probe.operands.len(), 3);
        assert_eq!(probe.operands[0].raw_class, "ZPR8");
        assert_eq!(probe.operands[0].role, OperandRole::ReadWrite);
        assert_eq!(probe.operands[1].raw_class, "PPR3bAny");
        assert_eq!(probe.operands[1].role, OperandRole::Read);
        assert_eq!(probe.operands[2].raw_class, "ZPR8");
        assert_eq!(probe.operands[2].role, OperandRole::Read);
    }

    #[test]
    fn unsupported_lowering_is_grouped_by_schema_family_and_examples() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = tblgen_json::parse_records(json).expect("fixture should parse");
        let catalog = normalize::normalize_records(&records);
        let registry = LoweringRegistry::default();

        let (_lowered, report) = registry.lower_catalog(&catalog);

        assert_eq!(report.lowered_records, 0);
        assert!(report.blocked_records > 0);
        assert!(report.blocked_buckets.iter().any(|bucket| {
            bucket.reason == LoweringBlockReason::UnsupportedLowering
                && bucket.families.contains(&InstructionFamily::DataProcessing)
                && bucket.examples.contains(&"ADDWRI".to_string())
        }));
    }
}
