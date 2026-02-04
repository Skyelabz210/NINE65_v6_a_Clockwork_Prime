# Dense Grover Implementations Security Assessment

**Date**: January 2026
**Classification**: Security Research (RedShirt Analysis)
**Scope**: dense_toric.rs, mana_grover.rs, dense_exact.rs

---

## Executive Summary

Three Dense Grover implementations were analyzed for security vulnerabilities:

| Implementation | Overflow Safety | Timing Resistance | Target Leakage | Rating |
|----------------|-----------------|-------------------|----------------|--------|
| **DenseToricGrover** | EXCELLENT | PARTIAL | HIGH | MODERATE |
| **ManaGrover** | EXCELLENT | PARTIAL | HIGH | MODERATE |
| **DenseExactGrover** | GOOD | PARTIAL | HIGH | MODERATE |

All implementations share the fundamental issue of **target index leakage** through array access and probability measurement - inherent to Grover's algorithm design.

---

## 1. DenseToricGrover (`dense_toric.rs`)

### Architecture

- Uses **Dual Codex** (M, A coprime moduli) for toric representation
- **K-Elimination** extracts exact values from (inner mod M, outer mod A)
- Helix climbing handles overflow without panic

### Strengths

| Feature | Benefit |
|---------|---------|
| No overflow panic | 50+ iterations without i128 overflow |
| Binary GCD | Division-free, more constant-time than Euclidean |
| Helix tracking | Exact value recovery at any iteration depth |
| CRT-based | Mathematically sound reconstruction |

### Timing Vulnerabilities

#### V1: Conditional Branch in helix_level() (MEDIUM)

```rust
// dense_toric.rs:95-99
let diff = if self.outer >= inner_mod_a {
    self.outer - inner_mod_a
} else {
    self.config.a - inner_mod_a + self.outer
};
```

**Attack**: Branch prediction reveals relationship between outer and inner.

#### V2: Conditional Negation (MEDIUM)

```rust
// dense_toric.rs:152-154
inner: if self.inner == 0 { 0 } else { self.config.m - self.inner },
```

**Attack**: Zero-check leaks whether amplitude is zero.

#### V3: Target Array Access (HIGH)

```rust
// dense_toric.rs:272-273
pub fn apply_oracle(&mut self) {
    self.amplitudes[self.target] = self.amplitudes[self.target].neg();
}
```

**Attack**: Cache timing reveals target index (same as AHOP).

### Security Rating: **MODERATE**

- **Positive**: No overflow, exact arithmetic
- **Negative**: Multiple timing side-channels

---

## 2. ManaGrover (`mana_grover.rs`)

### Architecture

- **Persistent Montgomery Form** - never converts during iteration
- **K-Elimination** for O(1) magnitude comparison
- Dual channel (alpha/beta) with coprime primes

### Strengths

| Feature | Benefit |
|---------|---------|
| Montgomery throughout | No conversion overhead |
| Threshold without reconstruction | Line 369-381 checks probability in ⊗ form |
| K-Elimination comparison | O(1) magnitude ordering |
| Deep iteration support | 100+ iterations in Montgomery form |

### Timing Vulnerabilities

#### V1: Montgomery Final Reduction (LOW-MEDIUM)

```rust
// mana_grover.rs:81
if result >= self.q { result - self.q } else { result }
```

**Attack**: Conditional subtraction timing leak (same as core Montgomery).

#### V2: Conditional Negation (LOW)

```rust
// mana_grover.rs:100
if a == 0 { 0 } else { self.q - a }
```

#### V3: Complex Comparison Logic (MEDIUM)

```rust
// mana_grover.rs:193-218
if self.negative == other.negative {
    // Same sign path
} else {
    // Different sign path with helix comparison
    let self_k = self.helix_level(codex);  // TIMING LEAK
    ...
}
```

**Attack**: Sign comparison and helix level extraction create timing patterns.

#### V4: Target Array Access (HIGH)

```rust
// mana_grover.rs:328
self.amplitudes[self.target] = self.amplitudes[self.target].neg();
```

### Unique Security Feature

```rust
/// Check if target probability exceeds threshold (without full reconstruction)
pub fn target_above_threshold(&self, num: u64, den: u64) -> bool {
    // Uses ONLY Montgomery operations + K-Elimination
    let lhs = target_sq.scale(den, &self.codex);
    let rhs = total_sq.scale(num, &self.codex);
    lhs.compare_magnitude(&rhs, &self.codex) == Ordering::Greater
}
```

This allows **threshold checking without revealing exact probability** - a partial mitigation.

### Security Rating: **MODERATE**

- **Positive**: Persistent Montgomery, threshold without reconstruction
- **Negative**: Timing leaks in comparison and negation

---

## 3. DenseExactGrover (`dense_exact.rs`)

### Architecture

- Uses **MobiusInt** for signed amplitudes (interference support)
- Integer arithmetic with rounding-to-nearest
- Simple, predictable computation pattern

### Strengths

| Feature | Benefit |
|---------|---------|
| Simple arithmetic | Predictable timing patterns |
| MobiusInt | Clean signed representation |
| WASSAN integration | Ready for 144:1 compression |

### Concerns

| Issue | Impact |
|-------|--------|
| Weight drift | ~20% drift after 1000 iterations (expected) |
| Integer division | div_round_nearest has some variability |

### Timing Vulnerabilities

#### V1: Target Array Access (HIGH)

```rust
// dense_exact.rs:62
self.amplitudes[self.target] = self.amplitudes[self.target].neg();
```

#### V2: Division Rounding (LOW)

```rust
// dense_exact.rs:168-178
fn div_round_nearest(numerator: i128, denominator: i128) -> i128 {
    if denominator == 0 { return 0; }
    let sign = if (numerator < 0) ^ (denominator < 0) { -1 } else { 1 };
    // ...
}
```

**Attack**: Sign comparison creates minor timing variance.

### Security Rating: **MODERATE**

- **Positive**: Simpler than toric/MANA, less attack surface
- **Negative**: Target index still leaks, some drift

---

## 4. Comparative Analysis

### Substrate Dimension vs Security

| Implementation | Substrate Dim | Peak Probability | Timing Resistance |
|----------------|---------------|------------------|-------------------|
| Sparse F_{p²} | 2 | ~3% (broken) | N/A |
| WASSAN | 144 | 90%+ | Not assessed |
| Dense Toric | 2^n | 96%+ | PARTIAL |
| MANA Grover | 2^n | 96%+ | PARTIAL |
| Dense Exact | 2^n | 96%+ | PARTIAL |

### Common Vulnerabilities (All Implementations)

1. **Target Index Leakage**: Array access pattern reveals target
2. **Probability Measurement**: Peaks reveal target state
3. **Conditional Branches**: Various comparison operations leak timing

### Unique Features

| Implementation | Unique Security Benefit |
|----------------|------------------------|
| DenseToricGrover | Helix overflow handling |
| ManaGrover | Threshold check without reconstruction |
| DenseExactGrover | Simplest attack surface |

---

## 5. Recommendations

### Immediate (HIGH Priority)

1. **Implement oblivious array access** for oracle operation:
   ```rust
   // Instead of: amplitudes[target].neg()
   for (i, amp) in amplitudes.iter_mut().enumerate() {
       let mask = constant_time_eq(i, target);
       *amp = conditional_neg(*amp, mask);
   }
   ```

2. **Replace conditional comparisons** with constant-time operations:
   ```rust
   // Instead of: if result >= q { result - q } else { result }
   let mask = -(result >= q) as u64;
   result - (mask & q)
   ```

### Medium-Term (MEDIUM Priority)

3. **Add timing randomization** for sensitive operations
4. **Implement masked arithmetic** for amplitude operations
5. **Document security boundaries** - when timing resistance matters vs not

### Architectural (For Cryptographic Use)

6. **MPC for target-secret applications** - no single party sees target
7. **Differential privacy** for probability queries
8. **Secure multi-party Grover** with secret-shared target

---

## 6. Minimum Interaction Substrate Theorem

The gap assessment noted:

| Substrate | Dimensions | Peak Probability |
|-----------|------------|------------------|
| Sparse (2-amp) | 2 | ~3% (broken) |
| WASSAN | 144 | 90%+ (working) |
| Dense | 2^n | 96%+ (optimal) |

**Security Implication**: The sparse representation sacrifices correctness for compression. For cryptographic applications requiring correct probability readout, **dense or WASSAN (144+) substrates are required**.

The 144-dimension WASSAN threshold appears to be the minimum for maintaining quantum interference dynamics while achieving compression.

---

## 7. Summary

### Security Ratings

| Implementation | Overflow | Timing | Target Leak | Overall |
|----------------|----------|--------|-------------|---------|
| DenseToricGrover | A | C+ | F | C |
| ManaGrover | A | C+ | F | C |
| DenseExactGrover | B+ | B- | F | C |

### Key Findings

1. **All implementations leak target through array access** - fundamental to Grover design
2. **Probability measurement reveals target** - inherent to quantum measurement
3. **Toric and MANA have excellent overflow properties** but timing side-channels
4. **DenseExact is simplest** but has weight drift

### Verdict

**For non-cryptographic use** (algorithm simulation, research): All implementations are suitable.

**For cryptographic use** (secret target): None are suitable without hardening. Requires:
- Constant-time oracle
- MPC for target distribution
- Differential privacy for measurements

---

*Report generated by RedShirt Security Testing Framework*
*NINE65/MANA FHE Security Research*
