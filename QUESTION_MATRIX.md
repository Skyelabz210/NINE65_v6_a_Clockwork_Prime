# QUESTION_MATRIX.md

Project: QMNF proof stack
Date: 2026-02-03

| Question | Why it matters | Resolve by |
| --- | --- | --- |
| Which directory is the canonical proof stack for public claims? | Avoids conflicting statuses and drift | Pick one root (recommended: qmnf-security-proofs) and mark others as archival |
| What is the acceptance bar for "VERIFIED"? | Determines labeling and release readiness | Decide: zero sorry/admitted vs allowed axioms; document in manifest |
| Should MYSTIC/nine65_v2_complete ship a proof stub or link to canonical proof? | Public package credibility | Replace stub with verified import or mark as "in progress" |
| Do you require Lean-only, Coq-only, or Lean+Coq parity? | Sets completion workload | Choose target (Lean primary, Coq optional) and scope tasks |
| Which security assumptions must be explicit in public docs? | Avoids over-claiming | List assumptions (RLWE, parameter validation) in manifest and README |
