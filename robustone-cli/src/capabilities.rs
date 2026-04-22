use robustone_core::{ArchitectureCapability, all_architecture_capabilities};
use serde::Serialize;
use std::fmt::Write;

const PARSER_ONLY_NOTE: &str = "Tokens marked parser-only are accepted by the CLI parser as capability placeholders, but they fail with a configuration error before decode because no backend is implemented yet. Run `robustone --capabilities` (or `robustone --support-matrix`) to inspect the current registry-derived support surface.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilitySummary {
    pub total_architectures: usize,
    pub decode_ready: usize,
    pub parser_only: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityRow {
    pub canonical_name: &'static str,
    pub category: &'static str,
    pub aliases: &'static [&'static str],
    pub parse_supported: bool,
    pub decode_supported: bool,
    pub detail_supported: bool,
    pub json_supported: bool,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityReport {
    pub summary: CapabilitySummary,
    pub architectures: Vec<CapabilityRow>,
    pub note: &'static str,
}

pub fn capability_report() -> CapabilityReport {
    let capabilities = all_architecture_capabilities();
    let architectures = capabilities
        .iter()
        .map(|capability| CapabilityRow {
            canonical_name: capability.canonical_name,
            category: capability.category,
            aliases: capability.aliases,
            parse_supported: capability.parse_supported,
            decode_supported: capability.decode_supported,
            detail_supported: capability.detail_supported,
            json_supported: capability.json_supported,
            status: capability_status(capability),
        })
        .collect::<Vec<_>>();

    let decode_ready = capabilities
        .iter()
        .filter(|capability| capability.decode_supported)
        .count();
    let total_architectures = capabilities.len();

    CapabilityReport {
        summary: CapabilitySummary {
            total_architectures,
            decode_ready,
            parser_only: total_architectures.saturating_sub(decode_ready),
        },
        architectures,
        note: PARSER_ONLY_NOTE,
    }
}

pub fn render_capabilities_text() -> String {
    let report = capability_report();
    let mut output = String::new();

    writeln!(output, "Architecture Capabilities (shared registry)")
        .expect("writing capability header should succeed");
    writeln!(
        output,
        "Summary: {} decode-ready / {} parser-only / {} total",
        report.summary.decode_ready, report.summary.parser_only, report.summary.total_architectures
    )
    .expect("writing capability summary should succeed");
    writeln!(output).expect("writing blank separator should succeed");
    writeln!(
        output,
        "| Token | Category | Parse | Decode | Detail | JSON | Status |"
    )
    .expect("writing capability table header should succeed");
    writeln!(
        output,
        "|-------|----------|-------|--------|--------|------|--------|"
    )
    .expect("writing capability table separator should succeed");

    for row in &report.architectures {
        writeln!(
            output,
            "| `{}` | `{}` | {} | {} | {} | {} | {} |",
            row.canonical_name,
            row.category,
            yes_no(row.parse_supported),
            yes_no(row.decode_supported),
            yes_no(row.detail_supported),
            yes_no(row.json_supported),
            row.status
        )
        .expect("writing capability table row should succeed");
    }

    writeln!(output).expect("writing blank separator should succeed");
    write!(output, "Note: {}", report.note).expect("writing capability note should succeed");
    output
}

pub fn render_capabilities_json() -> String {
    serde_json::to_string_pretty(&capability_report())
        .expect("serializing capability report should succeed")
}

pub fn parser_only_configuration_message(
    input_token: &str,
    capability: &ArchitectureCapability,
) -> String {
    if input_token == capability.canonical_name {
        format!(
            "Architecture '{}' is currently parser-only: the CLI accepts the token, but no decode backend is implemented yet. Run `robustone --capabilities` or `robustone --help` to inspect the advertised support surface.",
            capability.canonical_name
        )
    } else {
        format!(
            "Architecture input '{}' resolves to parser-only canonical token '{}': the CLI accepts the token, but no decode backend is implemented yet. Run `robustone --capabilities` or `robustone --help` to inspect the advertised support surface.",
            input_token, capability.canonical_name
        )
    }
}

fn capability_status(capability: &ArchitectureCapability) -> &'static str {
    if capability.decode_supported {
        "decode-ready"
    } else {
        "parser-only"
    }
}

fn yes_no(enabled: bool) -> &'static str {
    if enabled { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_capability_report_summary_tracks_shared_registry() {
        let report = capability_report();
        let capabilities = all_architecture_capabilities();

        assert_eq!(report.summary.total_architectures, capabilities.len());
        assert_eq!(report.architectures.len(), capabilities.len());
        assert_eq!(
            report.summary.decode_ready,
            capabilities
                .iter()
                .filter(|capability| capability.decode_supported)
                .count()
        );
        assert!(
            report
                .architectures
                .iter()
                .any(|row| row.status == "parser-only")
        );
    }

    #[test]
    fn test_render_capabilities_text_mentions_parser_only_note() {
        let output = render_capabilities_text();

        assert!(output.contains("Architecture Capabilities (shared registry)"));
        assert!(output.contains("decode-ready"));
        assert!(output.contains("parser-only"));
        assert!(output.contains("robustone --capabilities"));
    }

    #[test]
    fn test_render_capabilities_json_includes_aliases_and_status() {
        let output = render_capabilities_json();
        let parsed: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["summary"]["decode_ready"], 6);
        assert_eq!(parsed["architectures"][0]["canonical_name"], "riscv32");
        assert_eq!(parsed["architectures"][0]["status"], "decode-ready");
        assert!(parsed["architectures"][1]["aliases"].is_array());
    }

    #[test]
    fn test_parser_only_configuration_message_points_to_capability_surface() {
        let capability = all_architecture_capabilities()
            .iter()
            .find(|capability| capability.canonical_name == "riscv32e")
            .expect("riscv32e capability should exist");
        let message = parser_only_configuration_message("riscv32e", capability);

        assert!(message.contains("parser-only"));
        assert!(message.contains("riscv32e"));
        assert!(message.contains("robustone --capabilities"));
    }
}
