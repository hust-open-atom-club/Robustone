# New ISA Checklist

Use this checklist before adding a second architecture backend:

1. Define an `ArchitectureProfile` for the ISA, including bit width, endianness, and extension set.
2. Define opcode and field tables for the ISA, including any extension / mode gating rules that affect decode legality.
3. Implement decoder-to-IR lowering against the shared `DecodedInstruction` surface instead of a string-first backend payload.
4. Add formatter coverage requirements:
   - Capstone-style text formatting
   - canonical text formatting
   - verbose debug formatting
   - structured JSON output
5. Add test assets:
   - golden fixtures
   - differential/oracle coverage
   - property tests
   - fuzz targets
6. Register benchmark cases for:
   - IR decode throughput
   - detail overhead
   - CLI end-to-end cost
7. Update `README.md`, `README_CN.md`, `docs/support-matrix.md`, and `tests/differential/known-differences.toml` as needed for the new ISA surface.
