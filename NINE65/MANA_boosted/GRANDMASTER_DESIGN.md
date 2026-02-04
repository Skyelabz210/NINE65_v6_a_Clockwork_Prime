## Design Document

**Architecture**:
```
Plaintext
  -> BFVEncoder
  -> Encryptor (ShadowHarvester or secure)
  -> Ciphertext
      -> RNSFHEContext
          -> RNS tensor (NTT/coeff domain)
          -> Dual-RNS K-Elimination (coeff domain)
          -> Rescale + Relinearize
          -> Noise tracking (GSO-FHE)
  -> Decryptor
  -> Plaintext

Acceleration:
  MANA lanes/streams + UNHAL pipeline wrap RNS limbs

Entropy:
  Shadow/secure/CRT Shadow sources feed keygen and sampling
```

**Innovation Composition**:
| Step | Innovation | Input | Output | Complexity |
|------|-----------|-------|--------|------------|
| 1 | Persistent Montgomery | coeffs | mont coeffs | O(1) per op |
| 2 | NTT Engine | coeffs | NTT domain | O(N log N) |
| 3 | K-Elimination | dual RNS residues | exact value | O(k) |
| 4 | GSO-FHE | noise state | bounded noise | O(1) per op |
| 5 | CRT Shadow Entropy | residues | entropy bits | O(1) |

**Implementation Order** (respects dependencies):
1. RNSContext + Montgomery primitives (no dependencies)
2. DualRNSContext + K-Elimination (depends on 1)
3. RNSFHEContext ct x ct pipeline (depends on 1, 2)
4. GSO-FHE noise bounding (depends on 3)
5. MANA/UNHAL acceleration hooks (depends on 3)

**Invariant Chain**:
- After step 1: all coefficients remain in valid Montgomery form
- After step 2: reconstruction exact for values in range M * A
- After step 3: rescaled ciphertexts remain within modulus bounds
- After step 4: noise <= collapse threshold for configured depth
- After step 5: parallel lanes preserve per-prime correctness
