# Robustone 编码规范（参考 Asterinas）

Robustone 维护自己的编码规范，
并参考 Asterinas 的 coding guidelines
用于日常开发和 code review。

来源：
- Asterinas 文档：<https://asterinas.github.io/book/to-contribute/coding-guidelines/>
- 上游仓库目录：
  <https://github.com/asterinas/asterinas/tree/main/book/src/to-contribute/coding-guidelines>
- 本项目采用的快照：`asterinas/asterinas@670ce782`（获取日期：2026-03-06）

如需查看规则原文、边界条件或细节示例，
请直接查阅 Asterinas 对应文档页面。

## 适用范围与优先级

风格与评审决策的优先级：
1. 本文档（`docs/coding-guidelines-cn.md`）
2. 工具强制规则（`rustfmt`、`clippy`、`black`、`pylint`、pre-commit）
3. 被修改模块的既有本地约定

本文档中的规则级别：
- 必须：新增或修改代码时必须满足。
- 建议：原则上应满足，除非存在明确权衡。
- 条件适用：仅在对应代码形态出现时适用（例如 `unsafe`）。

## 通用规则（语言无关）

必须：
- `descriptive-names`：命名在使用点应当自解释。
- `accurate-names`：命名必须准确反映行为和副作用。
- `encode-units`：类型无法表达单位时，在命名中编码单位（如 `*_bytes`、`*_ms`）。
- `bool-names`：布尔命名采用断言式（`is_*`、`has_*`、`can_*`、`should_*`）。
- `explain-why`：注释解释“为什么”，而非复述“做了什么”。
- `design-decisions`：记录非显然设计决策与备选方案。
- `one-concept-per-file`：单文件只承载一个主要概念，过大及时拆分。
- `top-down-reading`：代码应自顶向下可读，先高层后细节。
- `logical-paragraphs`：函数内部按逻辑步骤分段组织。
- `error-message-format`：错误信息具体且风格统一。
- `hide-impl-details`：公开 API 与文档不暴露实现细节。

建议：
- `semantic-line-breaks`：Markdown 或文档注释优先语义换行。
- `cite-sources`：实现外部规范或算法时引用来源。
- `familiar-conventions`：优先沿用 Rust/Linux 的熟悉约定。

`validate-at-boundaries` 在 Robustone 的落地：
- 在 CLI 与解析边界验证用户输入（如 `robustone-cli`、十六进制解析、配置读取）。
- 进入核心解码路径后尽量避免重复校验，保持逻辑清晰和性能稳定。

## Rust 规则

必须：
- `camel-case-acronyms`：类型、trait、缩略词遵循 Rust 命名习惯。
- `minimize-nesting`：优先早返回，减少嵌套层级。
- `small-functions`：函数职责单一、聚焦。
- `no-bool-args`：避免语义不清的布尔参数，优先枚举或配置结构体。
- `rust-type-invariants`：尽可能用类型系统表达不变量。
- `propagate-errors`：可失败路径优先使用 `?` 传播错误。
- `narrow-visibility`：默认最小可见性（优先私有或 `pub(crate)`）。
- `narrow-lint-suppression`：lint 抑制范围最小化。
- `debug-assert`：`debug_assert!` 仅用于发布版可省略的正确性检查。

条件适用：
- `justify-unsafe-use`：每个 `unsafe` 必须写 `// SAFETY:` 说明。
- `document-safety-conds`：`unsafe fn` 或不安全约束需在文档写 `# Safety`。
- `module-boundary-safety`：在模块边界层面论证安全性，而不只在单点调用处说明。

建议：
- `explain-variables`：复杂表达式拆为语义明确的中间变量。
- `block-expressions`：使用块表达式约束临时状态生命周期。
- `checked-arithmetic`：存在溢出风险时优先 checked/saturating 算术。
- `enum-over-dyn`：封闭集合优先 `enum` 而非 trait object。
- `getter-encapsulation`：优先封装，避免泄露可变内部状态。
- `module-docs`：主要模块提供模块级文档。
- `macros-as-last-resort`：宏作为最后手段，优先函数与 trait。
- `minimize-copies`：热点路径避免不必要的拷贝与分配。
- `no-premature-optimization`：优化前先有证据（profile/benchmark）。

## 测试规则

必须：
- `add-regression-tests`：修复 bug 时尽量补回归测试。
- `test-visible-behavior`：测试用户可观察行为，避免耦合内部实现。
- `use-assertions`：使用断言宏，不做手工打印比对。
- `test-cleanup`：测试结束后清理创建的资源与子进程。

项目落地：
- 一致性测试以用户可见反汇编输出为准。
- 解码器修复建议同时补 Rust 单测与对应一致性用例（如适用）。

## Git 与 Pull Request 规则

必须：
- `atomic-commits`：一个 commit 一个逻辑变化。
- `refactor-then-feature`：先重构再功能，分开提交。
- `focused-prs`：一个 PR 聚焦一个主题。

提交信息策略（兼容本项目规范与参考的 Asterinas 规则）：
- 保留本项目 Conventional Commit 前缀：`feat:`、`fix:`、`docs:`、`refactor:`、`test:`、`chore:`。
- 前缀后的主题句使用祈使语气，描述清晰。
- 主题行尽量不超过 72 字符。

## 评审检查清单

- [ ] 命名清晰且准确（`descriptive-names`, `accurate-names`）。
- [ ] 单位和布尔语义明确（`encode-units`, `bool-names`）。
- [ ] 注释说明动机，关键设计有记录（`explain-why`, `design-decisions`）。
- [ ] 公共接口不泄露实现细节（`hide-impl-details`）。
- [ ] 函数职责聚焦、嵌套受控（`small-functions`, `minimize-nesting`）。
- [ ] 错误处理采用 `Result`/`?` 模式（`propagate-errors`）。
- [ ] 若使用 `unsafe`，安全说明完整（`justify-unsafe-use`, `document-safety-conds`）。
- [ ] bug 修复包含回归测试（`add-regression-tests`）。
- [ ] 测试验证可观察行为并完成资源清理（`test-visible-behavior`, `test-cleanup`）。
- [ ] commit 与 PR 规模原子、主题聚焦（`atomic-commits`, `focused-prs`）。

## 渐进式采用

本规范面向“新增与变更代码”。
历史代码可能尚未完全满足。
当你修改旧代码时，
优先做小步、低风险、可验证的清理改进，
逐步向该规范靠拢。
