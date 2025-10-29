//! Utility modules for common functionality across architectures.
//!
//! This module provides reusable utilities that can be used by different
//! architecture handlers to avoid code duplication and provide consistent
//! behavior across the disassembler.
//!
//! # Modules
//!
//! - [`hex`]: Hexadecimal string parsing utilities
//! - [`endian`]: Endianness handling utilities for multi-architecture support

pub mod endian;
pub mod hex;

// Re-export main utilities with explicit names to avoid conflicts
pub use endian::{EndianConvert, Endianness};
pub use hex::HexParser;
