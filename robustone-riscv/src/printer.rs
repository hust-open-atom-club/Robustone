//! RISC-V instruction formatting helpers.
//!
//! Inspired by Capstone's printer to maintain compatible output formatting.

use super::shared::operands::csr_name_lookup;
use super::shared::{OperandFormatter, operands::DefaultOperandFactory};
use super::types::*;
use robustone_core::Instruction;
use robustone_core::ir::{DecodedInstruction, Operand, RegisterId, TextRenderProfile};

/// Text formatting profiles for the RISC-V formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVTextProfile {
    Capstone,
    Canonical,
    VerboseDebug,
}

/// Pretty-printer for RISC-V instructions.
pub struct RiscVPrinter {
    /// Whether register aliases should be printed instead of canonical names.
    alias_regs: bool,
    /// Whether alias register behavior was explicitly chosen by the caller.
    alias_regs_explicit: bool,
    /// Whether Capstone-facing aliases should be emitted instead of canonical mnemonics.
    capstone_aliases: bool,
    /// Whether compressed instruction aliases should be emitted.
    compressed_aliases: bool,
    /// Whether immediates should be rendered as unsigned values when possible.
    unsigned_immediate: bool,
    /// Selected rendering profile.
    profile: RiscVTextProfile,
}

impl RiscVPrinter {
    fn text_render_profile(&self) -> TextRenderProfile {
        match self.profile {
            RiscVTextProfile::Capstone => TextRenderProfile::Capstone,
            RiscVTextProfile::Canonical => TextRenderProfile::Canonical,
            RiscVTextProfile::VerboseDebug => TextRenderProfile::VerboseDebug,
        }
    }

    /// Creates a printer with default formatting behaviour.
    pub fn new() -> Self {
        Self {
            alias_regs: true,
            alias_regs_explicit: false,
            capstone_aliases: true,
            compressed_aliases: true,
            unsigned_immediate: false,
            profile: RiscVTextProfile::Capstone,
        }
    }

    /// Enables or disables register alias printing.
    pub fn with_alias_regs(mut self, alias_regs: bool) -> Self {
        self.alias_regs = alias_regs;
        self.alias_regs_explicit = true;
        self
    }

    pub fn with_capstone_aliases(mut self, capstone_aliases: bool) -> Self {
        self.capstone_aliases = capstone_aliases;
        self
    }

    pub fn with_compressed_aliases(mut self, compressed_aliases: bool) -> Self {
        self.compressed_aliases = compressed_aliases;
        self
    }

    /// Enables or disables unsigned immediate formatting.
    pub fn with_unsigned_immediate(mut self, unsigned_immediate: bool) -> Self {
        self.unsigned_immediate = unsigned_immediate;
        self
    }

    /// Select the text rendering profile.
    pub fn with_profile(mut self, profile: RiscVTextProfile) -> Self {
        self.profile = profile;
        if !self.alias_regs_explicit {
            self.alias_regs = matches!(
                profile,
                RiscVTextProfile::Capstone | RiscVTextProfile::VerboseDebug
            );
        }
        self.capstone_aliases = !matches!(profile, RiscVTextProfile::Canonical);
        self.compressed_aliases = self.capstone_aliases;
        self
    }

    /// Formats an immediate according to the active configuration.
    fn format_immediate(&self, imm: i64) -> String {
        if self.unsigned_immediate && imm < 0 {
            return format!("0x{:x}", imm as u64);
        }
        DefaultOperandFactory::new().format_immediate(imm)
    }

    fn format_mode_unsigned_immediate(&self, imm: i64, mode: &str) -> String {
        match mode {
            "riscv32" => format!("0x{:x}", imm as u32),
            _ => format!("0x{:x}", imm as u64),
        }
    }

    /// Formats a register operand.
    fn format_register(&self, reg_id: u32) -> String {
        let reg = RiscVRegister::from_id(reg_id);
        if self.alias_regs {
            reg.name().to_string()
        } else {
            // Use stable architectural register names when aliases are disabled.
            if reg_id <= 31 {
                format!("x{reg_id}")
            } else if (32..=63).contains(&reg_id) {
                format!("f{}", reg_id - 32)
            } else {
                reg.name().to_string()
            }
        }
    }

    /// Formats a memory operand using `offset(base)` syntax.
    fn format_memory_operand(&self, base: u32, disp: i64) -> String {
        DefaultOperandFactory::new().format_memory_operand(disp, &self.format_register(base))
    }

    /// Formats a single operand into its textual form.
    pub fn format_operand(&self, operand: &RiscVOperand) -> String {
        match &operand.value {
            RiscVOperandValue::Register(reg_id) => self.format_register(*reg_id),
            RiscVOperandValue::Immediate(imm) => self.format_immediate(*imm),
            RiscVOperandValue::RoundingMode(rm) => rounding_mode_name(*rm).to_string(),
            RiscVOperandValue::Memory(mem) => self.format_memory_operand(mem.base, mem.disp),
        }
    }

    /// Formats a sequence of operands into a comma-separated string.
    pub fn format_operands(&self, operands: &[RiscVOperand]) -> String {
        if operands.is_empty() {
            String::new()
        } else {
            operands
                .iter()
                .map(|op| self.format_operand(op))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// Render the shared IR into mnemonic and operand text.
    pub fn render_ir_parts(&self, ir: &DecodedInstruction) -> (String, String) {
        let use_capstone_aliases =
            self.capstone_aliases && (self.compressed_aliases || !ir.mnemonic.starts_with("c."));
        let mnemonic = match self.profile {
            RiscVTextProfile::Capstone | RiscVTextProfile::VerboseDebug if use_capstone_aliases => {
                ir.render_hints
                    .capstone_mnemonic
                    .clone()
                    .unwrap_or_else(|| ir.mnemonic.clone())
            }
            _ => ir.mnemonic.clone(),
        };

        let hidden_operands = if matches!(
            self.profile,
            RiscVTextProfile::Capstone | RiscVTextProfile::VerboseDebug
        ) && use_capstone_aliases
        {
            ir.render_hints.capstone_hidden_operands.as_slice()
        } else {
            &[]
        };
        let visible_operands = ir
            .operands
            .iter()
            .enumerate()
            .filter(|(index, _)| !hidden_operands.contains(index))
            .collect::<Vec<_>>();
        let last_visible_index = visible_operands.last().map(|(index, _)| *index);

        let operands = if mnemonic == "jalr" {
            self.format_ir_jalr_operands(&visible_operands, ir.mode.as_str())
        } else if mnemonic.starts_with("lr.") {
            self.format_ir_load_reserved_operands(&visible_operands, ir.mode.as_str())
        } else if mnemonic.starts_with("sc.") || mnemonic.starts_with("amo") {
            self.format_ir_atomic_operands(&visible_operands, ir.mode.as_str())
        } else {
            visible_operands
                .iter()
                .map(|(index, operand)| {
                    self.format_ir_operand(
                        &mnemonic,
                        *index,
                        operand,
                        ir.mode.as_str(),
                        last_visible_index,
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        (mnemonic, operands)
    }

    fn format_ir_register(&self, register: &RegisterId) -> String {
        self.format_register(register.id)
    }

    fn format_ir_basic_operand(&self, operand: &Operand, mode: &str) -> String {
        match operand {
            Operand::Register { register } => self.format_ir_register(register),
            Operand::Immediate { value } => {
                if self.unsigned_immediate && *value < 0 {
                    self.format_mode_unsigned_immediate(*value, mode)
                } else {
                    self.format_immediate(*value)
                }
            }
            Operand::Text { value } => value.clone(),
            Operand::Memory { base, displacement } => base
                .as_ref()
                .map(|base| {
                    let disp = if self.unsigned_immediate && *displacement < 0 {
                        self.format_mode_unsigned_immediate(*displacement, mode)
                    } else {
                        self.format_immediate(*displacement)
                    };
                    format!("{disp}({})", self.format_ir_register(base))
                })
                .unwrap_or_else(|| self.format_immediate(*displacement)),
        }
    }

    fn format_ir_operand(
        &self,
        mnemonic: &str,
        index: usize,
        operand: &Operand,
        mode: &str,
        last_visible_index: Option<usize>,
    ) -> String {
        match operand {
            Operand::Immediate { value } if self.is_csr_operand(mnemonic, index) => {
                csr_name_lookup(*value as u16)
                    .map(str::to_string)
                    .unwrap_or_else(|| self.format_ir_basic_operand(operand, mode))
            }
            Operand::Immediate { value }
                if last_visible_index == Some(index) && self.is_control_flow_mnemonic(mnemonic) =>
            {
                if self.unsigned_immediate {
                    self.format_mode_unsigned_immediate(*value, mode)
                } else {
                    self.format_control_flow_immediate(*value, mode)
                }
            }
            _ => self.format_ir_basic_operand(operand, mode),
        }
    }

    fn format_ir_jalr_operands(&self, operands: &[(usize, &Operand)], mode: &str) -> String {
        let mut visible = operands.iter().map(|(_, operand)| *operand);
        match (visible.next(), visible.next(), visible.next()) {
            (
                Some(Operand::Register { register: rd }),
                Some(Operand::Register { register: rs1 }),
                Some(Operand::Immediate { value: imm }),
            ) => {
                let disp = if self.unsigned_immediate && *imm < 0 {
                    self.format_mode_unsigned_immediate(*imm, mode)
                } else {
                    self.format_immediate(*imm)
                };
                let target = format!("{disp}({})", self.format_ir_register(rs1));
                format!("{}, {target}", self.format_ir_register(rd))
            }
            (
                Some(Operand::Register { register: rs1 }),
                Some(Operand::Immediate { value: imm }),
                None,
            ) => {
                let disp = if self.unsigned_immediate && *imm < 0 {
                    self.format_mode_unsigned_immediate(*imm, mode)
                } else {
                    self.format_immediate(*imm)
                };
                format!("{disp}({})", self.format_ir_register(rs1))
            }
            _ => operands
                .iter()
                .map(|(_, operand)| self.format_ir_basic_operand(operand, mode))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    fn format_ir_load_reserved_operands(
        &self,
        operands: &[(usize, &Operand)],
        mode: &str,
    ) -> String {
        match operands {
            [
                (_, Operand::Register { register: rd }),
                (
                    _,
                    Operand::Memory {
                        base: Some(base),
                        displacement,
                    },
                ),
            ] => {
                if *displacement == 0 {
                    format!(
                        "{}, ({})",
                        self.format_ir_register(rd),
                        self.format_ir_register(base)
                    )
                } else {
                    format!(
                        "{}, {}({})",
                        self.format_ir_register(rd),
                        self.format_immediate(*displacement),
                        self.format_ir_register(base)
                    )
                }
            }
            _ => operands
                .iter()
                .map(|(_, operand)| self.format_ir_basic_operand(operand, mode))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    fn format_ir_atomic_operands(&self, operands: &[(usize, &Operand)], mode: &str) -> String {
        match operands {
            [
                (_, Operand::Register { register: first }),
                (
                    _,
                    Operand::Memory {
                        base: Some(base),
                        displacement,
                    },
                ),
                (_, Operand::Register { register: second }),
            ] => {
                if *displacement == 0 {
                    format!(
                        "{}, {}, ({})",
                        self.format_ir_register(first),
                        self.format_ir_register(second),
                        self.format_ir_register(base)
                    )
                } else {
                    format!(
                        "{}, {}, {}({})",
                        self.format_ir_register(first),
                        self.format_ir_register(second),
                        self.format_immediate(*displacement),
                        self.format_ir_register(base)
                    )
                }
            }
            _ => operands
                .iter()
                .map(|(_, operand)| self.format_ir_basic_operand(operand, mode))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    fn is_csr_operand(&self, mnemonic: &str, index: usize) -> bool {
        let csr_mnemonics = [
            "csrrw", "csrrs", "csrrc", "csrrwi", "csrrsi", "csrrci", "csrr", "csrc", "csrw",
        ];
        csr_mnemonics.contains(&mnemonic) && index == 1
    }

    fn is_control_flow_mnemonic(&self, mnemonic: &str) -> bool {
        matches!(
            mnemonic,
            "j" | "jal"
                | "jalr"
                | "beq"
                | "bne"
                | "blt"
                | "bge"
                | "bltu"
                | "bgeu"
                | "beqz"
                | "bnez"
                | "c.j"
                | "c.jal"
                | "c.beqz"
                | "c.bnez"
        )
    }

    fn format_control_flow_immediate(&self, value: i64, mode: &str) -> String {
        if value >= 0 {
            return DefaultOperandFactory::new().format_immediate(value);
        }

        match mode {
            "riscv64" | "riscv" => format!("0x{:x}", value as u64),
            _ => format!("0x{:x}", value as u32),
        }
    }

    /// Renders the instruction mnemonic and operand list.
    pub fn print_basic(&self, instruction: &Instruction) -> String {
        let (mnemonic, operands) = instruction
            .decoded
            .as_ref()
            .map(|decoded| self.render_ir_parts(decoded))
            .unwrap_or_else(|| instruction.rendered_text_parts(self.text_render_profile()));
        if operands.is_empty() {
            mnemonic
        } else {
            format!("{mnemonic} {operands}")
        }
    }

    /// Renders the detailed instruction representation including metadata.
    pub fn print_detailed(&self, instruction: &Instruction) -> String {
        let mut result = Vec::new();

        // Basic summary line.
        result.push(format!(
            "0x{:016x}: {} {}",
            instruction.address,
            hex::encode(&instruction.bytes),
            self.print_basic(instruction)
        ));

        // Emit detailed sections when available.
        if let Some(detail) = &instruction.detail {
            // Register access lists.
            let regs_read = detail.registers_read();
            if !regs_read.is_empty() {
                let regs_read: Vec<String> = regs_read
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters read: {}", regs_read.join(", ")));
            }

            let regs_write = detail.registers_written();
            if !regs_write.is_empty() {
                let regs_write: Vec<String> = regs_write
                    .iter()
                    .map(|&reg| format!("{} ({})", self.format_register(reg), reg))
                    .collect();
                result.push(format!("\tRegisters modified: {}", regs_write.join(", ")));
            }
        }

        result.join("\n")
    }

    /// Formats instruction bytes as a hex string and pads to the requested width.
    pub fn print_hex_bytes(&self, instruction: &Instruction, align_width: usize) -> String {
        let hex_str = hex::encode(&instruction.bytes);
        let padding = if hex_str.len() < align_width {
            " ".repeat(align_width - hex_str.len())
        } else {
            String::new()
        };
        format!("{hex_str}{padding}")
    }
}

impl Default for RiscVPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common RISC-V printing scenarios.
pub mod format {
    use super::*;

    /// Returns the basic printable form for a single instruction.
    pub fn basic_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_basic(instruction)
    }

    /// Returns the fully detailed printable form for a single instruction.
    pub fn detailed_format(instruction: &Instruction) -> String {
        let printer = RiscVPrinter::new();
        printer.print_detailed(instruction)
    }

    /// Renders a list of instructions using either basic or detailed mode.
    pub fn instruction_list(instructions: &[Instruction], detailed: bool) -> String {
        let printer = RiscVPrinter::new();
        let mut result = Vec::new();

        for instruction in instructions {
            if detailed {
                result.push(printer.print_detailed(instruction));
            } else {
                result.push(format!(
                    "0x{:016x}: {} {}",
                    instruction.address,
                    printer.print_hex_bytes(instruction, 16),
                    printer.print_basic(instruction)
                ));
            }
        }

        result.join("\n")
    }

    /// Formats a list of operands using default printer settings.
    pub fn operands_list(operands: &[RiscVOperand]) -> String {
        let printer = RiscVPrinter::new();
        printer.format_operands(operands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ArchitectureId, DecodeStatus, Operand, RegisterId, RenderHints};
    use crate::riscv::decoder::RiscVDecoder;

    #[test]
    fn test_printer_creation() {
        let printer = RiscVPrinter::new();
        assert!(printer.alias_regs);
        assert!(!printer.unsigned_immediate);

        let printer = RiscVPrinter::new()
            .with_alias_regs(true)
            .with_unsigned_immediate(true);
        assert!(printer.alias_regs);
        assert!(printer.unsigned_immediate);
    }

    #[test]
    fn test_format_immediate() {
        let printer = RiscVPrinter::new();

        // Positive values
        assert_eq!(printer.format_immediate(10), "0xa");
        assert_eq!(printer.format_immediate(0x1000), "0x1000");

        // Negative values
        assert_eq!(printer.format_immediate(-10), "-0xa");
        // assert_eq!(printer.format_immediate(-0x1000), "-0x1000");

        // Zero
        assert_eq!(printer.format_immediate(0), "0");
    }

    #[test]
    fn test_format_register() {
        let printer = RiscVPrinter::new().with_alias_regs(false);

        // Canonical register formatting
        assert_eq!(printer.format_register(0), "x0");
        assert_eq!(printer.format_register(1), "x1");
        assert_eq!(printer.format_register(10), "x10");
        assert_eq!(printer.format_register(37), "f5");

        // Alias-based formatting
        let printer_with_alias = printer.with_alias_regs(true);
        assert_eq!(printer_with_alias.format_register(0), "zero");
        assert_eq!(printer_with_alias.format_register(1), "ra");
        assert_eq!(printer_with_alias.format_register(10), "a0");
        assert_eq!(printer_with_alias.format_register(37), "ft5");
    }

    #[test]
    fn test_format_memory_operand() {
        let printer = RiscVPrinter::new().with_alias_regs(true);

        // Base register only
        assert_eq!(printer.format_memory_operand(2, 0), "0(sp)");

        // Positive offset
        assert_eq!(printer.format_memory_operand(10, 100), "0x64(a0)");

        // Negative offset
        assert_eq!(printer.format_memory_operand(10, -100), "-0x64(a0)");
    }

    #[test]
    fn test_format_operand() {
        let printer = RiscVPrinter::new().with_alias_regs(true);

        // Register operand
        let reg_op = RiscVOperand {
            op_type: RiscVOperandType::Register,
            access: Access::read(),
            value: RiscVOperandValue::Register(10),
        };
        assert_eq!(printer.format_operand(&reg_op), "a0");

        // Immediate operand
        let imm_op = RiscVOperand {
            op_type: RiscVOperandType::Immediate,
            access: Access::read(),
            value: RiscVOperandValue::Immediate(42),
        };
        assert_eq!(printer.format_operand(&imm_op), "0x2a");

        // Memory operand
        let mem_op = RiscVOperand {
            op_type: RiscVOperandType::Memory,
            access: Access::read(),
            value: RiscVOperandValue::Memory(RiscVMemoryOperand { base: 2, disp: 100 }),
        };
        assert_eq!(printer.format_operand(&mem_op), "0x64(sp)");
    }

    #[test]
    fn test_canonical_profile_renders_full_operands() {
        let decoder = RiscVDecoder::rv32gc();
        let decoded = decoder
            .decode(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0)
            .unwrap();
        let printer = RiscVPrinter::new().with_profile(RiscVTextProfile::Canonical);
        let (mnemonic, operands) = printer.render_ir_parts(&decoded);

        assert_eq!(mnemonic, "addi");
        assert_eq!(operands, "x1, x0, 1");
    }

    #[test]
    fn test_canonical_profile_renders_fp_registers_without_aliases() {
        let decoder = RiscVDecoder::rv64gc();
        let decoded = decoder
            .decode(&[0xd3, 0x02, 0x73, 0x00], "riscv64", 0)
            .unwrap();
        let printer = RiscVPrinter::new().with_profile(RiscVTextProfile::Canonical);
        let (mnemonic, operands) = printer.render_ir_parts(&decoded);

        assert_eq!(mnemonic, "fadd.s");
        assert_eq!(operands, "f5, f6, f7, rne");
    }

    #[test]
    fn test_render_ir_parts_uses_shared_ir() {
        let printer = RiscVPrinter::new().with_profile(RiscVTextProfile::Capstone);
        let decoded = DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: "riscv32".to_string(),
            mnemonic: "addi".to_string(),
            opcode_id: Some("addi".to_string()),
            size: 4,
            raw_bytes: vec![0x93, 0x00, 0x10, 0x00],
            operands: vec![
                Operand::Register {
                    register: RegisterId::riscv(1),
                },
                Operand::Register {
                    register: RegisterId::riscv(0),
                },
                Operand::Immediate { value: 1 },
            ],
            registers_read: vec![RegisterId::riscv(0)],
            registers_written: vec![RegisterId::riscv(1)],
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: vec!["arithmetic".to_string()],
            status: DecodeStatus::Success,
            render_hints: RenderHints {
                capstone_mnemonic: Some("li".to_string()),
                capstone_hidden_operands: vec![1],
            },
        };

        let (mnemonic, operands) = printer.render_ir_parts(&decoded);
        assert_eq!(mnemonic, "li");
        assert_eq!(operands, "ra, 1");
    }

    #[test]
    fn test_print_basic_honors_canonical_profile() {
        let decoder = RiscVDecoder::rv32gc();
        let decoded = decoder
            .decode(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0)
            .unwrap();
        let instruction =
            Instruction::from_decoded(decoded, "li".to_string(), "ra, 1".to_string(), None);
        let printer = RiscVPrinter::new().with_profile(RiscVTextProfile::Canonical);

        assert_eq!(printer.print_basic(&instruction), "addi x1, x0, 1");
    }

    #[test]
    fn test_default_printer_keeps_capstone_aliases_for_decoded_instructions() {
        let decoder = RiscVDecoder::rv32gc();
        let decoded = decoder
            .decode(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0)
            .unwrap();
        let instruction =
            Instruction::from_decoded(decoded, "li".to_string(), "ra, 1".to_string(), None);

        assert_eq!(RiscVPrinter::new().print_basic(&instruction), "li ra, 1");
    }

    #[test]
    fn test_with_alias_regs_false_is_honored_for_decoded_instructions() {
        let decoder = RiscVDecoder::rv32gc();
        let decoded = decoder
            .decode(&[0x93, 0x00, 0x10, 0x00], "riscv32", 0)
            .unwrap();
        let instruction =
            Instruction::from_decoded(decoded, "li".to_string(), "ra, 1".to_string(), None);
        let printer = RiscVPrinter::new().with_alias_regs(false);

        assert_eq!(printer.print_basic(&instruction), "li x1, 1");
    }

    #[test]
    fn test_print_basic_honors_unsigned_immediate_setting() {
        let decoded = DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: "riscv32".to_string(),
            mnemonic: "addi".to_string(),
            opcode_id: Some("addi".to_string()),
            size: 4,
            raw_bytes: vec![0x13, 0x01, 0x01, 0xff],
            operands: vec![
                Operand::Register {
                    register: RegisterId::riscv(2),
                },
                Operand::Register {
                    register: RegisterId::riscv(2),
                },
                Operand::Immediate { value: -16 },
            ],
            registers_read: vec![RegisterId::riscv(2)],
            registers_written: vec![RegisterId::riscv(2)],
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: vec!["arithmetic".to_string()],
            status: DecodeStatus::Success,
            render_hints: RenderHints {
                capstone_mnemonic: None,
                capstone_hidden_operands: Vec::new(),
            },
        };
        let instruction = Instruction::from_decoded(
            decoded,
            "addi".to_string(),
            "sp, sp, -0x10".to_string(),
            None,
        );
        let printer = RiscVPrinter::new()
            .with_profile(RiscVTextProfile::Canonical)
            .with_unsigned_immediate(true);

        assert_eq!(printer.print_basic(&instruction), "addi x2, x2, 0xfffffff0");
    }

    #[test]
    fn test_print_basic_legacy_instruction_uses_selected_profile_path() {
        let instruction = Instruction::new(
            0,
            vec![0x13, 0x01, 0x01, 0xff],
            "addi".to_string(),
            "x2, x2, -16".to_string(),
        );
        let printer = RiscVPrinter::new().with_profile(RiscVTextProfile::Canonical);

        assert_eq!(printer.print_basic(&instruction), "addi x2, x2, -16");
    }
}
