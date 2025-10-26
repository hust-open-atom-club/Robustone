//! Version information presenter.
//!
//! Provides the `-v/--version` CLI output with:
//! - dynamic architecture coverage statistics
//! - category breakdowns
//! - implementation status indicators
//! - dedicated helper functions for maintainability

use crate::Architecture;
use std::collections::HashMap;

/// Print the full version banner along with implementation stats.
pub fn print_version_info() {
    print_basic_info();
    print_architecture_summary();
    print_implementation_status();
    print_detailed_status();
}

/// Print the basic banner headline.
fn print_basic_info() {
    println!("Robustone v{}", clap::crate_version!());
    println!("Capstone-compatible disassembly engine");
    println!();
}

/// Print the high-level architecture support summary.
fn print_architecture_summary() {
    let archs = Architecture::all_architectures();
    let total = archs.len();
    let implemented = archs.iter().filter(|a| a.is_implemented()).count();
    println!("Architecture Support: {implemented}/{total}");

    // Dedicated breakdown for the RISC-V family.
    let riscv_count = archs.iter().filter(|a| a.category() == "RISC-V").count();
    let riscv_implemented = archs
        .iter()
        .filter(|a| a.category() == "RISC-V" && a.is_implemented())
        .count();

    match riscv_implemented {
        count if count == riscv_count && riscv_count > 0 => {
            println!("RISC-V: ✅ Complete ({riscv_implemented}/{riscv_count})");
        }
        count if count > 0 => {
            println!("RISC-V: ⚠️ Partial ({riscv_implemented}/{riscv_count})");
        }
        _ => {
            println!("RISC-V: ❌ Not supported");
        }
    }

    println!();
}

/// Print architecture categories and their readiness.
fn print_implementation_status() {
    println!("Supported Architectures:");

    let archs = Architecture::all_architectures();
    let mut categories: HashMap<&str, Vec<&Architecture>> = HashMap::new();

    for arch in &archs {
        categories.entry(arch.category()).or_default().push(arch);
    }

    // Display entries grouped by category for readability.
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

/// Print coverage percentages and status legend.
fn print_detailed_status() {
    let archs = Architecture::all_architectures();
    let total = archs.len();
    let implemented = archs.iter().filter(|a| a.is_implemented()).count();
    let percentage = (implemented * 100) / total;

    println!("Implementation Progress:");
    println!("  {percentage}% Complete ({implemented}/{total})");
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
        // Ensure the version banner prints without panicking.
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

        // Ensure key architecture categories are present in the summary.
        assert!(categories.contains_key("RISC-V"));
        assert!(categories.contains_key("ARM"));
        assert!(categories.contains_key("x86"));
    }

    #[test]
    fn test_implementation_calculation() {
        let archs = Architecture::all_architectures();
        let implemented = archs.iter().filter(|a| a.is_implemented()).count();
        let total = archs.len();

        // At least the two RISC-V variants should be implemented.
        assert!(implemented >= 2);
        assert!(total > implemented);
    }
}
