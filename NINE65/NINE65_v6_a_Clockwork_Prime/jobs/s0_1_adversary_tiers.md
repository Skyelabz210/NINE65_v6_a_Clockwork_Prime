# S0.1 — Adversary Tier Classification

**Plan Task**: S0.1 — Three tiers: Tier 1 (Network/Software), Tier 2 (Privileged Software/Hypervisor), Tier 3 (Physical Proximity). For each: capabilities, attack vectors, NINE65 guarantees, and explicit non-guarantees.
**Status**: COMPLETE
**Date**: 2026-02-19

---

## Executive Summary

This document establishes the 3-tier adversary model for NINE65/CryptKit, replacing the previous flat threat model in `docs/SIDE_CHANNEL_THREAT_MODEL.md`. Each tier defines adversary capabilities, attack vectors, NINE65's guarantees, and explicit non-guarantees (no overclaims).

---

## Tier 1: Network/Software Adversary

### Capabilities

- Observes network traffic (ciphertexts, public keys, encrypted payloads)
- Submits chosen plaintexts for encryption
- Submits chosen ciphertexts for homomorphic evaluation
- Observes encrypted computation outputs
- Measures network-visible timing (coarse: millisecond resolution)
- Runs arbitrary software on separate machines
- Has full knowledge of NINE65 source code, parameters, and algorithms

### Attack Vectors

| Vector | Description | NINE65 Mitigation |
|--------|-------------|-------------------|
| Lattice reduction (Primal/Dual/Hybrid) | Recover secret key from public key | LWE hardness at 128/192/256-bit security (Lattice Estimator verified) |
| CPA attack | Distinguish encryptions of chosen plaintexts | IND-CPA security from LWE |
| CPA^D oracle | Exploit decryption output to recover key (Li-Micciancio, Checri et al.) | Anti-CPA^D: decryption output never returned directly (planned S3.3) |
| Noise budget oracle | Provoke decryption failures to leak key info | `checked_sub()` prevents silent overflow; `TrackedEvaluator` budget checking |
| Malformed ciphertext | Inject structurally invalid ciphertexts | Input validation gate (planned S3.2) |
| Network timing | Measure response time to infer computation type | GRO timing gate on decrypt/keygen (T8-T10 formal spec) |
| Traffic analysis | Infer computation type from message sizes/patterns | Planned S4.3: fixed-size padding, timing normalization |

### NINE65 Guarantees (Tier 1)

| Property | Guarantee | Evidence |
|----------|-----------|----------|
| Key confidentiality | RESISTED | LWE >= 128-bit; Lattice Estimator baseline; secure_configs enforced |
| Plaintext confidentiality | RESISTED | IND-CPA from LWE |
| Computation integrity | RESISTED | K-Elimination exact division (proven sound + complete); INV-8 checks |
| Timing side-channel | RESISTED | GRO timing gates on keygen/decrypt; CT polynomial mul |
| Noise budget oracle | RESISTED | checked_sub, TrackedEvaluator, millibits integer-only tracking |

### Explicit Non-Guarantees (Tier 1)

- **Forward secrecy**: Key compromise reveals all past ciphertexts encrypted under that key
- **CPA^D full mitigation**: Anti-CPA^D constraints are PLANNED (S3.3), not yet implemented
- **Traffic analysis resistance**: Message padding is PLANNED (S4.3), not yet implemented
- **Ciphertext input validation**: Comprehensive validation is PLANNED (S3.2), partial today

---

## Tier 2: Privileged Software/Hypervisor Adversary

### Capabilities

Everything in Tier 1, plus:
- Runs code on the same physical machine as NINE65
- Has hypervisor/root access to the VM or container
- Can read process memory (DRAM) via `/proc/pid/mem`, debug interfaces, or VM introspection
- Can observe CPU cache state (Flush+Reload, Prime+Probe)
- Can measure fine-grained timing (nanosecond resolution via RDTSC)
- Can inject faults via software (RowHammer, voltage manipulation on some platforms)
- Can snapshot/restore VM state

### Attack Vectors

| Vector | Description | NINE65 Mitigation |
|--------|-------------|-------------------|
| DRAM memory read | Extract secret key from process memory | CPU-register key confinement (planned S2.3: TRESOR-style AVX-512) |
| Cache timing (F+R, P+P) | Observe NTT access patterns to learn coefficients | NTT twiddle precomputation; randomized butterfly ordering (planned S3.6) |
| Software fault injection | RowHammer bit-flips in key memory | Guard pages + mlock + redundant copy (planned S3.7) |
| VM introspection | Read all process state via hypervisor | TEE integration (planned S2.2: SGX/TDX/SEV-SNP attestation) |
| DMA attack | Read memory via Thunderbolt/PCIe DMA | IOMMU enforcement (planned S2.4) |
| DBI frameworks | Instrument NINE65 binary (Frida, DynamoRIO) | DBI detection (planned S2.1): code integrity hash, ptrace self-trace |
| Speculative execution | Spectre-class transient execution | NOT ADDRESSED (residual risk) |

### NINE65 Guarantees (Tier 2)

| Property | Guarantee | Evidence |
|----------|-----------|----------|
| Key confidentiality (with TEE) | PARTIALLY RESISTED | Planned S2.2+S2.3: key in registers, TEE attestation |
| Key confidentiality (without TEE) | NOT RESISTED | DRAM readable by privileged adversary |
| Timing side-channel | PARTIALLY RESISTED | GRO gates + CT ops; cache timing partially mitigated by twiddle preload |
| Computation integrity | PARTIALLY RESISTED | INV-8 + planned S3.7 rowhammer mitigation |
| DRAM intermediate exposure | NOT RESISTED | Ciphertexts and intermediate values in DRAM during computation |

### Explicit Non-Guarantees (Tier 2)

- **Key confidentiality without TEE/register confinement**: A Tier 2 adversary reading DRAM can recover the secret key in current implementation
- **DRAM intermediate values**: All ciphertexts, evaluation keys, and intermediate computation values are in DRAM and readable by a Tier 2 adversary. This is a fundamental limitation, not a bug.
- **Speculative execution**: Spectre-class attacks on NTT and K-Elimination are NOT mitigated
- **VM snapshot replay**: An adversary who can snapshot and restore VM state can replay computations
- **DBI detection is a speed bump, not a wall**: Sophisticated DBI users can bypass detection (millisecond key lifetime is the stronger defense)

---

## Tier 3: Physical Proximity Adversary

### Capabilities

Everything in Tier 1 and 2, plus:
- Physical access to the hardware
- Can measure electromagnetic emanations (EM probes)
- Can measure power consumption (oscilloscope on power rail)
- Can perform voltage/clock glitching
- Can cold-boot DRAM (liquid nitrogen freeze)
- Can decap chips and probe internal buses
- Can replace firmware

### Attack Vectors

| Vector | Description | NINE65 Mitigation |
|--------|-------------|-------------------|
| Power analysis (SPA/DPA) | Recover key from power traces of NTT/Montgomery | Randomized NTT butterfly ordering (planned S3.6) provides partial mitigation |
| EM emanation | Side-channel via electromagnetic radiation | NOT ADDRESSED (requires hardware shielding) |
| Cold boot attack | Freeze DRAM, extract key bits | Register key confinement (planned S2.3) prevents DRAM exposure |
| Voltage glitching | Skip fuse decrement instruction | Fuse-exhaustion model (planned S2.5): continuous consumption, no branch to skip |
| Firmware replacement | Boot with modified firmware, bypass all software defenses | NOT ADDRESSED (requires hardware attestation) |
| Bus probing | Physical probes on memory bus | NOT ADDRESSED (requires hardware isolation) |

### NINE65 Guarantees (Tier 3)

| Property | Guarantee | Evidence |
|----------|-----------|----------|
| Key confidentiality (registers) | PARTIALLY RESISTED | Planned S2.3: key never in DRAM; glitch attacker still sees register values during execution |
| Computation integrity | PARTIALLY RESISTED | Planned S2.5: fuse model, but sophisticated glitch could corrupt fuse counter |
| Power analysis | NOT RESISTED | Software cannot prevent power analysis; randomized NTT is speed bump only |
| EM emanation | NOT RESISTED | Requires physical Faraday cage (Track H: Hardware, deferred) |
| Physical key erasure | NOT PROVABLE | Classical provable deletion is impossible; computation receipt proves computation, not erasure |

### Explicit Non-Guarantees (Tier 3)

- **Physical key erasure cannot be proven**: Computation receipt (renamed from "destruction receipt") proves computation occurred, NOT that key material was erased. This is a fundamental impossibility, not a bug.
- **Power/EM analysis**: Software-only system cannot prevent physical side channels. Hardware shielding (Track H, deferred) is required.
- **Voltage glitching on fuse**: A sophisticated glitch could corrupt the fuse counter without corrupting computation. This is a Tier 3 residual risk documented honestly.
- **Firmware compromise**: If firmware is replaced, all software defenses are void. Hardware attestation (Track H) required.

---

## Defense Mapping: Existing Code to Tiers

### Currently Implemented (v6)

| Defense | Code Location | Tier Coverage | Status |
|---------|--------------|---------------|--------|
| GRO timing gate | `crates/clockwork-core/src/gro.rs` | Tier 1 (network timing) | ACTIVE |
| Constant-time poly mul | `crates/nine65/src/ring/polynomial.rs` (`mul_ct`) | Tier 1-2 (timing) | ACTIVE |
| SecretKeyPath trait | `crates/nine65/src/security/secret_data.rs` | Tier 1-2 (compile-time CT enforcement) | ACTIVE |
| Gated decryptor | `crates/nine65/src/security/gro_gate.rs` | Tier 1 (timing) | ACTIVE |
| Key zeroization | `crates/nine65/src/keys/mod.rs` (ZeroizeOnDrop) | Tier 1-2 (memory cleanup) | ACTIVE |
| Noise budget checking | `NoiseBudget::consume()` with `checked_sub` | Tier 1 (oracle prevention) | ACTIVE |
| Entropy health check | `crates/nine65/src/entropy/secure.rs` | Tier 1 (entropy failure) | ACTIVE |
| Parameter hardening | `crates/nine65/src/params/secure_configs.rs` | All tiers (parameter misuse) | ACTIVE |
| NTT twiddle precomputation | `crates/nine65/src/arithmetic/ntt.rs` | Tier 2 (partial cache timing) | ACTIVE |
| INV-8 integrity checks | `crates/nine65/src/entropy/shadow_entropy_monitor.rs` | Tier 1 (input attacks) | ACTIVE |
| Bound tracking | `crates/clockwork-core/src/basis.rs` | All tiers (overflow detection) | ACTIVE (clockwork feature) |
| Key lifecycle management | `crates/nine65/src/security/key_manager.rs` | Tier 1-2 (key hygiene) | ACTIVE (clockwork feature) |
| Limb integrity | `crates/nine65/src/security/integrity.rs` | Tier 1 (data corruption) | ACTIVE (clockwork feature) |
| Garner reconstruction | `crates/clockwork-core/src/garner.rs` | All tiers (cross-validates K-Elimination) | ACTIVE (clockwork feature) |

### Planned (Not Yet Implemented)

| Defense | Plan Task | Tier Coverage | Status |
|---------|-----------|---------------|--------|
| HWRNG integration | S1.2 | All tiers | PLANNED |
| Key-dependent entropy mixing | S1.3 | Tier 1-2 | PLANNED |
| VM/hypervisor detection | S2.1 | Tier 2 | PLANNED |
| TEE integration (SGX/TDX/SEV-SNP) | S2.2 | Tier 2 | PLANNED |
| CPU-register key confinement | S2.3 | Tier 2-3 | PLANNED |
| IOMMU/DMA protection | S2.4 | Tier 2 | PLANNED |
| Fuse-exhaustion destruction | S2.5 | Tier 2-3 | PLANNED |
| Anti-CPA^D constraints | S3.3 | Tier 1 | PLANNED |
| Ciphertext input validation | S3.2 | Tier 1 | PLANNED |
| Randomized NTT butterfly | S3.6 | Tier 2-3 | PLANNED |
| Rowhammer mitigation | S3.7 | Tier 2 | PLANNED |
| Clockwork Bootstrap security | S3.10 | All tiers | PLANNED |

---

## Guarantee Matrix (Response v2 Section 3 Alignment)

| Property | Tier 1 | Tier 2 (with TEE) | Tier 2 (no TEE) | Tier 3 |
|----------|--------|-------------------|-----------------|--------|
| Key confidentiality | RESISTED | PARTIALLY | NOT RESISTED | NOT RESISTED |
| Plaintext confidentiality | RESISTED | PARTIALLY | NOT RESISTED | NOT RESISTED |
| Computation integrity | RESISTED | PARTIALLY | PARTIALLY | PARTIALLY |
| Timing side-channel | RESISTED | PARTIALLY | PARTIALLY | NOT RESISTED |
| Physical side-channel | N/A | N/A | N/A | NOT RESISTED |
| Key erasure proof | N/A | NOT PROVABLE | NOT PROVABLE | NOT PROVABLE |

---

## Acceptance Criteria Status

- [x] Three tiers defined with capabilities, attack vectors, guarantees, non-guarantees
- [x] No overclaims (all "NOT RESISTED" and "NOT PROVABLE" explicitly documented)
- [x] Tiers align with plan Section 3 (Response v2)
- [x] Existing defenses mapped to tiers with code locations
- [x] Planned defenses mapped to plan tasks
- [x] Guarantee matrix complete
