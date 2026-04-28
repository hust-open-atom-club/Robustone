"""
Capstone YAML test file loader for the Robustone test framework.

Consumes Capstone's YAML test cases (from tests/MC/ and tests/details/)
and produces the same (hex_input, expected_output, note) tuples that
the existing text-based loader produces.
"""

from pathlib import Path
from typing import Dict, Iterator, List, Optional, Set, Tuple

try:
    import yaml
except ImportError as _exc:  # pragma: no cover
    raise RuntimeError(
        "PyYAML is required for YAML test loading. "
        "Install it with: pip install PyYAML"
    ) from _exc


# Mapping from Capstone arch+mode combinations to Robustone arch names.
# The mode set is matched as a subset: the YAML options must contain AT
# LEAST the keys listed here (and may contain additional ones like
# CS_OPT_SYNTAX_NO_ALIAS_TEXT).
ARCH_MODE_MAP: Dict[Tuple[str, frozenset], str] = {
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV32"})): "riscv32",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV64"})): "riscv64",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV32", "CS_MODE_RISCV_C"})): "riscv32",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_C"})): "riscv64",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV32", "CS_MODE_RISCV_A"})): "riscv32",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_A"})): "riscv64",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV32", "CS_MODE_RISCV_D"})): "riscv32",
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_D"})): "riscv64",
    (
        "CS_ARCH_RISCV",
        frozenset({"CS_MODE_RISCV32", "CS_MODE_RISCV_C", "CS_MODE_RISCV_D"}),
    ): "riscv32",
    (
        "CS_ARCH_RISCV",
        frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_C", "CS_MODE_RISCV_D"}),
    ): "riscv64",
    (
        "CS_ARCH_RISCV",
        frozenset({"CS_MODE_RISCV32", "CS_MODE_RISCV_A", "CS_MODE_RISCV_D"}),
    ): "riscv32",
    (
        "CS_ARCH_RISCV",
        frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_A", "CS_MODE_RISCV_D"}),
    ): "riscv64",
    (
        "CS_ARCH_RISCV",
        frozenset(
            {"CS_MODE_RISCV32", "CS_MODE_RISCV_C", "CS_MODE_RISCV_A", "CS_MODE_RISCV_D"}
        ),
    ): "riscv32",
    (
        "CS_ARCH_RISCV",
        frozenset(
            {"CS_MODE_RISCV64", "CS_MODE_RISCV_C", "CS_MODE_RISCV_A", "CS_MODE_RISCV_D"}
        ),
    ): "riscv64",
    # Vector extension combinations
    ("CS_ARCH_RISCV", frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_V"})): "riscv64",
    (
        "CS_ARCH_RISCV",
        frozenset({"CS_MODE_RISCV64", "CS_MODE_RISCV_V", "CS_MODE_RISCV_F"}),
    ): "riscv64",
    # LoongArch
    ("CS_ARCH_LOONGARCH", frozenset({"CS_MODE_LOONGARCH64"})): "loongarch64",
    ("CS_ARCH_LOONGARCH", frozenset({"CS_MODE_LOONGARCH32"})): "loongarch32",
}

# Capstone options that affect cstool CLI flags.
# Values are lists of flag strings to append.
OPTION_TO_CSTOOL_FLAGS: Dict[str, List[str]] = {
    "CS_OPT_SYNTAX_NO_ALIAS_TEXT": ["-s"],
}

# All CS_MODE_* constants found in Capstone headers.
ALL_MODE_OPTIONS: Set[str] = {
    "CS_MODE_LITTLE_ENDIAN",
    "CS_MODE_ARM",
    "CS_MODE_16",
    "CS_MODE_32",
    "CS_MODE_64",
    "CS_MODE_THUMB",
    "CS_MODE_MCLASS",
    "CS_MODE_V8",
    "CS_MODE_APPLE_PROPRIETARY",
    "CS_MODE_V9",
    "CS_MODE_QPX",
    "CS_MODE_SPE",
    "CS_MODE_BOOKE",
    "CS_MODE_PS",
    "CS_MODE_AIX_OS",
    "CS_MODE_PWR7",
    "CS_MODE_PWR8",
    "CS_MODE_PWR9",
    "CS_MODE_PWR10",
    "CS_MODE_PPC_ISA_FUTURE",
    "CS_MODE_MODERN_AIX_AS",
    "CS_MODE_MSYNC",
    "CS_MODE_M68K_000",
    "CS_MODE_M68K_010",
    "CS_MODE_M68K_020",
    "CS_MODE_M68K_030",
    "CS_MODE_M68K_040",
    "CS_MODE_M68K_060",
    "CS_MODE_BIG_ENDIAN",
    "CS_MODE_MIPS16",
    "CS_MODE_MIPS32",
    "CS_MODE_MIPS64",
    "CS_MODE_MICRO",
    "CS_MODE_MIPS1",
    "CS_MODE_MIPS2",
    "CS_MODE_MIPS32R2",
    "CS_MODE_MIPS32R3",
    "CS_MODE_MIPS32R5",
    "CS_MODE_MIPS32R6",
    "CS_MODE_MIPS3",
    "CS_MODE_MIPS4",
    "CS_MODE_MIPS5",
    "CS_MODE_MIPS64R2",
    "CS_MODE_MIPS64R3",
    "CS_MODE_MIPS64R5",
    "CS_MODE_MIPS64R6",
    "CS_MODE_OCTEON",
    "CS_MODE_OCTEONP",
    "CS_MODE_NANOMIPS",
    "CS_MODE_NMS1",
    "CS_MODE_I7200",
    "CS_MODE_MIPS_NOFLOAT",
    "CS_MODE_MIPS_PTR64",
    "CS_MODE_MICRO32R3",
    "CS_MODE_MICRO32R6",
    "CS_MODE_RISCV32",
    "CS_MODE_RISCV64",
    "CS_MODE_RISCV_C",
    "CS_MODE_RISCV_A",
    "CS_MODE_RISCV_D",
    "CS_MODE_RISCV_F",
    "CS_MODE_RISCV_V",
    "CS_MODE_RISCV_E",
    "CS_MODE_RISCV_BITMANIP",
    "CS_MODE_RISCV_COREV",
    "CS_MODE_RISCV_SIFIVE",
    "CS_MODE_RISCV_THEAD",
    "CS_MODE_RISCV_ZBA",
    "CS_MODE_RISCV_ZBB",
    "CS_MODE_RISCV_ZBC",
    "CS_MODE_RISCV_ZBKB",
    "CS_MODE_RISCV_ZBKC",
    "CS_MODE_RISCV_ZBKX",
    "CS_MODE_RISCV_ZBS",
    "CS_MODE_RISCV_ZCMP_ZCMT_ZCE",
    "CS_MODE_RISCV_ZFINX",
    "CS_MODE_RISCV_ZICFISS",
    "CS_MODE_SH2",
    "CS_MODE_SH2A",
    "CS_MODE_SH3",
    "CS_MODE_SH4",
    "CS_MODE_SH4A",
    "CS_MODE_SHDSP",
    "CS_MODE_SHFPU",
    "CS_MODE_SYSTEMZ_GENERIC",
    "CS_MODE_SYSTEMZ_Z10",
    "CS_MODE_SYSTEMZ_Z196",
    "CS_MODE_SYSTEMZ_ZEC12",
    "CS_MODE_SYSTEMZ_ARCH8",
    "CS_MODE_SYSTEMZ_ARCH9",
    "CS_MODE_SYSTEMZ_ARCH10",
    "CS_MODE_SYSTEMZ_ARCH11",
    "CS_MODE_SYSTEMZ_ARCH12",
    "CS_MODE_SYSTEMZ_ARCH13",
    "CS_MODE_SYSTEMZ_ARCH14",
    "CS_MODE_SYSTEMZ_Z13",
    "CS_MODE_SYSTEMZ_Z14",
    "CS_MODE_SYSTEMZ_Z15",
    "CS_MODE_SYSTEMZ_Z16",
    "CS_MODE_TRICORE_110",
    "CS_MODE_TRICORE_120",
    "CS_MODE_TRICORE_130",
    "CS_MODE_TRICORE_131",
    "CS_MODE_TRICORE_160",
    "CS_MODE_TRICORE_161",
    "CS_MODE_TRICORE_162",
    "CS_MODE_TRICORE_180",
    "CS_MODE_BPF_CLASSIC",
    "CS_MODE_BPF_EXTENDED",
    "CS_MODE_HPPA_11",
    "CS_MODE_HPPA_20",
    "CS_MODE_HPPA_20W",
    "CS_MODE_LOONGARCH32",
    "CS_MODE_LOONGARCH64",
    "CS_MODE_XTENSA_ESP32",
    "CS_MODE_XTENSA_ESP32S2",
    "CS_MODE_XTENSA_ESP8266",
    "CS_MODE_M680X_6301",
    "CS_MODE_M680X_6309",
    "CS_MODE_M680X_6800",
    "CS_MODE_M680X_6801",
    "CS_MODE_M680X_6805",
    "CS_MODE_M680X_6808",
    "CS_MODE_M680X_6809",
    "CS_MODE_M680X_6811",
    "CS_MODE_M680X_CPU12",
    "CS_MODE_M680X_HCS08",
    "CS_MODE_M680X_RS08",
    "CS_MODE_MOS65XX_6502",
    "CS_MODE_MOS65XX_65816",
    "CS_MODE_MOS65XX_65816_LONG_M",
    "CS_MODE_MOS65XX_65816_LONG_MX",
    "CS_MODE_MOS65XX_65816_LONG_X",
    "CS_MODE_MOS65XX_65C02",
    "CS_MODE_MOS65XX_W65C02",
}

# Backward-compatible alias for code that references MODE_OPTIONS.
MODE_OPTIONS: Set[str] = ALL_MODE_OPTIONS

# Mode options that the test framework will accept (all of them).
SUPPORTED_MODE_OPTIONS: Set[str] = ALL_MODE_OPTIONS


def _bytes_to_hex(byte_list: List[int]) -> str:
    """Convert a list of byte ints to a contiguous lowercase hex string."""
    return "".join(f"{b:02x}" for b in byte_list)


def _resolve_arch(options: List[str]) -> Optional[str]:
    """Map Capstone arch+mode options to a Robustone architecture name."""
    arch = None
    mode_set = set()
    for opt in options:
        if opt.startswith("CS_ARCH_"):
            arch = opt
        elif opt in MODE_OPTIONS:
            mode_set.add(opt)
    if arch is None:
        return None
    return ARCH_MODE_MAP.get((arch, frozenset(mode_set)))


def _build_cstool_flags(options: List[str]) -> List[str]:
    """Derive extra cstool CLI flags from Capstone options."""
    flags: List[str] = []
    for opt in options:
        if opt in OPTION_TO_CSTOOL_FLAGS:
            for f in OPTION_TO_CSTOOL_FLAGS[opt]:
                if f not in flags:
                    flags.append(f)
    return flags


def _build_cstool_arch(arch: str, options: List[str]) -> str:
    # pylint: disable=too-many-return-statements,too-many-branches
    """Build cstool architecture string from Capstone options."""
    opts = set(options)
    be = "CS_MODE_BIG_ENDIAN" in opts
    le = "CS_MODE_LITTLE_ENDIAN" in opts

    # ---- RISC-V ----
    if arch == "CS_ARCH_RISCV":
        base = "riscv32" if "CS_MODE_RISCV32" in opts else "riscv64"
        extras: List[str] = []
        if "CS_MODE_RISCV_C" in opts:
            extras.append("+c")
        if "CS_MODE_RISCV_D" in opts:
            extras.append("+fd")
        elif "CS_MODE_RISCV_F" in opts:
            extras.append("+fd")
        if "CS_MODE_RISCV_V" in opts:
            extras.append("+v")
        if "CS_OPT_SYNTAX_NO_ALIAS_TEXT" in opts:
            extras.append("+noalias")
        return base + "".join(extras)

    # ---- ARM ----
    if arch == "CS_ARCH_ARM":
        base = "armbe" if be else "arm"
        if "CS_MODE_THUMB" in opts:
            base += "+thumb"
        return base

    # ---- AArch64 ----
    if arch == "CS_ARCH_AARCH64":
        return "aarch64be" if be else "aarch64"

    # ---- X86 ----
    if arch == "CS_ARCH_X86":
        if "CS_MODE_16" in opts:
            return "x16"
        if "CS_MODE_64" in opts:
            return "x64"
        return "x32"

    # ---- MIPS ----
    if arch == "CS_ARCH_MIPS":
        # Determine endianness suffix
        esuf = "be" if be else "el" if le else ""
        if "CS_MODE_MICRO" in opts:
            if "CS_MODE_MIPS32R3" in opts:
                return "micromipselr3" if le else "micromipsr3"
            if "CS_MODE_MIPS32R6" in opts:
                return "micromipselr6" if le else "micromipsr6"
            return f"micromips{esuf}"
        if "CS_MODE_OCTEON" in opts:
            return "octeonle" if le else "octeon"
        if "CS_MODE_OCTEONP" in opts:
            return "octeonple" if le else "octeonp"
        if "CS_MODE_NANOMIPS" in opts:
            return "nanomips"
        if "CS_MODE_NMS1" in opts:
            return "nms1"
        if "CS_MODE_I7200" in opts:
            return "i7200"
        # Generic mips variants
        if "CS_MODE_MIPS64R6" in opts:
            return "mipsel64r6" if le else "mips64r6"
        if "CS_MODE_MIPS64R5" in opts:
            return "mipsel64r5" if le else "mips64r5"
        if "CS_MODE_MIPS64R3" in opts:
            return "mipsel64r3" if le else "mips64r3"
        if "CS_MODE_MIPS64R2" in opts:
            return "mipsel64r2" if le else "mips64r2"
        if "CS_MODE_MIPS64" in opts or "CS_MODE_MIPS_PTR64" in opts:
            return "mipsel64" if le else "mips64"
        if "CS_MODE_MIPS32R6" in opts:
            return "mipsel32r6" if le else "mips32r6"
        if "CS_MODE_MIPS32R5" in opts:
            return "mipsel32r5" if le else "mips32r5"
        if "CS_MODE_MIPS32R3" in opts:
            return "mipsel32r3" if le else "mips32r3"
        if "CS_MODE_MIPS32R2" in opts:
            return "mipsel32r2" if le else "mips32r2"
        if "CS_MODE_MIPS32" in opts:
            # cstool uses 'mips' / 'mipsel' for generic MIPS32
            return "mipsel" if le else "mips"
        if "CS_MODE_MIPS5" in opts:
            return "mipsel5" if le else "mips5"
        if "CS_MODE_MIPS4" in opts:
            return "mipsel4" if le else "mips4"
        if "CS_MODE_MIPS3" in opts:
            return "mipsel3" if le else "mips3"
        if "CS_MODE_MIPS2" in opts:
            return "mipsel2" if le else "mips2"
        if "CS_MODE_MIPS1" in opts:
            return "mipsel1" if le else "mips1"
        if "CS_MODE_MIPS16" in opts:
            return "mipsel16" if le else "mips16"
        # Fallback
        return "mipsel" if le else "mips"

    # ---- PowerPC ----
    if arch == "CS_ARCH_PPC":
        is64 = "CS_MODE_64" in opts
        if be:
            base = "ppc64be" if is64 else "ppc32be"
        else:
            base = "ppc64" if is64 else "ppc32"
        if "CS_MODE_PWR10" in opts:
            return base + "pwr10"
        if "CS_MODE_PWR9" in opts:
            return base + "pwr9"
        if "CS_MODE_PWR8" in opts:
            return base + "pwr8"
        if "CS_MODE_PWR7" in opts:
            return base + "pwr7"
        if "CS_MODE_PPC_ISA_FUTURE" in opts:
            return base + "FutureISA"
        return base

    # ---- SPARC ----
    if arch == "CS_ARCH_SPARC":
        return "sparcle" if le else "sparc"

    # ---- SystemZ ----
    if arch == "CS_ARCH_SYSTEMZ":
        if "CS_MODE_SYSTEMZ_Z16" in opts:
            return "systemz_arch16"
        if "CS_MODE_SYSTEMZ_Z15" in opts:
            return "systemz_arch15"
        if "CS_MODE_SYSTEMZ_Z14" in opts:
            return "systemz_arch14"
        if "CS_MODE_SYSTEMZ_Z13" in opts:
            return "systemz_arch13"
        if "CS_MODE_SYSTEMZ_ARCH14" in opts:
            return "systemz_arch14"
        if "CS_MODE_SYSTEMZ_ARCH13" in opts:
            return "systemz_arch13"
        if "CS_MODE_SYSTEMZ_ARCH12" in opts:
            return "systemz_arch12"
        if "CS_MODE_SYSTEMZ_ARCH11" in opts:
            return "systemz_arch11"
        if "CS_MODE_SYSTEMZ_ARCH10" in opts:
            return "systemz_arch10"
        if "CS_MODE_SYSTEMZ_ARCH9" in opts:
            return "systemz_arch9"
        if "CS_MODE_SYSTEMZ_ARCH8" in opts:
            return "systemz_arch8"
        if "CS_MODE_SYSTEMZ_ZEC12" in opts:
            return "systemz_arch12"
        if "CS_MODE_SYSTEMZ_Z196" in opts:
            return "systemz_arch11"
        if "CS_MODE_SYSTEMZ_Z10" in opts:
            return "systemz_arch10"
        return "systemz"

    # ---- TriCore ----
    if arch == "CS_ARCH_TRICORE":
        if "CS_MODE_TRICORE_180" in opts:
            return "tc180"
        if "CS_MODE_TRICORE_162" in opts:
            return "tc162"
        if "CS_MODE_TRICORE_161" in opts:
            return "tc161"
        if "CS_MODE_TRICORE_160" in opts:
            return "tc160"
        if "CS_MODE_TRICORE_131" in opts:
            return "tc131"
        if "CS_MODE_TRICORE_130" in opts:
            return "tc130"
        if "CS_MODE_TRICORE_120" in opts:
            return "tc120"
        if "CS_MODE_TRICORE_110" in opts:
            return "tc110"
        return "tc162"

    # ---- BPF ----
    if arch == "CS_ARCH_BPF":
        is_ext = "CS_MODE_BPF_EXTENDED" in opts
        base = "ebpf" if is_ext else "bpf"
        if be:
            base += "be"
        return base

    # ---- HPPA ----
    if arch == "CS_ARCH_HPPA":
        if "CS_MODE_HPPA_20W" in opts:
            return "hppa20wbe" if be else "hppa20w"
        if "CS_MODE_HPPA_20" in opts:
            return "hppa20be" if be else "hppa20"
        if "CS_MODE_HPPA_11" in opts:
            return "hppa11be" if be else "hppa11"
        return "hppa11be" if be else "hppa11"

    # ---- LoongArch ----
    if arch == "CS_ARCH_LOONGARCH":
        if "CS_MODE_LOONGARCH32" in opts:
            return "loongarch32"
        return "loongarch64"

    # ---- Alpha ----
    if arch == "CS_ARCH_ALPHA":
        return "alphabe" if be else "alpha"

    # ---- ARC ----
    if arch == "CS_ARCH_ARC":
        return "arc"

    # ---- Xtensa ----
    if arch == "CS_ARCH_XTENSA":
        if "CS_MODE_XTENSA_ESP32" in opts:
            return "esp32"
        if "CS_MODE_XTENSA_ESP32S2" in opts:
            return "esp32s2"
        if "CS_MODE_XTENSA_ESP8266" in opts:
            return "esp8266"
        return "xtensa"

    # ---- M68K ----
    if arch == "CS_ARCH_M68K":
        return "m68k"

    # ---- EVM ----
    if arch == "CS_ARCH_EVM":
        return "evm"

    # ---- WASM ----
    if arch == "CS_ARCH_WASM":
        return "wasm"

    # ---- XCore ----
    if arch == "CS_ARCH_XCORE":
        return "xcore"

    # ---- TMS320C64X ----
    if arch == "CS_ARCH_TMS320C64X":
        return "tms320c64xle" if le else "tms320c64x"

    # ---- M680X ----
    if arch == "CS_ARCH_M680X":
        return "m6808"

    # ---- MOS65XX ----
    if arch == "CS_ARCH_MOS65XX":
        return "6502"

    # ---- SH ----
    if arch == "CS_ARCH_SH":
        if "CS_MODE_SH4A" in opts:
            return "sh4abe" if be else "sh4a"
        if "CS_MODE_SH4" in opts:
            return "sh4be" if be else "sh4"
        if "CS_MODE_SH3" in opts:
            return "sh3be" if be else "sh3"
        if "CS_MODE_SH2A" in opts:
            return "sh2a"
        if "CS_MODE_SH2" in opts:
            return "sh2"
        return "sh"

    # Fallback: return lower-case arch name without CS_ARCH_ prefix
    return arch.replace("CS_ARCH_", "").lower()


def _options_note(options: List[str]) -> str:
    """Build a human-readable note from Capstone options."""
    modes = [opt for opt in options if opt in MODE_OPTIONS]
    flags = [opt for opt in options if opt not in MODE_OPTIONS]
    parts = []
    if modes:
        parts.append("modes=" + ",".join(modes))
    if flags:
        parts.append("opts=" + ",".join(flags))
    return "; ".join(parts)


class YamlFilter:
    """Filter specification for Capstone YAML test cases."""

    def __init__(
        self,
        arch: Optional[str] = None,
        options_include: Optional[List[str]] = None,
        options_exclude: Optional[List[str]] = None,
        hex_exclude: Optional[List[str]] = None,
        require_single_insn: bool = True,
        only_supported_modes: bool = True,
    ):
        self.arch = arch
        self.options_include = set(options_include) if options_include else None
        self.options_exclude = set(options_exclude) if options_exclude else None
        self.hex_exclude = set(hex_exclude) if hex_exclude else None
        self.require_single_insn = require_single_insn
        self.only_supported_modes = only_supported_modes

    def matches(self, test_case: dict) -> bool:
        # pylint: disable=too-many-return-statements
        inp = test_case.get("input", {})
        opts = set(inp.get("options", []))

        if self.arch is not None:
            if inp.get("arch") != self.arch:
                return False

        if self.options_include is not None:
            if not self.options_include.issubset(opts):
                return False

        if self.options_exclude is not None:
            if self.options_exclude & opts:
                return False

        if self.hex_exclude is not None:
            byte_list = inp.get("bytes", [])
            hex_str = _bytes_to_hex(byte_list)
            if hex_str in self.hex_exclude:
                return False

        if self.only_supported_modes:
            mode_opts = {opt for opt in opts if opt.startswith("CS_MODE_")}
            if not mode_opts.issubset(SUPPORTED_MODE_OPTIONS):
                return False

        if self.require_single_insn:
            expected = test_case.get("expected", {})
            if len(expected.get("insns", [])) != 1:
                return False

        return True


def load_yaml_test_cases(
    yaml_source: str,
    yaml_filter: Optional[Dict] = None,
    limit: Optional[int] = None,
) -> Iterator[Tuple[str, str, str]]:
    """
    Load test cases from Capstone YAML files.

    Args:
        yaml_source: Path to a YAML file or a directory containing YAML files.
                     Glob patterns are supported.
        yaml_filter: Optional filter dict with keys:
            - arch: str, required Capstone arch name
            - options_include: list of option strings that must be present
            - options_exclude: list of option strings that must NOT be present
            - require_single_insn: bool (default True)
        limit: Maximum number of test cases to yield.

    Yields:
        Tuples of (hex_input, expected_asm_text, note).
    """
    filt = YamlFilter(
        arch=yaml_filter.get("arch") if yaml_filter else None,
        options_include=yaml_filter.get("options_include") if yaml_filter else None,
        options_exclude=yaml_filter.get("options_exclude") if yaml_filter else None,
        hex_exclude=yaml_filter.get("hex_exclude") if yaml_filter else None,
        require_single_insn=(
            yaml_filter.get("require_single_insn", True) if yaml_filter else True
        ),
        only_supported_modes=(
            yaml_filter.get("only_supported_modes", True) if yaml_filter else True
        ),
    )

    source_path = Path(yaml_source)
    if source_path.is_dir():
        yaml_files = sorted(source_path.rglob("*.yaml"))
    else:
        yaml_files = [source_path]

    count = 0
    for yaml_file in yaml_files:
        try:
            with yaml_file.open("r", encoding="utf-8") as fh:
                data = yaml.safe_load(fh)
        except yaml.YAMLError:
            # Skip malformed YAML files gracefully
            continue

        if not isinstance(data, dict):
            continue

        for test_case in data.get("test_cases", []):
            if not filt.matches(test_case):
                continue

            inp = test_case.get("input", {})
            expected = test_case.get("expected", {})
            insns = expected.get("insns", [])

            if not insns:
                continue

            byte_list = inp.get("bytes", [])
            hex_input = _bytes_to_hex(byte_list)
            expected_text = insns[0].get("asm_text", "")
            opts = inp.get("options", [])
            arch = inp.get("arch", "")
            note_parts = [_options_note(opts)]
            cstool_arch = _build_cstool_arch(arch, opts)
            note_parts.append(f"cstool_arch={cstool_arch}")
            note = "; ".join(note_parts)

            yield (hex_input, expected_text, note)
            count += 1

            if limit is not None and count >= limit:
                return


def count_yaml_test_cases(
    yaml_source: str,
    yaml_filter: Optional[Dict] = None,
) -> int:
    """Count test cases without fully loading them."""
    return sum(1 for _ in load_yaml_test_cases(yaml_source, yaml_filter))
