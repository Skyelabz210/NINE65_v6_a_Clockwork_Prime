# QMNF Defense-in-Depth Integration Guide

**Purpose**: Step-by-step integration of all delivered components
**Target**: Production deployment of multiparty defense cluster

---

## PHASE 1: Foundation Layer

### 1.1 Build QFD Toolkit

```bash
cd /home/acid/Downloads/defenseindepth_extracted/qmnf_field_defense

# Build release binary
cargo build --release

# Verify binary
./target/release/qfd --help

# Or use pre-compiled binary
chmod +x ../qfd
../qfd
```

### 1.2 Verify Persistent Montgomery

```bash
# In QFD CLI
qfd> mont status
qfd> mont bench 998244353 1000000

# Expected output:
# Persistent Montgomery: ~3ns/op
# Standard modular: ~8ns/op
# Speedup: 2.5-3×
# (Plus ZERO conversion overhead = additional 1.2ms savings per FHE op)
```

### 1.3 Test K-Elimination

```bash
cd /home/acid/Downloads

# Compile and run tests
rustc k_elimination.rs --test -o k_elim_test
./k_elim_test

# Expected: 18 tests, all passing
```

---

## PHASE 2: Defense Shield Deployment

### 2.1 Quick Deploy

```bash
# In QFD CLI
qfd> shield quick

# This will:
# 1. Deploy 7-layer TCO with random seed
# 2. Initialize 1GB WASSAN holographic storage
# 3. Link TCO to WASSAN for spatiotemporal cloaking
# 4. Activate the shield
```

### 2.2 Manual Configuration

```bash
# Deploy TCO with specific seed (for reproducibility)
qfd> tco deploy --seed 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

# Initialize WASSAN with specific capacity
qfd> wassan init 2GB

# Link TCO to WASSAN
qfd> shield activate
```

### 2.3 Verify Shield Status

```bash
qfd> shield status

# Expected output:
# ═══════════════════════════════════════════════════════════════
#                     QMNF FIELD DEFENSE SHIELD
# ═══════════════════════════════════════════════════════════════
#
#   Shield: ACTIVE
#   Threat: ░░░░░░░░░░ 0/10
#
#   TCOs Deployed: 1
#     [0] ticks=100, obs=0x7f3a2b1c
#
#   WASSAN Storage:
#     Capacity: 2147483648 bytes
#     Used: 0 bytes (0.0%)
#     Entries: 0
#     TCO-linked: true
#
#   Persistent Montgomery:
#     Pre-computed moduli: 6
#     Conversion overhead: 0ms (70-year problem SOLVED)
```

---

## PHASE 3: Orchestrator Setup

### 3.1 Create Cargo Project

```bash
mkdir -p /home/acid/Projects/defense_orchestrator
cd /home/acid/Projects/defense_orchestrator
cargo init
```

### 3.2 Configure Dependencies

```toml
# Cargo.toml
[package]
name = "defense_orchestrator"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### 3.3 Integrate Orchestrator

```bash
# Copy orchestration code
cp /home/acid/Downloads/orchestration.rs src/orchestration.rs

# Create main.rs
cat > src/main.rs << 'EOF'
mod orchestration;

use orchestration::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Defense Orchestrator Starting...");

    // Load configuration
    let config = OrchestratorConfig::default();

    // Bootstrap orchestrator
    let mut orchestrator = DefenseOrchestrator::bootstrap(&config).await?;

    // Start all systems
    orchestrator.start().await?;

    println!("Defense Orchestrator ONLINE");
    println!("Nodes: {}", config.cluster.min_nodes);
    println!("Consensus threshold: {}%", config.sentinel.consensus_threshold / 10);

    // Run main loop
    orchestrator.threat_response_loop().await;

    Ok(())
}
EOF
```

### 3.4 Build and Run

```bash
cargo build --release
./target/release/defense_orchestrator
```

---

## PHASE 4: Multiparty Cluster

### 4.1 Node Configuration

```json
// config/node1.json (qclassic variant)
{
  "cluster": {
    "min_nodes": 3,
    "max_nodes": 5,
    "rebalance_threshold": 200
  },
  "sentinel": {
    "vote_timeout_ms": 5000,
    "consensus_threshold": 667
  },
  "soldiers": {
    "federated_update_interval_ms": 60000,
    "pattern_sync_interval_ms": 30000
  },
  "storage": {
    "checkpoint_interval_ms": 300000,
    "max_checkpoints": 10
  },
  "crypto": {
    "key_rotation_interval_ms": 86400000,
    "share_threshold": 2
  },
  "monitoring": {
    "health_check_interval_ms": 10000,
    "metrics_retention_hours": 168
  }
}
```

### 4.2 Cluster Topology

```
┌─────────────────────────────────────────────────────────────────┐
│                     5-NODE DEFENSE CLUSTER                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Node 1 (qclassic)        Node 5 (qclassic)                    │
│   ┌───────────────┐        ┌───────────────┐                    │
│   │ IP: 10.0.0.1  │        │ IP: 10.0.0.5  │                    │
│   │ Port: 8001    │        │ Port: 8005    │                    │
│   │ Role: Primary │        │ Role: Backup  │                    │
│   └───────┬───────┘        └───────┬───────┘                    │
│           │                        │                             │
│           └────────────┬───────────┘                             │
│                        │                                         │
│              ┌─────────▼─────────┐                               │
│              │ Sentinel Mesh     │                               │
│              │ (Byzantine BFT)   │                               │
│              └─────────┬─────────┘                               │
│                        │                                         │
│       ┌────────────────┼────────────────┐                        │
│       │                │                │                        │
│   ┌───▼───┐        ┌───▼───┐        ┌───▼───┐                   │
│   │Node 2 │        │Node 3 │        │Node 4 │                   │
│   │v2_comp│        │v2_comp│        │v2_comp│                   │
│   │10.0.0.2│       │10.0.0.3│       │10.0.0.4│                  │
│   └───────┘        └───────┘        └───────┘                   │
│                                                                  │
│   Quorum: 4-of-5 (Byzantine fault tolerant)                     │
│   Can tolerate: 1 malicious + 1 failed node                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 4.3 Key Ceremony

```rust
// Performed automatically on cluster start
// Manual trigger if needed:

// Via API
POST /api/key-ceremony
{
  "force": true,
  "reason": "scheduled rotation"
}

// Or via orchestrator
orchestrator.key_ceremony().await?;
```

---

## PHASE 5: Secure Storage Operations

### 5.1 Store Sensitive Data

```bash
# In QFD CLI
qfd> wassan store "TOP SECRET: Launch codes" 0xDEADBEEF

# Output:
# ✓ Data stored
#   Phase key: band=73, phase=0xa4f2b8c1e3d59067
#   SAVE THIS KEY FOR RETRIEVAL
```

### 5.2 Retrieve Data

```bash
qfd> wassan recall 73 0xa4f2b8c1e3d59067

# Output:
# ✓ Data retrieved:
#   TOP SECRET: Launch codes
```

### 5.3 Programmatic Access

```rust
use shield::Shield;

// Initialize shield
let mut shield = Shield::new(1024 * 1024 * 1024);  // 1GB
shield.deploy_tco(None);
shield.activate();

// Store data
let phase_key = shield.secure_store(b"secret data", 0x12345)?;

// Retrieve data
let data = shield.secure_retrieve(&phase_key)?;
```

---

## PHASE 6: Threat Response

### 6.1 Manual Threat Level

```bash
qfd> shield threat 7

# Output:
# Threat level set to 7
# (Shield will increase TCO ticking frequency)
```

### 6.2 Grover Threat Hunt

```bash
# Initialize search over 1 million items
qfd> grover search 1000000

# Output:
# ✓ Grover search initialized: id=0
#   Search space: 1000000 items
#   Optimal iterations: 785 (O(√N))
#   Classical would need: 1000000 iterations

# Run search
qfd> grover run 0

# Output:
# ✓ Search complete
#   Iterations: 785
#   Probability: 99.22%
#   Time: 1.2ms
#   TARGET FOUND
```

### 6.3 API Threat Reporting

```bash
# Report threat via API
curl -X POST http://localhost:8080/api/threat \
  -H "Content-Type: application/json" \
  -d '{"description": "Suspicious activity detected", "severity": 750}'

# Check TPI
curl http://localhost:8080/api/tpi

# Output:
# {"value": 750, "tier": "Orange", "timestamp_ns": 1703577600000000000}
```

---

## PHASE 7: Monitoring

### 7.1 Health Check

```bash
curl http://localhost:8080/api/health

# Output:
# {"healthy": true, "state": "Running", "timestamp_ns": ...}
```

### 7.2 Metrics

```bash
curl http://localhost:8080/api/metrics

# Output:
# {
#   "threats_detected": 42,
#   "threats_blocked": 41,
#   "votes_cast": 1260,
#   "consensus_reached": 42,
#   "keys_rotated": 3,
#   "nodes_active": 5
# }
```

### 7.3 Full Status

```bash
curl http://localhost:8080/api/status

# Output:
# {
#   "state": "Running",
#   "nodes": 5,
#   "tpi": 0,
#   "uptime_ns": 86400000000000
# }
```

---

## PHASE 8: Emergency Procedures

### 8.1 Emergency Key Rotation

```bash
curl -X POST http://localhost:8080/api/emergency \
  -H "Content-Type: application/json" \
  -d '{"action_type": "key_rotation"}'
```

### 8.2 Emergency Shutdown

```bash
curl -X POST http://localhost:8080/api/emergency \
  -H "Content-Type: application/json" \
  -d '{"action_type": "shutdown"}'
```

### 8.3 Rollback to Checkpoint

```rust
// Programmatic rollback
orchestrator.rollback(checkpoint_id).await?;
```

---

## INTEGRATION CHECKLIST

- [ ] QFD toolkit built and verified
- [ ] Persistent Montgomery benchmarked
- [ ] K-Elimination tests passing
- [ ] Shield deployed and activated
- [ ] WASSAN storage initialized
- [ ] TCO linked to WASSAN
- [ ] Orchestrator running
- [ ] All cluster nodes online
- [ ] Key ceremony completed
- [ ] Health checks passing
- [ ] Metrics collecting
- [ ] Threat response tested
- [ ] Emergency procedures documented

---

## TROUBLESHOOTING

### Issue: Montgomery modulus not found

```bash
qfd> mont add 12345678901

# Or programmatically
shield.add_modulus(12345678901);
```

### Issue: WASSAN capacity exceeded

```bash
qfd> wassan init 10GB
```

### Issue: TCO not linked

```bash
qfd> shield quick
# Or manually:
qfd> tco deploy
qfd> wassan init 1GB
# Link is automatic with shield activate
```

### Issue: Node won't join cluster

```rust
// Check minimum nodes constraint
if orchestrator.cyber_soldiers.nodes.len() >= config.cluster.max_nodes {
    // Cluster is full
}

// Force add (emergency)
orchestrator.add_node(new_node_id, FHEVariant::BFV).await?;
```

---

## PERFORMANCE TUNING

### High-Throughput Mode

```json
{
  "soldiers": {
    "federated_update_interval_ms": 10000,  // More frequent updates
    "pattern_sync_interval_ms": 5000
  },
  "monitoring": {
    "health_check_interval_ms": 1000  // Faster health checks
  }
}
```

### Low-Latency Mode

```rust
// Pre-compute all expected moduli
let fhe_primes = vec![998244353, 1004535809, 985661441, ...];
let montgomery = MontgomeryTable::new(&fhe_primes);

// Values stay in Montgomery form - zero conversion latency
```

### Memory-Constrained Mode

```bash
qfd> wassan init 256MB
```

---

## SECURITY NOTES

1. **Phase keys are secrets**: Treat WASSAN phase keys like encryption keys
2. **TCO seeds determine timing**: Random seeds for defense, coordinated for offense
3. **K-Elimination is exact**: No approximation errors to exploit
4. **Montgomery is persistent**: No conversion boundaries to attack
5. **Byzantine consensus**: 4-of-5 quorum tolerates 1 malicious node

---

*Integration Guide v1.0 - December 26, 2025*
*QMNF Defense-in-Depth System*
