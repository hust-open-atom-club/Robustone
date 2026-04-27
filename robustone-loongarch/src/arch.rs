//! LoongArch architecture types and implementations.
//!
//! This module defines LoongArch-specific types and detail structures used by
//! the instruction decoder and handler.

use robustone_core::traits::instruction::Detail;

/// Simple LoongArch instruction detail containing register access information.
#[derive(Debug, Clone)]
pub struct LoongArchInstructionDetail {
    /// Registers read by this instruction.
    pub regs_read: Vec<u32>,
    /// Registers written by this instruction.
    pub regs_write: Vec<u32>,
}

impl Default for LoongArchInstructionDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl LoongArchInstructionDetail {
    /// Creates a new LoongArch instruction detail.
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

impl Detail for LoongArchInstructionDetail {
    fn architecture_name(&self) -> &'static str {
        "loongarch"
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

    #[test]
    fn test_loongarch_instruction_detail() {
        let detail = LoongArchInstructionDetail::new()
            .reads_register(4)
            .writes_register(5);

        assert_eq!(detail.regs_read, vec![4]);
        assert_eq!(detail.regs_write, vec![5]);
    }

    #[test]
    fn test_detail_trait() {
        let detail = LoongArchInstructionDetail::new();
        assert_eq!(detail.architecture_name(), "loongarch");
        assert!(detail.registers_read().is_empty());
        assert!(detail.registers_written().is_empty());
    }
}
