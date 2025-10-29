//! Endianness handling utilities for multi-architecture support.
//!
//! This module provides utilities for handling different byte orderings
//! across various architectures, making it easier to support both little-endian
//! and big-endian systems in a generic way.

/// Endianness enumeration for byte ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endianness {
    /// Little-endian byte order (least significant byte first)
    Little,
    /// Big-endian byte order (most significant byte first)
    Big,
}

impl Endianness {
    /// Gets the default endianness for a given architecture.
    ///
    /// # Arguments
    ///
    /// * `arch_name` - The architecture name (e.g., "riscv32", "arm", "x86")
    ///
    /// # Returns
    ///
    /// Returns the default endianness for the specified architecture.
    pub fn for_architecture(arch_name: &str) -> Self {
        match arch_name.to_lowercase().as_str() {
            // RISC-V architectures are typically little-endian
            arch if arch.starts_with("riscv") => Endianness::Little,

            // ARM can be either, but ARMv7 and later are typically little-endian
            // AArch64 is little-endian by default
            arch if arch.starts_with("arm") || arch.starts_with("aarch64") => Endianness::Little,

            // x86/x64 are little-endian
            arch if arch.starts_with("x86") || arch.starts_with("x64") => Endianness::Little,

            // MIPS can be either, but we'll default to little-endian
            arch if arch.starts_with("mips") => Endianness::Little,

            // PowerPC is typically big-endian (though little-endian variants exist)
            arch if arch.starts_with("ppc") || arch.starts_with("powerpc") => Endianness::Big,

            // SPARC is big-endian
            arch if arch.starts_with("sparc") => Endianness::Big,

            // SystemZ is big-endian
            arch if arch.starts_with("systemz") => Endianness::Big,

            // Default to little-endian for unknown architectures
            _ => Endianness::Little,
        }
    }

    /// Applies the endianness to a byte slice.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte slice to reorder
    ///
    /// # Returns
    ///
    /// Returns a new Vec<u8> with the appropriate byte order applied.
    pub fn apply_to_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        match self {
            Endianness::Little => bytes.to_vec(),
            Endianness::Big => {
                let mut reversed = bytes.to_vec();
                reversed.reverse();
                reversed
            }
        }
    }

    /// Applies endianness to a 16-bit value.
    pub fn apply_to_u16(&self, value: u16) -> u16 {
        match self {
            Endianness::Little => value,
            Endianness::Big => value.swap_bytes(),
        }
    }

    /// Applies endianness to a 32-bit value.
    pub fn apply_to_u32(&self, value: u32) -> u32 {
        match self {
            Endianness::Little => value,
            Endianness::Big => value.swap_bytes(),
        }
    }

    /// Applies endianness to a 64-bit value.
    pub fn apply_to_u64(&self, value: u64) -> u64 {
        match self {
            Endianness::Little => value,
            Endianness::Big => value.swap_bytes(),
        }
    }

    /// Reads bytes as a 16-bit value with the specified endianness.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Byte slice containing at least 2 bytes
    ///
    /// # Returns
    ///
    /// Returns the 16-bit value read with the appropriate byte order.
    pub fn read_u16(&self, bytes: &[u8]) -> u16 {
        if bytes.len() < 2 {
            return 0;
        }

        match self {
            Endianness::Little => u16::from_le_bytes([bytes[0], bytes[1]]),
            Endianness::Big => u16::from_be_bytes([bytes[0], bytes[1]]),
        }
    }

    /// Reads bytes as a 32-bit value with the specified endianness.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Byte slice containing at least 4 bytes
    ///
    /// # Returns
    ///
    /// Returns the 32-bit value read with the appropriate byte order.
    pub fn read_u32(&self, bytes: &[u8]) -> u32 {
        if bytes.len() < 4 {
            return 0;
        }

        match self {
            Endianness::Little => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            Endianness::Big => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        }
    }

    /// Reads bytes as a 64-bit value with the specified endianness.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Byte slice containing at least 8 bytes
    ///
    /// # Returns
    ///
    /// Returns the 64-bit value read with the appropriate byte order.
    pub fn read_u64(&self, bytes: &[u8]) -> u64 {
        if bytes.len() < 8 {
            return 0;
        }

        match self {
            Endianness::Little => u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            Endianness::Big => u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
        }
    }
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Little
    }
}

/// Trait for types that can be converted between different endiannesses.
pub trait EndianConvert {
    /// Converts the value to the specified endianness.
    fn to_endian(self, endianness: Endianness) -> Self;

    /// Converts the value from the specified endianness to native endianness.
    fn from_endian(value: Self, endianness: Endianness) -> Self;
}

impl EndianConvert for u16 {
    fn to_endian(self, endianness: Endianness) -> Self {
        endianness.apply_to_u16(self)
    }

    fn from_endian(value: Self, endianness: Endianness) -> Self {
        endianness.apply_to_u16(value)
    }
}

impl EndianConvert for u32 {
    fn to_endian(self, endianness: Endianness) -> Self {
        endianness.apply_to_u32(self)
    }

    fn from_endian(value: Self, endianness: Endianness) -> Self {
        endianness.apply_to_u32(value)
    }
}

impl EndianConvert for u64 {
    fn to_endian(self, endianness: Endianness) -> Self {
        endianness.apply_to_u64(self)
    }

    fn from_endian(value: Self, endianness: Endianness) -> Self {
        endianness.apply_to_u64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_endianness() {
        assert_eq!(Endianness::for_architecture("riscv32"), Endianness::Little);
        assert_eq!(Endianness::for_architecture("riscv64"), Endianness::Little);
        assert_eq!(Endianness::for_architecture("x86"), Endianness::Little);
        assert_eq!(Endianness::for_architecture("arm"), Endianness::Little);
        assert_eq!(Endianness::for_architecture("ppc"), Endianness::Big);
        assert_eq!(Endianness::for_architecture("sparc"), Endianness::Big);
    }

    #[test]
    fn test_apply_to_bytes() {
        let bytes = [0x12, 0x34, 0x56, 0x78];

        let little_result = Endianness::Little.apply_to_bytes(&bytes);
        assert_eq!(little_result, vec![0x12, 0x34, 0x56, 0x78]);

        let big_result = Endianness::Big.apply_to_bytes(&bytes);
        assert_eq!(big_result, vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_apply_to_values() {
        let value = 0x12345678u32;

        assert_eq!(Endianness::Little.apply_to_u32(value), 0x12345678);
        assert_eq!(Endianness::Big.apply_to_u32(value), 0x78563412);
    }

    #[test]
    fn test_read_values() {
        let bytes = [0x12, 0x34, 0x56, 0x78];

        assert_eq!(Endianness::Little.read_u16(&bytes), 0x3412);
        assert_eq!(Endianness::Big.read_u16(&bytes), 0x1234);

        assert_eq!(Endianness::Little.read_u32(&bytes), 0x78563412);
        assert_eq!(Endianness::Big.read_u32(&bytes), 0x12345678);
    }

    #[test]
    fn test_endian_convert_trait() {
        let value = 0x12345678u32;

        let little_converted = value.to_endian(Endianness::Little);
        assert_eq!(little_converted, 0x12345678);

        let big_converted = value.to_endian(Endianness::Big);
        assert_eq!(big_converted, 0x78563412);

        let converted_back = u32::from_endian(big_converted, Endianness::Big);
        assert_eq!(converted_back, 0x12345678);
    }
}