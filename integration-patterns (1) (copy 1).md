# Integration Patterns

Cross-paper composition patterns for QMNF implementation.

## Pattern 1: K-Elimination + CRTBigInt

Exact division with parallel residue computation.

```rust
/// CRTBigInt with K-Elimination anchor
pub struct CRTBigIntWithAnchor {
    main: CRTBigInt,      // Main residue system
    anchor: CRTBigInt,    // Anchor for K-Elimination
}

impl CRTBigIntWithAnchor {
    pub fn from_int(x: i128, config: &DualConfig) -> Self {
        Self {
            main: CRTBigInt::from_int_with_primes(x, &config.main_primes),
            anchor: CRTBigInt::from_int_with_primes(x, &config.anchor_primes),
        }
    }
    
    /// Operations maintain both systems in parallel
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            main: self.main.mul(&other.main),
            anchor: self.anchor.mul(&other.anchor),
        }
    }
    
    /// Exact division via K-Elimination
    pub fn exact_div(&self, divisor: i64) -> (Self, i64) {
        let v_m = self.main.reconstruct();
        let v_a = self.anchor.reconstruct();
        
        let k = k_eliminate(v_m, v_a, 
                           self.main.product, 
                           self.anchor.product);
        let x = v_m + k * self.main.product;
        
        let quotient = x / divisor as i128;
        let remainder = (x % divisor as i128) as i64;
        
        (Self::from_int(quotient, self.main.config), remainder)
    }
}
```

**Validation:**
```
□ Anchor residues updated on every main operation
□ gcd(main_product, anchor_product) = 1
□ Division produces identical result to BigInt/d
```

## Pattern 2: Persistent Montgomery + CRTBigInt

All residues in Montgomery form throughout.

```rust
/// CRTBigInt with persistent Montgomery per lane
pub struct MontgomeryCRTBigInt {
    residues: Vec<MontgomeryPersistent>,  // Each lane in Montgomery form
    config: Arc<MontCRTConfig>,
}

impl MontgomeryCRTBigInt {
    /// Entry point (conversion happens here)
    pub fn from_int(x: i128, config: Arc<MontCRTConfig>) -> Self {
        let residues = config.primes.par_iter()
            .enumerate()
            .map(|(i, &p)| {
                let r = (x % p as i128) as u64;
                MontgomeryPersistent::from_standard(r, config.mont[i].clone())
            })
            .collect();
        Self { residues, config }
    }
    
    /// Internal mul - NO conversion
    pub fn mul(&self, other: &Self) -> Self {
        let residues = self.residues.par_iter()
            .zip(&other.residues)
            .map(|(a, b)| a.mul(b))
            .collect();
        Self { residues, config: self.config.clone() }
    }
    
    /// Exit point (conversion happens here)
    pub fn to_int(&self) -> i128 {
        let standard: Vec<i64> = self.residues.iter()
            .map(|r| r.to_standard() as i64)
            .collect();
        garner_reconstruct(&standard, &self.config.primes)
    }
}
```

**Validation:**
```
□ from_int → operations → to_int produces correct result
□ No to_standard() calls between operations
□ Montgomery constants precomputed for all primes
```

## Pattern 3: Shadow Entropy + FHE Operations

Harvest entropy from polynomial arithmetic.

```rust
/// FHE context with integrated shadow entropy
pub struct ShadowFHEContext {
    params: FHEParams,
    shadow: ShadowAccumulator,
    rns: MontgomeryCRTBigInt,
}

impl ShadowFHEContext {
    pub fn hom_mul(&mut self, ct1: &Cipher, ct2: &Cipher) -> Cipher {
        // NTT multiplication generates shadows
        let product = self.ntt_mul_with_harvest(&ct1.poly, &ct2.poly);
        
        // Rescale with K-Elimination
        let rescaled = self.exact_rescale(&product);
        
        // Noise from harvested shadow (zero additional cost)
        let noise = self.shadow.extract().unwrap_or(0);
        
        Cipher::new(rescaled, ct1.level - 1)
    }
    
    fn ntt_mul_with_harvest(&mut self, a: &Poly, b: &Poly) -> Poly {
        // Each butterfly generates shadow bits
        for i in 0..self.params.n {
            let (result, shadow) = butterfly_with_shadow(
                a.coeffs[i], b.coeffs[i], 
                self.params.twiddle[i]
            );
            self.shadow.ingest(shadow, 12);  // ~12 bits per reduction
        }
        // ... complete NTT
    }
}
```

**Validation:**
```
□ Shadow accumulator filled before noise needed
□ Fallback to CSPRNG if shadow insufficient
□ NIST tests pass on extracted entropy
```

## Pattern 4: Full Bootstrap-Free Stack

Complete integration of all innovations.

```rust
pub struct BootstrapFreeFHE {
    // Layer 1: Entropy
    shadow: ShadowAccumulator,
    
    // Layer 2: RNS Arithmetic
    main_rns: MontgomeryCRTConfig,
    anchor_rns: MontgomeryCRTConfig,
    
    // Layer 3: Polynomial
    ntt_config: NTTConfig,  // Twiddles in Montgomery form
    
    // Layer 4: FHE
    params: BFVParams,
}

impl BootstrapFreeFHE {
    pub fn encrypt(&mut self, m: &Plaintext) -> Ciphertext {
        // Entry: convert to Montgomery CRT
        let poly = self.encode_to_mont_crt(m);
        
        // Noise from shadow
        let noise = self.sample_noise();
        
        // NTT in Montgomery form
        let ntt = self.ntt_mont(&poly);
        
        // Encrypt (all integer-only)
        self.encrypt_ntt(&ntt, noise)
    }
    
    pub fn hom_mul(&mut self, a: &Ciphertext, b: &Ciphertext) -> Ciphertext {
        // All operations stay in Montgomery form
        let product = self.poly_mul_mont(&a.c0, &b.c0);
        
        // Rescale with K-Elimination (100% exact)
        let rescaled = self.k_elim_rescale(&product);
        
        // Shadow harvested during operations above
        let noise = self.shadow.extract().unwrap_or_else(|| self.csprng());
        
        Ciphertext { c0: rescaled, level: a.level - 1 }
    }
    
    pub fn decrypt(&self, ct: &Ciphertext, sk: &SecretKey) -> Plaintext {
        // Exit: convert from Montgomery CRT
        let poly = self.decrypt_to_mont_crt(ct, sk);
        self.decode_from_mont_crt(&poly)
    }
}
```

**Validation:**
```
□ No floating-point in entire computation path
□ Conversions only at encrypt/decrypt boundaries
□ K-Elimination used for all divisions
□ Shadow entropy harvested from NTT operations
□ end_to_end_latency < 500ms
□ drift = 0 (provable via integer-only)
```

## Pattern 5: AHOP with Integer-Only Principles

Post-quantum crypto following QMNF principles.

```rust
pub struct AHOPContext {
    q: u64,           // Prime modulus
    word_len: usize,  // Security parameter
}

impl AHOPContext {
    /// Constant-time reflection (no branches on secrets)
    pub fn reflect_ct(&self, k: &[u64; 4], i: usize) -> [u64; 4] {
        // Integer-only, constant-time
        let mut sum: u64 = 0;
        for j in 0..4 {
            let mask = ct_neq(j, i);  // Constant-time not-equal
            sum = sum.wrapping_add(k[j] & mask);
        }
        sum %= self.q;
        
        let new_val = (2u128 * sum as u128 + self.q as u128 - k[i] as u128) 
                      % self.q as u128;
        
        let mut result = *k;
        result[i] = new_val as u64;
        result
    }
    
    /// Apply word with constant-time indexing
    pub fn apply_word_ct(&self, k: &[u64; 4], word: &[u8]) -> [u64; 4] {
        let mut current = *k;
        for &s in word {
            // Compute all four reflections, select the right one
            let r0 = self.reflect_ct(&current, 0);
            let r1 = self.reflect_ct(&current, 1);
            let r2 = self.reflect_ct(&current, 2);
            let r3 = self.reflect_ct(&current, 3);
            
            // Constant-time select based on s
            current = ct_select_4(&[r0, r1, r2, r3], s);
        }
        current
    }
}

/// Constant-time not-equal
fn ct_neq(a: usize, b: usize) -> u64 {
    let diff = (a ^ b) as u64;
    ((diff | diff.wrapping_neg()) >> 63).wrapping_neg()
}
```

**Validation:**
```
□ No floating-point operations
□ No branches dependent on secret data
□ No memory access patterns dependent on secrets
□ Timing variance < 1% across all inputs
```

## Anti-Patterns (What NOT to Do)

### Don't: Mix floating-point into integer path
```rust
// BAD
let k = ((v_a - v_m) as f64 / M as f64).round() as i128;

// GOOD
let k = (v_a - v_m) * m_inv % A;
```

### Don't: Convert Montgomery on every operation
```rust
// BAD
fn mul(a: u64, b: u64) -> u64 {
    let a_mont = to_mont(a);
    let b_mont = to_mont(b);
    from_mont(mont_mul(a_mont, b_mont))
}

// GOOD
fn mul(a: &MontPersistent, b: &MontPersistent) -> MontPersistent {
    a.mul(b)  // Stays in Montgomery form
}
```

### Don't: Ignore anchor residues
```rust
// BAD
let result = main_crt.div(divisor);  // Where's the anchor?

// GOOD
let (result, rem) = main_crt.div_with_anchor(divisor, &anchor_crt);
```

### Don't: Use external RNG when shadows available
```rust
// BAD (for FHE noise)
let noise = thread_rng().gen::<u64>();

// GOOD
let noise = self.shadow.extract().unwrap_or_else(|| self.fallback_rng());
```
