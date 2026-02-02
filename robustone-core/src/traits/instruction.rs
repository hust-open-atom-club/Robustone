//! Instruction detail traits.

use std::collections::HashMap;

/// Trait for architecture-specific instruction details.
///
/// All architecture-specific detail types should implement this trait.
pub trait Detail: std::fmt::Debug + Send + Sync {
    /// Returns the name of the architecture that produced this detail.
    fn architecture_name(&self) -> &'static str;

    /// Returns a list of register identifiers that are read by this instruction.
    fn registers_read(&self) -> &[u32];

    /// Returns a list of register identifiers that are written by this instruction.
    fn registers_written(&self) -> &[u32];
}

/// A generic implementation of `Detail` for simple use cases.
#[derive(Debug, Clone, Default)]
pub struct BasicInstructionDetail {
    pub architecture: &'static str,
    pub regs_read: Vec<u32>,
    pub regs_write: Vec<u32>,
    pub groups: Vec<String>,
    pub properties: HashMap<String, String>,
}

impl BasicInstructionDetail {
    pub fn new(architecture: &'static str) -> Self {
        Self {
            architecture,
            regs_read: Vec::new(),
            regs_write: Vec::new(),
            groups: Vec::new(),
            properties: HashMap::new(),
        }
    }

    pub fn reads_register(mut self, reg: u32) -> Self {
        self.regs_read.push(reg);
        self
    }

    pub fn writes_register(mut self, reg: u32) -> Self {
        self.regs_write.push(reg);
        self
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

impl Detail for BasicInstructionDetail {
    fn architecture_name(&self) -> &'static str {
        self.architecture
    }

    fn registers_read(&self) -> &[u32] {
        &self.regs_read
    }

    fn registers_written(&self) -> &[u32] {
        &self.regs_write
    }
}

/// Convenience macro for creating basic instruction details.
#[macro_export]
macro_rules! basic_detail {
    ($arch:expr) => {
        $crate::traits::instruction::BasicInstructionDetail::new($arch)
    };
    ($arch:expr, $($group:expr),+ $(,)?) => {{
        let mut detail = $crate::traits::instruction::BasicInstructionDetail::new($arch);
        $(detail = detail.with_group($group);)+
        detail
    }};
}
