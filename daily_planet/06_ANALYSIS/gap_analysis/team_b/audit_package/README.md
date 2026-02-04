# Audit Package
This folder contains the QMNF 10×10 audit log for **Team B**. Team A works in `analysis/team_a/ITERATIVE_AUDIT.md`. Use this copy to avoid overlapping work.

## Contents
- `ITERATIVE_AUDIT.md` — current audit log with batch-by-batch findings.

## How to use
1) Read `ITERATIVE_AUDIT.md` before starting any fixes; it lists float violations, FFI risks, and binary artifacts to remove.
2) When addressing issues, annotate the log with resolution status instead of deleting entries.
3) Keep all new audit notes in this folder to avoid scattering state.

## Codebase standards (must follow during fixes)
- Zero floating-point in code paths: replace with `QMNFRational`/integers; floats only allowed in benchmark timing and must be clearly marked.
- Enforce boundary guards on all external inputs (FFI/API/tests) using `qmnf_guards` patterns.
- Python: PEP8, type hints everywhere, Google-style docstrings, four spaces.
- Rust: `cargo fmt`, `cargo clippy --all-targets`; no unsafe unless justified; no float arithmetic unless explicitly allowed for benchmarks.
- Naming: snake_case for functions/modules, PascalCase for classes/types; public functions require type hints/docstrings.
- Tests: add/adjust alongside code (`tests/python/…`, `hcvlang/tests/…`) and note coverage gaps in the audit log.
- FFI: only one canonical linkage; guard float ingress at boundaries; avoid duplicate binding variants.

## What to check/correct (priority cues)
- Float violations and timing-only float usage — convert or clearly isolate.
- Missing guards on FFI/API entry points.
- Duplicate or divergent modules (e.g., multiple adaptive CRT variants) — consolidate.
- Binary artifacts checked into repo — remove and regenerate on demand.
- Incomplete placeholders/stubs — implement or mark as non-production.
- Documentation claims vs. code reality — reconcile or downgrade claims.

## Update process
- After each batch of 10 files, append findings to `ITERATIVE_AUDIT.md`, including: file, description, benchmarks, float/standards/FFI check, issues/next steps.
- Do not delete prior rows; add a “Resolved” note when fixed.
- Keep paths exact and 1-based file references when you cite lines.
