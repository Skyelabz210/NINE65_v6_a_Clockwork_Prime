# RedTeam MCP Server - Quick Reference Card

## One-Time Setup

```bash
# 1. Install mkcert
sudo apt install mkcert  # or: brew install mkcert

# 2. Generate SSL certs
cd ~/Projects/RedTeam/tools
mkcert -install
mkcert localhost 127.0.0.1 ::1

# 3. First run (generates auth token)
python3 redteam_http_server.py
# Ctrl+C after seeing banner
```

---

## Daily Usage

### Start Server
```bash
nohup python3 ~/Projects/RedTeam/tools/redteam_http_server.py > /tmp/redteam.log 2>&1 &
```

### Get Token
```bash
cat ~/Projects/RedTeam/tools/.auth_token
```

### Stop Server
```bash
pkill -f redteam_http_server.py
```

### Check Status
```bash
curl -sk https://localhost:8765/health
```

---

## Claude.ai Setup

1. Settings > MCP Servers > Add
2. **URL**: `https://localhost:8765`
3. **Token**: `cat ~/.../tools/.auth_token`

---

## Tools Quick Reference

| Tool | Example Call |
|------|--------------|
| Grover | `{"name":"redteam_grover","arguments":{"search_space_bits":128}}` |
| Shor | `{"name":"redteam_shor","arguments":{"n":3233}}` |
| Order | `{"name":"redteam_order","arguments":{"a":2,"n":101}}` |
| Shadow LWE | `{"name":"redteam_shadow_lwe","arguments":{"n":256,"q":3329}}` |
| Analyze | `{"name":"redteam_analyze","arguments":{"scheme":"kyber"}}` |
| K-Elim | `{"name":"redteam_k_elimination","arguments":{"value":5000,"modulus":97,"anchor":101}}` |
| GSO-FHE | `{"name":"redteam_gso_fhe","arguments":{"depth":50}}` |

---

## Test Commands

```bash
TOKEN=$(cat ~/Projects/RedTeam/tools/.auth_token)

# Test Grover
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"redteam_grover","arguments":{"search_space_bits":128}}'

# Test Shor (factor 15)
curl -sk -X POST https://localhost:8765/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"redteam_shor","arguments":{"n":15}}'
```

---

## Endpoints

| Endpoint | Auth | Method | Purpose |
|----------|------|--------|---------|
| `/` | No | GET | Server info |
| `/health` | No | GET | Health check |
| `/tools` | No | GET | List tools |
| `/tools/call` | Yes | POST | Execute tool |
| `/initialize` | No | GET | MCP handshake |
| `/metrics` | Yes | GET | Usage stats |

---

## Files

```
~/Projects/RedTeam/tools/
├── redteam_http_server.py  # Main server
├── .auth_token             # Auth token (auto-generated)
├── localhost+2.pem         # SSL cert
├── localhost+2-key.pem     # SSL key
└── logs/                   # Audit logs
```

---

## Troubleshooting

| Issue | Fix |
|-------|-----|
| Port in use | `fuser -k 8765/tcp` |
| No SSL | Run `mkcert localhost 127.0.0.1 ::1` |
| Auth failed | Check token: `cat .auth_token` |
| Connection refused | Start server first |

---

## Slash Command

```
/redteam start   # Start server
/redteam stop    # Stop server
/redteam status  # Check status
/redteam token   # Show token
```

---

**Server**: https://localhost:8765
**Repo**: github.com/Skyelabz210/redteam-mcp (private)
**Version**: 3.0.0
