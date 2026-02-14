# FORENSIC AUDIT: Build 18_hackfate

**Audit Date**: 2026-02-14
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/18_hackfate/`
**Build Type**: HackFate public website (hackfate.us) + benchmark evidence + formal proofs
**Domain**: hackfate.us (GitHub Pages, via CNAME)
**Git Remote**: https://github.com/DeuxAxios/hackfate.git (branch: main)
**Auditor**: Forensic Code Auditor (Claude Opus 4.6)

---

## TABLE OF CONTENTS

1. [Structure Mapping](#1-structure-mapping)
2. [Content Analysis](#2-content-analysis)
3. [Evidence Verification](#3-evidence-verification)
4. [Link Integrity](#4-link-integrity)
5. [Dead Content](#5-dead-content)
6. [Cross-Reference with Other Builds](#6-cross-reference-with-other-builds)
7. [Anomaly Catalogue](#7-anomaly-catalogue)
8. [Summary Verdict](#8-summary-verdict)

---

## 1. STRUCTURE MAPPING

### 1.1 File Tree Summary

```
18_hackfate/
├── HTML Pages (14 files)
│   ├── index.html              (Homepage / landing)
│   ├── research.html           (Research grid, JS-populated)
│   ├── technology.html         (Technology grid, JS-populated)
│   ├── innovations.html        (Innovation catalog, JS-populated)
│   ├── benchmarks.html         (Benchmark summary, JS-populated)
│   ├── benchmarks-detailed.html (Detailed audit benchmarks, hardcoded tables)
│   ├── proofs.html             (Formal verification proofs, mixed JS/hardcoded)
│   ├── nine65-saas.html        (SaaS product page, hardcoded)
│   ├── walkthrough.html        (Developer tutorial, hardcoded Rust code)
│   ├── shadow-entropy.html     (Shadow Entropy deep-dive, hardcoded)
│   ├── about.html              (About page)
│   ├── contact.html            (Contact information)
│   ├── privacy.html            (Privacy policy)
│   └── 404.html                (Custom error page)
│
├── Assets (5 files)
│   ├── styles.css              (2105 lines, dark cyberpunk theme)
│   ├── script.js               (559 lines, data loaders + animations)
│   ├── favicon.svg
│   ├── favicon-32x32.png
│   ├── apple-touch-icon.png
│   ├── og-image.png
│   └── og-image.svg
│
├── Data Files (4 files)
│   ├── reports/inventory_public.json   (13 innovation entries)
│   ├── reports/benchmarks_curated.json (6 benchmark entries)
│   ├── reports/evidence_map.csv        (11 evidence rows)
│   └── reports/disclosure_matrix.csv   (12 disclosure rows)
│
├── Reports (17 files in reports/)
│   ├── hackfate-claims-verification.md
│   ├── hackfate-claims-verification.docx
│   ├── nine65-v5-technical-audit.md
│   ├── nine65-v5-technical-audit-latest.md
│   ├── nine65-v5-benchmark-results.md
│   ├── nine65-v5-benchmark-results-latest.md
│   ├── nine65-v5-operations-testing.md
│   ├── nine65-v5-operations-testing-latest.md
│   ├── nine65-v5-test-results-summary.md
│   ├── nine65-v5-test-summary-latest.md
│   ├── k-elimination-technical-paper.pdf
│   ├── test-nine65-fhe.pdf
│   └── public_links.md
│
├── Formal Proofs (50+ files)
│   ├── proofs/coq/              (19 .v files + compiled artifacts)
│   ├── proofs/lean4/            (28+ .lean files + lakefile/manifest)
│   ├── proofs/nist/             (14 .lean files)
│   ├── proofs/critiques/        (7 critique rounds .md)
│   ├── proofs/tests/            (3 JSON results + 3 Python test scripts)
│   ├── proofs/README.md
│   ├── proofs/theorem_stack.md
│   └── proofs/shadow_entropy_blueprint.json
│
├── Planning Documents (6 .md files)
│   ├── BENCHMARK_EVIDENCE.md
│   ├── ENHANCEMENT_PLAN.md
│   ├── EXECUTION_PLAN.md
│   ├── GRANDMASTER_REPORT.md
│   ├── INNOVATION_CATALOG_PUBLIC.md
│   ├── INSIGHT_LOG.md
│   └── QUESTION_MATRIX.md
│
├── Infrastructure
│   ├── CNAME                   (hackfate.us)
│   ├── sitemap.xml             (12 URLs)
│   ├── robots.txt
│   ├── .gitignore
│   ├── README.md
│   └── scripts/validate-inventory.js
│
└── .git/                       (full git repo)
```

### 1.2 File Counts

| Category | Count |
|----------|-------|
| HTML pages | 14 |
| CSS files | 1 |
| JS files | 2 (script.js + validate-inventory.js) |
| JSON data files | 4 (+3 test results, +1 blueprint, +1 lake-manifest) |
| CSV data files | 2 |
| Markdown files | 22 |
| PDF files | 2 |
| DOCX files | 1 |
| Image assets | 4 (2 PNG, 1 SVG favicon, 1 SVG og-image) |
| Coq proof files (.v) | 19 |
| Lean4 proof files (.lean) | 42+ |
| Python test scripts | 3 |
| Total non-.git files | ~120 |

---

## 2. CONTENT ANALYSIS

### 2.1 Page-by-Page Breakdown

#### index.html (Homepage)
- **Purpose**: Landing page with hero section, overview cards, SaaS highlight, contact preview
- **Claims made**:
  - "Bootstrap-free homomorphic encryption"
  - "Formally verified in Coq and Lean4"
  - "Open Benchmarks"
  - "Sub-Millisecond Latency" with "Depth-50 circuits in 139ms with zero bootstraps"
  - "627 passing tests across the workspace"
- **Data source**: Static HTML; overview cards link to subpages
- **Navigation**: 8 nav links (Research, Technology, Innovations, Benchmarks, Proofs, SaaS, About, Contact)

#### research.html
- **Purpose**: Research grid populated dynamically from `inventory_public.json`
- **Content**: Filters for `disclosure === 'Public'` and `tier === 'Proved' || 'Validated'`
- **When empty**: Grid renders empty if JSON fails to load

#### technology.html
- **Purpose**: Technology overview grouped by `system` field from inventory
- **Content**: Dynamically populated; groups items by system (QMNF, NINE65, MYSTIC, etc.)

#### innovations.html
- **Purpose**: Full innovation catalog of all public items
- **Content**: Dynamically populated from `inventory_public.json`, filtered by `disclosure === 'Public'`

#### benchmarks.html
- **Purpose**: Curated benchmark cards populated from `benchmarks_curated.json`
- **Content**: 6 benchmark entries displayed as cards
- **Link to detail**: Points to `benchmarks-detailed.html#audit-2026`

#### benchmarks-detailed.html
- **Purpose**: Comprehensive benchmark tables including audit snapshot, batch/parallel data, FHE operations, deep circuits, K-Elimination, security validation, methodology
- **Key claims**:
  - 627/627 workspace tests (446 nine65 + 46 clockwork-core + 30 nexgen_rational + 95 mana + 10 unhal)
  - Parallel encrypt 10x: 28.10ms (~4.9x speedup)
  - Batch decode 64: 1.378us (46.45 Melem/s)
  - Encryption: 4.76ms, Decryption: 2.16ms, HomAdd: 0.19ms, HomMul: 26.51ms
  - Depth-50 circuit: 1.32s, zero bootstraps
  - Deep circuit comparison table with "Traditional FHE*" estimates
  - Lattice estimator security: secure_128 (N=4096, 123.6-bit), secure_192 (N=8192, 165.6-bit), secure_256 (N=16384, 268.1-bit)
  - Test environment: Debian Linux, Intel Core i7-3632QM @ 2.20GHz, Rust 1.90.0

#### proofs.html
- **Purpose**: Formal verification hub with proof grid (JS-populated), walkthroughs, verification status
- **Hardcoded claims**:
  - 19 Coq proof files, 0 admitted statements
  - 28 core + 14 NIST Lean4 proof files, 0 sorry statements
  - 3 axioms used (Core-SVP hardness, Ring-LWE to SVP reduction, BKZ cost model)
  - NIST Category 5 compliance
- **Links to**: Shadow entropy walkthrough, theorem stack, innovations page
- **GitHub link**: `https://github.com/DeuxAxios/hackfate` (clone instructions)

#### nine65-saas.html
- **Purpose**: Product/marketing page positioning NINE65 FHE v5 as a SaaS privacy solution
- **Key claims**:
  - "Sub-Millisecond Latency" (< 1ms core ops)
  - "Symmetric Mul (D50): 139ms" (zero bootstraps)
  - "CPU Performance: 3-6x Faster" vs Industry Standard
  - "Memory Footprint: ~200 MB" vs "1.5 - 3.3 GB"
  - "Throughput: 10-25M ops/s" vs "1-5M ops/s"
  - "READY FOR DEPLOYMENT" status badge
  - Roadmap: Q1 2026 developer ecosystem, Q2-Q3 2026 SDK, 2027 FIPS 140-3

#### walkthrough.html
- **Purpose**: Developer-facing hands-on tutorial with Rust code examples
- **Content**: 8 sections covering setup through deep circuits
- **Code quality**: Syntactically plausible Rust code using `nine65::prelude::*`
- **References**: 3 external citations (homomorphicencryption.org, ePrint 2025/473, ACM)

#### shadow-entropy.html
- **Purpose**: Technical deep-dive on Shadow Entropy innovation
- **Content**: Core insight, how-it-works steps, properties, performance comparison, FHE integration, formal verification links
- **Claims**: ~30 Kbits/sec entropy rate, near-zero CPU cost, 5-50x fewer cycles than CSPRNG
- **Proof links**: Points to 5 proof files in proofs/ directory (all present in build)

#### about.html
- **Purpose**: Minimal about page for Anthony Diaz (Founder)
- **Content**: Brief description, technical focus areas list

#### contact.html
- **Purpose**: Contact information
- **Content**: Email (founder@hackfate.us), GitHub (@Skyelabz210), inquiry types

#### privacy.html
- **Purpose**: Privacy policy
- **Content**: No cookies, no analytics, no data collection. Last updated Jan 21, 2026
- **Meta**: Has `noindex` robots directive (correct for privacy pages)

#### 404.html
- **Purpose**: Custom error page
- **Content**: Glitch-styled "404" with navigation links
- **Note**: Uses absolute paths (`/index.html`, `/styles.css`, `/script.js`) -- correct for GitHub Pages

### 2.2 Dynamic Content Architecture

The site uses a data-driven architecture where 4 pages (research, technology, innovations, benchmarks) are populated at runtime via JavaScript from JSON data files:

- `reports/inventory_public.json` feeds: innovations.html, technology.html, research.html, proofs.html
- `reports/benchmarks_curated.json` feeds: benchmarks.html

The `script.js` file contains async loaders (`loadInventoryData()`, `loadBenchmarkData()`) with caching and error handling. If JSON fails to load, the pages degrade gracefully (console warning, empty grid or fallback text).

### 2.3 Benchmark Claims Inventory

| Claim | Source | Location |
|-------|--------|----------|
| 627 workspace tests | benchmarks_curated.json, benchmarks-detailed.html, index.html | Multiple pages |
| Parallel encrypt 10x: 28.10ms | benchmarks_curated.json, benchmarks-detailed.html | Benchmarks |
| 4.9x speedup | benchmarks_curated.json, benchmarks-detailed.html | Benchmarks |
| Batch decode 64: 1.378us | benchmarks_curated.json, benchmarks-detailed.html | Benchmarks |
| Depth-50 circuit: 139ms (symmetric mul) | nine65-saas.html, index.html | SaaS, Home |
| Depth-50 circuit: 1.32s (full circuit) | benchmarks-detailed.html | Detailed benchmarks |
| Encryption: 4.76ms | benchmarks-detailed.html | FHE benchmarks |
| HomAdd: 0.19ms | benchmarks-detailed.html | FHE benchmarks |
| HomMul: 26.51ms | benchmarks-detailed.html | FHE benchmarks |
| Montgomery chain 1000@N=1024: 3.98ms | benchmarks-detailed.html | Core benchmarks |
| Barrett 100K reductions: 1.01ms | benchmarks-detailed.html | Core benchmarks |
| CRT full cycle: 419ns | benchmarks-detailed.html | Core benchmarks |
| NTT forward N=4096: <50us | benchmarks-detailed.html | Core benchmarks |
| RNS ADD: 65.7ns | benchmarks-detailed.html | RNS benchmarks |
| RNS MUL: 95.6ns | benchmarks-detailed.html | RNS benchmarks |
| K-Elimination: O(k) complexity | benchmarks-detailed.html | K-Elim section |
| 40x speedup vs full CRT reconstruction | benchmarks-detailed.html | K-Elim section |
| Memory footprint: ~200 MB | nine65-saas.html | SaaS page |
| Throughput: 10-25M ops/s | nine65-saas.html | SaaS page |
| 3-6x CPU performance vs industry | nine65-saas.html | SaaS page |
| Shadow entropy: ~30 Kbits/sec | shadow-entropy.html | Shadow entropy |

---

## 3. EVIDENCE VERIFICATION

### 3.1 BENCHMARK_EVIDENCE.md Analysis

The file (`/home/acid/Projects/Homomorphic_Armada/builds/18_hackfate/BENCHMARK_EVIDENCE.md`) lists:

**Primary sources**:
- NINE65 MANA Boosted Benchmark Report (Criterion) at `/home/acid/Projects/NINE65/MANA_boosted/BENCHMARK_REPORT.md`
- NINE65 + SEAL Benchmark Report at `/home/acid/Projects/NINE65/MANA_boosted/NINE65_SEAL_BENCHMARK_REPORT.md`
- Canonical tarball at `/home/acid/Projects/NINE65/MANA_boosted/nine65_mana_with_proofs_20260119.tar.gz`
- NINE65 v2 benchmarks
- MYSTIC v2/v3 performance summaries

**Additional uncurated artifacts**: 16 benchmark files in `/home/acid/Downloads/` (not included in build)

**Bench commands**: Provides `cargo bench -p nine65 --bench fhe_scaling` for reproduction

**FINDING**: Evidence paths reference files outside this build (local filesystem paths). These cannot be verified from within the build itself but are documented for provenance.

### 3.2 Test Count Discrepancy

**CRITICAL FINDING**: There is a significant numerical discrepancy in test counts across the build:

| Source | Test Count | Date/Context |
|--------|-----------|--------------|
| benchmarks-detailed.html, index.html, nine65-saas.html, benchmarks_curated.json | **627** | "Feb 6, 2026" (post-remediation) |
| nine65-v5-test-results-summary.md | **465** | Original audit |
| nine65-v5-test-summary-latest.md | **465** | "Latest" label but still says 465 |
| nine65-v5-technical-audit.md | **465** | Manus AI audit, Jan 27 2026 |
| nine65-v5-technical-audit-latest.md | **465** | Manus AI audit labeled "latest" |
| hackfate-claims-verification.md | **465** | Claims verification |
| CLAUDE.md (project-level) | **480+/510** | System guide |

**Analysis**: The website (HTML pages) consistently uses 627, which it attributes to the full workspace (446 nine65 + 46 clockwork-core + 30 nexgen_rational + 95 mana + 10 unhal). The audit reports in `reports/` consistently use 465, which represents only the nine65 crate tests at the time of the original Manus AI audit (Jan 27, 2026). The "-latest" suffixed files have NOT been updated with the 627 figure despite their names suggesting they are the latest versions. The 627 count appears to be the result of adding workspace crates (clockwork-core, nexgen_rational, mana, unhal) to the test count after the original audit.

**Verdict**: The 627 figure on the website is plausible (446 + 46 + 30 + 95 + 10 = 627) but the "latest" report files have NOT been updated to reflect this, creating an internal inconsistency within the build.

### 3.3 Depth-50 Timing Discrepancy

**FINDING**: Two different numbers are used for depth-50 circuits:

| Claim | Value | Location |
|-------|-------|----------|
| "Symmetric Mul (D50)" | **139ms** | nine65-saas.html, index.html |
| "Depth-50 Circuit" | **1.32s** | benchmarks-detailed.html |

**Analysis**: The 139ms figure is used on the SaaS marketing page and homepage. The 1.32s figure is used on the detailed benchmarks page. These likely represent different configurations (symmetric vs public-key, or different parameter sets), but no explicit disambiguation is provided on the pages that cite 139ms. The benchmarks-detailed.html says the 1.32s figure is for "50 consecutive multiplies, zero bootstraps" and is measured against the "canonical baseline (2026-02-06)". The nine65-saas.html says 139ms for "50 consecutive multiplications, zero bootstraps." The 9.5x difference is unexplained in the public-facing pages.

**Verdict**: The discrepancy between 139ms and 1.32s for depth-50 is a substantive anomaly that needs explicit disambiguation.

### 3.4 Comparative Claims Assessment

The following claims involve comparisons to external systems:

| Claim | Evidence Level | Notes |
|-------|---------------|-------|
| "Bootstrap-free FHE" | **CONFIRMED** | Verified by Manus AI audit; test suite runs without bootstrapping |
| "Formally verified in Coq and Lean4" | **PARTIALLY VERIFIED** | Proof files exist in build (19 Coq, 42+ Lean4). Manus audit could not independently verify. Proofs.html claims "0 admitted" and "0 sorry" -- not independently confirmed |
| "400x speedup" | **UNSUBSTANTIATED** | No comparative benchmark against SEAL/OpenFHE exists in this build. Manus audit rates it "Plausible." benchmarks-detailed.html labels traditional FHE estimates with asterisk footnote |
| "3-6x faster CPU performance" | **UNSUBSTANTIATED** | SaaS page claim with no source data or methodology |
| "Memory ~200 MB vs 1.5-3.3 GB" | **UNSUBSTANTIATED** | No memory profiling data in this build |
| "10-25M ops/s vs 1-5M ops/s" | **PARTIALLY SUBSTANTIATED** | RNS benchmarks show 10-18M ops/s range. The "vs 1-5M ops/s" comparison lacks source |
| "40x K-Elimination speedup" | **UNSUBSTANTIATED** | No comparative benchmark against traditional CRT reconstruction in this build |
| "419ns CRT cycle" | **NOT DIRECTLY MEASURED** | Manus audit says "consistent with observed performance" but did not measure directly |
| "O(k) K-Elimination complexity" | **CLAIMED** | Theoretical claim, consistent with algorithm description |
| Lattice security estimates | **SELF-REPORTED** | No independent lattice estimator run results included |
| Shadow entropy ~30 Kbits/sec | **INCONSISTENT** | inventory_public.json says "284 Mbit/s"; shadow-entropy.html says "~30 Kbits/sec" -- 9000x difference |

### 3.5 Audit Remediation Claims

The benchmarks-detailed.html page states: "All prior audit caveats have been resolved. Floating-point usage has been fully eliminated from the entire stack (zero f64/f32 anywhere)."

**Context**: The original Manus AI audit (Jan 27, 2026) found 216 critical float violations. The website claims these were all resolved by Feb 6, 2026. No post-remediation audit report confirming elimination is included in this build. The "-latest" audit reports still contain the original 465-test findings.

**Verdict**: Remediation is claimed but not independently verified within this build.

### 3.6 Rust Version Discrepancy

| Source | Rust Version |
|--------|-------------|
| benchmarks-detailed.html (test environment) | Rust 1.90.0 |
| benchmarks_curated.json, benchmarks-detailed.html (audit benchmarks) | Rust 1.93.0 |

Two different Rust versions are cited for benchmarks on the same page.

---

## 4. LINK INTEGRITY

### 4.1 Internal Navigation Links

All 14 HTML pages share a consistent navigation bar with 8 links:
- research.html, technology.html, innovations.html, benchmarks.html, proofs.html, nine65-saas.html, about.html, contact.html

All footer links add: walkthrough.html, privacy.html

**All internal HTML-to-HTML links resolve to files present in the build.** Verified:

| Link Target | Exists | Referenced From |
|-------------|--------|-----------------|
| index.html | YES | All navs |
| research.html | YES | All navs |
| technology.html | YES | All navs |
| innovations.html | YES | All navs |
| benchmarks.html | YES | All navs |
| proofs.html | YES | All navs |
| nine65-saas.html | YES | All navs |
| about.html | YES | All navs |
| contact.html | YES | All navs |
| walkthrough.html | YES | All footers, hero CTA |
| privacy.html | YES | All footers |
| shadow-entropy.html | YES | proofs.html |
| benchmarks-detailed.html | YES | benchmarks.html |
| 404.html | YES | (GitHub Pages auto-serves) |

### 4.2 Internal Data/Report Links

| Link | Exists | Referenced From |
|------|--------|-----------------|
| reports/inventory_public.json | YES | script.js |
| reports/benchmarks_curated.json | YES | script.js |
| reports/hackfate-claims-verification.md | YES | benchmarks-detailed.html |
| reports/hackfate-claims-verification.docx | YES | benchmarks-detailed.html |
| reports/nine65-v5-technical-audit.md | YES | benchmarks-detailed.html |
| reports/nine65-v5-benchmark-results.md | YES | benchmarks-detailed.html |
| reports/nine65-v5-operations-testing.md | YES | benchmarks-detailed.html |
| reports/nine65-v5-test-results-summary.md | YES | benchmarks-detailed.html |
| reports/test-nine65-fhe.pdf | YES | benchmarks-detailed.html |
| reports/nine65-v5-technical-audit-latest.md | YES | inventory_public.json (gso-fhe public_link) |
| reports/nine65-v5-benchmark-results-latest.md | YES | inventory_public.json (mana-boosted public_link) |
| proofs/theorem_stack.md | YES | proofs.html, shadow-entropy.html |
| proofs/coq/CRTShadowEntropy.v | YES | shadow-entropy.html |
| proofs/lean4/ShadowSecurityTheorems.lean | YES | shadow-entropy.html |
| proofs/lean4/ShadowNISTCompliance.lean | YES | shadow-entropy.html |
| proofs/coq/ShadowIndependence.v | YES | shadow-entropy.html |
| proofs/README.md | YES | proofs.html |

**All internal links to data and report files resolve correctly.**

### 4.3 External Links

| URL | Referenced From | Type |
|-----|-----------------|------|
| https://github.com/Skyelabz210/k-elimination-lean4 | README.md, inventory_public.json | Public repo |
| https://github.com/Skyelabz210/MYSTIC | README.md, inventory_public.json | Public repo |
| https://github.com/Skyelabz210 | index.html (schema), contact.html | Profile |
| https://github.com/DeuxAxios/hackfate | proofs.html (clone URL), .git/config | Repo |
| https://homomorphicencryption.org/ | walkthrough.html | External reference |
| https://eprint.iacr.org/2025/473 | walkthrough.html | Academic reference |
| https://dl.acm.org/doi/10.1145/3729706.3729711 | walkthrough.html | Academic reference |
| https://docs.github.com/en/site-policy/privacy-policies/github-privacy-statement | privacy.html | GitHub privacy |
| https://fonts.googleapis.com, https://fonts.gstatic.com | index.html, proofs.html, privacy.html, 404.html | Font CDN |
| mailto:founder@hackfate.us | contact.html, README.md, privacy.html | Email |

**FINDING: GitHub Organization Mismatch**
- The structured data in index.html and contact.html reference `https://github.com/Skyelabz210` (personal profile)
- The proofs.html clone instruction and git remote reference `https://github.com/DeuxAxios/hackfate` (different organization)
- Both are used across the site without explanation of the relationship

### 4.4 Sitemap vs Actual Pages

The sitemap.xml lists 12 URLs. Comparing against actual HTML files:

| Page | In Sitemap | Exists |
|------|-----------|--------|
| / (index.html) | YES | YES |
| /research.html | YES | YES |
| /technology.html | YES | YES |
| /innovations.html | YES | YES |
| /benchmarks.html | YES | YES |
| /benchmarks-detailed.html | YES | YES |
| /about.html | YES | YES |
| /contact.html | YES | YES |
| /privacy.html | YES | YES |
| /walkthrough.html | YES | YES |
| /proofs.html | YES | YES |
| /nine65-saas.html | YES | YES |
| /shadow-entropy.html | **NO** | YES |
| /404.html | NO (correct) | YES |

**FINDING**: `shadow-entropy.html` exists and is linked from proofs.html and innovations grid but is NOT included in sitemap.xml.

---

## 5. DEAD CONTENT

### 5.1 Orphaned/Unused Files

The following files exist in the build but are NOT referenced by any HTML page, JS script, or JSON data file:

| File | Apparent Purpose | Status |
|------|-----------------|--------|
| ENHANCEMENT_PLAN.md | Internal planning document (47 gap analysis items) | **DEAD** -- planning artifact, not web content |
| EXECUTION_PLAN.md | Internal execution plan | **DEAD** -- planning artifact |
| GRANDMASTER_REPORT.md | Internal analysis methodology report | **DEAD** -- planning artifact |
| INNOVATION_CATALOG_PUBLIC.md | Text-based innovation catalog | **DEAD** -- superseded by inventory_public.json |
| INSIGHT_LOG.md | Internal analyst notes | **DEAD** -- planning artifact |
| QUESTION_MATRIX.md | Internal decision questions | **DEAD** -- planning artifact |
| BENCHMARK_EVIDENCE.md | Evidence source index | **DEAD** -- internal reference, not web content |
| proofs/SHADOW_ENTROPY_FORMALIZATION_PLAN.md | Proof planning | **DEAD** -- not linked |
| proofs/shadow_entropy_blueprint.json | Blueprint data | **DEAD** -- not consumed by any script |
| proofs/critiques/round1-7_critique.md (7 files) | Critique rounds | **DEAD** -- not linked from any page |
| reports/nine65-v5-operations-testing-latest.md | Operations testing | **DEAD** -- not directly linked (non-latest version IS linked) |
| reports/nine65-v5-test-summary-latest.md | Test summary | **DEAD** -- not directly linked |
| reports/disclosure_matrix.csv | Disclosure decisions | **DEAD** -- consumed only by validate-inventory.js |
| reports/evidence_map.csv | Evidence mapping | **DEAD** -- consumed only by validate-inventory.js |
| reports/public_links.md | Link audit | **DEAD** -- internal reference |
| scripts/validate-inventory.js | Data validation script | **DEAD** -- development tool, not web content |
| og-image.svg | SVG version of og-image | **DEAD** -- og:image tags reference .png only |

### 5.2 Duplicate Report Files

Several reports exist in both "original" and "-latest" versions:

| Original | Latest | Content Difference |
|----------|--------|--------------------|
| nine65-v5-technical-audit.md | nine65-v5-technical-audit-latest.md | **UNKNOWN** (both reference 465 tests) |
| nine65-v5-benchmark-results.md | nine65-v5-benchmark-results-latest.md | **UNKNOWN** |
| nine65-v5-operations-testing.md | nine65-v5-operations-testing-latest.md | **UNKNOWN** |
| nine65-v5-test-results-summary.md | nine65-v5-test-summary-latest.md | **UNKNOWN** (different filename pattern) |

**FINDING**: The "-latest" suffix implies these are updated versions, but the test count remains 465 in both the "-latest" and original versions. This contradicts the website's claim of 627 tests (post-remediation).

---

## 6. CROSS-REFERENCE WITH OTHER BUILDS

### 6.1 Available Builds for Comparison

22 builds exist in `/home/acid/Projects/Homomorphic_Armada/builds/`. The most relevant for cross-reference:

| Build | Relevance |
|-------|-----------|
| 03_v5_full_20260209 | NINE65 v5 full build |
| 08_FHE_v03_MANA_boosted | MANA-boosted FHE build |
| 09_NINE65_v5_live | Live NINE65 v5 |
| 15_MANA_boosted_live | Live MANA-boosted |
| 17_MANA_definitive | Definitive MANA build |
| 19_k_elimination_lean4 | K-Elimination proofs |

### 6.2 CLAUDE.md Cross-Reference

The project-level CLAUDE.md references:
- "480+/510 passing" tests with "11 ignored"
- This conflicts with both the 465 (audit) and 627 (website) counts
- CLAUDE.md performance expectations: CRTBigInt add ~120ns, mul ~250ns, FHE encrypt <1ms, FHE add ~50us, FHE mul <500us
- Website benchmarks-detailed.html: Encryption 4.76ms, HomAdd 0.19ms, HomMul 26.51ms
- The CLAUDE.md expectations and the actual website benchmarks diverge significantly for FHE encrypt (~1ms vs 4.76ms) and HomMul (~500us vs 26.51ms)

**FINDING**: The CLAUDE.md performance expectations appear to describe a different (theoretical or accelerated) configuration than what the website benchmarks report. The CLAUDE.md says "<1ms" for encrypt; the website measures 4.76ms. The CLAUDE.md says "<500us" for FHE mul; the website measures 26.51ms (53x slower). These are likely different parameter configurations but the gap is notable.

---

## 7. ANOMALY CATALOGUE

### 7.1 CRITICAL Anomalies

| ID | Category | Description |
|----|----------|-------------|
| A-01 | Test Count Discrepancy | Website claims 627 tests; all included audit reports say 465. "-latest" reports not updated. |
| A-02 | Depth-50 Timing Discrepancy | 139ms (SaaS page, homepage) vs 1.32s (detailed benchmarks) for depth-50 circuits with no disambiguation. |
| A-03 | Shadow Entropy Rate Contradiction | inventory_public.json: "284 Mbit/s"; shadow-entropy.html: "~30 Kbits/sec" -- a ~9400x difference. |
| A-04 | Remediation Not Independently Verified | Website claims "all prior audit caveats resolved" and "zero f64/f32" but no updated audit report confirms this. |

### 7.2 HIGH Anomalies

| ID | Category | Description |
|----|----------|-------------|
| A-05 | Unsubstantiated Comparisons | "400x speedup", "3-6x faster", "~200 MB vs 1.5-3.3 GB", "10-25M vs 1-5M ops/s" -- all lack comparative benchmark data in build. |
| A-06 | GitHub Organization Mismatch | Skyelabz210 (personal) vs DeuxAxios/hackfate (org) used interchangeably without explanation. |
| A-07 | Rust Version Inconsistency | benchmarks-detailed.html cites both Rust 1.90.0 (test environment section) and Rust 1.93.0 (audit benchmarks section) on the same page. |
| A-08 | "-latest" Files Stale | Files with "-latest" suffix contain identical 465-test data as originals, contradicting their naming. |
| A-09 | Formal Verification "0 admitted/0 sorry" | Claims of zero unproven statements in Coq/Lean4 proofs are not independently verified. Proof files exist but compilation status unknown. |
| A-10 | Sitemap Missing shadow-entropy.html | Page exists and is linked from proofs.html but omitted from sitemap.xml. |

### 7.3 MEDIUM Anomalies

| ID | Category | Description |
|----|----------|-------------|
| A-11 | Dead Planning Files | 7 markdown planning documents (ENHANCEMENT_PLAN.md, EXECUTION_PLAN.md, etc.) ship with the website but serve no web purpose. |
| A-12 | og-image.svg Unused | SVG version of og-image exists but all meta tags reference .png version only. |
| A-13 | "READY FOR DEPLOYMENT" Badge | nine65-saas.html displays "READY FOR DEPLOYMENT" but the product has no public API, SDK, or download link. |
| A-14 | Proofs Page Hardcoded "active" | proofs.html nav link has `class="active"` hardcoded, while all other pages use JS-based active detection. |
| A-15 | Proofs Page Footer Differs | proofs.html footer has different structure (fewer links, different tagline, "footer-copyright" class) vs all other pages. |
| A-16 | Proofs Page Script Loading | proofs.html uses `<script src="script.js">` (no defer) while all other pages use `<script defer src="script.js">`. |
| A-17 | Proofs Page Duplicates CSS | proofs.html contains ~260 lines of inline `<style>` that partially overlap with styles.css (e.g., `.btn`, `.btn-primary`, `.btn-secondary` redefined). |
| A-18 | 404 Page Path Convention | 404.html uses absolute paths (`/styles.css`, `/script.js`) while all other pages use relative paths (`styles.css`, `script.js`). Both work on GitHub Pages but the inconsistency is notable. |
| A-19 | Traditional FHE Estimates | benchmarks-detailed.html "Deep Circuit Performance" table includes "Traditional FHE*" estimates that are described only as "based on published SEAL/OpenFHE benchmarks" with no specific citation. |
| A-20 | Font Preconnect Inconsistency | Only 4 of 14 pages include font preconnect hints (index.html, proofs.html, privacy.html, 404.html). Other pages omit them but still load via styles.css. |
| A-21 | Evidence Map Missing Entries | evidence_map.csv has 11 rows but inventory_public.json has 13 entries. Missing: nine65-python, nine65-wasm. |

### 7.4 LOW Anomalies

| ID | Category | Description |
|----|----------|-------------|
| A-22 | Compiled Coq Artifacts | ShadowIndependence.vo, .vok, .vos, .glob, .lia.cache exist in proofs/coq/ -- suggests this one proof was compiled but others were not. |
| A-23 | Lean Toolchain | proofs/lean4/lean-toolchain file exists but contents not verified against installed Lean4 version. |
| A-24 | DOCX File in Reports | hackfate-claims-verification.docx is a binary file that cannot be audited for content consistency vs the .md version. |
| A-25 | No Google Fonts Fallback | Font loading failure would affect visual presentation but is handled via CSS font stack fallbacks. |
| A-26 | Console Easter Egg | script.js contains ASCII art console.log with contact email -- not a security issue but atypical for production sites. |

---

## 8. SUMMARY VERDICT

### 8.1 Overall Assessment

Build 18_hackfate is a well-structured, professionally designed static website for hackfate.us with a data-driven architecture. The codebase demonstrates competent web development practices including accessibility features (skip links, ARIA attributes, reduced motion), SEO optimization (OG tags, structured data, sitemap), and responsive design.

### 8.2 Strengths

1. **Clean architecture**: Data-driven content from JSON avoids content duplication across pages.
2. **Comprehensive evidence trail**: Evidence map, disclosure matrix, claims verification, and multiple audit reports provide strong provenance documentation.
3. **Internal link integrity**: All internal links resolve correctly. No broken links found.
4. **Accessibility**: Skip links, ARIA labels, focus-visible states, reduced motion support, noscript fallback, keyboard navigation.
5. **Proof artifacts present**: 19 Coq files, 42+ Lean4 files, 7 critique rounds, 3 test result JSONs physically present in the build.
6. **Honest hedging**: The claims verification document honestly labels several claims as "Partially Verified" or "Plausible" rather than confirmed.
7. **Consistent branding**: Navigation, footer, styling consistent across 13 of 14 pages (proofs.html is the outlier).

### 8.3 Weaknesses

1. **Test count inconsistency (A-01)**: The 627 vs 465 discrepancy between website claims and included audit reports is the most significant integrity issue. The "-latest" report files are stale.
2. **Depth-50 timing ambiguity (A-02)**: The 139ms vs 1.32s discrepancy for the same claimed operation (depth-50 circuit) is unexplained.
3. **Shadow entropy rate contradiction (A-03)**: A 9400x discrepancy between two sources in the same build.
4. **Unsubstantiated comparative claims (A-05)**: Marketing-grade comparisons ("3-6x faster", "400x speedup") lack supporting comparative data in the build.
5. **Proofs page inconsistencies (A-14 through A-17)**: The proofs.html page appears to have been developed separately and not fully integrated with the site's conventions.
6. **Dead planning files (A-11)**: 7 internal planning documents ship with the website, cluttering the build with non-web content.

### 8.4 Risk Assessment

| Risk | Level | Mitigation |
|------|-------|------------|
| Credibility risk from test count mismatch | HIGH | Update "-latest" reports or remove them |
| Marketing claim vulnerability from unsubstantiated comparisons | HIGH | Add citations or hedge language |
| Confusion from depth-50 timing discrepancy | MEDIUM | Add explicit parameter/mode labels |
| Shadow entropy rate contradiction undermines technical credibility | HIGH | Reconcile the two figures |
| Dead files in repo increase attack surface and confusion | LOW | Remove planning documents from deployed build |

### 8.5 Final Classification

**Build integrity**: MODERATE -- structurally sound but contains internal contradictions that undermine benchmark credibility.

**Evidence quality**: MIXED -- strong first-party documentation (audit reports, evidence maps, proof files) but comparative claims lack supporting data.

**Deployment readiness**: The website is functional and deployable. The content accuracy issues identified are documentation/claims issues, not technical defects that would prevent deployment.

---

*END OF FORENSIC AUDIT*
*Audit conducted 2026-02-14 by Claude Opus 4.6*
*Source: /home/acid/Projects/Homomorphic_Armada/builds/18_hackfate/*
