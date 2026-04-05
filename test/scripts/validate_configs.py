#!/usr/bin/env python3
"""
Validate all architecture configurations.
"""

import os
import sys
from pathlib import Path

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(PROJECT_ROOT)

# pylint: disable=wrong-import-position
try:
    from test.core.arch_config import discover_arch_configs, validate_config
except ImportError:  # pragma: no cover - script-mode fallback
    from core.arch_config import discover_arch_configs, validate_config

# pylint: enable=wrong-import-position

# Add the parent directory to Python path so we can import the core modules
sys.path.insert(0, str(Path(__file__).parent.parent))


def main():
    """Validate all discovered configurations."""
    test_root = Path(__file__).parent.parent
    archs = discover_arch_configs(test_root)

    if not archs:
        print("No architecture configurations found.")
        return 0

    print(f"Validating {len(archs)} architecture configurations...")
    print("=" * 60)

    all_valid = True
    for name, config in sorted(archs.items()):
        issues = validate_config(config)
        if issues:
            print(f"✗ {name}")
            for issue in issues:
                print(f"  - {issue}")
            all_valid = False
        else:
            print(f"✓ {name}")

    print("=" * 60)
    if all_valid:
        print("🎉 All configurations are valid!")
        return 0

    print("❌ Some configurations have issues.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
