# Capstone v6.0.0 指令集支持列表

我们归纳本支持列表，为了更好地规定 Robustone 项目的路线图。

本文件中，指令集名称参考 RISC-V RVA23 标准（[链接](https://github.com/riscv/riscv-profiles/releases/tag/rva23-rvb23-ratified)）。

测试方法而言，我们用命令行运行 Capstone 的 `cstool` 来完成目的。

例如，为了测试 `f3 22 30 b0` 字节码是否能被 Capstone 支持，运行以下指令。

```bash
cstool riscv64 f32230b0
```

测试得到正确反汇编结果，说明 Capstone 支持此指令，否则说明不支持。

```bash
PS C:\Users\luojia> cstool riscv64 f32230b0
 0  f3 22 30 b0  csrr   t0, mhpmcounter3
```

## 一、RVA23U64 profile

### 1. RVA23U64 基础指令集

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| RV64I | 是 | 9B024306 | addiw  t0, t1, 0x64 | [@manchangfengxu] |  Base Integer Instruction. |

### 2. RVA22U64 基础扩展指令集

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| M | 是 | b3027302 | mul t0, t1, t2 | [@luojia65] | Integer multiplication and division. |
| A | 是 | af220510 | lr.w t0, (a0) | [@manchangfengxu] | Atomic Instructions. |
| F | 是 | 53802010 | fmul.s ft0, ft1, ft2, rne | [@manchangfengxu] | Single-precision floating-point instructions. |
| D | 是 | 53802002 | fadd.d ft0, ft1, ft2, rne | [@manchangfengxu] | Double-Precision Floating-Point. |
| C | 是 | 0245 | c.lwsp a0, 0(sp) | [@manchangfengxu] | Compressed Instructions. |
| B | 否 | B3727340 | andn t0, t1, t2 | [@manchangfengxu] | Bit Manipulation. |
| Zicsr | 是 | F32200C0 | rdcycle t0 | [@manchangfengxu] | Control and Status Register Access. |
| Zicntr | 是 | F32210C0 | rdtime t0 | [@manchangfengxu] | Performance Counters. |
| Zihpm | 是 | f32230b0 | csrr t0, mhpmcounter3 | [@luojia65] | Hardware performance counters. |
| Ziccif | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Ziccrse | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Ziccamoa | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Zicclsm | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Za64rs | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Zihintpause | 否 | 17000000  | hint.pause | [@206432729] | inconsistent with the expected instruction |
| Zic64b | N/A | N/A | 本扩展不涉及指令 | [@manchangfengxu] |  |
| Zicbom | 否 | 0F200500 | CBO.INVAL | [@manchangfengxu] | Cache-block management instructions. |
| Zicbop | 是 | 13600500 | ori zero, a0, 0 /prefetch.r 0(a0) | [@manchangfengxu] | Cache-block prefetch instructions. |
| Zicboz | 否 | 0F204500 | cbo.zero (a0) | [@manchangfengxu] | Cache-Block Zero Instructions. |
| Zfhmin | 否 | 07900200 | FLH rd, off(rs1) | [@manchangfengxu] | Half-precision floating-point. |
| Zkt | N/A | N/A | 本扩展不涉及指令 | [@206432729] |  |

### 3. RVA23U64 新指令集

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| V | 否 | D7101102 | vadd.vv v1, v2, v3 | [@manchangfengxu] | Vector extension. |
| Zvfhmin | 否 | 5000070B | vfmins.h v1, v2, v3 | [@206432729] | Vector half-precision min extension. |
| Zvbb | 否 | 00000723 | vandn.vv v1, v2, v3 | [@206432729] | Vector basic bitwise extension |
| Zvkt | N/A | N/A | 本扩展不涉及指令 | [@206432729] |  |
| Zihintntl | 否 | 00200013 | NTL.P1（ADD x0, x0, x2） | [@206432729] | Non-temporal locality hint extension. |
| Zicond | 否 | 00311063 | czero.eqz x0, x1, x2 | [@206432729] | Conditional zero extension. |
| Zimop | 否 | F3427387 | MOP.RR.3 t0, t1, t2 | [@manchangfengxu] | may-be-operations. |
| Zcmop | 否 | 8160 | C.MOP.1 | [@manchangfengxu] | Compressed may-be-operations. |
| Zcb | 否 | 0880 | c.lbu a0, 0(s0) | [@manchangfengxu] | Additional compressed instructions. |
| Zfa | 否 | 53A02028 | fminm.s ft0, ft1, ft2 | [@manchangfengxu] | Additional floating-Point instructions. |
| Zawrs | 否 | 0021842F | amorsw.w x0, x2, (x1) | [@206432729] | Atomic write-read-modify extension. |
| Supm | 是 | 73220010; 73620010 | csrr tp, sstatus; csrrsi tp, sstatus, 0 | [@206432729] | Depends on senvcfg CSR ，tested with sstatus CSR bytecode, need to replace with senvcfg-specific bytecode |

## 二、RVA23S64 profile

RVA23S64 包含 RVA23U64 的所有指令集，以下增加 RVA23S64 特有的指令集。

### 1. RVA23S64 基础指令集

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| ecall 指令 | 是 | 73000000 | ecall | [@manchangfengxu] | Environment call to execution environment. |

### 2. RVA23S64 基础扩展指令集

除了 RVA23U64 的所有指令集之外，增加以下指令集。

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| Ss1p13 | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |

以下指令集与 RVA22S64 相同。

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| Svbare | N/A | N/A | 本扩展不涉及指令 | [@luojia65] |  |
| Sv39 | 是 | 73220018 | csrr t0, satp | [@206432729] | 39-bit Virtual Memory. |
| Svade | 否 | 80000002 | lw t0, 0(t1) | [@206432729] | A/D Bit Exception. |
| Ssccptr | 否 | 00010103 | lw t0, 0(t2) | [@206432729] | HW Page-Table Read. |
| Sstvecd | 是 | 73220005 | csrr t0, stvec | [@206432729] | stvec Direct Mode. |
| Sstvala | 是 | 73220003 | csrr t0, stval | [@206432729] | stval Fault Info. |
| Sscounterenw | 是 | 73220070 | csrr t0, scounteren | [@206432729] | Writable Counter Enable. |
| Svpbmt | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Svinval | 否 | 7300B516 | sinval.vma rs1, rs2 | [@manchangfengxu] | Fine-grained address-translation cache invalidation. |
| Svnapot | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sstc | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sscofpmf | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Ssnpm | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Ssu64xl | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sha | 否 | 7300B522 | hfence.vvma rs1, rs2 | [@manchangfengxu] | The augmented hypervisor extension. |

### 3.RVA23S64 可选指令集

| 指令集 | Capstone 支持 | 测试字节码 | 测试指令（汇编） | 测试人 | 说明 |
|:------|:-----------|:--------------|:------------|:-----------|:--------|
| Sv48 | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sv57 | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Zkr | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Svadu | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sdtrig | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Ssstrict | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Svvptc | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |
| Sspm | N/A | N/A | 本拓展不涉及指令 | [@manchangfengxu] |  |

注：“附录 Glossary of ISA Extensions”因指令集重复，本文件不再赘述支持情况。

[@luojia65]: https://github.com/luojia65
[@manchangfengxu]: https://github.com/manchangfengxu
[@206432729]: https://github.com/206432729
