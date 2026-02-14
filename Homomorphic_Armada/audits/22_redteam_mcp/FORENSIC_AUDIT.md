# FORENSIC AUDIT: Build 22 -- redteam_mcp

**Audit Date**: 2026-02-13
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/`
**Source Origin**: `https://github.com/Skyelabz210/redteam-mcp.git` (private)
**Auditor**: Automated forensic code audit (Claude Opus 4.6)
**Classification**: Red team MCP security tooling (Python)
**Audit Scope**: INSPECT, ANALYZE, REPORT -- no modifications to source

---

## TABLE OF CONTENTS

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Detection](#5-dead-code-detection)
6. [Security Audit](#6-security-audit)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. STRUCTURE MAPPING

### 1.1 File Inventory

| File | Lines | Size | Purpose |
|------|-------|------|---------|
| `grover_swarm_server.py` | 940 | 34,398 B | MCP server: knowledge graph + Grover swarm search (stdio transport) |
| `redteam_http_server.py` | 945 | 34,115 B | MCP server: cryptanalysis tools over HTTPS (HTTP transport) |
| `redteam_server.py` | 1,050 | 39,173 B | MCP server: cryptanalysis tools + external binary execution (stdio transport) |
| `README.md` | 387 | 11,059 B | Full documentation for redteam_http_server.py |
| `QUICKSTART.md` | 137 | 3,091 B | Quick reference card for redteam_http_server.py |
| `.gitignore` | 52 | 405 B | Standard Python + auth/cert exclusions |

**Total Python**: 2,935 lines across 3 files.

### 1.2 Git History

Only 2 commits exist on branch `main`:
```
b3cbbfe  feat: RedTeam MCP Server v3.0.0 - Security-hardened cryptanalysis toolkit
8d7eae4  docs: Add quick reference card
```

### 1.3 Dependency Map (Imports)

#### grover_swarm_server.py
```
Standard Library Only:
  json, sys, hashlib, time, pathlib.Path, datetime.datetime,
  typing (Dict, List, Optional, Set, Tuple),
  dataclasses (dataclass, asdict), enum.Enum
Conditional (inside function): math
```

#### redteam_http_server.py
```
Standard Library Only:
  json, hashlib, time, os, sys, math, random, ssl, signal,
  secrets, functools,
  http.server (HTTPServer, BaseHTTPRequestHandler),
  urllib.parse (urlparse, parse_qs),
  datetime.datetime, pathlib.Path,
  threading (Timer, Thread),
  typing (Optional, Dict, Any, Callable)
```

#### redteam_server.py
```
Standard Library Only:
  json, subprocess, sys, os, hashlib, time,
  pathlib.Path, datetime.datetime
Conditional (inside functions): math, random, re, tarfile, tempfile (imported but unused)
```

**Observation**: All three files are zero-dependency (standard library only). No `requirements.txt` or `pyproject.toml` exists. No external packages are needed.

### 1.4 Hardcoded Paths

| File | Line(s) | Path | Purpose |
|------|---------|------|---------|
| `grover_swarm_server.py` | 44 | `/home/acid/Projects/NINE65/MANA-private/mcp-server/logs` | Log directory |
| `grover_swarm_server.py` | 45 | `/home/acid/Projects/NINE65/MANA-private/mcp-server/knowledge_graph.json` | Knowledge graph persistence |
| `redteam_http_server.py` | 39 | `/home/acid/Projects/RedTeam/tools/logs` | Log directory |
| `redteam_http_server.py` | 40 | `/tmp/redteam.pid` | PID file |
| `redteam_http_server.py` | 41 | `/home/acid/Projects/RedTeam/tools/.auth_token` | Token file |
| `redteam_server.py` | 40 | `/home/acid/Projects/NINE65/MANA-private/target/release` | Rust binary directory |
| `redteam_server.py` | 41 | `/home/acid/Projects/NINE65/MANA-private/mcp-server/logs` | Log directory |

**All paths are absolute and user-specific (`/home/acid/...`). None of these directories necessarily exist on the machine running the build.**

---

## 2. DATA FLOW TRACING

### 2.1 grover_swarm_server.py -- Knowledge Graph MCP (stdio)

```
TRANSPORT: stdio (stdin line-by-line JSON, stdout JSON responses)

STARTUP:
  1. Sends pre-baked JSON-RPC "initialize" response to stdout (line 905-918)
  2. KnowledgeGraph() constructor loads from KNOWLEDGE_DB if it exists (line 443)
  3. Enters blocking stdin read loop (line 921)

REQUEST FLOW:
  stdin (JSON line) --> json.loads --> handle_request(request)
                                        |
                                        +-> method == "tools/list" --> return tool schemas
                                        |
                                        +-> method == "tools/call" --> rate limit check
                                             |                         --> dispatch to tool
                                             |                         --> log_request()
                                             |                         --> return MCP content
                                             |
                                             +-> tool: grover_register_fact
                                             +-> tool: grover_add_edge
                                             +-> tool: grover_query_recon
                                             +-> tool: grover_launch_swarm
                                             +-> tool: grover_validate_discovery
                                             +-> tool: grover_compute_potential
                                             +-> tool: grover_get_status
                                             +-> tool: grover_initialize_kelimination
                                             +-> tool: grover_toric_validation

PERSISTENCE:
  - KnowledgeGraph saves to disk (JSON) after every mutation
  - Audit logs written to LOG_DIR per request

OUTPUT:
  stdout JSON-RPC responses with "content" array containing text items
```

### 2.2 redteam_http_server.py -- Cryptanalysis MCP (HTTPS)

```
TRANSPORT: HTTP/HTTPS on localhost:8765 (configurable via REDTEAM_PORT env)

STARTUP:
  1. get_or_create_token() -- reads env, file, or generates new token
  2. startup_checks() -- validates log dir, token file permissions
  3. Signal handlers registered (SIGINT, SIGTERM)
  4. PID file written
  5. SSL context loaded if certs exist
  6. HTTPServer("127.0.0.1", PORT) -- localhost only
  7. serve_forever() blocking loop

REQUEST FLOW (GET):
  HTTP GET --> MCPHandler.do_GET()
    /            --> server info (no auth)
    /health      --> health check (no auth)
    /tools       --> tool list (no auth)
    /initialize  --> MCP handshake (no auth)
    /prompts     --> empty list (no auth)
    /resources   --> empty list (no auth)
    /metrics     --> usage stats (auth REQUIRED)
    else         --> 404

REQUEST FLOW (POST):
  HTTP POST --> MCPHandler.do_POST()
    1. Content-Length check (max 64KB)
    2. Bearer token auth check
    3. Rate limit check (30/min)
    4. JSON body parse
    5. Route: /tools/call only
       --> lookup tool in TOOLS dict
       --> handler(**args) with timeout wrapper
       --> log_request()
       --> return MCP content response

TOOL EXECUTION:
  Each tool function wrapped in @timeout_wrapper(30):
    - Spawns Thread, joins with 30s timeout
    - Returns timeout error if thread alive after 30s
    NOTE: Thread continues running even after timeout (no kill mechanism)

OUTPUT:
  HTTP JSON responses, optionally wrapped in JSON-RPC format if request had "id"
```

### 2.3 redteam_server.py -- Cryptanalysis MCP (stdio)

```
TRANSPORT: stdio (stdin line-by-line JSON, stdout JSON responses)

STARTUP:
  1. Sends pre-baked JSON-RPC "initialize" response
  2. Enters blocking stdin read loop

REQUEST FLOW:
  stdin JSON --> handle_request(request)
    |
    +-> "tools/list" --> returns large tools array (6 binary tools + 8 Python tools)
    |
    +-> "tools/call" --> dispatch by tool_name:
         |
         +-> redshirt_estimate         --> estimate_security()
         +-> redshirt_shor_factor      --> shor_factor()
         +-> redshirt_quantum_assessment --> quantum_security_assessment()
         +-> redshirt_grover_speedup   --> inline computation
         +-> redshirt_grover_attack    --> grover_attack()
         +-> redshirt_combined_quantum --> combined_quantum_attack()
         +-> redshirt_cryptanalysis    --> analyze_crypto_codebase() [READS FILESYSTEM]
         +-> redshirt_shadow_lwe       --> shadow_entropy_lwe_attack()
         +-> redshirt_*               --> run_tool() [EXECUTES EXTERNAL BINARIES]

EXTERNAL BINARY EXECUTION (run_tool):
  subprocess.run([TOOLS_DIR / tool_name] + args, timeout=60)
  Binaries expected at: /home/acid/Projects/NINE65/MANA-private/target/release/
  Expected binaries:
    - attack-estimator
    - calibrated-estimator
    - self-cryptanalysis
    - lattice-attack
    - k-elimination-attack
    - redshirt-testbed

FILESYSTEM ACCESS (analyze_crypto_codebase):
  - Accepts arbitrary path argument
  - Reads tarballs (.tar.gz, .tgz, .tar) and directories
  - Scans for crypto patterns using regex
  - NO path sanitization or restriction
```

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 grover_swarm_server.py

#### Enums
| Name | Values | Line |
|------|--------|------|
| `DiscoveryState` | UNKNOWN, SIGHTED, VALIDATED, INTEGRATED | 52-56 |

#### Data Classes
| Name | Fields | Line |
|------|--------|------|
| `Atom` | id, content, topic, weight, state, source_uri, timestamp | 59-73 |
| `Edge` | from_id, to_id, relation, confidence | 76-85 |
| `SwarmNode` | id, partition_size, weight, budget, marked_count, neighbor_count | 88-96 |

#### Classes
| Name | Methods | Line |
|------|---------|------|
| `KnowledgeGraph` | __init__, load, save, register_fact, add_edge, get_neighbors, compute_innovation_potential, query_recon, launch_swarm, validate_discovery, get_status | 99-439 |

#### Functions
| Name | Parameters | Line | Purpose |
|------|-----------|------|---------|
| `log_request` | action, params, result | 446 | Audit logging |
| `check_rate_limit` | (none) | 462 | Rate limiting (100/min) |
| `initialize_kelimination_knowledge` | (none) | 475 | Seeds knowledge graph with domain data |
| `get_toric_validation` | qubits=10 | 629 | Returns pre-computed Grover validation results |
| `handle_request` | request | 699 | MCP request dispatcher |
| `main` | (none) | 902 | Entry point (stdio loop) |

#### MCP Tools Exposed (9)
| Tool Name | Handler | Line |
|-----------|---------|------|
| `grover_register_fact` | graph.register_fact | 834 |
| `grover_add_edge` | graph.add_edge | 844 |
| `grover_query_recon` | graph.query_recon | 853 |
| `grover_launch_swarm` | graph.launch_swarm | 861 |
| `grover_validate_discovery` | graph.validate_discovery | 869 |
| `grover_compute_potential` | graph.compute_innovation_potential | 876 |
| `grover_get_status` | graph.get_status | 882 |
| `grover_initialize_kelimination` | initialize_kelimination_knowledge | 885 |
| `grover_toric_validation` | get_toric_validation | 889 |

### 3.2 redteam_http_server.py

#### Functions
| Name | Decorator | Parameters | Line | Purpose |
|------|-----------|-----------|------|---------|
| `get_or_create_token` | -- | (none) | 56 | Token management |
| `check_rate_limit` | -- | (none) | 84 | Rate limiting (30/min) |
| `log_request` | -- | action, args, result, client_ip, duration_ms | 95 | Audit logging |
| `timeout_wrapper` | -- | timeout_sec | 116 | Decorator factory for tool timeouts |
| `validate_int` | -- | value, name, min_val, max_val | 139 | Input validation helper |
| `grover_attack` | @timeout_wrapper(30) | search_space_bits=128 | 157 | Grover simulation |
| `shor_factor` | @timeout_wrapper(30) | n, attempts=10 | 196 | Shor factoring |
| `order_finding` | @timeout_wrapper(30) | a, n | 286 | BSGS order finding |
| `shadow_entropy_lwe` | @timeout_wrapper(30) | n=256, q=3329, samples=10 | 346 | LWE attack analysis |
| `analyze_scheme` | @timeout_wrapper(30) | scheme, key_bits=256 | 388 | Crypto scheme assessment |
| `k_elimination_demo` | @timeout_wrapper(30) | value, modulus, anchor | 433 | K-Elimination demo |
| `gso_fhe_analysis` | @timeout_wrapper(30) | depth=50, noise_budget=200 | 490 | GSO-FHE analysis |
| `signal_handler` | -- | signum, frame | 839 | Graceful shutdown |
| `write_pid` | -- | (none) | 849 | PID file management |
| `startup_checks` | -- | (none) | 854 | Pre-flight validation |
| `main` | -- | (none) | 877 | Entry point |

#### Classes
| Name | Parent | Methods | Line |
|------|--------|---------|------|
| `MCPHandler` | BaseHTTPRequestHandler | log_message, get_client_ip, get_origin, send_json, check_auth, do_OPTIONS, do_GET, do_POST | 632 |

#### TOOLS Registry (7 entries)
| Key | Handler Function | Line |
|-----|-----------------|------|
| `redteam_grover` | grover_attack | 535 |
| `redteam_shor` | shor_factor | 551 |
| `redteam_order` | order_finding | 563 |
| `redteam_shadow_lwe` | shadow_entropy_lwe | 575 |
| `redteam_analyze` | analyze_scheme | 586 |
| `redteam_k_elimination` | k_elimination_demo | 601 |
| `redteam_gso_fhe` | gso_fhe_analysis | 614 |

#### HTTP Endpoints
| Path | Method | Auth Required | Handler |
|------|--------|--------------|---------|
| `/` | GET | No | Server info |
| `/health` | GET | No | Health check |
| `/tools` | GET | No | Tool listing |
| `/initialize` | GET | No | MCP handshake |
| `/prompts` | GET | No | Empty list |
| `/resources` | GET | No | Empty list |
| `/metrics` | GET | Yes | Usage stats |
| `/tools/call` | POST | Yes | Tool execution |
| `*` | OPTIONS | No | CORS preflight |

### 3.3 redteam_server.py

#### Functions
| Name | Parameters | Line | Purpose |
|------|-----------|------|---------|
| `log_request` | action, params, result | 58 | Audit logging |
| `check_rate_limit` | (none) | 74 | Rate limiting (10/min) |
| `run_tool` | tool_name, args=None | 87 | External binary execution |
| `gcd` | a, b | 123 | Euclidean GCD |
| `mod_pow` | base, exp, mod | 130 | Modular exponentiation |
| `find_order_bsgs` | a, n, bound=None | 142 | BSGS order finding |
| `shor_factor` | n, attempts=10 | 197 | Shor factoring |
| `quantum_security_assessment` | scheme, key_bits | 286 | Post-quantum assessment |
| `grover_attack` | search_space_bits, target_prob=0.9613 | 368 | Grover simulation |
| `combined_quantum_attack` | target_bits, scheme="hybrid" | 412 | Combined Shor+Grover |
| `shadow_entropy_lwe_attack` | n=256, q=3329, samples=10 | 488 | LWE attack analysis |
| `analyze_crypto_codebase` | path | 606 | Filesystem scanning for crypto patterns |
| `estimate_security` | scheme, params | 750 | BKZ security estimation |
| `handle_request` | request | 802 | MCP request dispatcher |
| `main` | (none) | 1014 | Entry point (stdio loop) |

#### MCP Tools Exposed (14)
| Tool Name | Handler | Type |
|-----------|---------|------|
| `redshirt_attack-estimator` | run_tool (subprocess) | Binary |
| `redshirt_calibrated-estimator` | run_tool (subprocess) | Binary |
| `redshirt_self-cryptanalysis` | run_tool (subprocess) | Binary |
| `redshirt_lattice-attack` | run_tool (subprocess) | Binary |
| `redshirt_k-elimination-attack` | run_tool (subprocess) | Binary |
| `redshirt_redshirt-testbed` | run_tool (subprocess) | Binary |
| `redshirt_estimate` | estimate_security | Python |
| `redshirt_shor_factor` | shor_factor | Python |
| `redshirt_quantum_assessment` | quantum_security_assessment | Python |
| `redshirt_grover_speedup` | inline | Python |
| `redshirt_grover_attack` | grover_attack | Python |
| `redshirt_combined_quantum` | combined_quantum_attack | Python |
| `redshirt_cryptanalysis` | analyze_crypto_codebase | Python |
| `redshirt_shadow_lwe` | shadow_entropy_lwe_attack | Python |

---

## 4. WIRING VERIFICATION

### 4.1 grover_swarm_server.py

**Startup Flow**: CLEAN
- `main()` emits JSON-RPC initialize response, then enters stdin loop.
- All 9 tools listed in `tools/list` have matching handler branches in `tools/call`.
- Global `graph = KnowledgeGraph()` is instantiated at module load (line 443).

**Tool Wiring Verification**:

| Tool Name (tools/list) | Dispatched in tools/call? | Handler exists? | Status |
|------------------------|--------------------------|----------------|--------|
| grover_register_fact | YES (line 834) | graph.register_fact | WIRED |
| grover_add_edge | YES (line 844) | graph.add_edge | WIRED |
| grover_query_recon | YES (line 853) | graph.query_recon | WIRED |
| grover_launch_swarm | YES (line 861) | graph.launch_swarm | WIRED |
| grover_validate_discovery | YES (line 869) | graph.validate_discovery | WIRED |
| grover_compute_potential | YES (line 876) | graph.compute_innovation_potential | WIRED |
| grover_get_status | YES (line 882) | graph.get_status | WIRED |
| grover_initialize_kelimination | YES (line 885) | initialize_kelimination_knowledge | WIRED |
| grover_toric_validation | YES (line 889) | get_toric_validation | WIRED |

**Result**: All 9 tools are fully wired. No orphans. No missing handlers.

### 4.2 redteam_http_server.py

**Startup Flow**: CLEAN
- `main()` runs startup_checks, registers signal handlers, writes PID, configures SSL if available, starts HTTP server on 127.0.0.1.
- All 7 tools in TOOLS dict have handler functions assigned.
- GET and POST routes are handled in do_GET / do_POST.

**Tool Wiring Verification**:

| Tool Name (TOOLS dict) | Handler assigned? | Handler exists? | Status |
|------------------------|------------------|----------------|--------|
| redteam_grover | YES | grover_attack | WIRED |
| redteam_shor | YES | shor_factor | WIRED |
| redteam_order | YES | order_finding | WIRED |
| redteam_shadow_lwe | YES | shadow_entropy_lwe | WIRED |
| redteam_analyze | YES | analyze_scheme | WIRED |
| redteam_k_elimination | YES | k_elimination_demo | WIRED |
| redteam_gso_fhe | YES | gso_fhe_analysis | WIRED |

**Endpoint Wiring**:

| Endpoint | Dispatched? | Status |
|----------|------------|--------|
| GET / | YES | WIRED |
| GET /health | YES | WIRED |
| GET /tools | YES | WIRED |
| GET /initialize | YES | WIRED |
| GET /prompts | YES | WIRED |
| GET /resources | YES | WIRED |
| GET /metrics | YES | WIRED |
| POST /tools/call | YES | WIRED |
| OPTIONS * | YES | WIRED |

**Result**: All 7 tools and all endpoints are fully wired.

### 4.3 redteam_server.py

**Startup Flow**: PARTIALLY CLEAN
- `main()` sends JSON-RPC initialize, enters stdin loop. No startup validation.
- The `tools/list` response combines 6 binary tool descriptors with 8 Python tool descriptors.
- The binary tools reference external Rust binaries at a hardcoded path.

**Tool Wiring Verification (Python tools)**:

| Tool Name | Dispatched? | Handler exists? | Status |
|-----------|------------|----------------|--------|
| redshirt_estimate | YES (line 926) | estimate_security | WIRED |
| redshirt_shor_factor | YES (line 937) | shor_factor | WIRED |
| redshirt_quantum_assessment | YES (line 944) | quantum_security_assessment | WIRED |
| redshirt_grover_speedup | YES (line 951) | inline computation | WIRED |
| redshirt_grover_attack | YES (line 974) | grover_attack | WIRED |
| redshirt_combined_quantum | YES (line 981) | combined_quantum_attack | WIRED |
| redshirt_cryptanalysis | YES (line 988) | analyze_crypto_codebase | WIRED |
| redshirt_shadow_lwe | YES (line 996) | shadow_entropy_lwe_attack | WIRED |

**Tool Wiring Verification (Binary tools)**:

| Tool Name Pattern | Dispatched? | Handler | Status |
|-------------------|------------|---------|--------|
| redshirt_attack-estimator | YES (line 1004, fallthrough) | run_tool("attack-estimator") | WIRED* |
| redshirt_calibrated-estimator | YES (fallthrough) | run_tool("calibrated-estimator") | WIRED* |
| redshirt_self-cryptanalysis | YES (fallthrough) | run_tool("self-cryptanalysis") | WIRED* |
| redshirt_lattice-attack | YES (fallthrough) | run_tool("lattice-attack") | WIRED* |
| redshirt_k-elimination-attack | YES (fallthrough) | run_tool("k-elimination-attack") | WIRED* |
| redshirt_redshirt-testbed | YES (fallthrough) | run_tool("redshirt-testbed") | WIRED* |

\* These are wired via the generic fallthrough at line 1004-1006:
```python
elif tool_name.startswith("redshirt_"):
    actual_tool = tool_name.replace("redshirt_", "")
    result = run_tool(actual_tool, arguments.get("args", []))
```
**However**: The actual tool name after stripping `redshirt_` would be, e.g., `attack-estimator`, which IS in the TOOLS dict, so `run_tool()` will accept it. BUT the binary at `TOOLS_DIR / "attack-estimator"` must exist on disk. These binaries are NOT included in the repository.

**Result**: All Python tools wired. Binary tools wired in code but depend on external binaries not present in this build.

### 4.4 Grover Swarm Integration

**Question**: Is grover_swarm_server.py integrated with the other two servers?

**Answer**: NO. The three servers are completely independent:
- `grover_swarm_server.py` is a standalone MCP server (stdio transport, different tool namespace `grover_*`)
- `redteam_http_server.py` is a standalone MCP server (HTTP transport, tool namespace `redteam_*`)
- `redteam_server.py` is a standalone MCP server (stdio transport, tool namespace `redshirt_*`)

They share no imports, no data structures, no communication channels, and no cross-references. They are three separate MCP servers that happen to live in the same repository.

---

## 5. DEAD CODE DETECTION

### 5.1 grover_swarm_server.py

| Item | Line | Type | Status |
|------|------|------|--------|
| `SwarmNode` dataclass | 88-96 | Class | **DEAD CODE** -- defined but never instantiated anywhere. No function creates or returns a SwarmNode. |
| `Edge.to_dict()` | 84-85 | Method | Used in `add_edge` and `save`. LIVE. |
| `Atom.to_dict()` | 70-73 | Method | Used in `save` and `register_fact` result. LIVE. |

### 5.2 redteam_http_server.py

| Item | Line | Type | Status |
|------|------|------|--------|
| `parse_qs` (import) | 28 | Import | **DEAD CODE** -- imported from urllib.parse but never used. |
| `Timer` (import) | 31 | Import | **DEAD CODE** -- imported from threading but never used. |
| `signal_handler` references `server_instance` | 843 | Logic | LIVE (used in signal handler to shutdown). |

### 5.3 redteam_server.py

| Item | Line | Type | Status |
|------|------|------|--------|
| `tempfile` (import) | 621 | Import | **DEAD CODE** -- imported inside `analyze_crypto_codebase` but never used. |
| `grover_attack` parameter `target_prob` | 368 | Parameter | The default value 0.9613 is used as float. The function uses floats extensively (`1.0 / math.sqrt(n)`, etc.). |
| `os` module | 33 | Import | Used only in `analyze_crypto_codebase` for `os.walk`, `os.path.isdir`, `os.path.join`. Could use pathlib instead but LIVE. |
| Binary tools dict entries | 46-52 | Dict | All 6 binary tool entries map to descriptions. They ARE used in `run_tool` for existence checks. LIVE (though binaries absent). |

### Summary of Dead Code

| File | Dead Items | Severity |
|------|-----------|----------|
| grover_swarm_server.py | `SwarmNode` class (never used) | Low |
| redteam_http_server.py | `parse_qs` import, `Timer` import | Trivial |
| redteam_server.py | `tempfile` import | Trivial |

---

## 6. SECURITY AUDIT

### 6.1 CRITICAL FINDINGS

#### [SEC-CRITICAL-01] Arbitrary Command Execution via `run_tool()` in redteam_server.py

**File**: `redteam_server.py`, lines 87-120
**Severity**: CRITICAL
**Description**: The `run_tool` function executes external binaries via `subprocess.run()` with user-controlled arguments passed as a list. The tool name is validated against the TOOLS dict, but the `args` parameter is passed through without any sanitization.

```python
def run_tool(tool_name: str, args: list = None) -> dict:
    cmd = [str(tool_path)] + (args or [])
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=60, ...)
```

The MCP tool schema declares args as `{"type": "array", "items": {"type": "string"}}`, meaning any strings can be passed as command-line arguments to the external binaries. If any of the 6 external binaries interpret arguments unsafely (e.g., `--output /etc/passwd`), this is exploitable.

**Mitigating Factor**: The tool runs on stdio transport which implies the MCP client already has local access. However, an LLM-mediated attack could trick the model into calling `run_tool` with malicious arguments.

#### [SEC-CRITICAL-02] Arbitrary Filesystem Read via `analyze_crypto_codebase()` in redteam_server.py

**File**: `redteam_server.py`, lines 606-747
**Severity**: CRITICAL
**Description**: The `redshirt_cryptanalysis` tool accepts an arbitrary filesystem path and reads all matching files recursively:

```python
def analyze_crypto_codebase(path: str) -> dict:
    # No path sanitization
    if path.endswith(('.tar.gz', '.tgz', '.tar')):
        with tarfile.open(path, 'r:*') as tf: ...
    elif os.path.isdir(path):
        for root, _, files in os.walk(path): ...
```

An attacker (or a confused LLM) could pass `path="/"` or `path="/etc"` or `path="/home/acid/.ssh"` to read sensitive files. The function scans `.rs`, `.py`, `.c`, `.h`, `.toml`, `.json`, `.yaml`, `.yml` files and returns content matches in the response.

**No restrictions exist on**:
- Path traversal
- Symlink following
- Reading outside project directories
- File size limits per file

#### [SEC-CRITICAL-03] Tarball Path Traversal in `analyze_crypto_codebase()`

**File**: `redteam_server.py`, lines 696-706
**Severity**: CRITICAL
**Description**: The tarball extraction uses `tarfile.open()` and iterates members, but does NOT guard against path traversal within tar archives (the classic "zip slip" attack). While `extractfile()` returns a file-like object rather than extracting to disk, the member names are not validated and could contain `../` sequences.

Current code reads but does not extract to disk, so the immediate risk is information disclosure rather than arbitrary file write. However, the `member.name` is included in output, which could leak internal paths.

#### [SEC-CRITICAL-04] No Authentication on stdio Servers

**File**: `grover_swarm_server.py` and `redteam_server.py`
**Severity**: HIGH (context-dependent)
**Description**: Both stdio-transport servers have zero authentication. Any process that can connect to their stdin/stdout can execute all tools including:
- `redshirt_cryptanalysis` (arbitrary filesystem reads)
- `run_tool` (external binary execution)
- `grover_register_fact` (data mutation)

MCP stdio servers typically rely on the process launcher for access control, but if these servers are exposed via a socket bridge or similar mechanism, there is no authentication layer.

### 6.2 HIGH FINDINGS

#### [SEC-HIGH-01] Floating-Point Usage in grover_attack() (redteam_server.py)

**File**: `redteam_server.py`, lines 368-409
**Severity**: HIGH (per QMNF mandate)
**Description**: The `grover_attack()` function uses floating-point extensively:
```python
alpha = 1.0 / math.sqrt(n)  # line 381
beta = 1.0 / math.sqrt(n)   # line 382
mean = (alpha + (n - 1) * beta) / n  # line 389
```

This violates the integer-only mandate documented in `CLAUDE.md` for the QMNF system. Similar violations exist in `redteam_http_server.py` and throughout the math functions in all three files.

**Scope**: Pervasive across all files. The `math.sqrt()`, `math.log2()`, `math.sin()`, `math.asin()`, and float division operators appear throughout.

#### [SEC-HIGH-02] CORS "null" Origin Allowed

**File**: `redteam_http_server.py`, line 53
**Severity**: HIGH
**Description**: The ALLOWED_ORIGINS list includes `"null"`:
```python
ALLOWED_ORIGINS = [
    "https://claude.ai",
    "https://localhost:8765",
    "http://localhost:8765",
    "null"  # For local file testing
]
```

The `null` origin is sent by browsers for file:// URLs, data: URLs, sandboxed iframes, and cross-origin redirects. Allowing it effectively permits any local HTML file to make authenticated requests to the server if the user has the auth token. This is a known CORS bypass vector.

#### [SEC-HIGH-03] Timeout Wrapper Does Not Kill Threads

**File**: `redteam_http_server.py`, lines 116-136
**Severity**: HIGH
**Description**: The `timeout_wrapper` decorator spawns a Thread and joins with a timeout:
```python
thread = Thread(target=target)
thread.start()
thread.join(timeout=timeout_sec)
if thread.is_alive():
    return {"error": f"Tool execution timed out after {timeout_sec}s"}
```

If the thread is still alive after timeout, the function returns an error but the **thread continues running indefinitely**. Python threads cannot be forcefully killed. This means:
- Resource exhaustion via repeated timeout-triggering requests
- The `shor_factor()` and `order_finding()` functions have nested loops that could run for extended periods
- Memory consumption from abandoned threads

#### [SEC-HIGH-04] Auth Token Leaked in Server Banner

**File**: `redteam_http_server.py`, lines 915-916
**Severity**: HIGH
**Description**: The startup banner prints a partial auth token to stdout:
```python
║  Token: {AUTH_TOKEN[:16]}...{AUTH_TOKEN[-8:]}
```

This reveals 24 out of 64 hex characters (37.5% of the token). While not the full token, it significantly reduces brute-force space and could be captured in logs, terminal scrollback, or process output.

### 6.3 MEDIUM FINDINGS

#### [SEC-MED-01] PID File in /tmp (Symlink Race)

**File**: `redteam_http_server.py`, line 40
**Severity**: MEDIUM
**Description**: `PID_FILE = Path("/tmp/redteam.pid")` is written with `PID_FILE.write_text(str(os.getpid()))`. On multi-user systems, `/tmp/redteam.pid` could be pre-created as a symlink pointing to a sensitive file, causing the PID write to overwrite that file.

#### [SEC-MED-02] No Input Sanitization for `content` in grover_register_fact

**File**: `grover_swarm_server.py`, lines 156-179
**Severity**: MEDIUM
**Description**: The `register_fact` function accepts arbitrary `content` strings and stores them in JSON. If the knowledge graph JSON is ever rendered in a web context (unlikely but possible), stored XSS payloads would execute. The content is also included in log files.

#### [SEC-MED-03] JSON Deserialization Without Schema Validation

**File**: All three servers
**Severity**: MEDIUM
**Description**: All servers parse JSON input and extract fields with `.get()` without validating the overall schema. Unexpected keys are silently ignored, and missing required keys may cause `KeyError` exceptions that bubble up as generic error messages potentially leaking internal structure.

`redteam_http_server.py` has the best validation with `validate_int()` for numeric params, but the other two servers have minimal validation.

#### [SEC-MED-04] Rate Limiter Uses In-Memory State Only

**File**: All three servers
**Severity**: MEDIUM
**Description**: Rate limiting is implemented via an in-memory list of timestamps. This means:
- Server restart resets the rate limiter
- For the HTTP server, each restart allows fresh bursts
- The list grows unboundedly if `time.time()` calls are faster than the cleanup (unlikely but no cap)

#### [SEC-MED-05] `random` Module Used for Shor's Algorithm (Not CSPRNG)

**File**: `redteam_http_server.py` (line 22), `redteam_server.py` (implicit via `import random`)
**Severity**: MEDIUM
**Description**: The `random` module (Mersenne Twister, not cryptographically secure) is used for base selection in Shor's factoring algorithm. While this is a simulation and not actual cryptographic key generation, using non-CSPRNG for security tool demonstrations could be misleading. The `secrets` module is available (imported in `redteam_http_server.py`) but not used for this purpose.

### 6.4 LOW FINDINGS

#### [SEC-LOW-01] Error Messages Expose Internal Paths

**File**: `redteam_server.py`, line 97
**Severity**: LOW
**Description**: `return {"error": f"Tool not found: {tool_path}"}` exposes the full absolute path `/home/acid/Projects/NINE65/MANA-private/target/release/<tool_name>`.

#### [SEC-LOW-02] MCP Protocol Version Hardcoded

**File**: All three servers
**Severity**: LOW
**Description**: All servers hardcode `"protocolVersion": "2024-11-05"`. No negotiation occurs. If a client sends a different protocol version, the servers will not detect or handle the mismatch.

#### [SEC-LOW-03] Knowledge Graph File World-Readable

**File**: `grover_swarm_server.py`, lines 141-154
**Severity**: LOW
**Description**: `knowledge_graph.json` is written with default permissions (likely 0644 on most systems). No explicit permission restriction is applied to the data file, unlike the auth token file in `redteam_http_server.py` which gets `chmod 0o600`.

---

## 7. ANOMALY CATALOGUE

### 7.1 Architectural Anomalies

#### [ANOM-01] Three Independent Servers, No Integration

The repository contains three completely independent MCP servers with overlapping functionality:

| Capability | grover_swarm_server.py | redteam_http_server.py | redteam_server.py |
|-----------|----------------------|----------------------|-------------------|
| Grover simulation | Via knowledge graph | grover_attack() | grover_attack() |
| Shor factoring | -- | shor_factor() | shor_factor() |
| LWE analysis | -- | shadow_entropy_lwe() | shadow_entropy_lwe_attack() |
| K-Elimination | Via knowledge graph | k_elimination_demo() | -- |
| Scheme analysis | -- | analyze_scheme() | quantum_security_assessment() |
| Knowledge graph | YES | -- | -- |
| External binaries | -- | -- | YES (subprocess) |
| Codebase scanning | -- | -- | analyze_crypto_codebase() |
| FHE analysis | -- | gso_fhe_analysis() | -- |
| Transport | stdio | HTTP/HTTPS | stdio |
| Authentication | None | Bearer token | None |
| Tool prefix | grover_* | redteam_* | redshirt_* |

The Shor and Grover implementations are duplicated with slight variations between `redteam_http_server.py` and `redteam_server.py`. This is a maintenance burden and potential source of behavioral inconsistency.

#### [ANOM-02] Naming Inconsistency: "RedShirt" vs "RedTeam"

- `redteam_server.py` has docstring "RedShirt MCP Server" and uses `redshirt_*` tool prefix
- `redteam_http_server.py` uses `redteam_*` tool prefix
- The repository is named `redteam-mcp`
- README documents `redteam_*` tools only
- QUICKSTART documents `redteam_*` tools only

`redteam_server.py` appears to be an older version with the "RedShirt" branding that was superseded by `redteam_http_server.py` but kept in the repository.

#### [ANOM-03] Pre-Baked Initialize Response (Non-Standard MCP)

Both stdio servers (`grover_swarm_server.py` and `redteam_server.py`) send an initialize response immediately on startup (lines 905-918 and 1016-1029 respectively) without waiting for an initialize request. This is non-standard MCP behavior. The server should wait for the client to send `{"method": "initialize", ...}` before responding.

The HTTP server (`redteam_http_server.py`) handles this correctly via the `/initialize` GET endpoint, which responds only when requested.

#### [ANOM-04] JSON-RPC Framing Issues in stdio Servers

Both stdio servers modify the response dict in-place after `handle_request()`:
```python
response = handle_request(request)
response["jsonrpc"] = "2.0"
response["id"] = request.get("id", 0)
```

If `handle_request()` returns a dict that already has a `"content"` key (from a tools/call response), the resulting JSON-RPC object will have both `"content"` and `"jsonrpc"` at the top level. This is non-standard -- the `content` should be nested under `"result"`. The correct structure should be:
```json
{"jsonrpc": "2.0", "id": 1, "result": {"content": [...]}}
```
But the current code produces:
```json
{"content": [...], "jsonrpc": "2.0", "id": 1}
```

### 7.2 Code Quality Anomalies

#### [ANOM-05] Duplicate Utility Functions

The following functions are implemented independently in multiple files:

| Function | grover_swarm | redteam_http | redteam_server |
|----------|-------------|-------------|---------------|
| `log_request()` | line 446 | line 95 | line 58 |
| `check_rate_limit()` | line 462 | line 84 | line 74 |
| `gcd()` | -- | inline in shor_factor | line 123 |
| `mod_pow()` | -- | inline in shor_factor | line 130 |
| Shor's algorithm | -- | shor_factor (line 196) | shor_factor (line 197) |
| Grover simulation | -- | grover_attack (line 157) | grover_attack (line 368) |
| Shadow LWE | -- | shadow_entropy_lwe (line 346) | shadow_entropy_lwe_attack (line 488) |

No shared module or library extracts common functionality.

#### [ANOM-06] Inconsistent Rate Limits

| Server | Rate Limit |
|--------|-----------|
| grover_swarm_server.py | 100 requests/minute |
| redteam_http_server.py | 30 requests/minute |
| redteam_server.py | 10 requests/minute |

No justification is documented for the different limits.

#### [ANOM-07] Float Usage Throughout (QMNF Violation)

All three files use floating-point arithmetic extensively for mathematical computations:

- `grover_swarm_server.py` line 636: `import math` inside `get_toric_validation()`, uses `math.pi`, `math.sqrt`, float division
- `redteam_http_server.py`: `math.sqrt`, `math.sin`, `math.asin`, `math.log2`, float division, float comparisons throughout
- `redteam_server.py`: Same float functions, plus `n ** (1/k)` (float exponentiation) at line 221

This pervasive float usage contradicts the QMNF integer-only mandate but is likely acceptable for these simulation/demonstration tools that are not part of the core QMNF compute pipeline.

#### [ANOM-08] `grover_register_fact` weight Schema Mismatch

**File**: `grover_swarm_server.py`, line 716
The inputSchema declares `weight` as `{"type": "string"}` but the handler calls `int(arguments["weight"])` at line 838. The schema should declare `{"type": "integer"}` for consistency. Similarly, `confidence` in `grover_add_edge` is declared as string but cast to int.

#### [ANOM-09] `grover_launch_swarm` config parameters as strings

**File**: `grover_swarm_server.py`, lines 758-764
The config sub-schema declares `alpha`, `beta`, and `lambda` as strings:
```python
"alpha": {"type": "string"},
"beta": {"type": "string"},
"lambda": {"type": "string"}
```
But the handler casts them to int at lines 315-317:
```python
alpha = int(config.get("alpha", 150)) if config else 150
```

### 7.3 Missing Components

| Expected | Status | Notes |
|----------|--------|-------|
| `SECURITY_POLICY.md` | MISSING | Referenced in README.md line 271 and line 386 but not present |
| SSL certificates | MISSING | `localhost+2.pem` and `localhost+2-key.pem` not in repo (correct -- in .gitignore) |
| `.auth_token` | MISSING | Not in repo (correct -- in .gitignore) |
| External binaries | MISSING | 6 Rust binaries referenced by redteam_server.py are not in this build |
| `requirements.txt` | MISSING | No dependency file (acceptable since stdlib only) |
| Test suite | MISSING | No tests of any kind |
| `logs/` directory | MISSING | Created at runtime (correct) |

### 7.4 TODO/FIXME Markers

**None found** in any file. No explicit TODO, FIXME, HACK, or XXX comments exist.

### 7.5 Implicit TODOs (Inferred from Code)

| File | Line | Description |
|------|------|-------------|
| grover_swarm_server.py | 255 | Comment: `# Simple text matching for now (could be enhanced with embeddings)` -- query_recon uses naive substring matching |
| grover_swarm_server.py | 323 | `discovered_this_wave = max(0, len(recon["frontier"]) // wave_num)` -- swarm search is simulated, not a real algorithm execution |
| redteam_http_server.py | 668 | `self.send_header("Access-Control-Allow-Origin", origin or "*")` -- falls back to `*` when origin is empty string, which is overly permissive |

---

## SUMMARY OF FINDINGS

### Finding Counts by Severity

| Severity | Count | IDs |
|----------|-------|-----|
| CRITICAL | 4 | SEC-CRITICAL-01 through SEC-CRITICAL-04 |
| HIGH | 4 | SEC-HIGH-01 through SEC-HIGH-04 |
| MEDIUM | 5 | SEC-MED-01 through SEC-MED-05 |
| LOW | 3 | SEC-LOW-01 through SEC-LOW-03 |
| Anomaly | 9 | ANOM-01 through ANOM-09 |
| Dead Code | 4 items | SwarmNode class, parse_qs import, Timer import, tempfile import |

### Critical Risk Summary

1. **Arbitrary command execution** via `run_tool()` in `redteam_server.py` -- user-controlled args passed to subprocess without sanitization.
2. **Arbitrary filesystem read** via `analyze_crypto_codebase()` in `redteam_server.py` -- accepts any path, reads files recursively with no restrictions.
3. **Tarball path traversal** potential in `analyze_crypto_codebase()` -- tar member names not validated.
4. **No authentication** on stdio servers that have dangerous capabilities (filesystem read, binary execution).

### Architecture Assessment

The build contains three independent, non-integrated MCP servers with significant code duplication. `redteam_http_server.py` is the most mature and security-hardened of the three. `redteam_server.py` appears to be an older version ("RedShirt" branding) with the most dangerous capabilities (subprocess execution, filesystem scanning) and the least security controls (no auth, no input bounds, no timeout wrapping). `grover_swarm_server.py` is functionally distinct (knowledge graph) and the least security-concerning, though it persists data to disk without access controls.

### Files Audited

- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/grover_swarm_server.py`
- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/redteam_http_server.py`
- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/redteam_server.py`
- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/README.md`
- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/QUICKSTART.md`
- `/home/acid/Projects/Homomorphic_Armada/builds/22_redteam_mcp/.gitignore`

---

*End of forensic audit report.*
