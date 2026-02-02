# Robustone 贡献指南

感谢您对 Robustone 项目的关注。本文档提供了参与项目贡献的指南和说明。

## 目录

- [环境要求](#环境要求)
- [开发环境配置](#开发环境配置)
- [Pre-commit 钩子](#pre-commit-钩子)
- [代码规范](#代码规范)
- [测试](#测试)
- [提交更改](#提交更改)
- [Pull Request 检查清单](#pull-request-检查清单)

## 环境要求

在开始贡献之前，请确保已安装以下工具：

- [Rust](https://www.rust-lang.org/tools/install) 1.75 或更新版本（2021 edition）
- [Python](https://www.python.org/) 3.8 或更新版本
- `git` 和基本构建工具
- `make`（用于执行 Makefile 命令）

## 开发环境配置

1. 克隆仓库：

```bash
git clone https://github.com/hust-open-atom-club/robustone.git
cd robustone
```

2. 配置 Python 虚拟环境：

```bash
make virt-env
```

3. 安装 pre-commit 钩子：

```bash
source virt-py/bin/activate
pre-commit install
pre-commit install --hook-type pre-push
```

4. 验证配置：

```bash
make build
make check
```

## Pre-commit 钩子

本项目使用 pre-commit 钩子在每次提交前确保代码质量。钩子包括：

**提交时执行：**
- `rustfmt` - Rust 代码格式化
- `clippy` - Rust 代码检查（使用 `-D warnings`）
- `cargo check` - Rust 编译检查
- `black` - Python 代码格式化
- `pylint` - Python 代码检查
- 尾随空格移除
- 文件末尾换行符修复
- YAML/TOML/JSON 格式验证
- 合并冲突检测

**推送时执行：**
- `cargo test` - 完整测试套件

手动运行所有钩子：

```bash
pre-commit run --all-files
```

临时跳过钩子（不推荐）：

```bash
git commit --no-verify
```

## 代码规范

### Rust

- 遵循 `rustfmt` 强制执行的代码风格
- 所有公共项需要文档注释（`///`）
- 可失败的操作使用 `Result<T, DisasmError>`
- 库代码中禁止使用 `unwrap()`，使用 `?` 操作符
- 函数参数优先使用 `&str` 而非 `String`
- 导入分组：标准库、外部 crate、本地 crate

示例：

```rust
use std::collections::HashMap;

use clap::Parser;

use crate::error::DisasmError;
use super::types::*;
```

### Python

- 遵循 PEP 8 规范，由 `black` 和 `pylint` 强制执行
- 最大行长度：120 字符
- 适当使用类型提示
- 配置位于 `pyproject.toml`

## 测试

### 运行测试

运行完整测试套件：

```bash
make test
```

此命令会：
1. 克隆 Capstone（如果不存在）
2. 构建 Capstone 比较工具
3. 运行与 Capstone 的一致性测试
4. 运行 Rust 单元测试

### 快速测试

开发过程中进行快速迭代：

```bash
# 快速一致性测试（20 个用例）
make test-quick

# 仅运行 Rust 单元测试
cargo test --manifest-path robustone/Cargo.toml

# 仅运行一致性测试
make test-parity
```

### 添加测试

添加新指令或功能时：

1. 在相关 Rust 模块中添加单元测试
2. 在 `test/` 目录中添加一致性测试用例
3. 验证配置：`make test-validate`

## 提交更改

1. 从 `main` 创建新分支：

```bash
git checkout -b feature/your-feature-name
```

2. 进行更改，确保：
   - 所有 pre-commit 钩子通过
   - 所有测试通过
   - 必要时更新文档

3. 使用清晰的提交信息提交更改：

```bash
git add .
git commit -m "feat: add support for XYZ instruction"
```

提交信息格式：
- `feat:` 新功能
- `fix:` 错误修复
- `docs:` 文档更改
- `refactor:` 代码重构
- `test:` 测试添加或更改
- `chore:` 维护任务

4. 推送并创建 Pull Request：

```bash
git push origin feature/your-feature-name
```

## Pull Request 检查清单

提交 Pull Request 前，请验证：

- [ ] 代码编译无警告（`make build`）
- [ ] 所有 lint 检查通过（`make check`）
- [ ] 所有测试通过（`make test`）
- [ ] Pre-commit 钩子通过（`pre-commit run --all-files`）
- [ ] 新代码包含适当的测试
- [ ] 公共 API 包含文档
- [ ] 提交信息遵循上述格式

## 添加新架构

实现新架构支持时：

1. 在 `robustone-core/src/<arch>/` 下创建新模块
2. 实现 `ArchitectureHandler` trait
3. 在 `robustone-core/Cargo.toml` 中添加 feature flag
4. 在 `ArchitectureDispatcher::new()` 中注册处理器
5. 在 `test/<arch>/` 中添加一致性测试
6. 更新文档

## 问题咨询

如有贡献相关问题，请在 GitHub 上提交 issue。
