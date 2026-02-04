---
title: "Dashboard Quick Start"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/DASHBOARD_QUICK_START.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Master Dashboard - Quick Start

## Installation (30 seconds)

```bash
cd ~/QMNF_System
./setup_dashboard.sh
```

## Start Dashboard (5 seconds)

```bash
python3 qmnf_master_dashboard.py
```

Then open: **http://localhost:5000**

## Command Line Options

```bash
# Custom port
python3 qmnf_master_dashboard.py --port 8080

# Network access
python3 qmnf_master_dashboard.py --host 0.0.0.0

# Debug mode
python3 qmnf_master_dashboard.py --debug

# Custom data directory
python3 qmnf_master_dashboard.py --data-dir /path/to/data

# Export data and exit
python3 qmnf_master_dashboard.py --export --export-dir /path/to/export
```

## Dashboard Pages

| Page | URL | Purpose |
|------|-----|---------|
| Overview | `/` | Main dashboard with all key metrics |
| Metrics | `/metrics` | Detailed metrics visualization |
| Benchmarks | `/benchmarks` | Performance testing |
| Services | `/services` | Service management |
| Learning | `/learning` | Learning system monitoring |
| Storage | `/storage` | Holographic storage status |
| AI/Agents | `/ai` | Multi-agent coordination |
| Diagnostics | `/diagnostics` | System health and troubleshooting |

## API Quick Reference

### Get Latest Metrics
```bash
curl http://localhost:5000/api/metrics/latest
```

### Run Benchmark
```bash
curl -X POST http://localhost:5000/api/benchmarks/run \
  -H "Content-Type: application/json" \
  -d '{"iterations": 1000, "name": "Test Benchmark"}'
```

### Start a Service
```bash
curl -X POST http://localhost:5000/api/services/learning_orchestrator/start
```

### Get System Health
```bash
curl http://localhost:5000/api/services/health
```

### Get System Overview
```bash
curl http://localhost:5000/api/status/overview
```

## Common Tasks

### Monitor System Resources
1. Open dashboard: `http://localhost:5000`
2. View CPU, Memory, Temperature in quick stats
3. Watch real-time charts update every 2 seconds

### Run Performance Benchmark
1. Go to Benchmarks page: `http://localhost:5000/benchmarks`
2. Click "Run Benchmark"
3. View results and compare with history

### Start All Services
1. Go to Services page: `http://localhost:5000/services`
2. Click "Start All" button
3. Monitor service status

### Export Metrics Data
```bash
python3 qmnf_master_dashboard.py --export
```
or via API:
```bash
curl -X POST http://localhost:5000/api/metrics/export \
  -H "Content-Type: application/json" \
  -d '{"filepath": "/tmp/metrics.json"}'
```

## Troubleshooting

### Port Already in Use
```bash
python3 qmnf_master_dashboard.py --port 8080
```

### Dependencies Missing
```bash
pip3 install -r dashboard_requirements.txt --user
```

### Dashboard Not Loading
1. Check if running: `ps aux | grep qmnf_master_dashboard`
2. Check logs: `cat ~/.qmnf_dashboard/dashboard.log`
3. Try debug mode: `python3 qmnf_master_dashboard.py --debug`

### No Metrics Showing
1. Open browser console (F12)
2. Check for JavaScript errors
3. Verify API: `curl http://localhost:5000/api/health`

## Data Location

- **Data Directory**: `~/.qmnf_dashboard/`
- **Benchmarks**: `~/.qmnf_dashboard/benchmarks/`
- **Service Configs**: `~/.qmnf_dashboard/services/`
- **Exports**: `~/.qmnf_dashboard/exports/`
- **Logs**: `~/.qmnf_dashboard/dashboard.log`

## Production Deployment

### With Gunicorn
```bash
pip3 install gunicorn
gunicorn -w 4 -b 0.0.0.0:5000 qmnf_master_dashboard:app
```

### With Systemd
```bash
systemctl --user enable qmnf-dashboard
systemctl --user start qmnf-dashboard
systemctl --user status qmnf-dashboard
```

## Key Features

✓ **Real-time monitoring** (1-2 second updates)
✓ **10 dashboard sections** (Overview, Metrics, Benchmarks, Services, Learning, Storage, AI, Diagnostics)
✓ **RESTful API** (Full programmatic access)
✓ **Service management** (Start/stop/restart services)
✓ **Performance benchmarking** (Component and system-wide tests)
✓ **Integer-only metrics** (All values use exact integer representations)
✓ **Historical tracking** (Trends and comparisons)
✓ **Data export** (JSON export for analysis)
✓ **Production-ready** (Suitable for development and production)

## Next Steps

1. ✓ Start dashboard
2. ✓ Check system health on Overview page
3. ✓ Run baseline benchmark
4. ✓ Start required services
5. ✓ Monitor real-time metrics
6. Read full guide: `QMNF_DASHBOARD_GUIDE.md`

## Help

- **Full Documentation**: `QMNF_DASHBOARD_GUIDE.md`
- **QMNF System Docs**: `CLAUDE.md`
- **Logs**: `~/.qmnf_dashboard/dashboard.log`
- **API Health**: `http://localhost:5000/api/health`

---

**QMNF Master Dashboard v1.0.0**
Production-Ready Control System for QMNF Integer-Only AI Architecture
