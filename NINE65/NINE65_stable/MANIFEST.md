# QMNF FHE Transfer Pack - File Manifest
## December 20, 2024

### Source Tarball
```
qmnf_fhe_src.tar.gz    132KB    Complete source (no build artifacts)
```

### Key Code Files (for updating existing codebase)

| File | Lines | Purpose |
|------|-------|---------|
| `code/Cargo.toml` | 24 | Dependencies: zeroize, getrandom, subtle, sha2 |
| `code/lib.rs` | 274 | Main lib with security, kat modules |
| `code/entropy_secure.rs` | 170 | **NEW** OS CSPRNG wrapper |
| `code/entropy_mod.rs` | 35 | Updated exports for secure entropy |
| `code/entropy_shadow.rs` | 255 | Shadow Entropy + from_os_seed() |
| `code/keys_mod.rs` | 382 | Zeroize + generate_secure APIs |
| `code/ring_polynomial.rs` | 260 | Zeroize impl for RingPolynomial |
| `code/params_mod.rs` | 405 | HE Standard 128 config |
| `code/noise_budget.rs` | 340 | **NEW** Millibit noise tracking |
| `code/security_mod.rs` | 280 | **NEW** LWE security estimation |
| `code/kat.rs` | 320 | **NEW** Known Answer Tests |

### Scripts

| File | Purpose |
|------|---------|
| `scripts/lwe_estimate.py` | External LWE estimator (uses lattice-estimator) |

### Documentation

| File | Purpose |
|------|---------|
| `docs/SESSION_REPORT.md` | Complete session summary |
| `docs/EXECUTION_PLAN.md` | 47-action plan with status |
| `docs/CRYPTO_AUDIT_REPORT.md` | Security audit findings |
| `TRANSFER_GUIDE.md` | This instantiation guide |

---

## Installation Paths

When copying individual files, use these target paths:

```
code/Cargo.toml          → ./Cargo.toml
code/lib.rs              → ./src/lib.rs
code/entropy_secure.rs   → ./src/entropy/secure.rs    (NEW FILE)
code/entropy_mod.rs      → ./src/entropy/mod.rs
code/entropy_shadow.rs   → ./src/entropy/shadow.rs
code/keys_mod.rs         → ./src/keys/mod.rs
code/ring_polynomial.rs  → ./src/ring/polynomial.rs
code/params_mod.rs       → ./src/params/mod.rs
code/noise_budget.rs     → ./src/noise/budget.rs      (NEW FILE)
code/security_mod.rs     → ./src/security/mod.rs      (NEW DIR/FILE)
code/kat.rs              → ./src/kat.rs               (NEW FILE)
scripts/lwe_estimate.py  → ./scripts/lwe_estimate.py  (NEW DIR/FILE)
```

### Directory Creation Commands

```bash
mkdir -p src/security
mkdir -p scripts
```

---

## Verification Commands

```bash
# After installation
cargo build --release          # Should compile with 4 warnings
cargo test --release           # Should show 171 passed
cargo test --release secure    # 9 tests
cargo test --release budget    # 9 tests
cargo test --release kat       # 4 tests
```

---

## Git Diff Summary

If applying to existing repo, these are the changes:

### Modified Files
- `Cargo.toml` - Added 4 dependencies
- `src/lib.rs` - Added security, kat modules + prelude exports
- `src/entropy/mod.rs` - Added secure exports
- `src/entropy/shadow.rs` - Added from_os_seed()
- `src/keys/mod.rs` - Added Zeroize, generate_secure()
- `src/ring/polynomial.rs` - Added Zeroize impl
- `src/params/mod.rs` - Added he_standard_128(), tests

### New Files
- `src/entropy/secure.rs` (170 lines)
- `src/noise/budget.rs` (340 lines)
- `src/security/mod.rs` (280 lines)
- `src/kat.rs` (320 lines)
- `scripts/lwe_estimate.py` (180 lines)

### New Directories
- `src/security/`
- `scripts/`
