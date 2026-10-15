# Robustone

Robustone 是由华中科技大学开源原子俱乐部用 Rust 编写的实验性反汇编引擎。受 Capstone 项目启发，它探索如何利用 Rust 强大的安全保证来提供与 Capstone 兼容的体验，同时拥有更清洁的代码库和现代化的工具链。

## 兼容性边界

Robustone 将与 Capstone 的兼容目标拆分为三层：

- CLI 兼容：尽量保持与 `cstool` 接近的命令行体验，包括 `arch+mode` 语法、原始十六进制输入和细节输出选项。
- 语义兼容：在当前一致性测试覆盖的指令流上，尽量对齐助记符、操作数格式、寄存器命名和 detail 输出。
- API 兼容：在 Rust API 中表达相同语义，但明确标注当前哪些地方还不是 Capstone 的 handle/options/detail 风格接口。

当前仓库状态：

- 公开支持矩阵：[`docs/support-matrix.md`](docs/support-matrix.md)
- 后端稳定性分级：
  - **Beta**：`riscv32`、`riscv64` — 已通过一致性测试，有模糊测试覆盖，适合受控的内部使用。
  - **ExperimentalDecode**：`aarch64`、`x32`、`x64`、`loongarch64` — `x32`、`x64` 的解码覆盖有限；`loongarch64` 依赖语料库精确字匹配，尚未推广到任意的有效编码。`aarch64` 后端为 spec-driven，覆盖基础整数、分支、存取、标量浮点、向量（SIMD/SVE/SME）、加密和系统指令；尚未实现 detail 输出。
  - **ParserOnly**：其余所有 token — CLI 接受解析以进行能力发现，但未注册解码后端。
- 这是一个处于活跃开发中的多 ISA 框架，尚未成为 Capstone 的直接替代品。

## 系统要求

- [Rust](https://www.rust-lang.org/tools/install) 1.85 或更新版本（支持 2024 edition）。
- [Python](https://www.python.org/) 3.8 或更新版本（用于一致性测试）。
- `git` 和基本构建工具（用于获取 Capstone 参考实现）。

## 项目结构

```
robustone/         # 顶层 crate（同时提供库和二进制入口）
robustone-core/    # 架构相关的解码与格式化核心
robustone-cli/     # CLI 参数解析、输入校验与展示逻辑
robustone-arm/     # AArch64 spec-driven 解码后端
docs/              # 支持矩阵和项目文档
xtask/             # 基于 TableGen 的架构 spec 代码生成管线
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
make run -- riscv32 93001000 -d
```

十六进制输入遵循 `cstool` 约定，按内存中的原始字节顺序解释。

或者，您可以使用 `RUN_ARGS` 变量。这样可以防止 `make` 错误解析 `-d` 等标志：

```bash
make run RUN_ARGS="riscv32 93001000 -d"
```

如果需要查看当前 CLI 暴露的完整参数面，请运行：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --help
```

如果需要直接查看共享能力注册表导出的支持面，请运行：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --capabilities
cargo run --manifest-path robustone/Cargo.toml -- --json --capabilities
```

如果需要从共享解码 IR 导出结构化 JSON，请运行：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json riscv32 93001000
```

如需解码 AArch64 指令，可通过 spec-driven 后端：

```bash
cargo run --manifest-path robustone/Cargo.toml -- --json aarch64 d503201f
```

## Docker 镜像

在本地构建并运行生产 CLI 镜像：

```bash
docker build -t robustone:local .
docker run --rm robustone:local --help
docker run --rm robustone:local riscv32 93001000
```

多阶段 `Dockerfile` 使用 Rust 1.99.0 和仓库中的 `Cargo.lock` 构建
`robustone` 二进制程序，再以非 root 用户在 Debian Bookworm 中运行。
单独的 `Dockerfile.dev` 不参与生产镜像工作流。

### 手动构建并推送至 GHCR

[Docker Image 工作流](.github/workflows/docker.yml) 仅通过
`workflow_dispatch` 手动触发；推送代码或创建 PR 都不会触发它。

1. 先将工作流合并到**你自己仓库的默认分支**。GitHub 要求该文件存在于默认分支，
   才会显示 **Run workflow** 按钮；仅添加到功能分支不会显示该按钮。
2. 如果 GitHub 提示，在 fork 中启用 Actions，然后打开
   **Actions → Docker Image → Run workflow**。
3. 选择要构建的分支并运行。工作流会检出选定的提交，构建 `linux/amd64` 镜像，
   测试 CLI，再将经过测试的镜像推送到 GHCR。

镜像名称为全小写的 `ghcr.io/<owner>/<repository>`，例如该 fork 中的
`ghcr.io/paulkm2006/robustone`。每次运行都会发布对应检出提交的
`sha-<完整提交 SHA>` 标签。仅针对默认分支的运行会同时更新 `latest`；
功能分支或 Git 标签的运行不会覆盖它。发布的镜像标签会列在运行摘要中。

工作流使用内置 `GITHUB_TOKEN`，权限仅为 `contents: read` 和
`packages: write`，无需额外的镜像仓库密钥。如果同名 package 已存在，需要为当前
仓库授予该 package 的 Actions 写入权限。新建的 GHCR package 默认是私有的；
除非将其设为公开，否则拉取镜像需要相应的仓库访问权限。

```bash
# 将 owner 和 SHA 替换为工作流运行摘要中的值。
docker pull ghcr.io/<owner>/robustone:sha-<完整提交SHA>
docker run --rm ghcr.io/<owner>/robustone:sha-<完整提交SHA> riscv32 93001000
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

以上命令已于 2026-05-29 在本地验证通过。

## CI 与项目文档

- CI 工作流：`.github/workflows/ci.yml`（执行 `make check`、`cargo test --workspace --all-features`、`make test`，并提供定时 fuzz smoke）
- 支持矩阵：[`docs/support-matrix.md`](docs/support-matrix.md)
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
