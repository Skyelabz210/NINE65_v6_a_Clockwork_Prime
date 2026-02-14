# FORENSIC AUDIT: Build 11_MYSTIC_v2

**Build**: `11_MYSTIC_v2`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/11_MYSTIC_v2/`
**Symlink Target**: `~/Projects/MYSTIC/nine65_v2_complete/`
**Crate**: `qmnf_fhe` v0.1.0
**Auditor**: Claude Opus 4.6 (Forensic Code Auditor)
**Date**: 2026-02-13

---

## 1. STRUCTURE MAPPING

### 1.1 Module Tree

```
src/
  lib.rs                          (root: 11 pub mod declarations + prelude + integration_tests)
  compiler.rs                     (FHE circuit compiler)
  kat.rs                          (Known Answer Tests)
  v2_integration_tests.rs         (#[cfg(test)] only)

  arithmetic/
    mod.rs                        (15 submodules)
    montgomery.rs                 (Gen 1 Montgomery)
    persistent_montgomery.rs      (Gen 2 - stays in Montgomery form)
    barrett.rs                    (Barrett reduction)
    ntt.rs                        (Gen 3 DFT-based NTT)
    ntt_fft.rs                    (V2: O(N log N) FFT-based NTT)
    rns.rs                        (RNS/CRT parallel arithmetic)
    k_elimination.rs              (Exact integer division)
    exact_divider.rs              (Dual-track reconstruction)
    exact_coeff.rs                (Dual-track coefficients)
    ct_mul_exact.rs               (Exact ciphertext multiplication)
    mobius_int.rs                  (Signed arithmetic via polarity separation)
    pade_engine.rs                (Integer-only transcendentals)
    mq_relu.rs                    (O(1) sign detection)
    integer_softmax.rs            (Exact-sum softmax)
    cyclotomic_phase.rs           (Native ring trigonometry)
    polypoly.rs                   (Polynomial multiply strategies)

  entropy/
    mod.rs                        (3 submodules)
    shadow.rs                     (Deterministic LFSR-based PRNG)
    secure.rs                     (OS CSPRNG wrappers)
    wassan_noise.rs               (V2: 144 phi-harmonic holographic noise field)

  params/
    mod.rs                        (FHEConfig definitions)
    primes.rs                     (NTT-friendly primes)
    production.rs                 (128-bit production parameters)
    validation.rs                 (Parameter validation)

  ring/
    mod.rs                        (Ring module)
    polynomial.rs                 (Ring polynomial operations)

  keys/
    mod.rs                        (SecretKey, PublicKey, EvaluationKey, KeySet)

  ops/
    mod.rs                        (4 submodules)
    encrypt.rs                    (BFV encode/encrypt/decrypt)
    homomorphic.rs                (Homomorphic add/sub/mul/negate)
    rns_mul.rs                    (RNS-based ciphertext multiplication)
    neural.rs                     (FHE neural network evaluator)

  noise/
    mod.rs                        (Noise tracking, EMA, P2 quantile)
    budget.rs                     (Noise budget management)

  security/
    mod.rs                        (LWE security estimation)

  ahop/
    mod.rs                        (AHOP quantum simulation)
    grover.rs                     (Grover search)
    grover_full.rs                (Full Grover implementation)

  quantum/
    mod.rs                        (4 submodules + quantum_demo)
    entanglement.rs               (Entangled pairs, GHZ states)
    teleport.rs                   (Quantum teleportation via K-Elimination)
    amplitude.rs                  (Signed amplitudes for interference)
    period_grover.rs              (Period-Grover fusion, WASSAN Grover)

  chaos/                          (* MYSTIC-UNIQUE *)
    mod.rs                        (13 submodules)
    lorenz.rs                     (Exact Lorenz attractor, Montgomery-accelerated)
    lorenz_kelim.rs               (K-Elimination Lorenz, exact beta division)
    lyapunov.rs                   (Exact Lyapunov exponent analysis)
    attractor.rs                  (Attractor detection + basin analysis)
    crt_shadow.rs                 (CRT shadow entropy harvesting)
    data_ingest.rs                (Training data loading)
    global_attractor_db.rs        (Global attractor database + WASSAN delta)
    poisson.rs                    (Poisson brackets + symplectic integrator)
    sensor_aggregation.rs         (Secure multi-station sensor aggregation)
    wassan_delta.rs               (WASSAN delta timeline compression)
    weather.rs                    (DELUGE weather/flood prediction engine)
    liouville.rs                  (Liouville equation: probability density evolution)
    quantum_enhanced.rs           (Quantum-enhanced pattern detection)
    spanky.rs                     (SPANKY unified 10-method forecaster)

  epram/                          (* MYSTIC-UNIQUE *)
    mod.rs                        (5 submodules)
    cell.rs                       (EPRAMCell trait, ModularCell, MontgomeryEPRAMCell)
    field.rs                      (EPRAMField, Topology, CouplingMode)
    attractor.rs                  (Fourth Attractor: dithered convergence algorithm)
    parallel.rs                   (Rayon parallel field evolution, ParallelRNSCell)
    spanky_bridge.rs              (EPRAM-SPANKY forecasting bridge)

  bin/
    crypto_audit.rs
    fhe_benchmarks.rs
    neural_bench.rs
    mystic_demo.rs
    historical_failures_demo.rs
    test_camp_mystic_2007.rs
    train_mystic.rs
    lorenz_bench.rs
    spanky_eval.rs
    spanky_forecast.rs
```

### 1.2 Features (Cargo.toml)

| Feature | Definition | Default |
|---------|-----------|---------|
| `ntt_fft` | Enables FFT-based NTT (O(N log N)) | YES (in `default`) |
| `wassan` | Enables WASSAN holographic noise field | No |
| `v2` | Meta-feature: enables `ntt_fft` + `wassan` | No |
| `secure-keygen` | Enables OS CSPRNG key generation | No |

### 1.3 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| zeroize | 1.7 (derive) | Secure memory clearing |
| getrandom | 0.2 | OS CSPRNG |
| subtle | 2.5 | Constant-time ops |
| sha2 | 0.10 | KAT hashing |
| serde + serde_json | 1.0 | JSON model loading |
| rayon | 1.10 | EPRAM parallel field evolution |

---

## 2. DATA FLOW TRACING

### 2.1 Standard BFV FHE Pipeline

```
FHEConfig::light() / he_standard_128()
  -> NTTEngine::new(q, n)  [or NTTEngineFFT via feature gate]
  -> KeySet::generate(&config, &ntt, &mut ShadowHarvester)
     -> SecretKey (ternary polynomial)
     -> PublicKey (pk0, pk1)
     -> EvaluationKey
  -> BFVEncoder::new(&config)  [delta = floor(q/t)]
  -> BFVEncryptor::new(&pk, &encoder, &ntt, eta)
     -> encrypt(msg, &mut ShadowHarvester) -> Ciphertext(c0, c1)
  -> BFVDecryptor::new(&sk, &encoder, &ntt)
     -> decrypt(&ct) -> u64
  -> BFVEvaluator::new(&ntt, &encoder, Some(&eval_key))
     -> add/sub/negate/mul_plain/add_plain/mul
```

The NTT engine selection is conditional:
- `src/ops/encrypt.rs:7-11`: `#[cfg(feature = "ntt_fft")] use NTTEngineFFT as NTTEngine`
- `src/lib.rs:141-145`: prelude re-exports `NTTEngineFFT as NTTEngine` when `ntt_fft` enabled

Since `ntt_fft` is in `default`, the FFT engine is always active unless defaults are disabled.

### 2.2 Chaos Module Integration with FHE Core

**Connection points**:

1. `chaos::lorenz` uses `arithmetic::persistent_montgomery::PersistentMontgomery` for scale_mul acceleration (lorenz.rs:30, 40-43, 282-306). This is a DIRECT integration with the core arithmetic layer.

2. `chaos::lorenz_kelim` uses `arithmetic::k_elimination::KElimination` for exact beta=8/3 division (lorenz_kelim.rs:24, 137-139, 162-183). Direct integration with the K-Elimination innovation.

3. `chaos::attractor` uses `arithmetic::pade_engine::{PadeEngine, PADE_SCALE}` for integer-only exp(-x) computation in match scoring (attractor.rs:27-28, 130-145). Direct integration.

4. `chaos::crt_shadow` uses `arithmetic::k_elimination::KElimination` for entropy harvesting from division byproducts.

5. `chaos::quantum_enhanced` uses `quantum::period_grover::{WassanGroverState, PeriodGroverFusion}` for holographic attractor search and flood timing.

6. `chaos::sensor_aggregation` uses K-Elimination for secure multi-station sensor aggregation.

7. `chaos::spanky` orchestrates ALL of the above: ExactLorenz, KElimLorenz, Liouville, Poisson brackets, sensor aggregation, CRT shadow entropy, WASSAN delta, quantum long-term, and the global attractor database into a unified 10-method forecaster.

**Verdict**: Chaos module is GENUINELY WIRED to the FHE core arithmetic layer. Not standalone.

### 2.3 EPRAM Module Integration

**Connection points**:

1. `epram::cell::MontgomeryEPRAMCell` uses `arithmetic::PersistentMontgomery` (cell.rs:6). Cells permanently live in Montgomery form.

2. `epram::parallel::ParallelEPRAM` uses `rayon` for parallel field evolution (parallel.rs:12). `ParallelRNSCell` uses `arithmetic::k_elimination::KElimination` for exact division across RNS lanes (parallel.rs:18).

3. `epram::spanky_bridge::EPRAMForecaster` bridges EPRAM fields to chaos weather forecasting. Creates `EPRAMField<ModularCell>` from weather data, runs convergence, detects basin membership (spanky_bridge.rs:199-288). Also invokes `ParallelEPRAM::step` for parallel forecasting (spanky_bridge.rs:308).

**Verdict**: EPRAM is WIRED through. PersistentMontgomery cells, K-Elimination in parallel RNS, and the spanky_bridge connects EPRAM directly to the chaos forecasting pipeline.

### 2.4 WASSAN Feature Behavior

**Critical finding**: The `wassan` feature flag has NO conditional compilation gates in source code.

Grep for `#[cfg(feature = "wassan")]` returns ZERO matches. The `WassanNoiseField` struct is:
- Always compiled (no feature gates in `entropy/wassan_noise.rs`)
- Always exported in prelude (`lib.rs:153` -- unconditional)
- Always re-exported from `entropy/mod.rs:30` -- unconditional

The `wassan` feature exists only in `Cargo.toml:12` as an empty feature (`wassan = []`) that is included by the `v2` meta-feature.

**The `wassan` feature does NOT change behavior when enabled.** `WassanNoiseField` is always available regardless of feature flags. The feature flag is vestigial.

However, `WassanNoiseField` is NOT wired into the FHE encryption pipeline. The encrypt path (`ops/encrypt.rs:138-170`) uses ONLY `ShadowHarvester` for noise generation. `WassanNoiseField` exists as an independent noise source but is never called from the FHE encrypt/keygen paths. It is tested independently in `v2_integration_tests.rs` and `entropy/wassan_noise.rs` tests.

**Verdict**: WASSAN feature flag is a dead flag. The type is always compiled. It is NOT integrated into the FHE pipeline -- it is a standalone noise generator that could be used but is not wired through to encrypt/keygen.

---

## 3. CONSTRUCT IDENTIFICATION: MYSTIC-Unique Types

### 3.1 Chaos Types

| Type | File:Line | Purpose |
|------|-----------|---------|
| `ExactLorenz` | chaos/lorenz.rs:145 | Montgomery-accelerated Lorenz integrator |
| `LorenzState` | chaos/lorenz.rs:85 | Scaled i128 state (x,y,z) |
| `LorenzParams` | chaos/lorenz.rs:46 | Scaled sigma/rho/beta parameters |
| `KElimLorenz` | chaos/lorenz_kelim.rs:117 | K-Elimination exact beta division Lorenz |
| `KElimLorenzState` | chaos/lorenz_kelim.rs:67 | State for KElim Lorenz |
| `KElimLorenzParams` | chaos/lorenz_kelim.rs:31 | Params for KElim Lorenz |
| `LyapunovAnalyzer` | chaos/lyapunov.rs:83 | Exact Lyapunov exponent computation |
| `LyapunovExponent` | chaos/lyapunov.rs:27 | Accumulated log divergence |
| `ChaosSignature` | chaos/lyapunov.rs:221 | Chaos state fingerprint for attractor matching |
| `AttractorDetector` | chaos/attractor.rs:208 | Multi-attractor detection engine |
| `AttractorSignature` | chaos/attractor.rs:33 | Learned attractor profile |
| `AttractorBasin` | chaos/attractor.rs:148 | Geometric basin in phase space |
| `WeatherState` | chaos/weather.rs:34 | Atmospheric state mapped to Lorenz phase space |
| `FloodDetector` | chaos/weather.rs:233 | DELUGE flash flood detector |
| `DelugeEngine` | chaos/weather.rs:472 | Multi-station coordinator |
| `FloodPrediction` | chaos/weather.rs:213 | Prediction result with alert level |
| `AlertLevel` | chaos/weather.rs:177 | Clear/Watch/Advisory/Warning/Emergency |
| `RawSensorData` | chaos/weather.rs:59 | 19-field raw sensor struct |
| `CrtShadowHarvester` | chaos/crt_shadow.rs:27 | Entropy from K-Elimination byproducts |
| `BasinTransitionEntropy` | chaos/crt_shadow.rs (re-exported) | Basin transition entropy source |
| `KElimWithShadow` | chaos/crt_shadow.rs (re-exported) | K-Elim + shadow entropy combined |
| `LiouvilleEvolver` | chaos/liouville.rs:334 | Probability density evolution beyond 14 days |
| `PhaseDensity` | chaos/liouville.rs (re-exported) | Phase-space probability distribution |
| `PhaseCell` | chaos/liouville.rs (re-exported) | Discretized phase-space cell |
| `MobiusInt` (liouville) | chaos/liouville.rs:33 | Signed integer for Poisson brackets (duplicate of arithmetic::MobiusInt) |
| `PoissonBracket` | chaos/poisson.rs (re-exported) | {rho, H} computation |
| `VerletIntegrator` | chaos/poisson.rs (re-exported) | Symplectic time integration |
| `QuantumPeriodDetector` | chaos/quantum_enhanced.rs:34 | Autocorrelation-based period finder |
| `HolographicAttractorSearch` | chaos/quantum_enhanced.rs:132 | WASSAN Grover basin search |
| `QuantumFloodTimer` | chaos/quantum_enhanced.rs:203 | Period-Grover flood timing |
| `QuantumMYSTIC` | chaos/quantum_enhanced.rs:329 | Quantum-enhanced weather engine |
| `SpankyForecaster` | chaos/spanky.rs:87 | Unified 10-method forecaster |
| `SpankyForecast` | chaos/spanky.rs:79 | Unified forecast result |
| `DiagnosticsReport` | chaos/spanky.rs:33 | Per-method diagnostics |
| `SecureSensorAggregator` | chaos/sensor_aggregation.rs | RNS-based multi-station aggregation |
| `WassanDeltaEncoder` | chaos/wassan_delta.rs | Timeline compression encoder |
| `GlobalAttractorDB` | chaos/global_attractor_db.rs | Global basin database + transition history |

### 3.2 EPRAM Types

| Type | File:Line | Purpose |
|------|-----------|---------|
| `EPRAMCell` (trait) | epram/cell.rs:18 | Cell trait: modulus, value, transition, coupled_transition |
| `ModularCell` | epram/cell.rs:108 | Simple modular arithmetic cell |
| `MontgomeryEPRAMCell` | epram/cell.rs:164 | Cell in persistent Montgomery form (27ns mul) |
| `EPRAMField<C>` | epram/field.rs:148 | Generic field over EPRAMCell implementations |
| `Topology` | epram/field.rs:28 | Independent/Ring/Grid/Complete graph |
| `CouplingMode` | epram/field.rs:83 | Independent/Coupled/Hybrid dynamics |
| `TerminationContract` | epram/field.rs:113 | FixedPoint/Lyapunov/CycleDetect/MaxSteps |
| `TerminationResult` | epram/field.rs:127 | Result enum with step count |
| `FourthAttractorParams` | epram/attractor.rs:29 | k_num/k_den convergence factor |
| `fourth_attractor_step_dithered` | epram/attractor.rs:89 | Core convergence function (100% guarantee) |
| `ParallelEPRAM` | epram/parallel.rs:28 | Rayon-based parallel field executor |
| `ParallelRNSCell` | epram/parallel.rs:152 | Multi-lane RNS cell with K-Elimination |
| `EPRAMForecaster` | epram/spanky_bridge.rs:115 | EPRAM-to-weather bridge |
| `EPRAMWeatherState` | epram/spanky_bridge.rs:44 | Weather encoded for EPRAM processing |
| `ForecastMode` | epram/spanky_bridge.rs:33 | Independent/Consensus/Hybrid |
| `ForecastResult` | epram/spanky_bridge.rs:334 | Basin signatures + confidence |
| `BasinSignature` | epram/spanky_bridge.rs:102 | Basin ID + consensus + votes |

### 3.3 WASSAN Noise Field Type

| Type | File:Line | Purpose |
|------|-----------|---------|
| `WassanNoiseField` | entropy/wassan_noise.rs:32 | 144-band x 4096-sample holographic noise (4.7 MB) |

---

## 4. WIRING VERIFICATION

### 4.1 Chaos to FHE Core Wiring

| Connection | Source | Target | Status |
|-----------|--------|--------|--------|
| Montgomery acceleration | chaos/lorenz.rs:30,284-305 | arithmetic/persistent_montgomery.rs | LIVE -- `scale_mul` uses `PersistentMontgomery` for every Lorenz derivative |
| K-Elimination division | chaos/lorenz_kelim.rs:137-139,162-183 | arithmetic/k_elimination.rs | LIVE -- `exact_div` and `beta_z` route through KElimination |
| Pade integer exponential | chaos/attractor.rs:130-145 | arithmetic/pade_engine.rs | LIVE -- `exp_neg` uses PadeEngine for match scoring |
| K-Elim sensor aggregation | chaos/sensor_aggregation.rs | arithmetic/k_elimination.rs | LIVE -- re-exported and used in SPANKY |
| CRT shadow entropy | chaos/crt_shadow.rs | arithmetic/k_elimination.rs | LIVE -- harvests from K-Elim operations |
| Quantum period detection | chaos/quantum_enhanced.rs:17-19 | quantum/period_grover.rs | LIVE -- uses WassanGroverState, PeriodGroverFusion |

### 4.2 EPRAM to FHE Core Wiring

| Connection | Source | Target | Status |
|-----------|--------|--------|--------|
| PersistentMontgomery cells | epram/cell.rs:6,164-257 | arithmetic/persistent_montgomery.rs | LIVE -- MontgomeryEPRAMCell wraps PersistentMontgomery |
| K-Elimination in ParallelRNS | epram/parallel.rs:18,159,187 | arithmetic/k_elimination.rs | AVAILABLE but OPTIONAL -- `k_elim` field is `Option<Arc<KElimination>>`, `with_k_elimination` must be called |
| Rayon parallelism | epram/parallel.rs:12,46-98 | rayon crate | LIVE -- `into_par_iter()` used in step() |
| Fourth Attractor | epram/attractor.rs:89-115 | (self-contained) | LIVE -- pure integer algorithm |

### 4.3 WASSAN Feature Wiring

| Connection | Source | Target | Status |
|-----------|--------|--------|--------|
| `wassan` feature flag | Cargo.toml:12 | (none) | DEAD -- no `#[cfg(feature = "wassan")]` anywhere in source |
| WassanNoiseField in prelude | lib.rs:153 | entropy/wassan_noise.rs | UNCONDITIONAL -- always exported |
| WassanNoiseField in FHE encrypt | (none) | ops/encrypt.rs | NOT WIRED -- encrypt uses ShadowHarvester only |
| WassanNoiseField in FHE keygen | (none) | keys/mod.rs | NOT WIRED -- keygen uses ShadowHarvester only |
| WassanNoiseField in chaos | chaos/quantum_enhanced.rs | quantum/period_grover.rs | INDIRECT -- quantum_enhanced uses WassanGroverState (different type) |

---

## 5. DEAD CODE DETECTION

### 5.1 Chaos Module

| Item | File:Line | Status |
|------|-----------|--------|
| `scale_mul_plain` | chaos/lorenz.rs:310 | `#[allow(dead_code)]` -- benchmark baseline, intentionally dead |
| `BasinEntry.sample_count` | chaos/attractor.rs:165 | `#[allow(dead_code)]` -- deserialized but unused |
| `weather_detector()` free fn | chaos/attractor.rs:351 | No callers found in source (all callers use `FloodDetector::new()` which creates its own detector) |
| `data_ingest` module | chaos/data_ingest.rs | Re-exported but used only from bin targets (train_mystic, etc.) |

### 5.2 EPRAM Module

| Item | File:Line | Status |
|------|-----------|--------|
| `ParallelRNSCell.k_elim` | epram/parallel.rs:159 | OPTIONAL field -- `with_k_elimination` never called in any non-test code. Wiring exists but is dormant. |
| `ForecastResult::transition_probability` | epram/spanky_bridge.rs:379 | Returns f64 probability but no callers found in main code paths. Tests exercise it. |

### 5.3 WASSAN

| Item | File:Line | Status |
|------|-----------|--------|
| `WassanNoiseField::from_os_seed` | entropy/wassan_noise.rs:89 | Gated behind `#[cfg(feature = "secure_seed")]` -- this feature does not exist in Cargo.toml. PERMANENTLY DEAD CODE. |
| `WassanNoiseField::sample_band` | entropy/wassan_noise.rs:133 | No callers outside tests. |
| `WassanNoiseField::checkpoint/restore` | entropy/wassan_noise.rs:207-215 | No callers outside the struct itself. |
| Entire `wassan` feature flag | Cargo.toml:12 | DEAD -- enables nothing (empty feature with no cfg gates) |

### 5.4 Quantum Module

| Item | File:Line | Status |
|------|-----------|--------|
| `quantum_demo()` | quantum/mod.rs:83 | Called only from test (`test_quantum_demo_runs`), not from main code. |

---

## 6. CROSS-REFERENCE: v2_integration_tests.rs COVERAGE

`v2_integration_tests.rs` contains 6 tests, all under `#[cfg(test)]`:

| Test | What it covers | Modules exercised |
|------|---------------|-------------------|
| `test_fft_ntt_roundtrip` | NTT FFT forward/inverse roundtrip (N=8) | arithmetic/ntt_fft |
| `test_fft_matches_dft` | FFT and DFT produce identical multiplication results | arithmetic/ntt_fft, arithmetic/ntt |
| `test_fft_negacyclic` | Negacyclic convolution property (x^4 = -1) | arithmetic/ntt_fft |
| `test_fft_1024_benchmark` | FFT NTT 1024 x 100 multiplications performance | arithmetic/ntt_fft |
| `test_wassan_deterministic` | Two WASSAN fields from same seed produce identical samples | entropy/wassan_noise |
| `test_wassan_ternary_distribution` | Ternary distribution roughly uniform | entropy/wassan_noise |
| `test_wassan_polynomial` | FHE noise polynomial coefficients within bounds | entropy/wassan_noise |
| `test_wassan_benchmark` | 1M samples under 5ms | entropy/wassan_noise |
| `test_v2_speedup_summary` | DFT vs FFT timing comparison | arithmetic/ntt, arithmetic/ntt_fft, entropy/wassan_noise |

**NOT covered by v2_integration_tests**:
- Chaos module (0 tests)
- EPRAM module (0 tests)
- WASSAN integration with FHE pipeline (not possible -- not wired)
- `ntt_fft` feature conditional compilation (always compiled, tested unconditionally)
- `wassan` feature flag behavior change (none to test)

**Note**: Chaos and EPRAM have extensive UNIT tests within their own modules (each module file has `#[cfg(test)] mod tests`).

---

## 7. ANOMALY CATALOGUE

### 7.1 Float Violations

**Total f64 occurrences in src/**: 200+ lines across 20+ files.

**Classification**:

**Category A: Boundary conversion (tolerable per CLAUDE.md "Float->Rational conversion")**:
- `chaos/lorenz.rs:69-73,99-103`: `LorenzParams::new()` and `LorenzState::new()` accept f64, convert to scaled i128. These are input boundary functions.
- `chaos/lorenz.rs:119-131`: `x_f64()`, `y_f64()`, `z_f64()` -- output display functions converting back to f64.
- `chaos/lorenz_kelim.rs:80-93`: `from_float()`, `to_float()` -- boundary conversions.
- `chaos/attractor.rs:255-263`: `register_basin()` accepts f64 center/radius, converts to scaled i128.
- `chaos/liouville.rs:280-283`: `VerletIntegrator::new()` accepts dt as f64, converts to scaled i128.

**Category B: Display/reporting (non-computational)**:
- `noise/mod.rs:74-80,270-282`: `noise_bits()`, `budget_bits()` return f64 for display only.
- `noise/budget.rs:202-239`: Remaining/initial bits as f64 for display.
- `v2_integration_tests.rs:167`: Speedup ratio for println.
- `chaos/global_attractor_db.rs:391,572`: Compression ratio and size_gb for display.

**Category C: VIOLATIONS -- f64 in computational paths**:

| File:Line | Violation | Severity |
|-----------|-----------|----------|
| `security/mod.rs:19,47,67,88-89,137` | `LWEParams.sigma: f64`, `SecurityEstimate.ratio: f64`, and all ratio/estimate computations use f64 arithmetic. `.sqrt()` at line 67. | MEDIUM -- security estimation only, not in crypto path |
| `params/mod.rs:316,371` | Security ratio checks use `n as f64 / log_q as f64` | LOW -- parameter validation only |
| `params/production.rs:63,155` | `ProductionConfig128.sigma: f64`, max_depth calculation uses f64 division | LOW -- configuration, not crypto |
| `params/validation.rs:153` | Security ratio as f64 | LOW -- validation only |
| `noise/mod.rs:424-582` | `P2QuantileEstimator` uses f64 fields (`p`, `n_prime`, `dn`) for P2 quantile algorithm. Multiple f64 computations. | HIGH -- noise tracking uses f64 in core computation. P2 algorithm inherently requires floating-point. |
| `compiler.rs:120-125,156,210-212,239-240,380-382,400-403` | `NoiseParams` uses 6 f64 fields. `FHECompiler.node_noise: HashMap<usize, f64>`. `CompilationResult` uses f64 for noise. Speedup estimation uses f64. | HIGH -- circuit compiler noise tracking is entirely f64-based |
| `chaos/weather.rs:192-193,215,219,225,245,344-395` | `FloodPrediction.probability: f64`, `confidence: f64`, `time_to_onset: Option<f64>`. Multiple f64 computations in `predict()`. | MEDIUM -- weather prediction output layer uses f64 for probability/confidence scores |
| `chaos/lyapunov.rs:46-50,70-73,199-216,232-263` | `LyapunovExponent.value() -> f64`, `confidence() -> f64`, `local_exponent() -> f64`, all ChaosSignature fields are f64. | MEDIUM -- chaos analysis output layer |
| `chaos/attractor.rs:40-41,45,59,83-84,91-127,135-144,194-201,214,230` | `AttractorSignature` Lyapunov bounds are f64, `match_score` returns f64, `distance_to_boundary` returns f64. `exp_neg` converts between i128/f64. | MEDIUM -- attractor matching mixes f64 with integer Pade |
| `chaos/liouville.rs:242-244,248-258,334,376-413,423-429,451,498` | `conservation_error() -> f64`, `region_probability() -> f64`, `evolve_days()` takes f64, `ExtendedForecast` has 3 f64 fields. | MEDIUM -- Liouville output layer |
| `chaos/quantum_enhanced.rs:156,168,187-188,250,391,397,414` | Multiple f64 in search results, probability, compression ratio. `.sin()` call at line 414 (test only). | LOW -- mostly output/display |
| `epram/spanky_bridge.rs:230,243,379-385` | `.sqrt() as usize` for grid width, `transition_probability() -> f64` | LOW -- grid sizing is cosmetic, probability is output |
| `chaos/spanky.rs:72` | `ShortTermForecast.regional_probability: f64` | LOW -- output layer |

### 7.2 WASSAN Feature Flag Anomaly

The `wassan` feature flag in Cargo.toml is defined as `wassan = []` (empty dependency list) and is referenced by `v2 = ["ntt_fft", "wassan"]`. However:

1. ZERO instances of `#[cfg(feature = "wassan")]` exist in any source file.
2. `WassanNoiseField` is compiled unconditionally.
3. Enabling `wassan` feature changes NOTHING in compiled behavior.
4. The `from_os_seed` method is gated on `#[cfg(feature = "secure_seed")]` which is a feature that does NOT EXIST in Cargo.toml. This method is permanently unreachable.

### 7.3 Incomplete Chaos-EPRAM Integration

The `chaos/spanky.rs` SpankyForecaster uses DelugeEngine + QuantumMYSTIC but does NOT use `EPRAMForecaster`. The EPRAM-SPANKY bridge (`epram/spanky_bridge.rs`) exists as a fully implemented module with tests, but is never called from `chaos/spanky.rs`. The SpankyForecaster's 10-method pipeline includes:

1. deluge_short_term
2. lorenz_exact
3. lorenz_kelim
4. poisson_bracket
5. sensor_aggregation
6. liouville_probability
7. global_attractor_db
8. wassan_delta
9. crt_shadow_entropy
10. quantum_long_term

EPRAM forecasting is method #0 that was NOT included. The bridge module is available but unwired from the unified forecaster.

### 7.4 Duplicate MobiusInt Definition

`chaos/liouville.rs:33` defines a `MobiusInt` struct that is separate from `arithmetic/mobius_int.rs:MobiusInt`. The Liouville version is simpler (magnitude + positive bool) while the arithmetic version is a full signed RNS type with polarity separation. The chaos module re-exports its own `MobiusInt` from `chaos/mod.rs:59`, which shadows the arithmetic version in the prelude. This creates a naming collision:
- `chaos::liouville::MobiusInt` (i128-based, simple)
- `arithmetic::mobius_int::MobiusInt` (full RNS-based)

Both are exported from the prelude, but Rust allows this since they are from different paths.

### 7.5 Unsafe Send/Sync Implementations

| Type | File:Line | Concern |
|------|-----------|---------|
| `ModularCell` | epram/cell.rs:123-124 | `unsafe impl Send/Sync` -- unnecessary since `ModularCell` contains only `u64` fields which are automatically Send+Sync |
| `MontgomeryEPRAMCell` | epram/cell.rs:222-223 | `unsafe impl Send/Sync` -- PersistentMontgomery contains only integer fields, likely auto-derives |
| `ParallelRNSCell` | epram/parallel.rs:288-289 | `unsafe impl Send/Sync` -- contains `Arc<Vec<u64>>` and `Option<Arc<KElimination>>`, both should auto-derive |

These unsafe impls are unnecessary boilerplate but not harmful since the underlying types are trivially thread-safe.

### 7.6 `secure_seed` Feature Ghost

`entropy/wassan_noise.rs:88` has `#[cfg(feature = "secure_seed")]` on `from_os_seed()`, but `secure_seed` is not defined in Cargo.toml features. This method can never be compiled. The correct feature name would be `secure-keygen` (which exists in Cargo.toml:10).

### 7.7 Test-Only Float in chaos/quantum_enhanced.rs:414

```rust
let value = ((i % 10) as f64 * 0.628).sin() as i64 * 100;
```

This is inside a `#[cfg(test)]` block but uses `.sin()` -- a floating-point transcendental. Within the test context this generates synthetic periodic data and is not a production violation.

### 7.8 Assertion Vacuity

`chaos/quantum_enhanced.rs:420`:
```rust
assert!(!periods.is_empty() || periods.len() == 0);
```
This assertion is a tautology -- it is always true regardless of the value. It was likely meant to be a comment about expected behavior rather than an actual assertion.

---

## 8. SUMMARY OF FINDINGS

### Wiring Status

| Module | Connected to Core? | Integration Depth |
|--------|-------------------|-------------------|
| chaos/lorenz | YES | PersistentMontgomery in hot loop |
| chaos/lorenz_kelim | YES | KElimination for exact division |
| chaos/attractor | YES | PadeEngine for integer exp |
| chaos/weather | YES (through lorenz/lyapunov/attractor) | Multi-layer |
| chaos/spanky | YES | 10-method unified orchestrator |
| chaos/quantum_enhanced | YES | Period-Grover, WassanGroverState |
| epram/cell | YES | PersistentMontgomery |
| epram/parallel | YES | Rayon + KElimination (optional) |
| epram/spanky_bridge | PARTIALLY | Has wiring to EPRAM but NOT called from SPANKY |
| WASSAN noise field | STANDALONE | Not wired to FHE encrypt/keygen |
| wassan feature | DEAD FLAG | No cfg gates exist |

### Risk Assessment

| Finding | Risk | Impact |
|---------|------|--------|
| f64 in P2QuantileEstimator | HIGH | Non-deterministic noise tracking across platforms |
| f64 in FHE Compiler noise | HIGH | Non-deterministic circuit compilation across platforms |
| f64 in security estimation | MEDIUM | Non-deterministic security level reporting |
| f64 in weather output | MEDIUM | Probability scores may differ across platforms |
| wassan feature dead | LOW | No functional impact, cosmetic only |
| secure_seed ghost feature | LOW | Dead code, no functional impact |
| EPRAM-SPANKY unwired | LOW | Missed optimization, not correctness issue |
| MobiusInt duplication | LOW | Naming confusion, both are correct |
| Unnecessary unsafe impls | LOW | No safety issue, just boilerplate |
| Tautological assertion | LOW | Cosmetic, test-only |
