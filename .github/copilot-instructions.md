# Robustone AI Coding Conventions

This document outlines the architecture, workflows, and conventions for the Robustone disassembly engine. Following these guidelines will help you contribute effectively.

## Project Overview

Robustone is a Rust-based disassembly engine inspired by Capstone. It is structured as a Cargo workspace with three main crates:

- `robustone-core`: The heart of the engine. It defines the `ArchitectureHandler` trait for architecture-specific disassembly logic and includes modules for instruction decoding and error handling. The core logic is a Rust port of Capstone's C implementation.
- `robustone-cli`: A command-line interface for the engine. It handles argument parsing, input validation, and output formatting, similar to Capstone's `cstool`.
- `robustone`: The main crate that integrates the `robustone-core` and `robustone-cli` crates, providing a unified binary.

The project also includes a `test` directory for parity testing against the original Capstone engine and a `third_party` directory for a submodule of the Capstone project.

## Development Workflow

The primary development workflows are managed through a `Makefile`.

- **Build**: `make build` compiles the project in debug mode.
- **Check**: `make check` runs `cargo check` for fast verification without a full compile.
- **Test**: `make test` is the most comprehensive command. It:
    1. Clones/updates the `capstone` submodule in `third_party/capstone`.
    2. Builds Capstone's `cstool` for comparison.
    3. Runs Python-based parity tests in `test/riscv32/` to compare Robustone's output against Capstone's.
    4. Executes Rust unit tests with `cargo test`.
- **Run CLI**: `make run -- <args>` executes the CLI. For example: `make run -- riscv32 13000513 -d`.

## Architectural Patterns

- **Trait-based Architecture**: `robustone-core` uses the `ArchitectureHandler` trait to support multiple architectures. Each architecture (e.g., RISC-V) implements this trait to provide its disassembly logic. The `ArchitectureDispatcher` selects the appropriate handler at runtime.
- **Feature Flags**: Cargo features are used to enable architecture-specific code. For example, the `riscv` feature in `robustone-core/Cargo.toml` includes the RISC-V implementation.
- **CLI Layer**: The `robustone-cli` crate is responsible for all user-facing interactions. It uses the `clap` crate for argument parsing and provides a user experience consistent with `cstool`.

## Key Files and Directories

- `robustone-core/src/lib.rs`: Defines the core traits and dispatcher.
- `robustone-core/src/riscv/decoder.rs`: The RISC-V instruction decoder.
- `robustone-cli/src/main.rs`: The entry point for the command-line tool.
- `test/riscv32/test_vs_cstool.py`: The Python script for parity testing against Capstone.
- `Makefile`: The central point for all build, test, and run commands.

When adding a new architecture, you will need to:
1. Create a new module under `robustone-core/src/`.
2. Implement the `ArchitectureHandler` trait for the new architecture.
3. Add a feature flag to `robustone-core/Cargo.toml` and `robustone/Cargo.toml`.
4. Update the `ArchitectureDispatcher` in `robustone-core/src/lib.rs` to include the new handler.
5. Add parity tests in the `test/` directory.
