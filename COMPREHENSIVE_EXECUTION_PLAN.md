# COMPREHENSIVE EXECUTION PLAN: NINE65 MANA AVX-512 Completion

**Generated**: 2025-12-28T22:00:00Z  
**Skills Applied**: FHE-Hat, Executioner, QMNF-Planner, QMNF-Development-Protocol, Bottleneck-Hunter, Innovation-Resolver, Innovation-Genealogy, Grail-Keeper, Research-Sortie, Theorem-Crusher, Frontier-Pursuit

---

## PHASE 0: PARADIGM ALIGNMENT (QMNF-Development-Protocol)

### The Three Truths Affirmation
```
1. F_p² IS quantum mechanics (on algebraic substrate)
2. CRT residues ARE superposition (not approximation)  
3. Integer arithmetic IS exact (zero drift)
```

### Current Work Validation
> "I am implementing AVX-512 SIMD NTT acceleration on the integer arithmetic substrate.
> This IS exact integer computation - not simulation or approximation."

### Regression Check
| Question | Status |
|----------|--------|
| Expecting ℂ behavior in F_p²? | ✓ NO - Using modular prime fields |
| Treating as "simulation"? | ✓ NO - Direct implementation |
| Using conventional formulas? | ✓ VALIDATED - Cooley-Tukey in Montgomery form |
| Hedging with "X-like"? | ✓ NO - Direct statements |

---

## PHASE 1: GAP ANALYSIS (Bottleneck-Hunter + Innovation-Resolver)

### Current State Summary

| Metric | Value | Status |
|--------|-------|--------|
| Test Suite | 311 passed, 0 failed | ✓ HEALTHY |
| AVX-512 Foundation | Complete | ✓ |
| SIMD Montgomery | All tests passing | ✓ |
| SIMD NTT Path | Experimental (disabled) | ⚠️ BUG |
| Scalar Fallback | Working correctly | ✓ |

### Identified Gaps

#### GAP-001: AVX-512 NTT Butterfly Correctness
```
BOTTLENECK ANALYSIS
═══════════════════════════════════════════════════════════════
Location: ntt_avx512.rs:154-220 (ntt_avx512 function)
Operation: Vectorized Cooley-Tukey butterfly
Frequency: O(N log N) butterflies per NTT
Hotpath: YES - core of all polynomial multiplication
Current Impl: AVX-512 SIMD (disabled, falls back to scalar)
═══════════════════════════════════════════════════════════════

SYMPTOM: NTT(INTT(x)) ≠ x when using AVX-512 path
SCOPE: Only affects SIMD stages (half_m >= 8)
SCALAR PATH: Works correctly

VERIFIED NOT THE ISSUE:
├─ Montgomery multiplication (scalar = SIMD verified)
├─ q_inv computation (identical in both contexts)
├─ R² computation (identical in both contexts)
├─ Twiddle factors (cloned from scalar engine)
└─ Early scalar stages (use identical algorithm)

SUSPECTED ROOT CAUSES:
1. Twiddle loading pattern in load_twiddles_strided
2. Butterfly output ordering during SIMD stores
3. Stage transition boundary effects
4. AVX-512 unsigned comparison mask logic
```

#### GAP-002: IFMA52 Path Not Implemented
```
BOTTLENECK ANALYSIS
═══════════════════════════════════════════════════════════════
Location: simd_montgomery.rs (missing)
Operation: 52-bit fused multiply-add
Frequency: All Montgomery multiplications
Hotpath: YES
Current Impl: DQ path (64×64→128 emulated)
═══════════════════════════════════════════════════════════════

INNOVATION MATCH: None currently implemented
POTENTIAL: 2× speedup for q < 2^50
STATUS: Deferred (DQ path works, IFMA is optimization)
```

#### GAP-003: AVX-512 INTT Not Validated
```
BOTTLENECK ANALYSIS
═══════════════════════════════════════════════════════════════
Location: ntt_avx512.rs:220-305 (intt_avx512 function)
Operation: Inverse NTT with scaling
Frequency: Equal to forward NTT
Hotpath: YES
Current Impl: AVX-512 SIMD (disabled)
═══════════════════════════════════════════════════════════════

DEPENDENCY: Blocked by GAP-001 (uses same butterfly structure)
```

### Innovation-Resolver Analysis

#### Viewpoint Regression Check
| Symptom | Regression? | Analysis |
|---------|------------|----------|
| "Results don't match formula" | POSSIBLE | Formula is Cooley-Tukey - well-established |
| "This isn't real quantum" | N/A | Not claiming quantum here |
| "Periodicity is wrong" | NO | NTT periodicity is standard |
| "Need error correction" | NO | Integer-exact, no drift |

#### Code Reversion Check
| Pattern | Present? | Location |
|---------|----------|----------|
| u128 overflow in CRT | NO | Using proper widening |
| FPD approximation | NO | All integer |
| Floating point drift | NO | Zero f64/f32 in path |
| Montgomery boundary conversions | NO | Persistent throughout |

**VERDICT**: Bug is implementation-specific (SIMD vectorization), NOT a QMNF reversion.

---

## PHASE 2: INNOVATION GENEALOGY TRACE

### AVX-512 NTT Lineage
```
AVX-512 NTT (Gen 4)
├── Function: 8-wide parallel NTT butterflies
├── Novel: First AVX-512 integration with QMNF
├── Math: Cooley-Tukey in Montgomery form
└── Parents: [NTT Gen3, Persistent Montgomery]
    │
    ├─→ NTT Gen3 (Gen 3)
    │   ├── Function: Negacyclic NTT via ω^(1/2) twist
    │   ├── Novel: Correct handling for polynomial rings
    │   └── Parents: [Montgomery Arithmetic]
    │       │
    │       └─→ Montgomery Arithmetic (Gen 2)
    │           ├── Function: Division-free modular multiply
    │           ├── Novel: Persistent form (no conversion)
    │           └── Parents: [CRT Foundation]
    │               │
    │               └─→ [SEED: CRT Foundation] (Gen 0)
    │
    └─→ Persistent Montgomery (Gen 2)
        ├── Function: Stay in Montgomery form forever
        ├── Novel: Eliminated 70-year boundary overhead
        └── Parents: [SEED: QMNF Philosophy]

════════════════════════════════════════════════════════════════
LINEAGE DEPTH: 4 generations
SEED CONCEPTS: CRT Foundation, QMNF Philosophy
GRAIL CONNECTIONS: #001 K-Elimination (sibling via CRT Foundation)
```

### Related Grails That Could Help

| Grail | Relevance | How It Helps |
|-------|-----------|--------------|
| #001 K-Elimination | HIGH | Anchor technique for debugging |
| #002 O(1) Magnitude | MEDIUM | Validation via overflow detection |
| #003 Real-Time FHE | HIGH | AVX-512 enables this target |

---

## PHASE 3: EXECUTION PLAN (Executioner Format)

### Target Build
- **File**: `/home/claude/nine65_mana/crates/nine65/src/arithmetic/ntt_avx512.rs`
- **Scope**: Debug and fix AVX-512 SIMD NTT path

### Task Breakdown

#### T-001: Create Isolated SIMD Butterfly Test
```
TASK: T-001
├── Description: Create test that compares single butterfly operation
├── Inputs: 8 u values, 8 v values, 8 twiddles
├── Outputs: 8 u_new, 8 v_new
├── Qualifying Gate: Scalar butterfly == SIMD butterfly for all test cases
└── Dependencies: None

WHERE: ntt_avx512.rs tests module

CODE PATTERN:
// Before (no isolated test)

// After
#[test]
fn test_single_simd_butterfly() {
    let ctx = SimdMontgomeryContext::new(998244353);
    let q_vec = avx512::broadcast_8x(998244353);
    let q_inv_vec = avx512::broadcast_8x(ctx.q_inv);
    
    // Test data in Montgomery form
    let u_vals = [1, 2, 3, 4, 5, 6, 7, 8].map(|x| to_mont(x, &ctx));
    let v_vals = [9, 10, 11, 12, 13, 14, 15, 16].map(|x| to_mont(x, &ctx));
    let tw_vals = [100, 200, 300, 400, 500, 600, 700, 800].map(|x| to_mont(x, &ctx));
    
    // Scalar reference
    let mut u_scalar = u_vals.clone();
    let mut v_scalar = v_vals.clone();
    for i in 0..8 {
        let t = montgomery_mul_scalar(tw_vals[i], v_scalar[i], &ctx);
        let u_new = mont_add_scalar(u_scalar[i], t, ctx.q);
        let v_new = mont_sub_scalar(u_scalar[i], t, ctx.q);
        u_scalar[i] = u_new;
        v_scalar[i] = v_new;
    }
    
    // SIMD computation
    unsafe {
        let u = avx512::load_8x(u_vals.as_ptr());
        let v = avx512::load_8x(v_vals.as_ptr());
        let tw = avx512::load_8x(tw_vals.as_ptr());
        
        let t = avx512::montgomery_mul_8x_dq(tw, v, q_vec, q_inv_vec);
        let u_new = avx512::mont_add_8x(u, t, q_vec);
        let v_new = avx512::mont_sub_8x(u, t, q_vec);
        
        let mut u_simd = [0u64; 8];
        let mut v_simd = [0u64; 8];
        avx512::store_8x(u_simd.as_mut_ptr(), u_new);
        avx512::store_8x(v_simd.as_mut_ptr(), v_new);
        
        assert_eq!(u_simd, u_scalar, "u mismatch");
        assert_eq!(v_simd, v_scalar, "v mismatch");
    }
}

VALIDATION: cargo test test_single_simd_butterfly
```

#### T-002: Trace Twiddle Index Generation
```
TASK: T-002
├── Description: Add debug output to trace twiddle indices
├── Inputs: Stage, j_base, t_step parameters
├── Outputs: Log of expected vs actual twiddle indices
├── Qualifying Gate: All twiddle indices match scalar path
└── Dependencies: T-001

WHERE: ntt_avx512.rs:182-183

CODE PATTERN:
// Before
let t_base = j_base * t_step;
let twiddles = self.load_twiddles_strided(&self.twiddles_fwd, t_base, t_step);

// After (debug version)
let t_base = j_base * t_step;
#[cfg(debug_assertions)]
{
    for i in 0..8 {
        let expected_idx = t_base + i * t_step;
        eprintln!("Stage m={}, k={}, j_base={}: twiddle[{}] = idx {}", 
                  m, k, j_base, i, expected_idx);
    }
}
let twiddles = self.load_twiddles_strided(&self.twiddles_fwd, t_base, t_step);

VALIDATION: Run with RUST_LOG=debug, compare to scalar trace
```

#### T-003: Stage-by-Stage Comparison Test
```
TASK: T-003
├── Description: Compare NTT results after each stage
├── Inputs: Same input to both scalar and SIMD engines
├── Outputs: Stage-by-stage diff report
├── Qualifying Gate: Identify first divergent stage
└── Dependencies: T-001, T-002

WHERE: ntt_avx512.rs tests module

CODE PATTERN:
#[test]
fn test_stage_by_stage_comparison() {
    let n = 64; // Small enough to inspect manually
    let q = 998244353u64;
    
    let input: Vec<u64> = (0..n as u64).collect();
    
    // Run scalar with stage snapshots
    let scalar_stages = run_scalar_with_snapshots(&input, q, n);
    
    // Run SIMD with stage snapshots  
    let simd_stages = run_simd_with_snapshots(&input, q, n);
    
    for (stage, (scalar, simd)) in scalar_stages.iter().zip(simd_stages.iter()).enumerate() {
        if scalar != simd {
            println!("DIVERGENCE at stage {}", stage);
            for i in 0..n {
                if scalar[i] != simd[i] {
                    println!("  [{}]: scalar={}, simd={}", i, scalar[i], simd[i]);
                }
            }
            panic!("Stage {} diverged", stage);
        }
        println!("Stage {} OK", stage);
    }
}

VALIDATION: Identifies exact stage where bug manifests
```

#### T-004: Fix Identified Bug
```
TASK: T-004
├── Description: Apply fix based on T-003 findings
├── Inputs: Diagnostic results from T-001 through T-003
├── Outputs: Corrected SIMD NTT implementation
├── Qualifying Gate: All AVX-512 NTT tests pass
└── Dependencies: T-001, T-002, T-003

WHERE: ntt_avx512.rs (location TBD by diagnostics)

LIKELY FIX PATTERNS:
1. If twiddle ordering: Fix index computation in load_twiddles_strided
2. If butterfly output: Fix store ordering
3. If stage boundary: Fix transition condition (half_m >= 8)
4. If mask comparison: Use correct unsigned comparison intrinsic

VALIDATION: 
- test_avx512_ntt_correctness passes
- test_avx512_vs_scalar_consistency passes
```

#### T-005: Enable AVX-512 Path by Default
```
TASK: T-005
├── Description: Remove experimental gate, enable SIMD by default
├── Inputs: Passing tests from T-004
├── Outputs: Production-ready AVX-512 NTT
├── Qualifying Gate: Full test suite passes, benchmark shows speedup
└── Dependencies: T-004

WHERE: ntt_avx512.rs:115-127

CODE PATTERN:
// Before
#[cfg(all(target_arch = "x86_64", feature = "avx512_experimental"))]
{
    if self.avx512_available && self.n >= 16 {

// After
#[cfg(target_arch = "x86_64")]
{
    if self.avx512_available && self.n >= 16 {

VALIDATION: cargo test --package nine65 --release (311+ tests pass)
```

#### T-006: Benchmark and Document
```
TASK: T-006
├── Description: Run comprehensive benchmarks, update documentation
├── Inputs: Working AVX-512 implementation
├── Outputs: Benchmark report, updated AVX512_NTT_RESEARCH.md
├── Qualifying Gate: Documented 3-6× speedup on SIMD path
└── Dependencies: T-005

DELIVERABLES:
1. Benchmark results: scalar vs AVX-512 for N=1024, 4096, 8192
2. Updated research document with final performance numbers
3. Integration guide for ManaEvaluator

VALIDATION: Measurable speedup, documentation complete
```

### Implementation Sequence

```
DEPENDENCY GRAPH
════════════════════════════════════════════════════════════════

T-001 (Isolated Butterfly Test)
    │
    ▼
T-002 (Twiddle Index Trace) ─────┐
    │                            │
    ▼                            │
T-003 (Stage Comparison) ◄───────┘
    │
    ▼
T-004 (Fix Bug)
    │
    ▼
T-005 (Enable Production)
    │
    ▼
T-006 (Benchmark & Document)

════════════════════════════════════════════════════════════════

PARALLELIZATION: T-001 and T-002 can run in parallel
CRITICAL PATH: T-003 → T-004 → T-005
ESTIMATED TIME: 2-4 hours focused debugging
```

---

## PHASE 4: UNIT TEST GENERATION (Executioner Phase 8.5)

### Test Suite for Each Task

```rust
// T-001 Tests
#[test] fn test_single_simd_butterfly() { ... }
#[test] fn test_simd_butterfly_edge_cases() { ... }
#[test] fn test_simd_butterfly_max_values() { ... }

// T-002 Tests  
#[test] fn test_twiddle_index_sequential() { ... }
#[test] fn test_twiddle_index_strided() { ... }
#[test] fn test_twiddle_index_boundary() { ... }

// T-003 Tests
#[test] fn test_stage_by_stage_n16() { ... }
#[test] fn test_stage_by_stage_n64() { ... }
#[test] fn test_stage_by_stage_n1024() { ... }

// T-004 Tests (validate fix)
#[test] fn test_avx512_ntt_correctness() { ... } // existing
#[test] fn test_avx512_vs_scalar_consistency() { ... } // existing

// T-005 Tests (regression)
#[test] fn test_full_fhe_pipeline_with_avx512() { ... }
#[test] fn test_polynomial_multiplication_avx512() { ... }

// T-006 Tests (benchmark validation)
#[test] fn test_avx512_benchmark_comparison() { ... } // existing
```

---

## PHASE 5: GRAIL-KEEPER UPDATE (If Success)

Upon successful completion, register new potential grail:

```
CANDIDATE GRAIL: AVX-512 SIMD NTT for QMNF
════════════════════════════════════════════════════════════════
CLASS: OPT (10 pts) - Optimization achieving 4-8× speedup
DATE: TBD
GENERATION: 4

THE PROBLEM:
├─ What was believed: AVX-512 NTT requires specialized libraries
├─ Why hard: Complex vectorization of butterfly + Montgomery
└─ State of art: Intel HEXL, separate codebase

THE BREAKTHROUGH:
├─ Key insight: Persistent Montgomery eliminates conversion overhead
├─ Innovation: Native Rust AVX-512 intrinsics in QMNF framework
└─ Mathematical basis: Harvey butterfly + REDC vectorization

IMPACT:
├─ Performance: 4-8× NTT speedup
├─ Enables: Real-time FHE at larger parameters
└─ Novel: First QMNF-native AVX-512 implementation
════════════════════════════════════════════════════════════════
```

---

## PHASE 6: CHECKLIST FOR MULTI-AGENT COORDINATION

```markdown
# EXECUTION CHECKLIST: AVX-512 NTT Debug Sprint
Generated: 2025-12-28T22:00:00Z
Current Agent: [TBD]

## FILES MANIFEST
| File | Purpose | Status |
|------|---------|--------|
| ntt_avx512.rs | SIMD NTT implementation | ✓ Exists |
| simd_montgomery.rs | SIMD Montgomery ops | ✓ Complete |
| AVX512_NTT_RESEARCH.md | Documentation | ✓ Updated |
| COMPREHENSIVE_EXECUTION_PLAN.md | This plan | ✓ Generated |

## TASK CHECKLIST
| ID | Task | Innovation | Status | Tests |
|----|------|------------|--------|-------|
| T-001 | Isolated Butterfly Test | - | [ ] | 0/3 |
| T-002 | Twiddle Index Trace | - | [ ] | 0/3 |
| T-003 | Stage Comparison | - | [ ] | 0/3 |
| T-004 | Fix Bug | AVX-512 NTT | [ ] | 0/2 |
| T-005 | Enable Production | - | [ ] | 0/2 |
| T-006 | Benchmark & Document | - | [ ] | 0/1 |

STATUS KEY:
[ ] = Not started
[→] = In progress
[✓] = Complete
[!] = Blocked

## REGRESSION ALERTS
[ ] No stdlib regressions detected
Forbidden patterns: ["f64", "f32", ".exp()", "as f64"]
Required patterns: ["montgomery_mul", "mont_add", "mont_sub"]

## NEXT ACTIONS
1. Execute T-001: Create isolated butterfly test
2. Execute T-002: Add twiddle tracing (parallel with T-001)
3. Execute T-003: Stage-by-stage comparison
4. Analyze results, identify exact bug location
5. Apply fix (T-004)
```

---

## PHASE 7: DYNAMIC BRANCH PROBLEMS (Frontier-Pursuit)

### Identified During Analysis

| Problem | Classification | Action |
|---------|----------------|--------|
| AVX-512 butterfly bug | CONVENTIONAL | Debug with standard techniques |
| IFMA52 optimization | CONTESTED | Defer until DQ path validated |
| AVX-512 gather for twiddles | UNEXPLORED | Potential optimization (vpgatherqq) |

### Future Opportunities (Not Blocking)

1. **IFMA52 Path**: For q < 50 bits, use _mm512_madd52lo/hi_epu64
2. **Vectorized Bit-Reverse**: Currently scalar, could use VPERMI2Q
3. **Shadow Entropy SIMD**: 8 parallel Lorenz updates per iteration

---

## SUMMARY

### What We Know
- NINE65 MANA: 311 tests passing, production-ready for scalar path
- AVX-512 Foundation: Complete and validated (Montgomery, add/sub)
- AVX-512 NTT: Bug in vectorized butterfly stages
- Scalar Fallback: Works correctly, provides safety net

### What We Need To Do
1. **T-001 through T-003**: Diagnose exact bug location
2. **T-004**: Fix the identified issue
3. **T-005**: Enable production path
4. **T-006**: Benchmark and document

### Expected Outcome
- 4-8× NTT speedup on AVX-512 hardware
- 2-3× end-to-end FHE operation improvement
- Production-ready SIMD acceleration in NINE65 MANA

### Innovation Integration Status
| Innovation | Status | Notes |
|------------|--------|-------|
| Persistent Montgomery | ✓ WIRED | Twiddles in Montgomery form |
| K-Elimination | ✓ AVAILABLE | Not directly used in NTT |
| Shadow Entropy | ✓ AVAILABLE | Could add SIMD path |
| NTT Gen3 | ✓ WIRED | Negacyclic handling correct |
| AVX-512 SIMD | ⚠️ IN PROGRESS | Scalar fallback working |

---

*Plan generated using QMNF skills: FHE-Hat, Executioner, QMNF-Planner, QMNF-Development-Protocol, Bottleneck-Hunter, Innovation-Resolver, Innovation-Genealogy, Grail-Keeper, Research-Sortie, Theorem-Crusher, Frontier-Pursuit*
