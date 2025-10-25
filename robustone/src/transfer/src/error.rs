/// 反汇编错误
#[derive(Debug)]
pub enum DisasmError {
    UnsupportedArchitecture(String),
    DecodingError(String),
    InvalidHexCode(String),
    InvalidAddress(String),
}

impl std::fmt::Display for DisasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DisasmError::UnsupportedArchitecture(arch) => {
                write!(f, "ERROR: Unsupported architecture: {}", arch)
            }
            DisasmError::DecodingError(msg) => {
                write!(f, "ERROR: Decoding failed: {}", msg)
            }
            DisasmError::InvalidHexCode(msg) => {
                write!(f, "ERROR: invalid assembly code: {}", msg)
            }
            DisasmError::InvalidAddress(msg) => {
                write!(f, "ERROR: invalid address argument: {}", msg)
            }
        }
    }
}

impl std::error::Error for DisasmError {}