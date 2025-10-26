/// Validate an architecture string while preserving the legacy return type.
pub fn validate_architecture(arch_str: &str) -> Result<String, String> {
    // Expanded list of architecture prefixes derived from Capstone support.
    let valid_prefixes = [
        // RISC-V
        "riscv32",
        "riscv64",
        "riscv32e",
        // ARM
        "arm",
        "armle",
        "armbe",
        "thumb",
        // AArch64
        "aarch64",
        "aarch64be",
        // x86
        "x16",
        "x32",
        "x86",
        "x64",
        "x86-64",
        "x86_64",
        // MIPS
        "mips",
        "mipsel",
        "mips64",
        "mips64el",
        // PowerPC
        "ppc",
        "powerpc",
        "ppc32",
        "powerpc32",
        "ppcbe",
        "powerpcbe",
        "ppc32be",
        "powerpc32be",
        "ppc64",
        "powerpc64",
        "ppc64be",
        "powerpc64be",
        // SPARC
        "sparc",
        "sparcle",
        "sparc64",
        // Other
        "systemz",
        "s390x",
        "xcore",
        "m68k",
        "tms320c64x",
        "c64x",
        "m680x",
        "evm",
        "bpf",
    ];

    let arch_str_lower = arch_str.to_lowercase();
    let parts: Vec<&str> = arch_str_lower.split('+').collect();

    if parts.is_empty() {
        return Err("Empty architecture string".to_string());
    }

    // Ensure the base architecture is supported before considering modifiers.
    let base_arch = parts[0];
    let is_valid = valid_prefixes.iter().any(|&prefix| base_arch == prefix);

    if !is_valid {
        return Err(format!(
            "Invalid architecture: {}. Supported: riscv32, riscv64, arm, armle, armbe, aarch64, x16, x32, x64, mips, mipsel, ppc, ppc64, sparc, systemz, and others",
            base_arch
        ));
    }

    Ok(arch_str.to_string())
}
