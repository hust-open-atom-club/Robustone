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
    fn test_bit_manipu_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/bit-manipu.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "bit-manipu.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
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

    #[test]
    fn test_base_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/base.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "base.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_pseudos_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/pseudos.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "pseudos.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_valid_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/valid.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "valid.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_f_move_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/f-move.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "f-move.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_f_memory_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/f-memory.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "f-memory.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_f_arith_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/f-arith.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "f-arith.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_d_arith_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/d-arith.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "d-arith.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_d_move_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/d-move.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "d-move.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_lvz_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/lvz.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "lvz.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_shuf_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/shuf.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "shuf.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }

    #[test]
    fn test_misc_yaml() {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch/misc.s.yaml"
        ));
        let (pass, fail, known_diff, unsupported) = run_yaml_file(path);
        eprintln!(
            "misc.s.yaml summary: {} pass, {} fail, {} known_diff, {} unsupported",
            pass, fail, known_diff, unsupported
        );
        assert_eq!(fail, 0, "unexpected mismatches");
    }
}

#[cfg(test)]
mod bulk_tests {
    use super::harness;
    use robustone_core::ArchitectureDispatcher;
    use robustone_loongarch::LoongArchHandler;

    fn loongarch_dispatcher() -> ArchitectureDispatcher {
        let mut dispatcher = ArchitectureDispatcher::new();
        dispatcher.register(Box::new(LoongArchHandler::new()));
        dispatcher
    }

    #[test]
    #[ignore]
    fn bulk_run_all_loongarch_yaml() {
        let dispatcher = loongarch_dispatcher();
        let dir = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../third_party/capstone/tests/MC/LoongArch"
        ));
        let results = harness::run_yaml_dir(&dispatcher, dir).unwrap();

        let mut total_pass = 0usize;
        let mut total_fail = 0usize;
        let mut total_known = 0usize;
        let mut total_unsupported = 0usize;

        use std::collections::HashMap;
        let mut per_file: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
        for (file, _idx, res) in &results {
            let file = file.file_name().unwrap().to_str().unwrap();
            let entry = per_file.entry(file.to_string()).or_insert((0, 0, 0, 0));
            match res {
                Ok(()) => entry.0 += 1,
                Err(msg) if msg.contains("unsupported arch/options") => entry.3 += 1,
                Err(msg) if msg.contains("nor") && msg.contains("orn") => entry.2 += 1,
                Err(msg) if msg.contains("screl") => entry.2 += 1,
                Err(msg) => {
                    entry.1 += 1;
                    if entry.1 <= 3 {
                        eprintln!("{}: {}", file, msg);
                    }
                }
            }
        }

        for (file, (pass, fail, known, unsupported)) in &per_file {
            total_pass += pass;
            total_fail += fail;
            total_known += known;
            total_unsupported += unsupported;
            if *fail > 0 {
                eprintln!(
                    "{}: {} pass, {} fail, {} known, {} unsupported",
                    file, pass, fail, known, unsupported
                );
            }
        }

        eprintln!(
            "TOTAL: {} pass, {} fail, {} known, {} unsupported",
            total_pass, total_fail, total_known, total_unsupported
        );
    }
}
