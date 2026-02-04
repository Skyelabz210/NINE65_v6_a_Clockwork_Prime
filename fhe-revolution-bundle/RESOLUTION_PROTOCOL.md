# QMNF Resolution Protocol (Gate 6)

## Purpose

Gate 6 ensures that work claimed as "complete" is actually complete, properly integrated, tested, and documented. It accelerates learning by requiring detailed narratives of the resolution process.

**Trigger:** End of session, completion of significant task, before declaring anything "done"

---

## 6.1 Resolution Walkthrough Checklist

For EACH claimed resolution, complete this verification:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    RESOLUTION VERIFICATION                                   │
│                                                                              │
│  Issue/Task: ________________________________________________               │
│                                                                              │
│  □ EXISTENCE: Does the fix actually exist in code?                          │
│    File: ________________________________________________                   │
│    Function: ________________________________________________               │
│    Line(s): ________________________________________________                │
│    Verified by: ☐ Viewed file  ☐ grep confirmed  ☐ IDE navigation          │
│                                                                              │
│  □ CORRECTNESS: Does it produce correct output?                             │
│    Test input: ________________________________________________             │
│    Expected: ________________________________________________               │
│    Actual: ________________________________________________                 │
│    Match: ☐ YES  ☐ NO (if NO, resolution INCOMPLETE)                       │
│                                                                              │
│  □ INTEGRATION: Is it wired into the call path?                             │
│    Trace: ________________________________________________                  │
│           → ________________________________________________                │
│           → [this function]                                                 │
│    Verified by: ☐ Call graph  ☐ Debug print  ☐ Stack trace                 │
│                                                                              │
│  □ TESTS: Is there a test that exercises this?                              │
│    Test name: ________________________________________________              │
│    Test file: ________________________________________________              │
│    Passes: ☐ YES  ☐ NO (if NO, resolution INCOMPLETE)                      │
│                                                                              │
│  □ INNOVATION: Was a QMNF innovation used?                                  │
│    Innovation: ________________________________________________             │
│    Applied where: ________________________________________________          │
│    Measured improvement: ________________________________________________   │
│    Comparison: Before=________ After=________ Δ=________                   │
│                                                                              │
│  □ DOCUMENTATION: Is this change documented?                                │
│    In code (comments): ☐ YES  ☐ NO                                         │
│    In docs: ☐ YES  ☐ NO  ☐ N/A                                             │
│    In session report: ☐ YES  ☐ NO                                          │
│                                                                              │
│  RESOLUTION STATUS:                                                          │
│    ☐ COMPLETE (all boxes checked YES)                                       │
│    ☐ INCOMPLETE (detail what's missing below)                               │
│    Missing: ________________________________________________                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6.2 Process Narrative Template

**Required for every significant change.** This creates institutional memory.

```markdown
# Resolution Narrative

## Metadata
- Issue ID: ________________
- Date: ________________
- Time spent: ________________
- Resolved by: ________________

## Problem Statement

### What was wrong
[Describe the exact failure - error message, unexpected behavior, test failure]

### Why it matters
[Business/technical impact - "ct×ct doesn't work means no FHE circuits"]

### First observed
[When/where this was discovered]

### Expected behavior
[What should have happened instead]

---

## Investigation Process

### Step 1: Initial Assessment

**What I observed:**
```
[Exact error message or output]
```

**What I expected:**
```
[Expected correct behavior]
```

**Initial gap identified:**
[First theory about what's wrong]

---

### Step 2: Hypothesis Generation

| # | Hypothesis | Rationale | Probability |
|---|------------|-----------|-------------|
| A | | | High/Med/Low |
| B | | | High/Med/Low |
| C | | | High/Med/Low |

**Most likely cause:** Hypothesis ___

**Why:** ________________

---

### Step 3: Testing Hypotheses

#### Hypothesis A: [Description]

**Test performed:**
```
[Code or command executed]
```

**Result:**
```
[Actual output]
```

**Conclusion:** ☐ Confirmed  ☐ Disproven  ☐ Inconclusive

**Reasoning:** ________________

---

#### Hypothesis B: [Description]

**Test performed:**
```
[Code or command executed]
```

**Result:**
```
[Actual output]
```

**Conclusion:** ☐ Confirmed  ☐ Disproven  ☐ Inconclusive

**Reasoning:** ________________

---

#### Hypothesis C: [Description]

**Test performed:**
```
[Code or command executed]
```

**Result:**
```
[Actual output]
```

**Conclusion:** ☐ Confirmed  ☐ Disproven  ☐ Inconclusive

**Reasoning:** ________________

---

### Step 4: Innovation Application

**Did I check for QMNF innovation applicability?** ☐ YES  ☐ NO

**Innovations considered:**

| Innovation | Could Apply? | Why/Why Not |
|------------|--------------|-------------|
| K-Elimination | | |
| Persistent Montgomery | | |
| Shadow Entropy | | |
| NTT Gen3 | | |
| CRTBigInt | | |
| Integer Noise | | |

**Innovation applied:** ________________

**Why this innovation:** ________________

**Expected improvement:** ________________

---

### Step 5: Implementation

**Files changed:**
| File | Lines Added | Lines Modified | Lines Removed |
|------|-------------|----------------|---------------|
| | | | |

**Key code change:**
```diff
- [old code]
+ [new code]
```

**Explanation of change:**
[Why this specific change fixes the issue]

---

### Step 6: Verification

**Test command:**
```bash
cargo test [specific_test] --release
```

**Result:**
```
[Test output]
```

**Status:** ☐ PASS  ☐ FAIL

**If FAIL, back to Step 2 with new information**

---

## Attribution Table

| Factor | Contribution |
|--------|--------------|
| **Root cause** | [What actually caused the issue] |
| **Solution type** | ☐ Bug fix  ☐ Missing feature  ☐ Wiring  ☐ Design flaw |
| **Key insight** | [The "aha" moment that led to solution] |
| **Innovation used** | [QMNF innovation if applicable] |
| **Innovation impact** | [Measured improvement from innovation] |
| **Verification method** | [How we confirmed the fix works] |
| **Time to diagnose** | [Hours spent finding the issue] |
| **Time to fix** | [Hours spent implementing fix] |

---

## Lessons Learned

### What went well
1. ________________
2. ________________

### What could be improved
1. ________________
2. ________________

### Key takeaways
1. ________________
2. ________________

---

## Future Prevention

**How to catch this earlier:**
________________

**Test to add:**
```rust
#[test]
fn test_prevent_recurrence_[issue_id]() {
    // Test that catches this issue
}
```

**Documentation to update:**
________________

**Checklist item to add:**
________________

---

## Related Issues

| Related Issue | Relationship |
|---------------|--------------|
| | Caused by / Causes / Similar to |

---

## Sign-off

- [ ] Resolution verified complete (Section 6.1)
- [ ] All code changes committed
- [ ] Tests added and passing
- [ ] Documentation updated
- [ ] Session report includes this narrative
```

---

## 6.3 Session Completion Report Template

**Required at end of EVERY session**

```markdown
# Session Completion Report

## Session Metadata
- **Session ID:** YYYY-MM-DD-HH-MM
- **Start time:** ________________
- **End time:** ________________
- **Duration:** ________________
- **Focus area:** ________________

---

## Executive Summary

[One paragraph: What was the goal? What was accomplished? What remains?]

---

## Tasks Completed

| # | Task | Files Changed | Tests Added | Innovation Used | Narrative Link |
|---|------|---------------|-------------|-----------------|----------------|
| 1 | | | | | [Link to narrative] |
| 2 | | | | | |

### Completion Verification

For each completed task, confirm:

| Task | Exists | Correct | Integrated | Tested | Documented |
|------|--------|---------|------------|--------|------------|
| 1 | ☐ | ☐ | ☐ | ☐ | ☐ |
| 2 | ☐ | ☐ | ☐ | ☐ | ☐ |

---

## Tasks In Progress

| # | Task | % Complete | Current State | Blocker | Next Step |
|---|------|------------|---------------|---------|-----------|
| 1 | | | | | |

---

## Tasks Deferred

| # | Task | Reason | Estimated Effort | Priority | Notes |
|---|------|--------|------------------|----------|-------|
| 1 | | | | | |

---

## Resolution Narratives

### Narrative 1: [Issue Title]
[Embed or link to full narrative]

### Narrative 2: [Issue Title]
[Embed or link to full narrative]

---

## Trouble Log Summary

| Issue | Hypotheses Tested | Winner | Resolution | Time |
|-------|-------------------|--------|------------|------|
| | A, B, C | B | [brief] | Xh |

---

## Innovation Usage

### Innovations Applied

| Innovation | Where Applied | Measured Impact | Evidence |
|------------|---------------|-----------------|----------|
| | | | [link to benchmark] |

### Innovations Available But Not Applied

| Innovation | Could Apply To | Why Not Used This Session |
|------------|----------------|---------------------------|
| K-Elimination | | |
| Persistent Montgomery | | |
| Shadow Entropy | | |
| NTT Gen3 | | |
| SIMD/AVX-512 | | |
| UNHAL | | |

---

## Verification Status

### Code Quality

| Check | Status | Notes |
|-------|--------|-------|
| Float scan | ☐ CLEAN  ☐ DIRTY | `grep -r "f32\|f64" src/` |
| Tests pass | ☐ ALL  ☐ SOME  ☐ NONE | X/Y passing |
| Warnings | ☐ ZERO  ☐ SOME | N warnings |

### Integration Verification

| Component | Wired Correctly | Test Exists | Test Passes |
|-----------|-----------------|-------------|-------------|
| | ☐ | ☐ | ☐ |

---

## Code Changes Summary

```
Total files changed: X
Lines added: +Y
Lines removed: -Z
Net change: ±N
```

### File-by-File Changes

| File | +Lines | -Lines | Nature of Change |
|------|--------|--------|------------------|
| | | | |

### Key Diffs

```diff
// [File: path/to/file.rs]
- [old code]
+ [new code]
```

**Why this change:** ________________

---

## Benchmark Results

| Operation | Before | After | Δ | Statistically Significant? |
|-----------|--------|-------|---|---------------------------|
| | | | | ☐ YES (p<0.05)  ☐ NO |

---

## Open Questions

1. ________________
   - Context: ________________
   - Attempted: ________________
   - Need: ________________

2. ________________

---

## Recommendations for Next Session

### Start With
________________

### Priority Order
1. ________________
2. ________________
3. ________________

### Avoid
________________

### Key Context to Remember
________________

---

## Raw Data Archive

Location: `./audit/YYYY-MM-DD_HH-MM/`

| File | Contents | Size |
|------|----------|------|
| test_output.log | Full test output | |
| benchmark_raw.csv | Raw timing data | |
| git_diff.patch | All changes this session | |
| float_scan.txt | Float contamination scan | |
| error_logs.txt | Any error output | |

---

## Session Sign-off

- [ ] All completed tasks verified (Section 6.1)
- [ ] All narratives written
- [ ] Trouble log updated
- [ ] Code committed
- [ ] Raw data archived
- [ ] Next session recommendations clear
```

---

## Integration with Gates 1-5

```
SESSION FLOW:

START SESSION
    │
    ▼
┌─────────────────────────────────────────┐
│ Gate 1: Check for existing code         │
│ Gate 2: Debug systematically            │
│ Gate 3: Check for innovation wiring     │
│ Gate 4: Evidence-based assessment       │
└────────────────────┬────────────────────┘
                     │
                     ▼ [Work happens]
                     │
┌────────────────────▼────────────────────┐
│ Gate 5: Prepare session report          │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌────────────────────────────────────────────────────────────┐
│ Gate 6: Resolution Walkthrough                              │
│                                                             │
│   For each "completed" item:                               │
│     ☐ Verify existence                                     │
│     ☐ Verify correctness                                   │
│     ☐ Verify integration                                   │
│     ☐ Verify tests                                         │
│     ☐ Write narrative                                      │
│                                                             │
│   Complete session report                                   │
│   Archive raw data                                         │
└────────────────────┬───────────────────────────────────────┘
                     │
                     ▼
               END SESSION
```

---

## Why This Matters

**Without Gate 6:**
- "It's done" means "I think it's done"
- No record of how problems were solved
- Same issues recur because no learning captured
- Failures are black boxes

**With Gate 6:**
- "It's done" means "verified complete with evidence"
- Full record of investigation process
- Issues solved once, solution documented
- Failures produce maximum learning
