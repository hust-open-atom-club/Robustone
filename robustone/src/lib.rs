//! Robustone meta-crate.

#[doc(inline)]
pub use robustone_core::*;

#[doc(inline)]
pub use robustone_arm as arm;
#[doc(inline)]
pub use robustone_loongarch as loongarch;
#[doc(inline)]
pub use robustone_riscv as riscv;
#[doc(inline)]
pub use robustone_x86 as x86;

pub fn dispatcher() -> ArchitectureDispatcher {
    let mut dispatcher = ArchitectureDispatcher::new();
    dispatcher.register(Box::new(riscv::RiscVHandler::new()));
    dispatcher.register(Box::new(arm::ArmHandler::new()));
    dispatcher.register(Box::new(x86::X86Handler::new()));
    dispatcher.register(Box::new(loongarch::LoongArchHandler::new()));
    dispatcher
}
