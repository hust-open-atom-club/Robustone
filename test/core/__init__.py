"""
Robustone Test Framework

A modular testing framework for comparing Robustone output with cstool reference.
"""

from .test_runner import TestRunner
from .arch_config import ArchConfig, load_config
from .comparator import OutputComparator
from .utils import run_command, normalize_output

__version__ = "1.0.0"
__all__ = ["TestRunner", "ArchConfig", "load_config", "OutputComparator", "run_command", "normalize_output"]