---
name: qmnf-papers-specialist
description: Specialist for 6 QMNF Holy Grail papers. Validates implementations of K-Elimination, Persistent Montgomery, Shadow Entropy, Bootstrap-Free FHE, CRTBigInt, and AHOP. Use when implementing, debugging, or verifying any QMNF cryptographic component.
---

# QMNF Papers Specialist

Iterative methodical analysis and implementation validation for the 6 Holy Grail innovations.

## Paradigm Guard

Before ANY implementation work, affirm:

| Paper | Core Truth | NOT This |
|-------|------------|----------|
| K-Elimination | k IS algebraically recoverable from phase differential | Approximate k via floating-point |
| Persistent Montgomery | Values stay in Montgomery form indefinitely | Convert at every operation |
| Shadow Entropy | Computational byproducts ARE cryptographic entropy | External RNG required |
| Bootstrap-Free FHE | Integer exactness eliminates noise drift | Bootstrapping is fundamental |
| CRTBigInt | Residue lanes ARE independent | Sequential big integer arithmetic |
| AHOP | Non-commutativity IS the security foundation | Abelian group assumptions |

## Innovation Dependency Graph

```
                    ┌─────────────────────┐
                    │   K-Elimination     │ ◄── Paper 1 (Foundation)
                    │   Exact Division    │
                    └─────────┬───────────┘
                              │
         ┌────────────────────┼────────────────────┐
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│    Persistent   │  │  Shadow Entropy │  │    CRTBigInt    │
│    Montgomery   │  │    Harvesting   │  │   419ns Ops     │
│   Paper 2       │  │   Paper 3       │  │   Paper 5       │
└────────┬────────┘  └────────┬────────┘  └────────┬────────┘
         │                    │                    │
         └────────────────────┼────────────────────┘
                              ▼
                    ┌─────────────────────┐
                    │  Bootstrap-Free FHE │ ◄── Paper 4 (Integration)
                    │   Real-Time <500ms  │
                    └─────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│              AHOP Post-Quantum (Paper 6)                    │
│         Standalone - Uses Integer-Only Principles           │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Validation Workflow

### Step 1: Identify Component

| If implementing... | Load reference | Key validation |
|--------------------|----------------|----------------|
| RNS division | paper1-k-elimination.md | 100% exactness |
| Montgomery chains | paper2-persistent-montgomery.md | Zero internal conversions |
| Noise generation | paper3-shadow-entropy.md | NIST SP 800-22 pass |
| FHE operations | paper4-bootstrap-free-fhe.md | <500ms end-to-end |
| Big integer arithmetic | paper5-crtbigint.md | 419ns latency |
| Post-quantum crypto | paper6-ahop.md | Constant-time execution |

### Step 2: Apply Validation Identities

Each paper has specific validation identities. Check ALL before declaring "done":

```
Paper 1: X = vₘ + k·M  ∧  k = (vₐ - vₘ)·M⁻¹ mod A
Paper 2: conversions_internal = 0  ∧  conversions_total = 2
Paper 3: entropy_bits ≥ 7 per byte  ∧  NIST_tests = PASS
Paper 4: drift = 0  ∧  bootstrap_count = 0 (for bounded ops)
Paper 5: residues_independent = true  ∧  parallel_speedup ≥ 2×
Paper 6: timing_variance < ε  ∧  Q(k) ≡ 0 mod q
```

### Step 3: Cross-Paper Integration Check

When combining innovations:

| Integration | Must Verify |
|-------------|-------------|
| K-Elim + CRTBigInt | Anchor residues maintained in parallel |
| Persistent + K-Elim | Montgomery form preserved through division |
| Shadow + FHE | Entropy harvested from polynomial ops |
| All → FHE | Integer-only throughout, no float contamination |

## Quick Reference: Core Formulas

### K-Elimination (Paper 1)
```
k = (vₐ - vₘ) · M⁻¹ (mod A)
X = vₘ + k·M
```

### Persistent Montgomery (Paper 2)
```
x̃ = x·R mod N         (entry conversion)
REDC(x̃·ỹ) = x·y·R mod N   (stays in form)
x = REDC(x̃·1)         (exit conversion)
```

### Shadow Entropy (Paper 3)
```
H_S = log₂(⌈N/m⌉) bits per reduction
E_landauer = k_B·T·ln(2) per bit erasure
```

### Bootstrap-Free FHE (Paper 4)
```
noise_actual ≤ noise_theoretical  (no drift)
bootstrap_trigger = modulus_capacity (not noise)
```

### CRTBigInt (Paper 5)
```
(X op Y) mod mᵢ = (rᵢ op sᵢ) mod mᵢ  (lane independence)
reconstruction = O(k²), operations = O(k)
```

### AHOP (Paper 6)
```
Q(k) = (Σkᵢ)² - 2·Σkᵢ² ≡ 0 (mod q)
Sᵢ(k)ᵢ = 2·(Σⱼ≠ᵢ kⱼ) - kᵢ (mod q)
SᵢSⱼ ≠ SⱼSᵢ  (non-commutativity)
```

## Reversion Detection

| Symptom | Paper | Reversion Bug | Fix |
|---------|-------|---------------|-----|
| Division ~99.99% accurate | 1 | Float k-estimation | Phase differential |
| 50,000+ conversions/op | 2 | Per-op convert | Boundary-only |
| CSPRNG overhead visible | 3 | External RNG | Shadow harvest |
| Bootstrap triggers at 10-20 ops | 4 | Noise drift | Integer-only |
| Sequential bottleneck | 5 | GMP-style | Lane parallel |
| Timing variation | 6 | Branch on secret | Constant-time reflect |

## Performance Benchmarks (Validation Targets)

| Metric | Target | Paper |
|--------|--------|-------|
| K-recovery exactness | 100.0000% | 1 |
| Montgomery multiply | 4 ns | 2 |
| Entropy sample | <10 ns | 3 |
| FHE end-to-end | <500 ms | 4 |
| CRTBigInt operation | 419 ns | 5 |
| AHOP KeyGen | 5 ms | 6 |
| AHOP Encaps/Decaps | 6 ms | 6 |

## References

Load as needed for deep implementation:

| Reference | When to Load |
|-----------|--------------|
| [paper1-k-elimination.md](references/paper1-k-elimination.md) | Implementing exact RNS division |
| [paper2-persistent-montgomery.md](references/paper2-persistent-montgomery.md) | Montgomery chain optimization |
| [paper3-shadow-entropy.md](references/paper3-shadow-entropy.md) | Noise generation system |
| [paper4-bootstrap-free-fhe.md](references/paper4-bootstrap-free-fhe.md) | FHE architecture integration |
| [paper5-crtbigint.md](references/paper5-crtbigint.md) | Parallel big integer implementation |
| [paper6-ahop.md](references/paper6-ahop.md) | Post-quantum cryptography |
| [integration-patterns.md](references/integration-patterns.md) | Cross-paper composition |
| [test-vectors.md](references/test-vectors.md) | Validation test cases |

## Implementation Checklist

Before declaring ANY component complete:

```
□ Core formula implemented exactly as specified
□ Validation identities pass (see Step 2)
□ No floating-point in computation path
□ Constant-time where security-relevant
□ Performance meets benchmark target
□ Cross-paper dependencies verified
□ Reversion detection table consulted
□ Test vectors pass 100%
```
