#!/usr/bin/env python3
"""
Generate robustone-loongarch/src/patterns.rs from Capstone decoder tables.

This replaces the exact-word match in decoder_generated.rs with a proper
mask/value pattern table derived from Capstone's spec-driven decoder tree.
"""

import re

# ---------------------------------------------------------------------------
# Parse Capstone decoder table
# ---------------------------------------------------------------------------

def parse_decoder_table(content):
    start = content.find('static const uint8_t DecoderTable32[] = {')
    start += len('static const uint8_t DecoderTable32[] = {')
    depth = 1
    end = start
    for i, ch in enumerate(content[start:], start):
        if ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                end = i
                break
    return content[start:end]

def _uleb128_bytes(operands, start_idx):
    idx = start_idx
    while idx < len(operands):
        b = int(operands[idx])
        idx += 1
        if (b & 0x80) == 0:
            break
    return idx - start_idx

def calc_op_size(op, operands):
    if op == 'MCD_OPC_ExtractField':
        return 1 + len(operands)
    elif op == 'MCD_OPC_FilterValue':
        val_bytes = _uleb128_bytes(operands, 0)
        return 1 + val_bytes + 3
    elif op == 'MCD_OPC_CheckField':
        val_bytes = _uleb128_bytes(operands, 2)
        return 1 + 1 + 1 + val_bytes + 3
    elif op == 'MCD_OPC_CheckPredicate':
        val_bytes = _uleb128_bytes(operands, 0)
        return 1 + val_bytes + 3
    elif op == 'MCD_OPC_Decode':
        val1_bytes = _uleb128_bytes(operands, 0)
        val2_bytes = _uleb128_bytes(operands, val1_bytes)
        return 1 + val1_bytes + val2_bytes
    elif op == 'MCD_OPC_TryDecode':
        val1_bytes = _uleb128_bytes(operands, 0)
        val2_bytes = _uleb128_bytes(operands, val1_bytes)
        return 1 + val1_bytes + val2_bytes + 3
    elif op == 'MCD_OPC_SoftFail':
        return 1 + 1 + 1
    elif op == 'MCD_OPC_Fail':
        return 1
    else:
        return len(operands) + 1

def build_bytecode(table_text):
    lines = table_text.split('\n')
    bytes_list = []
    offset_to_pc = {}
    pc = 0
    for line in lines:
        line = line.strip()
        if not line:
            continue
        om = re.search(r'/\*\s*(\d+)\s*\*/', line)
        if not om:
            continue
        offset = int(om.group(1))
        code = re.sub(r'//.*$', '', line).strip()
        code = re.sub(r'/\*\s*\d+\s*\*/', '', code).strip()
        code = code.rstrip(',')
        parts = [p.strip() for p in code.split(',') if p.strip()]
        if not parts:
            continue
        op = parts[0]
        operands = parts[1:]
        size = calc_op_size(op, operands)
        offset_to_pc[offset] = pc
        for part in parts:
            if part.startswith('0x'):
                bytes_list.append(int(part, 16))
            elif part.lstrip('-').isdigit():
                bytes_list.append(int(part))
            elif part == 'MCD_OPC_ExtractField':
                bytes_list.append(0)
            elif part == 'MCD_OPC_FilterValue':
                bytes_list.append(1)
            elif part == 'MCD_OPC_CheckField':
                bytes_list.append(2)
            elif part == 'MCD_OPC_CheckPredicate':
                bytes_list.append(3)
            elif part == 'MCD_OPC_Decode':
                bytes_list.append(4)
            elif part == 'MCD_OPC_TryDecode':
                bytes_list.append(5)
            elif part == 'MCD_OPC_SoftFail':
                bytes_list.append(6)
            elif part == 'MCD_OPC_Fail':
                bytes_list.append(7)
        pc += size
    return bytes_list, offset_to_pc, lines

def read_uleb128(data, idx):
    val = 0
    shift = 0
    while idx < len(data):
        b = data[idx]
        val |= (b & 0x7F) << shift
        idx += 1
        if (b & 0x80) == 0:
            break
        shift += 7
    return val, idx

def build_insn_map(bytes_list):
    EXTRACT, FILTER, CHECK, CHECK_PRED, DECODE, TRY_DECODE, SOFT_FAIL, FAIL = range(8)
    i = 0
    insn_map = {}
    while i < len(bytes_list):
        opc = bytes_list[i]
        if opc == EXTRACT:
            if i + 2 < len(bytes_list):
                insn_map[i] = ('extract', bytes_list[i+1], bytes_list[i+2], 3)
            i += 3
        elif opc == FILTER:
            val, ni = read_uleb128(bytes_list, i+1)
            if ni + 2 < len(bytes_list):
                skip = bytes_list[ni] | (bytes_list[ni+1] << 8) | (bytes_list[ni+2] << 16)
                insn_map[i] = ('filter', val, skip, ni + 3 - i)
                i = ni + 3
            else:
                break
        elif opc == CHECK:
            if i + 2 < len(bytes_list):
                start = bytes_list[i+1]
                length = bytes_list[i+2]
                val, ni = read_uleb128(bytes_list, i+3)
                if ni + 2 < len(bytes_list):
                    skip = bytes_list[ni] | (bytes_list[ni+1] << 8) | (bytes_list[ni+2] << 16)
                    insn_map[i] = ('check', start, length, val, skip, ni + 3 - i)
                    i = ni + 3
                else:
                    break
            else:
                break
        elif opc == CHECK_PRED:
            val, ni = read_uleb128(bytes_list, i+1)
            if ni + 2 < len(bytes_list):
                skip = bytes_list[ni] | (bytes_list[ni+1] << 8) | (bytes_list[ni+2] << 16)
                insn_map[i] = ('check_pred', val, skip, ni + 3 - i)
                i = ni + 3
            else:
                break
        elif opc == DECODE:
            val1, ni = read_uleb128(bytes_list, i+1)
            val2, ni = read_uleb128(bytes_list, ni)
            insn_map[i] = ('decode', val1, val2, ni - i)
            i = ni
        elif opc == TRY_DECODE:
            val1, ni = read_uleb128(bytes_list, i+1)
            val2, ni = read_uleb128(bytes_list, ni)
            if ni + 2 < len(bytes_list):
                skip = bytes_list[ni] | (bytes_list[ni+1] << 8) | (bytes_list[ni+2] << 16)
                insn_map[i] = ('try_decode', val1, val2, skip, ni + 3 - i)
                i = ni + 3
            else:
                break
        elif opc == SOFT_FAIL:
            val1, ni = read_uleb128(bytes_list, i+1)
            val2, ni = read_uleb128(bytes_list, ni)
            insn_map[i] = ('soft_fail', val1, val2, ni - i)
            i = ni
        elif opc == FAIL:
            insn_map[i] = ('fail', 1)
            i += 1
        else:
            i += 1
    return insn_map

def calc_mask_value(constraints):
    mask = 0
    value = 0
    for start, length, val in constraints:
        for b in range(start, start + length):
            mask |= (1 << b)
            if val & (1 << (b - start)):
                value |= (1 << b)
    return mask, value

def trace_decoder(bytes_list, insn_map, offset_to_pc, lines):
    opcode_names = {}
    for line in lines:
        m = re.search(r'// Opcode: ([A-Za-z_0-9]+)', line)
        if m:
            om = re.search(r'/\*\s*(\d+)\s*\*/', line)
            if om:
                opcode_names[int(om.group(1))] = m.group(1)

    pc_to_offset = {v: k for k, v in offset_to_pc.items()}
    results = []
    visited = set()

    def trace(pc, constraints, current_field):
        key = (pc, tuple(sorted(constraints)), current_field)
        if key in visited:
            return
        visited.add(key)
        if pc >= len(bytes_list):
            return
        if pc not in insn_map:
            trace(pc + 1, constraints, current_field)
            return
        insn = insn_map[pc]
        op = insn[0]
        if op == 'extract':
            trace(pc + insn[3], constraints, (insn[1], insn[2]))
        elif op == 'filter':
            if current_field is not None:
                new_constraints = constraints + ((current_field[0], current_field[1], insn[1]),)
                trace(pc + insn[3], new_constraints, None)
            trace(pc + insn[3] + insn[2], constraints, current_field)
        elif op == 'check':
            new_constraints = constraints + ((insn[1], insn[2], insn[3]),)
            trace(pc + insn[5], new_constraints, current_field)
            trace(pc + insn[5] + insn[4], constraints, current_field)
        elif op == 'check_pred':
            trace(pc + insn[3], constraints, current_field)
            trace(pc + insn[3] + insn[2], constraints, current_field)
        elif op == 'decode':
            mask, value = calc_mask_value(constraints)
            offset = pc_to_offset.get(pc)
            name = opcode_names.get(offset, f"OPCODE_{insn[1]}") if offset else f"OPCODE_{insn[1]}"
            results.append((name, mask, value, insn[2]))
        elif op == 'try_decode':
            mask, value = calc_mask_value(constraints)
            offset = pc_to_offset.get(pc)
            name = opcode_names.get(offset, f"OPCODE_{insn[1]}") if offset else f"OPCODE_{insn[1]}"
            results.append((name, mask, value, insn[2]))
            trace(pc + insn[4] + insn[3], constraints, current_field)
        elif op == 'soft_fail':
            trace(pc + insn[3], constraints, current_field)
        elif op == 'fail':
            pass

    trace(0, (), None)
    return results

# ---------------------------------------------------------------------------
# Parse case definitions
# ---------------------------------------------------------------------------

def parse_cases(content):
    macro_start = content.find('#define DecodeToMCInst')
    macro_end = content.find('#undef DecodeToMCInst', macro_start)
    macro = content[macro_start:macro_end]
    cases = {}
    case_blocks = re.findall(r'case (\d+):\s*\\\s*\n(.*?)return S;\s*\\', macro, re.DOTALL)
    for case_num_str, block in case_blocks:
        case_num = int(case_num_str)
        fields = []
        lines_in_block = [l.strip() for l in block.split('\\') if l.strip()]
        for idx, line in enumerate(lines_in_block):
            fm = re.search(r'fieldname\(insn, (\d+), (\d+)\)', line)
            if fm:
                start_bit = int(fm.group(1))
                length = int(fm.group(2))
                reg_class = None
                imm_decoder = None
                if idx + 1 < len(lines_in_block):
                    next_line = lines_in_block[idx + 1]
                    rc = re.search(r'Decode([A-Z0-9]+)RegisterClass', next_line)
                    if rc:
                        reg_class = rc.group(1)
                    idm = re.search(r'decode([A-Z][A-Za-z0-9_]*)', next_line)
                    if idm:
                        imm_decoder = idm.group(1)
                    if 'MCOperand_CreateImm0' in next_line:
                        imm_decoder = 'raw'
                fields.append((start_bit, length, reg_class, imm_decoder))
        cases[case_num] = fields
    return cases

# ---------------------------------------------------------------------------
# Generate Rust code
# ---------------------------------------------------------------------------

REG_OFFSETS = {
    'GPR': 0,
    'FPR': 32,
    'FPR32': 32,
    'FPR64': 32,
    'LSX128': 64,
    'LASX256': 64,
    'CFR': 96,
    'FCSR': 108,
    'SCR': 104,
}

def generate_extract_fn(case_num, fields):
    lines = [f"fn extract_case_{case_num}(word: u32) -> Vec<Operand> {{"]
    lines.append("    let mut ops = Vec::new();")
    for start, length, reg_class, imm_decoder in fields:
        mask = (1 << length) - 1
        if reg_class:
            offset = REG_OFFSETS.get(reg_class, 0)
            lines.append(f"    ops.push(Operand::Register {{ register: RegisterId::loongarch(((word >> {start}) & 0x{mask:X}) as u32 + {offset}) }});")
        elif imm_decoder:
            if imm_decoder == 'raw':
                lines.append(f"    ops.push(Operand::Immediate {{ value: ((word >> {start}) & 0x{mask:X}) as i64 }});")
            elif imm_decoder.startswith('SImmOperand_'):
                parts = imm_decoder[len('SImmOperand_'):].split('_')
                n = int(parts[0])
                s = int(parts[1]) if len(parts) > 1 else 0
                if s == 0:
                    lines.append(f"    ops.push(Operand::Immediate {{ value: sign_extend((word >> {start}) & 0x{mask:X}, {n}) }});")
                else:
                    lines.append(f"    ops.push(Operand::Immediate {{ value: sign_extend(((word >> {start}) & 0x{mask:X}) << {s}, {n + s}) }});")
            elif imm_decoder.startswith('UImmOperand_'):
                parts = imm_decoder[len('UImmOperand_'):].split('_')
                n = int(parts[0])
                p = int(parts[1]) if len(parts) > 1 else 0
                if p == 0:
                    lines.append(f"    ops.push(Operand::Immediate {{ value: ((word >> {start}) & 0x{mask:X}) as i64 }});")
                else:
                    lines.append(f"    ops.push(Operand::Immediate {{ value: (((word >> {start}) & 0x{mask:X}) + {p}) as i64 }});")
            else:
                lines.append(f"    // TODO: unknown imm_decoder {imm_decoder}")
        else:
            lines.append(f"    // TODO: unknown field at {start}:{length}")
    lines.append("    ops")
    lines.append("}")
    return '\n'.join(lines)

def generate_patterns_rs(results, cases):
    out = []
    out.append("//! LoongArch instruction patterns derived from Capstone decoder tables.")
    out.append("//!")
    out.append("//! This file is auto-generated from Capstone's spec-driven decoder tree.")
    out.append("//! It replaces the exact-word match with proper mask/value patterns.")
    out.append("#![allow(clippy::all, dead_code, unused_mut, unused_variables)]")
    out.append("")
    out.append("use robustone_core::{")
    out.append("    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},")
    out.append("    types::error::{DecodeErrorKind, DisasmError},")
    out.append("};")
    out.append("")
    out.append("/// A single LoongArch instruction pattern.")
    out.append("pub struct LoongArchPattern {")
    out.append("    pub mask: u32,")
    out.append("    pub value: u32,")
    out.append("    pub mnemonic: &'static str,")
    out.append("    pub layout: u16,")
    out.append("}")
    out.append("")

    # Generate extract functions
    used_cases = set(r[3] for r in results)
    for case_num in sorted(used_cases):
        if case_num in cases:
            out.append(generate_extract_fn(case_num, cases[case_num]))
            out.append("")

    # Generate pattern table
    out.append("/// Static pattern table for LoongArch instructions.")
    out.append("pub const LOONGARCH_PATTERNS: &[LoongArchPattern] = &[")
    for name, mask, value, layout in results:
        mnemonic = name.lower().replace('_', '.')
        out.append(f'    LoongArchPattern {{ mask: 0x{mask:08X}, value: 0x{value:08X}, mnemonic: "{mnemonic}", layout: {layout} }},')
    out.append("];")
    out.append("")

    # Generate decode function
    out.append("/// Try to decode a LoongArch instruction from a pattern match.")
    out.append("pub fn try_decode_from_patterns(word: u32, addr: u64) -> Option<Result<DecodedInstruction, DisasmError>> {")
    out.append("    for pattern in LOONGARCH_PATTERNS {")
    out.append("        if (word & pattern.mask) == pattern.value {")
    out.append("            return Some(decode_from_pattern(word, pattern, addr));")
    out.append("        }")
    out.append("    }")
    out.append("    None")
    out.append("}")
    out.append("")

    out.append("fn decode_from_pattern(word: u32, pattern: &LoongArchPattern, addr: u64) -> Result<DecodedInstruction, DisasmError> {")
    out.append("    let operands = match pattern.layout {")
    for case_num in sorted(used_cases):
        out.append(f"        {case_num} => extract_case_{case_num}(word),")
    out.append("        _ => return Err(DisasmError::DecodeFailure {")
    out.append("            kind: DecodeErrorKind::InvalidEncoding,")
    out.append("            architecture: Some(\"loongarch64\".to_string()),")
    out.append("            detail: format!(\"unknown layout {} for LoongArch\", pattern.layout),")
    out.append("        }),")
    out.append("    };")
    out.append("")
    out.append("    Ok(DecodedInstruction {")
    out.append("        architecture: ArchitectureId::LoongArch,")
    out.append("        address: addr,")
    out.append("        mode: \"LA64\".to_string(),")
    out.append("        mnemonic: pattern.mnemonic.to_string(),")
    out.append("        opcode_id: None,")
    out.append("        size: 4,")
    out.append("        raw_bytes: word.to_le_bytes().to_vec(),")
    out.append("        operands,")
    out.append("        registers_read: Vec::new(),")
    out.append("        registers_written: Vec::new(),")
    out.append("        implicit_registers_read: Vec::new(),")
    out.append("        implicit_registers_written: Vec::new(),")
    out.append("        groups: Vec::new(),")
    out.append("        status: DecodeStatus::Success,")
    out.append("        render_hints: RenderHints::default(),")
    out.append("    })")
    out.append("}")
    out.append("")

    # Sign-extend helper
    out.append("fn sign_extend(value: u32, bit_width: u8) -> i64 {")
    out.append("    let mask = 1u32 << (bit_width - 1);")
    out.append("    if value & mask != 0 {")
    out.append("        let sign_extended = value | !((1u32 << bit_width) - 1);")
    out.append("        sign_extended as i32 as i64")
    out.append("    } else {")
    out.append("        value as i64")
    out.append("    }")
    out.append("}")
    out.append("")

    return '\n'.join(out)

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    with open('third_party/capstone/arch/LoongArch/LoongArchGenDisassemblerTables.inc') as f:
        content = f.read()

    table_text = parse_decoder_table(content)
    bytes_list, offset_to_pc, lines = build_bytecode(table_text)
    insn_map = build_insn_map(bytes_list)
    results = trace_decoder(bytes_list, insn_map, offset_to_pc, lines)
    print(f"Traced {len(results)} decode paths")

    cases = parse_cases(content)
    print(f"Parsed {len(cases)} cases")

    rust_code = generate_patterns_rs(results, cases)
    with open('robustone-loongarch/src/patterns.rs', 'w') as f:
        f.write(rust_code)
    print("Wrote robustone-loongarch/src/patterns.rs")

if __name__ == '__main__':
    main()
