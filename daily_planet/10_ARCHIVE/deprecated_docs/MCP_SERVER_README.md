---
title: "Mcp Server Readme"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/MCP_SERVER_README.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF MCP Server - Complete Documentation

## Overview

The QMNF MCP (Model Context Protocol) Server exposes all QMNF system functionality through standardized tools that AI assistants like Claude can use directly.

**Status:** Phase 1 Implementation
**Tools Available:** 10+ (expanding to 120+)
**Version:** 1.0.0

---

## Quick Start

### 1. Install Dependencies

```bash
# Install MCP SDK
pip install mcp

# Install QMNF dependencies (if not already installed)
pip install numpy scipy psutil

# Build Rust components (if not already built)
cd /home/user/QMNF_System
python3 setup.py develop
```

### 2. Configure Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or
`%APPDATA%/Claude/claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "qmnf-system": {
      "command": "python3",
      "args": ["/home/user/QMNF_System/qmnf_mcp_server.py"],
      "env": {
        "PYTHONPATH": "/home/user/QMNF_System"
      }
    }
  }
}
```

### 3. Test the Server

```bash
# Test locally
python3 qmnf_mcp_server.py

# Check logs
tail -f /tmp/qmnf_mcp_server.log
```

### 4. Use in Claude Desktop

Restart Claude Desktop, then ask:
```
Can you use the QMNF system to add 1/2 and 1/3?
```

Claude will use `math_rational_add(1, 2, 1, 3)` → `{numerator: 5, denominator: 6}`

---

## Available Tools

### Category 1: Core Mathematics (10 tools, expanding to 20)

#### `math_rational_create(numerator, denominator)`
Create a rational number.

**Example:**
```python
math_rational_create(3, 4)  # Returns {numerator: 3, denominator: 4}
```

#### `math_rational_add(r1_num, r1_den, r2_num, r2_den)`
Add two rational numbers.

**Example:**
```python
math_rational_add(1, 2, 1, 3)  # Returns {numerator: 5, denominator: 6}
```

#### `math_rational_subtract(r1_num, r1_den, r2_num, r2_den)`
Subtract rational numbers (r1 - r2).

#### `math_rational_multiply(r1_num, r1_den, r2_num, r2_den)`
Multiply rational numbers.

#### `math_rational_divide(r1_num, r1_den, r2_num, r2_den)`
Divide rational numbers (r1 / r2).

#### `math_gcd(a, b)`
Compute greatest common divisor.

**Example:**
```python
math_gcd(48, 18)  # Returns {result: 6}
```

#### `math_rational_compare(r1_num, r1_den, r2_num, r2_den)`
Compare two rationals. Returns -1, 0, or 1.

### Category 2: System Monitoring (2 tools, expanding to 26)

#### `system_get_latest_metrics()`
Get current system metrics.

**Returns:**
- system: CPU, memory, disk
- performance: ops/sec, latency
- energy: power, battery
- stability: phi-coherence, escape effectiveness
- learning: cycle count, convergence
- storage: capacity, compression
- ai: active agents, coordination

#### `system_list_services()`
List all managed services (ollama, learning, tensor, escape, storage, agents).

---

## Planned Tool Categories

### Phase 2: Cryptography (15 tools)
- `crypto_gaussian_sample()` - Discrete Gaussian sampling
- `crypto_fhe_encrypt()` - Fully homomorphic encryption
- `crypto_fhe_decrypt()` - FHE decryption
- `crypto_fhe_add()` - FHE addition
- `crypto_fhe_multiply()` - FHE multiplication
- `crypto_ntt_forward()` - Number theoretic transform
- `crypto_ntt_inverse()` - Inverse NTT
- And 8 more...

### Phase 3: Neural Networks (20 tools)
- `neural_gso_optimize()` - GSO optimization
- `neural_tensor_process()` - Tensor processing
- `neural_gradient_compute()` - Gradient computation
- `neural_layer_forward()` - Forward pass
- `neural_layer_backward()` - Backpropagation
- And 15 more...

### Phase 4: Learning System (15 tools)
- `learning_schedule_task()` - Schedule learning task
- `learning_consolidate()` - Trigger consolidation
- `learning_get_patterns()` - Retrieve learned patterns
- `learning_set_hyperparams()` - Configure hyperparameters
- And 11 more...

### Phase 5: Storage Operations (8 tools)
- `storage_cosmos_read()` - Read from COSMOS
- `storage_cosmos_write()` - Write to COSMOS
- `storage_holodrive_encode()` - HoloDrive encoding
- `storage_holodrive_decode()` - HoloDrive decoding
- And 4 more...

### Phase 6: Agent Coordination (10 tools)
- `agent_create()` - Create new agent
- `agent_assign_task()` - Assign task to agent
- `agent_get_status()` - Get agent status
- `agent_synchronize()` - Sync agent states
- And 6 more...

### Phase 7: Escape System (6 tools)
- `escape_modulate()` - Apply escape modulation
- `escape_set_parameters()` - Configure escape system
- `escape_get_chaos_level()` - Get chaos measurement
- And 3 more...

### Phase 8: Consciousness (4 tools)
- `consciousness_get_level()` - Get consciousness level
- `consciousness_workspace_query()` - Query global workspace
- `consciousness_update_state()` - Update consciousness state
- `consciousness_get_awareness()` - Get awareness metrics

---

## Resources

### `qmnf://system/status`
Get comprehensive system status including loaded modules, errors, and tool count.

### `qmnf://docs/quickstart`
Get quick start guide and usage examples.

---

## Prompts

### `math_assistant`
Activates mathematical problem-solving mode with QMNF tools.

**Usage in Claude:**
```
Use the math_assistant prompt
```

---

## Architecture

### Import Safety
The server uses safe imports with fallback behavior:
- If Rust bindings unavailable, tools return helpful errors
- Partial functionality maintained even if some modules fail
- Detailed logging of import issues

### Error Handling
All tools return structured errors:
```json
{
  "error": "Description of what went wrong",
  "details": "Additional context"
}
```

### Integer-Only Guarantee
All mathematical operations use exact rational arithmetic:
- No floating-point operations
- All percentages as basis points (1-10000 = 0.01-100%)
- All measurements in standard SI units

---

## Troubleshooting

### Issue: "QMNFRational not available"

**Cause:** Rust bindings (hcvlang_pyo3) not built

**Solution:**
```bash
cd /home/user/QMNF_System
# Fix network access if needed
cargo build --release --features python
# Or use setup.py
python3 setup.py develop
```

See `SYSTEM_RECOVERY_GUIDE.md` for detailed recovery steps.

### Issue: "ModuleNotFoundError: No module named 'mcp'"

**Solution:**
```bash
pip install mcp
```

### Issue: Server not appearing in Claude Desktop

**Check:**
1. Restart Claude Desktop completely
2. Verify config file location and syntax
3. Check logs: `tail -f /tmp/qmnf_mcp_server.log`
4. Test server directly: `python3 qmnf_mcp_server.py`

---

## Development

### Adding New Tools

1. Define tool function with `@app.tool()` decorator
2. Add comprehensive docstring (becomes tool description)
3. Use type hints for parameters
4. Return Dict[str, Any] with structured data
5. Handle errors gracefully

**Example:**
```python
@app.tool()
async def my_new_tool(param1: int, param2: str) -> Dict[str, Any]:
    \"\"\"
    Brief description of what the tool does.

    Args:
        param1: Description of param1
        param2: Description of param2

    Returns:
        Description of return value
    \"\"\"
    try:
        # Implementation
        result = do_something(param1, param2)
        return {"success": True, "result": result}
    except Exception as e:
        return {"error": str(e)}
```

### Testing Tools

```bash
# Unit test individual tools
python3 -m pytest tests/test_mcp_tools.py

# Integration test
python3 tools/test_mcp_integration.py

# Manual test in Claude Desktop
# Just ask Claude to use the tool
```

---

## Performance

- Tool invocation overhead: ~10-50ms
- Rational arithmetic: ~100-500ns per operation
- Network latency: Depends on MCP transport
- Batch operations recommended for multiple calculations

---

## Security

**Current Status:** Development mode
- No authentication
- No rate limiting
- No input sanitization

**For Production:**
Add authentication, input validation, rate limiting, and audit logging.

---

## Roadmap

### Phase 1 (Current)
- [x] Core math operations (8 tools)
- [x] System monitoring (2 tools)
- [x] MCP server infrastructure
- [x] Configuration and documentation

### Phase 2 (Week 1-2)
- [ ] Complete math operations (20 total)
- [ ] Add geometry tools
- [ ] Add modular arithmetic tools
- [ ] Expand system monitoring (26 total)

### Phase 3 (Week 3-4)
- [ ] Cryptography tools (15 tools)
- [ ] Neural network tools (20 tools)
- [ ] Learning system tools (15 tools)

### Phase 4 (Week 5-6)
- [ ] Storage tools (8 tools)
- [ ] Agent coordination (10 tools)
- [ ] Escape system (6 tools)
- [ ] Consciousness integration (4 tools)

### Phase 5 (Week 7+)
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Production hardening
- [ ] Security implementation

---

## License

Part of the QMNF System. See main repository LICENSE.

---

## Support

- Issues: GitHub Issues
- Documentation: This file + inline tool docstrings
- Logs: `/tmp/qmnf_mcp_server.log`
- Recovery Guide: `SYSTEM_RECOVERY_GUIDE.md`

---

**Last Updated:** 2025-11-06
**Status:** Phase 1 Implementation
**Next:** Phase 2 tool expansion
