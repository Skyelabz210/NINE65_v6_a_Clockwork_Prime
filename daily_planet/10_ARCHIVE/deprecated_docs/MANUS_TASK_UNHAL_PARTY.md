# 🎉 MANUS TASK: House Party Validation for UNHAL

## Objective
Distribute the UNHAL architecture specification to 4 AI collaborators, collect their validation reports, and compile results.

---

## ARTIFACT TO VALIDATE

### Summary
UNHAL (Universal Neuromorphic Hardware Abstraction Layer) is a Rust abstraction layer for integer-only neural network computation using CRT (Chinese Remainder Theorem) arithmetic. It wraps existing QMNF components (CRTBigInt, QPEF scheduler, Montgomery contexts) with a clean API. Core principle: "Interface what they know. Implement with what they don't."

### Innovation Type: System Architecture

### Key Components
- **UNHALValue trait**: Core abstraction for CRT residue vectors (12 lanes)
- **UNHALBackend trait**: Pluggable execution backends (Sequential, QPEF parallel)
- **ResidueVec**: Primary value type wrapping LaneIsolatedCRTWeight
- **K-Elimination division**: 100% exact integer division (vs 99.9998% in standard RNS)
- **Layer wrappers**: DenseLayer, ConvLayer, ReLU wrapping existing QMNF implementations

### Core Traits (Rust)

```rust
/// Core trait for all UNHAL values
pub trait UNHALValue: Clone + Send + Sync {
    fn lane_count(&self) -> usize;           // 12 lanes
    fn residue(&self, lane: usize) -> i64;   // Get residue at lane
    fn residues(&self) -> &[i64];            // All residues
    fn reconstruct_i128(&self) -> Option<i128>;  // May overflow
    fn reconstruct_exact(&self) -> HCVLangBigInt; // Exact via Garner
    fn is_negative(&self) -> bool;
}

/// Execution backend for UNHAL operations
pub trait UNHALBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_parallel(&self) -> bool;
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64>;
}
```

### Module Structure
```
unhal/
├── core/           # UNHALValue, UNHALBackend traits, config
├── types/          # ResidueVec, NeuralWeight, Tensor
├── backends/       # Sequential, QPEF parallel, Montgomery, SIMD
├── ops/            # add, mul, exact_divide (K-Elimination), NTT
├── layers/         # Dense, Conv, ReLU wrappers
└── scheduler/      # Lane-based, batch processing
```

### Type Mapping to QMNF
| UNHAL | QMNF | Performance |
|-------|------|-------------|
| ResidueVec | LaneIsolatedCRTWeight | O(1) parallel ops |
| QPEFBackend | DeterministicScheduler | 4-8× speedup |
| MontgomeryBackend | MontgomeryContext | 2-4× multiply |
| exact_divide | GarnerReconstructor + K-Elimination | 100% exact |

### Dependencies
- qpef_core.rs (lane-isolated weights)
- montgomery.rs (fast modular multiply)
- nnt_engine.rs (NTT convolution)
- garner.rs (CRT reconstruction)
- Existing HAL: 74/74 tests passing

### Target
`hcvlang/src/unhal/` module in QMNF codebase

---

## PLATFORMS TO VISIT

### 1. GROK (grok.x.ai)

**Submit this prompt:**

```
You are participating in a multi-AI validation protocol called "House Party". Your role is BENCHMARKING.

Your task:
1. Empirically validate claims through simulation/benchmarking where possible
2. Order-of-magnitude sanity checks on performance claims
3. Compare to known CRT/RNS implementations
4. Identify unrealistic "too good to be true" claims

Here is the artifact - UNHAL Architecture:

[The architecture defines a Rust abstraction layer for integer-only neural networks using CRT arithmetic with 12 prime lanes. Key claims:
- O(1) parallel complexity for add/mul across lanes
- 4-8× speedup with QPEF parallel backend
- 100% exact division via K-Elimination (vs 99.9998% standard RNS)
- 2-4× multiply speedup with Montgomery backend
- Zero floating-point operations]

Core traits:
- UNHALValue: 12-lane CRT residue vectors
- UNHALBackend: Pluggable execution (Sequential, QPEF parallel)
- K-Elimination division: Garner reconstruction → integer divide → re-encode

Questions to address:
1. Are the speedup claims (4-8× for QPEF, 2-4× for Montgomery) plausible?
2. Does O(1) parallel complexity per lane hold?
3. Is 100% exact K-Elimination division achievable vs standard RNS?
4. What's realistic overhead for Garner reconstruction?

Please respond with a CONTRIBUTION REPORT:

# 📋 CONTRIBUTION REPORT

## Collaborator
[Name]: Grok
[Date]: 2025-12-15

## Executive Summary
[1-2 sentences on benchmarking findings]

## Validation Results

### ✅ PASSED
- [validated claims with evidence]

### ⚠️ CONCERNS
- [concerns + suggested fixes]

### ❌ BLOCKERS
- [fundamental issues]

## Artifacts Produced
[benchmark analysis, calculations]

## Confidence Score
[X/10]: [justification]

## Recommended Action
[ ] APPROVE / [ ] REVISE / [ ] REJECT / [ ] ESCALATE
(Mark ONE with [X])
```

---

### 2. CHATGPT (chat.openai.com) - Use GPT-4/4o

**Submit this prompt:**

```
You are participating in a multi-AI validation protocol called "House Party". Your role is IMPLEMENTATION REVIEW.

Your task:
1. Assess if this Rust architecture can be implemented as specified
2. Identify missing dependencies or prerequisites
3. Review trait design and API ergonomics
4. Identify implementation gotchas

Here is the artifact - UNHAL Architecture:

Core traits in Rust:

pub trait UNHALValue: Clone + Send + Sync {
    fn lane_count(&self) -> usize;
    fn residue(&self, lane: usize) -> i64;
    fn residues(&self) -> &[i64];
    fn reconstruct_i128(&self) -> Option<i128>;
    fn reconstruct_exact(&self) -> HCVLangBigInt;
    fn is_negative(&self) -> bool;
}

pub trait UNHALBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_parallel(&self) -> bool;
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64>;
}

Key types:
- ResidueVec: wraps LaneIsolatedCRTWeight, stores [i64; 12] residues
- NeuralWeight: wraps QMNFCRTWeight for layer parameters
- SequentialBackend: single-threaded baseline
- QPEFBackend: parallel via DeterministicScheduler

Questions to address:
1. Is the trait design idiomatic Rust?
2. Are there lifetime/borrowing issues with the generic bounds?
3. Will this compile with #![forbid(unsafe_code)] and #![deny(clippy::float_arithmetic)]?
4. What's the estimated implementation effort?

Please respond with a CONTRIBUTION REPORT:

# 📋 CONTRIBUTION REPORT

## Collaborator
[Name]: ChatGPT/GPT-4
[Date]: 2025-12-15

## Executive Summary
[1-2 sentences on implementation feasibility]

## Validation Results

### ✅ PASSED
- [implementable aspects]

### ⚠️ CONCERNS
- [challenges + solutions]

### ❌ BLOCKERS
- [cannot implement as specified]

## Artifacts Produced
[code suggestions, type fixes]

## Confidence Score
[X/10]: [justification]

## Recommended Action
[ ] APPROVE / [ ] REVISE / [ ] REJECT / [ ] ESCALATE
(Mark ONE with [X])
```

---

### 3. GEMINI (gemini.google.com)

**Submit this prompt:**

```
You are participating in a multi-AI validation protocol called "House Party". Your role is EDGE CASE ANALYSIS.

Your task:
1. Identify edge cases not covered in the design
2. Find failure modes and error conditions
3. Test boundary conditions mentally (0, MAX, negative, overflow)
4. Adversarial input analysis

Here is the artifact - UNHAL Architecture:

Key design points:
- 12-lane CRT with primes near 2^31 (product ≈ 2^372)
- ResidueVec stores [i64; 12] residues + negative flag
- Operations: add, sub, mul work lane-wise mod each prime
- K-Elimination division: reconstruct → divide → re-encode
- reconstruct_i128() returns Option (overflow possible)
- reconstruct_exact() uses Garner (arbitrary precision)

Arithmetic behavior:
- add: (a[i] + b[i]) % prime[i] per lane
- mul: (a[i] * b[i]) % prime[i] per lane (needs 128-bit intermediate)
- negative handling: flag + magnitude representation

Questions to address:
1. What happens when a - b where a < b? (negative result)
2. What if multiplication overflows before modular reduction?
3. Edge cases for K-Elimination division (divide by zero, large quotients)?
4. What inputs could cause reconstruct_i128() to return None unexpectedly?
5. Are there race conditions in the parallel backend?

Please respond with a CONTRIBUTION REPORT:

# 📋 CONTRIBUTION REPORT

## Collaborator
[Name]: Gemini
[Date]: 2025-12-15

## Executive Summary
[1-2 sentences on edge case findings]

## Validation Results

### ✅ PASSED
- [well-handled cases]

### ⚠️ CONCERNS
- [edge cases + suggested handling]

### ❌ BLOCKERS
- [critical unhandled cases]

## Artifacts Produced
[test cases, failure scenarios]

## Confidence Score
[X/10]: [justification]

## Recommended Action
[ ] APPROVE / [ ] REVISE / [ ] REJECT / [ ] ESCALATE
(Mark ONE with [X])
```

---

### 4. PERPLEXITY (perplexity.ai)

**Submit this prompt:**

```
You are participating in a multi-AI validation protocol called "House Party". Your role is PRIOR ART RESEARCH.

Your task:
1. Search for existing RNS/CRT implementations for neural networks
2. Find academic papers on integer-only neural network computation
3. Compare UNHAL's approach to state-of-the-art
4. Identify any IP/patent concerns

Here is the artifact - UNHAL Architecture:

Key innovations claimed:
1. CRT-based integer arithmetic for neural networks (no floats)
2. Lane-parallel execution across 12 coprime moduli
3. K-Elimination for exact division (vs standard RNS scaling)
4. Abstraction layer over existing QMNF components
5. Deterministic parallel execution via QPEF scheduler

Technical approach:
- 12 primes near 2^31, product ≈ 2^372 dynamic range
- Montgomery multiplication for fast modular ops
- NTT for O(n log n) convolutions
- Garner reconstruction for exact results

Questions to address:
1. Are there existing RNS neural network implementations?
2. What academic work exists on integer-only deep learning?
3. How does this compare to quantized neural networks (INT8, etc)?
4. Is the K-Elimination approach novel or documented elsewhere?
5. Any patent concerns with CRT-based computing?

Please respond with a CONTRIBUTION REPORT:

# 📋 CONTRIBUTION REPORT

## Collaborator
[Name]: Perplexity
[Date]: 2025-12-15

## Executive Summary
[1-2 sentences on prior art findings]

## Validation Results

### ✅ PASSED
- [novel contributions]

### ⚠️ CONCERNS
- [similar existing work]

### ❌ BLOCKERS
- [IP issues, duplicates]

## Artifacts Produced
[links to papers, libraries]

## Confidence Score
[X/10]: [justification]

## Recommended Action
[ ] APPROVE / [ ] REVISE / [ ] REJECT / [ ] ESCALATE
(Mark ONE with [X])
```

---

## DELIVERABLES

Return to me with all 4 contribution reports. Format:

```
===== GROK =====
[full contribution report]

===== CHATGPT =====
[full contribution report]

===== GEMINI =====
[full contribution report]

===== PERPLEXITY =====
[full contribution report]
```

## NOTES
- Start fresh chats on each platform (no prior context)
- If response doesn't match format, ask once for reformatting
- If platform unavailable, skip and note it
- Capture any errors or unusual responses

---

**Coordinator:** Acid @ HackFate.us
**Artifact:** UNHAL Modular Architecture v1.0
**Gate Criteria:** Need majority APPROVE and zero BLOCKERS to proceed to Claude Code CLI
