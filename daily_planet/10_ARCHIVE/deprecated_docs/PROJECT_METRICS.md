# QMNF Project Metrics

**Last Updated**: 2025-11-13

## Codebase Statistics

### Source Code (Authored)
```
Total Lines: 553,124
├── Rust:    334,404 lines (727 files)
└── Python:  218,720 lines (447 files)
```

### Documentation
```
Markdown/Text: 257,762 lines
```

### Grand Total
```
Code + Documentation: ~810,000 lines
```

### File Breakdown
```
Total Source Files: 1,211
├── Rust files (.rs):     727
├── Python files (.py):   447
├── Markdown docs (.md):  ~350
└── Config/Other:         ~200
```

### Modules
```
Rust Core Modules:        45 (hcvlang/src/*.rs)
Python Modules:           77 (qmnf/**/*.py)
Arithmetic/Math Modules:  58+ (across Rust & Python)
Python Subdirectories:    29 frameworks
```

### Project Size
```
Total Disk Space:         1.2 GB
├── Source Code:          ~150 MB
├── Build Artifacts:      ~532 MB
├── Documentation:        ~50 MB
└── Examples/Tests:       ~100 MB
```

## Key Architecture Components

### Core Arithmetic (from scratch)
- **CRTBigInt**: Fast bounded integers (~120ns ops, ±2^126)
- **HCVLangBigInt**: Arbitrary-precision integers (unlimited scale)
- **Adaptive CRT**: 3 variants with dynamic precision scaling
- **FFI Boundary**: Deferred reconstruction (22× performance improvement)

### Subsystems (lines of code)
- Arithmetic primitives: ~80,000 lines
- FHE/Cryptography: ~40,000 lines
- Neural primitives: ~35,000 lines
- Storage systems: ~45,000 lines
- Execution frameworks: ~50,000 lines
- Testing/Benchmarks: ~60,000 lines

## Counting Methodology

**What's Included**:
- All authored source code (.rs, .py)
- Project documentation (.md, .txt)
- Configuration files (.toml, .json, .yaml)
- Test suites and benchmarks
- Example code and demos

**What's Excluded**:
- Build artifacts (target/ directories)
- Generated code (by build scripts)
- Vendored dependencies
- Cache files (__pycache__, .pytest_cache)
- Git metadata

## Historical Context

The ~810K line count reflects the codebase after documentation cleanup (Nov 2025). Previous counts may have included:
- Redundant documentation copies
- Deprecated experimental modules
- Analysis artifacts and reports

## Verification Commands

```bash
# Count Rust code
find . -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1

# Count Python code
find . -name "*.py" -exec wc -l {} + 2>/dev/null | tail -1

# Count documentation
find . -name "*.md" -exec wc -l {} + 2>/dev/null | tail -1

# Total project size
du -sh .
```

## Development Approach

**Built 100% from first principles with AI collaboration**. No existing codebases used as foundation. All architectural decisions, algorithmic innovations, and implementations emerged from collaborative problem-solving sessions.

Acknowledgments: Anthropic (Claude), OpenAI (Codex), xAI (Grok), Google (Gemini), Perplexity, Alibaba (Qwen), Manus Architect.
