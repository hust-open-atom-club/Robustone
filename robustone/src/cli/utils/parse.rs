use crate::cli::error::ValidationError;

/// Normalise a raw hex string into a list of canonical tokens.
///
/// Examples:
/// - `"0x00000000 0x00000011"` → `vec!["0x00000000", "0x00000011"]`
/// - tokens may include `0x`/`0X` prefixes
/// - only hexadecimal characters are accepted
/// - tokens must contain an even number of digits (excluding the prefix)
pub fn parse_hex_code(input: &str) -> std::result::Result<Vec<String>, ValidationError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ValidationError::EmptyHexCode);
    }

    let mut words: Vec<String> = Vec::new();
    for raw in trimmed.split_whitespace() {
        if raw.is_empty() {
            continue;
        }

        // Normalise the prefix and force lowercase for consistent downstream parsing.
        let lower = raw.to_lowercase();
        let no_prefix = if lower.starts_with("0x") || lower.starts_with("0X") {
            &lower[2..]
        } else {
            &lower
        };

        if no_prefix.is_empty() {
            return Err(ValidationError::EmptyHexCode);
        }

        // Validate that every character is a hexadecimal digit.
        for c in no_prefix.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(ValidationError::InvalidHexChar(c));
            }
        }

        // Require an even number of digits so each token expands to whole bytes.
        if no_prefix.len() % 2 != 0 {
            return Err(ValidationError::OddHexLength);
        }

        // Canonical form: lowercase with a `0x` prefix.
        words.push(format!("0x{}", no_prefix));
    }

    if words.is_empty() {
        return Err(ValidationError::EmptyHexCode);
    }

    Ok(words)
}

/// Parse a hexadecimal address, accepting optional `0x`/`0X` prefixes.
pub fn parse_address(input: &str) -> std::result::Result<u64, ValidationError> {
    if input.trim().is_empty() {
        return Err(ValidationError::EmptyAddress);
    }

    let trimmed = input.trim().to_lowercase();
    let hex_str = if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        &trimmed[2..]
    } else {
        &trimmed
    };

    if hex_str.is_empty() {
        return Err(ValidationError::InvalidAddressFormat);
    }

    // Ensure all characters are valid hexadecimal digits.
    for c in hex_str.chars() {
        if !c.is_ascii_hexdigit() {
            return Err(ValidationError::InvalidAddressFormat);
        }
    }

    u64::from_str_radix(hex_str, 16).map_err(|_| ValidationError::InvalidAddressFormat)
}

/// Expand canonical hex tokens into a contiguous byte buffer.
pub fn hex_words_to_bytes(words: &[String]) -> std::result::Result<Vec<u8>, ValidationError> {
    let mut out: Vec<u8> = Vec::new();
    for w in words {
        let lower = w.to_lowercase();
        let no_prefix = if lower.starts_with("0x") || lower.starts_with("0X") {
            &lower[2..]
        } else {
            &lower
        };

        if no_prefix.is_empty() {
            return Err(ValidationError::EmptyHexCode);
        }

        if no_prefix.len() % 2 != 0 {
            return Err(ValidationError::OddHexLength);
        }

        for i in (0..no_prefix.len()).step_by(2) {
            let byte = u8::from_str_radix(&no_prefix[i..i + 2], 16)
                .map_err(|_| ValidationError::InvalidHexChar(' '))?;
            out.push(byte);
        }
    }
    Ok(out)
}
