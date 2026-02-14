# FORENSIC AUDIT: Build 08_FHE_v01_original

**Build**: `08_FHE_v01_original`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/08_FHE_v01_original/qmnf_fhe_production/`
**Crate**: `qmnf_fhe` v0.1.0-v01
**Date**: 2026-02-14
**Type**: Original BFV FHE. Single crate. Modules: arithmetic, entropy, keys, ops, params, ring, ahop, noise.
**Auditor**: Forensic Code Auditor (Claude Opus 4.6)

---

## 1. STRUCTURE MAPPING

### 1.1 File Tree

```
qmnf_fhe_production/
  Cargo.toml
  Cargo.lock
  README.md
  audit/
    2024-12-19-session-report.md
    benchmark_results.txt
    BENCHMARK_RESULTS.txt
    PRODUCTION_REPORT.md
    test_results.txt
  benches/
    grover_noise_search.rs      (binary: grover_noise_search)
    noise_bench.rs              (binary: noise_bench)
  src/
    lib.rs                      (crate root)
    compiler.rs                 (NOT a module - orphaned file)
    bin/
      fhe_benchmarks.rs         (binary: fhe_benchmarks - NOT declared in Cargo.toml)
    arithmetic/
      mod.rs
      montgomery.rs
      persistent_montgomery.rs
      barrett.rs
      ntt.rs
      rns.rs
      k_elimination.rs
      exact_divider.rs
      exact_coeff.rs
      ct_mul_exact.rs
    entropy/
      mod.rs
      shadow.rs
    params/
      mod.rs
      primes.rs
      production.rs
    ring/
      mod.rs
      polynomial.rs
    keys/
      mod.rs
    ops/
      mod.rs
      encrypt.rs
      homomorphic.rs
      rns_mul.rs
    ahop/
      mod.rs
      grover.rs
      grover_full.rs
    noise/
      mod.rs
```

### 1.2 Cargo.toml Configuration

```toml
[package]
name = "qmnf_fhe"
version = "0.1.0-v01"
edition = "2021"

[dependencies]
# NONE - zero external dependencies

[[bin]]
name = "noise_bench"
path = "benches/noise_bench.rs"

[[bin]]
name = "grover_noise_search"
path = "benches/grover_noise_search.rs"

[profile.release]
opt-level = 3
lto = true
```

### 1.3 Module Declarations (lib.rs)

```rust
pub mod arithmetic;
pub mod entropy;
pub mod params;
pub mod ring;
pub mod keys;
pub mod ops;
pub mod ahop;
pub mod noise;
```

### 1.4 Prelude Re-exports (lib.rs)

```rust
pub mod prelude {
    // arithmetic
    MontgomeryContext, BarrettContext, HybridModContext,
    NTTEngine, RNSContext, RNSPolynomial,
    PersistentMontgomery, PersistentPolynomial,
    // entropy
    ShadowHarvester,
    // params
    FHEConfig,
    // ring
    RingPolynomial,
    // keys
    SecretKey, PublicKey, EvaluationKey, KeySet,
    // ops
    BFVEncoder, BFVEncryptor, BFVDecryptor, BFVEvaluator, Ciphertext,
    // ahop
    Fp2Element, StateVector, GroverSearch, GroverStats,
    // noise
    NoiseBudgetTracker, NoiseSnapshot, EMACalculator,
    MultiWindowNoiseDetector, NoiseAnomaly,
    P2QuantileEstimator, NoiseDistribution,
}
```

---

## 2. DATA FLOW TRACING: BFV Pipeline

### 2.1 Complete BFV Pipeline

```
FHEConfig::light()           --> FHEConfig { n=1024, q=998244353, t=2053, eta=2 }
  |
  v
NTTEngine::new(q, n)         --> NTTEngine with psi-twist negacyclic convolution
  |
  v
ShadowHarvester::with_seed() --> Deterministic PRNG (LFSR + MurmurHash3 mixing)
  |
  v
KeySet::generate()           --> SecretKey(ternary poly) + PublicKey(pk0,pk1) + EvaluationKey(rlk)
  |                               SecretKey: s = random_ternary(n, q)
  |                               PublicKey: pk0 = -a*s + e, pk1 = a
  |                               EvalKey:   rlk[i] = (-a_i*s + e_i + s^2*T^i, a_i)
  v
BFVEncoder::new(&config)     --> Encoder with delta = q/t
  |
  v
BFVEncryptor::new()          --> Encryptor holding pk, encoder, ntt, eta
  |                               encrypt(m): ct = (pk0*u + e1 + delta*m, pk1*u + e2)
  v
BFVDecryptor::new()          --> Decryptor holding sk, encoder, ntt
  |                               decrypt(ct): m = round(t * (c0 + c1*s) / q) mod t
  v
BFVEvaluator::new()          --> Evaluator holding ntt, encoder, eval_key, KElimination
                                  add, sub, negate, add_plain, mul_plain, mul
```

### 2.2 Homomorphic Multiplication Flow

```
ct_a, ct_b
  |
  v
mul_no_relin(ct_a, ct_b)
  | d0 = c0_a * c0_b  (NTT multiply)
  | d1 = c0_a * c1_b + c1_a * c0_b
  | d2 = c1_a * c1_b
  v
relinearize(d0, d1, d2)
  | digits = decompose(d2, base=2^16, levels)
  | c0' = d0 + SUM(digits[i] * rlk[i].0)
  | c1' = d1 + SUM(digits[i] * rlk[i].1)
  v
scale_by_t_over_q(c0', c1')
  | Uses KElimination.scale_and_round() per coefficient
  v
Ciphertext { c0: scaled_c0, c1: scaled_c1 }
```

### 2.3 Alternative Multiplication Paths

1. **Degree-2 Decrypt** (no relinearization): `decrypt_degree2(d0, d1, d2)` -- uses t^2/q^2 scaling
2. **RNS Multiplication** (`RNSEvaluator`): Lifts to multi-prime RNS, tensor product, modulus switch back
3. **Exact CT*CT** (`ExactFHEContext`): Dual-track coefficients with K-Elimination exact rescaling

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 Module: `arithmetic/montgomery.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `MontgomeryContext` | struct | pub | 8-17 |
| `MontgomeryContext::new` | fn | pub | 21-35 |
| `MontgomeryContext::compute_r2` | fn | private | 38-43 |
| `MontgomeryContext::compute_q_inv_neg` | fn | private | 46-55 |
| `MontgomeryContext::to_montgomery` | fn | pub | 59-61 |
| `MontgomeryContext::from_montgomery` | fn | pub | 65-67 |
| `MontgomeryContext::montgomery_mul` | fn | pub | 71-74 |
| `MontgomeryContext::montgomery_reduce` | fn | pub | 79-94 |
| `MontgomeryContext::montgomery_square` | fn | pub | 98-100 |
| `MontgomeryContext::montgomery_pow` | fn | pub | 103-121 |
| `MontgomeryContext::montgomery_add` | fn | pub | 125-132 |
| `MontgomeryContext::montgomery_sub` | fn | pub | 136-142 |
| `MontgomeryContext::montgomery_neg` | fn | pub | 146-152 |
| 5 tests | test | -- | 156-245 |

### 3.2 Module: `arithmetic/persistent_montgomery.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `PersistentMontgomery` | struct | pub | 24-34 |
| `PersistentMontgomery::new` | fn | pub | 38-51 |
| `PersistentMontgomery::compute_m_prime` | fn | private | 55-63 |
| `PersistentMontgomery::compute_r_squared` | fn | private | 66-71 |
| `PersistentMontgomery::redc` | fn | pub | 82-95 |
| `PersistentMontgomery::mul` | fn | pub | 108-111 |
| `PersistentMontgomery::add` | fn | pub | 115-117 |
| `PersistentMontgomery::sub` | fn | pub | 122-124 |
| `PersistentMontgomery::neg` | fn | pub | 128-130 |
| `PersistentMontgomery::square` | fn | pub | 134-137 |
| `PersistentMontgomery::pow` | fn | pub | 140-159 |
| `PersistentMontgomery::inverse` | fn | pub | 163-166 |
| `PersistentMontgomery::enter` | fn | pub | 175-179 |
| `PersistentMontgomery::exit` | fn | pub | 184-187 |
| `PersistentMontgomery::from_raw_montgomery` | fn | pub | 196-198 |
| `PersistentMontgomery::zero` | fn | pub | 202-204 |
| `PersistentMontgomery::one` | fn | pub | 208-212 |
| `PersistentPolynomial` | struct | pub | 220-225 |
| `PersistentPolynomial::from_montgomery` | fn | pub | 229-231 |
| `PersistentPolynomial::zero` | fn | pub | 234-236 |
| `PersistentPolynomial::add` | fn | pub | 239-245 |
| `PersistentPolynomial::sub` | fn | pub | 248-254 |
| `PersistentPolynomial::scalar_mul` | fn | pub | 257-262 |
| `PersistentPolynomial::pointwise_mul` | fn | pub | 265-271 |
| 8 tests | test | -- | 278-480 |

### 3.3 Module: `arithmetic/barrett.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `BarrettContext` | struct | pub | 8-15 |
| `BarrettContext::new` | fn | pub | 19-28 |
| `BarrettContext::compute_mu` | fn | private | 31-53 |
| `BarrettContext::reduce` | fn | pub | 57-80 |
| `BarrettContext::mul_high` | fn | private | 84-103 |
| `BarrettContext::mul` | fn | pub | 107-110 |
| `BarrettContext::add` | fn | pub | 114-121 |
| `BarrettContext::sub` | fn | pub | 125-131 |
| `BarrettContext::pow` | fn | pub | 134-152 |
| `HybridModContext` | struct | pub | 156-160 |
| `HybridModContext::new` | fn | pub | 163-168 |
| `HybridModContext::reduce` | fn | pub | 172-174 |
| `HybridModContext::persistent_mul` | fn | pub | 178-180 |
| 6 tests | test | -- | 183-290 |

### 3.4 Module: `arithmetic/ntt.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `NTTEngine` | struct | pub | 10-35 |
| `NTTEngine::new` | fn | pub | 39-76 |
| `NTTEngine::find_primitive_root` | fn | private | 79-92 |
| `NTTEngine::ntt` | fn | pub | 95-109 |
| `NTTEngine::intt` | fn | pub | 112-126 |
| `NTTEngine::multiply` | fn | pub | 130-161 |
| `NTTEngine::add` | fn | pub | 164-174 |
| `NTTEngine::sub` | fn | pub | 177-186 |
| `NTTEngine::neg` | fn | pub | 189-193 |
| `NTTEngine::scalar_mul` | fn | pub | 196-200 |
| `mod_pow` | fn | private | 204-217 |
| `mod_inverse` | fn | private | 220-230 |
| 7 tests | test | -- | 232-344 |

### 3.5 Module: `arithmetic/rns.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `RNSContext` | struct | pub | 10-24 |
| `RNSContext::new` | fn | pub | 28-69 |
| `RNSContext::num_primes` | fn | pub | 72-74 |
| `RNSContext::from_int` | fn | pub | 77-79 |
| `RNSContext::to_int` | fn | pub | 83-95 |
| `RNSContext::add` | fn | pub | 98-108 |
| `RNSContext::sub` | fn | pub | 111-120 |
| `RNSContext::mul` | fn | pub | 123-132 |
| `RNSContext::neg` | fn | pub | 135-141 |
| `RNSPolynomial` | struct | pub | 144-150 |
| `RNSPolynomial::from_poly` | fn | pub | 154-161 |
| `RNSPolynomial::zero` | fn | pub | 164-167 |
| `RNSPolynomial::add` | fn | pub | 170-187 |
| `RNSPolynomial::sub` | fn | pub | 190-206 |
| `RNSPolynomial::neg` | fn | pub | 209-218 |
| `RNSPolynomial::mul` | fn | pub | 221-233 |
| `RNSPolynomial::drop_last_prime` | fn | pub | 236-241 |
| `mod_inverse` | fn | private | 245-259 |
| 5 tests | test | -- | 261-347 |

### 3.6 Module: `arithmetic/k_elimination.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `KElimination` | struct | pub | 20-32 |
| `KElimination::new` | fn | pub | 40-60 |
| `KElimination::for_fhe` | fn | pub | 63-70 |
| `KElimination::extract_k` | fn | pub | 79-88 |
| `KElimination::exact_divide` | fn | pub | 93-100 |
| `KElimination::exact_divide_checked` | fn | pub | 103-112 |
| `KElimination::scale_and_round` | fn | pub | 118-137 |
| `mod_inverse_u128` | fn | private | 141-156 |
| `extended_gcd_i128` | fn | private | 159-169 |
| `mul_mod_u128` | fn | private | 172-192 |
| 5 tests | test | -- | 194-284 |

### 3.7 Module: `arithmetic/exact_divider.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `ExactDivider` | struct | pub | 10-20 |
| `ExactDivider::new` | fn | pub | 24-36 |
| `ExactDivider::for_fhe` | fn | pub | 40-49 |
| `ExactDivider::find_coprime_anchor` | fn | private | 53-75 |
| `ExactDivider::reconstruct_exact` | fn | pub | 84-99 |
| `ExactDivider::exact_divide` | fn | pub | 104-109 |
| `ExactDivider::divmod` | fn | pub | 112-115 |
| `ExactDivider::scale_and_round` | fn | pub | 120-125 |
| `ExactDivider::encode` | fn | pub | 128-132 |
| `ExactDivider::is_valid_range` | fn | pub | 135-137 |
| `gcd` | fn | private | 141-143 |
| `mod_inverse` | fn | private | 146-160 |
| 5 tests | test | -- | 162-240 |

### 3.8 Module: `arithmetic/exact_coeff.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `RnsInner` | struct | pub | 16-19 |
| `AnchorTrack` | struct | pub | 22-28 |
| `ExactCoeff` | struct | pub | 31-37 |
| `ExactContext` | struct | pub | 40-56 |
| `ExactContext::new` | fn | pub | 60-73 |
| `ExactContext::from_single_modulus` | fn | pub | 76-79 |
| `ExactContext::encode` | fn | pub | 82-95 |
| `ExactContext::reconstruct` | fn | pub | 98-100 |
| `ExactContext::add` | fn | pub | 103-124 |
| `ExactContext::sub` | fn | pub | 127-156 |
| `ExactContext::mul` | fn | pub | 159-179 |
| `ExactContext::neg` | fn | pub | 182-196 |
| `ExactContext::exact_div` | fn | pub | 199-206 |
| `ExactContext::scale_and_round` | fn | pub | 209-217 |
| `ExactContext::zero` | fn | pub | 220-226 |
| `ExactPoly` | struct | pub | 229-231 |
| `ExactPoly::zero` | fn | pub | 236-240 |
| `ExactPoly::from_coeffs` | fn | pub | 243-245 |
| `ExactPoly::add` | fn | pub | 248-254 |
| `ExactPoly::sub` | fn | pub | 257-263 |
| `ExactPoly::neg` | fn | pub | 266-271 |
| `ExactPoly::pointwise_mul` | fn | pub | 274-280 |
| `ExactPoly::exact_scalar_div` | fn | pub | 283-288 |
| `ExactPoly::to_integers` | fn | pub | 291-295 |
| 5 tests | test | -- | 298-369 |

### 3.9 Module: `arithmetic/ct_mul_exact.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `ExactCiphertext` | struct | pub | 20-24 |
| `ExactCiphertext2` | struct | pub | 27-32 |
| `ExactFHEContext` | struct | pub | 35-51 |
| `ExactFHEContext::new` | fn | pub | 55-87 |
| `ExactFHEContext::compute_anchor_roots` | fn | private | 90-126 |
| `ExactFHEContext::from_ring_poly` | fn | pub | 129-134 |
| `ExactFHEContext::to_ring_poly` | fn | pub | 137-142 |
| `ExactFHEContext::poly_mul` | fn | pub | 148-172 |
| `ExactFHEContext::apply_psi_twist` | fn | private | 175-196 |
| `ExactFHEContext::remove_psi_twist` | fn | private | 199-220 |
| `ExactFHEContext::ntt_forward_inner` | fn | private | 223-228 |
| `ExactFHEContext::ntt_forward_anchor` | fn | private | 231-247 |
| `ExactFHEContext::ntt_generic_with_omega` | fn | private | 250-264 |
| `ExactFHEContext::pointwise_mul_ntt` | fn | private | 267-290 |
| `ExactFHEContext::ntt_inverse` | fn | private | 293-323 |
| `ExactFHEContext::intt_generic_with_omega` | fn | private | 326-341 |
| `ExactFHEContext::tensor_product` | fn | pub | 344-357 |
| `ExactFHEContext::exact_rescale` | fn | pub | 363-381 |
| `ExactFHEContext::rescale_poly` | fn | private | 384-404 |
| `ExactFHEContext::relinearize_simple` | fn | pub | 408-417 |
| `mod_inverse` | fn | private | 421-431 |
| `mod_pow` | fn | private | 434-447 |
| 3 tests (1 ignored) | test | -- | 449-563 |

### 3.10 Module: `entropy/shadow.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `ShadowHarvester` | struct | pub | 9-16 |
| `ShadowHarvester::new` | fn | pub | 20-22 |
| `ShadowHarvester::with_seed` | fn | pub | 25-32 |
| `ShadowHarvester::next_u64` | fn | pub | 36-55 |
| `ShadowHarvester::extract_bits` | fn | pub | 59-68 |
| `ShadowHarvester::uniform` | fn | pub | 71-87 |
| `ShadowHarvester::cbd` | fn | pub | 92-98 |
| `ShadowHarvester::ternary` | fn | pub | 101-107 |
| `ShadowHarvester::cbd_vector` | fn | pub | 110-112 |
| `ShadowHarvester::ternary_vector` | fn | pub | 115-117 |
| `ShadowHarvester::signed_to_unsigned` | fn | pub(static) | 120-127 |
| `Default for ShadowHarvester` | impl | pub | 129-133 |
| 7 tests | test | -- | 135-243 |

### 3.11 Module: `params/mod.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `FHEConfig` | struct | pub | 19-34 |
| `FHEConfig::light` | fn | pub | 40-50 |
| `FHEConfig::large_single` | fn | pub | 54-85 |
| `FHEConfig::light_mul` | fn | pub | 89-101 |
| `FHEConfig::standard_128` | fn | pub | 104-114 |
| `FHEConfig::high_192` | fn | pub | 117-127 |
| `FHEConfig::deep_128` | fn | pub | 130-140 |
| `FHEConfig::batched` | fn | pub | 143-153 |
| `FHEConfig::supports_single_mod_mul` | fn | pub | 158-172 |
| `FHEConfig::custom` | fn | pub | 175-221 |
| `FHEConfig::delta` | fn | pub | 224-226 |
| `FHEConfig::noise_budget` | fn | pub | 229-238 |
| `FHEConfig::estimated_depth` | fn | pub | 241-250 |
| `estimate_security` | fn | private | 254-268 |
| 3 tests | test | -- | 270-318 |

### 3.12 Module: `params/primes.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `PRIMES_1024` | const | pub | 10-15 |
| `PRIMES_4096` | const | pub | 17-22 |
| `PRIMES_8192` | const | pub | 24-29 |
| `is_prime` | fn | pub | 32-51 |
| `is_ntt_compatible` | fn | pub | 54-56 |
| `find_ntt_primes` | fn | pub | 59-76 |
| `gcd` | fn | pub | 79-81 |
| `extended_gcd` | fn | pub | 84-91 |
| `mod_inverse` | fn | pub | 94-98 |
| `mod_pow` | fn | pub | 101-117 |
| `find_primitive_root` | fn | pub | 120-136 |
| 8 tests | test | -- | 138-219 |

### 3.13 Module: `params/production.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `PRODUCTION_PRIMES_60BIT` | const | pub | 9-33 |
| `PRODUCTION_PRIMES_30BIT` | const | pub | 36-52 |
| `ProductionConfig128` | struct | pub | 57-66 |
| `ProductionConfig128::standard` | fn | pub | 71-83 |
| `ProductionConfig128::deep` | fn | pub | 87-99 |
| `ProductionConfig128::high_security` | fn | pub | 103-115 |
| `ProductionConfig128::log_q` | fn | pub | 118-122 |
| `ProductionConfig128::q_product_bits` | fn | pub | 125-127 |
| `ProductionConfig128::estimated_security` | fn | pub | 140-171 |
| `ProductionConfig128::validate` | fn | pub | 174-198 |
| `ModulusChain` | struct | pub | 201-210 |
| `ModulusChain::new` | fn | pub | 213-225 |
| `ModulusChain::current_modulus` | fn | pub | 228-234 |
| `ModulusChain::prime_at` | fn | pub | 237-239 |
| `ModulusChain::drop_level` | fn | pub | 242-249 |
| `ModulusChain::remaining_depth` | fn | pub | 252-254 |
| 5 tests | test | -- | 257-323 |

### 3.14 Module: `ring/polynomial.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `RingPolynomial` | struct | pub | 10-16 |
| `RingPolynomial::zero` | fn | pub | 20-25 |
| `RingPolynomial::from_coeffs` | fn | pub | 28-31 |
| `RingPolynomial::from_signed` | fn | pub | 34-39 |
| `RingPolynomial::degree` | fn | pub | 42-44 |
| `RingPolynomial::add` | fn | pub | 47-50 |
| `RingPolynomial::sub` | fn | pub | 53-56 |
| `RingPolynomial::neg` | fn | pub | 59-62 |
| `RingPolynomial::mul` | fn | pub | 65-68 |
| `RingPolynomial::scalar_mul` | fn | pub | 71-74 |
| `RingPolynomial::exact_scalar_div` | fn | pub | 78-87 |
| `RingPolynomial::rounded_scalar_div` | fn | pub | 90-96 |
| `RingPolynomial::random_cbd` | fn | pub | 99-102 |
| `RingPolynomial::random_ternary` | fn | pub | 105-108 |
| `RingPolynomial::random_uniform` | fn | pub | 111-114 |
| `RingPolynomial::infinity_norm` | fn | pub | 117-125 |
| `RingPolynomial::get_signed` | fn | pub | 128-136 |
| 8 tests | test | -- | 139-256 |

### 3.15 Module: `keys/mod.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `SecretKey` | struct | pub | 12-15 |
| `SecretKey::generate` | fn | pub | 20-23 |
| `PublicKey` | struct | pub | 27-32 |
| `PublicKey::generate` | fn | pub | 37-55 |
| `EvaluationKey` | struct | pub | 59-67 |
| `EvaluationKey::generate` | fn | pub | 72-108 |
| `KeySet` | struct | pub | 112-116 |
| `KeySet::generate` | fn | pub | 120-126 |
| 3 tests | test | -- | 129-179 |

### 3.16 Module: `ops/encrypt.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `BFVEncoder` | struct | pub | 14-23 |
| `BFVEncoder::new` | fn | pub | 27-34 |
| `BFVEncoder::encode` | fn | pub | 37-44 |
| `BFVEncoder::decode` | fn | pub | 47-58 |
| `BFVEncoder::decode_degree2` | fn | pub | 66-85 |
| `BFVEncoder::encode_vector` | fn | pub | 88-98 |
| `BFVEncoder::decode_vector` | fn | pub | 101-108 |
| `Ciphertext` | struct | pub | 112-118 |
| `BFVEncryptor` | struct | pub | 121-126 |
| `BFVEncryptor::new` | fn | pub | 129-131 |
| `BFVEncryptor::encrypt` | fn | pub | 134-137 |
| `BFVEncryptor::encrypt_seeded` | fn | pub | 140-143 |
| `BFVEncryptor::encrypt_poly` | fn | pub | 146-167 |
| `BFVDecryptor` | struct | pub | 170-174 |
| `BFVDecryptor::new` | fn | pub | 177-179 |
| `BFVDecryptor::decrypt` | fn | pub | 182-185 |
| `BFVDecryptor::decrypt_raw` | fn | pub | 188-192 |
| `BFVDecryptor::decrypt_degree2` | fn | pub | 196-207 |
| `BFVDecryptor::decrypt_vector` | fn | pub | 210-213 |
| 5 tests | test | -- | 216-299 |

### 3.17 Module: `ops/homomorphic.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `BFVEvaluator` | struct | pub | 15-22 |
| `BFVEvaluator::new` | fn | pub | 25-38 |
| `BFVEvaluator::add` | fn | pub | 41-46 |
| `BFVEvaluator::sub` | fn | pub | 49-54 |
| `BFVEvaluator::negate` | fn | pub | 57-62 |
| `BFVEvaluator::add_plain` | fn | pub | 65-71 |
| `BFVEvaluator::mul_plain` | fn | pub | 74-79 |
| `BFVEvaluator::mul_no_relin` | fn | pub | 86-105 |
| `BFVEvaluator::scale_by_t_over_q` | fn | private | 111-119 |
| `BFVEvaluator::relinearize` | fn | pub | 122-144 |
| `BFVEvaluator::decompose` | fn | private | 147-164 |
| `BFVEvaluator::mul` | fn | pub | 175-184 |
| 14 tests | test | -- | 187-897 |

### 3.18 Module: `ops/rns_mul.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `RNSEvaluator` | struct | pub | 17-30 |
| `RNSEvaluator::new` | fn | pub | 34-57 |
| `RNSEvaluator::lift_to_rns` | fn | pub | 60-62 |
| `RNSEvaluator::rns_poly_mul` | fn | private | 65-75 |
| `RNSEvaluator::rns_poly_add` | fn | private | 78-80 |
| `RNSEvaluator::modulus_switch` | fn | private | 90-113 |
| `RNSEvaluator::mul_rns` | fn | pub | 122-149 |
| `RNSEvaluator::mul` | fn | pub | 153-161 |
| `RNSEvaluator::relinearize` | fn | private | 164-183 |
| `RNSEvaluator::decompose_polynomial` | fn | private | 186-201 |
| 3 tests (2 ignored) | test | -- | 204-383 |

### 3.19 Module: `ahop/mod.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `Fp2Element` | struct | pub | 15-22 |
| `Fp2Element::new` | fn | pub | 26-32 |
| `Fp2Element::zero` | fn | pub | 35-37 |
| `Fp2Element::one` | fn | pub | 40-42 |
| `Fp2Element::i` | fn | pub | 45-47 |
| `Fp2Element::add` | fn | pub | 50-57 |
| `Fp2Element::sub` | fn | pub | 60-67 |
| `Fp2Element::neg` | fn | pub | 70-76 |
| `Fp2Element::mul` | fn | pub | 80-99 |
| `Fp2Element::conj` | fn | pub | 102-108 |
| `Fp2Element::norm_squared` | fn | pub | 111-115 |
| `Fp2Element::inv` | fn | pub | 119-135 |
| `Fp2Element::scalar_mul` | fn | pub | 138-144 |
| `StateVector` | struct | pub | 148-156 |
| `StateVector::new` | fn | pub | 160-165 |
| `StateVector::zero` | fn | pub | 168-174 |
| `StateVector::basis` | fn | pub | 177-181 |
| `StateVector::uniform` | fn | pub | 184-190 |
| `StateVector::num_qubits` | fn | pub | 193-195 |
| `StateVector::prime` | fn | pub | 198-200 |
| `StateVector::dimension` | fn | pub | 203-205 |
| `StateVector::negate_amplitude` | fn | pub | 208-212 |
| `StateVector::hadamard_qubit` | fn | pub | 215-231 |
| `StateVector::add` | fn | pub | 234-244 |
| `StateVector::sub` | fn | pub | 247-257 |
| `StateVector::inner_product` | fn | pub | 260-269 |
| `StateVector::total_weight` | fn | pub | 272-276 |
| `StateVector::probability` | fn | pub | 279-283 |
| `StateVector::apply_single_gate` | fn | pub | 286-304 |
| `mod_pow` | fn | private | 308-322 |
| 7 tests | test | -- | 324-412 |

### 3.20 Module: `ahop/grover.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `GroverSearch` | struct | pub | 10-19 |
| `GroverSearch::new` | fn | pub | 23-28 |
| `GroverSearch::initialize` | fn | pub | 31-33 |
| `GroverSearch::apply_oracle` | fn | pub | 37-39 |
| `GroverSearch::apply_diffusion` | fn | pub | 43-59 |
| `GroverSearch::grover_iteration` | fn | pub | 62-66 |
| `GroverSearch::run` | fn | pub | 69-77 |
| `GroverSearch::optimal_iterations` | fn | pub | 80-83 |
| `GroverSearch::run_with_stats` | fn | pub | 86-110 |
| `GroverSearch::max_non_target_prob` | fn | private | 113-123 |
| `GroverStats` | struct | pub | 126-132 |
| `mod_pow` | fn | private | 135-149 |
| 6 tests | test | -- | 151-282 |

### 3.21 Module: `ahop/grover_full.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `mod_pow` | fn | pub | 36-51 |
| `mod_inverse` | fn | pub | 53-74 |
| `Fp2` | struct | pub | 80-85 |
| (23 methods on Fp2) | fn | pub | 88-193 |
| `StateVector` (shadow) | struct | pub | 199-204 |
| (7 methods on StateVector) | fn | pub | 206-254 |
| `Unitary` | struct | pub | 260-265 |
| (5 methods on Unitary) | fn | pub | 267-321 |
| `IterationData` | struct | pub | 328-339 |
| `FullAnalysis` | struct | pub | 342-380 |
| `FullGroverSuite` | struct | pub | 386-391 |
| `FullGroverSuite::new` | fn | pub | 395-402 |
| `FullGroverSuite::single` | fn | pub | 405-407 |
| `FullGroverSuite::oracle` | fn | pub | 414-424 |
| `FullGroverSuite::diffusion` | fn | pub | 427-447 |
| `FullGroverSuite::grover_operator` | fn | pub | 450-452 |
| `FullGroverSuite::theoretical_optimal` | fn | pub | 459-465 |
| `FullGroverSuite::grover_angle` | fn | pub | 468-472 |
| `FullGroverSuite::theoretical_max_probability` | fn | pub | 475-479 |
| `FullGroverSuite::run_full_analysis` | fn | pub | 486-573 |
| `FullGroverSuite::snapshot` | fn | private | 576-603 |
| `FullGroverSuite::estimate_eigenvalues` | fn | private | 606-613 |
| `FullGroverSuite::estimate_solution_count` | fn | private | 616-640 |
| `FullGroverSuite::fixed_point_search` | fn | pub | 647-668 |
| `FullGroverSuite::find_minimum` | fn | pub | 672-704 |
| `FullGroverSuite::print_full_report` | fn | pub(static) | 710-814 |
| `main` | fn | -- | 821-913 |
| 5 tests | test | -- | 916-961 |

### 3.22 Module: `noise/mod.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| Constants (5) | const | pub | 25-35 |
| `NoiseSnapshot` | struct | pub | 42-55 |
| `NoiseSnapshot::new` | fn | pub | 58-66 |
| `NoiseSnapshot::noise_bits` | fn | pub | 69-71 |
| `NoiseSnapshot::budget_bits` | fn | pub | 74-76 |
| `EMACalculator` | struct | pub | 83-93 |
| (7 methods) | fn | pub | 96-149 |
| `NoiseBudgetTracker` | struct | pub | 157-175 |
| (14 methods) | fn | pub | 178-332 |
| `NoiseWindow` | struct | pub | 339-345 |
| (4 methods) | fn | pub | 348-397 |
| `P2QuantileEstimator` | struct | pub | 416-434 |
| (12 methods) | fn | pub | 437-606 |
| `NoiseDistribution` | struct | pub | 609-623 |
| (9 methods) | fn | pub | 626-689 |
| `integer_sqrt` | fn | pub | 692-704 |
| `MultiWindowNoiseDetector` | struct | pub | 708-717 |
| (3 methods) | fn | pub | 720-762 |
| `NoiseAnomaly` | struct | pub | 765-773 |
| (2 methods) | fn | pub | 776-785 |
| 12 tests | test | -- | 792-1043 |

### 3.23 File: `compiler.rs`

| Construct | Type | Visibility | Lines |
|-----------|------|-----------|-------|
| `OpType` | enum | pub | 18-26 |
| `CircuitNode` | struct | pub | 29-36 |
| `Circuit` | struct | pub | 39-46 |
| `Circuit::new` | fn | pub | 49-57 |
| `Circuit::add_node` | fn | pub | 59-101 |
| `Circuit::operation_counts` | fn | pub | 104-111 |
| `NoiseModel` | struct | pub | 119-126 |
| `NoiseModel::conservative` | fn | pub | 129-138 |
| `NoiseModel::noise_for_op` | fn | pub | 140-151 |
| `NoiseAnalyzer` | struct | pub | 154-157 |
| `NoiseAnalyzer::new` | fn | pub | 160-166 |
| `NoiseAnalyzer::analyze` | fn | pub | 169-206 |
| `NoiseAnalysisResult` | struct | pub | 208-213 |
| `ParameterSelector` | struct | pub | 220-223 |
| (2 methods) | fn | pub | 225-290 |
| `FHEParameters` (compiler's) | struct | pub | 292-300 |
| `FHEParameters::summary` | fn | pub | 302-323 |
| `BootstrapFreeFHECompiler` | struct | pub | 329-333 |
| (2 methods) | fn | pub | 336-416 |
| `CompilationResult` | struct | pub | 418-424 |
| `example_polynomial_circuit` | fn | pub | 430-459 |
| `example_deep_circuit` | fn | pub | 461-476 |
| 3 tests | test | -- | 483-533 |

---

## 4. WIRING VERIFICATION

### 4.1 BFV Pipeline Wiring: COMPLETE

All core BFV pipeline constructs are wired end-to-end:

| Stage | Input | Output | Wired? |
|-------|-------|--------|--------|
| FHEConfig | parameters | config | YES |
| NTTEngine | q, n | engine | YES |
| ShadowHarvester | seed | rng | YES |
| KeySet::generate | config, ntt, rng | keys | YES |
| BFVEncoder::new | config | encoder | YES |
| BFVEncryptor::new | pk, encoder, ntt, eta | encryptor | YES |
| BFVEncryptor::encrypt | m, rng | Ciphertext | YES |
| BFVDecryptor::new | sk, encoder, ntt | decryptor | YES |
| BFVDecryptor::decrypt | Ciphertext | m | YES |
| BFVEvaluator::new | ntt, encoder, eval_key | evaluator | YES |
| BFVEvaluator::add/sub/negate | ct | ct | YES |
| BFVEvaluator::mul | ct1, ct2 | ct | YES |
| BFVEvaluator::relinearize | d0,d1,d2 | ct | YES |

### 4.2 AHOP Wiring: PARTIALLY CONNECTED

The AHOP module is wired into the prelude (`Fp2Element`, `StateVector`, `GroverSearch`, `GroverStats`), and the `grover_noise_search` binary uses both `ahop` and `noise` modules together. However:

- `grover_full.rs` defines its own `StateVector` and `Fp2` types that **shadow** the ones in `ahop/mod.rs`. It is NOT wired to the main AHOP module. It has its own `main()` function and acts as a standalone executable embedded in a library module file.
- `grover_full.rs` is declared via `pub mod grover;` in `ahop/mod.rs`, but `grover_full` itself is NOT declared as a submodule. It exists as a file but is **NEVER `mod`-declared**, meaning it is **dead code** that Rust never compiles.

### 4.3 Compiler Wiring: NOT CONNECTED

`compiler.rs` is present in `src/` but is **NOT declared as a module** in `lib.rs`. It has `#![forbid(unsafe_code)]` and `#![deny(clippy::float_arithmetic)]` inner attributes (which only apply to crate roots or module-level), and it contains its own `FHEParameters` struct that conflicts with `params::FHEConfig`. This file is **completely orphaned** -- Rust does not compile it.

### 4.4 Benchmark Binary Wiring

| Binary | Declared in Cargo.toml? | Compiles? |
|--------|------------------------|-----------|
| `noise_bench` | YES | YES |
| `grover_noise_search` | YES | YES |
| `fhe_benchmarks` | **NO** | **NO** (not declared as `[[bin]]`) |

---

## 5. DEAD CODE DETECTION

### 5.1 Completely Dead Files (Never Compiled)

| File | Reason |
|------|--------|
| `src/compiler.rs` | Not declared as module in `lib.rs`. Orphaned file. |
| `src/ahop/grover_full.rs` | Not declared as submodule in `ahop/mod.rs`. Orphaned file. |
| `src/bin/fhe_benchmarks.rs` | Not declared as `[[bin]]` in `Cargo.toml`. Will not compile as binary. |

### 5.2 Dead/Unused Constructs Within Live Code

| Construct | Location | Evidence |
|-----------|----------|----------|
| `BarrettContext::k` field | `barrett.rs:14` | Field stored but never read (Barrett uses fixed k=128) |
| `MontgomeryContext::r` field | `montgomery.rs:12` | Field `r: u128` stored but never read after construction |
| `NTTEngine::psi` field | `ntt.rs:18` | Stored but only `psi_powers` used |
| `NTTEngine::omega` field | `ntt.rs:19` | Stored but only `omega_powers` used |
| `NTTEngine::omega_inv` field | `ntt.rs:22` | Stored but only `omega_inv_powers` used |
| `NTTEngine::psi_inv` field | `ntt.rs:24` | Stored but only `psi_inv_powers` used |
| `NTTEngine::n_inv` field | `ntt.rs:26` | Stored, used only in `intt()` |
| `PersistentMontgomery::r_log` field | `persistent_montgomery.rs:33` | Stored but never read |
| `PersistentMontgomery::from_raw_montgomery` | `persistent_montgomery.rs:196` | Identity function, never called |
| `PersistentMontgomery::zero` | `persistent_montgomery.rs:202` | Never called from outside tests |
| `FHEConfig::large_single` | `params/mod.rs:54` | Config defined but never used in any test or pipeline |
| `RNSPolynomial::drop_last_prime` | `rns.rs:236` | Defined but never called |
| `RingPolynomial::rounded_scalar_div` | `polynomial.rs:90` | Defined but never called |
| `RingPolynomial::degree` | `polynomial.rs:42` | Defined but never called |
| `BFVEncoder::encode_vector` | `encrypt.rs:88` | Defined but never called from production code |
| `BFVEncoder::decode_vector` | `encrypt.rs:101` | Only called from `decrypt_vector` (untested) |
| `BFVDecryptor::decrypt_vector` | `encrypt.rs:210` | Defined but never called from production code |
| `StateVector::prime` | `ahop/mod.rs:198` | Defined but never called |
| `StateVector::dimension` | `ahop/mod.rs:203` | Defined but never called |
| `StateVector::sub` | `ahop/mod.rs:247` | Defined but never called |
| `StateVector::apply_single_gate` | `ahop/mod.rs:286` | Defined but never called outside tests |
| `ExactFHEContext::to_ring_poly` | `ct_mul_exact.rs:137` | Defined but never called |
| `ExactFHEContext::relinearize_simple` | `ct_mul_exact.rs:408` | Defined but never called |
| `RNSContext::mont_contexts` field | `rns.rs:16` | Stored but never read (NTT engines used instead) |
| `ExactFHEContext::anchor_psi` field | `ct_mul_exact.rs:47` | Stored but never directly read |
| `ExactFHEContext::anchor_psi_inv` field | `ct_mul_exact.rs:48` | Stored but never directly read |
| `ExactFHEContext::anchor_omega_inv` field | `ct_mul_exact.rs:43` | Stored but never directly read |
| `ExactFHEContext::anchor_omega` field | `ct_mul_exact.rs:42` | Stored but never directly read |
| `ExactDivider::is_valid_range` | `exact_divider.rs:135` | Defined but never called |
| `ExactDivider::divmod` | `exact_divider.rs:112` | Defined but never called |
| `KElimination::exact_divide_checked` | `k_elimination.rs:103` | Defined but never called |
| `BarrettContext::pow` | `barrett.rs:134` | Never called from outside tests |

### 5.3 Prefixed-Underscore Variables (Intentionally Suppressed Warnings)

| Variable | Location | Context |
|----------|----------|---------|
| `_r_mod_q` | `montgomery.rs:26` | Computed then discarded |
| `_n` | `ct_mul_exact.rs:149` | Unused variable in `poly_mul` |
| `_q` | `ct_mul_exact.rs:295` | Unused variable in `ntt_inverse` |
| `_n` | `ct_mul_exact.rs:232` | Unused variable in `ntt_forward_anchor` |
| `_t` | `params/mod.rs:60` | Unused variable in `large_single` |
| `_q_product` | `rns_mul.rs:91` | Unused variable in `modulus_switch` |
| `_s`, `_s2` | `ct_mul_exact.rs:363` | Parameters in `exact_rescale` signature but never used |
| `_num_qubits` | `ahop/mod.rs:286` | Unused parameter in `apply_single_gate` |

---

## 6. CROSS-REFERENCE CHECK

### 6.1 Module Import Alignment

| Module | Imports From | Alignment |
|--------|-------------|-----------|
| `ring/polynomial.rs` | `arithmetic::NTTEngine`, `entropy::ShadowHarvester` | CORRECT |
| `keys/mod.rs` | `arithmetic::NTTEngine`, `entropy::ShadowHarvester`, `params::FHEConfig`, `ring::RingPolynomial` | CORRECT |
| `ops/encrypt.rs` | `arithmetic::NTTEngine`, `entropy::ShadowHarvester`, `keys::{PublicKey,SecretKey}`, `params::FHEConfig`, `ring::RingPolynomial` | CORRECT |
| `ops/homomorphic.rs` | `arithmetic::{NTTEngine,KElimination}`, `keys::EvaluationKey`, `ops::encrypt::{BFVEncoder,Ciphertext}`, `ring::RingPolynomial` | CORRECT |
| `ops/rns_mul.rs` | `arithmetic::{NTTEngine,RNSContext,RNSPolynomial,KElimination}`, `ring::RingPolynomial`, `ops::Ciphertext`, `params::FHEConfig` | CORRECT |
| `ct_mul_exact.rs` | `exact_coeff::*`, `arithmetic::NTTEngine`, `ring::RingPolynomial` | CORRECT |

### 6.2 Re-export Consistency

All items re-exported in the prelude are verified to exist in their source modules. No phantom re-exports detected.

### 6.3 Duplicate Function Definitions

The function `mod_inverse` is independently defined in **5 separate files**:
1. `arithmetic/ntt.rs` (line 220)
2. `arithmetic/rns.rs` (line 245)
3. `arithmetic/exact_divider.rs` (line 146)
4. `arithmetic/ct_mul_exact.rs` (line 421)
5. `params/primes.rs` (line 94) -- **pub** version

The function `mod_pow` is independently defined in **5 separate files**:
1. `arithmetic/ntt.rs` (line 204)
2. `arithmetic/ct_mul_exact.rs` (line 434)
3. `ahop/mod.rs` (line 308)
4. `ahop/grover.rs` (line 135)
5. `params/primes.rs` (line 101) -- **pub** version

The function `gcd` is independently defined in **2 files**:
1. `arithmetic/exact_divider.rs` (line 141)
2. `params/primes.rs` (line 79) -- **pub** version

All private copies are functionally identical but represent maintenance debt.

---

## 7. ANOMALY CATALOGUE

### 7.1 Float Violations

| File | Line | Construct | Severity | Context |
|------|------|-----------|----------|---------|
| `params/mod.rs` | 257 | `n as f64 / log_q as f64` | MEDIUM | `estimate_security()` uses f64 division |
| `params/production.rs` | 63 | `sigma: f64` | LOW | `ProductionConfig128.sigma` field is f64 (conceptual, never used in computation) |
| `params/production.rs` | 154 | `(n as f64 / 37.5) as usize` | MEDIUM | Interpolation in `estimated_security()` |
| `ahop/mod.rs` | 194 | `(self.dim as f64).log2() as usize` | MEDIUM | `num_qubits()` uses f64 log2 |
| `ahop/mod.rs` | 282 | `weight_k / total` | HIGH | `probability()` returns f64 using float division |
| `ahop/grover.rs` | 82 | `PI / 4.0 * n.sqrt()` | HIGH | `optimal_iterations()` uses f64 sqrt and PI |
| `ahop/grover.rs` | 95,114-122 | `f64` in GroverStats and max_non_target_prob | MEDIUM | Stats struct uses f64 for probabilities |
| `ahop/grover_full.rs` | 29 | `use std::f64::consts::PI` | HIGH | Full Grover uses PI extensively |
| `ahop/grover_full.rs` | 182-192 | `to_complex_f64`, `phase` | MEDIUM | Display-only conversions |
| `ahop/grover_full.rs` | 233-253 | `probabilities`, `phases` | HIGH | StateVector methods use f64 |
| `ahop/grover_full.rs` | 462-479 | Theoretical calculations | HIGH | All theoretical predictions use f64 |
| `ahop/grover_full.rs` | 584-586 | `entropy` calculation | HIGH | Shannon entropy uses f64 ln() |
| `noise/mod.rs` | 69-70 | `noise_bits()` | LOW | Display-only f64 conversion |
| `noise/mod.rs` | 74-76 | `budget_bits()` | LOW | Display-only f64 conversion |
| `noise/mod.rs` | 372 | `speedup` calculation | MEDIUM | `persistent_montgomery.rs` test uses f64 |
| `noise/mod.rs` | 417 | `p: f64` | MEDIUM | P2QuantileEstimator stores quantile as f64 |
| `noise/mod.rs` | 429 | `n_prime: [f64; 5]`, `dn: [f64; 5]` | HIGH | P2 internal state uses f64 arrays |
| `noise/mod.rs` | 545-576 | Parabolic/linear interpolation | HIGH | P2 adjustment calculations use f64 |
| `compiler.rs` | 120-151 | `NoiseModel` fields | HIGH | All noise model values are f64 |
| `compiler.rs` | 181-201 | `NoiseAnalyzer::analyze` | HIGH | All noise analysis uses f64 |
| `compiler.rs` | 238-243 | Required bits calculation | HIGH | Uses f64 ceiling |
| `compiler.rs` | 380-414 | Verification and speedup | HIGH | Uses f64 comparisons |
| `bin/fhe_benchmarks.rs` | 31-58 | `BenchResult` | HIGH | All statistical measures in f64 |
| `benches/noise_bench.rs` | 87-110 | Format functions | MEDIUM | Rate/latency formatting uses f64 |
| `benches/grover_noise_search.rs` | 91-92,119 | Optimal iterations, speedup | HIGH | Calculation uses f64 |

**Summary**: 30+ float violation points. The core BFV arithmetic pipeline (`montgomery`, `barrett`, `ntt`, `rns`, `k_elimination`, `exact_divider`, `exact_coeff`, `ct_mul_exact`, `shadow`, `keys`, `encrypt`, `ring/polynomial`) is **float-free** in computation paths. Float violations concentrate in:
- Display/reporting code (LOW severity)
- AHOP/Grover simulation analysis (HIGH but contained)
- Noise module P2 estimator internals (HIGH -- uses f64 for quantile parameter and parabolic interpolation)
- Compiler (HIGH but dead code -- never compiled)
- Benchmark binaries (acceptable for measurement)

### 7.2 Unsafe Code

| File | Line | Context |
|------|------|---------|
| `benches/noise_bench.rs` | 18 | `unsafe { std::ptr::read_volatile(&x) }` in `black_box()` |
| `compiler.rs` | 7 | `#![forbid(unsafe_code)]` (declares intent but file is dead) |

**Only 1 instance** of actual unsafe code, in a benchmark binary's optimization barrier. Core library has **zero unsafe**.

### 7.3 Unwrap/Panic Points

| File | Line | Construct | Risk |
|------|------|-----------|------|
| `ntt.rs` | 91 | `panic!("No primitive root found")` | MEDIUM -- could fail for bad prime |
| `k_elimination.rs` | 51 | `.expect("alpha_cap and beta_cap must be coprime")` | LOW -- validated by construction |
| `exact_divider.rs` | 106-107 | `assert!(x % d == 0, ...)` | LOW -- precondition |
| `primes.rs` | 96 | `assert_eq!(g, 1, "No inverse exists")` | MEDIUM -- no graceful fallback |
| `primes.rs` | 135 | `panic!("No primitive root found")` | MEDIUM -- could fail for bad prime |
| `homomorphic.rs` | 123 | `.expect("Evaluation key required")` | MEDIUM -- runtime crash if no eval key |
| `ahop/grover_full.rs` | 431 | `.expect("N invertible")` | LOW -- construction guarantees |
| `ring/polynomial.rs` | 124 | `.unwrap_or(0)` | SAFE |
| `ops/encrypt.rs` | 38 | `assert!(m < self.t, ...)` | LOW -- precondition check |

### 7.4 Hardcoded Magic Numbers

| File | Line | Value | Purpose |
|------|------|-------|---------|
| `shadow.rs` | 22 | `0xDEADBEEF_CAFEBABE` | Default seed |
| `shadow.rs` | 31 | `0x9E3779B97F4A7C15` | Golden ratio mixing constant |
| `shadow.rs` | 49 | `0xFF51AFD7ED558CCD` | MurmurHash3 constant |
| `shadow.rs` | 51 | `0xC4CEB9FE1A85EC53` | MurmurHash3 constant |
| `montgomery.rs` | 50 | `6` (iterations) | Newton convergence iterations for 64-bit |
| `persistent_montgomery.rs` | 59 | `6` (iterations) | Same |
| `barrett.rs` | 22 | `128` (k value) | Barrett reduction bit width |
| `k_elimination.rs` | 66-67 | `65537, 65521, 65519, 65497, 65479` | Alpha/beta primes for FHE |
| `exact_divider.rs` | 57-60 | `1073479681, 1073217537, 1072627713, 469762049` | Coprime anchor candidates |
| `keys/mod.rs` | 79 | `1u64 << 16` | Decomposition base T=65536 |
| `params/mod.rs` | 44 | `998244353` | Primary NTT-friendly prime |
| `params/mod.rs` | 45 | `2053` | Plaintext modulus for light config |
| `params/mod.rs` | 59 | `1152921504606846593` | 60-bit NTT prime |
| `params/mod.rs` | 80 | `1099511627777` | ~2^40 plaintext modulus |
| `params/mod.rs` | 96 | `500000` | Large t for small delta |
| `noise/mod.rs` | 25 | `1_000_000` | FixedQ scale factor |
| `noise/mod.rs` | 28 | `1000` | Millibits per bit |
| `noise/mod.rs` | 32 | `500_000` | Max noise budget |
| `noise/mod.rs` | 35 | `3200` | Initial noise millibits |
| `lib.rs` | 175 | `0xDEAD_BEEF` | Test seed in production test |
| `benches/grover_noise_search.rs` | 73 | `65537` | Prime for Grover simulation |
| `compiler.rs` | 131 | `2.0, 25.0, 15.0, 60.0, 5.0, 1.3` | Noise model parameters |

### 7.5 Test Configuration Anomalies

| Test | File | Issue |
|------|------|-------|
| `test_production_128bit` | `lib.rs:149` | `#[ignore]` -- needs noise tracking for N=8192 |
| `test_exact_poly_mul_constant` | `ct_mul_exact.rs:454` | `#[ignore]` -- anchor NTT primitive root issue |
| `test_rns_mul_with_relin` | `rns_mul.rs:335` | `#[ignore]` -- RNS needs proper CT generation |
| `test_rns_mul_multiple_values` | `rns_mul.rs:358` | `#[ignore]` -- same as above |
| `test_cbd_mean` | `shadow.rs:200` | Uses `f64` for mean calculation in test assertion |
| `test_grover_integration` | `lib.rs:140` | Uses `f64` for probability in assertion: `println!("...{:.4}", prob)` |

### 7.6 Type Shadowing

| Type | Original | Shadow | Issue |
|------|----------|--------|-------|
| `StateVector` | `ahop/mod.rs:148` | `ahop/grover_full.rs:199` | Different fields: `amplitudes` vs `components` |
| `Fp2Element` / `Fp2` | `ahop/mod.rs:15` | `ahop/grover_full.rs:80` | Same semantics, different names |

The `grover_full.rs` file is dead code (not declared as module), so shadowing is currently harmless. If ever integrated, it would cause compilation conflicts.

### 7.7 Algorithm Correctness Notes

1. **NTT Implementation**: Uses O(N^2) DFT matrix approach, not O(N log N) butterfly FFT. Correct but slow for large N. This is noted as "Gen 3" but is actually the naive approach.

2. **KElimination::scale_and_round**: Line 136 applies `% q` after division, which may truncate valid results for values exceeding q.

3. **ExactFHEContext::compute_anchor_roots**: Lines 93-97 return dummy values (all 1s) when anchor modulus A doesn't support 2N-th roots. This silently produces incorrect anchor NTT results for those moduli, which is why `test_exact_poly_mul_constant` is `#[ignore]`d.

4. **P2QuantileEstimator**: Uses f64 internally for parabolic interpolation (lines 545-576). While the interface accepts/returns i64 millibits, the internal P2 algorithm computations use floating-point, potentially breaking cross-platform determinism.

5. **RNSEvaluator::modulus_switch**: Reconstructs full CRT value then scales by t/q. For large moduli products, the u128 CRT reconstruction may overflow silently (product > 2^128).

---

## 8. SUMMARY STATISTICS

| Metric | Count |
|--------|-------|
| Total source files | 22 (+ 3 dead files) |
| Total modules (live) | 8 top-level, 17 submodules |
| Total structs | ~45 |
| Total public functions | ~250 |
| Total test functions | ~120 |
| Tests marked `#[ignore]` | 4 |
| Dead files (never compiled) | 3 (`compiler.rs`, `grover_full.rs`, `fhe_benchmarks.rs`) |
| External dependencies | 0 |
| Unsafe blocks | 1 (benchmark only) |
| Float violations (core pipeline) | 0 |
| Float violations (total) | 30+ |
| Duplicate function copies | `mod_inverse`(5x), `mod_pow`(5x), `gcd`(2x) |
| Hardcoded magic numbers | 25+ |
| Panic/unwrap points | 9 |
| Dead constructs (unused public API) | 25+ |

---

## 9. LINEAGE ASSESSMENT

This build (`08_FHE_v01_original`) represents the **base BFV FHE implementation** with the following architectural characteristics:

1. **Self-contained**: Zero external dependencies. All arithmetic, entropy, and cryptographic primitives built from scratch.

2. **Core BFV pipeline is float-free**: The encrypt-evaluate-decrypt path uses only integer arithmetic with exact operations.

3. **Multiple multiplication strategies coexist**: Standard BFV relinearization, degree-2 decrypt, RNS-based multiplication, and exact dual-track (K-Elimination) multiplication are all present as parallel approaches at different maturity levels.

4. **AHOP is a separate subsystem**: Quantum simulation over F_{p^2} is functional but uses f64 for analysis/reporting. The core F_{p^2} arithmetic itself is integer-only.

5. **Noise tracking is comprehensive but partially float-contaminated**: The P2 quantile estimator and noise model both use f64 internally, breaking the integer-only guarantee.

6. **Significant dead code**: Three entire files never compile. 25+ public functions are never called outside tests. The compiler module represents a future direction never integrated.

7. **Code duplication**: `mod_inverse` and `mod_pow` are copy-pasted across 5 files each, creating maintenance risk.

8. **NTT is O(N^2)**: The "Gen 3" NTT uses naive DFT matrix multiplication, not butterfly FFT. This is a performance limitation for production-scale N (4096+).

**VERDICT**: The build is a functional BFV FHE prototype with clean core arithmetic but accumulated dead code, significant code duplication, and float contamination in peripheral subsystems. It serves as the foundation for the FHE lineage but requires cleanup before production use.
