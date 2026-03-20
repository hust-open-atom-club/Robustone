use std::str::FromStr;

use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Architecture {
    // RISC-V variants
    Riscv32,
    Riscv64,
    Riscv32E,

    // 32-bit ARM variants
    Arm,
    ArmLE,
    ArmBE,
    Thumb,

    // 64-bit ARM variants
    Aarch64,
    Aarch64BE,

    // x86 family
    X86_16,
    X86_32,
    X86_64,

    // MIPS family
    Mips,
    MipsEL,
    Mips64,
    MipsEL64,

    // PowerPC family
    PowerPC32,
    PowerPC32BE,
    PowerPC64,
    PowerPC64BE,

    // SPARC family
    Sparc,
    SparcLE,
    Sparc64,

    // Other architectures
    SystemZ,
    Xcore,
    M68k,
    Tms320c64x,
    M680x,
    Evm,
    Bpf,
}

/// Architecture specification holding the resolved architecture, mode flags, and modifiers.
#[derive(Clone)]
pub struct ArchitectureSpec {
    pub arch: Architecture,
    pub mode: u32,            // Capstone mode bitmask
    pub options: Vec<String>, // Architecture-specific option modifiers
}

impl ArchitectureSpec {
    /// Parses an architecture string, supporting `+`-separated modifiers.
    pub fn parse(input: &str) -> std::result::Result<Self, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let parts: Vec<&str> = input.split('+').collect();
        if parts.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Interpret the base architecture token.
        let mut arch = Architecture::from_str(parts[0])
            .map_err(|_| ParseError::UnknownArchitecture(parts[0].to_string()))?;

        let mut mode = arch.default_mode();
        let mut options = Vec::new();

        // Apply modifiers according to Capstone semantics.
        for modifier in &parts[1..] {
            let canonical_modifier = normalize_modifier(modifier);
            if !arch.supports_modifier(&canonical_modifier) {
                return Err(ParseError::UnknownOption(modifier.to_string()));
            }

            if matches!(canonical_modifier.as_str(), "be" | "le") {
                arch = arch.apply_endianness_modifier(&canonical_modifier)?;
                mode = arch.default_mode();
                continue;
            }

            options.push(canonical_modifier);
        }

        Ok(ArchitectureSpec {
            arch,
            mode,
            options,
        })
    }
}

impl std::fmt::Debug for ArchitectureSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch_name = self.arch.name();
        if self.options.is_empty() {
            write!(f, "ArchitectureSpec({arch_name})",)
        } else {
            let options_str = self.options.join("+");
            write!(f, "ArchitectureSpec({arch_name}+{options_str})")
        }
    }
}

impl Architecture {
    fn supports_modifier(&self, modifier: &str) -> bool {
        match self {
            Architecture::Arm | Architecture::ArmLE | Architecture::ArmBE | Architecture::Thumb => {
                matches!(modifier, "thumb" | "m" | "v8" | "noregname" | "regalias" | "be" | "le")
            }
            Architecture::Aarch64 | Architecture::Aarch64BE => {
                matches!(modifier, "apple" | "noregname" | "regalias" | "be" | "le")
            }
            Architecture::X86_16 | Architecture::X86_32 | Architecture::X86_64 => {
                matches!(modifier, "att" | "intel" | "masm" | "nasm")
            }
            Architecture::Mips
            | Architecture::MipsEL
            | Architecture::Mips64
            | Architecture::MipsEL64 => {
                matches!(modifier, "nofloat" | "ptr64" | "noregname" | "nodollar" | "be" | "le")
            }
            Architecture::PowerPC32
            | Architecture::PowerPC32BE
            | Architecture::PowerPC64
            | Architecture::PowerPC64BE => {
                matches!(
                    modifier,
                    "aix"
                        | "booke"
                        | "maix"
                        | "msync"
                        | "qpx"
                        | "ps"
                        | "spe"
                        | "noregname"
                        | "percentage"
                        | "be"
                        | "le"
                )
            }
            Architecture::Sparc | Architecture::SparcLE | Architecture::Sparc64 => {
                matches!(modifier, "v9" | "be" | "le")
            }
            _ => false,
        }
    }

    fn apply_endianness_modifier(&self, modifier: &str) -> std::result::Result<Self, ParseError> {
        match (self, modifier) {
            (Architecture::Arm | Architecture::ArmBE, "be") => Ok(Architecture::ArmBE),
            (Architecture::Arm | Architecture::ArmLE, "le") => Ok(Architecture::ArmLE),
            (Architecture::Aarch64 | Architecture::Aarch64BE, "be") => Ok(Architecture::Aarch64BE),
            (Architecture::Aarch64 | Architecture::Aarch64BE, "le") => Ok(Architecture::Aarch64),
            (Architecture::Mips, "be") | (Architecture::Mips64, "be") => Ok(self.clone()),
            (Architecture::MipsEL, "le") | (Architecture::MipsEL64, "le") => Ok(self.clone()),
            (Architecture::Mips, "le") => Ok(Architecture::MipsEL),
            (Architecture::Mips64, "le") => Ok(Architecture::MipsEL64),
            (Architecture::MipsEL, "be") => Ok(Architecture::Mips),
            (Architecture::MipsEL64, "be") => Ok(Architecture::Mips64),
            (Architecture::PowerPC32 | Architecture::PowerPC32BE, "be") => {
                Ok(Architecture::PowerPC32BE)
            }
            (Architecture::PowerPC32 | Architecture::PowerPC32BE, "le") => {
                Ok(Architecture::PowerPC32)
            }
            (Architecture::PowerPC64 | Architecture::PowerPC64BE, "be") => {
                Ok(Architecture::PowerPC64BE)
            }
            (Architecture::PowerPC64 | Architecture::PowerPC64BE, "le") => {
                Ok(Architecture::PowerPC64)
            }
            (Architecture::Sparc | Architecture::SparcLE, "be") => Ok(Architecture::Sparc),
            (Architecture::Sparc | Architecture::SparcLE, "le") => Ok(Architecture::SparcLE),
            _ => Err(ParseError::UnknownOption(modifier.to_string())),
        }
    }

    pub fn default_mode(&self) -> u32 {
        match self {
            Architecture::Riscv32 | Architecture::Riscv64 | Architecture::Riscv32E => 0x0,
            Architecture::Arm | Architecture::Aarch64 => 0x0,
            Architecture::X86_16 | Architecture::X86_32 | Architecture::X86_64 => 0x0,
            Architecture::Mips
            | Architecture::MipsEL
            | Architecture::Mips64
            | Architecture::MipsEL64 => 0x0,
            Architecture::PowerPC32
            | Architecture::PowerPC32BE
            | Architecture::PowerPC64
            | Architecture::PowerPC64BE => 0x0,
            Architecture::Sparc | Architecture::Sparc64 => 0x0,
            _ => 0x0,
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            // RISC-V
            "riscv32" => Ok(Architecture::Riscv32),
            "riscv64" => Ok(Architecture::Riscv64),
            "riscv32e" => Ok(Architecture::Riscv32E),

            // ARM
            "arm" => Ok(Architecture::Arm),
            "armle" => Ok(Architecture::ArmLE),
            "armbe" => Ok(Architecture::ArmBE),
            "thumb" => Ok(Architecture::Thumb),

            // AArch64
            "aarch64" => Ok(Architecture::Aarch64),
            "aarch64be" => Ok(Architecture::Aarch64BE),

            // x86
            "x16" => Ok(Architecture::X86_16),
            "x32" => Ok(Architecture::X86_32),
            "x86" => Ok(Architecture::X86_32),
            "x64" | "x86-64" | "x86_64" => Ok(Architecture::X86_64),

            // MIPS
            "mips" => Ok(Architecture::Mips),
            "mipsel" => Ok(Architecture::MipsEL),
            "mips64" => Ok(Architecture::Mips64),
            "mips64el" => Ok(Architecture::MipsEL64),

            // PowerPC
            "ppc" | "powerpc" | "ppc32" => Ok(Architecture::PowerPC32),
            "powerpc32" => Ok(Architecture::PowerPC32),
            "ppcbe" | "powerpcbe" | "ppc32be" => Ok(Architecture::PowerPC32BE),
            "powerpc32be" => Ok(Architecture::PowerPC32BE),
            "ppc64" | "powerpc64" => Ok(Architecture::PowerPC64),
            "ppc64be" | "powerpc64be" => Ok(Architecture::PowerPC64BE),

            // SPARC
            "sparc" => Ok(Architecture::Sparc),
            "sparcle" => Ok(Architecture::SparcLE),
            "sparc64" => Ok(Architecture::Sparc64),

            // Other architectures
            "systemz" | "s390x" => Ok(Architecture::SystemZ),
            "xcore" => Ok(Architecture::Xcore),
            "m68k" => Ok(Architecture::M68k),
            "tms320c64x" | "c64x" => Ok(Architecture::Tms320c64x),
            "m680x" => Ok(Architecture::M680x),
            "evm" => Ok(Architecture::Evm),
            "bpf" => Ok(Architecture::Bpf),

            _ => Err(format!(
                "Invalid <arch+mode>: {input}. Supported: riscv32, riscv64, riscv32e, arm, armle, armbe, thumb, aarch64, aarch64be, x16, x32, x64, mips, mipsel, mips64, mips64el, ppc, ppc32, ppc64, sparc, sparc64, systemz, and others"
            )),
        }
    }
}

fn normalize_modifier(modifier: &str) -> String {
    match modifier.to_lowercase().as_str() {
        "at&t" => "att".to_string(),
        "micro" => "m".to_string(),
        "big" => "be".to_string(),
        "little" => "le".to_string(),
        canonical => canonical.to_string(),
    }
}

impl FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Architecture {
    pub fn name(&self) -> &'static str {
        match self {
            // RISC-V
            Architecture::Riscv32 => "riscv32",
            Architecture::Riscv64 => "riscv64",
            Architecture::Riscv32E => "riscv32e",

            // ARM
            Architecture::Arm => "arm",
            Architecture::ArmLE => "armle",
            Architecture::ArmBE => "armbe",
            Architecture::Thumb => "thumb",

            // AArch64
            Architecture::Aarch64 => "aarch64",
            Architecture::Aarch64BE => "aarch64be",

            // x86
            Architecture::X86_16 => "x16",
            Architecture::X86_32 => "x32",
            Architecture::X86_64 => "x64",

            // MIPS
            Architecture::Mips => "mips",
            Architecture::MipsEL => "mipsel",
            Architecture::Mips64 => "mips64",
            Architecture::MipsEL64 => "mips64el",

            // PowerPC
            Architecture::PowerPC32 => "powerpc32",
            Architecture::PowerPC32BE => "powerpc32be",
            Architecture::PowerPC64 => "powerpc64",
            Architecture::PowerPC64BE => "powerpc64be",

            // SPARC
            Architecture::Sparc => "sparc",
            Architecture::SparcLE => "sparcle",
            Architecture::Sparc64 => "sparc64",

            // Other architectures
            Architecture::SystemZ => "systemz",
            Architecture::Xcore => "xcore",
            Architecture::M68k => "m68k",
            Architecture::Tms320c64x => "tms320c64x",
            Architecture::M680x => "m680x",
            Architecture::Evm => "evm",
            Architecture::Bpf => "bpf",
        }
    }

    pub fn is_implemented(&self) -> bool {
        matches!(self, Architecture::Riscv32 | Architecture::Riscv64)
    }

    pub fn implementation_status(&self) -> &'static str {
        if self.is_implemented() { "✅" } else { "🚧" }
    }

    pub fn category(&self) -> &'static str {
        match self {
            // RISC-V
            Architecture::Riscv32 | Architecture::Riscv64 | Architecture::Riscv32E => "RISC-V",

            // ARM
            Architecture::Arm | Architecture::ArmLE | Architecture::ArmBE | Architecture::Thumb => {
                "ARM"
            }

            // AArch64
            Architecture::Aarch64 | Architecture::Aarch64BE => "ARM",

            // x86
            Architecture::X86_16 | Architecture::X86_32 | Architecture::X86_64 => "x86",

            // MIPS
            Architecture::Mips
            | Architecture::MipsEL
            | Architecture::Mips64
            | Architecture::MipsEL64 => "MIPS",

            // PowerPC
            Architecture::PowerPC32
            | Architecture::PowerPC32BE
            | Architecture::PowerPC64
            | Architecture::PowerPC64BE => "PowerPC",

            // SPARC
            Architecture::Sparc | Architecture::SparcLE | Architecture::Sparc64 => "SPARC",

            // Other
            Architecture::SystemZ
            | Architecture::Xcore
            | Architecture::M68k
            | Architecture::Tms320c64x
            | Architecture::M680x
            | Architecture::Evm
            | Architecture::Bpf => "Other",
        }
    }

    pub fn all_architectures() -> Vec<Self> {
        vec![
            // RISC-V
            Architecture::Riscv32,
            Architecture::Riscv64,
            Architecture::Riscv32E,
            // ARM
            Architecture::Arm,
            Architecture::ArmLE,
            Architecture::ArmBE,
            Architecture::Thumb,
            // AArch64
            Architecture::Aarch64,
            Architecture::Aarch64BE,
            // x86
            Architecture::X86_16,
            Architecture::X86_32,
            Architecture::X86_64,
            // MIPS
            Architecture::Mips,
            Architecture::MipsEL,
            Architecture::Mips64,
            Architecture::MipsEL64,
            // PowerPC
            Architecture::PowerPC32,
            Architecture::PowerPC32BE,
            Architecture::PowerPC64,
            Architecture::PowerPC64BE,
            // SPARC
            Architecture::Sparc,
            Architecture::SparcLE,
            Architecture::Sparc64,
            // Other architectures
            Architecture::SystemZ,
            Architecture::Xcore,
            Architecture::M68k,
            Architecture::Tms320c64x,
            Architecture::M680x,
            Architecture::Evm,
            Architecture::Bpf,
        ]
    }
}
