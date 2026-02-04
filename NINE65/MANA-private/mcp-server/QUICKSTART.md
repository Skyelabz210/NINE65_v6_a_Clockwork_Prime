# Grover Swarm MCP - Quick Start Guide

## 🚀 5-Minute Setup

### 1. Verify Installation

```bash
# Check Python version (need 3.8+)
python3 --version

# Test server
cd /home/acid/Projects/NINE65/MANA-private/mcp-server
python3 grover_swarm_server.py
# Press Ctrl+C after seeing initialization message
```

### 2. Run Demo

```bash
# Run K-Elimination integration demo
python3 test_kelimination_integration.py
```

**Expected Output:**
```
✓ 8 atoms initialized (K-Elimination knowledge)
✓ 7 edges created (relationships)
✓ kelim_core has Innovation Potential: 3005
✓ Topics: RNS (1), FHE (2), Formal Verification (3)
```

### 3. Use via MCP Client

The server is configured in `config.json`:

```json
{
  "mcpServers": {
    "grover-swarm": {
      "command": "python3",
      "args": ["/home/acid/Projects/NINE65/MANA-private/mcp-server/grover_swarm_server.py"]
    }
  }
}
```

---

## 📖 Common Operations

### Initialize K-Elimination Knowledge

```python
# Via MCP tool
grover_initialize_kelimination()
```

This creates:
- **8 atoms**: K-Elim core, CRT, key congruence, modular inverse, bootstrap-free FHE, performance, Lean proofs, Coq proofs
- **7 edges**: Relationships showing how concepts enable/prove each other
- **3 topics**: RNS mathematics (1), FHE applications (2), Formal verification (3)

### Search for Knowledge

```python
# Reconnaissance query
grover_query_recon({
    "query": "exact division",
    "maxResults": 10
})
```

**Returns frontier atoms** (boundary between known/unknown) sorted by innovation potential.

### Analyze Innovation Potential

```python
# See why an atom is important
grover_compute_potential({
    "atomId": "kelim_core",
    "lambda": "75"  # Mix multiplier
})
```

**Interpretation:**
- `innovationPotential`: Total priority score
- `weightSum`: Importance of neighbors
- `mixTerm`: Cross-topic pairs (diversity indicator)
- `isDiverseCrossroads`: true if ≥3 topics connected

### Launch Full Search

```python
# Multi-wave convergent search
grover_launch_swarm({
    "query": "bootstrap-free encryption",
    "maxWaves": 20,
    "config": {
        "alpha": "150",   # Grover iteration base
        "beta": "20",     # Weight decay
        "lambda": "75"    # Mix multiplier
    }
})
```

**Metrics:**
- `wavesExecuted`: How many waves ran
- `converged`: true if frontier is empty
- `discoveryRate`: Efficiency (discoveries / 1000 iterations)
- `innovationScore`: topics × discoveries (synthesis quality)

---

## 🎯 Example Workflow

```python
# 1. Initialize
grover_initialize_kelimination()

# 2. Check status
status = grover_get_status()
# → atomCount: 8, edgeCount: 7, topicCount: 3

# 3. Find research gaps
recon = grover_query_recon({
    "query": "modular arithmetic optimization",
    "maxResults": 20
})

# 4. Prioritize by innovation potential
for atom in recon['frontier']:
    if atom['mixTerm'] > 0:  # Cross-topic
        print(f"{atom['atomId']}: I(v) = {atom['innovationPotential']}")

# 5. Deep search on highest potential
result = grover_launch_swarm({
    "query": recon['frontier'][0]['atomId'],
    "maxWaves": 15
})

# 6. Validate discoveries
for discovery in result['discoveries']:
    grover_validate_discovery({
        "atomId": discovery['id'],
        "witnesses": ["kelim_core", "crt_uniqueness"],
        "compositionRule": "Explain validation logic here"
    })

# 7. Extend knowledge
grover_register_fact({
    "content": "New insight discovered...",
    "topic": 4,  # New topic
    "weight": "400"
})
```

---

## 📊 Understanding the Output

### Knowledge Graph Stats

```json
{
  "atomCount": 45,      // Total facts
  "edgeCount": 67,      // Total relationships
  "topicCount": 6,      // Distinct domains
  "maxDegree": 8        // Most connected atom
}
```

### Discovery States

- **UNKNOWN**: Not yet explored
- **SIGHTED**: Found during search
- **VALIDATED**: Confirmed with witnesses
- **INTEGRATED**: Fully incorporated

### Innovation Potential Formula

```
I(v) = Σ w(u) + λ × mix
       u∈neighbors

mix = Σ_{topics r<s} count_r × count_s
```

**Example:**
- 5 neighbors across 3 topics: [A:2, B:2, C:1]
- Weight sum: 2000
- Mix: (2×2) + (2×1) + (2×1) = 8
- λ = 75
- **I(v) = 2000 + 75×8 = 2600**

Higher mix → more interdisciplinary → more innovation potential

---

## 🔍 Pre-Loaded Knowledge Graph

```
Topic 1: K-Elimination & RNS
  ├─ kelim_core (weight: 500)
  │   ├─ enables → bootstrap_free
  │   ├─ required_by ← modular_inverse
  │   ├─ enables ← crt_uniqueness
  │   ├─ proves ← key_congruence
  │   └─ validates ← lean_verification, coq_verification
  ├─ crt_uniqueness (weight: 400)
  ├─ key_congruence (weight: 450)
  └─ modular_inverse (weight: 300)

Topic 2: FHE Applications
  ├─ bootstrap_free (weight: 450)
  │   └─ achieves → fhe_performance
  └─ fhe_performance (weight: 400)

Topic 3: Formal Verification
  ├─ lean_verification (weight: 300)
  │   └─ validates → kelim_core
  └─ coq_verification (weight: 280)
      └─ validates → kelim_core
```

**kelim_core is the crossroads:**
- 6 neighbors across 3 topics
- Mix term: 11 (high diversity)
- Innovation potential: 3005

---

## 🛠️ Troubleshooting

### "Rate limit exceeded"
- Default: 100 req/min
- Edit `MAX_REQUESTS_PER_MINUTE` in `grover_swarm_server.py`

### "Atom not found"
- Run `grover_get_status()` to see available atoms
- Check `knowledge_graph.json` for IDs

### Low discovery rate
- Increase `maxWaves` parameter
- Verify atoms are VALIDATED (not just SIGHTED)
- Add more edges to connect isolated atoms

### Reset knowledge graph
```bash
# Backup first!
cp knowledge_graph.json knowledge_graph.json.backup

# Delete
rm knowledge_graph.json

# Reinitialize
grover_initialize_kelimination()
```

---

## 📚 Next Steps

1. **Read Full Documentation**: `GROVER_SWARM_README.md`
2. **Explore MCP Examples**: `MCP_TOOL_EXAMPLES.md` (if available)
3. **Study Wave Protocol**: `WAVE_PROTOCOL_REFERENCE.md` (if available)
4. **Review K-Elimination**: https://skyelabz210.github.io/k-elimination-lean4/

---

## 🎓 Key Concepts

| Term | Meaning |
|------|---------|
| **Atom** | Knowledge fact with topic, weight, state |
| **Edge** | Relationship (enables, proves, validates, etc.) |
| **Frontier** | Boundary between known/unknown (∂U) |
| **Mix Term** | Cross-topic pair count (diversity) |
| **Innovation Potential** | Priority score: I(v) = Σw + λ×mix |
| **Wave** | One iteration of search protocol |
| **Convergence** | Frontier empty (∂U = ∅) |

---

## 💡 Pro Tips

✅ **Use topic filters** to focus searches:
```python
grover_query_recon({
    "query": "optimization",
    "topicFilter": [1, 3]  # Only RNS + Verification
})
```

✅ **Check `isDiverseCrossroads`** to find synthesis opportunities:
```python
potential = grover_compute_potential({"atomId": "atom_123"})
if potential['isDiverseCrossroads']:
    # High-value target for innovation
```

✅ **Monitor `discoveryRate`** to judge search efficiency:
- `> 500`: Excellent (>0.5 per 1000 iterations)
- `200-500`: Good
- `< 200`: Need more waves or better targets

✅ **Use validation** to promote discoveries:
```python
grover_validate_discovery({
    "atomId": "new_insight",
    "witnesses": ["kelim_core", "bootstrap_free"],
    "compositionRule": "K-Elim exact division + FHE rescaling = real-time homomorphic ML"
})
```

---

## 🔗 Links

- **K-Elimination Paper**: https://skyelabz210.github.io/k-elimination-lean4/K_Elimination_Technical_Paper.pdf
- **GitHub Repository**: https://github.com/Skyelabz210/k-elimination-lean4
- **QMNF Advanced Mathematics**: Contact founder@hackfate.us

---

**License**: QMNF Advanced Mathematics | K-Elimination Theorem: MIT License
