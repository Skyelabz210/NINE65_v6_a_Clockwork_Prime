## Context Establishment

**Problem**: Audit NINE65/MANA_boosted for integer-only, bootstrap-free, theorem-grounded alignment and produce a remediation plan.

**Success Criteria**:
1. Runtime-critical modules have zero floating-point usage (or explicit allowlist with fixed-point replacements).
2. Dual-RNS ct x ct path passes non-ignored tests for K-Elimination rescaling.
3. Proof debt is tracked with an enforced gate (no unexpected Admitted statements).

**Innovation Map**:
| Innovation | Role | Key Theorem |
|-----------|------|-------------|
| K-Elimination | Core exact rescaling | kElimination_core |
| GSO-FHE | Noise bounds for deep circuits | depth_50_achievable |
| Persistent Montgomery | Modular speedups | mont_mul_correct |
| CRT Shadow Entropy | Deterministic entropy | shadow_reconstruction |
| Exact Coefficient | Dual-track invariants | div_exact |

**Baseline**: Current code and published metrics in `README.md` and `BENCHMARK_REPORT.md`.

**Constraints**:
- Performance: depth-50 without bootstrapping; target ~812ms per README
- Memory: ~200MB per README
- Security: RLWE-based claims in `docs/SECURITY_PROOFS.md`

**Non-Goals**:
- Implementing bootstrapping
- Changing the BFV/RLWE scheme family
- Rewriting quantum modules beyond policy alignment
