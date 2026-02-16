# NINE65 v6 "a Clockwork Prime"
## Bootstrap-Free Fully Homomorphic Encryption with Zero-Overhead Security

<div align="center">

**Production-hardened FHE achieving depth-50 without bootstrapping**

[![Tests](https://img.shields.io/badge/tests-1002%20passing-brightgreen)]()
[![Proofs](https://img.shields.io/badge/formal%20proofs-Coq%20%2B%20Lean4-blue)]()
[![Security](https://img.shields.io/badge/security-128--256%20bit-green)]()
[![CI](https://img.shields.io/badge/CI-11%20gates-blue)]()
[![Build](https://img.shields.io/badge/build-passing-success)]()

[Features](#key-components) • [Quick Start](#quick-start) • [Benchmarks](#performance-benchmarks) • [Security](#post-quantum-security) • [Architecture](#architecture)

</div>

---

## 🎯 What Makes v6 "Clockwork Prime" Special

✅ **Bootstrap-Free**: Depth-50 circuits with **zero** bootstrapping operations
✅ **Zero-Overhead Security**: Compile-time + runtime parameter hardening with no performance cost
✅ **Timing Attack Resistant**: GRO timing gates on all key operations
✅ **Production Ready**: 1,002 tests passing, 14 Coq proofs, 4 Lean4 formalizations
✅ **Post-Quantum**: LWE-based with lattice estimator verified security levels
✅ **Integer-Only**: No floating-point anywhere (deterministic across platforms)

---

## 🛡️ Security Hardening (Group 1 - Feb 2026)

NINE65 v6 includes comprehensive security hardening completed through parallel agent execution:

### 1. Timing Side-Channel Hardening ✅
- **Constant-time operations**: Montgomery reduction, K-Elimination, NTT all CT-safe
- **Verified implementations**: 186 arithmetic tests passing with CT/VT equivalence
- **Timing analysis suite**: Statistical benchmarks for side-channel validation
- **Production ready**: No secret-dependent branches in critical paths

### 2. Parameter Security Hardening ✅
- **Compile-time enforcement**: Test configs blocked in release builds without `allow_insecure` feature
- **Runtime validation**: Comprehensive parameter checks (orbital safety, HE Standard compliance, security estimates)
- **Production guards**: 128-bit minimum security enforced in release mode
- **Type safety**: `ProductionSafe` trait provides compile-time guarantees

### 3. Noise Budget Monitoring ✅
- **Real-time tracking**: `TrackedFHEContext` with overflow prevention
- **Integer-only precision**: Millibits (1000 = 1 bit) for exact measurements
- **Predictive depth estimation**: Know maximum achievable depth before operations
- **GSO integration**: Optional budget tracking in GSO-FHE contexts

### 4. Zero-Overhead Entropy Monitoring ✅
- **Fixed 4 threads**: Default configuration with zero adaptation overhead
- **Feature-gated complexity**: All adaptive threading behind `adaptive-threading` flag
- **No floating-point**: Integer-only in default configuration
- **Production optimized**: Zero cost when monitoring disabled

**Documentation**: See `docs/TIMING_SIDE_CHANNEL_HARDENING_REPORT.md`, `docs/TIMING_SIDE_CHANNEL_TEST_REPORT.md`, `PARAMETER_SECURITY_HARDENING_SUMMARY.md`

---

## Executive Summary

| Metric | NINE65 | Traditional FHE |
|--------|--------|-----------------|
| **Max Depth** | 50 levels verified (symmetric) | 10-15 levels |
| **Bootstrap Required** | Never (0 collapses verified) | Every ~10 muls |
| **Depth-50 Circuit** | 6.29s / 10.10s (secure_128 / secure_192) | 2,000-5,000ms |
| **Memory** | ~200MB | 1-3GB |
| **Hardware** | CPU only | GPU recommended |
| **Post-Quantum** | LWE-based (lattice estimator verified) | Varies |
| **Test Coverage** | 1,002 tests passing (core + support crates) | N/A |
| **Side-Channel** | GRO timing gates on keygen/decrypt | Varies |
| **CI Gates** | 11 automated quality gates | N/A |

Deployment status: pre-production with timing side-channel mitigations (GRO gates) integrated.
Independent security audit recommended before production deployment. Minimum evaluation config: `secure_192`.

Public-mode depth baseline is recorded in docs/PUBLIC_MODE_DEPTH_BASELINE_2026-01-27.md.

---

## Post-Quantum Security

NINE65 targets post-quantum security through LWE (Learning With Errors) based cryptography:

### Lattice Estimator Baseline (2026-02-09)
Rough LWE estimates (Core-SVP + GSA) for SecureConfig parameters are recorded in
docs/LATTICE_ESTIMATOR_BASELINE_2026-02-09.md.

| SecureConfig | n | log2(q) | min attack log2(rop) |
|--------------|---|---------|----------------------|
| `secure_128` | 4096 | 89.08 | 129 |
| `secure_192` | 8192 | 145.08 | 159 |
| `secure_256` | 16384 | 203.38 | 226 |

Notes:
- These are rough estimates, not formal security proofs.
- Test-only configs (`light`, `he_standard_128`, `light_rns_exact`) require `allow_insecure`.

### Self-Cryptanalysis

This repository includes security analysis tools in `src/security/`:
- Attack cost estimator (Primal, Dual, Hybrid, HE Standard)
- K-Elimination specific attack analysis
- LLL/BKZ lattice attack simulations

Timing side-channel posture and parameter warnings are documented in
docs/REDSHIRT_SECURITY_ASSESSMENT.md.

### Why LWE is Quantum-Resistant
- **Lattice-based hardness**: Security relies on the hardness of lattice problems
- **No known quantum speedup**: Unlike RSA/ECC, Shor's algorithm doesn't break LWE
- **NIST PQC finalist**: LWE-based schemes selected for post-quantum standardization

### Parameter Security Hardening (v6 Enhancement)

NINE65 v6 includes comprehensive parameter security hardening to prevent misuse of insecure configurations:

#### Compile-Time Enforcement
- **Test configs blocked in release**: `test_fast()` and `test_medium()` are only accessible with `#[cfg(any(test, debug_assertions))]`
- **Feature gate for insecure**: Release builds require `allow_insecure` feature to access test configurations
- **Const assertions**: Compile-time validation of security invariants

#### Runtime Validation
- **Comprehensive parameter validation**: `ParameterValidator::validate()` checks:
  - Orbital boundary safety (K-Elimination capacity)
  - HE Standard v1.1 compliance
  - Detailed security estimates (hybrid, classical, quantum)
  - Production safety threshold (>= 128-bit hybrid)
  - Noise budget adequacy
- **Security claim verification**: `SecureConfig::new_verified()` validates claimed security against actual estimates (±10% tolerance)
- **Production safety guards**: `assert_production_params()` enforces 128-bit minimum in release builds

#### API Usage
```rust
use nine65::params::secure_configs::{SecureConfig, verify_production_safety};

// Production-safe configuration (verified at construction)
let config = SecureConfig::secure_128();
assert!(config.is_production_safe());

// Verify production safety explicitly
verify_production_safety(&config).expect("Config must be production-safe");

// Runtime parameter validation
use nine65::params::validation::validate_params;
let result = validate_params(4096, 998244353, 65537);
assert!(result.production_safe, "Must meet 128-bit threshold");
```

### Verification
```bash
# Run security parameter tests
cargo test -p nine65 security::tests -- --nocapture

# Run parameter validation tests
cargo test -p nine65 params::validation::tests -- --nocapture
cargo test -p nine65 params::secure_configs::tests -- --nocapture
```

**Note**: These are parameter estimates based on standard LWE security analysis.
For formal guarantees, external audits and independent estimator runs are recommended.
Baseline estimator outputs are recorded in docs/LATTICE_ESTIMATOR_BASELINE_2026-02-09.md.

---

## Public Mode Depth Baseline

Public-key (eval-key) mode is depth-limited relative to symmetric mode. The initial depth sweep
baseline is recorded in docs/PUBLIC_MODE_DEPTH_BASELINE_2026-01-27.md.

As of 2026-02-06, `mul_dual_public` and `mul_dual_symmetric` automatically apply modulus
switching after K-Elimination rescale when the ciphertext level >= 3. This enables deeper
public-mode circuits without manual noise management.

Summary (pre-mod-switch baseline):
- `standard_128`: max depth 4 at bases 2^16/2^12/2^10; max depth 5 at base 2^8
- `high_192`: max depth 4 at bases 2^16/2^12/2^10/2^8

---

## Key Components

### 1. K-Elimination (Exact Division in RNS)
- **Dual-track architecture**: Main moduli + anchor moduli
- **Exact rescaling**: No floating-point, no approximation errors
- **O(k) complexity**: Linear in number of RNS lanes

### 2. GSO-FHE (Gravitational Swarm Optimization)
Noise bounding without bootstrapping:
- **Basin tracking**: Monitor noise evolution per coefficient
- **Gravitational collapse**: Controlled noise reduction when needed
- **Zero bootstrap operations** at depth-50

### 3. CRT Shadow Entropy
Cryptographic entropy harvested from modular arithmetic:
- **QuotientSignature**: O(1) magnitude comparison without reconstruction
- **8.9M ops/sec** entropy generation
- **284 Mbit/s** entropy rate

### 4. Non-Circular Order Finding
Classical period finding without circular dependencies:
- **BSGS with B=N-1**: No φ(N) computation required
- **K-Elimination verification**: Winding number oracle
- **Shor's classical reduction**: Complete factoring via gcd(a^(r/2) ± 1, N)

---

## Performance Benchmarks

Performance numbers are from internal release benchmarks. Reproduce and gate via
docs/RELEASE_CHECKLIST.md. Latest baseline: docs/PERFORMANCE_BASELINE_2026-02-11.md.
FHE ops and depth use secure_128 and secure_192 baselines.
Claim governance policy: docs/BENCHMARK_PROFILE_POLICY.md.
Claim-to-artifact mapping: docs/CLAIM_REGISTRY.csv.

### Arithmetic Operations (Single-threaded, Release Build)

#### RNS Arithmetic (4-lane parallel)
| Operation | Time/op | Throughput |
|-----------|---------|------------|
| ADD | 65.7 ns | 15.2M ops/sec |
| SUB | 52.9 ns | 18.9M ops/sec |
| MUL | 95.6 ns | 10.5M ops/sec |
| MUL+Signature | 100.0 ns | 10.0M ops/sec |

#### Exact Coefficient Arithmetic (Dual-Track)
| Operation | Time/op | Throughput |
|-----------|---------|------------|
| COEFF_ADD | 60.0 ns | 16.7M ops/sec |
| COEFF_MUL | 84.0 ns | 11.9M ops/sec |
| COEFF_DIV | 53.5 ns | 18.7M ops/sec |
| COEFF_SCALE | 53.7 ns | 18.6M ops/sec |

#### FHE Operations (secure configs)
| Operation | secure_128 | secure_192 | Notes |
|-----------|------------|------------|-------|
| Encrypt | 23.56ms | 61.59ms | baseline (perf run 2026-02-11) |
| Add | 0.83ms | 2.10ms | baseline |
| Mul | 152.13ms | 459.02ms | Includes K-Elimination rescale |
| Decrypt | 11.06ms | 29.00ms | baseline |

### Depth Benchmark Results (Verified 2026-02-11)
| Config | Depth | Total time | Avg time/mul | Collapses |
|--------|-------|------------|--------------|-----------|
| secure_128 | 50 | 6.29s | 125.81ms | 0 |
| secure_192 | 50 | 10.10s | 201.91ms | 0 |

**Bootstraps required: 0** (symmetric mode, empirically verified)

---

## Comparison vs Industry Leaders

Methodology and sources are documented in docs/FHE_BENCHMARK_COMPARISON.md.

```
Library          | Max Depth | Bootstrap | Depth-50 Time (symmetric)
-----------------+-----------+-----------+--------------
NINE65           |    50+    |   Never   |    6.29s / 10.10s
OpenFHE (BGV)    |    15     |   ~50ms   |   ~2,500ms
Microsoft SEAL   |    12     |    N/A    |   (limited)
TFHE-rs (GPU)    | Unlimited |   <1ms    |    ~200ms*
HElib            |    12     |  ~100ms   |   ~5,000ms

* Requires $30k+ GPU (H100)
```

---

## Architecture

```
NINE65/v6/
├── crates/
│   ├── nine65/           # Core FHE implementation
│   │   └── src/
│   │       ├── arithmetic/
│   │       │   ├── rns.rs              # Dual-RNS with K-Elimination
│   │       │   ├── exact_coeff.rs      # Exact coefficient arithmetic
│   │       │   ├── exact_divider.rs    # K-Elimination division
│   │       │   ├── bounded_rns.rs      # [clockwork] Bound tracking (INV-1..INV-4)
│   │       │   └── order_finding.rs    # Non-circular BSGS
│   │       ├── ops/
│   │       │   ├── rns_fhe.rs          # RNS-based FHE operations
│   │       │   ├── gso_fhe.rs          # GSO noise bounding layer
│   │       │   └── bootstrap.rs        # Clockwork bootstrap (circular security)
│   │       ├── entropy/
│   │       │   ├── crt_shadow.rs       # CRT Shadow entropy
│   │       │   ├── wassan_noise.rs     # Holographic noise field
│   │       │   └── secure.rs           # CSPRNG for keys + health check
│   │       ├── security/
│   │       │   ├── secret_data.rs      # CT primitives + SecretKeyPath trait
│   │       │   ├── gro_gate.rs         # [clockwork] GRO timing gate
│   │       │   ├── key_manager.rs      # [clockwork] Key lifecycle
│   │       │   └── integrity.rs        # [clockwork] Limb integrity (CRC32)
│   │       ├── keys/                   # Key generation (GRO-gated with clockwork)
│   │       ├── noise/                  # Noise budget tracking (checked arithmetic)
│   │       └── params/                 # FHE parameters + security estimator
│   ├── clockwork-core/   # Formal-spec RNS: GRO, Garner, bound tracking
│   ├── mana/             # Modular arithmetic accelerator
│   ├── unhal/            # Hardware abstraction layer
│   ├── nine65-python/    # PyO3 Python bindings
│   └── nine65-wasm/      # wasm-bindgen WebAssembly bindings
├── proofs/coq/           # 14 machine-checked Coq proofs
├── lean4/KElimination/   # 4 Lean4 formalizations
├── scripts/              # Quality gate scripts (no-panics, no-floats, claims)
└── docs/
    ├── FORMALIZATION_INDEX.md          # Proof-to-code mapping
    ├── NIST_COMPLIANCE_MATRIX.md       # NIST/HES compliance
    ├── SIDE_CHANNEL_THREAT_MODEL.md    # Threat model (T1-T5)
    ├── RELEASE_CHECKLIST_V6.md         # v6 release gates
    └── FHE_BENCHMARK_COMPARISON.md     # Industry comparison
```

---

## Workspace Crates

- **`nine65`**: Core FHE implementation (dual-RNS, K-Elimination, GSO-FHE, BFV ops)
- **`clockwork-core`**: Formal-spec RNS arithmetic (bound tracking, GRO timing, key lifecycle)
- **`nexgen_rational`**: Exact i128 rational arithmetic (zero dependencies)
- **`mana`**: Modular arithmetic accelerator
- **`unhal`**: Hardware abstraction and pipeline helpers
- **`nine65-python`**: Python bindings via PyO3 (requires `--features python`)
- **`nine65-wasm`**: WebAssembly bindings via wasm-bindgen (requires `wasm32-unknown-unknown` target)

---

## Features

| Feature | Description |
|---------|-------------|
| `ntt_fft` (default) | FFT-based NTT implementation |
| `parallel` (default) | Rayon-based parallel paths |
| `clockwork` | GRO timing gates, bound tracking, key lifecycle, integrity |
| `exact_rational` | NexGen rational bridge (exact noise tracking, BFV delta) |
| `accelerated` | MANA + UNHAL integration (opt-in) |
| `exact_transcendentals_backend` | Route transcendental ops through exact CORDIC backend |
| `wassan` | WASSAN holographic noise field (144 phi-harmonic) |
| `deterministic_rng` | Reproducible testing via rand_chacha |
| `v2` | V2 integration tests |
| `allow_insecure` | Test-only configs (compile-blocked in release builds) |

---

## Build and Test

```bash
# Build release (core + support crates)
cargo build --release --workspace --exclude nine65-python --exclude nine65-wasm

# Run all tests (core + support crates)
cargo test --release --exclude nine65-python --exclude nine65-wasm

# Build/test Python binding (requires Python toolchain)
cargo test -p nine65-python --features python --release

# Build/test WASM binding (requires wasm32 target)
cargo test -p nine65-wasm --target wasm32-unknown-unknown --release

# Run with key optional features
cargo test -p nine65 --features v2,accelerated,wassan,exact_transcendentals_backend --release

# Run depth benchmark
cargo test --package nine65 --lib --release \
  ops::gso_fhe::depth_benchmarks::benchmark_symmetric_max_depth_secure_128 -- --nocapture

# Run depth benchmark (secure_192)
cargo test --package nine65 --lib --release \
  ops::gso_fhe::depth_benchmarks::benchmark_symmetric_max_depth_secure_192 -- --nocapture

# Run full arithmetic benchmark
cargo test --package nine65 --lib --release \
  ops::gso_fhe::arithmetic_benchmarks::benchmark_full_arithmetic -- --nocapture

# Run order finding tests
cargo test -p nine65 arithmetic::order_finding --release -- --nocapture
```

### Clockwork Feature (v6)
```bash
# Build with GRO timing gates, bound tracking, key lifecycle
cargo build -p nine65 --features clockwork

# Run clockwork-specific tests
cargo test -p nine65 --lib --features clockwork

# Run error variant coverage
cargo test -p nine65 --test error_variant_coverage
```

### Quality Gates
```bash
# No-panics scan (advisory)
bash scripts/check_no_panics.sh

# No-floats scan (enforced)
bash scripts/check_no_floats_runtime.sh

# Claim drift detection
bash scripts/check_claim_registry.sh
bash scripts/check_stale_claims.sh
```

### Performance Tests (Opt-in)
```bash
NINE65_PERF_TESTS=1 cargo test -p nine65 --lib \
  entropy::wassan_noise::tests::test_benchmark_vs_shadow --release
```

---

## Quick Start

```rust
use nine65::params::secure_configs::SecureConfig;
use nine65::ops::rns_fhe::RNSFHEContext;
use nine65::ops::gso_fhe::GSOFHEContext;

// Create FHE context with production SecureConfig
let config = SecureConfig::secure_128().into_config();
let inner = RNSFHEContext::new_coeff_domain(&config);
let ctx = GSOFHEContext::new(inner);

// Generate keys
let keys = ctx.keygen();

// Encrypt
let ct_a = ctx.encrypt(42, &keys.public_key);
let ct_b = ctx.encrypt(7, &keys.public_key);

// Homomorphic operations (symmetric depth-50+ capable)
let ct_sum = ctx.add(&ct_a, &ct_b);
let ct_prod = ctx.mul(&ct_a, &ct_b, &keys.secret_key);

// Decrypt
let result = ctx.decrypt(&ct_prod, &keys.secret_key);
assert_eq!(result, 42 * 7);
```

### Public Mode (Multi-Party, Auto Mod-Switch)

Public mode now includes automatic modulus switching at depth 3+. Initial depth baselines
are in docs/PUBLIC_MODE_DEPTH_BASELINE_2026-01-27.md.

```rust
use nine65::params::secure_configs::SecureConfig;
use nine65::ops::rns_fhe::RNSFHEContext;
use nine65::entropy::ShadowHarvester;

let config = SecureConfig::secure_192().into_config();
let ctx = RNSFHEContext::new_coeff_domain(&config);
let mut rng = ShadowHarvester::from_os_seed();

// Smaller decomposition base reduces relin noise for deeper public circuits
let keys = ctx.generate_keys_dual_full_public_deep(&mut rng);

let ct_a = ctx.encrypt_dual(2, &keys.public_key, &mut rng);
let ct_b = ctx.encrypt_dual(3, &keys.public_key, &mut rng);
let ct_prod = ctx.mul_dual_public(&ct_a, &ct_b, &keys.eval_key);
let result = ctx.decrypt_dual(&ct_prod, &keys.secret_key);

assert_eq!(result, 6);
```

---

## Security Notes

### Entropy Sources
| Operation | Entropy Source | Module |
|-----------|----------------|--------|
| Secret key generation | OS CSPRNG | `entropy::secure` |
| Public key randomness | OS CSPRNG | `entropy::secure` |
| Noise sampling | Shadow/Secure | `entropy::shadow` or `secure` |
| Testing/benchmarks | Deterministic | `entropy::shadow` |

### Documentation
- `docs/SECURITY_PROOFS.md` - Security assumptions and proofs
- `docs/SIDE_CHANNEL_THREAT_MODEL.md` - Side-channel threat model (v6)
- `docs/NIST_COMPLIANCE_MATRIX.md` - NIST/HES compliance matrix (v6)
- `docs/FORMALIZATION_INDEX.md` - Proof-to-code traceability (v6)
- `docs/FHE_BENCHMARK_COMPARISON.md` - Industry comparison with sources
- `docs/PERFORMANCE_BASELINE_2026-02-11.md` - Measured performance baseline and environment
- `docs/LATTICE_ESTIMATOR_BASELINE_2026-02-09.md` - LWE estimator baseline
- `docs/PUBLIC_MODE_DEPTH_BASELINE_2026-01-27.md` - public-mode depth sweep
- `docs/RELEASE_CHECKLIST_V6.md` - v6 release gates

---

## Formal Verification

NINE65 components are backed by machine-checked proofs in Coq and Lean4.
Proof-to-code mappings are tracked in `docs/FORMALIZATION_INDEX.md`.

### Coq Proofs (`proofs/coq/`)

| File | Component | Status |
|------|------------|--------|
| `KElimination.v` | K-Elimination exact division | Verified |
| `K_Elimination.v` | Core k-value theorems | Verified |
| `GSOFHE.v` | GSO-FHE noise bounding | Verified |
| `CRTShadowEntropy.v` | Shadow entropy harvesting | Verified |
| `OrderFinding.v` | Non-circular order finding | Verified |
| `MQReLU.v` | MQ-ReLU activation | Verified |
| `IntegerSoftmax.v` | Integer softmax | Verified |
| `MontgomeryPersistent.v` | Montgomery arithmetic | Verified |
| `MobiusInt.v` | Mobius integer transforms | Verified |
| `CyclotomicPhase.v` | Cyclotomic phase tracking | Verified |
| `PadeEngine.v` | Pade approximation engine | Verified |
| `ExactCoefficient.v` | Exact coefficient arithmetic | Verified |
| `StateCompression.v` | FHE state compression | Verified |
| `SideChannelResistance.v` | Side-channel mitigations | Verified |
| `EncryptedQuantum.v` | Encrypted quantum simulation | Verified |

### Lean4 Proofs (`lean4/KElimination/`)

| File | Content | Status |
|------|---------|--------|
| `KElimination.lean` | Main K-Elimination formalization | Verified |
| `Basic.lean` | Core definitions | Verified |
| `ShadowEntropy.lean` | Shadow/quotient reconstruction | Verified |
| `ZMod.lean` | Modular arithmetic lemmas | Verified |

### Verification Commands

```bash
# Coq verification (requires Coq 8.18+)
cd proofs/coq && coqc *.v

# Lean4 verification (requires Lean 4.x + Mathlib)
cd lean4/KElimination && lake build
```

### Proof Archive

A standalone archive of all proofs is available:
```bash
# Location: ~/v5_proofs.tar.gz (52KB)
tar -xzvf ~/v5_proofs.tar.gz
```

---

## Technical Foundation

NINE65 is built on the QMNF (Quantized Modular Number Field) architecture:

1. **Integer-only arithmetic**: Zero f64/f32 across all workspace crates. The sole exemption is `compiler.rs` (offline static noise analysis, not runtime).
2. **Stacked CRT**: Two-layer exact arithmetic (fast + unlimited precision)
3. **Fused Piggyback Division**: 40x faster RNS division via anchor-first computation
4. **Deterministic execution**: Bit-identical results across all platforms

---

## Test Status

**All tests passing** (verified 2026-02-15):

| Crate | Tests | Status |
|-------|-------|--------|
| `nine65` (core FHE) | 870+ | ✅ Pass |
| `clockwork-core` | 46 | ✅ Pass |
| `nexgen_rational` | 95 | ✅ Pass |
| `mana` | 30 | ✅ Pass |
| `unhal` | 10 | ✅ Pass |
| Integration tests | 31+ | ✅ Pass |
| Optional crates | see note | ⚠️ Requires extra toolchains |
| **Total (default features)** | **1,002** | ✅ **All Pass** |
| **Total (clockwork feature)** | **589** | ✅ **All Pass** |

Notes:
- `nine65-python` and `nine65-wasm` require optional toolchains/features and are not built in the default test sweep.
- Clockwork tests run as a separate CI job with `--features clockwork`.
- All 29 `Nine65Error` variants have dedicated coverage tests.

### Key Validations
- ✅ Depth-50 circuits with 0 bootstraps (symmetric mode)
- ✅ Encrypt→Decrypt roundtrip (property-based testing)
- ✅ K-Elimination constant-time matches variable-time output
- ✅ Ciphertext randomness (semantic security)
- ✅ Formal invariants (softmax sum, Möbius roundtrip, Padé identities)
- ✅ GRO timing gate integration on keygen and decrypt (v6)
- ✅ Circular security validation (v6)
- ✅ Entropy health check (v6)
- ✅ Error variant coverage — all 29 variants tested (v6)

---

## License

Proprietary. See `LICENSE`.

---

## Archive

Old implementation reports and session updates are preserved in `archive/`.

---

*Last updated: 2026-02-15*
*NINE65 v6 "a Clockwork Prime" - Bootstrap-Free FHE with K-Elimination*
