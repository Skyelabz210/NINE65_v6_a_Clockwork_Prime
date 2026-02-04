# INSIGHT_LOG.md

Project: NINE65/MANA_boosted
Date: 2026-01-20
Analyst: Codex
Objective: Full removal of floating-point usage across workspace (core, tests, benches, examples).

## 1) Evidence Map

Applied (implemented + wired):
- [ ] Dual-RNS + K-Elimination core path in `crates/nine65/src/arithmetic/rns.rs` and `crates/nine65/src/ops/rns_fhe.rs`
- [ ] MANA K-Anchor exact division in `crates/mana/src/anchor.rs`
- [ ] GSO-FHE noise bounding layer in `crates/nine65/src/ops/gso_fhe.rs`
- [ ] CRT Shadow entropy pipeline in `crates/nine65/src/entropy/crt_shadow.rs`
- [ ] Key zeroization in `crates/nine65/src/keys/mod.rs` and `crates/nine65/src/ring/polynomial.rs`
- [ ] Coq proof corpus and summary in `proofs/coq/*.v` and `proofs/VALIDATION_SUMMARY.md`
- [ ] Project inventory captured in `/tmp/projects_files.txt` (md/rs/py splits in `/tmp/projects_*_files.txt`)
- [ ] Float usage inventory completed across `crates/*`, tests, benches, examples

Intended (documented, not wired):
- [ ] AcceleratedFHE notes partial acceleration coverage in `crates/nine65/src/accelerated.rs`
- [ ] Integer-only and constant-time claims lack repo-enforced gates in `README.md` and `SECURITY_ANALYSIS_REPORT.md`

Expected (claims not yet verified):
- [ ] Depth-50 and benchmark claims in `README.md` and `BENCHMARK_REPORT.md` (not re-run here)
- [ ] "All proofs compile" vs admitted statements in `proofs/coq/*.v` and `proofs/VALIDATION_SUMMARY.md`
- [ ] Zero floating-point anywhere vs f64 usage in core/runtime/tests/benches

## 2) Gaps

Logic gaps:
- [ ] Ignored RNS multiply test due to NTT consistency mismatch in `crates/nine65/src/ops/rns_mul.rs`
- [ ] Single-RNS rescaling path documented as failing for multi-prime in `crates/nine65/src/ops/rns_fhe.rs`
- [ ] Admitted statements remain in `proofs/coq/OrderFinding.v`, `proofs/coq/StateCompression.v`, `proofs/coq/CyclotomicPhase.v`
- [ ] Float-based algorithms embedded in core modules (example: `crates/nine65/src/ops/gso_fhe.rs` golden-angle placement)

Assumptions:
- [ ] `anchor_product` fits in u128 or fallback path used; overflow note in `crates/nine65/src/arithmetic/rns.rs`
- [ ] K-Elimination must run in coefficient domain; requirement noted but not enforced in `crates/nine65/src/arithmetic/rns.rs`
- [ ] Noise/security estimates rely on float heuristics in `crates/nine65/src/compiler.rs` and `crates/nine65/src/security/mod.rs`
- [ ] Float-based metrics are present in tests/benches (e.g., `crates/nine65/tests/pqeaq_harness.rs`, `crates/mana/examples/benchmark.rs`)

Bias risks:
- [ ] Self-cryptanalysis only; no external review in `SECURITY_ANALYSIS_REPORT.md`

Practicality gaps:
- [ ] Full-project rg scans hit "No space left on device" while reading `/home/acid/Projects/MYSTIC/SCRAPE_*`
- [ ] Float usage present across runtime modules despite integer-only claims (e.g., `crates/nine65/src/ahop/grover_full.rs`, `crates/nine65/src/params/mod.rs`, `crates/nine65/src/noise/mod.rs`)
- [ ] CORDIC implementation not located in this workspace; may be external or missing

Operational gaps:
- [ ] No CI gate to fail on `Admitted.` in proofs (`proofs/coq/*.v`)
- [ ] No repo-wide lint to enforce integer-only policy outside exceptions

## 3) Risks and Constraints
- [ ] Integer-only policy drift (f64 in runtime) could undermine determinism claims - mitigate by isolating float metrics or replacing with fixed-point
- [ ] Dual-RNS correctness relies on coefficient-domain K-Elim; misuse would break correctness - mitigate with explicit API checks/tests
- [ ] Anchor overflow handling not centralized; increases risk of silent errors - mitigate by explicit guard + tests
- [ ] Disk space errors may block full audits and reproducibility - mitigate by cleaning or excluding heavy scrape files

## 4) Opportunities
- [ ] Add lint/CI step: `rg -n "\\bf32\\b|\\bf64\\b" crates/nine65/src` with allowlist to enforce integer-only policy
- [ ] Promote dual-RNS tests (non-ignored) using native DualRNS encryption to validate K-Elimination path
- [ ] Add proof gate: `rg -n "^Admitted\\." proofs/coq` to keep theorem status visible
- [ ] Add fixed-point utilities for ratios, quantiles, and trig approximations; remove float types entirely

## 5) Open Questions
- [ ] Which fixed-point scale should replace probabilities/ratios (per-mille, per-million, or power-of-two)?
- [ ] Should `anchor_product` overflow force errors instead of `0` sentinel?
- [ ] Which admitted proofs are release blockers vs acceptable research debt?
 - [ ] Do any benchmark outputs require a specific numeric format after float removal?

## 6) Notes
- [ ] Project-wide rg inventories stored in `/tmp/projects_files.txt`, `/tmp/projects_md_files.txt`, `/tmp/projects_rs_files.txt`, `/tmp/projects_py_files.txt`
- [ ] Scan error: `rg` hit "No space left on device" reading `/home/acid/Projects/MYSTIC/SCRAPE_*` during ops/test/data searches
