# Property Tests

Property tests live in `robustone-core/tests/property_riscv.rs`.

They currently assert that:

- random RISC-V byte sequences do not panic the low-level decode API
- structured decode failures remain structured instead of collapsing to opaque strings
