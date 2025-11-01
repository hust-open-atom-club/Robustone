"""
Utility functions for the test framework.
"""

import subprocess
import re
from pathlib import Path
from typing import List, Tuple, Optional


def run_command(cmd: List[str], timeout: Optional[int] = 60) -> Tuple[int, str, str]:
    """
    Run a command and return (returncode, stdout, stderr).

    Args:
        cmd: Command to execute as a list of strings
        timeout: Optional timeout in seconds (default: 60)

    Returns:
        Tuple of (returncode, stdout, stderr)
    """
    try:
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=timeout,
            check=False,
        )
        return result.returncode, result.stdout.strip(), result.stderr.strip()
    except subprocess.TimeoutExpired:
        return 124, "", f"Command timed out after {timeout} seconds: {' '.join(cmd)}"
    except Exception as exc:
        return 1, "", f"Failed to run command: {exc}"


def normalize_output(output: str) -> str:
    """
    Normalize output string for comparison by collapsing whitespace.

    Args:
        output: Raw output string

    Returns:
        Normalized output string with consistent whitespace
    """
    return " ".join(output.split())


def find_repo_root(start_path: Optional[Path] = None) -> Path:
    """
    Find the repository root directory by looking for .git directory.

    Args:
        start_path: Starting path for search (default: current directory)

    Returns:
        Path to repository root directory

    Raises:
        FileNotFoundError: If git repository not found
    """
    if start_path is None:
        start_path = Path.cwd()

    current = start_path.resolve()
    while current != current.parent:
        if (current / ".git").exists():
            return current
        current = current.parent

    raise FileNotFoundError("Git repository not found")


def ensure_binary(
    binary_path: Path, build_commands: List[str], verbose: bool = False
) -> Path:
    """
    Ensure a binary exists, building it if necessary.

    Args:
        binary_path: Path to the binary
        build_commands: List of shell commands to build the binary
        verbose: Whether to print build output

    Returns:
        Path to the binary

    Raises:
        RuntimeError: If binary cannot be built
    """
    if binary_path.exists() and binary_path.is_file():
        return binary_path

    if verbose:
        print(f"Building binary: {binary_path}")

    for cmd in build_commands:
        result = subprocess.run(
            cmd, shell=True, capture_output=True, text=True, check=True
        )
        if result.returncode != 0:
            raise RuntimeError(f"Build command failed: {cmd}\nstderr: {result.stderr}")

    if not binary_path.exists():
        raise RuntimeError(f"Binary not found after build: {binary_path}")

    return binary_path


def parse_test_case(line: str) -> Tuple[str, str, str]:
    """
    Parse a test case line into (hex_input, expected_output, note).

    Args:
        line: Test case line from test file

    Returns:
        Tuple of (hex_input, expected_output, note)
    """
    line = line.strip()
    if not line or line.startswith("#"):
        return "", "", ""

    # Split on first #
    if "#" not in line:
        return line, "", ""

    hex_input, right = [seg.strip() for seg in line.split("#", 1)]

    if "|" in right:
        parts = right.split("|", 1)
        if len(parts) == 2:
            expected_output = parts[0].strip()
            note = parts[1].strip()
        else:
            expected_output = right.strip()
            note = ""
    else:
        expected_output = right.strip()
        note = ""

    return hex_input, expected_output, note


def format_test_result(
    hex_input: str, expected: str, actual: str, note: str = ""
) -> str:
    """
    Format a test result for display.

    Args:
        hex_input: Hexadecimal input instruction
        expected: Expected output
        actual: Actual output
        note: Optional note

    Returns:
        Formatted test result string
    """
    result = f"Instruction: {hex_input}"
    if expected:
        result += f"\n  Expected: {expected}"
    result += f"\n  Actual:   {actual}"
    if note:
        result += f"\n  Note:     {note}"
    return result


def truncate_string(s: str, max_length: int = 100) -> str:
    """
    Truncate a string to a maximum length with ellipsis.

    Args:
        s: String to truncate
        max_length: Maximum length

    Returns:
        Truncated string
    """
    if len(s) <= max_length:
        return s
    return s[: max_length - 3] + "..."


def safe_filename(name: str) -> str:
    """
    Convert a string to a safe filename.

    Args:
        name: Input string

    Returns:
        Safe filename string
    """

    # Remove or replace unsafe characters
    safe = re.sub(r'[<>:"/\\|?*]', "_", name)
    # Remove leading/trailing spaces and dots
    safe = safe.strip(". ")
    # Ensure it's not empty
    return safe or "unnamed"
