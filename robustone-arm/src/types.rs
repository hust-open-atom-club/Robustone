//! AArch64-specific types and immediate decoders.

/// AArch64 instruction mnemonics (Stage 1 subset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    // Data Processing — Immediate
    Adr,
    Adrp,
    Add,
    Adds,
    Sub,
    Subs,
    Movz,
    Movn,
    Movk,
    And,
    Orr,
    Eor,
    Bic,
    Orn,
    Ands,
    Cmp,
    Cmn,
    Tst,
    // Data Processing — Register
    Lsl,
    Lsr,
    Asr,
    Ror,
    Csel,
    Csinc,
    Csinv,
    Csneg,
    Cset,
    Csetm,
    Cinc,
    Cinv,
    Cneg,
    Madd,
    Msub,
    Smaddl,
    Smsubl,
    Umaddl,
    Umsubl,
    Sdiv,
    Udiv,
    // Branches
    B,
    Bl,
    Br,
    Blr,
    Ret,
    Cbz,
    Cbnz,
    Tbz,
    Tbnz,
    BCond,
    // System / Hints
    Nop,
    Svc,
    Hvc,
    Smc,
    Brk,
    Isb,
    Dsb,
    Dmb,
    Msr,
    Mrs,
    // Loads and Stores
    Ldr,
    Ldrb,
    Ldrh,
    Ldrsb,
    Ldrsh,
    Ldrsw,
    Str,
    Strb,
    Strh,
    Ldur,
    Ldurb,
    Ldurh,
    Ldursb,
    Ldursh,
    Ldursw,
    Stur,
    Sturb,
    Sturh,
    Ldp,
    Stp,
    Ldpsw,
    Ldnp,
    Stnp,
    Ldxr,
    Ldxrb,
    Ldxrh,
    Stxr,
    Stxrb,
    Stxrh,
    Ldxp,
    Stxp,
    // SIMD/FP — Scalar FP
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    Fmadd,
    Fmsub,
    Fnmadd,
    Fnmsub,
    Fmla,
    Fmls,
    Fmov,
    Fcmp,
    Fcmpe,
    Fcsel,
    Fmax,
    Fmin,
    Fmaxnm,
    Fminnm,
    Fnmla,
    Fnmls,
    Fnmul,
    Fcvt,
    Scvtf,
    Ucvtf,
    Frinta,
    Frintm,
    Frintn,
    Frintp,
    Frintz,
    Frintx,
    Frinti,
    Fabs,
    Fneg,
    Fsqrt,
    // SIMD/FP — Loads/Stores (structure)
    Ld1,
    St1,
    Ld2,
    St2,
    Ld3,
    St3,
    Ld4,
    St4,
    // SIMD/FP — Vector Integer (Three Same / Three Different / Two-reg Misc)
    Mla,
    Mls,
    Cmeq,
    Cmge,
    Cmgt,
    Cmhi,
    Cmhs,
    Cmle,
    Cmlt,
    Cmtst,
    Smax,
    Smin,
    Umax,
    Umin,
    Sabd,
    Uabd,
    Saba,
    Uaba,
    Bsl,
    Bit,
    Bif,
    Shadd,
    Uhadd,
    Srhadd,
    Urhadd,
    Shsub,
    Uhsub,
    Sqadd,
    Uqadd,
    Sqsub,
    Uqsub,
    Suqadd,
    Usqadd,
    Sshl,
    Ushl,
    Sqshl,
    Uqshl,
    Srshl,
    Urshl,
    Sqrshl,
    Uqrshl,
    Sqshlu,
    Sqdmulh,
    Sqrdmulh,
    Sqabs,
    Sqneg,
    Addp,
    Addv,
    Saddl,
    Saddw,
    Ssubl,
    Ssubw,
    Uaddl,
    Uaddw,
    Usubl,
    Usubw,
    Smlal,
    Smlsl,
    Umlal,
    Umlsl,
    Smull,
    Umull,
    Sqdmlal,
    Sqdmlsl,
    Sqdmull,
    Sabal,
    Uabal,
    Sabdl,
    Uabdl,
    Addhn,
    Raddhn,
    Subhn,
    Rsubhn,
    Saddl2,
    Uaddl2,
    Saddw2,
    Uaddw2,
    Ssubl2,
    Usubl2,
    Ssubw2,
    Usubw2,
    Smlal2,
    Umlal2,
    Smlsl2,
    Umlsl2,
    Smull2,
    Umull2,
    Sqdmlal2,
    Sqdmlsl2,
    Sqdmull2,
    Sabal2,
    Uabal2,
    Sabdl2,
    Uabdl2,
    Addhn2,
    Raddhn2,
    Subhn2,
    Rsubhn2,
    Sadalp,
    Uadalp,
    Saddlp,
    Uaddlp,
    Smaxp,
    Sminp,
    Umaxp,
    Uminp,
    Saddlv,
    Uaddlv,
    Saddv,
    Smaxv,
    Sminv,
    Umaxv,
    Uminv,
    // SIMD/FP — Vector Narrow / Widen / Move
    Xtn,
    Xtn2,
    Sqxtn,
    Sqxtn2,
    Sqxtun,
    Sqxtun2,
    Uqxtn,
    Uqxtn2,
    Shll,
    Shll2,
    Pmul,
    Pmull,
    Pmull2,
    // SIMD/FP — Vector Permute / Copy
    Dup,
    Ins,
    Ext,
    Umov,
    Smov,
    Zip1,
    Zip2,
    Uzp1,
    Uzp2,
    Trn1,
    Trn2,
    Rev16,
    Rev32,
    Rev64,
    Cls,
    Clz,
    Cnt,
    Rbit,
    Not,
    // SIMD/FP — Vector FP Misc / Compare
    Faddp,
    Fmaxp,
    Fminp,
    Fmaxnmp,
    Fminnmp,
    Fmaxv,
    Fminv,
    Fmaxnmv,
    Fminnmv,
    Fcvtl,
    Fcvtl2,
    Fcvtn,
    Fcvtn2,
    Fcvtxn,
    Fcvtxn2,
    Frecpe,
    Frsqrte,
    Frecpx,
    Fcmge,
    Fcmgt,
    Fcmeq,
    Fcmle,
    Fcmlt,
    Fcvtx,
    Fcvtas,
    Fcvtau,
    Fcvtms,
    Fcvtmu,
    Fcvtns,
    Fcvtnu,
    Fcvtps,
    Fcvtpu,
    Fcvtzs,
    Fcvtzu,
    Fabd,
    Facge,
    Facgt,
    Fmulx,
    Frecps,
    Frsqrts,
    Fmlal,
    Fmlal2,
    Fmlsl,
    Fmlsl2,
    Fscale,
    Famax,
    Famin,
    // SIMD/FP — Shift Immediate
    Sshr,
    Ssra,
    Srshr,
    Srsra,
    Ushr,
    Usra,
    Urshr,
    Ursra,
    Sri,
    Sli,
    Shl,
    Shrn,
    Shrn2,
    Rshrn,
    Rshrn2,
    Sqshrn,
    Sqshrn2,
    Sqrshrn,
    Sqrshrn2,
    Sqshrun,
    Sqshrun2,
    Sqrshrun,
    Sqrshrun2,
    Uqshrn,
    Uqshrn2,
    Uqrshrn,
    Uqrshrn2,
    Sshll,
    Sshll2,
    Ushll,
    Ushll2,
    // SIMD/FP — Modified Immediate
    Movi,
    Mvni,
    // SIMD/FP — Table Lookup
    Tbl,
    Tbx,
    // SIMD/FP — Cryptographic
    Aese,
    Aesd,
    Aesmc,
    Aesimc,
    Sha1c,
    Sha1h,
    Sha1p,
    Sha1m,
    Sha1su0,
    Sha1su1,
    Sha256h,
    Sha256h2,
    Sha256su0,
    Sha256su1,
    // Aliases (render-time only)
    Mov,
    Neg,
    Mvn,
    Mul,
    Mulh,
    Abs,
}

impl Mnemonic {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Mnemonic::Adr => "adr",
            Mnemonic::Adrp => "adrp",
            Mnemonic::Add => "add",
            Mnemonic::Adds => "adds",
            Mnemonic::Sub => "sub",
            Mnemonic::Subs => "subs",
            Mnemonic::Movz => "movz",
            Mnemonic::Movn => "movn",
            Mnemonic::Movk => "movk",
            Mnemonic::And => "and",
            Mnemonic::Orr => "orr",
            Mnemonic::Eor => "eor",
            Mnemonic::Bic => "bic",
            Mnemonic::Orn => "orn",
            Mnemonic::Ands => "ands",
            Mnemonic::Cmp => "cmp",
            Mnemonic::Cmn => "cmn",
            Mnemonic::Tst => "tst",
            Mnemonic::Lsl => "lsl",
            Mnemonic::Lsr => "lsr",
            Mnemonic::Asr => "asr",
            Mnemonic::Ror => "ror",
            Mnemonic::Csel => "csel",
            Mnemonic::Csinc => "csinc",
            Mnemonic::Csinv => "csinv",
            Mnemonic::Csneg => "csneg",
            Mnemonic::Cset => "cset",
            Mnemonic::Csetm => "csetm",
            Mnemonic::Cinc => "cinc",
            Mnemonic::Cinv => "cinv",
            Mnemonic::Cneg => "cneg",
            Mnemonic::Madd => "madd",
            Mnemonic::Msub => "msub",
            Mnemonic::Smaddl => "smaddl",
            Mnemonic::Smsubl => "smsubl",
            Mnemonic::Umaddl => "umaddl",
            Mnemonic::Umsubl => "umsubl",
            Mnemonic::Sdiv => "sdiv",
            Mnemonic::Udiv => "udiv",
            Mnemonic::B => "b",
            Mnemonic::Bl => "bl",
            Mnemonic::Br => "br",
            Mnemonic::Blr => "blr",
            Mnemonic::Ret => "ret",
            Mnemonic::Cbz => "cbz",
            Mnemonic::Cbnz => "cbnz",
            Mnemonic::Tbz => "tbz",
            Mnemonic::Tbnz => "tbnz",
            Mnemonic::BCond => "b",
            Mnemonic::Nop => "nop",
            Mnemonic::Svc => "svc",
            Mnemonic::Hvc => "hvc",
            Mnemonic::Smc => "smc",
            Mnemonic::Brk => "brk",
            Mnemonic::Isb => "isb",
            Mnemonic::Dsb => "dsb",
            Mnemonic::Dmb => "dmb",
            Mnemonic::Msr => "msr",
            Mnemonic::Mrs => "mrs",
            Mnemonic::Ldr => "ldr",
            Mnemonic::Ldrb => "ldrb",
            Mnemonic::Ldrh => "ldrh",
            Mnemonic::Ldrsb => "ldrsb",
            Mnemonic::Ldrsh => "ldrsh",
            Mnemonic::Ldrsw => "ldrsw",
            Mnemonic::Str => "str",
            Mnemonic::Strb => "strb",
            Mnemonic::Strh => "strh",
            Mnemonic::Ldur => "ldur",
            Mnemonic::Ldurb => "ldurb",
            Mnemonic::Ldurh => "ldurh",
            Mnemonic::Ldursb => "ldursb",
            Mnemonic::Ldursh => "ldursh",
            Mnemonic::Ldursw => "ldursw",
            Mnemonic::Stur => "stur",
            Mnemonic::Sturb => "sturb",
            Mnemonic::Sturh => "sturh",
            Mnemonic::Ldp => "ldp",
            Mnemonic::Stp => "stp",
            Mnemonic::Ldpsw => "ldpsw",
            Mnemonic::Ldnp => "ldnp",
            Mnemonic::Stnp => "stnp",
            Mnemonic::Ldxr => "ldxr",
            Mnemonic::Ldxrb => "ldxrb",
            Mnemonic::Ldxrh => "ldxrh",
            Mnemonic::Stxr => "stxr",
            Mnemonic::Stxrb => "stxrb",
            Mnemonic::Stxrh => "stxrh",
            Mnemonic::Ldxp => "ldxp",
            Mnemonic::Stxp => "stxp",
            Mnemonic::Fadd => "fadd",
            Mnemonic::Fsub => "fsub",
            Mnemonic::Fmul => "fmul",
            Mnemonic::Fdiv => "fdiv",
            Mnemonic::Fmadd => "fmadd",
            Mnemonic::Fmsub => "fmsub",
            Mnemonic::Fnmadd => "fnmadd",
            Mnemonic::Fnmsub => "fnmsub",
            Mnemonic::Fmla => "fmla",
            Mnemonic::Fmls => "fmls",
            Mnemonic::Fmov => "fmov",
            Mnemonic::Fcmp => "fcmp",
            Mnemonic::Fcmpe => "fcmpe",
            Mnemonic::Fcsel => "fcsel",
            Mnemonic::Fmax => "fmax",
            Mnemonic::Fmin => "fmin",
            Mnemonic::Fmaxnm => "fmaxnm",
            Mnemonic::Fminnm => "fminnm",
            Mnemonic::Fnmla => "fnmla",
            Mnemonic::Fnmls => "fnmls",
            Mnemonic::Fnmul => "fnmul",
            Mnemonic::Fcvt => "fcvt",
            Mnemonic::Scvtf => "scvtf",
            Mnemonic::Ucvtf => "ucvtf",
            Mnemonic::Frinta => "frinta",
            Mnemonic::Frintm => "frintm",
            Mnemonic::Frintn => "frintn",
            Mnemonic::Frintp => "frintp",
            Mnemonic::Frintz => "frintz",
            Mnemonic::Frintx => "frintx",
            Mnemonic::Frinti => "frinti",
            Mnemonic::Fabs => "fabs",
            Mnemonic::Fneg => "fneg",
            Mnemonic::Fsqrt => "fsqrt",
            Mnemonic::Ld1 => "ld1",
            Mnemonic::St1 => "st1",
            Mnemonic::Ld2 => "ld2",
            Mnemonic::St2 => "st2",
            Mnemonic::Ld3 => "ld3",
            Mnemonic::St3 => "st3",
            Mnemonic::Ld4 => "ld4",
            Mnemonic::St4 => "st4",
            // Vector Integer
            Mnemonic::Mla => "mla",
            Mnemonic::Mls => "mls",
            Mnemonic::Cmeq => "cmeq",
            Mnemonic::Cmge => "cmge",
            Mnemonic::Cmgt => "cmgt",
            Mnemonic::Cmhi => "cmhi",
            Mnemonic::Cmhs => "cmhs",
            Mnemonic::Cmle => "cmle",
            Mnemonic::Cmlt => "cmlt",
            Mnemonic::Cmtst => "cmtst",
            Mnemonic::Smax => "smax",
            Mnemonic::Smin => "smin",
            Mnemonic::Umax => "umax",
            Mnemonic::Umin => "umin",
            Mnemonic::Sabd => "sabd",
            Mnemonic::Uabd => "uabd",
            Mnemonic::Saba => "saba",
            Mnemonic::Uaba => "uaba",
            Mnemonic::Bsl => "bsl",
            Mnemonic::Bit => "bit",
            Mnemonic::Bif => "bif",
            Mnemonic::Shadd => "shadd",
            Mnemonic::Uhadd => "uhadd",
            Mnemonic::Srhadd => "srhadd",
            Mnemonic::Urhadd => "urhadd",
            Mnemonic::Shsub => "shsub",
            Mnemonic::Uhsub => "uhsub",
            Mnemonic::Sqadd => "sqadd",
            Mnemonic::Uqadd => "uqadd",
            Mnemonic::Sqsub => "sqsub",
            Mnemonic::Uqsub => "uqsub",
            Mnemonic::Suqadd => "suqadd",
            Mnemonic::Usqadd => "usqadd",
            Mnemonic::Sshl => "sshl",
            Mnemonic::Ushl => "ushl",
            Mnemonic::Sqshl => "sqshl",
            Mnemonic::Uqshl => "uqshl",
            Mnemonic::Srshl => "srshl",
            Mnemonic::Urshl => "urshl",
            Mnemonic::Sqrshl => "sqrshl",
            Mnemonic::Uqrshl => "uqrshl",
            Mnemonic::Sqshlu => "sqshlu",
            Mnemonic::Sqdmulh => "sqdmulh",
            Mnemonic::Sqrdmulh => "sqrdmulh",
            Mnemonic::Sqabs => "sqabs",
            Mnemonic::Sqneg => "sqneg",
            Mnemonic::Addp => "addp",
            Mnemonic::Addv => "addv",
            Mnemonic::Saddl => "saddl",
            Mnemonic::Saddw => "saddw",
            Mnemonic::Ssubl => "ssubl",
            Mnemonic::Ssubw => "ssubw",
            Mnemonic::Uaddl => "uaddl",
            Mnemonic::Uaddw => "uaddw",
            Mnemonic::Usubl => "usubl",
            Mnemonic::Usubw => "usubw",
            Mnemonic::Smlal => "smlal",
            Mnemonic::Smlsl => "smlsl",
            Mnemonic::Umlal => "umlal",
            Mnemonic::Umlsl => "umlsl",
            Mnemonic::Smull => "smull",
            Mnemonic::Umull => "umull",
            Mnemonic::Sqdmlal => "sqdmlal",
            Mnemonic::Sqdmlsl => "sqdmlsl",
            Mnemonic::Sqdmull => "sqdmull",
            Mnemonic::Sabal => "sabal",
            Mnemonic::Uabal => "uabal",
            Mnemonic::Sabdl => "sabdl",
            Mnemonic::Uabdl => "uabdl",
            Mnemonic::Addhn => "addhn",
            Mnemonic::Raddhn => "raddhn",
            Mnemonic::Subhn => "subhn",
            Mnemonic::Rsubhn => "rsubhn",
            Mnemonic::Saddl2 => "saddl2",
            Mnemonic::Uaddl2 => "uaddl2",
            Mnemonic::Saddw2 => "saddw2",
            Mnemonic::Uaddw2 => "uaddw2",
            Mnemonic::Ssubl2 => "ssubl2",
            Mnemonic::Usubl2 => "usubl2",
            Mnemonic::Ssubw2 => "ssubw2",
            Mnemonic::Usubw2 => "usubw2",
            Mnemonic::Smlal2 => "smlal2",
            Mnemonic::Umlal2 => "umlal2",
            Mnemonic::Smlsl2 => "smlsl2",
            Mnemonic::Umlsl2 => "umlsl2",
            Mnemonic::Smull2 => "smull2",
            Mnemonic::Umull2 => "umull2",
            Mnemonic::Sqdmlal2 => "sqdmlal2",
            Mnemonic::Sqdmlsl2 => "sqdmlsl2",
            Mnemonic::Sqdmull2 => "sqdmull2",
            Mnemonic::Sabal2 => "sabal2",
            Mnemonic::Uabal2 => "uabal2",
            Mnemonic::Sabdl2 => "sabdl2",
            Mnemonic::Uabdl2 => "uabdl2",
            Mnemonic::Addhn2 => "addhn2",
            Mnemonic::Raddhn2 => "raddhn2",
            Mnemonic::Subhn2 => "subhn2",
            Mnemonic::Rsubhn2 => "rsubhn2",
            Mnemonic::Sadalp => "sadalp",
            Mnemonic::Uadalp => "uadalp",
            Mnemonic::Saddlp => "saddlp",
            Mnemonic::Uaddlp => "uaddlp",
            Mnemonic::Smaxp => "smaxp",
            Mnemonic::Sminp => "sminp",
            Mnemonic::Umaxp => "umaxp",
            Mnemonic::Uminp => "uminp",
            Mnemonic::Saddlv => "saddlv",
            Mnemonic::Uaddlv => "uaddlv",
            Mnemonic::Saddv => "saddv",
            Mnemonic::Smaxv => "smaxv",
            Mnemonic::Sminv => "sminv",
            Mnemonic::Umaxv => "umaxv",
            Mnemonic::Uminv => "uminv",
            // Narrow / Widen / Move
            Mnemonic::Xtn => "xtn",
            Mnemonic::Xtn2 => "xtn2",
            Mnemonic::Sqxtn => "sqxtn",
            Mnemonic::Sqxtn2 => "sqxtn2",
            Mnemonic::Sqxtun => "sqxtun",
            Mnemonic::Sqxtun2 => "sqxtun2",
            Mnemonic::Uqxtn => "uqxtn",
            Mnemonic::Uqxtn2 => "uqxtn2",
            Mnemonic::Shll => "shll",
            Mnemonic::Shll2 => "shll2",
            Mnemonic::Pmul => "pmul",
            Mnemonic::Pmull => "pmull",
            Mnemonic::Pmull2 => "pmull2",
            // Permute / Copy
            Mnemonic::Dup => "dup",
            Mnemonic::Ins => "ins",
            Mnemonic::Ext => "ext",
            Mnemonic::Umov => "umov",
            Mnemonic::Smov => "smov",
            Mnemonic::Zip1 => "zip1",
            Mnemonic::Zip2 => "zip2",
            Mnemonic::Uzp1 => "uzp1",
            Mnemonic::Uzp2 => "uzp2",
            Mnemonic::Trn1 => "trn1",
            Mnemonic::Trn2 => "trn2",
            Mnemonic::Rev16 => "rev16",
            Mnemonic::Rev32 => "rev32",
            Mnemonic::Rev64 => "rev64",
            Mnemonic::Cls => "cls",
            Mnemonic::Clz => "clz",
            Mnemonic::Cnt => "cnt",
            Mnemonic::Rbit => "rbit",
            Mnemonic::Not => "not",
            // Vector FP Misc / Compare
            Mnemonic::Faddp => "faddp",
            Mnemonic::Fmaxp => "fmaxp",
            Mnemonic::Fminp => "fminp",
            Mnemonic::Fmaxnmp => "fmaxnmp",
            Mnemonic::Fminnmp => "fminnmp",
            Mnemonic::Fmaxv => "fmaxv",
            Mnemonic::Fminv => "fminv",
            Mnemonic::Fmaxnmv => "fmaxnmv",
            Mnemonic::Fminnmv => "fminnmv",
            Mnemonic::Fcvtl => "fcvtl",
            Mnemonic::Fcvtl2 => "fcvtl2",
            Mnemonic::Fcvtn => "fcvtn",
            Mnemonic::Fcvtn2 => "fcvtn2",
            Mnemonic::Fcvtxn => "fcvtxn",
            Mnemonic::Fcvtxn2 => "fcvtxn2",
            Mnemonic::Frecpe => "frecpe",
            Mnemonic::Frsqrte => "frsqrte",
            Mnemonic::Frecpx => "frecpx",
            Mnemonic::Fcmge => "fcmge",
            Mnemonic::Fcmgt => "fcmgt",
            Mnemonic::Fcmeq => "fcmeq",
            Mnemonic::Fcmle => "fcmle",
            Mnemonic::Fcmlt => "fcmlt",
            Mnemonic::Fcvtx => "fcvtx",
            Mnemonic::Fcvtas => "fcvtas",
            Mnemonic::Fcvtau => "fcvtau",
            Mnemonic::Fcvtms => "fcvtms",
            Mnemonic::Fcvtmu => "fcvtmu",
            Mnemonic::Fcvtns => "fcvtns",
            Mnemonic::Fcvtnu => "fcvtnu",
            Mnemonic::Fcvtps => "fcvtps",
            Mnemonic::Fcvtpu => "fcvtpu",
            Mnemonic::Fcvtzs => "fcvtzs",
            Mnemonic::Fcvtzu => "fcvtzu",
            Mnemonic::Fabd => "fabd",
            Mnemonic::Facge => "facge",
            Mnemonic::Facgt => "facgt",
            Mnemonic::Fmulx => "fmulx",
            Mnemonic::Frecps => "frecps",
            Mnemonic::Frsqrts => "frsqrts",
            Mnemonic::Fmlal => "fmlal",
            Mnemonic::Fmlal2 => "fmlal2",
            Mnemonic::Fmlsl => "fmlsl",
            Mnemonic::Fmlsl2 => "fmlsl2",
            Mnemonic::Fscale => "fscale",
            Mnemonic::Famax => "famax",
            Mnemonic::Famin => "famin",
            // Shift Immediate
            Mnemonic::Sshr => "sshr",
            Mnemonic::Ssra => "ssra",
            Mnemonic::Srshr => "srshr",
            Mnemonic::Srsra => "srsra",
            Mnemonic::Ushr => "ushr",
            Mnemonic::Usra => "usra",
            Mnemonic::Urshr => "urshr",
            Mnemonic::Ursra => "ursra",
            Mnemonic::Sri => "sri",
            Mnemonic::Sli => "sli",
            Mnemonic::Shl => "shl",
            Mnemonic::Shrn => "shrn",
            Mnemonic::Shrn2 => "shrn2",
            Mnemonic::Rshrn => "rshrn",
            Mnemonic::Rshrn2 => "rshrn2",
            Mnemonic::Sqshrn => "sqshrn",
            Mnemonic::Sqshrn2 => "sqshrn2",
            Mnemonic::Sqrshrn => "sqrshrn",
            Mnemonic::Sqrshrn2 => "sqrshrn2",
            Mnemonic::Sqshrun => "sqshrun",
            Mnemonic::Sqshrun2 => "sqshrun2",
            Mnemonic::Sqrshrun => "sqrshrun",
            Mnemonic::Sqrshrun2 => "sqrshrun2",
            Mnemonic::Uqshrn => "uqshrn",
            Mnemonic::Uqshrn2 => "uqshrn2",
            Mnemonic::Uqrshrn => "uqrshrn",
            Mnemonic::Uqrshrn2 => "uqrshrn2",
            Mnemonic::Sshll => "sshll",
            Mnemonic::Sshll2 => "sshll2",
            Mnemonic::Ushll => "ushll",
            Mnemonic::Ushll2 => "ushll2",
            // Modified Immediate
            Mnemonic::Movi => "movi",
            Mnemonic::Mvni => "mvni",
            // Table Lookup
            Mnemonic::Tbl => "tbl",
            Mnemonic::Tbx => "tbx",
            // Cryptographic
            Mnemonic::Aese => "aese",
            Mnemonic::Aesd => "aesd",
            Mnemonic::Aesmc => "aesmc",
            Mnemonic::Aesimc => "aesimc",
            Mnemonic::Sha1c => "sha1c",
            Mnemonic::Sha1h => "sha1h",
            Mnemonic::Sha1p => "sha1p",
            Mnemonic::Sha1m => "sha1m",
            Mnemonic::Sha1su0 => "sha1su0",
            Mnemonic::Sha1su1 => "sha1su1",
            Mnemonic::Sha256h => "sha256h",
            Mnemonic::Sha256h2 => "sha256h2",
            Mnemonic::Sha256su0 => "sha256su0",
            Mnemonic::Sha256su1 => "sha256su1",
            // Aliases
            Mnemonic::Mov => "mov",
            Mnemonic::Neg => "neg",
            Mnemonic::Mvn => "mvn",
            Mnemonic::Mul => "mul",
            Mnemonic::Mulh => "mulh",
            Mnemonic::Abs => "abs",
        }
    }
}

/// Condition codes for `b.cond` and conditional selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCode {
    Eq,
    Ne,
    Cs,
    Cc,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
    Nv,
}

impl ConditionCode {
    pub fn from_bits(cond: u8) -> Option<Self> {
        match cond & 0xF {
            0x0 => Some(ConditionCode::Eq),
            0x1 => Some(ConditionCode::Ne),
            0x2 => Some(ConditionCode::Cs),
            0x3 => Some(ConditionCode::Cc),
            0x4 => Some(ConditionCode::Mi),
            0x5 => Some(ConditionCode::Pl),
            0x6 => Some(ConditionCode::Vs),
            0x7 => Some(ConditionCode::Vc),
            0x8 => Some(ConditionCode::Hi),
            0x9 => Some(ConditionCode::Ls),
            0xA => Some(ConditionCode::Ge),
            0xB => Some(ConditionCode::Lt),
            0xC => Some(ConditionCode::Gt),
            0xD => Some(ConditionCode::Le),
            0xE => Some(ConditionCode::Al),
            0xF => Some(ConditionCode::Nv),
            _ => unreachable!(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            ConditionCode::Eq => "eq",
            ConditionCode::Ne => "ne",
            ConditionCode::Cs => "cs",
            ConditionCode::Cc => "cc",
            ConditionCode::Mi => "mi",
            ConditionCode::Pl => "pl",
            ConditionCode::Vs => "vs",
            ConditionCode::Vc => "vc",
            ConditionCode::Hi => "hi",
            ConditionCode::Ls => "ls",
            ConditionCode::Ge => "ge",
            ConditionCode::Lt => "lt",
            ConditionCode::Gt => "gt",
            ConditionCode::Le => "le",
            ConditionCode::Al => "al",
            ConditionCode::Nv => "nv",
        }
    }

    pub const fn as_str_capstone(&self) -> &'static str {
        // Capstone uses hs/lo aliases for cs/cc
        match self {
            ConditionCode::Cs => "hs",
            ConditionCode::Cc => "lo",
            other => other.as_str(),
        }
    }
}

/// Shift types for register-shifted operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftType {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl ShiftType {
    pub fn from_bits(shift: u8) -> Option<Self> {
        match shift & 0x3 {
            0b00 => Some(ShiftType::Lsl),
            0b01 => Some(ShiftType::Lsr),
            0b10 => Some(ShiftType::Asr),
            0b11 => Some(ShiftType::Ror),
            _ => unreachable!(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            ShiftType::Lsl => "lsl",
            ShiftType::Lsr => "lsr",
            ShiftType::Asr => "asr",
            ShiftType::Ror => "ror",
        }
    }
}

/// Extend types for extended register operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendType {
    Uxtb,
    Uxth,
    Uxtw,
    Uxtx,
    Sxtb,
    Sxth,
    Sxtw,
    Sxtx,
}

impl ExtendType {
    pub fn from_bits(ext: u8) -> Option<Self> {
        match ext & 0x7 {
            0b000 => Some(ExtendType::Uxtb),
            0b001 => Some(ExtendType::Uxth),
            0b010 => Some(ExtendType::Uxtw),
            0b011 => Some(ExtendType::Uxtx),
            0b100 => Some(ExtendType::Sxtb),
            0b101 => Some(ExtendType::Sxth),
            0b110 => Some(ExtendType::Sxtw),
            0b111 => Some(ExtendType::Sxtx),
            _ => unreachable!(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            ExtendType::Uxtb => "uxtb",
            ExtendType::Uxth => "uxth",
            ExtendType::Uxtw => "uxtw",
            ExtendType::Uxtx => "uxtx",
            ExtendType::Sxtb => "sxtb",
            ExtendType::Sxth => "sxth",
            ExtendType::Sxtw => "sxtw",
            ExtendType::Sxtx => "sxtx",
        }
    }
}

/// Extension set for AArch64 ArchitectureProfile gating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AArch64Extensions {
    bits: u32,
}

impl AArch64Extensions {
    pub const BASE: Self = Self { bits: 1 << 0 };
    pub const FP: Self = Self { bits: 1 << 1 };
    pub const SIMD: Self = Self { bits: 1 << 2 };
    pub const SVE: Self = Self { bits: 1 << 3 };
    pub const SVE2: Self = Self { bits: 1 << 4 };

    pub const fn empty() -> Self {
        Self { bits: 0 }
    }
    pub const fn all() -> Self {
        Self { bits: 0xFFFFFFFF }
    }

    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub fn from_profile(
        extensions: &[&str],
    ) -> Result<Self, robustone_core::types::error::DisasmError> {
        use robustone_core::types::error::{DecodeErrorKind, DisasmError};
        let mut result = Self::empty();
        for ext in extensions {
            match *ext {
                "base" => result.bits |= Self::BASE.bits,
                "+fp" => result.bits |= Self::FP.bits,
                "+simd" | "+neon" => result.bits |= Self::SIMD.bits,
                "+sve" => result.bits |= Self::SVE.bits,
                "+sve2" => result.bits |= Self::SVE2.bits,
                other => {
                    return Err(DisasmError::decode_failure(
                        DecodeErrorKind::UnsupportedExtension,
                        None::<String>,
                        format!("unsupported AArch64 extension `{other}`"),
                    ));
                }
            }
        }
        Ok(result)
    }
}

impl core::ops::BitOr for AArch64Extensions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl core::ops::BitOrAssign for AArch64Extensions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

/// Decode a 12-bit unsigned immediate with optional shift-by-12.
/// Returns (imm_value, shift_applied).
pub fn decode_imm12(word: u32) -> (i64, bool) {
    let imm = ((word >> 10) & 0xFFF) as i64;
    let shift = ((word >> 22) & 0x1) != 0;
    if shift {
        (imm << 12, true)
    } else {
        (imm, false)
    }
}

/// Decode a 16-bit shifted immediate (hw:imm16 for movz/movn/movk).
/// Returns (imm_value, shift_amount_in_bits).
pub fn decode_imm16_hw(word: u32) -> (i64, u32) {
    let imm16 = ((word >> 5) & 0xFFFF) as i64;
    let hw = (word >> 21) & 0x3;
    (imm16 << (hw * 16), hw * 16)
}

/// Decode a 21-bit PC-relative immediate for ADR.
pub fn decode_adr_imm(word: u32) -> i64 {
    let immlo = ((word >> 29) & 0x3) as i64;
    let immhi = (((word >> 5) & 0x7FFFF) as i64) << 2;
    let imm = immhi | immlo;
    // Sign extend 21 bits
    if (imm & (1 << 20)) != 0 {
        imm | !((1 << 21) - 1)
    } else {
        imm
    }
}

/// Decode a 21-bit PC-relative immediate for ADRP (page-aligned).
pub fn decode_adrp_imm(word: u32) -> i64 {
    let immlo = ((word >> 29) & 0x3) as i64;
    let immhi = (((word >> 5) & 0x7FFFF) as i64) << 2;
    let imm = (immhi | immlo) << 12;
    // Sign extend 33 bits
    if (imm & (1 << 32)) != 0 {
        imm | !((1i64 << 33) - 1)
    } else {
        imm
    }
}

/// Decode a 19-bit conditional branch immediate (signed, scaled by 4).
pub fn decode_bcond_imm(word: u32) -> i64 {
    let imm = ((word >> 5) & 0x7FFFF) as i64;
    let imm = imm << 2;
    // Sign extend 21 bits
    if (imm & (1 << 20)) != 0 {
        imm | !((1 << 21) - 1)
    } else {
        imm
    }
}

/// Decode a 26-bit unconditional branch immediate (signed, scaled by 4).
pub fn decode_b_imm(word: u32) -> i64 {
    let imm = (word & 0x3FFFFFF) as i64;
    let imm = imm << 2;
    // Sign extend 28 bits
    if (imm & (1 << 27)) != 0 {
        imm | !((1 << 28) - 1)
    } else {
        imm
    }
}

/// Decode a 19-bit compare-and-branch / test-and-branch immediate.
pub fn decode_cbz_imm(word: u32) -> i64 {
    let imm = ((word >> 5) & 0x7FFFF) as i64;
    let imm = imm << 2;
    // Sign extend 21 bits
    if (imm & (1 << 20)) != 0 {
        imm | !((1 << 21) - 1)
    } else {
        imm
    }
}

/// Decode a bitmask immediate (N:immr:imms).
/// Returns the expanded 64-bit bitmask, or None for reserved encodings.
pub fn decode_bitmask_imm(n: u8, immr: u8, imms: u8, is_64bit: bool) -> Option<u64> {
    // See ARM ARM section "Bit Mask Encoding"
    let len = if n != 0 {
        // N=1: len = 6 + 1 = 7? No, len is the index of the highest set bit in N:NOT(imms)
        // Actually: len = highest_set_bit(concat(N, NOT(imms)))
        let not_imms = !imms & 0x3F;
        let concat = ((n as u32) << 6) | (not_imms as u32);
        32 - concat.leading_zeros()
    } else {
        let not_imms = !imms & 0x3F;
        if not_imms == 0 {
            return None; // Reserved
        }
        32 - ((not_imms as u32).leading_zeros())
    };

    if len < 1 || len > (if is_64bit { 6 } else { 5 }) {
        return None;
    }

    let size = 1u64 << len;
    let levels = size - 1;

    let s = (imms as u64) & levels;
    let r = (immr as u64) & levels;

    if s == levels {
        return None; // Reserved: all ones in field
    }

    // Compute the bitmask pattern
    let mut pattern = (1u64 << (s + 1)) - 1;
    // Rotate right by r
    pattern = pattern.rotate_right(r as u32);
    // Replicate to 64 bits
    let mut result = 0u64;
    let mut i = 0u64;
    while i < 64 {
        result |= pattern << i;
        i += size;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_code_roundtrip() {
        for i in 0..16u8 {
            let cc = ConditionCode::from_bits(i).unwrap();
            assert_eq!(ConditionCode::from_bits(i).unwrap(), cc);
        }
    }

    #[test]
    fn test_condition_code_capstone_aliases() {
        assert_eq!(ConditionCode::Cs.as_str_capstone(), "hs");
        assert_eq!(ConditionCode::Cc.as_str_capstone(), "lo");
    }

    #[test]
    fn test_imm12_decode() {
        // add x0, x1, #2: imm12 = 2, shift = 0
        let word = 0x91000820;
        let (imm, shift) = decode_imm12(word);
        assert_eq!(imm, 2);
        assert!(!shift);

        // add x0, x1, #0x1000: imm12 = 1, shift = 1
        let word = 0x91400420;
        let (imm, shift) = decode_imm12(word);
        assert_eq!(imm, 0x1000);
        assert!(shift);
    }

    #[test]
    fn test_imm16_hw_decode() {
        // movz x0, #0x1234
        let word = 0xD2824680;
        let (imm, shift) = decode_imm16_hw(word);
        assert_eq!(imm, 0x1234);
        assert_eq!(shift, 0);
    }

    #[test]
    fn test_bcond_imm_decode() {
        // b.eq #+0x40 (offset = 16 instructions = 64 bytes)
        // imm19 = 16, encoded at bits[23:5]
        let word = 0x54000000 | (16 << 5);
        let imm = decode_bcond_imm(word);
        assert_eq!(imm, 64);
    }

    #[test]
    fn test_b_imm_decode() {
        // b #+0x400 (offset = 256 instructions = 1024 bytes)
        let word = 0x14000100;
        let imm = decode_b_imm(word);
        assert_eq!(imm, 1024);
    }

    #[test]
    fn test_bitmask_imm() {
        // N=0, immr=0, imms=0b100000 (32) => valid 32-bit encoding, len=5
        let result = decode_bitmask_imm(0, 0, 32, false);
        assert!(result.is_some());

        // N=0, imms=0b111111 => reserved (all ones in NOT(imms))
        let result = decode_bitmask_imm(0, 0, 0b111111, false);
        assert!(result.is_none());

        // N=0, immr=0, imms=0 => len=6 which exceeds 5 for 32-bit, should be None
        let result = decode_bitmask_imm(0, 0, 0, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_extension_parsing() {
        let ext = AArch64Extensions::from_profile(&["base", "+fp", "+neon"]).unwrap();
        assert!(ext.contains(AArch64Extensions::BASE));
        assert!(ext.contains(AArch64Extensions::FP));
        assert!(ext.contains(AArch64Extensions::SIMD));
        assert!(!ext.contains(AArch64Extensions::SVE));
    }
}
