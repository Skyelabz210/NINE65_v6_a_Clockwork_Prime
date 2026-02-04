# QMNF System Knowledge Base

> Auto-generated index - Last updated: 2025-12-16 21:05

## Quick Navigation

- [[MOC-Architecture|Architecture]] - System design and core components
- [[MOC-Implementation|Implementation]] - Rust codebase
- [[MOC-Python|Python]] - Python package and bindings
- [[MOC-Sessions|Sessions]] - Analysis and session reports
- [[MOC-Documentation|Documentation]] - System docs
- [[MOC-Tooling|Tooling]] - Development tools

## Current State

### Key Files
- [[analysis/QMNF_SYSTEM_GAP_REPORT_2025-12-15.md|Gap Report]] - Production readiness analysis
- [[analysis/SESSION_REPORT_2025-12-15_Claude.md|Latest Session]] - Current status
- [[CLAUDE.md]] - Development guide

### Priority: FHE Revenue Path
The critical path to revenue generation through FHE:

1. **FFI Bridge** - Rust to Python bindings
   - Current: 9 symbols exported
   - Target: Full FHE surface

2. **Core Arithmetic** - Integer-only foundation
   - [[crates/qmnf-arithmetic|qmnf-arithmetic]]
   - [[crates/qmnf-primitives|qmnf-primitives]]

3. **FHE Module** - Homomorphic encryption
   - [[crates/qmnf-fhe|qmnf-fhe]]
   - [[hcvlang/src/fhe|hcvlang FHE]]

## Statistics

- **Rust modules**: 267
- **Python modules**: 122
- **Documentation files**: 138

## Tags

Common tags used in this vault:
- `#blocker` - Blocking issues
- `#decision-needed` - Requires decision
- `#fhe` - FHE related
- `#ffi` - FFI/bindings related
- `#revenue-critical` - On critical path

