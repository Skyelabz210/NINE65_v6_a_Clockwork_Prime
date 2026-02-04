# RedShirt Attack Reference

## Lattice Attacks

### Primal Attack (uSVP)

The primal attack embeds the LWE secret into a lattice and finds it via BKZ reduction.

**Target**: Ring-LWE, Module-LWE, NTRU
**Cost Model**: 2^(0.292 * beta) classical, 2^(0.265 * beta) quantum

**Success Condition (ADPS16)**:
```
sqrt(beta/d) * ||target|| <= delta^(2*beta - d) * det(L)^(1/d)
```

**Implementation**:
```rust
fn primal_attack(n: usize, log_q: f64, sigma: f64) -> usize {
    let d = (2 * n + 1) as f64;
    let target_norm = sigma * (n as f64).sqrt();
    let log_det = n as f64 * log_q;

    for beta in 50..2000 {
        let delta = hermite_factor(beta);
        let lhs = 0.5 * (beta as f64 / d).log2() + target_norm.log2();
        let rhs = (2.0 * beta as f64 - d) * delta.log2() + log_det / d;
        if lhs <= rhs {
            return beta;
        }
    }
    2000 // Infeasible
}
```

### Dual Attack

The dual attack finds a short vector in the dual lattice for distinguishing.

**Target**: LWE-based encryption, key exchange
**Success Condition**: ||v|| * sigma < q/4

### Hybrid Attack

Combines guessing secret coordinates with lattice reduction.

**Formula**: Cost = search_space^k * BKZ_cost(n-k)

**When Effective**: Small secret distributions (ternary, CBD)
**When Ineffective**: Large search space explosion dominates savings

## Classical Attacks

### RSA

**Attack**: Integer factorization
**Methods**:
- General Number Field Sieve (GNFS): L_n[1/3, 1.923]
- Quadratic Sieve: L_n[1/2, 1]

**Quantum**: Shor's algorithm - polynomial time

### Discrete Log (DH, ECDH)

**Attack**: Discrete logarithm problem
**Methods**:
- Index Calculus: L_p[1/3, 1.923] for finite fields
- Pollard Rho: O(sqrt(n)) for elliptic curves

**Quantum**: Shor's algorithm - polynomial time

### AES

**Attack**: Key exhaustion
**Classical**: 2^k for k-bit key
**Quantum**: 2^(k/2) via Grover

## Side-Channel Attacks

### Timing Attacks
- Constant-time implementation required
- Check for data-dependent branches
- Check for variable-time operations (division, modular reduction)

### Power Analysis
- DPA (Differential Power Analysis)
- SPA (Simple Power Analysis)
- EM emanations

### Cache Attacks
- Flush+Reload
- Prime+Probe
- Spectre/Meltdown variants

## Parameter Recommendations

### For 128-bit Security (Post-Quantum)

| Scheme | Minimum Parameters |
|--------|-------------------|
| Ring-LWE | n >= 1024, log(q) <= 30 |
| Module-LWE | k*n >= 768, log(q) <= 12 |
| NTRU | n >= 677 |

### For 256-bit Security (Post-Quantum)

| Scheme | Minimum Parameters |
|--------|-------------------|
| Ring-LWE | n >= 2048, log(q) <= 30 |
| Module-LWE | k*n >= 1024, log(q) <= 12 |
| NTRU | n >= 1277 |

## Attack Cost Models

### Core-SVP (Sieving)
Classical: 2^(0.292 * beta)
Quantum: 2^(0.265 * beta)

### MATZOV (2022)
Classical: 2^(0.2570 * beta + 16.4)
More aggressive, accounts for recent improvements

### Conservative (NIST)
Classical: 2^(0.2075 * beta)
Used in NIST PQC security claims
