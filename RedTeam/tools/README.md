# RedTeam MCP Server

**NINE65 Cryptanalysis Toolkit** - A security-hardened Model Context Protocol (MCP) server for quantum cryptanalysis and FHE innovation demonstrations.

[![Version](https://img.shields.io/badge/version-3.0.0-blue.svg)]()
[![Python](https://img.shields.io/badge/python-3.10+-green.svg)]()
[![License](https://img.shields.io/badge/license-Proprietary-red.svg)]()

---

## Overview

RedTeam MCP Server provides Claude.ai (and other MCP-compatible clients) with cryptanalysis tools for:

- **Quantum Attack Simulation** - Grover's search, Shor's factoring
- **Post-Quantum Cryptography Analysis** - Lattice-based scheme evaluation
- **NINE65 Innovation Demos** - K-Elimination, GSO-FHE, Shadow Entropy

All tools run locally with no external network exposure.

---

## Features

### Security-Hardened Design

| Feature | Description |
|---------|-------------|
| **HTTPS Only** | TLS 1.2+ with mkcert local certificates |
| **Secure Tokens** | `secrets.token_hex(32)` with persistent storage |
| **Input Validation** | Bounds checking on all parameters |
| **Rate Limiting** | 30 requests/minute per client |
| **Request Limits** | 64KB max payload, 30s execution timeout |
| **Audit Logging** | Full request logging with IP, timing, args |
| **CORS Restricted** | Only claude.ai and localhost allowed |

### Available Tools (7)

| Tool | Description | Use Case |
|------|-------------|----------|
| `redteam_grover` | Toric Grover 2-amplitude simulation | Symmetric key security analysis |
| `redteam_shor` | BSGS order finding + Shor reduction | RSA/ECC vulnerability assessment |
| `redteam_order` | Standalone multiplicative order | Number theory research |
| `redteam_shadow_lwe` | Shadow Entropy attack analysis | Lattice scheme side-channel risk |
| `redteam_analyze` | Cryptographic scheme assessment | Security posture evaluation |
| `redteam_k_elimination` | K-Elimination exact division demo | RNS innovation demonstration |
| `redteam_gso_fhe` | GSO-FHE parameter analysis | Bootstrap-free FHE planning |

### MCP Protocol Compliance

- `/initialize` - Protocol handshake
- `/tools` - Tool discovery
- `/tools/call` - Tool execution
- `/health` - Health check
- `/metrics` - Usage statistics (auth required)
- `/prompts`, `/resources` - Empty (per spec)

---

## Installation

### Prerequisites

- Python 3.10+
- mkcert (for HTTPS certificates)

### Setup

```bash
# Clone repository
git clone git@github.com:Skyelabz210/redteam-mcp.git
cd redteam-mcp

# Install mkcert (if not installed)
# macOS: brew install mkcert
# Linux: apt install mkcert or download from https://github.com/FiloSottile/mkcert

# Generate local SSL certificates
mkcert -install
mkcert localhost 127.0.0.1 ::1
mv localhost+2*.pem ./

# Start server
python3 redteam_http_server.py
```

### First Run

On first run, the server:
1. Generates a secure auth token (saved to `.auth_token`)
2. Creates log directory (`logs/`)
3. Writes PID file (`/tmp/redteam.pid`)

---

## Usage

### Start Server

```bash
python3 redteam_http_server.py
```

Output:
```
╔══════════════════════════════════════════════════════════════╗
║          REDTEAM HTTPS MCP SERVER v3.0.0                     ║
╠══════════════════════════════════════════════════════════════╣
║  Port: 8765                                                  ║
║  SSL:  ENABLED (mkcert)                                      ║
║  PID:  12345                                                 ║
╠══════════════════════════════════════════════════════════════╣
║  Tools: 7 available                                          ║
║  Rate:  30 requests/minute                                   ║
╚══════════════════════════════════════════════════════════════╝
```

### Connect to Claude.ai

1. Go to Claude.ai Settings > MCP Servers
2. Add new server:
   - **URL**: `https://localhost:8765`
   - **Token**: Contents of `.auth_token` file

### API Examples

```bash
# Get auth token
TOKEN=$(cat .auth_token)

# Health check
curl -sk https://localhost:8765/health

# List tools
curl -sk https://localhost:8765/tools

# Grover's algorithm (128-bit search space)
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "redteam_grover", "arguments": {"search_space_bits": 128}}'

# Factor a semiprime
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "redteam_shor", "arguments": {"n": 3233}}'

# K-Elimination demo
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "redteam_k_elimination", "arguments": {"value": 5000, "modulus": 97, "anchor": 101}}'

# Analyze Kyber security
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "redteam_analyze", "arguments": {"scheme": "kyber", "key_bits": 256}}'
```

---

## Tool Reference

### redteam_grover

Simulates Grover's quantum search algorithm using Toric 2-amplitude tracking.

**Parameters:**
- `search_space_bits` (int, 1-1024): Search space size as 2^n

**Returns:**
- Optimal iterations, final probability, post-quantum security bits

**Example:**
```json
{
  "algorithm": "Toric Grover (2-amplitude)",
  "search_space_bits": 128,
  "optimal_iterations": "~2^64",
  "post_quantum_security": 64,
  "recommendation": "AES-256 for 128-bit post-quantum security"
}
```

### redteam_shor

Factors integers using Shor's algorithm with Baby-Step Giant-Step order finding.

**Parameters:**
- `n` (int, required): Number to factor (2 to 10^15)
- `attempts` (int, 1-100): Max factoring attempts

**Returns:**
- Factors, method used, order found (if applicable)

### redteam_shadow_lwe

Analyzes the Shadow Entropy side-channel attack on LWE-based cryptography.

**Parameters:**
- `n` (int, 16-4096): LWE dimension
- `q` (int): LWE modulus

**Returns:**
- Attack complexity, samples needed, affected schemes, mitigations

### redteam_k_elimination

Demonstrates K-Elimination exact RNS division algorithm.

**Parameters:**
- `value` (int): Value to process
- `modulus` (int): Main modulus M
- `anchor` (int): Anchor modulus A (must be coprime with M)

**Note:** For exact reconstruction, value must be < M * A

### redteam_gso_fhe

Analyzes GSO-FHE bootstrap-free homomorphic encryption parameters.

**Parameters:**
- `depth` (int, 1-1000): Target circuit depth
- `noise_budget` (int, 10-10000): Initial noise budget in bits

**Returns:**
- Achievable depth, performance comparison with bootstrapping

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REDTEAM_AUTH_TOKEN` | (generated) | Override auth token |
| `REDTEAM_PORT` | 8765 | Server port |

### Files

| File | Purpose |
|------|---------|
| `.auth_token` | Persistent auth token (chmod 600) |
| `localhost+2.pem` | SSL certificate |
| `localhost+2-key.pem` | SSL private key |
| `logs/audit_YYYYMMDD.log` | Audit logs |

---

## Security Policy

### Authorized Use Only

This toolkit is for:
- Self-testing of owned systems
- Authorized penetration testing
- CTF competitions
- Security research with ethics approval
- Educational purposes

### Prohibited Uses

- Attacking systems without authorization
- Cryptocurrency theft or financial fraud
- Breaking encryption on private communications
- Creating malware or ransomware

See `SECURITY_POLICY.md` for full policy.

---

## Architecture

```
redteam_http_server.py
├── Configuration
│   ├── Token management (secrets.token_hex)
│   ├── Rate limiting (30 req/min)
│   └── Input validation
├── Tool Implementations
│   ├── grover_attack() - Toric 2-amplitude
│   ├── shor_factor() - BSGS order finding
│   ├── order_finding() - Standalone BSGS
│   ├── shadow_entropy_lwe() - Side-channel analysis
│   ├── analyze_scheme() - Crypto assessment
│   ├── k_elimination_demo() - RNS division
│   └── gso_fhe_analysis() - FHE parameters
├── MCP Protocol Handler
│   ├── GET endpoints (/, /health, /tools, /initialize)
│   └── POST /tools/call (authenticated)
└── Server Lifecycle
    ├── Signal handlers (graceful shutdown)
    ├── PID file management
    └── Startup validation
```

---

## Development

### Adding a New Tool

1. Implement the tool function with `@timeout_wrapper(30)` decorator
2. Add input validation using `validate_int()` helper
3. Register in `TOOLS` dictionary with schema
4. Test with curl

```python
@timeout_wrapper(TOOL_TIMEOUT_SECONDS)
def my_new_tool(param1: int, param2: str = "default") -> dict:
    valid, result = validate_int(param1, "param1", 1, 1000)
    if not valid:
        return {"error": result}
    # Implementation
    return {"result": "..."}

TOOLS["redteam_my_tool"] = {
    "description": "My new tool description",
    "inputSchema": {
        "type": "object",
        "properties": {
            "param1": {"type": "integer", "description": "..."},
            "param2": {"type": "string", "default": "default"}
        },
        "required": ["param1"]
    },
    "handler": my_new_tool
}
```

### Running Tests

```bash
# Health check
curl -sk https://localhost:8765/health

# Test all tools
TOKEN=$(cat .auth_token)
for tool in grover shor order shadow_lwe analyze k_elimination gso_fhe; do
  echo "Testing redteam_$tool..."
  curl -sk -X POST https://localhost:8765/tools/call \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"name\": \"redteam_$tool\", \"arguments\": {}}" | head -c 100
  echo ""
done
```

---

## Changelog

### v3.0.0 (2026-01-21)
- Secure token generation with persistent storage
- Input validation on all parameters
- Constant-time token comparison
- CORS restricted to claude.ai/localhost
- Request size and timeout limits
- Enhanced audit logging
- MCP protocol compliance (initialize, prompts, resources)
- New tools: order, k_elimination, gso_fhe
- Graceful shutdown with signal handlers
- PID file management

### v2.1 (2026-01-21)
- HTTPS support via mkcert
- Localhost-only binding

### v2.0 (2026-01-21)
- Initial HTTP MCP server
- Basic tools: grover, shor, shadow_lwe, analyze

---

## License

Proprietary - NINE65 Security Research

---

## Contact

For authorized access requests, see `SECURITY_POLICY.md`.
