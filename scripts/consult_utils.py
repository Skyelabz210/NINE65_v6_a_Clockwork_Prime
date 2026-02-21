#!/usr/bin/env python3
"""
Utility functions for the /consult command system.
Handles context discovery, file operations, and consultant configuration.
"""

import os
import json
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from datetime import datetime


def get_project_root(cwd: Path) -> Tuple[Path, str]:
    """
    Determine project root and project name from current working directory.

    Detects project root by looking for project markers (Cargo.toml, package.json,
    pyproject.toml, setup.py, .git, README.md) walking up from CWD.

    Args:
        cwd: Current working directory

    Returns:
        Tuple of (project_root_path, project_name)

    Raises:
        ValueError: If not in a valid project under ~/Projects
    """
    projects_dir = Path.home() / "Projects"

    if not str(cwd).startswith(str(projects_dir)):
        raise ValueError(
            f"Must be in a project under {projects_dir}\n"
            f"Current directory: {cwd}"
        )

    # Walk up from CWD looking for project markers
    current = cwd
    project_markers = [
        "Cargo.toml", "package.json", "pyproject.toml", "setup.py",
        ".git", "README.md"
    ]

    while str(current).startswith(str(projects_dir)) and current != projects_dir:
        # Check for project markers
        for marker in project_markers:
            if (current / marker).exists():
                project_name = current.name
                return current, project_name
        current = current.parent

    # Fallback: use first subdirectory of ~/Projects
    relative = cwd.relative_to(projects_dir)
    parts = relative.parts

    if not parts:
        raise ValueError(f"Cannot run /consult in {projects_dir} root")

    project_name = parts[0]
    project_root = projects_dir / project_name

    return project_root, project_name


def find_current_context_source(project_root: Path) -> Optional[Path]:
    """
    Find the most current execution context to copy for consultants.

    Searches multiple locations relative to the project root, returning
    the most recently modified match. All paths are relative to
    project_root so this works from any working directory.

    Priority:
    1. Audit/analysis reports in project root (*AUDIT*.md, *ANALYSIS*.md, *REPORT*.md)
    2. Most recent plan in ~/.claude/plans/ (Claude Code plan mode)
    3. Most recent plan in docs/plans/ (project-specific)
    4. state/blueprint.json
    5. Latest gap analysis in .claude/gap-analysis/
    6. Any COMPREHENSIVE_*.md or SYSTEM_*.md in project root

    Args:
        project_root: Root directory of the project

    Returns:
        Path to current context file, or None if none found
    """
    candidates: list[tuple[float, Path]] = []

    # 1. Check project root for audit/analysis/report files
    audit_patterns = [
        "*AUDIT*.md", "*ANALYSIS*.md", "*REPORT*.md",
        "COMPREHENSIVE_*.md", "SYSTEM_*.md",
    ]
    for pattern in audit_patterns:
        for f in project_root.glob(pattern):
            if f.is_file():
                candidates.append((f.stat().st_mtime, f))

    # 2. Check Claude Code plan directory
    claude_plans_dir = Path.home() / ".claude" / "plans"
    if claude_plans_dir.exists():
        for f in claude_plans_dir.glob("*.md"):
            if f.is_file():
                candidates.append((f.stat().st_mtime, f))

    # 3. Check project docs/plans/
    plans_dir = project_root / "docs" / "plans"
    if plans_dir.exists():
        for f in plans_dir.glob("*.md"):
            if f.is_file():
                candidates.append((f.stat().st_mtime, f))

    # 4. Check state/blueprint.json
    blueprint = project_root / "state" / "blueprint.json"
    if blueprint.exists():
        candidates.append((blueprint.stat().st_mtime, blueprint))

    # 5. Check gap analysis directory
    gap_dir = project_root / ".claude" / "gap-analysis"
    if gap_dir.exists():
        for f in gap_dir.glob("*.md"):
            if f.is_file():
                candidates.append((f.stat().st_mtime, f))

    # Return most recently modified candidate
    if candidates:
        candidates.sort(key=lambda t: t[0], reverse=True)
        return candidates[0][1]

    return None


def discover_context_files(project_root: Path, project_name: str) -> Dict[str, List[str]]:
    """
    Discover context files for consultants (minimal, temporal focus).

    Returns file PATHS, not contents.

    Args:
        project_root: Root directory of the project
        project_name: Name of the project

    Returns:
        Dict with 'primary' and 'supporting' keys containing file paths
    """
    context = {
        "primary": [],
        "supporting": []
    }

    # Primary context: CURRENT_CONTEXT file (will be created by Claude)
    jobs_dir = project_root / "jobs" / project_name
    # Find most recent CURRENT_CONTEXT file
    if jobs_dir.exists():
        current_contexts = sorted(
            jobs_dir.glob("CURRENT_CONTEXT_*.md"),
            key=lambda p: p.stat().st_mtime,
            reverse=True
        )
        if current_contexts:
            context["primary"].append(str(current_contexts[0].absolute()))

    # Supporting files (minimal)
    readme = project_root / "README.md"
    if readme.exists():
        context["supporting"].append(str(readme.absolute()))

    claude_md = project_root / "CLAUDE.md"
    if claude_md.exists():
        context["supporting"].append(str(claude_md.absolute()))

    return context


def get_consultant_config(consultant_name: str) -> Dict[str, str]:
    """
    Get configuration for a specific consultant CLI tool.

    Args:
        consultant_name: Name of consultant (codex, qwen, gemini, claude)

    Returns:
        Dict with 'cli_path', 'model', 'invocation_style'

        invocation_style can be:
        - 'codex': codex exec -m MODEL - (reads from stdin, outputs to stdout)
        - 'aider': <cli> --model MODEL --yes --file <output> <prompt_file>
        - 'claude': claude --model MODEL --headless --prompt-file <prompt>
    """
    configs = {
        "codex": {
            "cli_path": "/usr/local/bin/codex",
            "model": "gpt-5.3-codex",  # Highest frontier model + xhigh reasoning
            "invocation_style": "codex"  # codex exec -m MODEL -
        },
        "qwen": {
            "cli_path": "/home/acid/.npm-global/bin/qwen",
            "model": "coder-model",  # From ~/.qwen/settings.json (highest coder)
            "invocation_style": "aider"  # Similar to aider CLI
        },
        "gemini": {
            "cli_path": "/home/acid/.npm-global/bin/gemini",
            "model": "gemini-2.5-pro",  # Upgraded per EO-002 A12 (fallback: gemini-2.0-flash if quota unavailable)
            "invocation_style": "gemini"  # gemini -p <prompt> --yolo (non-interactive)
        },
        "claude": {
            "cli_path": "claude",  # In PATH
            "model": "opus",  # Highest Claude model
            "invocation_style": "claude"  # claude -p --model MODEL
        }
    }

    if consultant_name not in configs:
        raise ValueError(f"Unknown consultant: {consultant_name}")

    return configs[consultant_name]


def check_consultant_available(consultant_name: str) -> bool:
    """
    Check if a consultant CLI tool is available.

    Args:
        consultant_name: Name of consultant

    Returns:
        True if CLI tool exists, False otherwise
    """
    config = get_consultant_config(consultant_name)
    cli_path = Path(config["cli_path"])

    # If it's just a command name (like 'claude'), check PATH
    if not str(cli_path).startswith("/"):
        import shutil
        return shutil.which(config["cli_path"]) is not None

    return cli_path.exists()


def generate_timestamp() -> str:
    """
    Generate ISO 8601 timestamp for filenames.

    Returns:
        Timestamp string in format: YYYYMMDD_HHMMSS
    """
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def create_notification(
    project: str,
    consultants: List[Dict[str, str]],
    expected_reports: List[str],
    receipt_time: str
) -> Dict:
    """
    Create notification JSON for main Claude session.

    Args:
        project: Project name
        consultants: List of dicts with 'name' and 'model' keys
        expected_reports: List of expected report paths (relative to project root)
        receipt_time: ISO 8601 timestamp

    Returns:
        Notification dict
    """
    return {
        "type": "consult_launched",
        "consultants": consultants,
        "project": project,
        "receipt_time": receipt_time,
        "expected_reports": expected_reports
    }


def save_notification(project_root: Path, notification: Dict, timestamp: str):
    """
    Save notification JSON to .claude/notifications/

    Args:
        project_root: Root directory of the project
        notification: Notification dict
        timestamp: Timestamp string for filename
    """
    notif_dir = project_root / ".claude" / "notifications"
    notif_dir.mkdir(parents=True, exist_ok=True)

    notif_file = notif_dir / f"consult_{timestamp}.json"
    notif_file.write_text(json.dumps(notification, indent=2))


def format_notification_message(
    consultants: List[Dict[str, str]],
    jobs_dir: str,
    receipt_time: str,
    expected_reports: List[str]
) -> str:
    """
    Format the notification message printed to stdout for Claude.

    Args:
        consultants: List of dicts with 'name' and 'model'
        jobs_dir: Path to jobs directory
        receipt_time: ISO timestamp
        expected_reports: List of report filenames

    Returns:
        Formatted notification string
    """
    consultant_list = "\n".join([
        f"   - {c['name'].capitalize()} ({c['model']})"
        for c in consultants
    ])

    report_list = "\n".join([
        f"   - {Path(r).name}"
        for r in expected_reports
    ])

    return f"""
🔔 External consultant(s) launched:
{consultant_list}

📂 Reports will be available at:
   {jobs_dir}

⏱️  Receipt time: {receipt_time}

   Expected reports:
{report_list}

   (Silent wait protocol: 5 minutes, 4 checks)

Note: This is a consultation for assessment, not directive execution.
""".strip()
