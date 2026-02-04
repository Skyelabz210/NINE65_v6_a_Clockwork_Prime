# QMNF System Resource Catalog

**Last Updated:** 2025-11-06  
**Scope:** `.` – unified Python/Rust integer-only compute stack (QMNF + HCVlang).  
**Maintainers:** QMNF Core Team (see `AGENTS.md` for process guidelines).

This catalog maps the major resources that currently live inside the repository so new contributors can find the right code, documentation, data, and tooling without scraping the entire tree.

---

## 1. Top-Level Resource Map

| Category | Path | Contents & Highlights | Status / Notes |
|----------|------|----------------------|----------------|
| Python runtime | `qmnf/` | Boundary layer (`conversion_boundary.py`), API, arithmetic engines, Cosmos/MANA orchestration, storage, neural/VSA frameworks, noise engines | **Active** – all runtime user-facing entry points |
| Rust flagship crate | `hcvlang/` | Core CRT BigInt implementation, SIMD/SIMT math, FHE modules, benches, tests | **Active** – edition 2021; builds on stable toolchain |
| Supporting Rust crates | `qmnf_crtbigint/`, `qmnf_fast_ops/`, `qmnf_bindings/`, `realtime_fhe/` | Specialized crates for CRT BigInt, accelerated kernels, PyO3 bindings, and FHE runtime | **Active** – keep in sync with `hcvlang` interfaces |
| Standalone packages | `standalone_extractions/` | Frozen copies of the core crates for external distribution (`qmnf-core`, `qmnf-rust-core`) | **Reference** – don't edit without regen scripts |
| Documentation | `docs/`, `README.md`, `INDEX.md`, `FILE_INVENTORY.md`, `00_START_HERE.md` | Architecture guides, status reports, quick starts, research notes | **Canonical** documentation set |
| Tests & validation | `tests/`, `hcvlang/tests/`, `test_*_validation.rs`, `examples/`, `test_entropy_shadow/` | Python + Rust suites, integration tests, special-case demos | **Must run** before release |
| Benchmarks & profiling | `benchmarks/`, `qmnf_benchmark_*`, `hcvlang/benches/`, `profiling_and_bottleneck_analysis.py` | Performance harnesses for CRTBigInt, SIMD, entropy shadow, etc. | **Keep updated** with perf claims |
| Operational tooling | `tools/`, `scripts/`, `dashboard/`, `analysis/` | Float guards, benchmark automation, tmux scripts, AI dashboard, evidence archives | **Support** assets |
| Artifacts & env | `hcvlang_pyo3*.so`, `qmnf.egg-info/`, `venv/`, `target/` | Build outputs and environment caches | **Generated** – not hand edited |

---

## 2. Python Runtime Resources (`qmnf/`)

- **Boundary & API layer**: `conversion_boundary.py`, `boundary.py`, `api.py`, `core.py`, `core_fast.py`, and `core_optimized.py` implement the phase-1 guard consolidation and public interface. These are the only modules that should touch external inputs.
- **Arithmetic frameworks**: `qmnf/arithmetic/` contains the multi-part `QMNF_Unified_Arithmetic_Framework_v4*` trilogy plus subpackages for `core/`, `cryptographic/fhe/`, `field_theory/`, `geometry/`, `quantum/`, `sequences/`, and `validation/`. Each subpackage exports deterministic integer primitives; see local README/docstrings for specific invariants.
- **Cosmos / MANA orchestration**: `qmnf/cosmos_mana/` and `qmnf/execution/` host the task orchestration, lease system, temporal schedulers, and neural execution engines. Pair with docs in `docs/architecture/` for sequence diagrams.
- **Storage & Holodrive**: `qmnf/storage/` (decanal, holodrive, cosmos backends) and `holodrive_phase2/` capture high-density storage R&D plus production adapters.
- **Neural, VSA, Noise**: `qmnf/neural/`, `qmnf/vsa/`, and `qmnf/noise/` collect higher-level cognitive primitives, vector-symbolic processors, and entropy/noise utilities.
- **Supporting data**: `qmnf/data/`, `qmnf/frameworks/`, `qmnf/harmonic_primitives.py`, etc., include structured manifests referenced by dashboards and analysis scripts.

> **Developer tip:** No floats are allowed anywhere below `qmnf/`. Use `DataBoundary` helpers and the integer-only helpers exposed by the Rust crates.

---

## 3. Rust Crates & Native Libraries

| Crate | Path | Type | Highlights | Validation Hooks |
|-------|------|------|------------|------------------|
| `hcvlang` | `hcvlang/` | `rlib`, `cdylib` | Flagship CRTBigInt engine, SIMD math, Apollonian primitives, FHE module under `src/fhe/`. Optional `pyo3` + `rayon` features. | Unit tests in `hcvlang/tests/`, benches in `hcvlang/benches/`, examples under `hcvlang/examples/`. |
| `qmnf_crtbigint` | `qmnf_crtbigint/` | `rlib` | Standalone CRT BigInt for pure Rust consumers. Mirrors core algorithms. | No benches here; reuse `hcvlang` suites. |
| `qmnf_fast_ops` | `qmnf_fast_ops/` | `cdylib` | High-speed kernels exposed to Python via FFI. | Build with `cargo build --release`; integrate via `qmnf/core_fast.py`. |
| `qmnf_bindings` | `qmnf_bindings/` | `cdylib` (`pyo3`) | Python bindings that wrap `hcvlang` surface area (`QMNFRational`, mod rational, division optimizer, etc.). | Use `maturin develop` or `cargo test -p qmnf_bindings`. |
| `realtime_fhe` | `realtime_fhe/` | `rlib` | Minimal BFV FHE runtime leveraging QMNF noise. | Tests under `test_noise_validation.rs` + dedicated harnesses in repo root. |
| Standalone bundles | `standalone_extractions/qmnf-core`, `standalone_extractions/qmnf-rust-core` | Mixed workspace | Frozen copies for external partners (no dev work here). | Validate with upstream crates before snapshotting. |

All manifests target **Rust edition = "2021"**, keeping the toolchain on stable (`rustup default stable`). There is no `rust-toolchain` pin; ensure your local default stays aligned.

The PyO3 artifacts are produced as `hcvlang_pyo3*.so` in the repository root; keep `LD_LIBRARY_PATH` and `PYTHONPATH` pointing to the repo when testing (`see 00_START_HERE.md`).

---

## 4. Documentation & Evidence

- **Entry points**: `00_START_HERE.md`, `DEVELOPER_QUICK_START.md`, `README_UPDATED.md`, `REFACTORING_PROJECT_STATUS.md`.
- **Comprehensive catalogs**: `FILE_INVENTORY.md` (full file listing), `INDEX.md` (doc navigation), `COMPREHENSIVE_ARITHMETIC_CATALOG.md`, `QMNF_SYNERGISTIC_ARCHITECTURE.md`.
- **Phase reports**: `PHASE_1_*`, `PHASE2_*`, `PHASE3_*`, `PHASE4_*` files plus `SESSION_REPORT_*`.
- **Research & design**: `docs/architecture/`, `docs/guides/`, `docs/integration/`, `docs/mathematical/`, `FLOAT_PROHIBITION_*`, `GUARD_MECHANISMS_*`, `RUST_FLOAT_PREVENTION_ANALYSIS.md`.
- **Evidence archives**: `analysis/` (catalogs, manifests, supporting CSVs), `extreme_scale_plots/`, `benchmarks/README.md`.

Keep these synced with major code changes; doc drift is tracked in reviews.

---

## 5. Testing, Validation & Benchmarks

- **Python tests**: `tests/python/` mirrors package layout; top-level `tests/test_*` cover integration (e.g., harmonic fractal, arithmetic workflows).
- **Rust tests**: `hcvlang/tests/` focus on extreme scale correctness, transcendental coverage, and new PI cache suites (`test_pi_cache_demo.rs`, `test_pi_mismatch.rs`). Additional Rust validation lives under `test_entropy_shadow.rs`, `test_noise_validation.rs`, `test_qmnf_noise_validation.rs`, and crate-specific `tests/`.
- **Examples & demos**: `hcvlang/examples/` + `examples/` directory (Python + Rust) provide runnable benchmarks and cosmos demos (`qmnf_noise_demo`, `transcendental_demo.py`, etc.).
- **Benchmarks**: `hcvlang/benches/` (criterion), `benchmarks/`, `qmnf_benchmark_suite.py`, `hot_benchmark_phase2.py`, `quick_bench.py`, `milestone_benchmark.py`, `profiling_and_bottleneck_analysis.py`.
- **Specialized harnesses**: `holodrive_full_test.py`, `test_phase2_integration.py`, `validate_integration.py`, `test_entropy_shadow/` dataset-driven checks.

Run the appropriate suites (`pytest`, `cargo test`, `cargo bench`) whenever touching the corresponding layer.

---

## 6. Tooling, Automation & Operational Assets

- **`tools/`**: Guard/float validators, benchmark comparison scripts, data extraction utilities. These are pure Python helpers invoked during CI or manual audits.
- **`scripts/`**: Shell helpers for tmux, applying optimizations, and repeatable environment setup.
- **`dashboard/`**: AI Agents dashboard (FastAPI + templates) for monitoring services, benchmarks, and project metadata. Pairs with documentation in `DASHBOARD_*` files.
- **`analysis/`**: CSV manifests (`downloads_catalog`, `inventory/*`), forensic notes, bottleneck reports—all supporting evidence referenced by status docs.
- **`holodrive_phase2/`, `realtime_fhe/target/`, `extreme_scale_plots/`**: Contain generated assets for storage experiments, compiled FHE binaries, and visualization artifacts. Treat as derived data (check `.gitignore` before editing).

---

## 7. Standalone & Distribution Targets

- **`standalone_extractions/qmnf-core/`** – Multi-crate workspace (hcvlang, qmnf_crtbigint, qmnf_bindings, qmnf_fast_ops) prepared for partners who only need the Rust core. Update via the extraction scripts before releases.
- **`standalone_extractions/qmnf-rust-core/`** – Proprietary packaging (`qmnf-core` crate) with explicit `rust-version = 1.70`. Keep licensing details in sync with `LICENSE` and proprietary headers.
- **`qmnf_bindings/` & `hcvlang_pyo3.*`** – Ship these if consumers depend on Python wheels; prefer `maturin` for reproducible builds.
- **`qmnf.egg-info/`** – Metadata for the Python package when installed locally (`pip install -e .`).

---

## 8. Binary Artifacts & Environment Notes

- Shared libraries: `hcvlang_pyo3.cpython-313-x86_64-linux-gnu.so`, `hcvlang_pyo3.so` in repo root. Regenerate after Rust changes (`cargo build -p hcvlang --features python --release`).
- Virtual environment: `venv/` houses Python 3.13 site-packages (pip, pkg_resources, etc.). Not tracked upstream—recreate via `python3 -m venv venv && pip install -e .[dev]`.
- Cargo targets: `hcvlang/target/`, `realtime_fhe/target/`, etc., store build intermediates. Clean with `cargo clean` as needed.

Environment variables (per `00_START_HERE.md`):
```bash
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
export PYTHONPATH=.:$PYTHONPATH
```

---

## 9. Operational Checklist

1. **Toolchain**: Stable Rust (edition 2021) + Python 3.11/3.13. No nightly-required features.
2. **Testing Sequence**: `cargo fmt && cargo clippy --all-targets`, `cargo test --all`, `pytest tests -v`, targeted benchmarks as needed.
3. **Docs**: Update `INDEX.md`, `FILE_INVENTORY.md`, and relevant `docs/` entries when adding/removing subsystems.
4. **Extraction Pipelines**: When refreshing `standalone_extractions`, tag the source commit and document in `STANDALONE_REPOSITORIES_STATUS.md`.
5. **Dashboards & Evidence**: Regenerate `analysis/` manifests and `extreme_scale_plots/` when benchmark data changes.

Use this catalog as the authoritative map during reviews or onboarding; keep it updated whenever major resources move, are added, or are retired.
