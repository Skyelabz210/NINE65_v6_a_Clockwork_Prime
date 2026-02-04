# FHE Hat: New Scheme Checklist

Use this checklist when implementing a new FHE scheme with QMNF innovations.

---

## Scheme: _________________

**Base Scheme:** ☐ BFV  ☐ BGV  ☐ CKKS  ☐ TFHE  ☐ FHEW  ☐ Other: ________

**Ring:** R_q = Z_q[X] / (X^N + 1)

**Parameters:**
- N (ring dimension): _______
- q (ciphertext modulus): _______ bits
- t (plaintext modulus): _______
- Security level: _______ bits

---

## Phase 1: Initialize Hat Components

### 1.1 Persistent Montgomery
```rust
let moduli: Vec<u64> = vec![/* your primes */];
let pm = PersistentMontgomery::new(&moduli);
```

- [ ] All moduli listed
- [ ] PM initialized once at setup
- [ ] PM stored in context/state

### 1.2 K-Elimination
```rust
let alpha_moduli: Vec<u64> = vec![/* main primes */];
let beta_moduli: Vec<u64> = vec![/* anchor primes, coprime to alpha */];
let ke = KElimination::new(&alpha_moduli, &beta_moduli);
```

- [ ] Alpha moduli selected (main computation)
- [ ] Beta moduli selected (anchors, coprime to alpha)
- [ ] gcd(alpha_cap, beta_cap) == 1 verified
- [ ] KE initialized and stored

### 1.3 NTT Gen3
```rust
let ntt = NTTContext::new(n, modulus);
// Verify primitive roots
assert!(ntt.verify_roots());
```

- [ ] N is power of 2
- [ ] Each modulus q ≡ 1 (mod 2N)
- [ ] Primitive N-th roots computed
- [ ] ψ (2N-th root) computed for negacyclic

### 1.4 Shadow Entropy
```rust
let entropy = ShadowEntropy::from_seed(secure_seed);
```

- [ ] Seed source documented
- [ ] Entropy passes statistical tests
- [ ] Used for ALL noise generation

### 1.5 Integer Noise Tracking
```rust
let noise = NoiseBudget::new(initial_millibits);
```

- [ ] Initial noise budget calculated
- [ ] Noise growth formulas implemented (add, mul, relin)
- [ ] No floats in noise tracking

---

## Phase 2: Wire Innovations into Operations

### 2.1 Key Generation

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| Sample secret key | CSPRNG | Shadow Entropy | ☐ |
| Sample noise | CSPRNG | Shadow Entropy | ☐ |
| Compute pk = (a, a·s + e) | Standard | PM + NTT | ☐ |
| Generate relin keys | Standard | PM + NTT + SE | ☐ |

### 2.2 Encryption

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| Sample u, e₀, e₁ | CSPRNG | Shadow Entropy | ☐ |
| Compute Δ·m | Integer | Integer (exact) | ☐ |
| Polynomial multiply | Standard NTT | NTT Gen3 | ☐ |
| Modular multiply | Standard Montgomery | Persistent Montgomery | ☐ |

### 2.3 Decryption

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| c₀ + c₁·s | Standard NTT | NTT Gen3 | ☐ |
| Round(result / Δ) | Float approx | K-Elimination exact | ☐ |

### 2.4 Homomorphic Addition

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| ct₁.c₀ + ct₂.c₀ | Coefficient add | Coefficient add | ☐ |
| Noise update | Float estimate | Integer millibits | ☐ |

### 2.5 Homomorphic Multiplication

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| Tensor product (3 NTTs) | Standard NTT | NTT Gen3 + PM | ☐ |
| Rescale Δ² → Δ | Float division | K-Elimination | ☐ |
| Relinearize | Standard | PM + NTT Gen3 | ☐ |
| Noise update | Float estimate | Integer millibits | ☐ |

### 2.6 Rescaling / Modulus Switching

| Operation | Standard | QMNF Innovation | Wired? |
|-----------|----------|-----------------|--------|
| Divide by prime | Float (99.9998%) | K-Elimination (100%) | ☐ |
| Drop prime from RNS | Standard | CRTBigInt | ☐ |

---

## Phase 3: Verify Integer-Only

Run contamination scan:
```bash
grep -rn "f32\|f64\|as f\|\.0f\|float" src/ | grep -v "// display"
```

- [ ] Zero results in computation path
- [ ] Any floats are display-only (documented)

List all division operations:
```bash
grep -rn " / " src/ | grep -v "K-Elimination\|exact_divide"
```

- [ ] All divisions are either exact (K-Elimination) or integer quotient (documented safe)

---

## Phase 4: Test Suite

### 4.1 Correctness Tests

- [ ] `test_encrypt_decrypt_roundtrip`
- [ ] `test_homo_add_correctness`
- [ ] `test_homo_mul_correctness` (ct × plaintext)
- [ ] `test_homo_mul_ct_ct` (ct × ct)
- [ ] `test_homo_chain` (add + mul chain)
- [ ] `test_edge_cases` (m=0, m=t-1, overflow)

### 4.2 Innovation-Specific Tests

- [ ] `test_pm_never_converts_in_hot_path`
- [ ] `test_ke_100_percent_accuracy`
- [ ] `test_ntt_negacyclic_convolution`
- [ ] `test_shadow_entropy_nist_sp800_22`
- [ ] `test_noise_tracking_integer_only`

### 4.3 Benchmark Tests

- [ ] KeyGen timing
- [ ] Encrypt timing
- [ ] Homo Add timing
- [ ] Homo Mul timing
- [ ] Decrypt timing
- [ ] Compare to baseline (OpenFHE/SEAL/Zama)

---

## Phase 5: Documentation

- [ ] API documentation (rustdoc)
- [ ] Parameter selection guide
- [ ] Security assumptions stated
- [ ] Innovation credits (QMNF Hat components used)
- [ ] Benchmark methodology documented

---

## Sign-Off

| Check | Verified By | Date |
|-------|-------------|------|
| All innovations wired | | |
| Zero float in computation | | |
| All tests passing | | |
| Benchmarks complete | | |
| Docs complete | | |

**Scheme Status:** ☐ Ready for use  ☐ Needs work: _______________
