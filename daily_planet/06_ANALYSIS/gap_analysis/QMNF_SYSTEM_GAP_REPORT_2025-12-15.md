# QMNF System Comprehensive Gap Report (Offline)

Date: 2025-12-15
Repo: `/home/acid/Projects/QMNF_System`

This report is written to support a private, solo, AI-augmented workflow:
- No external peer review assumed
- Offline validation favored
- Integer-only covenant enforced (no floating point types or decimal literals in production code paths)

## Executive Summary (What Blocks “Production Ready” Right Now)

- Offline production gate fails on float contamination:
  - Python scan: 300 total violations, 294 critical, 75 files (`py_float_violations_current3.json`)
  - Rust scan: 1611 total violations, 186 files (`rust_float_violations_current.json`)
  - `python3 tools/prod_gate.py` reports: Python compile OK, Python no-floats FAIL, Rust no-floats FAIL
- The repo contains multiple overlapping arithmetic stacks (legacy + modular) with inconsistent semantics and incomplete consolidation:
  - Rust “legacy monolith”: `hcvlang/`
  - Rust “layered crates”: `crates/qmnf-*`
  - Additional legacy crates: `qmnf_crtbigint/`, `realtime_fhe/`, `standalone_extractions/`, `m2m-tokenizer/`
- Python binding reality does not match the intended surface:
  - Installed `hcvlang_pyo3` currently exposes 9 symbols (checked via `dir(hcvlang_pyo3)`)
  - In-repo `hcvlang/src/ffi_minimal.rs` is written to expose far more, but the installed wheel is not built from this repo state
- Several “math correctness” invariants are not enforced as written in your theorem formalism:
  - Saturating arithmetic in reconstruction paths (silent clamp is not “exactness”)
  - Division by zero semantics differ across implementations (return zero vs error)
  - K-Elimination in `crates/qmnf-arithmetic` is constrained to `u128` dynamic range and cannot match large-range claims

Bottom line: the main gap is not “missing features”, it is “lack of a single canonical production surface” plus enforcement that matches your covenant.

## Current State Snapshot (Measured, Not Assumed)

### Offline Gate

- `tools/prod_gate.py` runs:
  - Python `compileall` on `qmnf/` and `tools/` (currently OK)
  - Python float scanner on `qmnf/` (FAIL)
  - Rust float scanner on `hcvlang/`, `crates/`, `src/` (FAIL)

### Python Core Shape

- `qmnf/` is a large mixed tree with:
  - Production-facing wrapper: `qmnf/api.py`
  - Conversion boundary: `qmnf/conversion_boundary.py`
  - Many “framework” modules that include decimal literals in code, comments, and docstrings (violations)
  - Symlinks to root-level modules (`qmnf/core_fast.py` → `qmnf_core_fast.py`) which also contain decimal formatting in debug code

### Rust Core Shape

- Workspace has a modular design intent (`crates/qmnf-primitives`, `crates/qmnf-arithmetic`, `crates/qmnf-fhe`, etc) but also includes multiple legacy crates.
- `hcvlang/` is the active Python extension build target (`setup.py` builds `hcvlang_pyo3` from `hcvlang/Cargo.toml`).
- Rust float violations are concentrated in:
  - `hcvlang/src/ffi.rs` and many `hcvlang/experiments/**` files
  - Some “layered” crates files (for example `crates/qmnf-optimization/src/gso_swarm.rs`)

### FFI Reality

- The currently installed `hcvlang_pyo3` (site-packages) exposes only:
  - Types: `CRTBigInt`, `Rational`, `ModInt`, `ModRational`, `AdaptiveCRTBigInt`
  - Functions: `batch_add_crtbigint`, `batch_mul_crtbigint`, `sum_crtbigint`, `product_crtbigint`
- Therefore many Python tests and higher-level wrappers that expect broader bindings will fail unless:
  - You rebuild the extension from this repo, and/or
  - You reduce Python expectations to match what is actually shipped

## High Priority Gaps (What You “Don’t Have Yet”)

### P0 Blockers (Must Resolve for Production Readiness)

1. Single canonical “production surface” definition
   - Right now the repo contains production code, research code, backups, and experiments in the same scanned trees.
   - Without a boundary, enforcement will never converge.

2. Float covenant enforcement is not reachable with current file placement
   - You can either:
     - Refactor all violating modules to remove decimal literals and float types, or
     - Move non-production files out of scanned paths (recommended), or
     - Change scanner scope (only if you formally redefine policy)

3. FFI build pipeline mismatch
   - The repo state and the installed `hcvlang_pyo3` wheel do not match.
   - Production requires deterministic “build from repo” and a documented “supported export surface”.

4. Correctness semantics mismatches across stacks
   - Examples:
     - `crates/qmnf-arithmetic/src/rational.rs` returns zero on division by zero (not an error)
     - `crates/qmnf-arithmetic/src/k_elimination.rs` uses saturating arithmetic in reconstruction
     - `crates/qmnf-arithmetic/src/crt.rs` panics on out-of-range rather than promoting

### P1 Risks (Security, Determinism, Maintainability)

- Determinism policy is not centrally enforced:
  - Multiple modules use time, randomness, or have undefined behavior in edge cases.
- Cryptographic correctness risks:
  - Randomness strategy is not clearly specified end-to-end (seed sourcing, reproducibility vs security).
- Unsafe Rust usage is present and not consistently documented with safety invariants.
- Packaging brittleness:
  - Symlink-based imports and `sys.path` mutation make installed behavior diverge from repo behavior.

### P2 Gaps (Traceability, Documentation, Workflow)

- Theorem-to-code traceability exists as text, but is not code-aligned across stacks.
- Large parts of documentation contain decimal literals and float types in examples; if you ever expand scanning beyond `.py` and `.rs`, this becomes a large backlog.

## Recommended Roadmap (Minimal Risk, Fastest Convergence)

### Phase A: Define Production Boundary (One Decision, Many Problems Disappear)

Pick one of these and document it:
- Option A (recommended): “Production core” lives in a limited set of directories; everything else moves to `archive/` or `research/` and is excluded from gates.
- Option B: Everything stays in-place; you commit to removing all decimals and float types from every scanned file in `qmnf/`, `hcvlang/`, and `crates/`.

### Phase B: Make `tools/prod_gate.py` Green

- Python compileall already passes.
- Focus on removing float violations by reducing scan scope (move files) before rewriting math.
- Rust: move `hcvlang/experiments/` and backup files out of `hcvlang/` if they are not in production.

### Phase C: FFI Contract + Build Repro

- Define the supported `hcvlang_pyo3` API (exports list).
- Build and test that exact export list deterministically, offline.
- Update Python wrappers and tests to match.

### Phase D: Semantics Alignment With Theorems

- Remove silent clamps (`saturating_*`) in exactness paths; replace with:
  - explicit error returns, or
  - explicit promotion to a larger exact type
- Harmonize division-by-zero behavior across Python and Rust.

## Agent Task Lists (Functional Subagents)

Each focus group below is designed to be assigned to an agent with clear deliverables.

### Focus Group A — Policy and Enforcement Alignment

Goal: Make the covenant unambiguous and enforceable.

Tasks:
- Produce `docs/PRODUCTION_BOUNDARY.md` defining:
  - Which directories are “production”
  - Which directories are “research” and excluded
  - The exact definition of “float contamination” (types, literals, scientific notation, decimal formatting, comments)
- Update tooling to match the boundary definition:
  - If using `research/` or `archive/`, ensure scanners exclude it
  - Ensure `tools/prod_gate.py` only scans production roots
- Add a new repo-wide “forbidden patterns” scan if you want to enforce beyond `.py` and `.rs`

Acceptance:
- `python3 tools/prod_gate.py` returns success on a clean tree
- Policy doc and tool behavior match

### Focus Group B — Python Production Surface Cleanup (`qmnf/`)

Goal: Reduce `qmnf/` to a float-free, import-stable production package.

Tasks:
- Inventory imports from `qmnf/__init__.py` and remove or gate imports of legacy modules:
  - Replace eager imports with lazy imports where possible
  - Remove `sys.path` mutation if not strictly required
- Remove or relocate non-production Python modules that carry decimal literals:
  - Candidates (high violation density): `qmnf/unified_qmnf.py`, `qmnf/frameworks/phi_harmonic_engine.py`, `qmnf/storage/decanal_cylindrical_architecture.py`
  - Move unused “vN” or “PartX” legacy files into `archive/legacy_python/`
- Fix obvious debug-only float formatting in scanned files:
  - Example: debug prints and formatting that include decimal formatting should be rewritten using integer-only representations

Acceptance:
- Run `python3 tools/check_no_floats.py --path qmnf`
- Target: zero critical violations (warnings only in explicitly permitted boundary modules, if policy allows)

### Focus Group C — Rust Float Remediation (Prune First, Rewrite Second)

Goal: Make Rust no-floats scan pass without rewriting research code.

Tasks:
- Move non-production Rust sources out of scanned roots:
  - `hcvlang/experiments/` (and any `hcvlang/src/**/experiments/**` backup files)
  - `hcvlang/examples/` if not part of production
  - Any `backup_*.rs` or investigation snapshots
- In production Rust modules, remove decimal literals in comments and strings:
  - Example: version strings and numeric examples using digits-dot-digits
- Remove float types if present in production code paths (`f32`, `f64`) and replace with integer metadata where required

Acceptance:
- Run `python3 tools/check_no_floats_rust.py --path hcvlang crates src`
- Target: zero violations

### Focus Group D — Python FFI Contract and Build Repro (`hcvlang_pyo3`)

Goal: Ensure Python uses the bindings built from this repo and that exports are intentional.

Tasks:
- Define the export surface:
  - Write `docs/FFI_EXPORTS.md` listing each exported symbol and what guarantees it provides (types, invariants, exactness)
- Rebuild `hcvlang_pyo3` from repo state and verify exports match the contract:
  - Ensure no in-tree stale `.so` shadows the installed module
  - Add a small Python script under `tools/` that asserts `dir(hcvlang_pyo3)` contains the expected set
- Align Python API wrappers (`qmnf/api.py`) with the contract:
  - If a symbol is optional, document it as such
  - If a symbol is required, fail fast with a clear error
- Update tests to import the correct module name (`hcvlang_pyo3`) and to test only the contracted surface

Acceptance:
- `python3 -c "import hcvlang_pyo3; import qmnf"` succeeds
- A “FFI export check” script passes on a clean environment

### Focus Group E — Core Arithmetic Semantics (Exactness Over Convenience)

Goal: Make code match theorem semantics for exactness and error handling.

Tasks:
- Remove silent clamps in exactness paths:
  - Replace saturating math in K-Elimination reconstruction with explicit checks and errors
- Unify division-by-zero behavior:
  - Decide: error or sentinel value
  - Implement consistently in Rust crates and Python wrappers
- Define overflow strategy for CRT-based integers:
  - Decide between auto-promotion vs explicit error return
  - Implement once and reuse
- Add “invariant validation identities” as tests:
  - Reconstruction roundtrip
  - Division identity: q * d + r equals dividend
  - Remainder bounds

Acceptance:
- Unit tests for invariants pass (Rust and Python where applicable)
- No silent wrap and no silent clamp in exactness paths

### Focus Group F — Offline Validation Harness

Goal: One command that gives you confidence without publishing.

Tasks:
- Extend `tools/prod_gate.py` (or add `tools/validate_offline.py`) to run:
  - Python compile check
  - Float scans
  - Minimal unit tests for invariants
  - Optional: Rust `cargo test` for production crates only (avoid research crates)
- Ensure it runs offline and deterministically:
  - No network calls
  - No time-based thresholds

Acceptance:
- `python3 tools/prod_gate.py` (or replacement) is the single “ready” signal

### Focus Group G — Theorem and Documentation Traceability (No Drift)

Goal: Reduce implementation gaps by making theorem-to-code linkage executable.

Tasks:
- Create a canonical “traceability matrix” that points to real files and line numbers:
  - K-Elimination: `crates/qmnf-arithmetic/src/k_elimination.rs`
  - Rational: `crates/qmnf-arithmetic/src/rational.rs` and `hcvlang/src/rational.rs` (if both remain)
  - FHE and entropy: link to the actual implemented modules used by FFI
- Flag any claim that cannot be reproduced with current code as “unverified” and attach reproduction instructions

Acceptance:
- For each theorem claim: you can point to an implementation and a test that validates the identity offline

### Focus Group H — Security and Determinism Review (Private, Offline)

Goal: Improve safety posture without external publication.

Tasks:
- Write a short `docs/THREAT_MODEL.md`:
  - What is being protected (keys, plaintext, model weights)
  - Attacker capabilities (local timing, remote API, co-resident)
- Audit all randomness sources:
  - Identify where deterministic PRNG is acceptable and where OS entropy is required
- Audit `unsafe` blocks:
  - Add explicit safety invariants
  - Prefer removing `unsafe` where performance does not require it

Acceptance:
- Determinism is explicit (seeded) where needed
- Cryptographic randomness is explicitly sourced where required
- `unsafe` is documented or removed in production code

