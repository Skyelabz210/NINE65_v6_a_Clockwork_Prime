# Gap Resolution Manifest
## Priority-Ordered Action Items

**Generated:** December 16, 2025  
**Source:** GAP_ANALYSIS_REPORT.md  
**Scope:** 9 Critical + 25 High Priority Gaps

---

## SPRINT 0: CRITICAL BLOCKERS (P0)
**Timeline:** Immediate (before any production use)  
**Total Effort:** ~30 hours

### 1. GAP-I-0005: AVX-512 Modular Reduction [4h]

**Problem:** Code uses `_mm512_rem_epi64` which doesn't exist.

**Resolution:**
```rust
// WRONG: Non-existent intrinsic
let result = _mm512_rem_epi64(values, moduli);

// CORRECT: Barrett reduction using real intrinsics
fn barrett_reduce_avx512(values: __m512i, moduli: __m512i, mu: __m512i) -> __m512i {
    unsafe {
        let q = _mm512_mulhi_epu64(values, mu);  // high part of values * mu
        let q_m = _mm512_mullo_epi64(q, moduli); // q * modulus
        let r = _mm512_sub_epi64(values, q_m);   // values - q*m
        
        // Conditional subtraction if r >= m
        let mask = _mm512_cmpge_epu64_mask(r, moduli);
        _mm512_mask_sub_epi64(r, mask, r, moduli)
    }
}
```

**Verification:** `cargo build --features avx512` compiles successfully.

---

### 2. GAP-I-0007: WebGPU 64-bit Integer [8h]

**Problem:** WGSL only guarantees 32-bit integers.

**Resolution:** Use 32-bit prime configuration:
```wgsl
// 32-bit NTT-friendly primes
const PRIMES_32: array<u32, 12> = array<u32, 12>(
    998244353u,  // 2^23 * 119 + 1
    985661441u,  // 2^22 * 235 + 1
    754974721u,  // 2^24 * 45 + 1
    167772161u,  // 2^25 * 5 + 1
    469762049u,  // 2^26 * 7 + 1
    // ... 7 more primes
);

fn barrett_reduce_32(value: u32, modulus: u32, mu: u32) -> u32 {
    let q = (u64(value) * u64(mu)) >> 32u;
    var r = value - u32(q) * modulus;
    if (r >= modulus) { r = r - modulus; }
    return r;
}
```

**Verification:** `navigator.gpu.createShaderModule()` validates shader.

---

### 3. GAP-S-0002: Rayon Non-Determinism [16h]

**Problem:** Work-stealing scheduler violates determinism requirement.

**Resolution Options:**

**Option A: Prove Order-Independence (8h)**
- Prove mathematically that all CRT operations are commutative/associative
- Document that Rayon path is safe for CRT operations
- Restrict Rayon to order-independent operations only

**Option B: Deterministic Alternative (16h)**
```rust
// Replace work-stealing with fixed partitioning
fn parallel_crt_deterministic<T, F>(data: &mut [T], f: F)
where
    F: Fn(&mut T) + Sync,
{
    let chunk_size = data.len() / num_cpus::get();
    std::thread::scope(|s| {
        for chunk in data.chunks_mut(chunk_size) {
            s.spawn(|| {
                for item in chunk {
                    f(item);
                }
            });
        }
    });
}
```

**Option C: Document Non-Determinism (2h)**
- Mark Rayon path as "performance mode" (not reproducible)
- Add `--deterministic` flag for exact reproduction

**Verification:** 1000 identical runs produce identical output.

---

### 4. GAP-X-0001: API Key Exposure [2h]

**Problem:** House-party skill lacks key management documentation.

**Resolution:**
```rust
// BAD: Hardcoded keys
let api_key = "sk-1234567890abcdef";

// GOOD: Environment variables
let api_key = std::env::var("GROK_API_KEY")
    .expect("GROK_API_KEY environment variable not set");

// BETTER: Secret manager integration
let api_key = secrets::get("grok-api-key")
    .await
    .expect("Failed to retrieve API key from secret manager");
```

**Documentation additions:**
```markdown
## API Key Management

### Required Environment Variables
- `GROK_API_KEY` - Grok API access key
- `GEMINI_API_KEY` - Google Gemini API key
- `PERPLEXITY_API_KEY` - Perplexity API key
- `ANTHROPIC_API_KEY` - Claude API key

### Key Rotation
Keys should be rotated every 90 days. Update via:
```bash
export GROK_API_KEY="new-key-value"
```

### Production Deployment
Use a secret manager (AWS Secrets Manager, HashiCorp Vault, etc.)
```

**Verification:** `grep -r "sk-" . | wc -l` returns 0.

---

## SPRINT 1: HIGH PRIORITY (P1)
**Timeline:** This sprint  
**Total Effort:** ~103 hours

### Mathematical Formalization (64h)

**GAP-M-0001: K-Elimination Lean Proof [40h]**
```lean
-- Target Lean 4 formalization
theorem k_elimination 
  (M A : ℕ) 
  (hcoprime : Nat.gcd M A = 1)
  (X : ℕ) 
  (hrange : X < M * A)
  (v_M : ℕ := X % M)
  (v_A : ℕ := X % A)
  (M_inv : ℕ := Nat.modInverse M A) :
  (X / M) = ((v_A - v_M) * M_inv) % A := by
  sorry  -- Full proof to be developed
```

**GAP-M-0003: Residue Learning Convergence [24h]**
- Invoke theorem-crusher on gradient descent in Z/mZ
- Establish convergence conditions
- Bound distance to optimum

### Implementation Fixes (13h)

**GAP-I-0003: Montgomery Even Modulus [1h]**
```rust
pub fn montgomery_mul(a: u64, b: u64, m: u64) -> Result<u64, MontgomeryError> {
    if m & 1 == 0 {
        return Err(MontgomeryError::EvenModulus(m));
    }
    // ... rest of implementation
}
```

**GAP-I-0004: NTT Primitive Root Validation [2h]**
```rust
fn is_primitive_root(g: u64, p: u64) -> bool {
    // g is primitive root iff g^((p-1)/q) ≠ 1 for all prime factors q of p-1
    let factors = prime_factors(p - 1);
    for q in factors {
        if mod_pow(g, (p - 1) / q, p) == 1 {
            return false;
        }
    }
    true
}

// Validate at module load
#[cfg(debug_assertions)]
fn validate_ntt_primes() {
    assert!(is_primitive_root(3, 998244353), "Invalid primitive root for 998244353");
    assert!(is_primitive_root(3, 7340033), "Invalid primitive root for 7340033");
}
```

**GAP-I-0006: ARM NEON Documentation [4h]**
```markdown
## ARM NEON Constraints

### 64-bit Limitations
- `vmull_u32` produces 64-bit result from 32-bit inputs
- No native 64-bit multiply-add
- Recommend 32-bit prime configuration on ARM

### Recommended ARM Prime Set
Use these primes for optimal NEON performance:
- 998244353 (fits 30 bits)
- 7340033 (fits 23 bits)
- ... (all < 2^32)
```

**GAP-I-0009: Gnuplot Dependency [2h]**
```python
def check_gnuplot():
    """Check for gnuplot availability and provide helpful error."""
    import shutil
    if shutil.which('gnuplot') is None:
        print("ERROR: gnuplot not found")
        print("Install with:")
        print("  Ubuntu/Debian: sudo apt-get install gnuplot")
        print("  macOS: brew install gnuplot")
        print("  Windows: choco install gnuplot")
        print("\nAlternatively, use --no-charts to skip chart generation")
        sys.exit(1)
```

### Verification Gaps (24h)

**GAP-V-0001: Boundary Tests [4h]**
```rust
#[cfg(test)]
mod boundary_tests {
    #[test]
    fn test_k_elimination_boundaries() {
        // Minimum: X = 0
        assert_eq!(k_elimination_divide(0, 7), Ok((0, 0)));
        
        // Maximum: X = M*A - 1
        let max_x = M * A - 1;
        let (q, r) = k_elimination_divide(max_x, 7).unwrap();
        assert_eq!(q * 7 + r, max_x);
        
        // k = 0 (no overflow)
        let no_overflow = M - 1;
        assert_eq!(recover_k(no_overflow), 0);
        
        // k = A - 1 (max overflow)
        let max_overflow = M * (A - 1) + M - 1;
        assert_eq!(recover_k(max_overflow), A - 1);
    }
}
```

**GAP-V-0003: Parallel Correctness [4h]**
```rust
#[test]
fn test_parallel_equals_sequential() {
    let data = random_crt_values(10000);
    
    let seq_result = sequential_crt_add(&data);
    let par_result = parallel_crt_add(&data);
    
    assert_eq!(seq_result, par_result, 
        "Parallel and sequential results must match");
}
```

**GAP-V-0005: Cross-Platform CI [8h]**
```yaml
# .github/workflows/cross-platform.yml
name: Cross-Platform CI
on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        arch: [x64, arm64]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: cargo test --all-features
      - name: Hash outputs
        run: |
          cargo run --example determinism_check > output.txt
          sha256sum output.txt >> hashes.txt
      - uses: actions/upload-artifact@v4
        with:
          name: hash-${{ matrix.os }}-${{ matrix.arch }}
          path: hashes.txt
  
  verify-determinism:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Download all hashes
        uses: actions/download-artifact@v4
      - name: Compare hashes
        run: |
          cat */hashes.txt | sort -u | wc -l
          # Should output "1" if all platforms produce same hash
```

**GAP-V-0006: Executioner Plan Validation [4h]**
```python
def validate_execution_plan(plan: dict) -> list[str]:
    """Validate execution plan before execution."""
    errors = []
    
    # Check all tasks have required fields
    for task in plan.get('tasks', []):
        if 'id' not in task:
            errors.append(f"Task missing 'id': {task}")
        if 'depends_on' in task:
            for dep in task['depends_on']:
                if not task_exists(plan, dep):
                    errors.append(f"Task {task['id']} depends on non-existent task {dep}")
    
    # Check for circular dependencies
    if has_circular_deps(plan):
        errors.append("Circular dependency detected in task graph")
    
    # Check resource constraints
    if exceeds_resource_limits(plan):
        errors.append("Plan exceeds available resource limits")
    
    return errors
```

### Tooling Gaps (14h)

**GAP-S-0005: gap_scanner.py Implementation [8h]**
See separate file: `gap-master/tools/gap_scanner.py`

**GAP-S-0006: Skill Dependency Graph [2h]**
```python
# Generate dependency graph
dependencies = {
    'gap-master': ['innovation-mining', 'innovation-genealogy', 'theorem-crusher'],
    'executioner': ['gap-master', 'bottleneck-hunter'],
    'house-party': ['executioner'],
    'document-generator': ['all'],  # Can consume any skill output
    'innovation-mining': ['conversation_search'],
    'innovation-genealogy': ['innovation-mining'],
    'theorem-crusher': [],  # No skill dependencies
    'bottleneck-hunter': [],
}

# Verify DAG (no cycles)
def verify_dag(deps):
    visited = set()
    rec_stack = set()
    
    def dfs(node):
        visited.add(node)
        rec_stack.add(node)
        for neighbor in deps.get(node, []):
            if neighbor not in visited:
                if dfs(neighbor):
                    return True
            elif neighbor in rec_stack:
                return True
        rec_stack.remove(node)
        return False
    
    for node in deps:
        if node not in visited:
            if dfs(node):
                raise ValueError(f"Circular dependency detected involving {node}")
    return True
```

**GAP-N-0001: Float Checker [4h]**
```python
#!/usr/bin/env python3
"""Check for floating-point operations in Rust codebase."""

import re
import sys
from pathlib import Path

FLOAT_PATTERNS = [
    r'\bf32\b', r'\bf64\b',           # Float types
    r'\d+\.\d+[fF]?',                  # Float literals
    r'\.sin\(', r'\.cos\(', r'\.exp\(', # Float methods
    r'\.sqrt\(', r'\.pow\(',           # More float methods
    r'as f32', r'as f64',              # Float casts
]

def check_file(path: Path) -> list[tuple[int, str, str]]:
    """Check a single file for float usage."""
    violations = []
    content = path.read_text()
    
    for line_num, line in enumerate(content.split('\n'), 1):
        # Skip comments
        if line.strip().startswith('//'):
            continue
        
        for pattern in FLOAT_PATTERNS:
            if re.search(pattern, line):
                violations.append((line_num, pattern, line.strip()))
    
    return violations

def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    total_violations = 0
    
    for rs_file in root.rglob('*.rs'):
        violations = check_file(rs_file)
        if violations:
            print(f"\n{rs_file}:")
            for line_num, pattern, line in violations:
                print(f"  L{line_num}: [{pattern}] {line[:60]}")
                total_violations += 1
    
    if total_violations == 0:
        print("✓ No floating-point operations detected")
        sys.exit(0)
    else:
        print(f"\n✗ {total_violations} floating-point violations found")
        sys.exit(1)

if __name__ == '__main__':
    main()
```

---

## SPRINT 2: MEDIUM PRIORITY (P2)
**Timeline:** Before release  
**Items:** 35 gaps  
**Estimated Effort:** ~80 hours

Key items:
- GAP-V-0004: Browser compatibility matrix
- GAP-M-0002: NTT prime selection formalization
- GAP-S-0001: Twiddle factor caching strategy
- GAP-D-0002: Bottleneck-hunter worked example
- GAP-D-0003: Rust benchmark integration

---

## VERIFICATION CHECKLIST

After all resolutions, verify:

- [ ] `cargo build --all-features` succeeds
- [ ] `cargo build --target wasm32-unknown-unknown` succeeds  
- [ ] `cargo test` passes (including new boundary tests)
- [ ] `python3 tools/check_no_floats.py .` returns 0
- [ ] `python3 tools/gap_scanner.py . --ci` returns 0
- [ ] Cross-platform CI shows identical hashes
- [ ] All API keys read from environment variables
- [ ] gnuplot dependency check provides helpful error

---

**Resolution Tracking:**
- P0 Resolved: 0/4
- P1 Resolved: 0/12
- P2 Resolved: 0/35
- Total Progress: 0%
