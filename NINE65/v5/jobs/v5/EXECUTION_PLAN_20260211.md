# Plan: FHE Microservice — Isolate Python Bindings

## Context

`nine65-python` (pyo3 0.28 cdylib) has 81+ compilation errors that break `cargo check --workspace`. Rather than chasing pyo3 API churn, we restructure the Python integration as a microservice: the Rust FHE engine runs as an HTTP service, and Python talks to it via a thin SDK client. This isolates external binding dependencies from the core workspace permanently.

## Architecture

```
Core Workspace (always green)          External Clients
┌──────────────────────────────┐       ┌─────────────────┐
│ nine65, clockwork-core,      │       │ sdks/python/     │
│ nexgen_rational, mana, unhal │       │   nine65_sdk/    │
│ exact_transcendentals        │       │   (pure HTTP)    │
│ fhe-service (HTTP boundary)  │◄──────┤                  │
└──────────────────────────────┘       └─────────────────┘

Excluded (standalone builds):
  crates/nine65-python/  (pyo3)
  crates/nine65-wasm/    (wasm-bindgen)
```

---

## Completion Status (re-baselined 2026-02-11)

| Step | Description | Status |
|------|-------------|--------|
| 1 | Workspace isolation (Cargo.toml exclude) | **DONE** |
| 2 | fhe-service Cargo.toml dep updates | **DONE** |
| 3 | Module extraction (http.rs, wire.rs, session.rs, handlers.rs) | **DONE** (1713 LOC) |
| 4 | Python SDK scaffold | **DONE** (client, session, ciphertext, errors) |
| 5 | REST endpoints (healthz, version, metrics, sessions CRUD, encrypt, decrypt, evaluate) | **DONE** |
| 6 | Body size + connection limits (HVT-4) | **DONE** |
| 7 | Noise budget tracking in evaluate ops | **DONE** (add, sub, negate, mul, add_plain, mul_plain) |
| 8 | Prometheus `/v1/metrics` endpoint | **DONE** |
| 9 | Batch throughput benchmarks | **DONE** (throughput.rs) |

**Workspace compiles clean:** `cargo check --workspace --release` passes (0 errors, 2 warnings in clockwork-core).

---

## Remaining Work (re-baselined 2026-02-11 — post-execution verification)

**Context:** The prior plan (CF-1 through Q-5) was executed by a separate AI session. Cross-verification confirms 8/10 checked items are DONE. A parallel reanalysis (Codex deep-planning-audit) identified a new axis: benchmark claim credibility gaps. Both tracks are merged below.

### VERIFIED COMPLETE — Prior Consultant Findings

| Item | Status | Verified by |
|------|--------|-------------|
| CF-1 | **DONE** — compile_error! in fhe-service/main.rs:8-9 | Opus cross-check |
| CF-2 | **DONE** — noise_budget.consume in handle_encrypt:227-231 | Opus cross-check |
| CF-3 | **DONE** — single unix_now_seconds in main.rs:141 | Opus cross-check |
| CF-4 | **DONE** — per-session Arc<RwLock<Session>> | Opus cross-check |
| CF-5 | **DONE** — all wire fields renamed to _estimate_millibits | Opus cross-check |
| CF-6 | **DONE** — rescale credit in handlers.rs:416-424 | Opus cross-check |
| H-1 | **DONE** — TTL reaper every 60s, default 3600s | Opus cross-check |
| H-2 | **DONE** — README matches 9 endpoints | Opus cross-check |

### STILL OPEN — cf-1 placement issue

#### CF-1b: `allow_insecure` compile guard in nine65 lib.rs (5 min)
**Source:** Opus cross-verification
**Issue:** CF-1 was placed in `fhe-service/main.rs`, not `nine65/lib.rs`. This means the guard only protects the service binary. A downstream consumer using `nine65` as a library can still enable `allow_insecure` in release builds.
**Fix:** Add the same compile_error! to `crates/nine65/src/lib.rs`

#### CF-2: Encrypt handler missing noise budget consumption (1 hr)
**Source:** Opus (Finding R2)
**Issue:** `handle_encrypt` in `handlers.rs:194-233` calls `encryptor.encrypt_secure(v)` but never calls `session.noise_budget.consume()`. Method `NoiseBudget::encrypt_cost()` exists at `budget.rs:171` but is never invoked. Budget is optimistic from first operation.
**Fix:** Add `session.noise_budget.consume(NoiseOpType::Encrypt, NoiseBudget::encrypt_cost(&session.config))` for each value encrypted.

#### CF-3: Duplicate `unix_now_seconds` (15 min)
**Source:** Opus (Finding R3)
**Issue:** Two identical implementations: `main.rs:111` (pub crate) and `session.rs:36` (private). Session.rs version should import from crate root.
**Fix:** Remove `session.rs:36` version, use `crate::unix_now_seconds()` in Session::new/new_test.

#### CF-4: SessionStore global RwLock bottleneck (4 hr)
**Source:** Codex + Opus (consensus)
**Issue:** `SessionStore` uses single `RwLock<HashMap<String, Session>>`. Any encrypt/decrypt/evaluate takes a write lock on the ENTIRE map via `with_session_mut`, serializing all FHE ops across all sessions.
**Fix:** Replace with per-session locking: `HashMap<String, Arc<RwLock<Session>>>`. Outer HashMap only needs write lock for session create/delete, not per-operation mutations.

#### CF-5: Noise budget is heuristic, not empirical — document clearly (1.5 hr)
**Source:** Opus (Finding R1), Codex R2 (wire rename migration)
**Issue:** `NoiseBudget` uses static worst-case estimates (e.g., `add_cost()` returns constant 1000 millibits). GSO-FHE tracks actual basin evolution in core, but this isn't plumbed through to REST. Clients may make incorrect depth decisions.
**Fix:** Rename wire field from `noise_budget_millibits` to `noise_budget_estimate_millibits` in all wire types. Add doc comment: "Upper-bound heuristic estimate; actual noise may be lower."
**SDK migration:** Rename breaks Python SDK at `client.py:161` and `session.py:64`. Update SDK in same changeset. Temporarily support both field names in SDK (read `noise_budget_estimate_millibits` with fallback to `noise_budget_millibits`) for one deprecation window.

#### CF-6: Noise budget doesn't credit rescaling in mul path (1 hr)
**Source:** Opus (Finding R5)
**Issue:** `handle_evaluate` mul path consumes `mul_ct_cost` + `relin_cost` but doesn't apply `rescale_cost` (which is negative — a budget gain). If K-Elimination rescaling is applied in the core evaluator path, the budget is pessimistic.
**Fix:** Audit whether `BFVEvaluator::mul_no_relin + relinearize` applies K-Elimination rescaling. If yes, add corresponding negative noise consumption.

### HIGH — Hardening

#### H-1: Session TTL + reaper (4 hr)
**Source:** All 3 consultants (consensus)
**Issue:** No session expiry. Memory exhaustion attack: create sessions indefinitely up to `max_sessions`, each holding ~200MB key material, sessions never expire.
**Fix:** Add `ttl_seconds: u64` to SessionStore config (default 3600). Background cleanup on a timer or lazy eviction on access. Add `last_accessed: u64` to Session, update on every operation.

#### H-2: README endpoint documentation drift (30 min)
**Source:** Codex
**Issue:** `crates/fhe-service/README.md` still advertises old `/v1/fhe/*` endpoints while code uses `/v1/sessions/*`.
**Fix:** Update README to match current routing in handlers.rs.

#### H-3: `Connection: close` → HTTP keep-alive (1 hr)
**Source:** Opus (Raw obs. 9)
**Issue:** Server sends `Connection: close` on every response (`http.rs:151`), forcing new TCP connection per request. For encrypt → evaluate → decrypt flows, this triples connection overhead.
**Fix:** Support `Connection: keep-alive` in HTTP parser, reuse connections.

#### H-4: Base64 payload size vs body limit validation (30 min)
**Source:** Opus (Raw obs. 6)
**Issue:** Each ciphertext for N=4096 with 3 primes is ~100KB in bincode → ~133KB in base64. Batch ops (1024 values) could yield ~130MB JSON — exceeding the 10MB body limit. Clients get a confusing error.
**Fix:** Validate batch size against body limit early. Return clear error: "batch size N would exceed maximum body size".

#### H-5: CSPRNG panic → graceful 500 (30 min)
**Source:** Adversarial sweep (entropy exhaustion)
**Issue:** `getrandom().expect()` at `entropy/secure.rs:127` panics if OS entropy pool is unavailable (container cold start, fork bomb, `/dev/urandom` blocked). In fhe-service, this hard-crashes the entire process — instant DoS.
**Fix:** Wrap `getrandom()` in `Result`, propagate error through key generation and encryption paths. fhe-service returns HTTP 500 with generic message instead of crashing. Add test: service survives entropy failure gracefully.

#### H-6: Protocol-correct parse error mapping (30 min)
**Source:** Codex R2 (Finding 3)
**Issue:** Server maps ALL parse errors to generic 400 Bad Request (`main.rs:97`), including oversized bodies. HTTP semantics require 413 Payload Too Large for body exceeding `MAX_BODY_BYTES`. Clients cannot distinguish malformed JSON from size violations.
**Fix:** Match on `HttpParseError::BodyTooLarge` variant (already exists in `http.rs:24`) and return 413 with `Content-Length-exceeds-limit` message. All other parse errors remain 400.

#### H-7: Response amplification governance (1 hr)
**Source:** Codex R2 (Finding 4)
**Issue:** Encrypt allows up to 1024 values (`wire.rs:125`) and returns ciphertext arrays (`wire.rs:58`) with no response-body ceiling. A single encrypt request for 1024 values can generate ~130MB response, creating memory and bandwidth pressure under load.
**Fix:** Add `MAX_RESPONSE_BYTES` constant. Preflight-reject encrypt/evaluate batches where estimated response size exceeds limit. Return 413 with clear message: "response would exceed maximum size; reduce batch count".

#### H-8: Request-side memory amplification cap (1 hr)
**Source:** Adversarial sweep (memory exhaustion)
**Issue:** Decrypt/evaluate accept 1024 ciphertexts x 512KB each (`wire.rs:140-145`) = **512MB per request**. 64 concurrent sessions = 32GB potential memory. H-7 caps response size but request-side allocation within body limit is uncapped.
**Fix:** Reduce `MAX_CIPHERTEXT_FIELD_LEN` or add aggregate allocation cap per request. Validate `count * estimated_ct_size < MAX_REQUEST_ALLOCATION` before deserialization. Consider streaming base64 decoder to avoid full materialization.

#### H-9: Serde unwrap → proper error handling (15 min)
**Source:** Adversarial sweep (panic safety)
**Issue:** 5 response serialization calls use `.unwrap()` at `handlers.rs:168,181,230,263,410`. While `serde_json::to_value()` on well-typed structs shouldn't fail, a panic here crashes the connection handler thread.
**Fix:** Replace all 5 `unwrap()` calls with `.map_err(|e| ...)` returning HTTP 500. Defense-in-depth pattern.

### MEDIUM — Quality

#### Q-1: Python SDK integration test validation (2 hr)
**Source:** Opus (Finding U2)
**Issue:** Python SDK exists but isn't mentioned in the original plan's verification matrix. Wire type changes will break SDK consumers.
**Fix:** Add Python SDK roundtrip test to CI. Verify SDK against any wire field renames (e.g., CF-5 rename).

#### Q-2: Error message + error oracle security pattern (1.5 hr)
**Source:** Opus (Finding U3), Codex R2 (evaluate error leaks), Adversarial sweep (error oracle)
**Issue:** Two problems:
1. Evaluate errors at `handlers.rs:336,355` include actual plaintext modulus value in error messages.
2. **Error oracle**: `NoiseExhausted` returns **422** while all other failures return **400** (`handlers.rs:412-422`). Attacker can distinguish valid-but-exhausted ciphertexts from invalid ones — classic adaptive CCA vector against a decryption-as-a-service.
**Fix:**
- Scrub `t`, `q`, and key material from all error responses.
- **Uniform status codes**: Return 400 for ALL operation failures (encrypt, decrypt, evaluate). Remove 422 for NoiseExhausted — it leaks ciphertext validity.
- Unify error messages to generic "operation failed" for decrypt/evaluate; only provide detail for clearly client-error cases (missing fields, bad JSON).
- Add tests: `evaluate_error_does_not_leak_plaintext_modulus`, `decrypt_error_does_not_leak_key_material`, `error_status_codes_are_uniform`.

#### Q-3: RwLock poison recovery documentation (15 min)
**Source:** Opus (Raw obs. 5)
**Issue:** `unwrap_or_else(|e| e.into_inner())` pattern on RwLock silently recovers from panics, potentially leaving sessions in inconsistent state. This is intentional but undocumented.
**Fix:** Add doc comment on `with_session`/`with_session_mut` explaining the design choice.

#### Q-4: Decrypt path constant-time audit (2 hr)
**Source:** Adversarial sweep (timing side-channel)
**Issue:** `decrypt_raw()` at `encrypt.rs:501` applies secret key via `mul()` (non-constant-time), not `mul_ct()`. Key generation correctly uses `mul_ct()`, but decryption does not. In a decryption-as-a-service context, this is a timing side-channel: response latency could leak information about secret key coefficients.
**Fix:** Audit the full decrypt call chain. Replace `ct.c1.mul(&sk.s, ntt)` with `ct.c1.mul_ct(&sk.s, ntt)` in the decrypt path. Add timing-invariance test (multiple decryptions of different ciphertexts should have consistent latency).

#### Q-5: Document IND-CPA security boundary (30 min)
**Source:** Adversarial sweep (CCA analysis)
**Issue:** System provides IND-CPA security (standard BFV). The HTTP service functions as a decryption oracle by design. For single-server symmetric mode, CPA is sufficient. But this must be explicitly documented so downstream consumers don't assume CCA protection. No noise flooding or OAEP wrapping is implemented.
**Fix:** Add security model documentation to fhe-service README: "This service provides IND-CPA security. The decryption endpoint should NOT be exposed to untrusted clients in production without additional application-layer authentication. CCA defenses (noise flooding, ciphertext authentication) are deferred to multiparty mode."

---

## Implementation Order (remaining work — merged tracks)

### Track A: Fix Broken Claims (IMMEDIATE — before any publication)

```
F-1:  Regenerate README security estimates from baseline     [30 min]  CRITICAL
F-2:  Fix README test count inconsistency (627 vs 640)       [15 min]  CRITICAL
F-3:  Fix README benchmark command (→ benchmark_symmetric_)  [5 min]   CRITICAL
F-4:  Reclassify FHE_BENCHMARK_COMPARISON.md (test config)   [1 hr]    CRITICAL
F-5:  Fix failing add_plain test (message mismatch)          [15 min]  CRITICAL
F-6:  Add CF-1 compile_error! to nine65/lib.rs (not just svc)[5 min]   CRITICAL
```

### Track B: Remaining Service Hardening

```
S-1:  H-5: getrandom → Result (not .expect())               [30 min]  HIGH
S-2:  H-9: Replace 5 serde unwrap() in handlers.rs           [15 min]  HIGH
S-3:  Verify H-3/H-4/H-6/H-7/H-8 status                    [1 hr]    HIGH
S-4:  Q-2: Uniform error codes (remove 422 → all 400)        [30 min]  HIGH
S-5:  Q-4: Decrypt path mul → mul_ct (constant-time)         [30 min]  HIGH
S-6:  Q-5: Document IND-CPA boundary in fhe-service README   [15 min]  HIGH
```

### Track C: Benchmark Credibility Infrastructure

```
B-1:  Profile policy (secure = claims, light = internal)     [30 min]  HIGH
B-2:  Fix 0ns artifacts (Criterion for K-Elim ops)           [2 hr]    HIGH
B-3:  Unify baseline gen (Criterion JSON where available)    [2 hr]    HIGH
B-4:  Mandatory env metadata in baseline scripts             [1 hr]    HIGH
B-5:  Claim registry (README claim → artifact mapping)       [2 hr]    HIGH
B-6:  CI benchmark-check: advisory → tiered blocking         [2 hr]    HIGH
```

### Track D: Documentation Reconciliation

```
D-1:  Stale-claim scanner (CI fails on metric drift)         [3 hr]    MEDIUM
D-2:  Benchmark statistics policy (10 runs, CV<15%)          [30 min]  MEDIUM
D-3:  Nightly fuzz CI job (encrypt_decrypt + k_elimination)  [1 hr]    MEDIUM
D-4:  Comparator manifest (OpenFHE, SEAL, parameter map)     [1 hr]    MEDIUM
D-5:  Depth-correctness matrix (JSON from depth benchmarks)  [2 hr]    MEDIUM
```

### Execution Log (2026-02-11, current run)

Completed:
- `F-1` README security estimate table aligned to `docs/LATTICE_ESTIMATOR_BASELINE_2026-02-09.md`.
- `F-2` README test-count inconsistency fixed (`627` -> `640`).
- `F-3` README depth benchmark command fixed to `benchmark_symmetric_max_depth_secure_{128,192}`.
- `F-4` `docs/FHE_BENCHMARK_COMPARISON.md` reclassified as exploratory/non-claim with secure-vs-test profile policy.
- `F-5` failing `fhe-service` add-plain hardening test fixed (aligned to hardened generic evaluate error contract).
- `F-6` compile-time release guard added in `crates/nine65/src/lib.rs` for `allow_insecure`.
- `S-2` replaced handler JSON serialization `unwrap()` paths with explicit 500 error handling.
- `S-1` service-critical CSPRNG panic path converted to `Result`:
  - Added fallible secure entropy APIs (`try_secure_*`) in `nine65::entropy::secure`.
  - Added fallible keygen APIs (`SecretKey/PublicKey/EvaluationKey/KeySet::try_generate_secure`).
  - Added fallible secure encryption path (`BFVEncryptor::try_encrypt_poly_secure` / hardened `try_encrypt_secure`).
  - Updated `fhe-service` to use fallible keygen/encrypt paths and return HTTP errors instead of crashing.
- `S-4` evaluate error contract validated as uniform `400` with generic message.
- `S-5` decrypt path constant-time status validated (`mul_ct` in `decrypt_raw`).

Partially completed:
- Legacy compatibility wrappers (`secure_*`, `generate_secure`, `encrypt_secure`, `from_os_seed`) still intentionally panic on entropy failure for non-fallible call sites outside the service path; full API-wide migration to `Result` signatures is deferred.

Validation run in this execution:
- `cargo test -p fhe-service --tests -- --nocapture` (19 passed)
- `cargo test -p nine65 noise::budget::tests -- --nocapture` (7 passed)
- `cargo test -p nine65 --lib --release ops::gso_fhe::depth_benchmarks::benchmark_symmetric_max_depth_secure_128 -- --nocapture` (passed)
- `cargo test -p nine65 --lib --release ops::gso_fhe::depth_benchmarks::benchmark_symmetric_max_depth_secure_192 -- --nocapture` (passed)
- `cargo check --workspace --release` (passed)
- Negative guard check: `cargo check -p nine65 --release --features allow_insecure` (fails with compile_error as intended)

Incremental update (2026-02-11, later pass):
- `S-3` completed for remaining hardening checks:
  - `H-3` implemented: per-connection request loop now honors keep-alive semantics in `serve_connection`.
  - `H-6` confirmed: `BodyTooLarge` parse errors map to HTTP 413.
  - `H-7` confirmed: response amplification guard remains in encrypt/evaluate/decrypt paths.
  - `H-8` confirmed: aggregate request allocation cap remains enforced in wire validation.
- `S-4` finalized: decrypt/evaluate operation failures now return generic non-leaky error messages.
- `B-1` completed: benchmark profile policy added (`docs/BENCHMARK_PROFILE_POLICY.md`).
- `B-4` completed: baseline scripts now emit mandatory env metadata (UTC timestamp, CPU, toolchain, commit hash).
- `B-5` completed: claim registry added (`docs/CLAIM_REGISTRY.csv`) and linked from README.
- `B-6` completed (initial tiered gate): CI benchmark-check now runs advisory mode on PRs and enforced mode on schedule/main, plus registry validation via `scripts/check_claim_registry.sh`.

Validation run in incremental pass:
- `cargo test -p fhe-service --tests` (22 passed)
- `bash scripts/check_claim_registry.sh` (passed)

Incremental update (2026-02-11, benchmark integrity pass):
- `B-2` completed: K-Elimination divider operations moved to Criterion benchmarking path (`crates/nine65/benches/timing.rs`) to remove timer-resolution `0 ns` artifacts from claim workflows.
- `B-3` completed: `scripts/generate_performance_baseline.sh` now generates:
  - `docs/PERFORMANCE_BASELINE_YYYY-MM-DD.md`
  - `docs/PERFORMANCE_BASELINE_YYYY-MM-DD.json`
  - `docs/PERFORMANCE_BASELINE_YYYY-MM-DD_criterion.json`
  using Criterion JSON where available.
- `D-1` completed: stale claim scanner implemented (`scripts/check_stale_claims.sh`) and wired into CI (`claim-drift-check` job) to fail on README/artifact metric drift.
- Claim artifacts re-baselined to the latest generated performance baseline:
  - registry updated to `docs/PERFORMANCE_BASELINE_2026-02-11.md`
  - README and comparison docs reconciled to current measured values.

Validation run in benchmark integrity pass:
- `cargo bench -p nine65 --bench timing --features benchmarks -- --list` (build + benchmark registration passed)
- `bash scripts/generate_performance_baseline.sh` (passed; zero-time guard passed)
- `python3 scripts/extract_criterion_summary.py --criterion-root target/criterion --out /tmp/criterion_check.json` (passed)
- `bash scripts/check_claim_registry.sh` (passed)
- `bash scripts/check_stale_claims.sh` (passed)

**Total remaining: ~30 hours** across 4 tracks, 23 items
**Track A (claims): ~2.5 hr | Track B (hardening): ~3 hr | Track C (benchmarks): ~9.5 hr | Track D (docs): ~7.5 hr**

Prior items verified DONE: CF-1 through CF-6, H-1, H-2 (8 items, ~12 hr of prior plan executed by separate session)

---

## Gap Analysis

### API Coverage Gaps

| Feature | nine65-python had | Plan covers | nine65 has | Action |
|---------|:-:|:-:|:-:|--------|
| BFV encrypt/decrypt | Y | Y | Y | **DONE** |
| add/sub/negate | Y | Y | Y | **DONE** |
| mul_plain/add_plain | Y | Y | Y | **DONE** |
| mul (relinearize) | Y (deprecated) | Y | Y | **DONE** |
| batch encrypt/decrypt | Y | Y | Y | **DONE** |
| **TrackedEvaluator** | - | Y | Y | **DONE** (NoiseBudget in session) |
| **GSO-FHE depth-50** | - | Y | Y | Needs `gso_mul` evaluate op |
| **NoiseBudget tracking** | - | Y | Y | **DONE** (needs CF-2, CF-5, CF-6 fixes) |
| Galois/SIMD rotations | - | - | Y | Defer Phase 2 |
| BatchEncoder (slot packing) | - | - | Y | Defer Phase 2 |
| ParallelEncryptor/Decryptor | - | - | Y | Defer Phase 2 |
| Dual-RNS / K-Elimination | - | - | Y | Defer Phase 2 |
| FHENeuralEvaluator | - | - | Y | Defer Phase 3 |
| Depth-aware config presets | - | - | Y | Defer Phase 2 |

### Security Gaps

| Severity | Issue | Status |
|----------|-------|--------|
| **Critical** | Unbounded body size (DoS) | **DONE** (MAX_BODY_BYTES in http.rs) |
| **Critical** | Unbounded thread spawn (DoS) | **DONE** (MAX_CONNECTIONS in main.rs) |
| **Critical** | `allow_insecure` in release builds | **CF-1** (compile_error! guard) |
| **High** | Global RwLock serializes all sessions | **CF-4** (per-session locking) |
| **High** | No session TTL / memory exhaustion | **H-1** (TTL + reaper) |
| **High** | Noise budget missing encrypt cost | **CF-2** (consume on encrypt) |
| **Medium** | Noise field name implies measurement | **CF-5** (rename to _estimate_) |
| **High** | CSPRNG panic crashes service | **H-5** (graceful 500) |
| **High** | Request memory amplification (512MB/req) | **H-8** (allocation cap) |
| **Medium** | Error oracle via HTTP status differentiation | **Q-2** (uniform status codes) |
| **Medium** | Decrypt path not constant-time | **Q-4** (mul → mul_ct audit) |
| **Medium** | Serde unwrap panics in handlers | **H-9** (proper error handling) |
| **Medium** | IND-CPA boundary undocumented | **Q-5** (security model docs) |
| ~~Medium~~ | ~~No `#![deny(clippy::float_arithmetic)]`~~ | **DONE** (already at `main.rs:6`) |
| **Low** | Duplicate unix_now_seconds | **CF-3** (remove session.rs copy) |
| Note | No TLS | Out of scope — deploy behind reverse proxy |
| Note | No replay protection | Out of scope — symmetric mode only |
| Note | No per-IP rate limiting | Out of scope — deploy behind reverse proxy |

---

## REST API (current, live)

| Method | Path | Purpose | Status |
|--------|------|---------|--------|
| GET | `/healthz` | Health check + active_sessions | **DONE** |
| GET | `/v1/version` | Version + supported configs | **DONE** |
| GET | `/v1/metrics` | Prometheus metrics | **DONE** |
| POST | `/v1/sessions` | Create session (keygen) | **DONE** |
| GET | `/v1/sessions/{id}` | Session info (noise, ops) | **DONE** |
| DELETE | `/v1/sessions/{id}` | Destroy session (zeroize) | **DONE** |
| POST | `/v1/sessions/{id}/encrypt` | Encrypt value(s) | **DONE** |
| POST | `/v1/sessions/{id}/decrypt` | Decrypt ciphertext(s) | **DONE** |
| POST | `/v1/sessions/{id}/evaluate` | Homomorphic ops | **DONE** (add,sub,negate,mul,add_plain,mul_plain) |

---

## Consultant Cross-Reference

| Finding | Codex | Opus | Gemini | Mapped To |
|---------|:-----:|:----:|:------:|-----------|
| Plan staleness / already-done items | Y | Y | Y | Re-baselined above |
| `allow_insecure` compile guard | Y | Y | Y | CF-1 |
| SessionStore RwLock bottleneck | Y | Y | - | CF-4 |
| Session TTL/reaper | Y | Y | Y | H-1 |
| Encrypt missing noise consume | - | Y | - | CF-2 |
| Noise budget is estimate not measurement | - | Y | - | CF-5 |
| Rescale credit missing from mul | - | Y | - | CF-6 |
| Duplicate unix_now_seconds | - | Y | - | CF-3 |
| README endpoint drift | Y | - | - | H-2 |
| Base64 payload vs body limit | - | Y | - | H-4 |
| Connection: close overhead | - | Y | - | H-3 |
| Parse error 400 vs 413 mapping | Y | - | - | H-5 |
| Response amplification (no ceiling) | Y | - | - | H-6 |
| Evaluate error leaks plaintext modulus | Y | - | - | Q-2 |
| `#![deny(clippy::float_arithmetic)]` exists | Y | - | - | Resolved (stale gap) |
| Wire rename needs SDK migration | Y | - | - | CF-5 (SDK migration) |
| Python SDK in test matrix | - | Y | - | Q-1 |
| CSPRNG panic = service crash | - | - | - | H-5 (adversarial sweep) |
| Error oracle via status codes | - | - | - | Q-2 (adversarial sweep) |
| Request memory amplification | - | - | - | H-8 (adversarial sweep) |
| Serde unwrap panics | - | - | - | H-9 (adversarial sweep) |
| Decrypt path not constant-time | - | - | - | Q-4 (adversarial sweep) |
| IND-CPA boundary undocumented | - | - | - | Q-5 (adversarial sweep) |
| Thread-per-connection scaling | - | Y | - | Phase 2 (async runtime) |
| O(log N) rotation keys | - | Y | - | Phase 2 (Galois) |
| Replay protection | - | - | - | Phase 2 (multiparty only) |
| Per-IP rate limiting | - | - | - | Operational (reverse proxy) |

---

## Critical Files

- `crates/nine65/src/lib.rs` — CF-1: add compile_error! for allow_insecure
- `crates/nine65/src/entropy/secure.rs` — H-5: wrap getrandom in Result
- `crates/nine65/src/ops/encrypt.rs` — Q-4: decrypt mul → mul_ct
- `crates/fhe-service/src/handlers.rs` — CF-2: encrypt noise consume, CF-6: rescale credit, H-9: unwrap→map_err, Q-2: uniform error status
- `crates/fhe-service/src/session.rs` — CF-3: remove duplicate fn, CF-4: per-session lock
- `crates/fhe-service/src/wire.rs` — CF-5: rename noise_budget field, H-8: allocation cap
- `crates/fhe-service/src/main.rs` — H-1: TTL config, H-3: keep-alive, H-6: 413 error mapping
- `crates/fhe-service/src/http.rs` — H-3: keep-alive, H-4: batch size check, H-7: response size guard
- `crates/fhe-service/README.md` — H-2: endpoint docs, Q-5: IND-CPA security model
- `sdks/python/` — Q-1: integration test, field rename sync

## Phase 2 Roadmap (deferred)

- Galois automorphisms + SIMD slot rotations (with O(log N) key generation per Opus F4)
- BatchEncoder for N/2 value packing per ciphertext
- ParallelEncryptor/ParallelDecryptor (Rayon throughput)
- Dual-RNS / K-Elimination direct exposure
- Depth-aware config presets (`for_depth(n)`)
- Exact rational noise tracking (`exact_rational` feature)
- Async runtime migration (tokio or thread pool — replaces thread-per-connection)
- API versioning strategy for wire type evolution
- Per-operation latency histograms + lock-wait metrics
- Audit logging for privileged operations (decrypt, session-create/delete)
