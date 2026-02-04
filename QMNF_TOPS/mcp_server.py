#!/usr/bin/env python3
"""
QMNF_TOPS MCP Server
====================
Exposes the QMNF arithmetic modules and 64+ Impossible Problems via MCP protocol.

EXTENSIBLE DESIGN:
- Add new modules: Edit modules.json
- Add new problems: Edit problems.json
- Add new files: Just drop .rs files, add entry to modules.json

Usage:
    python mcp_server.py

Tools provided:
    - list_modules: List all arithmetic modules
    - get_module: Get source code of a specific module
    - list_problems: List impossible problems by domain
    - get_problem: Get details of a specific problem
    - search_modules: Search modules by keyword
    - search_problems: Search problems by keyword
    - get_fhe_versions: List the 4 FHE versions
    - get_master_reference: Full 64 Impossible Problems document
    - add_module: Add a new module to the registry
    - add_problem: Add a new problem to the registry
"""

import json
import sys
import os
from pathlib import Path
from typing import Any, Optional

# Base path for QMNF_TOPS
QMNF_TOPS_PATH = Path(__file__).parent.resolve()
MODULES_JSON = QMNF_TOPS_PATH / "modules.json"
PROBLEMS_JSON = QMNF_TOPS_PATH / "problems.json"


def load_modules() -> dict:
    """Load modules from JSON file."""
    if MODULES_JSON.exists():
        with open(MODULES_JSON) as f:
            data = json.load(f)
            return data.get("modules", {})
    return {}


def load_problems() -> dict:
    """Load problems from JSON file."""
    if PROBLEMS_JSON.exists():
        with open(PROBLEMS_JSON) as f:
            data = json.load(f)
            return data.get("domains", {})
    return {}


def save_modules(modules: dict):
    """Save modules to JSON file."""
    data = {"_comment": "QMNF_TOPS Module Registry - Add new modules here", "modules": modules}
    with open(MODULES_JSON, 'w') as f:
        json.dump(data, f, indent=2)


def save_problems(domains: dict):
    """Save problems to JSON file."""
    data = {"_comment": "64+ Impossible Problems Registry - Add new problems here", "domains": domains}
    with open(PROBLEMS_JSON, 'w') as f:
        json.dump(data, f, indent=2)


# FHE Versions (static for now)
FHE_VERSIONS = {
    "01_original": {
        "name": "NINE65 Original",
        "file": "FHE_VERSIONS/01_NINE65_original.zip",
        "size": "116KB",
        "rank": "Initial implementation"
    },
    "02_stable": {
        "name": "NINE65 Stable",
        "file": "FHE_VERSIONS/02_NINE65_stable.tar.gz",
        "size": "134KB",
        "rank": "12th globally"
    },
    "03_mana_boosted": {
        "name": "MANA Boosted",
        "file": "FHE_VERSIONS/03_MANA_boosted.tar.gz",
        "size": "168KB",
        "rank": "4th globally"
    },
    "04_qclassic": {
        "name": "QClassic Quantum Complete",
        "file": "FHE_VERSIONS/04_QClassic_quantum_complete.tar.gz",
        "size": "188KB",
        "rank": "Newest - Grover+Shor+holoHD144"
    }
}


def read_request() -> Optional[dict]:
    """Read a JSON-RPC request from stdin."""
    line = sys.stdin.readline()
    if not line:
        return None
    return json.loads(line)


def write_response(response: dict):
    """Write a JSON-RPC response to stdout."""
    sys.stdout.write(json.dumps(response) + "\n")
    sys.stdout.flush()


def handle_initialize(request_id: int, params: dict) -> dict:
    """Handle the initialize request."""
    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "qmnf-tops",
                "version": "1.0.0"
            }
        }
    }


def handle_list_tools(request_id: int) -> dict:
    """Handle tools/list request."""
    tools = [
        {
            "name": "list_modules",
            "description": "List all QMNF arithmetic modules with descriptions",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "domain": {
                        "type": "string",
                        "description": "Optional domain filter (RNS, FHE, QUANTUM, CHAOS, CRYPTO, ENTROPY, ARITHMETIC, WASSAN)"
                    }
                }
            }
        },
        {
            "name": "get_module",
            "description": "Get the source code and details of a specific arithmetic module",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "module_id": {
                        "type": "string",
                        "description": "Module ID (e.g., '06_k_elimination' or just 'k_elimination')"
                    }
                },
                "required": ["module_id"]
            }
        },
        {
            "name": "list_problems",
            "description": "List all impossible problems organized by domain",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "domain": {
                        "type": "string",
                        "description": "Optional domain filter (RNS, CHAOS, FHE, QUANTUM, FLOATING_POINT)"
                    }
                }
            }
        },
        {
            "name": "get_problem",
            "description": "Get details of a specific impossible problem",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "problem_id": {
                        "type": "string",
                        "description": "Problem ID (e.g., '1.1', '2.3', '3.5')"
                    }
                },
                "required": ["problem_id"]
            }
        },
        {
            "name": "search_modules",
            "description": "Search modules by keyword in name, description, or problem solved",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    }
                },
                "required": ["query"]
            }
        },
        {
            "name": "search_problems",
            "description": "Search problems by keyword",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    }
                },
                "required": ["query"]
            }
        },
        {
            "name": "get_fhe_versions",
            "description": "List all 4 FHE versions (Original, Stable 12th, MANA 4th, QClassic)",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "get_master_reference",
            "description": "Get the full 64 Impossible Problems master reference document",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "add_module",
            "description": "Add a new module to the registry",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "module_id": {
                        "type": "string",
                        "description": "Module ID (e.g., '28_new_module')"
                    },
                    "name": {
                        "type": "string",
                        "description": "Human-readable name"
                    },
                    "description": {
                        "type": "string",
                        "description": "What this module does"
                    },
                    "problem_solved": {
                        "type": "string",
                        "description": "What problem this solves"
                    },
                    "file": {
                        "type": "string",
                        "description": "Filename (e.g., '28_new_module.rs')"
                    },
                    "domain": {
                        "type": "string",
                        "description": "Domain (RNS, FHE, QUANTUM, CHAOS, CRYPTO, ENTROPY, ARITHMETIC, WASSAN)"
                    }
                },
                "required": ["module_id", "name", "description", "problem_solved", "file", "domain"]
            }
        },
        {
            "name": "add_problem",
            "description": "Add a new impossible problem to the registry",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "domain_key": {
                        "type": "string",
                        "description": "Domain key (e.g., 'DOMAIN_1_RNS')"
                    },
                    "problem_id": {
                        "type": "string",
                        "description": "Problem ID (e.g., '1.4')"
                    },
                    "name": {
                        "type": "string",
                        "description": "Problem name"
                    },
                    "conventional": {
                        "type": "string",
                        "description": "Conventional position"
                    },
                    "solution": {
                        "type": "string",
                        "description": "QMNF solution"
                    },
                    "evidence": {
                        "type": "string",
                        "description": "Evidence"
                    },
                    "implementation": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Implementation files"
                    }
                },
                "required": ["domain_key", "problem_id", "name", "conventional", "solution", "evidence", "implementation"]
            }
        },
        {
            "name": "get_stats",
            "description": "Get statistics on modules and problems",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }
    ]

    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {"tools": tools}
    }


def tool_list_modules(domain: Optional[str] = None) -> str:
    """List all modules."""
    modules = load_modules()
    result = f"# QMNF_TOPS Arithmetic Modules ({len(modules)} total)\n\n"

    for mod_id, mod in modules.items():
        if domain and mod.get("domain", "").upper() != domain.upper():
            continue
        internal = " [INTERNAL]" if mod.get("internal") else ""
        result += f"## {mod_id}{internal}\n"
        result += f"**{mod['name']}**\n"
        result += f"- Domain: {mod.get('domain', 'N/A')}\n"
        result += f"- Description: {mod['description']}\n"
        result += f"- Problem Solved: {mod['problem_solved']}\n\n"
    return result


def tool_get_module(module_id: str) -> str:
    """Get module details and source code."""
    modules = load_modules()

    # Normalize module_id
    normalized = module_id.lower().replace("-", "_")

    # Find matching module
    found_key = None
    for key in modules:
        if normalized in key.lower() or key.lower() in normalized:
            found_key = key
            break

    if not found_key:
        return f"Module '{module_id}' not found. Use list_modules to see available modules."

    mod = modules[found_key]
    file_path = QMNF_TOPS_PATH / mod["file"]

    internal = " [INTERNAL - KEEP SECRET]" if mod.get("internal") else ""
    result = f"# {mod['name']}{internal}\n\n"
    result += f"**ID:** {found_key}\n"
    result += f"**Domain:** {mod.get('domain', 'N/A')}\n"
    result += f"**Description:** {mod['description']}\n"
    result += f"**Problem Solved:** {mod['problem_solved']}\n\n"

    if file_path.exists():
        result += f"## Source Code ({mod['file']})\n\n```rust\n"
        result += file_path.read_text()
        result += "\n```"
    else:
        result += f"*Source file not found: {file_path}*"

    return result


def tool_list_problems(domain: Optional[str] = None) -> str:
    """List problems, optionally filtered by domain."""
    problems = load_problems()
    total = sum(len(d["problems"]) for d in problems.values())
    result = f"# {total}+ Impossible Problems Refuted by QMNF\n\n"

    for domain_key, domain_data in problems.items():
        if domain and domain.upper() not in domain_key:
            continue

        result += f"## {domain_data['name']} ({domain_data['age']})\n\n"
        for prob in domain_data["problems"]:
            result += f"### {prob['id']}: {prob['name']}\n"
            result += f"- **Conventional:** {prob['conventional']}\n"
            result += f"- **QMNF Solution:** {prob['solution']}\n"
            result += f"- **Evidence:** {prob['evidence']}\n"
            result += f"- **Implementation:** {', '.join(prob['implementation'])}\n\n"

    return result


def tool_get_problem(problem_id: str) -> str:
    """Get specific problem details."""
    problems = load_problems()
    for domain_data in problems.values():
        for prob in domain_data["problems"]:
            if prob["id"] == problem_id:
                result = f"# Problem {prob['id']}: {prob['name']}\n\n"
                result += f"**Domain:** {domain_data['name']}\n"
                result += f"**Age:** {domain_data['age']}\n\n"
                result += f"## Conventional Position\n{prob['conventional']}\n\n"
                result += f"## QMNF Solution\n{prob['solution']}\n\n"
                result += f"## Evidence\n{prob['evidence']}\n\n"
                result += f"## Implementation Files\n"
                for impl in prob["implementation"]:
                    result += f"- `{impl}`\n"
                return result

    return f"Problem '{problem_id}' not found. Use list_problems to see available problems."


def tool_search_modules(query: str) -> str:
    """Search modules by keyword."""
    modules = load_modules()
    query_lower = query.lower()
    matches = []

    for mod_id, mod in modules.items():
        searchable = f"{mod_id} {mod['name']} {mod['description']} {mod['problem_solved']} {mod.get('domain', '')}".lower()
        if query_lower in searchable:
            matches.append((mod_id, mod))

    if not matches:
        return f"No modules found matching '{query}'"

    result = f"# Modules matching '{query}' ({len(matches)} found)\n\n"
    for mod_id, mod in matches:
        result += f"## {mod_id}: {mod['name']}\n"
        result += f"- Domain: {mod.get('domain', 'N/A')}\n"
        result += f"- {mod['description']}\n"
        result += f"- Solves: {mod['problem_solved']}\n\n"

    return result


def tool_search_problems(query: str) -> str:
    """Search problems by keyword."""
    problems = load_problems()
    query_lower = query.lower()
    matches = []

    for domain_data in problems.values():
        for prob in domain_data["problems"]:
            searchable = f"{prob['name']} {prob['conventional']} {prob['solution']} {prob['evidence']}".lower()
            if query_lower in searchable:
                matches.append((domain_data["name"], prob))

    if not matches:
        return f"No problems found matching '{query}'"

    result = f"# Problems matching '{query}' ({len(matches)} found)\n\n"
    for domain_name, prob in matches:
        result += f"## {prob['id']}: {prob['name']}\n"
        result += f"- Domain: {domain_name}\n"
        result += f"- Solution: {prob['solution']}\n\n"

    return result


def tool_get_fhe_versions() -> str:
    """Get FHE version information."""
    result = "# NINE65 FHE Versions (4 total)\n\n"
    for ver_id, ver in FHE_VERSIONS.items():
        result += f"## {ver['name']}\n"
        result += f"- File: `{ver['file']}`\n"
        result += f"- Size: {ver['size']}\n"
        result += f"- Rank: {ver['rank']}\n\n"
    return result


def tool_get_master_reference() -> str:
    """Get the full master reference document."""
    master_path = QMNF_TOPS_PATH / "00_MASTER_REFERENCE_64_IMPOSSIBLE_PROBLEMS.md"
    if master_path.exists():
        return master_path.read_text()
    return "Master reference document not found."


def tool_add_module(module_id: str, name: str, description: str,
                    problem_solved: str, file: str, domain: str) -> str:
    """Add a new module to the registry."""
    modules = load_modules()

    if module_id in modules:
        return f"Module '{module_id}' already exists. Use a different ID."

    modules[module_id] = {
        "name": name,
        "description": description,
        "problem_solved": problem_solved,
        "file": file,
        "domain": domain.upper()
    }

    save_modules(modules)
    return f"Module '{module_id}' added successfully. Total modules: {len(modules)}"


def tool_add_problem(domain_key: str, problem_id: str, name: str,
                     conventional: str, solution: str, evidence: str,
                     implementation: list) -> str:
    """Add a new problem to the registry."""
    problems = load_problems()

    if domain_key not in problems:
        return f"Domain '{domain_key}' not found. Available: {list(problems.keys())}"

    # Check if problem_id already exists
    for prob in problems[domain_key]["problems"]:
        if prob["id"] == problem_id:
            return f"Problem '{problem_id}' already exists in {domain_key}."

    problems[domain_key]["problems"].append({
        "id": problem_id,
        "name": name,
        "conventional": conventional,
        "solution": solution,
        "evidence": evidence,
        "implementation": implementation
    })

    save_problems(problems)
    total = sum(len(d["problems"]) for d in problems.values())
    return f"Problem '{problem_id}' added to {domain_key}. Total problems: {total}"


def tool_get_stats() -> str:
    """Get statistics on modules and problems."""
    modules = load_modules()
    problems = load_problems()

    # Count by domain
    module_domains = {}
    for mod in modules.values():
        d = mod.get("domain", "UNKNOWN")
        module_domains[d] = module_domains.get(d, 0) + 1

    total_problems = sum(len(d["problems"]) for d in problems.values())

    result = "# QMNF_TOPS Statistics\n\n"
    result += f"## Modules: {len(modules)} total\n\n"
    result += "| Domain | Count |\n|--------|-------|\n"
    for domain, count in sorted(module_domains.items()):
        result += f"| {domain} | {count} |\n"

    result += f"\n## Problems: {total_problems} total\n\n"
    result += "| Domain | Problems |\n|--------|----------|\n"
    for domain_key, domain_data in problems.items():
        result += f"| {domain_data['name']} | {len(domain_data['problems'])} |\n"

    result += f"\n## FHE Versions: {len(FHE_VERSIONS)}\n"

    return result


def handle_tool_call(request_id: int, params: dict) -> dict:
    """Handle tools/call request."""
    tool_name = params.get("name")
    args = params.get("arguments", {})

    try:
        if tool_name == "list_modules":
            result = tool_list_modules(args.get("domain"))
        elif tool_name == "get_module":
            result = tool_get_module(args.get("module_id", ""))
        elif tool_name == "list_problems":
            result = tool_list_problems(args.get("domain"))
        elif tool_name == "get_problem":
            result = tool_get_problem(args.get("problem_id", ""))
        elif tool_name == "search_modules":
            result = tool_search_modules(args.get("query", ""))
        elif tool_name == "search_problems":
            result = tool_search_problems(args.get("query", ""))
        elif tool_name == "get_fhe_versions":
            result = tool_get_fhe_versions()
        elif tool_name == "get_master_reference":
            result = tool_get_master_reference()
        elif tool_name == "add_module":
            result = tool_add_module(
                args.get("module_id", ""),
                args.get("name", ""),
                args.get("description", ""),
                args.get("problem_solved", ""),
                args.get("file", ""),
                args.get("domain", "")
            )
        elif tool_name == "add_problem":
            result = tool_add_problem(
                args.get("domain_key", ""),
                args.get("problem_id", ""),
                args.get("name", ""),
                args.get("conventional", ""),
                args.get("solution", ""),
                args.get("evidence", ""),
                args.get("implementation", [])
            )
        elif tool_name == "get_stats":
            result = tool_get_stats()
        else:
            result = f"Unknown tool: {tool_name}"

        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "content": [{"type": "text", "text": result}]
            }
        }
    except Exception as e:
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {"code": -32000, "message": str(e)}
        }


def main():
    """Main MCP server loop."""
    while True:
        request = read_request()
        if request is None:
            break

        method = request.get("method")
        request_id = request.get("id")
        params = request.get("params", {})

        if method == "initialize":
            response = handle_initialize(request_id, params)
        elif method == "notifications/initialized":
            continue  # No response needed
        elif method == "tools/list":
            response = handle_list_tools(request_id)
        elif method == "tools/call":
            response = handle_tool_call(request_id, params)
        else:
            response = {
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {"code": -32601, "message": f"Unknown method: {method}"}
            }

        write_response(response)


if __name__ == "__main__":
    main()
