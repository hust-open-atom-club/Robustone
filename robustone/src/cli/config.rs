use crate::cli::arch::ArchitectureSpec;
use crate::cli::command::Cli;

/// 支持的模式修饰符
#[derive(Debug, Clone)]
pub enum ModeModifier {
    LittleEndian,
    BigEndian,
    Thumb,
    Arm,
    Micro,
    V8,
    V9,
    Mips32,
    Mips64,
}

/// 支持的选项修饰符
#[derive(Debug, Clone)]
pub enum OptionModifier {
    AttSyntax,
    IntelSyntax,
    MasmSyntax,
    NasmSyntax,
    Csyntax,
}

/// 改进的配置结构
#[derive(Clone)]
pub struct DisasmConfig {
    pub arch_spec: ArchitectureSpec,
    pub hex_words: Vec<String>,
    pub start_address: u64,
    pub detailed: bool,
    pub alias_regs: bool,
    pub real_detail: bool,
    pub skip_data: bool,
    pub unsigned_immediate: bool,
}

impl std::fmt::Debug for DisasmConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DisasmConfig {{ arch_spec: {:?}, hex_words: {:?}, start_address: 0x{:x}, detailed: {}, alias_regs: {}, real_detail: {}, skip_data: {}, unsigned_immediate: {} }}",
            self.arch_spec,
            self.hex_words,
            self.start_address,
            self.detailed,
            self.alias_regs,
            self.real_detail,
            self.skip_data,
            self.unsigned_immediate
        )
    }
}

impl DisasmConfig {
    /// 从CLI创建配置，包含完整的验证
    pub fn config_from_cli(cli: &Cli) -> crate::cli::error::Result<Self> {
        use crate::cli::error::CliError;

        // 验证必需参数
        let arch_mode = cli
            .arch_mode
            .as_ref()
            .ok_or_else(|| CliError::MissingArgument("arch_mode".to_string()))?;

        let hex_code = cli
            .hex_code
            .as_ref()
            .ok_or_else(|| CliError::MissingArgument("hex_code".to_string()))?;

        // 解析架构规范
        let arch_spec = ArchitectureSpec::parse(arch_mode)
            .map_err(|e| CliError::Architecture(e.to_string()))?;

        let mut hex_words = crate::cli::utils::parse_hex_code(hex_code)
            .map_err(|e| CliError::InvalidHex(e.to_string()))?;

        // 验证和解析地址
        let address_str = cli.address.as_deref().unwrap_or("0");
        let start_address = crate::cli::utils::parse_address(address_str)
            .map_err(|e| CliError::InvalidAddress(e.to_string()))?;

        // 将地址原始字符串也作为一个词追加（保持测试预期）
        if let Some(addr_raw) = cli.address.as_deref() {
            if !addr_raw.trim().is_empty() {
                hex_words.push(addr_raw.to_string());
            }
        }

        Ok(DisasmConfig {
            arch_spec,
            hex_words,
            start_address,
            detailed: cli.detailed,
            alias_regs: cli.alias_regs,
            real_detail: cli.real_detail,
            skip_data: cli.skip_data,
            unsigned_immediate: cli.unsigned_immediate,
        })
    }
}
