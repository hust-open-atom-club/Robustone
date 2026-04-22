//! RISC-V architecture types and implementations.
//!
//! This module defines RISC-V specific types and extensions used by
//! the RISC-V instruction decoder and handler.

use robustone_core::traits::instruction::Detail;

/// Simple RISC-V instruction detail containing register access information.
#[derive(Debug, Clone)]
pub struct RiscVInstructionDetail {
    /// Registers read by this instruction
    pub regs_read: Vec<u32>,
    /// Registers written by this instruction
    pub regs_write: Vec<u32>,
}

impl Default for RiscVInstructionDetail {
    fn default() -> Self {
        Self::new()
    }
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

/// Re-export register type from the existing types module
pub use crate::types::RiscVRegister;

impl Detail for RiscVInstructionDetail {
    fn architecture_name(&self) -> &'static str {
        "riscv"
    }

    fn registers_read(&self) -> &[u32] {
        &self.regs_read
    }

    fn registers_written(&self) -> &[u32] {
        &self.regs_write
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::extensions::standard::Standard;

    #[test]
    fn test_riscv_standard_extensions() {
        let i = Standard::I;
        assert!(i.contains(Standard::I));
        assert!(!i.contains(Standard::M));
        assert!(!i.contains(Standard::F));

        let gc = Standard::G | Standard::C;
        assert!(gc.contains(Standard::I));
        assert!(gc.contains(Standard::M));
        assert!(gc.contains(Standard::A));
        assert!(gc.contains(Standard::F));
        assert!(gc.contains(Standard::D));
        assert!(gc.contains(Standard::C));
        assert_eq!(gc, Standard::G | Standard::C);

        let combined = Standard::I | Standard::M;
        assert!(combined.contains(Standard::I));
        assert!(combined.contains(Standard::M));
        assert!(!combined.contains(Standard::F));
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
