//! RISC-V architecture types and implementations.
//!
//! This module defines RISC-V specific types and extensions used by
//! the RISC-V instruction decoder and handler.


/// RISC-V extensions bit mask type.
///
/// This type represents the set of enabled RISC-V extensions
/// using bit flags for efficient checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RiscVExtensions(u32);

impl std::ops::BitOr for RiscVExtensions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl RiscVExtensions {
    /// Creates a new extensions mask with no extensions enabled.
    pub fn none() -> Self {
        Self(0)
    }

    /// Creates extensions mask with the base I extension.
    pub fn i() -> Self {
        Self(1 << 0)
    }

    /// Creates extensions mask with the M extension (multiply/divide).
    pub fn m() -> Self {
        Self(1 << 1)
    }

    /// Creates extensions mask with the A extension (atomic).
    pub fn a() -> Self {
        Self(1 << 2)
    }

    /// Creates extensions mask with the F extension (single-precision float).
    pub fn f() -> Self {
        Self(1 << 3)
    }

    /// Creates extensions mask with the D extension (double-precision float).
    pub fn d() -> Self {
        Self(1 << 4)
    }

    /// Creates extensions mask with the C extension (compressed).
    pub fn c() -> Self {
        Self(1 << 5)
    }

    /// Creates extensions mask with all standard extensions (GC).
    pub fn gc() -> Self {
        Self(RiscVExtensions::i().0 | RiscVExtensions::m().0 | RiscVExtensions::a().0 | RiscVExtensions::f().0 | RiscVExtensions::d().0 | RiscVExtensions::c().0)
    }

    /// Checks if the I extension is enabled.
    pub fn has_i(self) -> bool {
        self.0 & RiscVExtensions::i().0 != 0
    }

    /// Checks if the M extension is enabled.
    pub fn has_m(self) -> bool {
        self.0 & RiscVExtensions::m().0 != 0
    }

    /// Checks if the A extension is enabled.
    pub fn has_a(self) -> bool {
        self.0 & RiscVExtensions::a().0 != 0
    }

    /// Checks if the F extension is enabled.
    pub fn has_f(self) -> bool {
        self.0 & RiscVExtensions::f().0 != 0
    }

    /// Checks if the D extension is enabled.
    pub fn has_d(self) -> bool {
        self.0 & RiscVExtensions::d().0 != 0
    }

    /// Checks if the C extension is enabled.
    pub fn has_c(self) -> bool {
        self.0 & RiscVExtensions::c().0 != 0
    }

    /// Checks if all standard extensions are enabled (GC).
    pub fn is_gc(self) -> bool {
        self == Self::gc()
    }

    /// Returns the raw bit mask value.
    pub fn bits(self) -> u32 {
        self.0
    }

    /// Creates extensions from a raw bit mask.
    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}



/// Simple RISC-V instruction detail containing register access information.
#[derive(Debug, Clone)]
pub struct RiscVInstructionDetail {
    /// Registers read by this instruction
    pub regs_read: Vec<u32>,
    /// Registers written by this instruction
    pub regs_write: Vec<u32>,
}

impl RiscVInstructionDetail {
    /// Creates a new RISC-V instruction detail.
    pub fn new() -> Self {
        Self {
            regs_read: Vec::new(),
            regs_write: Vec::new(),
        }
    }

    /// Adds a register to the read list.
    pub fn reads_register(mut self, reg: u32) -> Self {
        self.regs_read.push(reg);
        self
    }

    /// Adds a register to the write list.
    pub fn writes_register(mut self, reg: u32) -> Self {
        self.regs_write.push(reg);
        self
    }
}

// Re-export register type from the existing types module
pub use crate::riscv::types::RiscVRegister;

impl crate::instruction::InstructionDetail for RiscVInstructionDetail {
    fn architecture_name(&self) -> &'static str {
        "riscv"
    }

    fn registers_read(&self) -> Vec<u32> {
        self.regs_read.clone()
    }

    fn registers_written(&self) -> Vec<u32> {
        self.regs_write.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_riscv_extensions() {
        let i = RiscVExtensions::i();
        assert!(i.has_i());
        assert!(!i.has_m());
        assert!(!i.has_f());

        let gc = RiscVExtensions::gc();
        assert!(gc.has_i());
        assert!(gc.has_m());
        assert!(gc.has_a());
        assert!(gc.has_f());
        assert!(gc.has_d());
        assert!(gc.has_c());
        assert!(gc.is_gc());

        let combined = RiscVExtensions::i() | RiscVExtensions::m();
        assert!(combined.has_i());
        assert!(combined.has_m());
        assert!(!combined.has_f());
    }

    #[test]
    fn test_riscv_instruction_detail() {
        let detail = RiscVInstructionDetail::new()
            .reads_register(5)
            .writes_register(10);

        assert_eq!(detail.regs_read, vec![5]);
        assert_eq!(detail.regs_write, vec![10]);
    }
}