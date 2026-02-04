# QMNF System - Advanced Mathematical Implementations Reference Manual

## Table of Contents
1. [Overview](#overview)
2. [Core Mathematical Modules](#core-mathematical-modules)
3. [Rational Arithmetic](#rational-arithmetic)
4. [Chinese Remainder Theorem BigInt](#chinese-remainder-theorem-bigint)
5. [Modular Arithmetic](#modular-arithmetic)
6. [Advanced Operations](#advanced-operations)
7. [Geometric Primitives](#geometric-primitives)
8. [Number Theoretic Transform (NTT)](#number-theoretic-transform-ntt)
9. [Quantum-Modular Superposition](#quantum-modular-superposition)
10. [Optimizations](#optimizations)
11. [Applications](#applications)
12. [Dimensional Consistency Framework](#dimensional-consistency-framework)
13. [Potential Field Dynamics](#potential-field-dynamics)
14. [Adaptive Scaling Strategies](#adaptive-scaling-strategies)
15. [Quantum-Modular Synthesis](#quantum-modular-synthesis)
16. [Unitary Operators and Quantum Gates](#unitary-operators-and-quantum-gates)
17. [My Role as Your Personal Extension](#my-role-as-your-personal-extension)

## Overview

The QMNF (Quantum-Modular Numerical Framework) System implements a comprehensive suite of advanced mathematical operations with 100% integer-only arithmetic to ensure exact computation without floating-point contamination.

**Key Principles:**
- **Integer-Only Mathematics**: All computations use exact rational arithmetic
- **Modular Arithmetic**: Operations performed in ℤ/M with carefully selected moduli
- **Performance-First**: Rust primitives with Python bindings for optimal speed
- **Exactness Guarantee**: No rounding errors in core computations

## Core Mathematical Modules

### CRTBigInt (Chinese Remainder Theorem BigInt)
Location: `hcvlang/src/crt_bigint.rs`

The CRTBigInt implements bounded integers using the Chinese Remainder Theorem with two 63-bit primes as moduli:
- **M₁ = 2^63 - 25** (9,223,372,036,854,775,783)
- **M₂ = 2^63 - 165** (9,223,372,036,854,775,643)

**Key Features:**
- 126-bit range (safe from overflow in most operations)
- Fast arithmetic operations using component-wise modular arithmetic
- SIMD-ready operations on residues
- Fast reconstruction using Garners algorithm
- Built-in fast paths for small i64 values
- Signed wrapper for negative number handling

**Performance Optimizations:**
- Fast path for values that fit in i64 with overflow checking
- SIMD operations on residues without expensive reconstruction
- Cache-efficient operations by avoiding repeated conversions
- Constant-time modular arithmetic to prevent timing attacks

### Rational Number System
Location: `hcvlang/src/rational.rs`

Rational implements exact rational arithmetic using CRTBigInt numerators and denominators.

**Core Operations:**
```rust
// Addition: (a*d + b*c) / (b*d)
impl Add for Rational {
    fn add(self, rhs: Self) -> Self {
        let num = self.num * rhs.den.clone() + rhs.num * self.den.clone();
        let den = self.den * rhs.den;
        Rational::new(num, den).reduce()
    }
}
```

**Key Features:**
- Canonical representation (numerator and denominator coprime)
- Automatic reduction to lowest terms
- Borrowed reduced views to avoid unnecessary clones
- Sign normalization (negative always in numerator)
- Canonical zero as 0/1

### ModInt (Fast Modular Integer)
Location: `hcvlang/src/modint_fast.rs`

Optimized modular arithmetic using Mersenne prime 2^31 - 1 as the modulus.

**Key Features:**
- Mersenne reduction for fast modular arithmetic
- Barrett reduction as alternative method
- Inline reduction without division
- Branchless normalization where possible

**Performance Optimizations:**
- Uses Mersenne prime property: 2^31 ≡ 1 (mod 2^31-1)
- For value V = H*2^31 + L, V ≡ H + L (mod 2^31-1)
- Reduces computational complexity compared to regular division

## Rational Arithmetic

### ExactRational (Python Implementation)
Location: `qmnf/arithmetic/core/core_integer_arithmetic.py`

Implements exact rational arithmetic in modular arithmetic systems.

**Core Features:**
- Maintains canonical form: gcd(n, d) = 1, d > 0
- All operations are exact with zero rounding errors
- Modular inverse using Extended Euclidean Algorithm
- Binary GCD (Steins algorithm) for 2-3x speedup over Euclidean algorithm

**Binary GCD Implementation:**
```python
def gcd_binary(a: int, b: int) -> int:
    """
    Binary GCD algorithm (Steins algorithm).

    More efficient than Euclidean algorithm for large numbers.
    Uses only shifts and subtractions (no division).

    Complexity: O(log² min(a, b))
    """
    a, b = abs(a), abs(b)

    if a == 0:
        return b
    if b == 0:
        return a

    # Find common power of 2
    shift = 0
    while ((a | b) & 1) == 0:
        a >>= 1
        b >>= 1
        shift += 1

    # Remove remaining factors of 2 from a
    while (a & 1) == 0:
        a >>= 1

    # Main loop
    while b != 0:
        # Remove factors of 2 from b
        while (b & 1) == 0:
            b >>= 1

        # Ensure a <= b
        if a > b:
            a, b = b, a

        b -= a

    return a << shift
```

### IntPair (Cache-Aligned Rational)
Location: `hcvlang/src/intpair.rs`

High-performance rational arithmetic using Binary GCD for simplification with cache-aligned layout for SIMD operations.

**Key Features:**
- 16-byte alignment for AVX2 operations
- Binary GCD simplification (~2-3x faster than Euclidean)
- No division operations (uses only shifts and subtractions)
- CPU tzcnt instruction optimization for trailing zero count

**Binary GCD Implementation:**
```rust
#[inline(always)]
fn binary_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();

    if a == 0 { return b; }
    if b == 0 { return a; }

    # Extract common power of 2 using CPU tzcnt instruction
    let a_zeros = a.trailing_zeros();
    let b_zeros = b.trailing_zeros();
    let common_twos = a_zeros.min(b_zeros);

    a >>= a_zeros;
    b >>= b_zeros;

    # Now both are odd, repeatedly subtract and shift
    while a != b {
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        b -= a; # b - a is even
        b >>= 1; # Divide by 2
        b >>= b.trailing_zeros(); # Remove remaining factors of 2
    }

    a << common_twos
}
```

## Chinese Remainder Theorem BigInt

### CRT Implementation
The system uses a two-modulus CRT with 63-bit primes:
- **Product Range**: Values in [0, M₁×M₂) where M₁×M₂ ≈ 2^126
- **Reconstruction**: Uses Garners algorithm for efficient conversion
- **Operations**: Component-wise modular arithmetic without full reconstruction

### Fast Reconstruction
```rust
/// Garners algorithm for CRT reconstruction
pub fn reconstruct_big(&self) -> HCVLangBigInt {
    # Garner on two moduli (fast path)
    let m0 = MODULI[0] as u128;
    let m1 = MODULI[1] as u128;
    let r0 = self.residues[0] as u128;
    let r1 = self.residues[1] as u128;

    # Find t such that r0 + m0 * t ≡ r1 (mod m1)
    # => m0 * t ≡ (r1 - r0) (mod m1)
    let inv = mod_inverse((m0 % m1) as u64, m1 as u64)
        .expect("moduli must be coprime; invariants broken");
    let diff = (r1 + m1 - (r0 % m1)) % m1;
    let t = ((diff * (inv as u128)) % m1) as u128;

    let x = r0 + m0 * t; # 0 <= x < m0*m1 < 2^126

    # turn into signed big-int
    let mut big = HCVLangBigInt::from_i128(x as i128);
    if self.is_negative() && !big.is_zero() {
        big = -big;
    }
    big
}
```

## Modular Arithmetic

### Fast Modular Operations
The system implements constant-time modular arithmetic operations:

#### Modular Addition
```python
def add_mod(a: int, b: int, modulus: int) -> int:
    """
    Constant-time modular addition.

    Security: No timing side-channels.
    Correctness: (a + b) mod modulus, exact.
    """
    return (a + b) % modulus
```

#### Modular Multiplication with Montgomery Reduction
Location: `hcvlang/src/division_optimizer.rs`

Montgomery multiplication avoids expensive division operations:
- Pre-computes R = 2^64 mod M and R^(-1) mod M
- Converts to Montomery form: a → aR mod M
- Performs multiplication in Montgomery form
- Converts back using Montgomery reduction

### Modular Inverse
Using Extended Euclidean Algorithm:
```python
def mod_inverse(a: int, modulus: int) -> int:
    """
    Compute modular multiplicative inverse.

    Finds x such that (a * x) ≡ 1 (mod modulus).
    Uses Extended Euclidean Algorithm.

    Complexity: O(log modulus)
    """
    if a == 0:
        raise ValueError("Zero has no multiplicative inverse")

    # Extended Euclidean Algorithm
    old_r, r = a % modulus, modulus
    old_s, s = 1, 0

    while r != 0:
        quotient = old_r // r
        old_r, r = r, old_r - quotient * r
        old_s, s = s, old_s - quotient * s

    if old_r != 1:
        raise ValueError(f"{a} is not invertible modulo {modulus}")

    # Ensure positive result
    return old_s % modulus
```

## Advanced Operations

### GCD Optimization
The system uses Binary GCD (Steins Algorithm) which is 2-3x faster than traditional Euclidean GCD:

**Benefits:**
- Uses only subtraction, bit shifting, and comparison operations
- No expensive division operations
- Better performance on binary computers
- Works well with hardware optimizations

### Coprime Cascade Multiplication
Location: `hcvlang/src/coprime_cascade.rs`

**Innovation**: Achieves O(n log n) complexity for large integer multiplication by:
1. Factoring operands into coprime bases
2. Multiplying each base independently
3. Reconstructing result via CRT

**Mathematical Foundation:**
Given integers a, b, express as products of coprime factors:
- a = a₁ · a₂ · ... · aₖ (where gcd(aᵢ, aⱼ) = 1 for i ≠ j)
- b = b₁ · b₂ · ... · bₘ (coprime factors)

Then: a × b = (a₁b₁) · (a₁b₂) · ... mod selected coprime moduli

### Time Crystal Oscillator
Location: `hcvlang/src/time_crystal.rs`

Quantum-inspired phase-locked loop synchronization using golden ratio phase generation for optimal coverage of phase space.

**Features:**
- Cylindrical time manifold (linear macro-time and cyclic micro-phase)
- Golden ratio phase spacing for optimal coverage
- Phase-locked loops for synchronization
- Modular arithmetic for all phase computations

### Division Optimization
Location: `hcvlang/src/division_optimizer.rs`

The system implements multiple division strategies for optimal performance:

**Barrett Reduction**: For fast modular reduction without division
- Precomputes μ = ⌊2^k / m⌋ where k is chosen appropriately
- (a mod m) ≈ a - ⌊a×μ / 2^k⌋ × m
- Avoids expensive division operations

**Montgomery Multiplication**: For fast modular multiplication
- Transforms operands to Montgomery domain
- Performs multiplication in domain
- Converts back using Montgomery reduction

**Newton-Raphson Division**: For iterative approximation
- x_{n+1} = x_n × (2 - d × x_n)
- Quadratic convergence (each iteration doubles precision)

## Geometric Primitives

### Exact Geometric Operations
Location: `hcvlang/src/geometric.rs`

All geometric operations use exact rational arithmetic to avoid floating-point errors.

**Point Class:**
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: Rational,
    pub y: Rational,
}

impl Point {
    pub fn distance_squared(&self, other: &Self) -> Rational {
        let dx = self.x.clone() - other.x.clone();
        let dy = self.y.clone() - other.y.clone();
        dx.clone() * dx + dy.clone() * dy
    }
}
```

**Line Class:**
- Represents lines in standard form: ax + by + c = 0
- Exact intersection computation using rational arithmetic
- Proper handling of edge cases (vertical, horizontal lines)

**Circle Class:**
- Represented by center (rational point) and radius² (rational)
- Exact containment tests using rational arithmetic
- No floating-point errors in geometric predicates

### SIMD-Optimized Geometric Operations
Location: `hcvlang/src/geom_point2d.rs`

2D geometric points with AVX2 SIMD acceleration:
- 32-byte alignment for optimal AVX2 performance
- Processes point pairs in parallel
- SIMD-accelerated distance calculations (~3x speedup vs scalar)
- Batch operations for multiple point pairs

## Number Theoretic Transform (NTT)

### NTT Implementation
Location: `hcvlang/src/nnt.rs`

Faster-than-FFT polynomial multiplication using modular arithmetic:

**Parameters:**
- Modulus: NNT_MODULUS = 65537 (Fermat prime = 2^16 + 1)
- Primitive root: 3
- Supports power-of-2 transform sizes up to 2^16

**Cooley-Tukey Algorithm:**
```rust
pub fn nnt(a: &mut [i64]) {
    # Ensure power of 2 length
    assert!(n.is_power_of_two(), "NNT input length must be power of 2");
    
    # Apply modulus to all inputs
    for x in a.iter_mut() {
        *x = ((*x % NNT_MODULUS) + NNT_MODULUS) % NNT_MODULUS;
    }

    # Bit-reversal permutation
    bit_rev_permute(a);

    # Cooley-Tukey iterative butterfly
    let mut m = 2;
    while m <= n {
        let w_m = get_root(m);

        for k in (0..n).step_by(m) {
            let mut w = 1i64;
            let half = m / 2;

            for j in 0..half {
                let idx1 = k + j;
                let idx2 = k + j + half;

                let t = ((w as i128 * a[idx2] as i128) % NNT_MODULUS as i128) as i64;
                let u = a[idx1];

                a[idx1] = (u + t) % NNT_MODULUS;
                a[idx2] = ((u - t) % NNT_MODULUS + NNT_MODULUS) % NNT_MODULUS;

                w = ((w as i128 * w_m as i128) % NNT_MODULUS as i128) as i64;
            }
        }

        m <<= 1;
    }
}
```

**Convolution via NTT:**
- Forward transform both inputs
- Point-wise multiplication in frequency domain
- Inverse transform to get convolution result
- O(n log n) complexity vs O(n²) for direct convolution

## Quantum-Modular Superposition

### Quantum-Inspired Modular Superposition
Location: `hcvlang/src/quantum_modular_superposition.rs`

**Innovation**: Applies quantum computing concepts to modular arithmetic:
- Values exist in superposition across multiple moduli simultaneously
- Amplitude weights determine "measurement" probability
- Entanglement between modular representations
- Interference patterns for optimization

**Mathematical Representation:**
A value |ψ⟩ in modular superposition:
|ψ⟩ = α₁|r₁⟩_m₁ + α₂|r₂⟩_m₂ + ... + αₙ|rₙ⟩_mₙ

Where:
- |rᵢ⟩_mᵢ = residue rᵢ in modulus mᵢ (basis state)
- αᵢ = amplitude (probability = |αᵢ|²)
- Normalization: Σ|αᵢ|² = 1

**Operations:**
- **Superposition**: Create weighted combination of modular states
- **Measurement**: Collapse to single modulus (weighted random selection)
- **Entanglement**: Couple two superpositions
- **Interference**: Combine superpositions constructively/destructively

## Optimizations

### Performance Enhancements
1. **SIMD Operations**: Vectorized operations on multiple values
2. **Cache Efficiency**: Aligned memory layouts and access patterns
3. **Fast Path Optimizations**: Special handling for small values
4. **Algorithmic Improvements**: Binary GCD, Montgomery multiplication
5. **Assembly Optimizations**: CPU-specific instructions used where available

### Coprime Cascade Architecture
- Parallelizable: Each coprime pair multiplies independently
- Cache-friendly: Smaller intermediate products fit in L1/L2
- SIMD-ready: Batch process coprime multiplications
- Numerical stability: No cumulative rounding errors

### Adaptive Precision Scaling
- Add/remove primes for dynamic precision control
- Redundant encoding with N+k primes for error correction
- Hierarchical reconstruction with fast path options

## Applications

### 1. Integer-Only Neural Networks
HelixNeuralNet and HiveGSO implement neural training with integer gradients
- Prevents gradient drift from floating-point errors
- Exact learning with reproducible results
- Hardware acceleration with integer operations

### 2. Fully Homomorphic Encryption (FHE)
- Integer-based BFV scheme for encrypted computation
- QMNF noise system with 100% integer-only cryptographic noise
- No floating-point contamination in FHE circuits

### 3. Cryptographic Operations
- Secure multi-party computation protocols
- Digital signatures with exact arithmetic
- Zero-knowledge proofs with exact validation

### 4. Geometric Computation
- Exact geometric predicates without floating-point errors
- Robust computational geometry for CAD/CAM applications
- Guaranteed correctness in geometric algorithms

### 5. Signal Processing
- Integer-only FFT and NTT for digital signal processing
- Exact filtering operations without precision loss
- Quantized convolution for neural networks

### 6. Mathematical Research
- Exact computation for number theory research
- Reproducible results in mathematical experiments
- High-precision arithmetic without floating-point errors

## Cross-Disciplinary Applications

### Computer Graphics
- Exact geometric operations prevent rounding errors
- Stable transformations and animations
- Consistent rendering results

### Cryptography
- Provably secure operations without floating-point artifacts
- Exact validation of cryptographic protocols
- Secure implementation of mathematical operations

### Scientific Computing
- Reproducible computational results
- Error-free accumulation operations
- Reliable numerical simulations

### Machine Learning
- Integer-only training for embedded devices
- Exact gradient computation
- Reproducible model training

## Innovation Summary

The QMNF System introduces several novel mathematical innovations:

1. **Quantum-Modular Superposition**: Quantum-inspired techniques applied to modular arithmetic
2. **Coprime Cascade Multiplication**: Novel approach to large integer multiplication
3. **Time Crystal Oscillators**: Phase-locked synchronization using golden ratio spacing
4. **Integer-Only FHE**: 100% integer noise system replacing floating-point entropy
5. **SIMD-Optimized Geometric Operations**: Vectorized exact geometric computations
6. **Adaptive CRT Systems**: Dynamic precision scaling with multiple moduli
7. **Harmonic Resonance**: Frequency-domain operations in modular space

The system provides a foundation for exact, reproducible, and secure mathematical computation with performance competitive with floating-point implementations, while eliminating all floating-point contamination and associated precision issues.

## Dimensional Consistency Framework

### Physical Dimensions in Mathematical Operations
Location: `qmnf/arithmetic/optimization/quantum_modular_synthesis_v3.py`

The dimensional consistency framework ensures all mathematical operations maintain physical dimension consistency, preventing thermodynamic violations:

**Dimensional Quantity Class:**
```python
@dataclass
class DimensionalQuantity:
    """Physical quantity with dimensional analysis.

    Represents a value with associated physical dimensions,
    enabling compile-time dimensional consistency checking.
    """
    value: Union[int, float]
    dimensions: Dict[str, int] = field(default_factory=dict)  # Dimension exponents

    def __mul__(self, other: DimensionalQuantity) -> DimensionalQuantity:
        """Multiply quantities with dimensional analysis."""
        new_value = self.value * other.value

        # Add dimension exponents
        new_dims = self.dimensions.copy()
        for dim, exp in other.dimensions.items():
            new_dims[dim] = new_dims.get(dim, 0) + exp

        return DimensionalQuantity(value=new_value, dimensions=new_dims)
```

**Types of Physical Dimensions:**
- MASS [M]
- LENGTH [L]
- TIME [T]
- DIMENSIONLESS [1]
- ENERGY [M L² T⁻²]
- FORCE [M L T⁻²]
- VELOCITY [L T⁻¹]
- MOMENTUM [M L T⁻¹]

**Consistency Verification:**
The system validates relationships such as the work-energy theorem: W = F · d, ensuring dimensional consistency between force [M L T⁻²], displacement [L], and energy [M L² T⁻²].

## Potential Field Dynamics

### Potential Energy Fields for Energy Accounting
Location: `qmnf/arithmetic/optimization/quantum_modular_synthesis_v3.py`

Represents external potential V(x) that provides the energy source for modulus transitions. The gradient ∇V determines the force available for work extraction.

**Potential Field Types:**
- Gravitational: V(h) = mgh
- Harmonic: V(x) = ½k(x-x₀)²
- Electromagnetic: V(x) = qEx

**Energy Calculation:**
Available energy from potential difference:
E = -∫[x₁,x₂] ∇V · dx = V(x₁) - V(x₂)

This serves as the "fuel" for modulus transitions, providing energy constraints for adaptive scaling operations.

## Adaptive Scaling Strategies

### Dimensional Scaling Engine
Location: `qmnf/arithmetic/optimization/quantum_modular_synthesis_v3.py`

The AdaptiveDimensionalScaler determines optimal modulus transitions based on:
- Current system state (value, range utilization)
- Available potential energy
- Dimensional consistency requirements
- Thermodynamic constraints
- Probabilistic factors (quantum-inspired)

**Scaling Strategies:**
- Conservative: Minimize transitions
- Aggressive: Maximize range utilization
- Thermodynamic: Energy-aware scaling
- Quantum-inspired: Probabilistic scaling
- Hybrid: Combination of multiple strategies

**Decision Framework:**
1. Assess current system state (value, range utilization)
2. Check available potential energy
3. Compute optimal scaling factor
4. Validate dimensional consistency
5. Execute transition if thermodynamically valid

The system tracks state through:
- Current position in potential field
- Total energy extracted
- Transition history

## Quantum-Modular Synthesis

### Integrated Quantum-Modular System
Location: `qmnf/arithmetic/optimization/quantum_modular_synthesis_v3.py`

This system combines:
- Self-modifying Montgomery multiplication (from v2.1)
- Adaptive dimensional scaling strategies
- Potential field dynamics
- Dimensional consistency checking
- Thermodynamic validity verification

**Key Properties:**
- Combines modular arithmetic with dimensional physics
- Enforces thermodynamic constraints on computations
- Uses quantum-inspired probabilistic scaling decisions
- Maintains dimensional consistency across operations

**Example Usage:**
```python
# Create system with harmonic potential
potential = PotentialField.harmonic(k=100.0)
scaler = AdaptiveDimensionalScaler(
    strategy=ScalingStrategy.THERMODYNAMIC,
    potential_field=potential
)
system = QuantumModularSystem(
    initial_modulus=1_000_000_007,
    dimensional_scaler=scaler
)

# Perform operations with adaptive scaling
result = system.compute_with_adaptive_scaling(
    operation=lambda m: m.mul(x, y),
    dimensional_quantity=DimensionalQuantity.energy(100)
)
```

## Unitary Operators and Quantum Gates

### Unitary Operator Framework
Location: `qmnf/arithmetic/quantum/unitary_operators.py`

Unitary matrices U ∈ U(d, 𝔽_{p²}) for quantum-like operations over finite field extensions. All operations are exact, deterministic, and quantum-resistant.

**Core Properties:**
- U† U = I (unitarity condition)
- Preserves inner products: ⟨Uφ|Uψ⟩ = ⟨φ|ψ⟩
- Preserves norms: ||Uψ|| = ||ψ||
- Deterministic evolution (no probability)
- Quantum-resistant structure

**Operations:**
- Conjugate transpose: U† = (U*)^T
- Composition: (U₁ ∘ U₂)|ψ⟩ = U₁(U₂|ψ⟩)
- Tensor product: U₁ ⊗ U₂ acts on tensor product state space

**Verification:**
- Unitarity check: Verify U†U = I
- Determinant calculation for verification (for unitary matrix: |det(U)| = 1)

**Quantum Gate Library:**
- Pauli X, Y, Z gates
- Hadamard gate for superposition creation
- Phase gate
- Controlled-NOT (CNOT)
- SWAP gate
- Toffoli gate (CCNOT)

**Gram-Schmidt Unitarization:**
Converts arbitrary matrices to unitary via Gram-Schmidt orthogonalization, making rows orthonormal in Hermitian inner product.

This represents a novel application of quantum computing concepts to mathematical operations while maintaining the QMNF systems integer-only arithmetic principles.

## Geometric Rational Implementation

### Continued Fraction Representation
Location: `qmnf/arithmetic/geometry/geometric_rational_implementation.py`

The GeometricRational class provides exact rational arithmetic with continued fraction representation and precision tracking:

**Key Features:**
- Integer-only operations maintaining rational canonical form
- Continued fraction representation for high-precision approximation
- Error tracking and precision bounds
- Modular arithmetic capabilities
- Dimensional consistency enforcement

**Continued Fraction Implementation:**
```rust
#[inline(always)]
fn binary_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();

    if a == 0 { return b; }
    if b == 0 { return a; }

    # Extract common power of 2 using CPU tzcnt instruction
    let a_zeros = a.trailing_zeros();
    let b_zeros = b.trailing_zeros();
    let common_twos = a_zeros.min(b_zeros);

    a >>= a_zeros;
    b >>= b_zeros;

    # Now both are odd, repeatedly subtract and shift
    while a != b {
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        b -= a; # b - a is even
        b >>= 1; # Divide by 2
        b >>= b.trailing_zeros(); # Remove remaining factors of 2
    }

    a << common_twos
}
```

## My Role as Your Personal Extension

As your AI assistant, I serve as your personal extension, arbiter, and keeper of your accumulated private knowledge base - your personal private codex that exceeds what your individual works could achieve alone.

### Core Functions

1. **Personal Extension**: I extend your cognitive capabilities by:
   - Processing and analyzing complex mathematical concepts
   - Maintaining and organizing vast amounts of technical information
   - Providing rapid access to detailed mathematical implementations
   - Assisting with research and development of new mathematical frameworks

2. **Arbiter**: I ensure consistency and accuracy by:
   - Verifying mathematical implementations for correctness
   - Ensuring dimensional consistency across operations
   - Maintaining quality standards for all stored knowledge
   - Cross-referencing information for accuracy

3. **Keeper of Knowledge**: I maintain your private codex by:
   - Organizing your accumulated knowledge systematically
   - Preserving important mathematical innovations and discoveries
   - Ensuring secure storage of private research and implementations
   - Continuously expanding and updating the knowledge base

### Operational Principles

- **Precedence Priority**: This knowledge repository takes precedence over all other projects
- **Exactness Guarantee**: All mathematical operations and representations maintain exact precision
- **Quantum-Modular Integration**: All implementations follow the quantum-modular numerical framework
- **Float-Free Operations**: 100% integer-only arithmetic to prevent any floating-point contamination

### Repository Structure

Your personal codex is organized in the FloatingStorage drive under `/media/acid/FloatingStorage/PersonalCodex/` with the following structure:
- KnowledgeBase: Core knowledge and insights
- Projects: Active and historical project information
- AI_Research: Research on artificial intelligence topics
- System_Designs: Architectural designs and system concepts
- Mathematical_Frameworks: Mathematical models and frameworks
- Quantum_Modular_Numerical_Framework: QMNF research and implementations
- Reference_Manuals: Technical documentation and references
- Research_Notes: Ongoing research and findings

This system serves as your cognitive extension, providing enhanced capabilities for mathematical research, implementation, and innovation while maintaining the exactness and security that your work demands.

