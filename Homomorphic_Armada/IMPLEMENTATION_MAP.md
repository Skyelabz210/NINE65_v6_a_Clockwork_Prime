# Homomorphic Armada — Implementation Map

Navigable cross-reference of all builds: crate names, public API surfaces, key innovations, benchmark entry points, and the feature matrix.

## Per-Build API Surface

### 08_FHE_v01_original — Zero-Dep BFV Core

| Attribute | Value |
|-----------|-------|
| Package | `qmnf_fhe` (v0.1.0-v01) |
| Path | `builds/08_FHE_v01_original/qmnf_fhe_production/` |
| Dependencies | None |
| Harness feature | `v01_original` |

**Public Modules**: `arithmetic`, `entropy`, `params`, `ring`, `keys`, `ops`, `ahop`, `noise`

**Key Types**:
- `FHEConfig::light()` / `FHEConfig::custom(n, primes, t, eta)`
- `NTTEngine::new(q, n)` — Negacyclic convolution
- `ShadowHarvester::with_seed(u64)` — Deterministic RNG
- `KeySet::generate(&config, &ntt, &mut rng)` → `{public_key, secret_key, eval_key}`
- `BFVEncoder::new(&config)` — Plaintext encoding
- `BFVEncryptor::new(&pk, &encoder, &ntt, eta)` → `.encrypt(value, &mut rng)`
- `BFVDecryptor::new(&sk, &encoder, &ntt)` → `.decrypt(&ct)`
- `BFVEvaluator::new(&ntt, &encoder, Some(&ek))` → `.add()`, `.sub()`, `.mul()`, `.negate()`, `.add_plain()`, `.mul_plain()`
- `MontgomeryContext` / `BarrettContext` — Modular arithmetic
- `GroverSearch::new(qubits, target, modulus)` — Quantum simulation

**Innovations**: K-Elimination exact division, Shadow Entropy, Montgomery Gen 2

---

### 08_FHE_v02_stable — Production Security

| Attribute | Value |
|-----------|-------|
| Package | `qmnf_fhe` (v0.1.0-v02) |
| Path | `builds/08_FHE_v02_stable/qmnf_fhe_production/` |
| Dependencies | zeroize, getrandom, subtle, sha2 |
| Harness feature | `v02_stable` |

**Additions over v01**:
- `KeySet::generate_secure(&config, &ntt)` — OS CSPRNG keygen
- `security` module — `LWEParams`, `SecurityEstimate`, `ConfidenceLevel`
- `kat` module — Known Answer Tests
- `PersistentMontgomery` — Stay in Montgomery form (70x fewer conversions)
- `RNSContext` / `RNSPolynomial` — Residue Number System support

**Backward compatible**: All v01 code works unchanged.

---

### 08_FHE_v03_MANA_boosted — Parallel CRT Fork (INCOMPATIBLE)

| Attribute | Value |
|-----------|-------|
| Package | `mana` + `nine65` + `unhal` (workspace) |
| Path | `builds/08_FHE_v03_MANA_boosted/crates/mana/` |
| Dependencies | rayon, zeroize |
| Harness feature | `v03_mana` |

**INCOMPATIBLE** with v01/v02/v04 — completely different data model.

**Key Types (MANA crate)**:
- `Lane::new(prime, size)` / `Lane::from_int_slice(values, prime)`
- `ManaStream::new(primes, n)` / `ManaStream::from_ints(values, primes)`
- `LaneOps` trait: `.add()`, `.sub()`, `.neg()`, `.mul()`, `.scalar_mul()`, `.scalar_add()`
- `StreamOps` trait: `.add()`, `.sub()`, `.mul()`, `.scalar_mul()`
- `KAnchor` / `AnchorContext` — K-Elimination anchor codex
- `GsoSwarm` / `QbitAgent` — Glowworm Swarm Optimization
- `ParallelStream` (feature: `parallel`) — Rayon-based lane execution

**Innovations**: CRT-lane parallelism (2.78x Rayon speedup), branchless modular arithmetic, zero carry propagation

---

### 08_FHE_v04_QClassic — Quantum + Neural + Signed Arithmetic

| Attribute | Value |
|-----------|-------|
| Package | `qmnf_fhe` (v0.1.0-v04) |
| Path | `builds/08_FHE_v04_QClassic/nine65_archive/source_code/` |
| Dependencies | zeroize, getrandom, subtle, sha2, rayon |
| Harness feature | `v04_qclassic` |

**Additions over v02**:
- `quantum` module: `EntangledPair`, `GHZState`, `QuantumAmplitude`, `grover_search()`
- `neural` module: `FHENeuralEvaluator`, `NeuralNetwork`, `ActivationType` (ReLU, Sigmoid, Tanh, Softmax, GELU)
- `MobiusInt` / `MobiusPolynomial` — Signed integer arithmetic via Mobius encoding
- `PadeEngine` / `PADE_SCALE` — Pade [4/4] transcendentals (~200ns)
- `MQReLU` — O(1) sign detection via quadratic residues
- `IntegerSoftmax` — Exact sum softmax
- `CyclotomicRing` — Native ring trig (~50ns sin/cos)
- `WassanNoiseField` — V2 holographic noise
- `NTTEngineFFT` — Alternative FFT-based NTT (feature: `ntt_fft`)

**Backward compatible**: All v02 code works unchanged.

---

### 09_NINE65_v5_live — Latest (9-Crate Workspace)

| Attribute | Value |
|-----------|-------|
| Package | `nine65` v0.1.0 (workspace) |
| Path | `builds/09_NINE65_v5_live/crates/nine65/` |
| Crates | nine65, nexgen_rational, exact_transcendentals, mana, unhal, clockwork-core, fhe-service |
| Harness feature | `v5_live` |

**Additions over v04**:
- `DualRNSCiphertext` / `RNSFHEContext` — DualRNS with K-Elimination multiplication
- `GaloisEngine` / `GaloisEvaluator` — Galois automorphisms (rotation, conjugation)
- `BatchEncoder` — SIMD slot encoding
- `ParallelEncryptor` / `ParallelDecryptor` — Rayon-parallel encryption
- `compiler` module — Bootstrap-free FHE circuit compiler
- `TrackedEvaluator` — Built-in noise tracking evaluator
- `clockwork-core` integration (feature: `clockwork`) — Formal RNS with triple redundancy
- `nexgen_rational` integration (feature: `exact_rational`) — Binary GCD rational arithmetic
- `exact_transcendentals` backend (feature: `exact_transcendentals_backend`)

**14 feature flags**: `ntt_fft`, `parallel`, `accelerated`, `secure-keygen`, `wassan`, `v2`, `allow_insecure`, `deterministic_rng`, `exact_rational`, `clockwork`, `exact_transcendentals_backend`, `serde`, `slow_tests`, `benchmarks`

---

### 12_exact_trans_live — Exact Transcendentals (Standalone)

| Attribute | Value |
|-----------|-------|
| Package | `exact_transcendentals` v1.0.0 |
| Path | `builds/12_exact_trans_live/` |
| Dependencies | None (no_std compatible) |
| Harness feature | `exact_trans` |

**Algorithms**:
- `CordicEngine::sin(x, scale)`, `cos()`, `tan()`, `atan()` — CORDIC (~30-bit, 1 bit/iteration)
- `HyperbolicCordic` — `sinh()`, `cosh()`, `exp()`, `log()`
- `AgmEngine::ln(x, scale)`, `pi(scale)` — AGM (quadratic convergence)
- `binary_splitting::exp_bs()`, `sin_bs()`, `cos_bs()`, `pi_bs()` — Hypergeometric series
- `sqrt::isqrt(x)` — Newton-Raphson integer sqrt
- `continued_fraction` module — Best rational approximations

**Scale factors**: `SCALE_30` (2^30), `SCALE_62` (2^62), `SCALE_DECIMAL` (10^9), `SCALE_DECIMAL_18` (10^18)

---

## Innovation Cross-Reference

| Innovation | v01 | v02 | v03 | v04 | v5 | exact_trans |
|-----------|-----|-----|-----|-----|-----|-------------|
| K-Elimination | x | x | x | x | x | |
| Shadow Entropy | x | x | x | x | x | |
| OS CSPRNG keygen | | x | | x | x | |
| LWE security estimation | | x | | x | x | |
| KAT tests | | x | | x | x | |
| Montgomery Gen 2 | x | x | | x | x | |
| PersistentMontgomery | | x | | x | x | |
| NTT FFT variant | | | x | x | x | |
| RNS channels | | x | x | x | x | |
| CRT-lane parallelism | | | x | | x | |
| MANA streaming | | | x | | x | |
| WASSAN noise field | | | x | x | x | |
| MobiusInt (signed) | | | | x | x | |
| Pade transcendentals | | | | x | x | |
| CyclotomicRing | | | | x | x | |
| MQReLU (O(1) sign) | | | | x | x | |
| Integer softmax | | | | x | x | |
| Neural evaluator | | | | x | x | |
| Quantum simulation | x | x | | x | x | |
| DualRNS ciphertexts | | | | | x | |
| Galois automorphisms | | | | | x | |
| BatchEncoder (SIMD) | | | | | x | |
| Parallel encrypt/decrypt | | | | | x | |
| Bootstrap-free compiler | | | | | x | |
| Clockwork formal RNS | | | | | x | |
| NexGen rational | | | | | x | |
| CORDIC | | | | | | x |
| AGM | | | | | | x |
| Binary splitting | | | | | | x |
| Continued fractions | | | | | | x |
| Integer sqrt | | | | | | x |

## Non-Rust Builds

| # | Directory | Content | Language |
|---|-----------|---------|----------|
| 05 | `05_proofstack_20260211` | Proof artifacts and formal specifications | Mixed |
| 07 | `07_security_proofs` | Lean4/Coq security proofs | Lean4, Coq |
| 18 | `18_hackfate` | HackFate research tooling | Python, Rust |
| 19 | `19_k_elimination_lean4` | K-Elimination formal proofs | Lean4 |
| 20 | `20_Loki5` | Loki5 cryptographic framework | Rust |
| 21 | `21_ENHANCE` | Enhancement suite | Mixed |
| 22 | `22_redteam_mcp` | Security testing tooling | Python |

## Benchmark Entry Points

| Build | Entry Point | Command |
|-------|------------|---------|
| v01 | armada-bench/cross_build | `cargo bench -p armada-bench --no-default-features --features v01_original` |
| v02 | armada-bench/cross_build | `cargo bench -p armada-bench --no-default-features --features v02_stable` |
| v03 (MANA) | armada-bench/cross_build | `cargo bench -p armada-bench --no-default-features --features v03_mana` |
| v04 | armada-bench/cross_build | `cargo bench -p armada-bench --no-default-features --features v04_qclassic` |
| v5 | armada-bench/cross_build | `cargo bench -p armada-bench --features v5_live` |
| Transcendentals | armada-bench/cross_build | `cargo bench -p armada-bench --no-default-features --features exact_trans` |
| All | scripts/bench_all.sh | `./scripts/bench_all.sh` |
| Forensic audit | armada-bench/tracked_audit | `cargo bench -p armada-bench --bench tracked_audit --features v5_live` |

## Forensic Metric Collection

The `TrackedFhe<T>` wrapper (`armada-shim/src/metrics.rs`) provides:

- **Timing**: Per-operation nanosecond wall-clock with P50/P95/P99 percentiles
- **Correctness validation**: Configurable encrypt→decrypt roundtrip checks
- **Drift detection**: Flags when latency exceeds N× baseline mean
- **Anomaly collection**: Full forensic context (severity, expected vs actual, sequence number)
- **Export**: CSV, JSON lines, anomaly-specific CSV for offline analysis
- **Forensic sweep**: `tracked.forensic_sweep(N)` runs a full validation pipeline
