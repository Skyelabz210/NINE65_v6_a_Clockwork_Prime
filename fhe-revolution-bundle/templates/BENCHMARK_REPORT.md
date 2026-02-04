# FHE Benchmark Report Template

## Metadata

**Date:** YYYY-MM-DD HH:MM:SS
**Git Commit:** ________________
**Rust Version:** ________________

### Hardware Configuration

| Component | Specification |
|-----------|---------------|
| CPU | Model, cores, frequency |
| RAM | Size, speed |
| OS | Distribution, version |
| Disk | Type (SSD/NVMe/HDD) |

### FHE Parameters

| Parameter | Value |
|-----------|-------|
| N (ring dimension) | |
| q (ciphertext modulus) | bits |
| t (plaintext modulus) | |
| Security level | bits |
| RNS primes | |

---

## Benchmark Methodology

### Warmup
- Iterations: ________ (recommend: 100)
- Purpose: CPU cache warming, JIT compilation

### Measurement
- Iterations: ________ (minimum: 10,000)
- Timing: `std::time::Instant`
- Outlier removal: >3σ from mean

### Statistical Analysis
- Mean (arithmetic)
- Standard deviation
- Percentiles: P50, P95, P99
- Confidence interval: 95%

---

## Results

### Core Operations

| Operation | Mean | σ | P50 | P95 | P99 | N | Unit |
|-----------|------|---|-----|-----|-----|---|------|
| KeyGen | | | | | | | ms |
| Encrypt | | | | | | | μs |
| Decrypt | | | | | | | μs |
| Homo Add | | | | | | | μs |
| Homo Mul (ct×pt) | | | | | | | μs |
| Homo Mul (ct×ct) | | | | | | | ms |
| Rescale | | | | | | | μs |
| Relinearize | | | | | | | μs |

### Innovation Components

| Component | Mean | σ | P50 | P95 | P99 | N | Unit |
|-----------|------|---|-----|-----|-----|---|------|
| Montgomery mul | | | | | | | ns |
| NTT forward | | | | | | | μs |
| NTT inverse | | | | | | | μs |
| K-Elimination div | | | | | | | ns |
| Shadow Entropy sample | | | | | | | ns |

---

## Baseline Comparison

### vs OpenFHE

| Operation | QMNF | OpenFHE | Speedup | p-value | Significant? |
|-----------|------|---------|---------|---------|--------------|
| Homo Mul | | | ×| | ☐ YES ☐ NO |

### vs SEAL

| Operation | QMNF | SEAL | Speedup | p-value | Significant? |
|-----------|------|------|---------|---------|--------------|
| Homo Mul | | | ×| | ☐ YES ☐ NO |

### vs Zama TFHE-rs

| Operation | QMNF | Zama | Speedup | p-value | Significant? |
|-----------|------|------|---------|---------|--------------|
| Homo Mul | | | ×| | ☐ YES ☐ NO |

**Note:** Comparisons only valid if same parameters and hardware.

---

## Innovation Impact Analysis

### A/B Test: Innovation Enabled vs Disabled

| Innovation | Disabled | Enabled | Improvement | Overhead Eliminated |
|------------|----------|---------|-------------|---------------------|
| Persistent Montgomery | | | ×| μs/op |
| K-Elimination | | | ×| |
| Shadow Entropy | | | ×| |
| NTT Gen3 | | | ×| |

### Breakdown: Where Time is Spent

```
Homo Mul (ct×ct) Total: ________ ms

├── Tensor Product: ________ μs (___%)
│   ├── NTT forward × 6: ________ μs
│   ├── Pointwise mul × 3: ________ μs
│   └── NTT inverse × 3: ________ μs
│
├── Rescaling: ________ μs (___%)
│   ├── K-Elimination: ________ μs
│   └── RNS split: ________ μs
│
└── Relinearization: ________ μs (___%)
    ├── Decomposition: ________ μs
    └── Key application: ________ μs
```

---

## Latency Distribution

### Histogram: Homo Mul Latency

```
  Count
    │
    │  ████
    │  ████████
    │  ████████████████
    │  ████████████████████
    │  ████████████████████████████
    │  ████████████████████████████████████
    └──────────────────────────────────────── Latency (ms)
       P50    P95    P99
```

### Outlier Analysis

- Total samples: ________
- Outliers (>3σ): ________
- Outlier rate: ________%
- Max latency: ________
- Possible causes: ________________

---

## Throughput

| Operation | Ops/sec | GB/s (data) | Notes |
|-----------|---------|-------------|-------|
| Encrypt | | | |
| Homo Add | | | |
| Homo Mul | | | |
| Decrypt | | | |

---

## Memory Usage

| Operation | Peak RSS | Allocation Count | Notes |
|-----------|----------|------------------|-------|
| KeyGen | | | |
| Encrypt | | | |
| Homo Mul | | | |

---

## Reproducibility

### Commands

```bash
# Build
cargo build --release

# Run benchmarks
cargo bench --bench fhe_benchmarks

# Run with specific parameters
FHE_N=4096 FHE_SECURITY=128 cargo bench
```

### Environment Variables

| Variable | Value |
|----------|-------|
| RUSTFLAGS | |
| OPT_LEVEL | |
| LTO | |

---

## Conclusions

### Performance Summary

[One paragraph summarizing key findings]

### Innovation Impact

[Which innovations had the biggest impact and why]

### Comparison to Prior Art

[How does this compare to other FHE libraries]

### Recommendations

[What optimizations should be prioritized next]

---

## Raw Data Location

```
./audit/YYYY-MM-DD_HH-MM/
├── benchmark_raw.csv      # All timing measurements
├── benchmark_summary.json # Statistical summary
├── histogram_data.csv     # Distribution data
└── system_info.txt        # Hardware/software config
```

---

## Verification

- [ ] Warmup iterations completed before measurement
- [ ] N ≥ 10,000 for all timing measurements
- [ ] Statistical significance calculated for comparisons
- [ ] Hardware configuration recorded
- [ ] Git commit hash recorded
- [ ] Raw data archived
