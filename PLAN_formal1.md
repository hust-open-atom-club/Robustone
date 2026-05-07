# 彻底移除 Capstone-generated Specs 并建立 Authoritative Spec 架构

## Goal Description

将 Robustone 仓库中所有由 Capstone YAML 衍生出的 `InstructionSpec` 彻底删除，并建立一套完全基于官方 ISA 手册的手写/宏声明式 authoritative spec 体系。Capstone 仅允许作为**测试输入源**与**兼容性 oracle**，禁止以任何形式（fallback、candidate、generator 输出）进入生产代码路径。

核心交付物：
1. 生产 crate 中不存在任何 `capstone_specs.rs`、`ALL_CAPSTONE_SPECS`、`G_0000` 类匿名 spec、auto-generated 标记；
2. `loongarch_lookup()` 仅查询单一 authoritative spec 表，无 fallback；
3. CI 增加硬 gate，只要检测到 Capstone-generated spec 痕迹即失败；
4. `make capstone-tests` 仅通过 production decoder + compat harness 运行，失败报告用于指引开发者在 authoritative spec 中补缺口，而非反向生成 spec。

---

## Acceptance Criteria

- **AC-1**: 生产路径中不存在任何 Capstone-generated spec 文件或符号
  - Positive Tests (expected to PASS):
    - `find robustone-loongarch/src -name "capstone_specs.rs"` 返回空
    - `grep -r "ALL_CAPSTONE_SPECS" robustone-loongarch/src/` 返回空
    - `grep -r "Auto-generated.*Capstone YAML" robustone-loongarch/src/` 返回空
    - `grep -r "G_0000\|G_0001" robustone-loongarch/src/` 返回空
    - `cargo build -p robustone-loongarch` 编译通过
  - Negative Tests (expected to FAIL):
    - 若保留 `capstone_specs.rs` 在 `backend/` 下，CI gate 应报错并阻断合并
    - 若 `backend.rs` 中仍存在 `pub use capstone_specs::ALL_CAPSTONE_SPECS;`，编译失败或 lint 失败

- **AC-2**: `loongarch_lookup()` 为单一权威查询，无 fallback
  - Positive Tests (expected to PASS):
    - `backend.rs` 中 `loongarch_lookup()` 仅遍历 `LOONGARCH_ALL_SPECS`（或等价单一聚合表）
    - 移除 `exact.is_some()` 与 `fallback_tables` 两段式逻辑
    - 对已知合法指令（如 `0x5400_f800` BL）仍可正确解码
  - Negative Tests (expected to FAIL):
    - 若 fallback 到 Capstone table 的代码路径仍存在，`cargo xtask audit-no-capstone-generated-specs` 返回非零
    - 若 `loongarch_lookup()` 接受多个 table 参数并按优先级查找，审计脚本应识别并失败

- **AC-3**: 删除或重构所有 Capstone spec generator
  - Positive Tests (expected to PASS):
    - 仓库根目录下不存在 `generate_capstone_specs.py`、`generate_loongarch_from_capstone.py`、`capstone_to_specs.rs` 等 generator
    - 现存脚本（如 `scripts/infer_formats.py`）若保留，必须确认其输出**不**写入 `robustone-loongarch/src/`，仅用于本地一次性分析或测试报告
    - `cargo xtask` 子命令列表中无 `gen-specs-from-capstone`
  - Negative Tests (expected to FAIL):
    - 若 `Makefile` 中存在 `generate-capstone-specs` target，`make` 执行时应报错或该 target 已被删除

- **AC-4**: CI 增加 `audit-no-capstone-generated-specs` 硬 gate
  - Positive Tests (expected to PASS):
    - `cargo xtask audit-no-capstone-generated-specs` 在干净仓库中返回 0
    - 该命令检查 `robustone-{loongarch,riscv,arm,x86}/src/**` 中的禁用关键词列表（`capstone_specs`、`ALL_CAPSTONE_SPECS`、`Auto-generated comprehensive`、`from Capstone YAML`、`G_0000`、`generated from Capstone`）
    - GitHub Actions workflow 已将该命令加入 `pull_request` 与 `push` 触发器
  - Negative Tests (expected to FAIL):
    - 在任意 architecture crate 的 `src/` 下新建包含 `ALL_CAPSTONE_SPECS` 的 `.rs` 文件，审计命令返回非零
    - 在 CI 中提交包含 `capstone_specs.rs` 的 PR，workflow 状态为失败

- **AC-5**: `make capstone-tests` 仅使用 production decoder
  - Positive Tests (expected to PASS):
    - `make capstone-tests ARCH=loongarch` 调用 `robustone-capstone-compat` 读取 YAML fixture，再调用 production decoder 解码
    - 成功 case 输出 `PASS`，失败 case 输出结构化报告（含 `bytes`、`expected`、`actual_error`、`category`），报告中**不含** `suggested_mask`、`suggested_value`、`suggested_format`、`suggested_operands`
    - 失败报告中的 `category` 字段枚举为：`MissingInstruction`、`OperandMismatch`、`RenderMismatch`、`DetailMismatch`、`FeatureMismatch`
  - Negative Tests (expected to FAIL):
    - 若 compat harness 在 decode 失败后尝试 fallback 到 Capstone spec 表再输出 `PASS`，测试框架应标记为违规并失败
    - 若失败报告中包含任何 spec 生成建议字段，报告生成器自身应断言失败

- **AC-6**: `specs_remaining.rs` 中 Capstone-generated 内容被清理
  - Positive Tests (expected to PASS):
    - `specs_remaining.rs` 不再包含 `Auto-generated comprehensive LoongArch instruction specs from Capstone YAML` 头部注释
    - `specs_remaining.rs` 不再导出 `ALL_CAPSTONE_SPECS`
    - 若 `specs_remaining.rs` 保留，其内容必须为手写/宏生成的 authoritative spec，且通过 `check-spec` 校验
  - Negative Tests (expected to FAIL):
    - 若 `specs_remaining.rs` 仍以 `G_0000` 形式平铺匿名 spec，`cargo xtask check-spec` 报错

---

## Path Boundaries

### Upper Bound (Maximum Scope)
- 完全删除 `capstone_specs.rs`、`specs_remaining.rs` 中的 `ALL_CAPSTONE_SPECS` 及其所有 `G_xxxx` 匿名 spec；
- 重写 `loongarch_lookup()` 为纯 authoritative 查询；
- 删除所有 Capstone spec generator 脚本；
- 新增 CI 硬 gate：`audit-no-capstone-generated-specs`；
- 重构 `make capstone-tests` 为纯 production decoder + structured fail report；
- 将现有 `LOONGARCH_BASE_SPECS` 扩展为 `LOONGARCH_ALL_SPECS`，覆盖 branch、memory、atomic、system、privileged、float、vector 等全部类别，使用 family macro 编写；
- `make capstone-tests` 最终目标为 `fail = 0, xfail = 0, unsupported = 0`。

### Lower Bound (Minimum Scope)
- 删除 `capstone_specs.rs` 文件；
- 从 `backend.rs` 移除 `mod capstone_specs;` 与 `pub use capstone_specs::ALL_CAPSTONE_SPECS;`；
- 从 `loongarch_lookup()` 移除 `ALL_CAPSTONE_SPECS` fallback 路径；
- 从 `specs_remaining.rs` 移除 `ALL_CAPSTONE_SPECS` 及 auto-generated 头部；
- 新增至少一个脚本或 cargo xtask 命令，能够在 CI 中检测 `capstone_specs` / `ALL_CAPSTONE_SPECS` / `G_0000` 等关键词并失败；
- `cargo build -p robustone-loongarch` 与现有 native tests 继续通过。

### Allowed Choices
- **Can use**:
  - Rust macro（如 `define_instructions!`、`loongarch_r3_family!`）来减少重复 spec 声明；
  - `robustone-capstone-compat` crate 作为测试 harness；
  - Capstone YAML 作为只读测试 fixture；
  - 结构化 JSON/text 失败报告作为缺口分析输入；
  - 官方 LoongArch 手册（Vol.1 / Vol.2）作为 spec 权威来源。
- **Cannot use**:
  - 任何将 Capstone YAML / Capstone disassembly 自动转换为 `InstructionSpec`、`mask_value`、`format`、`operands` 的脚本或程序；
  - `candidate_specs/`、`generated-candidates/` 等中间目录保存 Capstone 输出；
  - `ALL_CAPSTONE_SPECS` 作为 fallback、rescue path、test-only feature；
  - 以 exact-word spec（`mask_value!(0xFFFFFFFF, ...)`）平铺数千条指令的形式作为生产代码。

> **Note on Deterministic Designs**: 本计划的核心约束为绝对禁止 Capstone-generated specs。Upper Bound 与 Lower Bound 在此约束上收敛为同一点：无论实现范围大小，`capstone_specs.rs`、`ALL_CAPSTONE_SPECS`、`G_xxxx` 匿名 spec 都必须完全消失。

---

## Feasibility Hints and Suggestions

### Conceptual Approach
1. **清理阶段**：先删文件、再删引用、再删 fallback，确保编译通过；
2. **防护阶段**：新增 audit 脚本，将清理结果固化为 CI 规则；
3. **填补阶段**：运行 `make capstone-tests`，收集失败报告，按类别（missing instruction / operand mismatch / render mismatch）在 authoritative spec 中逐条补充；
4. **重构阶段**：将平铺的 `LOONGARCH_BASE_SPECS` 按指令族（integer、branch、memory、float、vector…）拆分为 `specs/` 子模块，使用 family macro 提高可维护性。

### Relevant References
- `robustone-loongarch/src/backend.rs` — `loongarch_lookup()` 与 `LOONGARCH_BASE_SPECS` 定义位置
- `robustone-loongarch/src/backend/capstone_specs.rs` — 待删除的 auto-generated spec 文件（~834KB）
- `robustone-loongarch/src/backend/specs_remaining.rs` — 包含另一份 `ALL_CAPSTONE_SPECS` 的 auto-generated 文件（~780KB）
- `robustone-loongarch/src/backend/specs_integer.rs` — 现有手写 spec 的参考范例
- `scripts/infer_formats.py`、`scripts/infer_formats2.py` — 需审计是否属于 Capstone spec generator
- `robustone-capstone-compat/src/` — 允许的 Capstone 兼容层，仅用于测试

---

## Dependencies and Sequence

### Milestones

1. **Milestone 1：完全删除 Capstone-generated specs**
   - Phase A：删除 `capstone_specs.rs` 文件本体
   - Phase B：从 `backend.rs` 移除 `mod capstone_specs;`、`pub use capstone_specs::ALL_CAPSTONE_SPECS;`、fallback 逻辑
   - Phase C：清理 `specs_remaining.rs` 中的 `ALL_CAPSTONE_SPECS` 与 auto-generated 头部
   - Phase D：审计并删除/重构 `scripts/` 下的 Capstone spec generator
   - Phase E：验证 `cargo build -p robustone-loongarch` 与 native tests 通过

2. **Milestone 2：新增 CI 硬 gate**
   - Phase A：实现 `cargo xtask audit-no-capstone-generated-specs`
   - Phase B：在 `.github/workflows/` 中新增/更新 workflow，将 audit 命令加入 CI
   - Phase C：验证 gate 能有效拦截含 `capstone_specs.rs` 的提交

3. **Milestone 3：重构 `make capstone-tests` 为纯 production decoder**
   - Phase A：修改 compat harness，移除任何 fallback 到 Capstone spec 的路径
   - Phase B：确保失败报告仅输出缺口信息，不输出 spec 生成建议
   - Phase C：运行测试，确认失败报告中的 `category` 与 `reason` 字段准确

4. **Milestone 4：修复 LoongArch 基础指令覆盖**
   - Phase A：Branch（b、bl、jirl、beq、bne、blt、bge、bltu、bgeu、beqz、bnez、bceqz、bcnez）
   - Phase B：Memory（ld.*、st.*、ldx.*、stx.*）
   - Phase C：Base Integer（add/sub/and/or/xor/nor/shift/mul/div/mod/bit）
   - Phase D：每类补充后运行 native test + capstone compat test

5. **Milestone 5：修复 system / privileged / float / vector / LBT**
   - Phase A：System（syscall、break、cpucfg、rdtime）
   - Phase B：Privileged（csr、tlb、iocsr、invtlb、ertn、idle）
   - Phase C：Float（fadd、fsub、fmul、fdiv、fcmp、fcvt、fld/fst）
   - Phase D：Vector / LBT（LSX、LASX、LVZ、LBT_ARM、LBT_X86、LBT_MIPS）
   - Phase E：全量 `make capstone-tests ARCH=loongarch` 达成 `fail = 0`

6. **Milestone 6：Spec 目录结构重构（可选，建议在 Milestone 5 后执行）**
   - Phase A：将 `LOONGARCH_BASE_SPECS` 拆分为 `specs/{branch,memory,atomic,system,privileged,float_arith,float_cmp,float_mem,float_convert,lsx,lasx,lvz,lbt_arm,lbt_x86,lbt_mips}.rs`
   - Phase B：引入 family macro 统一同类指令声明风格
   - Phase C：聚合为 `LOONGARCH_ALL_SPECS`，验证无 overlap

---

## Task Breakdown

| Task ID | Description | Target AC | Tag | Depends On |
|---------|-------------|-----------|-----|------------|
| task1 | 删除 `robustone-loongarch/src/backend/capstone_specs.rs` | AC-1 | coding | — |
| task2 | 从 `backend.rs` 移除 `mod capstone_specs;` 与 `pub use capstone_specs::ALL_CAPSTONE_SPECS;` | AC-1 | coding | task1 |
| task3 | 清理 `specs_remaining.rs` 中的 `ALL_CAPSTONE_SPECS` 与 auto-generated 头部 | AC-6 | coding | — |
| task4 | 重写 `loongarch_lookup()` 为单一 authoritative 查询，移除 fallback | AC-2 | coding | task2, task3 |
| task5 | 审计并处理 `scripts/infer_formats.py`、`infer_formats2.py`、`analyze_loongarch_yaml.py` | AC-3 | analyze | — |
| task6 | 实现 `cargo xtask audit-no-capstone-generated-specs` | AC-4 | coding | task1, task2, task3 |
| task7 | 将 audit 命令加入 GitHub Actions CI workflow | AC-4 | coding | task6 |
| task8 | 修改 compat harness，确保失败报告不含 spec 生成建议 | AC-5 | coding | task4 |
| task9 | 补充 Branch 类 authoritative spec 并通过 capstone tests | AC-5 | coding | task4 |
| task10 | 补充 Memory 类 authoritative spec 并通过 capstone tests | AC-5 | coding | task9 |
| task11 | 补充 Base Integer 类 authoritative spec 并通过 capstone tests | AC-5 | coding | task10 |
| task12 | 补充 System / Privileged 类 authoritative spec | AC-5 | coding | task11 |
| task13 | 补充 Float 类 authoritative spec | AC-5 | coding | task12 |
| task14 | 补充 Vector / LBT 类 authoritative spec | AC-5 | coding | task13 |
| task15 | 全量 capstone-tests 验证 `fail = 0` | AC-5 | coding | task14 |

---

## Claude-Codex Deliberation

### Agreements
- 必须彻底删除 `capstone_specs.rs` 与 `ALL_CAPSTONE_SPECS`，不存在过渡或 fallback 方案。
- Capstone 的角色严格限定为测试输入与兼容性 oracle。
- `loongarch_lookup()` 必须为单一 authoritative 表查询。

### Resolved Disagreements
- **是否保留 `specs_remaining.rs` 作为临时容器**：Claude 建议完全删除；Codex 认为若其中包含部分尚未迁移到手写 spec 的指令，可先移除 `ALL_CAPSTONE_SPECS` 与 auto-generated 标记，将剩余内容重构为手写 macro。最终决议：以 AC-6 为准，`specs_remaining.rs` 不允许保留 `G_xxxx` 匿名 spec 或 `ALL_CAPSTONE_SPECS`，若保留文件则内容须为 authoritative。
- **`scripts/` 下的 Python 脚本是否全部删除**：Claude 建议全部删除；Codex 认为 `analyze_loongarch_yaml.py` 若仅用于一次性本地分析可保留，但须确认其输出不进入 `src/`。最终决议：以 AC-3 为准，generator 脚本须删除或重构为仅输出测试报告，由 task5 审计后决定。

### Convergence Status
- Final Status: `converged`

---

## Pending User Decisions

- **DEC-1**: `specs_remaining.rs` 的处理策略
  - Claude Position: 直接删除，将其中的有效指令逐步迁移到新的 `specs/` 子模块中手写补充。
  - Codex Position: 若文件体积过大（~780KB），手动重写成本高，建议先批量删除 `G_xxxx` 与 `ALL_CAPSTONE_SPECS`，保留部分已人工校验过的指令定义，再逐步迁移。
  - Tradeoff Summary: 完全删除更干净，但可能导致短期内 `make capstone-tests` 失败数激增；保留重构则需要额外校验哪些定义是 authoritative。
  - **User Decision**: 先清理再迁移。删除 `G_xxxx` 匿名 spec 与 `ALL_CAPSTONE_SPECS`，保留已人工校验的指令定义，逐步迁移至 authoritative spec 模块。

- **DEC-2**: 是否将 `capstone.rs` compat adapter 从 `robustone-loongarch` 迁移到 `robustone-capstone-compat`
  - Claude Position: 更严格的做法是将所有 Capstone 相关代码移出 architecture crate，architecture crate 完全不出现 `Capstone` 字样。
  - Codex Position: 若现有 `capstone.rs` 已在 `robustone-capstone-compat` 中，则无需迁移；若仍在 `robustone-loongarch/src/` 下，建议迁移。
  - Tradeoff Summary: 迁移可增强边界清晰性，但可能引入额外耦合改动。
  - **User Decision**: 将 `capstone.rs` compat adapter 从 `robustone-loongarch` 完全移出至 `robustone-capstone-compat`。`robustone-loongarch/src/` 下不再保留任何 Capstone 相关文件。

- **DEC-3**: family macro 的引入时机
  - Claude Position: 在 Milestone 1-3 后立即引入，以便后续补指令时保持优雅。
  - Codex Position: 先以现有平铺方式快速补全基础指令（Milestone 4-5），再统一重构为 family macro（Milestone 6），避免前期 macro 设计不成熟导致返工。
  - Tradeoff Summary: 先 macro 则后期补指令效率高，但 macro 设计错误代价大；后 macro 则前期代码冗余，但风险低。
  - **User Decision**: 等基础指令补全（Milestone 4-5 完成）后再统一重构为 family macro（Milestone 6）。

---

## Implementation Notes

### Code Style Requirements
- 实现代码与注释中**不得**出现计划术语，如 `AC-`、`Milestone`、`Step`、`Phase`、`task1` 等。
- 这些术语仅用于计划文档，不进入代码库。
- 代码中应使用领域相关的命名，如 `loongarch_lookup`、`LOONGARCH_ALL_SPECS`、`audit_no_capstone_specs` 等。

### 关键文件变更清单
- **删除**：`robustone-loongarch/src/backend/capstone_specs.rs`
- **修改**：`robustone-loongarch/src/backend.rs` — 移除 `mod capstone_specs;`、`pub use capstone_specs::ALL_CAPSTONE_SPECS;`、重写 `loongarch_lookup()`
- **修改**：`robustone-loongarch/src/backend/specs_remaining.rs` — 移除 auto-generated 头部、`ALL_CAPSTONE_SPECS` 导出、`G_xxxx` 匿名 spec
- **新增**：`xtask/src/audit_no_capstone.rs`（或等效实现）— CI 硬 gate
- **修改**：`.github/workflows/*.yml` — 加入 audit 步骤
- **修改**：`robustone-capstone-compat/src/` — 确保 compat harness 不 fallback 到 Capstone spec

### 风险与缓解
- **风险**：删除 `capstone_specs.rs` 后，`make capstone-tests` 可能产生大量失败，开发者体验下降。
  - **缓解**：失败报告必须结构化且可读，按 mnemonic / category 聚合，优先暴露高频基础指令（branch、memory、integer）缺口。
- **风险**：`specs_remaining.rs` 中可能混有已人工校验的指令定义，一刀切删除导致重复劳动。
  - **缓解**：由 DEC-1 决定；若选择保留重构，须先运行脚本扫描其中非 `G_xxxx` 命名的符号，人工确认后再迁移。
- **风险**：family macro 设计不当会限制后续指令扩展。
  - **缓解**：由 DEC-3 决定；若先平铺后重构，可在重构前充分收集不同指令族的共性需求。
