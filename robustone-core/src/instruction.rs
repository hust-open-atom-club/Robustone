//! Core data structures shared by decoded instructions.
//!
//! This module provides both legacy instruction structures for compatibility
//! and new architecture-agnostic abstractions for future multi-architecture support.

use std::collections::HashMap;

/// Decoded instruction returned by the disassembler.
///
/// This is the legacy instruction structure that maintains backward compatibility
/// with existing code while providing the essential information needed for
/// most disassembly use cases.
#[derive(Debug)]
pub struct Instruction {
    /// The memory address where this instruction would be located.
    ///
    /// This is useful for calculating relative addresses, jump targets,
    /// and for displaying addresses in disassembly output.
    pub address: u64,

    /// The raw bytes that make up this instruction, in execution order.
    ///
    /// This allows consumers to work with the binary representation
    /// of the instruction if needed.
    pub bytes: Vec<u8>,

    /// The instruction mnemonic (operation name).
    ///
    /// Examples: "add", "mov", "jmp", "call", etc.
    /// This should be architecture-standardized where possible.
    pub mnemonic: String,

    /// Formatted operand string as commonly seen in assembly output.
    ///
    /// This provides a human-readable representation of the operands
    /// in the format that developers would expect to see in assembly
    /// listings or debuggers.
    ///
    /// Examples: "rax, rbx", "[rsp + 8]", "0x1000", etc.
    pub operands: String,

    /// The size of this instruction in bytes.
    ///
    /// This is particularly important for variable-length instruction
    /// sets like x86, but is useful for all architectures for
    /// calculating instruction boundaries.
    pub size: usize,

    /// Optional architecture-specific detail information.
    ///
    /// This field allows architectures to provide additional
    /// structured information beyond the basic text representation.
    /// The content is architecture-specific and should be handled
    /// with appropriate type checking or pattern matching.
    pub detail: Option<Box<dyn InstructionDetail>>,
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            bytes: self.bytes.clone(),
            mnemonic: self.mnemonic.clone(),
            operands: self.operands.clone(),
            size: self.size,
            detail: self.detail.as_ref().map(|_d| {
                // For now, we'll use BasicInstructionDetail as a fallback for cloning
                // This maintains compatibility while avoiding the complex clone trait
                Box::new(BasicInstructionDetail::new("cloned")) as Box<dyn InstructionDetail>
            }),
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            address: 0,
            bytes: Vec::new(),
            mnemonic: "unknown".to_string(),
            operands: String::new(),
            size: 0,
            detail: None,
        }
    }
}

impl Instruction {
    /// Creates a new instruction with the provided basic information.
    ///
    /// This is a convenience constructor for the common case where
    /// only basic instruction information is available.
    ///
    /// # Arguments
    ///
    /// * `address` - Memory address of the instruction
    /// * `bytes` - Raw instruction bytes
    /// * `mnemonic` - Instruction mnemonic
    /// * `operands` - Formatted operand string
    ///
    /// # Returns
    ///
    /// A new `Instruction` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::prelude::*;
    /// let instruction = Instruction::new(
    ///     0x1000,
    ///     vec![0x48, 0x89, 0xd8], // mov rax, rbx
    ///     "mov".to_string(),
    ///     "rax, rbx".to_string(),
    /// );
    /// ```
    pub fn new(address: u64, bytes: Vec<u8>, mnemonic: String, operands: String) -> Self {
        let size = bytes.len();
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: None,
        }
    }

    /// Creates a new instruction with architecture-specific details.
    ///
    /// This constructor is used by architecture handlers that can
    /// provide structured information about the instruction.
    ///
    /// # Arguments
    ///
    /// * `address` - Memory address of the instruction
    /// * `bytes` - Raw instruction bytes
    /// * `mnemonic` - Instruction mnemonic
    /// * `operands` - Formatted operand string
    /// * `detail` - Architecture-specific detail information
    ///
    /// # Returns
    ///
    /// A new `Instruction` instance with detailed information.
    pub fn with_detail(
        address: u64,
        bytes: Vec<u8>,
        mnemonic: String,
        operands: String,
        detail: Box<dyn InstructionDetail>,
    ) -> Self {
        let size = bytes.len();
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: Some(detail),
        }
    }

    /// Creates a new instruction with basic detail information.
    ///
    /// This is a convenience constructor for architectures that want to
    /// provide basic detail information without creating a custom detail type.
    ///
    /// # Arguments
    ///
    /// * `address` - Memory address of the instruction
    /// * `bytes` - Raw instruction bytes
    /// * `mnemonic` - Instruction mnemonic
    /// * `operands` - Formatted operand string
    /// * `architecture` - Architecture name
    ///
    /// # Returns
    ///
    /// A new `Instruction` instance with basic detail information.
    pub fn with_basic_detail(
        address: u64,
        bytes: Vec<u8>,
        mnemonic: String,
        operands: String,
        architecture: &'static str,
    ) -> Self {
        let size = bytes.len();
        let detail = BasicInstructionDetail::new(architecture);
        Self {
            address,
            bytes,
            mnemonic,
            operands,
            size,
            detail: Some(Box::new(detail)),
        }
    }

    /// Creates an unknown instruction placeholder.
    ///
    /// This is useful when decoding fails but you still need to
    /// represent the bytes in the instruction stream.
    ///
    /// # Arguments
    ///
    /// * `address` - Memory address
    /// * `bytes` - Raw bytes that couldn't be decoded
    ///
    /// # Returns
    ///
    /// An `Instruction` representing unknown data.
    pub fn unknown(address: u64, bytes: Vec<u8>) -> Self {
        let size = bytes.len();
        let hex_repr = format!("0x{}", hex::encode(&bytes));
        Self {
            address,
            bytes,
            mnemonic: "unknown".to_string(),
            operands: hex_repr,
            size,
            detail: None,
        }
    }

    /// Returns true if this instruction represents an unknown/invalid instruction.
    pub fn is_unknown(&self) -> bool {
        self.mnemonic == "unknown"
    }

    /// Gets the instruction as a formatted assembly line.
    ///
    /// This is a convenience method that formats the instruction
    /// in the classic "address: mnemonic    operands" format.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the instruction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use robustone_core::prelude::*;
    /// let instruction = Instruction::new(0x1000, vec![0x48, 0x89, 0xD8], "mov".to_string(), "rax, rbx".to_string());
    /// let formatted = instruction.assembly_line();
    /// // Might return: "0x1000: mov    rax, rbx"
    /// ```
    pub fn assembly_line(&self) -> String {
        format!(
            "0x{:08x}: {:<7} {}",
            self.address, self.mnemonic, self.operands
        )
    }
}

/// Trait for architecture-specific instruction details.
///
/// This trait allows different architectures to provide structured
/// information about their instructions while maintaining a common
/// interface. All architecture-specific detail types should implement
/// this trait.
pub trait InstructionDetail: std::fmt::Debug + Send + Sync {
    /// Returns the name of the architecture that produced this detail.
    fn architecture_name(&self) -> &'static str;

    /// Returns a list of register identifiers that are read by this instruction.
    fn registers_read(&self) -> &[u32];

    /// Returns a list of register identifiers that are written by this instruction.
    fn registers_written(&self) -> &[u32];
}

/// A generic implementation of `InstructionDetail` for simple use cases.
///
/// This can be used by architectures that don't need complex
/// structured details but still want to provide basic information.
#[derive(Debug, Clone, Default)]
pub struct BasicInstructionDetail {
    /// Name of the architecture
    pub architecture: &'static str,
    /// Registers read by the instruction
    pub regs_read: Vec<u32>,
    /// Registers written by the instruction
    pub regs_write: Vec<u32>,
    /// Semantic group tags
    pub groups: Vec<String>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl BasicInstructionDetail {
    /// Creates a new basic instruction detail.
    ///
    /// # Arguments
    ///
    /// * `architecture` - Name of the architecture
    ///
    /// # Returns
    ///
    /// A new `BasicInstructionDetail` instance.
    pub fn new(architecture: &'static str) -> Self {
        Self {
            architecture,
            regs_read: Vec::new(),
            regs_write: Vec::new(),
            groups: Vec::new(),
            properties: HashMap::new(),
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

    /// Adds a group tag.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// Adds a property.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

impl InstructionDetail for BasicInstructionDetail {
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
///
/// This macro provides a fluent interface for creating `BasicInstructionDetail`
/// instances with common patterns.
///
/// # Example
///
/// ```rust
/// use robustone_core::prelude::*;
/// use robustone_core::instruction::BasicInstructionDetail;
/// use robustone_core::basic_detail;
/// let detail = basic_detail!("riscv")
///     .reads_register(5)   // t0
///     .writes_register(10) // a0
///     .with_group("arithmetic")
///     .with_property("extension", "M");
/// ```
#[macro_export]
macro_rules! basic_detail {
    ($arch:expr) => {
        BasicInstructionDetail::new($arch)
    };
    ($arch:expr, $($group:expr),+ $(,)?) => {{
        let mut detail = BasicInstructionDetail::new($arch);
        $(detail = detail.with_group($group);)+
        detail
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        let instruction = Instruction::new(
            0x1000,
            vec![0x90, 0x90], // nop nop
            "nop".to_string(),
            String::new(),
        );

        assert_eq!(instruction.address, 0x1000);
        assert_eq!(instruction.mnemonic, "nop");
        assert_eq!(instruction.size, 2);
        assert!(instruction.detail.is_none());
    }

    #[test]
    fn test_instruction_with_detail() {
        let detail = BasicInstructionDetail::new("test")
            .reads_register(1)
            .writes_register(2)
            .with_group("test_group");

        let instruction = Instruction::with_detail(
            0x1000,
            vec![0x01, 0x02, 0x03, 0x04],
            "test".to_string(),
            "r1, r2".to_string(),
            Box::new(detail),
        );

        assert_eq!(instruction.mnemonic, "test");
        assert!(instruction.detail.is_some());
    }

    #[test]
    fn test_unknown_instruction() {
        let instruction = Instruction::unknown(0x1000, vec![0xFF, 0xFF]);

        assert!(instruction.is_unknown());
        assert_eq!(instruction.mnemonic, "unknown");
        assert_eq!(instruction.operands, "0xffff");
    }

    #[test]
    fn test_assembly_line_formatting() {
        let instruction = Instruction::new(
            0x100,
            vec![0x48, 0x89, 0xD8],
            "mov".to_string(),
            "rax, rbx".to_string(),
        );

        let formatted = instruction.assembly_line();
        assert_eq!(formatted, "0x00000100: mov     rax, rbx");
    }

    #[test]
    fn test_basic_instruction_detail() {
        let detail = BasicInstructionDetail::new("test_arch")
            .reads_register(5)
            .writes_register(10)
            .with_group("arithmetic")
            .with_property("width", "32");

        assert_eq!(detail.architecture_name(), "test_arch");
        assert_eq!(detail.registers_read(), vec![5]);
        assert_eq!(detail.registers_written(), vec![10]);
        assert_eq!(detail.groups, vec!["arithmetic"]);
        assert_eq!(detail.properties.get("width"), Some(&"32".to_string()));
    }

    #[test]
    fn test_basic_detail_macro() {
        let detail = basic_detail!("test_arch", "memory", "load")
            .reads_register(1)
            .writes_register(2);

        assert_eq!(detail.architecture_name(), "test_arch");
        assert_eq!(detail.groups, vec!["memory", "load"]);
    }
}
