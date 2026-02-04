# QMNF Benchmarking Policy

**Date**: November 30, 2025
**Status**: Policy Effective
**Scope**: All performance benchmarking and claims

---

## Core Policy: Hardware-Limited Metrics

The QMNF System runs on a single Intel i7-3632QM laptop with 8GB RAM. This hardware constraint shapes our benchmarking philosophy:

### ✅ What We DO Measure

1. **Absolute Performance on Our Hardware**
   - Encryption time: X microseconds (measured)
   - Decryption time: Y microseconds (measured)
   - Homomorphic operations: Z microseconds (measured)
   - Throughput: N operations/second (measured)

2. **Internal Comparisons**
   - System 01 vs System 02 vs System 06 (all on same hardware)
   - Optimization impact (Montgomery, GSO, entropy shadow)
   - Scaling behavior (with parameter size, with circuit depth)

3. **Consistency Metrics**
   - Standard deviation across runs
   - Percentile distributions (p50, p95, p99)
   - Reproducibility across test iterations

4. **Resource Metrics**
   - Memory footprint (kilobytes)
   - Cache efficiency
   - Thread scaling (if using Rayon parallelization)

### ❌ What We DO NOT Do

1. **External Comparisons to SEAL/HElib/OpenFHE**
   - These libraries don't run on i7-3632QM (designed for server hardware)
   - "2-20× faster than SEAL" requires SEAL on our hardware → impossible
   - Different target architectures make comparison meaningless
   - Normalized speedup calculations are speculative without actual SEAL runtime

2. **Claims Without Measurement**
   - No "theoretical" performance numbers
   - No "extrapolated" results based on asymptotic analysis
   - No "estimated" speedups without actual benchmarks
   - All claims must be: measured, reproducible, documented with hardware specs

3. **Comparisons to Published Results**
   - Microsoft SEAL papers tested on Xeon E5 / i7-6700K (4.0 GHz)
   - Our hardware: i7-3632QM (2.2 GHz) - 40% slower base clock
   - Different parameters: SEAL used n=4096, different log_q
   - Mixing different hardware/parameters violates scientific rigor

---

## Benchmark Standards

### Required for Every Benchmark

```json
{
  "system": "System XX - Description",
  "date": "YYYY-MM-DD",
  "hardware": {
    "cpu": "Intel Core i7-3632QM @ 2.20GHz",
    "cores": 4,
    "ram_gb": 8,
    "architecture": "Ivy Bridge (2012)"
  },
  "parameters": {
    "n": 4096,
    "log_q": 109,
    "security_bits": 128
  },
  "operation": "homomorphic_multiply",
  "results": {
    "median_us": 4500,
    "mean_us": 4523,
    "std_dev_us": 156,
    "p95_us": 4891,
    "trials": 1000
  }
}
```

### Claims We CAN Make

- ✅ "Encryption is 873µs on average (±23µs, n=1000 trials)"
- ✅ "System 02 is 2.3× faster than System 01 on our hardware"
- ✅ "GSO noise enables 18 multiplications vs 12 for standard noise"
- ✅ "Memory footprint: 110KB for swarm state management"
- ✅ "Throughput: 1147 encryptions/second (median, i7-3632QM)"

### Claims We CANNOT Make

- ❌ "2-20× faster than SEAL" (SEAL doesn't run on this hardware)
- ❌ "Encryption <1ms based on theoretical analysis" (must measure)
- ❌ "Faster than state-of-the-art" (no meaningful comparison possible)
- ❌ "Breakthrough performance" (only if measured on cryptographic parameters)
- ❌ "Faster than HElib" (HElib requires different hardware)

---

## Performance Targets (Measurable Goals)

### System 01: BFV Core FHE
- **Target**: Functional validation with cryptographic parameters
- **Gate**: All operations complete correctly
- **Metric**: Absolute timing (ms per operation)

### System 02: BFV Realtime FHE
- **Target**: <1ms encryption on cryptographic parameters
- **Gate**: Median < 1000µs across 1000 trials
- **Metric**: Throughput > 1000 encryptions/sec

### System 04: AHOP Unified FHE
- **Target**: >10 multiplicative depth without bootstrapping
- **Gate**: Can chain 10+ homomorphic multiplications
- **Metric**: Maximum depth before decryption failure

### System 06: GSO Swarm FHE
- **Target**: Measurable depth improvement via noise optimization
- **Gate**: D_gso > D_standard (statistically significant)
- **Metric**: Multiplicative depth achieved

---

## Success Criteria Philosophy

### Measurement First
1. Run benchmark with cryptographic parameters
2. Save results to JSON file (timestamped)
3. Run 3+ times to verify reproducibility
4. Calculate statistics (median, mean, std dev, percentiles)
5. Document all hardware specifications

### Conservative Claims
- "Competitive" requires demonstrated usability
- "Fast" requires <10ms per operation on cryptographic params
- "Breakthrough" requires >2× absolute improvement vs documented baseline
- Never claim relative speedup without actual measurements

### Documentation
- Every claim must reference benchmark JSON file
- Hardware specifications must be included
- Parameter specifications must be exact
- Limitations and caveats must be disclosed

---

## Hardware Reality Check

**Our system**: Intel i7-3632QM @ 2.20GHz, 4 cores, 8GB RAM (laptop from 2012)

**Server hardware used for SEAL benchmarks**: Xeon E5, i7-6700K @ 4.00GHz (servers from 2014+)

**Performance gap**: 1.8× clock difference alone means any direct comparison is misleading.

**Implication**: We measure what we have. We optimize for our hardware. We don't pretend to have server-class equipment.

---

## Work Request Implications

All outstanding work requests that ask for "SEAL comparison" or "comparison to state-of-the-art" should be interpreted as:

**Revised**: Measure absolute performance on our hardware with cryptographic parameters, document thoroughly, and derive metrics from QMNF's actual behavior.

This enables:
- ✅ Honest benchmarking
- ✅ Reproducible results
- ✅ Publication-quality methodology
- ✅ Avoidance of misleading claims

---

**Last Updated**: November 30, 2025
**Status**: Effective Immediately
**Applies To**: All QMNF performance documentation and claims

