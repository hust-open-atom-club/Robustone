use robustone_cli::executor::run;

fn main() {
    if let Err(e) = run() {
        if !e.is_reported() {
            eprintln!("Error: {e}");
        }
        std::process::exit(e.exit_code());
    }
}
