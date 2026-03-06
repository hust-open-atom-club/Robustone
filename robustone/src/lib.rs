//! Robustone meta-crate.

#[doc(inline)]
pub use robustone_core::*;

#[doc(inline)]
pub use robustone_riscv as riscv;

pub fn dispatcher() -> ArchitectureDispatcher {
    let mut dispatcher = ArchitectureDispatcher::new();
    dispatcher.register(Box::new(riscv::RiscVHandler::new()));
    dispatcher
}
