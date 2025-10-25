//! 版本信息模块
//!
//! 这个模块提供版本信息的显示功能，支持：
//! - 动态架构支持统计
//! - 分类显示
//! - 实现状态可视化
//! - 简洁的配置化管理

use crate::cli::Architecture;
use std::collections::HashMap;

/// 打印版本信息
pub fn print_version_info() {
    print_basic_info();
    print_architecture_summary();
    print_implementation_status();
    print_detailed_status();
}

/// 打印基本信息
fn print_basic_info() {
    println!("Robustone v0.1.0");
    println!("Capstone-compatible disassembly engine");
    println!();
}

/// 打印架构支持概要
fn print_architecture_summary() {
    let archs = Architecture::all_architectures();
    let total = archs.len();
    let implemented = archs.iter().filter(|a| a.is_implemented()).count();

    println!("Architecture Support: {}/{}", implemented, total);

    // RISC-V专门状态
    let riscv_count = archs.iter().filter(|a| a.category() == "RISC-V").count();
    let riscv_implemented = archs
        .iter()
        .filter(|a| a.category() == "RISC-V" && a.is_implemented())
        .count();

    match riscv_implemented {
        count if count == riscv_count && riscv_count > 0 => {
            println!(
                "RISC-V: ✅ Complete ({}/{})",
                riscv_implemented, riscv_count
            );
        }
        count if count > 0 => {
            println!("RISC-V: ⚠️ Partial ({}/{})", riscv_implemented, riscv_count);
        }
        _ => {
            println!("RISC-V: ❌ Not supported");
        }
    }

    println!();
}

/// 打印实现状态
fn print_implementation_status() {
    println!("Supported Architectures:");

    let archs = Architecture::all_architectures();
    let mut categories: HashMap<&str, Vec<&Architecture>> = HashMap::new();

    for arch in &archs {
        categories
            .entry(arch.category())
            .or_insert_with(Vec::new)
            .push(arch);
    }

    // 按类别显示
    let category_order = ["RISC-V", "ARM", "x86", "MIPS", "PowerPC", "SPARC", "Other"];

    for category in category_order {
        if let Some(category_archs) = categories.get(category) {
            let names: Vec<String> = category_archs
                .iter()
                .map(|a| {
                    format!(
                        "{}{}",
                        a.name(),
                        if a.is_implemented() { "✅" } else { "❌" }
                    )
                })
                .collect();
            println!("  {}: {}", category, names.join(", "));
        }
    }

    println!();
}

/// 打印详细状态
fn print_detailed_status() {
    let archs = Architecture::all_architectures();
    let total = archs.len();
    let implemented = archs.iter().filter(|a| a.is_implemented()).count();
    let percentage = (implemented * 100) / total;

    println!("Implementation Progress:");
    println!("  {}% Complete ({}/{})", percentage, implemented, total);
    println!();
    println!("Status Legend:");
    println!("  ✅ Implemented and tested");
    println!("  ❌ Not implemented");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info_display() {
        // 确保版本信息可以正常显示
        print_version_info();
    }

    #[test]
    fn test_architecture_categories() {
        let archs = Architecture::all_architectures();
        let mut categories = HashMap::new();

        for arch in &archs {
            categories
                .entry(arch.category())
                .or_insert_with(Vec::new)
                .push(arch);
        }

        // 验证RISC-V类别存在
        assert!(categories.contains_key("RISC-V"));
        assert!(categories.contains_key("ARM"));
        assert!(categories.contains_key("x86"));
    }

    #[test]
    fn test_implementation_calculation() {
        let archs = Architecture::all_architectures();
        let implemented = archs.iter().filter(|a| a.is_implemented()).count();
        let total = archs.len();

        // 应该至少有RISC-V 32和64两个实现
        assert!(implemented >= 2);
        assert!(total > implemented);
    }
}
