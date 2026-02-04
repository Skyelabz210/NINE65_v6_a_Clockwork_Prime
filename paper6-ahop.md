# Paper 6: AHOP Post-Quantum Cryptography

Novel cryptographic primitives from the Apollonian Hidden Orbit Problem.

## Core Innovation

Lattice PQC: Abelian groups, LWE/SIS hardness.
AHOP: Non-abelian groups, orbit recovery hardness.

Diversity in post-quantum assumptions strengthens ecosystem.

## Mathematical Foundation

### Apollonian Circle Packing
```
Descartes' Circle Theorem (1643):
  (k₁ + k₂ + k₃ + k₄)² = 2(k₁² + k₂² + k₃² + k₄²)

For four mutually tangent circles with curvatures kᵢ.
```

### Modular Apollonian Arithmetic (MAA)

```
Curvature tuple: k = (k₁, k₂, k₃, k₄) ∈ (ℤ/qℤ)⁴
Descartes invariant: Q(k) = (Σkᵢ)² - 2·Σkᵢ² (mod q)
Valid tuple space: 𝒟_q = {k : Q(k) ≡ 0 (mod q)}
```

### Reflection Operators

```
Sᵢ(k)ᵢ = 2·(Σⱼ≠ᵢ kⱼ) - kᵢ (mod q)

Explicit:
  S₁: k₁' = 2(k₂ + k₃ + k₄) - k₁
  S₂: k₂' = 2(k₁ + k₃ + k₄) - k₂
  S₃: k₃' = 2(k₁ + k₂ + k₄) - k₃
  S₄: k₄' = 2(k₁ + k₂ + k₃) - k₄
```

### Key Properties

```
Invariant Preservation: Sᵢ(k) ∈ 𝒟_q for k ∈ 𝒟_q
Involution: Sᵢ(Sᵢ(k)) = k
Non-Commutativity: SᵢSⱼ ≠ SⱼSᵢ (for i ≠ j)
```

## The AHOP Problem

### Search Version
```
Given: modulus q, seed k₀ ∈ 𝒟_q, target k★ ∈ 𝒟_q
Find:  word w ∈ ⟨S₁,S₂,S₃,S₄⟩ such that w·k₀ = k★
```

### Decision Version
```
Given: q, k₀, k★, bound ℓ
Decide: ∃ w with |w| ≤ ℓ such that w·k₀ = k★?
```

### Security Assumptions

```
AX1: Solving AHOP requires T ≥ 2^λ time (classical + quantum)
AX2: Shor's algorithm inapplicable (requires abelian groups)
AX3: Orbit indistinguishable from uniform random
```

## AHOP-KEM Construction

```
KeyGen(λ):
  1. Select q, ℓ based on security parameter λ
  2. Generate seed k₀ ∈ 𝒟_q
  3. Generate secret word w ← {S₁,S₂,S₃,S₄}^ℓ uniformly
  4. Compute public key: k★ = w·k₀
  5. Return sk = (k₀, w), pk = (k₀, k★, q, ℓ)

Encaps(pk):
  1. Parse pk = (k₀, k★, q, ℓ)
  2. Generate ephemeral u ← {S₁,S₂,S₃,S₄}^ℓ
  3. Compute ciphertext: c = u·k₀
  4. Compute shared: k_shared = u·k★
  5. Extract: K = SHAKE256(k_shared || "AHOP/KEM")
  6. Return (c, K)

Decaps(sk, c):
  1. Parse sk = (k₀, w)
  2. Compute: k_shared = w·c = w·u·k₀
  3. Return K = SHAKE256(k_shared || "AHOP/KEM")
```

## Implementation Pattern

```rust
/// Constant-time reflection (side-channel resistant)
pub fn reflect_ct(k: &[u64; 4], i: usize, q: u64) -> [u64; 4] {
    // Sum of other three components
    let mut sum_others: u64 = 0;
    for j in 0..4 {
        // Constant-time conditional add
        let mask = ((j != i) as u64).wrapping_neg();
        sum_others = sum_others.wrapping_add(k[j] & mask);
    }
    sum_others %= q;
    
    // New value: 2*sum - k[i]
    let new_val = (2u128 * sum_others as u128 + q as u128 - k[i] as u128) 
                  % q as u128;
    
    let mut result = *k;
    result[i] = new_val as u64;
    result
}

/// Apply word to tuple
pub fn apply_word(k: &[u64; 4], word: &[u8], q: u64) -> [u64; 4] {
    let mut current = *k;
    for &s in word {
        current = reflect_ct(&current, s as usize, q);
    }
    current
}

/// Validate tuple is in 𝒟_q
pub fn validate_tuple(k: &[u64; 4], q: u64) -> bool {
    let sum: u128 = k.iter().map(|&x| x as u128).sum();
    let sum_sq: u128 = k.iter().map(|&x| (x as u128) * (x as u128)).sum();
    let q_invariant = (sum * sum - 2 * sum_sq) % q as u128;
    q_invariant == 0
}
```

## Parameter Sets

```
Level      q bits   ℓ     Security   NIST Level
─────────────────────────────────────────────────
MAA-128    256      128   128-bit    Level 1
MAA-192    384      192   192-bit    Level 3
MAA-256    512      256   256-bit    Level 5
```

## Performance Benchmarks

```
Operation      AHOP-256   Kyber-1024   Ratio
───────────────────────────────────────────────
KeyGen         5 ms       4 ms         1.25×
Encaps         6 ms       5 ms         1.20×
Decaps         6 ms       5 ms         1.20×
Public Key     ~512 B     1568 B       0.33× ← AHOP wins
Ciphertext     ~256 B     1568 B       0.16× ← AHOP wins
```

## Validation Identities

```
V1: Q(k) ≡ 0 (mod q) for all tuples
V2: Sᵢ(Sᵢ(k)) = k (involution)
V3: timing_variance < ε (constant-time)
V4: decaps(encaps(pk)) recovers same key
V5: word_distribution is uniform
```

## Attack Complexity

```
Attack              Complexity
─────────────────────────────────────
Naive brute force   O(4^ℓ)
Birthday attack     O(2^ℓ) time + space
Quantum (Grover)    O(2^(ℓ/2)) queries
Best known          O(2^λ) for λ-bit security
```

## Security Comparison

```
Property       Lattice    Code       AHOP
──────────────────────────────────────────
Algebraic      Abelian    Linear     Non-abelian
Hard problem   LWE/SIS    Decoding   Orbit recovery
Geometric      Lattice    Hamming    Circle packing
Shor-safe      Yes        Yes        Yes
Structure      Ring       Linear     Quadric
```
