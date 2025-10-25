//! RISC-V反汇编测试生成器
//!
//! 用法: cargo run -p transfer --bin test_generator
//!
//! 这个工具会：
//! 1. 运行cstool获取参考结果
//! 2. 运行robustone获取测试结果
//! 3. 对比结果并生成测试用例

use std::collections::HashMap;
use std::process::Command;
use std::fs;

// 精选的RISC-V测试指令 - 覆盖主要指令类型
const TEST_INSTRUCTIONS: &[&str] = &[
    // 基础addi指令 - 测试不同寄存器
    "00000093", // addi ra, zero, 0
    "00000193", // addi gp, zero, 0
    "00000293", // addi t0, zero, 0
    "00000313", // addi t1, zero, 0
    "00000413", // addi s0, zero, 0
    "00000513", // addi a0, zero, 0
    "00000613", // addi a1, zero, 0
    "00000713", // addi a2, zero, 0

    // 不同立即数值
    "00100513", // addi a0, zero, 1
    "00a00513", // addi a0, zero, 10
    "0ff00513", // addi a0, zero, 255
    "fff00513", // addi a0, zero, -1
    "80000513", // addi a0, zero, -2048
    "ff800513", // addi a0, zero, 2047

    // LUI和AUIPC指令
    "00000537", // lui a0, 0x0
    "00100537", // lui a0, 0x10000
    "fff00537", // lui a0, 0xfffff000
    "00000097", // auipc ra, 0x0
    "00000517", // auipc a0, 0x0

    // 分支指令
    "00004063", // beq zero, zero, 0x0
    "00004163", // bne zero, zero, 0x0
    "fe0504e3", // blt zero, a0, -32
    "fe0554e3", // bge zero, a0, -32
    "0000006f", // jal ra, 0x0
    "00008067", // jalr zero, zero, 0x0

    // 加载指令
    "00050503", // lb a0, 0(zero)
    "00051503", // lh a0, 0(zero)
    "00052503", // lw a0, 0(zero)
    "00054503", // lbu a0, 0(zero)
    "00055503", // lhu a0, 0(zero)

    // 存储指令
    "00050023", // sb a0, 0(zero)
    "00051023", // sh a0, 0(zero)
    "00052023", // sw a0, 0(zero)

    // 算术指令
    "00005033", // add zero, zero, zero
    "40005033", // sub zero, zero, zero
    "00005133", // sll zero, zero, zero
    "00005333", // slt zero, zero, zero
    "000054b3", // xor zero, zero, zero
    "00005533", // srl zero, zero, zero
    "40005533", // sra zero, zero, zero
    "000057b3", // or zero, zero, zero
    "000077b3", // and zero, zero, zero

    // 移位指令
    "00001513", // slli a0, zero, 0
    "00401513", // slli a0, zero, 4
    "000055b3", // srli a1, zero, 0
    "004055b3", // srli a1, zero, 4
    "400055b3", // srai a1, zero, 32

    // 系统指令
    "00000073", // ecall
    "00100073", // ebreak

    // 压缩指令
    "4101",   // c.addi4spn s0, sp, 4
    "4102",   // c.lw s0, 0(a0)
    "c102",   // c.sw s0, 0(a0)
    "4501",   // c.addi s0, 0
    "4502",   // c.li s0, 0
    "8481",   // c.srli s0, 0
    "8c41",   // c.sub s0, s1
    "a001",   // c.j 0x8
    "c021",   // c.beqz s0, 0x0
    "0501",   // c.slli ra, 0
];

#[derive(Debug, Clone)]
struct TestResult {
    hex: String,
    cstool_mnemonic: String,
    cstool_operands: String,
    robustone_mnemonic: String,
    robustone_operands: String,
    size: usize,
    match_mnemonic: bool,
    match_operands: bool,
}

impl TestResult {
    fn is_complete_match(&self) -> bool {
        self.match_mnemonic && self.match_operands
    }
}

fn run_cstool(hex: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let output = Command::new("/home/plucky/capstone/cstool/cstool")
        .args(&["riscv64", hex])
        .output()?;

    if !output.status.success() {
        return Err(format!("cstool failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = stdout.trim().lines().collect();

    if lines.is_empty() {
        return Err("Empty output".into());
    }

    // cstool输出格式类似: "0  13 00 05 13  addi	zero, a0, 0x130"
    // 查找包含指令的行（包含制表符的行）
    let instruction_line = lines.iter()
        .find(|line| line.contains('\t') && !line.trim().is_empty())
        .ok_or("No instruction line found")?;

    // 使用制表符分割获取助记符和操作数
    let parts: Vec<&str> = instruction_line.split('\t').collect();
    if parts.len() < 2 {
        return Err("Invalid cstool output format".into());
    }

    let mnemonic = parts[0].trim().to_string();
    let operands = if parts.len() > 1 {
        parts[1].trim().to_string()
    } else {
        String::new()
    };

    Ok((mnemonic, operands))
}

fn run_robustone(hex: &str) -> Result<(String, String, usize), Box<dyn std::error::Error>> {
    use transfer::*;

    let dispatcher = ArchitectureDispatcher::new();
    let instruction = dispatcher.disassemble(hex, "riscv32".to_string());

    Ok((instruction.mnemonic, instruction.operands, instruction.size))
}

fn generate_test_results() -> Vec<TestResult> {
    let mut results = Vec::new();

    println!("正在对比cstool和robustone结果...\n");

    for &hex in TEST_INSTRUCTIONS {
        print!("测试: {} -> ", hex);

        // 获取cstool结果
        let (cstool_mnemonic, cstool_operands) = match run_cstool(hex) {
            Ok(result) => result,
            Err(e) => {
                println!("cstool错误: {}", e);
                continue;
            }
        };

        // 获取robustone结果
        let (robustone_mnemonic, robustone_operands, size) = match run_robustone(hex) {
            Ok(result) => result,
            Err(e) => {
                println!("robustone错误: {}", e);
                continue;
            }
        };

        let match_mnemonic = cstool_mnemonic == robustone_mnemonic;
        let match_operands = cstool_operands == robustone_operands;

        if match_mnemonic && match_operands {
            println!("✓");
        } else {
            println!("✗");
            println!("  cstool:   {} {}", cstool_mnemonic, cstool_operands);
            println!("  robustone: {} {}", robustone_mnemonic, robustone_operands);
        }

        results.push(TestResult {
            hex: hex.to_string(),
            cstool_mnemonic,
            cstool_operands,
            robustone_mnemonic,
            robustone_operands,
            size,
            match_mnemonic,
            match_operands,
        });
    }

    results
}

fn generate_test_file(results: &[TestResult]) -> Result<(), Box<dyn std::error::Error>> {
    let mut code = String::new();

    // 文件头
    code.push_str("//! 自动生成的RISC-V反汇编测试用例\n");
    code.push_str("//! 基于cstool参考实现生成\n\n");
    code.push_str("use transfer::*;\n\n");

    // 生成所有测试用例
    code.push_str("#[test]\n");
    code.push_str("fn test_riscv_vs_cstool_reference() {\n");
    code.push_str("    let dispatcher = ArchitectureDispatcher::new();\n\n");

    for result in results {
        if result.is_complete_match() {
            code.push_str(&format!(
                "    // {} - cstool参考: {} {}\n",
                result.hex, result.cstool_mnemonic, result.cstool_operands
            ));
            code.push_str(&format!(
                "    {{\n        let instruction = dispatcher.disassemble(\"{}\", \"riscv32\".to_string());\n",
                result.hex
            ));
            code.push_str(&format!(
                "        assert_eq!(instruction.mnemonic, \"{}\");\n",
                result.cstool_mnemonic
            ));
            code.push_str(&format!(
                "        assert_eq!(instruction.operands, \"{}\");\n",
                result.cstool_operands
            ));
            code.push_str(&format!(
                "        assert_eq!(instruction.size, {});\n",
                result.size
            ));
            code.push_str("    }\n\n");
        }
    }

    code.push_str("}\n");

    // 写入文件
    fs::write("tests/riscv_vs_cstool.rs", code)?;
    println!("\n已生成测试文件: tests/riscv_vs_cstool.rs");

    Ok(())
}

fn generate_mismatch_report(results: &[TestResult]) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();

    report.push_str("# RISC-V反汇编器对比报告\n\n");
    report.push_str("本报告对比了cstool和robustone的解码结果。\n\n");

    let total = results.len();
    let matched = results.iter().filter(|r| r.is_complete_match()).count();
    let unmatched = total - matched;

    report.push_str("## 统计信息\n\n");
    report.push_str(&format!("- 总测试指令数: {}\n", total));
    report.push_str(&format!("- 完全匹配数: {}\n", matched));
    report.push_str(&format!("- 不匹配数: {}\n", unmatched));
    report.push_str(&format!("- 匹配率: {:.1}%\n\n", matched as f64 / total as f64 * 100.0));

    if unmatched > 0 {
        report.push_str("## 不匹配的指令\n\n");
        report.push_str("| Hex | CsTool | Robustone | 类型 |\n");
        report.push_str("|-----|--------|-----------|------|\n");

        for result in results {
            if !result.is_complete_match() {
                let cstool_full = format!("{} {}", result.cstool_mnemonic, result.cstool_operands);
                let robustone_full = format!("{} {}", result.robustone_mnemonic, result.robustone_operands);

                report.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    result.hex, cstool_full, robustone_full,
                    if !result.match_mnemonic && !result.match_operands { "助记符+操作数" }
                    else if !result.match_mnemonic { "助记符" }
                    else { "操作数" }
                ));
            }
        }
    }

    fs::write("riscv_disasm_comparison.md", report)?;
    println!("已生成对比报告: riscv_disasm_comparison.md");

    Ok(())
}

fn main() {
    println!("RISC-V反汇编测试生成器");
    println!("========================\n");

    // 生成测试结果
    let results = generate_test_results();

    // 统计信息
    let total = results.len();
    let matched = results.iter().filter(|r| r.is_complete_match()).count();
    let unmatched = total - matched;

    println!("\n=== 结果统计 ===");
    println!("总测试指令数: {}", total);
    println!("完全匹配数: {}", matched);
    println!("不匹配数: {}", unmatched);
    println!("匹配率: {:.1}%", matched as f64 / total as f64 * 100.0);

    // 生成测试文件
    if let Err(e) = generate_test_file(&results) {
        eprintln!("生成测试文件失败: {}", e);
    }

    // 生成不匹配报告
    if let Err(e) = generate_mismatch_report(&results) {
        eprintln!("生成报告失败: {}", e);
    }

    if unmatched > 0 {
        println!("\n注意: 有{}个指令不匹配，请查看生成的报告了解详情。", unmatched);
    } else {
        println!("\n所有测试指令都与cstool匹配！");
    }
}