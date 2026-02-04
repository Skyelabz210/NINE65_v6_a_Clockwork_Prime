# NINE65/MANA FHE Security Report

**Date**: January 2026
**Classification**: Post-Quantum Cryptographic Security Assessment
**Framework**: RedShirt Security Testing
**Status**: COMPREHENSIVE ASSESSMENT COMPLETE

---

## Executive Summary

This report consolidates all security assessments conducted on the NINE65/MANA FHE system, including CPAD attack resistance, lattice security analysis, side-channel evaluation, and quantum simulation security.

### Overall Security Posture

| Component | Status | Critical Issues | Rating |
|-----------|--------|-----------------|--------|
| **FHE Core (he_standard_128)** | SECURE | None | A |
| **FHE Core (light configs)** | INSECURE | 36-bit classical security | F |
| **CPAD Resistance** | PASS | 0 bits recovered | A |
| **K-Elimination** | SECURE | No structural weakness | A |
| **NTT Operations** | SECURE | Precomputed tables | A |
| **Montgomery Arithmetic** | PARTIAL | Minor timing leak | B |
| **AHOP (F_{p2} Quantum Sim)** | VULNERABLE | Timing side-channels | C |
| **Dense Grover** | MODERATE | Target index leakage | C |

---

## 1. CPAD Attack Resistance

### Test Results: RESISTANT

| Configuration | Bits Recovered | Attack Success | Status |
|---------------|----------------|----------------|--------|
| `light_rns_exact` | 0.0 avg | 0/5 | **PASS** |
| `light_rns` | 0.0 avg | 0/3 | **PASS** |

### Key Defenses

1. **Private Noise Margin API**: `decrypt_dual_with_diagnostics()` is private
2. **Release Build Protection**: Margin returns 0 in release mode
3. **K-Elimination Exact Arithmetic**: No floating-point rounding oracle
4. **Constant-Time Operations**: Montgomery reduction is constant-time
5. **GSO-FHE Noise Bounding**: Swarm-based collapse available

### Information Leakage Analysis

```
Maximum Leakage from Public API:
  - Failure point: ~log2(ops_until_failure) bits
  - At 500 ops without failure: ~9 bits maximum
  - Security level (N): 1024 bits
  - Leakage ratio: 0.88%

Key Recovery Requirement:
  - Full recovery needs ~N bits
  - Current parameters: SECURE
```

### Test Command
```bash
cargo test -p nine65 --test cpad_attack_test --release
# Result: 6 passed, 0 failed
```

---

## 2. Lattice Attack Security

### Attack Estimator Results

#### Configuration: `light` (N=1024) - INSECURE

| Attack | Classical | Quantum | Assessment |
|--------|-----------|---------|------------|
| Primal (uSVP) | 2936 bits | 1473 bits | Theoretical |
| Dual | 209 bits | 109 bits | |
| **Hybrid** | **36 bits** | **41 bits** | **INSECURE** |
| HE Standard | 80 bits | 50 bits | Below threshold |

**CRITICAL**: Hybrid attack achieves only 36-bit classical security!

#### Configuration: `he_standard_128` (N=2048) - SECURE

| Attack | Classical | Quantum | Assessment |
|--------|-----------|---------|------------|
| Primal (uSVP) | 2936+ bits | 1473+ bits | Theoretical |
| HE Standard | 128+ bits | 80+ bits | **SECURE** |

### QMNF-Specific Attacks

| Attack | Security | Assessment |
|--------|----------|------------|
| K-Elim Inversion | Infinite | Does not apply - reduces to Ring-LWE |
| RNS Correlation | Infinite | CRT provides perfect reconstruction |
| Exact Arith Exploit | 97+ bits | Speculation only |

### Secure vs Insecure Configurations

```
INSECURE (DO NOT USE FOR PRODUCTION):
- light (N=1024, 36-bit security)
- light_mul
- light_rns (without parameter upgrade)

SECURE (128-bit+ security):
- he_standard_128 (N=2048)
- he_standard_128_deep (N=2048, deep circuits)
- high_192 (N=4096, 192-bit security)
```

---

## 3. AHOP (F_{p2} Quantum Simulation) Security

### Status: UNSUITABLE FOR CRYPTOGRAPHIC USE WITHOUT MODIFICATION

### Critical Vulnerabilities

| Vulnerability | Severity | File Location | Attack Vector |
|---------------|----------|---------------|---------------|
| Oracle index access | HIGH | grover.rs:37-39 | L1/L3 cache timing |
| Fermat's mod_pow | HIGH | mod.rs | Branch prediction timing |
| Probability measurement | HIGH | mod.rs:279-283 | Statistical inference |
| Prime validation | MEDIUM | grover_full.rs:400 | Composite p -> field breakdown |
| Cache-timing (oracle) | MEDIUM | grover_full.rs:414-424 | Store buffer analysis |

### Recommended Mitigations

1. Replace Fermat's with constant-time Montgomery ladder
2. Validate all primes (primality + p = 3 mod 4)
3. Implement oblivious memory access for oracle
4. Use MPC for target-secret applications

---

## 4. Dense Grover Implementation Security

### Implementations Assessed

| Implementation | File | Overflow Safety | Timing Resistance | Target Leakage |
|----------------|------|-----------------|-------------------|----------------|
| **DenseToricGrover** | dense_toric.rs | EXCELLENT | PARTIAL | HIGH |
| **ManaGrover** | mana_grover.rs | EXCELLENT | PARTIAL | HIGH |
| **DenseExactGrover** | dense_exact.rs | GOOD | PARTIAL | HIGH |

### Common Vulnerabilities (All Implementations)

1. **Target Index Leakage**: Array access pattern reveals target
2. **Probability Measurement**: Peaks reveal target state
3. **Conditional Branches**: Various comparison operations leak timing

### Implementation-Specific Issues

#### DenseToricGrover (dense_toric.rs)
| Location | Vulnerability | Severity |
|----------|--------------|----------|
| Line 95-99 | Conditional branch in helix_level() | MEDIUM |
| Line 152-154 | Zero-check leaks amplitude state | MEDIUM |
| Line 272-273 | Target array access | HIGH |

#### ManaGrover (mana_grover.rs)
| Location | Vulnerability | Severity |
|----------|--------------|----------|
| Line 81 | Montgomery final reduction branch | LOW-MEDIUM |
| Line 100 | Conditional negation | LOW |
| Line 193-218 | Sign comparison + helix level | MEDIUM |
| Line 328 | Target array access | HIGH |

**Unique Security Feature**: `target_above_threshold()` allows threshold checking without revealing exact probability.

#### DenseExactGrover (dense_exact.rs)
| Location | Vulnerability | Severity |
|----------|--------------|----------|
| Line 62 | Target array access | HIGH |
| Line 168-178 | Division rounding sign comparison | LOW |

### Minimum Substrate Theorem

| Substrate | Dimensions | Peak Probability | Status |
|-----------|------------|------------------|--------|
| Sparse (2-amp) | 2 | ~3% | BROKEN |
| WASSAN | 144 | 90%+ | WORKING |
| Dense | 2^n | 96%+ | OPTIMAL |

---

## 5. Side-Channel Analysis

### Montgomery Arithmetic

**File**: `crates/nine65/src/arithmetic/montgomery.rs`

```rust
// Montgomery REDC - mostly constant-time
pub fn montgomery_reduce(&self, t: u128) -> u64 {
    // ...
    // POTENTIAL LEAK: Conditional subtraction (lines 89-93)
    if result >= self.q {
        result - self.q
    } else {
        result
    }
}
```

**Vulnerability**: Conditional final reduction
**Severity**: LOW (mitigated by CPU prefetching)
**Mitigation**: Use constant-time selection: `result - (mask & self.q)`

### NTT Operations

**File**: `crates/nine65/src/arithmetic/ntt.rs`

**Assessment**: PASS
- Precomputed twiddle factor tables
- No data-dependent branches in transform
- Psi-twist for negacyclic convolution uses table lookup

---

## 6. Grover Oracle Cost Analysis (FHE Key Recovery)

For N=2048 Ring-LWE:

| Metric | Value |
|--------|-------|
| Key space | 2^2048 (ring elements) |
| Oracle cost (BFV decrypt) | ~10^6 gates |
| Grover iterations | pi/4 * sqrt(2^2048) ~ 2^1024 |
| Total quantum gates | ~2^1030 |

**Assessment**: Grover provides no practical speedup against Ring-LWE.

### Shor's Algorithm Applicability

**Result**: NOT APPLICABLE

Ring-LWE/Module-LWE does NOT reduce to:
- Integer factorization
- Discrete logarithm
- Period finding

The dihedral HSP (Kuperberg) achieves only subexponential speedup.

---

## 7. Test Results Summary

### Library Tests
```
cargo test -p nine65 --lib --release
Result: 406 passed, 0 failed, 13 ignored
```

### Security Tests
```
cargo test -p nine65 --test cpad_attack_test --release
Result: 6 passed, 0 failed
```

### PQEAQ Harness (Sparse Grover - Known Limitations)
```
cargo test -p nine65 --test pqeaq_harness --release
Result: 7/10 passed (3 failures are expected - sparse substrate limitation)
```

---

## 8. Vulnerability Summary

### HIGH Severity (Immediate Action Required)

| ID | Component | Vulnerability | Mitigation |
|----|-----------|---------------|------------|
| H1 | FHE | `light` config 36-bit security | Use he_standard_128+ |
| H2 | AHOP | Oracle timing leak | Constant-time implementation |
| H3 | AHOP | mod_pow branch leak | Montgomery ladder |
| H4 | AHOP | Probability measurement | MPC or differential privacy |
| H5 | Dense Grover | Target array access | Oblivious memory access |

### MEDIUM Severity (Should Fix)

| ID | Component | Vulnerability | Mitigation |
|----|-----------|---------------|------------|
| M1 | AHOP | Weak prime validation | Primality test |
| M2 | AHOP | Hardcoded primes | Runtime generation |
| M3 | Montgomery | Conditional subtraction | Constant-time mask |
| M4 | Dense Grover | Helix level timing | Constant-time comparison |
| M5 | Dense Grover | Sign comparison | Bit manipulation |

### LOW Severity (Good Practice)

| ID | Component | Vulnerability | Mitigation |
|----|-----------|---------------|------------|
| L1 | AHOP | Conditional negation | Bit manipulation |
| L2 | GCD | Variable iteration count | Binary GCD |
| L3 | Dense Exact | Division rounding | Constant-time division |

---

## 9. Recommendations

### Immediate Actions

1. **DO NOT USE** `light`, `light_mul` configs for production
2. **Upgrade minimum parameters** to N=2048 (128-bit security)
3. **Add deprecation warnings** to insecure config constructors

### Short-Term (1-3 months)

4. **Implement constant-time Montgomery** final reduction
5. **Add primality validation** in AHOP F_{p2} construction
6. **Create secure parameter selection guide**

### Medium-Term (3-6 months)

7. **Harden AHOP** for cryptographic use (if needed)
8. **Implement oblivious array access** for Grover oracles
9. **Add side-channel test suite** to CI/CD

### Long-Term (6+ months)

10. **External security audit** of FHE implementation
11. **NIST-style public evaluation** period
12. **Formal verification** of constant-time properties

---

## 10. Security Posture Visualization

```
FHE (he_standard_128):     ████████████████████ SECURE (128+ bits)
FHE (light):               ████                 INSECURE (36 bits)
CPAD Resistance:           ████████████████████ PASS (0 bits leaked)
K-Elimination:             ████████████████████ SECURE (no weakness)
NTT Operations:            ██████████████████   SECURE (precomputed)
Montgomery:                ████████████████     PARTIAL (minor leak)
AHOP (Quantum Sim):        ████████             VULNERABLE (timing)
Dense Grover:              ████████████         MODERATE (target leak)
```

---

## Attribution

- **Security Framework**: RedShirt Security Testing Framework
- **Attack Tools**: MANA-private attack-estimator, calibrated-estimator
- **Testing Date**: January 2026
- **Validated Against**: CPAD (CCS 2024), Lattice Estimator methodology

---

## Related Documents

- `docs/CPAD_RESISTANCE_REPORT.md` - Detailed CPAD testing
- `docs/AHOP_SECURITY_ASSESSMENT.md` - F_{p2} vulnerability analysis
- `docs/DENSE_GROVER_SECURITY_ASSESSMENT.md` - Grover implementation analysis
- `docs/QMNF_SECURITY_ASSESSMENT_COMPLETE.md` - Full technical assessment
- `crates/nine65/tests/cpad_attack_test.rs` - CPAD test implementation

---

*Report generated by RedShirt Security Testing Framework*
*NINE65/MANA FHE Post-Quantum Security Research*
*Classification: Authorized Security Testing*
