use std::fmt;

/// Unified CLI error type that consolidates all error categories.
#[derive(Debug, Clone)]
pub enum CliError {
    /// Architecture parsing failure.
    Architecture(String),
    /// Configuration validation failure.
    Configuration(String),
    /// Disassembly backend failure.
    Disassembly(String),
    /// Validation errors with detailed context.
    Validation { field: String, message: String },
    /// Parse errors with detailed context.
    Parse { context: String, message: String },
    /// I/O errors from file operations.
    Io(String),
    /// Generic errors.
    Generic(String),
    /// Missing required argument.
    MissingArgument(String),
    /// Invalid command request.
    InvalidCommand(String),
    /// Invalid hexadecimal code.
    InvalidHex(String),
    /// Invalid address format.
    InvalidAddress(String),
}

impl CliError {
    /// Create a new validation error.
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new parse error.
    pub fn parse(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Parse {
            context: context.into(),
            message: message.into(),
        }
    }

    /// Create a generic error.
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic(message.into())
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Architecture(msg) => write!(f, "Architecture error: {msg}"),
            CliError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            CliError::Disassembly(msg) => write!(f, "Disassembly error: {msg}"),
            CliError::Validation { field, message } => {
                write!(f, "Validation error for '{field}': {message}")
            }
            CliError::Parse { context, message } => {
                write!(f, "Parse error in '{context}': {message}")
            }
            CliError::Io(err) => write!(f, "I/O error: {err}"),
            CliError::Generic(msg) => write!(f, "Error: {msg}"),
            CliError::MissingArgument(msg) => write!(f, "Missing required argument: {msg}"),
            CliError::InvalidCommand(msg) => write!(f, "Invalid command: {msg}"),
            CliError::InvalidHex(msg) => write!(f, "Invalid hex code: {msg}"),
            CliError::InvalidAddress(msg) => write!(f, "Invalid address: {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

// Automatic conversions from common error types
impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<ValidationError> for CliError {
    fn from(err: ValidationError) -> Self {
        Self::validation("input", err.to_string())
    }
}

impl From<ParseError> for CliError {
    fn from(err: ParseError) -> Self {
        Self::parse("architecture", err.to_string())
    }
}

/// Legacy validation error types for backward compatibility.
#[derive(Debug)]
pub enum ValidationError {
    EmptyHexCode,
    OddHexLength,
    InvalidHexChar(char),
    EmptyAddress,
    InvalidAddressFormat,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::EmptyHexCode => write!(f, "Empty hex code provided"),
            ValidationError::OddHexLength => {
                write!(f, "Hex code must have even number of characters")
            }
            ValidationError::InvalidHexChar(c) => write!(f, "Invalid hex character: {c}"),
            ValidationError::EmptyAddress => write!(f, "Empty address provided"),
            ValidationError::InvalidAddressFormat => write!(f, "Invalid address format"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Legacy parse error types for backward compatibility.
#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    UnknownArchitecture(String),
    UnknownMode(String),
    UnknownOption(String),
    InvalidFormat(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty architecture input"),
            ParseError::UnknownArchitecture(arch) => write!(f, "Unknown architecture: {arch}"),
            ParseError::UnknownMode(mode) => write!(f, "Unknown mode: {mode}"),
            ParseError::UnknownOption(option) => write!(f, "Unknown option: {option}"),
            ParseError::InvalidFormat(format) => write!(f, "Invalid format: {format}"),
        }
    }
}

impl std::error::Error for ParseError {}

pub type Result<T> = std::result::Result<T, CliError>;
