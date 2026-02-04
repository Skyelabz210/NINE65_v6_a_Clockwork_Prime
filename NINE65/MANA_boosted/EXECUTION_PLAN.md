# EXECUTION_PLAN.md

Project: NINE65/MANA_boosted
Date: 2026-01-20
Scope: Full float purge across workspace (core, tests, benches, examples), plus dual-RNS correctness and proof hygiene
Principles: Rust-first, no feature-flag fallbacks, deterministic arithmetic.

## 1) Goals and Acceptance Criteria
- Goal 1: Remove all floating-point types and operations across workspace code.
- Goal 2: Validate dual-RNS K-Elimination correctness paths and tests.
- Goal 3: Track and reduce proof debt (Admitted statements) with gating.
- Acceptance criteria:
  - [ ] `rg -n "\\bf32\\b|\\bf64\\b|float|double" crates crates/nine65/tests crates/mana/examples crates/unhal/examples benches` returns empty for Rust sources
  - [ ] `cargo test -p nine65 --features v2,parallel,accelerated --release` passes
  - [ ] `rg -n "^Admitted\\." proofs/coq` returns empty or matches a documented allowlist
  - [ ] Dual-RNS ct x ct test passes without `#[ignore]` in `crates/nine65/src/ops/rns_mul.rs` or equivalent replacement test in `crates/nine65/src/ops/rns_fhe.rs`

## 2) Task List (Ordered)
- [ ] Fix fixed-point standard to SCALE=2^30 and define shared helpers (mul/div/round) plus PI-based constants
- [ ] Locate existing CORDIC implementation; if absent, implement `cordic_sincos` with 32-step atan table scaled by 2^30
- [ ] Replace float types in public structs (compiler, security, params) with fixed-point or rational fields; update all callers
- [ ] Refactor algorithmic float usage (GSO basin placement, P2 quantile estimator, Grover stats, quantum metrics) to integer-only math
- [ ] Refactor diagnostics/bench/test output to integer or scaled integer formats (no float formatting)
- [ ] Add enforcement: clippy deny + rg gate for float usage across all Rust sources
- [ ] Replace ignored RNS multiply test with coefficient-domain dual-RNS test using `RNSFHEContext::mul_dual`; ensure it runs by default
- [ ] Add explicit `anchor_product` overflow guard and tests in `crates/nine65/src/arithmetic/rns.rs`
- [ ] Proof hygiene pass: list admitted theorems, decide which must be closed, add CI gate to flag remaining admits

## 3) Validation Gates
- Gate A: `cargo test --workspace --release`
- Gate B: `cargo test -p nine65 --features v2,parallel,accelerated --release`
- Gate C: `rg -n "^Admitted\\." proofs/coq` (empty or allowlisted)
- Gate D: `rg -n "\\bf32\\b|\\bf64\\b|float|double" crates crates/nine65/tests crates/mana/examples crates/unhal/examples benches` (empty)

## 4) Risks
- Risk: Fixed-point replacements may change output semantics - mitigation: define stable scaling constants and document conversions
- Risk: Integer-only trig/quantile approximations may impact behavior - mitigation: verify against integer invariants and regression tests
- Risk: Dual-RNS test changes may expose latent NTT-domain mismatches - mitigation: enforce coefficient-domain conversions with explicit API
- Risk: Proof completion may require additional Coq libraries - mitigation: lock toolchain and document dependencies

## 5) Dependencies
- Coq toolchain for proofs (8.17+)
- Fixed-point scale decisions for ratios and probabilistic metrics
- Sufficient disk space to re-run full-project scans without errors
