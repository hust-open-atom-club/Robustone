use crate::cli::error::ValidationError;

/// 将输入解析为“十六进制字符串”列表，例如：
/// 输入："0x00000000 0x00000011" => 输出：vec!["0x00000000", "0x00000011"]
/// 规则：
/// - 以空白分隔多个 token；
/// - 每个 token 可带 0x/0X 前缀；
/// - 仅允许十六进制字符；
/// - 必须为偶数字符长度（不含前缀），否则报错；
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

        // 处理前缀并统一为小写
        let lower = raw.to_lowercase();
        let no_prefix = if lower.starts_with("0x") || lower.starts_with("0X") {
            &lower[2..]
        } else {
            &lower
        };

        if no_prefix.is_empty() {
            return Err(ValidationError::EmptyHexCode);
        }

        // 校验字符
        for c in no_prefix.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(ValidationError::InvalidHexChar(c));
            }
        }

        // 必须偶数字符长度
        if no_prefix.len() % 2 != 0 {
            return Err(ValidationError::OddHexLength);
        }

        // 规范化：保留 0x 前缀，小写
        words.push(format!("0x{}", no_prefix));
    }

    if words.is_empty() {
        return Err(ValidationError::EmptyHexCode);
    }

    Ok(words)
}

/// 解析十六进制地址（允许带0x/0X前缀）
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

    // 验证所有字符都是有效的十六进制字符
    for c in hex_str.chars() {
        if !c.is_ascii_hexdigit() {
            return Err(ValidationError::InvalidAddressFormat);
        }
    }

    u64::from_str_radix(hex_str, 16).map_err(|_| ValidationError::InvalidAddressFormat)
}

/// 将 parse_hex_code 产生的十六进制字符串列表展开为字节序列
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
