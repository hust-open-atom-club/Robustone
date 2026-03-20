# Robustone

Robustone 是由华中科技大学开源原子俱乐部用 Rust 编写的实验性反汇编引擎。受 Capstone 项目启发，它探索如何利用 Rust 强大的安全保证来提供与 Capstone 兼容的体验，同时拥有更清洁的代码库和现代化的工具链。

## 兼容性边界

Robustone 将与 Capstone 的兼容目标拆分为三层：

- CLI 兼容：尽量保持与 `cstool` 接近的命令行体验，包括 `arch+mode` 语法、原始十六进制输入和细节输出选项。
- 语义兼容：在当前一致性测试覆盖的指令流上，尽量对齐助记符、操作数格式、寄存器命名和 detail 输出。
- API 兼容：在 Rust API 中表达相同语义，但明确标注当前哪些地方还不是 Capstone 的 handle/options/detail 风格接口。

当前仓库状态：

- 已实现的解码后端：`riscv`、`riscv32`、`riscv64`
- 当前支持矩阵：[`docs/support-matrix.md`](docs/support-matrix.md)
- 版本路线图：[`docs/roadmap.md`](docs/roadmap.md)

## 系统要求

- [Rust](https://www.rust-lang.org/tools/install) 1.85 或更新版本（支持 2024 edition）。
- [Python](https://www.python.org/) 3.8 或更新版本（用于一致性测试）。
- `git` 和基本构建工具（用于获取 Capstone 参考实现）。

## 项目结构

```
robustone/         # 顶层 crate（同时提供库和二进制入口）
robustone-core/    # 架构相关的解码与格式化核心
robustone-cli/     # CLI 参数解析、输入校验与展示逻辑
docs/              # 路线图、支持矩阵和项目文档
tests/             # golden/property/differential 测试资源
fuzz/              # 解码器与 JSON 格式化的 fuzz 目标
Makefile           # build/check/run/test 入口
test/
	architectures/ # 各架构的一致性测试配置和测试集
	run_tests.py   # 一致性测试主入口
third_party/
	capstone/      # 原始 Capstone 项目的可选检出（测试使用）
Cargo.toml        # Workspace 清单
```

## 快速开始

克隆仓库（包括子模块，如果有的话）并安装上述工具链要求。捆绑的 `Makefile` 为常见工作流程提供了快捷方式：

| 目标          | 描述 |
| ------------- | ---- |
| `make build`  | 以调试模式编译 crate。 |
| `make check`  | 对 workspace 代码和测试框架 Python 脚本运行仓库检查（`rustfmt`、`clippy`、`black`、`pylint`）。 |
| `make format` | 格式化 Rust workspace 和 Python 一致性测试脚本。 |
| `make run`    | 以调试模式启动 CLI（接受与 `cargo run` 相同的参数）。 |
| `make test`   | 构建 Capstone（如果缺失），运行一致性测试，并执行 Rust workspace 测试。 |
| `make test-quick` | 运行较小规模的一致性测试，便于快速迭代。 |
| `make help`   | 输出仓库命令摘要。 |

`test` 目标在首次使用时会将 Capstone 下载到 `third_party/capstone`，通过 `test/scripts/build_cstool.sh` 构建比较工具，运行 `python3 test/run_tests.py --all`，最后执行 `cargo test --workspace --all-features`。

## 运行 CLI

当前已实现的 RISC-V 后端尽量镜像经典 `cstool` 的使用方式。例如，要解码一条 RISC-V 指令并显示详细信息：

```bash
make run -- riscv32 130101ff -d
```

或者，您可以使用 `RUN_ARGS` 变量。这样可以防止 `make` 错误解析 `-d` 等标志：

```bash
make run RUN_ARGS="riscv32 130101ff -d"
```

如果需要查看当前 CLI 暴露的完整参数面，请运行：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --help
```

如果需要从共享解码 IR 导出结构化 JSON，请运行：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

## 测试

从仓库根目录运行完整的回归测试套件：

```bash
make test
```

此命令会：

1. 确保 Capstone 在 `third_party/capstone` 下可用（如有必要会克隆仓库）。
2. 使用 `test/scripts/build_cstool.sh` 构建 Capstone 的 `cstool` 辅助工具。
3. 执行 Python 一致性测试入口 `python3 test/run_tests.py --all`。
4. 运行 `cargo test --workspace --all-features` 检查 Rust workspace 测试。

其他常用验证命令：

```bash
python3 test/run_tests.py --list
python3 test/run_tests.py --arch riscv32 --limit 20 --verbose
cargo test --workspace --all-features
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

以上命令已于 2026-03-20 在本地验证通过。

## CI 与项目文档

- CI 工作流：`.github/workflows/ci.yml`（执行 `make check`、`cargo test --workspace --all-features`、`make test`，并提供定时 fuzz smoke）
- 支持矩阵：[`docs/support-matrix.md`](docs/support-matrix.md)
- 路线图：[`docs/roadmap.md`](docs/roadmap.md)
- 已知差异：[`tests/differential/known-differences.toml`](tests/differential/known-differences.toml)
- 新 ISA 清单：[`docs/isa-checklist.md`](docs/isa-checklist.md)
- 基准基线：[`docs/benchmark-baselines.md`](docs/benchmark-baselines.md)
- 发布清单：[`docs/release-checklist.md`](docs/release-checklist.md)
- 版本策略：[`docs/versioning-policy.md`](docs/versioning-policy.md)
- 测试框架说明：[`test/README.md`](test/README.md)

## 贡献

我们欢迎贡献。请阅读 [CONTRIBUTING_CN.md](CONTRIBUTING_CN.md) 了解以下内容：

- 开发环境配置
- Pre-commit 钩子的安装和使用
- 代码规范要求
- 测试流程
- 提交 Pull Request
