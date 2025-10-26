//! Consolidated utility functions for CLI operations.
//!
//! This module provides common validation, parsing, and conversion utilities
//! used throughout the robustone-cli application.

use crate::error::{CliError, Result};

/// Validate an architecture string with comprehensive error reporting.
pub fn validate_architecture(arch_str: &str) -> Result<String> {
    // Expanded list of architecture prefixes derived from Capstone support.
    let valid_prefixes = [
        // RISC-V
        "riscv32",
        "riscv64",
        "riscv32e",
        // ARM
        "arm",
        "armle",
        "armbe",
        "thumb",
        // AArch64
        "aarch64",
        "aarch64be",
        // x86
        "x16",
        "x32",
        "x86",
        "x64",
        "x86-64",
        "x86_64",
        // MIPS
        "mips",
        "mipsel",
        "mips64",
        "mips64el",
        // PowerPC
        "ppc",
        "powerpc",
        "ppc32",
        "powerpc32",
        "ppcbe",
        "powerpcbe",
        "ppc32be",
        "powerpc32be",
        "ppc64",
        "powerpc64",
        "ppc64be",
        "powerpc64be",
        // SPARC
        "sparc",
        "sparcle",
        "sparc64",
        // Other
        "systemz",
        "s390x",
        "xcore",
        "m68k",
        "tms320c64x",
        "c64x",
        "m680x",
        "evm",
        "bpf",
    ];

    let arch_str_lower = arch_str.to_lowercase();
    let parts: Vec<&str> = arch_str_lower.split('+').collect();

    if parts.is_empty() {
        return Err(CliError::validation(
            "architecture",
            "Empty architecture string",
        ));
    }

    // Ensure the base architecture is supported before considering modifiers.
    let base_arch = parts[0];
    let is_valid = valid_prefixes.contains(&base_arch);

    if !is_valid {
        return Err(CliError::validation(
            "architecture",
            format!(
                "Invalid architecture: {base_arch}. Supported: riscv32, riscv64, arm, aarch64, x86, mips, ppc, sparc, systemz, and others",
            ),
        ));
    }

    Ok(arch_str.to_string())
}

/// Parse and validate hexadecimal code into canonical tokens.
///
/// Examples:
/// - `"0x00000000 0x00000011"` → `vec!["0x00000000", "0x00000011"]`
/// - Accepts `0x`/`0X` prefixes
/// - Only hexadecimal characters are accepted
/// - Tokens must contain an even number of digits (excluding the prefix)
pub fn parse_hex_code(input: &str) -> Result<Vec<String>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CliError::validation("hex_code", "Empty hex code provided"));
    }

    let mut words: Vec<String> = Vec::new();
    for raw in trimmed.split_whitespace() {
        if raw.is_empty() {
            continue;
        }

        let normalized = normalize_hex_token(raw)?;
        words.push(normalized);
    }

    if words.is_empty() {
        return Err(CliError::validation(
            "hex_code",
            "No valid hex tokens found",
        ));
    }

    Ok(words)
}

/// Parse a hexadecimal address with validation.
pub fn parse_address(input: &str) -> Result<u64> {
    if input.trim().is_empty() {
        return Err(CliError::validation("address", "Empty address provided"));
    }

    let normalized = normalize_hex_token(input)?;

    u64::from_str_radix(&normalized[2..], 16)
        .map_err(|_| CliError::validation("address", "Invalid hexadecimal address format"))
}

/// Convert hex words to raw bytes.
pub fn hex_words_to_bytes(words: &[String]) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    for word in words {
        let normalized = normalize_hex_token(word)?;
        let hex_part = &normalized[2..]; // Remove "0x" prefix

        for i in (0..hex_part.len()).step_by(2) {
            let byte_str = &hex_part[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16).map_err(|_| {
                CliError::validation("hex_code", format!("Invalid hex byte: {byte_str}"))
            })?;
            bytes.push(byte);
        }
    }

    Ok(bytes)
}

/// Parse hex string directly to bytes (convenience function).
pub fn parse_hex_to_bytes(input: &str) -> Result<Vec<u8>> {
    let words = parse_hex_code(input)?;
    hex_words_to_bytes(&words)
}

/// Normalize a hex token to canonical format (0x prefix + lowercase).
fn normalize_hex_token(token: &str) -> Result<String> {
    let trimmed = token.trim().to_lowercase();

    if trimmed.is_empty() {
        return Err(CliError::validation("hex_token", "Empty hex token"));
    }

    let hex_part = if trimmed.strip_prefix("0x").is_some() {
        &trimmed[2..]
    } else {
        &trimmed
    };

    if hex_part.is_empty() {
        return Err(CliError::validation("hex_token", "Empty hex content"));
    }

    if hex_part.len() % 2 != 0 {
        return Err(CliError::validation(
            "hex_token",
            "Hex token must have even number of digits",
        ));
    }

    // Validate all characters are hexadecimal
    for c in hex_part.chars() {
        if !c.is_ascii_hexdigit() {
            return Err(CliError::validation(
                "hex_token",
                format!("Invalid hex character: {c}"),
            ));
        }
    }

    Ok(format!("0x{hex_part}"))
}

/// Format bytes as a hex string with optional spaces.
pub fn format_bytes_as_hex(bytes: &[u8], with_spaces: bool) -> String {
    if with_spaces {
        bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        hex::encode(bytes)
    }
}

/// Validate that a string represents a valid hexadecimal number.
pub fn is_valid_hex(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    let hex_part = if trimmed.to_lowercase().starts_with("0x") {
        &trimmed[2..]
    } else {
        trimmed
    };

    !hex_part.is_empty() && hex_part.chars().all(|c| c.is_ascii_hexdigit())
}

/// Calculate the number of bytes needed to represent hex data.
pub fn hex_byte_count(hex_str: &str) -> Result<usize> {
    let normalized = normalize_hex_token(hex_str)?;
    Ok(normalized.len() / 2 - 1) // Subtract 1 for "0x" prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_architecture() {
        assert!(validate_architecture("riscv32").is_ok());
        assert!(validate_architecture("riscv64+compressed").is_ok());
        assert!(validate_architecture("invalid").is_err());
    }

    #[test]
    fn test_parse_hex_code() {
        let result = parse_hex_code("0x1234 5678").unwrap();
        assert_eq!(result, vec!["0x1234", "0x5678"]);
    }

    #[test]
    fn test_parse_address() {
        assert_eq!(parse_address("0x1000").unwrap(), 0x1000);
        assert_eq!(parse_address("1000").unwrap(), 0x1000);
    }

    #[test]
    fn test_hex_words_to_bytes() {
        let words = vec!["0x1234".to_string(), "0x5678".to_string()];
        let bytes = hex_words_to_bytes(&words).unwrap();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_is_valid_hex() {
        assert!(is_valid_hex("0x1234"));
        assert!(is_valid_hex("1234"));
        assert!(!is_valid_hex("12x4"));
        assert!(!is_valid_hex(""));
    }
}

// Legacy re-exports for backward compatibility
pub use self::{
    parse_address as parse_address_legacy, parse_hex_code as parse_hex_code_legacy,
    validate_architecture as validate_architecture_legacy,
};
