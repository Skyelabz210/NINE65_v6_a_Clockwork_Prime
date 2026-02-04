# Innovation Bundle Manifest
## FPD Implementation Sprint

**Version:** 1.0.0
**Generated:** 2025-12-29
**Purpose:** Pre-wired implementations ensuring QMNF innovations are used

---

## Bundle Contents

### /innovations/
Pre-validated QMNF innovation implementations:

| Innovation | Directory | Status | Tests |
|------------|-----------|--------|-------|
| Binary GCD | `binary_gcd/` | Production | 30,000+ |
| Montgomery | `montgomery/` | Production | Validated |
| K-Elimination | `k_elimination/` | Production | 100% exact |
| CRTBigInt | `crt_bigint/` | Production | 2.4M ops/sec |
| Shadow Entropy | `shadow_entropy/` | Production | 5-10× faster |
| Barrett | `barrett/` | Production | Validated |

### /scaffolds/
Pre-generated code with innovations slotted in:

| Task | Scaffold | Innovation Wired |
|------|----------|------------------|
| T-001 | `T-001_mod_residue.rs` | — |
| T-002 | `T-002_binary_gcd.rs` | Binary GCD |
| T-003 | `T-003_anchor_set.rs` | — |
| T-004 | `T-004_error.rs` | — |
| T-005 | `T-005_egcd.rs` | Binary GCD |
| T-006 | `T-006_fast_path.rs` | Montgomery + Barrett |
| T-007 | `T-007_piggyback.rs` | K-Elimination |
| T-008 | `T-008_gcd_reduction.rs` | K-Elimination |
| T-009 | `T-009_crt_tower.rs` | CRTBigInt |
| T-010 | `T-010_lib.rs` | All |
| T-011 | `T-011_constant_time.rs` | Shadow Entropy |
| T-012 | `T-012_audit.rs` | — |

### /regression/
Automated regression detection:

| File | Purpose |
|------|---------|
| `FORBIDDEN_PATTERNS` | Patterns that indicate stdlib regression |
| `REQUIRED_PATTERNS` | Patterns that must be present |
| `scan.sh` | Automated regression check script |

---

## Usage Instructions

### Starting a Task

1. Check CHECKLIST.md for next available task
2. Copy scaffold to src/:
   ```bash
   cp scaffolds/T-XXX_name.rs src/name.rs
   ```
3. Implement TODO sections (innovations pre-wired)
4. Run tests:
   ```bash
   cargo test --lib name
   ```
5. Verify benchmark targets (if PERF gate)
6. Update CHECKLIST.md status

### Using an Innovation

Each innovation has:
- `INTERFACE.md` - Input/output contract
- `impl.rs` - Reference implementation
- `tests.rs` - Validation suite
- `INSTEAD_OF.md` - What stdlib to replace

Example:
```rust
// INSTEAD OF: num::Integer::gcd(&a, &b)
// USE: binary_gcd(&a, &b)

use crate::innovations::binary_gcd::binary_gcd;

let g = binary_gcd(&a, &b);
```

### Running Regression Scan

```bash
./regression/scan.sh src/
```

Expected output:
```
✓ No forbidden patterns found
✓ All required patterns present
REGRESSION STATUS: CLEAN
```

---

## Innovation Interfaces

### Binary GCD
```rust
/// 2.16× faster than Euclidean GCD
/// Uses only shifts and subtractions (no division)
fn binary_gcd(a: u64, b: u64) -> u64;
fn binary_gcd_bigint(a: &BigInt, b: &BigInt) -> BigInt;
```

### Montgomery Multiplication
```rust
/// 15-20% faster modular multiplication
/// Stay in Montgomery form, convert only at boundaries
struct MontgomeryContext {
    modulus: u64,
    r: u64,      // R = 2^k > modulus
    r_inv: u64,  // R^-1 mod modulus
    n_prime: u64 // -modulus^-1 mod R
}

impl MontgomeryContext {
    fn mul(&self, a: u64, b: u64) -> u64;
    fn to_montgomery(&self, a: u64) -> u64;
    fn from_montgomery(&self, a: u64) -> u64;
}
```

### K-Elimination
```rust
/// 100% exact RNS division (vs 99.9998% baseline)
/// Recovers overflow quotient k from anchor modulus
fn k_eliminate(
    v_alpha: &BigInt,  // Value mod alpha_cap
    v_beta: &BigInt,   // Value mod beta_cap
    alpha_cap: &BigInt,
    beta_cap: &BigInt,
) -> (BigInt, BigInt); // Returns (exact_value, k)
```

### CRTBigInt
```rust
/// Parallel big integer via CRT
/// k× speedup where k = number of residue lanes
struct CRTBigInt {
    residues: Vec<u64>,
    moduli: Vec<u64>,
}

impl CRTBigInt {
    fn add(&self, other: &Self) -> Self;  // Parallel
    fn mul(&self, other: &Self) -> Self;  // Parallel
    fn reconstruct(&self) -> BigInt;      // CRT
}
```

### Shadow Entropy
```rust
/// 5-10× faster than CSPRNG
/// Harvests entropy from computation byproducts
struct ShadowEntropy {
    state: [u64; 4],
}

impl ShadowEntropy {
    fn extract_bits(&mut self, n: usize) -> u64;  // <10ns
    fn sample_coprime(&mut self, modulus: &BigInt) -> BigInt;
}
```

---

## Float Policy

**FORBIDDEN - Integer/Rational Only**

The regression scanner will flag any:
- `f64`, `f32` types
- `.exp()`, `.ln()`, `.log()` methods
- `.to_f64()`, `.as_f64()` conversions
- `std::f64::*` constants

Use QMNF alternatives:
- Floats → CRTBigInt + QMNFRational
- Transcendentals → Padé approximants
- Noise → Shadow Entropy
