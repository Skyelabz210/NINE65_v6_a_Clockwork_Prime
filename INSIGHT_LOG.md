# INSIGHT_LOG.md

Project: QMNF proof stack review
Date: 2026-02-03
Analyst: Codex

## 1) Evidence Map

Applied (implemented + wired):
- [ ] Lean core K-Elimination proof with 0 sorry in swarm_run Lean project - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/KElimination.lean
- [ ] Lean CRT foundations with 0 sorry - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/CRT.lean
- [ ] Swarm proof status and artifacts (verified stack + test harness) - /home/acid/Projects/qmnf-security-proofs/swarm_run/synthesis/FINAL_VERIFICATION_STATUS.md
- [ ] Proof dependency DAG and counts - /home/acid/Projects/qmnf-security-proofs/swarm_run/state/blueprint.json

Intended (documented, not wired):
- [ ] MYSTIC package proof stub referenced by index - /home/acid/Projects/MYSTIC/nine65_v2_complete/INDEX.md
- [ ] K-Elimination Lean proof stub in MYSTIC package (proof in progress, uses sorry) - /home/acid/Projects/MYSTIC/nine65_v2_complete/proofs/KElimination.lean
- [ ] Coq proof catalog labeled VERIFIED but with admitted statements noted - /home/acid/Projects/qmnf-security-proofs/coq_proofs/README.md
- [ ] Formal proof progress tracking and partial status - /home/acid/Projects/qmnf-formalization-swarm/PROGRESS_REPORT.md

Expected (claims not yet verified):
- [ ] "14 formally verified innovations (Coq proofs)" claim vs admitted counts in proof stack - /home/acid/Projects/NINE65_CODEX_REFERENCE.md and /home/acid/Projects/qmnf-security-proofs/swarm_run/state/blueprint_round2.json
- [ ] "All major proofs verified" claim vs Lean sorry in SecurityLemmas and INDCPAGame - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/SecurityLemmas.lean and /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/INDCPAGame.lean

## 2) Gaps

Logic gaps:
- [ ] Canonical MYSTIC proof file contains sorry placeholders (not a completed proof) - /home/acid/Projects/MYSTIC/nine65_v2_complete/proofs/KElimination.lean
- [ ] Lean security lemmas still contain sorry (RLWE indistinguishability, encryption hides message) - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/SecurityLemmas.lean
- [ ] IND-CPA game proof has a sorry for decrypt correctness (noise bound) - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/INDCPAGame.lean
- [ ] Multiple proof stacks with inconsistent completeness (Lean/Coq copies in RedTeam and formalization swarm) - /home/acid/Projects/RedTeam/proofs/02_QMNF_Lean4_Proofs.lean and /home/acid/Projects/qmnf-formalization-swarm/02_QMNF_Lean4_Proofs.lean

Assumptions:
- [ ] External RLWE hardness assumption (A001) is required for security theorems - /home/acid/Projects/qmnf-security-proofs/swarm_run/synthesis/QMNF_SECURITY_THEOREM_STACK.md
- [ ] Noise bound correctness assumed in IND-CPA proof (explicit sorry) - /home/acid/Projects/qmnf-security-proofs/swarm_run/lean_project/SwarmProofs/INDCPAGame.lean
- [ ] Coq admitted counts tracked as non-critical in proof docs - /home/acid/Projects/qmnf-security-proofs/coq_proofs/README.md

Bias risks:
- [ ] "VERIFIED/COMPLETE" labels used while admitted/sorry remain (optimism bias) - /home/acid/Projects/qmnf-security-proofs/README.md and /home/acid/Projects/qmnf-security-proofs/coq_proofs/README.md
- [ ] Multiple copies enable selective status reporting (best-case vs canonical) - /home/acid/Projects/qmnf-formalization-swarm/PROGRESS_REPORT.md and /home/acid/Projects/RedTeam/proofs/03_QMNF_Coq_Proofs.v

Practicality gaps:
- [ ] No single canonical build path for all proof sources (Lean/Coq split across directories) - /home/acid/Projects/qmnf-security-proofs/README.md and /home/acid/Projects/qmnf-formalization-swarm/PROGRESS_REPORT.md
- [ ] Proof status not wired to product package (MYSTIC ships a stub) - /home/acid/Projects/MYSTIC/nine65_v2_complete/INDEX.md

Operational gaps:
- [ ] No unified manifest mapping public claims to exact proof files and build results - /home/acid/Projects/MYSTIC/INNOVATION_RESOURCE_INDEX.md

## 3) Risks and Constraints
- [ ] Risk: public claims of fully verified proofs mismatch actual sorry/admitted status - credibility risk - mitigate by canonical manifest + strict labels
- [ ] Risk: fragmentation (RedTeam vs swarm vs MYSTIC) causes drift - mitigate by choosing a single source of truth and deprecating others
- [ ] Constraint: security proofs depend on external assumptions (RLWE) and asymptotic lemmas - requires Mathlib/Coq libraries or accepted axioms

## 4) Opportunities
- [ ] Create PROOF_STACK_MANIFEST.md that maps each claim to a proof file, status, and build command
- [ ] Add automated checks for sorry/Admitted before labeling any proof as VERIFIED
- [ ] Port the verified K-Elimination proof into the production package and remove the stub

## 5) Open Questions
- [ ] Which directory is the canonical proof stack for public claims: qmnf-security-proofs, qmnf-formalization-swarm, or MYSTIC/nine65_v2_complete?
- [ ] What is the acceptance bar for "VERIFIED" (zero sorry/admitted, or allowed standard axioms)?
- [ ] Should Coq proof completion be a requirement before marketing "formally verified"?
- [ ] Do you want a single Lean 4 stack only, or Lean + Coq parity?

## 6) Notes
- [ ] The MYSTIC package indexes a proof file that is explicitly marked "proof in progress"; this likely conflicts with "complete package" positioning
- [ ] Security lemmas still contain sorry for asymptotic proofs; consider isolating these as "assumed" and labeling theorems as conditional
