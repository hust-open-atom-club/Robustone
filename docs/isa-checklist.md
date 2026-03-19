# New ISA Checklist

Use this checklist before adding a second architecture backend:

1. Define an `ArchitectureProfile` for the ISA, including bit width, endianness, and extension set.
2. Add shared decode coverage requirements:
   - low-level IR decode
   - Capstone-style text formatting
   - canonical formatting
   - structured JSON output
3. Add test assets:
   - golden fixtures
   - differential/oracle coverage
   - property tests
   - fuzz targets
4. Register benchmark cases for both IR decode throughput and compatibility text formatting.
5. Update `docs/support-matrix.md` and `docs/roadmap.md`.
