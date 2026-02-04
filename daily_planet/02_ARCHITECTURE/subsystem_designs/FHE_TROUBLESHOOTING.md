# FHE Troubleshooting Guide

**System**: QMNF FHE (ACC - Axiom-Crystalline Cryptosystem)
**Version**: 1.0
**Date**: 2025-11-11

---

## Quick Diagnostic Checklist

Before diving into specific errors, run this quick diagnostic:

```bash
# 1. Verify Python tests pass
cd /path/to/QMNF_System
python3 tests/python/fhe_comprehensive_test.py

# 2. Check for float contamination
python3 tools/check_no_floats.py

# 3. Run Rust tests
cd hcvlang
cargo test --release fhe

# 4. Check example works
cargo run --release --example fhe_demo
```

**All passing?** Your FHE system is healthy. Continue to specific issues below.

**Something failed?** See relevant section for your error.

---

## Common Issues and Solutions

### 1. "Noise Budget Exhausted" / Decryption Fails

**Symptoms**:
- Decrypted result is incorrect or garbage
- Error message: "Noise budget exhausted, bootstrapping required"
- Ciphertext shows `noise_budget_bits < 10`

**Root Cause**: Too many homomorphic operations consumed the noise budget

**Background**:
Every FHE ciphertext has a "noise budget" - essentially, how much computation it can withstand before decryption fails:
- **Fresh ciphertext**: ~120 bits
- **After addition**: -1 to -2 bits per addition
- **After multiplication**: -10 to -15 bits per multiplication
- **Decryption fails**: When < 10 bits remain

**Solutions**:

#### Solution 1: Reduce Circuit Depth
```rust
// ❌ BAD: Too many multiplications
let result = ct1.clone();
for _ in 0..15 {
    result = ctx.mul(&result, &ct2, &eval_key)?; // Will fail!
}

// ✅ GOOD: Stay within budget (~10 multiplications)
let result = ct1.clone();
for _ in 0..8 {
    result = ctx.mul(&result, &ct2, &eval_key)?; // OK
}
```

#### Solution 2: Use Bootstrapping (when available)
```rust
// Refresh noise budget (requires bootstrap implementation)
if ctx.estimate_noise_magnitude(&ciphertext) > threshold {
    ciphertext = ctx.bootstrap(&ciphertext, &secret_key)?;
}
```

**Note**: Bootstrap is currently stubbed (GAP-001). Use circuit depth reduction until implemented.

#### Solution 3: Increase Security Level
```rust
// Higher security = larger noise budget
let ctx = FHEContext::new(SecurityLevel::Bit192); // ~200 bits instead of ~120
```

**Trade-off**: 2x slower operations, 2x memory usage

#### Solution 4: Monitor Noise Budget
```rust
fn compute_with_monitoring(ctx: &FHEContext, ct: &Ciphertext) {
    let noise_bits = ctx.estimate_noise_magnitude(ct);
    println!("Noise budget remaining: {} bits", noise_bits);

    if noise_bits < 20 {
        eprintln!("WARNING: Low noise budget, consider bootstrapping");
    }
}
```

---

### 2. "Dimension Mismatch" Errors

**Symptoms**:
- `assertion failed: ct1.ct0.dimension == ct2.ct0.dimension`
- `DimensionMismatch: expected 4096, got 2048`

**Root Cause**: Mixing ciphertexts/keys from different security levels or contexts

**Example of WRONG usage**:
```rust
let ctx1 = FHEContext::new(SecurityLevel::Bit128); // N = 4096
let ctx2 = FHEContext::new(SecurityLevel::Bit192); // N = 8192

let (sk1, pk1) = ctx1.generate_keypair();
let (sk2, pk2) = ctx2.generate_keypair();

let ct1 = ctx1.encrypt(&ctx1.encode(10), &pk1);
let ct2 = ctx2.encrypt(&ctx2.encode(20), &pk2);

// ❌ ERROR: Dimension mismatch!
let result = ctx1.add(&ct1, &ct2);
```

**Solutions**:

#### Solution 1: Use Same Context
```rust
// ✅ GOOD: Single context for all operations
let ctx = FHEContext::new(SecurityLevel::Bit128);
let (sk, pk) = ctx.generate_keypair();

let ct1 = ctx.encrypt(&ctx.encode(10), &pk);
let ct2 = ctx.encrypt(&ctx.encode(20), &pk);
let result = ctx.add(&ct1, &ct2); // OK!
```

#### Solution 2: Re-encrypt with Correct Keys
```rust
// If you have ciphertexts from different contexts, decrypt and re-encrypt
let plaintext = ctx_old.decrypt(&ct_old, &sk_old);
let ct_new = ctx_new.encrypt(&plaintext, &pk_new);
```

#### Solution 3: Validate Before Operating
```rust
fn safe_add(ctx: &FHEContext, ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext, FHEError> {
    if ct1.ct0.dimension != ct2.ct0.dimension {
        return Err(FHEError::DimensionMismatch {
            expected: ct1.ct0.dimension,
            got: ct2.ct0.dimension,
            operation: "add".to_string(),
        });
    }
    Ok(ctx.add(ct1, ct2))
}
```

---

### 3. Slow Performance / "FHE is too slow"

**Symptoms**:
- Encryption takes > 10ms per operation
- Multiplication takes > 100ms
- Real-time targets (< 1ms) not met

**Diagnostic**:
```bash
# Run performance benchmarks
python3 tests/python/fhe_comprehensive_test.py

# Expected performance (release mode):
# - Encryption: ~50K ops/sec (~0.02 ms/op)
# - Decryption: ~1.7M ops/sec (~0.0006 ms/op)
# - Addition: ~1.8M ops/sec
# - Multiplication: ~1.1M ops/sec
```

**Solutions**:

#### Solution 1: Enable Release Mode
```bash
# ❌ BAD: Debug mode (10-100x slower)
cargo build
cargo run --example fhe_demo

# ✅ GOOD: Release mode (optimized)
cargo build --release
cargo run --release --example fhe_demo
```

**Python equivalent**:
```python
# Ensure hcvlang was built in release mode
cd hcvlang
cargo build --release
```

#### Solution 2: Enable SIMD Features
```toml
# Cargo.toml
[features]
default = ["fast-paths", "simd", "parallel"]
simd = []  # Enable AVX2/SSE2 optimizations
parallel = ["rayon"]  # Multi-threading
```

```bash
cargo build --release --features simd,parallel
```

#### Solution 3: Use Batch Operations
```rust
// ❌ SLOW: Individual operations
for i in 0..1000 {
    let ct = ctx.encrypt(&ctx.encode(i), &pk);
    results.push(ct);
}

// ✅ FAST: Batch operations (use real-time FHE)
use hcvlang::fhe_realtime::RealTimeFHEContext;
let rt_ctx = RealTimeFHEContext::new(SecurityLevel::Bit128);

// Batch encrypt with SIMD
let ciphertexts = rt_ctx.batch_encrypt(&plaintexts, &pk)?;
```

#### Solution 4: Profile and Optimize
```bash
# Profile with perf
cargo build --release
perf record --call-graph=dwarf ./target/release/examples/fhe_demo
perf report

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --example fhe_demo
```

**Common bottlenecks**:
- **NNT transform**: Should be O(n log n), check if naive O(n²) is being used
- **Modular reduction**: Use optimized ModInt for Mersenne primes
- **Key generation**: Can be slow, consider caching keys

---

### 4. "Cannot Find hcvlang Module" (Python)

**Symptoms**:
- `ImportError: No module named 'hcvlang'`
- `ModuleNotFoundError: No module named 'hcvlang_pyo3'`

**Root Cause**: Python bindings not built or not in PYTHONPATH

**Solutions**:

#### Solution 1: Build Python Bindings
```bash
cd hcvlang
maturin develop --release  # For development
# OR
maturin build --release    # For distribution wheel
pip install target/wheels/hcvlang-*.whl
```

#### Solution 2: Check PYTHONPATH
```bash
# Add hcvlang to Python path
export PYTHONPATH=/path/to/QMNF_System/hcvlang/target/release:$PYTHONPATH

# Verify
python3 -c "import hcvlang; print(hcvlang.__file__)"
```

#### Solution 3: Use pip install (editable mode)
```bash
cd hcvlang
pip install -e .
```

---

### 5. "Float Contamination Detected"

**Symptoms**:
- `check_no_floats.py` reports float usage
- `FloatContaminationError` in tests

**Root Cause**: Floating-point operations in FHE code path (violates QMNF integer-only principle)

**Solutions**:

#### Solution 1: Use QMNFRational Instead of float
```python
# ❌ BAD: Float arithmetic
result = 0.5 * value

# ✅ GOOD: Rational arithmetic
from qmnf_boundary_fixed import QMNFRational
result = QMNFRational(1, 2) * value
```

#### Solution 2: Use Scaled Integers
```rust
// ❌ BAD: f64 for fractional values
let error_stddev: f64 = 3.2;

// ✅ GOOD: Scaled u64 (16-bit fractional precision)
let error_stddev_scaled: u64 = (3.2 * 65536.0) as u64; // 209715
let actual_stddev = error_stddev_scaled as f64 / 65536.0; // Only for display
```

#### Solution 3: Isolate Float Conversions
```python
# All float conversions must go through conversion boundary
from qmnf.conversion_boundary import DataBoundary

# Convert external float input
float_input = 3.14159
rational = DataBoundary.float_to_rational(float_input)

# Use rational in FHE operations
ciphertext = ctx.encrypt_rational(rational, public_key)
```

---

### 6. "Cargo Build Fails" (Rust)

**Common Build Errors**:

#### Error: "could not compile `hcvlang`"
```bash
error[E0433]: failed to resolve: use of undeclared crate or module `rayon`
```

**Solution**: Add missing dependency
```bash
cargo add rayon
# OR edit Cargo.toml:
[dependencies]
rayon = "1.7"
```

#### Error: "linker 'cc' not found"
**Solution**: Install build tools
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows
# Install Visual Studio with C++ tools
```

#### Error: "cannot find -lpython3"
**Solution**: Install Python development headers
```bash
# Ubuntu/Debian
sudo apt-get install python3-dev

# macOS
brew install python@3.11

# Windows
# Reinstall Python with "Development libraries" option
```

---

### 7. "Tests Fail with Incorrect Results"

**Symptoms**:
- `assert_eq!(decrypted, 42)` fails
- Homomorphic operations give wrong results
- Non-deterministic failures

**Diagnostic**:
```rust
#[test]
fn debug_decryption() {
    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = ctx.generate_keypair();

    let plaintext = ctx.encode(42);
    let ciphertext = ctx.encrypt(&plaintext, &pk);

    // Check noise budget
    let noise_bits = ctx.estimate_noise_magnitude(&ciphertext);
    println!("Noise budget: {} bits", noise_bits);

    let decrypted = ctx.decrypt(&ciphertext, &sk);
    let result = ctx.decode(&decrypted);

    println!("Expected: 42, Got: {}", result);
    assert_eq!(result, 42);
}
```

**Solutions**:

#### Solution 1: Check Noise Budget
```rust
if !ctx.can_decrypt(&ciphertext) {
    eprintln!("ERROR: Noise budget exhausted!");
    // See Section 1 for solutions
}
```

#### Solution 2: Verify Parameters
```rust
// Ensure consistent parameters
assert_eq!(ciphertext.ct0.modulus, ctx.params().ciphertext_modulus);
assert_eq!(ciphertext.ct0.dimension, ctx.params().ring_dimension);
```

#### Solution 3: Check for Overflow
```rust
// IntPair encoding has limits
let max_value = ctx.params().plaintext_modulus / 2;
if value > max_value {
    eprintln!("WARNING: Value {} exceeds plaintext modulus", value);
}
```

---

### 8. "Serialization / Saving Keys Fails"

**Symptoms**:
- Cannot save keys to disk
- Cannot reload ciphertexts
- `serde` errors

**Root Cause**: Serialization support is being added (GAP-014)

**Workaround (until serde support is complete)**:

#### Manual Serialization
```rust
use std::fs::File;
use std::io::Write;

fn save_secret_key(sk: &SecretKey, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write dimension
    file.write_all(&(sk.s.dimension as u64).to_le_bytes())?;

    // Write modulus
    file.write_all(&sk.s.modulus.to_le_bytes())?;

    // Write coefficients
    for coeff in &sk.s.coeffs {
        file.write_all(&coeff.value().to_le_bytes())?;
    }

    Ok(())
}

fn load_secret_key(path: &str) -> std::io::Result<SecretKey> {
    // Implementation left as exercise
    // Read dimension, modulus, coefficients and reconstruct
    unimplemented!()
}
```

**Better solution**: Wait for GAP-014 completion (serde support)

---

### 9. Memory Usage Issues

**Symptoms**:
- Out of memory errors
- Excessive RAM usage (>8GB for simple operations)
- Memory leaks

**Diagnostic**:
```bash
# Monitor memory usage
/usr/bin/time -v cargo run --release --example fhe_demo

# Look for:
# - Maximum resident set size
# - Page faults
```

**Solutions**:

#### Solution 1: Use Lower Security Level
```rust
// ❌ MEMORY-INTENSIVE: Bit256
let ctx = FHEContext::new(SecurityLevel::Bit256); // N=16384, ~2GB per key

// ✅ MEMORY-EFFICIENT: Bit128
let ctx = FHEContext::new(SecurityLevel::Bit128); // N=4096, ~512MB per key
```

#### Solution 2: Clear Intermediate Ciphertexts
```rust
// ❌ BAD: Keeps all intermediate results
let mut results = Vec::new();
for i in 0..1000 {
    let ct = ctx.encrypt(&ctx.encode(i), &pk);
    results.push(ct); // Memory grows!
}

// ✅ GOOD: Process and discard
for i in 0..1000 {
    let ct = ctx.encrypt(&ctx.encode(i), &pk);
    // Process ct immediately
    let result = ctx.decrypt(&ct, &sk);
    // ct is dropped here
}
```

#### Solution 3: Use Streaming Operations
```rust
// Instead of loading all data at once, stream it
fn process_large_dataset(ctx: &FHEContext, data: &[i64], pk: &PublicKey) {
    for chunk in data.chunks(100) {
        let ciphertexts: Vec<_> = chunk.iter()
            .map(|&val| ctx.encrypt(&ctx.encode(val), pk))
            .collect();

        // Process chunk
        // ...

        // Ciphertexts dropped when chunk goes out of scope
    }
}
```

---

### 10. "Example Code Doesn't Work"

**Symptoms**:
- Examples from documentation fail
- Code from FHE_TOUR.md doesn't compile
- Import errors

**Solutions**:

#### Solution 1: Check Rust Version
```bash
rustc --version  # Should be 1.70+
cargo --version

# Update if needed
rustup update stable
```

#### Solution 2: Verify Feature Flags
```bash
# Some examples require specific features
cargo run --release --example fhe_demo --features simd,parallel
```

#### Solution 3: Check Python Version
```python
import sys
print(sys.version)  # Should be 3.9+
```

#### Solution 4: Report Documentation Issues
If examples in documentation don't work:
1. Check if there's an erratum in README
2. Try the example from `hcvlang/examples/` instead
3. Report issue with details

---

## Advanced Debugging

### Enable Debug Logging
```rust
// In your code, enable logging
env_logger::init();
log::debug!("Noise budget: {}", noise_bits);
```

```bash
# Run with logging
RUST_LOG=debug cargo run --release --example fhe_demo
```

### Verify NNT Correctness
```rust
#[test]
fn verify_nnt_transform() {
    use crate::nnt::{nnt, innt};

    let input = vec![1, 2, 3, 4];
    let transformed = nnt(&input);
    let recovered = innt(&transformed);

    assert_eq!(input, recovered, "NNT round-trip failed!");
}
```

### Check Polynomial Multiplication
```rust
#[test]
fn verify_polynomial_mul() {
    let ring = PolynomialRing::from_params(&params);
    let p1 = ring.sample_ternary();
    let p2 = ring.sample_ternary();

    // Naive O(n²) multiplication
    let result_naive = p1.mul_naive(&p2);

    // NNT O(n log n) multiplication
    let result_nnt = p1.clone() * p2.clone();

    assert_eq!(result_naive, result_nnt, "Multiplication mismatch!");
}
```

---

## Getting Help

### Self-Service Resources
1. **FHE_TOUR.md**: Comprehensive guide with examples
2. **INTEGER_ONLY_DESIGN.md**: Architecture documentation
3. **FHE_DELIVERABLES_INDEX.md**: Complete deliverables overview
4. **SYSTEM_DEVELOPER_GUIDE.md**: Component integration

### Community Support
- **GitHub Issues**: https://github.com/Skyelabz210/QMNF_System/issues
- **Email**: founder@hackfate.us

### Reporting Bugs
When reporting issues, include:
1. **Environment**: OS, Rust version, Python version
2. **Minimal reproducible example**
3. **Expected vs actual behavior**
4. **Error messages** (full stack trace)
5. **Performance metrics** (if relevant)

**Template**:
```markdown
## Environment
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Python: 3.11.4
- QMNF version: [git hash or release]

## Issue
[Description]

## Steps to Reproduce
```rust
// Code here
```

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happens]

## Error Message
```
[Full error trace]
```
```

---

## Performance Benchmarking

### Expected Performance Targets

| Operation | Target | Acceptable | Poor |
|-----------|--------|------------|------|
| **Encryption** | > 50K ops/sec | > 10K ops/sec | < 1K ops/sec |
| **Decryption** | > 1M ops/sec | > 100K ops/sec | < 10K ops/sec |
| **Hom. Add** | > 1M ops/sec | > 100K ops/sec | < 10K ops/sec |
| **Hom. Mult** | > 100K ops/sec | > 10K ops/sec | < 1K ops/sec |

**If below "Acceptable"**: See Section 3 (Performance Issues)

### Run Comprehensive Benchmarks
```bash
# Python benchmarks
python3 tests/python/fhe_comprehensive_test.py

# Rust benchmarks (when available)
cargo bench --bench fhe_benchmark

# Real-time FHE benchmarks
cargo run --release --example realtime_fhe_demo
```

---

## Known Limitations

1. **Bootstrap not implemented** (GAP-001)
   - Limited to ~10 multiplications per ciphertext
   - Workaround: Decrypt and re-encrypt periodically

2. **No constant-time operations** (GAP-009)
   - Vulnerable to timing attacks in adversarial environments
   - Mitigation: Deploy in trusted single-tenant environments

3. **No division operation**
   - FHE doesn't natively support division
   - Workaround: Use multiplicative inverse or approximations

4. **Ciphertext size**
   - Each ciphertext is ~32KB (for N=4096)
   - Storage/bandwidth considerations for large datasets

5. **No automatic parallelization**
   - Must manually use batch operations for SIMD
   - Future: Automatic parallelization of independent operations

---

## Changelog

**Version 1.0** (2025-11-11):
- Initial troubleshooting guide
- Covers 10 common issue categories
- Added debugging techniques

**Future additions**:
- Constant-time operation guidance (after GAP-009)
- Bootstrap usage patterns (after GAP-001)
- Serialization examples (after GAP-014)

---

**Maintained by**: founder@hackfate.us
**Last Updated**: 2025-11-11
**Feedback**: Report gaps/errors via GitHub Issues
