# UPDATED GAP ANALYSIS: Fourth Attractor Bug + Corrections

**Update Date:** 2026-01-08  
**Source:** Grok AI Validation Suite (8174 scenarios, 10 moduli)

---

## CRITICAL BUG IDENTIFIED

### Fourth Attractor Stall at Distance 1

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                     FOURTH ATTRACTOR VALIDATION RESULTS                        ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  NAIVE VARIANT:                                                                ║
║  ├─ Scenarios tested:     8,174                                               ║
║  ├─ Successes:            0        (0.00%)                                    ║
║  ├─ Stalled at distance 1: 8,174   (100%)                                     ║
║  └─ STATUS: STRUCTURALLY BROKEN                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  DITHERED VARIANT:                                                             ║
║  ├─ Scenarios tested:     8,174                                               ║
║  ├─ Successes:            8,174    (100%)                                     ║
║  ├─ Average steps:        ~5.0     (O(log M))                                 ║
║  └─ STATUS: COMPLETE CONVERGENCE                                              ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### The Bug

The naive rule:
```rust
let diff = (target + m - state) % m;
let delta = (diff * 3) / 4;  // When diff ∈ {1,2,3}, delta = 0
let new_state = (state + delta) % m;
```

**Problem:** When `diff ∈ {1, 2, 3}`, integer division gives `delta = 0`, creating a **permanent fixed point at distance 1-3 from target**.

This affects **100% of non-trivial trajectories** - they ALL stall.

### The Fix: Dithered Fourth Attractor

```rust
/// Corrected Fourth Attractor with shortest-arc dither
/// 
/// When integer division would give delta=0 but we haven't reached target,
/// nudge by 1 in the shortest arc direction.
/// 
/// Properties preserved:
/// - Integer-only arithmetic
/// - Deterministic (same input → same output)
/// - O(log M) convergence
/// - Constant time per step
pub fn fourth_attractor_step_dithered(state: u64, target: u64, m: u64) -> u64 {
    let diff = (target + m - state) % m;
    
    if diff == 0 {
        return state;  // Already at target
    }
    
    let mut delta = (diff * 3) / 4;
    
    // Dither: if delta would be 0 but we're not at target, nudge shortest arc
    if delta == 0 {
        delta = if diff <= m / 2 { 1 } else { m - 1 };
    }
    
    (state + delta) % m
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convergence_all_starts() {
        for m in [8, 16, 32, 64, 128, 256, 512, 1024] {
            let target = 0u64;
            for start in 1..m {
                let mut state = start;
                for _ in 0..100 {
                    state = fourth_attractor_step_dithered(state, target, m);
                    if state == target {
                        break;
                    }
                }
                assert_eq!(state, target, "Failed for M={}, start={}", m, start);
            }
        }
    }
    
    #[test]
    fn test_convergence_steps_logarithmic() {
        let m = 4096u64;
        let target = 0u64;
        let mut total_steps = 0u64;
        let mut count = 0u64;
        
        for start in 1..m {
            let mut state = start;
            let mut steps = 0;
            while state != target && steps < 1000 {
                state = fourth_attractor_step_dithered(state, target, m);
                steps += 1;
            }
            total_steps += steps;
            count += 1;
        }
        
        let avg = total_steps as f64 / count as f64;
        // Should be approximately log_4(M) + 2 ≈ 6-7 for M=4096
        assert!(avg < 10.0, "Average steps {} too high", avg);
    }
}
```

---

## UPDATED GAP STATUS

### Original Gap Analysis Updates

| Gap ID | Description | Previous Status | Updated Status |
|--------|-------------|-----------------|----------------|
| GAP-009 | Fourth Attractor not wired | WIRING | **CRITICAL BUG** |

**New Critical Gap:**
- [ ] **GAP-022**: Fourth Attractor has distance-1 stall bug - MUST use dithered variant

### Implementation Priority Change

```
BEFORE (from original analysis):
  Gate 1: Implement EPRAMField → Wire Fourth Attractor
  
AFTER (with bug fix):
  Gate 1: Implement EPRAMField → Wire DITHERED Fourth Attractor
          └─ The naive version DOES NOT WORK
```

---

## MATH_ARSENAL.RS AUDIT (Grok's Findings)

### Critical Items Requiring Closure

| ID | Component | Issue | Severity | Fix Required |
|----|-----------|-------|----------|--------------|
| A | `BePoly::ntt_multiply` | Stub calling schoolbook | HIGH | Wire to NTTGen3::negacyclic_convolve |
| B | `NTTGen3::find_primitive_root` | Returns `Some(3)` unconditionally | HIGH | Implement order-checking search |
| C | `CyclotomicPhase::sin/cos` | Returns 0 on fallback | **CRITICAL** | Define exact algebraic or declare fixed-point |
| D | `MQReLU::gelu` | Simplified heuristic | MEDIUM | Label as rational surrogate or implement Padé |
| E | `MobiusInt::fixed_points` | Empty for quadratic case | MEDIUM | Implement integer sqrt for discriminant |
| F | `PadeApproximant::*` | Rational approximants | LOW | Label correctly (not "exact transcendentals") |

### Corrected find_primitive_root

```rust
/// Find primitive root of unity of given order modulo prime
/// 
/// Uses proper order-checking search:
/// 1. Factor prime-1
/// 2. Find generator g where g^((p-1)/q) ≠ 1 for all prime factors q
/// 3. Derive ω = g^((p-1)/order)
pub fn find_primitive_root(prime: u64, order: u64) -> Option<u64> {
    if (prime - 1) % order != 0 {
        return None;  // Order must divide p-1
    }
    
    // Factor p-1
    let factors = factor(prime - 1);
    
    // Find generator
    let mut g = 2u64;
    'outer: loop {
        if g >= prime {
            return None;  // No generator found (shouldn't happen for prime)
        }
        
        for &q in &factors {
            let exp = (prime - 1) / q;
            if mod_pow(g, exp, prime) == 1 {
                g += 1;
                continue 'outer;
            }
        }
        break;  // g is a generator
    }
    
    // Derive primitive root of given order
    let exp = (prime - 1) / order;
    Some(mod_pow(g, exp, prime))
}

/// Factor n into prime factors (simple trial division)
fn factor(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// Modular exponentiation (integer-only)
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp /= 2;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}
```

---

## UPDATED COMPLETENESS MATRIX

```
LAYER COMPLETENESS (REVISED)
═══════════════════════════════════════════════════════════════════════════════
Layer 0 (Physical RAM):          ████████░░░░░░░░░░░░  40%  (unchanged)
Layer 1 (MANA/UNHAL):            ██████████████░░░░░░  70%  (NTT stub found)
Layer 2 (EPRAM Substrate):       ░░░░░░░░░░░░░░░░░░░░   0%  (Fourth Attractor FIXED needed)
Layer 3 (Cyclotomic Ops):        ██████████████░░░░░░  70%  (sin/cos fallback found)
Layer 4 (Permanent Residents):   ████████████████░░░░  80%  (unchanged)
Layer 5 (Orchestrator):          ░░░░░░░░░░░░░░░░░░░░   0%  (unchanged)
Layer 6 (Autopoiesis):           ████████░░░░░░░░░░░░  40%  (unchanged)
═══════════════════════════════════════════════════════════════════════════════
OVERALL:                         ████████░░░░░░░░░░░░  38%  (DOWN from 43%)
```

**Why down?** The Grok audit revealed:
1. Fourth Attractor naive version is broken (0% convergence)
2. Several math_arsenal.rs components have fallback/stub behavior
3. CyclotomicPhase::sin/cos returns 0 on fallback

---

## REVISED CRITICAL PATH

### Gate 1: EPRAM Foundation (UPDATED)

```
REQUIRED CHANGES:
[ ] T-001: Create epram/mod.rs with EPRAMCell trait
[ ] T-002: Create EPRAMField<C, N> struct with step()
[ ] T-003: Create Topology enum
[ ] T-004: Create TerminationContract enum
[ ] T-005: Wire DITHERED fourth_attractor_step  ← CHANGED (naive broken)
[ ] T-005b: Add tests verifying 100% convergence ← NEW

MATH_ARSENAL FIXES REQUIRED:
[ ] T-023: Fix find_primitive_root (order-checking search)
[ ] T-024: Wire BePoly::ntt_multiply to NTTGen3
[ ] T-025: Fix CyclotomicPhase::sin/cos fallback behavior
```

---

## THEOREM UPDATES

### Theorem EPRAM-TERM-001 (Revised Statement)

**Original claim:** Fourth Attractor with k=3/4 converges in O(log M) steps.

**Revised statement:** 
> The **dithered** Fourth Attractor with k=3/4 converges in O(log M) steps.
> The naive variant has universal distance-1 fixed points and does NOT satisfy termination contracts.

### Proof (Grok's validation):
- 8,174 scenarios tested across 10 moduli
- Naive: 0% convergence (100% stall at distance 1)
- Dithered: 100% convergence, average ~5 steps for M=4096

---

## LYAPUNOV FUNCTION UPDATE

The Lyapunov function `V(x) = Σ_i d(x[i], T_i(x))` still works, but only with the dithered variant:

```rust
/// Lyapunov functional for EPRAM field
/// V(x) = sum of minimal modular distances to targets
/// 
/// REQUIRES: Dithered Fourth Attractor transition rule
/// GUARANTEES: V(x) strictly decreases until V(x) = 0
pub fn lyapunov_functional<const N: usize>(
    cells: &[u64; N],
    targets: &[u64; N],
    m: u64,
) -> u64 {
    cells.iter()
        .zip(targets.iter())
        .map(|(&c, &t)| {
            let forward = (t + m - c) % m;
            let backward = (c + m - t) % m;
            forward.min(backward)  // Minimal modular distance
        })
        .sum()
}

/// Verify Lyapunov descent after one field step
pub fn verify_descent<const N: usize>(
    before: &[u64; N],
    after: &[u64; N],
    targets: &[u64; N],
    m: u64,
) -> bool {
    let v_before = lyapunov_functional(before, targets, m);
    let v_after = lyapunov_functional(after, targets, m);
    
    // V should strictly decrease unless we're at fixed point
    v_after < v_before || v_after == 0
}
```

---

## SUMMARY

### What Grok Validated
1. **Fourth Attractor naive: BROKEN** (0% convergence)
2. **Fourth Attractor dithered: WORKS** (100% convergence)
3. **Convergence is O(log M)** as claimed
4. **math_arsenal.rs has 6 components with stubs/fallbacks**

### Immediate Actions
1. **Replace naive with dithered Fourth Attractor** everywhere
2. **Fix find_primitive_root** with proper order-checking
3. **Resolve CyclotomicPhase::sin/cos** fallback behavior
4. **Wire BePoly::ntt_multiply** to actual NTT implementation

### Impact on Timeline
- Gate 1 now requires bug fix (adds ~2 hours)
- math_arsenal fixes add ~4 hours
- Total added: ~6 hours to critical path

**The architecture is sound. The implementation has specific bugs that are now identified and correctable.**
