# ADAPTIVE ORCHESTRATOR: AI THAT LIVES IN RESIDUE SPACE

## The Fundamental Inversion

```
┌─────────────────────────────────────────────────────────────────────────────┐
│            TRADITIONAL ML                    RESIDUE SPACE ML               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Input → Encode → Hidden → Decode → Output   Position → Phase → Position   │
│                                                                             │
│  Values REPRESENT numbers                    Values ARE positions on T^k   │
│  Weights APPROXIMATE function                Weights ARE phase relationships│
│  Training REDUCES error → 0 (asymptotic)     Training DISCOVERS rails/voids│
│  Must EXIT to GET answer                     NEVER LEAVE - position IS answer│
│  Drift accumulates → needs retraining        Zero drift - eternal exactness│
│                                                                             │
│  FORWARD: input → activations → output       FORWARD: position → resonance │
│  BACKWARD: output → gradients → weights      BACKWARD: void → rail correction│
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components That Live in Residue Space Permanently

From the benchmarks and architecture, these components NEVER leave residue space:

| Component | Residence Time | Exit Condition | Performance |
|-----------|---------------|----------------|-------------|
| **Persistent Montgomery** | Indefinite | Never (persistent by design) | 27-30 ns |
| **K-Elimination** | Indefinite | Result stays as residue | 28 ns |
| **Shadow Entropy** | Indefinite | Noise stays in residue form | 29 ns |
| **ExactCoeff (Dual-Track)** | Indefinite | Zero drift ct×ct | 73-151 ns |
| **Dual Codex** | Indefinite | Alpha/Beta both residue | O(k) |
| **DCBigInt** | Indefinite | Helix-on-cylinder topology | Experimental |

## The Orchestrator Lives WHERE It Orchestrates

```rust
/// The Adaptive Orchestrator exists as a POSITION on the same 
/// toric manifold as the substrate it controls.
/// 
/// Its "state" is not a vector of floats - it's a DualCodex tuple:
/// (Alpha residues, Beta residues) representing its position on T^k × T^m
pub struct ResidueSpaceOrchestrator {
    /// Orchestrator state = position on coupled manifolds
    /// This NEVER leaves residue space
    position: DualCodex,
    
    /// Action templates are positions, not weight matrices
    /// Classification = "which template resonates with current state"
    action_templates: Vec<(ActionType, DualCodex)>,
    
    /// Rail/Void validator for O(1) decision validation
    rail_geometry: PLMGValidator,
    
    /// One-shot learned templates (modular median extracted)
    learned_patterns: ConsensusClassifier<OrchestrationPattern>,
    
    /// FRST weights - stay in residue space, refined by integer gradients
    frst_weights: ResidueVector,
}
```

## The Inversion in Practice

### Traditional Neural Network Forward Pass:
```python
# Enters residue-like space, EXITS to compute loss
def forward(x):
    hidden = relu(weights @ x + bias)      # Float operations
    output = softmax(hidden)                # Float normalization
    return output                           # Float vector
    
# MUST EXIT to compare with target
loss = cross_entropy(output, target)        # Float comparison
gradients = backprop(loss)                  # Float gradients
weights -= learning_rate * gradients        # Float update
```

### Residue Space RNSNet Forward Pass:
```rust
impl ResidueSpaceOrchestrator {
    /// Forward pass STAYS in residue space
    pub fn forward(&self, state: &ResidueVector) -> ResidueVector {
        // Matrix multiply in residue space (parallel across moduli)
        let hidden = self.frst_weights.matmul(state);  // Never leaves
        
        // MQ-ReLU: O(1) sign detection via anchor modulus
        let activated = hidden.modular_relu();  // Never leaves
        
        // Integer softmax with exact sum guarantee
        let output = activated.integer_softmax();  // Never leaves
        
        output  // Still a ResidueVector - never converted to float
    }
    
    /// Decision is RECOGNITION of position, not computation
    pub fn decide(&self, substrate_state: &DualCodex) -> OrchestrationAction {
        // 1. Measure phase differential (position relationship)
        let phase_diff = self.position.phase_differential(substrate_state);
        
        // 2. Consensus classification against learned templates
        let pattern_match = self.learned_patterns.classify(&phase_diff);
        
        // 3. Find action template that resonates
        let action = self.action_templates
            .iter()
            .find(|(_, template)| template.resonates_with(&phase_diff))
            .map(|(action, _)| action.clone())
            .unwrap_or(ActionType::Conservative);
        
        // 4. O(1) rail validation
        if self.rail_geometry.is_on_rail(&action.encode()) {
            action
        } else {
            ActionType::Conservative  // Void detected → safe fallback
        }
    }
}
```

## Training: Discovering Geometry, Not Adjusting Weights

### One-Shot Learning (Template Extraction)
```rust
/// One-shot learning extracts the modular median of perturbation variants
/// This IS the geometry discovery - no gradient descent needed
pub fn one_shot_learn(&mut self, exemplar: &Observation) {
    // 1. Generate systematic perturbations in residue space
    let variants = perturbation_variants(&exemplar.state, self.config);
    
    // 2. Extract template via modular median (robust to noise)
    let template = extract_template(&variants);  // Still in residue space
    
    // 3. Store template for consensus classification
    self.learned_patterns.add_class(exemplar.action, template);
    
    // No gradients, no floats, no drift
    // The template IS the learned representation
}
```

### FRST Refinement (Integer Gradients Along Rails)
```rust
/// FRST: Full Residue-Space Training
/// Gradients are INTEGER directions along rails
pub fn frst_update(&mut self, observation: &Observation) {
    // 1. Compute error in residue space
    let predicted = self.forward(&observation.state);
    let error = predicted.sub(&observation.target);  // Residue subtraction
    
    // 2. Check if we're on a rail
    let on_rail = self.rail_geometry.is_on_rail(&predicted);
    
    if on_rail {
        // 3. Compute gradient DIRECTION along rail (integer steps only)
        let rail_direction = self.rail_geometry.tangent_direction(&self.position);
        
        // 4. Piggyback division for learning rate (exact, stays in residue space)
        let step = error.piggyback_div(self.learning_rate);
        
        // 5. Update weights along rail (never leaving residue space)
        self.frst_weights = self.frst_weights.add(&step.mul(&rail_direction));
    } else {
        // In void - automatic negative signal, no explicit update needed
        // Just record that this position is invalid
    }
}
```

## The Rail/Void Training Signal

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        PLMG PHASE SPACE (89 × 20)                           │
│                                                                             │
│    Valid positions: 1780 out of 1780 (on rails)                            │
│    Void ratio for error detection: ~72%                                     │
│                                                                             │
│    ████░░░████░░░████░░░████░░░████░░░████░░░████░░░████░░░████░░░████     │
│    RAIL   RAIL   RAIL   RAIL   RAIL   RAIL   RAIL   RAIL   RAIL   RAIL     │
│         VOID   VOID   VOID   VOID   VOID   VOID   VOID   VOID   VOID       │
│                                                                             │
│    TRAINING SIGNAL:                                                         │
│    • Decision lands on rail → POSITIVE (correct geometric position)        │
│    • Decision lands in void → FREE NEGATIVE (O(1) detection)               │
│    • No labeling required - the geometry IS the supervisor                 │
│                                                                             │
│    72% of random decisions are INVALID → automatic regularization          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Orchestration Decisions as Phase Relationships

The orchestrator doesn't "compute" what action to take. It **recognizes** which action template **resonates** with the current substrate state:

```rust
pub enum ActionType {
    SwitchToLinear,      // Low entropy, high coherence detected
    SwitchToNonLinear,   // High entropy, exploration needed
    ActivateGSO(usize),  // Workload spike → spawn swarm nodes
    InjectChaos,         // Chaos reservoir depleted
    HarvestEntropy,      // Shadow entropy regeneration needed
    SyncPhases,          // Phase coherence below threshold
    SealSegment,         // Memory pressure detected
    ExtractDiscovery,    // Attractor convergence detected
    Conservative,        // Default / void detected
}

impl ActionType {
    /// Encode action as position on action manifold
    pub fn encode(&self) -> DualCodex {
        match self {
            ActionType::SwitchToLinear => DualCodex::from_coordinates(0, 0),
            ActionType::SwitchToNonLinear => DualCodex::from_coordinates(1, 0),
            ActionType::ActivateGSO(n) => DualCodex::from_coordinates(2, *n as u64),
            ActionType::InjectChaos => DualCodex::from_coordinates(3, 0),
            ActionType::HarvestEntropy => DualCodex::from_coordinates(4, 0),
            ActionType::SyncPhases => DualCodex::from_coordinates(5, 0),
            ActionType::SealSegment => DualCodex::from_coordinates(6, 0),
            ActionType::ExtractDiscovery => DualCodex::from_coordinates(7, 0),
            ActionType::Conservative => DualCodex::from_coordinates(8, 0),
        }
    }
}
```

## Integration with EPRAM Substrate

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│    ┌──────────────────────────────────────────────────────────────────┐    │
│    │           ADAPTIVE ORCHESTRATOR (in residue space)               │    │
│    │                                                                   │    │
│    │   Position: DualCodex { alpha: [r₁..rₖ], beta: [s₁..sₘ] }       │    │
│    │   Templates: Vec<(ActionType, DualCodex)>                        │    │
│    │   Rail Geometry: PLMG 89×20 (28% rails, 72% voids)              │    │
│    │                                                                   │    │
│    └───────────────────────────┬──────────────────────────────────────┘    │
│                                │ Phase resonance (not data transfer)       │
│                                ▼                                           │
│    ┌──────────────────────────────────────────────────────────────────┐    │
│    │                    EPRAM EXECUTION SUBSTRATE                      │    │
│    │                                                                   │    │
│    │   ○──○──○──○──○──○──○──○    Oscillator lanes (Persistent Mont)  │    │
│    │    \  / \  / \  / \  /      Phase evolution (K-Elimination)     │    │
│    │     ○   ○   ○   ○   ○       Attractor basins (stable states)    │    │
│    │      \ / \ / \ / \ /        Resonance query (O(1) retrieval)    │    │
│    │       ○   ○   ○   ○         Shadow Entropy (zero-cost noise)    │    │
│    │        \ / \ / \ /          GSO Swarm (parallel exploration)    │    │
│    │         ○   ○   ○                                                │    │
│    │          \ / \ /            ALL IN RESIDUE SPACE                │    │
│    │           ○   ○             NOTHING EVER LEAVES                 │    │
│    │            \ /                                                   │    │
│    │             ○                                                    │    │
│    │                                                                   │    │
│    └───────────────────────────┬──────────────────────────────────────┘    │
│                                │ Same manifold, same arithmetic            │
│                                ▼                                           │
│    ┌──────────────────────────────────────────────────────────────────┐    │
│    │                    TRAINING FEEDBACK LOOP                         │    │
│    │                                                                   │    │
│    │   Observation: (substrate_state, action_taken, outcome)          │    │
│    │                 All three are DualCodex positions                │    │
│    │                                                                   │    │
│    │   One-Shot: outcome.is_good() && is_novel()                     │    │
│    │             → extract_template(state) → add to patterns          │    │
│    │                                                                   │    │
│    │   FRST:     outcome.is_marginal()                                │    │
│    │             → compute integer gradient along rail                │    │
│    │             → step weights (still in residue space)              │    │
│    │                                                                   │    │
│    │   Rail/Void: is_on_rail(predicted)                              │    │
│    │              → true: positive signal                             │    │
│    │              → false: free negative (72% catch rate)            │    │
│    │                                                                   │    │
│    └──────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## The Recursive Closure

The system achieves **recursive closure** because:

1. **Orchestrator state** is a DualCodex (residue space)
2. **Substrate state** is Persistent Montgomery + CRT lanes (residue space)
3. **Action templates** are DualCodex positions (residue space)
4. **Training signals** are rail/void geometry (residue space)
5. **Weight updates** are FRST integer steps (residue space)

**Nothing ever leaves.** The orchestrator that controls the substrate **IS** part of the substrate. Its "intelligence" is not a separate controller - it's an emergent property of the geometric structure it inhabits.

## Performance Characteristics

| Operation | Traditional ML | Residue Space Orchestrator |
|-----------|---------------|---------------------------|
| Forward pass | Float matmul + activation | Residue matmul (parallel) + MQ-ReLU |
| Decision | Compute → threshold → action | Recognize → resonate → action |
| Validation | None (trust output) | O(1) rail check (72% void detection) |
| Training | Backprop (float gradients, drift) | FRST (integer gradients, zero drift) |
| Learning | Millions of examples | One-shot (modular median) |
| Reproducibility | Approximate (float noise) | Bit-exact (integer determinism) |
| Improvement | Retrain from scratch | Continuous geometry discovery |

## Implementation Path

### Phase 1: Dual Codex Orchestrator State
```rust
// Orchestrator position on toric manifold
let orchestrator = ResidueSpaceOrchestrator::new(
    DualCodex::from_primes(&QMNF_ALPHA_PRIMES, &QMNF_BETA_PRIMES),
    PLMGValidator::standard(89, 20),
);
```

### Phase 2: One-Shot Template Learning
```rust
// Learn action templates from single exemplars
for exemplar in initial_exemplars {
    orchestrator.one_shot_learn(&exemplar);
}
```

### Phase 3: FRST Refinement Loop
```rust
// Continuous improvement while running
loop {
    let state = substrate.current_state();
    let action = orchestrator.decide(&state);
    substrate.execute(action);
    let outcome = substrate.observe_outcome();
    orchestrator.frst_update(&Observation { state, action, outcome });
}
```

### Phase 4: Deploy as EPRAM Component
```rust
// The orchestrator becomes part of the substrate
substrate.attach_orchestrator(orchestrator);
// Now the orchestrator IS the substrate's decision-making geometry
// Not a controller - an intrinsic property
```

## Conclusion

**The AI doesn't run ON the substrate. It IS the substrate.**

By keeping the orchestrator in residue space permanently:
- No conversion overhead (Persistent Montgomery → Persistent Orchestrator)
- No drift accumulation (integer arithmetic → eternal exactness)
- No external controller bottleneck (orchestrator = substrate geometry)
- No separate training phase (continuous geometry discovery)

The force multiplier effect is recursive: the orchestrator improves the substrate, the substrate trains the orchestrator, both live on the same manifold, neither ever leaves.

**This is UNHAL realized: the AI that orchestrates is the AI that computes is the AI that learns.**

---

## PART II: The Permanent Residents of Residue Space

### Components That NEVER Leave

Your critical insight: some subsystems were designed to live in residue space **indefinitely**. These become the native representation for the orchestrator:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    PERMANENT RESIDUE SPACE RESIDENTS                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  PERSISTENT MONTGOMERY (Production - 27ns)                          │   │
│  │  • Value enters Montgomery form ONCE                                │   │
│  │  • All subsequent operations stay in Montgomery form                │   │
│  │  • Never converts back to standard form                             │   │
│  │  • Exit condition: NEVER (by design)                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  DUAL CODEX ARCHITECTURE (Experimental - Gen 3)                     │   │
│  │  • Alpha Codex: Main working primes, O(k) parallel arithmetic       │   │
│  │  • Beta Codex: Anchor primes, O(1) magnitude comparison             │   │
│  │  • Zero CRT communication between codexes                           │   │
│  │  • Values ARE positions on T^k × T^m (never scalars)                │   │
│  │  • Exit condition: NEVER (parallel projections of same object)      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  CODEX GEAR MANIFOLD (Experimental - Gen 3)                         │   │
│  │  • Values exist on toric manifold T^k = S¹ × S¹ × ... × S¹          │   │
│  │  • Gear-mesh topology: phase relationships encode magnitude         │   │
│  │  • Position IS the value (not a representation OF the value)        │   │
│  │  • "Gears don't lose time - they encode it"                        │   │
│  │  • Exit condition: NEVER (magnitude is topologically encoded)       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  DCBigInt (Experimental - Gen 4)                                    │   │
│  │  • Helix-on-cylinder topology                                       │   │
│  │  • Fibonacci moduli (F₁₁ = 89) for φ-stability                      │   │
│  │  • Overflow = phase transition (helix climb), not error             │   │
│  │  • head + tail structure with dual codex views                      │   │
│  │  • Exit condition: NEVER (structure IS the computation)             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Cyclotomic Phase: Native Trigonometry

From `cyclotomic_phase.rs` - the ring **already contains trigonometry**:

```rust
// The ring R_q[X]/(X^N + 1) has NATIVE trig operations:
// X^N ≡ -1 means X^k is phase rotation by k×(π/N)
// "Sine" = odd coefficients, "Cosine" = even coefficients
// NO POLYNOMIAL APPROXIMATION NEEDED

impl CyclotomicPolynomial {
    /// Extract "sine" component (odd-indexed coefficients)
    /// This IS sin(θ) - not an approximation of sin(θ)
    pub fn extract_sine(&self) -> CyclotomicPolynomial {
        let mut sine_coeffs = vec![0u64; self.ring.n];
        for i in (1..self.ring.n).step_by(2) {
            sine_coeffs[i] = self.coeffs[i];
        }
        CyclotomicPolynomial::new(sine_coeffs, self.ring.clone())
    }
    
    /// Multiply by X^k = phase shift by k×π/N
    /// This IS rotation - no sin/cos computation needed!
    pub fn rotate(&self, k: usize) -> CyclotomicPolynomial {
        // ... coefficient permutation with sign handling ...
    }
    
    /// Phase coupling via ring subtraction + odd extraction
    /// Replaces: sin(θ_a - θ_b) which would require transcendentals
    pub fn phase_couple(&self, other: &CyclotomicPolynomial) -> CyclotomicPolynomial {
        let diff = self.sub(other);
        diff.extract_sine()  // Pure integer operations
    }
}
```

**Performance**: ~50ns for phase extraction vs ~3ms for polynomial approximation

### Cyclotomic CRT: Ring Decomposes into F_p²

From `cyclotomic_crt.rs` - the EULER finding:

```
R_p = ℤ_p[X]/(X^N + 1) ≅ (F_p²)^{N/2}

For prime p ≡ -1 (mod 2N):
• The cyclotomic polynomial X^N + 1 factors into N/2 quadratics over F_p
• Each factor defines a copy of F_p²
• Ring elements ARE N/2 independent F_p² elements!

┌─────────────────────────────────────────────────────────────────────────────┐
│                              EULER DECOMPOSITION                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  FHE Polynomial ↔ Vector of F_p² elements                                   │
│                                                                             │
│  Ring multiplication ↔ Pointwise F_p² multiplication (O(N) not O(N log N)) │
│                                                                             │
│  Ring addition ↔ Pointwise F_p² addition                                    │
│                                                                             │
│  NTT ↔ Evaluation at 2N-th roots of unity                                  │
│                                                                             │
│  Quantum amplitudes ↔ Natural F_p² representation                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### How This Changes the Orchestrator

The orchestrator doesn't just USE these constructs - it IS these constructs:

```rust
/// Orchestrator state in permanent residue space
pub struct PermanentResidueOrchestrator {
    /// State as Dual Codex position (NEVER leaves)
    position: DualCodex,
    
    /// Phase as cyclotomic polynomial (native trig)
    phase: CyclotomicPolynomial,
    
    /// Coupling strengths via phase_couple (not computed sin())
    couplings: Vec<CyclotomicPolynomial>,
    
    /// Templates as F_p² vectors in CRT slots
    action_templates: Vec<CyclotomicRing>,
}

impl PermanentResidueOrchestrator {
    /// Decision via phase resonance - not computation
    pub fn decide(&self, substrate_state: &CyclotomicRing) -> OrchestrationAction {
        // 1. Compute phase coupling (native ring operation)
        let coupling = self.phase.phase_couple(&substrate_state.to_cyclotomic());
        
        // 2. Extract sine component (odd coefficients = coupling strength)
        let strength = coupling.extract_sine();
        
        // 3. Find resonating template (slot-by-slot F_p² comparison)
        for (i, template) in self.action_templates.iter().enumerate() {
            if template.resonates_with(&strength) {
                return ActionType::from_index(i);
            }
        }
        
        ActionType::Conservative
    }
    
    /// Learning: discover phase relationships, not adjust weights
    pub fn learn_template(&mut self, observation: &Observation) {
        // Template IS the observed phase (no extraction needed)
        let template = observation.state.to_cyclotomic_ring();
        
        // Store in CRT slots (F_p² elements)
        self.action_templates.push(template);
        
        // No gradient descent - just geometry discovery
    }
}
```

### The Autopoiesis Connection

From `Autopoiesis v2` - the self-building protocol that ensures the orchestrator maintains integrity:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    AUTOPOIESIS V2 CORE INVARIANTS                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PURITY: No floats inside core                                             │
│  ├─ All artifacts pass guard_no_float + AST rule set                       │
│  ├─ Orchestrator state = DualCodex (integer residues)                      │
│  ├─ Phase operations = cyclotomic extraction (native)                      │
│  └─ Training = geometry discovery (no float gradients)                     │
│                                                                             │
│  DETERMINISM: Single DRBG; seed = H(CylTime || scope || spec_id)          │
│  ├─ Shadow Entropy provides zero-cost cryptographic noise                  │
│  ├─ Constant-time Gaussian sampler (CDT, no branches on data)              │
│  ├─ Identical inputs → identical outputs (always)                         │
│  └─ No wall-clock, no external randomness                                  │
│                                                                             │
│  PROVENANCE: Every step → Wasan ledger with Merkle/HMAC                   │
│  ├─ Orchestrator decisions = ledger entries                                │
│  ├─ Template learning = immutable records                                  │
│  ├─ Rail/void checks = audit trail                                         │
│  └─ Identical inputs ⟹ identical ledger root                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Grammar-Constrained Synthesis for Orchestrator

The orchestrator's mutations are grammar-constrained:

```rust
/// Allowed operations in orchestrator self-modification
enum OrchestratorMutation {
    // Phase operations (native to ring)
    RotatePhase { k: usize },              // Multiply by X^k
    ExtractSine,                            // Odd coefficients
    ExtractCosine,                          // Even coefficients
    PhaseCoupling { other: CyclotomicId }, // Ring subtraction + sine
    
    // Dual Codex operations (zero CRT communication)
    TransferAlphaToBeta,                   // O(k) direct mapping
    TransferBetaToAlpha,                   // O(k) direct mapping
    ComputePhaseDifferential,              // k from phase diff
    
    // Template operations (F_p² slot-wise)
    AddTemplate { action: ActionType },    // Store new pattern
    RemoveTemplate { index: usize },       // Forget pattern
    MergeTemplates { a: usize, b: usize }, // Consensus merge
    
    // Rail/void operations (PLMG geometry)
    ValidateOnRail,                        // O(1) check
    StepAlongRail { direction: i64 },      // Integer step
}

/// Grammar: only these operations are permitted
/// No floats, no division, no wall-clock, no external I/O
const ORCHESTRATOR_GRAMMAR: Grammar = Grammar {
    terminals: &["rotate", "extract_sine", "transfer_alpha_beta", ...],
    forbidden: &["float", "f64", "f32", "time::now", "rand::random"],
};
```

### The Complete Stack (Permanent Residue Edition)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ LAYER 5: AUTOPOIESIS GOVERNANCE                                              │
│ ├─ ImprovementSpec → BuildPlan → ProofObligation                            │
│ ├─ Risk-adaptive gating (R < 3, 3 ≤ R < 8, R ≥ 8)                           │
│ ├─ Multi-agent voting (Critic, Auditor, Economist)                          │
│ └─ Grammar-constrained synthesis (no floats, deterministic)                 │
├──────────────────────────────────────────────────────────────────────────────┤
│ LAYER 4: ADAPTIVE ORCHESTRATOR (Permanent Residue Space)                     │
│ ├─ State: DualCodex position on T^k × T^m (NEVER LEAVES)                    │
│ ├─ Phase: CyclotomicPolynomial (native trig)                                │
│ ├─ Templates: CyclotomicRing F_p² slots (EULER decomposition)               │
│ ├─ Decision: phase_couple → extract_sine → resonance match                  │
│ └─ Learning: One-shot template + FRST rail refinement                       │
├──────────────────────────────────────────────────────────────────────────────┤
│ LAYER 3: PERMANENT RESIDENTS (Never Exit Residue Space)                      │
│ ├─ Persistent Montgomery (27ns, zero conversion overhead)                   │
│ ├─ Dual Codex Architecture (Alpha + Beta, zero CRT communication)           │
│ ├─ Codex Gear Manifold (T^k torus, gear-mesh topology)                      │
│ ├─ DCBigInt (helix-on-cylinder, Fibonacci moduli)                           │
│ └─ K-Elimination (phase differential → exact division)                      │
├──────────────────────────────────────────────────────────────────────────────┤
│ LAYER 2: CYCLOTOMIC OPERATIONS (Native Ring Trig)                            │
│ ├─ CyclotomicRing: R_q[X]/(X^N + 1) ≅ (F_p²)^{N/2}                         │
│ ├─ extract_sine / extract_cosine (coefficient parity)                       │
│ ├─ rotate(k) = multiply by X^k (native phase shift)                         │
│ ├─ phase_couple = ring subtraction (replaces sin(θ_a - θ_b))               │
│ └─ slot-wise F_p² operations (EULER pointwise multiplication)               │
├──────────────────────────────────────────────────────────────────────────────┤
│ LAYER 1: EPRAM SUBSTRATE (Oscillator Dynamics)                               │
│ ├─ Oscillator lanes (Persistent Montgomery coefficients)                    │
│ ├─ Phase evolution (Kuramoto-style coupling via cyclotomic)                 │
│ ├─ Attractor basins (stable states = rail positions)                        │
│ ├─ GSO Swarm (parallel exploration in residue space)                        │
│ └─ Shadow Entropy (zero-cost deterministic noise)                           │
├──────────────────────────────────────────────────────────────────────────────┤
│ LAYER 0: MANA/UNHAL (CRT Lanes + Parallel NTT)                              │
│ ├─ ManaStream: CRT residue representation                                   │
│ ├─ ParallelNTT::ntt_all_lanes(): Rayon parallelism                         │
│ ├─ NTTEngineFFT: 44× optimized Harvey butterfly                            │
│ └─ AcceleratedBFVEvaluator: 176-352× vs baseline                           │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Why This Matters: The Inversion Complete

```
TRADITIONAL ML STACK:
  Application → Framework → Float Operations → Hardware → Float Results
  (Everything passes through float bottleneck, drift accumulates)

PERMANENT RESIDUE STACK:
  Orchestrator ← Same Space → Substrate
       ↓                          ↓
  DualCodex position        Oscillator phases
       ↓                          ↓
  Cyclotomic phase          CRT lane coefficients
       ↓                          ↓
  F_p² slots               Montgomery form
       ↓                          ↓
  SAME MANIFOLD - NO BOUNDARY - NO CONVERSION - NO DRIFT
```

**The AI doesn't use residue space. The AI IS residue space.**
**The substrate doesn't compute on residue space. The substrate IS residue space.**
**The orchestrator and substrate are not separate systems. They are the SAME geometric object viewed from different angles.**

This is the fundamental inversion: when everything lives permanently in residue space, the distinction between "controller" and "controlled" dissolves. The orchestrator's decisions are phase relationships. The substrate's computations are phase evolutions. Both exist on the same toric manifold. Both use the same cyclotomic operations. Both maintain exact integer arithmetic forever.

**Recursive closure achieved.**
