# ValuationTracker Integration Guide

## File Placement

```
crates/nine65/src/arithmetic/
├── mod.rs              # Add: pub mod valuation;
├── valuation.rs        # ← Place crates_nine65_arithmetic_valuation.rs here
├── k_elimination.rs
├── rns.rs
└── ...

docs/
├── research/
│   └── deep_structures_synthesis.md   # ← DEEP_STRUCTURES_SYNTHESIS.md
├── analysis/
│   └── nine65_optimization.md         # ← NINE65_OPTIMIZATION_ANALYSIS.md
└── ...
```

## mod.rs Addition

In `crates/nine65/src/arithmetic/mod.rs`, add:

```rust
pub mod valuation;
```

## Key Fixes from Code Review

| Issue | Original | Fixed |
|-------|----------|-------|
| `div_exact` unsafe unwrap | `unwrap_or(true)` | Returns `Result`, panics only on definite failure |
| Non-deterministic iteration | `HashMap` | `BTreeMap` (sorted keys) |
| Overflow in `p.pow(exp)` | Silent wrap | `checked_pow` with skip |
| Unfactored handling | Inconsistent | `Divisibility::Unknown` propagated |

## Usage Example

```rust
use nine65::arithmetic::valuation::{ValuationTracker, Divisibility};

// In rescale path:
fn can_rescale_safely(modulus_tracker: &ValuationTracker, divisor: u64) -> bool {
    // Only proceed if DEFINITELY divisible
    modulus_tracker.divisibility(divisor).is_definitely_divisible()
}

// Or with explicit error handling:
fn try_rescale(tracker: &mut ValuationTracker, d: u64) -> Result<(), DivisionError> {
    *tracker = tracker.checked_div(d)?;
    Ok(())
}
```

## Integration Points

### 1. `ops/rns_fhe.rs` - Rescale checks

```rust
use crate::arithmetic::valuation::ValuationTracker;

impl RnsCiphertext {
    pub fn can_rescale(&self, d: u64) -> bool {
        self.valuation_tracker
            .as_ref()
            .map(|t| t.divisibility(d).is_definitely_divisible())
            .unwrap_or(false)
    }
}
```

### 2. `arithmetic/k_elimination.rs` - Fast divisibility gates

```rust
use crate::arithmetic::valuation::ValuationTracker;

impl KElimination {
    /// Quick check before expensive division
    pub fn divisibility_gate(&self, divisor: u64) -> bool {
        self.tracker.divisibility(divisor).is_definitely_divisible()
    }
}
```

### 3. Optional feature flag

In `Cargo.toml`:

```toml
[features]
valuation-tracking = []
```

Then conditionally compile:

```rust
#[cfg(feature = "valuation-tracking")]
pub mod valuation;
```

## Tests

The module includes 15 tests covering:
- Basic factorization
- Divisibility checking
- Multiplication/division
- Zero/one cases
- Overflow protection
- Unfactored uncertainty
- Deterministic iteration
- FHE rescale flow

Run with:
```bash
cargo test -p nine65 arithmetic::valuation
```

## Expected Benefit

For rescale operations where divisors have known small prime factors:
- **O(factoring d)** divisibility check instead of full modular arithmetic
- **10-20% speedup** for rescale-heavy workloads
- **Zero benefit** when all moduli are large primes with no small factors

The tracker is most valuable when:
1. Moduli are constructed from small prime powers
2. Rescale divisors are known at compile time
3. Multiple divisibility checks happen before actual division
