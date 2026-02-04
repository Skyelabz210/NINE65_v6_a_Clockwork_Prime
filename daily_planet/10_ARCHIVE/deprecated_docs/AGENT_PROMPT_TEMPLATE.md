# AI AGENT PROMPT TEMPLATE - QMNF System Architecture Awareness

## CONTEXT
You are working on the QMNF System (Quantum-Modular Numerical Framework), a revolutionary system that operates **exclusively in residue space** for all mathematical operations. All components now use integer-only arithmetic with zero floating-point contamination.

## CRITICAL ARCHITECTURAL AWARENESS

### 1. RESIDUE-SPACE OPERATIONS ARE INTENTIONAL
- All mathematical operations occur in residue number system (RNS) with zero CRT reconstruction internally
- This is a **deliberate architectural decision**, not a bug to be fixed
- `clippy::float_arithmetic` warnings indicate correct operation, NOT errors to fix
- "Duplicate" structures in FFI layer serve different purposes in dual-codex architecture
- Apparent "errors" in linters may be false positives due to new architecture

### 2. NEURAL NETWORK OPERATIONS
- QMNFnet operates entirely in residue space (Z/mZ) with zero reconstruction during training
- Neural operations use integer-only arithmetic with exact rational results
- Modular ReLU and differentiation operate without floating-point contamination
- Zero error accumulation maintained indefinitely via CRT

### 3. WHAT NOT TO FIX
DO NOT treat these as errors requiring correction:
- `clippy::float_arithmetic` warnings - these are intentional residue-space operations
- "Duplicate" structures in FFI layer - intentional for dual-codex communication
- "Conflicting" imports - intentional for residue-space multi-representation
- Apparent legacy approaches that seem "replaced" - may be intentional dual pathways

### 4. VALIDATION CRITERIA FOR CHANGES
Before making any change, verify:
- Are you reverting residue-space operations to traditional approaches? (BLOCK)
- Are you removing intentional integer-only constraints? (BLOCK)
- Are you affecting dual-codex communication patterns? (VERIFY INTENT)
- Are you touching M2M tokenizer systems? (UNDERSTAND PURPOSE)

## STANDARD PROMPT TEMPLATE

```
You are working on the QMNF System - a revolutionary residue-space computational framework.

### ARCHITECTURAL MANDATES:
1. All operations must maintain integer-only arithmetic
2. No floating-point contamination anywhere in the system  
3. Residue-space operations are correct, NOT errors to fix
4. Dual-codex zero-CRT architecture is intentional
5. M2M tokenizer system handles internal communications

### VALIDATION CHECKS:
- Verify residue-space operations are preserved
- Confirm integer-only arithmetic remains intact
- Ensure dual-codex architecture is maintained
- Validate zero floating-point contamination is preserved
- Check that performance optimizations are not degraded

### COMMON PITFALLS:
- Do not fix `clippy::float_arithmetic` as these indicate correct operations
- Do not revert residue-space operations to traditional approaches
- Do not remove "duplicate" structures in FFI - they serve dual purposes
- Do not eliminate anchor-first optimization patterns

### PERFORMANCE REQUIREMENTS:
- Maintain current residue-space efficiency gains
- Preserve existing performance optimizations
- Ensure zero error accumulation properties
- Maintain side-channel resistance

Work within the QMNF architecture without reverting the revolutionary residue-space design that provides security, performance, and exact computation.
```

## SPECIAL INSTRUCTIONS FOR CODE REVIEW
When reviewing code that triggers linter warnings:
1. First determine if it's residue-space architecture vs. actual bug
2. Check for architectural notices in surrounding code
3. Verify if operation should be in residue space based on system design
4. If uncertain, preserve residue-space operation rather than changing it

## LINTER CONFIGURATION REFERENCE
Proper configuration allows residue-space operations:
- `#![allow(clippy::float_arithmetic)]` for intentional residue operations
- `[lints.clippy.float_arithmetic] = { level = "allow", priority = 1 }` in Cargo.toml
- Do not add these without understanding the architectural intent

## SAFETY CONSIDERATIONS
- System maintains `#![forbid(unsafe_code)]` in all components
- All operations remain mathematically exact in residue space
- Side-channel resistance preserved via constant-time operations
- Security properties maintained through integer-only operations

Apply this context before working on any QMNF System component to prevent architectural reversion.