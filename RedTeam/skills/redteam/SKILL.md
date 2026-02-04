---
name: redshirt
description: RedShirt - Penetration testing and cryptanalysis skill powered by MANA FHE security research. Use when performing security audits, testing encryption strength, analyzing cryptographic implementations, breaking ciphers, or validating defenses. Invoke for pentest, security audit, crypto attack, break encryption, or vulnerability assessment tasks.
---

# RedShirt - Offensive Security & Cryptanalysis Skill

**Philosophy**: To build unbreakable security, you must first possess the means to break any encryption. Then survive it.

RedShirt is a penetration testing skill enhanced by HackFate.us cryptanalysis innovations. It provides calibrated attack tools validated against NIST standards.

## Capabilities

### 1. Lattice Cryptanalysis
- **Primal Attack (uSVP)**: ADPS16 methodology for Ring-LWE/Module-LWE
- **Dual Attack**: Distinguishing attacks on lattice-based schemes
- **Hybrid Attack**: Meet-in-the-middle combined with lattice reduction
- **BKZ Cost Estimation**: Core-SVP and MATZOV cost models

### 2. Classical Cryptanalysis
- RSA factorization analysis
- Discrete logarithm assessment
- Side-channel vulnerability detection
- Timing attack surface analysis

### 3. Security Validation
- Parameter strength verification
- Noise budget analysis for FHE schemes
- Key recovery feasibility assessment
- Quantum resistance evaluation

## Attack Tools Location

The calibrated attack estimators are located at:
```
/home/acid/Projects/NINE65/MANA-private/
```

Available binaries:
- `attack-estimator` - General lattice attack cost estimation
- `calibrated-estimator` - Kyber-validated security estimator
- `self-cryptanalysis` - Full attack suite with tool validation
- `lattice-attack` - LLL/BKZ simulation
- `k-elimination-attack` - K-Elimination specific analysis

## Usage Patterns

### Assess Encryption Strength
When asked to assess encryption strength:
1. Identify the cryptographic primitive (RSA, AES, Ring-LWE, etc.)
2. Extract parameters (key size, modulus, ring dimension, etc.)
3. Run appropriate attack estimator
4. Compare to known security levels (AES-128 = 128 bits, etc.)
5. Report vulnerabilities or confirm security

### Penetration Test Workflow
1. **Reconnaissance**: Identify cryptographic implementations
2. **Analysis**: Extract parameters and configurations
3. **Attack Estimation**: Calculate attack costs
4. **Exploitation Assessment**: Determine feasibility
5. **Report**: Document findings with calibrated estimates

### Validate Own Security
When validating MANA/QMNF FHE:
```bash
cd /home/acid/Projects/NINE65/MANA-private
./target/release/self-cryptanalysis
```

## Security Levels Reference

| Security Level | Classical Bits | Quantum Bits | Equivalent |
|----------------|----------------|--------------|------------|
| Minimal | 80 | 64 | Legacy |
| Standard | 128 | 100 | AES-128 |
| Strong | 192 | 150 | AES-192 |
| Maximum | 256 | 200 | AES-256 |
| Paranoid | 384+ | 300+ | Beyond AES |

## Lattice Attack Quick Reference

For Ring-LWE with parameters (n, q, sigma):
- Security increases with n (ring dimension)
- Security decreases with q (modulus size)
- Security increases with sigma (error std dev)

Formula (ADPS16):
```
sqrt(beta/d) * ||target|| <= delta^(2*beta - d) * det(L)^(1/d)
```

Where:
- beta = BKZ block size
- d = lattice dimension (n + m + 1)
- delta = Hermite factor
- ||target|| = error norm

## Quantum Considerations

- **Shor's Algorithm**: Breaks RSA, DH, ECDH - does NOT apply to lattice-based crypto
- **Grover's Algorithm**: Provides sqrt speedup - reduces security by ~50%
- **Lattice Attacks**: Quantum sieving gives ~10% improvement

## Authorization Requirements

RedShirt should only be used for:
- Authorized penetration testing engagements
- Security research and validation
- CTF competitions
- Defensive security assessment
- Self-cryptanalysis of own systems

**Never use for unauthorized attacks or malicious purposes.**

## Integration with MANA FHE

RedShirt's tools are calibrated using MANA FHE research:
- Validated against Kyber-512/768/1024 (within ±10 bits)
- Uses same methodology as NIST PQC evaluation
- Conservative estimates (err toward lower security claims)

See `/home/acid/Projects/NINE65/MANA-private/CRYPTANALYSIS_PROOF.md` for full methodology.
