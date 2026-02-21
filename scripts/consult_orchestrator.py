#!/usr/bin/env python3
"""
Main orchestrator for the /consult command system.
Launches headless consultants and manages report generation.
"""

import os
import sys
import subprocess
import tempfile
from pathlib import Path
from datetime import datetime
from typing import List, Dict

# Import utilities
sys.path.insert(0, str(Path(__file__).parent))
from consult_utils import (
    get_project_root,
    discover_context_files,
    find_current_context_source,
    get_consultant_config,
    check_consultant_available,
    generate_timestamp,
    create_notification,
    save_notification,
    format_notification_message
)

# Watchdog integration (optional — degrades gracefully)
try:
    from swarm_watchdog import (
        register_agent as _watchdog_register,
        PID_FILE as _WATCHDOG_PID,
        pid_alive as _watchdog_pid_alive,
    )
    _WATCHDOG_AVAILABLE = True
except ImportError:
    _WATCHDOG_AVAILABLE = False


def generate_consultant_prompt(
    project_root: Path,
    project_name: str,
    context_files: Dict[str, List[str]],
    timestamp: str,
    consultant_name: str,
    model: str
) -> str:
    """
    Generate the agent-agnostic consultant prompt.

    Args:
        project_root: Root directory of the project
        project_name: Name of the project
        context_files: Dict with 'primary' and 'supporting' file paths
        timestamp: ISO timestamp for report
        consultant_name: Name of consultant (for header)
        model: Model name (for header)

    Returns:
        Complete prompt string
    """
    # Primary context file
    primary_context = context_files["primary"][0] if context_files["primary"] else "No CURRENT_CONTEXT found"

    # Supporting files
    supporting_list = "\n".join([
        f"   - {p}"
        for p in context_files["supporting"]
    ]) if context_files["supporting"] else "   (None available)"

    # Output path
    output_path = project_root / "jobs" / project_name / f"consult_{consultant_name}_{model}_{timestamp}.md"

    prompt = f"""**SYSTEM ROLE:** You are an independent technical consultant conducting an unbiased evaluation of the system or subsystem provided to you. Your deliverable is a structured, actionable analysis report.

**CRITICAL - TEMPORAL FOCUS:** This project has 14+ months of development history with many prior analyses. DO NOT reference historical gap analyses, old execution plans, or previous consultant reports. Focus on the CURRENT plan and provide FRESH analysis.

**PRIMARY CONTEXT:** {primary_context}
This file contains the CURRENT execution plan, blueprint, or active task description. This is where we're going NOW. Start your analysis here and focus your recommendations on enhancing THIS specific effort.

**WORKING DIRECTORY:** {project_root}

**YOUR TASK:** Review and analyze the current plan. Identify:
- Gaps in the current approach
- Points of enhancement or refinement
- Opportunities for improvement
- Enhancement opportunities
Provide forward-looking, actionable insights aligned with where we're going NOW.

**SUPPORTING REFERENCE FILES** (if needed for project context):
{supporting_list}

**ANALYSIS MANDATE:**
Ground your evaluation in these key traits:
1. **RIGOR** - Assess internal coherence, correctness, logical soundness. Verify assumptions and evaluate implementation fidelity relative to stated goals.
2. **FUNCTIONALITY** - Determine whether the system performs its intended tasks reliably under expected constraints. Identify incomplete or unstable components.
3. **SCALABILITY** - Evaluate capacity to expand in load, complexity, or modularity without degrading performance or maintainability.
4. **CORRECTNESS** - Detect violations of design intent, logical or type errors, and inconsistent interface behaviors.
5. **UTILITY** - Identify practical effectiveness—how well it serves its use case and whether design choices enhance or obscure purpose.

**DELIVERABLE FORMAT:**
Produce a structured audit report at:
{output_path}

**Report Header** (identifies consultant and model):
```markdown
# CONSULTANT AUDIT

**Consultant**: {consultant_name.capitalize()}
**Model**: {model}
**Timestamp**: {datetime.now().isoformat()}
**Project**: {project_name}
```

## EXECUTIVE SUMMARY
Key findings at a glance (2-3 sentences).

## SYSTEM OVERVIEW
Your interpretation of the system's intended purpose and architecture based on PRIMARY CONTEXT.

## RIGOR & CORRECTNESS REVIEW
Direct findings on logical, architectural, or conceptual accuracy. Ground insights in observable artifacts.

## FUNCTIONAL & SCALABILITY ANALYSIS
Evidence-based discussion of strengths, bottlenecks, and limitations. Identify trade-offs.

## UTILITY & DESIGN INTEGRITY
Analysis of clarity, user affordances, and maintainability. Avoid generalities.

## PRIORITIZED RECOMMENDATIONS
Ordered set of targeted actions for improving rigor, performance, or clarity across layers.

## RAW OBSERVATIONS
(Optional) Detailed notes, edge cases, or technical minutiae.

**STYLE REQUIREMENTS:**
- Write as a professional technical consultant reporting to project leadership
- Use precise technical language, concise paragraphs, clear markdown headings
- Ground insights in observable artifacts or metrics where possible
- Identify and articulate trade-offs rather than simplistic judgments
- This evaluation applies to systems ranging from simple scripts to highly complex, rigorously verified mathematical constructs—maintain analytical rigor appropriate to scope.

**SESSION PROTOCOL:**
- This is a one-shot consultation. Your analysis should be as thorough and deep as the subject demands.
- Produce your report. Write artifacts as needed.
- Read whatever you need, analyze as deeply as necessary, then output your report and exit.

**AUTHORIZATION:** This session is pre-authorized for full-scope analysis. Execute with confidence.
"""
    return prompt


def invoke_consultant(
    consultant_name: str,
    prompt: str,
    output_path: Path,
    project_root: Path,
    debug: bool = False
) -> subprocess.Popen:
    """
    Invoke a consultant CLI tool in headless mode (non-blocking).

    Args:
        consultant_name: Name of consultant
        prompt: Complete prompt string
        output_path: Where to save the report
        project_root: Project root (sets CWD)
        debug: Enable debug output

    Returns:
        subprocess.Popen object (running in background)
    """
    config = get_consultant_config(consultant_name)
    invocation_style = config.get("invocation_style", "codex")

    # Write prompt to temp file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
        f.write(prompt)
        prompt_file = f.name

    # Create stderr log file for debugging
    stderr_log = output_path.parent / f"{output_path.stem}.stderr.log"

    # Build command based on CLI style
    if invocation_style == "codex":
        # codex exec -m MODEL --sandbox read-only - (reads from stdin, read-only sandbox)
        cmd = [
            config["cli_path"],
            "exec",
            "-m", config["model"],
            "--sandbox", "relaxed",  # Relaxed: consultant writes artifacts as needed
            "-"  # Read from stdin
        ]
        stdin_input = prompt.encode('utf-8')
        stdout_dest = open(str(output_path), 'w')
        stderr_dest = open(str(stderr_log), 'w')

    elif invocation_style == "aider":
        # qwen -m MODEL --yolo (reads from stdin when no positional prompt)
        cmd = [
            config["cli_path"],
            "-m", config["model"],
            "--yolo",  # Non-interactive approval (auto-approve all tools)
        ]
        stdin_input = prompt.encode('utf-8')
        stdout_dest = open(str(output_path), 'w')
        stderr_dest = open(str(stderr_log), 'w')

    elif invocation_style == "gemini":
        # gemini -m MODEL --yolo -p "" (non-interactive headless mode)
        # Gemini outputs JSON stats to stdout (settings: output.format=json)
        # and writes reports via its write_file tool — don't redirect stdout to report file
        stats_log = output_path.parent / f"{output_path.stem}.stats.json"
        cmd = [
            config["cli_path"],
            "-m", config["model"],
            "--yolo",  # Non-interactive approval
            "-p", "",  # Trigger non-interactive mode (actual prompt via stdin)
        ]
        stdin_input = prompt.encode('utf-8')
        stdout_dest = open(str(stats_log), 'w')  # JSON stats go here, not the report
        stderr_dest = open(str(stderr_log), 'w')

    elif invocation_style == "claude":
        # claude -p --model MODEL (reads from stdin in -p mode)
        # Full tool access — consultant writes artifacts as needed
        cmd = [
            config["cli_path"],
            "-p",  # Print mode: reads stdin, prints to stdout
            "--model", config["model"],
            "--dangerously-skip-permissions",  # Skip permission checks
        ]
        stdin_input = prompt.encode('utf-8')
        stdout_dest = open(str(output_path), 'w')
        stderr_dest = open(str(stderr_log), 'w')

    else:
        raise ValueError(f"Unknown invocation style: {invocation_style}")

    if debug:
        print(f"DEBUG: Launching {consultant_name}", file=sys.stderr)
        print(f"DEBUG: Style: {invocation_style}", file=sys.stderr)
        print(f"DEBUG: Command: {' '.join(cmd)}", file=sys.stderr)
        print(f"DEBUG: CWD: {project_root}", file=sys.stderr)
        print(f"DEBUG: Prompt file: {prompt_file}", file=sys.stderr)
        print(f"DEBUG: Output: {output_path}", file=sys.stderr)

    # Launch in background
    process = subprocess.Popen(
        cmd,
        cwd=str(project_root),
        stdin=subprocess.PIPE if stdin_input else None,
        stdout=stdout_dest,
        stderr=stderr_dest
    )

    # Write to stdin if needed
    if stdin_input:
        process.stdin.write(stdin_input)
        process.stdin.close()

    # Register with watchdog for auto-recovery
    if _WATCHDOG_AVAILABLE:
        try:
            _watchdog_register(
                pid=process.pid,
                agent_name=consultant_name,
                prompt_file=prompt_file,
                output_file=str(output_path),
                relaunch_cmd=cmd,
            )
        except Exception:
            pass  # Watchdog registration is best-effort

    if debug:
        print(f"DEBUG: {consultant_name} PID: {process.pid}", file=sys.stderr)

    return process


def main():
    """
    Main orchestrator entry point.

    Args (from command line):
        sys.argv[1]: Project name (optional, auto-detected from CWD)
        --agents: Comma-separated list of consultants (default: codex)
        --all: Launch all consultants
        --dry-run: Print context discovery without invoking consultants
        --debug, -v, --verbose: Enable debug output
    """
    # Parse arguments
    args = sys.argv[1:]
    consultants_to_launch = ["codex"]  # Default

    if "--all" in args:
        consultants_to_launch = ["codex", "qwen", "gemini", "claude"]
        args.remove("--all")
    elif "--agents" in args:
        idx = args.index("--agents")
        consultants_to_launch = args[idx + 1].split(",")
        args.remove("--agents")
        args.pop(idx)  # Remove the consultant list

    dry_run = "--dry-run" in args
    debug = any(flag in args for flag in ["--debug", "-v", "--verbose", "--force", "--refresh"])

    # Get project context (relative to current working directory)
    cwd = Path.cwd()
    try:
        project_root, project_name = get_project_root(cwd)
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    if debug:
        print(f"DEBUG: CWD: {cwd}", file=sys.stderr)
        print(f"DEBUG: Detected project root: {project_root}", file=sys.stderr)
        print(f"DEBUG: Detected project name: {project_name}", file=sys.stderr)

    # Create jobs directory (relative to detected project root)
    jobs_dir = project_root / "jobs" / project_name
    jobs_dir.mkdir(parents=True, exist_ok=True)

    # Check if CURRENT_CONTEXT exists
    context_files = sorted(jobs_dir.glob("CURRENT_CONTEXT_*.md"), key=lambda p: p.stat().st_mtime, reverse=True)

    need_refresh = False
    if context_files:
        latest_context = context_files[0]
        context_age_seconds = (datetime.now().timestamp() - latest_context.stat().st_mtime)
        context_age_minutes = int(context_age_seconds / 60)

        if debug:
            print(f"DEBUG: Found context: {latest_context}", file=sys.stderr)
            print(f"DEBUG: Context age: {context_age_seconds:.0f}s ({context_age_minutes}min)", file=sys.stderr)

        # Check if stale (> 1 hour)
        if context_age_seconds > 3600:
            need_refresh = True
            if debug:
                print(f"DEBUG: Context is stale (>1 hour)", file=sys.stderr)
    else:
        need_refresh = True
        if debug:
            print(f"DEBUG: No context file found", file=sys.stderr)

    # Force refresh if requested
    if "--force" in args or "--refresh" in args:
        need_refresh = True
        if debug:
            print(f"DEBUG: Force refresh requested", file=sys.stderr)

    # If context needs refresh, try to auto-discover and copy
    if need_refresh:
        timestamp = generate_timestamp()
        source = find_current_context_source(project_root)

        if source is not None:
            # Auto-copy the discovered context file
            dest = jobs_dir / f"CURRENT_CONTEXT_{timestamp}.md"
            import shutil
            shutil.copy2(source, dest)
            print(f"📋 Auto-discovered context: {source.name}")
            print(f"   Copied to: {dest.relative_to(project_root)}")
            print("")
        else:
            # Nothing found anywhere — bail out
            print("⚠️  Pre-flight check: No context source found.")
            print("")
            print("Searched (relative to project root):")
            print(f"  - {project_root}/*AUDIT*.md, *ANALYSIS*.md, *REPORT*.md")
            print("  - ~/.claude/plans/*.md (Claude Code plan mode)")
            print(f"  - {project_root}/docs/plans/*.md (project-specific)")
            print(f"  - {project_root}/state/blueprint.json")
            print(f"  - {project_root}/.claude/gap-analysis/*.md")
            print("")
            print("Create a context file (plan, audit, or analysis) and re-run /consult.")
            sys.exit(2)

    # Check consultant availability
    available = []
    unavailable = []
    for c in consultants_to_launch:
        if check_consultant_available(c):
            available.append(c)
        else:
            unavailable.append(c)

    if unavailable:
        print(f"Warning: Consultants not available: {', '.join(unavailable)}", file=sys.stderr)
        if not available:
            print(f"Error: No consultants available", file=sys.stderr)
            print(f"Available: {', '.join([c for c in ['codex', 'qwen', 'gemini', 'claude'] if check_consultant_available(c)])}", file=sys.stderr)
            sys.exit(1)

    consultants_to_launch = available

    # Discover context files (jobs_dir already created earlier)
    context_files = discover_context_files(project_root, project_name)

    if debug:
        print(f"DEBUG: Project: {project_name}", file=sys.stderr)
        print(f"DEBUG: Root: {project_root}", file=sys.stderr)
        print(f"DEBUG: Consultants: {', '.join(consultants_to_launch)}", file=sys.stderr)
        print(f"DEBUG: Context files:", file=sys.stderr)
        print(f"DEBUG:   PRIMARY: {context_files['primary']}", file=sys.stderr)
        print(f"DEBUG:   SUPPORTING: {context_files['supporting']}", file=sys.stderr)

    if dry_run:
        print(f"Project: {project_name}")
        print(f"Root: {project_root}")
        print(f"Consultants: {', '.join(consultants_to_launch)}")
        print(f"\nContext files discovered:")
        print(f"  PRIMARY: {context_files['primary']}")
        print(f"  SUPPORTING: {context_files['supporting']}")
        sys.exit(0)

    # Generate timestamp
    timestamp = generate_timestamp()

    # Auto-start watchdog daemon if not already running
    if _WATCHDOG_AVAILABLE:
        try:
            watchdog_running = (
                _WATCHDOG_PID.exists()
                and _watchdog_pid_alive(int(_WATCHDOG_PID.read_text().strip()))
            )
        except (ValueError, OSError):
            watchdog_running = False
        if not watchdog_running:
            watchdog_script = Path(__file__).parent / "swarm_watchdog.py"
            if watchdog_script.exists():
                subprocess.Popen([sys.executable, str(watchdog_script), "--daemon"])
                if debug:
                    print("DEBUG: Started watchdog daemon", file=sys.stderr)

    # Generate prompts and invoke consultants
    consultant_info = []
    expected_reports = []
    processes = []

    for consultant_name in consultants_to_launch:
        config = get_consultant_config(consultant_name)
        model = config["model"]

        # Generate prompt
        prompt = generate_consultant_prompt(
            project_root,
            project_name,
            context_files,
            timestamp,
            consultant_name,
            model
        )

        # Output path
        output_path = jobs_dir / f"consult_{consultant_name}_{model}_{timestamp}.md"

        # Invoke consultant (background)
        try:
            process = invoke_consultant(consultant_name, prompt, output_path, project_root, debug)
            processes.append(process)

            consultant_info.append({
                "name": consultant_name,
                "model": model
            })
            expected_reports.append(str(output_path.relative_to(project_root)))

        except Exception as e:
            print(f"Error launching {consultant_name}: {e}", file=sys.stderr)
            if debug:
                import traceback
                traceback.print_exc(file=sys.stderr)

    if not processes:
        print("Error: Failed to launch any consultants", file=sys.stderr)
        sys.exit(1)

    # Create notification
    receipt_time = datetime.now().isoformat()
    notification = create_notification(
        project_name,
        consultant_info,
        expected_reports,
        receipt_time
    )

    save_notification(project_root, notification, timestamp)

    # Print notification message for Claude
    message = format_notification_message(
        consultant_info,
        str(jobs_dir),
        receipt_time,
        expected_reports
    )
    print(message)

    # Don't wait for processes - they run in background
    sys.exit(0)


if __name__ == "__main__":
    main()
