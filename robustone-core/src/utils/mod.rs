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

pub mod hex;
pub mod endian;

// Re-export main utilities with explicit names to avoid conflicts
pub use hex::HexParser;
pub use endian::{Endianness, EndianConvert};