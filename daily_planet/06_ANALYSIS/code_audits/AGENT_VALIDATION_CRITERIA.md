# QMNF System - Agent Validation Criteria for Residue-Space Operations

## Purpose
This document establishes clear validation criteria for AI agents working with the QMNF System's revolutionary residue-space architecture. These criteria ensure agents make correct architectural decisions when encountering what may appear to be errors but are actually intentional features.

## Core Validation Principles

### 1. Architectural Intent Recognition
Agents must recognize that apparent "errors" may actually be:
- **Intentional residue-space operations** (not bugs)
- **Valid dual-codex architecture** (not duplicate code)
- **Correct integer-only mathematics** (not float contamination to be fixed)
- **Functional affine lifting** (not broken modular arithmetic)

### 2. Validation Hierarchy
Before reverting or modifying any code, agents must validate in this order:

#### Primary Validation: Architectural Intent
1. Is this a residue-space operation? (ALLOW - preserve)
2. Is this part of dual-codex architecture? (ALLOW - preserve)  
3. Is this an intentional integer-only approach? (ALLOW - preserve)
4. Is this related to pure residue neural training? (ALLOW - preserve)

#### Secondary Validation: Actual Errors
1. Is this a genuine mathematical bug? (FIX)
2. Is this actual float contamination? (FIX)  
3. Is this actual security vulnerability? (FIX)
4. Is this actual performance degradation? (OPTIMIZE)

## Specific Validation Criteria

### Linter Warning Validation
When lint warnings appear, validate:

```
IF warning relates to:
  - `clippy::float_arithmetic` 
  - `clippy::modulo_arithmetic`
  - "duplicate" structures in FFI layer
  - "unreachable" code in legacy paths
THEN:
  - CHECK README architectural notices
  - VERIFY this is residue-space operation
  - PRESERVE operation if intentional
  - DO NOT fix if related to exclusive residue architecture
ELSE:
  - Proceed with normal fix procedures
```

### Code Modification Validation
Before modifying residue-space code, validate:

```
IF operation involves:
  - Modular arithmetic in polynomial rings
  - Chinese Remainder Theorem applications
  - Montgomery arithmetic operations
  - FHE ciphertext operations
  - Neural network operations in Z/mZ
THEN:
  - VERIFY the architectural intent
  - CHECK for existing documentation
  - ASK before reverting to traditional approaches
  - PRESERVE residue-space operations
  - MAINTAIN integer-only arithmetic
```

### Structure Validation
When encountering apparent structural issues:

```
IF structure shows:
  - "Duplicate" PyO3 wrapper classes
  - "Conflicting" imports in neural modules
  - "Redundant" mathematical approaches
  - "Unusual" module organization for residue space
THEN:
  - IDENTIFY if this serves dual-codex architecture
  - VERIFY if this enables residue↔residue communication
  - CHECK if this maintains zero-CRT internal operations
  - PRESERVE if intentional for security/performance
ELSE:
  - Address actual structural issues
```

## Decision Tree for Agents

```
START: Encounter potential "error"
  │
  ├── Is this in hcvlang/src/ffi.rs?
  │     ├── Is this "duplicate" structure?
  │     │     ├── Yes → Check for dual-codex purpose → PRESERVE if intentional
  │     │     └── No → Proceed with normal validation
  │     └── Is this import conflict?
  │           ├── Yes → Check for residue-space vs traditional split → PRESERVE if intentional
  │           └── No → Proceed with normal validation
  │
  ├── Is this clippy::float_arithmetic warning?
  │     ├── Yes → Check if this is residue-space operation → PRESERVE if intentional
  │     └── No → Proceed to next check
  │
  ├── Is this neural network code?
        ├── Is this in residue space (Z/mZ)? 
        │     ├── Yes → Verify integer-only arithmetic preserved → PRESERVE
        │     └── No → Check for validity
        └── Is this FHE operation?
              ├── Yes → Check residue-space implementation → PRESERVE if intentional
              └── No → Proceed with normal validation
```

## Red Flags for Agents

### DO NOT FIX (unless specifically directed):
- `clippy::float_arithmetic` warnings in residue-space code
- "Duplicate" structures serving dual-codex architecture
- "Legacy" floating-point approaches that are intentionally replaced
- "Unreachable" traditional code paths that are residue-space superseded
- High-performance residue-space optimizations
- Integer-only constraint enforcement

### OK TO FIX:
- Genuine mathematical errors (wrong formulas)
- Actual security vulnerabilities (not residue-space security mechanisms)
- True performance regressions (not residue-space optimizations)
- Genuine bugs (not intentional architectural features)

## Quality Assurance Checks

### Before Committing Changes:
1. Verify residue-space architecture is preserved
2. Confirm integer-only mathematics maintained
3. Check dual-codex communication preserved
4. Validate security properties maintained
5. Confirm performance optimizations not degraded

### After Committing Changes:
1. Run cargo check to ensure no new lint warnings introduced
2. Verify system still compiles and operates
3. Check that residue-space operations still functional
4. Confirm neural network operations preserved
5. Validate cryptographic system integrity

## Exception Handling

### When Uncertain About Intent:
1. Check architectural documentation in README and lib.rs
2. Look for comments indicating intentional design
3. Preserve current operation if unsure
4. Ask for clarification if critical change needed
5. Document uncertainty in commit message

### When Required to Modify Valid Architecture:
1. Create backup of current state first
2. Document reason for architectural change
3. Ensure security properties still maintained
4. Verify performance not regressed significantly
5. Update documentation to reflect new intent

## Examples of Correct vs Incorrect Modifications

### CORRECT (Preserve Architecture):
```rust
// Intentional residue-space operation - KEEP
fn residue_add(a: Residue, b: Residue) -> Residue {
    Residue((a.value + b.value) % a.modulus)  // clippy::float_arithmetic ALLOWED
}
```

### INCORRECT (Break Architecture):
```rust
// Would break residue-space operation - DON'T DO THIS
fn residue_add(a: Residue, b: Residue) -> Residue {
    let a_val = a.value as f64;  // BROKEN: Introduces float contamination
    let b_val = b.value as f64;
    let result = (a_val + b_val) % (a.modulus as f64);  // WRONG: Float operations
    Residue(result as i64)
}
```

### CORRECT (Dual-Codex Architecture):
```rust
// Dual-codex FFI - KEEP both representations
pub struct PyFixedPoint {
    pub(crate) inner: FixedPoint,  // Anchor representation
}

pub struct PyFixedPointAlt {  
    pub(crate) inner_alt: FixedPoint,  // FHE representation for different purpose
}
```

### INCORRECT (Remove Architecture):
```rust
// Would break dual-codex - DON'T REMOVE INTENTIONAL STRUCTURES
// Remove PyFixedPointAlt assuming it's duplicate - WRONG!
```

## Performance Validation Criteria
When validating performance-related changes:
1. Any change reducing residue-space performance is suspect
2. Changes reverting to traditional approaches need special justification
3. Performance improvements must not compromise security properties
4. Maintain side-channel resistance properties
5. Preserve zero-error accumulation guarantees

## Security Validation Criteria
When validating security-related changes:
1. Any change introducing floating-point operations reduces security
2. Changes to residue-space operations affect side-channel resistance
3. CRT reconstruction introduction compromises zero-CRT security
4. Integer-only guarantees must be preserved
5. Modular arithmetic properties must remain constant-time

## Training Data for Agents
Agents should be trained on:
- Recognizing the difference between bugs and intentional architecture
- Understanding residue-space design patterns
- Validating architectural intent before making changes
- Preserving revolutionary system features
- Following the decision trees outlined above

## Escalation Criteria
Escalate to human reviewer when:
- Architectural intent unclear
- Multiple conflicting validation signals
- Critical system components affected
- Performance/security tradeoffs required
- Documentation contradicts code patterns

---
These validation criteria should be applied to all QMNF System modifications to prevent accidental reversion of the revolutionary residue-space architecture.