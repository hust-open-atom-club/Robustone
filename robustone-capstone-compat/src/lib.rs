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

    fn run_yaml_file(path: &std::path::Path) -> (usize, usize, usize, usize) {
        let dispatcher = loongarch_dispatcher();
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
                Err(msg) if msg.contains("screl") => {
                    known_diff += 1;
                    eprintln!("case {} known diff: {}", idx, msg);
                }
                Err(msg) => {
                    fail += 1;
                    eprintln!("case {} failed: {}", idx, msg);
                }
            }
        }

        (pass, fail, known_diff, unsupported)
    }

    #[test]
    fn test_arith_yaml_smoke() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/arith.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "arith.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_bit_shift_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/bit-shift.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "bit-shift.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_branch_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/branch.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "branch.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_memory_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/memory.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "memory.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_atomic_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/atomic.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "atomic.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_barrier_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/barrier.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "barrier.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }
}
