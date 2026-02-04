# QUESTION_MATRIX.md

Project: NINE65/MANA_boosted
Date: 2026-01-20

| Question | Why it matters | Method to resolve |
|---|---|---|
| Where is the existing CORDIC implementation and constants (SCALE=2^30, HALF_PI, ATAN table)? | Needed to avoid duplicate implementations | Point to the file/module or confirm it is external |
| What fixed-point scale is mandated for probability outputs (SCALE=2^30 assumed)? | Affects API outputs and test expectations | Confirm SCALE and update formatting helpers |
| What integer quantile estimator replaces P2 (which uses f64)? | Impacts noise distribution tracking | Select integer quantile algorithm and compare against baseline on sample sets |
| Do we accept `anchor_product = 0` sentinel behavior, or should overflow be a hard error? | Silent overflow risks correctness | Add explicit guard and test; decide on error return vs BigUint |
| Which admitted theorems are release blockers? | Proof debt affects formal claims | Produce allowlist or close proofs starting with OrderFinding and StateCompression |
| Should coefficient-domain K-Elim be enforced at API boundaries? | Prevents NTT-domain misuse | Add wrapper types or checks in `DualRNSContext` |
| Do we need to re-run depth-50 and benchmark claims for this audit? | Align docs with current code | Re-run `cargo bench` and depth tests, record artifacts |
| How should audit scans handle `/home/acid/Projects/MYSTIC/SCRAPE_*` disk errors? | Ensures reproducible audits | Clean disk or add scan exclusions |
