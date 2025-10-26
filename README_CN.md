# Robustone

Robustone 是由华中科技大学开源原子俱乐部用 Rust 编写的实验性反汇编引擎。受 Capstone 项目启发，它探索如何利用 Rust 强大的安全保证来提供与 Capstone 兼容的体验，同时拥有更清洁的代码库和现代化的工具链。

## 系统要求

- [Rust](https://www.rust-lang.org/tools/install) 1.75 或更新版本（2021 版本）。
- [Python](https://www.python.org/) 3.8 或更新版本（用于一致性测试）。
- `git` 和基本构建工具（用于获取 Capstone 参考实现）。

## 项目结构

```
robustone/
	src/
		cli/         # 命令行解析、输入验证和展示逻辑
		transfer/    # 架构特定的解码和格式化（Capstone 的 Rust 移植版）
	Cargo.toml     # 主要 crate 清单
test/
	riscv32/       # RISC-V 一致性检查的 Python 脚本和测试用例
third_party/
	capstone/      # 原始 Capstone 项目的可选检出（测试使用）
```

## 快速开始

克隆仓库（包括子模块，如果有的话）并安装上述工具链要求。捆绑的 `Makefile` 为常见工作流程提供了快捷方式：

| 目标          | 描述 |
| ------------- | ---- |
| `make build`  | 以调试模式编译 crate。 |
| `make check`  | 运行 `cargo check` 进行快速类型验证。 |
| `make format` | 使用 `rustfmt` 格式化 Rust 代码库。 |
| `make run`    | 以调试模式启动 CLI（接受与 `cargo run` 相同的参数）。 |
| `make test`   | 构建 Capstone（如果缺失），运行一致性测试，并执行 Rust 单元测试。 |

`test` 目标在首次使用时会将 Capstone 下载到 `third_party/capstone`，构建比较工具，运行 `test/riscv32/test_vs_cstool.py`，最后执行 `cargo test`。

## 运行 CLI

CLI 镜像了经典的 `cstool` 用户体验。例如，要解码 RISC-V 指令并显示详细信息：

```bash
make run -- riscv32 13000513 -d
```

该命令也接受不带显式 `--` 分隔符的参数：

```bash
make run riscv32 13000513 -d
```

在内部，目标会将任何尾随单词转发给二进制文件（或者您可以通过 `RUN_ARGS="..."` 传递它们）。

## 测试

从仓库根目录运行完整的回归测试套件：

```bash
make test
```

此命令会：

1. 确保 Capstone 在 `third_party/capstone` 下可用（如有必要会克隆仓库）。
2. 使用 `test/build_cstool.sh` 构建 Capstone 的 `cstool` 辅助工具。
3. 执行 Python 一致性测试工具 `test/riscv32/test_vs_cstool.py`，并在精选的指令列表中比较 Robustone 输出与 Capstone。
4. 运行 `cargo test` 进行 Rust 单元测试覆盖。

如果您只需要验证 Python 比较脚本，可以直接运行：

```bash
python3 test/riscv32/test_vs_cstool.py
```
