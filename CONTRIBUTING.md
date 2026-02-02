# Contributing to Robustone

Thank you for your interest in contributing to Robustone. This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Pre-commit Hooks](#pre-commit-hooks)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Pull Request Checklist](#pull-request-checklist)

## Prerequisites

Before contributing, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or newer (edition 2021)
- [Python](https://www.python.org/) 3.8 or newer
- `git` and basic build tools
- `make` (for running Makefile commands)

## Development Setup

1. Clone the repository:

```bash
git clone https://github.com/hust-open-atom-club/robustone.git
cd robustone
```

2. Set up the Python virtual environment:

```bash
make virt-env
```

3. Install pre-commit hooks:

```bash
source virt-py/bin/activate
pre-commit install
pre-commit install --hook-type pre-push
```

4. Verify the setup:

```bash
make build
make check
```

## Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality before each commit. The hooks include:

**On commit:**
- `rustfmt` - Rust code formatting
- `clippy` - Rust linting with `-D warnings`
- `cargo check` - Rust compilation check
- `black` - Python code formatting
- `pylint` - Python linting
- Trailing whitespace removal
- End-of-file fixer
- YAML/TOML/JSON validation
- Merge conflict detection

**On push:**
- `cargo test` - Full test suite

To run all hooks manually:

```bash
pre-commit run --all-files
```

To skip hooks temporarily (not recommended):

```bash
git commit --no-verify
```

## Code Style

### Rust

- Follow the existing code style enforced by `rustfmt`
- All public items require documentation comments (`///`)
- Use `Result<T, DisasmError>` for fallible operations
- Never use `unwrap()` in library code; use the `?` operator
- Prefer `&str` over `String` for function parameters
- Group imports: std, external crates, crate-local

Example:

```rust
use std::collections::HashMap;

use clap::Parser;

use crate::error::DisasmError;
use super::types::*;
```

### Python

- Follow PEP 8, enforced by `black` and `pylint`
- Maximum line length: 120 characters
- Use type hints where practical
- Configuration is in `pyproject.toml`

## Testing

### Running Tests

Run the full test suite:

```bash
make test
```

This command:
1. Clones Capstone if not present
2. Builds the Capstone comparison tool
3. Runs parity tests against Capstone
4. Runs Rust unit tests

### Quick Testing

For faster iteration during development:

```bash
# Quick parity test (20 cases)
make test-quick

# Rust unit tests only
cargo test --manifest-path robustone/Cargo.toml

# Parity tests only
make test-parity
```

### Adding Tests

When adding new instructions or features:

1. Add unit tests in the relevant Rust module
2. Add parity test cases in `test/` directory
3. Validate configurations: `make test-validate`

## Submitting Changes

1. Create a new branch from `main`:

```bash
git checkout -b feature/your-feature-name
```

2. Make your changes, ensuring:
   - All pre-commit hooks pass
   - All tests pass
   - Documentation is updated if needed

3. Commit your changes with a clear message:

```bash
git add .
git commit -m "feat: add support for XYZ instruction"
```

Commit message format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for test additions or changes
- `chore:` for maintenance tasks

4. Push and create a pull request:

```bash
git push origin feature/your-feature-name
```

## Pull Request Checklist

Before submitting your pull request, verify:

- [ ] Code compiles without warnings (`make build`)
- [ ] All lints pass (`make check`)
- [ ] All tests pass (`make test`)
- [ ] Pre-commit hooks pass (`pre-commit run --all-files`)
- [ ] New code includes appropriate tests
- [ ] Public APIs include documentation
- [ ] Commit messages follow the format above

## Adding a New Architecture

When implementing support for a new architecture:

1. Create a new module under `robustone-core/src/<arch>/`
2. Implement the `ArchitectureHandler` trait
3. Add a feature flag in `robustone-core/Cargo.toml`
4. Register the handler in `ArchitectureDispatcher::new()`
5. Add parity tests in `test/<arch>/`
6. Update documentation

## Questions

If you have questions about contributing, please open an issue on GitHub.
