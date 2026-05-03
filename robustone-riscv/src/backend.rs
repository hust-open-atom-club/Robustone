#![forbid(unsafe_code)]

//! Minimal RISC-V backend using the shared ISA framework.
//!
//! This is a shadow backend that demonstrates the framework for RISC-V.
//! Full migration (replacing the legacy extension-based decoder) is tracked
//! under AC-9.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec, RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xlen {
    X32,
    X64,
}

robustone_isa_macros::define_arch! {
    pub arch RiscV {
        word = u32;
        endian = little;
        instruction_length = variable(4);
        modes {
            RV32 = "riscv32";
            RV64 = "riscv64";
        };
        extern_features;
        features: u16 {
            I = 0; M = 1; A = 2; F = 3; D = 4; C = 5; THEAD = 6;
        };
        registers = riscv_registers;
        formats = riscv_formats;
        specs = riscv_specs;
        render = RiscVRenderPolicy;
        backend_impl {
            field = RiscVField;
            register_class = RiscVRegisterClass;
            architecture_id = ArchitectureId::Riscv;
            read_instruction = riscv_read_instruction;
            lookup = riscv_lookup;
            lower_register = riscv_lower_register;
            render_policy = riscv_render_policy;
            extract_field = riscv_extract_field;
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RiscVFeature: u16 {
        const I = 1 << 0;
        const M = 1 << 1;
        const A = 1 << 2;
        const F = 1 << 3;
        const D = 1 << 4;
        const C = 1 << 5;
        const THEAD = 1 << 6;
        const G = Self::I.bits() | Self::M.bits() | Self::A.bits() | Self::F.bits() | Self::D.bits();
        const CF = Self::C.bits() | Self::F.bits();
    }
}

impl FeatureSet for RiscVFeature {
    fn empty() -> Self {
        Self::empty()
    }
    fn all_supported_for_tests() -> Self {
        Self::G | Self::C | Self::THEAD
    }
    fn contains(self, required: Self) -> bool {
        self.bits() & required.bits() == required.bits()
    }
}

impl RiscVFeature {
    /// Build feature set from an extension name list (e.g. ["I", "M", "A", "F", "D", "C"]).
    pub fn from_extension_names(names: &[&str]) -> Self {
        let mut features = Self::empty();
        for name in names {
            match *name {
                "I" => features |= Self::I,
                "M" => features |= Self::M,
                "A" => features |= Self::A,
                "F" => features |= Self::F,
                "D" => features |= Self::D,
                "C" => features |= Self::C,
                "G" => features |= Self::G,
                "THEAD" => features |= Self::THEAD,
                _ => {}
            }
        }
        features
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVField {
    Rd,
    Rs1,
    Rs2,
    Funct3,
    Funct7,
    Opcode,
    Imm12,
    Imm12S,
    Imm20U,
    // Compressed fields
    Rs2C,     // rs2 in CR format (bits 2-6)
    RdPrime,  // rd' in CA/CL format (bits 2-4, actual reg = +8)
    Rs2Prime, // rs2' in CA format (bits 2-4, actual reg = +8)
    Rs1Prime, // rs1' in CL format (bits 7-9, actual reg = +8)
    Imm6,     // 6-bit CI immediate (bit12 << 5 | bits2-6)
    ImmCL,    // CL-format immediate for c.ld: {bits[6:5], bits[12:10]} << 3
    ImmCLW,   // CL-format immediate for c.flw: {bit[5], bits[12:10], bit[6], 0}
}

robustone_isa_macros::define_registers! {
    arch = RiscV;
    bank Gpr      { count = 32; base_id = 0;  prefix = "x"; }
    bank Fpr      { count = 32; base_id = 32; prefix = "f"; }
    bank GprPrime { count = 8;  base_id = 8; }
    bank FprPrime { count = 8;  base_id = 40; }
}

robustone_isa_macros::define_formats! {
    arch = RiscV; extern_fields;
    fields {
        Rd; Rs1; Rs2; Funct3; Funct7;
        Imm12; Imm12S; Imm20U;
        Rs2C; RdPrime; Rs2Prime; Rs1Prime; Imm6; ImmCL; ImmCLW;
    };
    format R_TYPE {
        rd: bits(7, 5) as Rd,
        rs1: bits(15, 5) as Rs1,
        rs2: bits(20, 5) as Rs2,
        funct3: bits(12, 3) as Funct3,
        funct7: bits(25, 7) as Funct7,
    };
    format I_TYPE {
        rd: bits(7, 5) as Rd,
        rs1: bits(15, 5) as Rs1,
        imm12: bits(20, 12) as Imm12,
        funct3: bits(12, 3) as Funct3,
    };
    format S_TYPE {
        rs1: bits(15, 5) as Rs1,
        rs2: bits(20, 5) as Rs2,
        imm12s: bits(0, 12) as Imm12S,
        funct3: bits(12, 3) as Funct3,
    };
    format B_TYPE {
        rs1: bits(15, 5) as Rs1,
        rs2: bits(20, 5) as Rs2,
        funct3: bits(12, 3) as Funct3,
    };
    format U_TYPE {
        rd: bits(7, 5) as Rd,
        imm20u: bits(0, 20) as Imm20U,
    };
    format J_TYPE {
        rd: bits(7, 5) as Rd,
    };
    format CI_TYPE {
        rd: bits(7, 5) as Rd,
        imm6: bits(2, 6) as Imm6,
    };
    format CR_TYPE {
        rs1: bits(7, 5) as Rs1,
        rs2: bits(2, 5) as Rs2C,
    };
    format CA_TYPE {
        rd_prime: bits(7, 3) as RdPrime,
        rs2_prime: bits(2, 3) as Rs2Prime,
    };
    format CL_TYPE {
        rd_prime: bits(2, 3) as RdPrime,
        rs1_prime: bits(7, 3) as Rs1Prime,
        imm_cl: bits(5, 5) as ImmCL,
        imm_clw: bits(5, 5) as ImmCLW,
    };
    format CS_TYPE {
        rs1_prime: bits(7, 3) as Rs1Prime,
        rs2_prime: bits(2, 3) as Rs2Prime,
        imm_csd: bits(5, 5) as ImmCL,
        imm_csw: bits(5, 5) as ImmCLW,
    }
}

pub mod specs_i {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_i.rs");
}
pub mod specs_m {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_m.rs");
}
pub mod specs_a {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_a.rs");
}
pub mod specs_f {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_f.rs");
}
pub mod specs_d {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_d.rs");
}
pub mod specs_c {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_c.rs");
}
pub mod specs_system {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_system.rs");
}
pub mod specs_thead {
    use super::*;
    use robustone_isa::ModeSet;
    include!("backend/specs_thead.rs");
}

static ALL_SPEC_SLICES: &[&[InstructionSpec<RiscVBackend>]] = &[
    specs_i::SPECS,
    specs_m::SPECS,
    specs_a::SPECS,
    specs_f::SPECS,
    specs_d::SPECS,
    specs_c::SPECS,
    specs_system::SPECS,
    specs_thead::SPECS,
];

fn all_riscv_specs() -> impl Iterator<Item = &'static InstructionSpec<RiscVBackend>> {
    ALL_SPEC_SLICES.iter().flat_map(|s| s.iter())
}

fn riscv_read_instruction(bytes: &[u8]) -> Result<InstructionRead<u32>, DisasmError> {
    if bytes.len() >= 2 && (bytes[0] & 0x3) != 0x3 {
        let word = (bytes[0] as u32) | ((bytes[1] as u32) << 8);
        Ok(InstructionRead {
            raw: word,
            length: 2,
        })
    } else if bytes.len() >= 4 {
        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(InstructionRead {
            raw: word,
            length: 4,
        })
    } else {
        Err(DisasmError::decode_failure(
            DecodeErrorKind::NeedMoreBytes,
            Some("riscv".to_string()),
            "incomplete instruction",
        ))
    }
}

fn riscv_lookup(
    word: u32,
    profile: &DecodeProfile<RiscVBackend>,
) -> Option<&'static InstructionSpec<RiscVBackend>> {
    let is_compressed = (word & 0x3) != 0x3;
    let exact = all_riscv_specs().find(|spec| {
        (word & spec.pattern().mask) == spec.pattern().value
            && spec.modes().matches(profile.mode)
            && profile.features.contains(spec.features())
    });
    if exact.is_some() {
        return exact;
    }
    if is_compressed {
        let mode_match = all_riscv_specs().find(|spec| {
            (word & spec.pattern().mask) == spec.pattern().value
                && spec.modes().matches(profile.mode)
        });
        if mode_match.is_some() {
            return None;
        }
        return all_riscv_specs().find(|spec| {
            (word & spec.pattern().mask) == spec.pattern().value
                && profile.features.contains(spec.features())
        });
    }
    all_riscv_specs().find(|spec| (word & spec.pattern().mask) == spec.pattern().value)
}

fn riscv_lower_register(
    class: RiscVRegisterClass,
    raw: u32,
    _profile: &DecodeProfile<RiscVBackend>,
) -> RegisterId {
    match lower_register(class, raw) {
        Ok(id) => RegisterId::riscv(id),
        Err(_) => RegisterId::riscv(raw),
    }
}

fn riscv_render_policy(_profile: &DecodeProfile<RiscVBackend>) -> RenderPolicy<RiscVBackend> {
    RenderPolicy::new(
        robustone_isa::RenderDialect::Canonical,
        robustone_isa::AliasPolicy::None,
    )
}

fn riscv_extract_field(
    word: u32,
    format: &FormatSpec<RiscVField>,
    field: RiscVField,
) -> Result<u32, DisasmError> {
    match field {
        RiscVField::Imm12S => {
            let imm115 = (word >> 25) & 0x7F;
            let imm40 = (word >> 7) & 0x1F;
            Ok((imm115 << 5) | imm40)
        }
        RiscVField::Imm20U => Ok((word >> 12) & 0xFFFFF),
        RiscVField::Imm6 => {
            let high = (word >> 12) & 1;
            let low = (word >> 2) & 0x1F;
            Ok((high << 5) | low)
        }
        RiscVField::ImmCL => {
            let low = (word >> 5) & 0x3;
            let high = (word >> 8) & 0x1C;
            Ok((low | high) << 3)
        }
        RiscVField::ImmCLW => {
            let bit5 = (word >> 5) & 1;
            let bits12_10 = (word >> 10) & 0x7;
            let bit6 = (word >> 6) & 1;
            Ok((bit5 << 6) | (bits12_10 << 3) | (bit6 << 2))
        }
        _ => {
            for f in format.fields() {
                if f.field_type() == field {
                    let mask = ((1u64 << f.length()) - 1) as u32;
                    return Ok((word >> f.start()) & mask);
                }
            }
            Err(DisasmError::decode_failure(
                DecodeErrorKind::InvalidField,
                Some("riscv".to_string()),
                format!("field {:?} not found in format {}", field, format.name()),
            ))
        }
    }
}

pub type RiscVIsaDecoder = robustone_isa::Decoder<RiscVBackend>;
