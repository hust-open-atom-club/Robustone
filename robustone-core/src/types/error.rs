use thiserror::Error;

/// Machine-readable decode failure classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeErrorKind {
    NeedMoreBytes,
    InvalidEncoding,
    UnsupportedExtension,
    UnimplementedInstruction,
    UnsupportedMode,
}

impl std::fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            DecodeErrorKind::NeedMoreBytes => "need_more_bytes",
            DecodeErrorKind::InvalidEncoding => "invalid_encoding",
            DecodeErrorKind::UnsupportedExtension => "unsupported_extension",
            DecodeErrorKind::UnimplementedInstruction => "unimplemented_instruction",
            DecodeErrorKind::UnsupportedMode => "unsupported_mode",
        };
        write!(f, "{text}")
    }
}

/// Structured decode failure payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeFailure {
    pub kind: DecodeErrorKind,
    pub architecture: Option<String>,
    pub detail: String,
}

impl DecodeFailure {
    /// Create a new structured decode failure.
    pub fn new(
        kind: DecodeErrorKind,
        architecture: impl Into<Option<String>>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            architecture: architecture.into(),
            detail: detail.into(),
        }
    }
}

/// Errors produced by the architecture-agnostic disassembly layer.
#[derive(Error, Debug)]
pub enum DisasmError {
    #[error("ERROR: Unsupported architecture: {0}")]
    UnsupportedArchitecture(String),
    #[error(
        "ERROR: decoding failed ({kind}){arch}: {detail}",
        arch = architecture
            .as_ref()
            .map(|arch| format!(" for {arch}"))
            .unwrap_or_default()
    )]
    DecodeFailure {
        kind: DecodeErrorKind,
        architecture: Option<String>,
        detail: String,
    },
    #[error("ERROR: Decoding failed: {0}")]
    DecodingError(String),
    #[error("ERROR: invalid assembly code: {0}")]
    InvalidHexCode(String),
    #[error("ERROR: invalid address argument: {0}")]
    InvalidAddress(String),
}

impl DisasmError {
    /// Create a structured decode failure.
    pub fn decode_failure(
        kind: DecodeErrorKind,
        architecture: impl Into<Option<String>>,
        detail: impl Into<String>,
    ) -> Self {
        let failure = DecodeFailure::new(kind, architecture, detail);
        Self::DecodeFailure {
            kind: failure.kind,
            architecture: failure.architecture,
            detail: failure.detail,
        }
    }

    /// Return a stable, machine-readable error kind identifier.
    pub fn stable_kind(&self) -> &'static str {
        match self {
            DisasmError::UnsupportedArchitecture(_) => "unsupported_architecture",
            DisasmError::DecodeFailure { kind, .. } => match kind {
                DecodeErrorKind::NeedMoreBytes => "need_more_bytes",
                DecodeErrorKind::InvalidEncoding => "invalid_encoding",
                DecodeErrorKind::UnsupportedExtension => "unsupported_extension",
                DecodeErrorKind::UnimplementedInstruction => "unimplemented_instruction",
                DecodeErrorKind::UnsupportedMode => "unsupported_mode",
            },
            DisasmError::DecodingError(_) => "decoding_error",
            DisasmError::InvalidHexCode(_) => "invalid_hex_code",
            DisasmError::InvalidAddress(_) => "invalid_address",
        }
    }

    /// Return the architecture involved in this error when available.
    pub fn architecture_name(&self) -> Option<&str> {
        match self {
            DisasmError::UnsupportedArchitecture(arch) => Some(arch.as_str()),
            DisasmError::DecodeFailure { architecture, .. } => architecture.as_deref(),
            _ => None,
        }
    }

    /// Return the human-readable detail payload without the prefixed display wrapper.
    pub fn detail_message(&self) -> String {
        match self {
            DisasmError::UnsupportedArchitecture(arch) => {
                format!("unsupported architecture: {arch}")
            }
            DisasmError::DecodeFailure { detail, .. } => detail.clone(),
            DisasmError::DecodingError(detail) => detail.clone(),
            DisasmError::InvalidHexCode(detail) => detail.clone(),
            DisasmError::InvalidAddress(detail) => detail.clone(),
        }
    }
}
