//! AArch64 generator model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct InstructionRecord {
    pub llvm_name: String,
    pub opcode_id: String,
    pub mnemonic: String,
    pub family: InstructionFamily,
    pub groups: Vec<InstructionGroup>,
    pub operands: Vec<OperandRecord>,
    pub encoding: String,
    pub encoding_mask: Option<u32>,
    pub encoding_value: Option<u32>,
    pub features: Vec<String>,
    pub is_pseudo: bool,
    pub is_alias: bool,
    pub active: bool,
    pub skip_reason: Option<SkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InstructionFamily {
    Branch,
    DataProcessing,
    LoadStore,
    SimdFp,
    System,
    Sve,
    Sme,
    Crypto,
    MemTag,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InstructionGroup {
    Branch,
    Integer,
    LoadStore,
    Float,
    Vector,
    System,
    Crypto,
    MemoryTagging,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OperandRecord {
    pub name: String,
    pub kind: OperandKind,
    pub role: OperandRole,
    pub optional: bool,
    pub raw_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OperandKind {
    Register,
    Immediate,
    Memory,
    Condition,
    SystemRegister,
    Label,
    Predicate,
    VectorList,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OperandRole {
    Read,
    Write,
    ReadWrite,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SkipReason {
    Pseudo,
    Alias,
    MissingEncoding,
    UnknownOperand,
    UnsupportedOperandSchema,
    MalformedOperand,
    UnsupportedFeature,
    UnsupportedNamespace,
    AmbiguousEncoding,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NormalizedCatalog {
    pub instructions: Vec<NormalizedInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NormalizedInstruction {
    pub llvm_name: String,
    pub opcode_id: String,
    pub mnemonic: String,
    pub family: InstructionFamily,
    pub groups: Vec<InstructionGroup>,
    pub operands: Vec<NormalizedOperand>,
    pub encoding_mask: Option<u32>,
    pub encoding_value: Option<u32>,
    pub features: Vec<String>,
    pub stage: NormalizationStage,
    pub diagnostic: Option<NormalizationDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NormalizedOperand {
    pub name: String,
    pub kind: OperandKind,
    pub role: OperandRole,
    pub optional: bool,
    pub raw_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NormalizationStage {
    Normalized,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NormalizationDiagnostic {
    MissingEncoding,
    UnknownOperand,
    MalformedOperand,
    UnsupportedNamespace,
    Pseudo,
    Alias,
    UnsupportedFeature,
    UnsupportedOperandSchema,
    AmbiguousEncoding,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct SchemaId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SchemaBuckets {
    pub buckets: Vec<SchemaBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SchemaBucket {
    pub schema_id: SchemaId,
    pub count: usize,
    pub families: Vec<InstructionFamily>,
    pub examples: Vec<String>,
    pub blocked_diagnostics: Vec<NormalizationDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LoweringReport {
    pub lowered_records: usize,
    pub blocked_records: usize,
    pub blocked_buckets: Vec<LoweringBlockedBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LoweringBlockedBucket {
    pub schema_id: SchemaId,
    pub count: usize,
    pub families: Vec<InstructionFamily>,
    pub examples: Vec<String>,
    pub reason: LoweringBlockReason,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoweringBlockReason {
    UnsupportedLowering,
    BlockedBeforeLowering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LoweredOperandProbe {
    pub kind: OperandKind,
    pub role: OperandRole,
    pub raw_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LoweredInstructionProbe {
    pub opcode_id: String,
    pub mnemonic: String,
    pub family: InstructionFamily,
    pub schema_id: SchemaId,
    pub encoding_mask: Option<u32>,
    pub encoding_value: Option<u32>,
    pub format_name: String,
    pub element_width_bits: Option<u16>,
    pub predicate_register_class: Option<String>,
    pub vector_register_class: Option<String>,
    pub operands: Vec<LoweredOperandProbe>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LoweredCatalog {
    pub opcode_ids: Vec<String>,
    pub probes: Vec<LoweredInstructionProbe>,
}

impl InstructionRecord {
    pub(crate) fn mark_skipped(&mut self, reason: SkipReason) {
        self.active = false;
        self.skip_reason = Some(reason);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_instruction_with_skip_reason_and_metadata() {
        let record = InstructionRecord {
            llvm_name: "ADDWri".to_string(),
            opcode_id: "ADDWRI".to_string(),
            mnemonic: "add".to_string(),
            family: InstructionFamily::DataProcessing,
            groups: vec![InstructionGroup::Integer],
            operands: vec![OperandRecord {
                name: "Rd".to_string(),
                kind: OperandKind::Register,
                role: OperandRole::Write,
                optional: false,
                raw_class: "GPR32".to_string(),
            }],
            encoding: "0001000100??????????????????????".to_string(),
            encoding_mask: Some(0xffc0_0000),
            encoding_value: Some(0x1100_0000),
            features: vec!["HasV8_0a".to_string()],
            is_pseudo: false,
            is_alias: false,
            active: true,
            skip_reason: None,
        };

        let json = serde_json::to_string(&record).expect("record should serialize");

        assert!(json.contains("\"llvm_name\":\"ADDWri\""));
        assert!(json.contains("\"opcode_id\":\"ADDWRI\""));
        assert!(json.contains("\"groups\":[\"integer\"]"));
        assert!(json.contains("\"active\":true"));
        assert!(json.contains("\"skip_reason\":null"));
    }

    #[test]
    fn serializes_skipped_instruction_reason_as_snake_case() {
        let record = InstructionRecord {
            llvm_name: "PseudoRET".to_string(),
            opcode_id: "PSEUDORET".to_string(),
            mnemonic: "ret".to_string(),
            family: InstructionFamily::Branch,
            groups: vec![InstructionGroup::Branch],
            operands: Vec::new(),
            encoding: String::new(),
            encoding_mask: None,
            encoding_value: None,
            features: Vec::new(),
            is_pseudo: true,
            is_alias: false,
            active: false,
            skip_reason: Some(SkipReason::Pseudo),
        };

        let json = serde_json::to_string(&record).expect("record should serialize");

        assert!(json.contains("\"skip_reason\":\"pseudo\""));
        assert!(json.contains("\"active\":false"));
    }

    #[test]
    fn serializes_unsupported_operand_schema_skip_reason() {
        let record = InstructionRecord {
            llvm_name: "ComplexVectorOp".to_string(),
            opcode_id: "COMPLEXVECTOROP".to_string(),
            mnemonic: "complex".to_string(),
            family: InstructionFamily::SimdFp,
            groups: vec![InstructionGroup::Vector],
            operands: Vec::new(),
            encoding: "01010101010101010101010101010101".to_string(),
            encoding_mask: Some(0xffff_ffff),
            encoding_value: Some(0x5555_5555),
            features: Vec::new(),
            is_pseudo: false,
            is_alias: false,
            active: false,
            skip_reason: Some(SkipReason::UnsupportedOperandSchema),
        };

        let json = serde_json::to_string(&record).expect("record should serialize");

        assert!(json.contains("\"skip_reason\":\"unsupported_operand_schema\""));
    }
}
