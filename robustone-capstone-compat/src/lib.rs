//! Capstone compatibility testing crate.
//!
//! Parses Capstone's YAML test files and drives Robustone's decoders
//! against them, producing pass / fail / xfail reports.

pub mod adapter;
pub mod harness;
pub mod xfail;
pub mod yaml;

#[cfg(test)]
fn capstone_yaml_enabled() -> bool {
    std::env::var_os("ROBUSTONE_ENABLE_CAPSTONE_TESTS").is_some()
}

#[cfg(test)]
fn capstone_yaml_dir() -> std::path::PathBuf {
    if let Some(dir) = std::env::var_os("CAPSTONE_TEST_DIR") {
        return std::path::PathBuf::from(dir);
    }
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../third_party/capstone/tests/MC/LoongArch")
}

#[cfg(test)]
fn require_capstone_yaml(yaml_path: &str) -> bool {
    if !capstone_yaml_enabled() {
        eprintln!(
            "skip: ROBUSTONE_ENABLE_CAPSTONE_TESTS not set — {}",
            yaml_path
        );
        return false;
    }
    let resolved = capstone_yaml_dir().join(yaml_path);
    if !resolved.exists() {
        panic!("Capstone YAML fixture missing: {}", resolved.display());
    }
    true
}

#[cfg(test)]
mod tests {
    use super::harness::{self, TestResult};
    use super::require_capstone_yaml;
    use super::xfail;
    use robustone_core::ArchitectureDispatcher;
    use robustone_loongarch::LoongArchHandler;

    fn loongarch_dispatcher() -> ArchitectureDispatcher {
        let mut dispatcher = ArchitectureDispatcher::new();
        dispatcher.register(Box::new(LoongArchHandler::new()));
        dispatcher
    }

    fn run_yaml_file(path: &std::path::Path) -> (usize, usize, usize, usize) {
        let dispatcher = loongarch_dispatcher();
        let xfail = xfail::loongarch_default_xfails();
        let results = harness::run_yaml_file::<
            crate::adapter::CapstoneLoongArchYaml,
            robustone_loongarch::backend::LoongArchBackend,
        >(&dispatcher, path, &xfail);
        assert!(!results.is_empty(), "expected at least one test case");

        let (pass, fail, known_diff, unsupported) = harness::count_results(&results);

        for (idx, res) in results.iter().enumerate() {
            if let TestResult::Xfail(reason, detail) = res {
                eprintln!("case {} known diff ({}): {}", idx, reason, detail);
            } else if let TestResult::Fail(msg) = res {
                eprintln!("case {} failed: {}", idx, msg);
            }
        }

        (pass, fail, known_diff, unsupported)
    }

    #[test]
    fn test_arith_yaml_smoke() {
        if !require_capstone_yaml("arith.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("bit-shift.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("bit-manipu.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("branch.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("memory.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("atomic.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("barrier.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("base.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("pseudos.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("valid.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("f-move.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("f-memory.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("f-arith.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("d-arith.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("d-move.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("lvz.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("shuf.s.yaml") {
            return;
        }
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
        if !require_capstone_yaml("misc.s.yaml") {
            return;
        }
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
    use super::capstone_yaml_dir;
    use super::harness::{self, TestResult};
    use super::require_capstone_yaml;
    use super::xfail;
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
        if !require_capstone_yaml(".") {
            return;
        }
        let dispatcher = loongarch_dispatcher();
        let xfail = xfail::loongarch_default_xfails();
        let dir = capstone_yaml_dir();
        let dir: &std::path::Path = dir.as_ref();
        let results = harness::run_yaml_dir::<
            crate::adapter::CapstoneLoongArchYaml,
            robustone_loongarch::backend::LoongArchBackend,
        >(&dispatcher, dir, &xfail)
        .unwrap();

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
                TestResult::Pass => entry.0 += 1,
                TestResult::Unsupported(_) => entry.3 += 1,
                TestResult::Xfail(_, _) => entry.2 += 1,
                TestResult::Fail(msg) => {
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
