# NINE65 Comprehensive Benchmark Update
Date: 2025-12-24
Location: /home/acid/Projects/side_by_side_trackb/nine65_extracted/nine65_v2_complete

## Build/Runtime Context
- Build: release
- Features: default (includes ntt_fft and parallel)
- NTT engine: NTTEngineFFT (FFT path active)
- Encryption path: encrypt_montgomery (persistent Montgomery domain)

## Methodology (per bench)
- NTT multiply: 100 iterations
- Encrypt: 100 iterations
- Decrypt: 100 iterations
- Homomorphic add: 10,000 iterations
- Homomorphic mul: 50 iterations

## Scaling Curve (per-op timings)

| N    | NTT mul (ms) | Encrypt (ms) | Decrypt (ms) | Homo Add (us) | Homo Mul (ms) |
|------|--------------|--------------|--------------|----------------|---------------|
| 2048 | 2.307        | 5.13         | 2.24         | 9.891          | 8.67          |
| 4096 | 3.772        | 10.79        | 4.28         | 19.152         | 13.53         |
| 8192 | 7.555        | 18.54        | 8.06         | 44.534         | 28.49         |

## Scaling Ratios (relative to N=2048)

| Metric       | 4096/2048 | 8192/4096 | 8192/2048 |
|--------------|-----------|-----------|-----------|
| NTT mul      | 1.635x    | 2.003x    | 3.275x    |
| Encrypt      | 2.103x    | 1.718x    | 3.614x    |
| Decrypt      | 1.911x    | 1.883x    | 3.598x    |
| Homo Add     | 1.936x    | 2.325x    | 4.502x    |
| Homo Mul     | 1.561x    | 2.106x    | 3.286x    |

## Notes
- NTT mul and homo mul scale close to O(N log N) between 4k and 8k.
- Encrypt/decrypt scale roughly linearly with N for this parameter range.
- Homo add grows slightly faster than linear at 8k; likely cache effects.

## Raw Outputs
- /tmp/bench_n2048.txt
- /tmp/bench_n4096.txt
- /tmp/bench_n8192.txt
