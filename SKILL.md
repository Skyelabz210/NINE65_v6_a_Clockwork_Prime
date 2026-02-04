---
name: innovation-mining
description: "Search conversation history for innovations, research, and solutions to synthesize comprehensive compendiums. Use when: (1) User encounters a hard problem that may have been solved in past research, (2) User wants to catalog innovations across conversation history, (3) User needs to find prior solutions to current challenges, (4) User requests synthesis of past work into documentation, (5) Dynamic Branch problems spawn from validation work. Triggers on phrases like 'search our history', 'what have we done before', 'find solutions from past conversations', 'create a compendium', 'mine our innovations'."
---

# Innovation Mining Skill

Mine conversation history for innovations and synthesize into comprehensive documentation.

## Workflow

### Phase 1: Problem Identification

Identify hard problems or topics requiring mining:

1. **Explicit problems**: User states specific challenges
2. **Dynamic Branch triggers**: Problems spawned from validation (DBS-XXX-NNN)
3. **Research gaps**: Missing solutions during development
4. **Integration needs**: Components requiring prior work

Document each: Problem ID, technical description, why existing approaches fail, what solution enables.

### Phase 2: Systematic Search

Execute targeted `conversation_search` queries (5-10 per problem, `max_results: 10`):

**Search order:**
1. Direct terminology (exact technical terms)
2. Alternative formulations (synonyms, related concepts)
3. Solution patterns (approaches that might apply)
4. Implementation details (code patterns, data structures)
5. Cross-domain (related fields with similar problems)

### Phase 3: Solution Extraction

For each relevant result:

1. **Innovation**: What problem solved, core insight, novelty
2. **Implementation**: Code snippets, formulas, pseudocode
3. **Evidence level**:
   - L1: Formal proof (Lean/Coq)
   - L2: Exhaustive empirical (100K+ tests)
   - L3: Statistical validation
   - L4: Provisional
   - L5: Conjectural
4. **Source**: Conversation date/context

### Phase 4: Synthesis

Group innovations by:
- Problem domain
- Evidence level (proven → conjectural)
- Implementation readiness
- Dependency chain

Document for each: Problem solved, mathematical basis, implementation code, evidence, source, integration notes.

### Phase 5: Document Generation

Create compendium via docx skill:

**Structure:**
1. Executive Summary
2. Table of Contents
3. Parts by problem domain
4. Innovation catalog table
5. Recommended solutions appendix
6. Conclusion

## Search Query Templates

**Arithmetic:**
- `"K-Elimination theorem division exact RNS quotient recovery"`
- `"Montgomery persistence domain conversion optimization"`
- `"CRT anchor moduli coprime reconstruction"`

**Neural Networks:**
- `"integer softmax exponential approximation Taylor Padé CORDIC"`
- `"backpropagation gradient RNS training FRST"`
- `"activation function integer ReLU sigmoid"`

**Signed Arithmetic:**
- `"MobiusInt polarity signed magnitude separation"`
- `"symmetric balanced CRT representation negative"`
- `"sign detection quotient clustered moduli"`

**Geometry/Validation:**
- `"PLMG rail void geometry error detection boolean"`
- `"torus topology wraparound phase space manifold"`
- `"Fibonacci moduli golden ratio phi stability"`

**Cryptography:**
- `"AHOP Apollonian Descartes circle packing"`
- `"FHE homomorphic integer encryption BFV"`

## Dynamic Branch Integration

When DBS-XXX-NNN spawns:

1. Immediately trigger innovation mining
2. Search using problem's technical terms
3. Check for existing solution in history
4. If found: Document and integrate
5. If not found: Flag as research frontier

Prevents re-solving solved problems. Accelerates sprint velocity.
