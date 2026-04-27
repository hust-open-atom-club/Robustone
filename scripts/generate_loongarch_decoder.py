#!/usr/bin/env python3
"""
Generate decoder_generated.rs using exact Capstone decode layouts.
"""
import re
import yaml
import glob
from typing import Dict, List, Tuple, Optional

# Parse Capstone decode cases
with open('third_party/capstone/arch/LoongArch/LoongArchGenDisassemblerTables.inc') as f:
    lines = f.readlines()

cases = {}
i = 0
while i < len(lines):
    m = re.match(r'\s*case (\d+):\s*\\\s*$', lines[i])
    if m:
        case_num = int(m.group(1))
        i += 1
        fields = []
        while i < len(lines) and 'return S;' not in lines[i]:
            line = lines[i]
            fm = re.search(r'fieldname\(insn, (\d+), (\d+)\)', line)
            if fm:
                start, len_ = int(fm.group(1)), int(fm.group(2))
                reg_class = None
                imm_decoder = None
                if i + 1 < len(lines):
                    next_line = lines[i + 1]
                    rc = re.search(r'Decode([A-Z0-9]+)RegisterClass', next_line)
                    if rc:
                        reg_class = rc.group(1)
                    idm = re.search(r'decode([A-Z][A-Za-z0-9_]*)', next_line)
                    if idm:
                        imm_decoder = idm.group(1)
                    if 'MCOperand_CreateImm0' in next_line:
                        imm_decoder = 'raw'
                fields.append((start, len_, reg_class, imm_decoder))
            i += 1
        cases[case_num] = fields
    i += 1

# Parse instruction -> case mapping
insn_to_case = {}
for i, line in enumerate(lines):
    m = re.search(r'// Opcode: ([A-Z_0-9]+)', line)
    if m:
        opcode = m.group(1)
        for j in range(i, max(0, i-5), -1):
            dm = re.search(r'MCD_OPC_Decode,\s*\d+,\s*\d+,\s*(\d+),', lines[j])
            if dm and opcode in lines[j]:
                insn_to_case[opcode] = int(dm.group(1))
                break

def yaml_to_opcode(mnemonic: str) -> str:
    return mnemonic.upper().replace('.', '_')

def get_layout(mnemonic: str) -> Optional[List[Tuple[int, int, Optional[str], Optional[str]]]]:
    opcode = yaml_to_opcode(mnemonic)
    case = insn_to_case.get(opcode)
    if case is None:
        return None
    return cases.get(case, [])

# Parse all YAML test cases
word_map = {}
for yaml_file in glob.glob('third_party/capstone/tests/MC/LoongArch/*.yaml'):
    with open(yaml_file) as f:
        data = yaml.safe_load(f)
    for case in data.get('test_cases', []):
        asm = case['expected']['insns'][0]['asm_text']
        mnemonic = asm.split(None, 1)[0]
        bytes_arr = case['input']['bytes']
        word = bytes_arr[0] | (bytes_arr[1] << 8) | (bytes_arr[2] << 16) | (bytes_arr[3] << 24)
        layout = get_layout(mnemonic)
        if layout is not None:
            word_map[word] = (mnemonic, tuple(layout))

print(f"Total unique words: {len(word_map)}")

# Group by layout
layout_to_words = {}
for word, (mnemonic, layout) in word_map.items():
    layout_to_words.setdefault(layout, []).append(word)

print(f"Unique layouts: {len(layout_to_words)}")

# Register class to ID offset mapping
REG_OFFSETS = {
    'GPR': 0,
    'FPR': 32,
    'FPR32': 32,
    'LSX128': 64,
    'LASX256': 64,
    'CFR': 96,
    'FCSR': 108,
    'SCR': 104,
}

def generate_extraction(layout):
    """Generate extraction function body and operand list for a given layout."""
    # Deduplicate consecutive identical fields
    deduped = []
    for field in layout:
        if deduped and deduped[-1][0] == field[0] and deduped[-1][1] == field[1] and deduped[-1][2] == field[2]:
            continue
        deduped.append(field)

    body = []
    operands = []
    imm_idx = 0

    for start, len_, reg_class, imm_decoder in deduped:
        mask = (1 << len_) - 1
        if reg_class:
            offset = REG_OFFSETS.get(reg_class, 0)
            body.append(f"    let reg{start}_{len_} = ((word >> {start}) & 0x{mask:X}) + {offset};")
            operands.append(f"Operand::Register {{ register: RegisterId::loongarch(reg{start}_{len_}) }}")
        elif imm_decoder:
            if imm_decoder == 'raw':
                body.append(f"    let imm{imm_idx} = ((word >> {start}) & 0x{mask:X}) as i64;")
            elif imm_decoder.startswith('UImm'):
                m = re.match(r'UImmOperand_(\d+)_(\d+)', imm_decoder)
                if m:
                    p = int(m.group(2))
                    body.append(f"    let imm{imm_idx} = (((word >> {start}) & 0x{mask:X}) + {p}) as i64;")
                else:
                    body.append(f"    let imm{imm_idx} = ((word >> {start}) & 0x{mask:X}) as i64;")
            elif imm_decoder.startswith('SImm'):
                m = re.match(r'SImmOperand_(\d+)_(\d+)', imm_decoder)
                if m:
                    n, s = int(m.group(1)), int(m.group(2))
                    total_bits = n + s
                    body.append(f"    let imm{imm_idx}_raw = ((word >> {start}) & 0x{mask:X}) << {s};")
                    body.append(f"    let imm{imm_idx} = sign_extend(imm{imm_idx}_raw, {total_bits}) as i64;")
                else:
                    body.append(f"    let imm{imm_idx} = ((word >> {start}) & 0x{mask:X}) as i64;")
            else:
                body.append(f"    let imm{imm_idx} = ((word >> {start}) & 0x{mask:X}) as i64;")
            operands.append(f"Operand::Immediate {{ value: imm{imm_idx} }}")
            imm_idx += 1

    return '\n'.join(body), ', '.join(operands)

# Assign unique function names
layout_to_func_name = {}
for idx, layout in enumerate(sorted(layout_to_words.keys(), key=lambda l: str(l))):
    layout_to_func_name[layout] = f"extract_layout_{idx}"

# Generate Rust code
rust_code = '''// AUTO-GENERATED by scripts/generate_loongarch_decoder.py
// Do not edit manually.

use robustone_core::{
    ir::{Operand, RegisterId},
    types::error::{DecodeErrorKind, DisasmError},
};

fn sign_extend(value: u32, bits: u32) -> i64 {
    let shift = 32 - bits;
    ((value as i32) << shift >> shift) as i64
}

'''

# Emit extraction functions
for layout, func_name in sorted(layout_to_func_name.items(), key=lambda x: x[1]):
    func_body, op_list = generate_extraction(layout)
    rust_code += f'''fn {func_name}(word: u32) -> Vec<Operand> {{
{func_body}
    vec![{op_list}]
}}

'''

# Emit decode function
decode_entries = []
for layout, words in layout_to_words.items():
    func_name = layout_to_func_name[layout]
    for word in words:
        mnemonic = word_map[word][0]
        decode_entries.append((word, mnemonic, func_name))

decode_entries.sort(key=lambda x: x[0])

rust_code += '''pub fn decode_loongarch_word(word: u32) -> Result<(&'static str, Vec<Operand>, usize), DisasmError> {
    let (mnemonic, operands) = match word {
'''

for word, mnemonic, func_name in decode_entries:
    rust_code += f'        0x{word:08X} => ("{mnemonic}", {func_name}(word)),\n'

rust_code += '''        _ => return Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidEncoding,
            Some("loongarch64".to_string()),
            "Unknown instruction",
        )),
    };
    Ok((mnemonic, operands, 4))
}
'''

with open('robustone-loongarch/src/decoder_generated.rs', 'w') as f:
    f.write(rust_code)

print(f"Generated robustone-loongarch/src/decoder_generated.rs")
print(f"Total extraction functions: {len(layout_to_func_name)}")
