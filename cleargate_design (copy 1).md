# QMNF FHE Frontend: Design Specification
## Codename: "ClearGate" - Where Complexity Disappears

**Date:** December 29, 2025
**Goal:** Zero-friction FHE with obfuscated backend

---

## DESIGN PHILOSOPHY

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║   USER SEES:          "encrypted_sum = a + b"                                ║
║                                                                               ║
║   BACKEND DOES:       Dual-RNS decomposition                                 ║
║                       Centered representative crossing                        ║
║                       K-elimination for exact arithmetic                      ║
║                       Shadow entropy noise generation                         ║
║                       Automatic rescaling                                     ║
║                       Anchor consistency verification                         ║
║                       Montgomery domain persistence                           ║
║                                                                               ║
║   USER KNOWS:         Nothing about any of that                              ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## LAYER ARCHITECTURE

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 1: User Interface                                                    │
│  ─────────────────────────────────────────────────────────────────────────  │
│  • Web UI (optional)                                                        │
│  • CLI tool                                                                 │
│  • Language bindings (Python, JS, Go)                                       │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 2: ClearGate API (Public)                                            │
│  ─────────────────────────────────────────────────────────────────────────  │
│  • SecureInt, SecureVec, SecureMat                                          │
│  • Arithmetic operators (+, -, *, /)                                        │
│  • Comparisons (==, <, >)                                                   │
│  • Aggregations (sum, mean, max, min)                                       │
│  • encrypt() / decrypt()                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 3: Orchestration (Internal)                                          │
│  ─────────────────────────────────────────────────────────────────────────  │
│  • Operation routing                                                        │
│  • Automatic parameter selection                                            │
│  • Noise budget tracking                                                    │
│  • Key management                                                           │
│  • Batching optimization                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 4: QMNF Engine (Obfuscated)                                          │
│  ─────────────────────────────────────────────────────────────────────────  │
│  • Dual-RNS with centered representative invariant                          │
│  • K-Elimination exact division                                             │
│  • Persistent Montgomery multiplication                                     │
│  • Shadow Entropy noise                                                     │
│  • CRTBigInt parallel arithmetic                                            │
│  • Bi-Anchor CRT recovery                                                   │
│  • Bootstrap-free noise management                                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## PUBLIC API DESIGN

### Core Types

```rust
// What users see - dead simple
pub struct SecureInt { /* opaque */ }
pub struct SecureVec { /* opaque */ }
pub struct SecureMat { /* opaque */ }
pub struct SecureContext { /* opaque */ }
pub struct SecureKey { /* opaque */ }
```

### Initialization

```rust
// One-liner setup
let ctx = SecureContext::new(SecurityLevel::Standard)?;  // 128-bit
let key = ctx.generate_key()?;

// Or with options
let ctx = SecureContext::builder()
    .security_level(SecurityLevel::High)  // 256-bit
    .max_multiplications(100)              // depth budget
    .build()?;
```

### Encryption/Decryption

```rust
// Encrypt single value
let a: SecureInt = ctx.encrypt(42, &key)?;
let b: SecureInt = ctx.encrypt(-17, &key)?;

// Encrypt vector
let prices: SecureVec = ctx.encrypt_vec(&[100, 200, 150], &key)?;

// Encrypt matrix
let data: SecureMat = ctx.encrypt_mat(&[[1,2],[3,4]], &key)?;

// Decrypt
let result: i64 = ctx.decrypt(&a, &key)?;
let vec_result: Vec<i64> = ctx.decrypt_vec(&prices, &key)?;
```

### Arithmetic (Natural Syntax)

```rust
// These "just work" - all QMNF magic hidden
let sum = &a + &b;           // Homomorphic add
let diff = &a - &b;          // Homomorphic sub
let prod = &a * &b;          // Homomorphic mul (K-elimination hidden)
let scaled = &a * 5;         // Plaintext multiply

// Chained operations
let result = (&a + &b) * &c - &d;

// Vector operations
let total = prices.sum();           // Encrypted sum
let avg = prices.mean();            // Encrypted average
let dot = &vec1.dot(&vec2);         // Dot product

// Matrix operations
let product = &mat1.matmul(&mat2);  // Matrix multiply
```

### Comparisons (Returning Encrypted Booleans)

```rust
let is_greater: SecureBool = a.gt(&b);     // a > b (encrypted result)
let is_equal: SecureBool = a.eq(&b);       // a == b
let max_val: SecureInt = a.max(&b);        // max(a, b)
let min_val: SecureInt = a.min(&b);        // min(a, b)
```

### Privacy-Preserving Queries

```rust
// Find if value exists (returns encrypted bool)
let exists: SecureBool = prices.contains(&target);

// Conditional selection (oblivious)
let selected: SecureInt = SecureInt::select(&condition, &if_true, &if_false);

// Private lookup (no index leakage)
let value: SecureInt = secure_table.lookup(&encrypted_index);
```

---

## CLI INTERFACE

```bash
# Initialize a new secure context
cleargate init --security high --output ctx.cg

# Generate keys
cleargate keygen --context ctx.cg --output keys.cgk

# Encrypt data
echo "42" | cleargate encrypt --key keys.cgk --output value.cge
cleargate encrypt-file data.csv --key keys.cgk --output data.cge

# Compute on encrypted data
cleargate compute "a + b * c" \
    --inputs a=val1.cge,b=val2.cge,c=val3.cge \
    --output result.cge

# Decrypt result
cleargate decrypt result.cge --key keys.cgk
# Output: 127

# Batch processing
cleargate batch-compute script.cgs \
    --data encrypted_db.cge \
    --output results.cge
```

### Script Language (`.cgs` files)

```
# ClearGate Script - simple DSL for encrypted computation

LOAD prices FROM "prices.cge"
LOAD quantities FROM "quantities.cge"

total = prices * quantities
revenue = SUM(total)
average = MEAN(total)
maximum = MAX(total)

SAVE revenue TO "revenue.cge"
SAVE average TO "average.cge"
```

---

## PYTHON BINDINGS

```python
import cleargate as cg

# Setup
ctx = cg.Context(security="standard")
key = ctx.keygen()

# Encrypt
salary = ctx.encrypt(75000, key)
bonus = ctx.encrypt(5000, key)

# Compute (looks like normal Python!)
total_comp = salary + bonus
tax = total_comp * 0.25
net = total_comp - tax

# Decrypt
print(ctx.decrypt(net, key))  # 60000

# Numpy-like arrays
import numpy as np
data = np.array([100, 200, 300])
secure_data = ctx.encrypt_array(data, key)
secure_sum = secure_data.sum()
print(ctx.decrypt(secure_sum, key))  # 600
```

---

## WEB UI CONCEPT

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  ClearGate                                              [Docs] [API] [Login]│
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ENCRYPTED COMPUTATION                                              │   │
│  ├─────────────────────────────────────────────────────────────────────┤   │
│  │                                                                     │   │
│  │  Input A: [________42________] [Encrypt]  🔒 Encrypted             │   │
│  │  Input B: [________17________] [Encrypt]  🔒 Encrypted             │   │
│  │                                                                     │   │
│  │  Operation: [  A + B  ▼]                                           │   │
│  │                                                                     │   │
│  │  [═══════════ COMPUTE ═══════════]                                 │   │
│  │                                                                     │   │
│  │  Result: 🔒 ████████████████  [Decrypt]  → 59                      │   │
│  │                                                                     │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │ Operation Log:                                              │   │   │
│  │  │ • Encrypted A (128-bit security)           ✓               │   │   │
│  │  │ • Encrypted B (128-bit security)           ✓               │   │   │
│  │  │ • Homomorphic ADD                          ✓ 0.8ms         │   │   │
│  │  │ • Noise budget remaining: 94%              ✓               │   │   │
│  │  └─────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  BATCH UPLOAD                                                       │   │
│  ├─────────────────────────────────────────────────────────────────────┤   │
│  │  [Drag CSV/JSON here or click to upload]                           │   │
│  │                                                                     │   │
│  │  Computation: [SQL-like query editor]                              │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │ SELECT SUM(salary), AVG(bonus)                              │   │   │
│  │  │ FROM encrypted_employees                                     │   │   │
│  │  │ WHERE department = 'Engineering'                             │   │   │
│  │  └─────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## OBFUSCATION STRATEGY

### What Users NEVER See

| Hidden Component | Why It's Hidden |
|-----------------|-----------------|
| Dual-RNS decomposition | Implementation detail |
| Centered representative invariant | Novel contribution |
| K-Elimination algorithm | Core IP |
| Shadow Entropy harvesting | Competitive advantage |
| Anchor prime selection | Security parameter |
| Montgomery domain persistence | Optimization |
| Bi-Anchor CRT recovery | Novel theorem |
| Noise budget internals | Complexity |

### Obfuscation Techniques

1. **Opaque Types**: All internal state behind `/* opaque */`
2. **No Debug Output**: Internal values never printed
3. **Compiled Backend**: Core engine as pre-compiled `.so`/`.dylib`
4. **Encrypted Logging**: If telemetry needed, encrypt it
5. **API Versioning**: Backend can change without API breaks

### Code Structure

```
cleargate/
├── src/
│   ├── lib.rs              # Public API only
│   ├── types.rs            # SecureInt, SecureVec (opaque)
│   ├── ops.rs              # Operator overloads
│   └── bindings/           # Python, JS, etc.
│
└── engine/                 # SEPARATE CRATE - not published
    ├── src/
    │   ├── dual_rns.rs     # The secret sauce
    │   ├── k_elim.rs       # K-Elimination
    │   ├── shadow.rs       # Shadow Entropy
    │   ├── montgomery.rs   # Persistent Montgomery
    │   └── crt.rs          # Bi-Anchor CRT
    └── Cargo.toml          # [lib] crate-type = ["staticlib"]
```

---

## ERROR MESSAGES (User-Friendly)

```rust
// INTERNAL: AnchorConsistencyError { prime_idx: 2, expected: 17, got: 23 }

// USER SEES:
Error: Computation failed due to internal state inconsistency.
       This is a bug - please report it.
       Error ID: CGE-7291 (include this in bug report)
```

```rust
// INTERNAL: NoiseOverflow { budget: 0.02, required: 0.15 }

// USER SEES:
Error: Computation too deep for current security settings.
       
       You've performed ~47 multiplications, but your context
       supports ~40 at this security level.
       
       Solutions:
       1. Use a higher multiplication budget: 
          SecureContext::builder().max_multiplications(100)
       2. Restructure computation to reduce depth
       3. Use .refresh() to reset noise (adds latency)
```

---

## PERFORMANCE PROMISES

```rust
// User-visible benchmarks (what we advertise)
// Backend details hidden

/// Performance characteristics for SecureInt operations
/// 
/// | Operation      | Latency    | Notes                    |
/// |----------------|------------|--------------------------|
/// | encrypt()      | ~2ms       | One-time cost            |
/// | decrypt()      | ~1ms       | One-time cost            |
/// | add/sub        | ~10μs      | Near-instant             |
/// | multiply       | ~3ms       | Includes auto-rescale    |
/// | comparison     | ~5ms       | Returns encrypted bool   |
/// | refresh()      | ~50ms      | Resets noise budget      |
///
/// All operations maintain 128-bit security by default.
```

---

## NEXT STEPS

1. **Phase 1**: Core API in Rust (SecureInt, basic ops)
2. **Phase 2**: Python bindings via PyO3
3. **Phase 3**: CLI tool
4. **Phase 4**: Web UI (optional, for demos)
5. **Phase 5**: Enterprise features (audit logs, key escrow)

---

## NAMING OPTIONS

| Name | Vibe |
|------|------|
| **ClearGate** | "Clarity through the gate" - complexity disappears |
| **VeilMath** | Math behind a veil |
| **CipherFlow** | Encrypted data flows naturally |
| **SecureCalc** | Boring but clear |
| **Enigma** | Classic crypto reference |
| **QMNF Studio** | If we want to brand the engine |

---

*Design document created: December 29, 2025*
*Next: Implementation of Layer 2 (ClearGate API)*
