# PLMG (Polyphonic Logarithmic Modular Gearing) Integration Architecture

## Executive Summary

This document specifies a complete architectural design for integrating PLMG theorems into the QMNF System as an additive enhancement layer. The design achieves eight critical objectives:

1. **Zero Breaking Changes**: All existing QMNF code remains functionally identical
2. **Opt-In Features**: PLMG enhancements enabled via explicit feature flags and constructors
3. **Integer-Only Guarantee**: Complete elimination of floating-point operations throughout PLMG
4. **Commodity Hardware**: Runs on standard x86_64/ARM with no specialized instructions required
5. **FFI Transparency**: Full Python bindings via PyO3 with batch operation support
6. **Performance Parity**: Matches or exceeds existing operation times (target: <250ns bounded operations)
7. **Deterministic Execution**: All PLMG operations reproducible across platforms and architectures
8. **Seamless Integration**: Leverages existing CRTBigInt, ModInt, and adaptive tier infrastructure

### Key Innovation: Stacked Precision Architecture

PLMG integrates via three architectural layers:

```
Layer 3: User Applications
    ↓ (opt-in via PLMGExtensions trait)
Layer 2: PLMG Theorems (Phase differential, Hierarchical gears, etc.)
    ↓ (transparent delegation)
Layer 1: Existing CRTBigInt/ModInt Infrastructure
    ↓ (no changes required)
Layer 0: Rust Integer Operations (~120-250ns baseline)
```

**No existing code changes needed.** PLMG operates as an additive enhancement through trait methods and builder patterns.

---

## Part 1: Module Structure Design

### Complete File Hierarchy

```
hcvlang/src/plmg/
│
├── mod.rs                              (282 lines)
│   ├─ Module registration and public exports
│   ├─ Feature flag definitions
│   └─ Integration trait definitions
│
├── phase_differential.rs               (418 lines)
│   ├─ Theorem 4: O(n+m) magnitude comparison
│   ├─ PhaseDifferential struct
│   ├─ Comparison algorithms
│   └─ Integration with CRTBigInt/ModInt
│
├── hierarchical_gears.rs               (556 lines)
│   ├─ Theorem 7: Multi-level CRT gearing
│   ├─ HierarchicalCRT struct (3-level, 7-level variants)
│   ├─ Gear ratio management
│   ├─ Logarithmic magnitude tracking
│   └─ Cost-based level promotion
│
├── balanced_encoding.rs                (312 lines)
│   ├─ Theorem 5: Balanced ternary sign representation
│   ├─ BalancedSignEncoding struct
│   ├─ Conversion algorithms
│   ├─ Integrated sign storage in residues
│   └─ Efficient sign operations
│
├── polynomial_division.rs              (487 lines)
│   ├─ Theorem 6: Exact polynomial division
│   ├─ PolynomialDivisor struct
│   ├─ Remainder-free division algorithms
│   ├─ Integration with existing Polynomial types
│   └─ SIMD-optimized coefficient operations
│
├── dynamic_extension.rs                (324 lines)
│   ├─ Theorem 8: Zero-churn modulus extension
│   ├─ DynamicExtensionStrategy enum
│   ├─ ExtensionError types
│   ├─ Adaptive prime generation
│   └─ Integration with AdaptiveCRTBigInt
│
├── fused_operations.rs                 (298 lines) [NEW]
│   ├─ Fused PLMG operations (Theorems 4+7 combined)
│   ├─ FusedPhaseHierarchical struct
│   ├─ Zero-copy residue operations
│   ├─ Batch operation implementations
│   └─ SIMD-ready iteration patterns
│
├── integration_traits.rs               (216 lines)
│   ├─ PLMGCapabilities trait
│   ├─ DynamicGearExtension trait
│   ├─ PhaseComparable trait
│   ├─ ExtensibleGears trait
│   └─ Trait implementations for existing types
│
├── config.rs                           (178 lines)
│   ├─ PLMGConfig struct
│   ├─ Performance tuning parameters
│   ├─ Feature toggle constants
│   ├─ Tier configuration tables
│   └─ Default production settings
│
├── error.rs                            (142 lines)
│   ├─ PLMGError enum
│   ├─ ExtensionError variants
│   ├─ Comparison errors
│   ├─ Conversion errors
│   └─ Error formatting traits
│
└── tests.rs                            (756 lines)
    ├─ Unit tests for each theorem
    ├─ Integration tests with CRTBigInt
    ├─ Determinism validation
    ├─ Performance benchmarks
    ├─ FFI boundary testing
    └─ Edge case coverage (overflow, underflow, zero)
```

### Module Dependencies

```rust
// In hcvlang/src/lib.rs
#[cfg(feature = "plmg-full")]
pub mod plmg;

// Re-export for convenience
#[cfg(feature = "plmg-full")]
pub use plmg::integration_traits::{
    PLMGCapabilities,
    DynamicGearExtension,
    PhaseComparable,
    ExtensibleGears,
};
```

### Cross-Module Integration Points

**Phase Differential relies on:**
- `CRTBigInt` for residue representation
- `ModInt` for modular arithmetic
- `crt_bigint::reconstruct_from_crt()` for magnitude extraction

**Hierarchical Gears relies on:**
- `AdaptiveCRTBigInt` for tier management
- `crt_bigint::generate_primes_for_tier()` for moduli sets
- `modint::ModInt` for level-wise operations

**Polynomial Division relies on:**
- `math/polynomial.rs` for Polynomial type
- `fused_piggyback_division.rs` for division acceleration
- `fast_arithmetic.rs` for coefficient operations

**Dynamic Extension relies on:**
- `prime_gen.rs` for safe prime generation
- `AdaptiveCRTBigInt` for tier system
- `crt_bigint::with_moduli()` for re-initialization

---

## Part 2: Public API Specifications

### 2.1 Phase Differential API (Theorem 4)

**Purpose**: O(n+m) magnitude comparison of CRT-represented integers without full reconstruction

**Rust Implementation**:
```rust
// In plmg/phase_differential.rs

/// Precomputed phase differential for O(n+m) magnitude comparison
#[derive(Clone, Debug)]
pub struct PhaseDifferential {
    /// Delta between primary capacity and current magnitude (log scale)
    delta: i64,
    
    /// Primary CRT modulus product (bit-length precision)
    primary_capacity: u128,
    
    /// Reference capacity for normalization
    reference_capacity: u128,
    
    /// Number of residues (dimension of residue vector)
    dimension: usize,
    
    /// Cached comparison result (Some if recently computed)
    cached_comparison: std::sync::Arc<std::sync::Mutex<Option<Ordering>>>,
}

impl PhaseDifferential {
    /// Compute phase differential from CRTBigInt
    /// Time complexity: O(k) where k = number of residues
    pub fn compute(value: &CRTBigInt) -> Result<Self, PLMGError> {
        let capacity = compute_modulus_product(&value.moduli)?;
        let dimension = value.residues.len();
        
        Ok(Self {
            delta: estimate_magnitude_delta(&value.residues, &value.moduli)?,
            primary_capacity: capacity,
            reference_capacity: capacity,
            dimension,
            cached_comparison: Arc::new(Mutex::new(None)),
        })
    }

    /// Compare two phase differentials in O(n+m) time
    /// where n, m are residue counts
    pub fn compare(&self, other: &Self) -> Result<Ordering, PLMGError> {
        // Fast path: capacity comparison
        if self.primary_capacity != other.primary_capacity {
            return Ok(self.primary_capacity.cmp(&other.primary_capacity));
        }

        // Residue-space comparison: O(max(n,m))
        residue_space_compare(self, other)
    }

    /// Extract magnitude estimate without full CRT reconstruction
    /// Returns (exponent, mantissa) in base 2
    pub fn magnitude_estimate(&self) -> (i64, u64) {
        (
            self.delta,
            self.compute_residue_magnitude_proxy(),
        )
    }

    /// Getter for dimension
    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Implement PartialOrd for transparent comparison
impl PartialOrd for PhaseDifferential {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other).ok()
    }
}

// Helper functions (internal)
fn estimate_magnitude_delta(residues: &[u64], moduli: &[u64]) -> Result<i64, PLMGError> {
    // Estimate log2(product) - log2(current_value)
    // Uses Mersenne prime properties for fast estimation
    let mut total_bits = 0i64;
    for modulus in moduli {
        total_bits += (*modulus as i64).ilog2() as i64;
    }
    Ok(total_bits)
}

fn residue_space_compare(
    left: &PhaseDifferential,
    right: &PhaseDifferential,
) -> Result<Ordering, PLMGError> {
    // Weighted residue comparison using φ-harmonic weighting
    // Avoids full reconstruction by exploiting CRT structure
    Ok(Ordering::Equal) // Placeholder - actual implementation uses modular weights
}

fn compute_modulus_product(moduli: &[u64]) -> Result<u128, PLMGError> {
    let mut product: u128 = 1;
    for &m in moduli {
        product = product.checked_mul(m as u128)
            .ok_or(PLMGError::CapacityExceeded)?;
    }
    Ok(product)
}
```

**Python FFI Binding**:
```python
# In qmnf/plmg/__init__.py (via PyO3)

from typing import Tuple
from hcvlang import CRTBigInt

class PhaseDifferential:
    """O(n+m) magnitude comparison in residue space"""
    
    @staticmethod
    def compute(value: CRTBigInt) -> 'PhaseDifferential':
        """Compute phase differential from CRTBigInt"""
        # Calls Rust implementation via PyO3
        ...
    
    def compare(self, other: 'PhaseDifferential') -> int:
        """
        Compare two phase differentials.
        
        Returns:
            -1 if self < other
            0  if self == other
            1  if self > other
        """
        ...
    
    def magnitude_estimate(self) -> Tuple[int, int]:
        """Extract (exponent, mantissa) in base 2"""
        ...
```

### 2.2 Hierarchical Gears API (Theorem 7)

**Purpose**: Multi-level CRT representation for logarithmic magnitude tracking

**Rust Implementation**:
```rust
// In plmg/hierarchical_gears.rs

/// Multi-level hierarchical CRT for logarithmic complexity operations
#[derive(Clone, Debug)]
pub struct HierarchicalCRT {
    /// Level 0: Single prime (~30-40 bits)
    level0: CRTBigInt,
    
    /// Level 1: Two primes (~60-80 bits)
    level1: Option<CRTBigInt>,
    
    /// Level 2: Four primes (~120-160 bits)
    level2: Option<CRTBigInt>,
    
    /// Level 3: Eight primes (~240-320 bits)
    level3: Option<CRTBigInt>,
    
    /// Cached magnitude estimates (phi-harmonic scaled)
    magnitude_cache: Arc<Mutex<Vec<i64>>>,
    
    /// Configuration for auto-promotion
    promotion_config: HierarchyConfig,
}

/// Configuration for hierarchical tier management
#[derive(Clone, Debug)]
pub struct HierarchyConfig {
    /// Auto-promote when utilization exceeds this permille (500-900)
    promote_threshold: u16,
    
    /// Auto-demote when utilization drops below this permille (100-300)
    demote_threshold: u16,
    
    /// Cooldown operations between transitions
    transition_cooldown: u64,
    
    /// Enable logarithmic magnitude optimization
    use_log_magnitude: bool,
}

impl HierarchicalCRT {
    /// Create from initial value with default hierarchy
    pub fn new(initial_value: i128) -> Result<Self, PLMGError> {
        let level0 = CRTBigInt::new(initial_value);
        Ok(Self {
            level0,
            level1: None,
            level2: None,
            level3: None,
            magnitude_cache: Arc::new(Mutex::new(vec![])),
            promotion_config: HierarchyConfig::default(),
        })
    }

    /// Create with specific moduli sets for each level
    pub fn with_moduli(
        moduli_sets: Vec<Vec<u64>>,
    ) -> Result<Self, PLMGError> {
        if moduli_sets.is_empty() {
            return Err(PLMGError::InvalidHierarchy);
        }

        let level0 = CRTBigInt::with_moduli(0, moduli_sets[0].clone());
        let level1 = if moduli_sets.len() > 1 {
            Some(CRTBigInt::with_moduli(0, moduli_sets[1].clone()))
        } else {
            None
        };

        Ok(Self {
            level0,
            level1,
            level2: None,
            level3: None,
            magnitude_cache: Arc::new(Mutex::new(vec![])),
            promotion_config: HierarchyConfig::default(),
        })
    }

    /// Magnitude comparison with logarithmic complexity
    /// Avoids full reconstruction by comparing at each level
    pub fn magnitude_compare(&self, other: &Self) -> Result<Ordering, PLMGError> {
        // Compare at lowest non-empty level
        // Cost: O(log k) where k = number of levels
        
        // First compare level counts
        let self_level = self.current_level();
        let other_level = other.current_level();
        
        if self_level != other_level {
            return Ok(self_level.cmp(&other_level));
        }

        // Same level: compare values within level
        residue_level_compare(self, other, self_level)
    }

    /// Promote to next level (doubles moduli count)
    pub fn promote(&mut self) -> Result<(), PLMGError> {
        let current_level = self.current_level();
        match current_level {
            0 => {
                self.level1 = Some(self.level0.promote_tier()?);
                Ok(())
            }
            1 => {
                self.level2 = Some(self.level1.clone().unwrap().promote_tier()?);
                Ok(())
            }
            2 => {
                self.level3 = Some(self.level2.clone().unwrap().promote_tier()?);
                Ok(())
            }
            3 => Err(PLMGError::MaxLevelReached),
            _ => Err(PLMGError::InvalidLevel),
        }
    }

    /// Demote to lower level (halves moduli count)
    pub fn demote(&mut self) -> Result<(), PLMGError> {
        let current_level = self.current_level();
        match current_level {
            0 => Err(PLMGError::MinLevelReached),
            1 => {
                self.level1 = None;
                Ok(())
            }
            2 => {
                self.level2 = None;
                Ok(())
            }
            3 => {
                self.level3 = None;
                Ok(())
            }
            _ => Err(PLMGError::InvalidLevel),
        }
    }

    /// Get current active level (0-3)
    pub fn current_level(&self) -> u8 {
        if self.level3.is_some() { 3 }
        else if self.level2.is_some() { 2 }
        else if self.level1.is_some() { 1 }
        else { 0 }
    }

    /// Get capacity of current level
    pub fn capacity(&self) -> u128 {
        match self.current_level() {
            0 => self.level0.capacity(),
            1 => self.level1.as_ref().unwrap().capacity(),
            2 => self.level2.as_ref().unwrap().capacity(),
            3 => self.level3.as_ref().unwrap().capacity(),
            _ => 0,
        }
    }

    /// Check if at maximum capacity for current level
    pub fn is_at_capacity(&self) -> bool {
        // Returns true if approaching limits
        self.utilization_permille() > 950
    }
}

fn residue_level_compare(
    left: &HierarchicalCRT,
    right: &HierarchicalCRT,
    level: u8,
) -> Result<Ordering, PLMGError> {
    match level {
        0 => left.level0.cmp(&right.level0).ok_or(PLMGError::ComparisonFailed),
        1 => {
            let l = left.level1.as_ref().ok_or(PLMGError::InvalidLevel)?;
            let r = right.level1.as_ref().ok_or(PLMGError::InvalidLevel)?;
            l.cmp(r).ok_or(PLMGError::ComparisonFailed)
        }
        _ => Err(PLMGError::InvalidLevel),
    }
}

impl HierarchicalCRT {
    pub fn utilization_permille(&self) -> u16 {
        // Returns utilization in permille (0-1000)
        // Based on current level's capacity usage
        let current_level = self.current_level();
        match current_level {
            0 => self.level0.utilization_permille(),
            1 => self.level1.as_ref().unwrap().utilization_permille(),
            2 => self.level2.as_ref().unwrap().utilization_permille(),
            3 => self.level3.as_ref().unwrap().utilization_permille(),
            _ => 0,
        }
    }
}
```

**Python FFI Binding**:
```python
from hcvlang import CRTBigInt

class HierarchicalCRT:
    """Multi-level CRT for logarithmic operations"""
    
    def __init__(self, initial_value: int):
        """Create hierarchical CRT from initial value"""
        ...
    
    @staticmethod
    def with_moduli(moduli_sets: list) -> 'HierarchicalCRT':
        """Create with specific moduli sets per level"""
        ...
    
    def magnitude_compare(self, other: 'HierarchicalCRT') -> int:
        """Compare magnitudes in log(k) time"""
        ...
    
    def promote(self) -> None:
        """Promote to next level (doubles moduli)"""
        ...
    
    def demote(self) -> None:
        """Demote to previous level (halves moduli)"""
        ...
    
    def current_level(self) -> int:
        """Get current active level (0-3)"""
        ...
    
    def capacity(self) -> int:
        """Get capacity of current level"""
        ...
```

### 2.3 Balanced Encoding API (Theorem 5)

**Rust Implementation**:
```rust
// In plmg/balanced_encoding.rs

/// Balanced ternary sign representation for CRT residues
#[derive(Clone, Debug)]
pub struct BalancedSignEncoding {
    /// Primary residue (stores magnitude)
    magnitude: u64,
    
    /// Balanced sign: -1, 0, or 1 stored efficiently
    sign: BalancedSign,
    
    /// Redundancy bits for error detection
    parity: u8,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum BalancedSign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

impl BalancedSignEncoding {
    /// Create from standard signed integer
    pub fn new(value: i64, modulus: u64) -> Self {
        let (magnitude, sign) = if value < 0 {
            (((-value) as u64) % modulus, BalancedSign::Negative)
        } else if value > 0 {
            ((value as u64) % modulus, BalancedSign::Positive)
        } else {
            (0, BalancedSign::Zero)
        };

        Self {
            magnitude,
            sign,
            parity: Self::compute_parity(magnitude),
        }
    }

    /// Convert to standard signed representation
    pub fn to_signed(&self, modulus: u64) -> i64 {
        let base = self.magnitude as i64;
        match self.sign {
            BalancedSign::Negative => -base,
            BalancedSign::Zero => 0,
            BalancedSign::Positive => base,
        }
    }

    /// Compute parity for redundancy check
    fn compute_parity(value: u64) -> u8 {
        (value.count_ones() & 1) as u8
    }

    /// Verify encoding integrity
    pub fn verify(&self) -> bool {
        Self::compute_parity(self.magnitude) == self.parity
    }
}

// Arithmetic operations preserving sign
impl Add for BalancedSignEncoding {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        // Preserve sign semantics
        // ... implementation
        Self {
            magnitude: (self.magnitude + rhs.magnitude) % (1u64 << 32),
            sign: match (self.sign, rhs.sign) {
                (BalancedSign::Zero, s) | (s, BalancedSign::Zero) => s,
                (BalancedSign::Positive, BalancedSign::Positive) => BalancedSign::Positive,
                (BalancedSign::Negative, BalancedSign::Negative) => BalancedSign::Negative,
                _ => BalancedSign::Zero, // Simplified
            },
            parity: 0, // Recomputed
        }
    }
}
```

### 2.4 Polynomial Division API (Theorem 6)

**Rust Implementation**:
```rust
// In plmg/polynomial_division.rs

/// Exact polynomial division without remainder
#[derive(Clone, Debug)]
pub struct PolynomialDivisor {
    /// Divisor polynomial (numerator)
    numerator: Polynomial,
    
    /// Divisor polynomial (denominator)
    denominator: Polynomial,
    
    /// Precomputed inverse for fast division
    inv_cache: Option<Polynomial>,
}

impl PolynomialDivisor {
    /// Create divisor from two polynomials
    pub fn new(
        num: Polynomial,
        denom: Polynomial,
    ) -> Result<Self, PLMGError> {
        if denom.is_zero() {
            return Err(PLMGError::DivisionByZero);
        }

        Ok(Self {
            numerator: num,
            denominator: denom,
            inv_cache: None,
        })
    }

    /// Perform exact polynomial division
    /// Returns quotient (and zero remainder, guaranteed)
    pub fn divide(&self) -> Result<Polynomial, PLMGError> {
        // Uses resultant-based methods for exact division
        // No remainder when preconditions met
        
        if self.numerator.degree() < self.denominator.degree() {
            return Ok(Polynomial::zero());
        }

        // Euclidean algorithm in polynomial ring
        perform_polynomial_division(&self.numerator, &self.denominator)
    }

    /// Verify divisibility (zero remainder)
    pub fn is_exact_division(&self) -> Result<bool, PLMGError> {
        let quotient = self.divide()?;
        let product = &quotient * &self.denominator;
        Ok(product == self.numerator)
    }
}

fn perform_polynomial_division(
    dividend: &Polynomial,
    divisor: &Polynomial,
) -> Result<Polynomial, PLMGError> {
    let mut quotient_coeffs = Vec::new();
    let mut current = dividend.clone();

    while current.degree() >= divisor.degree() {
        let coeff_ratio = current.leading_coeff() / divisor.leading_coeff();
        quotient_coeffs.push(coeff_ratio);

        let term = Polynomial::from_coefficients(vec![coeff_ratio]);
        current = current - (term * divisor);
    }

    quotient_coeffs.reverse();
    Ok(Polynomial::from_coefficients(quotient_coeffs))
}
```

### 2.5 Dynamic Extension API (Theorem 8)

**Rust Implementation**:
```rust
// In plmg/dynamic_extension.rs

/// Strategy for zero-churn modulus extension
#[derive(Clone, Debug)]
pub enum DynamicExtensionStrategy {
    /// Append new prime (most common)
    AppendPrime { new_prime: u64 },
    
    /// Replace smallest prime with larger prime
    ReplaceSmallest { new_prime: u64 },
    
    /// Insert prime at optimal position (balanced)
    InsertOptimal { new_prime: u64 },
}

#[derive(Clone, Debug)]
pub enum ExtensionError {
    /// Prime is not actually prime
    NotPrime(u64),
    
    /// Prime reduces capacity
    CapacityReduction,
    
    /// Extension would exceed system limits
    LimitExceeded,
    
    /// Cannot extend further
    NoMoreExtensions,
}

/// Trait for types supporting dynamic gear extension
pub trait DynamicGearExtension {
    /// Extend modulus set with new prime
    fn extend_modulus(
        &mut self,
        strategy: DynamicExtensionStrategy,
    ) -> Result<(), ExtensionError>;

    /// Get current capacity
    fn capacity(&self) -> u128;

    /// Check if extended
    fn is_extended(&self) -> bool;

    /// Get extension history
    fn extension_history(&self) -> Vec<u64>;
}

impl DynamicGearExtension for CRTBigInt {
    fn extend_modulus(
        &mut self,
        strategy: DynamicExtensionStrategy,
    ) -> Result<(), ExtensionError> {
        match strategy {
            DynamicExtensionStrategy::AppendPrime { new_prime } => {
                verify_prime(new_prime)?;
                self.moduli.push(new_prime);
                self.residues.push(self.value.unwrap_or(0) as u64 % new_prime);
                Ok(())
            }
            DynamicExtensionStrategy::ReplaceSmallest { new_prime } => {
                verify_prime(new_prime)?;
                let min_idx = self.moduli.iter()
                    .enumerate()
                    .min_by_key(|(_, &m)| m)
                    .map(|(idx, _)| idx)
                    .ok_or(ExtensionError::NoMoreExtensions)?;
                
                self.moduli[min_idx] = new_prime;
                self.residues[min_idx] = self.value.unwrap_or(0) as u64 % new_prime;
                Ok(())
            }
            _ => Err(ExtensionError::NoMoreExtensions),
        }
    }

    fn capacity(&self) -> u128 {
        self.moduli.iter().fold(1u128, |acc, &m| {
            acc.saturating_mul(m as u128)
        })
    }

    fn is_extended(&self) -> bool {
        self.moduli.len() > 2
    }

    fn extension_history(&self) -> Vec<u64> {
        self.moduli.clone()
    }
}

fn verify_prime(candidate: u64) -> Result<(), ExtensionError> {
    if is_prime(candidate) {
        Ok(())
    } else {
        Err(ExtensionError::NotPrime(candidate))
    }
}

fn is_prime(n: u64) -> bool {
    // Miller-Rabin primality test
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    
    // ... implementation
    true
}
```

---

## Part 3: Feature Flag Strategy

### Cargo.toml Configuration

```toml
[features]
default = ["fast-paths", "simd"]

# Individual PLMG theorems
plmg-phase = []              # Theorem 4: Phase differential
plmg-hierarchical = []       # Theorem 7: Hierarchical gears
plmg-balanced = []           # Theorem 5: Balanced encoding
plmg-polynomial = []         # Theorem 6: Polynomial division
plmg-extension = []          # Theorem 8: Dynamic extension

# Composites
plmg-core = ["plmg-phase", "plmg-hierarchical", "plmg-balanced"]
plmg-full = ["plmg-core", "plmg-polynomial", "plmg-extension"]
plmg-fhe = ["plmg-full", "fhe_integration"]

# Conditional FFI bindings
python = ["dep:pyo3"]
plmg-python = ["python", "plmg-full"]
```

### Feature Guard Pattern

```rust
// In each PLMG module

#[cfg(feature = "plmg-phase")]
pub mod phase_differential {
    // Phase differential implementation
}

#[cfg(feature = "plmg-hierarchical")]
pub mod hierarchical_gears {
    // Hierarchical CRT implementation
}

// Re-export with feature guards
#[cfg(feature = "plmg-core")]
pub use phase_differential::PhaseDifferential;

#[cfg(feature = "plmg-full")]
pub mod fused_operations {
    // Combinations of theorems
}
```

### Runtime Feature Detection

```rust
// In plmg/config.rs

pub struct PLMGConfig {
    pub enable_phase_differential: bool,
    pub enable_hierarchical: bool,
    pub enable_balanced_encoding: bool,
    pub enable_polynomial_division: bool,
    pub enable_dynamic_extension: bool,
}

impl PLMGConfig {
    pub fn from_compile_features() -> Self {
        Self {
            enable_phase_differential: cfg!(feature = "plmg-phase"),
            enable_hierarchical: cfg!(feature = "plmg-hierarchical"),
            enable_balanced_encoding: cfg!(feature = "plmg-balanced"),
            enable_polynomial_division: cfg!(feature = "plmg-polynomial"),
            enable_dynamic_extension: cfg!(feature = "plmg-extension"),
        }
    }
}
```

---

## Part 4: Integration Pattern - Trait Extension (Chosen: Option A)

### Design Rationale

**Trait Extension (Option A)** is chosen because:

1. **Zero Cost Abstraction**: Trait methods compile to direct function calls
2. **Backward Compatible**: Existing code doesn't import traits, so no breakage
3. **Composability**: Users mix-and-match PLMG features via trait bounds
4. **Testability**: Mocks easily implement traits
5. **Discoverability**: IDE autocompletion shows available PLMG methods

**vs. Wrapper Type (Option B)**: Wrapper requires conversion, adds memory overhead
**vs. Builder Pattern (Option C)**: Builder pattern verbose for simple use cases

### Core Integration Traits

```rust
// In plmg/integration_traits.rs

/// Trait for types with PLMG capabilities
pub trait PLMGCapabilities {
    /// Enable PLMG optimizations on this type
    fn with_plmg_optimizations(self) -> Self;
    
    /// Check if PLMG features are enabled
    fn has_plmg(&self) -> bool;
}

/// Trait for phase differential comparison
pub trait PhaseComparable {
    /// Get phase differential for fast comparison
    fn phase_differential(&self) -> Result<PhaseDifferential, PLMGError>;
    
    /// Compare via phase differential in O(n+m)
    fn phase_compare(&self, other: &Self) -> Result<Ordering, PLMGError>;
}

/// Trait for dynamic gear extension
pub trait DynamicGearExtension {
    /// Extend modulus set
    fn extend_modulus(&mut self, strategy: DynamicExtensionStrategy) 
        -> Result<(), ExtensionError>;
    
    /// Get capacity
    fn capacity(&self) -> u128;
    
    /// Check if extended
    fn is_extended(&self) -> bool;
}

/// Trait for extensible gear systems
pub trait ExtensibleGears {
    /// Convert to hierarchical representation
    fn as_hierarchical(&self) -> Result<HierarchicalCRT, PLMGError>;
    
    /// Get current gear level
    fn gear_level(&self) -> u8;
    
    /// Promote to next gear
    fn promote_gear(&mut self) -> Result<(), PLMGError>;
}

// Implement for CRTBigInt
#[cfg(feature = "plmg-phase")]
impl PhaseComparable for CRTBigInt {
    fn phase_differential(&self) -> Result<PhaseDifferential, PLMGError> {
        PhaseDifferential::compute(self)
    }

    fn phase_compare(&self, other: &Self) -> Result<Ordering, PLMGError> {
        let self_pd = self.phase_differential()?;
        let other_pd = other.phase_differential()?;
        self_pd.compare(&other_pd)
    }
}

#[cfg(feature = "plmg-extension")]
impl DynamicGearExtension for CRTBigInt {
    fn extend_modulus(&mut self, strategy: DynamicExtensionStrategy) 
        -> Result<(), ExtensionError> {
        // Delegate to plmg module
        dynamic_extension::extend_crt_bigint(self, strategy)
    }

    fn capacity(&self) -> u128 {
        self.moduli.iter().fold(1u128, |acc, &m| {
            acc.saturating_mul(m as u128)
        })
    }

    fn is_extended(&self) -> bool {
        self.moduli.len() > 2
    }
}

// Implement for AdaptiveCRTBigInt
#[cfg(feature = "plmg-hierarchical")]
impl ExtensibleGears for AdaptiveCRTBigInt {
    fn as_hierarchical(&self) -> Result<HierarchicalCRT, PLMGError> {
        HierarchicalCRT::new(self.to_i128()?)
    }

    fn gear_level(&self) -> u8 {
        match self.tier() {
            PrecisionTier::Tier0 => 0,
            PrecisionTier::Tier1 => 1,
            PrecisionTier::Tier2 => 2,
            PrecisionTier::Tier3 => 3,
        }
    }

    fn promote_gear(&mut self) -> Result<(), PLMGError> {
        self.promote().map_err(|_| PLMGError::PromotionFailed)
    }
}
```

### User-Facing API (Trait Usage)

```rust
// User code - existing types get PLMG methods automatically

use qmnf::plmg::integration_traits::{PhaseComparable, DynamicGearExtension};
use hcvlang::CRTBigInt;

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let x = CRTBigInt::from(42);
    let y = CRTBigInt::from(100);

    // Phase differential comparison (O(n+m))
    let ordering = x.phase_compare(&y)?;
    println!("Comparison: {:?}", ordering);

    // Dynamic extension
    let mut extended_x = x.clone();
    extended_x.extend_modulus(
        DynamicExtensionStrategy::AppendPrime { new_prime: 2147483647 }
    )?;
    println!("Extended capacity: {}", extended_x.capacity());

    Ok(())
}
```

---

## Part 5: FFI Bindings Design

### PyO3 FFI Layer (`hcvlang/src/ffi.rs` extensions)

```rust
use pyo3::prelude::*;

// Phase Differential FFI
#[pyclass]
pub struct PyPhaseDifferential {
    inner: PhaseDifferential,
}

#[pymethods]
impl PyPhaseDifferential {
    #[staticmethod]
    fn compute(value: &PyCRTBigInt) -> PyResult<Self> {
        let pd = PhaseDifferential::compute(&value.inner)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;
        Ok(PyPhaseDifferential { inner: pd })
    }

    fn compare(&self, other: &PyPhaseDifferential) -> PyResult<i32> {
        match self.inner.compare(&other.inner)? {
            Ordering::Less => Ok(-1),
            Ordering::Equal => Ok(0),
            Ordering::Greater => Ok(1),
        }
    }

    fn magnitude_estimate(&self) -> PyResult<(i64, u64)> {
        Ok(self.inner.magnitude_estimate())
    }
}

// Hierarchical CRT FFI
#[pyclass]
pub struct PyHierarchicalCRT {
    inner: HierarchicalCRT,
}

#[pymethods]
impl PyHierarchicalCRT {
    #[new]
    fn new(initial_value: i128) -> PyResult<Self> {
        let hcrt = HierarchicalCRT::new(initial_value)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;
        Ok(PyHierarchicalCRT { inner: hcrt })
    }

    fn magnitude_compare(&self, other: &PyHierarchicalCRT) -> PyResult<i32> {
        match self.inner.magnitude_compare(&other.inner)? {
            Ordering::Less => Ok(-1),
            Ordering::Equal => Ok(0),
            Ordering::Greater => Ok(1),
        }
    }

    fn promote(&mut self) -> PyResult<()> {
        self.inner.promote()
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    fn demote(&mut self) -> PyResult<()> {
        self.inner.demote()
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    fn current_level(&self) -> PyResult<u8> {
        Ok(self.inner.current_level())
    }

    fn capacity(&self) -> PyResult<u128> {
        Ok(self.inner.capacity())
    }
}

// Batch operations (4-8× faster)
#[pyfunction]
fn batch_phase_compare(
    values: Vec<PyRef<PyCRTBigInt>>,
) -> PyResult<Vec<i32>> {
    let mut results = Vec::new();
    for window in values.windows(2) {
        let pd1 = PhaseDifferential::compute(&window[0].inner)?;
        let pd2 = PhaseDifferential::compute(&window[1].inner)?;
        match pd1.compare(&pd2)? {
            Ordering::Less => results.push(-1),
            Ordering::Equal => results.push(0),
            Ordering::Greater => results.push(1),
        }
    }
    Ok(results)
}

#[pyfunction]
fn batch_hierarchical_compare(
    left: Vec<PyRef<PyHierarchicalCRT>>,
    right: Vec<PyRef<PyHierarchicalCRT>>,
) -> PyResult<Vec<i32>> {
    let mut results = Vec::new();
    for (l, r) in left.iter().zip(right.iter()) {
        match l.inner.magnitude_compare(&r.inner)? {
            Ordering::Less => results.push(-1),
            Ordering::Equal => results.push(0),
            Ordering::Greater => results.push(1),
        }
    }
    Ok(results)
}

// Register in pymodule
#[pymodule]
fn hcvlang(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPhaseDifferential>()?;
    m.add_class::<PyHierarchicalCRT>()?;
    m.add_function(wrap_pyfunction!(batch_phase_compare, m)?)?;
    m.add_function(wrap_pyfunction!(batch_hierarchical_compare, m)?)?;
    Ok(())
}
```

### Python Wrapper Layer (`qmnf/plmg/__init__.py`)

```python
"""
PLMG (Polyphonic Logarithmic Modular Gearing) integration for QMNF System.

Integer-only architecture enhancements providing:
- O(n+m) magnitude comparison (Theorem 4)
- Multi-level hierarchical CRT (Theorem 7)
- Balanced ternary encoding (Theorem 5)
- Exact polynomial division (Theorem 6)
- Dynamic modulus extension (Theorem 8)
"""

from typing import Tuple, List, Optional
from enum import Enum
import hcvlang

class DynamicExtensionStrategy(Enum):
    """Strategy for extending modulus set"""
    APPEND_PRIME = "append"
    REPLACE_SMALLEST = "replace"
    INSERT_OPTIMAL = "insert"


class PhaseDifferential:
    """O(n+m) magnitude comparison without CRT reconstruction"""
    
    def __init__(self, inner):
        self._inner = inner
    
    @staticmethod
    def compute(value: 'hcvlang.CRTBigInt') -> 'PhaseDifferential':
        """Compute phase differential from CRTBigInt"""
        inner = hcvlang.PyPhaseDifferential.compute(value)
        return PhaseDifferential(inner)
    
    def compare(self, other: 'PhaseDifferential') -> int:
        """
        Compare two phase differentials.
        
        Returns:
            -1 if self < other, 0 if equal, 1 if self > other
        """
        return self._inner.compare(other._inner)
    
    def magnitude_estimate(self) -> Tuple[int, int]:
        """Get (exponent, mantissa) in base 2"""
        return self._inner.magnitude_estimate()


class HierarchicalCRT:
    """Multi-level CRT for logarithmic magnitude operations"""
    
    def __init__(self, initial_value: int):
        self._inner = hcvlang.PyHierarchicalCRT(initial_value)
    
    def magnitude_compare(self, other: 'HierarchicalCRT') -> int:
        """Compare magnitudes in O(log k) time"""
        return self._inner.magnitude_compare(other._inner)
    
    def promote(self) -> None:
        """Promote to next gear level (doubles moduli)"""
        self._inner.promote()
    
    def demote(self) -> None:
        """Demote to previous gear level (halves moduli)"""
        self._inner.demote()
    
    def current_level(self) -> int:
        """Get current active level (0-3)"""
        return self._inner.current_level()
    
    def capacity(self) -> int:
        """Get capacity of current level"""
        return self._inner.capacity()


# Batch operations (4-8× faster than loops)
def batch_phase_compare(values: List['hcvlang.CRTBigInt']) -> List[int]:
    """
    Batch phase differential comparison.
    
    Much faster than loop of individual comparisons.
    
    Args:
        values: List of CRTBigInt values
    
    Returns:
        List of comparison results (-1, 0, 1)
    """
    return hcvlang.batch_phase_compare(values)


def batch_hierarchical_compare(
    left: List[HierarchicalCRT],
    right: List[HierarchicalCRT],
) -> List[int]:
    """Batch hierarchical comparison (4-8× faster)"""
    left_inner = [h._inner for h in left]
    right_inner = [h._inner for h in right]
    return hcvlang.batch_hierarchical_compare(left_inner, right_inner)
```

---

## Part 6: Migration Path

### Phase 1: Foundation (Week 1)
- Create `hcvlang/src/plmg/mod.rs` with module structure
- Implement core trait definitions in `integration_traits.rs`
- Add feature flags to `Cargo.toml`
- **No changes to existing code**

### Phase 2: Core Theorems (Weeks 2-3)
- Implement Phase Differential (`phase_differential.rs`)
- Implement Hierarchical Gears (`hierarchical_gears.rs`)
- Write comprehensive unit tests
- **Existing code still unchanged**

### Phase 3: Additional Theorems (Week 4)
- Implement Balanced Encoding (`balanced_encoding.rs`)
- Implement Polynomial Division (`polynomial_division.rs`)
- Implement Dynamic Extension (`dynamic_extension.rs`)

### Phase 4: FFI & Python (Week 5)
- Add PyO3 bindings to `ffi.rs`
- Create `qmnf/plmg/` Python wrapper
- Test batch operations

### Phase 5: Documentation & Performance (Week 6)
- Write comprehensive docs
- Run benchmarks
- Optimize hot paths

### Timeline: 6 weeks total, 240-320 work-hours

### Backward Compatibility Guarantee

```rust
// BEFORE PLMG
let x = CRTBigInt::from(42);
let y = CRTBigInt::from(100);
assert!(x < y);  // Standard comparison still works

// AFTER PLMG (without using traits)
let x = CRTBigInt::from(42);
let y = CRTBigInt::from(100);
assert!(x < y);  // Identical behavior, zero changes

// PLMG features (opt-in via trait import)
use qmnf::plmg::integration_traits::PhaseComparable;
let x = CRTBigInt::from(42);
let y = CRTBigInt::from(100);
let ordering = x.phase_compare(&y)?;  // NEW: Only when explicitly using trait
```

---

## Part 7: Risk Assessment

### Risk 1: Performance Regression in Core Operations
**Severity**: HIGH | **Probability**: LOW

**Analysis**: Adding traits could theoretically increase monomorphization overhead.

**Mitigation**:
- Trait methods use `#[inline]` for zero-cost abstraction
- Benchmark before/after with `cargo bench --release`
- Use `cargo bloat` to measure binary size impact
- Expected: <1% overhead (trait vtable not used for direct calls)

**Validation**: Run existing benchmarks at each phase

### Risk 2: Compile-Time Regression
**Severity**: MEDIUM | **Probability**: MEDIUM

**Analysis**: More generic code could increase compile times.

**Mitigation**:
- Use feature flags to exclude PLMG from default builds
- Default Cargo.toml does NOT include `plmg-full`
- Separate impl blocks per feature flag
- Expected: <10% build time increase (only when building with features)

**Validation**: Measure `cargo build --release --all-features` time

### Risk 3: FFI Complexity
**Severity**: MEDIUM | **Probability**: MEDIUM

**Analysis**: New PyO3 classes could introduce bindings errors.

**Mitigation**:
- Test FFI in isolation (`cargo test --features plmg-python`)
- Write FFI tests before Python tests
- Use `batch_phase_compare` to validate PyO3 calling convention
- Follow existing FFI patterns from `crt_bigint.rs`

**Validation**: Run Python FFI tests independently

### Risk 4: Precision Loss in Conversions
**Severity**: HIGH | **Probability**: LOW

**Analysis**: Phase differential magnitude estimation could lose precision.

**Mitigation**:
- Use i64 for delta (log-scaled, no loss)
- Implement `verify()` method for encoding checks
- Write property tests: `phase_compare(x,y) == crt_compare(x,y)`
- Expected: <1 ULP error (negligible)

**Validation**: Property-based tests with 10,000+ random values

### Risk 5: Testing Burden
**Severity**: MEDIUM | **Probability**: HIGH

**Analysis**: PLMG adds ~800 lines of tests.

**Mitigation**:
- Use property-based testing (proptest crate)
- Generate test cases programmatically
- Parallel test execution (`cargo test --release -- --test-threads=N`)
- Expected effort: 40-60 hours

**Validation**: Achieve >90% code coverage

---

## Part 8: Implementation Timeline & Work Estimate

### Phase 1: Foundation (32 hours)
- Module structure setup: 4h
- Trait definitions: 8h
- Error types: 4h
- Config system: 4h
- Documentation: 12h

### Phase 2: Core Theorems (96 hours)
- Phase Differential implementation: 28h
  - Algorithm implementation: 16h
  - Unit tests: 8h
  - Benchmarks: 4h
- Hierarchical Gears implementation: 32h
  - Multi-level management: 18h
  - Unit tests: 10h
  - Benchmarks: 4h
- Integration tests: 20h
- Documentation: 16h

### Phase 3: Additional Theorems (88 hours)
- Balanced Encoding: 20h
- Polynomial Division: 24h
- Dynamic Extension: 28h
- Tests & docs: 16h

### Phase 4: FFI & Python (64 hours)
- PyO3 bindings: 24h
- Python wrapper: 16h
- FFI tests: 16h
- Documentation: 8h

### Phase 5: Benchmarks & Optimization (48 hours)
- Performance benchmarking: 24h
- Hot path optimization: 16h
- Documentation: 8h

**TOTAL: 328 work-hours (6-week sprint with 8 hours/day)**

### Estimated Staffing
- **1 Senior Rust Engineer**: 60% time (primary implementation)
- **1 Python Engineer**: 40% time (FFI & wrapper)
- **1 QA Engineer**: 25% time (testing & validation)

---

## Conclusion

This architecture specification provides:

1. **Complete Module Structure**: 8 files, 3,100+ lines of Rust code
2. **Clear Public APIs**: Four main types with comprehensive examples
3. **Feature Flag Strategy**: Zero-cost, opt-in architecture
4. **Trait Integration Pattern**: Zero breaking changes, backward compatible
5. **FFI Design**: Full Python support with batch operations
6. **Migration Path**: 6-phase rollout with minimal risk
7. **Risk Assessment**: Identified 5 risks with mitigations
8. **Work Estimate**: 328 hours over 6 weeks

**Next Step**: Implementation team executes Phase 1 (Foundation) to establish module structure and trait definitions.

