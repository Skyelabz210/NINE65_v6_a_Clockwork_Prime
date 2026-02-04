## Reconnaissance Report

**Files to Modify**: 8
- crates/nine65/src/ops/rns_fhe.rs - dual-RNS rescaling path and tests
- crates/nine65/src/ops/rns_mul.rs - ignored test and NTT consistency
- crates/nine65/src/arithmetic/rns.rs - DualRNSContext overflow handling
- crates/nine65/src/accelerated.rs - partial acceleration coverage
- crates/nine65/src/compiler.rs - float-based static analysis
- crates/nine65/src/noise/mod.rs - float metrics in runtime
- proofs/coq/*.v - admitted statements
- README.md - align claims with enforcement

**Float Check**: CONTAMINATED at `crates/nine65/src/ahop/grover_full.rs`, `crates/nine65/src/params/mod.rs`, `crates/nine65/src/compiler.rs`, `crates/nine65/src/noise/mod.rs`, `crates/nine65/src/ops/gso_fhe.rs`, `crates/mana/examples/benchmark.rs`, `crates/unhal/examples/unhal_benchmark.rs`

**Bootstrap Check**: CONTAMINATED at `crates/nine65/src/compiler.rs`, `crates/nine65/src/ops/gso_fhe.rs`, `crates/nine65/src/ops/rns_fhe.rs`

**Existing Patterns**:
- Dual-RNS + K-Elimination in `crates/nine65/src/arithmetic/rns.rs`
- ShadowHarvester vs secure entropy in `crates/nine65/src/entropy/`
- Error taxonomy mapped to proofs in `crates/nine65/src/errors.rs`
- Zeroize for key material in `crates/nine65/src/keys/mod.rs`

**Test Coverage**: Unknown (no coverage report); tests live in `crates/nine65/src` and `crates/nine65/tests`
