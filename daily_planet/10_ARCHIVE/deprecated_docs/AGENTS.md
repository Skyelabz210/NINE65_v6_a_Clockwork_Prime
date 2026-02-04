---
title: "Agents"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/AGENTS.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# Repository Guidelines

---
title: "Agents"
description: "Placeholder description — please update."
authors:
	- "maintainer <maintainer@example.org>"
maintainers:
	- "See AGENTS.md"
tags: []
status: published
canonical_path: "/docs/AGENTS.md"
last_reviewed: 2025-11-07
version: "1.0"
references: []
---

## Project Structure & Module Organization
QMNF core logic lives under `qmnf/`, covering boundary guards, neural modules, storage backends, and cryptography utilities. Performance-critical primitives reside in `hcvlang/src/` with supporting `benches/` 
and `tests/`. Python integration tests sit in `tests/python/`, while Rust harnesses and wrappers live in `tests/rust/`. Reference material is collected in `docs/`, quick demos in `examples/`, and benchmark artifacts under `benchmarks/`. Tooling scripts that enforce invariants (float detection, boundary validation, benchmark generation) are in `tools/`.

## Build, Test, and Development Commands
Install dependencies with `pip install -e ".[dev]"`. Use `pre-commit install` to enable local hooks. Run `pytest tests/ -v` for the complete Python suite, or target Rust components via `cargo test` inside `hcvlang/`. Generate coverage by `pytest tests/ --cov=qmnf --cov-report=html`. The sanity umbrella `make quality-check` (or run `ruff check . && mypy qmnf && pytest tests/ -v`) should pass before a push. Execute `python milestone_benchmark.py` or `python tools/qmnf_benchmark_suite.py` when performance claims are affected.

## Coding Style & Naming Conventions
Python code follows PEP 8 with four-space indents, `snake_case` for functions/modules, and `PascalCase` for classes. Every public function requires type hints and Google-style docstrings; floats are prohibited—use `QMNFRational` helpers instead. Rust modules must stay formatted with `cargo fmt` and lint-clean under `cargo clippy --all-targets`. Guard decorators (`qmnf_guards.py`) must wrap any external interface to preserve integer-only guarantees.

## Testing Guidelines
Add or update tests beside the feature: unit tests belong in `tests/python/` mirroring package names (e.g., `tests/python/test_core_boundary.py`). Name test functions `test_<behavior>` and prefer property checks for arithmetic invariants. Validate new Rust primitives with unit tests under `hcvlang/tests/` and extend Python integration coverage when bridging Rust bindings. Provide coverage deltas or HTML reports when changes touch numerical kernels.

## Commit & Pull Request Guidelines
Adopt Conventional Commit prefixes (`feat:`, `fix:`, `docs:`, `perf:`) aligned with the current history. Keep commits scoped—tests and docs for one logical change. Pull requests must include a concise summary, linked issues or task IDs, test and benchmark outputs, and any security considerations (boundary guards, crypto paths). Confirm confidentiality constraints remain intact; avoid attaching proprietary artifacts or external datasets.
