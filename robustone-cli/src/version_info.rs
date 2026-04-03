//! Version information presenter.
//!
//! Provides the `-v/--version` CLI output with dynamic capability statistics
//! sourced from the shared architecture registry.

use robustone_core::all_architecture_capabilities;
use std::collections::HashMap;

/// Print the full version banner along with capability stats.
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
    let capabilities = all_architecture_capabilities();
    let total = capabilities.len();
    let implemented = capabilities
        .iter()
        .filter(|cap| cap.decode_supported)
        .count();
    println!("Architecture Support: {implemented}/{total}");

    let riscv_count = capabilities
        .iter()
        .filter(|cap| cap.category == "RISC-V")
        .count();
    let riscv_implemented = capabilities
        .iter()
        .filter(|cap| cap.category == "RISC-V" && cap.decode_supported)
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

    let capabilities = all_architecture_capabilities();
    let mut categories: HashMap<&str, Vec<&robustone_core::ArchitectureCapability>> =
        HashMap::new();

    for capability in capabilities {
        categories
            .entry(capability.category)
            .or_default()
            .push(capability);
    }

    let category_order = ["RISC-V", "ARM", "x86", "MIPS", "PowerPC", "SPARC", "Other"];

    for category in category_order {
        if let Some(category_archs) = categories.get(category) {
            let names: Vec<String> = category_archs
                .iter()
                .map(|capability| {
                    format!(
                        "{}{}",
                        capability.canonical_name,
                        capability.implementation_status()
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
    let capabilities = all_architecture_capabilities();
    let total = capabilities.len();
    let implemented = capabilities
        .iter()
        .filter(|cap| cap.decode_supported)
        .count();
    let percentage = (implemented * 100) / total;

    println!("Implementation Progress:");
    println!("  {percentage}% Complete ({implemented}/{total})");
    println!();
    println!("Status Legend:");
    println!("  ✅ Decode backend implemented");
    println!("  ❌ Parser-only or not implemented");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info_display() {
        print_version_info();
    }

    #[test]
    fn test_architecture_categories() {
        let capabilities = all_architecture_capabilities();
        let mut categories = HashMap::new();

        for capability in capabilities {
            categories
                .entry(capability.category)
                .or_insert_with(Vec::new)
                .push(capability);
        }

        assert!(categories.contains_key("RISC-V"));
        assert!(categories.contains_key("ARM"));
        assert!(categories.contains_key("x86"));
    }

    #[test]
    fn test_implementation_calculation() {
        let capabilities = all_architecture_capabilities();
        let implemented = capabilities
            .iter()
            .filter(|cap| cap.decode_supported)
            .count();
        let total = capabilities.len();

        assert!(implemented >= 2);
        assert!(total > implemented);
    }
}
