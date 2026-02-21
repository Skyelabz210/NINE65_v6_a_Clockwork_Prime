# A0.5 — Dependency and Build Audit

**Plan Task**: A0.5 — Dependency/Build Audit
**Status**: COMPLETE
**Date**: 2026-02-19

---

## Summary

The NINE65 v6 workspace has 7 crates, 14 unique direct runtime dependencies, 115 total resolved packages (Cargo.lock), 1 unsafe block (justified), and `cargo-deny` configured. The dependency surface is minimal for a cryptographic workspace.

---

## Workspace Structure

| Crate | Path | Runtime Deps | Zero-Dep? |
|-------|------|-------------|-----------|
| `nine65` | `crates/nine65/` | 5 core + 12 optional | No |
| `clockwork-core` | `crates/clockwork-core/` | 2 (`subtle`, `crc32fast`) | No |
| `exact_transcendentals` | `crates/exact_transcendentals/` | 0 | YES |
| `nexgen_rational` | `crates/nexgen_rational/` | 0 | YES |
| `fhe-service` | `crates/fhe-service/` | 6 | No |
| `mana` | `crates/mana/` | 1 (`zeroize`) + optional | No |
| `unhal` | `crates/unhal/` | 1 (`mana`) + optional | No |

Excluded from workspace: `fuzz`, `nine65-python`, `nine65-wasm`

---

## Direct Runtime Dependencies (14 unique)

| Dependency | Version | Used By | Category |
|------------|---------|---------|----------|
| `zeroize` | 1.7 (derive) | nine65, mana | Crypto (key erasure) |
| `getrandom` | 0.2 | nine65, fhe-service | Crypto (OS CSPRNG) |
| `subtle` | 2.5 | nine65, clockwork-core | Crypto (constant-time) |
| `sha2` | 0.10 | nine65 | Crypto (hashing) |
| `thiserror` | 1.0 | nine65, fhe-service | Error handling |
| `rayon` | 1.10 | nine65, mana, unhal (optional) | Parallelism |
| `rand_core` | 0.6 | nine65 (optional) | RNG traits |
| `rand_chacha` | 0.3 | nine65 (optional) | Deterministic RNG |
| `serde` | 1.0 | nine65, fhe-service (optional) | Serialization |
| `serde_json` | 1.0 | nine65, fhe-service (optional) | JSON |
| `bincode` | 2.0 / 1.3 | nine65 / fhe-service | Binary encoding |
| `crc32fast` | 1.3 | clockwork-core, nine65 (clockwork) | Integrity |
| `base64` | 0.22 | fhe-service | Encoding |
| `wide` | 0.7 | mana (optional, disabled) | SIMD |

### Version Conflict

`bincode` appears at **two incompatible major versions**: v2.0 (nine65/serde feature) and v1.3 (fhe-service). These have incompatible APIs. `cargo deny` warns but does not block. If `fhe-service` ever needs to deserialize data from nine65's bincode serde, this will fail silently.

**Recommendation**: Migrate fhe-service to bincode 2.0 for consistency.

---

## Transitive Dependency Chains

Key transitive chains:
- `sha2` -> `digest` -> `crypto-common` -> `generic-array` -> `typenum`
- `sha2` -> `cpufeatures`
- `rayon` -> `rayon-core` -> `crossbeam-deque` -> `crossbeam-epoch` -> `crossbeam-utils`
- `serde_json` -> `itoa`, `ryu` (note: `ryu` is float-to-string; transitive only, not used by nine65)
- `thiserror` -> `thiserror-impl` (proc-macro)
- `zeroize` -> `zeroize_derive` (proc-macro with `derive` feature)

**Total resolved packages**: 115 (Cargo.lock)

---

## Unsafe Code Audit

**Total unsafe blocks in workspace**: 1

| File | Line | Purpose | Justified? |
|------|------|---------|-----------|
| `clockwork-core/src/key_lifecycle.rs` | 274 | `core::ptr::write_volatile` for key material zeroing | YES — prevents dead-store elimination (Formal Spec A4) |

The single unsafe block is:
- Minimal (single volatile write)
- Well-documented with safety comment
- Followed by `SeqCst` fence
- Required to prevent compiler optimization of key zeroing

### `#![forbid(unsafe_code)]` Crates

| Crate | Forbids Unsafe? |
|-------|----------------|
| nine65 | YES (`lib.rs:10`) |
| mana | YES (`lib.rs:16`) |
| nexgen_rational | YES (`lib.rs:13`) |
| clockwork-core | No (has 1 justified unsafe) |
| exact_transcendentals | No (but no unsafe found) |
| fhe-service | No (but no unsafe found) |
| unhal | No (but no unsafe found) |

---

## Panic/Unwrap/Expect in Production Code

### Category A: OS CSPRNG Failures (Correct — Unrecoverable)

| File | Pattern | Count |
|------|---------|-------|
| `keys/mod.rs` | `.expect("CRITICAL: OS CSPRNG failure...")` | 4 |
| `ops/encrypt.rs` | `.expect("CRITICAL: OS CSPRNG failure...")` | 1 |
| `entropy/wassan_noise.rs` | `.expect("CRITICAL: OS entropy...")` | 1 |
| `entropy/shadow.rs` | `.expect("CRITICAL: OS CSPRNG...")` | 1 |

These are correct — OS entropy failure is unrecoverable in a cryptographic system.

### Category B: API Wrapper Panics (All Have `try_*` Counterparts)

| File | Panicking fn | Fallible Alternative |
|------|-------------|---------------------|
| `ops/encrypt.rs` | `encode()` | `try_encode()` |
| `ops/encrypt.rs` | `encode_vector()` | `try_encode_vector()` |
| `ops/encrypt.rs` | `encrypt_*()` | `try_encrypt_*()` |
| `ops/homomorphic.rs` | `relinearize()` | `try_relinearize()` |

### Category C: Mutex `.unwrap()` in `shadow_entropy_monitor.rs`

~8 call sites. Safe under `panic = "abort"` release profile (poison is impossible since panicking threads abort the process).

### Category D: Structural Invariants

| File | Pattern | Risk |
|------|---------|------|
| `ring/pool.rs` | `.expect("PoolGuard invariant")` | LOW — violated only on memory corruption |
| `params/primes.rs` | `panic!("No primitive root found")` | LOW — mathematically unreachable for valid primes |
| `params/validation.rs` | `panic!("INVALID FHE PARAMETERS!")` | LOW — assertion function by design |

### Summary

~25-30 panic/unwrap/expect sites in production code. All are either:
1. OS CSPRNG failure (correct to abort)
2. Wrapper with fallible alternative (correct API design)
3. Invariant checks (unreachable for valid inputs)
4. Mutex unwrap (safe under panic=abort)

---

## Supply-Chain Audit Configuration

`deny.toml` is present and configured:

| Setting | Value | Notes |
|---------|-------|-------|
| `vulnerability` | `deny` | Known CVEs block builds |
| `unmaintained` | `warn` | |
| `yanked` | `warn` | |
| `unlicensed` | `deny` | No unlicensed deps |
| `multiple-versions` | `warn` | Will warn on bincode dual-version |
| `unknown-registry` | `warn` | |
| `allow-git` | `[]` | No git deps allowed |

Allowed licenses: MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, Unicode-3.0, BSL-1.0, ISC, Unicode-DFS-2016

---

## Build Configuration

### Release Profile

```toml
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

- `panic = "abort"`: Eliminates unwinding attack surface, reduces binary size
- `lto = "fat"`: Cross-crate optimization, important for crypto performance
- `codegen-units = 1`: Maximum optimization at cost of compile time

### No build.rs or proc-macros in workspace crates

All proc-macros come from external deps (`thiserror-impl`, `zeroize_derive`).

### Feature Flags (nine65)

21 features defined. Defaults: `ntt_fft`, `parallel`. Key production features:
- `shadow-entropy`: CRT shadow entropy harvester
- `clockwork`: Clockwork-Core integration
- `exact_rational`: NexGen rational bridge
- `allow_insecure`: Test configs (NOT for production)

---

## Findings and Recommendations

| Finding | Severity | Recommendation |
|---------|----------|----------------|
| bincode v1.3/v2.0 dual-version | MEDIUM | Migrate fhe-service to bincode 2.0 |
| `ryu` (float-to-string) in transitive deps | LOW | From serde_json; not used by nine65 directly |
| `exact_transcendentals` missing `#![forbid(unsafe_code)]` | LOW | Add for completeness |
| `fhe-service` missing `#![forbid(unsafe_code)]` | LOW | Add for completeness |
| `unhal` missing `#![forbid(unsafe_code)]` | LOW | Add for completeness |
| `primes.rs` panic could be Result | LOW | Optional refactor, unreachable for valid inputs |

---

## Acceptance Criteria

- [x] All 7 workspace crates audited
- [x] Runtime dependency inventory complete (14 unique direct deps)
- [x] Unsafe code cataloged (1 block, justified)
- [x] Panic/unwrap/expect sites classified
- [x] Supply-chain config (`deny.toml`) verified
- [x] Build profile documented
- [x] Feature flags inventoried
- [x] Version conflicts identified (bincode)
