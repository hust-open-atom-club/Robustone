// Auto-generated comprehensive LoongArch instruction specs from Capstone YAML.
use robustone_isa::{ModeSet, SpecSeal};

pub static G_0000: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "adc.b",
        "ADC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003018A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0001: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "adc.d",
        "ADC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003198A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0002: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "adc.h",
        "ADC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003098A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0003: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "adc.w",
        "ADC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003118A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0004: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "add.w",
        "ADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00107C29),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0005: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "addi.w",
        "ADDI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0283D8E5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0006: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "addu12i.d",
        "ADDU12I_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002984A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0007: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "addu12i.w",
        "ADDU12I_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002904A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0008: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "alsl.w",
        "ALSL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00058A22),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0009: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "and",
        "AND",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001487F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0010: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "andi",
        "ANDI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0341A819),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0011: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "andn",
        "ANDN",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0016973C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0012: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armadc.w",
        "ARMADC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00381491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0013: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armadd.w",
        "ARMADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00371491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0014: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armand.w",
        "ARMAND_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00391491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0015: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armmfflag",
        "ARMMFFLAG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005C0444),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0016: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armmov.d",
        "ARMMOV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003FC49E),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0017: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armmov.w",
        "ARMMOV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003FC49D),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0018: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armmove",
        "ARMMOVE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003644A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0019: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armmtflag",
        "ARMMTFLAG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005C0464),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0020: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armnot.w",
        "ARMNOT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003FC49C),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0021: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armor.w",
        "ARMOR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00399491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0022: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armrotr.w",
        "ARMROTR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003C1491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0023: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armrotri.w",
        "ARMROTRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E0491),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0024: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armrrx.w",
        "ARMRRX_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003FC49F),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0025: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsbc.w",
        "ARMSBC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00389491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0026: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsll.w",
        "ARMSLL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003A9491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0027: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armslli.w",
        "ARMSLLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003C8491),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0028: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsra.w",
        "ARMSRA_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003B9491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0029: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsrai.w",
        "ARMSRAI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003D8491),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0030: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsrl.w",
        "ARMSRL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003B1491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0031: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsrli.w",
        "ARMSRLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003D0491),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0032: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armsub.w",
        "ARMSUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00379491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0033: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "armxor.w",
        "ARMXOR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003A1491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0034: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "b",
        "B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x5000F800),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0035: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bceqz",
        "BCEQZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x48000CC0),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0036: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bcnez",
        "BCNEZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x480049C0),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0037: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "beq",
        "BEQ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x5800B147),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0038: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "beqz",
        "BEQZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x40006120),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0039: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bge",
        "BGE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x6400958F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0040: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bgeu",
        "BGEU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x6C008CD7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0041: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bitrev.4b",
        "BITREV_4B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00004B75),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0042: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bitrev.w",
        "BITREV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x000050B9),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0043: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bl",
        "BL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x5400EC00),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0044: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "blt",
        "BLT",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x6000A9FE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0045: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bltu",
        "BLTU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x68000625),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0046: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bne",
        "BNE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x5C008B21),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0047: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bnez",
        "BNEZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x4400D460),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0048: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "break",
        "BREAK",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002A00C7),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0049: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bstrins.w",
        "BSTRINS_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00670968),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0050: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bstrpick.w",
        "BSTRPICK_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x006A9121),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0051: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "bytepick.w",
        "BYTEPICK_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0008401D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0052: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "cacop",
        "CACOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06006D40),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0053: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "clo.w",
        "CLO_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00001061),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0054: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "clz.w",
        "CLZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00001547),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0055: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "cpucfg",
        "CPUCFG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00006D03),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0056: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crc.w.b.w",
        "CRC_W_B_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002408F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0057: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crc.w.d.w",
        "CRC_W_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0025FD7C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0058: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crc.w.h.w",
        "CRC_W_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0024C95F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0059: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crc.w.w.w",
        "CRC_W_W_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002528DC),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0060: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crcc.w.b.w",
        "CRCC_W_B_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00260E4F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0061: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crcc.w.d.w",
        "CRCC_W_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0027EEBE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0062: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crcc.w.h.w",
        "CRCC_W_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0026CBB5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0063: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "crcc.w.w.w",
        "CRCC_W_W_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002735D1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0064: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "csrrd",
        "CSRRD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0400781A),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0065: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "csrwr",
        "CSRWR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x04030838),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0066: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "csrxchg",
        "CSRXCHG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x04035B66),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0067: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "cto.w",
        "CTO_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x000018C2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0068: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ctz.w",
        "CTZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00001EC5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0069: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "dbar",
        "DBAR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38720000),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0070: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "dbcl",
        "DBCL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002A80C9),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0071: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "div.w",
        "DIV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002065BE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0072: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "div.wu",
        "DIV_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002102F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0073: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ertn",
        "ERTN",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06483800),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0074: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fabs.d",
        "FABS_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01140877),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0075: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fabs.s",
        "FABS_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114059C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0076: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fadd.d",
        "FADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010134F9),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0077: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fadd.s",
        "FADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0100E5FD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0078: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fclass.d",
        "FCLASS_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01143853),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0079: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fclass.s",
        "FCLASS_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01143534),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0080: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.caf.d",
        "FCMP_CAF_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C200400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0081: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.caf.s",
        "FCMP_CAF_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C100400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0082: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.ceq.d",
        "FCMP_CEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C220400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0083: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.ceq.s",
        "FCMP_CEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C120400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0084: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cle.d",
        "FCMP_CLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C230400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0085: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cle.s",
        "FCMP_CLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C130400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0086: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.clt.d",
        "FCMP_CLT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C210400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0087: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.clt.s",
        "FCMP_CLT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C110400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0088: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cne.d",
        "FCMP_CNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C280400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0089: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cne.s",
        "FCMP_CNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C180400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0090: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cor.d",
        "FCMP_COR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C2A0400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0091: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cor.s",
        "FCMP_COR_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C1A0400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0092: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cueq.d",
        "FCMP_CUEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C260400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0093: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cueq.s",
        "FCMP_CUEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C160400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0094: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cule.d",
        "FCMP_CULE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C270400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0095: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cule.s",
        "FCMP_CULE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C170400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0096: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cult.d",
        "FCMP_CULT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C250400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0097: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cult.s",
        "FCMP_CULT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C150400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0098: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cun.d",
        "FCMP_CUN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C240400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0099: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cun.s",
        "FCMP_CUN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C140400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0100: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cune.d",
        "FCMP_CUNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C2C0400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0101: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.cune.s",
        "FCMP_CUNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C1C0400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0102: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.saf.d",
        "FCMP_SAF_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C208400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0103: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.saf.s",
        "FCMP_SAF_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C108400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0104: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.seq.d",
        "FCMP_SEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C228400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0105: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.seq.s",
        "FCMP_SEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C128400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0106: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sle.d",
        "FCMP_SLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C238400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0107: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sle.s",
        "FCMP_SLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C138400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0108: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.slt.d",
        "FCMP_SLT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C218400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0109: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.slt.s",
        "FCMP_SLT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C118400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0110: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sne.d",
        "FCMP_SNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C288400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0111: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sne.s",
        "FCMP_SNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C188400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0112: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sor.d",
        "FCMP_SOR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C2A8400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0113: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sor.s",
        "FCMP_SOR_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C1A8400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0114: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sueq.d",
        "FCMP_SUEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C268400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0115: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sueq.s",
        "FCMP_SUEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C168400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0116: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sule.d",
        "FCMP_SULE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C278400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0117: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sule.s",
        "FCMP_SULE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C178400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0118: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sult.d",
        "FCMP_SULT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C258400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0119: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sult.s",
        "FCMP_SULT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C158400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0120: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sun.d",
        "FCMP_SUN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C248400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0121: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sun.s",
        "FCMP_SUN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C148400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0122: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sune.d",
        "FCMP_SUNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C2C8400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0123: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcmp.sune.s",
        "FCMP_SUNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C1C8400),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0124: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcopysign.d",
        "FCOPYSIGN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01131B50),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0125: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcopysign.s",
        "FCOPYSIGN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0112DF0D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0126: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcvt.d.ld",
        "FCVT_D_LD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01150820),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0127: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcvt.d.s",
        "FCVT_D_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011924CA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0128: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcvt.ld.d",
        "FCVT_LD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114E020),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0129: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcvt.s.d",
        "FCVT_S_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01191A6C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0130: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fcvt.ud.d",
        "FCVT_UD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114E420),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0131: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fdiv.d",
        "FDIV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01077323),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0132: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fdiv.s",
        "FDIV_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0106CF14),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0133: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ffint.d.l",
        "FFINT_D_L",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011D2B57),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0134: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ffint.d.w",
        "FFINT_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011D2258),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0135: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ffint.s.l",
        "FFINT_S_L",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011D18A6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0136: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ffint.s.w",
        "FFINT_S_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011D10BE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0137: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fld.d",
        "FLD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2B81CA36),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0138: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fld.s",
        "FLD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2B03E9F7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0139: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldgt.d",
        "FLDGT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3874FCBA),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0140: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldgt.s",
        "FLDGT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38743763),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0141: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldle.d",
        "FLDLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3875D9E3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0142: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldle.s",
        "FLDLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387547B8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0143: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldx.d",
        "FLDX_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38347DBB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0144: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fldx.s",
        "FLDX_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38304DE1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0145: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "flogb.d",
        "FLOGB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01142BB5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0146: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "flogb.s",
        "FLOGB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011426FF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0147: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmadd.d",
        "FMADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x082C7315),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0148: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmadd.s",
        "FMADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x08178E03),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0149: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmax.d",
        "FMAX_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0109374B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0150: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmax.s",
        "FMAX_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0108ECD6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0151: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmaxa.d",
        "FMAXA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010D11B8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0152: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmaxa.s",
        "FMAXA_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010CFF69),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0153: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmin.d",
        "FMIN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010B6DA1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0154: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmin.s",
        "FMIN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010ACD4E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0155: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmina.d",
        "FMINA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010F0152),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0156: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmina.s",
        "FMINA_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x010E864F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0157: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmov.d",
        "FMOV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114993E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0158: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmov.s",
        "FMOV_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011496ED),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0159: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmsub.d",
        "FMSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x086DD246),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0160: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmsub.s",
        "FMSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x08525577),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0161: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmul.d",
        "FMUL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01051FC4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0162: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fmul.s",
        "FMUL_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0104C4E0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0163: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fneg.d",
        "FNEG_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01141B4B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0164: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fneg.s",
        "FNEG_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01141715),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0165: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fnmadd.d",
        "FNMADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x08AF4DB9),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0166: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fnmadd.s",
        "FNMADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x089A603D),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0167: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fnmsub.d",
        "FNMSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x08EC1F5E),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0168: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fnmsub.s",
        "FNMSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x08DCE088),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0169: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frecip.d",
        "FRECIP_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01145B7B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0170: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frecip.s",
        "FRECIP_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01145771),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0171: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frecipe.d",
        "FRECIPE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01147800),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0172: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frecipe.s",
        "FRECIPE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01147400),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0173: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frint.d",
        "FRINT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011E485D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0174: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frint.s",
        "FRINT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011E4625),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0175: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frsqrt.d",
        "FRSQRT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01146876),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0176: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frsqrt.s",
        "FRSQRT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01146599),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0177: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frsqrte.d",
        "FRSQRTE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01148821),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0178: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "frsqrte.s",
        "FRSQRTE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01148421),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0179: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fscaleb.d",
        "FSCALEB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011169CC),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0180: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fscaleb.s",
        "FSCALEB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01109AF5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0181: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fsel",
        "FSEL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0D025692),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0182: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fsqrt.d",
        "FSQRT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x01144962),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0183: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fsqrt.s",
        "FSQRT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114465B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0184: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fst.d",
        "FST_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2BC318FC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0185: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fst.s",
        "FST_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2B439A7E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0186: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstgt.d",
        "FSTGT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3876E96D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0187: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstgt.s",
        "FSTGT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387679BF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0188: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstle.d",
        "FSTLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3877B532),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0189: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstle.s",
        "FSTLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38771DAD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0190: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstx.d",
        "FSTX_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x383C45E6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0191: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fstx.s",
        "FSTX_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3838587A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0192: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fsub.d",
        "FSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0103483D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0193: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "fsub.s",
        "FSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0102FCCE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0194: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftint.l.d",
        "FTINT_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011B2B10),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0195: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftint.l.s",
        "FTINT_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011B271F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0196: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftint.w.d",
        "FTINT_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011B09C3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0197: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftint.w.s",
        "FTINT_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011B05B5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0198: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrm.l.d",
        "FTINTRM_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A2929),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0199: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrm.l.s",
        "FTINTRM_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A2558),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0200: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrm.w.d",
        "FTINTRM_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A0907),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0201: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrm.w.s",
        "FTINTRM_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A0610),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0202: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrne.l.d",
        "FTINTRNE_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AE8DC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0203: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrne.l.s",
        "FTINTRNE_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AE776),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0204: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrne.w.d",
        "FTINTRNE_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AC99F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0205: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrne.w.s",
        "FTINTRNE_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AC624),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0206: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrp.l.d",
        "FTINTRP_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A6BA4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0207: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrp.l.s",
        "FTINTRP_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A6600),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0208: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrp.w.d",
        "FTINTRP_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A486C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0209: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrp.w.s",
        "FTINTRP_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A47EE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0210: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrz.l.d",
        "FTINTRZ_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AA943),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0211: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrz.l.s",
        "FTINTRZ_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011AA4B7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0212: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrz.w.d",
        "FTINTRZ_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A8B19),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0213: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ftintrz.w.s",
        "FTINTRZ_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x011A87A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0214: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "gcsrrd",
        "GCSRRD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x05000404),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0215: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "gcsrwr",
        "GCSRWR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x05000424),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0216: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "gcsrxchg",
        "GCSRXCHG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x050004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0217: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "gtlbflush",
        "GTLBFLUSH",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06482401),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0218: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "hvcl",
        "HVCL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002B8001),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0219: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ibar",
        "IBAR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38728000),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0220: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "idle",
        "IDLE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x064880CC),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0221: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "invtlb",
        "INVTLB",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0649E7B0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0222: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrrd.b",
        "IOCSRRD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0648031A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0223: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrrd.h",
        "IOCSRRD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06480765),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0224: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrrd.w",
        "IOCSRRD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06480A8A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0225: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrwr.b",
        "IOCSRWR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x064812E4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0226: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrwr.h",
        "IOCSRWR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0648140B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0227: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "iocsrwr.w",
        "IOCSRWR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06481B54),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0228: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "jirl",
        "JIRL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x4C000481),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0229: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "jiscr0",
        "JISCR0",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x48006600),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0230: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "jiscr1",
        "JISCR1",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x48006700),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0231: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ld.b",
        "LD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x28005518),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0232: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ld.bu",
        "LD_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2A0259AD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0233: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ld.h",
        "LD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x28414247),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0234: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ld.hu",
        "LD_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2A431BB2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0235: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ld.w",
        "LD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x28817352),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0236: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "lddir",
        "LDDIR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x064173CC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0237: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldgt.b",
        "LDGT_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387874C6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0238: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldgt.d",
        "LDGT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3879FF37),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0239: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldgt.h",
        "LDGT_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387887E5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0240: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldgt.w",
        "LDGT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3879234F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0241: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldl.d",
        "LDL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2E8004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0242: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldl.w",
        "LDL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2E0004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0243: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldle.b",
        "LDLE_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387A3D89),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0244: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldle.d",
        "LDLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387BC1F4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0245: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldle.h",
        "LDLE_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387ADD6B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0246: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldle.w",
        "LDLE_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387B0858),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0247: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldpte",
        "LDPTE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06472240),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0248: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldr.d",
        "LDR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2EC004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0249: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ldr.w",
        "LDR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2E4004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0250: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ll.w",
        "LL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2000DF62),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0251: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "llacq.w",
        "LLACQ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x385781CD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0252: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "lu12i.w",
        "LU12I_W",
        robustone_isa::mask_value!(0xFE00062B, 0x14000620),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0253: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "maskeqz",
        "MASKEQZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00134974),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0254: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "masknez",
        "MASKNEZ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0013E9B4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0255: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "mod.w",
        "MOD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0020AB41),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0256: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "mod.wu",
        "MOD_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0021C53B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0257: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movcf2fr",
        "MOVCF2FR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114D410),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0258: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movcf2gr",
        "MOVCF2GR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114DCF5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0259: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "move",
        "MOVE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00150128),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0260: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movfcsr2gr",
        "MOVFCSR2GR",
        robustone_isa::mask_value!(0xFFFFFF9F, 0x0114C804),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0261: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movfr2cf",
        "MOVFR2CF",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114D164),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0262: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movfr2gr.s",
        "MOVFR2GR_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114B6CA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0263: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movgr2cf",
        "MOVGR2CF",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114D825),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0264: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movgr2fcsr",
        "MOVGR2FCSR",
        robustone_isa::mask_value!(0xFFFFFFFC, 0x0114C080),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0265: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movgr2fr.w",
        "MOVGR2FR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0114A446),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0266: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movgr2scr",
        "MOVGR2SCR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x000008A0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0267: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "movscr2gr",
        "MOVSCR2GR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00000C24),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0268: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "mul.w",
        "MUL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001C0E44),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0269: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "mulh.w",
        "MULH_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001C82FB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0270: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "mulh.wu",
        "MULH_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001D622A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0271: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "nop",
        "NOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x03400000),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0272: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "nor",
        "NOR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00141645),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0273: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "or",
        "OR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00157A11),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0274: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "ori",
        "ORI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0380BCB1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0275: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "orn",
        "ORN",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00166462),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0276: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "pcaddi",
        "PCADDI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x18001769),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0277: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "pcaddu12i",
        "PCADDU12I",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x1C0004A0),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0278: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "pcalau12i",
        "PCALAU12I",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x1A000B2A),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0279: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "preld",
        "PRELD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2AC05C0A),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0280: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcr.b",
        "RCR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003418A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0281: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcr.d",
        "RCR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003598A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0282: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcr.h",
        "RCR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003498A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0283: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcr.w",
        "RCR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003518A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0284: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcri.b",
        "RCRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005024A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0285: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcri.d",
        "RCRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005104A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0286: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcri.h",
        "RCRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005044A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0287: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rcri.w",
        "RCRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005084A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0288: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rdtimeh.w",
        "RDTIMEH_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x000064AB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0289: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rdtimel.w",
        "RDTIMEL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00006098),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0290: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "revb.2h",
        "REVB_2H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00003174),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0291: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotr.b",
        "ROTR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001A18A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0292: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotr.h",
        "ROTR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001A98A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0293: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotr.w",
        "ROTR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x001B4B41),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0294: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotri.b",
        "ROTRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x004C24A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0295: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotri.h",
        "ROTRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x004C44A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0296: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "rotri.w",
        "ROTRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x004CDE97),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0297: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sbc.b",
        "SBC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003218A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0298: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sbc.d",
        "SBC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003398A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0299: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sbc.h",
        "SBC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003298A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0300: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sbc.w",
        "SBC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003318A4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0301: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sc.w",
        "SC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x210039D3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0302: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "screl.w",
        "SCREL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x385785CD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0303: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "setarmj",
        "SETARMJ",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0036C404),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0304: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "setx86j",
        "SETX86J",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00368404),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0305: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "setx86loope",
        "SETX86LOOPE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x000078A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0306: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "setx86loopne",
        "SETX86LOOPNE",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00007CA4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0307: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sll.w",
        "SLL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00175F78),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0308: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "slli.w",
        "SLLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0040825A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0309: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "slt",
        "SLT",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00120B5D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0310: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "slti",
        "SLTI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0203AC3B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0311: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sltu",
        "SLTU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0012F6AB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0312: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sltui",
        "SLTUI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x02428900),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0313: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sra.w",
        "SRA_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00182B8C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0314: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "srai.w",
        "SRAI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0048E228),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0315: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "srl.w",
        "SRL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00179E3F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0316: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "srli.w",
        "SRLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0044F9CA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0317: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "st.b",
        "ST_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x29017CE3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0318: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "st.h",
        "ST_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2941EA19),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0319: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "st.w",
        "ST_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2982BDAD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0320: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stgt.b",
        "STGT_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387C527B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0321: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stgt.d",
        "STGT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387DE2BE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0322: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stgt.h",
        "STGT_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387C9890),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0323: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stgt.w",
        "STGT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387D3B9F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0324: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stl.d",
        "STL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2F8004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0325: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stl.w",
        "STL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2F0004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0326: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stle.b",
        "STLE_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387E408A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0327: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stle.d",
        "STLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387FF719),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0328: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stle.h",
        "STLE_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387ED631),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0329: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "stle.w",
        "STLE_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x387F7797),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0330: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "str.d",
        "STR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2FC004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0331: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "str.w",
        "STR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2F4004A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0332: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "sub.w",
        "SUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00114F35),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0333: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "syscall",
        "SYSCALL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x002B0064),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0334: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbclr",
        "TLBCLR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06482000),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0335: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbfill",
        "TLBFILL",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06483400),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0336: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbflush",
        "TLBFLUSH",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06482400),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0337: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbrd",
        "TLBRD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06482C00),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0338: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbsrch",
        "TLBSRCH",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06482800),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0339: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "tlbwr",
        "TLBWR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x06483000),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0340: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.d.b",
        "VEXT2XV_D_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F1B39),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0341: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.d.h",
        "VEXT2XV_D_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F2268),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0342: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.d.w",
        "VEXT2XV_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F2724),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0343: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.du.bu",
        "VEXT2XV_DU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F332C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0344: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.du.hu",
        "VEXT2XV_DU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F38D2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0345: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.du.wu",
        "VEXT2XV_DU_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F3EAA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0346: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.h.b",
        "VEXT2XV_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F127E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0347: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.hu.bu",
        "VEXT2XV_HU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F2999),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0348: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.w.b",
        "VEXT2XV_W_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F14BB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0349: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.w.h",
        "VEXT2XV_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F1E94),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0350: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.wu.bu",
        "VEXT2XV_WU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F2DBF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0351: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vext2xv.wu.hu",
        "VEXT2XV_WU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F3597),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0352: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vreplvei.b",
        "VREPLVEI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x72F78C77),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0353: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vreplvei.d",
        "VREPLVEI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x72F7F58F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0354: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vreplvei.h",
        "VREPLVEI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x72F7C21B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0355: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "vreplvei.w",
        "VREPLVEI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x72F7EEF2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0356: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86adc.b",
        "X86ADC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0357: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86adc.d",
        "X86ADC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0358: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86adc.h",
        "X86ADC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0359: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86adc.w",
        "X86ADC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0360: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.b",
        "X86ADD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1484),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0361: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.d",
        "X86ADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1487),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0362: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.du",
        "X86ADD_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1481),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0363: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.h",
        "X86ADD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1485),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0364: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.w",
        "X86ADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1486),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0365: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86add.wu",
        "X86ADD_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1480),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0366: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86and.b",
        "X86AND_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9490),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0367: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86and.d",
        "X86AND_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9493),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0368: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86and.h",
        "X86AND_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0369: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86and.w",
        "X86AND_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9492),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0370: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86clrtm",
        "X86CLRTM",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008028),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0371: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86dec.b",
        "X86DEC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008084),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0372: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86dec.d",
        "X86DEC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008087),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0373: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86dec.h",
        "X86DEC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008085),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0374: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86dec.w",
        "X86DEC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008086),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0375: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86dectop",
        "X86DECTOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008029),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0376: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86inc.b",
        "X86INC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008080),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0377: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86inc.d",
        "X86INC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008083),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0378: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86inc.h",
        "X86INC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008081),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0379: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86inc.w",
        "X86INC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008082),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0380: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86inctop",
        "X86INCTOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008009),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0381: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mfflag",
        "X86MFFLAG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005C0404),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0382: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mftop",
        "X86MFTOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00007404),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0383: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mtflag",
        "X86MTFLAG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x005C0424),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0384: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mttop",
        "X86MTTOP",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00007020),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0385: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.b",
        "X86MUL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9480),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0386: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.bu",
        "X86MUL_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9484),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0387: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.d",
        "X86MUL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9483),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0388: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.du",
        "X86MUL_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9487),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0389: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.h",
        "X86MUL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9481),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0390: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.hu",
        "X86MUL_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9485),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0391: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.w",
        "X86MUL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9482),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0392: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86mul.wu",
        "X86MUL_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003E9486),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0393: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86or.b",
        "X86OR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9494),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0394: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86or.d",
        "X86OR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9497),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0395: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86or.h",
        "X86OR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9495),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0396: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86or.w",
        "X86OR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9496),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0397: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcl.b",
        "X86RCL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0398: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcl.d",
        "X86RCL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0399: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcl.h",
        "X86RCL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0400: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcl.w",
        "X86RCL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0401: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcli.b",
        "X86RCLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542498),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0402: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcli.d",
        "X86RCLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0055049B),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0403: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcli.h",
        "X86RCLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544499),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0404: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcli.w",
        "X86RCLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0054849A),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0405: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcr.b",
        "X86RCR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9488),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0406: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcr.d",
        "X86RCR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0407: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcr.h",
        "X86RCR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9489),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0408: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcr.w",
        "X86RCR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F948A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0409: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcri.b",
        "X86RCRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542490),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0410: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcri.d",
        "X86RCRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00550493),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0411: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcri.h",
        "X86RCRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544491),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0412: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rcri.w",
        "X86RCRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00548492),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0413: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotl.b",
        "X86ROTL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9484),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0414: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotl.d",
        "X86ROTL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9487),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0415: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotl.h",
        "X86ROTL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9485),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0416: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotl.w",
        "X86ROTL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9486),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0417: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotli.b",
        "X86ROTLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542494),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0418: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotli.d",
        "X86ROTLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00550497),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0419: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotli.h",
        "X86ROTLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544495),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0420: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotli.w",
        "X86ROTLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00548496),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0421: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotr.b",
        "X86ROTR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9480),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0422: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotr.d",
        "X86ROTR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9482),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0423: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotr.h",
        "X86ROTR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9481),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0424: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotr.w",
        "X86ROTR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9483),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0425: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotri.b",
        "X86ROTRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0054248C),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0426: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotri.d",
        "X86ROTRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0055048F),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0427: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotri.h",
        "X86ROTRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0054448D),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0428: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86rotri.w",
        "X86ROTRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0054848E),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0429: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sbc.b",
        "X86SBC_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1490),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0430: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sbc.d",
        "X86SBC_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1493),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0431: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sbc.h",
        "X86SBC_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1491),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0432: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sbc.w",
        "X86SBC_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1492),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0433: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86settag",
        "X86SETTAG",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00580424),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0434: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86settm",
        "X86SETTM",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00008008),
        &FMT_NONE,
        &[],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0435: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sll.b",
        "X86SLL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1494),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0436: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sll.d",
        "X86SLL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1497),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0437: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sll.h",
        "X86SLL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1495),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0438: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sll.w",
        "X86SLL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1496),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0439: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86slli.b",
        "X86SLLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542480),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0440: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86slli.d",
        "X86SLLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00550483),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0441: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86slli.h",
        "X86SLLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544481),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0442: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86slli.w",
        "X86SLLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00548482),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0443: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sra.b",
        "X86SRA_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0444: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sra.d",
        "X86SRA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0445: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sra.h",
        "X86SRA_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0446: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sra.w",
        "X86SRA_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0447: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srai.b",
        "X86SRAI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542488),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0448: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srai.d",
        "X86SRAI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0055048B),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0449: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srai.h",
        "X86SRAI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544489),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0450: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srai.w",
        "X86SRAI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0054848A),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0451: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srl.b",
        "X86SRL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1498),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0452: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srl.d",
        "X86SRL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0453: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srl.h",
        "X86SRL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1499),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0454: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srl.w",
        "X86SRL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F149A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0455: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srli.b",
        "X86SRLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00542484),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0456: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srli.d",
        "X86SRLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00550487),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0457: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srli.h",
        "X86SRLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00544485),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0458: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86srli.w",
        "X86SRLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x00548486),
        &FMT_R1,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0459: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.b",
        "X86SUB_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1488),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0460: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.d",
        "X86SUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0461: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.du",
        "X86SUB_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1483),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0462: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.h",
        "X86SUB_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1489),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0463: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.w",
        "X86SUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F148A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0464: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86sub.wu",
        "X86SUB_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F1482),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0465: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86xor.b",
        "X86XOR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9498),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0466: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86xor.d",
        "X86XOR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F949B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0467: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86xor.h",
        "X86XOR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F9499),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0468: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "x86xor.w",
        "X86XOR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x003F949A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0469: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xor",
        "XOR",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0015A26F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0470: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xori",
        "XORI",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x03C18EF2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0471: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.b",
        "XVABSD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74604436),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0472: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.bu",
        "XVABSD_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74623C90),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0473: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.d",
        "XVABSD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7461CEFE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0474: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.du",
        "XVABSD_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7463915A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0475: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.h",
        "XVABSD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7460A711),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0476: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.hu",
        "XVABSD_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7462EEED),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0477: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.w",
        "XVABSD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7461753C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0478: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvabsd.wu",
        "XVABSD_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74633E5F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0479: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadd.b",
        "XVADD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740A1674),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0480: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadd.d",
        "XVADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740BB4D3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0481: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadd.h",
        "XVADD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740AB8F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0482: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadd.q",
        "XVADD_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752D1B84),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0483: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadd.w",
        "XVADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740B5433),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0484: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadda.b",
        "XVADDA_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745C6F0A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0485: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadda.d",
        "XVADDA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745DE42A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0486: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadda.h",
        "XVADDA_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745CF780),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0487: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvadda.w",
        "XVADDA_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745D253F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0488: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddi.bu",
        "XVADDI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768A0AC1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0489: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddi.du",
        "XVADDI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768B9C06),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0490: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddi.hu",
        "XVADDI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768AF543),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0491: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddi.wu",
        "XVADDI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768B0D65),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0492: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.d.w",
        "XVADDWEV_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x741F6528),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0493: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.d.wu",
        "XVADDWEV_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x742F5210),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0494: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.d.wu.w",
        "XVADDWEV_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x743F21A0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0495: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.h.b",
        "XVADDWEV_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x741E13D7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0496: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.h.bu",
        "XVADDWEV_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x742E69BE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0497: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.h.bu.b",
        "XVADDWEV_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x743E24E3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0498: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.q.d",
        "XVADDWEV_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x741FF6DD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0499: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.q.du",
        "XVADDWEV_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x742FCA4A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0500: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.q.du.d",
        "XVADDWEV_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x743F8D53),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0501: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.w.h",
        "XVADDWEV_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x741EFE74),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0502: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.w.hu",
        "XVADDWEV_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x742EC3EF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0503: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwev.w.hu.h",
        "XVADDWEV_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x743EEE1A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0504: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.d.w",
        "XVADDWOD_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7423512C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0505: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.d.wu",
        "XVADDWOD_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74332E7A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0506: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.d.wu.w",
        "XVADDWOD_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74417F8C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0507: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.h.b",
        "XVADDWOD_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x742262AE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0508: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.h.bu",
        "XVADDWOD_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x743224C6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0509: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.h.bu.b",
        "XVADDWOD_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74406355),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0510: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.q.d",
        "XVADDWOD_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7423A04B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0511: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.q.du",
        "XVADDWOD_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7433A2D5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0512: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.q.du.d",
        "XVADDWOD_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7441B09D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0513: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.w.h",
        "XVADDWOD_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7422DF53),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0514: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.w.hu",
        "XVADDWOD_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7432E761),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0515: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvaddwod.w.hu.h",
        "XVADDWOD_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7440C0DF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0516: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvand.v",
        "XVAND_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75264EEE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0517: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvandi.b",
        "XVANDI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77D108EB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0518: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvandn.v",
        "XVANDN_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75280DE3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0519: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.b",
        "XVAVG_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746457C5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0520: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.bu",
        "XVAVG_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7466408B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0521: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.d",
        "XVAVG_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7465EC1B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0522: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.du",
        "XVAVG_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7467F697),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0523: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.h",
        "XVAVG_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7464D632),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0524: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.hu",
        "XVAVG_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7466CC22),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0525: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.w",
        "XVAVG_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746552E3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0526: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavg.wu",
        "XVAVG_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74676E9B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0527: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.b",
        "XVAVGR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74681DFD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0528: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.bu",
        "XVAVGR_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746A6456),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0529: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.d",
        "XVAVGR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746982FD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0530: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.du",
        "XVAVGR_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746BB562),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0531: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.h",
        "XVAVGR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7468BF40),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0532: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.hu",
        "XVAVGR_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746AD559),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0533: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.w",
        "XVAVGR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74690017),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0534: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvavgr.wu",
        "XVAVGR_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x746B0DD1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0535: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclr.b",
        "XVBITCLR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750C38B8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0536: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclr.d",
        "XVBITCLR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750DE4AE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0537: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclr.h",
        "XVBITCLR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750CB53E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0538: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclr.w",
        "XVBITCLR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750D1C62),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0539: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclri.b",
        "XVBITCLRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77103F56),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0540: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclri.d",
        "XVBITCLRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77111D8A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0541: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclri.h",
        "XVBITCLRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x771075C2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0542: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitclri.w",
        "XVBITCLRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77108043),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0543: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrev.b",
        "XVBITREV_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75100E90),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0544: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrev.d",
        "XVBITREV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7511EC2D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0545: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrev.h",
        "XVBITREV_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7510D070),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0546: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrev.w",
        "XVBITREV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75115F58),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0547: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrevi.b",
        "XVBITREVI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77183567),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0548: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrevi.d",
        "XVBITREVI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77192461),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0549: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrevi.h",
        "XVBITREVI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77187CA1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0550: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitrevi.w",
        "XVBITREVI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7718CAAD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0551: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitsel.v",
        "XVBITSEL_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0D2ABFB2),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0552: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitseli.b",
        "XVBITSELI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77C5E6AD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0553: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitset.b",
        "XVBITSET_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750E7206),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0554: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitset.d",
        "XVBITSET_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750FB204),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0555: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitset.h",
        "XVBITSET_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750EFDA5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0556: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitset.w",
        "XVBITSET_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750F2387),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0557: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitseti.b",
        "XVBITSETI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7714207A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0558: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitseti.d",
        "XVBITSETI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x771508F4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0559: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitseti.h",
        "XVBITSETI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77146669),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0560: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbitseti.w",
        "XVBITSETI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77148A6C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0561: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbsll.v",
        "XVBSLL_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768E52AE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0562: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvbsrl.v",
        "XVBSRL_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768EF4A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0563: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclo.b",
        "XVCLO_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C0189),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0564: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclo.d",
        "XVCLO_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C0CBF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0565: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclo.h",
        "XVCLO_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C05D0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0566: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclo.w",
        "XVCLO_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C0A5E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0567: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclz.b",
        "XVCLZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C10C5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0568: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclz.d",
        "XVCLZ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C1C01),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0569: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclz.h",
        "XVCLZ_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C14E4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0570: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvclz.w",
        "XVCLZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C180C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0571: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.b",
        "XVDIV_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E02329),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0572: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.bu",
        "XVDIV_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E47AC0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0573: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.d",
        "XVDIV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E1B35B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0574: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.du",
        "XVDIV_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E59F27),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0575: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.h",
        "XVDIV_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E0EC32),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0576: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.hu",
        "XVDIV_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E4E6FF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0577: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.w",
        "XVDIV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E16F45),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0578: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvdiv.wu",
        "XVDIV_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E51F21),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0579: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.d.w",
        "XVEXTH_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EEB62),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0580: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.du.wu",
        "XVEXTH_DU_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EFB3B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0581: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.h.b",
        "XVEXTH_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EE14F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0582: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.hu.bu",
        "XVEXTH_HU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EF3D5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0583: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.q.d",
        "XVEXTH_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EEF36),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0584: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.qu.du",
        "XVEXTH_QU_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EFF90),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0585: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.w.h",
        "XVEXTH_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EE57A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0586: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvexth.wu.hu",
        "XVEXTH_WU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EF57C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0587: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextl.q.d",
        "XVEXTL_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7709019D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0588: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextl.qu.du",
        "XVEXTL_QU_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x770D029B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0589: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextrins.b",
        "XVEXTRINS_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x778FF2FE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0590: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextrins.d",
        "XVEXTRINS_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77821FDF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0591: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextrins.h",
        "XVEXTRINS_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x778B21A0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0592: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvextrins.w",
        "XVEXTRINS_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x778662AE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0593: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfadd.d",
        "XVFADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7531051B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0594: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfadd.s",
        "XVFADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7530BEA6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0595: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfclass.d",
        "XVFCLASS_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CD956),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0596: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfclass.s",
        "XVFCLASS_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CD4E3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0597: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.caf.d",
        "XVFCMP_CAF_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA053F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0598: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.caf.s",
        "XVFCMP_CAF_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C907D01),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0599: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.ceq.d",
        "XVFCMP_CEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA252FD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0600: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.ceq.s",
        "XVFCMP_CEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C920020),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0601: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cle.d",
        "XVFCMP_CLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA33335),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0602: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cle.s",
        "XVFCMP_CLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C933ED6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0603: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.clt.d",
        "XVFCMP_CLT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA15493),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0604: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.clt.s",
        "XVFCMP_CLT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C910524),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0605: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cne.d",
        "XVFCMP_CNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA80332),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0606: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cne.s",
        "XVFCMP_CNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C986A27),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0607: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cor.d",
        "XVFCMP_COR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CAA5E6C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0608: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cor.s",
        "XVFCMP_COR_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C9A3841),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0609: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cueq.d",
        "XVFCMP_CUEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA61EC4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0610: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cueq.s",
        "XVFCMP_CUEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C967DA5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0611: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cule.d",
        "XVFCMP_CULE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA72CA0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0612: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cule.s",
        "XVFCMP_CULE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C977441),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0613: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cult.d",
        "XVFCMP_CULT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA51A34),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0614: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cult.s",
        "XVFCMP_CULT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C950E2F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0615: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cun.d",
        "XVFCMP_CUN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA472D3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0616: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cun.s",
        "XVFCMP_CUN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C947528),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0617: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cune.d",
        "XVFCMP_CUNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CAC33D4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0618: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.cune.s",
        "XVFCMP_CUNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C9C1235),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0619: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.saf.d",
        "XVFCMP_SAF_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA09D87),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0620: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.saf.s",
        "XVFCMP_SAF_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C908977),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0621: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.seq.d",
        "XVFCMP_SEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA28ECF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0622: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.seq.s",
        "XVFCMP_SEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C92EEEF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0623: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sle.d",
        "XVFCMP_SLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA3DC23),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0624: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sle.s",
        "XVFCMP_SLE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C93C0A1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0625: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.slt.d",
        "XVFCMP_SLT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA1E351),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0626: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.slt.s",
        "XVFCMP_SLT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C91FE59),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0627: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sne.d",
        "XVFCMP_SNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA8C694),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0628: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sne.s",
        "XVFCMP_SNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C98F99B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0629: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sor.d",
        "XVFCMP_SOR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CAA9B86),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0630: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sor.s",
        "XVFCMP_SOR_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C9A89AB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0631: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sueq.d",
        "XVFCMP_SUEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA6C645),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0632: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sueq.s",
        "XVFCMP_SUEQ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C96A74C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0633: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sule.d",
        "XVFCMP_SULE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA7C54B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0634: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sule.s",
        "XVFCMP_SULE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C978577),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0635: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sult.d",
        "XVFCMP_SULT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA59484),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0636: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sult.s",
        "XVFCMP_SULT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C95C9E8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0637: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sun.d",
        "XVFCMP_SUN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CA4F964),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0638: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sun.s",
        "XVFCMP_SUN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C94F8E0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0639: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sune.d",
        "XVFCMP_SUNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0CACECBE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0640: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcmp.sune.s",
        "XVFCMP_SUNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0C9CA20B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0641: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvt.h.s",
        "XVFCVT_H_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75465E29),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0642: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvt.s.d",
        "XVFCVT_S_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7546F55B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0643: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvth.d.s",
        "XVFCVTH_D_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769DF63D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0644: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvth.s.h",
        "XVFCVTH_S_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769DEF29),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0645: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvtl.d.s",
        "XVFCVTL_D_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769DF0B8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0646: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfcvtl.s.h",
        "XVFCVTL_S_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769DE9D0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0647: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfdiv.d",
        "XVFDIV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753B795F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0648: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfdiv.s",
        "XVFDIV_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753AB0BD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0649: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffint.d.l",
        "XVFFINT_D_L",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E0A65),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0650: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffint.d.lu",
        "XVFFINT_D_LU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E0FBF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0651: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffint.s.l",
        "XVFFINT_S_L",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75480F6A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0652: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffint.s.w",
        "XVFFINT_S_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E00A3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0653: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffint.s.wu",
        "XVFFINT_S_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E0783),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0654: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffinth.d.w",
        "XVFFINTH_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E1787),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0655: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvffintl.d.w",
        "XVFFINTL_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E10E2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0656: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvflogb.d",
        "XVFLOGB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CC83A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0657: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvflogb.s",
        "XVFLOGB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CC591),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0658: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmadd.d",
        "XVFMADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0A2CFE09),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0659: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmadd.s",
        "XVFMADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0A1DFFE5),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0660: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmax.d",
        "XVFMAX_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753D5F3F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0661: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmax.s",
        "XVFMAX_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753CA31D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0662: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmaxa.d",
        "XVFMAXA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75417682),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0663: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmaxa.s",
        "XVFMAXA_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7540964F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0664: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmin.d",
        "XVFMIN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753F67CD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0665: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmin.s",
        "XVFMIN_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x753EC0BF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0666: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmina.d",
        "XVFMINA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75434A8C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0667: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmina.s",
        "XVFMINA_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7542C77D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0668: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmsub.d",
        "XVFMSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0A6741FE),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0669: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmsub.s",
        "XVFMSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0A5B8C71),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0670: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmul.d",
        "XVFMUL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75394F5C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0671: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfmul.s",
        "XVFMUL_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7538F9C9),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0672: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfnmadd.d",
        "XVFNMADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0AA65FC1),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0673: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfnmadd.s",
        "XVFNMADD_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0A9C5ECE),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0674: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfnmsub.d",
        "XVFNMSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0AEE7408),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0675: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfnmsub.s",
        "XVFNMSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0AD590B6),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0676: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrecip.d",
        "XVFRECIP_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CFB11),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0677: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrecip.s",
        "XVFRECIP_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CF603),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0678: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrecipe.d",
        "XVFRECIPE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D1B11),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0679: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrecipe.s",
        "XVFRECIPE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D1603),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0680: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrint.d",
        "XVFRINT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D3A5F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0681: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrint.s",
        "XVFRINT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D3715),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0682: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrm.d",
        "XVFRINTRM_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D4B6E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0683: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrm.s",
        "XVFRINTRM_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D45BB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0684: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrne.d",
        "XVFRINTRNE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D7BAC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0685: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrne.s",
        "XVFRINTRNE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D7633),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0686: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrp.d",
        "XVFRINTRP_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D5B81),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0687: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrp.s",
        "XVFRINTRP_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D561A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0688: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrz.d",
        "XVFRINTRZ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D68BD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0689: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrintrz.s",
        "XVFRINTRZ_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D652A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0690: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrsqrt.d",
        "XVFRSQRT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D0ACE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0691: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrsqrt.s",
        "XVFRSQRT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D073F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0692: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrsqrte.d",
        "XVFRSQRTE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D2ACE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0693: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrsqrte.s",
        "XVFRSQRTE_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769D273F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0694: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrstp.b",
        "XVFRSTP_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752B4A57),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0695: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrstp.h",
        "XVFRSTP_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752B9BCD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0696: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrstpi.b",
        "XVFRSTPI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769A7F98),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0697: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfrstpi.h",
        "XVFRSTPI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769ACB16),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0698: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfsqrt.d",
        "XVFSQRT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CE85A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0699: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfsqrt.s",
        "XVFSQRT_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CE764),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0700: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfsub.d",
        "XVFSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75333F24),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0701: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvfsub.s",
        "XVFSUB_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75328C16),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0702: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftint.l.d",
        "XVFTINT_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E36C7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0703: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftint.lu.d",
        "XVFTINT_LU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E5C42),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0704: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftint.w.d",
        "XVFTINT_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7549F6C7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0705: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftint.w.s",
        "XVFTINT_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E332B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0706: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftint.wu.s",
        "XVFTINT_WU_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E58CE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0707: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftinth.l.s",
        "XVFTINTH_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E84AF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0708: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintl.l.s",
        "XVFTINTL_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E817F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0709: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrm.l.d",
        "XVFTINTRM_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E3E2C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0710: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrm.w.d",
        "XVFTINTRM_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x754A1EFD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0711: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrm.w.s",
        "XVFTINTRM_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E3AE8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0712: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrmh.l.s",
        "XVFTINTRMH_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E8E6A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0713: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrml.l.s",
        "XVFTINTRML_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E89F6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0714: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrne.l.d",
        "XVFTINTRNE_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E55DE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0715: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrne.w.d",
        "XVFTINTRNE_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x754B968D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0716: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrne.w.s",
        "XVFTINTRNE_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E51B4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0717: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrneh.l.s",
        "XVFTINTRNEH_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EA7B0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0718: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrnel.l.s",
        "XVFTINTRNEL_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769EA39F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0719: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrp.l.d",
        "XVFTINTRP_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E470A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0720: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrp.w.d",
        "XVFTINTRP_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x754AFF4E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0721: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrp.w.s",
        "XVFTINTRP_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E4032),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0722: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrph.l.s",
        "XVFTINTRPH_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E9417),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0723: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrpl.l.s",
        "XVFTINTRPL_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E900E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0724: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrz.l.d",
        "XVFTINTRZ_L_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E4F41),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0725: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrz.lu.d",
        "XVFTINTRZ_LU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E7478),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0726: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrz.w.d",
        "XVFTINTRZ_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x754B6D0D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0727: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrz.w.s",
        "XVFTINTRZ_W_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E48AE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0728: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrz.wu.s",
        "XVFTINTRZ_WU_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E726D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0729: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrzh.l.s",
        "XVFTINTRZH_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E9D4E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0730: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvftintrzl.l.s",
        "XVFTINTRZL_L_S",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769E9BBB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0731: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.d.w",
        "XVHADDW_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7455603E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0732: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.du.wu",
        "XVHADDW_DU_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74594F06),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0733: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.h.b",
        "XVHADDW_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7454767F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0734: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.hu.bu",
        "XVHADDW_HU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74580A2E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0735: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.q.d",
        "XVHADDW_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7455C5F0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0736: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.qu.du",
        "XVHADDW_QU_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7459B58A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0737: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.w.h",
        "XVHADDW_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7454DE1F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0738: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhaddw.wu.hu",
        "XVHADDW_WU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7458A055),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0739: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.d.w",
        "XVHSUBW_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74574EFE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0740: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.du.wu",
        "XVHSUBW_DU_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745B52E5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0741: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.h.b",
        "XVHSUBW_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745640F6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0742: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.hu.bu",
        "XVHSUBW_HU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745A404A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0743: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.q.d",
        "XVHSUBW_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7457F1B4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0744: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.qu.du",
        "XVHSUBW_QU_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745BA09F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0745: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.w.h",
        "XVHSUBW_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7456BD13),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0746: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvhsubw.wu.hu",
        "XVHSUBW_WU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x745ACB41),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0747: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvh.b",
        "XVILVH_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751C6AD3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0748: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvh.d",
        "XVILVH_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751D8858),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0749: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvh.h",
        "XVILVH_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751C9EEA),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0750: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvh.w",
        "XVILVH_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751D7805),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0751: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvl.b",
        "XVILVL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751A01DD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0752: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvl.d",
        "XVILVL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751BAA99),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0753: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvl.h",
        "XVILVL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751AD53E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0754: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvilvl.w",
        "XVILVL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751B26D8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0755: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvinsgr2vr.d",
        "XVINSGR2VR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76EBE6BB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0756: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvinsgr2vr.w",
        "XVINSGR2VR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76EBDFD9),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0757: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvinsve0.d",
        "XVINSVE0_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76FFE03C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0758: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvinsve0.w",
        "XVINSVE0_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76FFDC26),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0759: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvld",
        "XVLD",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2CB5B863),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0760: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvldrepl.b",
        "XVLDREPL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x329D92B3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0761: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvldrepl.d",
        "XVLDREPL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x3213DD9C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0762: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvldrepl.h",
        "XVLDREPL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x324DC620),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0763: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvldrepl.w",
        "XVLDREPL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x322A0F4B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0764: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvldx",
        "XVLDX",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x38483937),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0765: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmadd.b",
        "XVMADD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A823E5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0766: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmadd.d",
        "XVMADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A9C913),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0767: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmadd.h",
        "XVMADD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A8F004),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0768: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmadd.w",
        "XVMADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A961A2),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0769: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.d.w",
        "XVMADDWEV_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AD6317),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0770: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.d.wu",
        "XVMADDWEV_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B5737D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0771: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.d.wu.w",
        "XVMADDWEV_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BD0B8A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0772: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.h.b",
        "XVMADDWEV_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AC25F9),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0773: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.h.bu",
        "XVMADDWEV_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B469B7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0774: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.h.bu.b",
        "XVMADDWEV_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BC7F5E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0775: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.q.d",
        "XVMADDWEV_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74ADD927),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0776: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.q.du",
        "XVMADDWEV_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B5A95D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0777: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.q.du.d",
        "XVMADDWEV_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BDE290),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0778: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.w.h",
        "XVMADDWEV_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AC803A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0779: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.w.hu",
        "XVMADDWEV_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B48C6D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0780: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwev.w.hu.h",
        "XVMADDWEV_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BCFE26),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0781: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.d.w",
        "XVMADDWOD_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AF3680),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0782: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.d.wu",
        "XVMADDWOD_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B72E17),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0783: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.d.wu.w",
        "XVMADDWOD_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BF380B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0784: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.h.b",
        "XVMADDWOD_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AE4910),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0785: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.h.bu",
        "XVMADDWOD_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B61EFF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0786: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.h.bu.b",
        "XVMADDWOD_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BE2C5B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0787: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.q.d",
        "XVMADDWOD_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AFCAEF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0788: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.q.du",
        "XVMADDWOD_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B7CD49),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0789: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.q.du.d",
        "XVMADDWOD_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BFFE7D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0790: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.w.h",
        "XVMADDWOD_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AEBB0B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0791: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.w.hu",
        "XVMADDWOD_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74B6A21D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0792: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaddwod.w.hu.h",
        "XVMADDWOD_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74BECF0C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0793: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.b",
        "XVMAX_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74703517),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0794: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.bu",
        "XVMAX_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74742FDD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0795: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.d",
        "XVMAX_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7471B622),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0796: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.du",
        "XVMAX_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7475A6C5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0797: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.h",
        "XVMAX_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7470F24D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0798: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.hu",
        "XVMAX_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7474EEE4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0799: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.w",
        "XVMAX_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7471083A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0800: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmax.wu",
        "XVMAX_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7475001F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0801: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.b",
        "XVMAXI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769004E6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0802: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.bu",
        "XVMAXI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7694736C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0803: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.d",
        "XVMAXI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7691D4B5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0804: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.du",
        "XVMAXI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7695A5BF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0805: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.h",
        "XVMAXI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7690E558),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0806: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.hu",
        "XVMAXI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7694C099),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0807: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.w",
        "XVMAXI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76916258),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0808: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmaxi.wu",
        "XVMAXI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769554FB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0809: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.b",
        "XVMIN_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74721F55),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0810: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.bu",
        "XVMIN_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74760E0F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0811: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.d",
        "XVMIN_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74738B7B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0812: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.du",
        "XVMIN_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7477947B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0813: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.h",
        "XVMIN_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7472A4BD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0814: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.hu",
        "XVMIN_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7476EFE4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0815: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.w",
        "XVMIN_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7473531F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0816: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmin.wu",
        "XVMIN_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x747771AF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0817: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.b",
        "XVMINI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76922636),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0818: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.bu",
        "XVMINI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76961F06),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0819: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.d",
        "XVMINI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7693AFEA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0820: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.du",
        "XVMINI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7697FAF0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0821: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.h",
        "XVMINI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7692C6EC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0822: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.hu",
        "XVMINI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7696F4A8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0823: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.w",
        "XVMINI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76934E21),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0824: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmini.wu",
        "XVMINI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76974DB1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0825: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.b",
        "XVMOD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E20068),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0826: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.bu",
        "XVMOD_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E66830),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0827: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.d",
        "XVMOD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E3C94B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0828: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.du",
        "XVMOD_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E7986E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0829: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.h",
        "XVMOD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E2F222),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0830: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.hu",
        "XVMOD_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E681AF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0831: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.w",
        "XVMOD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E3350E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0832: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmod.wu",
        "XVMOD_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E7526B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0833: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmskgez.b",
        "XVMSKGEZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C50BE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0834: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmskltz.b",
        "XVMSKLTZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C40AE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0835: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmskltz.d",
        "XVMSKLTZ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C4EE7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0836: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmskltz.h",
        "XVMSKLTZ_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C472B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0837: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmskltz.w",
        "XVMSKLTZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C4B6E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0838: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmsknz.b",
        "XVMSKNZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C62D6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0839: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmsub.b",
        "XVMSUB_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AA1E96),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0840: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmsub.d",
        "XVMSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AB8B4B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0841: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmsub.h",
        "XVMSUB_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AAB240),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0842: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmsub.w",
        "XVMSUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74AB76C3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0843: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.b",
        "XVMUH_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74861104),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0844: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.bu",
        "XVMUH_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7488628F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0845: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.d",
        "XVMUH_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7487A406),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0846: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.du",
        "XVMUH_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7489FD13),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0847: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.h",
        "XVMUH_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7486EAE5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0848: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.hu",
        "XVMUH_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7488ED9C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0849: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.w",
        "XVMUH_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7487647C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0850: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmuh.wu",
        "XVMUH_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x748928D9),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0851: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmul.b",
        "XVMUL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74846CF2),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0852: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmul.d",
        "XVMUL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7485A1E0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0853: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmul.h",
        "XVMUL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7484CAE9),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0854: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmul.w",
        "XVMUL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74856D15),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0855: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.d.w",
        "XVMULWEV_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74913F10),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0856: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.d.wu",
        "XVMULWEV_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74997B01),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0857: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.d.wu.w",
        "XVMULWEV_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A144ED),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0858: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.h.b",
        "XVMULWEV_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749040E2),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0859: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.h.bu",
        "XVMULWEV_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749874F4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0860: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.h.bu.b",
        "XVMULWEV_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A0338D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0861: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.q.d",
        "XVMULWEV_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74919211),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0862: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.q.du",
        "XVMULWEV_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7499EEC1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0863: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.q.du.d",
        "XVMULWEV_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A1BE89),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0864: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.w.h",
        "XVMULWEV_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7490996C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0865: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.w.hu",
        "XVMULWEV_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7498C70D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0866: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwev.w.hu.h",
        "XVMULWEV_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A09E1B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0867: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.d.w",
        "XVMULWOD_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7493237E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0868: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.d.wu",
        "XVMULWOD_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749B52D8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0869: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.d.wu.w",
        "XVMULWOD_D_WU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A3046A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0870: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.h.b",
        "XVMULWOD_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74920A50),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0871: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.h.bu",
        "XVMULWOD_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749A1F53),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0872: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.h.bu.b",
        "XVMULWOD_H_BU_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A271F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0873: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.q.d",
        "XVMULWOD_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7493BEB4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0874: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.q.du",
        "XVMULWOD_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749B9FFC),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0875: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.q.du.d",
        "XVMULWOD_Q_DU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A389EF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0876: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.w.h",
        "XVMULWOD_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7492DC5E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0877: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.w.hu",
        "XVMULWOD_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x749A9A2E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0878: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvmulwod.w.hu.h",
        "XVMULWOD_W_HU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74A28518),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0879: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvneg.b",
        "XVNEG_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C3097),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0880: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvneg.d",
        "XVNEG_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C3E34),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0881: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvneg.h",
        "XVNEG_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C35C8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0882: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvneg.w",
        "XVNEG_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C39D7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0883: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvnor.v",
        "XVNOR_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75278EE4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0884: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvnori.b",
        "XVNORI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77DF4427),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0885: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvor.v",
        "XVOR_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7526D7A6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0886: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvori.b",
        "XVORI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77D7BC46),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0887: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvorn.v",
        "XVORN_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752897B1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0888: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackev.b",
        "XVPACKEV_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75162055),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0889: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackev.d",
        "XVPACKEV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75179120),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0890: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackev.h",
        "XVPACKEV_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75169A48),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0891: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackev.w",
        "XVPACKEV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751778C0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0892: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackod.b",
        "XVPACKOD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75187FBC),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0893: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackod.d",
        "XVPACKOD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75198932),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0894: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackod.h",
        "XVPACKOD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7518994E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0895: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpackod.w",
        "XVPACKOD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75190AB6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0896: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpcnt.b",
        "XVPCNT_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C2368),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0897: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpcnt.d",
        "XVPCNT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C2D9A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0898: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpcnt.h",
        "XVPCNT_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C248C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0899: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpcnt.w",
        "XVPCNT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C2AFF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0900: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvperm.w",
        "XVPERM_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x757D42F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0901: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpermi.d",
        "XVPERMI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77EA0CD1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0902: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpermi.q",
        "XVPERMI_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77EEE1EA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0903: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpermi.w",
        "XVPERMI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77E59587),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0904: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickev.b",
        "XVPICKEV_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751E1B76),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0905: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickev.d",
        "XVPICKEV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751FA701),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0906: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickev.h",
        "XVPICKEV_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751E8D6E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0907: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickev.w",
        "XVPICKEV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x751F379E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0908: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickod.b",
        "XVPICKOD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75203ECE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0909: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickod.d",
        "XVPICKOD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7521C0AA),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0910: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickod.h",
        "XVPICKOD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7520B2BF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0911: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickod.w",
        "XVPICKOD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7521781F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0912: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve.d",
        "XVPICKVE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7703E02D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0913: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve.w",
        "XVPICKVE_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7703C799),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0914: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve2gr.d",
        "XVPICKVE2GR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76EFE0C8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0915: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve2gr.du",
        "XVPICKVE2GR_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F3E10A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0916: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve2gr.w",
        "XVPICKVE2GR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76EFD96E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0917: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvpickve2gr.wu",
        "XVPICKVE2GR_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F3D02C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0918: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrepl128vei.b",
        "XVREPL128VEI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F78A6A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0919: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrepl128vei.d",
        "XVREPL128VEI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F7F2FF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0920: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrepl128vei.h",
        "XVREPL128VEI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F7CA66),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0921: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrepl128vei.w",
        "XVREPL128VEI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76F7E5AB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0922: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplgr2vr.b",
        "XVREPLGR2VR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F0210),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0923: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplgr2vr.d",
        "XVREPLGR2VR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F0F10),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0924: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplgr2vr.h",
        "XVREPLGR2VR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F06C7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0925: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplgr2vr.w",
        "XVREPLGR2VR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769F09E4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0926: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve.b",
        "XVREPLVE_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75222E14),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0927: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve.d",
        "XVREPLVE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7523DC64),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0928: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve.h",
        "XVREPLVE_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7522E2A0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0929: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve.w",
        "XVREPLVE_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75234A54),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0930: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve0.b",
        "XVREPLVE0_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7707028B),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0931: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve0.d",
        "XVREPLVE0_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7707E094),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0932: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve0.h",
        "XVREPLVE0_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7707834D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0933: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve0.q",
        "XVREPLVE0_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7707F291),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0934: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvreplve0.w",
        "XVREPLVE0_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7707C188),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0935: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotr.b",
        "XVROTR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EE78C0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0936: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotr.d",
        "XVROTR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EFAEEB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0937: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotr.h",
        "XVROTR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EEAA33),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0938: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotr.w",
        "XVROTR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EF1C52),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0939: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotri.b",
        "XVROTRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A02CA1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0940: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotri.d",
        "XVROTRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A19707),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0941: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotri.h",
        "XVROTRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A04E21),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0942: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvrotri.w",
        "XVROTRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A0CEF9),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0943: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.b",
        "XVSADD_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74465BDB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0944: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.bu",
        "XVSADD_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744A729D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0945: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.d",
        "XVSADD_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7447EA45),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0946: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.du",
        "XVSADD_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744BBB12),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0947: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.h",
        "XVSADD_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7446841D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0948: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.hu",
        "XVSADD_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744A9A07),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0949: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.w",
        "XVSADD_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74477F96),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0950: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsadd.wu",
        "XVSADD_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744B3D42),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0951: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.b",
        "XVSAT_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772428F6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0952: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.bu",
        "XVSAT_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772830C6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0953: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.d",
        "XVSAT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77250503),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0954: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.du",
        "XVSAT_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77291E85),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0955: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.h",
        "XVSAT_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77245403),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0956: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.hu",
        "XVSAT_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7728732C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0957: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.w",
        "XVSAT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77248209),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0958: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsat.wu",
        "XVSAT_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77288C34),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0959: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseq.b",
        "XVSEQ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74004C83),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0960: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseq.d",
        "XVSEQ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7401B5A8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0961: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseq.h",
        "XVSEQ_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740096A0),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0962: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseq.w",
        "XVSEQ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74014E06),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0963: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseqi.b",
        "XVSEQI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7680032C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0964: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseqi.d",
        "XVSEQI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76819CEB),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0965: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseqi.h",
        "XVSEQI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7680A889),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0966: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseqi.w",
        "XVSEQI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76815099),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0967: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetallnez.b",
        "XVSETALLNEZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CB3A5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0968: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetallnez.d",
        "XVSETALLNEZ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CBE87),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0969: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetallnez.h",
        "XVSETALLNEZ_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CB485),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0970: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetallnez.w",
        "XVSETALLNEZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CB8A4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0971: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetanyeqz.b",
        "XVSETANYEQZ_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CA105),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0972: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetanyeqz.d",
        "XVSETANYEQZ_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CAE26),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0973: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetanyeqz.h",
        "XVSETANYEQZ_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CA685),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0974: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetanyeqz.w",
        "XVSETANYEQZ_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769CA8C7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0975: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvseteqz.v",
        "XVSETEQZ_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C9827),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0976: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsetnez.v",
        "XVSETNEZ_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x769C9DA7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0977: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf.b",
        "XVSHUF_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x0D67ACD4),
        &FMT_R4,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Ra, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0978: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf.d",
        "XVSHUF_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x757BBE5B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0979: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf.h",
        "XVSHUF_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x757A871D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0980: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf.w",
        "XVSHUF_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x757B770F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0981: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf4i.b",
        "XVSHUF4I_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7792A395),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0982: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf4i.d",
        "XVSHUF4I_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x779D8C98),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0983: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf4i.h",
        "XVSHUF4I_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77945872),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0984: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvshuf4i.w",
        "XVSHUF4I_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77994B20),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0985: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsigncov.b",
        "XVSIGNCOV_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752E3701),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0986: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsigncov.d",
        "XVSIGNCOV_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752FFE3A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0987: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsigncov.h",
        "XVSIGNCOV_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752EBAE8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0988: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsigncov.w",
        "XVSIGNCOV_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752F2B23),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0989: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.b",
        "XVSLE_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740277D8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0990: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.bu",
        "XVSLE_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74040B69),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0991: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.d",
        "XVSLE_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7403A34D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0992: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.du",
        "XVSLE_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7405C8C5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0993: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.h",
        "XVSLE_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7402D1B7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0994: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.hu",
        "XVSLE_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7404DB3D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0995: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.w",
        "XVSLE_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740363EA),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0996: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsle.wu",
        "XVSLE_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74053B30),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0997: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.b",
        "XVSLEI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7682596E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0998: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.bu",
        "XVSLEI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76842B51),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_0999: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.d",
        "XVSLEI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7683ABD3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1000: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.du",
        "XVSLEI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7685E3F9),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1001: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.h",
        "XVSLEI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7682BEC2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1002: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.hu",
        "XVSLEI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7684C974),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1003: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.w",
        "XVSLEI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768331C3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1004: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslei.wu",
        "XVSLEI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76852BA1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1005: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsll.b",
        "XVSLL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E827A8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1006: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsll.d",
        "XVSLL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E9E8D3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1007: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsll.h",
        "XVSLL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E8F795),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1008: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsll.w",
        "XVSLL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74E92BD1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1009: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslli.b",
        "XVSLLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772C2759),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1010: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslli.d",
        "XVSLLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772DBB8A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1011: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslli.h",
        "XVSLLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772C7B91),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1012: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslli.w",
        "XVSLLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x772CF7FA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1013: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.d.w",
        "XVSLLWIL_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7708E283),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1014: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.du.wu",
        "XVSLLWIL_DU_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x770CFCA3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1015: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.h.b",
        "XVSLLWIL_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77083AAD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1016: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.hu.bu",
        "XVSLLWIL_HU_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x770C39EF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1017: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.w.h",
        "XVSLLWIL_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x770843B4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1018: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsllwil.wu.hu",
        "XVSLLWIL_WU_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x770C43B6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1019: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.b",
        "XVSLT_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740637FE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1020: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.bu",
        "XVSLT_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740875B4),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1021: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.d",
        "XVSLT_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7407FD43),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1022: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.du",
        "XVSLT_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74098E9E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1023: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.h",
        "XVSLT_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740682F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1024: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.hu",
        "XVSLT_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7408EBAC),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1025: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.w",
        "XVSLT_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74070F57),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1026: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslt.wu",
        "XVSLT_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74097F3A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1027: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.b",
        "XVSLTI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76861B7F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1028: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.bu",
        "XVSLTI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76880881),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1029: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.d",
        "XVSLTI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76878A4D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1030: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.du",
        "XVSLTI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7689F4AA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1031: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.h",
        "XVSLTI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76869A65),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1032: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.hu",
        "XVSLTI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7688D0A0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1033: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.w",
        "XVSLTI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76872D14),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1034: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvslti.wu",
        "XVSLTI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76896320),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1035: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsra.b",
        "XVSRA_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EC004B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1036: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsra.d",
        "XVSRA_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74ED85E6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1037: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsra.h",
        "XVSRA_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EC9B71),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1038: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsra.w",
        "XVSRA_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74ED318D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1039: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrai.b",
        "XVSRAI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77342C50),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1040: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrai.d",
        "XVSRAI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7735128A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1041: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrai.h",
        "XVSRAI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7734706E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1042: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrai.w",
        "XVSRAI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7734D651),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1043: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsran.b.h",
        "XVSRAN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F68DBE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1044: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsran.h.w",
        "XVSRAN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F71352),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1045: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsran.w.d",
        "XVSRAN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F7D67B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1046: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrani.b.h",
        "XVSRANI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77587EEE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1047: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrani.d.q",
        "XVSRANI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x775BC4F1),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1048: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrani.h.w",
        "XVSRANI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77589502),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1049: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrani.w.d",
        "XVSRANI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77593965),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1050: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrar.b",
        "XVSRAR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F22E49),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1051: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrar.d",
        "XVSRAR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F399F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1052: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrar.h",
        "XVSRAR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F2874F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1053: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrar.w",
        "XVSRAR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F33A71),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1054: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrari.b",
        "XVSRARI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A82F8A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1055: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrari.d",
        "XVSRARI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A9213D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1056: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrari.h",
        "XVSRARI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A8783C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1057: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrari.w",
        "XVSRARI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A8B0ED),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1058: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarn.b.h",
        "XVSRARN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FABE92),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1059: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarn.h.w",
        "XVSRARN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FB102C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1060: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarn.w.d",
        "XVSRARN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FBEA49),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1061: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarni.b.h",
        "XVSRARNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x775C7FF5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1062: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarni.d.q",
        "XVSRARNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x775E1CA7),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1063: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarni.h.w",
        "XVSRARNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x775CE6C4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1064: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrarni.w.d",
        "XVSRARNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x775DA518),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1065: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrl.b",
        "XVSRL_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EA7714),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1066: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrl.d",
        "XVSRL_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EBEBCD),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1067: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrl.h",
        "XVSRL_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EAFE2B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1068: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrl.w",
        "XVSRL_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74EB2142),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1069: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrli.b",
        "XVSRLI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77302C9D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1070: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrli.d",
        "XVSRLI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7731B880),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1071: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrli.h",
        "XVSRLI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x773071DC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1072: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrli.w",
        "XVSRLI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77309E4C),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1073: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrln.b.h",
        "XVSRLN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F495A7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1074: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrln.h.w",
        "XVSRLN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F51646),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1075: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrln.w.d",
        "XVSRLN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F5F18C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1076: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlni.b.h",
        "XVSRLNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77404905),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1077: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlni.d.q",
        "XVSRLNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77437F8F),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1078: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlni.h.w",
        "XVSRLNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7740D087),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1079: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlni.w.d",
        "XVSRLNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774145FE),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1080: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlr.b",
        "XVSRLR_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F01572),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1081: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlr.d",
        "XVSRLR_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F19F64),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1082: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlr.h",
        "XVSRLR_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F0D4BF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1083: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlr.w",
        "XVSRLR_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F104A7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1084: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlri.b",
        "XVSRLRI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A433DD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1085: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlri.d",
        "XVSRLRI_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A5D294),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1086: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlri.h",
        "XVSRLRI_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A478D0),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1087: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlri.w",
        "XVSRLRI_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x76A4F158),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1088: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrn.b.h",
        "XVSRLRN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F8EB24),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1089: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrn.h.w",
        "XVSRLRN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F904B1),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1090: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrn.w.d",
        "XVSRLRN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74F9C43D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1091: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrni.b.h",
        "XVSRLRNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7744722A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1092: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrni.d.q",
        "XVSRLRNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7746A919),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1093: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrni.h.w",
        "XVSRLRNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7744B6F6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1094: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsrlrni.w.d",
        "XVSRLRNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7745EAD2),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1095: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.b.h",
        "XVSSRAN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FE8491),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1096: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.bu.h",
        "XVSSRAN_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7506E183),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1097: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.h.w",
        "XVSSRAN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FF379C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1098: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.hu.w",
        "XVSSRAN_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75070719),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1099: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.w.d",
        "XVSSRAN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FFFC35),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1100: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssran.wu.d",
        "XVSSRAN_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7507A9DE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1101: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.b.h",
        "XVSSRANI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77607ADA),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1102: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.bu.h",
        "XVSSRANI_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77646866),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1103: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.d.q",
        "XVSSRANI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7762ED49),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1104: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.du.q",
        "XVSSRANI_DU_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77663C50),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1105: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.h.w",
        "XVSSRANI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7760E9D3),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1106: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.hu.w",
        "XVSSRANI_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77649934),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1107: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.w.d",
        "XVSSRANI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77616F61),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1108: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrani.wu.d",
        "XVSSRANI_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77652178),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1109: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.b.h",
        "XVSSRARN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750281A7),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1110: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.bu.h",
        "XVSSRARN_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750A8984),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1111: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.h.w",
        "XVSSRARN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75033856),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1112: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.hu.w",
        "XVSSRARN_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750B0F0F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1113: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.w.d",
        "XVSSRARN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7503C0ED),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1114: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarn.wu.d",
        "XVSSRARN_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750BA13E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1115: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.b.h",
        "XVSSRARNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77687480),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1116: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.bu.h",
        "XVSSRARNI_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x776C4275),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1117: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.d.q",
        "XVSSRARNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x776B4FE8),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1118: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.du.q",
        "XVSSRARNI_DU_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x776F79CF),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1119: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.h.w",
        "XVSSRARNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7768A408),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1120: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.hu.w",
        "XVSSRARNI_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x776C85B6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1121: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.w.d",
        "XVSSRARNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7769A8A5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1122: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrarni.wu.d",
        "XVSSRARNI_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x776D68B5),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1123: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.b.h",
        "XVSSRLN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FC9098),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1124: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.bu.h",
        "XVSSRLN_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7504E93A),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1125: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.h.w",
        "XVSSRLN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FD01E5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1126: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.hu.w",
        "XVSSRLN_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75050687),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1127: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.w.d",
        "XVSSRLN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74FDFB20),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1128: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrln.wu.d",
        "XVSSRLN_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7505D1AF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1129: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.b.h",
        "XVSSRLNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77486653),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1130: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.bu.h",
        "XVSSRLNI_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774C5559),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1131: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.d.q",
        "XVSSRLNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774BE568),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1132: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.du.q",
        "XVSSRLNI_DU_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774EAC88),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1133: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.h.w",
        "XVSSRLNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77488FBD),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1134: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.hu.w",
        "XVSSRLNI_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774CEA49),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1135: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.w.d",
        "XVSSRLNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7749ADE9),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1136: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlni.wu.d",
        "XVSSRLNI_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x774D36D4),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1137: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.b.h",
        "XVSSRLRN_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7500CA88),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1138: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.bu.h",
        "XVSSRLRN_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7508CAEF),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1139: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.h.w",
        "XVSSRLRN_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75014DA2),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1140: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.hu.w",
        "XVSSRLRN_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750941D6),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1141: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.w.d",
        "XVSSRLRN_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x750194F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1142: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrn.wu.d",
        "XVSSRLRN_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75099794),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1143: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.b.h",
        "XVSSRLRNI_B_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7750635A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1144: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.bu.h",
        "XVSSRLRNI_BU_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77544F97),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1145: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.d.q",
        "XVSSRLRNI_D_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77530208),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1146: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.du.q",
        "XVSSRLRNI_DU_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7756B132),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1147: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.h.w",
        "XVSSRLRNI_H_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7750CC06),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1148: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.hu.w",
        "XVSSRLRNI_HU_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7754C959),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1149: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.w.d",
        "XVSSRLRNI_W_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7751DDFC),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1150: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssrlrni.wu.d",
        "XVSSRLRNI_WU_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77553F90),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1151: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.b",
        "XVSSUB_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7448626E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1152: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.bu",
        "XVSSUB_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744C45AB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1153: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.d",
        "XVSSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74498A1C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1154: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.du",
        "XVSSUB_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744DEF52),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1155: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.h",
        "XVSSUB_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7448CD0D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1156: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.hu",
        "XVSSUB_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744CF150),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1157: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.w",
        "XVSSUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7449737C),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1158: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvssub.wu",
        "XVSSUB_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x744D3415),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1159: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvst",
        "XVST",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x2CCEBD8E),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1160: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvstelm.b",
        "XVSTELM_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x33AA5C54),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1161: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvstelm.d",
        "XVSTELM_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x331DE3D6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1162: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvstelm.h",
        "XVSTELM_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x33514028),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1163: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvstelm.w",
        "XVSTELM_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x33219E53),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1164: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvstx",
        "XVSTX",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x384C5527),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1165: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsub.b",
        "XVSUB_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740C438B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1166: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsub.d",
        "XVSUB_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740D9DA5),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1167: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsub.h",
        "XVSUB_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740CE06B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1168: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsub.q",
        "XVSUB_Q",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x752DFF4D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1169: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsub.w",
        "XVSUB_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x740D1AEE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1170: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubi.bu",
        "XVSUBI_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768C0772),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1171: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubi.du",
        "XVSUBI_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768DBB9A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1172: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubi.hu",
        "XVSUBI_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768CCEE6),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1173: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubi.wu",
        "XVSUBI_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x768D146D),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1174: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.d.w",
        "XVSUBWEV_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74212C86),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1175: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.d.wu",
        "XVSUBWEV_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74315C3F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1176: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.h.b",
        "XVSUBWEV_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7420703D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1177: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.h.bu",
        "XVSUBWEV_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74300A81),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1178: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.q.d",
        "XVSUBWEV_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7421B7FB),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1179: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.q.du",
        "XVSUBWEV_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7431C79F),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1180: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.w.h",
        "XVSUBWEV_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7420FE98),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1181: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwev.w.hu",
        "XVSUBWEV_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7430B0D3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1182: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.d.w",
        "XVSUBWOD_D_W",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74250DC8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1183: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.d.wu",
        "XVSUBWOD_D_WU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74356B01),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1184: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.h.b",
        "XVSUBWOD_H_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74244523),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1185: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.h.bu",
        "XVSUBWOD_H_BU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7434045B),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1186: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.q.d",
        "XVSUBWOD_Q_D",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7425C9F8),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1187: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.q.du",
        "XVSUBWOD_Q_DU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x74359F5D),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1188: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.w.h",
        "XVSUBWOD_W_H",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7424D4AE),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1189: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvsubwod.w.hu",
        "XVSUBWOD_W_HU",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x7434D8F3),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1190: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvxor.v",
        "XVXOR_V",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x75272B4E),
        &FMT_R3,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rk, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static G_1191: InstructionSpec<LoongArchBackend> =
    InstructionSpec::__macro_new(
        "xvxori.b",
        "XVXORI_B",
        robustone_isa::mask_value!(0xFFFFFFFF, 0x77DA551A),
        &FMT_R2,
        &[
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rd, Access::Write),
        robustone_isa::reg!(LoongArchRegisterClass::Gpr, LoongArchField::Rj, Access::Read),
    ],
        LoongArchFeature::BASE,
        ModeSet::All,
        &[InstructionGroup::Integer],
        None,
        &[],
        None,
        0,
        SpecSeal::__private_seal_token(),
    );

pub static ALL_CAPSTONE_SPECS: &[InstructionSpec<LoongArchBackend>] = &[
    G_0000,
    G_0001,
    G_0002,
    G_0003,
    G_0004,
    G_0005,
    G_0006,
    G_0007,
    G_0008,
    G_0009,
    G_0010,
    G_0011,
    G_0012,
    G_0013,
    G_0014,
    G_0015,
    G_0016,
    G_0017,
    G_0018,
    G_0019,
    G_0020,
    G_0021,
    G_0022,
    G_0023,
    G_0024,
    G_0025,
    G_0026,
    G_0027,
    G_0028,
    G_0029,
    G_0030,
    G_0031,
    G_0032,
    G_0033,
    G_0034,
    G_0035,
    G_0036,
    G_0037,
    G_0038,
    G_0039,
    G_0040,
    G_0041,
    G_0042,
    G_0043,
    G_0044,
    G_0045,
    G_0046,
    G_0047,
    G_0048,
    G_0049,
    G_0050,
    G_0051,
    G_0052,
    G_0053,
    G_0054,
    G_0055,
    G_0056,
    G_0057,
    G_0058,
    G_0059,
    G_0060,
    G_0061,
    G_0062,
    G_0063,
    G_0064,
    G_0065,
    G_0066,
    G_0067,
    G_0068,
    G_0069,
    G_0070,
    G_0071,
    G_0072,
    G_0073,
    G_0074,
    G_0075,
    G_0076,
    G_0077,
    G_0078,
    G_0079,
    G_0080,
    G_0081,
    G_0082,
    G_0083,
    G_0084,
    G_0085,
    G_0086,
    G_0087,
    G_0088,
    G_0089,
    G_0090,
    G_0091,
    G_0092,
    G_0093,
    G_0094,
    G_0095,
    G_0096,
    G_0097,
    G_0098,
    G_0099,
    G_0100,
    G_0101,
    G_0102,
    G_0103,
    G_0104,
    G_0105,
    G_0106,
    G_0107,
    G_0108,
    G_0109,
    G_0110,
    G_0111,
    G_0112,
    G_0113,
    G_0114,
    G_0115,
    G_0116,
    G_0117,
    G_0118,
    G_0119,
    G_0120,
    G_0121,
    G_0122,
    G_0123,
    G_0124,
    G_0125,
    G_0126,
    G_0127,
    G_0128,
    G_0129,
    G_0130,
    G_0131,
    G_0132,
    G_0133,
    G_0134,
    G_0135,
    G_0136,
    G_0137,
    G_0138,
    G_0139,
    G_0140,
    G_0141,
    G_0142,
    G_0143,
    G_0144,
    G_0145,
    G_0146,
    G_0147,
    G_0148,
    G_0149,
    G_0150,
    G_0151,
    G_0152,
    G_0153,
    G_0154,
    G_0155,
    G_0156,
    G_0157,
    G_0158,
    G_0159,
    G_0160,
    G_0161,
    G_0162,
    G_0163,
    G_0164,
    G_0165,
    G_0166,
    G_0167,
    G_0168,
    G_0169,
    G_0170,
    G_0171,
    G_0172,
    G_0173,
    G_0174,
    G_0175,
    G_0176,
    G_0177,
    G_0178,
    G_0179,
    G_0180,
    G_0181,
    G_0182,
    G_0183,
    G_0184,
    G_0185,
    G_0186,
    G_0187,
    G_0188,
    G_0189,
    G_0190,
    G_0191,
    G_0192,
    G_0193,
    G_0194,
    G_0195,
    G_0196,
    G_0197,
    G_0198,
    G_0199,
    G_0200,
    G_0201,
    G_0202,
    G_0203,
    G_0204,
    G_0205,
    G_0206,
    G_0207,
    G_0208,
    G_0209,
    G_0210,
    G_0211,
    G_0212,
    G_0213,
    G_0214,
    G_0215,
    G_0216,
    G_0217,
    G_0218,
    G_0219,
    G_0220,
    G_0221,
    G_0222,
    G_0223,
    G_0224,
    G_0225,
    G_0226,
    G_0227,
    G_0228,
    G_0229,
    G_0230,
    G_0231,
    G_0232,
    G_0233,
    G_0234,
    G_0235,
    G_0236,
    G_0237,
    G_0238,
    G_0239,
    G_0240,
    G_0241,
    G_0242,
    G_0243,
    G_0244,
    G_0245,
    G_0246,
    G_0247,
    G_0248,
    G_0249,
    G_0250,
    G_0251,
    G_0252,
    G_0253,
    G_0254,
    G_0255,
    G_0256,
    G_0257,
    G_0258,
    G_0259,
    G_0260,
    G_0261,
    G_0262,
    G_0263,
    G_0264,
    G_0265,
    G_0266,
    G_0267,
    G_0268,
    G_0269,
    G_0270,
    G_0271,
    G_0272,
    G_0273,
    G_0274,
    G_0275,
    G_0276,
    G_0277,
    G_0278,
    G_0279,
    G_0280,
    G_0281,
    G_0282,
    G_0283,
    G_0284,
    G_0285,
    G_0286,
    G_0287,
    G_0288,
    G_0289,
    G_0290,
    G_0291,
    G_0292,
    G_0293,
    G_0294,
    G_0295,
    G_0296,
    G_0297,
    G_0298,
    G_0299,
    G_0300,
    G_0301,
    G_0302,
    G_0303,
    G_0304,
    G_0305,
    G_0306,
    G_0307,
    G_0308,
    G_0309,
    G_0310,
    G_0311,
    G_0312,
    G_0313,
    G_0314,
    G_0315,
    G_0316,
    G_0317,
    G_0318,
    G_0319,
    G_0320,
    G_0321,
    G_0322,
    G_0323,
    G_0324,
    G_0325,
    G_0326,
    G_0327,
    G_0328,
    G_0329,
    G_0330,
    G_0331,
    G_0332,
    G_0333,
    G_0334,
    G_0335,
    G_0336,
    G_0337,
    G_0338,
    G_0339,
    G_0340,
    G_0341,
    G_0342,
    G_0343,
    G_0344,
    G_0345,
    G_0346,
    G_0347,
    G_0348,
    G_0349,
    G_0350,
    G_0351,
    G_0352,
    G_0353,
    G_0354,
    G_0355,
    G_0356,
    G_0357,
    G_0358,
    G_0359,
    G_0360,
    G_0361,
    G_0362,
    G_0363,
    G_0364,
    G_0365,
    G_0366,
    G_0367,
    G_0368,
    G_0369,
    G_0370,
    G_0371,
    G_0372,
    G_0373,
    G_0374,
    G_0375,
    G_0376,
    G_0377,
    G_0378,
    G_0379,
    G_0380,
    G_0381,
    G_0382,
    G_0383,
    G_0384,
    G_0385,
    G_0386,
    G_0387,
    G_0388,
    G_0389,
    G_0390,
    G_0391,
    G_0392,
    G_0393,
    G_0394,
    G_0395,
    G_0396,
    G_0397,
    G_0398,
    G_0399,
    G_0400,
    G_0401,
    G_0402,
    G_0403,
    G_0404,
    G_0405,
    G_0406,
    G_0407,
    G_0408,
    G_0409,
    G_0410,
    G_0411,
    G_0412,
    G_0413,
    G_0414,
    G_0415,
    G_0416,
    G_0417,
    G_0418,
    G_0419,
    G_0420,
    G_0421,
    G_0422,
    G_0423,
    G_0424,
    G_0425,
    G_0426,
    G_0427,
    G_0428,
    G_0429,
    G_0430,
    G_0431,
    G_0432,
    G_0433,
    G_0434,
    G_0435,
    G_0436,
    G_0437,
    G_0438,
    G_0439,
    G_0440,
    G_0441,
    G_0442,
    G_0443,
    G_0444,
    G_0445,
    G_0446,
    G_0447,
    G_0448,
    G_0449,
    G_0450,
    G_0451,
    G_0452,
    G_0453,
    G_0454,
    G_0455,
    G_0456,
    G_0457,
    G_0458,
    G_0459,
    G_0460,
    G_0461,
    G_0462,
    G_0463,
    G_0464,
    G_0465,
    G_0466,
    G_0467,
    G_0468,
    G_0469,
    G_0470,
    G_0471,
    G_0472,
    G_0473,
    G_0474,
    G_0475,
    G_0476,
    G_0477,
    G_0478,
    G_0479,
    G_0480,
    G_0481,
    G_0482,
    G_0483,
    G_0484,
    G_0485,
    G_0486,
    G_0487,
    G_0488,
    G_0489,
    G_0490,
    G_0491,
    G_0492,
    G_0493,
    G_0494,
    G_0495,
    G_0496,
    G_0497,
    G_0498,
    G_0499,
    G_0500,
    G_0501,
    G_0502,
    G_0503,
    G_0504,
    G_0505,
    G_0506,
    G_0507,
    G_0508,
    G_0509,
    G_0510,
    G_0511,
    G_0512,
    G_0513,
    G_0514,
    G_0515,
    G_0516,
    G_0517,
    G_0518,
    G_0519,
    G_0520,
    G_0521,
    G_0522,
    G_0523,
    G_0524,
    G_0525,
    G_0526,
    G_0527,
    G_0528,
    G_0529,
    G_0530,
    G_0531,
    G_0532,
    G_0533,
    G_0534,
    G_0535,
    G_0536,
    G_0537,
    G_0538,
    G_0539,
    G_0540,
    G_0541,
    G_0542,
    G_0543,
    G_0544,
    G_0545,
    G_0546,
    G_0547,
    G_0548,
    G_0549,
    G_0550,
    G_0551,
    G_0552,
    G_0553,
    G_0554,
    G_0555,
    G_0556,
    G_0557,
    G_0558,
    G_0559,
    G_0560,
    G_0561,
    G_0562,
    G_0563,
    G_0564,
    G_0565,
    G_0566,
    G_0567,
    G_0568,
    G_0569,
    G_0570,
    G_0571,
    G_0572,
    G_0573,
    G_0574,
    G_0575,
    G_0576,
    G_0577,
    G_0578,
    G_0579,
    G_0580,
    G_0581,
    G_0582,
    G_0583,
    G_0584,
    G_0585,
    G_0586,
    G_0587,
    G_0588,
    G_0589,
    G_0590,
    G_0591,
    G_0592,
    G_0593,
    G_0594,
    G_0595,
    G_0596,
    G_0597,
    G_0598,
    G_0599,
    G_0600,
    G_0601,
    G_0602,
    G_0603,
    G_0604,
    G_0605,
    G_0606,
    G_0607,
    G_0608,
    G_0609,
    G_0610,
    G_0611,
    G_0612,
    G_0613,
    G_0614,
    G_0615,
    G_0616,
    G_0617,
    G_0618,
    G_0619,
    G_0620,
    G_0621,
    G_0622,
    G_0623,
    G_0624,
    G_0625,
    G_0626,
    G_0627,
    G_0628,
    G_0629,
    G_0630,
    G_0631,
    G_0632,
    G_0633,
    G_0634,
    G_0635,
    G_0636,
    G_0637,
    G_0638,
    G_0639,
    G_0640,
    G_0641,
    G_0642,
    G_0643,
    G_0644,
    G_0645,
    G_0646,
    G_0647,
    G_0648,
    G_0649,
    G_0650,
    G_0651,
    G_0652,
    G_0653,
    G_0654,
    G_0655,
    G_0656,
    G_0657,
    G_0658,
    G_0659,
    G_0660,
    G_0661,
    G_0662,
    G_0663,
    G_0664,
    G_0665,
    G_0666,
    G_0667,
    G_0668,
    G_0669,
    G_0670,
    G_0671,
    G_0672,
    G_0673,
    G_0674,
    G_0675,
    G_0676,
    G_0677,
    G_0678,
    G_0679,
    G_0680,
    G_0681,
    G_0682,
    G_0683,
    G_0684,
    G_0685,
    G_0686,
    G_0687,
    G_0688,
    G_0689,
    G_0690,
    G_0691,
    G_0692,
    G_0693,
    G_0694,
    G_0695,
    G_0696,
    G_0697,
    G_0698,
    G_0699,
    G_0700,
    G_0701,
    G_0702,
    G_0703,
    G_0704,
    G_0705,
    G_0706,
    G_0707,
    G_0708,
    G_0709,
    G_0710,
    G_0711,
    G_0712,
    G_0713,
    G_0714,
    G_0715,
    G_0716,
    G_0717,
    G_0718,
    G_0719,
    G_0720,
    G_0721,
    G_0722,
    G_0723,
    G_0724,
    G_0725,
    G_0726,
    G_0727,
    G_0728,
    G_0729,
    G_0730,
    G_0731,
    G_0732,
    G_0733,
    G_0734,
    G_0735,
    G_0736,
    G_0737,
    G_0738,
    G_0739,
    G_0740,
    G_0741,
    G_0742,
    G_0743,
    G_0744,
    G_0745,
    G_0746,
    G_0747,
    G_0748,
    G_0749,
    G_0750,
    G_0751,
    G_0752,
    G_0753,
    G_0754,
    G_0755,
    G_0756,
    G_0757,
    G_0758,
    G_0759,
    G_0760,
    G_0761,
    G_0762,
    G_0763,
    G_0764,
    G_0765,
    G_0766,
    G_0767,
    G_0768,
    G_0769,
    G_0770,
    G_0771,
    G_0772,
    G_0773,
    G_0774,
    G_0775,
    G_0776,
    G_0777,
    G_0778,
    G_0779,
    G_0780,
    G_0781,
    G_0782,
    G_0783,
    G_0784,
    G_0785,
    G_0786,
    G_0787,
    G_0788,
    G_0789,
    G_0790,
    G_0791,
    G_0792,
    G_0793,
    G_0794,
    G_0795,
    G_0796,
    G_0797,
    G_0798,
    G_0799,
    G_0800,
    G_0801,
    G_0802,
    G_0803,
    G_0804,
    G_0805,
    G_0806,
    G_0807,
    G_0808,
    G_0809,
    G_0810,
    G_0811,
    G_0812,
    G_0813,
    G_0814,
    G_0815,
    G_0816,
    G_0817,
    G_0818,
    G_0819,
    G_0820,
    G_0821,
    G_0822,
    G_0823,
    G_0824,
    G_0825,
    G_0826,
    G_0827,
    G_0828,
    G_0829,
    G_0830,
    G_0831,
    G_0832,
    G_0833,
    G_0834,
    G_0835,
    G_0836,
    G_0837,
    G_0838,
    G_0839,
    G_0840,
    G_0841,
    G_0842,
    G_0843,
    G_0844,
    G_0845,
    G_0846,
    G_0847,
    G_0848,
    G_0849,
    G_0850,
    G_0851,
    G_0852,
    G_0853,
    G_0854,
    G_0855,
    G_0856,
    G_0857,
    G_0858,
    G_0859,
    G_0860,
    G_0861,
    G_0862,
    G_0863,
    G_0864,
    G_0865,
    G_0866,
    G_0867,
    G_0868,
    G_0869,
    G_0870,
    G_0871,
    G_0872,
    G_0873,
    G_0874,
    G_0875,
    G_0876,
    G_0877,
    G_0878,
    G_0879,
    G_0880,
    G_0881,
    G_0882,
    G_0883,
    G_0884,
    G_0885,
    G_0886,
    G_0887,
    G_0888,
    G_0889,
    G_0890,
    G_0891,
    G_0892,
    G_0893,
    G_0894,
    G_0895,
    G_0896,
    G_0897,
    G_0898,
    G_0899,
    G_0900,
    G_0901,
    G_0902,
    G_0903,
    G_0904,
    G_0905,
    G_0906,
    G_0907,
    G_0908,
    G_0909,
    G_0910,
    G_0911,
    G_0912,
    G_0913,
    G_0914,
    G_0915,
    G_0916,
    G_0917,
    G_0918,
    G_0919,
    G_0920,
    G_0921,
    G_0922,
    G_0923,
    G_0924,
    G_0925,
    G_0926,
    G_0927,
    G_0928,
    G_0929,
    G_0930,
    G_0931,
    G_0932,
    G_0933,
    G_0934,
    G_0935,
    G_0936,
    G_0937,
    G_0938,
    G_0939,
    G_0940,
    G_0941,
    G_0942,
    G_0943,
    G_0944,
    G_0945,
    G_0946,
    G_0947,
    G_0948,
    G_0949,
    G_0950,
    G_0951,
    G_0952,
    G_0953,
    G_0954,
    G_0955,
    G_0956,
    G_0957,
    G_0958,
    G_0959,
    G_0960,
    G_0961,
    G_0962,
    G_0963,
    G_0964,
    G_0965,
    G_0966,
    G_0967,
    G_0968,
    G_0969,
    G_0970,
    G_0971,
    G_0972,
    G_0973,
    G_0974,
    G_0975,
    G_0976,
    G_0977,
    G_0978,
    G_0979,
    G_0980,
    G_0981,
    G_0982,
    G_0983,
    G_0984,
    G_0985,
    G_0986,
    G_0987,
    G_0988,
    G_0989,
    G_0990,
    G_0991,
    G_0992,
    G_0993,
    G_0994,
    G_0995,
    G_0996,
    G_0997,
    G_0998,
    G_0999,
    G_1000,
    G_1001,
    G_1002,
    G_1003,
    G_1004,
    G_1005,
    G_1006,
    G_1007,
    G_1008,
    G_1009,
    G_1010,
    G_1011,
    G_1012,
    G_1013,
    G_1014,
    G_1015,
    G_1016,
    G_1017,
    G_1018,
    G_1019,
    G_1020,
    G_1021,
    G_1022,
    G_1023,
    G_1024,
    G_1025,
    G_1026,
    G_1027,
    G_1028,
    G_1029,
    G_1030,
    G_1031,
    G_1032,
    G_1033,
    G_1034,
    G_1035,
    G_1036,
    G_1037,
    G_1038,
    G_1039,
    G_1040,
    G_1041,
    G_1042,
    G_1043,
    G_1044,
    G_1045,
    G_1046,
    G_1047,
    G_1048,
    G_1049,
    G_1050,
    G_1051,
    G_1052,
    G_1053,
    G_1054,
    G_1055,
    G_1056,
    G_1057,
    G_1058,
    G_1059,
    G_1060,
    G_1061,
    G_1062,
    G_1063,
    G_1064,
    G_1065,
    G_1066,
    G_1067,
    G_1068,
    G_1069,
    G_1070,
    G_1071,
    G_1072,
    G_1073,
    G_1074,
    G_1075,
    G_1076,
    G_1077,
    G_1078,
    G_1079,
    G_1080,
    G_1081,
    G_1082,
    G_1083,
    G_1084,
    G_1085,
    G_1086,
    G_1087,
    G_1088,
    G_1089,
    G_1090,
    G_1091,
    G_1092,
    G_1093,
    G_1094,
    G_1095,
    G_1096,
    G_1097,
    G_1098,
    G_1099,
    G_1100,
    G_1101,
    G_1102,
    G_1103,
    G_1104,
    G_1105,
    G_1106,
    G_1107,
    G_1108,
    G_1109,
    G_1110,
    G_1111,
    G_1112,
    G_1113,
    G_1114,
    G_1115,
    G_1116,
    G_1117,
    G_1118,
    G_1119,
    G_1120,
    G_1121,
    G_1122,
    G_1123,
    G_1124,
    G_1125,
    G_1126,
    G_1127,
    G_1128,
    G_1129,
    G_1130,
    G_1131,
    G_1132,
    G_1133,
    G_1134,
    G_1135,
    G_1136,
    G_1137,
    G_1138,
    G_1139,
    G_1140,
    G_1141,
    G_1142,
    G_1143,
    G_1144,
    G_1145,
    G_1146,
    G_1147,
    G_1148,
    G_1149,
    G_1150,
    G_1151,
    G_1152,
    G_1153,
    G_1154,
    G_1155,
    G_1156,
    G_1157,
    G_1158,
    G_1159,
    G_1160,
    G_1161,
    G_1162,
    G_1163,
    G_1164,
    G_1165,
    G_1166,
    G_1167,
    G_1168,
    G_1169,
    G_1170,
    G_1171,
    G_1172,
    G_1173,
    G_1174,
    G_1175,
    G_1176,
    G_1177,
    G_1178,
    G_1179,
    G_1180,
    G_1181,
    G_1182,
    G_1183,
    G_1184,
    G_1185,
    G_1186,
    G_1187,
    G_1188,
    G_1189,
    G_1190,
    G_1191,
];
