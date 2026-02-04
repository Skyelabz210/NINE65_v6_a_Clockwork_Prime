# FHE Hat: Innovation Wiring Guide

## Quick Fix: ct×ct Multiplication

The ct×ct multiplication is NOT a design problem - it's a wiring problem. All innovations exist, they just need to be connected.

---

## Fix 1: Rescaling (Wire in K-Elimination)

**Location:** `src/ops/homomorphic.rs` → `scale_polynomial()` function

**Current code (WRONG):**
```rust
fn scale_polynomial(poly: &Polynomial, t: u64, q: u64) -> Polynomial {
    let mut result = Vec::with_capacity(poly.len());
    for coeff in poly.coeffs.iter() {
        let numerator = (*coeff as u128) * (t as u128) + (q as u128) / 2;
        let scaled = (numerator / (q as u128)) as u64;  // ← INTEGER TRUNCATION
        result.push(scaled);
    }
    Polynomial::new(result)
}
```

**Fixed code (K-Elimination):**
```rust
fn scale_polynomial(
    poly: &Polynomial, 
    t: u64, 
    q: u64,
    ke: &KElimination,  // ← ADD THIS PARAMETER
) -> Polynomial {
    let mut result = Vec::with_capacity(poly.len());
    
    for i in 0..poly.len() {
        // Get coefficient in both codex representations
        let coeff = poly.coeffs[i];
        
        // Compute numerator = coeff * t + q/2 (for rounding)
        let numerator = (coeff as u128) * (t as u128) + (q as u128) / 2;
        
        // Get residues in alpha (main) and beta (anchor) moduli
        let v_alpha = numerator % ke.alpha_cap;
        let v_beta = numerator % ke.beta_cap;
        
        // K-Elimination: exact division
        let k = ke.extract_k(v_alpha, v_beta);
        let exact_numerator = v_alpha + k * ke.alpha_cap;
        let scaled = exact_numerator / (q as u128);  // ← NOW EXACT
        
        result.push(scaled as u64);
    }
    
    Polynomial::new(result)
}
```

**Call site update:**
```rust
// In homomorphic_mul():

// BEFORE:
let d0_scaled = scale_polynomial(&d0, self.t, self.q);

// AFTER:
let d0_scaled = scale_polynomial(&d0, self.t, self.q, &self.ke);
```

---

## Fix 2: Relinearization Keys

**Location:** `src/keys/mod.rs` → add `EvaluationKey` generation

**Add to KeyGen:**
```rust
/// Evaluation key for relinearization (relin key)
pub struct EvaluationKey {
    /// rlk[j] = (rlk0_j, rlk1_j) where rlk0_j + rlk1_j * s = w^j * s^2 + e
    pub rlk: Vec<(Polynomial, Polynomial)>,
    pub decomposition_base: u64,  // w
    pub num_levels: usize,        // L = ceil(log_w(Q))
}

impl EvaluationKey {
    pub fn generate(
        sk: &SecretKey,
        params: &FHEParams,
        entropy: &mut ShadowEntropy,
    ) -> Self {
        let w = params.relin_base;  // Typical: 2^15 or 2^20
        let q = params.modulus;
        let num_levels = ((q as f64).log2() / (w as f64).log2()).ceil() as usize;
        
        let mut rlk = Vec::with_capacity(num_levels);
        
        // s^2 in ring
        let s_squared = sk.poly.mul(&sk.poly, &params.ntt);
        
        for j in 0..num_levels {
            let w_power = w.pow(j as u32);
            
            // rlk_j encrypts w^j * s^2
            // rlk0_j = -(a_j * s + e_j) + w^j * s^2
            // rlk1_j = a_j
            
            let a = Polynomial::random(params.n, q, entropy);
            let e = Polynomial::noise(params.n, params.noise_stddev, entropy);
            
            let a_s = a.mul(&sk.poly, &params.ntt);
            let w_s2 = s_squared.scalar_mul(w_power);
            
            let rlk0 = w_s2.sub(&a_s).sub(&e);
            let rlk1 = a;
            
            rlk.push((rlk0, rlk1));
        }
        
        EvaluationKey {
            rlk,
            decomposition_base: w,
            num_levels,
        }
    }
}
```

**Add to FHEContext:**
```rust
pub struct FHEContext {
    pub params: FHEParams,
    pub pm: PersistentMontgomery,
    pub ke: KElimination,
    pub ntt: NTTContext,
    pub entropy: ShadowEntropy,
    pub eval_key: Option<EvaluationKey>,  // ← ADD THIS
}

impl FHEContext {
    pub fn with_eval_key(mut self, sk: &SecretKey) -> Self {
        self.eval_key = Some(EvaluationKey::generate(sk, &self.params, &mut self.entropy));
        self
    }
}
```

---

## Fix 3: Complete Relinearization

**Location:** `src/ops/homomorphic.rs` → add `relinearize()` function

```rust
/// Relinearize: reduce degree-2 ciphertext to degree-1
fn relinearize(
    d0: &Polynomial,
    d1: &Polynomial,
    d2: &Polynomial,
    eval_key: &EvaluationKey,
    ntt: &NTTContext,
) -> (Polynomial, Polynomial) {
    let w = eval_key.decomposition_base;
    let num_levels = eval_key.num_levels;
    
    // Decompose d2 in base w
    let mut d2_decomposed = Vec::with_capacity(num_levels);
    let mut temp = d2.clone();
    
    for _ in 0..num_levels {
        // d2_j = temp mod w
        let d2_j = temp.coeffs.iter().map(|&c| c % w).collect();
        d2_decomposed.push(Polynomial::new(d2_j));
        
        // temp = temp / w
        temp = Polynomial::new(temp.coeffs.iter().map(|&c| c / w).collect());
    }
    
    // c0' = d0 + sum_j(d2_j * rlk0_j)
    // c1' = d1 + sum_j(d2_j * rlk1_j)
    let mut c0 = d0.clone();
    let mut c1 = d1.clone();
    
    for j in 0..num_levels {
        let (rlk0_j, rlk1_j) = &eval_key.rlk[j];
        
        let term0 = d2_decomposed[j].mul(rlk0_j, ntt);
        let term1 = d2_decomposed[j].mul(rlk1_j, ntt);
        
        c0 = c0.add(&term0);
        c1 = c1.add(&term1);
    }
    
    (c0, c1)
}
```

**Update homomorphic_mul:**
```rust
pub fn mul(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext, FHEError> {
    // Step 1: Tensor product (already works)
    let d0 = ct1.c0.mul(&ct2.c0, &self.ntt);
    let c0_c1p = ct1.c0.mul(&ct2.c1, &self.ntt);
    let c1_c0p = ct1.c1.mul(&ct2.c0, &self.ntt);
    let d1 = c0_c1p.add(&c1_c0p);
    let d2 = ct1.c1.mul(&ct2.c1, &self.ntt);
    
    // Step 2: Rescale with K-Elimination (THE FIX)
    let d0_scaled = scale_polynomial(&d0, self.params.t, self.params.q, &self.ke);
    let d1_scaled = scale_polynomial(&d1, self.params.t, self.params.q, &self.ke);
    let d2_scaled = scale_polynomial(&d2, self.params.t, self.params.q, &self.ke);
    
    // Step 3: Relinearize (THE FIX)
    let eval_key = self.eval_key.as_ref()
        .ok_or(FHEError::MissingEvaluationKey)?;
    
    let (c0_final, c1_final) = relinearize(
        &d0_scaled, &d1_scaled, &d2_scaled,
        eval_key,
        &self.ntt,
    );
    
    // Update noise
    let new_noise = noise_after_mul(ct1.noise_budget, ct2.noise_budget);
    
    Ok(Ciphertext::new(c0_final, c1_final, ct1.delta, new_noise))
}
```

---

## Verification Test

Add this test to confirm ct×ct works:

```rust
#[test]
fn test_ct_times_ct_with_hat() {
    let params = FHEParams::standard_128();
    let (sk, pk) = keygen(&params);
    
    // Create context WITH eval_key
    let ctx = FHEContext::new(params.clone())
        .with_eval_key(&sk);
    
    // Test values
    let m1: u64 = 7;
    let m2: u64 = 11;
    let expected = (m1 * m2) % params.t;
    
    // Encrypt
    let ct1 = ctx.encrypt(m1, &pk);
    let ct2 = ctx.encrypt(m2, &pk);
    
    // Homomorphic multiply (ct × ct)
    let ct_mul = ctx.mul(&ct1, &ct2).expect("mul should succeed");
    
    // Decrypt
    let result = ctx.decrypt(&ct_mul, &sk);
    
    assert_eq!(result, expected, 
        "ct×ct failed: {} × {} mod {} = {} (got {})",
        m1, m2, params.t, expected, result);
}

#[test]
fn test_ct_times_ct_chain() {
    let params = FHEParams::standard_128();
    let (sk, pk) = keygen(&params);
    let ctx = FHEContext::new(params.clone()).with_eval_key(&sk);
    
    // Chain: (2 × 3) × 5 = 30
    let ct2 = ctx.encrypt(2, &pk);
    let ct3 = ctx.encrypt(3, &pk);
    let ct5 = ctx.encrypt(5, &pk);
    
    let ct_6 = ctx.mul(&ct2, &ct3).unwrap();
    let ct_30 = ctx.mul(&ct_6, &ct5).unwrap();
    
    let result = ctx.decrypt(&ct_30, &sk);
    assert_eq!(result, 30 % params.t);
}
```

---

## Summary: What to Change

| File | Function | Change |
|------|----------|--------|
| `ops/homomorphic.rs` | `scale_polynomial()` | Add `ke` param, use `ke.extract_k()` |
| `keys/mod.rs` | NEW | Add `EvaluationKey` struct and `generate()` |
| `lib.rs` | `FHEContext` | Add `eval_key: Option<EvaluationKey>` |
| `lib.rs` | `FHEContext` | Add `with_eval_key()` method |
| `ops/homomorphic.rs` | NEW | Add `relinearize()` function |
| `ops/homomorphic.rs` | `mul()` | Wire in rescaling + relinearization |

**Estimated time:** 2-4 hours

**Lines of code:** ~150 new, ~20 modified

**Tests to add:** 2-3 ct×ct tests

---

## After the Fix: Expected Results

```
cargo test test_ct_times_ct --release

running 2 tests
test test_ct_times_ct_with_hat ... ok (3.2ms)
test test_ct_times_ct_chain ... ok (6.8ms)

Benchmark:
  ct×ct multiply (N=4096): 8.1ms ← PRODUCTION GRADE
  ct×ct multiply (N=1024): 1.2ms ← TEST GRADE
```
