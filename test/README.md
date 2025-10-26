# Robustone 测试框架

一个模块化的测试框架，用于比较 Robustone CLI 输出与 Capstone 的 cstool 参考实现。

## 🚀 快速开始

### 基本用法

```bash
# 测试所有架构
python3 test/run_tests.py --all

# 测试特定架构
python3 test/run_tests.py --arch riscv32

# 限制测试用例数量（适合快速验证）
python3 test/run_tests.py --arch riscv32 --limit 20

# 列出所有可用架构
python3 test/run_tests.py --list
```

### 详细输出

```bash
# 显示详细的失败信息
python3 test/run_tests.py --arch riscv32 --show-details

# 显示每个测试用例的进度
python3 test/run_tests.py --arch riscv32 --verbose

# 遇到第一个失败就停止
python3 test/run_tests.py --arch riscv32 --fail-fast
```

## 📁 目录结构

```
test/
├── core/                           # 核心测试框架
│   ├── __init__.py
│   ├── test_runner.py             # 测试运行器
│   ├── comparator.py              # 输出比较器
│   ├── arch_config.py             # 架构配置管理
│   └── utils.py                   # 工具函数
├── architectures/                  # 架构特定配置
│   └── riscv32/
│       ├── config.json            # 架构配置
│       └── test_cases.txt         # 测试用例
├── scripts/                        # 辅助脚本
│   ├── build_cstool.sh           # cstool 构建脚本
│   ├── generate_test_cases.py    # 生成测试用例
│   └── validate_configs.py       # 验证配置文件
├── reports/                        # 测试报告目录
├── run_tests.py                   # 主测试入口
└── README.md
```

## ⚙️ 架构配置

### 添加新架构

```bash
# 创建新架构配置
python3 test/run_tests.py --init new_arch

# 这将创建：
# test/architectures/new_arch/config.json
# test/architectures/new_arch/test_cases.txt
```

### 配置文件格式

`config.json` 示例：

```json
{
  "name": "riscv32",
  "robustone_arch": "riscv32",
  "cstool_arch": "riscv32",
  "cases_file": "test_cases.txt",
  "robustone_flags": [],
  "cstool_flags": [],
  "description": "RISC-V 32-bit instruction set tests",
  "category": "riscv"
}
```

配置项说明：
- `name`: 架构名称（唯一标识）
- `robustone_arch`: Robustone 使用的架构参数
- `cstool_arch`: cstool 使用的架构参数
- `cases_file`: 测试用例文件路径
- `robustone_flags`: Robustone 的额外命令行参数
- `cstool_flags`: cstool 的额外命令行参数
- `description`: 架构描述
- `category`: 测试分类（用于组织）

## 📝 测试用例格式

`test_cases.txt` 格式：

```
# 注释行以 # 开始
# 格式：<hex_bytes> [| <期望的cstool输出>] [| <备注>]

37010000  # 0  37 01 00 00  lui    sp, 0
130101ff  # 0  13 01 01 ff  addi   sp, sp, -0x10 | 加载立即数到栈指针
b3003100  # 0  b3 00 31 00  add    ra, sp, gp
```

格式说明：
- `hex_bytes`: 十六进制指令字节（必选）
- `期望的cstool输出`: 用于检测文档漂移（可选）
- `备注`: 测试用例说明（可选）

## 🛠️ 辅助工具

### 验证配置

```bash
# 验证所有架构配置
python3 test/scripts/validate_configs.py
```

### 生成测试用例

```bash
# 使用预定义指令集生成测试用例
python3 test/scripts/generate_test_cases.py --arch riscv32

# 使用指定指令生成测试用例
python3 test/scripts/generate_test_cases.py --arch riscv32 \
  --instructions 37010000 130101ff b3003100
```

### 构建 cstool

```bash
# 手动构建 cstool（通常由测试框架自动处理）
bash test/scripts/build_cstool.sh
```

## 🔧 高级用法

### 自定义比较模式

```bash
# 宽松匹配（忽略空白字符）
python3 test/run_tests.py --arch riscv32 --loose-match

# 严格匹配（精确字符串比较）
python3 test/run_tests.py --arch riscv32 --ignore-whitespace=false
```

### 批量操作

```bash
# 测试多个架构
python3 test/run_tests.py --arch riscv32 --arch riscv64

# 限制失败显示数量
python3 test/run_tests.py --all --show-failures 20
```

## 📊 测试报告

测试运行后会显示：

```
============================================================
Results for riscv32:
============================================================
Total cases:     60
Matches:         58 (96.7%)
Mismatches:      2
Command failures: 0
Documentation drift: 0
Execution time:  1250ms

Overall success rate: 96.7%
```

失败时会显示详细差异：

```
1. 130101ff (mismatch)
   Expected: addi   sp, sp, -0x10
   Robustone: addi   sp, sp, -16
   Cstool:    addi   sp, sp, -0x10
```

## 🐛 故障排除

### 常见问题

1. **cstool 未找到**
   ```
   错误: cstool not found at path/to/cstool
   解决: 确保已安装 Capstone 并运行构建脚本
   ```

2. **robustone 构建失败**
   ```
   错误: Failed to build robustone
   解决: 检查 Rust 环境和依赖项
   ```

3. **测试用例解析错误**
   ```
   错误: Invalid test case format
   解决: 检查 test_cases.txt 格式是否符合要求
   ```

### 调试技巧

```bash
# 详细输出模式
python3 test/run_tests.py --arch riscv32 --verbose --show-details

# 单个测试用例调试
python3 test/run_tests.py --arch riscv32 --limit 1 --show-details
```

## 🔄 与旧框架的迁移

如果你在使用旧的测试框架，迁移步骤：

1. **架构配置迁移**：
   ```bash
   # 旧配置在 test/riscv32/config.json
   # 新配置在 test/architectures/riscv32/config.json
   # 格式基本兼容，可直接复制
   ```

2. **测试用例迁移**：
   ```bash
   # 重命名测试文件
   mv test/riscv32/verified_instructions.txt test/architectures/riscv32/test_cases.txt
   ```

3. **运行方式更新**：
   ```bash
   # 旧方式
   python3 test/run_parity.py --arch riscv32

   # 新方式
   python3 test/run_tests.py --arch riscv32
   ```

## 🤝 贡献指南

### 添加新测试用例

1. 找到对应的架构目录
2. 在 `test_cases.txt` 中添加新行
3. 可选：运行 cstool 获取期望输出
4. 提交前运行完整测试验证

### 扩展框架

核心模块位于 `test/core/`：
- `test_runner.py`: 测试执行逻辑
- `comparator.py`: 输出比较算法
- `arch_config.py`: 配置管理
- `utils.py`: 通用工具函数

## 📄 许可证

本测试框架遵循与项目相同的许可证。