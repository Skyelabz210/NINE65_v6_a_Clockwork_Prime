# WORK REQUEST: FHE Integration with Codex/Residue Stack

**Date:** 2025-11-17
**Priority:** HIGH
**Type:** Architecture Integration
**Status:** PROPOSED

---

## Executive Summary

**Objective:** Integrate FHE polynomial operations with the existing Codex Gear Manifold + Residue Space + M2M infrastructure, eliminating redundant implementations and achieving 170-1000× performance improvement through proper architectural alignment.

**Current State:** FHE polynomial multiplication implemented independently with custom ModInt, falling back to naive O(n²) algorithm.

**Target State:** FHE polynomials represented as `Vec<ResidueVector>` using CodexManifold CRT framework, Montgomery arithmetic, and per-prime NTT for O(n log n) operations.

**Expected Impact:**
- **Performance:** 170× speedup (820ms → ~5ms encryption)
- **Code Reduction:** Eliminate ~400 lines of duplicate RNS/CRT code
- **Architecture:** Unify FHE with neural/symbolic stack
- **Maintainability:** Single source of truth for CRT operations

---

## Current Architecture Analysis

### **Existing Infrastructure (DO NOT REWRITE)**

#### 1. Codex Gear Manifold (`codex_gear_manifold.rs` - 900+ lines)
```rust
pub struct CodexManifold {
    gears: Vec<CodexGear>,          // One per prime modulus
    capacity: i128,                  // Product of all moduli
    morphisms: BTreeMap<...>,        // Categorical structure
}

// Encode: ℤ → CRT (Functor F)
pub fn encode(&mut self, value: i128)

// Decode: CRT → ℤ (Functor G, Garner's O(k²))
pub fn decode(&self) -> i128

// Adjunction: F ⊣ G (categorical optimality)
```

**Features:**
- ✅ CRT with Garner's algorithm (O(k²) reconstruction)
- ✅ Category theory (adjoint functors, morphisms)
- ✅ Theorem validation framework
- ✅ Montgomery constant precomputation
- ✅ Optimal prime selection

**Location:** `hcvlang/src/codex_gear_manifold.rs`

#### 2. Residue Space (`neural/residue_space.rs` - 795 lines)
```rust
pub struct ResidueVector {
    residues: Vec<i64>,              // One per modulus (Montgomery form)
    anchor: i64,                     // Control flow modulus
    config: Arc<ResidueConfig>,
}

// Operations in residue space (4.1ns Montgomery)
pub fn add(&self, other: &Self) -> Self
pub fn mul(&self, other: &Self) -> Self
pub fn sub(&self, other: &Self) -> Self

// Zero reconstruction during computation
```

**Features:**
- ✅ Montgomery arithmetic (constant-time, 4.1ns ops)
- ✅ Perfect RNS parallelism (zero sync)
- ✅ Configurable moduli (up to 1000 primes)
- ✅ Anchor modulus (61-bit Mersenne: 2^61-1)
- ✅ Garner's algorithm for reconstruction

**Location:** `hcvlang/src/neural/residue_space.rs`

#### 3. M2M Tokenizer (`neural/manifold_tokenizer.rs` - 643 lines)
```rust
pub struct M2MTokenizer {
    config: Arc<ResidueConfig>,
    embed_dim: usize,
    vocab_size: usize,
    embeddings: Vec<ResidueVector>,
}

// Codex → Residue conversion
pub fn codex_to_residue(&self, manifold: &CodexManifold) -> ResidueVector

// Residue → Codex conversion
pub fn residue_to_codex(&self, vector: &ResidueVector) -> CodexManifold

// Sequence tokenization
pub fn tokenize_sequence(&self, manifolds: &[CodexManifold]) -> Vec<ResidueVector>
```

**Features:**
- ✅ Bidirectional Codex ↔ Residue conversion
- ✅ Semantic clustering
- ✅ 2.5-4× compression (CRT encoding)
- ✅ FNV-1a hashing for identifiers
- ✅ Deterministic initialization

**Location:** `hcvlang/src/neural/manifold_tokenizer.rs`

#### 4. NTT Implementation (`math/discrete.rs` - partial)
```rust
impl DiscreteMath {
    pub fn ntt(input: Vec<ModInt>, inverse: bool) -> Vec<ModInt>
    pub fn intt(input: Vec<ModInt>) -> Vec<ModInt>
    pub fn convolution_mod(a: Vec<ModInt>, b: Vec<ModInt>) -> Vec<ModInt>
}
```

**Features:**
- ✅ Cooley-Tukey algorithm
- ✅ Bit-reversal permutation
- ✅ Primitive root computation
- ❌ **ISSUE:** Hardcoded to ModInt (Mersenne prime 2^31-1)
- ❌ **ISSUE:** Mersenne prime NOT NTT-friendly for n=4096!

**Location:** `hcvlang/src/math/discrete.rs`

---

### **Current FHE Implementation (NEEDS INTEGRATION)**

#### FHE Polynomial (`fhe/polynomial.rs` - 835 lines)
```rust
pub struct Polynomial {
    coeffs: Vec<ModInt>,             // ❌ Single modulus only!
    dimension: usize,                // n = 4096 for security
    modulus: u64,                    // q = 2^31-1 (Mersenne)
}

impl Polynomial {
    pub fn mul_nnt(&self, other: &Polynomial) -> Polynomial {
        if self.modulus == NNT_MODULUS {
            // NTT multiplication
        } else {
            self.mul_naive(other)    // ❌ ALWAYS executes! (modulus mismatch)
        }
    }
}
```

**Issues:**
1. ❌ Uses single modulus (Mersenne 2^31-1) - NOT NTT-friendly
2. ❌ Falls back to naive O(n²) multiplication (16.7M operations!)
3. ❌ Duplicate CRT infrastructure (should use CodexManifold)
4. ❌ Duplicate Montgomery setup (should use ResidueVector)
5. ❌ No integration with M2M for encoding boundaries

**Location:** `hcvlang/src/fhe/polynomial.rs`

#### FHE RNS (`fhe/rns.rs` - 338 lines)
```rust
pub const Q0: u64 = 2013265921;  // NTT-friendly prime
pub const Q1: u64 = 1811939329;  // NTT-friendly prime

pub fn crt_reconstruct_u128(a0: u64, a1: u64, inv_q0_mod_q1: u64) -> u128
pub fn rescale_bfv_delta_rns(c_q0: &[u64], c_q1: &[u64], ...) -> (Vec<u64>, Vec<u64>)
```

**Issues:**
1. ❌ Hardcoded to 2 primes (Q0, Q1)
2. ❌ Duplicate Garner's algorithm (CodexManifold already has it!)
3. ❌ Separate CRT constants (CodexManifold manages this)
4. ✅ Uses NTT-friendly primes (good choice)

**Location:** `hcvlang/src/fhe/rns.rs`

---

## Root Cause Analysis

### **Why FHE is Slow (820ms encryption):**

1. **Mersenne Prime NOT NTT-Friendly:**
   ```
   p = 2^31 - 1 = 2147483647
   p - 1 = 2^31 - 2 = 2 × 1073741823

   Power of 2 in (p-1): 2^1
   Required for n=4096: 2^13 = 8192

   Result: Can only support NTT size 2, not 8192!
   ```

2. **Modulus Mismatch:**
   ```rust
   // polynomial.rs:370
   if self.modulus == NNT_MODULUS as u64 {  // 2147483647 == 65537? → FALSE
       // Use NTT (never executes)
   } else {
       self.mul_naive(other)  // ❌ ALWAYS O(n²)
   }
   ```

3. **Duplicate Infrastructure:**
   - FHE has its own CRT (rns.rs)
   - Codex already has optimal CRT (Garner's algorithm)
   - FHE has its own moduli selection
   - ResidueConfig already does this (1000 primes!)

---

## Proposed Integration Architecture

### **Target Structure:**

```rust
// NEW: FHE Polynomial using Codex/Residue stack
pub struct Polynomial {
    /// Coefficients in residue space (one ResidueVector per coefficient)
    coeffs: Vec<ResidueVector>,

    /// Codex manifold for CRT framework
    manifold: Arc<CodexManifold>,

    /// Dimension (n = 4096 for 128-bit security)
    dimension: usize,
}

impl Polynomial {
    /// Create from integer coefficients (encoding boundary)
    pub fn from_coeffs(coeffs: Vec<i64>, manifold: Arc<CodexManifold>) -> Self {
        let residue_coeffs: Vec<_> = coeffs.iter()
            .map(|&c| ResidueVector::from_int(c, manifold.config.clone()))
            .collect();

        Polynomial {
            coeffs: residue_coeffs,
            manifold,
            dimension: coeffs.len(),
        }
    }

    /// Reconstruct to integer coefficients (decoding boundary)
    pub fn to_coeffs(&self) -> Vec<i64> {
        self.coeffs.iter()
            .map(|rv| rv.to_int())
            .collect()
    }

    /// Addition (coefficient-wise ResidueVector add)
    pub fn add(&self, other: &Self) -> Self {
        let result_coeffs: Vec<_> = self.coeffs.iter()
            .zip(other.coeffs.iter())
            .map(|(a, b)| a.add(b))  // ResidueVector::add (4.1ns Montgomery)
            .collect();

        Polynomial {
            coeffs: result_coeffs,
            manifold: self.manifold.clone(),
            dimension: self.dimension,
        }
    }

    /// Multiplication using per-prime NTT (O(n log n))
    pub fn multiply(&self, other: &Self) -> Self {
        // For each prime in the manifold, apply NTT
        let num_primes = self.manifold.config.moduli.len();

        // Pre-allocate result residues
        let mut result_residues = vec![vec![0i64; self.dimension]; num_primes];

        // Parallel NTT across all primes (no synchronization needed!)
        for prime_idx in 0..num_primes {
            let modulus = self.manifold.config.moduli[prime_idx];
            let primitive_root = compute_primitive_root(modulus);

            // Extract coefficients for this prime
            let mut self_prime: Vec<i64> = self.coeffs.iter()
                .map(|rv| rv.residues[prime_idx])
                .collect();

            let mut other_prime: Vec<i64> = other.coeffs.iter()
                .map(|rv| rv.residues[prime_idx])
                .collect();

            // NTT forward transform
            ntt_forward_generic(&mut self_prime, modulus, primitive_root);
            ntt_forward_generic(&mut other_prime, modulus, primitive_root);

            // Pointwise multiplication in NTT domain (Montgomery)
            let montgomery_ctx = &self.manifold.config.montgomery_contexts[prime_idx];
            for i in 0..self.dimension {
                let product = montgomery_ctx.montgomery_mul(self_prime[i], other_prime[i]);
                self_prime[i] = product;
            }

            // Inverse NTT
            ntt_inverse_generic(&mut self_prime, modulus, primitive_root);

            // Apply negacyclic reduction: X^N ≡ -1
            for i in 0..self.dimension {
                let high_idx = i + self.dimension;
                if high_idx < self_prime.len() {
                    let negated = (modulus - self_prime[high_idx]) % modulus;
                    result_residues[prime_idx][i] = (self_prime[i] + negated) % modulus;
                } else {
                    result_residues[prime_idx][i] = self_prime[i];
                }
            }
        }

        // Reconstruct ResidueVectors from per-prime results
        let mut result_coeffs = Vec::with_capacity(self.dimension);
        for coeff_idx in 0..self.dimension {
            let residues: Vec<i64> = (0..num_primes)
                .map(|p| result_residues[p][coeff_idx])
                .collect();

            result_coeffs.push(ResidueVector {
                residues,
                anchor: 0,  // Recompute if needed
                config: self.manifold.config.clone(),
            });
        }

        Polynomial {
            coeffs: result_coeffs,
            manifold: self.manifold.clone(),
            dimension: self.dimension,
        }
    }
}
```

---

## Integration Tasks

### **Phase 1: NTT Generalization (8 hours)**

**Objective:** Make NTT work with arbitrary NTT-friendly moduli, not just Mersenne prime.

**Current Location:** `hcvlang/src/math/discrete.rs:88-143`

**Changes:**
1. **Extract NTT to generic function:**
   ```rust
   // NEW: Generic NTT for any NTT-friendly prime
   pub fn ntt_generic(
       input: &mut [i64],
       modulus: i64,
       primitive_root: i64,
       inverse: bool
   ) {
       // Same Cooley-Tukey algorithm, parameterized modulus
       // ...existing implementation...
   }
   ```

2. **Add primitive root computation:**
   ```rust
   pub fn compute_primitive_root(modulus: i64) -> i64 {
       // Check if modulus is NTT-friendly
       // Find primitive root via brute force or lookup table
       // Cache results
   }
   ```

3. **Verify NTT-friendliness:**
   ```rust
   pub fn is_ntt_friendly(modulus: i64, required_order: usize) -> bool {
       let p_minus_1 = modulus - 1;
       let power_of_2 = p_minus_1.trailing_zeros();
       power_of_2 >= required_order.ilog2()
   }
   ```

**Files to Modify:**
- `hcvlang/src/math/discrete.rs` (refactor existing NTT)

**Do NOT Create:** New NTT implementation (reuse existing structure)

**Tests:**
- Round-trip NTT for primes Q0, Q1 from `fhe/rns.rs`
- Convolution correctness vs naive multiply
- Performance benchmark (should be O(n log n))

---

### **Phase 2: Polynomial Integration with ResidueVector (12 hours)**

**Objective:** Replace `Vec<ModInt>` with `Vec<ResidueVector>` for polynomial coefficients.

**Current Location:** `hcvlang/src/fhe/polynomial.rs`

**Changes:**

1. **Update Polynomial struct:**
   ```rust
   // OLD
   pub struct Polynomial {
       coeffs: Vec<ModInt>,
       dimension: usize,
       modulus: u64,
   }

   // NEW
   pub struct Polynomial {
       coeffs: Vec<ResidueVector>,
       manifold: Arc<CodexManifold>,
       dimension: usize,
   }
   ```

2. **Update constructors:**
   ```rust
   // Use M2M for encoding boundary
   pub fn from_coeffs(coeffs: Vec<i64>, manifold: Arc<CodexManifold>) -> Self
   pub fn to_coeffs(&self) -> Vec<i64>  // Garner's algorithm via ResidueVector
   ```

3. **Update operations:**
   ```rust
   impl Add for Polynomial {
       fn add(self, other: Self) -> Self {
           // Use ResidueVector::add (Montgomery, 4.1ns)
       }
   }

   impl Mul for Polynomial {
       fn mul(self, other: Self) -> Self {
           self.multiply_ntt(&other)  // Per-prime NTT
       }
   }
   ```

4. **Remove old implementations:**
   - Delete `mul_naive` (replaced by NTT)
   - Delete `to_nnt/from_nnt` (use generic NTT)
   - Keep `sample_uniform`, `sample_error`, `sample_ternary` (update to use ResidueVector)

**Files to Modify:**
- `hcvlang/src/fhe/polynomial.rs` (main changes)
- `hcvlang/src/fhe/encrypt.rs` (update Plaintext/Ciphertext)
- `hcvlang/src/fhe/operations.rs` (update homomorphic ops)

**Do NOT Modify:**
- `hcvlang/src/neural/residue_space.rs` (existing infra)
- `hcvlang/src/codex_gear_manifold.rs` (existing infra)

**Tests:**
- Polynomial addition/multiplication correctness
- Encrypt-decrypt round-trip with new structure
- Homomorphic operations (add, multiply)
- Performance: encryption should be ~5ms (down from 820ms)

---

### **Phase 3: Remove Duplicate RNS Code (4 hours)**

**Objective:** Eliminate `fhe/rns.rs` and use CodexManifold infrastructure.

**Current Location:** `hcvlang/src/fhe/rns.rs` (338 lines to remove)

**Migration Plan:**

1. **Replace RNS constants with ResidueConfig:**
   ```rust
   // OLD (fhe/rns.rs)
   pub const Q0: u64 = 2013265921;
   pub const Q1: u64 = 1811939329;

   // NEW (use ResidueConfig)
   let config = ResidueConfig::from_moduli(
       vec![2013265921, 1811939329],
       2305843009213693951  // 61-bit anchor
   ).unwrap();
   ```

2. **Replace CRT reconstruction:**
   ```rust
   // OLD (fhe/rns.rs)
   pub fn crt_reconstruct_u128(a0: u64, a1: u64, inv_q0_mod_q1: u64) -> u128

   // NEW (use ResidueVector)
   let value = residue_vector.to_int();  // Garner's algorithm
   ```

3. **Replace rescaling:**
   ```rust
   // OLD (fhe/rns.rs:89-131)
   pub fn rescale_bfv_delta_rns(c_q0: &[u64], c_q1: &[u64], ...) -> ...

   // NEW (operate on ResidueVector directly)
   // Rescaling is per-coefficient operation in residue space
   ```

**Files to Delete:**
- `hcvlang/src/fhe/rns.rs` (entire file)

**Files to Update:**
- `hcvlang/src/fhe/mod.rs` (remove `pub mod rns;`)
- `hcvlang/src/fhe/operations.rs` (remove `use crate::fhe::rns::*;`)

**Do NOT Delete:**
- Prime selection logic (migrate to ResidueConfig if needed)
- Test cases (migrate to polynomial tests)

---

### **Phase 4: M2M Integration for Encoding (6 hours)**

**Objective:** Use M2M tokenizer for FHE encode/decode boundaries.

**Current Location:** `hcvlang/src/fhe/encoding.rs`

**Changes:**

1. **Update IntegerEncoder:**
   ```rust
   pub struct IntegerEncoder {
       params: FHEParams,
       manifold: Arc<CodexManifold>,
       tokenizer: Arc<M2MTokenizer>,  // NEW
   }

   impl IntegerEncoder {
       pub fn encode(&self, message: i64) -> Plaintext {
           // Convert message to ResidueVector via M2M
           let residue = ResidueVector::from_int(message, self.manifold.config.clone());

           // Scale by Δ
           let delta = self.compute_delta();
           let scaled = residue.scalar_mul(delta);

           Plaintext {
               poly: Polynomial::from_residues(vec![scaled], self.manifold.clone())
           }
       }
   }
   ```

2. **Update Plaintext struct:**
   ```rust
   pub struct Plaintext {
       poly: Polynomial,  // Already uses ResidueVector internally
   }
   ```

**Files to Modify:**
- `hcvlang/src/fhe/encoding.rs` (integrate M2M)
- `hcvlang/src/fhe/mod.rs` (update FHEContext::new to create manifold)

**Do NOT Modify:**
- `hcvlang/src/neural/manifold_tokenizer.rs` (existing M2M)

---

### **Phase 5: Testing & Validation (10 hours)**

**Objective:** Ensure correctness and measure performance gains.

**Test Suite:**

1. **Unit Tests:**
   ```rust
   #[test]
   fn test_polynomial_multiply_residue() {
       let manifold = Arc::new(CodexManifold::new(4).unwrap());
       let p1 = Polynomial::from_coeffs(vec![1, 2, 3, 4], manifold.clone());
       let p2 = Polynomial::from_coeffs(vec![5, 6, 7, 8], manifold.clone());

       let product = p1 * p2;

       // Verify against known result
       let result_coeffs = product.to_coeffs();
       assert_eq!(result_coeffs, vec![...expected...]);
   }
   ```

2. **Integration Tests:**
   ```rust
   #[test]
   fn test_fhe_encrypt_decrypt_residue() {
       let ctx = FHEContext::new(SecurityLevel::Bit128);
       let (sk, pk) = ctx.generate_keypair();

       let msg = 42;
       let pt = ctx.encode(msg);
       let ct = ctx.encrypt(&pt, &pk);
       let decrypted = ctx.decrypt(&ct, &sk);

       assert_eq!(ctx.decode(&decrypted), msg);
   }
   ```

3. **Performance Benchmarks:**
   ```rust
   #[bench]
   fn bench_polynomial_multiply(b: &mut Bencher) {
       let manifold = setup_manifold();
       let p1 = random_polynomial(4096, manifold.clone());
       let p2 = random_polynomial(4096, manifold.clone());

       b.iter(|| {
           black_box(p1.multiply(&p2))
       });
   }
   ```

**Success Criteria:**
- ✅ All existing FHE tests pass
- ✅ Polynomial multiply < 10ms for n=4096 (170× speedup)
- ✅ FHE encryption < 10ms (164× speedup from 820ms)
- ✅ Zero reconstruction during operations (use ResidueVector)
- ✅ Homomorphic operations correct (add, multiply)

---

## Performance Expectations

### **Before Integration:**
```
Polynomial multiply: O(n²) = 16,777,216 ops
  └─ Time: ~800ms (naive implementation)

FHE Encryption: ~820ms
  ├─ Polynomial multiply (3x): 800ms
  ├─ Noise sampling: 10ms
  └─ Encoding: 10ms
```

### **After Integration:**
```
Polynomial multiply: O(k × n log n) = 2 × 49,152 ops
  └─ Time: ~5ms (170× speedup)

FHE Encryption: ~5ms (164× speedup)
  ├─ Polynomial multiply (3x): 4.5ms
  ├─ Noise sampling: 0.3ms (QMNF entropy)
  └─ Encoding: 0.2ms (M2M boundary)

Montgomery ops: 4.1ns (existing from residue_space)
NTT transform: O(n log n) per prime
Zero sync overhead: Perfect RNS parallelism
```

---

## Code Reduction Analysis

### **Eliminate:**
- `fhe/rns.rs`: 338 lines → 0 (use CodexManifold)
- `fhe/rns_ntt.rs`: 374 lines → 0 (use generic NTT)
- Duplicate CRT constants → use ResidueConfig
- Duplicate Garner's algorithm → use ResidueVector::to_int()
- Custom moduli selection → use ResidueConfig::from_moduli()

**Total Reduction:** ~700 lines of duplicate code

### **Modify:**
- `fhe/polynomial.rs`: Update to use ResidueVector (~200 lines changed)
- `fhe/encoding.rs`: Integrate M2M (~50 lines changed)
- `fhe/encrypt.rs`: Update types (~30 lines changed)
- `math/discrete.rs`: Generalize NTT (~80 lines changed)

**Total Changes:** ~360 lines

### **Net Result:**
- **Remove:** 700 lines
- **Modify:** 360 lines
- **Add:** 0 lines (all infrastructure exists!)
- **Code Reduction:** 340 lines eliminated

---

## Risk Analysis

### **Low Risk:**
- ✅ All target infrastructure exists and is tested
- ✅ Codex Gear Manifold in production use
- ✅ ResidueVector tested in neural networks
- ✅ M2M tokenizer working with 2.5-4× compression
- ✅ Montgomery arithmetic validated (4.1ns ops)

### **Medium Risk:**
- ⚠️ NTT generalization may need tuning for different primes
  - **Mitigation:** Test with Q0, Q1 from existing rns.rs
  - **Fallback:** Keep naive multiply as backup

- ⚠️ Encoding boundary with M2M may have edge cases
  - **Mitigation:** Comprehensive test suite
  - **Fallback:** Keep existing IntegerEncoder as reference

### **Rollback Plan:**
If integration fails, the standalone RNS-NTT implementation (`fhe/rns_ntt.rs`) remains as fallback. This provides O(n log n) multiplication without full stack integration.

---

## Timeline Estimate

| Phase | Task | Hours | Dependencies |
|-------|------|-------|--------------|
| 1 | NTT Generalization | 8 | None |
| 2 | Polynomial + ResidueVector | 12 | Phase 1 |
| 3 | Remove Duplicate RNS | 4 | Phase 2 |
| 4 | M2M Integration | 6 | Phase 2 |
| 5 | Testing & Validation | 10 | All |
| **TOTAL** | | **40 hours** | |

**Completion Target:** 5 working days (8 hours/day)

---

## Success Metrics

### **Functional:**
- [ ] All FHE tests pass (encrypt/decrypt, homomorphic ops)
- [ ] Polynomial operations use ResidueVector
- [ ] Zero reconstruction during computation
- [ ] M2M boundaries working (encode/decode)
- [ ] Codex manifold integration verified

### **Performance:**
- [ ] Polynomial multiply < 10ms (n=4096)
- [ ] FHE encryption < 10ms
- [ ] Montgomery ops at 4.1ns (existing)
- [ ] NTT O(n log n) verified

### **Architecture:**
- [ ] No duplicate CRT code
- [ ] Single source of truth (CodexManifold)
- [ ] ResidueVector for all multi-moduli operations
- [ ] M2M for encoding boundaries
- [ ] Zero float contamination maintained

---

## References

### **Existing Infrastructure:**
- `hcvlang/src/codex_gear_manifold.rs` - Codex Gear Manifold (900 lines)
- `hcvlang/src/neural/residue_space.rs` - Residue Space (795 lines)
- `hcvlang/src/neural/manifold_tokenizer.rs` - M2M Tokenizer (643 lines)
- `hcvlang/src/neural/montgomery.rs` - Montgomery Arithmetic (607 lines)
- `hcvlang/src/math/discrete.rs` - NTT Implementation (partial)

### **FHE System:**
- `cryptographic_systems/05_Entropy_Shadow_FHE/` - Target system
- `hcvlang/src/fhe/polynomial.rs` - Current implementation
- `hcvlang/src/fhe/rns.rs` - To be removed
- `hcvlang/src/fhe/operations.rs` - QMNF noise integration

### **Documentation:**
- `m2m-tokenizer/README.md` - M2M architecture
- `FHE_ENTROPY_SHADOW_FIXES.md` - Current analysis
- `CLAUDE.md` - QMNF architecture principles

---

## Approval & Sign-Off

**Requestor:** Claude (Sonnet 4.5) - FHE System Review
**Date:** 2025-11-17
**Priority:** HIGH (Performance critical - 164× speedup)

**Architectural Principles:**
- ✅ Insert into existing stack (do not rewrite)
- ✅ Use CodexManifold for CRT (existing infrastructure)
- ✅ Use ResidueVector for operations (existing infrastructure)
- ✅ Use M2M for boundaries (existing infrastructure)
- ✅ Eliminate duplicate code
- ✅ Maintain integer-only guarantee
- ✅ Zero float contamination

**Next Steps:**
1. Review this work request
2. Approve integration plan
3. Execute phases 1-5
4. Validate performance gains
5. Update documentation

**Fallback:** Standalone RNS-NTT implementation ready if needed

---

**END OF WORK REQUEST**
