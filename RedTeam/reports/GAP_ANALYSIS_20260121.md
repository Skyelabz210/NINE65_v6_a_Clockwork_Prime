# RedTeam MCP Server Gap Analysis

**Date**: 2026-01-21
**Analyst**: Claude Code
**Version Analyzed**: v2.1

---

## Executive Summary

Gap analysis of `redteam_http_server.py` identified **23 gaps** across 5 categories:
- **Critical (Security)**: 6 gaps
- **High (Protocol)**: 5 gaps
- **Medium (Features)**: 7 gaps
- **Low (Operational)**: 5 gaps

---

## 1. Security Gaps (CRITICAL)

### S1. Predictable Auth Token
**Current**: Token derived from static seed `hashlib.sha256(b"redteam-nine65-2026")`
**Risk**: Attacker can predict token without access to server
**Fix**: Use `secrets.token_hex(32)` with persistent storage or env var

### S2. No Input Validation
**Current**: Tool arguments passed directly to functions
**Risk**: Integer overflow, DoS via large values, negative numbers
**Fix**: Validate all inputs with bounds checking

### S3. Wildcard CORS
**Current**: `Access-Control-Allow-Origin: *`
**Risk**: Cross-origin requests from any domain
**Fix**: Restrict to `https://claude.ai` and `https://localhost:8765`

### S4. No Request Timeout
**Current**: Tool execution runs until completion
**Risk**: DoS via computationally expensive parameters
**Fix**: Add timeout wrapper (max 30 seconds)

### S5. Auth After Parsing
**Current**: Body parsed before auth check in do_POST
**Risk**: Resource exhaustion via large payloads
**Fix**: Check auth and Content-Length first

### S6. Audit Log Missing Context
**Current**: Only logs action name and result hash
**Risk**: Insufficient forensics data
**Fix**: Log client IP, full arguments (sanitized), timing

---

## 2. MCP Protocol Compliance Gaps (HIGH)

### P1. Missing Initialize Endpoint
**MCP Spec**: `/initialize` handshake required
**Current**: Not implemented
**Fix**: Add `/initialize` returning capabilities

### P2. Missing JSON-RPC Format
**MCP Spec**: Responses should include `jsonrpc: "2.0"`, `id`, `result`
**Current**: Raw JSON responses
**Fix**: Wrap responses in JSON-RPC envelope

### P3. Missing Prompts Endpoint
**MCP Spec**: `/prompts` lists available prompt templates
**Current**: Not implemented
**Fix**: Add prompts endpoint (can be empty list)

### P4. Missing Resources Endpoint
**MCP Spec**: `/resources` lists available resources
**Current**: Not implemented
**Fix**: Add resources endpoint (can be empty list)

### P5. No Request ID Tracking
**MCP Spec**: Each request should have trackable ID
**Current**: Not implemented
**Fix**: Accept and return request IDs

---

## 3. Feature Gaps (MEDIUM)

### F1. Missing K-Elimination Tool
**Available in Coq proofs**: KElimination.v
**Current**: Not exposed via MCP
**Fix**: Add `redteam_k_elimination` tool

### F2. Missing GSO-FHE Analysis
**Available in Coq proofs**: GSOFHE.v
**Current**: Not exposed via MCP
**Fix**: Add `redteam_gso_fhe` tool

### F3. Missing Discrete Log Tool
**Use case**: ECDLP analysis
**Current**: Only Shor for factoring
**Fix**: Add `redteam_discrete_log` for small DLP

### F4. Missing Health Check
**Use case**: Monitoring and status
**Current**: Root `/` returns server info
**Fix**: Add dedicated `/health` endpoint

### F5. Missing Metrics Endpoint
**Use case**: Usage statistics
**Current**: Not available
**Fix**: Add `/metrics` endpoint

### F6. Missing Order Finding Standalone
**Available**: BSGS implementation exists
**Current**: Only used inside Shor
**Fix**: Expose `redteam_order_finding` separately

### F7. Missing Lattice Analysis
**Use case**: LWE parameter estimation
**Current**: Only Shadow Entropy attack info
**Fix**: Add lattice security estimator

---

## 4. Operational Gaps (LOW)

### O1. No Graceful Shutdown
**Current**: Ctrl+C abruptly terminates
**Fix**: Add signal handler for clean shutdown

### O2. No PID File
**Current**: No way to track running instance
**Fix**: Write PID to `/tmp/redteam.pid`

### O3. No Log Rotation
**Current**: Single daily log file, grows unbounded
**Fix**: Rotate logs at 10MB or use logrotate config

### O4. Outdated Comments
**Current**: Still references ngrok
**Fix**: Update documentation to reflect localhost-only

### O5. No Startup Validation
**Current**: No pre-flight checks
**Fix**: Validate certs exist, port available, etc.

---

## 5. Documentation Gaps

### D1. No OpenAPI Spec
**Fix**: Generate openapi.yaml for API documentation

### D2. No Usage Examples
**Fix**: Add curl examples for each tool

---

## Implementation Priority

| Priority | ID | Description | Effort |
|----------|----|----|--------|
| 1 | S1 | Secure token generation | Low |
| 2 | S2 | Input validation | Medium |
| 3 | S3 | CORS restriction | Low |
| 4 | P1-P4 | MCP protocol compliance | Medium |
| 5 | S4 | Request timeout | Low |
| 6 | F4 | Health check | Low |
| 7 | O1-O2 | Graceful shutdown + PID | Low |
| 8 | F1-F3 | Additional tools | High |
| 9 | S6 | Enhanced logging | Medium |

---

## Refinement Plan

1. **Phase 1 (Immediate)**: Security fixes S1-S5, Protocol P1-P4
2. **Phase 2 (Short-term)**: Features F1-F4, Operations O1-O2
3. **Phase 3 (Long-term)**: Remaining features and documentation

---

*Generated by RedTeam gap analysis - 2026-01-21*
