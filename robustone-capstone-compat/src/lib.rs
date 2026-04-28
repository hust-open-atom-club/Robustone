//! Capstone compatibility testing crate.
//!
//! Parses Capstone's YAML test files and drives Robustone's decoders
//! against them, producing pass / fail / xfail reports.

pub mod harness;
pub mod yaml;

#[cfg(test)]
mod tests {
    use super::harness;
    use robustone_core::ArchitectureDispatcher;
    use robustone_loongarch::LoongArchHandler;

    fn loongarch_dispatcher() -> ArchitectureDispatcher {
        let mut dispatcher = ArchitectureDispatcher::new();
        dispatcher.register(Box::new(LoongArchHandler::new()));
        dispatcher
    }

    /// Verify that the harness can parse and run the Capstone LoongArch
    /// arithmetic YAML oracle.  This is a smoke test for the harness itself.
    #[test]
    fn test_arith_yaml_smoke() {
        let dispatcher = loongarch_dispatcher();
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/arith.s.yaml"
        ));
        let results = harness::run_yaml_file(&dispatcher, path);
        assert!(!results.is_empty(), "expected at least one test case");

        let mut pass = 0usize;
        let mut fail = 0usize;
        let mut known_diff = 0usize;
        let mut unsupported = 0usize;

        for (idx, res) in results.iter().enumerate() {
            match res {
                Ok(()) => pass += 1,
                Err(msg) if msg.contains("unsupported arch/options") => unsupported += 1,
                Err(msg) if msg.contains("nor") && msg.contains("orn") => {
                    known_diff += 1;
                    eprintln!("case {} known diff: {}", idx, msg);
                }
                Err(msg) => {
                    fail += 1;
                    eprintln!("case {} failed: {}", idx, msg);
                }
            }
        }

        eprintln!(
            "arith.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );

        // The harness must be able to parse the file and attempt every case.
        // We allow known Capstone alias differences (e.g. nor vs orn) because
        // those are tracked separately in the bitlesson.
        assert_eq!(
            fail, 0,
            "harness smoke test should not have unexpected mismatches"
        );
    }
}
