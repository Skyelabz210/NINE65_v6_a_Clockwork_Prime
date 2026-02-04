# QMNF No-Float Linter Update (Offline)

Date: 2025-12-16
Repo: `/home/acid/Projects/QMNF_System`

## What Changed

- Tightened Python float scanner: `tools/check_no_floats.py`
  - No safe-context exemptions (strings/comments/docstrings are scanned)
  - Decimal detection avoids dotted chains (IPv4, etc)
  - Boundary allowlist removed (covenant applies everywhere)
- Tightened Rust float scanner: `tools/check_no_floats_rust.py`
  - Decimal detection avoids dotted chains (IPv4, etc)
- Added JSON/CSV sanitizer for float-like encodings: `tools/sanitize_json_csv_no_floats.py`
  - JSON numeric decimals/exponents become quoted canonical rationals (`num/den`)
  - Decimal tokens inside JSON strings / CSV cells are sanitized with NBSP
- Hardened decimal-token sanitizer: `tools/sanitize_digits_dot_digits_nbsp.py`
  - Matches only single-dot decimal tokens (avoids dotted chains)
  - Added `--fail-on-change` to use it as a linter check
  - Default file types are documentation text (`.md`, `.txt`)
- Extended offline gate: `tools/prod_gate.py`
  - New checks: text decimal-token scan + JSON/CSV float encoding scan
  - New `--scope` flag:
    - `production` (default): scans `qmnf/`, `hcvlang/`, `crates/`, `src/`, `tools/`, `docs/`
    - `repo`: scans the entire repository (includes benchmarks/archives/artifacts)
  - Current behavior:
    - `production`: enforces code + data (`qmnf/`, `hcvlang/src`, `crates/`, `src/`, `tools/`), does not scan docs text
    - `repo`: enforces everything (includes docs/benchmarks/archives) and also runs the `.md/.txt` decimal-token check

## Current Status (Expected Fails)

`python3 tools/prod_gate.py` currently fails in production scope due to existing violations in:
- Python sources under `qmnf/`
- Rust sources under `hcvlang/` (plus tests/benches/examples)
- Documentation `.md` / `.txt` in production scope
- A small set of JSON files under `docs/` and `hcvlang/`

## Enforcement Policy (Locked)

- No floating point types or operations.
- No decimal-token patterns (`digits-dot-digits`) in any repo text, including docs and comments.
- For JSON/CSV: no decimal numbers; represent fractional meaning as canonical rational strings (`num/den`).
