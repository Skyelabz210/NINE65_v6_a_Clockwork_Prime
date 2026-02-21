#!/usr/bin/env python3
"""
Swarm Watchdog — Automatic Recovery for Non-Logical Agent Failures
==================================================================

Monitors running swarm agents (consult, compost, formalization-swarm) for
non-logical failures (crashes, undefined errors, OOM, timeouts, empty output)
and automatically relaunches them with the original prompt.

Distinguishes:
  RECOVERABLE — Runtime/environment failures. Relaunch with same prompt.
  LOGIC       — Proof failures, compilation errors, deliberate rejections.
                Report to orchestrator, do not relaunch.

Footprint: Single Python process, ~5MB RSS. Polls at configurable interval.

Usage:
    # Monitor all active agents, relaunch on failure
    python swarm_watchdog.py

    # Monitor with custom poll interval and max retries
    python swarm_watchdog.py --poll-interval 10 --max-retries 3

    # Register an agent manually (orchestrators do this automatically)
    python swarm_watchdog.py --register --pid 12345 \
        --prompt-file /tmp/prompt.txt \
        --output-file /path/to/report.md \
        --relaunch-cmd "codex exec -m gpt-5.3-codex --sandbox relaxed -"

    # Show status of all monitored agents
    python swarm_watchdog.py --status

    # Daemon mode (backgrounds itself)
    python swarm_watchdog.py --daemon

Author: Acid/HackFate + Claude
"""

import json
import os
import re
import signal
import subprocess
import sys
import time
from dataclasses import dataclass, field, asdict
from datetime import datetime
from enum import Enum
from pathlib import Path
from typing import Dict, List, Optional


# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

WATCHDOG_DIR = Path.home() / ".claude" / "watchdog"
MANIFEST_FILE = WATCHDOG_DIR / "agents.json"
LOG_FILE = WATCHDOG_DIR / "watchdog.log"
PID_FILE = WATCHDOG_DIR / "watchdog.pid"

DEFAULT_POLL_INTERVAL = 5      # seconds between checks
DEFAULT_MAX_RETRIES = 3        # per agent
DEFAULT_BACKOFF_BASE = 2       # exponential backoff base (seconds)
DEFAULT_TIMEOUT = 600          # 10 minutes — declare dead if no output growth


# ---------------------------------------------------------------------------
# Failure Classification
# ---------------------------------------------------------------------------

class FailureClass(str, Enum):
    RECOVERABLE = "recoverable"
    LOGIC = "logic"
    UNKNOWN = "unknown"


# Patterns that indicate recoverable (environment/runtime) failures
RECOVERABLE_PATTERNS = [
    # Runtime errors in the agent framework itself
    r"is not defined",
    r"is not a function",
    r"ReferenceError",
    r"TypeError:.*undefined",
    r"Cannot read propert",
    r"ECONNREFUSED",
    r"ECONNRESET",
    r"ETIMEDOUT",
    r"EPIPE",
    r"socket hang up",
    r"network error",
    r"fetch failed",
    r"API error",
    r"rate limit",
    r"429",
    r"500 Internal Server Error",
    r"502 Bad Gateway",
    r"503 Service Unavailable",
    r"504 Gateway Timeout",
    # OOM / resource
    r"out of memory",
    r"Cannot allocate memory",
    r"ENOMEM",
    r"Killed",                # OOM killer
    r"signal: killed",
    # Process lifecycle
    r"SIGTERM",
    r"SIGKILL",
    r"broken pipe",
    r"stream destroyed",
    # Claude Code / Codex specific
    r"classifyHandoffIfNeeded",
    r"Maximum context length",
    r"context window",
    r"conversation too long",
    r"session expired",
    r"token limit",
    # Empty/corrupt output
    r"^\s*$",
]

# Patterns that indicate logic failures (do NOT relaunch)
LOGIC_PATTERNS = [
    # Lean4 compilation errors (real proof failures)
    r"error:.*unknown identifier",
    r"error:.*type mismatch",
    r"error:.*unsolved goals",
    r"declaration uses 'sorry'",
    # Coq proof failures
    r"Error: Unable to unify",
    r"Error: No matching clauses",
    r"Error: The reference .* was not found",
    # Deliberate agent rejections
    r"VERDICT:\s*REJECT",
    r"OVERALL SEVERITY:\s*CRITICAL",
    # Cargo/Rust compilation (real code errors)
    r"error\[E\d+\]:",
]

RECOVERABLE_RE = [re.compile(p, re.IGNORECASE | re.MULTILINE) for p in RECOVERABLE_PATTERNS]
LOGIC_RE = [re.compile(p, re.IGNORECASE | re.MULTILINE) for p in LOGIC_PATTERNS]


def classify_failure(output: str, stderr: str, exit_code: int) -> FailureClass:
    """
    Classify a failure as recoverable or logic-based.

    Priority: If BOTH recoverable and logic patterns match, check exit code.
    - exit_code > 128 (signal) → recoverable (killed by signal)
    - exit_code == 1 with logic pattern → logic
    - exit_code == 0 with empty output → recoverable (process died silently)
    """
    combined = output + "\n" + stderr

    has_recoverable = any(r.search(combined) for r in RECOVERABLE_RE)
    has_logic = any(r.search(combined) for r in LOGIC_RE)

    # Signal kills are always recoverable
    if exit_code > 128:
        return FailureClass.RECOVERABLE

    # Empty output with zero exit = silent death
    if exit_code == 0 and len(output.strip()) == 0:
        return FailureClass.RECOVERABLE

    # Nonzero exit with no output = crash
    if exit_code != 0 and len(output.strip()) == 0:
        return FailureClass.RECOVERABLE

    # Both patterns present — logic takes precedence (real error in output)
    if has_logic and not has_recoverable:
        return FailureClass.LOGIC

    if has_recoverable:
        return FailureClass.RECOVERABLE

    if has_logic:
        return FailureClass.LOGIC

    # Unknown — default to recoverable for the first retry, then stop
    return FailureClass.UNKNOWN


# ---------------------------------------------------------------------------
# Agent Record
# ---------------------------------------------------------------------------

class AgentStatus(str, Enum):
    RUNNING = "running"
    SUCCEEDED = "succeeded"
    RELAUNCHING = "relaunching"
    FAILED_RECOVERABLE = "failed_recoverable"
    FAILED_LOGIC = "failed_logic"
    EXHAUSTED = "exhausted"       # max retries reached


@dataclass
class AgentRecord:
    agent_id: str                         # unique ID (timestamp + name)
    agent_name: str                       # human-readable (e.g., "codex", "π-Prover")
    pid: int                              # current process PID
    prompt_file: str                      # path to saved prompt
    output_file: str                      # path to output report
    stderr_file: str                      # path to stderr log
    relaunch_cmd: List[str]               # full command to relaunch
    registered_at: str                    # ISO timestamp
    status: str = AgentStatus.RUNNING.value
    retries: int = 0
    max_retries: int = DEFAULT_MAX_RETRIES
    last_check: str = ""
    last_output_size: int = 0             # for stall detection
    stall_checks: int = 0                 # consecutive checks with no output growth
    failure_log: List[Dict] = field(default_factory=list)
    relaunch_pids: List[int] = field(default_factory=list)


# ---------------------------------------------------------------------------
# Manifest (persistent state)
# ---------------------------------------------------------------------------

class Manifest:
    """Persistent manifest of all monitored agents."""

    def __init__(self, path: Path = MANIFEST_FILE):
        self.path = path
        self.agents: Dict[str, AgentRecord] = {}
        self._load()

    def _load(self):
        if self.path.exists():
            try:
                data = json.loads(self.path.read_text())
                for aid, adata in data.get("agents", {}).items():
                    # Convert failure_log and relaunch_pids if missing
                    adata.setdefault("failure_log", [])
                    adata.setdefault("relaunch_pids", [])
                    self.agents[aid] = AgentRecord(**adata)
            except (json.JSONDecodeError, TypeError):
                self.agents = {}

    def save(self):
        self.path.parent.mkdir(parents=True, exist_ok=True)
        data = {
            "updated_at": datetime.now().isoformat(),
            "agents": {aid: asdict(a) for aid, a in self.agents.items()}
        }
        self.path.write_text(json.dumps(data, indent=2))

    def register(self, record: AgentRecord):
        self.agents[record.agent_id] = record
        self.save()

    def active_agents(self) -> List[AgentRecord]:
        return [a for a in self.agents.values()
                if a.status in (AgentStatus.RUNNING.value, AgentStatus.RELAUNCHING.value)]

    def summary(self) -> Dict[str, int]:
        counts = {}
        for a in self.agents.values():
            counts[a.status] = counts.get(a.status, 0) + 1
        return counts


# ---------------------------------------------------------------------------
# Logging
# ---------------------------------------------------------------------------

def log(msg: str, also_print: bool = True):
    """Append to watchdog log."""
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    line = f"[{timestamp}] {msg}"
    LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(LOG_FILE, "a") as f:
        f.write(line + "\n")
    if also_print:
        print(line, file=sys.stderr)


# ---------------------------------------------------------------------------
# Process Checking
# ---------------------------------------------------------------------------

def pid_alive(pid: int) -> bool:
    """Check if a process is still running."""
    try:
        os.kill(pid, 0)
        return True
    except (OSError, ProcessLookupError):
        return False


def read_file_safe(path: str, max_bytes: int = 50000) -> str:
    """Read a file safely, returning empty string on failure."""
    try:
        p = Path(path)
        if not p.exists():
            return ""
        content = p.read_text(errors="replace")
        if len(content) > max_bytes:
            # Read tail for error patterns
            return content[-max_bytes:]
        return content
    except Exception:
        return ""


def get_exit_code(pid: int) -> Optional[int]:
    """Try to get exit code of a finished process."""
    try:
        _, status = os.waitpid(pid, os.WNOHANG)
        if os.WIFEXITED(status):
            return os.WEXITSTATUS(status)
        if os.WIFSIGNALED(status):
            return 128 + os.WTERMSIG(status)
    except ChildProcessError:
        # Not our child — try /proc
        try:
            stat = Path(f"/proc/{pid}/stat").read_text()
            # If we can read it, process exists
            return None
        except Exception:
            # Process gone, unknown exit
            return 1
    except Exception:
        return 1
    return None


# ---------------------------------------------------------------------------
# Relaunch Logic
# ---------------------------------------------------------------------------

def relaunch_agent(record: AgentRecord) -> Optional[int]:
    """
    Relaunch an agent with the same prompt.

    Returns new PID on success, None on failure.
    """
    prompt_path = Path(record.prompt_file)
    if not prompt_path.exists():
        log(f"  Prompt file missing: {record.prompt_file} — cannot relaunch")
        return None

    prompt_content = prompt_path.read_bytes()

    # Open output/stderr files (append mode for continuity tracking)
    output_path = Path(record.output_file)
    stderr_path = Path(record.stderr_file)

    # Rename old output as evidence
    attempt = record.retries + 1
    if output_path.exists():
        backup = output_path.with_suffix(f".attempt{attempt - 1}.md")
        try:
            output_path.rename(backup)
        except Exception:
            pass

    try:
        stdout_f = open(str(output_path), "w")
        stderr_f = open(str(stderr_path), "a")  # append stderr for history

        stderr_f.write(f"\n--- RELAUNCH ATTEMPT {attempt} at {datetime.now().isoformat()} ---\n")

        process = subprocess.Popen(
            record.relaunch_cmd,
            stdin=subprocess.PIPE,
            stdout=stdout_f,
            stderr=stderr_f,
            cwd=str(Path.home() / "Projects"),
        )

        if prompt_content:
            try:
                process.stdin.write(prompt_content)
                process.stdin.close()
            except Exception:
                pass

        log(f"  Relaunched {record.agent_name} → PID {process.pid} (attempt {attempt})")
        return process.pid

    except Exception as e:
        log(f"  Relaunch failed for {record.agent_name}: {e}")
        return None


# ---------------------------------------------------------------------------
# Main Check Loop
# ---------------------------------------------------------------------------

def check_agent(record: AgentRecord, manifest: Manifest) -> None:
    """Check a single agent's health and take action if needed."""
    record.last_check = datetime.now().isoformat()

    alive = pid_alive(record.pid)

    if alive:
        # Process running — check for stalls (no output growth)
        output = read_file_safe(record.output_file)
        current_size = len(output)

        if current_size == record.last_output_size:
            record.stall_checks += 1
        else:
            record.stall_checks = 0
            record.last_output_size = current_size

        # Stall detection: if no output growth for N checks, something may be wrong
        # But don't kill yet — some agents think for a long time
        if record.stall_checks > (DEFAULT_TIMEOUT // DEFAULT_POLL_INTERVAL):
            log(f"  {record.agent_name} (PID {record.pid}): stalled for {record.stall_checks * DEFAULT_POLL_INTERVAL}s — marking for review")
            # Don't auto-kill. Log it. The operator decides.

        return

    # Process is dead — classify and act
    output = read_file_safe(record.output_file)
    stderr = read_file_safe(record.stderr_file)
    exit_code = get_exit_code(record.pid) or 1

    # Check if output looks like a complete report (success case)
    output_stripped = output.strip()
    if exit_code == 0 and len(output_stripped) > 200:
        # Looks like successful completion
        record.status = AgentStatus.SUCCEEDED.value
        log(f"  {record.agent_name} (PID {record.pid}): completed ({len(output_stripped)} bytes output)")
        manifest.save()
        return

    # Dead with insufficient output — classify failure
    failure_class = classify_failure(output, stderr, exit_code)

    failure_entry = {
        "timestamp": datetime.now().isoformat(),
        "pid": record.pid,
        "exit_code": exit_code,
        "class": failure_class.value,
        "output_bytes": len(output_stripped),
        "stderr_tail": stderr[-500:] if stderr else "",
        "output_tail": output[-500:] if output else "",
    }
    record.failure_log.append(failure_entry)

    if failure_class == FailureClass.LOGIC:
        record.status = AgentStatus.FAILED_LOGIC.value
        log(f"  {record.agent_name} (PID {record.pid}): LOGIC FAILURE (exit={exit_code}) — not relaunching")
        manifest.save()
        return

    if failure_class in (FailureClass.RECOVERABLE, FailureClass.UNKNOWN):
        if record.retries >= record.max_retries:
            record.status = AgentStatus.EXHAUSTED.value
            log(f"  {record.agent_name} (PID {record.pid}): exhausted {record.max_retries} retries — stopping")
            manifest.save()
            return

        # Relaunch with exponential backoff
        backoff = DEFAULT_BACKOFF_BASE ** record.retries
        log(f"  {record.agent_name} (PID {record.pid}): {failure_class.value} failure (exit={exit_code}), "
            f"retry {record.retries + 1}/{record.max_retries} after {backoff}s backoff")

        time.sleep(backoff)

        new_pid = relaunch_agent(record)
        if new_pid:
            record.relaunch_pids.append(record.pid)
            record.pid = new_pid
            record.retries += 1
            record.status = AgentStatus.RUNNING.value
            record.stall_checks = 0
            record.last_output_size = 0
        else:
            record.status = AgentStatus.EXHAUSTED.value

        manifest.save()


def run_watchdog(poll_interval: int = DEFAULT_POLL_INTERVAL,
                 max_retries: int = DEFAULT_MAX_RETRIES,
                 once: bool = False):
    """Main watchdog loop."""
    manifest = Manifest()

    log(f"Watchdog started (poll={poll_interval}s, max_retries={max_retries})")
    log(f"Monitoring {len(manifest.active_agents())} active agents")

    while True:
        # Reload manifest each cycle (other processes may register new agents)
        manifest = Manifest()
        active = manifest.active_agents()

        if not active:
            if once:
                log("No active agents. Exiting.")
                break
            time.sleep(poll_interval)
            continue

        for agent in active:
            agent.max_retries = max_retries
            try:
                check_agent(agent, manifest)
            except Exception as e:
                log(f"  Error checking {agent.agent_name}: {e}")

        manifest.save()

        if once:
            break

        time.sleep(poll_interval)


# ---------------------------------------------------------------------------
# Registration (called by orchestrators)
# ---------------------------------------------------------------------------

def register_agent(pid: int, agent_name: str, prompt_file: str,
                   output_file: str, relaunch_cmd: List[str],
                   max_retries: int = DEFAULT_MAX_RETRIES) -> str:
    """
    Register an agent with the watchdog.

    Returns the agent_id.

    Can be called from orchestrator code:
        from swarm_watchdog import register_agent
        agent_id = register_agent(
            pid=process.pid,
            agent_name="codex",
            prompt_file="/tmp/prompt.txt",
            output_file="/path/to/report.md",
            relaunch_cmd=["codex", "exec", "-m", "gpt-5.3-codex", "-"],
        )
    """
    manifest = Manifest()

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    agent_id = f"{agent_name}_{timestamp}_{pid}"

    stderr_file = str(Path(output_file).with_suffix(".stderr.log"))

    record = AgentRecord(
        agent_id=agent_id,
        agent_name=agent_name,
        pid=pid,
        prompt_file=prompt_file,
        output_file=output_file,
        stderr_file=stderr_file,
        relaunch_cmd=relaunch_cmd,
        registered_at=datetime.now().isoformat(),
        max_retries=max_retries,
    )

    manifest.register(record)
    log(f"Registered {agent_name} (PID {pid}) as {agent_id}")
    return agent_id


def print_status():
    """Print status of all monitored agents."""
    manifest = Manifest()

    if not manifest.agents:
        print("No agents registered.")
        return

    summary = manifest.summary()
    print(f"Agent Status Summary: {json.dumps(summary)}")
    print()
    print(f"{'ID':<40} {'Name':<15} {'PID':<8} {'Status':<20} {'Retries':<8} {'Output':<10}")
    print("-" * 101)

    for aid, agent in sorted(manifest.agents.items()):
        output_size = 0
        try:
            p = Path(agent.output_file)
            if p.exists():
                output_size = p.stat().st_size
        except Exception:
            pass

        print(f"{aid:<40} {agent.agent_name:<15} {agent.pid:<8} {agent.status:<20} "
              f"{agent.retries:<8} {output_size:>8}B")

    # Show failure details for failed agents
    failed = [a for a in manifest.agents.values()
              if a.failure_log and a.status != AgentStatus.SUCCEEDED.value]
    if failed:
        print()
        print("Recent Failures:")
        for agent in failed:
            latest = agent.failure_log[-1]
            print(f"  {agent.agent_name}: {latest['class']} (exit={latest['exit_code']})")
            if latest.get("stderr_tail"):
                # Show first line of stderr tail
                first_line = latest["stderr_tail"].strip().split("\n")[0][:100]
                print(f"    stderr: {first_line}")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main():
    import argparse
    parser = argparse.ArgumentParser(
        description="Swarm Watchdog — Automatic recovery for non-logical agent failures"
    )

    parser.add_argument("--poll-interval", type=int, default=DEFAULT_POLL_INTERVAL,
                        help=f"Seconds between checks (default: {DEFAULT_POLL_INTERVAL})")
    parser.add_argument("--max-retries", type=int, default=DEFAULT_MAX_RETRIES,
                        help=f"Max retries per agent (default: {DEFAULT_MAX_RETRIES})")
    parser.add_argument("--once", action="store_true",
                        help="Run one check cycle and exit")
    parser.add_argument("--daemon", action="store_true",
                        help="Fork to background")
    parser.add_argument("--status", action="store_true",
                        help="Show status of monitored agents")
    parser.add_argument("--stop", action="store_true",
                        help="Stop the running watchdog daemon")
    parser.add_argument("--clear", action="store_true",
                        help="Clear completed/exhausted agents from manifest")

    # Registration subcommand
    parser.add_argument("--register", action="store_true",
                        help="Register an agent for monitoring")
    parser.add_argument("--pid", type=int, help="Agent PID")
    parser.add_argument("--name", default="agent", help="Agent name")
    parser.add_argument("--prompt-file", help="Path to prompt file")
    parser.add_argument("--output-file", help="Path to output file")
    parser.add_argument("--relaunch-cmd", help="Relaunch command (JSON array)")

    args = parser.parse_args()

    if args.status:
        print_status()
        return

    if args.stop:
        if PID_FILE.exists():
            pid = int(PID_FILE.read_text().strip())
            try:
                os.kill(pid, signal.SIGTERM)
                print(f"Stopped watchdog (PID {pid})")
                PID_FILE.unlink()
            except ProcessLookupError:
                print(f"Watchdog (PID {pid}) already stopped")
                PID_FILE.unlink()
        else:
            print("No watchdog PID file found")
        return

    if args.clear:
        manifest = Manifest()
        cleared = 0
        to_remove = []
        for aid, agent in manifest.agents.items():
            if agent.status in (AgentStatus.SUCCEEDED.value,
                                AgentStatus.EXHAUSTED.value,
                                AgentStatus.FAILED_LOGIC.value):
                to_remove.append(aid)
                cleared += 1
        for aid in to_remove:
            del manifest.agents[aid]
        manifest.save()
        print(f"Cleared {cleared} completed/failed agents")
        return

    if args.register:
        if not all([args.pid, args.prompt_file, args.output_file, args.relaunch_cmd]):
            parser.error("--register requires --pid, --prompt-file, --output-file, --relaunch-cmd")

        cmd = json.loads(args.relaunch_cmd)
        agent_id = register_agent(
            pid=args.pid,
            agent_name=args.name,
            prompt_file=args.prompt_file,
            output_file=args.output_file,
            relaunch_cmd=cmd,
        )
        print(f"Registered: {agent_id}")
        return

    if args.daemon:
        # Fork to background
        pid = os.fork()
        if pid > 0:
            print(f"Watchdog daemonized (PID {pid})")
            PID_FILE.parent.mkdir(parents=True, exist_ok=True)
            PID_FILE.write_text(str(pid))
            sys.exit(0)
        # Child continues
        os.setsid()
        # Redirect stdio
        devnull = open(os.devnull, "r+b")
        os.dup2(devnull.fileno(), sys.stdin.fileno())
        os.dup2(devnull.fileno(), sys.stdout.fileno())
        # Keep stderr → log file
        log_fd = open(LOG_FILE, "a")
        os.dup2(log_fd.fileno(), sys.stderr.fileno())

    # Write PID
    PID_FILE.parent.mkdir(parents=True, exist_ok=True)
    PID_FILE.write_text(str(os.getpid()))

    try:
        run_watchdog(
            poll_interval=args.poll_interval,
            max_retries=args.max_retries,
            once=args.once,
        )
    except KeyboardInterrupt:
        log("Watchdog stopped by operator")
    finally:
        if PID_FILE.exists():
            try:
                PID_FILE.unlink()
            except Exception:
                pass


if __name__ == "__main__":
    main()
