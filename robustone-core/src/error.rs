use thiserror::Error;

/// Errors produced by the architecture-agnostic disassembly layer.
#[derive(Error, Debug)]
pub enum DisasmError {
    #[error("ERROR: Unsupported architecture: {0}")]
    UnsupportedArchitecture(String),
    #[error("ERROR: Decoding failed: {0}")]
    DecodingError(String),
    #[error("ERROR: invalid assembly code: {0}")]
    InvalidHexCode(String),
    #[error("ERROR: invalid address argument: {0}")]
    InvalidAddress(String),
}
