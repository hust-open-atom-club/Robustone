pub mod check;
pub mod parse;

// 重新导出统一入口，兼容 crate::cli::utils::* 的旧路径调用
pub use check::validate_architecture;
pub use parse::{parse_address, parse_hex_code};
