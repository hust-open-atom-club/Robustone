pub mod check;
pub mod parse;

// Re-export canonical entry points to keep `crate::utils::*` working for older code.
pub use check::validate_architecture;
pub use parse::{parse_address, parse_hex_code};
