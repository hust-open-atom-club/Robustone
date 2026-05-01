//! LoongArch backend implementing the `robustone-isa` `ArchitectureBackend` trait.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::{DecodeErrorKind, DisasmError};
use robustone_isa::{
    Access, ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, ImmediateKind,
    ImmediateTransform, InstructionGroup, InstructionRead, InstructionSpec, RenderPolicy, field,
};

// ============================================================================
// LoongArch types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchMode {
    LA32,
    LA64,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoongArchFeature: u16 {
        const BASE       = 1 << 0;
        const LA64       = 1 << 1;
        const FLOAT32    = 1 << 2;
        const FLOAT64    = 1 << 3;
        const PRIVILEGED = 1 << 4;
        const ATOMIC     = 1 << 5;
        const LSX        = 1 << 6;
        const LASX       = 1 << 7;
        const LVZ        = 1 << 8;
        const LBT        = 1 << 9;
    }
}

impl FeatureSet for LoongArchFeature {
    fn empty() -> Self {
        LoongArchFeature::empty()
    }

    fn all_supported_for_tests() -> Self {
        LoongArchFeature::all()
    }

    fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchField {
    Rd,
    Rj,
    Rk,
    Ra,
    Cd,
    I8,
    Si12,
    Ui12,
    Si14,
    Si16,
    Si20,
    Si21,
    Ui5,
    Ui6,
    MsbW,
    LsbW,
    MsbD,
    LsbD,
    I26Lo,
    I26Hi,
    Code,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoongArchRegisterClass {
    Gpr,
    Fpr,
    Xr,
    Fcc,
    Scr,
    Fcsr,
}

// ============================================================================
// Format specs
// ============================================================================

robustone_isa::format_specs! {
    format FMT_R2[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
    }
    format FMT_R3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
    }
    format FMT_R3I2[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        sa2: field("sa2", 15, 2, LoongArchField::Ui5),
    }
    format FMT_R3I3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        ui3: field("ui3", 15, 3, LoongArchField::Ui5),
    }
    format FMT_R4[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
        ra: field("ra", 15, 5, LoongArchField::Ra),
    }
    format FMT_FCMP[LoongArchField] {
        cd: field("cd", 0, 3, LoongArchField::Cd),
        fj: field("fj", 5, 5, LoongArchField::Rj),
        fk: field("fk", 10, 5, LoongArchField::Rk),
    }
    format FMT_CSR[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        csr: field("csr", 10, 14, LoongArchField::Si14),
    }
    format FMT_CSRXCHG[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        csr: field("csr", 10, 14, LoongArchField::Si14),
    }
    format FMT_INVTLB[LoongArchField] {
        imm: field("imm", 0, 5, LoongArchField::Ui5),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        rk: field("rk", 10, 5, LoongArchField::Rk),
    }
    format FMT_R2I8[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        i8: field("i8", 10, 8, LoongArchField::I8),
    }
    format FMT_R2I12[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si12: field("si12", 10, 12, LoongArchField::Si12),
    }
    format FMT_R2I14[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si14: field("si14", 10, 14, LoongArchField::Si14),
    }
    format FMT_R2I16[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_1RI21[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        si21: field("si21", 5, 21, LoongArchField::Si21),
    }
    format FMT_I26[LoongArchField] {
        i26_lo: field("i26_lo", 0, 16, LoongArchField::I26Lo),
        i26_hi: field("i26_hi", 16, 10, LoongArchField::I26Hi),
    }
    format FMT_R1[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
    }
    format FMT_NONE[LoongArchField] {}
    format FMT_BJ[LoongArchField] {
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_R1I16_BJ[LoongArchField] {
        rj: field("rj", 5, 5, LoongArchField::Rj),
        si16: field("si16", 10, 16, LoongArchField::Si16),
    }
    format FMT_I15[LoongArchField] {
        code: field("code", 0, 15, LoongArchField::Code),
    }
    format FMT_R2I5[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui5: field("ui5", 10, 5, LoongArchField::Ui5),
    }
    format FMT_R2I6[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui6: field("ui6", 10, 6, LoongArchField::Ui6),
    }
    format FMT_R2I3[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui3: field("ui3", 10, 3, LoongArchField::Ui5),
    }
    format FMT_R2I4[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui4: field("ui4", 10, 4, LoongArchField::Ui5),
    }
    format FMT_BSTR_W[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        msb: field("msb", 16, 5, LoongArchField::MsbW),
        lsb: field("lsb", 10, 5, LoongArchField::LsbW),
    }
    format FMT_BSTR_D[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        msb: field("msb", 16, 6, LoongArchField::MsbD),
        lsb: field("lsb", 10, 6, LoongArchField::LsbD),
    }
    format FMT_1RI20[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        si20: field("si20", 5, 20, LoongArchField::Si20),
    }
    format FMT_R2I12_U[LoongArchField] {
        rd: field("rd", 0, 5, LoongArchField::Rd),
        rj: field("rj", 5, 5, LoongArchField::Rj),
        ui12: field("ui12", 10, 12, LoongArchField::Ui12),
    }
    format FMT_LDPTE[LoongArchField] {
        rj: field("rj", 5, 5, LoongArchField::Rj),
        seq: field("seq", 10, 8, LoongArchField::I8),
    }
}

// ============================================================================

include!("backend/specs_integer.rs");
include!("backend/specs_float_arith.rs");
include!("backend/specs_float_cmp.rs");
include!("backend/specs_float_mem.rs");
include!("backend/specs_system.rs");

pub static LOONGARCH_BASE_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    BSTRINS_W,
    BSTRPICK_W,
    BSTRINS_D,
    BSTRPICK_D,
    ADDU16I_D,
    JIRL,
    B,
    BL,
    BEQZ,
    BNEZ,
    PRELD,
    SLL_W,
    SRL_W,
    SRA_W,
    SLL_D,
    SRL_D,
    SRA_D,
    ROTR_B,
    ROTR_H,
    ROTR_W,
    ROTR_D,
    ADC_B,
    ADC_H,
    ADC_W,
    ADC_D,
    SBC_B,
    SBC_H,
    SBC_W,
    SBC_D,
    RCR_B,
    RCR_H,
    RCR_W,
    RCR_D,
    SLLI_W,
    SLLI_D,
    SRLI_W,
    SRLI_D,
    SRAI_W,
    SRAI_D,
    ROTRI_B,
    ROTRI_H,
    ROTRI_W,
    ROTRI_D,
    RCRI_B,
    RCRI_H,
    RCRI_W,
    RCRI_D,
    ADD_W,
    ADD_D,
    SUB_W,
    SUB_D,
    SLT,
    SLTU,
    MASKEQZ,
    MASKNEZ,
    AND,
    OR,
    XOR,
    NOR,
    ORN,
    ANDN,
    CLO_W,
    CLZ_W,
    CTO_W,
    CTZ_W,
    CLO_D,
    CLZ_D,
    CTO_D,
    CTZ_D,
    REVB_2H,
    REVB_4H,
    REVB_2W,
    REVB_D,
    REVH_2W,
    REVH_D,
    BITREV_4B,
    BITREV_8B,
    BITREV_W,
    BITREV_D,
    EXT_W_H,
    EXT_W_B,
    ASRTLE_D,
    ASRTGT_D,
    ALSL_W,
    ALSL_WU,
    ALSL_D,
    BYTEPICK_W,
    BYTEPICK_D,
    MUL_W,
    MULH_W,
    MULH_WU,
    MUL_D,
    MULH_D,
    MULH_DU,
    MULW_D_W,
    MULW_D_WU,
    DIV_W,
    MOD_W,
    DIV_WU,
    MOD_WU,
    DIV_D,
    MOD_D,
    DIV_DU,
    MOD_DU,
    ADDI_W,
    ADDI_D,
    SLTI,
    SLTUI,
    ANDI,
    ORI,
    XORI,
    ADDU12I_W,
    ADDU12I_D,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LD_B,
    LD_H,
    LD_W,
    LD_D,
    LD_BU,
    LD_HU,
    LDL_W,
    LDR_W,
    LDL_D,
    LDR_D,
    ST_B,
    ST_H,
    ST_W,
    ST_D,
    STL_W,
    STR_W,
    STL_D,
    STR_D,
    LDX_B,
    LDX_H,
    LDX_W,
    LDX_D,
    STX_B,
    STX_H,
    STX_W,
    STX_D,
    LL_W,
    SC_W,
    LLACQ_W,
    SCREL_W,
    AMSWAP_W,
    AMADD_W,
    AMAND_W,
    AMOR_W,
    AMXOR_W,
    AMMAX_W,
    AMMIN_W,
    DBAR,
    IBAR,
    SYSCALL,
    BREAK,
    DBCL,
    HVCL,
    TLBCLR,
    TLBFLUSH,
    TLBSRCH,
    TLBRD,
    TLBWR,
    TLBFILL,
    ERTN,
    IOCSRRD_B,
    IOCSRRD_H,
    IOCSRRD_W,
    IOCSRRD_D,
    IOCSRWR_B,
    IOCSRWR_H,
    IOCSRWR_W,
    IOCSRWR_D,
    CSRRD,
    CSRWR,
    CSRXCHG,
    GCSRRD,
    GCSRWR,
    GCSRXCHG,
    INVTLB,
    CACOP,
    LDDIR,
    LDPTE,
    GTLBFLUSH,
    IDLE,
    LU12I_W,
    PCADDI,
    PCALAU12I,
    PCADDU12I,
    RDTIMEL_W,
    RDTIMEH_W,
    CPUCFG,
    FFINT_D_L,
    FFINT_D_W,
    FFINT_S_L,
    FFINT_S_W,
    FTINT_L_D,
    FTINT_L_S,
    FTINT_W_D,
    FTINT_W_S,
    FTINTRM_L_D,
    FTINTRM_L_S,
    FTINTRM_W_D,
    FTINTRM_W_S,
    FTINTRNE_L_D,
    FTINTRNE_L_S,
    FTINTRNE_W_D,
    FTINTRNE_W_S,
    FTINTRP_L_D,
    FTINTRP_L_S,
    FTINTRP_W_D,
    FTINTRP_W_S,
    FTINTRZ_L_D,
    FTINTRZ_L_S,
    FTINTRZ_W_D,
    FTINTRZ_W_S,
    FLD_D,
    FLD_S,
    FLDGT_D,
    FLDGT_S,
    FLDLE_D,
    FLDLE_S,
    FLDX_D,
    FLDX_S,
    FMOV_D,
    FMOV_S,
    FST_D,
    FST_S,
    FSTGT_D,
    FSTGT_S,
    FSTLE_D,
    FSTLE_S,
    FSTX_D,
    FSTX_S,
    MOVCF2FR,
    MOVFCSR2GR,
    MOVFR2CF,
    MOVFR2GR_D,
    MOVFR2GR_S,
    MOVGR2FCSR,
    MOVGR2FR_D,
    MOVGR2FR_W,
    FSEL,
    MOVGR2CF,
    MOVCF2GR,
    FABS_D,
    FABS_S,
    FADD_D,
    FADD_S,
    FCLASS_D,
    FCLASS_S,
    FCOPYSIGN_D,
    FCOPYSIGN_S,
    FDIV_D,
    FDIV_S,
    FLOGB_D,
    FLOGB_S,
    FMADD_D,
    FMADD_S,
    FMAX_D,
    FMAX_S,
    FMAXA_D,
    FMAXA_S,
    FMIN_D,
    FMIN_S,
    FMINA_D,
    FMINA_S,
    FMSUB_D,
    FMSUB_S,
    FMUL_D,
    FMUL_S,
    FNEG_D,
    FNEG_S,
    FNMADD_D,
    FNMADD_S,
    FNMSUB_D,
    FNMSUB_S,
    FRECIP_D,
    FRECIP_S,
    FRECIPE_D,
    FRECIPE_S,
    FRINT_D,
    FRINT_S,
    FRSQRT_D,
    FRSQRT_S,
    FRSQRTE_D,
    FRSQRTE_S,
    FSCALEB_D,
    FSCALEB_S,
    FSQRT_D,
    FSQRT_S,
    FSUB_D,
    FSUB_S,
    XVSHUF_B,
    XVSHUF_H,
    XVSHUF_W,
    XVSHUF_D,
    FCMP_CAF_D,
    FCMP_CAF_S,
    FCMP_CEQ_D,
    FCMP_CEQ_S,
    FCMP_CLE_D,
    FCMP_CLE_S,
    FCMP_CLT_D,
    FCMP_CLT_S,
    FCMP_CNE_D,
    FCMP_CNE_S,
    FCMP_COR_D,
    FCMP_COR_S,
    FCMP_CUEQ_D,
    FCMP_CUEQ_S,
    FCMP_CULE_D,
    FCMP_CULE_S,
    FCMP_CULT_D,
    FCMP_CULT_S,
    FCMP_CUN_D,
    FCMP_CUN_S,
    FCMP_CUNE_D,
    FCMP_CUNE_S,
    FCMP_SAF_D,
    FCMP_SAF_S,
    FCMP_SEQ_D,
    FCMP_SEQ_S,
    FCMP_SLE_D,
    FCMP_SLE_S,
    FCMP_SLT_D,
    FCMP_SLT_S,
    FCMP_SNE_D,
    FCMP_SNE_S,
    FCMP_SOR_D,
    FCMP_SOR_S,
    FCMP_SUEQ_D,
    FCMP_SUEQ_S,
    FCMP_SULE_D,
    FCMP_SULE_S,
    FCMP_SULT_D,
    FCMP_SULT_S,
    FCMP_SUN_D,
    FCMP_SUN_S,
    FCMP_SUNE_D,
    FCMP_SUNE_S,
    FCVT_D_LD,
    FCVT_D_S,
    FCVT_LD_D,
    FCVT_S_D,
    FCVT_UD_D,
];

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use robustone_isa::{AliasPolicy, DecodeProfile, RenderDialect, decode_one};

    fn la64_base_profile() -> DecodeProfile<LoongArchBackend> {
        DecodeProfile {
            mode: LoongArchMode::LA64,
            features: LoongArchFeature::BASE | LoongArchFeature::LA64,
            render_dialect: RenderDialect::Canonical,
            alias_policy: AliasPolicy::None,
        }
    }

    #[test]
    fn backend_reads_fixed_4byte_le() {
        let bytes = &[0xAC, 0x39, 0x10, 0x00];
        let result = LoongArchBackend::read_instruction(bytes);
        assert!(result.is_ok());
        let read = result.unwrap();
        assert_eq!(read.raw, 0x0010_39AC);
        assert_eq!(read.length, 4);
    }

    #[test]
    fn backend_decode_add_w() {
        // add.w $t0, $t1, $t2
        // rd=12, rj=13, rk=14
        let word: u32 = 0x0010_0000 | 12 | (13 << 5) | (14 << 10);
        let bytes = word.to_le_bytes();
        let result = decode_one::<LoongArchBackend>(&bytes, 0x1000, &la64_base_profile());
        assert!(result.is_ok(), "{:?}", result);
        let insn = result.unwrap();
        assert_eq!(insn.mnemonic, "add.w");
        assert_eq!(insn.size, 4);
        assert_eq!(insn.registers_written.len(), 1);
        assert_eq!(insn.registers_read.len(), 2);
    }

    #[test]
    fn backend_decode_addi_w() {
        // addi.w $t0, $t1, 42
        // rd=12, rj=13, si12=42
        let word: u32 = 0x0280_0000 | 12 | (13 << 5) | (42 << 10);
        let bytes = word.to_le_bytes();
        let result = decode_one::<LoongArchBackend>(&bytes, 0x1000, &la64_base_profile());
        assert!(result.is_ok(), "{:?}", result);
        let insn = result.unwrap();
        assert_eq!(insn.mnemonic, "addi.w");
        assert_eq!(insn.size, 4);
        assert_eq!(insn.registers_written.len(), 1);
        assert_eq!(insn.registers_read.len(), 1);
        // Immediate operand
        assert_eq!(insn.operands.len(), 3);
        match &insn.operands[2] {
            robustone_core::ir::Operand::Immediate { value } => {
                assert_eq!(*value, 42);
            }
            other => panic!("expected immediate operand, got {:?}", other),
        }
    }

    #[test]
    fn base_specs_have_no_overlaps() {
        assert!(robustone_isa::validate_no_overlaps(LOONGARCH_BASE_SPECS).is_ok());
    }
}

// ============================================================================
// Aliases (post-decode render hints)
// ============================================================================

mod aliases;

// ============================================================================
// Backend implementation
// ============================================================================

pub struct LoongArchBackend;

impl ArchitectureBackend for LoongArchBackend {
    type Word = u32;
    type Mode = LoongArchMode;
    type Feature = LoongArchFeature;
    type Field = LoongArchField;
    type RegisterClass = LoongArchRegisterClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::LoongArch
    }

    fn read_instruction(bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::decode_failure(
                robustone_core::types::error::DecodeErrorKind::NeedMoreBytes,
                Some("loongarch".to_string()),
                "need at least 4 bytes",
            ));
        }
        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(InstructionRead {
            raw: word,
            length: 4,
        })
    }

    fn lookup(
        word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        LOONGARCH_BASE_SPECS
            .iter()
            .find(|spec| (word & spec.pattern().mask) == spec.pattern().value)
    }

    fn lower_register(
        class: Self::RegisterClass,
        raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        let id = match class {
            LoongArchRegisterClass::Gpr => raw,
            LoongArchRegisterClass::Fpr => raw + 32,
            LoongArchRegisterClass::Xr => raw + 64,
            LoongArchRegisterClass::Fcc => raw + 96,
            LoongArchRegisterClass::Scr => raw + 104,
            LoongArchRegisterClass::Fcsr => raw + 108,
        };
        RegisterId::loongarch(id)
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        RenderPolicy::new(
            robustone_isa::RenderDialect::Assembler,
            robustone_isa::AliasPolicy::PreferPseudo,
        )
    }

    fn extract_field(
        word: Self::Word,
        format: &FormatSpec<Self::Field>,
        field: Self::Field,
    ) -> Result<u32, DisasmError> {
        for f in format.fields() {
            if f.field_type() == field {
                let mask = ((1u64 << f.length()) - 1) as u32;
                return Ok((word >> f.start()) & mask);
            }
        }
        Err(DisasmError::decode_failure(
            DecodeErrorKind::InvalidField,
            Some("loongarch".to_string()),
            format!("field {:?} not found in format {}", field, format.name()),
        ))
    }

    fn apply_aliases(decoded: &mut robustone_core::ir::DecodedInstruction) {
        aliases::apply_aliases(decoded);
    }
}
