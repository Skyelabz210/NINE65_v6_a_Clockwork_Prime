# EXECUTION CHECKLIST: EPRAM Gap Closure
**Generated:** 2026-01-08  
**Status:** 43% Complete → Target 100%

---

## CRITICAL PATH (Must Complete First)

### Gate 1: EPRAM Foundation
```
STATUS: [ ] NOT STARTED
BLOCKS: Everything else

Tasks:
[ ] T-001: Create epram/mod.rs with EPRAMCell trait
[ ] T-002: Create EPRAMField<C, N> struct with step()
[ ] T-003: Create Topology enum (Ring, Grid, Complete, Custom)
[ ] T-004: Create TerminationContract enum (FixedPoint, Lyapunov, Cycle)
[ ] T-005: Wire fourth_attractor_step as default transition

Tests Required:
[ ] test_epram_step_synchronous() - verify snapshot semantics
[ ] test_epram_fixed_point() - verify convergence
[ ] test_fourth_attractor_convergence() - verify k=3/4 bounds
```

### Gate 2: Permanent Residents → EPRAM
```
STATUS: [ ] NOT STARTED  
DEPENDS: Gate 1

Tasks:
[ ] T-006: impl EPRAMCell for MontgomeryValue
[ ] T-007: impl EPRAMCell for DualCodexLane
[ ] T-008: impl EPRAMField for DualCodex (paired fields)
[ ] T-009: Wire recover_k() into DualCodex::transition()

Tests Required:
[ ] test_montgomery_never_converts() - no to_standard() calls
[ ] test_dual_codex_phase_differential() - k recovery exact
[ ] test_k_elimination_in_field_step() - works inside EPRAM
```

### Gate 3: Cyclotomic → EPRAM
```
STATUS: [ ] NOT STARTED
DEPENDS: Gate 1

Tasks:
[ ] T-010: impl EPRAMCell for Fp2Slot
[ ] T-011: impl EPRAMField for CyclotomicRing (N/2 slots)
[ ] T-012: Wire extract_sine/cosine as slot accessors
[ ] T-013: Wire rotate(k) as field-level operation

Tests Required:
[ ] test_cyclotomic_slot_independence() - slots evolve independently
[ ] test_extract_sine_is_odd_coefficients() - coefficient parity
[ ] test_rotate_is_phase_shift() - X^k multiplication exact
```

### Gate 4: Orchestrator Implementation
```
STATUS: [ ] NOT STARTED
DEPENDS: Gates 1, 2, 3

Tasks:
[ ] T-014: Create ResidueSpaceOrchestrator struct
[ ] T-015: Implement decide() as attractor convergence
[ ] T-016: Implement one_shot_learn() template storage
[ ] T-017: Implement frst_update() rail refinement
[ ] T-018: Wire PLMGValidator for rail/void detection

Tests Required:
[ ] test_decide_converges_to_template() - attractor convergence
[ ] test_one_shot_stores_template() - template in F_p² slots
[ ] test_frst_integer_step() - no float gradients
[ ] test_plmg_void_ratio() - ~72% rejection rate
```

---

## FILES TO CREATE

```
src/
├── epram/
│   ├── mod.rs              [ ] EPRAMCell trait, EPRAMField struct
│   ├── topology.rs         [ ] Neighborhood definitions
│   ├── termination.rs      [ ] FixedPoint, Lyapunov, Cycle contracts
│   └── fourth_attractor.rs [ ] Default transition rule
├── residents/
│   ├── montgomery.rs       [ ] impl EPRAMCell for MontgomeryValue
│   ├── dual_codex.rs       [ ] impl EPRAMField for DualCodex
│   └── cyclotomic.rs       [ ] impl EPRAMField for CyclotomicRing
└── orchestrator/
    ├── mod.rs              [ ] ResidueSpaceOrchestrator
    ├── decide.rs           [ ] Attractor convergence decision
    ├── learn.rs            [ ] one_shot_learn, frst_update
    └── plmg.rs             [ ] Rail/void validator
```

---

## REGRESSION GUARDS

### Forbidden Patterns (FAIL if found after Gate completion)
```bash
# Run after each gate:
./regression/scan.sh

# Contents:
grep -rn "f64\|f32" --include="*.rs" src/epram/ src/residents/ src/orchestrator/
grep -rn "to_standard\|from_montgomery" --include="*.rs" src/
grep -rn "decode\|encode" --include="*.rs" src/
```

### Required Patterns (FAIL if missing after Gate completion)
```bash
# After Gate 1:
grep -rn "impl EPRAMCell" src/ | wc -l  # Should be >= 1

# After Gate 2:
grep -rn "impl EPRAMCell for Montgomery" src/  # Must exist

# After Gate 3:
grep -rn "impl EPRAMField for Cyclotomic" src/  # Must exist

# After Gate 4:
grep -rn "fn decide\|fn one_shot_learn\|fn frst_update" src/  # All must exist
```

---

## DEPENDENCY GRAPH

```
T-001 ────┬────► T-006 ────┬────► T-014
T-002 ────┤                │
T-003 ────┤     T-007 ────┤
T-004 ────┤                │      T-015
T-005 ────┘     T-008 ────┤
                           │      T-016
          T-009 ──────────┤
                           │      T-017
T-010 ────┬────► T-012 ────┤
T-011 ────┤                │      T-018
          │     T-013 ────┘
```

---

## HANDOFF LOG

| Timestamp | Agent | Action | Notes |
|-----------|-------|--------|-------|
| 2026-01-08 | Claude | GAP_ANALYSIS | 14 gaps identified, 4 critical |
| | | | |

---

## NEXT ACTIONS

1. **Implement `EPRAMCell` trait** (T-001) - ~50 lines, unlocks everything
2. **Implement `EPRAMField<C, N>`** (T-002) - ~100 lines, synchronous step
3. **Wire `fourth_attractor_step`** (T-005) - Already exists, just wire it

**Estimated time to Gate 1:** 2-3 hours of focused implementation
**Estimated time to full closure:** 2-3 days

---

## KILL COUNT UPDATE (Pending)

If all gates completed:
- **NEW KILL**: EPRAM Field Abstraction (NOV, 25 pts)
- **NEW KILL**: Residue Space Orchestrator (HRD, 50 pts) 
- **TOTAL**: +75 pts to grail collection
