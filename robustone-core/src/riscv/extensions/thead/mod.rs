//! T-Head (XuanTie) custom-specific extensions and configuration.
//!
//! This module defines the `THeadExtensions` bitflags for custom-specific
//! extensions and re-exports the corresponding extension handler types
//! under the `thead` namespace.

use bitflags::bitflags;

pub mod condmov;

pub use condmov::CMov;

bitflags! {
    /// Bitflags representing enabled T-Head custom extensions.
    pub struct THead: u32 {
        /// Conditional move extension (XTheadCondMov).
        const CMOV = 1;
    }
}
