---
title: "Qmnf Dashboard Guide"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/QMNF_DASHBOARD_GUIDE.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Master Dashboard - Comprehensive Guide

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Architecture](#architecture)
4. [Installation](#installation)
5. [Quick Start](#quick-start)
6. [Dashboard Sections](#dashboard-sections)
7. [API Reference](#api-reference)
8. [Configuration](#configuration)
9. [Deployment](#deployment)
10. [Troubleshooting](#troubleshooting)
11. [Development](#development)

---

## Overview

The QMNF Master Dashboard is a production-ready monitoring and control system for the QMNF (Quantum-Modular Numerical Framework) integer-only AI architecture. It provides unified access to all system components, real-time metrics, performance benchmarking, and service management.

### Key Principles

- **Integer-Only Mathematics**: All metrics use integer representations (no floats)
- **Real-Time Monitoring**: Live updates every 1-2 seconds
- **Comprehensive Coverage**: Monitors all QMNF subsystems
- **Production Ready**: Suitable for development and production environments
- **API-First Design**: RESTful API for programmatic access

---

## Features

### 1. Core System Endpoints

- **Learning Orchestrator**: Monitor and control learning cycles
- **Tensor Processing**: Track tensor pipeline throughput
- **Escape System**: View modulation metrics and effectiveness
- **HCVLang Primitives**: Performance monitoring (when implemented)
- **Ollama Integration**: LLM service status and management

### 2. Utility Access & Controls

- **Start/Stop Services**: Individual or bulk service control
- **Configure Parameters**: Runtime configuration updates
- **Manage Connections**: Service connectivity management
- **Resource Allocation**: System resource monitoring

### 3. Benchmarking Suite

- **Component Benchmarks**: Test individual subsystems
- **Comparative Metrics**: Historical performance comparison
- **Throughput Testing**: Operations per second tracking
- **Latency Analysis**: P95/P99 latency measurements
- **Memory Profiling**: Memory usage per operation

### 4. Performance Metrics

- **CPU Usage**: Real-time CPU utilization (basis points)
- **Memory**: Available RAM and usage patterns
- **Disk I/O**: Storage system performance
- **Network**: (Future) Network throughput
- **Queue Depths**: Processing queue monitoring

### 5. Energy Monitoring

- **Power Consumption**: Estimated power draw (milliwatts)
- **Battery Status**: Battery level and time remaining
- **Thermal Metrics**: CPU/GPU temperatures (millicelsius)
- **Thermal Throttling**: Detect performance degradation
- **Efficiency Ratings**: Power efficiency scoring

### 6. Stability Metrics

- **Φ-Coherence**: Golden ratio coherence tracking
- **Escape Effectiveness**: Chaos escape system performance
- **Error Rates**: System error counting
- **Health Checks**: Overall system health scoring
- **Anomaly Detection**: Statistical anomaly tracking

### 7. Learning Levels

- **Cycle Tracking**: Total learning cycles executed
- **Convergence Metrics**: Learning convergence rate
- **Pattern Recognition**: Patterns identified count
- **Learning Curves**: Historical learning progress
- **Effectiveness Scores**: Learning effectiveness rating

### 8. Holo Levels (Holographic Storage)

- **Wasan HD Status**: Holographic drive monitoring
- **Storage Allocation**: Page allocation tracking
- **Compression Ratios**: Data compression effectiveness
- **Retrieval Times**: Average retrieval latency
- **Redundancy Checks**: Data integrity verification

### 9. AI Level

- **Agent Coordination**: Multi-agent sync status
- **Active Agents**: Current agent count
- **Consciousness Tracking**: System consciousness level
- **Decision Metrics**: Decision-making latency
- **Task Completion**: Agent task statistics

### 10. Diagnostics

- **System Health**: Overall health assessment
- **Error Logs**: Error tracking and history
- **Performance Profiling**: Detailed performance analysis
- **Alert System**: Configurable alert thresholds
- **Recovery Mechanisms**: Auto-recovery status

---

## Architecture

### Backend Components

```
dashboard/
├── __init__.py                 # Package initialization
├── metrics_collector.py        # Real-time metrics collection
├── benchmark_runner.py         # Benchmark execution engine
├── service_controller.py       # Service management
├── api_routes.py              # RESTful API endpoints
├── static/                    # Static assets
│   ├── css/dashboard.css      # Custom styles
│   └── js/dashboard.js        # Client-side logic
└── templates/                 # HTML templates
    ├── base.html              # Base template
    ├── index.html             # Main dashboard
    ├── metrics.html           # Metrics page
    ├── benchmarks.html        # Benchmarks page
    ├── services.html          # Services control
    ├── diagnostics.html       # Diagnostics page
    ├── learning.html          # Learning system
    ├── storage.html           # Storage monitoring
    └── ai.html                # AI/Agent coordination
```

### Frontend Stack

- **Flask**: Web framework
- **Bootstrap 5**: UI components
- **Chart.js**: Real-time charts
- **Plotly**: Advanced visualizations
- **Font Awesome**: Icons
- **jQuery**: DOM manipulation

### Data Flow

```
User Browser
    ↓
Flask Routes (qmnf_master_dashboard.py)
    ↓
API Blueprint (dashboard/api_routes.py)
    ↓
Backend Services
    ├── MetricsCollector (collects system metrics)
    ├── BenchmarkRunner (executes benchmarks)
    └── ServiceController (manages services)
    ↓
QMNF System Components
```

---

## Installation

### Prerequisites

- Python 3.8 or higher
- pip (Python package manager)
- 2GB+ available RAM
- Modern web browser (Chrome, Firefox, Safari, Edge)

### Automatic Installation

```bash
# Navigate to QMNF directory
cd ~/QMNF_System

# Run setup script
./setup_dashboard.sh
```

### Manual Installation

```bash
# Install dependencies
pip3 install -r dashboard_requirements.txt --user

# Create data directories
mkdir -p ~/.qmnf_dashboard/{benchmarks,services,exports}

# Test imports
python3 -c "from dashboard.metrics_collector import MetricsCollector; print('OK')"
```

### Required Packages

- Flask 3.0.0
- flask-cors 4.0.0
- psutil 5.9.6
- requests 2.31.0
- gunicorn 21.2.0 (optional, for production)

---

## Quick Start

### Starting the Dashboard

```bash
# Basic start (localhost only)
python3 qmnf_master_dashboard.py

# Start on all interfaces
python3 qmnf_master_dashboard.py --host 0.0.0.0

# Custom port
python3 qmnf_master_dashboard.py --port 8080

# Debug mode
python3 qmnf_master_dashboard.py --debug

# Custom data directory
python3 qmnf_master_dashboard.py --data-dir /path/to/data
```

### Accessing the Dashboard

Open your web browser to:
- **Local**: http://localhost:5000
- **Network**: http://YOUR_IP:5000

### First Steps

1. **Check System Health**: View the health indicator in the top-right
2. **Start Services**: Use the Services page to start required services
3. **Run Benchmark**: Execute a baseline benchmark for comparison
4. **Monitor Metrics**: Watch real-time metrics on the Overview page

---

## Dashboard Sections

### 1. Overview Page (/)

**Purpose**: Central monitoring hub with high-level system status

**Features**:
- Quick stats cards (CPU, Memory, Health, Temperature)
- Real-time resource charts
- Service status table
- Learning system progress
- AI agent metrics
- Storage overview
- Stability indicators
- Latest benchmark results

**Auto-Refresh**: Every 2 seconds

### 2. Metrics Page (/metrics)

**Purpose**: Detailed metrics visualization and analysis

**Categories**:
- System Resources (CPU, Memory, Disk, Swap)
- Performance (Ops/sec, Latency, Throughput)
- Energy (Power, Battery, Thermal)
- Stability (Φ-Coherence, Health, Errors)
- Learning (Cycles, Convergence, Patterns)
- Storage (Capacity, Compression, Retrieval)
- AI (Agents, Coordination, Consciousness)

**Features**:
- Historical trend charts
- Metric export to JSON
- Summary statistics
- Customizable time ranges

### 3. Benchmarks Page (/benchmarks)

**Purpose**: Performance testing and analysis

**Benchmark Categories**:
- Rational Arithmetic
- Geometric Operations
- Learning System
- Tensor Processing
- Escape System
- Storage System
- Agent Coordination
- Full System Integration

**Features**:
- On-demand benchmark execution
- Historical comparison
- Performance trends
- Category-specific analysis
- Export results

### 4. Services Page (/services)

**Purpose**: Service management and control

**Managed Services**:
- Ollama LLM Service
- Learning Orchestrator
- Tensor Processor
- Escape System
- Storage Manager
- Agent Coordinator

**Controls**:
- Start/Stop individual services
- Restart services
- Configure service parameters
- Start/Stop all services
- View service status and PID

### 5. Diagnostics Page (/diagnostics)

**Purpose**: System health and troubleshooting

**Diagnostics**:
- Overall health assessment
- Error log viewing
- Anomaly detection
- Performance profiling
- System alerts
- Recovery status

### 6. Learning Page (/learning)

**Purpose**: Learning system monitoring

**Metrics**:
- Total learning cycles
- Tensors processed
- Convergence rate
- Pattern recognition count
- Learning effectiveness
- Cycle history

### 7. Storage Page (/storage)

**Purpose**: Holographic storage monitoring

**Wasan HD Metrics**:
- Total capacity
- Used capacity
- Compression ratio
- Retrieval times
- Active pages
- Data integrity score

### 8. AI Page (/ai)

**Purpose**: AI and multi-agent coordination

**Metrics**:
- Active agent count
- Tasks completed
- Coordination efficiency
- Consciousness level
- Decision latency
- Synchronization score

---

## API Reference

### Base URL

```
http://localhost:5000/api
```

### Authentication

Currently no authentication required (development mode). For production, implement authentication middleware.

### Endpoints

#### Metrics Endpoints

**GET /api/metrics/latest**
- Returns: Latest metrics from all categories
- Response: `{success: bool, data: {...}, timestamp_ms: int}`

**GET /api/metrics/{category}?limit=100**
- Parameters: category (system|performance|energy|stability|learning|storage|ai)
- Returns: Historical metrics for category
- Response: `{success: bool, category: str, data: [...], count: int}`

**GET /api/metrics/summary**
- Returns: Summary statistics across all metrics
- Response: `{success: bool, data: {...}}`

**POST /api/metrics/export**
- Body: `{filepath: str}`
- Returns: Export confirmation
- Response: `{success: bool, filepath: str}`

#### Benchmark Endpoints

**POST /api/benchmarks/run**
- Body: `{categories: [str], iterations: int, name: str}`
- Returns: Benchmark suite results
- Response: `{success: bool, suite: {...}}`

**GET /api/benchmarks/latest**
- Returns: Most recent benchmark suite
- Response: `{success: bool, suite: {...}}`

**GET /api/benchmarks/history?limit=10**
- Returns: Historical benchmark suites
- Response: `{success: bool, suites: [...], count: int}`

**GET /api/benchmarks/trends**
- Returns: Performance trend data
- Response: `{success: bool, trends: {...}}`

**GET /api/benchmarks/status**
- Returns: Current benchmark execution status
- Response: `{success: bool, is_running: bool, progress: {...}}`

#### Service Endpoints

**GET /api/services**
- Returns: All services and their status
- Response: `{success: bool, services: {...}}`

**GET /api/services/{service_id}**
- Returns: Specific service status
- Response: `{success: bool, service: {...}}`

**POST /api/services/{service_id}/start**
- Body: Optional configuration parameters
- Returns: Service start result
- Response: `{success: bool, service: {...}}`

**POST /api/services/{service_id}/stop**
- Body: `{force: bool}` (optional)
- Returns: Service stop result
- Response: `{success: bool, service: {...}}`

**POST /api/services/{service_id}/restart**
- Body: Optional configuration parameters
- Returns: Service restart result
- Response: `{success: bool, service: {...}}`

**POST /api/services/{service_id}/configure**
- Body: Configuration object
- Returns: Configuration result
- Response: `{success: bool, service_id: str, config: {...}}`

**GET /api/services/health**
- Returns: Overall system health
- Response: `{success: bool, health: {...}}`

**POST /api/services/start-all**
- Returns: Results for all services
- Response: `{success: bool, results: {...}}`

**POST /api/services/stop-all**
- Body: `{force: bool}` (optional)
- Returns: Results for all services
- Response: `{success: bool, results: {...}}`

#### System Status Endpoints

**GET /api/status/overview**
- Returns: Complete system overview
- Response: `{success: bool, overview: {...}}`

**GET /api/status/diagnostics**
- Returns: System diagnostics
- Response: `{success: bool, diagnostics: {...}}`

#### Utility Endpoints

**GET /api/health**
- Returns: API health check
- Response: `{success: bool, status: str, timestamp_ms: int}`

**GET /api/version**
- Returns: Dashboard version
- Response: `{success: bool, version: str, api_version: str}`

### Error Responses

All endpoints return error responses in the format:
```json
{
  "success": false,
  "error": "Error message description"
}
```

HTTP status codes:
- 200: Success
- 400: Bad request
- 404: Not found
- 500: Internal server error

---

## Configuration

### Environment Variables

```bash
# Dashboard host (default: 0.0.0.0)
export QMNF_DASHBOARD_HOST=0.0.0.0

# Dashboard port (default: 5000)
export QMNF_DASHBOARD_PORT=5000

# Data directory (default: ~/.qmnf_dashboard)
export QMNF_DASHBOARD_DATA_DIR=/path/to/data

# Debug mode (default: False)
export QMNF_DASHBOARD_DEBUG=True
```

### Data Directory Structure

```
~/.qmnf_dashboard/
├── benchmarks/              # Benchmark history
│   └── benchmark_*.json
├── services/                # Service configurations
│   └── {service_id}.json
├── exports/                 # Data exports
│   └── export_*/
└── dashboard.log           # Application log
```

### Service Configuration

Service configurations are stored as JSON files in `~/.qmnf_dashboard/services/`.

Example: `ollama.json`
```json
{
  "port": 11434,
  "timeout": 30
}
```

---

## Deployment

### Development Deployment

```bash
# Start with debug mode
python3 qmnf_master_dashboard.py --debug
```

### Production Deployment

#### Option 1: Gunicorn (Recommended)

```bash
# Install gunicorn
pip3 install gunicorn

# Run with gunicorn
gunicorn -w 4 -b 0.0.0.0:5000 qmnf_master_dashboard:app
```

#### Option 2: Systemd Service

```bash
# Create service file (done by setup_dashboard.sh)
systemctl --user enable qmnf-dashboard
systemctl --user start qmnf-dashboard

# Check status
systemctl --user status qmnf-dashboard

# View logs
journalctl --user -u qmnf-dashboard -f
```

#### Option 3: Docker (Future)

```dockerfile
# Dockerfile example
FROM python:3.10-slim
WORKDIR /app
COPY . .
RUN pip install -r dashboard_requirements.txt
EXPOSE 5000
CMD ["python3", "qmnf_master_dashboard.py", "--host", "0.0.0.0"]
```

### Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name dashboard.example.com;

    location / {
        proxy_pass http://localhost:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

### Security Considerations

1. **Authentication**: Implement authentication for production use
2. **HTTPS**: Use SSL/TLS in production
3. **Firewall**: Restrict access to trusted networks
4. **API Keys**: Add API key authentication for programmatic access
5. **Rate Limiting**: Implement rate limiting on API endpoints

---

## Troubleshooting

### Common Issues

#### Dashboard Won't Start

**Error**: `Address already in use`
**Solution**: Port 5000 is in use. Use a different port:
```bash
python3 qmnf_master_dashboard.py --port 8080
```

**Error**: `Module not found`
**Solution**: Install dependencies:
```bash
pip3 install -r dashboard_requirements.txt --user
```

#### No Metrics Showing

**Problem**: Dashboard shows but no metrics appear
**Solution**:
1. Check browser console for JavaScript errors
2. Verify API endpoints: `curl http://localhost:5000/api/health`
3. Restart dashboard with `--debug` flag

#### Services Won't Start

**Problem**: Services show "Error" status
**Solution**:
1. Check service logs in `~/.qmnf_dashboard/dashboard.log`
2. Verify service dependencies are installed
3. Check service configuration files

#### High CPU Usage

**Problem**: Dashboard uses excessive CPU
**Solution**:
1. Increase collection interval in metrics_collector.py
2. Reduce chart update frequency
3. Limit historical data retention

### Debug Mode

Enable debug mode for detailed error messages:

```bash
python3 qmnf_master_dashboard.py --debug
```

### Log Files

Check logs for errors:

```bash
# Application log
tail -f ~/.qmnf_dashboard/dashboard.log

# Systemd journal (if using systemd)
journalctl --user -u qmnf-dashboard -f
```

### Performance Optimization

1. **Reduce collection interval**: Increase from 1000ms to 2000ms or higher
2. **Limit history size**: Reduce from 1000 to 500 data points
3. **Disable auto-refresh**: Stop auto-refresh when not actively monitoring
4. **Use production server**: Deploy with gunicorn instead of Flask dev server

---

## Development

### Adding New Metrics

1. **Define metric dataclass** in `metrics_collector.py`:
```python
@dataclass
class CustomMetrics:
    timestamp_ms: int
    custom_value: int

    def to_dict(self) -> Dict:
        return asdict(self)
```

2. **Add collection method**:
```python
def _collect_custom_metrics(self, timestamp_ms: int) -> CustomMetrics:
    return CustomMetrics(
        timestamp_ms=timestamp_ms,
        custom_value=self._get_custom_value()
    )
```

3. **Update collection loop**:
```python
self.custom_metrics.append(self._collect_custom_metrics(timestamp_ms))
```

4. **Add API endpoint** in `api_routes.py`:
```python
@self.api.route('/custom/metrics', methods=['GET'])
def get_custom_metrics():
    # Implementation
```

### Adding New Services

1. **Add service definition** in `service_controller.py`:
```python
'my_service': ServiceInfo(
    service_id='my_service',
    name='My Service',
    status=ServiceStatus.STOPPED,
    config={'param': 'value'}
)
```

2. **Implement start method**:
```python
def _start_my_service(self, config: Dict) -> bool:
    # Service start logic
    return True
```

### Testing

```bash
# Test metrics collection
python3 -c "from dashboard.metrics_collector import MetricsCollector; m = MetricsCollector(); m.start(); import time; time.sleep(5); print(m.get_latest_metrics())"

# Test benchmark runner
python3 -c "from dashboard.benchmark_runner import BenchmarkRunner; b = BenchmarkRunner(); suite = b.run_benchmark_suite(); print(suite)"

# Test service controller
python3 -c "from dashboard.service_controller import ServiceController; s = ServiceController(); print(s.get_all_services())"
```

### Contributing

1. Follow QMNF integer-only principles
2. All metrics must use integer representations (basis points, microseconds, etc.)
3. Document all new features in this guide
4. Test thoroughly before committing

---

## Integer-Only Metric Conventions

The dashboard strictly follows QMNF's integer-only mathematics principles:

### Percentages → Basis Points (BP)
- 1 BP = 1/10000 = 0.0001 = 0.01%
- 100% = 10000 BP
- Example: 85.5% = 8550 BP

### Time → Microseconds/Milliseconds
- Latency: microseconds (μs)
- Execution time: microseconds (μs)
- Timestamps: milliseconds since epoch (ms)

### Temperature → Millicelsius (mC)
- 1°C = 1000 mC
- Example: 65.5°C = 65500 mC

### Power → Milliwatts (mW)
- 1W = 1000 mW
- Example: 25.5W = 25500 mW

### Ratios → Basis Points
- Compression ratio 2.5x = 25000 BP
- Efficiency 92.3% = 9230 BP

---

## Version History

### v1.0.0 (2025-10-18)
- Initial release
- Complete dashboard implementation
- All 10 monitoring sections
- RESTful API
- Real-time updates
- Benchmark integration
- Service management
- Production-ready deployment

---

## Support and Contact

For issues, questions, or contributions:
- Check logs in `~/.qmnf_dashboard/dashboard.log`
- Review this guide's troubleshooting section
- Refer to QMNF system documentation in `~/QMNF_System/CLAUDE.md`

---

## License

Part of the QMNF (Quantum-Modular Numerical Framework) project.
Integer-only mathematics research system.

---

**End of QMNF Master Dashboard Guide**
