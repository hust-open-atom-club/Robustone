# Robustone Coding Guidelines (Referencing Asterinas)

Robustone defines its own coding guidelines
with references to the Asterinas guidelines
for daily development and code review.

Source:
- Asterinas book: <https://asterinas.github.io/book/to-contribute/coding-guidelines/>
- Upstream repository path:
  <https://github.com/asterinas/asterinas/tree/main/book/src/to-contribute/coding-guidelines>
- Snapshot used by this project: `asterinas/asterinas@670ce782` (retrieved on March 6, 2026)

If exact wording or edge-case details are needed,
consult the corresponding Asterinas guideline pages directly.

## Scope and Priority

Priority order for style and review decisions:
1. This document (`docs/coding-guidelines.md`)
2. Tool-enforced rules (`rustfmt`, `clippy`, `black`, `pylint`, pre-commit hooks)
3. Existing local conventions in touched modules

Applicability levels used in this document:
- Required: must be satisfied in new or modified code.
- Recommended: should be followed unless there is a clear tradeoff.
- Contextual: apply only when the code pattern exists (for example `unsafe` code).

## General Rules (Language-Agnostic)

Required:
- `descriptive-names`: names must be descriptive at the point of use.
- `accurate-names`: names must reflect real behavior and side effects.
- `encode-units`: include units in names when types do not encode them (`*_bytes`, `*_ms`, `*_addr`).
- `bool-names`: use assertion-style booleans (`is_*`, `has_*`, `can_*`, `should_*`).
- `explain-why`: comments explain rationale, not a paraphrase of code.
- `design-decisions`: document non-obvious design choices and rejected alternatives.
- `one-concept-per-file`: split files when multiple major concepts are mixed.
- `top-down-reading`: public and high-level flow should appear before helpers.
- `logical-paragraphs`: separate function steps with blank lines and clear intent.
- `error-message-format`: keep messages specific and consistent.
- `hide-impl-details`: avoid leaking internals via public APIs or docs.

Recommended:
- `semantic-line-breaks`: prefer semantic line breaks in Markdown/doc prose.
- `cite-sources`: cite specs/manuals when behavior follows external references.
- `familiar-conventions`: use familiar Rust and Linux naming where possible.

Project mapping for `validate-at-boundaries`:
- Validate user input at the CLI and parsing boundaries (`robustone-cli`, hex parsing, config loaders).
- Keep internal decoder hot paths focused on core logic after boundary validation.

## Rust Rules

Required:
- `camel-case-acronyms`: follow Rust naming conventions for types, traits, and acronyms.
- `minimize-nesting`: return early to reduce nesting depth.
- `small-functions`: keep functions focused on one task.
- `no-bool-args`: avoid ambiguous boolean parameters; prefer enums/options/config structs.
- `rust-type-invariants`: encode invariants in types where possible.
- `propagate-errors`: use `?` for error propagation instead of manual branching boilerplate.
- `narrow-visibility`: default to the narrowest visibility (`pub(crate)`/private first).
- `narrow-lint-suppression`: suppress lints in the smallest possible scope.
- `debug-assert`: use `debug_assert!` only for correctness checks that are unnecessary in release.

Contextual:
- `justify-unsafe-use`: each `unsafe` block requires a `// SAFETY:` justification.
- `document-safety-conds`: unsafe APIs require a `# Safety` section in doc comments.
- `module-boundary-safety`: reason about safety at module boundaries, not only call sites.

Recommended:
- `explain-variables`: introduce well-named intermediates for complex expressions.
- `block-expressions`: use block expressions to scope temporary state.
- `checked-arithmetic`: prefer checked/saturating arithmetic where overflow is possible.
- `enum-over-dyn`: prefer enums over trait objects for closed sets.
- `getter-encapsulation`: prefer encapsulation over exposing mutable internals.
- `module-docs`: add module-level docs for major components.
- `macros-as-last-resort`: prefer functions/traits before introducing macros.
- `minimize-copies`: reduce avoidable copies and allocations on hot paths.
- `no-premature-optimization`: optimize based on profiling evidence.

## Testing Rules

Required:
- `add-regression-tests`: every bug fix should include a regression test when practical.
- `test-visible-behavior`: test public behavior and output, not private implementation details.
- `use-assertions`: use assertion helpers instead of manual print-and-check.
- `test-cleanup`: clean up files/processes/resources created during tests.

Project mapping:
- Keep parity tests behavior-oriented and aligned with user-visible disassembly output.
- For decoder regressions, add targeted Rust unit tests plus parity coverage when relevant.

## Git and Pull Request Rules

Required:
- `atomic-commits`: one logical change per commit.
- `refactor-then-feature`: separate prep refactors from functional changes.
- `focused-prs`: keep each PR focused on one topic.

Commit subject policy (combining project policy and referenced Asterinas guidance):
- Use existing Conventional Commit prefixes (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- Keep the subject imperative and descriptive.
- Target subject length at 72 characters or fewer.

## Practical Review Checklist

Use this checklist in self-review and PR review:

- [ ] Naming is descriptive and accurate (`descriptive-names`, `accurate-names`).
- [ ] Units and boolean names are explicit (`encode-units`, `bool-names`).
- [ ] Comments explain rationale, and non-obvious decisions are documented (`explain-why`, `design-decisions`).
- [ ] New/changed APIs avoid leaking implementation details (`hide-impl-details`).
- [ ] Functions are focused, with minimal nesting (`small-functions`, `minimize-nesting`).
- [ ] Error handling uses `Result`/`?` patterns (`propagate-errors`).
- [ ] Any `unsafe` usage includes full safety documentation (`justify-unsafe-use`, `document-safety-conds`).
- [ ] Bug fixes include regression tests (`add-regression-tests`).
- [ ] Tests assert observable behavior and clean up resources (`test-visible-behavior`, `test-cleanup`).
- [ ] Commits and PR scope are atomic and focused (`atomic-commits`, `focused-prs`).

## Incremental Adoption

This is a forward-looking standard.
Legacy code may not fully comply yet.
When touching existing code,
prefer small, safe cleanups that move it toward this guideline set.
