# NINE65 Bench Update 3 — Comprehensive Innovations Report
Date: 2025-12-24
Scope: FHE core + mathematical innovations + accelerator subsystems
Hardware: i7 CPU, 8 GB RAM (details: `HARDWARE_BASELINE.md`)

## 1) FHE Scaling (FFT NTT active)
Source: `examples/bench_update3.rs`  
Raw: `/tmp/bench_update3.txt`

Per‑op timings:

| N    | NTT mul (ms) | Encrypt (ms) | Decrypt (ms) | Homo Add (us) | Homo Mul (ms) |
|------|--------------|--------------|--------------|----------------|---------------|
| 2048 | 2.235        | 5.54         | 2.83         | 10.035         | 6.83          |
| 4096 | 4.890        | 10.92        | 3.98         | 19.538         | 13.60         |
| 8192 | 7.585        | 19.98        | 8.37         | 42.081         | 29.86         |

Scaling (relative to N=2048):

| N    | NTT   | Enc   | Dec   | Add   | Mul   |
|------|-------|-------|-------|-------|-------|
| 2048 | 1.00x | 1.00x | 1.00x | 1.00x | 1.00x |
| 4096 | 2.19x | 1.97x | 1.41x | 1.95x | 1.99x |
| 8192 | 3.39x | 3.60x | 2.96x | 4.19x | 4.37x |

## 2) Persistent Montgomery + K‑Elimination (microbench)
Source: `examples/bench_innovations.rs`  
Raw: `/tmp/bench_innovations.txt`

K‑Elimination:
- `scale_and_round`: **129.676 ns/op** (total 129.676 ms / 1,000,000 ops)

Persistent Montgomery:
- `enter`: **5.610 ns/op** (total 5.609 ms / 1,000,000 ops)
- `exit`: **5.289 ns/op** (total 5.288 ms / 1,000,000 ops)
- `mul`: **0.427 ns/op** (total 2.135 ms / 5,000,000 ops)
- `add`: **0.433 ns/op** (total 2.166 ms / 5,000,000 ops)
- `sub`: **0.429 ns/op** (total 2.146 ms / 5,000,000 ops)
- `neg`: **3.761 ns/op** (total 18.805 ms / 5,000,000 ops)

## 3) Innovation Components (FHE Bench Suite)
Source: `src/bin/fhe_benchmarks.rs`  
Raw: `/tmp/fhe_benchmarks.txt`

Key innovation metrics:
- Montgomery multiply: **56.41 ns/op** (P50 54 ns; 17.7M ops/sec)
- Persistent Montgomery multiply: **56.13 ns/op** (P50 54 ns; 17.8M ops/sec)
- NTT forward (N=1024): **74.68 μs/op**
- NTT polynomial multiply (N=1024): **1.782 ms/op**
- K‑Elimination exact division: **60.02 ns/op** (16.7M ops/sec)
- ExactDivider reconstruct: **56.11 ns/op** (17.8M ops/sec)
- Shadow entropy sample: **54.95 ns/op** (18.2M ops/sec)

FHE ops (Light config N=1024):
- KeyGen: **7.436 ms**
- Encrypt: **3.487 ms**
- Decrypt: **1.502 ms**
- Homo Add: **7.93 μs**
- Homo Mul (Plain): **47.87 μs**
- Tensor Product: **6.488 ms**
- Homo Mul (Full): **9.771 ms**

## 4) Super‑Poly / PolyPoly Innovations (Neural Bench)
Source: `src/bin/neural_bench.rs`  
Raw: `/tmp/neural_bench.txt`

Padé [4/4] Engine:
- exp(): **375.3 ns/op**
- sigmoid(): **426.0 ns/op**
- tanh(): **810.4 ns/op**

MQ‑ReLU (O(1) sign):
- scalar: **6.1 ns/op**
- polynomial(1024): **4.7 μs/op**

MobiusInt signed arithmetic:
- add(): **4.9 ns/op**
- mul(): **11.9 ns/op**

Integer Softmax (exact sum):
- 10‑class: **5.5 μs/op**
- 100‑class: **52.7 μs/op**
- 1000‑class: **517.3 μs/op**

FHE Neural Evaluator:
- dense(64→64, ReLU): **37.1 μs/op**
- dense(256→256, None): **0.6 ms/op**

## 5) QBit / Quantum Substrate Metrics
Source: `src/bin/quantum_bench.rs`  
Raw: `/tmp/quantum_bench.txt`

Grover iteration latency (selected):
- 100 qubits: **170 ns/iter** (~5.88M/sec)
- 1,000 qubits: **171 ns/iter** (~5.85M/sec)
- 100,000 qubits: **174 ns/iter** (~5.73M/sec)

Coherence (weight drift):
- 100 qubits, depth 1,000 → **ZERO drift**
- 100 qubits, depth 100,000 → **ZERO drift**
- 100,000 qubits, depth 10,000 → **ZERO drift**

Throughput stress:
- 10,000 qubits × 1,000,000 iterations: **177.77 ms total**, 5.63M/sec

Max‑qubit test:
- 1,000,000 qubits × 1,000 depth: **94.33 μs total**, 10.65M/sec

## 6) MANA Accelerator (CRT / Lane Ops)
Source: `crates/mana/examples/benchmark.rs`  
Raw: `/tmp/mana_bench.txt`

Lane Add (single prime):
- N=1024: **463M ops/sec**
- N=4096: **455M ops/sec**
- N=8192: **479M ops/sec**

Stream Add (8 CRT lanes):
- N=1024: Seq **391M** → Rayon **117M** (0.30x)
- N=4096: Seq **116M** → Rayon **490M** (4.22x)
- N=8192: Seq **102M** → Rayon **583M** (5.74x)

## 7) UNHAL
Source: `crates/unhal/examples/benchmark.rs`  
Raw: `/tmp/unhal_bench.txt` (copied to `docs/bench/unhal_bench.txt`)

UNHAL (8 CRT lanes) ops/sec:

| N | Mode | Add (M ops/s) | Sub (M ops/s) | Mul (M ops/s) |
|---|------|---------------|---------------|---------------|
| 1024 | Sequential | 376 | 383 | 32 |
| 1024 | Parallel   | 73  | 68  | 42 |
| 4096 | Sequential | 113 | 112 | 25 |
| 4096 | Parallel   | 29  | 27  | 27 |
| 8192 | Sequential | 87  | 109 | 26 |
| 8192 | Parallel   | 38  | 37  | 30 |

Notes:
- Parallel mode helps for mul at N=1024/8192, but hurts add/sub at these sizes.
- UNHAL defaults to `parallel` feature (Rayon on lane level).
- UNHAL plots:
  - `docs/bench/unhal_ops_vs_n.png`
  - `docs/bench/unhal_speedup.png`
  - `docs/bench/unhal_bars.png`

## 8) World Placement (Top 4)
Sources:
- NINE65: `examples/bench_update3.rs`
- Competitors: “Cross‑Platform Benchmarking of the FHE Libraries” (eprint 2025/473)

Homomorphic Add (ms/op, lower is better):

| System | ms/op | Source |
|--------|-------|--------|
| NINE65 (N=8192) | 0.042081 | bench_update3 |
| OpenFHE Linux (CKKS depth=300) | 13.333 | eprint‑2025‑473 |
| SEAL Linux (CKKS depth=300) | 23.333 | eprint‑2025‑473 |
| SEAL Windows (BGV depth=300) | 606.667 | eprint‑2025‑473 |

Homomorphic Mul (ms/op, lower is better):

| System | ms/op | Source |
|--------|-------|--------|
| NINE65 (N=8192) | 29.86 | bench_update3 |
| OpenFHE Linux (BGV depth=20) | 350.000 | eprint‑2025‑473 |
| OpenFHE Linux (CKKS depth=20) | 600.000 | eprint‑2025‑473 |
| SEAL Windows (CKKS depth=20) | 15000.000 | eprint‑2025‑473 |

Plots:
- `docs/bench/world_placement_add.png`
- `docs/bench/world_placement_mul.png`

Notes:
- These are not apples‑to‑apples (schemes, parameters, and depth differ).
- The tables provide a “top‑4” external placement snapshot based on reported public benchmarks.

---
## Notes
- All numbers above are **actual run outputs** captured in `/tmp/*_bench.txt`.
- Persistent Montgomery now has both microbench and full‑suite stats (Section 2 + Section 3).
- Super‑Poly/PolyPoly innovations map to Padé, MQ‑ReLU, MobiusInt, Integer Softmax, Cyclotomic phase paths in `neural_bench`.
