/// 验证架构字符串（保持原有签名以兼容现有调用处）
pub fn validate_architecture(arch_str: &str) -> Result<String, String> {
    // 扩展的架构前缀验证，基于Capstone支持
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

    // 检查基础架构是否有效
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
