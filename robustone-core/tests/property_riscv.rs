use proptest::prelude::*;
use robustone_core::{ArchitectureDispatcher, DisasmError};
use std::panic::AssertUnwindSafe;

proptest! {
    #[test]
    fn test_decode_instruction_never_panics_on_even_length_inputs(
        bytes in prop::collection::vec(any::<u8>(), 2..8)
    ) {
        prop_assume!(bytes.len() % 2 == 0);

        let dispatcher = ArchitectureDispatcher::new();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            dispatcher.decode_instruction(&bytes, "riscv32", 0)
        }));

        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_short_inputs_report_structured_failures(
        bytes in prop::collection::vec(any::<u8>(), 1..4)
    ) {
        let dispatcher = ArchitectureDispatcher::new();
        let result = dispatcher.decode_instruction(&bytes, "riscv32", 0);

        if bytes.len() < 2 {
            prop_assert!(result.is_err());
            if let Err(error) = result {
                match error {
                    DisasmError::DecodeFailure { .. } => prop_assert!(true),
                    _ => prop_assert!(false),
                }
            }
        }
    }
}
