//! AArch64 instruction detail.

use robustone_core::traits::instruction::Detail;

/// Architecture-specific detail for AArch64 instructions.
#[derive(Debug, Clone, Default)]
pub struct ArmInstructionDetail {
    pub regs_read: Vec<u32>,
    pub regs_write: Vec<u32>,
}

impl ArmInstructionDetail {
    pub fn new() -> Self {
        Self {
            regs_read: Vec::new(),
            regs_write: Vec::new(),
        }
    }

    pub fn reads_register(mut self, reg: u32) -> Self {
        if !self.regs_read.contains(&reg) {
            self.regs_read.push(reg);
        }
        self
    }

    pub fn writes_register(mut self, reg: u32) -> Self {
        if !self.regs_write.contains(&reg) {
            self.regs_write.push(reg);
        }
        self
    }
}

impl Detail for ArmInstructionDetail {
    fn architecture_name(&self) -> &'static str {
        "aarch64"
    }

    fn registers_read(&self) -> &[u32] {
        &self.regs_read
    }

    fn registers_written(&self) -> &[u32] {
        &self.regs_write
    }
}
