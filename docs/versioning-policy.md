# Versioning Policy

Robustone is still pre-1.0, but the repository is converging on these rules:

1. Document breaking API changes in `CHANGELOG.md`.
2. Keep the compatibility boundary explicit:
   - CLI compatibility
   - semantic compatibility
   - API compatibility
3. Treat the shared decode IR and JSON output as evolving interfaces until the project declares a stable `0.x` contract for them.
4. Once the low-level API and formatter profile boundaries stabilize, follow semantic versioning for public crates.
