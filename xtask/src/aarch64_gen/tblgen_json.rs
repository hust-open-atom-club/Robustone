//! AArch64 TableGen JSON ingestion.

use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::aarch64_gen::model::{
    InstructionFamily, InstructionGroup, InstructionRecord, OperandKind, OperandRecord,
    OperandRole, SkipReason,
};

pub(crate) fn load_instruction_records(
    llvm_project: &Path,
) -> Result<Vec<InstructionRecord>, String> {
    let json_path = llvm_project.join("llvm/lib/Target/AArch64/AArch64.td.json");
    let json = fs::read_to_string(&json_path)
        .map_err(|err| format!("failed to read {}: {err}", json_path.display()))?;
    parse_records(&json)
}

pub(crate) fn parse_records(json: &str) -> Result<Vec<InstructionRecord>, String> {
    let root: Value = serde_json::from_str(json).map_err(|err| format!("invalid JSON: {err}"))?;
    let root_object = root
        .as_object()
        .ok_or_else(|| "TableGen JSON root must be an object".to_string())?;
    let instruction_names = root
        .pointer("/!instanceof/Instruction")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing !instanceof.Instruction array".to_string())?;

    let mut records = Vec::with_capacity(instruction_names.len());
    for name_value in instruction_names {
        let name = name_value
            .as_str()
            .ok_or_else(|| "instruction name must be a string".to_string())?;
        let record = root_object
            .get(name)
            .ok_or_else(|| format!("missing record for instruction {name}"))?;
        records.push(parse_record(name, record));
    }

    records.sort_by(|left, right| {
        active_rank(left)
            .cmp(&active_rank(right))
            .then_with(|| skip_rank(left).cmp(&skip_rank(right)))
            .then_with(|| family_rank(&left.family).cmp(&family_rank(&right.family)))
            .then_with(|| left.llvm_name.cmp(&right.llvm_name))
    });
    Ok(records)
}

fn parse_record(name: &str, record: &Value) -> InstructionRecord {
    let asm_string = optional_string(record, "AsmString").unwrap_or(name);
    let namespace = optional_string(record, "Namespace").unwrap_or("");
    let out_operands = record.get("OutOperandList");
    let in_operands = record.get("InOperandList");
    let (encoding, encoding_mask, encoding_value) = parse_encoding(record);
    let mut features = parse_predicates(record);
    features.sort();
    features.dedup();

    let mut operands = Vec::new();
    operands.extend(parse_operand_list(out_operands, OperandRole::Write));
    operands.extend(parse_operand_list(in_operands, OperandRole::Read));
    apply_tied_operand_constraints(&mut operands, record);

    let is_pseudo = optional_bool(record, "isPseudo") || optional_bool(record, "isCodeGenOnly");
    let is_alias = optional_bool(record, "isAsmParserOnly");
    let family = classify_family(name, asm_string, &features, &operands);
    let groups = classify_groups(&family, &features, &operands);

    let mut parsed = InstructionRecord {
        llvm_name: name.to_string(),
        opcode_id: sanitize_opcode_id(name),
        mnemonic: mnemonic_from_asm(asm_string).to_string(),
        family,
        groups,
        operands,
        encoding,
        encoding_mask,
        encoding_value,
        features,
        is_pseudo,
        is_alias,
        active: true,
        skip_reason: None,
    };

    if namespace != "AArch64" {
        parsed.mark_skipped(SkipReason::UnsupportedNamespace);
    } else if parsed.is_pseudo {
        parsed.mark_skipped(SkipReason::Pseudo);
    } else if parsed.is_alias {
        parsed.mark_skipped(SkipReason::Alias);
    } else if parsed.encoding_mask.is_none() || parsed.encoding_value.is_none() {
        parsed.mark_skipped(SkipReason::MissingEncoding);
    } else if parsed.operands.iter().any(is_malformed_operand) {
        parsed.mark_skipped(SkipReason::MalformedOperand);
    } else if parsed
        .operands
        .iter()
        .any(|operand| operand.kind == OperandKind::Unknown)
    {
        parsed.mark_skipped(SkipReason::UnknownOperand);
    } else if has_unsupported_feature_predicate(&parsed.features) {
        parsed.mark_skipped(SkipReason::UnsupportedFeature);
    } else if !can_emit_initial_spec(&parsed) {
        parsed.mark_skipped(SkipReason::UnsupportedOperandSchema);
    }

    parsed
}

fn optional_string<'a>(record: &'a Value, field: &str) -> Option<&'a str> {
    record.get(field).and_then(Value::as_str)
}

fn optional_bool(record: &Value, field: &str) -> bool {
    match record.get(field) {
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(number)) => number.as_u64() == Some(1),
        _ => false,
    }
}

fn parse_encoding(record: &Value) -> (String, Option<u32>, Option<u32>) {
    if let Some(bits) = optional_string(record, "bits") {
        let encoding = bits.to_string();
        let (mask, value) = parse_bits(&encoding);
        return (encoding, mask, value);
    }

    let Some(inst) = record.get("Inst").and_then(Value::as_array) else {
        return (String::new(), None, None);
    };
    let Some(encoding) = parse_inst_bits(inst) else {
        return (String::new(), None, None);
    };
    let (mask, value) = parse_bits(&encoding);
    (encoding, mask, value)
}

fn parse_inst_bits(inst: &[Value]) -> Option<String> {
    if inst.len() != 32 {
        return None;
    }

    let mut bits = String::with_capacity(32);
    for bit in inst.iter().rev() {
        bits.push(match bit {
            Value::Number(number) if number.as_u64() == Some(0) => '0',
            Value::Number(number) if number.as_u64() == Some(1) => '1',
            Value::Object(object)
                if matches!(
                    object.get("kind").and_then(Value::as_str),
                    Some("varbit" | "var")
                ) =>
            {
                '?'
            }
            Value::Null => '?',
            _ => return None,
        });
    }

    Some(bits)
}

fn parse_bits(bits: &str) -> (Option<u32>, Option<u32>) {
    if bits.len() != 32 {
        return (None, None);
    }

    let mut mask = 0_u32;
    let mut value = 0_u32;
    for bit in bits.chars() {
        mask <<= 1;
        value <<= 1;
        match bit {
            '0' => mask |= 1,
            '1' => {
                mask |= 1;
                value |= 1;
            }
            '?' => {}
            _ => return (None, None),
        }
    }

    if mask == 0 {
        (None, None)
    } else {
        (Some(mask), Some(value))
    }
}

fn parse_predicates(record: &Value) -> Vec<String> {
    record
        .get("Predicates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|predicate| {
            predicate.as_str().or_else(|| {
                predicate
                    .get("def")
                    .or_else(|| predicate.get("printable"))
                    .and_then(Value::as_str)
            })
        })
        .map(str::to_string)
        .collect()
}

fn apply_tied_operand_constraints(operands: &mut Vec<OperandRecord>, record: &Value) {
    let Some(constraints) = optional_string(record, "Constraints") else {
        return;
    };

    for constraint in constraints.split(',') {
        let Some((left, right)) = constraint.split_once('=') else {
            continue;
        };
        let left = left.trim().trim_start_matches('$');
        let right = right.trim().trim_start_matches('$');
        if left.is_empty() || right.is_empty() {
            continue;
        }

        let Some(out_index) = operands
            .iter()
            .position(|operand| operand.role == OperandRole::Write && operand.name == left)
        else {
            continue;
        };
        let Some(in_index) = operands
            .iter()
            .position(|operand| operand.role == OperandRole::Read && operand.name == right)
        else {
            continue;
        };

        operands.remove(out_index);
        let adjusted_in_index = if out_index < in_index {
            in_index - 1
        } else {
            in_index
        };
        operands[adjusted_in_index].role = OperandRole::ReadWrite;
    }
}

fn parse_operand_list(value: Option<&Value>, role: OperandRole) -> Vec<OperandRecord> {
    let Some(value) = value else {
        return Vec::new();
    };

    if let Some(list) = value.as_str() {
        return parse_operands(list, role);
    }

    let Some(args) = value.get("args").and_then(Value::as_array) else {
        if let Some(printable) = value.get("printable").and_then(Value::as_str) {
            return parse_operands(printable, role);
        }
        return Vec::new();
    };

    args.iter()
        .map(|arg| parse_dag_operand(arg, role.clone()))
        .collect()
}

fn parse_dag_operand(arg: &Value, role: OperandRole) -> OperandRecord {
    let Some(items) = arg.as_array() else {
        return malformed_operand(&arg.to_string(), role);
    };
    if items.len() != 2 {
        return malformed_operand(&arg.to_string(), role);
    }

    let class = items[0]
        .get("printable")
        .or_else(|| items[0].get("def"))
        .and_then(Value::as_str);
    let name = items[1].as_str();

    match (class, name) {
        (Some(class), Some(name)) if !class.trim().is_empty() && !name.trim().is_empty() => {
            let raw_class = class.trim().to_string();
            OperandRecord {
                name: name.trim().to_string(),
                kind: classify_operand_kind(&raw_class, name.trim()),
                role,
                optional: false,
                raw_class,
            }
        }
        _ => malformed_operand(&arg.to_string(), role),
    }
}

fn parse_operands(list: &str, role: OperandRole) -> Vec<OperandRecord> {
    let trimmed = list.trim();
    let Some(open_paren) = trimmed.find('(') else {
        return Vec::new();
    };
    let Some(close_paren) = trimmed.rfind(')') else {
        return Vec::new();
    };
    if close_paren <= open_paren {
        return Vec::new();
    }

    let body = trimmed[open_paren + 1..close_paren].trim();
    let Some((direction, operands)) = body.split_once(char::is_whitespace) else {
        return Vec::new();
    };
    if !matches!(direction, "outs" | "ins") {
        return Vec::new();
    }

    operands
        .trim()
        .split(',')
        .map(str::trim)
        .filter(|operand| !operand.is_empty())
        .map(|operand| parse_operand(operand, role.clone()))
        .collect()
}

fn parse_operand(operand: &str, role: OperandRole) -> OperandRecord {
    let Some((class, name)) = operand.split_once(":$") else {
        return malformed_operand(operand, role);
    };
    let name = name.trim();
    if name.is_empty() {
        return malformed_operand(operand, role);
    }
    let raw_class = class.trim().to_string();

    OperandRecord {
        name: name.to_string(),
        kind: classify_operand_kind(&raw_class, name),
        role,
        optional: false,
        raw_class,
    }
}

fn malformed_operand(operand: &str, role: OperandRole) -> OperandRecord {
    OperandRecord {
        name: String::new(),
        kind: OperandKind::Unknown,
        role,
        optional: false,
        raw_class: operand.trim().to_string(),
    }
}

fn is_malformed_operand(operand: &OperandRecord) -> bool {
    operand.name.is_empty() && operand.kind == OperandKind::Unknown
}

fn mnemonic_from_asm(asm_string: &str) -> &str {
    asm_string
        .split(['\t', ' '])
        .next()
        .unwrap_or(asm_string)
        .trim()
        .trim_end_matches('{')
}

fn sanitize_opcode_id(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn classify_operand_kind(class: &str, name: &str) -> OperandKind {
    let class_lower = class.to_ascii_lowercase();
    let name_lower = name.to_ascii_lowercase();

    if class_lower.contains("ccode") || name_lower == "cond" {
        OperandKind::Condition
    } else if is_register_offset_memory_class(&class_lower)
        || class_lower.contains("mem")
        || name_lower.contains("addr")
    {
        OperandKind::Memory
    } else if class_lower.contains("br") || name_lower.contains("label") {
        OperandKind::Label
    } else if is_system_register_operand_class(&class_lower)
        || class_lower.contains("sysreg")
        || class_lower.contains("system")
    {
        OperandKind::SystemRegister
    } else if class_lower == "prfop"
        || class_lower == "svcr_op"
        || class_lower == "sve_pred_enum"
        || class_lower.starts_with("sve_fpimm_")
        || class_lower.starts_with("sme_elm_idx")
        || class_lower.starts_with("vectorindex")
    {
        OperandKind::Immediate
    } else if class_lower.contains("pred")
        || class_lower.starts_with("pnr")
        || is_predicate_list_operand_class(&class_lower)
        || class_lower.starts_with('p') && class_lower.contains("pr")
    {
        OperandKind::Predicate
    } else if class_lower == "matrixtilelist" {
        OperandKind::Immediate
    } else if is_vector_list_operand_class(&class_lower)
        || class_lower.contains("list")
        || class_lower.contains("tuple")
    {
        OperandKind::VectorList
    } else if class_lower.contains("imm")
        || name_lower.contains("imm")
        || is_extend_operand_class(&class_lower)
        || is_barrier_operand_class(&class_lower)
        || is_operation_immediate_operand_class(&class_lower)
        || is_simd_immediate_operand_class(&class_lower)
    {
        OperandKind::Immediate
    } else if class_lower.contains("gpr")
        || class_lower.contains("fpr")
        || class_lower.contains("zpr")
        || class_lower.contains("vec")
        || class_lower.contains("reg")
        || is_simd_vector_register_class(&class_lower)
        || is_sve_vector_register_class(&class_lower)
        || is_sme_register_class(&class_lower)
        || is_atomic_pair_register_class(&class_lower)
    {
        OperandKind::Register
    } else {
        OperandKind::Unknown
    }
}

fn is_simd_vector_register_class(class_lower: &str) -> bool {
    matches!(class_lower, "v64" | "v128" | "v128_lo")
}

fn is_sve_vector_register_class(class_lower: &str) -> bool {
    let Some((prefix, suffix)) = class_lower.split_once('_') else {
        return matches!(class_lower, "zk");
    };
    matches!(prefix, "z" | "zpr" | "ppr") && matches!(suffix, "b" | "h" | "s" | "d" | "q")
}

fn is_vector_list_operand_class(class_lower: &str) -> bool {
    let Some((prefix, suffix)) = class_lower.split_once('_') else {
        return false;
    };
    let element = suffix.split('_').next().unwrap_or(suffix);
    matches!(prefix, "pp" | "zz" | "zzz" | "zzzz")
        && matches!(element, "any" | "b" | "h" | "s" | "d" | "q" | "mul")
}

fn is_predicate_list_operand_class(class_lower: &str) -> bool {
    let Some((prefix, suffix)) = class_lower.split_once('_') else {
        return false;
    };
    let element = suffix.split('_').next().unwrap_or(suffix);
    prefix == "pp" && matches!(element, "b" | "h" | "s" | "d" | "q")
}

fn is_sme_register_class(class_lower: &str) -> bool {
    matches!(
        class_lower,
        "tileop16"
            | "tileop32"
            | "tileop64"
            | "matrixop"
            | "matrixop16"
            | "matrixop32"
            | "matrixop64"
            | "ztr"
    ) || class_lower.starts_with("tilevectorop")
}

fn is_atomic_pair_register_class(class_lower: &str) -> bool {
    matches!(class_lower, "wseqpairclassoperand" | "xseqpairclassoperand")
}

fn is_register_offset_memory_class(class_lower: &str) -> bool {
    class_lower.starts_with("ro_wextend") || class_lower.starts_with("ro_xextend")
}

fn is_extend_operand_class(class_lower: &str) -> bool {
    class_lower.starts_with("arith_extend") || class_lower.starts_with("sve_elm_idx_extdup")
}

fn is_barrier_operand_class(class_lower: &str) -> bool {
    matches!(class_lower, "barrier_op" | "barrier_nxs_op")
}

fn is_operation_immediate_operand_class(class_lower: &str) -> bool {
    matches!(class_lower, "sys_cr_op" | "sve_prfop")
}

fn is_system_register_operand_class(class_lower: &str) -> bool {
    matches!(class_lower, "pstatefield4_op")
}

fn is_simd_immediate_operand_class(class_lower: &str) -> bool {
    matches!(class_lower, "complexrotateop" | "complexrotateopodd")
        || class_lower.starts_with("fixedpoint_f")
}

fn classify_family(
    name: &str,
    asm_string: &str,
    features: &[String],
    operands: &[OperandRecord],
) -> InstructionFamily {
    let name_lower = name.to_ascii_lowercase();
    let mnemonic_lower = mnemonic_from_asm(asm_string).to_ascii_lowercase();

    let has_sme_schema = name_lower.contains("sme")
        || name_lower.contains("za")
        || operands.iter().any(|operand| {
            let class = operand.raw_class.to_ascii_lowercase();
            matches!(
                class.as_str(),
                "matrixop" | "matrixop16" | "matrixop32" | "matrixop64" | "matrixtilelist" | "ztr"
            ) || class.starts_with("tile")
                || operand.name.to_ascii_lowercase().contains("za")
        });
    let has_sve_feature = features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("sve"));
    let has_sme_feature = features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("sme"));

    if has_sme_schema || has_sme_feature && !has_sve_feature {
        InstructionFamily::Sme
    } else if has_sve_feature
        || name_lower.contains("sve")
        || operands
            .iter()
            .any(|operand| operand.raw_class.to_ascii_lowercase().contains("zpr"))
    {
        InstructionFamily::Sve
    } else if is_branch_mnemonic(&mnemonic_lower)
        || name_lower.starts_with("bcond")
        || name_lower.starts_with("branch")
    {
        InstructionFamily::Branch
    } else if is_crypto_feature(features)
        || name_lower.contains("crypto")
        || matches!(
            mnemonic_lower.trim_end_matches('{'),
            "aesd" | "aese" | "sha1c" | "sha256h"
        )
    {
        InstructionFamily::Crypto
    } else if features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("mte"))
        || name_lower.contains("mte")
    {
        InstructionFamily::MemTag
    } else if mnemonic_lower.starts_with("ld")
        || mnemonic_lower.starts_with("st")
        || mnemonic_lower == "cas"
        || mnemonic_lower == "prfm"
        || name_lower.starts_with("cas")
        || name_lower.starts_with("prfm")
        || name_lower.contains("load")
        || name_lower.contains("store")
        || operands
            .iter()
            .any(|operand| operand.kind == OperandKind::Memory)
    {
        InstructionFamily::LoadStore
    } else if features.iter().any(|feature| {
        let feature_lower = feature.to_ascii_lowercase();
        feature_lower.contains("neon")
            || feature_lower.contains("fp")
            || feature_lower.contains("simd")
    }) || operands.iter().any(|operand| {
        operand.kind == OperandKind::Register
            && operand.raw_class.to_ascii_lowercase().contains("fpr")
    }) || name_lower.contains("fp")
        || name_lower.contains("simd")
        || name_lower.starts_with("abs")
    {
        InstructionFamily::SimdFp
    } else if matches!(mnemonic_lower.as_str(), "mrs" | "msr" | "sys" | "sysl")
        || name_lower.contains("sys")
        || matches!(
            mnemonic_lower.as_str(),
            "brk" | "svc" | "hvc" | "smc" | "hlt"
        )
    {
        InstructionFamily::System
    } else if matches!(
        mnemonic_lower.as_str(),
        "add"
            | "sub"
            | "and"
            | "orr"
            | "eor"
            | "bic"
            | "mov"
            | "cmp"
            | "cmn"
            | "mul"
            | "sdiv"
            | "udiv"
    ) {
        InstructionFamily::DataProcessing
    } else {
        InstructionFamily::Unknown
    }
}

fn is_crypto_feature(features: &[String]) -> bool {
    features.iter().any(|feature| {
        matches!(
            feature.as_str(),
            "HasAES" | "HasSHA2" | "HasSHA3" | "HasSM4"
        ) || feature.to_ascii_lowercase().contains("crypto")
    })
}

fn classify_groups(
    family: &InstructionFamily,
    features: &[String],
    operands: &[OperandRecord],
) -> Vec<InstructionGroup> {
    let mut groups = match family {
        InstructionFamily::Branch => vec![InstructionGroup::Branch],
        InstructionFamily::DataProcessing => vec![InstructionGroup::Integer],
        InstructionFamily::LoadStore => vec![InstructionGroup::LoadStore],
        InstructionFamily::SimdFp => vec![InstructionGroup::Vector, InstructionGroup::Float],
        InstructionFamily::System => vec![InstructionGroup::System],
        InstructionFamily::Sve => vec![InstructionGroup::Vector],
        InstructionFamily::Sme => vec![InstructionGroup::Vector],
        InstructionFamily::Crypto => vec![InstructionGroup::Crypto],
        InstructionFamily::MemTag => vec![InstructionGroup::MemoryTagging],
        InstructionFamily::Unknown => vec![InstructionGroup::Unknown],
    };

    if features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("fp"))
        && !groups.contains(&InstructionGroup::Float)
    {
        groups.push(InstructionGroup::Float);
    }
    if operands
        .iter()
        .any(|operand| operand.kind == OperandKind::Memory)
        && !groups.contains(&InstructionGroup::LoadStore)
    {
        groups.push(InstructionGroup::LoadStore);
    }
    groups
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FeaturePredicateMapping {
    Base,
    Fp,
    Simd,
    Crypto,
    Atomics,
    Mte,
    Sve,
    Sme,
}

fn feature_predicate_mapping(feature: &str) -> Option<FeaturePredicateMapping> {
    match feature {
        "HasV8_0a" => Some(FeaturePredicateMapping::Base),
        "HasFPARMv8" | "HasFullFP16" => Some(FeaturePredicateMapping::Fp),
        "HasNEON" => Some(FeaturePredicateMapping::Simd),
        "HasAES" | "HasSHA2" | "HasSHA3" | "HasSM4" => Some(FeaturePredicateMapping::Crypto),
        "HasLSE" => Some(FeaturePredicateMapping::Atomics),
        "HasMTE" => Some(FeaturePredicateMapping::Mte),
        "HasSVE"
        | "HasSVE2"
        | "HasSVEAES"
        | "HasSVEAES2"
        | "HasSVE2AES"
        | "HasSVESHA3"
        | "HasSVESM4"
        | "HasSVE2BitPerm"
        | "HasSVE2SHA3"
        | "HasSVE2SM4"
        | "HasFAMINMAX"
        | "HasFP8"
        | "HasNonStreamingSVE_or_SSVE_AES"
        | "HasNonStreamingSVE_or_SSVE_BitPerm"
        | "HasSVEorStreamingSVE"
        | "HasSVE2orStreamingSVE"
        | "HasSVE_or_SME"
        | "HasSVE2_or_SME"
        | "HasSVE2p1_or_SME"
        | "HasSVE2p1_or_SME2"
        | "HasSVE2p1_or_SME2p1"
        | "HasSVE2p2_or_SME2p2"
        | "HasSVE2p3_or_SME2p3"
        | "HasNonStreamingSVE_or_SME2"
        | "HasNonStreamingSVE2_or_SME2"
        | "HasNonStreamingSVE_or_SME2p1"
        | "HasNonStreamingSVE_or_SME2p2"
        | "HasNonStreamingSVE2p2_or_SME2p2" => Some(FeaturePredicateMapping::Sve),
        "HasSME"
        | "HasSMEandIsNonStreamingSafe"
        | "HasSME2"
        | "HasSME2p1"
        | "HasSME_MOP4"
        | "HasSMEI64"
        | "HasSMEI16I64"
        | "HasSMEF64"
        | "HasSMEB16B16"
        | "HasSMEF64F64"
        | "HasSMEF16F16_or_SMEF8F16"
        | "HasSMEF8F16"
        | "HasSMEF8F32" => Some(FeaturePredicateMapping::Sme),
        _ => None,
    }
}

fn has_unsupported_feature_predicate(features: &[String]) -> bool {
    features
        .iter()
        .any(|feature| feature_predicate_mapping(feature).is_none())
}

fn can_emit_initial_spec(record: &InstructionRecord) -> bool {
    if !matches!(
        record.family,
        InstructionFamily::Branch
            | InstructionFamily::DataProcessing
            | InstructionFamily::LoadStore
            | InstructionFamily::System
            | InstructionFamily::SimdFp
            | InstructionFamily::Crypto
            | InstructionFamily::MemTag
            | InstructionFamily::Sve
            | InstructionFamily::Sme
    ) {
        return false;
    }

    if record.family == InstructionFamily::LoadStore
        && record.operands.iter().any(|operand| {
            operand.kind == OperandKind::Register
                && operand.raw_class.to_ascii_lowercase().contains("fpr")
        })
    {
        return false;
    }

    record.operands.iter().all(|operand| {
        matches!(
            operand.kind,
            OperandKind::Register
                | OperandKind::Immediate
                | OperandKind::Memory
                | OperandKind::Condition
                | OperandKind::SystemRegister
                | OperandKind::Label
                | OperandKind::Predicate
                | OperandKind::VectorList
        ) && !is_schema_deferred_for_record(record, operand)
    })
}

fn is_schema_deferred_for_record(record: &InstructionRecord, operand: &OperandRecord) -> bool {
    let lower = operand.raw_class.to_ascii_lowercase();
    if record.llvm_name == "EXT_ZZI_B" && (lower == "zz_b" || lower == "imm0_255") {
        return false;
    }

    if is_sve_prefetch_record(record) && (lower == "sve_prfop" || lower == "prfop") {
        return false;
    }

    if is_sve_tuple_memory_record(record) && is_vector_list_operand_class(&lower) {
        return false;
    }

    if (record.llvm_name.starts_with("FCPY_ZPmI_") || record.llvm_name.starts_with("FDUP_ZI_"))
        && lower.starts_with("fpimm")
    {
        return false;
    }

    if record.llvm_name.starts_with("SPLICE_ZPZZ_") && is_vector_list_operand_class(&lower) {
        return false;
    }

    if record.llvm_name.starts_with("TBL_ZZZZ_") && is_vector_list_operand_class(&lower) {
        return false;
    }

    if (record.opcode_id.starts_with("ADD_VG2_2ZZ_")
        || record.opcode_id.starts_with("ADD_VG4_4ZZ_"))
        && matches!(
            lower.as_str(),
            "zz_b_mul_r"
                | "zz_h_mul_r"
                | "zz_s_mul_r"
                | "zz_d_mul_r"
                | "zzzz_b_mul_r"
                | "zzzz_h_mul_r"
                | "zzzz_s_mul_r"
                | "zzzz_d_mul_r"
        )
    {
        return false;
    }

    if (record.opcode_id.starts_with("ADD_VG2_M2Z2Z_")
        || record.opcode_id.starts_with("ADD_VG4_M4Z4Z_"))
        && matches!(
            lower.as_str(),
            "matrixop32"
                | "matrixop64"
                | "matrixindexgpr32op8_11"
                | "sme_elm_idx0_7"
                | "zz_s_mul_r"
                | "zz_d_mul_r"
                | "zzzz_s_mul_r"
                | "zzzz_d_mul_r"
        )
    {
        return false;
    }

    if (record.opcode_id.starts_with("ADD_VG4_M4ZZ_")
        || record.opcode_id.starts_with("ADD_VG4_M4Z_"))
        && matches!(
            lower.as_str(),
            "matrixop32"
                | "matrixop64"
                | "matrixindexgpr32op8_11"
                | "sme_elm_idx0_7"
                | "zzzz_s"
                | "zzzz_d"
                | "zzzz_s_mul_r"
                | "zzzz_d_mul_r"
        )
    {
        return false;
    }

    if (record.opcode_id.starts_with("BFADD_VG2_M2Z_")
        || record.opcode_id.starts_with("FADD_VG2_M2Z_"))
        && matches!(
            lower.as_str(),
            "matrixop16"
                | "matrixop32"
                | "matrixop64"
                | "matrixindexgpr32op8_11"
                | "sme_elm_idx0_7"
                | "zz_h_mul_r"
                | "zz_s_mul_r"
                | "zz_d_mul_r"
        )
    {
        return false;
    }

    if (record.opcode_id.starts_with("BFADD_VG4_M4Z_")
        || record.opcode_id.starts_with("FADD_VG4_M4Z_"))
        && matches!(
            lower.as_str(),
            "matrixop16"
                | "matrixop32"
                | "matrixop64"
                | "matrixindexgpr32op8_11"
                | "sme_elm_idx0_7"
                | "zzzz_h_mul_r"
                | "zzzz_s_mul_r"
                | "zzzz_d_mul_r"
        )
    {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "F1CVT_2ZZ_BTOH"
            | "F1CVTL_2ZZ_BTOH"
            | "F2CVTL_2ZZ_BTOH"
            | "BF1CVT_2ZZ_BTOH"
            | "BF1CVTL_2ZZ_BTOH"
            | "F2CVT_2ZZ_BTOH"
            | "BF2CVTL_2ZZ_BTOH"
            | "BF2CVT_2ZZ_BTOH"
    ) && lower == "zz_h_mul_r"
    {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "FCVT_Z2Z_HTOB" | "BFCVT_Z2Z_HTOB" | "FCVT_Z2Z_STOH" | "BFCVT_Z2Z_STOH"
    ) && matches!(lower.as_str(), "zz_h_mul_r" | "zz_s_mul_r")
    {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "FCVT_Z4Z_STOB" | "FCVTN_Z4Z_STOB"
    ) && lower == "zzzz_s_mul_r"
    {
        return false;
    }

    if matches!(record.opcode_id.as_str(), "ADD_VG2_M2Z_S" | "ADD_VG2_M2Z_D")
        && matches!(
            lower.as_str(),
            "matrixop32"
                | "matrixop64"
                | "matrixindexgpr32op8_11"
                | "sme_elm_idx0_7"
                | "zz_s_mul_r"
                | "zz_d_mul_r"
        )
    {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "ADD_VG2_M2ZZ_S" | "ADD_VG2_M2ZZ_D"
    ) && matches!(
        lower.as_str(),
        "matrixop32" | "matrixop64" | "matrixindexgpr32op8_11" | "sme_elm_idx0_7" | "zz_s" | "zz_d"
    ) {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "FDOT_VG2_M2ZZ_BTOH"
            | "FDOT_VG4_M4ZZ_BTOH"
            | "FDOT_VG2_M2ZZI_BTOH"
            | "FDOT_VG4_M4ZZI_BTOH"
            | "FDOT_VG2_M2ZZ_BTOS"
            | "FDOT_VG4_M4ZZ_BTOS"
            | "FDOT_VG2_M2ZZI_BTOS"
            | "FDOT_VG4_M4ZZI_BTOS"
            | "FDOT_VG2_M2Z2Z_BTOH"
            | "FDOT_VG2_M2Z2Z_BTOS"
            | "FDOT_VG4_M4Z4Z_BTOH"
            | "FDOT_VG4_M4Z4Z_BTOS"
    ) && matches!(
        lower.as_str(),
        "matrixop16"
            | "matrixop32"
            | "matrixindexgpr32op8_11"
            | "sme_elm_idx0_7"
            | "zz_b"
            | "zzzz_b"
            | "zz_b_mul_r"
            | "zzzz_b_mul_r"
            | "vectorindexh32b_timm"
            | "vectorindexs32b_timm"
    ) {
        return false;
    }

    if matches!(
        record.opcode_id.as_str(),
        "FVDOT_VG2_M2ZZI_BTOH" | "FVDOTB_VG4_M2ZZI_BTOS" | "FVDOTT_VG4_M2ZZI_BTOS"
    ) && matches!(
        lower.as_str(),
        "matrixop16"
            | "matrixop32"
            | "matrixindexgpr32op8_11"
            | "sme_elm_idx0_7"
            | "zz_b_mul_r"
            | "zpr4b8"
            | "vectorindexh32b_timm"
            | "vectorindexs"
    ) {
        return false;
    }

    if matches!(record.llvm_name.as_str(), "LDR_ZA" | "STR_ZA")
        && matches!(
            lower.as_str(),
            "matrixop" | "matrixindexgpr32op12_15" | "sme_elm_idx0_15" | "imm32_0_15"
        )
    {
        return false;
    }

    is_schema_deferred(operand)
}

fn is_sve_prefetch_record(record: &InstructionRecord) -> bool {
    matches!(record.mnemonic.as_str(), "prfb" | "prfh" | "prfw" | "prfd")
}

fn is_sve_tuple_memory_record(record: &InstructionRecord) -> bool {
    matches!(
        record.mnemonic.as_str(),
        "ld2b"
            | "ld2h"
            | "ld2w"
            | "ld2d"
            | "ld3b"
            | "ld3h"
            | "ld3w"
            | "ld3d"
            | "ld4b"
            | "ld4h"
            | "ld4w"
            | "ld4d"
            | "st2b"
            | "st2h"
            | "st2w"
            | "st2d"
            | "st3b"
            | "st3h"
            | "st3w"
            | "st3d"
            | "st4b"
            | "st4h"
            | "st4w"
            | "st4d"
    ) && record
        .operands
        .iter()
        .any(|operand| is_vector_list_operand_class(&operand.raw_class.to_ascii_lowercase()))
}

fn is_schema_deferred(operand: &OperandRecord) -> bool {
    let raw_class = &operand.raw_class;
    let name = operand.name.to_ascii_lowercase();
    let lower = raw_class.to_ascii_lowercase();
    lower.contains("veclist")
        || lower.contains("tuple")
        || (lower.contains("tilevector") && !lower.starts_with("tilevectorop"))
        || (lower.contains("matrix")
            && lower != "matrixop"
            && lower != "matrixtilelist"
            && !lower.starts_with("matrixindexgpr"))
        || is_vector_list_operand_class(&lower)
        || lower.contains("vreg")
        || lower.contains("neon")
        || lower == "i32imm"
        || lower == "i64imm"
        || lower.starts_with("imm32_")
        || lower.contains("imm0_255")
        || (lower.contains("fpimm") && !lower.starts_with("sve_fpimm_"))
        || lower.contains("simdimm")
        || (is_simd_immediate_operand_class(&lower)
            && lower != "complexrotateop"
            && lower != "complexrotateopodd")
        || is_register_offset_memory_class(&lower)
        || is_extend_operand_class(&lower)
        || is_barrier_operand_class(&lower)
        || is_operation_immediate_operand_class(&lower)
        || (lower == "prfop" && name != "rt")
        || lower.contains("sysreg")
        || lower.contains("system")
        || is_system_register_operand_class(&lower)
        || is_atomic_pair_register_class(&lower)
        || name == "dst"
        || name == "vdst"
}
fn active_rank(record: &InstructionRecord) -> u8 {
    if record.active { 0 } else { 1 }
}

fn skip_rank(record: &InstructionRecord) -> u8 {
    match record.skip_reason {
        None => 0,
        Some(SkipReason::Alias) => 1,
        Some(SkipReason::UnsupportedNamespace) => 2,
        Some(SkipReason::UnknownOperand) => 3,
        Some(SkipReason::UnsupportedOperandSchema) => 4,
        Some(SkipReason::MalformedOperand) => 5,
        Some(SkipReason::Pseudo) => 6,
        Some(SkipReason::MissingEncoding) => 7,
        Some(SkipReason::UnsupportedFeature) => 8,
        Some(SkipReason::AmbiguousEncoding) => 9,
    }
}

fn is_branch_mnemonic(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "b" | "bl" | "blr" | "br" | "ret" | "eret" | "drps"
    ) || mnemonic.starts_with("b.")
        || matches!(mnemonic, "cbz" | "cbnz" | "tbz" | "tbnz")
}

fn family_rank(family: &InstructionFamily) -> u8 {
    match family {
        InstructionFamily::Branch => 0,
        InstructionFamily::DataProcessing => 1,
        InstructionFamily::LoadStore => 2,
        InstructionFamily::SimdFp => 3,
        InstructionFamily::System => 4,
        InstructionFamily::Sve => 5,
        InstructionFamily::Sme => 6,
        InstructionFamily::Crypto => 7,
        InstructionFamily::MemTag => 8,
        InstructionFamily::Unknown => 9,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64_gen::model::{InstructionGroup, SkipReason};

    #[test]
    fn parses_fixture_records_with_active_and_skipped_entries() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        assert_eq!(records.len(), 9);

        let names: Vec<_> = records
            .iter()
            .map(|record| record.llvm_name.as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "Bcc",
                "ADDWri",
                "BICWr",
                "ABSWr",
                "BRK",
                "AliasADD",
                "BadNamespace",
                "MysteryOp",
                "PseudoRET"
            ]
        );

        let bcc = record(&records, "Bcc");
        assert_eq!(bcc.mnemonic, "b.$cond");
        assert_eq!(bcc.family, InstructionFamily::Branch);
        assert_eq!(bcc.groups, vec![InstructionGroup::Branch]);
        assert_eq!(bcc.encoding_mask, Some(0xff00_0010));
        assert_eq!(bcc.encoding_value, Some(0x5400_0000));
        assert!(bcc.active);
        assert_eq!(bcc.skip_reason, None);
        assert_eq!(bcc.operands.len(), 2);
        assert_eq!(bcc.operands[0].name, "cond");
        assert_eq!(bcc.operands[0].kind, OperandKind::Condition);
        assert_eq!(bcc.operands[0].role, OperandRole::Read);
        assert_eq!(bcc.operands[0].raw_class, "ccode");
        assert_eq!(bcc.operands[1].name, "label");
        assert_eq!(bcc.operands[1].kind, OperandKind::Label);

        let add = record(&records, "ADDWri");
        assert_eq!(add.opcode_id, "ADDWRI");
        assert_eq!(add.mnemonic, "add");
        assert_eq!(add.family, InstructionFamily::DataProcessing);
        assert_eq!(add.groups, vec![InstructionGroup::Integer]);
        assert_eq!(add.encoding_mask, Some(0xffc0_0000));
        assert_eq!(add.encoding_value, Some(0x1100_0000));
        assert_eq!(add.features, vec!["HasV8_0a"]);
        assert_eq!(add.operands.len(), 3);
        assert_eq!(add.operands[0].name, "Rd");
        assert_eq!(add.operands[0].kind, OperandKind::Register);
        assert_eq!(add.operands[0].role, OperandRole::Write);
        assert_eq!(add.operands[1].name, "Rn");
        assert_eq!(add.operands[1].role, OperandRole::Read);
        assert_eq!(add.operands[2].kind, OperandKind::Immediate);

        let abs = record(&records, "ABSWr");
        assert_eq!(abs.family, InstructionFamily::SimdFp);
        assert_eq!(
            abs.groups,
            vec![InstructionGroup::Vector, InstructionGroup::Float]
        );
        assert_eq!(abs.features, vec!["HasFullFP16", "HasNEON"]);
        assert!(abs.active);
        assert_eq!(abs.skip_reason, None);

        let brk = record(&records, "BRK");
        assert_eq!(brk.family, InstructionFamily::System);
        assert_eq!(brk.groups, vec![InstructionGroup::System]);

        let pseudo = record(&records, "PseudoRET");
        assert!(pseudo.is_pseudo);
        assert!(!pseudo.active);
        assert_eq!(pseudo.skip_reason, Some(SkipReason::Pseudo));

        let alias = record(&records, "AliasADD");
        assert!(alias.is_alias);
        assert!(!alias.active);
        assert_eq!(alias.skip_reason, Some(SkipReason::Alias));

        let mystery = record(&records, "MysteryOp");
        assert!(!mystery.active);
        assert_eq!(mystery.skip_reason, Some(SkipReason::UnknownOperand));

        let bad_namespace = record(&records, "BadNamespace");
        assert!(!bad_namespace.active);
        assert_eq!(
            bad_namespace.skip_reason,
            Some(SkipReason::UnsupportedNamespace)
        );
    }

    #[test]
    fn parses_real_llvm_inst_array_as_lsb_first_encoding_bits() {
        let json = r#"{
            "!instanceof": { "Instruction": ["BccRealInst"] },
            "BccRealInst": {
                "AsmString": "b.$cond\t$target",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "ccode", "kind": "def", "printable": "ccode" }, "cond"],
                        [{ "def": "am_brcond", "kind": "def", "printable": "am_brcond" }, "target"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins ccode:$cond, am_brcond:$target)"
                },
                "Inst": [
                    { "index": 0, "kind": "varbit", "printable": "cond{0}", "var": "cond" },
                    { "index": 1, "kind": "varbit", "printable": "cond{1}", "var": "cond" },
                    { "index": 2, "kind": "varbit", "printable": "cond{2}", "var": "cond" },
                    { "index": 3, "kind": "varbit", "printable": "cond{3}", "var": "cond" },
                    0,
                    { "index": 0, "kind": "varbit", "printable": "target{0}", "var": "target" },
                    { "index": 1, "kind": "varbit", "printable": "target{1}", "var": "target" },
                    { "index": 2, "kind": "varbit", "printable": "target{2}", "var": "target" },
                    { "index": 3, "kind": "varbit", "printable": "target{3}", "var": "target" },
                    { "index": 4, "kind": "varbit", "printable": "target{4}", "var": "target" },
                    { "index": 5, "kind": "varbit", "printable": "target{5}", "var": "target" },
                    { "index": 6, "kind": "varbit", "printable": "target{6}", "var": "target" },
                    { "index": 7, "kind": "varbit", "printable": "target{7}", "var": "target" },
                    { "index": 8, "kind": "varbit", "printable": "target{8}", "var": "target" },
                    { "index": 9, "kind": "varbit", "printable": "target{9}", "var": "target" },
                    { "index": 10, "kind": "varbit", "printable": "target{10}", "var": "target" },
                    { "index": 11, "kind": "varbit", "printable": "target{11}", "var": "target" },
                    { "index": 12, "kind": "varbit", "printable": "target{12}", "var": "target" },
                    { "index": 13, "kind": "varbit", "printable": "target{13}", "var": "target" },
                    { "index": 14, "kind": "varbit", "printable": "target{14}", "var": "target" },
                    { "index": 15, "kind": "varbit", "printable": "target{15}", "var": "target" },
                    { "index": 16, "kind": "varbit", "printable": "target{16}", "var": "target" },
                    { "index": 17, "kind": "varbit", "printable": "target{17}", "var": "target" },
                    { "index": 18, "kind": "varbit", "printable": "target{18}", "var": "target" },
                    0, 0, 1, 0, 1, 0, 1, 0
                ],
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let bcc = record(&records, "BccRealInst");

        assert_eq!(bcc.encoding, "01010100???????????????????0????");
        assert_eq!(bcc.encoding_mask, Some(0xff00_0010));
        assert_eq!(bcc.encoding_value, Some(0x5400_0000));
        assert_ne!(bcc.skip_reason, Some(SkipReason::MissingEncoding));
    }

    #[test]
    fn parses_null_inst_entries_as_unknown_bits() {
        let json = r#"{
            "!instanceof": { "Instruction": ["NullBitsInst"] },
            "NullBitsInst": {
                "AsmString": "nullbits $Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn)",
                "Inst": [
                    1,
                    0,
                    { "index": 0, "kind": "varbit", "printable": "Rn{0}", "var": "Rn" },
                    null,
                    1,
                    1,
                    0,
                    null,
                    0,
                    1,
                    null,
                    0,
                    { "index": 1, "kind": "varbit", "printable": "Rn{1}", "var": "Rn" },
                    1,
                    0,
                    1,
                    1,
                    { "index": 0, "kind": "varbit", "printable": "Rd{0}", "var": "Rd" },
                    0,
                    null,
                    1,
                    0,
                    1,
                    null,
                    0,
                    0,
                    1,
                    1,
                    null,
                    { "index": 1, "kind": "varbit", "printable": "Rd{1}", "var": "Rd" },
                    0,
                    1
                ],
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let record = record(&records, "NullBitsInst");

        assert_eq!(record.encoding, "10??1100?101?0?1101?0?10?011??01");
        assert_eq!(record.encoding_mask, Some(0xcf75_eb73));
        assert_eq!(record.encoding_value, Some(0x8c51_a231));
        assert_ne!(record.skip_reason, Some(SkipReason::MissingEncoding));
    }

    #[test]
    fn parses_tablegen_whole_variable_inst_entries_for_sme_svcr() {
        let json = r#"{
            "!instanceof": { "Instruction": ["MSRpstatesvcrImm1"] },
            "MSRpstatesvcrImm1": {
                "AsmString": "msr\t$pstatefield, $imm",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "svcr_op", "kind": "def", "printable": "svcr_op" }, "pstatefield"],
                        [{ "def": "imm0_1", "kind": "def", "printable": "imm0_1" }, "imm"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins svcr_op:$pstatefield, imm0_1:$imm)"
                },
                "Inst": [
                    1, 1, 1, 1, 1, 1, 1, 0,
                    { "kind": "var", "printable": "imm", "var": "imm" },
                    { "index": 0, "kind": "varbit", "printable": "pstatefield{0}", "var": "pstatefield" },
                    { "index": 1, "kind": "varbit", "printable": "pstatefield{1}", "var": "pstatefield" },
                    { "index": 2, "kind": "varbit", "printable": "pstatefield{2}", "var": "pstatefield" },
                    0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0,
                    1, 0, 1, 0, 1, 0, 1, 1
                ],
                "Predicates": [{ "def": "HasSME", "kind": "def", "printable": "HasSME" }],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let record = record(&records, "MSRpstatesvcrImm1");

        assert_eq!(record.encoding, "11010101000000110100????01111111");
        assert_eq!(record.encoding_mask, Some(0xffff_f0ff));
        assert_eq!(record.encoding_value, Some(0xd503_407f));
        assert_eq!(record.family, InstructionFamily::Sme);
        assert!(record.active);
        assert_eq!(record.skip_reason, None);
        assert_eq!(record.operands[0].kind, OperandKind::Immediate);
        assert_eq!(record.operands[1].kind, OperandKind::Immediate);
    }

    #[test]
    fn parses_numeric_boolean_flags_for_pseudo_codegen_only_and_alias_records() {
        let json = r#"{
            "!instanceof": { "Instruction": [
                "NumericPseudo",
                "NumericCodeGenOnly",
                "NumericAlias",
                "NumericZeros"
            ] },
            "NumericPseudo": {
                "AsmString": "add $Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095:$imm)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 1,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "NumericCodeGenOnly": {
                "AsmString": "add $Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095:$imm)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 0,
                "isCodeGenOnly": 1,
                "isAsmParserOnly": 0
            },
            "NumericAlias": {
                "AsmString": "add $Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095:$imm)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 1
            },
            "NumericZeros": {
                "AsmString": "add $Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095:$imm)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");

        let pseudo = record(&records, "NumericPseudo");
        assert!(pseudo.is_pseudo);
        assert!(!pseudo.active);
        assert_eq!(pseudo.skip_reason, Some(SkipReason::Pseudo));

        let codegen_only = record(&records, "NumericCodeGenOnly");
        assert!(codegen_only.is_pseudo);
        assert!(!codegen_only.active);
        assert_eq!(codegen_only.skip_reason, Some(SkipReason::Pseudo));

        let alias = record(&records, "NumericAlias");
        assert!(alias.is_alias);
        assert!(!alias.active);
        assert_eq!(alias.skip_reason, Some(SkipReason::Alias));

        let numeric_zeros = record(&records, "NumericZeros");
        assert!(!numeric_zeros.is_pseudo);
        assert!(!numeric_zeros.is_alias);
        assert!(numeric_zeros.active);
        assert_eq!(numeric_zeros.skip_reason, None);
    }

    #[test]
    fn marks_record_skipped_when_operand_syntax_is_malformed() {
        let json = r#"{
            "!instanceof": { "Instruction": ["MalformedOperand"] },
            "MalformedOperand": {
                "AsmString": "add $Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": false,
                "isCodeGenOnly": false,
                "isAsmParserOnly": false
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let malformed = record(&records, "MalformedOperand");

        assert!(!malformed.active);
        assert_eq!(malformed.skip_reason, Some(SkipReason::MalformedOperand));
        assert_eq!(malformed.operands.len(), 3);
        assert_eq!(malformed.operands[2].raw_class, "imm0_4095");
        assert_eq!(malformed.operands[2].kind, OperandKind::Unknown);
    }

    #[test]
    fn parses_real_llvm_dag_operand_lists() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDWriRealOperands"] },
            "ADDWriRealOperands": {
                "AsmString": "add\t$Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [[{ "def": "GPR32sp", "kind": "def", "printable": "GPR32sp" }, "Rd"]],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs GPR32sp:$Rd)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "GPR32sp", "kind": "def", "printable": "GPR32sp" }, "Rn"],
                        [{ "def": "addsub_shifted_imm32", "kind": "def", "printable": "addsub_shifted_imm32" }, "imm"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins GPR32sp:$Rn, addsub_shifted_imm32:$imm)"
                },
                "bits": "0001000100??????????????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let add = record(&records, "ADDWriRealOperands");

        assert_eq!(add.operands.len(), 3);
        assert_eq!(add.operands[0].name, "Rd");
        assert_eq!(add.operands[0].raw_class, "GPR32sp");
        assert_eq!(add.operands[0].kind, OperandKind::Register);
        assert_eq!(add.operands[0].role, OperandRole::Write);
        assert_eq!(add.operands[1].name, "Rn");
        assert_eq!(add.operands[1].raw_class, "GPR32sp");
        assert_eq!(add.operands[1].role, OperandRole::Read);
        assert_eq!(add.operands[2].name, "imm");
        assert_eq!(add.operands[2].raw_class, "addsub_shifted_imm32");
        assert_eq!(add.operands[2].kind, OperandKind::Immediate);
        assert_ne!(add.skip_reason, Some(SkipReason::MalformedOperand));
    }

    #[test]
    fn parses_real_llvm_predicate_objects_from_def_or_printable_fields() {
        let json = r#"{
            "!instanceof": { "Instruction": ["PredicateObjectInst"] },
            "PredicateObjectInst": {
                "AsmString": "abs\t$Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs FPR32:$Rd)",
                "InOperandList": "(ins FPR32:$Rn)",
                "bits": "00011110001000001100000000000000",
                "Predicates": [
                    { "def": "HasNEON", "kind": "def", "printable": "HasNEON" },
                    { "kind": "def", "printable": "HasSVEorStreamingSVE" },
                    "HasFullFP16"
                ],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let predicate_object = record(&records, "PredicateObjectInst");

        assert_eq!(
            predicate_object.features,
            vec!["HasFullFP16", "HasNEON", "HasSVEorStreamingSVE"]
        );
        assert_eq!(predicate_object.family, InstructionFamily::Sve);
    }

    #[test]
    fn activates_simd_fp_family_when_operands_are_supported() {
        let json = r#"{
            "!instanceof": { "Instruction": ["NeonAbsSupportedOperands"] },
            "NeonAbsSupportedOperands": {
                "AsmString": "abs\t$Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs FPR32:$Rd)",
                "InOperandList": "(ins FPR32:$Rn)",
                "bits": "00011110001000001100000000000000",
                "Predicates": ["HasNEON"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let neon_abs = record(&records, "NeonAbsSupportedOperands");

        assert_eq!(neon_abs.family, InstructionFamily::SimdFp);
        assert_eq!(neon_abs.operands.len(), 2);
        assert!(
            neon_abs
                .operands
                .iter()
                .all(|operand| operand.kind == OperandKind::Register)
        );
        assert!(neon_abs.active);
        assert_eq!(neon_abs.skip_reason, None);
    }

    #[test]
    fn skips_initial_family_record_with_unmapped_feature_predicate() {
        let json = r#"{
            "!instanceof": { "Instruction": ["BranchWithLs64Predicate"] },
            "BranchWithLs64Predicate": {
                "AsmString": "b.$cond\t$label",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins ccode:$cond, am_brcond:$label)",
                "bits": "01010100???????????????????0????",
                "Predicates": ["HasLS64"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let branch = record(&records, "BranchWithLs64Predicate");

        assert_eq!(branch.family, InstructionFamily::Branch);
        assert_eq!(branch.encoding_mask, Some(0xff00_0010));
        assert!(
            branch
                .operands
                .iter()
                .all(|operand| matches!(operand.kind, OperandKind::Condition | OperandKind::Label))
        );
        assert!(!branch.active);
        assert_eq!(branch.skip_reason, Some(SkipReason::UnsupportedFeature));
    }

    #[test]
    fn keeps_initial_low_risk_families_active_with_supported_operands() {
        let json = r#"{
            "!instanceof": { "Instruction": ["BranchSupported", "DataProcessingSupported"] },
            "BranchSupported": {
                "AsmString": "b.$cond\t$label",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins ccode:$cond, am_brcond:$label)",
                "bits": "01010100???????????????????0????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "DataProcessingSupported": {
                "AsmString": "add\t$Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$Rd)",
                "InOperandList": "(ins GPR32:$Rn, imm0_4095:$imm)",
                "bits": "0001000100??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let branch = record(&records, "BranchSupported");
        let data_processing = record(&records, "DataProcessingSupported");

        assert_eq!(branch.family, InstructionFamily::Branch);
        assert!(branch.active);
        assert_eq!(branch.skip_reason, None);
        assert_eq!(data_processing.family, InstructionFamily::DataProcessing);
        assert!(data_processing.active);
        assert_eq!(data_processing.skip_reason, None);
    }

    #[test]
    fn skips_load_store_with_fpr_operand_schema_until_feature_mapping_is_ready() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LDRSUI"] },
            "LDRSUI": {
                "AsmString": "ldr\t$Rt, [$Rn, $offset]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs FPR32:$Rt)",
                "InOperandList": "(ins GPR64sp:$Rn, uimm12s4:$offset)",
                "bits": "1011110101??????????????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let ldrsui = record(&records, "LDRSUI");

        assert_eq!(ldrsui.family, InstructionFamily::LoadStore);
        assert_eq!(ldrsui.encoding_mask, Some(0xffc0_0000));
        assert_eq!(ldrsui.encoding_value, Some(0xbd40_0000));
        assert!(
            ldrsui
                .operands
                .iter()
                .any(|operand| operand.raw_class == "FPR32")
        );
        assert!(!ldrsui.active);
        assert_eq!(
            ldrsui.skip_reason,
            Some(SkipReason::UnsupportedOperandSchema)
        );
    }

    #[test]
    fn skips_known_encoding_when_operand_schema_is_not_ready_for_emission() {
        let json = r#"{
            "!instanceof": { "Instruction": ["VectorListRealInst"] },
            "VectorListRealInst": {
                "AsmString": "tbl\t$Rd, $Rn, $Rm",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [[{ "def": "FPR128", "kind": "def", "printable": "FPR128" }, "Rd"]],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs FPR128:$Rd)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "VecListTwo128", "kind": "def", "printable": "VecListTwo128" }, "Rn"],
                        [{ "def": "FPR128", "kind": "def", "printable": "FPR128" }, "Rm"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins VecListTwo128:$Rn, FPR128:$Rm)"
                },
                "bits": "00001110000000000000000000000000",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let vector = record(&records, "VectorListRealInst");

        assert_eq!(vector.encoding_mask, Some(0xffff_ffff));
        assert!(!vector.active);
        assert_eq!(
            vector.skip_reason,
            Some(SkipReason::UnsupportedOperandSchema)
        );
    }

    #[test]
    fn classifies_simd_vector_register_operand_classes() {
        let json = r#"{
            "!instanceof": { "Instruction": ["VectorOperands"] },
            "VectorOperands": {
                "AsmString": "abs\t$Rd, $Rn, $Rm",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [[{ "def": "V128", "kind": "def", "printable": "V128" }, "Rd"]],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs V128:$Rd)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "V64", "kind": "def", "printable": "V64" }, "Rn"],
                        [{ "def": "V128_lo", "kind": "def", "printable": "V128_lo" }, "Rm"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins V64:$Rn, V128_lo:$Rm)"
                },
                "bits": "01001110001000001011100000000000",
                "Predicates": ["HasNEON"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let vector = record(&records, "VectorOperands");

        assert_eq!(vector.family, InstructionFamily::SimdFp);
        assert_eq!(vector.operands.len(), 3);
        assert_eq!(vector.operands[0].raw_class, "V128");
        assert_eq!(vector.operands[0].kind, OperandKind::Register);
        assert_eq!(vector.operands[1].raw_class, "V64");
        assert_eq!(vector.operands[1].kind, OperandKind::Register);
        assert_eq!(vector.operands[2].raw_class, "V128_lo");
        assert_eq!(vector.operands[2].kind, OperandKind::Register);
        assert!(vector.active);
        assert_eq!(vector.skip_reason, None);
    }

    #[test]
    fn activates_representative_m5_sve_predicate_vector_records() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ABS_ZPmZ_B", "MOVPRFX_ZZ"] },
            "ABS_ZPmZ_B": {
                "AsmString": "abs\t$Zd, $Pg/m, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$_Zd)",
                "InOperandList": "(ins ZPR8:$_Zd, PPR3bAny:$Pg, ZPR8:$Zn)",
                "Constraints": "$_Zd = $_Zd",
                "bits": "0000010000010110101?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "MOVPRFX_ZZ": {
                "AsmString": "movprfx\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPRAny:$Zd)",
                "InOperandList": "(ins ZPRAny:$Zn)",
                "bits": "0000010000100000101111??????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let abs = record(&records, "ABS_ZPmZ_B");
        let movprfx = record(&records, "MOVPRFX_ZZ");

        assert_eq!(abs.family, InstructionFamily::Sve);
        assert_eq!(abs.operands.len(), 3);
        assert_eq!(abs.operands[0].raw_class, "ZPR8");
        assert_eq!(abs.operands[0].kind, OperandKind::Register);
        assert_eq!(abs.operands[0].role, OperandRole::ReadWrite);
        assert_eq!(abs.operands[1].raw_class, "PPR3bAny");
        assert_eq!(abs.operands[1].kind, OperandKind::Predicate);
        assert!(abs.active);
        assert_eq!(abs.skip_reason, None);

        assert_eq!(movprfx.family, InstructionFamily::Sve);
        assert!(movprfx.active);
        assert_eq!(movprfx.skip_reason, None);
    }

    #[test]
    fn activates_representative_m5_sme_tile_predicate_records() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDHA_MPPZ_D"] },
            "ADDHA_MPPZ_D": {
                "AsmString": "addha\t$ZAda, $Pn, $Pm, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs TileOp64:$ZAda)",
                "InOperandList": "(ins PPR3bAny:$Pn, PPR3bAny:$Pm, ZPR64:$Zn)",
                "bits": "1100000011010000???????????00???",
                "Predicates": ["HasSMEI64"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let addha = record(&records, "ADDHA_MPPZ_D");

        assert_eq!(addha.family, InstructionFamily::Sme);
        assert_eq!(addha.operands[0].raw_class, "TileOp64");
        assert_eq!(addha.operands[0].kind, OperandKind::Register);
        assert_eq!(addha.operands[1].kind, OperandKind::Predicate);
        assert_eq!(addha.operands[3].raw_class, "ZPR64");
        assert!(addha.active);
        assert_eq!(addha.skip_reason, None);
    }

    #[test]
    fn classifies_unmapped_sve_sme_feature_records_by_operand_schema() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SVEUnknownSchema"] },
            "SVEUnknownSchema": {
                "AsmString": "unknown\t$Zd, $mystery",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins MysterySVEOperand:$mystery)",
                "bits": "0000010000010110101?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let unknown = record(&records, "SVEUnknownSchema");

        assert_eq!(unknown.family, InstructionFamily::Sve);
        assert!(!unknown.active);
        assert_eq!(unknown.skip_reason, Some(SkipReason::UnknownOperand));
    }

    #[test]
    fn classifies_sve_records_with_unparsed_encoding_as_missing_encoding() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SVEUnknownEncoding"] },
            "SVEUnknownEncoding": {
                "AsmString": "movprfx\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPRAny:$Zd)",
                "InOperandList": "(ins ZPRAny:$Zn)",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let unknown = record(&records, "SVEUnknownEncoding");

        assert_eq!(unknown.family, InstructionFamily::Sve);
        assert!(!unknown.active);
        assert_eq!(unknown.skip_reason, Some(SkipReason::MissingEncoding));
    }

    #[test]
    fn classifies_llvm23_sme2_vector_group_and_matrix_operands() {
        let cases = [
            ("ZZ_b_mul_r", "_Zdn", OperandKind::VectorList),
            ("ZZ_h_mul_r_Lo", "Zn", OperandKind::VectorList),
            ("ZZ_h_mul_r_Hi", "Zm", OperandKind::VectorList),
            ("ZZZZ_d_mul_r", "Zn", OperandKind::VectorList),
            ("ZZZZ_b_strided", "Zt", OperandKind::VectorList),
            ("MatrixOp16", "_ZAdn", OperandKind::Register),
            ("MatrixOp32", "_ZAda", OperandKind::Register),
            ("MatrixOp64", "_ZAd", OperandKind::Register),
            ("TileOp16", "_ZAda", OperandKind::Register),
            ("ZK", "Zk", OperandKind::Register),
            ("PP_b_mul_r", "Pd", OperandKind::Predicate),
            ("PP_d_mul_r", "Pd", OperandKind::Predicate),
            ("PP_h_mul_r", "Pd", OperandKind::Predicate),
            ("PP_s_mul_r", "Pd", OperandKind::Predicate),
            ("ZZ_mul_r", "Zn", OperandKind::VectorList),
            ("ZZ_Any", "Zm", OperandKind::VectorList),
            ("sme_elm_idx0_3", "off2", OperandKind::Immediate),
            ("ZZZ_Any", "Zn", OperandKind::VectorList),
        ];

        for (raw_class, name, expected) in cases {
            assert_eq!(
                classify_operand_kind(raw_class, name),
                expected,
                "{raw_class}:{name}"
            );
        }
    }

    #[test]
    fn activates_llvm23_sve_sme_union_feature_predicates() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ABS_ZPmZ_B", "ADD_VG2_2ZZ_B", "SMOPA_SMOPA"] },
            "ABS_ZPmZ_B": {
                "AsmString": "abs\t$Zd, $Pg/m, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$_Zd)",
                "InOperandList": "(ins ZPR8:$_Zd, PPR3bAny:$Pg, ZPR8:$Zn)",
                "Constraints": "$_Zd = $_Zd",
                "bits": "0000010000010110101?????????????",
                "Predicates": ["HasSVE_or_SME"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ADD_VG2_2ZZ_B": {
                "AsmString": "add\t$Zdn, $Zdn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR4b8:$_Zdn)",
                "InOperandList": "(ins ZPR4b8:$_Zdn, ZPR4b8:$Zm)",
                "Constraints": "$_Zdn = $_Zdn",
                "bits": "1100000100100000100000??????????",
                "Predicates": ["HasSME2"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "SMOPA_SMOPA": {
                "AsmString": "smopa\t$ZAda.S, $Pn/M, $Pm/M, $Zn.B, $Zm.B",
                "Namespace": "AArch64",
                "OutOperandList": "(outs MatrixOp:$_ZAda)",
                "InOperandList": "(ins MatrixOp:$_ZAda, PPRAny:$Pn, PPRAny:$Pm, ZPR8:$Zn, ZPR8:$Zm)",
                "Constraints": "$_ZAda = $_ZAda",
                "bits": "1000000000000000000000??????????",
                "Predicates": ["HasSMEI16I64"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let sve = record(&records, "ABS_ZPmZ_B");
        let sme2 = record(&records, "ADD_VG2_2ZZ_B");
        let sme_i16i64 = record(&records, "SMOPA_SMOPA");

        assert!(sve.active);
        assert_eq!(sve.skip_reason, None);
        assert!(sme2.active);
        assert_eq!(sme2.skip_reason, None);
        assert!(sme_i16i64.active);
        assert_eq!(sme_i16i64.skip_reason, None);
    }

    #[test]
    fn activates_m5_fp8_and_faminmax_report_buckets() {
        let json = r#"{
            "!instanceof": { "Instruction": ["FAMIN_ZPmZ_H", "F1CVT_2ZZ_BtoH", "F1CVT_ZZ_BtoH", "FDOT_VG2_M2ZZ_BtoS"] },
            "FAMIN_ZPmZ_H": {
                "AsmString": "famin\t$Zdn, $Pg/m, $_Zdn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR16:$Zdn)",
                "InOperandList": "(ins ZPR16:$_Zdn, PPR3bAny:$Pg, ZPR16:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "0110010101001111100?????????????",
                "Predicates": ["HasNonStreamingSVE2_or_SME2", "HasFAMINMAX"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "F1CVT_2ZZ_BtoH": {
                "AsmString": "f1cvt\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZZ_h_mul_r:$Zd)",
                "InOperandList": "(ins ZPR8:$Zn)",
                "bits": "1100000100100110111000?????????0",
                "Predicates": ["HasSME2", "HasFP8"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "F1CVT_ZZ_BtoH": {
                "AsmString": "f1cvt\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR16:$Zd)",
                "InOperandList": "(ins ZPR8:$Zn)",
                "bits": "0110010100001000001100??????????",
                "Predicates": ["HasNonStreamingSVE2_or_SME2", "HasFP8"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "FDOT_VG2_M2ZZ_BtoS": {
                "AsmString": "fdot\t$_ZAd, $Rv, $imm3, $Zn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs MatrixOp32:$_ZAd)",
                "InOperandList": "(ins MatrixIndexGPR32Op8_11:$Rv, sme_elm_idx0_7:$imm3, ZZ_b:$Zn, ZPR4b8:$Zm)",
                "bits": "11000001001000000001????????????",
                "Predicates": ["HasSMEF8F32"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let famin = record(&records, "FAMIN_ZPmZ_H");
        let sme_fp8 = record(&records, "F1CVT_2ZZ_BtoH");
        let sve_fp8 = record(&records, "F1CVT_ZZ_BtoH");
        let sme_fp8_f32_fdot = record(&records, "FDOT_VG2_M2ZZ_BtoS");

        assert_eq!(famin.family, InstructionFamily::Sve);
        assert!(famin.active);
        assert_eq!(famin.skip_reason, None);
        assert_eq!(sme_fp8.family, InstructionFamily::Sme);
        assert_eq!(sme_fp8.operands[0].raw_class, "ZZ_h_mul_r");
        assert_eq!(sme_fp8.operands[0].kind, OperandKind::VectorList);
        assert!(sme_fp8.active);
        assert_eq!(sme_fp8.skip_reason, None);
        assert_eq!(sve_fp8.family, InstructionFamily::Sve);
        assert!(sve_fp8.active);
        assert_eq!(sve_fp8.skip_reason, None);
        assert_eq!(sme_fp8_f32_fdot.family, InstructionFamily::Sme);
        assert_eq!(sme_fp8_f32_fdot.features, vec!["HasSMEF8F32"]);
        assert_eq!(sme_fp8_f32_fdot.operands[0].raw_class, "MatrixOp32");
        assert_eq!(sme_fp8_f32_fdot.operands[3].raw_class, "ZZ_b");
        assert!(sme_fp8_f32_fdot.active);
        assert_eq!(sme_fp8_f32_fdot.skip_reason, None);
    }

    #[test]
    fn activates_sve2_aes_feature_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["AESE_ZZZ_B"] },
            "AESE_ZZZ_B": {
                "AsmString": "aese\t$Zdn, $_Zdn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zdn)",
                "InOperandList": "(ins ZPR8:$_Zdn, ZPR8:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "0100010100100010111000??????????",
                "Predicates": ["HasSVE2AES"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let aese = record(&records, "AESE_ZZZ_B");

        assert_eq!(aese.family, InstructionFamily::Sve);
        assert_eq!(aese.features, vec!["HasSVE2AES"]);
        assert_eq!(aese.operands[0].role, OperandRole::ReadWrite);
        assert!(aese.active);
        assert_eq!(aese.skip_reason, None);
    }

    #[test]
    fn activates_sve_complex_rotate_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["CADD_ZZI_B"] },
            "CADD_ZZI_B": {
                "AsmString": "cadd\t$Zdn, $_Zdn, $Zm, #$rot",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zdn)",
                "InOperandList": "(ins ZPR8:$_Zdn, ZPR8:$Zm, complexrotateopodd:$rot)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "01000101000?????110110??????????",
                "Predicates": ["HasSVE2"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let cadd = record(&records, "CADD_ZZI_B");

        assert_eq!(cadd.family, InstructionFamily::Sve);
        assert_eq!(cadd.operands[0].role, OperandRole::ReadWrite);
        assert_eq!(cadd.operands[2].raw_class, "complexrotateopodd");
        assert_eq!(cadd.operands[2].kind, OperandKind::Immediate);
        assert!(cadd.active);
        assert_eq!(cadd.skip_reason, None);
    }

    #[test]
    fn activates_sve2_sha3_feature_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["RAX1_ZZZ_D"] },
            "RAX1_ZZZ_D": {
                "AsmString": "rax1\t$Zd, $Zn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR64:$Zd)",
                "InOperandList": "(ins ZPR64:$Zn, ZPR64:$Zm)",
                "bits": "0100010100111111110100??????????",
                "Predicates": ["HasSVESHA3", "HasNonStreamingSVE_or_SME2p1"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let rax1 = record(&records, "RAX1_ZZZ_D");

        assert_eq!(rax1.family, InstructionFamily::Sve);
        assert_eq!(
            rax1.features,
            vec!["HasNonStreamingSVE_or_SME2p1", "HasSVESHA3"]
        );
        assert_eq!(rax1.operands.len(), 3);
        assert!(rax1.active);
        assert_eq!(rax1.skip_reason, None);
    }

    #[test]
    fn activates_sve2_sm4_feature_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SM4E_ZZZ_S"] },
            "SM4E_ZZZ_S": {
                "AsmString": "sm4e\t$Zdn, $_Zdn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR32:$Zdn)",
                "InOperandList": "(ins ZPR32:$_Zdn, ZPR32:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "0100010100100011111000??????????",
                "Predicates": ["HasSVESM4"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let sm4e = record(&records, "SM4E_ZZZ_S");

        assert_eq!(sm4e.family, InstructionFamily::Sve);
        assert_eq!(sm4e.features, vec!["HasSVESM4"]);
        assert_eq!(sm4e.operands[0].role, OperandRole::ReadWrite);
        assert!(sm4e.active);
        assert_eq!(sm4e.skip_reason, None);
    }

    #[test]
    fn activates_sve2_bitperm_feature_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["BDEP_ZZZ_B"] },
            "BDEP_ZZZ_B": {
                "AsmString": "bdep\t$Zd, $Zn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins ZPR8:$Zn, ZPR8:$Zm)",
                "bits": "0100010100011111101100??????????",
                "Predicates": ["HasSVE2BitPerm"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let bdep = record(&records, "BDEP_ZZZ_B");

        assert_eq!(bdep.family, InstructionFamily::Sve);
        assert_eq!(bdep.features, vec!["HasSVE2BitPerm"]);
        assert!(bdep.active);
        assert_eq!(bdep.skip_reason, None);
    }

    #[test]
    fn classifies_sve_records_with_unmapped_non_schema_feature_as_unsupported_feature() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SVEUnsupportedFeature"] },
            "SVEUnsupportedFeature": {
                "AsmString": "movprfx\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPRAny:$Zd)",
                "InOperandList": "(ins ZPRAny:$Zn)",
                "bits": "0000010000100000101111??????????",
                "Predicates": ["HasUnsupportedFutureFeature"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let unsupported = record(&records, "SVEUnsupportedFeature");

        assert_eq!(unsupported.family, InstructionFamily::Sve);
        assert!(!unsupported.active);
        assert_eq!(
            unsupported.skip_reason,
            Some(SkipReason::UnsupportedFeature)
        );
    }

    #[test]
    fn classifies_sve_records_with_unmapped_namespace() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SVEUnsupportedNamespace"] },
            "SVEUnsupportedNamespace": {
                "AsmString": "movprfx\t$Zd, $Zn",
                "Namespace": "AArch64Experimental",
                "OutOperandList": "(outs ZPRAny:$Zd)",
                "InOperandList": "(ins ZPRAny:$Zn)",
                "bits": "0000010000100000101111??????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let unsupported = record(&records, "SVEUnsupportedNamespace");

        assert_eq!(unsupported.family, InstructionFamily::Sve);
        assert!(!unsupported.active);
        assert_eq!(
            unsupported.skip_reason,
            Some(SkipReason::UnsupportedNamespace)
        );
    }

    #[test]
    fn classifies_sve_pseudo_records_as_pseudo() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SVEPseudo"] },
            "SVEPseudo": {
                "AsmString": "movprfx\t$Zd, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPRAny:$Zd)",
                "InOperandList": "(ins ZPRAny:$Zn)",
                "bits": "0000010000100000101111??????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 1,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let pseudo = record(&records, "SVEPseudo");

        assert_eq!(pseudo.family, InstructionFamily::Sve);
        assert!(!pseudo.active);
        assert_eq!(pseudo.skip_reason, Some(SkipReason::Pseudo));
    }

    #[test]
    fn activates_sve_pattern_count_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["CNTB_XPiI"] },
            "CNTB_XPiI": {
                "AsmString": "cntb\t$Rd, $pattern, mul #$imm4",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR64:$Rd)",
                "InOperandList": "(ins sve_pred_enum:$pattern, sve_incdec_imm:$imm4)",
                "bits": "000001000010????111000??????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let cntb = record(&records, "CNTB_XPiI");

        assert_eq!(cntb.family, InstructionFamily::Sve);
        assert_eq!(cntb.operands[1].raw_class, "sve_pred_enum");
        assert_eq!(cntb.operands[1].kind, OperandKind::Immediate);
        assert_eq!(cntb.operands[2].raw_class, "sve_incdec_imm");
        assert_eq!(cntb.operands[2].kind, OperandKind::Immediate);
        assert!(cntb.active);
        assert_eq!(cntb.skip_reason, None);
    }

    #[test]
    fn activates_sve_tuple_memory_vector_list_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LD2B", "LD3B", "ST4D"] },
            "LD2B": {
                "AsmString": "ld2b\t$Zt, $Pg/z, [$Rn, $Rm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZZ_b:$Zt)",
                "InOperandList": "(ins PPR3bAny:$Pg, GPR64sp:$Rn, GPR64NoXZRshifted8:$Rm)",
                "bits": "10100100001?????110?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "LD3B": {
                "AsmString": "ld3b\t$Zt, $Pg/z, [$Rn, $Rm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZZZ_b:$Zt)",
                "InOperandList": "(ins PPR3bAny:$Pg, GPR64sp:$Rn, GPR64NoXZRshifted8:$Rm)",
                "bits": "10100100010?????110?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ST4D": {
                "AsmString": "st4d\t$Zt, $Pg, [$Rn, $Rm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins ZZZZ_d:$Zt, PPR3bAny:$Pg, GPR64sp:$Rn, GPR64NoXZRshifted64:$Rm)",
                "bits": "11100101111?????011?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let ld2b = record(&records, "LD2B");
        let ld3b = record(&records, "LD3B");
        let st4d = record(&records, "ST4D");

        assert_eq!(ld2b.operands[0].kind, OperandKind::VectorList);
        assert_eq!(ld3b.operands[0].raw_class, "ZZZ_b");
        assert_eq!(st4d.operands[0].raw_class, "ZZZZ_d");
        assert!(ld2b.active);
        assert!(ld3b.active);
        assert!(st4d.active);
        assert_eq!(ld2b.skip_reason, None);
        assert_eq!(ld3b.skip_reason, None);
        assert_eq!(st4d.skip_reason, None);
    }

    #[test]
    fn activates_sme_tile_vector_index_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LD1_MXIPXX_H_B"] },
            "LD1_MXIPXX_H_B": {
                "AsmString": "ld1b\t$ZAt[$Rv, #$imm], $Pg/z, [$Rn, $Rm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs TileVectorOpH8:$ZAt)",
                "InOperandList": "(ins MatrixIndexGPR32Op12_15:$Rv, sme_elm_idx0_15:$imm, PPR3bAny:$Pg, GPR64sp:$Rn, GPR64shifted8:$Rm)",
                "bits": "11100000000?????0??????????0????",
                "Predicates": ["HasSME"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let ld1 = record(&records, "LD1_MXIPXX_H_B");

        assert_eq!(ld1.family, InstructionFamily::Sme);
        assert!(ld1.active);
        assert_eq!(ld1.skip_reason, None);
    }

    #[test]
    fn activates_sve_shift_immediate_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ASR_ZZI_B", "LSL_ZPMI_S"] },
            "ASR_ZZI_B": {
                "AsmString": "asr\t$Zd, $Zn, #$imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins ZPR8:$Zn, vecshiftR8:$imm)",
                "bits": "0000010000101???100100??????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "LSL_ZPMI_S": {
                "AsmString": "lsl\t$Zdn, $Pg/m, $Zdn, #$imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR32:$_Zdn)",
                "InOperandList": "(ins ZPR32:$_Zdn, PPR3bAny:$Pg, vecshiftL32:$imm)",
                "Constraints": "$_Zdn = $_Zdn",
                "bits": "0000010001000011100?????????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let asr = record(&records, "ASR_ZZI_B");
        let lsl = record(&records, "LSL_ZPMI_S");

        assert_eq!(asr.family, InstructionFamily::Sve);
        assert_eq!(asr.operands[2].raw_class, "vecshiftR8");
        assert_eq!(asr.operands[2].kind, OperandKind::Immediate);
        assert!(asr.active);
        assert_eq!(asr.skip_reason, None);

        assert_eq!(lsl.operands[2].raw_class, "vecshiftL32");
        assert_eq!(lsl.operands[2].kind, OperandKind::Immediate);
        assert_eq!(lsl.operands[0].role, OperandRole::ReadWrite);
        assert!(lsl.active);
        assert_eq!(lsl.skip_reason, None);
    }

    #[test]
    fn activates_sve_exact_fp_immediate_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["FADD_ZPMI_D"] },
            "FADD_ZPMI_D": {
                "AsmString": "fadd\t$Zdn, $Pg/m, $Zdn, #$i1",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR64:$_Zdn)",
                "InOperandList": "(ins ZPR64:$_Zdn, PPR3bAny:$Pg, sve_fpimm_half_one:$i1)",
                "Constraints": "$_Zdn = $_Zdn",
                "bits": "0110010111011000100???0000??????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let fadd = record(&records, "FADD_ZPMI_D");

        assert_eq!(fadd.family, InstructionFamily::Sve);
        assert_eq!(fadd.operands[2].raw_class, "sve_fpimm_half_one");
        assert_eq!(fadd.operands[2].kind, OperandKind::Immediate);
        assert_eq!(fadd.operands[0].role, OperandRole::ReadWrite);
        assert!(fadd.active);
        assert_eq!(fadd.skip_reason, None);
    }

    #[test]
    fn classifies_complex_rotate_operand_classes_as_immediates() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ComplexRotateOperands"] },
            "ComplexRotateOperands": {
                "AsmString": "fcmla\t$Rd, $Rn, $Rm, #$rot, #$rot_odd",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [[{ "def": "V128", "kind": "def", "printable": "V128" }, "Rd"]],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs V128:$Rd)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "V128", "kind": "def", "printable": "V128" }, "Rn"],
                        [{ "def": "V128", "kind": "def", "printable": "V128" }, "Rm"],
                        [{ "def": "complexrotateop", "kind": "def", "printable": "complexrotateop" }, "rot"],
                        [{ "def": "complexrotateopodd", "kind": "def", "printable": "complexrotateopodd" }, "rot_odd"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins V128:$Rn, V128:$Rm, complexrotateop:$rot, complexrotateopodd:$rot_odd)"
                },
                "bits": "00101110001000001100100000000000",
                "Predicates": ["HasNEON"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let complex = record(&records, "ComplexRotateOperands");

        assert_eq!(complex.operands[3].raw_class, "complexrotateop");
        assert_eq!(complex.operands[3].kind, OperandKind::Immediate);
        assert_eq!(complex.operands[4].raw_class, "complexrotateopodd");
        assert_eq!(complex.operands[4].kind, OperandKind::Immediate);
        assert_ne!(complex.skip_reason, Some(SkipReason::UnknownOperand));
    }

    #[test]
    fn classifies_fixed_point_conversion_operand_classes_as_immediates() {
        let json = r#"{
            "!instanceof": { "Instruction": ["FixedPointOperands"] },
            "FixedPointOperands": {
                "AsmString": "scvtf\t$Rd, $Rn, #$fbits32, #$fbits64",
                "Namespace": "AArch64",
                "OutOperandList": {
                    "args": [[{ "def": "FPR32", "kind": "def", "printable": "FPR32" }, "Rd"]],
                    "kind": "dag",
                    "operator": { "def": "outs", "kind": "def", "printable": "outs" },
                    "printable": "(outs FPR32:$Rd)"
                },
                "InOperandList": {
                    "args": [
                        [{ "def": "GPR32", "kind": "def", "printable": "GPR32" }, "Rn"],
                        [{ "def": "fixedpoint_f32_i32", "kind": "def", "printable": "fixedpoint_f32_i32" }, "fbits32"],
                        [{ "def": "fixedpoint_f64_i64", "kind": "def", "printable": "fixedpoint_f64_i64" }, "fbits64"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins GPR32:$Rn, fixedpoint_f32_i32:$fbits32, fixedpoint_f64_i64:$fbits64)"
                },
                "bits": "00011110001000000000000000000000",
                "Predicates": ["HasV8_0a"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let fixed = record(&records, "FixedPointOperands");

        assert_eq!(fixed.operands[2].raw_class, "fixedpoint_f32_i32");
        assert_eq!(fixed.operands[2].kind, OperandKind::Immediate);
        assert_eq!(fixed.operands[3].raw_class, "fixedpoint_f64_i64");
        assert_eq!(fixed.operands[3].kind, OperandKind::Immediate);
        assert_ne!(fixed.skip_reason, Some(SkipReason::UnknownOperand));
    }

    #[test]
    fn classifies_remaining_m3_operand_schema_families() {
        let cases = [
            ("arith_extendlsl64", "extend", OperandKind::Immediate),
            ("ro_Wextend64", "addr", OperandKind::Memory),
            ("ro_Xextend64", "addr", OperandKind::Memory),
            ("sys_cr_op", "op", OperandKind::Immediate),
            ("pstatefield4_op", "field", OperandKind::SystemRegister),
            ("barrier_op", "barrier", OperandKind::Immediate),
            ("barrier_nxs_op", "barrier", OperandKind::Immediate),
            ("WSeqPairClassOperand", "Rs", OperandKind::Register),
            ("XSeqPairClassOperand", "Rs", OperandKind::Register),
            ("sve_prfop", "prfop", OperandKind::Immediate),
            ("prfop", "Rt", OperandKind::Immediate),
            ("Z_d", "Zt", OperandKind::Register),
            ("ZZZ_s", "Zt", OperandKind::VectorList),
            ("sve_elm_idx_extdup_b", "idx", OperandKind::Immediate),
            ("TileOp32", "ZAda", OperandKind::Register),
            ("MatrixOp", "ZA", OperandKind::Register),
        ];

        for (raw_class, name, expected) in cases {
            assert_eq!(
                classify_operand_kind(raw_class, name),
                expected,
                "{raw_class}:{name}"
            );
        }
    }

    #[test]
    fn activates_representative_m4_scalar_fp_and_asimd_records() {
        let json = r#"{
            "!instanceof": { "Instruction": ["FADDSrr", "DUPv8i8gpr"] },
            "FADDSrr": {
                "AsmString": "fadd\t$Rd, $Rn, $Rm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs FPR32:$Rd)",
                "InOperandList": "(ins FPR32:$Rn, FPR32:$Rm)",
                "bits": "00011110001?????001010??????????",
                "Predicates": ["HasFPARMv8"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "DUPv8i8gpr": {
                "AsmString": "dup\t$Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs V64:$Rd)",
                "InOperandList": "(ins GPR32:$Rn)",
                "bits": "00001110000????1000011??????????",
                "Predicates": ["HasNEON"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let fadd = record(&records, "FADDSrr");
        let dup = record(&records, "DUPv8i8gpr");

        assert_eq!(fadd.family, InstructionFamily::SimdFp);
        assert_eq!(
            fadd.groups,
            vec![InstructionGroup::Vector, InstructionGroup::Float]
        );
        assert!(fadd.active);
        assert_eq!(fadd.skip_reason, None);
        assert_eq!(dup.family, InstructionFamily::SimdFp);
        assert_eq!(
            dup.groups,
            vec![InstructionGroup::Vector, InstructionGroup::Float]
        );
        assert!(dup.active);
        assert_eq!(dup.skip_reason, None);
    }

    #[test]
    fn activates_representative_m4_crypto_and_memtag_records() {
        let json = r#"{
            "!instanceof": { "Instruction": ["AESErr", "ADDG"] },
            "AESErr": {
                "AsmString": "aese\t$Rd, $Rn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs V128:$Rd)",
                "InOperandList": "(ins V128:$Rn)",
                "bits": "0100111000101000010010??????????",
                "Predicates": ["HasAES"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ADDG": {
                "AsmString": "addg\t$Rd, $Rn, #$imm6, #$imm4",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR64sp:$Rd)",
                "InOperandList": "(ins GPR64sp:$Rn, uimm6s16:$imm6, imm0_15:$imm4)",
                "bits": "1001000110??????00??????????????",
                "Predicates": ["HasMTE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let aes = record(&records, "AESErr");
        let addg = record(&records, "ADDG");

        assert_eq!(aes.family, InstructionFamily::Crypto);
        assert_eq!(aes.groups, vec![InstructionGroup::Crypto]);
        assert!(aes.active);
        assert_eq!(aes.skip_reason, None);
        assert_eq!(addg.family, InstructionFamily::MemTag);
        assert_eq!(addg.groups, vec![InstructionGroup::MemoryTagging]);
        assert!(addg.active);
        assert_eq!(addg.skip_reason, None);
    }

    #[test]
    fn keeps_complex_m4_vector_list_schema_deferred() {
        let json = r#"{
            "!instanceof": { "Instruction": ["TBLv16i8One"] },
            "TBLv16i8One": {
                "AsmString": "tbl\t$Vd, $Vn, $Vm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs V128:$Vd)",
                "InOperandList": "(ins VecListOne16b:$Vn, V128:$Vm)",
                "bits": "01001110000?????000000??????????",
                "Predicates": ["HasNEON"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let tbl = record(&records, "TBLv16i8One");

        assert_eq!(tbl.family, InstructionFamily::SimdFp);
        assert_eq!(tbl.operands[1].kind, OperandKind::VectorList);
        assert!(!tbl.active);
        assert_eq!(tbl.skip_reason, Some(SkipReason::UnsupportedOperandSchema));
    }

    #[test]
    fn applies_tablegen_tied_operand_constraints_to_lse_cas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["CASW"] },
            "CASW": {
                "AsmString": "cas\t$Rs, $Rt, [$Rn]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32:$out)",
                "InOperandList": "(ins GPR32:$Rs, GPR32:$Rt, GPR64sp:$Rn)",
                "Constraints": "$out = $Rs",
                "bits": "10001000101?????011111??????????",
                "Predicates": ["HasLSE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let cas = record(&records, "CASW");

        assert_eq!(cas.family, InstructionFamily::LoadStore);
        assert_eq!(cas.operands.len(), 3);
        assert_eq!(cas.operands[0].name, "Rs");
        assert_eq!(cas.operands[0].role, OperandRole::ReadWrite);
        assert!(cas.active);
        assert_eq!(cas.skip_reason, None);
    }

    #[test]
    fn activates_prefetch_literal_operand_schema() {
        let json = r#"{
            "!instanceof": { "Instruction": ["PRFMl"] },
            "PRFMl": {
                "AsmString": "prfm\t$Rt, $label",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins prfop:$Rt, am_ldrlit:$label)",
                "bits": "11011000????????????????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let prfm = record(&records, "PRFMl");

        assert_eq!(prfm.family, InstructionFamily::LoadStore);
        assert_eq!(prfm.operands[0].kind, OperandKind::Immediate);
        assert!(prfm.active);
        assert_eq!(prfm.skip_reason, None);
    }

    #[test]
    fn activates_sve_indexed_complex_rotate_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["CDOT_ZZZI_S"] },
            "CDOT_ZZZI_S": {
                "AsmString": "cdot\t$Zda, $Zn, $Zm$iop, $rot",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR32:$Zda)",
                "InOperandList": "(ins ZPR32:$_Zda, ZPR8:$Zn, ZPR3b8:$Zm, VectorIndexS32b:$iop, complexrotateop:$rot)",
                "Constraints": "$Zda = $_Zda",
                "bits": "01000100101?????0100????????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let cdot = record(&records, "CDOT_ZZZI_S");

        assert_eq!(cdot.family, InstructionFamily::Sve);
        assert_eq!(cdot.operands[3].raw_class, "VectorIndexS32b");
        assert_eq!(cdot.operands[3].kind, OperandKind::Immediate);
        assert_eq!(cdot.operands[4].raw_class, "complexrotateop");
        assert_eq!(cdot.operands[4].kind, OperandKind::Immediate);
        assert!(cdot.active);
        assert_eq!(cdot.skip_reason, None);
    }

    #[test]
    fn activates_sve_ext_vector_list_schema() {
        let json = r#"{
            "!instanceof": { "Instruction": ["EXT_ZZI_B"] },
            "EXT_ZZI_B": {
                "AsmString": "ext\t$Zd, $Zn, $imm8",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins ZZ_b:$Zn, imm0_255:$imm8)",
                "bits": "00000101011?????????????????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let ext = record(&records, "EXT_ZZI_B");

        assert_eq!(ext.family, InstructionFamily::Sve);
        assert_eq!(ext.operands[1].raw_class, "ZZ_b");
        assert_eq!(ext.operands[1].kind, OperandKind::VectorList);
        assert_eq!(ext.operands[2].raw_class, "imm0_255");
        assert_eq!(ext.operands[2].kind, OperandKind::Immediate);
        assert!(ext.active);
        assert_eq!(ext.skip_reason, None);
    }

    #[test]
    fn activates_sve2_splice_vector_list_schema() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SPLICE_ZPZZ_B"] },
            "SPLICE_ZPZZ_B": {
                "AsmString": "splice\t$Zd, $Pg, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZZ_b:$Zn)",
                "bits": "0000010100101101100?????????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let splice = record(&records, "SPLICE_ZPZZ_B");

        assert_eq!(splice.family, InstructionFamily::Sve);
        assert_eq!(splice.operands[2].raw_class, "ZZ_b");
        assert_eq!(splice.operands[2].kind, OperandKind::VectorList);
        assert!(splice.active);
        assert_eq!(splice.skip_reason, None);
    }

    #[test]
    fn activates_sve2_tbl_two_register_vector_list_schema() {
        let json = r#"{
            "!instanceof": { "Instruction": ["TBL_ZZZZ_B"] },
            "TBL_ZZZZ_B": {
                "AsmString": "tbl\t$Zd, $Zn, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zd)",
                "InOperandList": "(ins ZZ_b:$Zn, ZPR8:$Zm)",
                "bits": "00000101001?????001010??????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;

        let records = parse_records(json).expect("inline JSON should parse");
        let tbl = record(&records, "TBL_ZZZZ_B");

        assert_eq!(tbl.family, InstructionFamily::Sve);
        assert_eq!(tbl.operands[1].raw_class, "ZZ_b");
        assert_eq!(tbl.operands[1].kind, OperandKind::VectorList);
        assert!(tbl.active);
        assert_eq!(tbl.skip_reason, None);
    }

    fn record<'a>(records: &'a [InstructionRecord], name: &str) -> &'a InstructionRecord {
        records
            .iter()
            .find(|record| record.llvm_name == name)
            .unwrap_or_else(|| panic!("missing record {name}"))
    }
}
