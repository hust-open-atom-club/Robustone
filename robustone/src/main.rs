use robustone_cli::executor::run;

// Ensure all architecture handler crates are linked so inventory
// registers their ArchitectureHandler implementations.
use robustone_arm as _;
use robustone_loongarch as _;
use robustone_riscv as _;
use robustone_x86 as _;

fn main() {
    if let Err(e) = run() {
        if !e.is_reported() {
            eprintln!("Error: {e}");
        }
        std::process::exit(e.exit_code());
    }
}
