//! Standard RISC-V extensions and configuration.
//!
//! This module defines the `StandardExtensions` bitflags for core RISC-V
//! extensions (I/M/A/F/D/C) and re-exports the corresponding extension
//! handler types under the `standard` namespace.

use bitflags::bitflags;

pub mod rva;
pub mod rvc;
pub mod rvd;
pub mod rvf;
pub mod rvi;
pub mod rvm;

pub use rva::Rva;
pub use rvc::Rvc;
pub use rvd::Rvd;
pub use rvf::Rvf;
pub use rvi::Rvi;
pub use rvm::Rvm;

bitflags! {
    /// Bitflags representing enabled standard RISC-V extensions.
    pub struct Standard: u32 {
        const I    = 1;
        const M    = 1 << 1;
        const A    = 1 << 2;
        const F    = 1 << 3;
        const D    = 1 << 4;
        const C    = 1 << 5;
        /// Shorthand for the standard G profile (IMAFD).
        const G    = Self::I.bits()
            | Self::M.bits()
            | Self::A.bits()
            | Self::F.bits()
            | Self::D.bits();
    }
}
