# QMNF Audit Log (batch size 10)

- Generated: 2025-12-13T05:39:36
- Total files (excluding .git): 13394
- Batch size: 10; total batches: 1340

> Team A working copy. Team B uses `analysis/audit_package/ITERATIVE_AUDIT.md` to avoid overlap.

## Batch Index (all files, sorted alphabetically, then numeric, then special)
Batch 0001: ACHIEVEMENT_COMPLETION_SUMMARY.md, ACTUAL_PERFORMANCE_BENCHMARKS.md, ADMIN_ENABLE_RAPTOR.md, advanced_ai_capabilities_system.py, AGENT3_ADVANCED_TRAINING_REPORT.md, agent3_implementations/adversarial.rs, AGENT4_IMPLEMENTATION_REPORT.md, AGENT4_SERIALIZATION_BACKUP.rs, AGENT_PROMPT_TEMPLATE.md, AGENT_VALIDATION_CRITERIA.md
Batch 0002: AGENTS.md, AGI_SYSTEM_COMPENDIUM.md, AGI_SYSTEM_README.md, AHOP_TOY_VS_CRYPTO_EXPLORATION.md, AI_AGENT_CONFIGURATION_GUIDE.md, ai_agent_models.yml, analysis/inventory/archive_contents.csv, ANALYSIS_COMPLETION_SUMMARY.txt, API_ARCHITECTURE_MASTER_REPORT.md, API_CONSISTENCY_PATTERNS_REPORT.md
Batch 0003: API_DOCUMENTATION_ASSESSMENT.md, API_FINDINGS_SUMMARY.txt, ARCHITECTURAL_BOUNDARIES.md, ARCHITECTURE.md, ARCHITECTURE_DIAGRAM.txt, archive/2025-11-pre-benchmarking/ARCHIVE_SUMMARY.md, archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark.py, archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark_fixed.py, archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark_results.json, archive/2025-11-pre-benchmarking/old_summaries/ARCHITECTURAL_REVIEW_COMPLETION_SUMMARY.md
Batch 0004: ARITHMETIC_EXTRACTION_QUICKSTART.md, ARITHMETIC_IMPLEMENTATIONS_ANALYSIS.md, arithmetic_regression_analysis.py, arithmetic_regression_results.json, AUDIO_TERMINAL_UPGRADE_2025-12-02.md, AUDIT_EXECUTIVE_SUMMARY.txt, AUDIT_FINDINGS_QUICK_REFERENCE.md, AUDIT_INDEX.md, AUDIT_QUICK_REFERENCE.txt, AUDIT_README.md
Batch 0005: AUDIT_RECONCILIATION_2025-12-01.md, AUDIT_SUMMARY.json, crates/qmnf-arithmetic/benches/arithmetic_benchmark.rs, cryptographic_systems/01_BFV_Core_FHE/essential_mathematics/adaptive_crt_bigint.rs, cryptographic_systems/02_BFV_Realtime_FHE/ACTUAL_PERFORMANCE_RESULTS.md, cryptographic_systems/02_BFV_Realtime_FHE/essential_mathematics/adaptive_crt_bigint.rs, cryptographic_systems/02_BFV_Realtime_FHE/fhe_realtime/adaptive_polynomial.rs, cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/ahop_benchmark.py, cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/ahop_operations_bench.py, cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/results/ahop_operations_results.json
Batch 0006: cryptographic_systems/04_AHOP_Unified_FHE/examples/advanced_operations.py, cryptographic_systems/04_AHOP_Unified_FHE/examples/ahop_unified_demo.py, cryptographic_systems/07_MAA_Cryptosystem/novel_mathematics/apollonian.rs, cryptographic_systems/08_ACC_Cryptosystem/acc_crypto_python/acc_fhe_complete.py, cryptographic_systems/08_ACC_Cryptosystem/examples/acc_demo.py, cryptographic_systems/ARCHITECTURE_REVIEW_REPORT.md, dashboard/api_routes.py, dashboard/templates/ai.html, docs/ADAPTIVE_CRT_BENCHMARK_REPORT.md, docs/ADVANCED_ARITHMETIC_CATALOG.md
Batch 0007: docs/ARITHMETIC_STACK_COMPREHENSIVE_REVIEW.md, docs/AUDIT_AND_SORT_REPORT.md, docs/guides/acc_deployment_guide.md, docs/integration/acc_deployment_guide.md, docs/mathematical/AHOP_FHE_SECURITY_PROOFS.md, experiments/quantum_classical_divide/anchor_coordination_test.py, experiments/quantum_classical_divide/ANCHOR_FIRST_Q_AND_A.md, experiments/research/resnet/ablation_studies.py, experiments/research/resnet/ABLATION_STUDY_REPORT.md, experiments/research/resnet/active_learning_experiment.rs
Batch 0008: experiments/research/resnet/adversarial_experiment.rs, experiments/research/resnet/AGENT_2_COMPLETION_REPORT.md, experiments/research/resnet/AGENT_6_COMPLETION_REPORT.md, experiments/research/resnet/API_SERVER_IMPLEMENTATION_REPORT.md, experiments/research/resnet/experiments/active_learning_demo.rs, experiments/research/resnet/experiments/adversarial_robustness.rs, experiments/research/resnet/results/ablation_analysis.md, experiments/research/resnet/results/ABLATION_SUMMARY.md, hcvlang/ADAPTIVE_CRT_INTEGRATION_STATUS.md, hcvlang/adaptive_crt_test
Batch 0009: hcvlang/AVX2_SIMD_API_SUMMARY.md, hcvlang/benches/adaptive_crt_benchmark.rs, hcvlang/core/geometry/apollonian.rs, hcvlang/examples/ANTI_PATTERNS_README.md, hcvlang/experiments/resnet/benchmarking/anchor_first_ablation.rs, hcvlang/experiments/resnet/results/ablation_study_2025-11-18.md, hcvlang/experiments/resnet/results/ABLATION_STUDY_OVERVIEW.md, hcvlang/experiments/resnet/results/advanced_research_2025-11-18.md, hcvlang/python/ahop_fhe_noise.py, hcvlang/src/adaptive_crt_bigint.rs
Batch 0010: hcvlang/src/adaptive_crt_bigint_v1.rs, hcvlang/src/adaptive_crt_bigint_v2.rs, hcvlang/src/adaptive_crt_bigint_v3.rs, hcvlang/src/ahop.rs, hcvlang/src/apollonian.rs, hcvlang/src/attractor_memory.rs, hcvlang/src/diagnostics/anomaly_detection.rs, hcvlang/src/fhe_realtime/adaptive_polynomial.rs, hcvlang/src/math/apollonian_homomorphic.rs, hcvlang/src/neural/adversarial.rs

### Batch 0001 review (files 1-10)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `ACHIEVEMENT_COMPLETION_SUMMARY.md` | Narrative summary of residue-space ResNet breakthrough and validation. | Math/perf benchmarks described (validation suites, throughput numbers); no physical benchmarks. | Doc only; no float-bearing code; lint/naming N/A; no FFI touches. | None detected; informational only. |
| `ACTUAL_PERFORMANCE_BENCHMARKS.md` | Documented cargo bench outputs and performance analysis for residue-space networks. | Contains detailed mathematical/performance benchmark results; no physical benchmarks. | Doc only; no code to lint; no FFI. | None detected; keep as reference. |
| `ADMIN_ENABLE_RAPTOR.md` | Admin guide to enable raptor-mini-preview model access for agents. | No benchmarks. | Doc only; no code; no FFI. | None detected. |
| `advanced_ai_capabilities_system.py` | Phase 4 advanced AI orchestration (multi-modal processing, knowledge graph, capability discovery). | No embedded benchmark routines; collects stats only. | Float usage and conversions present (to_float(), processing ratios, float samples, time math) plus float type checks; inconsistent import `from qmnf_boundary` vs `from qmnf.boundary` and shebang ordering; lint/docstrings incomplete; no FFI guards. | Requires deeper remediation to purge floats, align boundary import, restore integer-only policy, and add guards/tests. |
| `AGENT3_ADVANCED_TRAINING_REPORT.md` | Report summarizing multi-shot, adversarial, and active learning protocol implementations. | Complexity/performance notes; no executable benchmarks. | Doc only; no code; no FFI. | None detected. |
| `agent3_implementations/adversarial.rs` | Rust FGSM adversarial generator and trainer for residue-space networks. | Performance characteristics described; no benchmark harness here. | Uses f64 in robustness scoring and Option<f64> style metrics, violating float prohibition; otherwise integer modular ops; linting likely to flag float arithmetic; no FFI. | Needs refactor to rational/integer metrics and compliance audit (mark for further analysis). |
| `AGENT4_IMPLEMENTATION_REPORT.md` | Status report for model serialization and verification modules. | No benchmarks. | Doc only; mentions f64 types conceptually but not code here; no FFI. | None detected in document. |
| `AGENT4_SERIALIZATION_BACKUP.rs` | Rust backup serialization/deserialization for ResNet with checksums. | No benchmark logic. | Contains accuracy Option<f64> and with_accuracy using f64, conflicting with float ban; placeholder template/labels indicate unfinished implementation; lint otherwise standard; no FFI. | Mark for follow-up to replace floats, complete TODOs, and add tests. |
| `AGENT_PROMPT_TEMPLATE.md` | Prompt template instructing agents on residue-space architectural mandates. | No benchmarks. | Doc only; no code; no FFI. | None detected. |
| `AGENT_VALIDATION_CRITERIA.md` | Validation criteria for agents dealing with residue-space operations. | No benchmarks. | Doc only; no code; no FFI. | None detected. |

### Batch 0002 review (files 11-20)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | Dual front-matter plus repository guidelines for structure, build/test, style. | No direct benchmarks; references standard test/coverage commands. | Doc only; duplicate YAML blocks with mixed tabs/spaces; no code/FFI. | Minor cleanup possible (dedupe front-matter) but otherwise informational. |
| `AGI_SYSTEM_COMPENDIUM.md` | High-level AGI compendium describing architecture, benchmarks, deployment. | Contains reported performance/validation stats; no executable benchmarks; no physical metrics. | Doc only; no code; no FFI. | None detected. |
| `AGI_SYSTEM_README.md` | Marketing-style README for QMS with claims and install steps. | Mentions performance metrics; no runnable benchmarks here. | Doc only; shields URLs and scripts; no FFI. | None detected; content is non-technical. |
| `AHOP_TOY_VS_CRYPTO_EXPLORATION.md` | Exploration notes comparing toy vs cryptographic AHOP parameters. | Lists timing observations for toy params; no physical benchmarks. | Doc only; code snippets illustrative; no FFI. | None detected. |
| `AI_AGENT_CONFIGURATION_GUIDE.md` | Guidance for agent configuration respecting residue-space architecture. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `ai_agent_models.yml` | YAML config recommending `raptor-mini-preview` model defaults. | No benchmarks. | Config only; no floats/FFI; values placeholder base_url. | None detected. |
| `analysis/inventory/archive_contents.csv` | CSV inventory of archive contents and sizes. | Contains file sizes; no benchmarks. | Data only; no code/FFI. | None detected. |
| `ANALYSIS_COMPLETION_SUMMARY.txt` | Float prohibition reevaluation summary and supporting doc list. | Cites performance numbers; no runnable benchmarks. | Doc only; no code/FFI. | None detected. |
| `API_ARCHITECTURE_MASTER_REPORT.md` | Master report on API architecture, FFI counts, grades. | Reports benchmark/FFI stats; no runnable benchmarks. | Doc only; no code/FFI. | None detected. |
| `API_CONSISTENCY_PATTERNS_REPORT.md` | Report on API naming/consistency patterns across stack. | No benchmarks. | Doc only; no code/FFI. | None detected. |

### Batch 0003 review (files 21-30)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `API_DOCUMENTATION_ASSESSMENT.md` | Assessment of API documentation coverage/quality. | Reports coverage stats; no runnable benchmarks. | Doc only; no code/FFI. | None detected. |
| `API_FINDINGS_SUMMARY.txt` | Summary of API infrastructure findings and endpoint coverage. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `ARCHITECTURAL_BOUNDARIES.md` | Hardware/performance tier boundaries with exactness guarantees. | Contains performance claims; no runnable benchmarks. | Doc only; no code/FFI. | None detected. |
| `ARCHITECTURE.md` | High-level system architecture overview and float prohibition notes. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `ARCHITECTURE_DIAGRAM.txt` | Text diagram of system/API structure. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `archive/2025-11-pre-benchmarking/ARCHIVE_SUMMARY.md` | Summary of files archived before benchmarking revamp. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark.py` | Legacy comprehensive arithmetic benchmark script. | Benchmark harness using hcvlang_pyo3/QMNF modules. | Uses floats (time/perf counters, ops/sec), no type hints, legacy; FFI import `hcvlang_pyo3` without guards. | Mark for further analysis/modernization or retire (archived legacy). |
| `archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark_fixed.py` | Legacy fixed benchmark script for core arithmetic. | Benchmark harness. | Uses floats for timing/ops/sec; minimal typing/linting; FFI `hcvlang_pyo3` without guards. | Mark for further analysis/cleanup or retire (archived legacy). |
| `archive/2025-11-pre-benchmarking/old_benchmarks/arithmetic_benchmark_results.json` | Stored benchmark results from legacy run. | Benchmark output data. | Data only; no code/FFI. | None detected. |
| `archive/2025-11-pre-benchmarking/old_summaries/ARCHITECTURAL_REVIEW_COMPLETION_SUMMARY.md` | Summary of completed architectural review tasks. | No benchmarks. | Doc only; no code/FFI. | None detected. |

### Batch 0004 review (files 31-40)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `ARITHMETIC_EXTRACTION_QUICKSTART.md` | Quickstart guide for arithmetic extraction. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `ARITHMETIC_IMPLEMENTATIONS_ANALYSIS.md` | Analysis of arithmetic implementations. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `arithmetic_regression_analysis.py` | Regression benchmark script comparing arithmetic performance to baselines. | Runs benchmarks via hcvlang_pyo3; writes JSON results. | Heavy float usage (time/ops/sec/ratios), no type hints, no guard decorators; FFI imports unguarded. | Mark for further analysis/refactor: replace floats with rational metrics where possible, add guards/linting, decide if float timing acceptable in bench context. |
| `arithmetic_regression_results.json` | Stored regression benchmark output. | Benchmark data. | Data only; no code/FFI. | None detected. |
| `AUDIO_TERMINAL_UPGRADE_2025-12-02.md` | Audio terminal upgrade note. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_EXECUTIVE_SUMMARY.txt` | Executive summary of audit findings. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_FINDINGS_QUICK_REFERENCE.md` | Quick reference of audit findings. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_INDEX.md` | Index of audit documents. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_QUICK_REFERENCE.txt` | Audit quick reference. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_README.md` | Audit README. | No benchmarks. | Doc only; no code/FFI. | None detected. |

### Batch 0005 review (files 41-50)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `AUDIT_RECONCILIATION_2025-12-01.md` | Audit reconciliation report (Dec 1) comparing fixes vs issues. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `AUDIT_SUMMARY.json` | JSON summary of audit status, build/test results, discrepancies. | No benchmarks. | Data only; no code/FFI. | None detected. |
| `crates/qmnf-primitives/benches/bigint_benchmark.rs` | Criterion benchmarks for HCVLangBigInt add/mul/gcd. | Criterion benchmarks. | Uses standard Criterion (floats in timing); integer-only operations; no FFI; lint/format standard. | Acceptable for benches; optionally add float-policy note if strict. |
| `cryptographic_systems/01_BFV_Core_FHE/essential_mathematics/adaptive_crt_bigint.rs` | Adaptive CRT bigint with tier management and integration points. | No benchmarks inside. | Integer-only logic; relies on integration TODOs; no explicit floats; no FFI. | Integration points/TODOs remain—needs completion/testing; otherwise compliant. |
| `cryptographic_systems/02_BFV_Realtime_FHE/ACTUAL_PERFORMANCE_RESULTS.md` | Actual performance results falsifying sub-ms claim. | Benchmark results. | Doc only; no code/FFI. | None detected. |
| `cryptographic_systems/02_BFV_Realtime_FHE/essential_mathematics/adaptive_crt_bigint.rs` | Duplicate adaptive CRT bigint module (Realtime FHE path). | No benchmarks inside. | Same as core version: integer-only; integration points TODOs; no floats/FFI. | Integration TODOs remain; ensure single canonical copy to avoid divergence. |
| `cryptographic_systems/02_BFV_Realtime_FHE/fhe_realtime/adaptive_polynomial.rs` | Adaptive polynomial operations for realtime FHE. | Not reviewed (pending). | Pending. | Pending analysis. |
| `cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/ahop_benchmark.py` | AHOP benchmark script (Python). | Benchmark harness. | Likely uses floats/timing and FFI; not reviewed yet. | Pending analysis. |
| `cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/ahop_operations_bench.py` | AHOP operations benchmark (Python). | Benchmark harness. | Likely uses floats/timing and FFI; not reviewed yet. | Pending analysis. |
| `cryptographic_systems/04_AHOP_Unified_FHE/benchmarks/results/ahop_operations_results.json` | AHOP benchmark results. | Benchmark data. | Data only. | None detected. |

### Batch 0006 review (files 51-60)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `cryptographic_systems/04_AHOP_Unified_FHE/examples/advanced_operations.py` | AHOP unified FHE advanced operations demo (RNS-Montgomery polynomials). | Demo only. | Integer-only; uses time import but no float metrics; FFI via sys.path injection; no guards. | None critical; optional guard/type-hint/ruff cleanup. |
| `cryptographic_systems/04_AHOP_Unified_FHE/examples/ahop_unified_demo.py` | AHOP unified FHE demo (encrypt/add/mul/invert/divide). | Demo only. | Integer-only; minimal error handling; FFI via sys.path injection; no guards. | None critical; optional guards/typing and structured logging. |
| `cryptographic_systems/07_MAA_Cryptosystem/novel_mathematics/apollonian.rs` | Apollonian gasket primitives using ModRational/CRTBigInt. | No benchmarks. | Integer-only; no floats; no FFI. | Looks compliant; ensure tests cover geometry invariants. |
| `cryptographic_systems/08_ACC_Cryptosystem/acc_crypto_python/acc_fhe_complete.py` | ACC FHE implementation placeholder with cylindrical time integration. | No benchmarks. | File malformed (smart quotes, fenced code inside docstring) and likely non-runnable; imports time; no guards. | Major: fix encoding/quoting, restore valid Python, add tests/guards; float ban likely ok but file broken. |
| `cryptographic_systems/08_ACC_Cryptosystem/examples/acc_demo.py` | ACC cryptosystem demo. | Demo only. | Uses float `sigma=3.2` for Gaussian sampler (violates float ban); otherwise integer prints; sys.path injection for imports. | Replace float sigma with rational/int config; add guards/typing; consider compliance policy. |
| `cryptographic_systems/ARCHITECTURE_REVIEW_REPORT.md` | Architecture review report. | No benchmarks. | Doc only; no code/FFI. | None detected. |
| `dashboard/api_routes.py` | Flask API routes for dashboard. | No benchmarks. | Uses time.time() floats for timestamps; no guards; minimal validation. | Add guard layer/validation; float timing acceptable if retained for API latency. |
| `dashboard/templates/ai.html` | Jinja template for AI dashboard. | No benchmarks. | Template only. | None detected. |
| `docs/ADAPTIVE_CRT_BENCHMARK_REPORT.md` | Benchmark report for adaptive CRT. | Benchmark results. | Doc only. | None detected. |
| `docs/ADVANCED_ARITHMETIC_CATALOG.md` | Catalog of advanced arithmetic modules. | No benchmarks. | Doc only. | None detected. |

### Batch 0007 review (files 61-70)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `docs/ARITHMETIC_STACK_COMPREHENSIVE_REVIEW.md` | Comprehensive review of arithmetic stack. | No benchmarks. | Doc only. | None detected. |
| `docs/AUDIT_AND_SORT_REPORT.md` | Report on audit/sort tooling and plan. | No benchmarks. | Doc only. | None detected. |
| `docs/guides/acc_deployment_guide.md` | ACC deployment guide. | No benchmarks. | Doc only. | None detected. |
| `docs/integration/acc_deployment_guide.md` | ACC deployment guide (integration). | No benchmarks. | Doc only. | None detected. |
| `docs/mathematical/AHOP_FHE_SECURITY_PROOFS.md` | AHOP security proofs. | No benchmarks. | Doc only. | None detected. |
| `experiments/quantum_classical_divide/anchor_coordination_test.py` | Anchor-first coordination validation script. | Timing comparisons. | Uses floats for timing/speedup (`float('inf')`), no guards/type hints. | Refactor to rational metrics or mark as benchmark-only; add guards/typing. |
| `experiments/quantum_classical_divide/ANCHOR_FIRST_Q_AND_A.md` | Q&A on anchor-first approach. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/ablation_studies.py` | Ablation experiments for ResNet (MNIST). | Benchmarks/timing, saves JSON/MD. | Heavy float usage (accuracy ratios, timing), no guards; relies on MNIST loaders. | Refactor to integer-safe metrics or classify as benchmark; add guards/typing. |
| `experiments/research/resnet/ABLATION_STUDY_REPORT.md` | Ablation study report. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/active_learning_experiment.rs` | Active learning experiment in Rust. | Benchmarks/learning curves. | Uses f64 accuracy/threshold formatting; float-heavy; no feature guards. | Violates float ban; needs rational/int metrics and guard policy. |

### Batch 0008 review (files 71-80)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `experiments/research/resnet/adversarial_experiment.rs` | Adversarial training experiment. | Benchmarks robustness. | Uses f64 for accuracy/ratios; float-heavy; no guards. | Violates float ban; convert metrics to rational/int, add guards. |
| `experiments/research/resnet/AGENT_2_COMPLETION_REPORT.md` | Agent 2 completion report. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/AGENT_6_COMPLETION_REPORT.md` | Agent 6 completion report. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/API_SERVER_IMPLEMENTATION_REPORT.md` | API server implementation report. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/experiments/active_learning_demo.rs` | Active learning demo. | Benchmarks/learning curves. | Uses f64 metrics, sqrt, float percentages; no guards. | Violates float ban; convert to rational/int metrics, add guards. |
| `experiments/research/resnet/experiments/adversarial_robustness.rs` | Adversarial robustness test. | Benchmarks robustness. | Uses f64 for accuracy/confidence; no guards. | Violates float ban; convert metrics to rational/int, add guards. |
| `experiments/research/resnet/results/ablation_analysis.md` | Ablation analysis doc. | No benchmarks. | Doc only. | None detected. |
| `experiments/research/resnet/results/ABLATION_SUMMARY.md` | Ablation summary doc. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/ADAPTIVE_CRT_INTEGRATION_STATUS.md` | Adaptive CRT integration status. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/adaptive_crt_test` | Binary test artifact. | Executable artifact. | Binary; not source; should not be in repo. | Remove/ignore in source audit; regenerate as needed from source. |

### Batch 0009 review (files 81-90)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `hcvlang/AVX2_SIMD_API_SUMMARY.md` | AVX2 SIMD API summary. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/benches/adaptive_crt_benchmark.rs` | Criterion benchmarks for adaptive CRT tiers and Montgomery baseline. | Criterion bench. | Uses float timing inherent to Criterion; integer operations; no guards. | Acceptable as bench; add float-policy note if strict. |
| `hcvlang/core/geometry/apollonian.rs` | PyO3 Descartes circle bindings. | No benchmarks. | Accepts ints/floats/ModRational; core uses BigInt; float inputs allowed → boundary risk; no explicit guards. | Add boundary guards/normalization to rationals; enforce integer-only inputs. |
| `hcvlang/examples/ANTI_PATTERNS_README.md` | Anti-pattern notes. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/experiments/resnet/benchmarking/anchor_first_ablation.rs` | Anchor-first speedup ablation. | Benchmarks. | Heavy f64 usage for timing/speedup; no guards. | Mark as benchmark-only or refactor to rational/int metrics with guards. |
| `hcvlang/experiments/resnet/results/ablation_study_2025-11-18.md` | Ablation study report. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/experiments/resnet/results/ABLATION_STUDY_OVERVIEW.md` | Ablation study overview. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/experiments/resnet/results/advanced_research_2025-11-18.md` | Advanced research report. | No benchmarks. | Doc only. | None detected. |
| `hcvlang/python/ahop_fhe_noise.py` | AHOP-based FHE noise generator. | No benchmarks. | Integer-only; no float use; uses ANSI logging; no guards. | Add guard/decorators if exposed externally; otherwise compliant. |
| `hcvlang/src/adaptive_crt_bigint.rs` | Adaptive CRT bigint core. | No benchmarks. | Integer-only; integration points TODOs; no floats. | Complete integration points/tests; ensure single canonical copy. |

### Batch 0010 review (files 91-100)

| File | Description | Benchmarks | Float / standards / FFI check | Issues / next steps |
| --- | --- | --- | --- | --- |
| `hcvlang/src/adaptive_crt_bigint_v1.rs` | Adaptive CRT bigint (v1 variant). | No benchmarks. | Integer-only; integration duplicated vs v2/v3; no floats. | Consolidate variants, ensure single canonical; add tests/guards. |
| `hcvlang/src/adaptive_crt_bigint_v2.rs` | Adaptive CRT bigint (v2 variant with calibration notes). | No benchmarks. | Integer-only; duplicate functionality; no floats. | Same: consolidate/remove duplicates; ensure tests. |
| `hcvlang/src/adaptive_crt_bigint_v3.rs` | Adaptive CRT bigint (v3) with forbid unsafe/float lint. | No benchmarks. | Integer-only; to_i128 magnitude estimation fallback; no floats. | Keep as canonical; remove older variants after consolidation. |
| `hcvlang/src/ahop.rs` | AHOP Unified FHE context (placeholder). | No benchmarks. | Integer-only; placeholders/stubs; no floats; no guards. | Needs full implementation, guards, tests; clarify FFI exposure. |
| `hcvlang/src/apollonian.rs` | Apollonian gasket primitives (core). | No benchmarks. | Integer-only; float_arithmetic denied; partial stub comments. | Ensure implementation complete/tests; verify no float ingress. |
| `hcvlang/src/attractor_memory.rs` | Attractor memory system. | No benchmarks. | Integer-only; clippy float_arithmetic denied; uses fixed-point; no floats. | Looks compliant; ensure tests for dynamics. |
| `hcvlang/src/diagnostics/anomaly_detection.rs` | Multi-scale anomaly detection. | No benchmarks. | Integer-only fixed-point; no floats; no guards. | Add guards if external input; add property tests. |
| `hcvlang/src/fhe_realtime/adaptive_polynomial.rs` | Adaptive polynomial coefficients using AdaptiveCRTBigInt. | No benchmarks. | Integer-only; maps to ModInt; no floats; no guards. | Add guards/tests; ensure modulus checks. |
| `hcvlang/src/math/apollonian_homomorphic.rs` | Homomorphic apollonian arithmetic. | No benchmarks. | Integer-only; uses ModInt/ModRational; no floats; no guards. | Add tests/guards; verify Tonelli/roots handling. |
| `hcvlang/src/neural/adversarial.rs` | Residue-space adversarial training (FGSM). | No benchmarks. | Integer-only; no floats; no guards. | Add guards/typing; ensure consensus scoring exposure is safe. |
