# EXECUTION_PLAN.md

Project: QMNF proof stack
Date: 2026-02-03
Scope: Consolidate proof sources, reconcile verification claims, and close priority gaps in Lean/Coq security proofs.
Principles: Rust-first, no feature-flag fallbacks, deterministic arithmetic.

## 1) Goals and Acceptance Criteria
- Goal 1: Establish a single canonical proof stack with traceable claims.
- Goal 2: Close the highest-impact sorry/admitted statements in security and K-Elimination proofs.
- Goal 3: Align public-facing verification claims with machine-checkable evidence.
- Acceptance criteria:
  - [ ] Canonical proof root selected and documented in PROOF_STACK_MANIFEST.md
  - [ ] Proof status table lists each innovation, file path, and sorry/admitted count
  - [ ] K-Elimination and IND-CPA proofs build with zero sorry in the canonical Lean stack
  - [ ] Coq proof status explicitly labels admitted statements (or none remain)

## 2) Task List (Ordered)
- [ ] Task 1 - Select canonical proof root and deprecate duplicates (mark non-canonical dirs as archival) - verification: manifest updated + links added
- [ ] Task 2 - Create PROOF_STACK_MANIFEST.md mapping claims to proof files and build commands - verification: manifest reviewed + paths valid
- [ ] Task 3 - Run Lean/Coq builds for canonical stack and capture logs + sorry/admitted counts - verification: BUILD_LOG.md with counts
- [ ] Task 4 - Close priority sorry in Lean security proofs (SecurityLemmas.lean, INDCPAGame.lean) - verification: lake build passes with 0 sorry for these modules
- [ ] Task 5 - Reconcile MYSTIC package proof stub by linking to verified proof or completing the stub - verification: no sorry in production proof file or explicit stub label
- [ ] Task 6 - Update public claims (NINE65_CODEX_REFERENCE, coq_proofs/README) to reflect actual status - verification: claims match manifest

## 3) Validation Gates
- Gate A: lake build succeeds for canonical Lean project with zero sorry in listed core modules
- Gate B: coqc runs for selected Coq innovations with zero Admitted or explicit admitted list in manifest
- Gate C: rg checks confirm no new sorry/admit in canonical proof paths

## 4) Risks
- Risk: asymptotic/analysis lemmas require extra libraries (Mathlib/Coq) - mitigation: isolate as explicit axioms if unavoidable
- Risk: proof drift across duplicated directories - mitigation: hard deprecate non-canonical paths

## 5) Dependencies
- Lean 4 toolchain and mathlib for canonical proof stack
- Coq toolchain for NINE65 proofs
- Agreed definition of "VERIFIED" vs "COMPLETE"
