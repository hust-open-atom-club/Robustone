//! Hexadecimal string parsing utilities.
//!
//! This module provides robust hex string parsing functionality that can be
//! used across different architectures. It handles various input formats and
//! provides clear error messages for invalid inputs.

use crate::error::DisasmError;

/// Parser for hexadecimal strings with various formats and prefixes.
///
/// This struct provides methods to parse hexadecimal strings into byte vectors,
/// handling common formats like:
/// - Plain hex: "deadbeef"
/// - With prefix: "0xdeadbeef"
/// - Mixed case: "DeAdBeEf"
/// - Spaced: "de ad be ef"
#[derive(Debug, Clone, Default)]
pub struct HexParser {
    /// Default endianness for parsing (can be overridden per architecture)
    default_endianness: crate::utils::Endianness,
}

impl HexParser {
    /// Creates a new hex parser with default little-endian ordering.
    pub fn new() -> Self {
        Self {
            default_endianness: crate::utils::Endianness::Little,
        }
    }

    /// Creates a hex parser with specified default endianness.
    pub fn with_endianness(endianness: crate::utils::Endianness) -> Self {
        Self {
            default_endianness: endianness,
        }
    }

    /// Parses a hexadecimal string into a byte vector.
    ///
    /// # Arguments
    ///
    /// * `hex_str` - The hexadecimal string to parse
    /// * `endianness` - Optional endianness override. If None, uses parser default
    ///
    /// # Returns
    ///
    /// Returns a vector of bytes on success, or a DisasmError on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use robustone_core::prelude::*;
    /// let parser = HexParser::new();
    /// let bytes = parser.parse("deadbeef", None).unwrap();
    /// assert_eq!(bytes, vec![0xef, 0xbe, 0xad, 0xde]);
    /// ```
    pub fn parse(
        &self,
        hex_str: &str,
        endianness: Option<crate::utils::Endianness>,
    ) -> Result<Vec<u8>, DisasmError> {
        let cleaned = self.clean_hex_string(hex_str)?;
        let bytes = self.convert_to_bytes(&cleaned)?;
        let final_endianness = endianness.unwrap_or(self.default_endianness);

        Ok(self.apply_endianness(bytes, final_endianness))
    }

    /// Parses a hex string with architecture-specific byte order handling.
    ///
    /// This method provides a convenience interface for architecture-specific
    /// parsing where the endianness is determined by the architecture name.
    ///
    /// # Arguments
    ///
    /// * `hex_str` - The hexadecimal string to parse
    /// * `arch_name` - The architecture name (e.g., "riscv32", "arm", "x86")
    ///
    /// # Returns
    ///
    /// Returns a vector of bytes properly ordered for the specified architecture.
    pub fn parse_for_architecture(
        &self,
        hex_str: &str,
        arch_name: &str,
    ) -> Result<Vec<u8>, DisasmError> {
        let endianness = self.determine_architecture_endianness(arch_name);
        self.parse(hex_str, Some(endianness))
    }

    /// Cleans and normalizes a hexadecimal string.
    ///
    /// This method removes prefixes, whitespace, and validates that the string
    /// contains only valid hexadecimal characters.
    fn clean_hex_string(&self, input: &str) -> Result<String, DisasmError> {
        let trimmed = input.trim().to_lowercase();

        // Remove common prefixes
        let no_prefix = if let Some(stripped) = trimmed.strip_prefix("0x") {
            stripped
        } else {
            &trimmed
        };

        // Remove whitespace and validate characters
        let cleaned: String = no_prefix.chars().filter(|c| !c.is_whitespace()).collect();

        if cleaned.is_empty() {
            return Err(DisasmError::DecodingError(
                "Empty hexadecimal string".to_string(),
            ));
        }

        // Validate that all characters are valid hex
        if !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DisasmError::DecodingError(
                "Invalid hexadecimal characters found".to_string(),
            ));
        }

        // Ensure even number of characters
        if !cleaned.len().is_multiple_of(2) {
            return Err(DisasmError::DecodingError(
                "Hexadecimal string must have even number of characters".to_string(),
            ));
        }

        Ok(cleaned)
    }

    /// Converts cleaned hex string to byte vector.
    fn convert_to_bytes(&self, cleaned: &str) -> Result<Vec<u8>, DisasmError> {
        let mut bytes = Vec::with_capacity(cleaned.len() / 2);
        let mut i = 0;

        while i < cleaned.len() {
            let byte_str = &cleaned[i..i + 2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => {
                    return Err(DisasmError::DecodingError(format!(
                        "Invalid hexadecimal byte: {byte_str}"
                    )));
                }
            }
            i += 2;
        }

        Ok(bytes)
    }

    /// Applies endianness conversion to the byte vector.
    /// Input bytes are in the order they appear in the hex string (big-endian textual order).
    /// For little-endian targets, this reverses the bytes to match memory layout.
    fn apply_endianness(
        &self,
        mut bytes: Vec<u8>,
        endianness: crate::utils::Endianness,
    ) -> Vec<u8> {
        match endianness {
            crate::utils::Endianness::Big => bytes,
            crate::utils::Endianness::Little => {
                bytes.reverse();
                bytes
            }
        }
    }

    /// Determines the appropriate endianness for a given architecture.
    ///
    /// This method contains architecture-specific knowledge about byte ordering.
    /// Future architectures should be added here as they are supported.
    fn determine_architecture_endianness(&self, arch_name: &str) -> crate::utils::Endianness {
        // RISC-V architectures use little-endian by default
        if arch_name.starts_with("riscv") {
            return crate::utils::Endianness::Little; // RISC-V uses little-endian byte order
        }

        // ARM can be either, but we'll use little-endian as default
        if arch_name.starts_with("arm") || arch_name.starts_with("aarch64") {
            return crate::utils::Endianness::Little;
        }

        // x86/x64 are little-endian
        if arch_name.starts_with("x86") || arch_name.starts_with("x64") {
            return crate::utils::Endianness::Little;
        }

        // Default to little-endian for unknown architectures
        crate::utils::Endianness::Little
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hex_parsing() {
        let parser = HexParser::new();
        let result = parser.parse("deadbeef", None).unwrap();
        assert_eq!(result, vec![0xef, 0xbe, 0xad, 0xde]);
    }

    #[test]
    fn test_hex_with_prefix() {
        let parser = HexParser::new();
        let result = parser.parse("0x1234", None).unwrap();
        assert_eq!(result, vec![0x34, 0x12]);
    }

    #[test]
    fn test_hex_with_whitespace() {
        let parser = HexParser::new();
        let result = parser.parse("de ad be ef", None).unwrap();
        assert_eq!(result, vec![0xef, 0xbe, 0xad, 0xde]);
    }

    #[test]
    fn test_mixed_case_hex() {
        let parser = HexParser::new();
        let result = parser.parse("DeAdBeEf", None).unwrap();
        assert_eq!(result, vec![0xef, 0xbe, 0xad, 0xde]);
    }

    #[test]
    fn test_endianness_handling() {
        let parser = HexParser::new();

        // Little-endian (default)
        let le_result = parser
            .parse("12345678", Some(crate::utils::Endianness::Little))
            .unwrap();
        assert_eq!(le_result, vec![0x78, 0x56, 0x34, 0x12]);

        // Big-endian
        let be_result = parser
            .parse("12345678", Some(crate::utils::Endianness::Big))
            .unwrap();
        assert_eq!(be_result, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_architecture_specific_parsing() {
        let parser = HexParser::new();

        // RISC-V should use little-endian (no reverse order)
        let riscv_result = parser
            .parse_for_architecture("deadbeef", "riscv32")
            .unwrap();
        assert_eq!(riscv_result, vec![0xef, 0xbe, 0xad, 0xde]);

        // x86 should use little-endian
        let x86_result = parser.parse_for_architecture("deadbeef", "x86").unwrap();
        assert_eq!(x86_result, vec![0xef, 0xbe, 0xad, 0xde]);
    }

    #[test]
    fn test_error_handling() {
        let parser = HexParser::new();

        // Empty string
        assert!(parser.parse("", None).is_err());

        // Odd number of characters
        assert!(parser.parse("123", None).is_err());

        // Invalid characters
        assert!(parser.parse("xyz", None).is_err());
    }
}
