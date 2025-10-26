# Parity Tests

本目录包含与 Capstone cstool 的对照测试（parity tests）。新的统一测试运行器会自动发现 `test/<arch>/config.json` 并对每个架构运行用例。

## 快速开始

- 运行所有架构：
  - `python3 test/run_parity.py --all`
- 仅运行某个架构（如 riscv32）：
  - `python3 test/run_parity.py --arch riscv32`
- 限制用例数量（加快本地验证）：
  - `python3 test/run_parity.py --arch riscv32 --limit 20`

也可以直接使用顶层 Makefile：

- `make test` 会自动构建 cstool、运行所有 parity 测试，然后执行 `cargo test`。

## 目录结构

- `test/run_parity.py`：通用测试运行器。
- `test/<arch>/config.json`：单个架构的配置。
- `test/<arch>/verified_instructions.txt`：用例文件，每行一个十六进制机器码；可在注释中附上期望的 cstool 输出（用于漂移检测）。

用例文件格式示例：

```
13000513 # addi a0, zero, 0x13 | 可选注释（例如来源或说明）
```

- 井号 `#` 左侧是十六进制指令串
- `#` 右侧第一段（到 `|` 之前）如存在，则被视为「当时记录的 cstool 输出」，用于检测 cstool 行为漂移
- `|` 之后是可选备注说明

## 新增架构指南

1. 新建目录：`test/<arch>/`
2. 生成配置文件 `config.json`（示例）：
   ```json
   {
     "name": "riscv64",
     "robustone_arch": "riscv64",
     "cstool_arch": "riscv64",
     "cases_file": "verified_instructions.txt",
     "robustone_flags": [],
     "cstool_flags": []
   }
   ```
3. 添加用例文件：`verified_instructions.txt`
4. 运行：`python3 test/run_parity.py --arch <arch>`

如需特殊命令行开关（例如 x86 语法选项），可在 `robustone_flags` / `cstool_flags` 中添加，如 `"-d"`、`"--unsigned-immediate"` 或 `"-s"` 等（对应 Robustone / cstool 自身的参数）。

## 注意事项

- 首次运行会自动尝试构建 cstool（通过 `test/build_cstool.sh`）。
- Robustone 默认构建 debug 版本，如未找到会自动构建。
- 输出比对采用严格文本相等（行内空白差异不忽略），确保完全一致的行为。