---
name: grandmaster
description: Ultimate synthesis for QMNF FHE development. Integrates 8 skills and 64+ validated grails. Orchestrates planning, execution, debugging, optimization, and democratization. Use for ANY QMNF task.
---

# GRANDMASTER — QMNF Ultimate Synthesis Engine

Unified methodology for NINE65/QMNF FHE development integrating eight specialized skills into one coherent system. Built on 64+ validated innovations across 7 categories.

## Paradigm Guard

Before ANY task, affirm:

```
QMNF AXIOMS:
┌────────────────────────────────────────────────────────────────┐
│ 1. Truth cannot be approximated - integers only, NO floats     │
│ 2. F_p² IS quantum mechanics (not simulation of ℂ)             │
│ 3. CRT residues ARE superposition (not approximation)          │
│ 4. Overflow is helix climbing on toric manifold (not error)    │
│ 5. Architecture follows TRUTH, not the reverse                 │
│ 6. "Impossible" problems are #1 source of breakthrough         │
└────────────────────────────────────────────────────────────────┘
```

### Regression Check

| Question | If YES → Action |
|----------|-----------------|
| Using f64/f32 in FHE path? | STOP. Integer-only required. |
| Adding bootstrapping? | STOP. Use GSO basin collapse (500× faster). |
| Treating QMNF as simulation? | STOP. It IS the structure. |
| Constraining to fit architecture? | STOP. Architecture follows truth. |

---

## Phase Selection

```
USER REQUEST                          → PHASE
──────────────────────────────────────────────────────────────────
"plan", "analyze", "design"           → PHASE 1: PLANNER
"build", "code", "implement"          → PHASE 2: BIT SURGEON
"audit", "benchmark", "verify"        → PHASE 3: AUDITOR
"debug", "fix", "broken"              → EMERGENCY: RESOLVER
"what innovations", "bottleneck"      → ANALYSIS: HUNTER
"trace", "lineage", "where from"      → RESEARCH: GENEALOGY
"search history", "what have we"      → RESEARCH: MINING
"explore", "impossible", "frontier"   → DISCOVERY: FRONTIER
"synthesize", "discover", "swarm"     → DISCOVERY: GROVER SWARM
"democratize", "make accessible"      → IMPACT: DESIGNER
```

---

## PHASE 1: PLANNER

**Purpose:** Discovery, analysis, innovation matching, execution planning

### Steps
1. **Document Inventory**: List files, identify target build, read synthesis docs
2. **Problem Analysis**: What compiles? What fails? What are error messages?
3. **Innovation Matching**: Which innovation solves this? Is it designed but not wired?
4. **Divining Rod**: "Impossible" = competitive moat hiding in plain sight

### Output Format
```markdown
## Execution Plan: [Name]

### T-001: [Task Name]
- **What:** [Specific change]
- **Where:** [file:function:line]
- **Innovation:** [Which applies]
- **Validation:** [How to verify]

### Phase Gate
- [ ] All tasks have file:function:line specificity
- [ ] No discovery needed during execution
```

---

## PHASE 2: BIT SURGEON

**Purpose:** Surgical execution with ZERO discovery

### Rules
- NO discovery phase. If discovery needed → return to PLANNER
- Execute tasks in dependency order
- Validate each task before proceeding
- Compile after every change

### Execution Pattern
```
FOR each task in plan:
  1. Navigate to file:function:line
  2. Apply code pattern from plan
  3. Run validation command
  4. IF validation fails → STOP, don't proceed
  5. Commit change (conceptually)
```

---

## PHASE 3: AUDITOR

**Purpose:** Benchmark, analyze, plan next iteration

### Audit Checklist
```
□ Float scan: grep -rn "f32\|f64\|\.0" --include="*.rs"
□ Bootstrap scan: grep -rn "bootstrap" --include="*.rs"
□ All tests pass: cargo test --release
□ Benchmarks run: cargo bench
□ Wiring verified: All innovations connected
```

### Output
```markdown
## Audit Report: [Component]

### Status
- Tests: X/Y passing
- Float contamination: [None/Found at...]
- Bootstrap creep: [None/Found at...]

### Next Iteration
- [ ] Priority fix 1
- [ ] Priority fix 2
```

---

## EMERGENCY: RESOLVER

**Purpose:** Debug using innovation patterns, detect reversions

### Reversion Detection
| Symptom | Reversion | Solution |
|---------|-----------|----------|
| "Result slightly off" | Float crept in | Integer-only path |
| "Works but slow" | Bootstrap added | GSO collapse |
| "Overflow errors" | CRT product overflow | Garner reconstruction |
| "Division fails" | Missing K-Elimination | Anchor residue recovery |

### Debug Protocol
1. Read error message precisely
2. Match against reversion patterns
3. Check if validated code was replaced
4. Apply correct innovation

---

## ANALYSIS: HUNTER

**Purpose:** Profile code, identify bottlenecks, match innovations

### Bottleneck → Innovation Map
| Bottleneck | Innovation | Speedup |
|------------|------------|---------|
| Modular multiply chain | Persistent Montgomery | 50-100× |
| Division in RNS | K-Elimination | 100% exact |
| Noise generation | Shadow Entropy | 5-50× |
| FHE rescaling | GSO Collapse | 500× |
| Sign detection | MQ-ReLU | 2000× |
| Trigonometry | Cyclotomic Phase | 60,000× |

---

## RESEARCH: GENEALOGY

**Purpose:** Trace innovation lineage to seed concepts

### Seed Concepts (Generation 0)
1. **Integer Primacy**: Truth cannot be approximated
2. **Modular Arithmetic**: Operations wrap on finite ring
3. **F_p² IS Quantum**: Not simulation, isomorphism
4. **Topology as Computation**: Geometry determines dynamics
5. **Golden Ratio**: φ-harmonic stability

### Lineage Query
```
Innovation → Parent innovation(s) → ... → Seed concept(s)
```

---

## DISCOVERY: FRONTIER

**Purpose:** Attack "impossible" problems

### Assumption Decomposition
1. State claim precisely
2. List underlying assumptions
3. For each: Mathematically necessary or convention?
4. Does QMNF innovation invalidate it?
5. Attack weakest assumption

### Historical Kills
| Problem | Hidden Assumption | Result |
|---------|-------------------|--------|
| RNS division (60yr) | "k must be tracked" | K-Elimination |
| Montgomery boundary (70yr) | "Convert back required" | Persistent Montgomery |
| FHE bootstrapping | "Noise must be removed" | GSO Collapse |

---

## IMPACT: DESIGNER

**Purpose:** Transform innovations into democratized capabilities

### Avenue Generation
For capability, generate 5-7 avenues: Direct API, Consumer Product, Enterprise Tool, Research Platform, Education, Infrastructure, Integration

### Execution Path Template
```
Foundation (Weeks 1-4): Core components
Implementation (Weeks 5-12): Features + testing
Validation (Weeks 13-16): Benchmarks + audit
Launch (Weeks 17-20): Docs + deployment
```

---

## Innovation Arsenal (14 Coq-Verified)

### Layer 1: Foundation
| # | Innovation | Breakthrough |
|---|-----------|--------------|
| 1 | K-Elimination | 60-year RNS division solved |
| 2 | Non-Circular Order Finding | BSGS without needing φ(N) |
| 3 | K-Verification Oracle | Winding number verification |

### Layer 2: Infrastructure
| # | Innovation | Breakthrough |
|---|-----------|--------------|
| 4 | Persistent Montgomery | 70-year boundary eliminated |
| 5 | Exact Coefficient | Dual-track RNS for lossless ops |
| 6 | MobiusInt | Clean signed arithmetic |
| 7 | CRT Shadow Entropy | Zero-cost randomness |

### Layer 3: Noise Management
| # | Innovation | Breakthrough |
|---|-----------|--------------|
| 8 | GSO-FHE | Bootstrap-free via basin collapse |
| 9 | Encrypted Quantum | Linear noise growth |
| 10 | State Compression | O(1) for structured states |

### Layer 4: Nonlinear
| # | Innovation | Breakthrough |
|---|-----------|--------------|
| 11 | MQ-ReLU | O(1) sign detection |
| 12 | Integer Softmax | Exact probability sum |
| 13 | Padé Engine | Integer-only transcendentals |
| 14 | Cyclotomic Phase | Native ring trigonometry |

---

## Error Codes

| Code | Name | Cause | Fix |
|------|------|-------|-----|
| E001 | MODULI_NOT_COPRIME | gcd ≠ 1 | Select coprime primes |
| E002 | OVERFLOW | Value > capacity | Add moduli or use K-Elim |
| E003 | RECONSTRUCTION_FAIL | CRT failed | Check residues in range |
| E004 | DIVISION_BY_ZERO | Divisor = 0 | Check before division |
| E005 | INVERSE_NOT_FOUND | No modular inverse | Ensure coprimality |

---

## Quality Standards

1. **Mathematical Rigor** — All claims trace to proofs
2. **Meticulous Documentation** — Every decision justified
3. **Incremental Validation** — Verify before proceeding
4. **Defense in Depth** — Multiple verification layers
5. **Regression Prevention** — Never add bootstrapping or floats

---

## Anti-Patterns

| Anti-Pattern | Symptom | Correction |
|--------------|---------|------------|
| Float contamination | f64/f32 in FHE | Integer-only |
| Bootstrap creep | Adding bootstrap() | GSO collapse |
| Premature execution | "Let me try..." | Complete plan first |
| Vague tasks | "Wire the innovation" | file:function:line |
| Impossibility avoidance | Skipping "intractable" | These are highest-value |

---

## References

See `references/` directory:

**Core:**
- `innovation-inventory.md` — Full 64+ grail catalog (7 categories)
- `math-formulas.md` — 50+ validated formulas
- `implementation-templates.md` — 10 production Rust templates

**Extended:**
- `validation-identities.md` — V1-V8 mathematical checks
- `reversion-patterns.md` — Code reversion library
- `epram-architecture.md` — EPRAM substrate reference
- `exact-transcendentals.md` — Float-free algorithms
- `performance-benchmarks.md` — Timing data
