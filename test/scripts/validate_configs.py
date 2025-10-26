#!/usr/bin/env python3
"""
Validate all architecture configurations.
"""

import sys
from pathlib import Path

# Add the parent directory to Python path so we can import the core modules
sys.path.insert(0, str(Path(__file__).parent.parent))

from core.arch_config import discover_arch_configs, validate_config


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
    else:
        print("❌ Some configurations have issues.")
        return 1


if __name__ == "__main__":
    sys.exit(main())