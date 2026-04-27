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
            fm = re.search(r'fieldname\(insn, (\d+), (\d+)\)', lines[i])
            if fm:
                fields.append((int(fm.group(1)), int(fm.group(2))))
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

def get_layout(mnemonic: str) -> Optional[List[Tuple[int, int]]]:
    opcode = yaml_to_opcode(mnemonic)
    case = insn_to_case.get(opcode)
    if case is None:
        return None
    return cases.get(case, [])

# Determine register class from decode function name
def get_reg_class(line: str) -> Optional[str]:
    m = re.search(r'Decode([A-Z0-9]+)RegisterClass', line)
    if m:
        return m.group(1)
    return None

# Parse all YAML test cases
word_map = {}  # word -> (mnemonic, layout, asm_text)
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
            word_map[word] = (mnemonic, tuple(layout), asm)

print(f"Total unique words: {len(word_map)}")

# Group by layout
layout_to_words = {}
for word, (mnemonic, layout, asm) in word_map.items():
    layout_to_words.setdefault(layout, []).append(word)

print(f"Unique layouts: {len(layout_to_words)}")

# Print some layouts
for layout, words in sorted(layout_to_words.items(), key=lambda x: -len(x[1]))[:10]:
    print(f"Layout {layout}: {len(words)} words, example: {word_map[words[0]][0]} {word_map[words[0]][2]}")
