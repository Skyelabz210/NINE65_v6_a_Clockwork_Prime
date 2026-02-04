# RedShirt Quick Reference Card

## One-Liners

```bash
# Assess any lattice scheme
cd /home/acid/Projects/NINE65/MANA-private && ./target/release/calibrated-estimator

# Full self-attack (validates tools then attacks QMNF)
./target/release/self-cryptanalysis

# Quick Kyber comparison
./target/release/calibrated-estimator | grep -A5 "Kyber"
```

## Security Level Cheat Sheet

| Bits | Status | Equivalent | Time to Break |
|------|--------|------------|---------------|
| 64 | BROKEN | DES | Hours |
| 80 | WEAK | 2-key 3DES | Months |
| 112 | LEGACY | 3-key 3DES | Years |
| 128 | SECURE | AES-128 | 10^20 years |
| 192 | STRONG | AES-192 | Heat death |
| 256 | MAXIMUM | AES-256 | Beyond physics |

## Hermite Factor Formula

```
delta = ((pi*beta)^(1/beta) * beta / (2*pi*e))^(1/(2*(beta-1)))
```

## BKZ Cost Models

| Model | Formula | Use Case |
|-------|---------|----------|
| Core-SVP | 2^(0.292*β) | Standard estimate |
| MATZOV | 2^(0.257*β + 16.4) | Aggressive estimate |
| Quantum | 2^(0.265*β) | Post-quantum |

## ADPS16 Success Condition

```
sqrt(β/d) × ||target|| ≤ δ^(2β-d) × det(L)^(1/d)
```

## Parameter Sanity Checks

**Ring-LWE n=1024**:
- log(q) <= 16: ~220 bits (SECURE)
- log(q) = 20: ~164 bits (SECURE)
- log(q) = 30: ~92 bits (INSECURE!)

**Ring-LWE n=4096**:
- log(q) = 30: ~536 bits (VERY SECURE)

## Attack Cost Interpretation

```
< 2^80   → Practically broken
2^80-100 → Nation-state risk
2^100-128 → Theoretically weak
> 2^128  → Secure
> 2^256  → Overkill
```

## Common Vulnerabilities

1. **RSA-1024**: ~80 bits, BROKEN
2. **RSA-2048**: ~112 bits, WEAK for long-term
3. **ECDH-256**: ~128 bits but QUANTUM VULNERABLE
4. **Kyber-512**: ~118 bits, POST-QUANTUM SECURE
5. **QMNF-Standard**: ~536 bits, EXTREMELY SECURE

## Emergency Response

If you find < 128-bit security:
1. Document the finding immediately
2. Assess data exposure window
3. Prioritize remediation
4. Consider harvest-now-decrypt-later threat
5. Plan migration to stronger parameters
