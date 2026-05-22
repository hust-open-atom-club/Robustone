//! AArch64 generator output emission.

use std::collections::{BTreeMap, BTreeSet};

use crate::aarch64_gen::model::{
    InstructionFamily, InstructionGroup, InstructionRecord, OperandKind, OperandRecord, OperandRole,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeaderMetadata {
    pub source: String,
    pub source_hash: String,
    pub command: String,
}

impl HeaderMetadata {
    fn default_generated() -> Self {
        Self {
            source: "LLVM TableGen JSON".to_string(),
            source_hash: "unknown".to_string(),
            command: "cargo xtask aarch64-gen --llvm-project third_party/llvm-project --out-dir robustone-arm/src/backend/generated --artifact-dir target/aarch64-gen".to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn for_tests() -> Self {
        Self {
            source: "test-source".to_string(),
            source_hash: "test-hash".to_string(),
            command: "cargo xtask aarch64-gen --llvm-project third_party/llvm-project --out-dir robustone-arm/src/backend/generated --artifact-dir target/aarch64-gen".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ModuleInfo {
    order: u8,
    filename: &'static str,
    module: &'static str,
}

pub(crate) fn emit_specs(records: &[InstructionRecord]) -> Result<Vec<(String, String)>, String> {
    emit_specs_with_metadata(records, &HeaderMetadata::default_generated())
}

pub(crate) fn emit_specs_with_metadata(
    records: &[InstructionRecord],
    metadata: &HeaderMetadata,
) -> Result<Vec<(String, String)>, String> {
    validate_active_encodings(records)?;

    let mut grouped: BTreeMap<ModuleInfo, Vec<&InstructionRecord>> = BTreeMap::new();
    for record in records.iter().filter(|record| record.active) {
        grouped
            .entry(module_info(&record.family))
            .or_default()
            .push(record);
    }

    let mut files = Vec::with_capacity(grouped.len() + 1);
    let mut modules = Vec::with_capacity(grouped.len());
    for (module, mut records) in grouped {
        records.sort_by(|left, right| left.llvm_name.cmp(&right.llvm_name));
        modules.push(module.module);
        files.push((
            module.filename.to_string(),
            emit_family_file(module, &records, metadata)?,
        ));
    }

    files.push(("mod.rs".to_string(), emit_mod_file(&modules, metadata)));
    Ok(files)
}

pub(crate) fn emit_rust_specs(records: &[InstructionRecord]) -> Result<String, String> {
    emit_rust_specs_with_metadata(records, &HeaderMetadata::default_generated())
}

pub(crate) fn emit_rust_specs_with_metadata(
    records: &[InstructionRecord],
    metadata: &HeaderMetadata,
) -> Result<String, String> {
    let files = emit_specs_with_metadata(records, metadata)?;
    let mut content = String::new();
    for (filename, file_content) in files {
        content.push_str("// ");
        content.push_str(&filename);
        content.push('\n');
        content.push_str(&file_content);
        if !content.ends_with('\n') {
            content.push('\n');
        }
    }
    Ok(content)
}

fn validate_active_encodings(records: &[InstructionRecord]) -> Result<(), String> {
    let mut seen: BTreeMap<(u32, u32), &InstructionRecord> = BTreeMap::new();
    for record in records.iter().filter(|record| record.active) {
        let Some(mask) = record.encoding_mask else {
            return Err(format!(
                "active record {} has no encoding mask",
                record.llvm_name
            ));
        };
        let Some(value) = record.encoding_value else {
            return Err(format!(
                "active record {} has no encoding value",
                record.llvm_name
            ));
        };
        if let Some(existing) = seen.insert((mask, value), record) {
            return Err(format!(
                "conflicting active encodings: {} and {} both use mask 0x{mask:08X} value 0x{value:08X}",
                existing.llvm_name, record.llvm_name
            ));
        }
        for ((existing_mask, existing_value), existing) in &seen {
            if existing.llvm_name == record.llvm_name {
                continue;
            }
            if encodings_overlap(*existing_mask, *existing_value, mask, value)
                && generated_priority(existing) == generated_priority(record)
            {
                return Err(format!(
                    "overlapping active encodings: {} mask 0x{existing_mask:08X} value 0x{existing_value:08X} overlaps {} mask 0x{mask:08X} value 0x{value:08X}",
                    existing.llvm_name, record.llvm_name
                ));
            }
        }
    }
    Ok(())
}

fn encodings_overlap(left_mask: u32, left_value: u32, right_mask: u32, right_value: u32) -> bool {
    ((left_value ^ right_value) & (left_mask & right_mask)) == 0
}

fn module_info(family: &InstructionFamily) -> ModuleInfo {
    match family {
        InstructionFamily::Branch => ModuleInfo {
            order: 0,
            filename: "branch.rs",
            module: "branch",
        },
        InstructionFamily::DataProcessing => ModuleInfo {
            order: 1,
            filename: "base_integer.rs",
            module: "base_integer",
        },
        InstructionFamily::LoadStore => ModuleInfo {
            order: 2,
            filename: "loadstore.rs",
            module: "loadstore",
        },
        InstructionFamily::SimdFp => ModuleInfo {
            order: 3,
            filename: "simd_fp.rs",
            module: "simd_fp",
        },
        InstructionFamily::System => ModuleInfo {
            order: 4,
            filename: "system.rs",
            module: "system",
        },
        InstructionFamily::Sve => ModuleInfo {
            order: 5,
            filename: "sve.rs",
            module: "sve",
        },
        InstructionFamily::Sme => ModuleInfo {
            order: 6,
            filename: "sme.rs",
            module: "sme",
        },
        InstructionFamily::Crypto => ModuleInfo {
            order: 7,
            filename: "crypto.rs",
            module: "crypto",
        },
        InstructionFamily::MemTag => ModuleInfo {
            order: 8,
            filename: "memtag.rs",
            module: "memtag",
        },
        InstructionFamily::Unknown => ModuleInfo {
            order: 9,
            filename: "unknown.rs",
            module: "unknown",
        },
    }
}

fn generated_header(metadata: &HeaderMetadata) -> String {
    format!(
        "// Generated AArch64 instruction specs.\n//\n// @generated by xtask aarch64-gen. Do not edit by hand.\n// Input: external AArch64 instruction metadata\n// Input hash: {}\n// Recreate: cargo xtask aarch64-gen\n\n",
        metadata.source_hash
    )
}

fn emit_family_file(
    module: ModuleInfo,
    records: &[&InstructionRecord],
    metadata: &HeaderMetadata,
) -> Result<String, String> {
    let mut content = generated_header(metadata);
    content.push_str("robustone_isa_macros::define_instructions! {\n");
    content.push_str("    arch = Arm; module = ");
    content.push_str(module.module);
    content.push_str(";\n\n");

    for record in records {
        emit_record(&mut content, record)?;
    }

    content.push_str("}\n");
    Ok(content)
}

fn emit_record(content: &mut String, record: &InstructionRecord) -> Result<(), String> {
    let mask = record
        .encoding_mask
        .ok_or_else(|| format!("record {} has no encoding mask", record.llvm_name))?;
    let value = record
        .encoding_value
        .ok_or_else(|| format!("record {} has no encoding value", record.llvm_name))?;

    let format = format_expression(record);
    let operands = operand_expression(record)?;

    content.push_str("    insn ");
    content.push_str(&record.opcode_id);
    content.push_str(" {\n");
    content.push_str("        mnemonic = \"");
    content.push_str(&escape_rust_string(&record.mnemonic));
    content.push_str("\";\n");
    content.push_str("        opcode_id = \"");
    content.push_str(&record.opcode_id);
    content.push_str("\";\n");
    content.push_str("        pattern = robustone_isa::mask_value!(");
    content.push_str(&format!("0x{mask:08X}, 0x{value:08X}"));
    content.push_str(");\n");
    content.push_str("        format = ");
    content.push_str(format);
    content.push_str(";\n");
    content.push_str("        operands = ");
    content.push_str(&operands);
    content.push_str(";\n");
    content.push_str("        modes = ModeSet::All;\n");
    content.push_str("        features = ");
    content.push_str(feature_expression(record));
    content.push_str(";\n");
    content.push_str("        groups = ");
    content.push_str(&group_expression(&record.groups));
    content.push_str(";\n");
    content.push_str("        manual = \"generated\";\n");
    if requires_generated_priority(record) {
        content.push_str("        priority = 1;\n");
    }
    content.push_str("    }\n");
    Ok(())
}

fn generated_priority(record: &InstructionRecord) -> u8 {
    if requires_generated_priority(record) {
        1
    } else {
        0
    }
}

fn requires_generated_priority(record: &InstructionRecord) -> bool {
    matches!(
        record.family,
        InstructionFamily::Crypto | InstructionFamily::MemTag
    )
}

fn format_expression(record: &InstructionRecord) -> &'static str {
    match record.family {
        InstructionFamily::Branch => branch_format_expression(record),
        InstructionFamily::DataProcessing => data_processing_format_expression(record),
        InstructionFamily::LoadStore => load_store_format_expression(record),
        InstructionFamily::System => system_format_expression(record),
        InstructionFamily::SimdFp => simd_fp_format_expression(record),
        InstructionFamily::Crypto => crypto_format_expression(record),
        InstructionFamily::MemTag => mem_tag_format_expression(record),
        InstructionFamily::Sve | InstructionFamily::Sme => sve_sme_format_expression(record),
        _ => "&R_TYPE",
    }
}

fn branch_format_expression(record: &InstructionRecord) -> &'static str {
    match record.opcode_id.as_str() {
        "B" | "BL" => "&B_UNCOND",
        "BCC" => "&FMT_B_COND",
        opcode if opcode.starts_with("CB") => "&FMT_CBZ",
        opcode if opcode.starts_with("TB") => "&FMT_TBZ",
        _ => "&R_TYPE",
    }
}

fn sve_sme_format_expression(record: &InstructionRecord) -> &'static str {
    let has_predicate = record
        .operands
        .iter()
        .any(|operand| operand.kind == OperandKind::Predicate);
    let has_immediate = record
        .operands
        .iter()
        .any(|operand| operand.kind == OperandKind::Immediate);
    if record.opcode_id == "RDSVLI_XI" {
        "&SME_RDSVL"
    } else if record.opcode_id.starts_with("ADD_VG2_2ZZ_")
        || record.opcode_id.starts_with("ADD_VG4_4ZZ_")
    {
        "&SME_Z_GROUP_R"
    } else if record.opcode_id.starts_with("ADD_VG2_M2ZZ_")
        || record.opcode_id.starts_with("ADD_VG2_M2Z2Z_")
        || record.opcode_id.starts_with("ADD_VG4_M4ZZ_")
        || record.opcode_id.starts_with("ADD_VG4_M4Z4Z_")
        || record.opcode_id.starts_with("FVDOT") && record.opcode_id.contains("_M")
        || record.opcode_id.starts_with("FDOT_VG") && record.opcode_id.contains("_M")
    {
        "&SME_ZA_GROUP_UNARY_R"
    } else if matches!(record.opcode_id.as_str(), "ADD_VG2_M2Z_S" | "ADD_VG2_M2Z_D")
        || record.opcode_id.starts_with("ADD_VG4_M4Z_")
        || record.opcode_id.starts_with("BFADD_VG")
        || record.opcode_id.starts_with("FADD_VG")
    {
        "&SME_ZA_GROUP_UNARY"
    } else if matches!(
        record.opcode_id.as_str(),
        "ADDPL_XXI" | "ADDVL_XXI" | "ADDSPL_XXI" | "ADDSVL_XXI"
    ) {
        "&SVE_VL_ADD"
    } else if record.opcode_id == "ZERO_M" {
        "&SME_ZERO"
    } else if record.opcode_id == "MSRPSTATESVCRIMM1" {
        "&SME_SVCR"
    } else if record.family == InstructionFamily::Sve && is_sve_prefetch_record(record) {
        "&SVE_PREFETCH"
    } else if record.family == InstructionFamily::Sve && is_sve_tuple_memory_record(record) {
        "&SVE_TUPLE_MEM"
    } else if record.family == InstructionFamily::Sve
        && (is_sve_while_compare_record(record)
            || is_sve_fp_immediate_dup_record(record)
            || is_sve_xar_record(record))
    {
        "&SVE_PRED_ZI"
    } else if matches!(
        record.opcode_id.as_str(),
        "F1CVT_2ZZ_BTOH"
            | "F1CVTL_2ZZ_BTOH"
            | "F2CVTL_2ZZ_BTOH"
            | "BF1CVT_2ZZ_BTOH"
            | "BF1CVTL_2ZZ_BTOH"
            | "F2CVT_2ZZ_BTOH"
            | "BF2CVTL_2ZZ_BTOH"
            | "BF2CVT_2ZZ_BTOH"
            | "FCVT_Z2Z_HTOB"
            | "BFCVT_Z2Z_HTOB"
            | "FCVT_Z2Z_STOH"
            | "BFCVT_Z2Z_STOH"
            | "FCVT_Z4Z_STOB"
            | "FCVTN_Z4Z_STOB"
    ) {
        "&SME_Z_GROUP_COPY"
    } else if matches!(record.opcode_id.as_str(), "MOVT_TIX" | "MOVT_XTI") {
        "&SME_MOVT"
    } else if record.opcode_id.starts_with("MOVAZ_ZMI_") {
        "&SME_MOVAZ"
    } else if record.opcode_id.starts_with("LUTI2_ZTZI_")
        || record.opcode_id.starts_with("LUTI4_ZTZI_")
    {
        "&SME_LUTI"
    } else if record.opcode_id.starts_with("CADD_ZZI_")
        || record.opcode_id.starts_with("SQCADD_ZZI_")
    {
        "&SVE_COMPLEX_ROTATE_IMM"
    } else if record.opcode_id.starts_with("PTRUE_") || record.opcode_id.starts_with("PTRUES_") {
        "&SVE_PRED_PATTERN"
    } else if record.family == InstructionFamily::Sve
        && (has_predicate && has_immediate || record.opcode_id.starts_with("TBL_ZZZZ_"))
    {
        "&SVE_PRED_ZI"
    } else if record.family == InstructionFamily::Sve && has_predicate {
        "&SVE_PRED_Z"
    } else if record.family == InstructionFamily::Sve && has_immediate {
        "&SVE_RI"
    } else if record.family == InstructionFamily::Sve {
        "&SVE_PRED_Z"
    } else if record.family == InstructionFamily::Sme && has_predicate {
        "&SME_TILE_PRED_Z"
    } else {
        let operand_count = record.operands.len();
        if operand_count >= 5 {
            "&R_4REG"
        } else if operand_count >= 3 {
            "&R_TYPE"
        } else {
            "&VEC_COPY"
        }
    }
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
    ) && record.operands.iter().any(|operand| {
        let class = operand.raw_class.to_ascii_lowercase();
        matches!(
            class.as_str(),
            "zz_b"
                | "zz_h"
                | "zz_s"
                | "zz_d"
                | "zzz_b"
                | "zzz_h"
                | "zzz_s"
                | "zzz_d"
                | "zzzz_b"
                | "zzzz_h"
                | "zzzz_s"
                | "zzzz_d"
        )
    })
}

fn is_sve_fp_immediate_dup_record(record: &InstructionRecord) -> bool {
    record.opcode_id.starts_with("FCPY_ZPMI_") || record.opcode_id.starts_with("FDUP_ZI_")
}

fn is_sve_xar_record(record: &InstructionRecord) -> bool {
    record.opcode_id.starts_with("XAR_ZZZI_")
}

fn is_sve_while_compare_record(record: &InstructionRecord) -> bool {
    matches!(
        record.mnemonic.as_str(),
        "whilege"
            | "whilegt"
            | "whilehi"
            | "whilehs"
            | "whilele"
            | "whilelo"
            | "whilels"
            | "whilelt"
    ) && (record.opcode_id.contains("_PWW_") || record.opcode_id.contains("_PXX_"))
}
fn data_processing_format_expression(record: &InstructionRecord) -> &'static str {
    let opcode = record.opcode_id.as_str();
    if record.mnemonic == "addg" {
        "&MTE_ADDG"
    } else if opcode.ends_with("RI") {
        if matches!(record.mnemonic.as_str(), "and" | "eor" | "orr") {
            "&I_LOGICAL"
        } else {
            "&I_ADD"
        }
    } else if opcode.ends_with("RS") {
        if matches!(record.mnemonic.as_str(), "and" | "bic" | "eor" | "orr") {
            "&R_LOGICAL"
        } else {
            "&R_DP_REG"
        }
    } else {
        "&R_TYPE"
    }
}

fn mem_tag_format_expression(record: &InstructionRecord) -> &'static str {
    if matches!(record.mnemonic.as_str(), "addg" | "subg") {
        "&MTE_ADDG"
    } else if record.operands.iter().any(|operand| operand.name == "Rt2") {
        "&LDP"
    } else if record
        .operands
        .iter()
        .any(|operand| matches!(operand.name.as_str(), "Rt" | "Wt" | "Xt"))
    {
        load_store_format_expression(record)
    } else {
        "&R_TYPE"
    }
}

fn load_store_format_expression(record: &InstructionRecord) -> &'static str {
    let opcode = record.opcode_id.as_str();
    if record
        .operands
        .iter()
        .any(|operand| matches!(operand.name.as_str(), "Rs" | "Ws" | "Xs"))
        && record
            .operands
            .iter()
            .any(|operand| matches!(operand.name.as_str(), "Rt2" | "Wt2" | "Xt2"))
    {
        "&LDR_EXCL_PAIR"
    } else if record
        .operands
        .iter()
        .any(|operand| matches!(operand.name.as_str(), "Rt2" | "Wt2" | "Xt2"))
    {
        "&LDP"
    } else if record
        .operands
        .iter()
        .any(|operand| matches!(operand.name.as_str(), "Rs" | "Ws" | "Xs"))
    {
        "&LDR_EXCL"
    } else if opcode.ends_with('L') {
        "&LDR_LIT"
    } else if opcode.contains("POST") {
        "&LDR_POST"
    } else if opcode.contains("PRE") {
        "&LDR_PRE"
    } else if record.operands.iter().any(|operand| {
        let class = operand.raw_class.to_ascii_lowercase();
        class.contains("simm9") || class.contains("imm9")
    }) {
        "&LDR_POST"
    } else if record
        .operands
        .iter()
        .any(|operand| matches!(operand.name.as_str(), "Rt" | "Wt" | "Xt"))
    {
        "&LDR_IMM"
    } else {
        "&R_TYPE"
    }
}

fn system_format_expression(record: &InstructionRecord) -> &'static str {
    if record.opcode_id == "MSRPSTATESVCRIMM1" {
        "&SME_SVCR"
    } else if record.operands.iter().all(|operand| {
        operand.kind == OperandKind::Immediate
            && operand.raw_class.eq_ignore_ascii_case("timm32_0_65535")
    }) {
        "&EXCEPT"
    } else {
        "&R_TYPE"
    }
}

fn simd_fp_format_expression(record: &InstructionRecord) -> &'static str {
    if record.operands.iter().any(|operand| operand.name == "Rt2") {
        "&LDP"
    } else if record.operands.iter().any(|operand| operand.name == "cond") {
        "&FP_CONDSEL"
    } else if record
        .operands
        .iter()
        .any(|operand| operand.kind == OperandKind::Immediate)
    {
        "&R_4REG"
    } else if record.operands.len() == 2
        && record
            .operands
            .iter()
            .any(|operand| matches!(operand.name.as_str(), "Rd" | "Vd"))
    {
        "&FP_1SOURCE"
    } else if record.operands.len() == 3 {
        "&FP_2SOURCE"
    } else if record.operands.len() == 4
        && record.operands.iter().any(|operand| operand.name == "Ra")
    {
        "&FP_3SOURCE"
    } else {
        "&R_TYPE"
    }
}

fn crypto_format_expression(record: &InstructionRecord) -> &'static str {
    match record.operands.len() {
        2 => "&VEC_CRYPTO2",
        3 => "&R_TYPE",
        4 if record
            .operands
            .iter()
            .any(|operand| operand.kind == OperandKind::Immediate) =>
        {
            "&R_DP_REG"
        }
        4 => "&R_4REG",
        _ => "&R_TYPE",
    }
}

fn operand_expression(record: &InstructionRecord) -> Result<String, String> {
    let emitted = record
        .operands
        .iter()
        .filter(|operand| !is_generated_writeback_operand(operand))
        .map(operand_spec_expression)
        .collect::<Result<Vec<_>, _>>()?;

    if emitted.is_empty() {
        Ok("&[]".to_string())
    } else {
        Ok(format!("&[\n{}\n        ]", emitted.join(",\n")))
    }
}

fn is_generated_writeback_operand(operand: &OperandRecord) -> bool {
    operand.name.eq_ignore_ascii_case("wback")
}

fn operand_spec_expression(operand: &OperandRecord) -> Result<String, String> {
    match operand.kind {
        OperandKind::Register => Ok(format!(
            "            robustone_isa::reg!(ArmRegisterClass::{}, ArmField::{}, robustone_isa::Access::{})",
            register_class_expression(&operand.raw_class),
            field_expression(&operand.name)?,
            access_expression(&operand.role)
        )),
        OperandKind::Immediate => {
            if operand.raw_class.to_ascii_lowercase().starts_with("tbz_imm") {
                Ok(
                    "            robustone_isa::imm_compose!(\n                parts = [\n                    robustone_isa::ImmComposePart { src_start: 24, src_length: 5, dst_start: 0 },\n                    robustone_isa::ImmComposePart { src_start: 19, src_length: 1, dst_start: 5 },\n                ],\n                transform = robustone_isa::ImmediateTransform::None,\n                kind = robustone_isa::ImmediateKind::Unsigned\n            )"
                        .to_string(),
                )
            } else {
                Ok(format!(
                    "            robustone_isa::imm!(ArmField::{}, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::{})",
                    immediate_field_expression(operand)?,
                    immediate_kind_expression(operand)
                ))
            }
        }
        OperandKind::Condition => Ok(
            "            robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None)"
                .to_string(),
        ),
        OperandKind::Label | OperandKind::Memory => Ok(format!(
            "            robustone_isa::imm!(ArmField::{}, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative)",
            label_field_expression(operand)?
        )),
        OperandKind::SystemRegister => Ok(format!(
            "            robustone_isa::imm!(ArmField::{}, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)",
            immediate_field_expression(operand)?
        )),
        OperandKind::Predicate => Ok(format!(
            "            robustone_isa::reg!(ArmRegisterClass::Pred, ArmField::{}, robustone_isa::Access::{})",
            field_expression(&operand.name)?,
            access_expression(&operand.role)
        )),
        OperandKind::VectorList => {
            let class = operand.raw_class.to_ascii_lowercase();
            let Some((prefix, suffix)) = class.split_once('_') else {
                return Err(format!(
                    "record operand {}:{} is not supported for generated emission",
                    operand.raw_class, operand.name
                ));
            };
            let element = suffix.split('_').next().unwrap_or(suffix);
            if matches!(prefix, "zz" | "zzz" | "zzzz")
                && matches!(element, "any" | "b" | "h" | "s" | "d" | "q" | "mul")
            {
                Ok(format!(
                    "            robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::{}, robustone_isa::Access::{})",
                    field_expression(&operand.name)?,
                    access_expression(&operand.role)
                ))
            } else {
                Err(format!(
                    "record operand {}:{} is not supported for generated emission",
                    operand.raw_class, operand.name
                ))
            }
        }
        OperandKind::Unknown => Err(format!(
            "record operand {}:{} is not supported for generated emission",
            operand.raw_class, operand.name
        )),
    }
}

fn access_expression(role: &OperandRole) -> &'static str {
    match role {
        OperandRole::Write => "Write",
        OperandRole::Read | OperandRole::None => "Read",
        OperandRole::ReadWrite => "ReadWrite",
    }
}

fn register_class_expression(raw_class: &str) -> &'static str {
    let lower = raw_class.to_ascii_lowercase();
    if lower.starts_with("ppr") || lower == "pprany" {
        "Pred"
    } else if lower.starts_with("zpr") || lower.starts_with("z_") {
        "ZVec"
    } else if lower.starts_with("tile") || lower.starts_with("matrixop") || lower == "ztr" {
        "Za"
    } else if lower.starts_with('v') || lower.contains("fpr") {
        "Vec"
    } else {
        "Gpr"
    }
}

fn field_expression(name: &str) -> Result<&'static str, String> {
    let lower = name.trim_start_matches('_').to_ascii_lowercase();
    match lower.as_str() {
        "rd" | "vd" | "wd" | "xd" | "zd" | "zda" | "zdn" | "zda1" | "rdn" | "vdn" | "zada" => {
            Ok("Rd")
        }
        "rn" | "vn" | "wn" | "xn" | "zn" | "zn1" | "zk" | "pn" | "rv" => Ok("Rn"),
        "rm" | "vm" | "wm" | "xm" | "zm" | "pm" | "rm_and_shift" | "rm_and_extend" => Ok("Rm"),
        "ra" | "va" | "wa" | "xa" | "za" | "zn2" => Ok("Ra"),
        "pt" | "zat" | "zan" => Ok("Rt"),
        "rt" | "wt" | "xt" | "zt" | "ztt" => Ok("Rt"),
        "rt2" | "wt2" | "xt2" => Ok("Rt2"),
        "rs" | "ws" | "xs" => Ok("Rs"),
        "pg" => Ok("Cond"),
        "pd" | "pdm" | "pdn" | "zad" | "zadn" => Ok("Rd"),
        _ => Err(format!("unsupported generated operand field {name}")),
    }
}

fn immediate_field_expression(operand: &OperandRecord) -> Result<&'static str, String> {
    let class = operand.raw_class.to_ascii_lowercase();
    let name = operand.name.to_ascii_lowercase();
    if class == "sve_pred_enum" || class == "svcr_op" || name == "cond" || class.contains("ccode") {
        Ok("Cond")
    } else if class == "prfop" {
        Ok("Rt")
    } else if class == "matrixtilelist"
        || class == "sve_prfop"
        || class.starts_with("sve_fpimm_")
        || name == "imm" && class == "imm0_1"
        || class == "timm0_1"
        || class == "complexrotateop"
        || class == "complexrotateopodd"
        || class.starts_with("vectorindex")
        || class == "imm0_255"
        || class.starts_with("sme_elm_idx")
        || class.starts_with("fpimm")
        || class.contains("imm16")
        || class == "timm32_0_65535"
        || class.contains("imm0_65535")
        || class == "imm32_0_15"
    {
        Ok("Imm16")
    } else if class.contains("logical_imm") {
        Ok("Imms")
    } else if class.contains("imm12")
        || class.contains("imm0_4095")
        || class.contains("addsub_shifted_imm")
        || class.starts_with("addsub_imm")
        || class.starts_with("cpy_imm")
    {
        Ok("Imm12")
    } else if class.contains("simm5") || class.contains("imm0_31") || class.starts_with("uimm5") {
        Ok("Imm7")
    } else if class.contains("simm8")
        || class.contains("imm4")
        || class.contains("imm0_15")
        || class == "uimm3s8"
        || class == "sve_incdec_imm"
        || class == "timm32_0_7"
        || class == "timm32_0_15"
    {
        Ok("Imm16")
    } else if class.contains("simm9") {
        Ok("Imm9")
    } else if class.contains("imm7") || class.contains("imm0_127") {
        Ok("Imm7")
    } else if class.contains("imm6") || class.contains("vecshift") {
        Ok("Imm6")
    } else {
        Err(format!(
            "unsupported generated immediate field {}:{}",
            operand.raw_class, operand.name
        ))
    }
}

fn immediate_kind_expression(operand: &OperandRecord) -> &'static str {
    let class = operand.raw_class.to_ascii_lowercase();
    if class.contains("simm") {
        "Absolute"
    } else {
        "Unsigned"
    }
}

fn label_field_expression(operand: &OperandRecord) -> Result<&'static str, String> {
    let class = operand.raw_class.to_ascii_lowercase();
    if class.contains("tbr") || class.contains("brcond") || class.contains("ldrlit") {
        Ok("Imm19")
    } else if class.contains("bl_target") || class.contains("b_target") {
        Ok("Imm26")
    } else {
        Err(format!(
            "unsupported generated label field {}:{}",
            operand.raw_class, operand.name
        ))
    }
}
fn feature_expression(record: &InstructionRecord) -> &'static str {
    if record
        .features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("sve"))
    {
        "ArmFeature::SVE"
    } else if record
        .features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("sme"))
    {
        "ArmFeature::SME"
    } else if record.features.iter().any(|feature| feature == "HasLSE") {
        "ArmFeature::ATOMICS"
    } else if record.features.iter().any(|feature| {
        feature == "HasAES" || feature == "HasSHA2" || feature == "HasSHA3" || feature == "HasSM4"
    }) {
        "ArmFeature::CRYPTO"
    } else if record
        .features
        .iter()
        .any(|feature| feature.to_ascii_lowercase().contains("mte"))
    {
        "ArmFeature::MTE"
    } else if record
        .features
        .iter()
        .any(|feature| matches!(feature.as_str(), "HasFPARMv8" | "HasFullFP16"))
    {
        "ArmFeature::FP"
    } else if record.features.iter().any(|feature| {
        let lower = feature.to_ascii_lowercase();
        lower.contains("neon") || lower.contains("simd")
    }) {
        "ArmFeature::SIMD"
    } else {
        "ArmFeature::BASE"
    }
}

fn group_expression(groups: &[InstructionGroup]) -> String {
    let mut emitted = BTreeSet::new();
    for group in groups {
        emitted.insert(match group {
            InstructionGroup::Branch => "robustone_isa::InstructionGroup::Branch",
            InstructionGroup::Integer => "robustone_isa::InstructionGroup::Integer",
            InstructionGroup::LoadStore => "robustone_isa::InstructionGroup::Memory",
            InstructionGroup::Float => "robustone_isa::InstructionGroup::Float",
            InstructionGroup::Vector => "robustone_isa::InstructionGroup::Vector",
            InstructionGroup::System => "robustone_isa::InstructionGroup::System",
            InstructionGroup::Crypto => "robustone_isa::InstructionGroup::Vector",
            InstructionGroup::MemoryTagging | InstructionGroup::Unknown => {
                "robustone_isa::InstructionGroup::System"
            }
        });
    }
    if emitted.is_empty() {
        emitted.insert("robustone_isa::InstructionGroup::Integer");
    }

    let values = emitted.into_iter().collect::<Vec<_>>().join(", ");
    format!("&[{values}]")
}

fn emit_mod_file(modules: &[&str], metadata: &HeaderMetadata) -> String {
    let mut content = generated_header(metadata);
    content.push_str("use super::*;\n\n");

    for module in modules {
        content.push_str("pub mod ");
        content.push_str(module);
        content.push_str(" {\n");
        content.push_str("    use super::super::*;\n");
        content.push_str("    use robustone_isa::ModeSet;\n");
        content.push_str("    include!(\"");
        content.push_str(module_filename(module));
        content.push_str("\");\n");
        content.push_str("}\n");
    }

    content.push('\n');
    if modules.is_empty() {
        content.push_str(
            "pub static ALL_GENERATED_SPEC_SLICES: &[&[InstructionSpec<ArmBackend>]] = &[];\n",
        );
    } else {
        content.push_str(
            "pub static ALL_GENERATED_SPEC_SLICES: &[&[InstructionSpec<ArmBackend>]] = &[\n",
        );
        for module in modules {
            content.push_str("    ");
            content.push_str(module);
            content.push_str("::SPECS,\n");
        }
        content.push_str("];\n");
    }

    content
}

fn module_filename(module: &str) -> &'static str {
    match module {
        "branch" => "branch.rs",
        "base_integer" => "base_integer.rs",
        "loadstore" => "loadstore.rs",
        "simd_fp" => "simd_fp.rs",
        "system" => "system.rs",
        "sve" => "sve.rs",
        "sme" => "sme.rs",
        "crypto" => "crypto.rs",
        "memtag" => "memtag.rs",
        "unknown" => "unknown.rs",
        _ => unreachable!("unknown generated module {module}"),
    }
}

fn escape_rust_string(value: &str) -> String {
    value.escape_default().to_string()
}

#[cfg(test)]
mod tests {
    use crate::aarch64_gen::tblgen_json::parse_records;

    use super::*;

    #[test]
    fn emits_only_active_records() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let joined = files
            .iter()
            .map(|(_, content)| content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(joined.contains("insn ADDWRI"));
        assert!(joined.contains("insn BICWR"));
        assert!(joined.contains("insn ABSWR"));
        assert!(!joined.contains("insn PSEUDORET"));
        assert!(!joined.contains("insn ALIASADD"));
        assert!(!joined.contains("insn MYSTERYOP"));
        assert!(!joined.contains("insn BADNAMESPACE"));
    }

    #[test]
    fn generated_header_contains_boundary_safe_build_metadata() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let base_integer = files
            .iter()
            .find(|(filename, _)| filename == "base_integer.rs")
            .map(|(_, content)| content)
            .expect("base_integer.rs should be emitted");

        assert!(base_integer.contains("// @generated by xtask aarch64-gen. Do not edit by hand."));
        assert!(base_integer.contains("// Input: external AArch64 instruction metadata"));
        assert!(base_integer.contains("// Input hash: test-hash"));
        assert!(base_integer.contains("// Recreate: cargo xtask aarch64-gen"));
        assert!(!base_integer.to_lowercase().contains("llvm"));
        assert!(!base_integer.to_lowercase().contains("capstone"));
        assert!(!base_integer.to_lowercase().contains("binutils"));
    }

    #[test]
    fn generated_mod_file_wraps_family_modules_with_imports() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let mod_file = files
            .iter()
            .find(|(filename, _)| filename == "mod.rs")
            .map(|(_, content)| content)
            .expect("mod.rs should be emitted");

        assert!(mod_file.contains(
            "pub mod base_integer {\n    use super::super::*;\n    use robustone_isa::ModeSet;\n    include!(\"base_integer.rs\");\n}"
        ));
    }

    #[test]
    fn generated_mod_file_exposes_empty_spec_slice_when_no_records_are_active() {
        let records = Vec::new();

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("empty generated specs should emit");
        let mod_file = files
            .iter()
            .find(|(filename, _)| filename == "mod.rs")
            .map(|(_, content)| content)
            .expect("mod.rs should be emitted");

        assert!(mod_file.contains("use super::*;"));
        assert!(mod_file.contains(
            "pub static ALL_GENERATED_SPEC_SLICES: &[&[InstructionSpec<ArmBackend>]] = &[];"
        ));
    }

    #[test]
    fn generated_mod_file_exposes_all_active_family_slices() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let mod_file = files
            .iter()
            .find(|(filename, _)| filename == "mod.rs")
            .map(|(_, content)| content)
            .expect("mod.rs should be emitted");

        assert!(mod_file.contains(
            "pub static ALL_GENERATED_SPEC_SLICES: &[&[InstructionSpec<ArmBackend>]] = &["
        ));
        assert!(mod_file.contains("    branch::SPECS,"));
        assert!(mod_file.contains("    base_integer::SPECS,"));
        assert!(mod_file.contains("    simd_fp::SPECS,"));
        assert!(mod_file.contains("    system::SPECS,"));
    }

    #[test]
    fn generated_specs_emit_m4_feature_and_register_classes() {
        let json = r#"{
            "!instanceof": { "Instruction": ["FADDSrr", "AESErr", "ADDG"] },
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("M4 specs should emit");
        let simd_fp = files
            .iter()
            .find(|(filename, _)| filename == "simd_fp.rs")
            .map(|(_, content)| content)
            .expect("simd_fp.rs should be emitted");
        let crypto = files
            .iter()
            .find(|(filename, _)| filename == "crypto.rs")
            .map(|(_, content)| content)
            .expect("crypto.rs should be emitted");
        let memtag = files
            .iter()
            .find(|(filename, _)| filename == "memtag.rs")
            .map(|(_, content)| content)
            .expect("memtag.rs should be emitted");

        assert!(simd_fp.contains("format = &FP_2SOURCE;"));
        assert!(simd_fp.contains("ArmRegisterClass::Vec"));
        assert!(simd_fp.contains("features = ArmFeature::FP;"));
        assert!(crypto.contains("format = &VEC_CRYPTO2;"));
        assert!(crypto.contains("features = ArmFeature::CRYPTO;"));
        assert!(crypto.contains("        priority = 1;"));
        assert!(memtag.contains("features = ArmFeature::MTE;"));
        assert!(memtag.contains("ArmField::Imm6"));
        assert!(memtag.contains("ArmField::Imm16"));
    }

    #[test]
    fn generated_specs_emit_sve_zpmz_predicated_vector_metadata_for_all_widths() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADD_ZPMZ_B", "ADD_ZPMZ_H", "ADD_ZPMZ_S", "ADD_ZPMZ_D"] },
            "ADD_ZPMZ_B": {
                "AsmString": "add\t$Zdn, $Pg, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zdn)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZPR8:$_Zdn, ZPR8:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "00000100000?????????????????????",
                "Predicates": ["HasSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ADD_ZPMZ_H": {
                "AsmString": "add\t$Zdn, $Pg, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR16:$Zdn)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZPR16:$_Zdn, ZPR16:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "00000100010?????????????????????",
                "Predicates": ["HasSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ADD_ZPMZ_S": {
                "AsmString": "add\t$Zdn, $Pg, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR32:$Zdn)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZPR32:$_Zdn, ZPR32:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "00000100100?????????????????????",
                "Predicates": ["HasSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "ADD_ZPMZ_D": {
                "AsmString": "add\t$Zdn, $Pg, $Zm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR64:$Zdn)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZPR64:$_Zdn, ZPR64:$Zm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "00000100110?????????????????????",
                "Predicates": ["HasSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE ZPmZ specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        for opcode in ["ADD_ZPMZ_B", "ADD_ZPMZ_H", "ADD_ZPMZ_S", "ADD_ZPMZ_D"] {
            assert!(sve.contains(&format!("insn {opcode}")));
        }
        assert_eq!(sve.matches("        format = &SVE_PRED_Z;").count(), 4);
        assert_eq!(
            sve.matches("robustone_isa::reg!(ArmRegisterClass::Pred, ArmField::Cond, robustone_isa::Access::Read)").count(),
            4
        );
        assert_eq!(
            sve.matches("robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::Rd, robustone_isa::Access::ReadWrite)").count(),
            4
        );
        assert_eq!(
            sve.matches("robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::Rm, robustone_isa::Access::Read)").count(),
            4
        );
        assert!(!sve.contains("robustone_isa::imm!("));
    }

    #[test]
    fn generated_specs_emit_operand_metadata_for_active_integer_immediates() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let base_integer = files
            .iter()
            .find(|(filename, _)| filename == "base_integer.rs")
            .map(|(_, content)| content)
            .expect("base_integer.rs should be emitted");

        assert!(base_integer.contains("insn ADDWRI"));
        assert!(base_integer.contains("        format = &I_ADD;"));
        assert!(base_integer.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rd, robustone_isa::Access::Write)"
        ));
        assert!(base_integer.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read)"
        ));
        assert!(base_integer.contains(
            "robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_operand_metadata_for_active_branch_conditions() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let branch = files
            .iter()
            .find(|(filename, _)| filename == "branch.rs")
            .map(|(_, content)| content)
            .expect("branch.rs should be emitted");

        assert!(branch.contains("insn BCC"));
        assert!(branch.contains("        format = &FMT_B_COND;"));
        assert!(branch.contains(
            "robustone_isa::text!(ArmField::Cond, robustone_isa::ImmediateTransform::None)"
        ));
        assert!(branch.contains(
            "robustone_isa::imm!(ArmField::Imm19, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::PcRelative)"
        ));
    }

    #[test]
    fn generated_specs_emit_operand_metadata_for_active_load_store() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LDRWui"] },
            "LDRWui": {
                "AsmString": "ldr\t$Rt, [$Rn, $offset]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32z:$Rt)",
                "InOperandList": "(ins GPR64sp:$Rn, uimm12s4:$offset)",
                "bits": "1011100101??????????????????????",
                "Predicates": ["HasV8_0a"],
                "isPseudo": false,
                "isCodeGenOnly": false,
                "isAsmParserOnly": false
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let loadstore = files
            .iter()
            .find(|(filename, _)| filename == "loadstore.rs")
            .map(|(_, content)| content)
            .expect("loadstore.rs should be emitted");

        assert!(loadstore.contains("insn LDRWUI"));
        assert!(loadstore.contains("        format = &LDR_IMM;"));
        assert!(loadstore.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rt, robustone_isa::Access::Write)"
        ));
        assert!(loadstore.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read)"
        ));
        assert!(loadstore.contains(
            "robustone_isa::imm!(ArmField::Imm12, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_llvm_compound_shifted_register_operand_names() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDWrs"] },
            "ADDWrs": {
                "AsmString": "add\t$Rd, $Rn, $Rm_and_shift",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32sp:$Rd)",
                "InOperandList": {
                    "args": [
                        [{ "def": "GPR32sp", "kind": "def", "printable": "GPR32sp" }, "Rn"],
                        [{ "def": "arith_shifted_reg32", "kind": "def", "printable": "arith_shifted_reg32" }, "Rm_and_shift"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins GPR32sp:$Rn, arith_shifted_reg32:$Rm_and_shift)"
                },
                "bits": "00001011000?????00??????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("shifted register specs should emit");
        let base_integer = files
            .iter()
            .find(|(filename, _)| filename == "base_integer.rs")
            .map(|(_, content)| content)
            .expect("base_integer.rs should be emitted");

        assert!(base_integer.contains("insn ADDWRS"));
        assert!(base_integer.contains("        format = &R_DP_REG;"));
        assert!(base_integer.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read)"
        ));
    }

    #[test]
    fn generated_specs_emit_llvm_compound_extended_register_operand_names() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDWrx"] },
            "ADDWrx": {
                "AsmString": "add\t$Rd, $Rn, $Rm_and_extend",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR32sp:$Rd)",
                "InOperandList": {
                    "args": [
                        [{ "def": "GPR32sp", "kind": "def", "printable": "GPR32sp" }, "Rn"],
                        [{ "def": "(arith_extended_reg32_i32 ?:$Rm, ?:$extend)", "kind": "def", "printable": "(arith_extended_reg32_i32 ?:$Rm, ?:$extend)" }, "Rm_and_extend"]
                    ],
                    "kind": "dag",
                    "operator": { "def": "ins", "kind": "def", "printable": "ins" },
                    "printable": "(ins GPR32sp:$Rn, arith_extended_reg32_i32:$Rm_and_extend)"
                },
                "bits": "00001011001?????00??????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("extended register specs should emit");
        let base_integer = files
            .iter()
            .find(|(filename, _)| filename == "base_integer.rs")
            .map(|(_, content)| content)
            .expect("base_integer.rs should be emitted");

        assert!(base_integer.contains("insn ADDWRX"));
        assert!(base_integer.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read)"
        ));
    }

    #[test]
    fn generated_specs_emit_timm_zero_one_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["MSRpstatesvcrImm1"] },
            "MSRpstatesvcrImm1": {
                "AsmString": "msr\t$pstatefield, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins svcr_op:$pstatefield, timm0_1:$imm)",
                "bits": "11010101000000110100????01111111",
                "Predicates": ["HasSME"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("timm0_1 specs should emit");
        let sme = files
            .iter()
            .find(|(filename, _)| filename == "sme.rs")
            .map(|(_, content)| content)
            .expect("sme.rs should be emitted");

        assert!(sme.contains("insn MSRPSTATESVCRIMM1"));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_prefetch_operation_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["PRFMui"] },
            "PRFMui": {
                "AsmString": "prfm\t$Rt, [$Rn, $offset]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins prfop:$Rt, GPR64sp:$Rn, uimm12s8:$offset)",
                "bits": "1111100110??????????????????????",
                "Predicates": [],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("prfop specs should emit");
        let loadstore = files
            .iter()
            .find(|(filename, _)| filename == "loadstore.rs")
            .map(|(_, content)| content)
            .expect("loadstore.rs should be emitted");

        assert!(loadstore.contains("insn PRFMUI"));
        assert!(loadstore.contains(
            "robustone_isa::imm!(ArmField::Rt, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_timm_four_bit_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["EXTQ_ZZI"] },
            "EXTQ_ZZI": {
                "AsmString": "extq\t$Zdn, $_Zdn, $Zm, $imm4",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR128:$_Zdn)",
                "InOperandList": "(ins ZPR128:$_Zdn, ZPR128:$Zm, timm32_0_15:$imm4)",
                "Constraints": "$_Zdn = $_Zdn",
                "bits": "00000101001?????111?????????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("timm32_0_15 specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn EXTQ_ZZI"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_splice_vector_list_schemas() {
        let json = r#"{
            "!instanceof": { "Instruction": ["SPLICE_ZPZZ_D"] },
            "SPLICE_ZPZZ_D": {
                "AsmString": "splice\t$Zd, $Pg, $Zn",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR64:$Zd)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZZ_d:$Zn)",
                "bits": "0000010111101101100?????????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SPLICE specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn SPLICE_ZPZZ_D"));
        assert!(sve.contains(
            "robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::Rn, robustone_isa::Access::Read)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve2_tbl_two_register_vector_list_schemas() {
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("TBL specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn TBL_ZZZZ_B"));
        assert!(sve.contains("        format = &SVE_PRED_ZI;"));
        assert!(sve.contains(
            "robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::Rn, robustone_isa::Access::Read)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_while_compare_metadata_with_rm_field_at_bits_16_to_20() {
        let json = r#"{
            "!instanceof": { "Instruction": ["WHILEGE_PWW_B", "WHILEGT_PXX_D"] },
            "WHILEGE_PWW_B": {
                "AsmString": "whilege\t$Pd, $Rn, $Rm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs PPR8:$Pd)",
                "InOperandList": "(ins GPR32:$Rn, GPR32:$Rm)",
                "bits": "00100101001?????000000?????0????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            },
            "WHILEGT_PXX_D": {
                "AsmString": "whilegt\t$Pd, $Rn, $Rm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs PPR64:$Pd)",
                "InOperandList": "(ins GPR64:$Rn, GPR64:$Rm)",
                "bits": "00100101111?????000100?????1????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("WHILE compare specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn WHILEGE_PWW_B"));
        assert!(sve.contains("insn WHILEGT_PXX_D"));
        assert!(sve.contains("        format = &SVE_PRED_ZI;"));
        assert!(sve.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rm, robustone_isa::Access::Read)"
        ));
    }

    #[test]
    fn generated_specs_emit_sme_tile_vector_index_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LD1_MXIPXX_H_B"] },
            "LD1_MXIPXX_H_B": {
                "AsmString": "ld1b\t$ZAn[$Rv, #$imm], $Pg/z, [$Rn, $Rm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs TileVectorOpH8:$ZAn)",
                "InOperandList": "(ins MatrixIndexGPR32Op12_15:$Rv, sme_elm_idx0_15:$imm, PPR3bAny:$Pg, GPR64sp:$Rn, GPR64shifted8:$Rm)",
                "bits": "11100000000?????0??????????0????",
                "Predicates": ["HasSME"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SME tile-vector specs should emit");
        let sme = files
            .iter()
            .find(|(filename, _)| filename == "sme.rs")
            .map(|(_, content)| content)
            .expect("sme.rs should be emitted");

        assert!(sme.contains("insn LD1_MXIPXX_H_B"));
        assert!(sme.contains("        format = &SME_TILE_PRED_Z;"));
        assert!(sme.contains(
            "robustone_isa::reg!(ArmRegisterClass::Za, ArmField::Rt, robustone_isa::Access::Write)"
        ));
        assert!(sme.contains(
            "robustone_isa::reg!(ArmRegisterClass::Gpr, ArmField::Rn, robustone_isa::Access::Read)"
        ));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
        assert!(sme.contains("ArmRegisterClass::Pred"));
    }

    #[test]
    fn generated_specs_emit_sme2_zt0_register_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["LUTI2_2ZTZI_B"] },
            "LUTI2_2ZTZI_B": {
                "AsmString": "luti2\t$ZTt, {$Zn1, $Zn2}, $Zm[$imm]",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZTR:$ZTt)",
                "InOperandList": "(ins ZPR8:$Zn1, ZPR8:$Zn2, ZPR8:$Zm, VectorIndexS8b_timm:$imm)",
                "bits": "110000010000000000000???????????",
                "Predicates": ["HasSME2"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SME2 ZT0 specs should emit");
        let sme = files
            .iter()
            .find(|(filename, _)| filename == "sme.rs")
            .map(|(_, content)| content)
            .expect("sme.rs should be emitted");

        assert!(sme.contains("insn LUTI2_2ZTZI_B"));
        assert!(sme.contains(
            "robustone_isa::reg!(ArmRegisterClass::Za, ArmField::Rt, robustone_isa::Access::Write)"
        ));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sme2_scaled_three_bit_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ZERO_MXI"] },
            "ZERO_MXI": {
                "AsmString": "zero\t$ZAd, $Rv, #$imm3",
                "Namespace": "AArch64",
                "OutOperandList": "(outs MatrixOp:$ZAd)",
                "InOperandList": "(ins GPR64:$Rv, uimm3s8:$imm3)",
                "bits": "110000001000000000000???????????",
                "Predicates": ["HasSME2"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SME2 scaled immediate specs should emit");
        let sme = files
            .iter()
            .find(|(filename, _)| filename == "sme.rs")
            .map(|(_, content)| content)
            .expect("sme.rs should be emitted");

        assert!(sme.contains("insn ZERO_MXI"));
        assert!(sme.contains(
            "robustone_isa::reg!(ArmRegisterClass::Za, ArmField::Rd, robustone_isa::Access::Write)"
        ));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_signed_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["CMPEQ_PPzZI_B"] },
            "CMPEQ_PPzZI_B": {
                "AsmString": "cmpeq\t$Pd, $Pg/z, $Zn, #$imm5",
                "Namespace": "AArch64",
                "OutOperandList": "(outs PPR8:$Pd)",
                "InOperandList": "(ins PPR3bAny:$Pg, ZPR8:$Zn, simm5_32b:$imm5)",
                "bits": "00100101000?????100????????0????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE signed immediate specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn CMPEQ_PPZZI_B"));
        assert!(sve.contains("        format = &SVE_PRED_ZI;"));
        assert!(sve.contains("ArmRegisterClass::Pred"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm7, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_register_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDPL_XXI"] },
            "ADDPL_XXI": {
                "AsmString": "addpl\t$Rd, $Rn, #$imm6",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR64sp:$Rd)",
                "InOperandList": "(ins GPR64sp:$Rn, simm6_32b:$imm6)",
                "bits": "00000100011?????01010???????????",
                "Predicates": ["HasSVEorStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE register immediate specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn ADDPL_XXI"));
        assert!(sve.contains("        format = &SVE_VL_ADD;"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_xar_metadata_with_zm_field_at_bits_5_to_9() {
        let json = r#"{
            "!instanceof": { "Instruction": ["XAR_ZZZI_B"] },
            "XAR_ZZZI_B": {
                "AsmString": "xar\t$Zdn, $_Zdn, $Zm, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs ZPR8:$Zdn)",
                "InOperandList": "(ins ZPR8:$_Zdn, ZPR8:$Zm, vecshiftR8:$imm)",
                "Constraints": "$Zdn = $_Zdn",
                "bits": "0000010000101???001101??????????",
                "Predicates": ["HasSVE2orStreamingSVE"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE XAR specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn XAR_ZZZI_B"));
        assert!(sve.contains("        format = &SVE_PRED_ZI;"));
        assert!(sve.contains(
            "robustone_isa::reg!(ArmRegisterClass::ZVec, ArmField::Rm, robustone_isa::Access::Read)"
        ));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }
    #[test]
    fn generated_specs_emit_sve_pattern_count_metadata() {
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE pattern count specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn CNTB_XPII"));
        assert!(sve.contains("        format = &SVE_RI;"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Cond, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }
    #[test]
    fn generated_specs_emit_sve_exact_fp_immediate_metadata() {
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE exact FP immediate specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn FADD_ZPMI_D"));
        assert!(sve.contains("        format = &SVE_PRED_ZI;"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sve_shift_immediate_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ASR_ZZI_B"] },
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
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SVE shift immediate specs should emit");
        let sve = files
            .iter()
            .find(|(filename, _)| filename == "sve.rs")
            .map(|(_, content)| content)
            .expect("sve.rs should be emitted");

        assert!(sve.contains("insn ASR_ZZI_B"));
        assert!(sve.contains("        format = &SVE_RI;"));
        assert!(sve.contains(
            "robustone_isa::imm!(ArmField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_specs_emit_sme_state_access_metadata() {
        let json = r#"{
            "!instanceof": { "Instruction": ["MSRpstatesvcrImm1"] },
            "MSRpstatesvcrImm1": {
                "AsmString": "msr\t$pstatefield, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs)",
                "InOperandList": "(ins svcr_op:$pstatefield, imm0_1:$imm)",
                "bits": "11010101000000110100????01111111",
                "Predicates": ["HasSME"],
                "isPseudo": 0,
                "isCodeGenOnly": 0,
                "isAsmParserOnly": 0
            }
        }"#;
        let records = parse_records(json).expect("inline JSON should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("SME state-access specs should emit");
        let sme = files
            .iter()
            .find(|(filename, _)| filename == "sme.rs")
            .map(|(_, content)| content)
            .expect("sme.rs should be emitted");

        assert!(sme.contains("insn MSRPSTATESVCRIMM1"));
        assert!(sme.contains("        format = &SME_SVCR;"));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Cond, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
        assert!(sme.contains(
            "robustone_isa::imm!(ArmField::Imm16, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Unsigned)"
        ));
    }

    #[test]
    fn generated_placeholder_specs_emit_lower_priority_than_manual_specs() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let records = parse_records(json).expect("fixture should parse");

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("specs should emit");
        let joined = files
            .iter()
            .map(|(_, content)| content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!joined.contains("        priority = "));
        assert!(joined.contains("        manual = \"generated\";"));
    }

    #[test]
    fn does_not_emit_deferred_fpr_load_store_as_base_spec() {
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("deferred FP load/store should not block emission");
        let joined = files
            .iter()
            .map(|(_, content)| content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!joined.contains("insn LDRSUI"));
        assert!(!joined.contains("0xBD400000"));
        assert!(!joined.contains("features = ArmFeature::BASE;"));
    }

    #[test]
    fn permits_m4_priority_refinements_over_broad_base_patterns() {
        let json = r#"{
            "!instanceof": { "Instruction": ["ADDXri", "ADDG"] },
            "ADDXri": {
                "AsmString": "add\t$Rd, $Rn, $imm",
                "Namespace": "AArch64",
                "OutOperandList": "(outs GPR64:$Rd)",
                "InOperandList": "(ins GPR64:$Rn, imm0_4095:$imm)",
                "bits": "10010001????????????????????????",
                "Predicates": ["HasV8_0a"],
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

        let files = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect("priority refinement should not be rejected as overlap");
        let memtag = files
            .iter()
            .find(|(filename, _)| filename == "memtag.rs")
            .map(|(_, content)| content)
            .expect("memtag.rs should be emitted");

        assert!(memtag.contains("insn ADDG"));
        assert!(memtag.contains("        priority = 1;"));
    }

    #[test]
    fn rejects_overlapping_active_encodings_with_different_masks() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let mut records = parse_records(json).expect("fixture should parse");
        let add = records
            .iter()
            .find(|record| record.llvm_name == "ADDWri")
            .expect("ADDWri should exist")
            .clone();
        let mut overlap = add.clone();
        overlap.llvm_name = "ADDWriOverlap".to_string();
        overlap.opcode_id = "ADDWRIOVERLAP".to_string();
        let mask = add.encoding_mask.expect("ADDWri mask");
        let least_significant_mask_bit = mask & mask.wrapping_neg();
        overlap.encoding_mask = Some(mask & !least_significant_mask_bit);
        records.push(overlap);

        let err = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect_err("overlap should fail");

        assert!(err.contains("overlapping active encodings"));
        assert!(err.contains("ADDWri"));
        assert!(err.contains("ADDWriOverlap"));
    }

    #[test]
    fn rejects_conflicting_active_encodings() {
        let json = include_str!("../../tests/fixtures/aarch64_tblgen_subset.json");
        let mut records = parse_records(json).expect("fixture should parse");
        let add = records
            .iter()
            .find(|record| record.llvm_name == "ADDWri")
            .expect("ADDWri should exist")
            .clone();
        let mut duplicate = add.clone();
        duplicate.llvm_name = "ADDWriDuplicate".to_string();
        duplicate.opcode_id = "ADDWRIDUPLICATE".to_string();
        records.push(duplicate);

        let err = emit_specs_with_metadata(&records, &HeaderMetadata::for_tests())
            .expect_err("conflict should fail");

        assert!(err.contains("conflicting active encodings"));
        assert!(err.contains("ADDWri"));
        assert!(err.contains("ADDWriDuplicate"));
    }
}
