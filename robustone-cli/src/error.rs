use std::fmt;

/// Unified CLI error type.
#[derive(Debug)]
pub enum CliError {
    /// Architecture parsing failure.
    Architecture(String),
    /// Configuration validation failure.
    Configuration(String),
    /// Disassembly backend failure.
    Disassembly(String),
    /// Invalid hexadecimal payload.
    InvalidHex(String),
    /// Invalid start address.
    InvalidAddress(String),
    /// Missing required argument.
    MissingArgument(String),
    /// Invalid command request.
    InvalidCommand(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Architecture(msg) => write!(f, "Architecture error: {}", msg),
            CliError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            CliError::Disassembly(msg) => write!(f, "Disassembly error: {}", msg),
            CliError::InvalidHex(msg) => write!(f, "Invalid hex code: {}", msg),
            CliError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            CliError::MissingArgument(msg) => write!(f, "Missing required argument: {}", msg),
            CliError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

/// Detailed validation error variants used by the CLI.
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
            ValidationError::InvalidHexChar(c) => write!(f, "Invalid hex character: {}", c),
            ValidationError::EmptyAddress => write!(f, "Empty address provided"),
            ValidationError::InvalidAddressFormat => write!(f, "Invalid address format"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Architecture parsing errors.
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
            ParseError::UnknownArchitecture(arch) => write!(f, "Unknown architecture: {}", arch),
            ParseError::UnknownMode(mode) => write!(f, "Unknown mode: {}", mode),
            ParseError::UnknownOption(option) => write!(f, "Unknown option: {}", option),
            ParseError::InvalidFormat(format) => write!(f, "Invalid format: {}", format),
        }
    }
}

impl std::error::Error for ParseError {}

pub type Result<T> = std::result::Result<T, CliError>;
