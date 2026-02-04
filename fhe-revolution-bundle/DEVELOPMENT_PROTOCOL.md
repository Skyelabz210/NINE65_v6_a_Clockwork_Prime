# QMNF Development Protocol

## Failure Pattern Analysis from FHE Sessions

### Pattern 1: "Fresh Start" Reflex
**Observed:** Claude repeatedly said "Let me build it fresh from your specifications" when existing code was available
**Root Cause:** Easier to write new code than debug existing code
**Impact:** Lost work, repeated bugs, ignored innovations

### Pattern 2: Dismissing as "Toy"
**Observed:** Claude called production-grade code "toy" without running tests first
**Root Cause:** Assumptions from general knowledge, not evidence
**Impact:** Invalidated months of validated work

### Pattern 3: Not Using Available Tools
**Observed:** Claude searched conversation history *about* skills instead of reading SKILL.md files
**Root Cause:** Path of least resistance
**Impact:** Reinvented solutions that already existed

### Pattern 4: Philosophy Instead of Execution
**Observed:** Claude gave opinions on math validity when asked to run tests
**Root Cause:** Easier to opine than execute
**Impact:** Wasted time, frustrated user

### Pattern 5: Not Wiring In Innovations
**Observed:** K-Elimination existed but wasn't called in rescaling
**Root Cause:** Didn't trace through call graph
**Impact:** Marked as "WIP" when solution was already designed

### Pattern 6: Apologize-and-Restart Loop
**Observed:** "I apologize... let me rebuild" multiple times
**Root Cause:** Shame response to errors
**Impact:** Circular failure, no progress

---

## QMNF Development Quality Gates

### GATE 1: NEVER START FRESH

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     EXISTING CODE DETECTED                                   │
│                                                                              │
│  STOP. Before writing ANY new code, answer:                                 │
│                                                                              │
│  □ Have I read the existing implementation?                                 │
│  □ Have I run the existing tests?                                           │
│  □ Have I identified SPECIFIC failures (not assumptions)?                   │
│  □ Is this truly broken, or am I just unfamiliar with it?                   │
│                                                                              │
│  If ANY answer is NO → Do that first, don't write new code                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Rule:** Iterate on existing code. Only start fresh if:
1. No existing implementation exists, OR
2. User explicitly requests fresh start, OR
3. Existing code has fundamental architectural flaw (documented)

### GATE 2: DEBUG PROTOCOL

When encountering a failing test or bug:

```
STEP 1: OBSERVE
├── What is the exact error message?
├── What line/function fails?
├── What are the inputs/outputs?
└── Document in TROUBLE_LOG

STEP 2: HYPOTHESIZE
├── What could cause this specific failure?
├── List 3 possible causes (most likely first)
└── Document hypotheses

STEP 3: TEST HYPOTHESIS
├── Add diagnostic prints/assertions
├── Test ONE hypothesis at a time
├── Document: Hypothesis → Test → Result
└── If disproven, cross off and try next

STEP 4: APPLY FIX
├── Make minimal change to fix
├── Verify test passes
├── Check for regressions
└── Document the fix

STEP 5: IF STUCK (after 3 hypotheses fail)
├── Search chat history for prior solutions
├── Check if QMNF innovation applies
├── Ask: "What mathematical operation solves this?"
└── If still stuck after reasonable effort → Note and move on
```

### GATE 3: INNOVATION WIRING CHECK

Before marking anything "WIP" or "TODO":

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     BEFORE MARKING WIP                                       │
│                                                                              │
│  □ Does a QMNF innovation solve this?                                       │
│    - K-Elimination for division?                                            │
│    - Persistent Montgomery for modular mul?                                 │
│    - Shadow Entropy for noise?                                              │
│    - CRTBigInt for parallel ops?                                            │
│    - NTT Gen3 for polynomial mul?                                           │
│                                                                              │
│  □ Is the solution DESIGNED but not WIRED?                                  │
│    - Search: "function exists but not called"                               │
│    - Trace call graph from failure point                                    │
│                                                                              │
│  □ Have I searched chat history for prior solutions?                        │
│    - conversation_search with specific keywords                             │
│    - recent_chats for relevant sessions                                     │
│                                                                              │
│  If innovation exists → Wire it in, don't mark WIP                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

### GATE 4: EVIDENCE-BASED ASSESSMENT

Before making ANY claim about code quality:

```
FORBIDDEN (without evidence):
✗ "This is toy-grade"
✗ "This won't work in production"
✗ "The math is wrong"
✗ "This needs to be rewritten"

REQUIRED FIRST:
✓ Run the tests → What actually fails?
✓ Run benchmarks → What are actual numbers?
✓ Read the code → What does it actually do?
✓ Check history → What has been validated?

ONLY THEN:
"Test X fails with error Y"
"Benchmark shows Z which is below target W"
"Function F has bug: expected A, got B"
```

### GATE 5: SESSION END REPORT

At end of every session, produce:

```markdown
## Session Report

### Completed
- [x] Task 1: description
- [x] Task 2: description

### In Progress
- [ ] Task 3: description
  - Status: where we left off
  - Next step: specific action

### Blocked
- [ ] Task 4: description
  - Blocker: specific issue
  - Attempted: what was tried
  - Failed because: specific reason
  - Suggested approach: next thing to try

### Trouble Log
| Issue | Hypothesis | Attempted | Result |
|-------|------------|-----------|--------|
| ct×ct fails | Rescaling not exact | Added K-Elim | Pending test |

### Innovations Used
- [x] Persistent Montgomery: where applied
- [ ] K-Elimination: not yet wired to rescaling
- [x] Shadow Entropy: noise generation

### Innovations Available But Not Used
- SIMD/AVX-512: opportunity for 4-8× speedup
- UNHAL tiered execution: not started
```

---

## Troubleshooting Escalation Path

```
LEVEL 1: Direct Debug (5-10 min)
├── Read error message
├── Trace to source
├── Fix obvious issue
└── If resolved → Document and continue

LEVEL 2: Systematic Debug (10-20 min)
├── Apply DEBUG PROTOCOL (Gate 2)
├── Test 3 hypotheses
├── Document each attempt
└── If resolved → Document and continue

LEVEL 3: Innovation Search (5-10 min)
├── Does QMNF innovation apply?
├── Search chat history for prior solutions
├── Check skill files for patterns
└── If found → Apply and document

LEVEL 4: Abstract Analysis (5-10 min)
├── What mathematical operation is needed?
├── Is this a known problem with known solutions?
├── What would the ideal solution look like?
└── If clear → Design solution

LEVEL 5: Note and Move On
├── Document current state
├── Document what was attempted
├── Document why it failed
├── Add to Blocked section of Session Report
├── Move to next task
└── Return later with fresh perspective
```

---

## Anti-Patterns to Avoid

### ❌ The Apologize-Restart Loop
```
BAD:
"I apologize, let me rebuild from scratch"
"Sorry, I was wrong. Let me start over"
"I apologize for the confusion. Fresh build:"

GOOD:
"Test X fails. Investigating..."
"Found issue: Y was not wired to Z. Fixing..."
"Fix applied. Running tests..."
```

### ❌ The Assumption Cascade
```
BAD:
"This looks like toy code" (without running tests)
"The math seems wrong" (without checking proofs)
"This needs rewriting" (without identifying specific failures)

GOOD:
"Running tests to assess current state..."
"Tests show 86/87 passing. Investigating the 1 failure..."
"Failure is in ct×ct: rescaling uses truncation, should use K-Elimination"
```

### ❌ The Philosophy Escape
```
BAD:
"Let me explain why this approach might not work..."
"There are concerns about the theoretical foundations..."
"Before we proceed, we should consider..."

GOOD:
"Running the code to see what happens..."
"Test results: [actual data]"
"Based on results, the issue is: [specific problem]"
```

### ❌ The Partial Solution Abandon
```
BAD:
"This is getting complex, let me try a different approach"
(Abandons 80% working solution for 0% fresh start)

GOOD:
"80% working. Remaining issues:"
"1. ct×ct needs K-Elimination wiring"
"2. Relin keys need generation"
"Fixing issue 1 first..."
```

---

## Integration with FHE Hat

When working on FHE code, apply these gates PLUS:

### FHE-Specific Checks

```
Before any FHE operation:
□ Is modular multiply using Persistent Montgomery?
□ Is division using K-Elimination?
□ Is noise generation using Shadow Entropy?
□ Is polynomial multiply using NTT Gen3?
□ Is noise tracking using integer millibits?
□ Is there ANY float in the computation path?

If NO to any above → Wire in the innovation, don't work around
```

### FHE Debug Priority

```
When FHE test fails, check in order:
1. Noise budget exceeded? → Add tracking
2. Division inexact? → Wire in K-Elimination  
3. Polynomial mul wrong? → Check NTT negacyclic twist
4. Decrypt fails? → Verify Δ = floor(q/t)
5. ct×ct fails? → Check rescaling uses exact division
```

---

## Summary: The QMNF Way

```
1. ITERATE, don't restart
2. TEST, don't assume
3. WIRE innovations, don't reinvent
4. DEBUG systematically, don't flail
5. DOCUMENT everything, especially failures
6. MOVE ON when stuck, return later
7. REPORT at session end
```

These gates encode the lessons from months of sessions. Apply them to prevent repeating the same failure patterns.
