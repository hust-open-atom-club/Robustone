pub mod arch;
pub mod command;
pub mod config;
pub mod disasm;
pub mod error;
pub mod executor;
pub mod utils;
pub mod version_info;

// 对外公开常用类型，保持对 crate::cli::{...} 的兼容
pub use arch::{Architecture, ArchitectureSpec};
pub use command::Cli;
pub use config::DisasmConfig;

// 兼容层：保留旧版直接从 crate::cli 调用的解析函数
use crate::cli::error::ValidationError;

/// 解析十六进制代码（保持向后兼容：返回字节序列）
/// 内部先解析为词列表，再展开为字节
pub fn parse_hex_code(input: &str) -> std::result::Result<Vec<u8>, ValidationError> {
    let words = crate::cli::utils::parse::parse_hex_code(input)?;
    crate::cli::utils::parse::hex_words_to_bytes(&words)
}

/// 解析地址（保持向后兼容）
pub fn parse_address(input: &str) -> std::result::Result<u64, ValidationError> {
    crate::cli::utils::parse_address(input)
}

/// 验证架构字符串（保持向后兼容）
pub fn validate_architecture(arch_str: &str) -> Result<String, String> {
    crate::cli::utils::validate_architecture(arch_str)
}

#[cfg(test)]
mod tests;
