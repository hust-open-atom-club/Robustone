//! Text formatting helpers for AArch64 operand rendering.

/// Format an immediate value for display.
/// Small non-negative values are shown as decimal; others as hex.
pub fn format_imm(value: i64) -> String {
    if (0..10).contains(&value) {
        value.to_string()
    } else {
        format!("0x{value:x}")
    }
}

/// Format a shift suffix: `, lsl #12`
pub fn format_shift(shift_type: &str, amount: u8) -> String {
    format!(", {shift_type} #{amount}")
}

/// Format an extend suffix: `, sxtw #3`
pub fn format_extend(extend_type: &str, amount: u8) -> String {
    if amount == 0 {
        format!(", {extend_type}")
    } else {
        format!(", {extend_type} #{amount}")
    }
}

/// Format a memory operand: `[x0, #0x8]` or `[x0, x1, lsl #3]`
pub fn format_mem(base: &str, index: Option<&str>, displacement: i64) -> String {
    match index {
        Some(idx) => format!("[{base}, {idx}]"),
        None => {
            if displacement == 0 {
                format!("[{base}]")
            } else {
                format!("[{base}, #{}]", format_imm(displacement))
            }
        }
    }
}

/// Format a condition code suffix for reference compatibility.
pub fn format_cond(cond: &str) -> String {
    format!(".{cond}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_imm() {
        assert_eq!(format_imm(5), "5");
        assert_eq!(format_imm(10), "0xa");
        assert_eq!(format_imm(-1), "0xffffffffffffffff");
    }

    #[test]
    fn test_format_shift() {
        assert_eq!(format_shift("lsl", 12), ", lsl #12");
    }

    #[test]
    fn test_format_mem() {
        assert_eq!(format_mem("x0", None, 0), "[x0]");
        assert_eq!(format_mem("x0", None, 8), "[x0, #8]");
        assert_eq!(format_mem("x0", Some("x1"), 0), "[x0, x1]");
    }
}
