# Grover Swarm MCP Server

**Quantum-Inspired Knowledge Graph Search & Innovation Discovery**

This MCP (Model Context Protocol) server implements the Grover Swarm algorithm for knowledge graph exploration and innovation discovery, with K-Elimination theorem knowledge pre-integrated.

---

## Features

✅ **Knowledge Graph Management**
- Fact/atom registration with topic tracking
- Edge-based relationship modeling
- Multi-topic cross-pollination detection

✅ **Discovery Mechanisms**
- Reconnaissance queries (frontier exploration)
- Full swarm search with multi-wave convergence
- Innovation potential scoring (I(v) = Σw + λ·mix)
- Automatic discovery state tracking (UNKNOWN → SIGHTED → VALIDATED → INTEGRATED)

✅ **Quantum-Inspired Search**
- Grover amplitude amplification simulation
- Priority-based target allocation
- Ripple effect propagation
- Weight-based resource optimization

✅ **Formal Guarantees**
- Guaranteed termination (≤ |K| waves)
- Frontier bound: |∂U| ≤ |D| × Δ
- Innovation monotonicity
- Progress guarantee per wave

---

## Quick Start

### 1. Installation

The server is already configured. Ensure Python 3.8+ is installed:

```bash
python3 --version
```

### 2. Initialize K-Elimination Knowledge

First use should initialize the knowledge base:

```python
# Via MCP tool call
{
  "tool": "grover_initialize_kelimination",
  "params": {}
}
```

This creates 8 foundational atoms:
- K-Elimination core theorem
- CRT uniqueness
- Key congruence
- Modular inverse properties
- Bootstrap-free FHE
- FHE performance metrics
- Lean 4 verification (27 theorems)
- Coq verification (10 proofs)

### 3. Basic Usage

```python
# Example: Query for exact division knowledge
{
  "tool": "grover_query_recon",
  "params": {
    "query": "exact division",
    "maxResults": 10
  }
}

# Example: Register new fact
{
  "tool": "grover_register_fact",
  "params": {
    "content": "Montgomery multiplication reduces modular ops by 50%",
    "topic": 4,
    "weight": "350"
  }
}

# Example: Launch full swarm search
{
  "tool": "grover_launch_swarm",
  "params": {
    "query": "bootstrap-free encryption",
    "maxWaves": 15
  }
}
```

---

## Available Tools

### Core Tools

#### `grover_register_fact`
Register a new knowledge atom

**Parameters:**
- `content` (string, required): Fact content
- `topic` (integer, required): Topic ID (1-based)
- `weight` (string, required): Importance weight (0-1000)
- `id` (string, optional): Custom atom ID
- `sourceUri` (string, optional): Source URL

**Returns:**
```json
{
  "success": true,
  "atomId": "atom_1735120000000_abc123",
  "atom": { /* full atom object */ }
}
```

#### `grover_add_edge`
Add relationship between atoms

**Parameters:**
- `from` (string, required): Source atom ID
- `to` (string, required): Target atom ID
- `relation` (string, required): Relationship type (e.g., "enables", "proves", "requires")
- `confidence` (string, required): Confidence level (0-1000000)

**Returns:**
```json
{
  "success": true,
  "edge": {
    "from_id": "crt_uniqueness",
    "to_id": "kelim_core",
    "relation": "enables",
    "confidence": 900000
  }
}
```

### Search Tools

#### `grover_query_recon`
Perform reconnaissance query to explore frontier

**Parameters:**
- `query` (string, required): Search query (text matching)
- `includeUnknown` (boolean, default: true): Include UNKNOWN atoms
- `topicFilter` (array[int], optional): Filter by topics
- `maxResults` (integer, default: 20): Max results

**Returns:**
```json
{
  "query": "exact division without approximation",
  "frontierSize": 15,
  "discoveredCount": 8,
  "unknownCount": 12,
  "frontier": [
    {
      "atomId": "atom_xyz",
      "content": "...",
      "innovationPotential": 2500,
      "validatedNeighborCount": 3,
      "mixTerm": 6,
      "topicDistribution": {"1": 2, "2": 1}
    }
  ]
}
```

**Interpretation:**
- `mixTerm`: Cross-topic pair count (higher = more interdisciplinary)
- `innovationPotential`: I(v) = Σw(u) + λ×mix (total priority score)
- `frontierSize`: Atoms on boundary between known/unknown

#### `grover_launch_swarm`
Launch full multi-wave swarm search

**Parameters:**
- `query` (string, required): Search query
- `maxWaves` (integer, default: 20): Maximum waves
- `topicFilter` (array[int], optional): Restrict to topics
- `config` (object, optional):
  - `alpha` (string, default: "150"): Grover iteration base
  - `beta` (string, default: "20"): Weight decay coefficient
  - `lambda` (string, default: "75"): Mix term multiplier

**Returns:**
```json
{
  "query": "bootstrap-free homomorphic encryption",
  "converged": true,
  "wavesExecuted": 12,
  "totalDiscoveries": 23,
  "innovationScore": 92,
  "waves": [
    {
      "waveNumber": 1,
      "discoveredCount": 3,
      "frontierSize": 18,
      "converged": false,
      "metrics": {
        "groverIterations": 4500,
        "discoveryRate": 667,
        "weightConcentration": 1250
      }
    }
  ],
  "discoveries": [ /* list of discovered atoms */ ]
}
```

**Metrics:**
- `discoveryRate`: Discoveries per 1000 iterations (efficiency)
- `weightConcentration`: Focus measure (1 = uniform distribution)
- `innovationScore`: topics × discoveries (synthesis quality)

### Validation Tools

#### `grover_validate_discovery`
Validate a discovery with witness atoms

**Parameters:**
- `atomId` (string, required): Atom to validate
- `witnesses` (array[string], required): Witness atom IDs (need ≥2 validated)
- `compositionRule` (string, required): Validation reasoning

**Returns:**
```json
{
  "success": true,
  "atomId": "atom_discovered_123",
  "previousState": "SIGHTED",
  "newState": "VALIDATED",
  "validWitnesses": ["crt_uniqueness", "atom_modular_inverse"],
  "compositionRule": "CRT + inverse existence implies exact recovery"
}
```

#### `grover_compute_potential`
Compute innovation potential for specific atom

**Parameters:**
- `atomId` (string, required): Target atom
- `lambda` (string, default: "75"): Mix multiplier

**Returns:**
```json
{
  "atomId": "atom_unknown_456",
  "content": "...",
  "state": "UNKNOWN",
  "innovationPotential": 4200,
  "components": {
    "weightSum": 3000,
    "mixTerm": 12,
    "lambdaTimesMix": 1200
  },
  "validatedNeighborCount": 5,
  "topicDistribution": {"1": 2, "2": 2, "3": 1},
  "isDiverseCrossroads": true
}
```

**Formula:**
```
I(v) = Σ w(u) + λ × mix
       u∈N_D(v)

mix = Σ c_r × c_s
      r<s
```

Where:
- `c_r` = count of neighbors in topic r
- `mix` = cross-topic pair count
- `isDiverseCrossroads` = true if ≥3 topics and mix > 0

### Status Tools

#### `grover_get_status`
Get current knowledge graph statistics

**Returns:**
```json
{
  "graphStats": {
    "atomCount": 45,
    "edgeCount": 67,
    "topicCount": 6,
    "maxDegree": 8
  },
  "discoveryStates": {
    "UNKNOWN": 12,
    "SIGHTED": 5,
    "VALIDATED": 25,
    "INTEGRATED": 3
  },
  "frontierSize": 15,
  "discoveredSize": 28,
  "waveCount": 7,
  "totalDiscoveries": 28
}
```

#### `grover_initialize_kelimination`
Initialize K-Elimination knowledge base

**Returns:**
```json
{
  "success": true,
  "message": "K-Elimination knowledge initialized"
}
```

---

## Topics in Pre-Loaded Knowledge

| Topic ID | Domain | Example Atoms |
|----------|--------|---------------|
| 1 | K-Elimination & RNS | Core theorem, CRT, modular inverse |
| 2 | FHE Applications | Bootstrap-free rescaling, performance |
| 3 | Formal Verification | Lean 4 proofs, Coq validation |
| 4+ | User-defined | Your custom domains |

---

## Usage Patterns

### Pattern 1: Exploratory Research

```python
# 1. Initialize if needed
grover_initialize_kelimination()

# 2. Reconnaissance
recon = grover_query_recon({
  "query": "exact arithmetic without approximation",
  "maxResults": 30
})

# 3. Analyze frontier
for atom in recon['frontier']:
    if atom['mixTerm'] > 0:  # Cross-topic opportunity
        print(f"High potential: {atom['atomId']} (I={atom['innovationPotential']})")

# 4. Deep search on promising areas
result = grover_launch_swarm({
  "query": recon['frontier'][0]['atomId'],  # Target highest potential
  "maxWaves": 20
})
```

### Pattern 2: Iterative Deepening

```python
converged = False
total_waves = 0

while not converged and total_waves < 100:
    # Limited swarm
    result = grover_launch_swarm({
        "query": target_query,
        "maxWaves": 10
    })

    # Register discoveries as validated facts
    for discovery in result['discoveries']:
        grover_register_fact({
            "id": discovery['id'],
            "content": discovery['content'],
            "topic": discovery['topic'],
            "weight": "300"
        })

    converged = result['converged']
    total_waves += result['wavesExecuted']
```

### Pattern 3: Cross-Domain Synthesis

```python
# Find cross-topic opportunities
recon = grover_query_recon({
    "query": "optimization",
    "maxResults": 50
})

# Filter for diversity
crossroads = [
    atom for atom in recon['frontier']
    if atom.get('isDiverseCrossroads', False)
]

# Sort by innovation potential
crossroads.sort(key=lambda x: x['innovationPotential'], reverse=True)

# Search top candidates
for candidate in crossroads[:5]:
    grover_launch_swarm({
        "query": candidate['atomId'],
        "maxWaves": 15
    })
```

---

## Wave Protocol

The swarm search follows this protocol:

```
Wave 0: Reconnaissance    → Build D_0, identify query targets
Wave 1: Frontier Extract  → Compute ∂U_t, prioritize by I(v)
Wave 2: Target Assign     → Allocate atoms to nodes by weight
Wave 3: Grover Cycles     → Amplitude amplification
Wave 4: Validate & Ripple → Confirm discoveries, broadcast
Loop until ∂U_t = ∅       → Convergence (Theorem T10)
```

**Guarantees:**
- Terminates in ≤ |K| waves (K = total atoms)
- Progress each wave: |D_{t+1}| > |D_t| or frontier empty
- Frontier bounded: |∂U_t| ≤ |D_t| × Δ (Δ = max degree)

---

## Persistence

Knowledge graph is automatically saved to:
```
/home/acid/Projects/NINE65/MANA-private/mcp-server/knowledge_graph.json
```

Structure:
```json
{
  "atoms": [
    {
      "id": "atom_...",
      "content": "...",
      "topic": 1,
      "weight": 500,
      "state": "VALIDATED",
      "source_uri": "https://...",
      "timestamp": 1735120000.0
    }
  ],
  "edges": [
    {
      "from_id": "atom_a",
      "to_id": "atom_b",
      "relation": "enables",
      "confidence": 900000
    }
  ],
  "discovery_history": ["atom_1", "atom_2", ...],
  "wave_count": 42,
  "last_updated": "2026-01-06T..."
}
```

---

## Security & Rate Limiting

- **Rate Limit**: 100 requests/minute (adjustable)
- **Audit Logging**: All requests logged to `logs/grover_audit_YYYYMMDD.log`
- **Data Isolation**: Knowledge graph is user-scoped

---

## Integration with K-Elimination Research

Pre-loaded knowledge includes:

1. **K-Elimination Theorem** (Topic 1)
   - O(k) exact division vs O(k²) traditional
   - Key congruence lemma
   - CRT uniqueness guarantee
   - Modular inverse requirements

2. **FHE Applications** (Topic 2)
   - Bootstrap-free rescaling
   - <500μs homomorphic operations
   - Real-time FHE capabilities

3. **Formal Verification** (Topic 3)
   - 27 Lean 4 theorems (0 sorry)
   - 10 Coq proofs (0 admitted)
   - Cross-validation across proof systems

**Edges (Relationships):**
- CRT → K-Elimination (enables)
- Key Congruence → K-Elimination (proves)
- Modular Inverse → K-Elimination (required_by)
- K-Elimination → Bootstrap-Free FHE (enables)
- Lean Verification → K-Elimination (validates)
- Coq Verification → K-Elimination (validates)

---

## References

- K-Elimination Landing Page: https://skyelabz210.github.io/k-elimination-lean4/
- GitHub Repository: https://github.com/Skyelabz210/k-elimination-lean4
- Technical Paper: [K_Elimination_Technical_Paper.pdf](https://skyelabz210.github.io/k-elimination-lean4/K_Elimination_Technical_Paper.pdf)
- QMNF Advanced Mathematics: Contact via founder@hackfate.us

---

## Troubleshooting

### Server won't start
```bash
# Check Python version
python3 --version  # Should be 3.8+

# Test server directly
python3 /home/acid/Projects/NINE65/MANA-private/mcp-server/grover_swarm_server.py

# Check logs
tail -f /home/acid/Projects/NINE65/MANA-private/mcp-server/logs/grover_audit_*.log
```

### Knowledge graph corrupted
```bash
# Backup current graph
cp knowledge_graph.json knowledge_graph.json.backup

# Reset (WARNING: deletes all custom knowledge)
rm knowledge_graph.json

# Reinitialize
grover_initialize_kelimination()
```

### Low discovery rate
- Increase `maxWaves` parameter
- Check `frontierSize` in recon results
- Verify atoms are VALIDATED (not just SIGHTED)
- Add more edges to connect isolated atoms

---

## License

**Grover Swarm MCP Server**: QMNF Advanced Mathematics
**K-Elimination Theorem**: MIT License
**MANA FHE**: Proprietary (All Rights Reserved)

Contact: founder@hackfate.us
