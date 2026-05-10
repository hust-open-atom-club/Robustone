//! AArch64 instruction decoder for Robustone.
//!
//! Top-level decoder: extracts the 32-bit instruction word, validates size,
//! and dispatches to extension modules via `op0[28:25]`.

use crate::extensions;
use crate::types::{AArch64Extensions, Mnemonic};
use robustone_core::{
    ir::{ArchitectureId, DecodeStatus, DecodedInstruction, Operand, RegisterId, RenderHints},
    types::error::{DecodeErrorKind, DisasmError},
};

/// AArch64 decoder with extension gating.
pub struct AArch64Decoder {
    extensions: AArch64Extensions,
}

impl Default for AArch64Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AArch64Decoder {
    /// Creates a new decoder with all extensions enabled.
    pub fn new() -> Self {
        Self {
            extensions: AArch64Extensions::all(),
        }
    }

    /// Creates a new decoder with the given extensions.
    pub fn with_extensions(extensions: AArch64Extensions) -> Self {
        Self { extensions }
    }

    /// Decodes a single AArch64 instruction from the given bytes.
    pub fn decode(
        &self,
        bytes: &[u8],
        _mode_name: &str,
        addr: u64,
    ) -> Result<DecodedInstruction, DisasmError> {
        if bytes.len() < 4 {
            return Err(DisasmError::DecodeFailure {
                kind: DecodeErrorKind::NeedMoreBytes,
                architecture: Some("aarch64".to_string()),
                detail: "need 4 bytes for AArch64".to_string(),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let (mnemonic, operands) = extensions::decode_by_op0(word, addr, &self.extensions)?;

        let mnemonic_str = mnemonic.as_str().to_string();

        // Compute metadata
        let (registers_read, registers_written, implicit_regs_read, implicit_regs_written, groups) =
            compute_metadata(mnemonic, &operands);

        // Compute render hints for alias expansion
        let render_hints = compute_render_hints(mnemonic, &operands);

        Ok(DecodedInstruction {
            architecture: ArchitectureId::Arm,
            address: addr,
            mode: "aarch64".to_string(),
            mnemonic: mnemonic_str.clone(),
            opcode_id: Some(mnemonic_str),
            size: 4,
            raw_bytes: bytes[..4].to_vec(),
            operands,
            registers_read,
            registers_written,
            implicit_registers_read: implicit_regs_read,
            implicit_registers_written: implicit_regs_written,
            groups,
            status: DecodeStatus::Success,
            render_hints,
            render: Some(crate::render::render_aarch64_text_parts),
        })
    }
}

type Metadata = (
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<RegisterId>,
    Vec<String>,
);

fn compute_metadata(mnemonic: Mnemonic, operands: &[Operand]) -> Metadata {
    use crate::types::Mnemonic::*;

    let mut read = Vec::new();
    let mut written = Vec::new();
    let mut implicit_read = Vec::new();
    let implicit_written = Vec::new();
    let mut groups = Vec::new();

    // Extract register IDs from operands
    let mut operand_regs = Vec::new();
    for op in operands {
        if let Operand::Register { register } = op {
            operand_regs.push(*register);
        }
    }

    match mnemonic {
        Add | Adds | Sub | Subs | And | Orr | Eor | Ands | Lsl | Lsr | Asr | Ror | Movz | Movn
        | Movk | Csel | Csinc | Csinv | Csneg | Madd | Msub | Smaddl | Smsubl | Umaddl | Umsubl
        | Sdiv | Udiv | Neg | Mvn | Mul | Mulh | Abs => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            for reg in operand_regs.iter().skip(1) {
                read.push(*reg);
            }
            groups.push("data".to_string());
        }
        Cmp | Cmn | Tst => {
            for reg in &operand_regs {
                read.push(*reg);
            }
            written.push(RegisterId {
                architecture: ArchitectureId::Arm,
                id: 33,
            }); // NZCV (PSTATE flags) — pseudo-register
            groups.push("data".to_string());
        }
        Adr | Adrp | Mov => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            groups.push("data".to_string());
        }
        B | Bl => {
            groups.push("branch".to_string());
            if mnemonic == Bl {
                written.push(RegisterId {
                    architecture: ArchitectureId::Arm,
                    id: 30,
                }); // LR (x30)
            }
        }
        Br | Blr | Ret => {
            groups.push("branch".to_string());
            if !operand_regs.is_empty() {
                read.push(operand_regs[0]);
            }
            if mnemonic == Ret {
                implicit_read.push(RegisterId {
                    architecture: ArchitectureId::Arm,
                    id: 30,
                }); // LR
            }
        }
        BCond => {
            groups.push("branch".to_string());
            implicit_read.push(RegisterId {
                architecture: ArchitectureId::Arm,
                id: 33,
            }); // NZCV
        }
        Cbz | Cbnz | Tbz | Tbnz => {
            groups.push("branch".to_string());
            if !operand_regs.is_empty() {
                read.push(operand_regs[0]);
            }
        }
        Cset | Csetm | Cinc | Cinv | Cneg => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            implicit_read.push(RegisterId {
                architecture: ArchitectureId::Arm,
                id: 33,
            }); // NZCV
            groups.push("data".to_string());
        }
        Nop | Svc | Hvc | Smc | Brk | Isb | Dsb | Dmb => {
            if matches!(mnemonic, Svc | Hvc | Smc | Brk) {
                groups.push("interrupt".to_string());
            } else {
                groups.push("system".to_string());
            }
        }
        Msr | Mrs => {
            groups.push("system".to_string());
        }
        Ldr | Ldrb | Ldrh | Ldrsb | Ldrsh | Ldrsw => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            groups.push("load".to_string());
        }
        Str | Strb | Strh => {
            groups.push("store".to_string());
        }
        Ldur | Ldurb | Ldurh | Ldursb | Ldursh | Ldursw => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            groups.push("load".to_string());
        }
        Stur | Sturb | Sturh => {
            groups.push("store".to_string());
        }
        Ldp | Ldpsw => {
            if operand_regs.len() >= 2 {
                written.push(operand_regs[0]);
                written.push(operand_regs[1]);
            }
            groups.push("load".to_string());
        }
        Stp => {
            groups.push("store".to_string());
        }
        Ldnp | Stnp => {
            if mnemonic == Ldnp && operand_regs.len() >= 2 {
                written.push(operand_regs[0]);
                written.push(operand_regs[1]);
            }
            groups.push(if mnemonic == Ldnp { "load".to_string() } else { "store".to_string() });
        }
        Ldxr | Ldxrb | Ldxrh => {
            if !operand_regs.is_empty() {
                written.push(operand_regs[0]);
            }
            groups.push("load".to_string());
        }
        Stxr | Stxrb | Stxrh => {
            groups.push("store".to_string());
        }
        Ldxp => {
            if operand_regs.len() >= 2 {
                written.push(operand_regs[0]);
                written.push(operand_regs[1]);
            }
            groups.push("load".to_string());
        }
        Stxp => {
            groups.push("store".to_string());
        }
        // SIMD/FP — Stage 3 (metadata computed in Step 4)
        Fadd | Fsub | Fmul | Fdiv | Fmadd | Fmsub | Fnmadd | Fnmsub | Fmov
        | Fcmp | Fcmpe | Fcsel | Fcvt | Scvtf | Ucvtf | Frinta | Frintm | Frintn
        | Frintp | Frintz | Frintx | Frinti | Fabs | Fneg | Fsqrt
        | Fmax | Fmin | Fmaxnm | Fminnm | Fnmla | Fnmls | Fnmul
        | Ld1 | St1 | Ld2 | St2 | Ld3 | St3 | Ld4 | St4
        // SIMD/FP — Stage 3C vector data processing
        | Mla | Mls | Cmeq | Cmge | Cmgt | Cmhi | Cmhs | Cmle | Cmlt | Cmtst
        | Smax | Smin | Umax | Umin | Sabd | Uabd | Saba | Uaba | Bsl | Bit | Bif
        | Shadd | Uhadd | Srhadd | Urhadd | Shsub | Uhsub
        | Sqadd | Uqadd | Sqsub | Uqsub | Suqadd | Usqadd
        | Sshl | Ushl | Sqshl | Uqshl | Srshl | Urshl | Sqrshl | Uqrshl | Sqshlu
        | Sqdmulh | Sqrdmulh
        | Sqabs | Sqneg | Addp | Addv | Saddl | Saddw | Ssubl | Ssubw
        | Uaddl | Uaddw | Usubl | Usubw | Smlal | Smlsl | Umlal | Umlsl
        | Smull | Umull | Sqdmlal | Sqdmlsl | Sqdmull
        | Sabal | Uabal | Sabdl | Uabdl | Addhn | Raddhn | Subhn | Rsubhn
        | Saddl2 | Uaddl2 | Saddw2 | Uaddw2 | Ssubl2 | Usubl2 | Ssubw2 | Usubw2
        | Smlal2 | Umlal2 | Smlsl2 | Umlsl2 | Smull2 | Umull2
        | Sqdmlal2 | Sqdmlsl2 | Sqdmull2
        | Sabal2 | Uabal2 | Sabdl2 | Uabdl2 | Addhn2 | Raddhn2 | Subhn2 | Rsubhn2
        | Sadalp | Uadalp | Saddlp | Uaddlp | Smaxp | Sminp | Umaxp | Uminp
        | Saddlv | Uaddlv | Saddv | Smaxv | Sminv | Umaxv | Uminv
        | Xtn | Xtn2 | Sqxtn | Sqxtn2 | Sqxtun | Sqxtun2 | Uqxtn | Uqxtn2
        | Shll | Shll2 | Pmul | Pmull | Pmull2
        | Dup | Ins | Ext | Umov | Smov | Zip1 | Zip2 | Uzp1 | Uzp2 | Trn1 | Trn2
        | Rev16 | Rev32 | Rev64 | Cls | Clz | Cnt | Rbit | Not
        | Faddp | Fmaxp | Fminp | Fmaxnmp | Fminnmp | Fmaxv | Fminv | Fmaxnmv | Fminnmv
        | Fcvtl | Fcvtl2 | Fcvtn | Fcvtn2 | Fcvtxn | Fcvtxn2
        | Frecpe | Frsqrte | Frecpx | Fcmge | Fcmgt | Fcmeq | Fcmle | Fcmlt
        | Fcvtx | Fcvtas | Fcvtau | Fcvtms | Fcvtmu | Fcvtns | Fcvtnu | Fcvtps | Fcvtpu | Fcvtzs | Fcvtzu
        | Fabd | Facge | Facgt | Fmulx | Frecps | Frsqrts
        | Fmlal | Fmlal2 | Fmlsl | Fmlsl2 | Fscale | Famax | Famin
        | Sshr | Ssra | Srshr | Srsra | Ushr | Usra | Urshr | Ursra | Sri | Sli | Shl
        | Shrn | Shrn2 | Rshrn | Rshrn2 | Sqshrn | Sqshrn2 | Sqrshrn | Sqrshrn2
        | Sqshrun | Sqshrun2 | Sqrshrun | Sqrshrun2 | Uqshrn | Uqshrn2 | Uqrshrn | Uqrshrn2
        | Sshll | Sshll2 | Ushll | Ushll2 | Movi | Mvni | Tbl | Tbx
        | Aese | Aesd | Aesmc | Aesimc | Sha1c | Sha1h | Sha1p | Sha1m | Sha1su0 | Sha1su1
        | Sha256h | Sha256h2 | Sha256su0 | Sha256su1
        // Also Bic and Orn (SIMD bitwise) and Fmla/Fmls (vector FP)
        | Bic | Orn | Fmla | Fmls => {
            groups.push("simd".to_string());
        }
    }

    (read, written, implicit_read, implicit_written, groups)
}

fn compute_render_hints(mnemonic: Mnemonic, operands: &[Operand]) -> RenderHints {
    use crate::types::Mnemonic::*;

    let mut hints = RenderHints::default();

    // Capstone alias expansion
    let capstone_mnemonic = match mnemonic {
        Orr => {
            // MOV = ORR (immediate) or ORR (register) with XZR
            if is_zero_reg_operand(operands.get(2)) || is_zero_reg_operand(operands.get(1)) {
                Some("mov".to_string())
            } else {
                None
            }
        }
        Sub => {
            // NEG = SUB with XZR as first source
            if is_zero_reg_operand(operands.get(1)) {
                Some("neg".to_string())
            } else {
                None
            }
        }
        Eor => {
            // MVN = EOR with -1 immediate (all ones)
            if is_all_ones_immediate(operands.get(2)) {
                Some("mvn".to_string())
            } else {
                None
            }
        }
        Madd => {
            // MUL = MADD with XZR as accumulator
            if is_zero_reg_operand(operands.get(3)) {
                Some("mul".to_string())
            } else {
                None
            }
        }
        Movz => {
            // MOV = MOVZ with no shift
            if operands.len() == 2 {
                Some("mov".to_string())
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(mnemonic) = capstone_mnemonic {
        hints.capstone_mnemonic = Some(mnemonic.clone());
        // Hide operands that are implied by the alias
        match mnemonic.as_str() {
            "mov" => {
                if let Some(Operand::Register { register }) = operands.get(2)
                    && register.id == 31
                {
                    hints.capstone_hidden_operands.push(2); // Hide XZR
                }
            }
            "neg" => {
                hints.capstone_hidden_operands.push(1); // Hide XZR
            }
            "mvn" => {
                hints.capstone_hidden_operands.push(2); // Hide all-ones imm
            }
            "mul" => {
                hints.capstone_hidden_operands.push(3); // Hide XZR accumulator
            }
            _ => {}
        }
    }

    // For B.cond, set capstone mnemonic to b.<cond> and hide the condition code operand
    if mnemonic == BCond
        && !operands.is_empty()
        && let Some(Operand::Text { value }) = operands.first()
    {
        hints.capstone_mnemonic = Some(format!("b.{}", value));
        hints.capstone_hidden_operands.push(0);
    }

    hints
}

fn is_zero_reg_operand(op: Option<&Operand>) -> bool {
    matches!(op, Some(Operand::Register { register }) if register.id == 31)
}

fn is_all_ones_immediate(op: Option<&Operand>) -> bool {
    matches!(op, Some(Operand::Immediate { value }) if *value == -1)
}
