"""
Architecture configuration management for the test framework.
"""

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional


@dataclass
class ArchConfig:
    """Configuration for a specific architecture test."""
    name: str
    robustone_arch: str
    cstool_arch: str
    cases_file: Path
    robustone_flags: List[str] = field(default_factory=list)
    cstool_flags: List[str] = field(default_factory=list)
    description: str = ""
    category: str = "general"  # Can be used to group tests

    def __post_init__(self):
        """Validate configuration after initialization."""
        if not self.name:
            raise ValueError("Architecture name cannot be empty")
        if not self.robustone_arch:
            raise ValueError("Robustone architecture cannot be empty")
        if not self.cstool_arch:
            raise ValueError("cstool architecture cannot be empty")


def load_config(config_path: Path) -> ArchConfig:
    """Load architecture configuration from JSON file."""
    if not config_path.exists():
        raise FileNotFoundError(f"Configuration file not found: {config_path}")

    try:
        with config_path.open("r", encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON in {config_path}: {e}")

    # Convert cases_file to absolute path
    cases_file = Path(data.get("cases_file", "test_cases.txt"))
    if not cases_file.is_absolute():
        cases_file = config_path.parent / cases_file

    return ArchConfig(
        name=data.get("name", config_path.parent.name),
        robustone_arch=data.get("robustone_arch", data.get("name", config_path.parent.name)),
        cstool_arch=data.get("cstool_arch", data.get("name", config_path.parent.name)),
        cases_file=cases_file,
        robustone_flags=data.get("robustone_flags", []),
        cstool_flags=data.get("cstool_flags", []),
        description=data.get("description", ""),
        category=data.get("category", "general")
    )


def discover_arch_configs(test_root: Path) -> Dict[str, ArchConfig]:
    """Discover all architecture configurations in the test directory."""
    archs: Dict[str, ArchConfig] = {}
    arch_dir = test_root / "architectures"

    if not arch_dir.exists():
        return archs

    for sub in sorted(arch_dir.iterdir()):
        if not sub.is_dir():
            continue

        config_path = sub / "config.json"
        if not config_path.is_file():
            continue

        try:
            config = load_config(config_path)
            archs[config.name] = config
        except Exception as e:
            print(f"Warning: Failed to load config from {config_path}: {e}")
            continue

    return archs


def validate_config(config: ArchConfig) -> List[str]:
    """Validate an architecture configuration and return list of issues."""
    issues: List[str] = []

    # Check if test cases file exists
    if not config.cases_file.exists():
        issues.append(f"Test cases file not found: {config.cases_file}")

    # Validate flag formats
    for flag in config.robustone_flags:
        if not isinstance(flag, str) or not flag.strip():
            issues.append(f"Invalid robustone flag: {flag}")

    for flag in config.cstool_flags:
        if not isinstance(flag, str) or not flag.strip():
            issues.append(f"Invalid cstool flag: {flag}")

    return issues


def create_sample_config(arch_name: str, output_dir: Path) -> Path:
    """Create a sample configuration file for a new architecture."""
    config_data = {
        "name": arch_name,
        "robustone_arch": arch_name,
        "cstool_arch": arch_name,
        "cases_file": "test_cases.txt",
        "robustone_flags": [],
        "cstool_flags": [],
        "description": f"Test configuration for {arch_name} architecture",
        "category": "general"
    }

    output_dir.mkdir(parents=True, exist_ok=True)
    config_path = output_dir / "config.json"

    with config_path.open("w", encoding="utf-8") as f:
        json.dump(config_data, f, indent=2, ensure_ascii=False)

    # Create empty test cases file
    cases_path = output_dir / "test_cases.txt"
    cases_path.write_text(f"# {arch_name} test cases\n# Format: <hex_bytes> [| <expected_cstool_output>] [| <note>]\n\n")

    return config_path