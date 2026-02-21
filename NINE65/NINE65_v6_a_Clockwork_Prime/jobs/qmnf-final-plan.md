# QMNF Consolidated Execution Plan
## MathCore + CryptKit — Final Work Breakdown Structure
### Replaces: Base Plan, Amendments 1–3

**132 tasks · 26 quality gates · 4 tracks**

---

## Plan Architecture

Four parallel tracks with explicit dependency crossings:

- **Track A** — MathCore open-source library (MIT). Exact integer arithmetic for developers.
- **Track B** — CryptKit commercial cryptography library (AGPL + commercial dual license). AHOP post-quantum crypto + NINE65 unlimited-depth FHE via Clockwork Bootstrap.
- **Track S** — Security Hardening. Adversarial mitigations integrated across both tracks.
- **Track X** — Cross-track integration nodes.

**Notation:** `⊘ GATE` = quality gate (hard/firm/soft). `→` = dependency. Tasks marked `[EXPANDED]` have scope additions from amendments. Tasks marked `[NEW]` were added by amendments.

---

## Track A: MathCore (57 tasks, 11 gates)

*Unchanged from base plan. MathCore is the foundation — K-Elimination exact arithmetic that enables everything downstream, including the Clockwork Bootstrap.*

### A0: Pre-Flight Assessment (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A0.1 | **API Surface Audit.** Catalog every function/struct/trait in the 800K+ line codebase. Classify as expose, wrap, or hide. | Every module reviewed. Zero unclassified items. |
| A0.2 | **Minimum Viable API Definition.** Thinnest wrapper for exact arithmetic: CRTBigInt creation, arithmetic, comparison, serialization, type conversion. | API covers core operations. Nothing more. |
| A0.3 | **Performance Baseline.** Benchmark against GMP, num-bigint, Python int, JS BigInt across operand sizes (64-bit to 4096-bit). | All benchmarks reproducible with statistical rigor. |
| A0.4 | **Formal Verification Coverage Audit.** Map Lean 4/Coq proofs to planned API. Identify gaps. | Coverage matrix complete. Division/modular proofs prioritized. |
| A0.5 | **Dependency and Build Audit.** Verify zero runtime deps. Reproducible builds on Linux/macOS/Windows. | Zero runtime dependencies. All platforms build. |

### ⊘ GATE A0: Foundation Verified [Hard]

### A1: Core Stabilization (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A1.1 | **Error Handling.** Unified error types. Replace all panics with Results in public paths. | `grep -r "panic!"` returns zero hits on public paths. |
| A1.2 | **Memory Safety.** Audit all `unsafe` blocks. Miri clean. Document safety invariants. | Miri clean. Every unsafe has `// SAFETY:` comment. |
| A1.3 | **Thread Safety.** Determine Send/Sync bounds for all public types. | All types documented as Send+Sync or !Send/!Sync with rationale. |
| A1.4 | **Input Validation.** Define valid input domains. Fuzz all entry points. | 1M fuzz iterations, zero panics. |
| A1.5 | **Edge Case Catalog.** Systematic edge cases: zero, max, min, primes, size mismatches. | 500+ edge case tests passing. ≥95% coverage on public paths. |

### ⊘ GATE A1: Core Hardened [Hard]

### A2: API Design (6 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A2.1 | **Type System.** ExactInt, RnsInt, ModularContext with conversion traits. | Ergonomic Rust: `ExactInt::from(42) * ExactInt::from(7)` works naturally. |
| A2.2 | **Error Types.** Actionable errors. Display, Error, From traits. No internal leaks. | Developer knows what went wrong from error message alone. |
| A2.3 | **Trait Hierarchy.** Implement Add/Sub/Mul/Div/Rem/Neg/Display/Debug/Clone/Hash/Eq/Ord/Serialize/Deserialize. From/Into for primitives. | All standard traits. Round-trip serialization verified. |
| A2.4 | **Developer Experience Review.** 10 realistic usage scenarios. Compare line count vs num-bigint. | No scenario requires >2× the lines of equivalent num-bigint code. |
| A2.5 | **Backward Compatibility Strategy.** Stability contract, deprecation policy, feature flags. | Policy documented in STABILITY.md. |
| A2.6 | **Versioning.** Semantic versioning + cargo-semver-checks in CI. | CI fails on unintentional breaking changes. |

### ⊘ GATE A2: API Locked [Hard] — Surface frozen for 1.0

### A3: C ABI (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A3.1 | **C Header.** `mathcore.h` with opaque pointers, ownership semantics. SQLite/libsodium patterns. | Compiles with `-Wall -Wextra -Werror` under gcc and clang. |
| A3.2 | **Rust→C Bridge.** `#[no_mangle] extern "C"` functions. catch_unwind at every boundary. | All functions callable from C. No panics escape. Valgrind clean. |
| A3.3 | **Memory Management.** Allocation/deallocation pairs. Debug allocator for leak detection. | Zero leaks under debug allocator. Double-free detected. |
| A3.4 | **Error Propagation.** Error codes + thread-local message buffer. `mathcore_last_error()`. | Every error path tested. Messages are useful. |
| A3.5 | **ABI Stability Testing.** Test on Linux (glibc/musl), macOS, Windows (MSVC/MinGW). | All platforms pass. Symbol table is stable. |

### ⊘ GATE A3: C ABI Stable [Hard] — ABI frozen

### A4: Python FFI (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A4.1 | **Binding Layer.** PyO3 or cffi implementation. | `import mathcore` works on Python 3.9+. |
| A4.2 | **Pythonic Wrapper.** snake_case, operator overloading, context managers, rich __repr__. | ExactInt behaves like Python int where possible. |
| A4.3 | **NumPy Interop.** Conversion to/from arrays. Batch operations. | Round-trip verified. Competitive with native NumPy for equivalent precision. |
| A4.4 | **Package Distribution.** manylinux/macOS/Windows wheels via cibuildwheel. PyPI publish. | `pip install mathcore` works without compiler on all platforms. |
| A4.5 | **Python Test Suite.** 200+ tests across Python 3.9–3.13. | All pass. |

### ⊘ GATE A4: Python Ready [Firm]

### A5: JavaScript/WASM FFI (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A5.1 | **WASM Compilation.** wasm-pack/wasm-bindgen. Target <500KB gzipped. | Loads in Chrome, Firefox, Safari, Node.js 18+. |
| A5.2 | **JavaScript Wrapper.** TypeScript definitions. BigInt interop. | `new ExactInt(42n).mul(new ExactInt(7n)).toString() === "1764"` |
| A5.3 | **npm Packaging.** ESM + CJS bundles. Publish pipeline. | `npm install mathcore` works. ESM and CJS resolve correctly. |
| A5.4 | **Browser + Node.js Compat.** Test Chrome, Firefox, Safari, Edge, Node 18/20/22. | All pass. |
| A5.5 | **JavaScript Test Suite.** 150+ tests in Jest/Vitest. | All pass in Node.js and browser. |

### ⊘ GATE A5: JavaScript Ready [Firm]

### A6: Documentation (6 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A6.1 | **API Reference.** rustdoc with examples for every public item. | Zero undocumented public items. |
| A6.2 | **Getting Started Guide.** Zero-to-working in 5 minutes for Rust/Python/JS. | A fresh developer succeeds on first attempt. |
| A6.3 | **Migration Guide.** From GMP, num-bigint, Python int, JS BigInt. Side-by-side code. | Top 10 operations per source library covered. |
| A6.4 | **Architecture Overview.** CRT representation, K-Elimination (conceptual), exact division's role in Clockwork Bootstrap. | Senior engineer can explain approach to a colleague after reading. |
| A6.5 | **Performance Documentation.** Honest benchmarks with methodology. Tradeoffs stated. | No unsupported claims. Methodology reproducible. |
| A6.6 | **Formal Verification Documentation.** What's proven, what's not, methodology. | Accurately represents current state. No overclaims. |

### ⊘ GATE A6: Documentation Complete [Firm]

### A7: Testing Infrastructure (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A7.1 | **Property-Based Tests.** 50+ properties (commutativity, associativity, distributivity, round-trip). | All hold for 10K random inputs each. |
| A7.2 | **Fuzz Testing Harness.** cargo-fuzz/AFL on C ABI. All entry points. | 24h clean run. Repeated weekly. |
| A7.3 | **Cross-Language Consistency.** 10K test vectors. Identical results Rust/Python/JS. | Zero discrepancies. |
| A7.4 | **Performance Regression Tests.** Criterion benchmarks in CI. Flag >5% degradation. | Regression detected automatically. |
| A7.5 | **Formal Verification CI.** Lean 4 proof checking on every merge. | Proof checking green on every CI pass. |

### ⊘ GATE A7: Testing Complete [Firm]

### A8: CI/CD and Packaging (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A8.1 | **GitHub Actions Pipeline.** Lint, test, fuzz, benchmark, docs. | PRs <15 min. Full nightly <2h. |
| A8.2 | **Multi-Platform Build Matrix.** Linux (x86_64/aarch64), macOS (arm64), Windows (x86_64). | All green on every merge. |
| A8.3 | **Automated Release.** Tag-triggered publish to crates.io/PyPI/npm. Changelog. GitHub release. | Tag `v1.0.0` → published packages within 30 min. |
| A8.4 | **Package Signing.** Sign all artifacts. Publish checksums. | All artifacts signed and verifiable. |
| A8.5 | **Reproducible Builds.** Deterministic output verification. | Two independent builds from same source → identical binaries. |

### ⊘ GATE A8: Release Pipeline Operational [Firm]

### A9: Community Launch (6 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A9.1 | **Repository Preparation.** README, LICENSE (MIT), CHANGELOG, CODE_OF_CONDUCT, SECURITY.md. | 30-second test: developer understands what/why/how instantly. |
| A9.2 | **Contributing Guidelines.** Bug reports, PRs, coding standards, proof requirements. | New contributor can follow the process. |
| A9.3 | **Issue Templates.** Bug, feature, performance, security (private). | Configured and tested. |
| A9.4 | **Example Repository.** 5+ working examples: financial calc, scientific, crypto, interop. | All compile and run unmodified. |
| A9.5 | **Launch Announcement.** [EXPANDED] Honest technical blog post. Positions K-Elimination as enabling Clockwork Bootstrap unlimited-depth FHE. | Reviewed for accuracy. No unsupported claims. |
| A9.6 | **Distribution Strategy.** HN, Reddit, Discord. Prepared FAQ. | FAQ covers anticipated questions. |

### ⊘ GATE A9: Launch Ready [Soft]

### A10: Post-Launch Stabilization (4 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| A10.1 | **Feedback Triage.** Monitor issues, categorize, respond within 24h. | No issue unacknowledged >24h. |
| A10.2 | **Bug Fix Cadence.** Critical bugs fixed 48h, patch release 72h. | No critical bug open >72h. |
| A10.3 | **Performance Optimization.** Profile real workloads, optimize hot paths. | Optimization backed by profiling data. |
| A10.4 | **API Evolution.** Track friction, plan 1.1 via RFC. | No silent breaking changes. |

### ⊘ GATE A10: Stable 1.0 [Soft]

---

## Track B: CryptKit (44 tasks, 9 gates)

### B0: Cryptographic Primitive Isolation (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B0.1 | **AHOP Extraction.** Standalone module with minimal QMNF deps (CRTBigInt via MathCore API). | Compiles and passes all tests independently. |
| B0.2 | **NINE65 Extraction.** [EXPANDED] Isolate FHE operations. Includes inventory of existing defense mechanisms (dead man's switches, fuse chain, Paranoid Mode detection tiers). Extracts Clockwork Bootstrap components as separable submodule (see B0.6). | Module independent. Dependency graph on MathCore explicit and minimal. Existing defenses catalogued with status. |
| B0.3 | **Parameter Set Definition.** [EXPANDED] Parameter sets for 128/192/256-bit security. Must satisfy Clockwork Bootstrap constraint: q_small = t. Must have ≥2 RNS limbs for CRT reconstruction in modswitch_to_t. | Each set has security margin documented. All satisfy q_small = t. |
| B0.6 | **Clockwork Bootstrap Extraction.** [NEW] Extract as distinct testable module: modswitch_to_t (K-Elimination exact division), homomorphic_inner_product (depth-1), key_switch_bootstrap, composed clockwork_bootstrap. Plus bootstrap key generation (BSK with q_small = t constraint). | Standalone module. 1,000-iteration test: encrypt → exhaust noise → bootstrap → decrypt → assert match. Post-bootstrap noise in expected range. |
| B0.7 | **Bootstrap Parameter Validation.** [NEW] For every parameter set: verify q_small = t, verify ≥2 RNS limbs, verify Q_boot accommodates inner product noise, compute and document post-bootstrap noise margin. | All sets pass. All have ≥64-bit post-bootstrap noise margin. |

### ⊘ GATE B0: Primitives Isolated [Hard]
*Previously B0.4 (Security Level Mapping) absorbed into S3.1.*

### B1: Cryptographic API Design (8 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B1.1 | **Key Generation API.** AHOP keypairs, NINE65 evaluation/encryption/relinearization keys. OS-provided entropy. Opaque key types. | Impossible to generate weak keys through misuse. |
| B1.2 | **Encryption/Decryption API.** [EXPANDED] AHOP KEM/DEM. NINE65 homomorphic encrypt/decrypt. Anti-CPA^D constraint: decryption output is never returned directly — re-encrypted under client's key before leaving computation unit. | Follows established patterns. No raw decryption output observable. |
| B1.3 | **Homomorphic Operations API.** [EXPANDED] Add/multiply/compare ciphertexts. Automatic noise tracking. Auto-bootstrap mode: when noise nears exhaustion, Clockwork Bootstrap triggers transparently. Users never see noise exhaustion errors on bootstrap-enabled ciphertexts. | 10,000+ chained multiplications succeed without manual intervention. |
| B1.4 | **Signature API.** Design now for forward compatibility even if not yet implemented. | Forward-compatible surface documented. |
| B1.5 | **Serialization Format.** Versioned binary, forward-compatible, canonical. Covers keys, ciphertexts, parameters, bootstrap keys. | Round-trip verified for all types. |
| B1.8 | **AHOP Session Key Exchange.** [NEW] Per-unit session key established via AHOP key exchange. Key derived via HKDF with domain separation. Bound to unit identity and circuit identity. Session key feeds into entropy mixing (S1.3), provisioning auth (S3.5), and NTT randomization (S3.6). | Session key requires valid AHOP keypair. Cannot be established without. |
| B1.9 | **Bootstrap API.** [NEW] Manual mode: `bootstrap(ct_exhausted, bsk)` for advanced users. Automatic mode: FHE context auto-bootstraps when noise nears threshold. Bootstrap key generation: `generate_bootstrap_key(sk, params)` enforces q_small = t. | Auto-mode chains 10,000+ multiplications without error. Impossible to generate BSK with wrong q_small. |
| B1.10 | **Bootstrap Key Lifecycle.** [NEW] BSK generated per-unit alongside working keypair. BSK too large for CPU registers → Tier 1: TEE-protected DRAM; Tier 2: IOMMU-protected DRAM; Tier 3: unprotected. BSK destroyed alongside working key during fuse exhaustion. | BSK follows same tier-specific security guarantees as working key. Verified in adversarial tests. |

### ⊘ GATE B1: Crypto API Designed [Hard]
*Previously B1.6 (Constant-Time Audit) absorbed into S3.6. B1.7 (Capsule API) deferred to post-1.0.*

### B2: Key Management (1 task)

| Task | Description | Acceptance |
|------|-------------|------------|
| B2.5 | **Key Management Best Practices.** Secret keys not Clone or Debug. Explicit lifetime management. Key rotation patterns. Secret material never in logs/errors. | Compiler prevents common misuse. |

### ⊘ GATE B2: Security Hardened [Hard]
*Gate criteria now reference S-phase work: side-channel analysis (S3.6), memory zeroization (S2.3), entropy audit (S1.2), constant-time verification (S3.6). B2.1–B2.4 absorbed into S-phase tasks.*

### B3: Legal/IP Foundation (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B3.1 | **Patent Landscape Analysis.** Search Apollonian crypto, depth-1 bootstrap, RNS division, shadow entropy. | Landscape documented. Freedom-to-operate assessment. |
| B3.2 | **Provisional Patent Filing.** AHOP, NINE65, Clockwork Bootstrap, Shadow Entropy, K-Elimination. 12-month window. | Filed within 30 days of plan start. |
| B3.3 | **Dual License Design.** AGPL v3 (open source) + commercial permissive. | License files clear. Commercial terms drafted. |
| B3.4 | **Contributor License Agreement.** Apache CLA model. | CLA template ready. |
| B3.5 | **Export Control Classification.** EAR 5D002. BIS notification for open-source crypto. | Classification determined. Notification filed if required. |

### ⊘ GATE B3: Legal Foundation Set [Hard]

### B4: CryptKit FFI Integration (4 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B4.1 | **CryptKit on MathCore C ABI.** Build exclusively against public C API. Define `cryptkit.h`. | Works everywhere MathCore works. |
| B4.2 | **Python Crypto Bindings.** Follow `cryptography.io` patterns. | Installable, Pythonic, correct. |
| B4.3 | **JavaScript Crypto Bindings.** WASM-based. Minimize secrets in JS heap. | Works in browsers and Node.js. |
| B4.4 | **Additional Language Stubs.** Go and Java/Kotlin minimal implementations. | Basic operations work. |

### ⊘ GATE B4: Multi-Language Crypto [Firm]

### B5: Test Vectors and Compliance (7 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B5.1 | **Known-Answer Test Vectors.** [EXPANDED] Deterministic KAT for all operations plus bootstrap cycle stages (pre-bootstrap, post-modswitch, post-inner-product, post-keyswitch). JSON format. | Third-party reproducible. |
| B5.2 | **NIST-Style Test Harness.** Reproducible by third parties. | Runs cleanly on clean install. |
| B5.3 | **Interoperability Test Suite.** Cross-language round-trip verification. | Rust/Python/JS produce identical results. |
| B5.4 | **Performance Benchmarks.** [EXPANDED] vs. liboqs, SEAL, TFHE-rs, lattigo. Includes bootstrap cycle latency as primary metric. | Fair comparison at same security level, same hardware. |
| B5.5 | **Algorithm Agility Tests.** Version negotiation. Old objects remain usable after updates. | Old serialized objects deserialize correctly. |
| B5.5b | **Unlimited Depth Test Harness.** [NEW] Per Clockwork Bootstrap spec: (a) random message loop, 1,000+ iterations, (b) 100+ consecutive bootstrap cycles showing no degradation, (c) edge cases (0, t-1, t/2), (d) single-limb rejection test, (e) public-mode auto-bootstrap tests. | Zero failures. Noise margin stable across 100 cycles. |
| B5.6 | **Bootstrap Benchmark Suite.** [NEW] Clockwork vs TFHE-rs per-gate bootstrap vs SEAL BFV bootstrap vs OpenFHE BGV bootstrap. Key metric: bootstrap-to-multiplication ratio (target ≈ 1). Measure depth-100 and depth-1000 total time. BSK memory overhead. | Benchmarks fair and reproducible. Results quantify advantage. |

### ⊘ GATE B5: Compliance Ready [Firm]

### B6: Documentation (6 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B6.1 | **Cryptographic Specification.** [EXPANDED] Formal spec for AHOP, NINE65, and Clockwork Bootstrap. Math definitions, pseudocode, parameter constraints, security model, reduction proofs. NIST PQC format. Includes Clockwork Bootstrap invariants (modswitch correctness, inner product correctness, noise reset, level restoration). | Complete enough for independent implementation. |
| B6.2 | **Security Model Documentation.** [EXPANDED] Adversary tier classification (S0.1 output). Per-tier guarantee matrix (RESISTED/PARTIALLY/NOT RESISTED for each property × tier). Honest proof gaps. Security claim inventory (S0.2 output). Cost asymmetry economic argument. Formal proof traceability (every "Proven" claim links to specific Lean 4/Coq file). | Honest. No overclaims. Guarantee matrix matches Response v2 §3. |
| B6.3 | **Integration Guide.** 5 use cases with complete code. "Add PQ key exchange," "Compute on encrypted data," "Unlimited depth ML inference." | Following guide produces working, secure code. |
| B6.4 | **Threat Model.** [EXPANDED] Quantum attackers, side channels, malicious servers, key compromise, hypervisor adversaries. Explicit per-tier guarantees. DRAM intermediate exposure caveat documented. | Covers realistic adversaries at all tiers. |
| B6.5 | **Competitive Positioning.** [REWRITE] Centers on unlimited depth via Clockwork Bootstrap. Comparison: NINE65 is the only system combining unlimited depth + exact arithmetic + trivial-cost bootstrap. | Fair comparison. Acknowledges where competitors have advantages (maturity, standardization). |
| B6.9 | **Unlimited Depth White Paper.** [NEW] Technical paper for engineers/CTOs: the depth problem, traditional bootstrapping cost, Clockwork Bootstrap mechanism, K-Elimination's enabling role, benchmarks, security analysis, honest limitations (BSK size, parameter constraints). 15–20 pages. | Technically rigorous. Claims backed by benchmarks and proofs. Limitations honest. |

### ⊘ GATE B6: Crypto Documentation Complete [Firm]

### B7: External Validation (4 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B7.1 | **Security Audit Scope.** [EXPANDED] Scope includes S-phase mitigations (shadow entropy redesign, anti-CPA^D, bootstrap security). Shortlist firms: NCC, Trail of Bits, Cure53. | Scope comprehensive. Budget realistic. |
| B7.2 | **Academic Papers.** [EXPANDED] Four papers: (1) AHOP construction, (2) K-Elimination theorem, (3) NINE65 system paper with unlimited depth, (4) Formal verification methodology (70+ Lean 4, 19 Coq files as first-class contribution). Proof repository cleaned and published with CI. | Papers mathematically rigorous. At least one submitted to peer-reviewed venue. |
| B7.4 | **Bug Bounty Program.** Scope: implementation bugs, cryptographic breaks, side channels. Severity levels, rewards, responsible disclosure. | Fair, meaningful rewards, clear scope. |
| B7.5 | **Clockwork Bootstrap Paper.** [NEW] Dedicated paper for CRYPTO/EUROCRYPT/PKC: depth-1 bootstrap construction, correctness proof, K-Elimination dependency, security analysis (S3.10), experimental verification of unlimited depth, performance comparison. Potentially highest-impact publication. | Mathematically rigorous. Proofs complete. Experimental methodology reproducible. |

### ⊘ GATE B7: Validation Pipeline Active [Soft]

### B8: Commercial Infrastructure (4 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| B8.1 | **Enterprise Support Tiers.** Community (free), Professional (email, 48h SLA), Enterprise (dedicated, 4h SLA). | Competitive with comparable crypto library support. |
| B8.2 | **SLA Framework.** Achievable by solo developer with AI assistance. | SLAs realistic. |
| B8.3 | **Customer Intake.** Inquiry → evaluation → licensing → onboarding → support. | Process documented end-to-end. |
| B8.4 | **Pricing Model.** Research comparable pricing. Per-seat, per-product, or annual unlimited. | Competitive. Sustainable for solo operation. |

### ⊘ GATE B8: Commercial Ready [Soft]

---

## Track S: Security Hardening (26 tasks, 6 gates)

### S0: Threat Model Formalization (2 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| S0.1 | **Adversary Tier Classification.** Three tiers: Tier 1 (Network/Software), Tier 2 (Privileged Software/Hypervisor), Tier 3 (Physical Proximity). For each: capabilities, attack vectors, NINE65 guarantees, and explicit non-guarantees. | No overclaims. Tiers align with Response v2 §3. |
| S0.2 | **Security Claim Inventory.** [EXPANDED] Enumerate every security claim. Classify as Proven/Argued/Assumed/False. Map each "Proven" claim to its specific Lean 4/Coq proof file (70+ Lean 4, 19 Coq). Establish honest documentation template enforcing "What This Does NOT Protect Against" sections. Key corrections: "shadow entropy is cryptographic-quality randomness" → reclassify per dual-regime model; "destruction receipt proves erasure" → False; "NINE65 is bootstrap-free" → False (correct claim: depth-1 Clockwork Bootstrap). | Every claim classified. Every "Proven" claim linked to proof file. All "False" claims have redesign plans. Terminology corrected throughout. |

### ⊘ GATE S0: Threat Model Formalized [Hard]

### S1: Shadow Entropy Redesign (4 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| S1.1 | **Dual-Regime Shadow Entropy Documentation.** Regime A (cryptographic): observer lacks operand knowledge; quotients proven NIST SP 800-22 compliant (ShadowNISTCompliance.lean, ShadowUniform.lean). Regime B (deterministic): observer knows parameters + operands; quotients fully predictable. Reclassify metering as Regime B. Ensure no system component uses Regime A properties where Regime B conditions hold. | Both regimes documented. Formal proofs cited for Regime A. No cross-regime confusion. |
| S1.2 | **HWRNG Integration with Health Monitoring.** [MERGED] OS CSPRNG wrapper (getrandom/BCryptGenRandom). No userspace PRNG. Handles entropy exhaustion (block or error, never fall back). Runtime health checking: detect constant RDRAND (AMD firmware bug) within 10 samples. Degrade gracefully to session-key-only mixing with logged warning. | All randomness from OS CSPRNG. Constant-RDRAND detected. Graceful degradation. |
| S1.3 | **Key-Dependent Entropy Mixing Pipeline.** Per-quotient XOR mixing with HKDF(session_key, counter) before quotients enter entropy ledger. Session key from AHOP exchange (B1.8). RDRAND mixed at each checkpoint. Adversary with all public params + observed ciphertext inputs but without session key cannot predict mixed values. | Adversary simulator: 10^6 forgery attempts, zero successes. HWRNG failure degrades gracefully (session-key mixing alone protects). |
| S1.4 | **INV-8 Augmentation.** INV-8 scoped as defense against external input attacks only (not substrate adversaries). Optional cryptographic MAC (HMAC keyed by ephemeral secret) behind feature flag for high-assurance deployments. Substrate defense comes from tier hardware (TEE/IOMMU). | External adversary: malformed ciphertext triggers INV-8 detection at first operation. MAC forgery impossible without secret key (when enabled). |

### ⊘ GATE S1: Shadow Entropy Redesigned [Hard]

### S2: Execution Environment Hardening (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| S2.1 | **VM/Hypervisor Detection.** [EXPANDED] Multi-signal detection: CPUID hypervisor bit, brand string, SMBIOS/DMI, RDTSC variance, MAC OUI, ACPI tables, PCI devices, /proc/cpuinfo. Includes defense-in-depth against DBI frameworks (code integrity hashing, timing anomaly detection, ptrace self-trace, /proc/self/maps scanning) and multi-source clock integrity (RDTSC, CLOCK_MONOTONIC_RAW, HPET, ACPI PM, cross-core TSC). Documented limitations: DBI detection is speed bump not wall (Response: "millisecond lifetime is the stronger defense"). VM detection + enforcement policy: strict (refuse) or advisory (warn). | Detects VMware/VirtualBox/Hyper-V/KVM/QEMU/Xen. DBI detects Frida/DynamoRIO/PIN/gdb in default configs. All limitations documented. |
| S2.2 | **TEE Integration.** [EXPANDED] Detect SGX/TDX/SEV-SNP via CPUID. Mandatory for Tier 1: remote attestation proving unmodified binary inside enclave. Enclave holds ephemeral key lifecycle (generation + destruction), not full FHE computation. TEE attestation freshness interval specified to prevent replay of stale attestation from compromised TEE. TEE vulnerabilities (TEE.Fail 2024, Plundervolt) documented as known limitations. | On TEE hardware: key never exits enclave. Without TEE: operates with documented reduced guarantees. Attestation replay resistance verified. |
| S2.3 | **CPU-Register Key Confinement.** [EXPANDED] TRESOR-style: secret key in AVX-512 zmm16-31 or debug registers, never DRAM. Hand-written assembly for key lifecycle. Memory zeroization: zeroize crate with compiler barriers, explicit_bzero, register zeroing via inline assembly. mlock for any key-adjacent memory. Verify with Miri, AddressSanitizer, and physical memory dump. | Key never appears in DRAM during normal execution. Memory dump post-destruction shows zero at all key locations. |
| S2.4 | **IOMMU/DMA Protection.** [EXPANDED] Mandatory for Tier 1/2 (refuse to operate if IOMMU not active). Detect Intel VT-d / AMD-Vi presence and status. Warn on Thunderbolt ports with IOMMU disabled. | IOMMU status reported accurately. Tier 1/2 refuse without active IOMMU. |
| S2.5 | **Fuse-Exhaustion Destruction Model.** [EXPANDED] Destruction is continuous fuse consumption, not discrete branch. Each Montgomery multiplication irreversibly consumes one fuse link. Unit dies by construction when fuse reaches zero — no "should I destroy?" decision to skip. Fuse counter: one-way (monotonically decrementing, not resettable) in CPU registers. Additionally: multi-path integrity verification for final cleanup (register/memory zeroing). Compiler barriers against dead-store elimination. Process calls _exit() after zeroing. Honest caveat: sophisticated glitch could corrupt fuse counter without corrupting computation (Tier 2 residual risk). | No single instruction skip prevents fuse consumption. Fuse not resettable. Zeroing survives -O3. Post-destruction memory clean. |

### ⊘ GATE S2: Execution Environment Hardened [Firm]

### S3: Cryptographic Protocol Hardening (9 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| S3.1 | **Parameter Compliance and Security Level Mapping.** [MERGED] Validate all NINE65 parameter sets against 2024 HE.org guidelines (ePrint 2024/463) using Lattice Estimator with RC.MATZOV model. Map to NIST levels 1–5. Analyze ternary secret security under hybrid attack model explicitly (HE.org: "not well understood"). Integrate Lattice Estimator as build-breaking CI gate: any PR modifying parameters must pass. Sparse secret usage documented with explicit caveat. | All sets ≥128-bit classical security. Ternary secret concrete bits estimated. CI rejects non-compliant parameters. Honest documentation of proof gaps. |
| S3.2 | **Ciphertext Input Validation.** Validate before compute: structural validity (ring dimension, component count, modulus chain), noise bound estimation (reject if exceeds fresh-encryption bound by configurable tolerance), parameter consistency, size limits. | Rejects all malformed test vectors. No false rejections. Validation cost <1% of evaluation cost. |
| S3.3 | **Anti-CPA^D Constraints.** Decryption output re-encrypted under client's public key before leaving computation unit. Error messages limited to generic enumerated codes (zero key-dependent information). Pre-computation-receipt noise flooding: all intermediate values mixed with HWRNG random values before folding, destroying noise-dependent correlations. | No decryption output directly observable. Error messages carry zero key-dependent bits. Noise-flooded receipts across 10K sessions show no key-correlated patterns. |
| S3.4 | **Computation Receipt Redesign.** Renamed from "destruction receipt." Proves: (a) genuine NINE65 instance processed declared circuit, (b) INV-8 check lane consistent at receipt time, (c) fuse fully consumed, (d) metering signal valid. Explicitly does NOT prove physical erasure (classical provable deletion is impossible). Receipt: `SHA-256(circuit_id ‖ flooded_state ‖ metering_signal ‖ nonce)`. | Receipt documentation states what it proves and explicitly what it does not. Word "proof" never used in context of erasure. |
| S3.5 | **Provisioning Protocol Security.** [EXPANDED] TLS 1.3 with mutual authentication and certificate pinning. AHOP key exchange (mandatory, not optional). Evaluation key MAC'd under TLS-session-derived key. Circuit signed by client (prevents reaction attack via circuit substitution). Per-session fresh nonce. Replay prevention. | MITM produces authentication failure. Replayed sessions rejected. Substituted circuits/keys detected. |
| S3.6 | **NTT Side-Channel Hardening.** [MERGED] Combines constant-time implementation + randomized butterfly ordering. Constant-time: no early-exit, constant-time modular reduction (cmov/csel, no branch-on-compare), constant-time discrete sampler (replace CDT with Bernoulli-based). Randomized: butterfly ordering shuffled per-unit using CSPRNG seeded with HKDF(session_key ‖ RDRAND, "ntt_permutation" ‖ level). Fisher-Yates shuffle stored in registers/cache. Mathematical result identical (butterflies at same level commute). | dudect: no timing differences at 10^6 samples. cachegrind: no data-dependent access patterns. Randomized NTT produces identical output to standard NTT for all test vectors. |
| S3.7 | **Rowhammer-Aware Memory.** Guard pages (mprotect PROT_NONE) around key memory. mlock key pages. Periodic integrity check via redundant copy (constant-time comparison). Hash destruction sequence code at startup, verify before execution. | Key memory mlocked + guarded. Code integrity verified pre-destruction. Single-bit corruption detected. |
| S3.8 | **[RESEARCH] Receipt Noise Leakage Analysis.** Formally analyze whether computation receipt, even after noise flooding, encodes information usable as partial CPA^D oracle across many observations. Apply Li-Micciancio / Checri et al. framework to receipt output. | Either: (a) prove zero key-dependent bits leak, or (b) quantify leakage rate and set operational limits. |
| S3.10 | **[NEW] Clockwork Bootstrap Security Analysis.** Four sub-analyses: (a) key-switch leakage — does post-key-switch ciphertext correlate with bootstrap secret key? (b) periodic noise reset pattern — does predictable noise periodicity create a distinguisher? (c) BSK as high-value target — BSK in DRAM exposes working key indirectly; analyze per tier. (d) K-Elimination side channels — does exact division during modswitch create observable execution profile distinct from NTT? | Each sub-analysis produces: proof of no leakage, quantified bounded leakage, or vulnerability with mitigation plan. |

### ⊘ GATE S3: Protocols Hardened [Firm]

### S4: Multi-Instance Hardening (3 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| S4.1 | **Multi-Instance Correlation Analysis.** Shared public parameters across units: do noise profiles leak across sessions? Does computation receipt enable cross-session correlation? Do traffic patterns reveal computation type? | Each surface: prove no leakage, or quantify rate with mitigation. |
| S4.2 | **Parameter Randomization.** If S4.1 finds correlation surfaces, randomize auxiliary values per-unit (same security level, different parameters). | Cross-session noise profiles statistically independent (p>0.05 on 10K sessions). |
| S4.3 | **Traffic and Covert Channel Analysis.** [MERGED] Traffic: pad messages to fixed sizes, normalize timing. Covert channels: estimate maximum bandwidth during computation window (~0.2 bits/session per adversarial review). Verify cumulative leakage across N sessions. | Traffic classifier accuracy not significantly above chance. Covert channel bounded. |

### ⊘ GATE S4: Multi-Instance Hardened [Soft]

### S5: Adversarial Tests (3 tasks)

*Most adversarial tests from Amendment 1 are now acceptance criteria on their parent tasks rather than standalone tasks. The three remaining standalone tests are integration-level tests that exercise multiple mitigations simultaneously.*

| Task | Description | Acceptance |
|------|-------------|------------|
| S5.4 | **Ciphertext Malformation Fuzzer.** Generate structurally varied malformed ciphertexts: wrong dimension, excessive noise, parameter mismatch, oversized, truncated, bit-flipped. Verify input validation gate (S3.2) catches all before computation. | 100% malformed rejected. Zero false rejections. Zero UB on adversarial input. |
| S5.5 | **CPA^D Oracle Simulation.** Simulate Li-Micciancio and Checri et al. attacks against NINE65 with all S3.3 mitigations. Attempt key recovery from: error messages, computation receipts, timing, any observable output. | Key recovery fails. Zero key-dependent bits in all observable outputs. |
| S5.10 | **[NEW] Clockwork Bootstrap Adversarial Test.** (a) Observe noise levels before/after bootstrap across 10K cycles — can adversary determine plaintext? (b) Observe periodic noise pattern across 10K sessions with same circuit — can adversary extract key? (c) Capture BSK from DRAM (Tier 2/3 scenario) — without bootstrap secret key (register-confined), does BSK alone reveal working key? | No key/plaintext recovery from bootstrap observations. BSK capture without bootstrap secret key insufficient for key recovery. |

### ⊘ GATE S5: Adversarial Tests Pass [Firm]

---

## Track X: Cross-Track Integration (5 tasks)

| Task | Description | Acceptance |
|------|-------------|------------|
| X1 | **Shared CI/CD.** Unified build scripts, test runners, release tooling for MathCore + CryptKit. | Both projects build from unified pipeline. |
| X2 | **Unified Documentation Portal.** mdBook presenting coherent MathCore + CryptKit narrative. | Coherent story across both products. |
| X3 | **MathCore→CryptKit Dependency Validation.** CryptKit CI tests against all MathCore versions. | Compatibility matrix maintained. |
| X4 | **Revenue Model Integration.** MathCore MIT (free). CryptKit AGPL + commercial. Commercial license includes MathCore rights. | Unified FAQ. No licensing confusion. |
| X5 | **Shared Security Posture.** Coordinated vulnerability disclosure. Cross-impact evaluation. | Policy covers both products. |

---

## Quality Gate Summary

### Hard Gates (must pass, no exceptions)

| Gate | Track | Critical Criteria |
|------|-------|-------------------|
| A0: Foundation Verified | A | Baselines established, gaps identified |
| A1: Core Hardened | A | Zero panics, Miri clean, fuzz clean |
| A2: API Locked | A | API frozen for 1.0 |
| A3: C ABI Stable | A | ABI frozen, all platforms |
| B0: Primitives Isolated | B | AHOP + NINE65 + Clockwork Bootstrap standalone |
| B1: Crypto API Designed | B | Key gen + encrypt + FHE ops + bootstrap API frozen |
| B2: Security Hardened | B | References S-phase gates for side channels, entropy, zeroization |
| B3: Legal Foundation | B | Patents filed, licenses designed, export classification |
| S0: Threat Model | S | All claims classified, terminology corrected |
| S1: Shadow Entropy | S | Dual-regime documented, key-dependent mixing operational |

### Firm Gates (must pass, documented exceptions allowed)

| Gate | Track | Critical Criteria |
|------|-------|-------------------|
| A4–A8 | A | Python/JS/Docs/Testing/CI ready |
| B4–B6 | B | Multi-language + test vectors + documentation |
| S2: Execution Environment | S | VM detection, TEE, register keys, IOMMU, destruction |
| S3: Protocols | S | Parameters, validation, CPA^D, receipt, provisioning, NTT, bootstrap security |
| S5: Adversarial Tests | S | Fuzzer + CPA^D sim + bootstrap adversarial |

### Soft Gates (quality targets)

| Gate | Track | Critical Criteria |
|------|-------|-------------------|
| A9–A10 | A | Launch and stabilization |
| B7–B8 | B | Validation and commercial |
| S4: Multi-Instance | S | Correlation and traffic analysis |

---

## Master DAG: Execution Layers

```
Layer 0 — Start Immediately (parallel):
  A0    Pre-Flight Assessment
  B0    Primitive Isolation (incl. Clockwork Bootstrap extraction)
  B3    Legal/IP Foundation (fully independent)
  S0    Threat Model Formalization

Layer 1 — Foundations:
  A1    Core Stabilization                    ← A0
  B1    Crypto API (incl. Bootstrap API)      ← B0, S0
  S1.1  Dual-Regime Shadow Entropy            ← S0, B0
  S1.2  HWRNG Integration                     ← S1.1
  S2.1  VM/TEE/DBI Detection                  ← S0
  S3.6  NTT Side-Channel Hardening            ← B0

Layer 2 — Design & Hardening:
  A2    API Design                            ← A1
  B2    Key Management                        ← B1
  S1.3  Key-Dependent Entropy Mixing          ← S1.2, B1.8 (AHOP session key)
  S2.2  TEE Integration                       ← S2.1
  S2.3  Register Key Confinement              ← S0, B0
  S2.4  IOMMU/DMA Protection                  ← S2.3
  S2.5  Fuse-Exhaustion Destruction           ← S2.3
  S3.1  Parameter Compliance                  ← B0.3
  S3.2  Ciphertext Input Validation           ← B0, S0
  S3.3  Anti-CPA^D                           ← S0, B1.2
  S3.5  Provisioning Protocol                 ← B1, B1.8
  S3.7  Rowhammer Mitigation                  ← S2.3

Layer 3 — Integration:
  A3    C ABI                                 ← A2
  S1.4  INV-8 Augmentation                    ← S1.3, B1
  S3.4  Computation Receipt                   ← S3.3, S1.3
  S3.10 Bootstrap Security Analysis           ← B0.6, S0, S3.3
  S4    Multi-Instance Hardening              ← S3.3, S2

Layer 4 — Bindings & Testing (parallelizable):
  A4    Python FFI                            ← A3
  A5    JavaScript FFI                        ← A3
  B4    CryptKit FFI                          ← A3, B1, B2, S2
  S5    Adversarial Test Suite                ← S1, S2, S3

Layer 5 — Documentation & Vectors:
  A6    Documentation                         ← A2, A4, A5
  A7    Testing Infrastructure                ← A2, A3, A4, A5
  B5    Test Vectors (incl. Bootstrap)        ← B1, B4, S1, S3, S5

Layer 6 — Pipeline & Crypto Docs:
  A8    CI/CD Pipeline                        ← A7
  B6    Crypto Documentation (incl. White Paper) ← B1, B4, B5, S0, S1, S3

Layer 7 — Launch & Validation:
  A9    Community Launch                      ← A6, A8
  B7    External Validation (incl. Papers)    ← B2, B5, B6, S4, S5
  X1-5  Cross-Track Integration              ← A8, B4+

Layer 8 — Post-Launch:
  A10   Stable 1.0                            ← A9
  B8    Commercial                            ← B5, B6, B7
```

**Critical Path (Track B with Bootstrap):**
```
B0.2 → B0.6 → B1.9 → B1.10 → B5.5b → B6.9 → B7.5
                                ↓
                           S3.10 → S5.10 → B7
```

**Critical Bottleneck:** A3 (C ABI) remains the single most consequential dependency — unblocks both Track A language bindings AND Track B's FFI integration. If A3 slips, both tracks shift.

**New Critical Dependency:** B1.8 (AHOP Session Key Exchange) feeds S1.3, S3.5, and S3.6. If AHOP extraction (B0.1) slips, the entire security hardening track that depends on the session key is blocked.

---

## Deferred Items

These items were evaluated and explicitly deferred, not forgotten:

| Item | Reason | When to Revisit |
|------|--------|-----------------|
| B1.7 Capsule Multi-Computation API | Post-1.0 feature | After B8 gate |
| S5.9 Randomized NTT vs ML Power Analysis | Requires physical hardware + external collaboration | When kiosk hardware available |
| S1.7 Homomorphic MAC Performance Analysis | MAC is optional; performance question is relevant only if MAC adoption is pursued | After initial Tier 1 deployment |
| Track H: Hardware Manufacturing | Physical Faraday cage, hardware watchdog, analog tamper detection, firmware attestation, physical key erasure circuits | When NINE65 moves from software to kiosk production |

---

## Terminology Corrections (Global)

Applied throughout this document; must be applied to all ancillary materials:

| Old | New |
|-----|-----|
| Bootstrap-free FHE | Depth-1 bootstrap FHE / Clockwork Bootstrap FHE |
| Destruction receipt | Computation receipt |
| Shadow entropy (unqualified) | Shadow entropy Regime A (cryptographic) or Regime B (metering) |
| Proof of destruction / proof of erasure | Proof of computation (erasure claims removed) |

---

## Reduction Summary

| Metric | Base Plan | +Amend 1 | +Amend 2 | +Amend 3 | Consolidated |
|--------|-----------|----------|----------|----------|-------------|
| Track A tasks | 57 | 57 | 57 | 57 | **57** |
| Track B tasks | 45 | 45 | 50 | 58 | **44** |
| Security tasks | 0 | 38 | 48 | 50 | **26** |
| Cross-Track tasks | 5 | 5 | 5 | 5 | **5** |
| **Total tasks** | **106** | **144** | **159** | **169** | **132** |
| Quality gates | 20 | 26 | 26 | 26 | **26** |

**37 tasks eliminated** via merges (redundant S5 tests → acceptance criteria), absorptions (B2.1-B2.4 into S-phase equivalents), deferrals (Capsule API, hardware-dependent research), and consolidations (traffic/covert channel merged, S0.3/S0.4 into S0.2).

---

*This document is the single source of truth for the QMNF execution plan. The base plan and Amendments 1–3 are superseded.*
