//! LoongArch LA64 disassembly module for Robustone.
//!
//! Provides instruction decoding for LoongArch LA64 targets using the
//! shared `robustone-isa` framework.

pub mod arch;
pub mod backend;
pub mod printer;
pub mod render;
pub mod shared;
pub mod types;

pub mod architecture {
    pub use robustone_core::architecture::*;
}

pub mod common {
    pub use robustone_core::common::*;
}

pub mod ir {
    pub use robustone_core::ir::*;
}

pub mod utils {
    pub use robustone_core::utils::*;
}

pub mod loongarch {
    pub use crate::arch;
    pub use crate::printer;
    pub use crate::render;
    pub use crate::shared;
    pub use crate::types;
}

pub use robustone_core::Instruction;

use arch::LoongArchInstructionDetail;
use robustone_core::{
    common::ArchitectureProfile,
    ir::{DecodedInstruction, Operand, TextRenderProfile},
    traits::ArchitectureHandler,
    traits::instruction::Detail,
    types::error::DisasmError,
};
use robustone_isa::{AliasPolicy, DecodeProfile, FeatureSet, RenderDialect, decode_one};

/// Architecture handler implementation for LoongArch LA64 targets.
pub struct LoongArchHandler {
    detail: bool,
}

impl LoongArchHandler {
    /// Creates a new handler.
    pub fn new() -> Self {
        Self { detail: true }
    }
}

impl Default for LoongArchHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureHandler for LoongArchHandler {
    fn set_detail(&mut self, detail: bool) {
        self.detail = detail;
    }

    fn renderer(&self) -> Option<&dyn robustone_core::renderer::Renderer> {
        Some(&crate::render::LoongArchRenderer)
    }

    fn decode_instruction(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        if !self.supports(arch_name) {
            return Err(DisasmError::UnsupportedArchitecture(arch_name.to_string()));
        }

        let profile = DecodeProfile {
            mode: backend::LoongArchMode::LA64,
            features: backend::LoongArchFeature::all_supported_for_tests(),
            render_dialect: RenderDialect::Assembler,
            alias_policy: AliasPolicy::PreferPseudo,
        };
        let mut decoded = decode_one::<backend::LoongArchBackend>(bytes, addr, &profile)?;
        let size = decoded.size;

        // Alias: andi $zero, $zero, 0 => nop
        if decoded.mnemonic == "andi"
            && decoded.operands.len() == 3
            && let (
                Operand::Register { register: rd },
                Operand::Register { register: rj },
                Operand::Immediate { value: 0 },
            ) = (
                &decoded.operands[0],
                &decoded.operands[1],
                &decoded.operands[2],
            )
            && rd.id == 0
            && rj.id == 0
        {
            decoded.mnemonic = "nop".to_string();
            decoded.operands.clear();
        }
        // Alias: or $rd, $rj, $zero => move $rd, $rj
        if decoded.mnemonic == "or"
            && decoded.operands.len() == 3
            && let (
                Operand::Register { register: _rd },
                Operand::Register { register: _rj },
                Operand::Register { register: rk },
            ) = (
                &decoded.operands[0],
                &decoded.operands[1],
                &decoded.operands[2],
            )
            && rk.id == 0
        {
            decoded.mnemonic = "move".to_string();
            decoded.operands.pop();
        }

        // Upstream decoder drops duplicated destination register for CSR ops.
        if (decoded.mnemonic == "csrwr" || decoded.mnemonic == "gcsrwr")
            && decoded.operands.len() == 3
            && let (Operand::Register { register: r0 }, Operand::Register { register: r1 }, _) = (
                &decoded.operands[0],
                &decoded.operands[1],
                &decoded.operands[2],
            )
            && r0.id == r1.id
        {
            decoded.operands.remove(1);
        }
        if (decoded.mnemonic == "csrxchg" || decoded.mnemonic == "gcsrxchg")
            && decoded.operands.len() == 4
            && let (Operand::Register { register: r0 }, Operand::Register { register: r1 }, _, _) = (
                &decoded.operands[0],
                &decoded.operands[1],
                &decoded.operands[2],
                &decoded.operands[3],
            )
            && r0.id == r1.id
        {
            decoded.operands.remove(1);
        }

        // Upstream decoder reorders invtlb operands to imm, rj, rk.
        if decoded.mnemonic == "invtlb"
            && decoded.operands.len() == 3
            && matches!(decoded.operands[0], Operand::Register { .. })
        {
            let imm = decoded.operands.pop().unwrap();
            let rj = decoded.operands.pop().unwrap();
            let rk = decoded.operands.pop().unwrap();
            decoded.operands.push(imm);
            decoded.operands.push(rj);
            decoded.operands.push(rk);
        }

        // Upstream decoder drops the .xs suffix from certain float instructions.
        if let Some(base) = decoded.mnemonic.strip_suffix(".xs") {
            decoded.mnemonic = base.to_string();
        }

        // Upstream decoder omits duplicated destination register for certain
        // vector instructions (e.g. xvpermi.w, xvsrarni, xvinsve0, xvshuf).
        if decoded.mnemonic.starts_with("xv")
            && decoded.operands.len() == 4
            && let (Operand::Register { register: r0 }, Operand::Register { register: r1 }, _, _) = (
                &decoded.operands[0],
                &decoded.operands[1],
                &decoded.operands[2],
                &decoded.operands[3],
            )
            && r0.id == r1.id
        {
            decoded.operands.remove(1);
        }

        Ok((decoded, size))
    }

    fn decode_instruction_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(DecodedInstruction, usize), DisasmError> {
        self.decode_instruction(bytes, profile.mode_name, addr)
    }

    fn disassemble(
        &self,
        bytes: &[u8],
        arch_name: &str,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        let (decoded, size) = self.decode_instruction(bytes, arch_name, addr)?;
        let (mnemonic, operands) = render::render_loongarch_text_parts(
            &decoded,
            TextRenderProfile::Capstone,
            true,
            true,
            true,
            false,
        );

        let detail: Option<Box<dyn Detail>> = if self.detail {
            let mut la_detail = LoongArchInstructionDetail::new();
            for register in decoded
                .registers_read
                .iter()
                .chain(decoded.implicit_registers_read.iter())
            {
                if !la_detail.regs_read.contains(&register.id) {
                    la_detail = la_detail.reads_register(register.id);
                }
            }
            for register in decoded
                .registers_written
                .iter()
                .chain(decoded.implicit_registers_written.iter())
            {
                if !la_detail.regs_write.contains(&register.id) {
                    la_detail = la_detail.writes_register(register.id);
                }
            }
            Some(Box::new(la_detail))
        } else {
            None
        };

        let instruction = Instruction::from_decoded(decoded, mnemonic, operands, detail);
        Ok((instruction, size))
    }

    fn disassemble_with_profile(
        &self,
        bytes: &[u8],
        profile: &ArchitectureProfile,
        addr: u64,
    ) -> Result<(Instruction, usize), DisasmError> {
        self.disassemble(bytes, profile.mode_name, addr)
    }

    fn name(&self) -> &'static str {
        "loongarch"
    }

    fn supports(&self, arch_name: &str) -> bool {
        matches!(arch_name, "loongarch" | "loongarch64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LoongArchRegister;

    #[test]
    fn test_handler_creation() {
        let handler = LoongArchHandler::new();
        assert_eq!(handler.name(), "loongarch");
        assert!(handler.supports("loongarch64"));
        assert!(!handler.supports("loongarch32"));
        assert!(!handler.supports("riscv64"));
    }

    #[test]
    fn test_nop_decode() {
        let handler = LoongArchHandler::new();
        let (instr, size) = handler
            .disassemble(&[0x00, 0x00, 0x40, 0x03], "loongarch64", 0)
            .unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "nop");
    }

    #[test]
    fn test_addi_w_decode() {
        let handler = LoongArchHandler::new();
        let bytes = [0xe5, 0xd8, 0x83, 0x02];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "addi.w");
        assert_eq!(instr.operands, "$a1, $a3, 0xf6");
    }

    #[test]
    fn test_add_w_decode() {
        let handler = LoongArchHandler::new();
        let bytes = [0x29, 0x7c, 0x10, 0x00];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "add.w");
        assert_eq!(instr.operands, "$a5, $ra, $s8");
    }

    #[test]
    fn test_or_decode() {
        let handler = LoongArchHandler::new();
        let bytes = [0x11, 0x7a, 0x15, 0x00];
        let (instr, size) = handler.disassemble(&bytes, "loongarch64", 0).unwrap();
        assert_eq!(size, 4);
        assert_eq!(instr.mnemonic, "or");
        assert_eq!(instr.operands, "$t5, $t4, $s7");
    }

    #[test]
    fn test_register_names() {
        assert_eq!(LoongArchRegister::R0.name(), "$zero");
        assert_eq!(LoongArchRegister::R1.name(), "$ra");
        assert_eq!(LoongArchRegister::R3.name(), "$sp");
        assert_eq!(LoongArchRegister::R4.name(), "$a0");
        assert_eq!(LoongArchRegister::R11.name(), "$a7");
        assert_eq!(LoongArchRegister::R20.name(), "$t8");
        assert_eq!(LoongArchRegister::R21.name(), "$r21");
        assert_eq!(LoongArchRegister::R22.name(), "$fp");
        assert_eq!(LoongArchRegister::R31.name(), "$s8");
        assert_eq!(LoongArchRegister::F0.name(), "$fa0");
        assert_eq!(LoongArchRegister::F7.name(), "$fa7");
        assert_eq!(LoongArchRegister::F8.name(), "$ft0");
        assert_eq!(LoongArchRegister::F24.name(), "$fs0");
        assert_eq!(LoongArchRegister::F31.name(), "$fs7");
        assert_eq!(LoongArchRegister::Xr0.name(), "$xr0");
        assert_eq!(LoongArchRegister::Xr31.name(), "$xr31");
        assert_eq!(LoongArchRegister::Fcc0.name(), "$fcc0");
        assert_eq!(LoongArchRegister::Fcc7.name(), "$fcc7");
        assert_eq!(LoongArchRegister::Scr0.name(), "$scr0");
        assert_eq!(LoongArchRegister::Scr3.name(), "$scr3");
    }
}

#[cfg(test)]
mod debug_tests {
    use crate::LoongArchHandler;
    use robustone_core::ArchitectureHandler;

    #[test]
    fn test_addu12i_d() {
        let arch = LoongArchHandler::new();
        let bytes = [0xa4, 0x84, 0x29, 0x00];
        let result = arch.disassemble(&bytes, "loongarch64", 0);
        match result {
            Ok((insn, _)) => {
                println!("mnemonic: {}", insn.mnemonic);
                println!("operands: {}", insn.operands);
                if let Some(decoded) = insn.decoded {
                    for (i, op) in decoded.operands.iter().enumerate() {
                        println!("op {}: {:?}", i, op);
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

// Register the LoongArch handler with the global inventory.
inventory::submit! {
    robustone_core::traits::HandlerFactory::new(|| Box::new(LoongArchHandler::new()))
}
