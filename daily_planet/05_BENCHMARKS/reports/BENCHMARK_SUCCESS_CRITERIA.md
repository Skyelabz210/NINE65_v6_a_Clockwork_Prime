# Cryptographic Systems Benchmark Success Criteria

**Purpose**: Define EXACT qualifying gates to measure actual success vs failure.
**Date**: 2025-11-17
**Classification**: CRITICAL - Prevents claiming unvalidated achievements

---

## Critical Rules

1. **NO theoretical projections count as achievements**
2. **NO toy parameter results count as cryptographic validation**
3. **NO claims without head-to-head comparison on SAME hardware**
4. **NO performance claims without reproducible benchmark data**

---

## Success Criteria by System

### System 01: BFV Core FHE

**Minimum Requirements**:
- ✅ Parameters: n ≥ 4096, log q ≥ 109 (cryptographic strength)
- ✅ Security: 128-bit classical minimum
- ✅ Encryption time measured (milliseconds)
- ✅ Decryption time measured
- ✅ Homomorphic operations (add, multiply) measured
- ✅ Correctness: 100% of operations decrypt correctly

**Performance Gates** (absolute performance targets):
- Encryption: < 5ms per operation
- Homomorphic add: < 100μs per operation
- Homomorphic multiply: < 10ms per operation

**Performance Gates** (competitive performance):
- Demonstrably usable for practical applications
- No catastrophic overhead (< 10ms per operation)
- Consistent, reproducible results across multiple runs

---

### System 02: BFV Realtime FHE

**Minimum Requirements**:
- ✅ Parameters: n ≥ 4096, log q ≥ 109
- ✅ Security: 128-bit classical
- ✅ ACTUAL measured time (not theoretical projection)
- ✅ 1000+ trial runs for statistical validity
- ✅ Median, mean, std dev all reported

**Performance Gates** (to claim "realtime"):
- Encryption: < 1ms consistently (95th percentile < 1ms)
- Throughput: > 1000 encryptions/second

**Performance Gates** (to claim "breakthrough"):
- Must demonstrate < 1ms encryption consistently (95th percentile)
- Must have measured results on cryptographic parameters
- Must show statistical significance across 1000+ trials
- Must document all hardware specifications and parameters

---

### System 03: BFV Montgomery FHE

**Minimum Requirements**:
- ✅ Test multiple moduli: 2^16, 2^30, 2^60
- ✅ Compare Montgomery vs naive on SAME data
- ✅ Statistical significance: p < 0.05

**Success Criteria** (Montgomery optimization):
- Speedup > 1.30× (30% faster) for moduli ≥ 2^30
- Validation rate ≥ 50% of tested moduli

**Current Status**: ❌ FAILED
- 0/6 moduli validated
- 53-87% SLOWER for all tested moduli < 2^20

**Action Required**:
- Retest with cryptographic moduli (2^60+)
- If still fails: Document as "Montgomery not beneficial for FHE"

---

### System 04: AHOP Unified FHE

**Minimum Requirements**:
- ✅ Parameters: n ≥ 4096, q ≥ 2^60 (cryptographic)
- ❌ Current: n ~ 16-bit moduli (TOY PARAMETERS)

**Success Criteria**:
- Encryption < 5ms with n=4096
- Chained multiplications ≥ 10 depth
- Correctness: 100% decryption accuracy

**Current Status**: ⚠️ PARTIAL
- ✅ Works on toy parameters (2.2μs encryption)
- ❌ NOT tested on cryptographic parameters

**Gate**: CANNOT claim "validated" until tested with n=4096, q=2^60

---

### System 05: Entropy Shadow FHE

**Minimum Requirements**:
- ✅ Entropy quality: Pass NIST randomness test suite
- ✅ Entropy rate: ≥ 1 Mb/s minimum for practical use
- ✅ Energy measurement: < 10× standard CSPRNG

**Success Criteria** (shadow entropy):
- NIST tests: p-value > 0.01 for ALL tests
- Thermodynamic efficiency: < 5× energy vs ChaCha20
- Integration: FHE works with shadow-derived noise

**Current Status**: ❌ NOT TESTED
- No NIST test results
- No energy measurements
- No FHE integration validation

**Gate**: CANNOT claim "thermodynamically efficient" without physical measurements

---

### System 06: GSO Swarm FHE

**Minimum Requirements**:
- ✅ Noise quality: Entropy ≥ 7.8 bits/sample
- ✅ Statistical tests: KS test p > 0.05
- ✅ Comparison: vs standard CSPRNG on SAME hardware

**Success Criteria** (noise optimization):
- Entropy: > 7.71 bits/sample (standard CSPRNG baseline)
- Uniformity: KS test p-value > 0.05
- Performance cost: < 20× slower than standard noise

**Success Criteria** (depth improvement):
- ❌ CANNOT claim "enables deeper circuits" without:
  1. Depth measurement with GSO noise: D_gso multiplications
  2. Depth measurement with standard noise: D_std multiplications
  3. D_gso > D_std with statistical significance (p < 0.05)

**Current Status**: ⚠️ PARTIAL
- ✅ Noise quality claims documented (7.82-7.91 bits/sample)
- ❌ Depth improvement NOT measured

---

### System 07: MAA Cryptosystem

**Minimum Requirements**:
- ✅ Key size measured (bytes)
- ✅ Comparison with Kyber-512, Kyber-768, Kyber-1024
- ✅ Security assumption documented

**Success Criteria** (compact keys):
- Public key: < 100 bytes
- Comparison: < 10× smaller than Kyber-512 (800 bytes)
- Functionality: KEM encaps/decaps works correctly

**Success Criteria** (security):
- ❌ CANNOT claim "post-quantum secure" without:
  1. Cryptanalysis challenge (≥ 6 months)
  2. Academic peer review
  3. Hardness proof or strong evidence

**Current Status**: ⚠️ PARTIAL
- ✅ 32-byte keys (25× smaller than Kyber)
- ❌ Security NOT validated (novel assumption, needs cryptanalysis)

**Gate**: Mark as "compact keys (UNPROVEN security)" until cryptanalysis complete

---

### System 08: ACC Cryptosystem

**Minimum Requirements**:
- ✅ Cylindrical time operations measured
- ✅ Comparison with standard cryptographic primitives
- ✅ Novel technique validated

**Success Criteria**:
- TBD based on ACC goals

**Current Status**: ❌ NOT TESTED
- 1 JSON file (cylindrical time only)
- No comprehensive validation

---

## Circuit Depth Measurement Gates

### To Claim "Deeper Circuits Without Bootstrapping"

**MANDATORY REQUIREMENTS**:

1. ✅ **Measure maximum depth** (repeated squaring):
   ```python
   ct = Enc(2)
   depth = 0
   while decrypt_correct(ct):
       ct = ct * ct  # Homomorphic squaring
       depth += 1
       expected = 2^(2^depth)
       actual = decrypt(ct)
       if actual != expected:
           break
   return depth - 1  # Maximum achieved depth
   ```

2. ✅ **Measure across systems**:
   - Our System 02: D_sys02
   - Our System 06 (GSO): D_sys06
   - Hybrid (02+06): D_hybrid

3. ✅ **Statistical validation**:
   - Run 10+ trials per system
   - Report mean ± std dev
   - Consistency check across runs

4. ✅ **Success criteria**:
   ```
   To claim "enables deeper circuits":
   - Cryptographic parameters maintained (n ≥ 4096, q ≥ 2^60)
   - No bootstrapping required beyond documented gates
   - Depth grows predictably with improved noise management

   Example baseline:
   - System 02 standard noise: 12 multiplications
   - System 06 GSO noise: 18 multiplications (50% improvement)
   - Gate: GSO shows measurable depth improvement ✅ PASS
   ```

**Current Status**: ⚠️ PARTIAL
- AHOP shows 10 chained multiplications (toy params)
- Maximum depth measurement pending on cryptographic parameters

---

## Reporting Standards

### Benchmark Result Format

Every benchmark MUST report:

```json
{
  "system": "System 02 - BFV Realtime FHE",
  "date": "2025-11-17",
  "hardware": {
    "cpu": "Intel Core i7-3632QM @ 2.20GHz",
    "cores": 4,
    "ram_gb": 8,
    "architecture": "Ivy Bridge (2012)"
  },
  "parameters": {
    "n": 4096,
    "log_q": 109,
    "log_t": 16,
    "security_bits": 128
  },
  "results": {
    "encryption_us": {
      "median": 870,
      "mean": 873,
      "std_dev": 23,
      "p95": 912,
      "trials": 1000
    }
  },
  "validation": {
    "correctness_rate": 1.0,
    "gate_passed": true,
    "gate_threshold_us": 1000,
    "notes": "Passed <1ms gate for realtime claim"
  }
}
```

### Success Marking Rules

**Mark as ✅ VALIDATED only if**:
1. Benchmark completed with cryptographic parameters
2. Results saved to JSON file
3. All gates passed
4. Reproducible (run 3+ times with consistent results)

**Mark as ⚠️ PARTIAL if**:
1. Works on toy parameters only
2. Missing comparison benchmarks
3. Results not statistically significant

**Mark as ❌ FAILED if**:
1. Gates not met
2. Performance worse than baseline
3. Correctness < 100%

---

## Example: Proper Claim Validation

### WRONG (what we did before):

> "System 02 achieves <1ms encryption (0.87ms measured), making it 2-20× faster than SEAL."

**Problems**:
- "0.87ms measured" - WHERE? No benchmark file
- "2-20× faster" - Compared to WHAT? No SEAL benchmark on our hardware
- Based on theoretical projection, not measurement

### RIGHT (how to do it):

> "System 02 encryption benchmarked at 873μs ± 23μs (median 870μs, n=1000 trials).
> Compared to SEAL v4.1 on same hardware (i7-3632QM): SEAL 2.1ms, ours 0.87ms.
> Speedup: 2.4× (hardware-normalized: 2.3×).
> Benchmark data: system_02_results.json, seal_baseline.json.
> Gate: PASSED (< 1ms for realtime claim)."

**Why this is right**:
- ✅ Actual measurement (873μs mean)
- ✅ Statistical data (±23μs std dev, n=1000)
- ✅ SEAL comparison on SAME hardware
- ✅ Data files referenced
- ✅ Gate validation documented

---

## Checklist for Claiming Achievement

Before claiming ANY performance achievement:

- [ ] Benchmark code written and reviewed
- [ ] Benchmark executed with cryptographic parameters (n ≥ 4096, q ≥ 2^60)
- [ ] Results saved to JSON file (timestamped)
- [ ] Statistical validity (n ≥ 1000 trials for realtime claims)
- [ ] Hardware specifications documented (CPU, RAM, OS)
- [ ] Success gates defined and all checked
- [ ] Reproducibility confirmed (run 3+ times with consistent results)
- [ ] README/docs updated with honest, measured claims only
- [ ] Absolute performance documented (not relative comparisons)
- [ ] Parameter specifications included with results

**If ANY checkbox unchecked: DO NOT CLAIM ACHIEVEMENT**

---

## Summary

**Core Principle**: Measure first, claim second. Never the other way around.

**Success Criteria Philosophy**:
- Set the bar HIGH (cryptographic parameters, statistical significance)
- Derive metrics from QMNF's own performance (absolute measurements, not comparisons)
- Document everything (hardware, parameters, methodology)
- Be conservative (claim "competitive" not "breakthrough" unless truly exceptional)

**This document prevents**:
- Claiming theoretical projections as measurements
- Comparing toy parameters to cryptographic baselines
- Ignoring hardware specifications
- Marking work as "complete" when it's "in progress"
- False external comparisons (e.g., against systems not on this hardware)

**Metrics Derived From QMNF Performance**:
- Throughput: operations per second (encryption/decryption/multiplication)
- Latency: absolute time per operation (microseconds/milliseconds)
- Scaling: how performance changes with parameter size
- Consistency: variance across trials (standard deviation)
- Correctness: 100% operation verification

**Use this checklist for EVERY benchmark.**

---

**Last Updated**: 2025-11-17
**Maintainer**: QMNF Research Team
**Status**: Living Document - Update with actual benchmark results
