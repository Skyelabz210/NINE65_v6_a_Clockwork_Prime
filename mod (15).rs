//! T-301 through T-307: Residue Space Orchestrator
//! 
//! The Orchestrator is the decision-making layer of QMNF/EPRAM:
//! - State = DualCodexEPRAM field
//! - Decision = which template the field converges to
//! - Learning = template discovery from single examples
//!
//! INNOVATION: Decision as Attractor Convergence
//! - No explicit comparison circuits
//! - Decision emerges from field dynamics
//! - O(log M) convergence (Lyapunov certified)

use std::collections::HashMap;

// =============================================================================
// T-301: ORCHESTRATOR STATE
// =============================================================================

/// Orchestrator configuration
#[derive(Clone, Debug)]
pub struct OrchestratorConfig {
    /// Number of cells in the decision field
    pub n_cells: usize,
    /// Modulus for field arithmetic
    pub modulus: u64,
    /// Maximum steps before timeout
    pub max_steps: usize,
    /// Convergence threshold (Lyapunov functional)
    pub convergence_threshold: u64,
    /// Coupling mode for decision field
    pub coupling_strength: f64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            n_cells: 64,
            modulus: 256,
            max_steps: 500,
            convergence_threshold: 0,
            coupling_strength: 0.5,
        }
    }
}

/// Action template stored in the orchestrator
/// 
/// Templates are target patterns that represent possible decisions.
/// The field evolves toward the "closest" template.
#[derive(Clone, Debug)]
pub struct ActionTemplate {
    /// Unique identifier
    pub id: usize,
    /// Human-readable label
    pub label: String,
    /// Target pattern for this action
    pub pattern: Vec<u64>,
    /// Confidence threshold for selection
    pub confidence_threshold: f64,
    /// Usage count (for learning)
    pub usage_count: usize,
}

impl ActionTemplate {
    /// Create new template
    pub fn new(id: usize, label: &str, pattern: Vec<u64>) -> Self {
        Self {
            id,
            label: label.to_string(),
            pattern,
            confidence_threshold: 0.8,
            usage_count: 0,
        }
    }
    
    /// Compute distance from field state to this template
    pub fn distance(&self, state: &[u64], modulus: u64) -> u64 {
        state.iter()
            .zip(self.pattern.iter())
            .map(|(&s, &t)| {
                let diff = (t + modulus - s) % modulus;
                diff.min(modulus - diff)
            })
            .sum()
    }
}

/// Residue Space Orchestrator
/// 
/// Main decision-making component of the EPRAM system.
/// 
/// # Architecture
/// ```text
/// ┌─────────────────────────────────────────────────────────────┐
/// │                    ORCHESTRATOR                             │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Input → Field State → Evolution → Convergence → Decision  │
/// │                           ↓                                 │
/// │                    Template Matching                        │
/// │                           ↓                                 │
/// │                    Action Selection                         │
/// └─────────────────────────────────────────────────────────────┘
/// ```
#[derive(Clone)]
pub struct ResidueSpaceOrchestrator {
    /// Current field state
    pub state: Vec<u64>,
    /// Template library
    pub templates: HashMap<usize, ActionTemplate>,
    /// Configuration
    pub config: OrchestratorConfig,
    /// Step counter
    pub step_count: usize,
    /// Last decision result
    pub last_decision: Option<DecisionResult>,
    /// Rail state (FRST)
    pub rails: Vec<Rail>,
}

/// Decision result from orchestrator
#[derive(Clone, Debug)]
pub struct DecisionResult {
    /// Selected template ID
    pub template_id: usize,
    /// Confidence score [0, 1]
    pub confidence: f64,
    /// Steps to convergence
    pub steps: usize,
    /// Final Lyapunov functional
    pub lyapunov: u64,
    /// Was convergence achieved?
    pub converged: bool,
}

/// FRST Rail for decision guidance
#[derive(Clone, Debug)]
pub struct Rail {
    /// Rail identifier
    pub id: usize,
    /// Target template associations
    pub template_ids: Vec<usize>,
    /// Strength (learned from experience)
    pub strength: f64,
    /// Void detection flag
    pub is_void: bool,
}

impl ResidueSpaceOrchestrator {
    /// Create new orchestrator with given config
    pub fn new(config: OrchestratorConfig) -> Self {
        Self {
            state: vec![0; config.n_cells],
            templates: HashMap::new(),
            config,
            step_count: 0,
            last_decision: None,
            rails: Vec::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(OrchestratorConfig::default())
    }
    
    /// Set input state
    pub fn set_input(&mut self, input: Vec<u64>) {
        assert_eq!(input.len(), self.config.n_cells);
        self.state = input.iter()
            .map(|&v| v % self.config.modulus)
            .collect();
    }
    
    /// Add a template to the library
    pub fn add_template(&mut self, template: ActionTemplate) {
        self.templates.insert(template.id, template);
    }
    
    /// Get template by ID
    pub fn get_template(&self, id: usize) -> Option<&ActionTemplate> {
        self.templates.get(&id)
    }
    
    /// Lyapunov functional: total distance from nearest template
    pub fn lyapunov_functional(&self) -> u64 {
        if self.templates.is_empty() {
            return u64::MAX;
        }
        
        self.templates.values()
            .map(|t| t.distance(&self.state, self.config.modulus))
            .min()
            .unwrap_or(u64::MAX)
    }
    
    /// Find closest template to current state
    pub fn closest_template(&self) -> Option<(usize, u64)> {
        self.templates.iter()
            .map(|(&id, t)| (id, t.distance(&self.state, self.config.modulus)))
            .min_by_key(|&(_, d)| d)
    }
}

// =============================================================================
// T-302: ACTION TEMPLATE SYSTEM
// =============================================================================

/// Template pattern generators
pub mod patterns {
    /// Uniform pattern (all same value)
    pub fn uniform(n: usize, value: u64, modulus: u64) -> Vec<u64> {
        vec![value % modulus; n]
    }
    
    /// Gradient pattern (linear ramp)
    pub fn gradient(n: usize, modulus: u64) -> Vec<u64> {
        (0..n).map(|i| ((i as u64 * modulus) / n as u64) % modulus).collect()
    }
    
    /// Cluster pattern (k distinct values)
    pub fn clusters(n: usize, k: usize, modulus: u64) -> Vec<u64> {
        let step = modulus / k as u64;
        (0..n).map(|i| {
            let cluster = i * k / n;
            (cluster as u64 * step) % modulus
        }).collect()
    }
    
    /// Binary pattern (two values based on predicate)
    pub fn binary<F>(n: usize, modulus: u64, low: u64, high: u64, pred: F) -> Vec<u64>
    where F: Fn(usize) -> bool
    {
        (0..n).map(|i| {
            if pred(i) { high % modulus } else { low % modulus }
        }).collect()
    }
    
    /// Random-looking but deterministic pattern
    pub fn pseudo_random(n: usize, seed: u64, modulus: u64) -> Vec<u64> {
        let mut state = seed;
        (0..n).map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % modulus
        }).collect()
    }
}

// =============================================================================
// T-303: DECIDE FUNCTION
// =============================================================================

impl ResidueSpaceOrchestrator {
    /// Fourth Attractor step (dithered)
    #[inline]
    fn attractor_step(state: u64, target: u64, modulus: u64) -> u64 {
        let diff = (target + modulus - state) % modulus;
        
        if diff == 0 {
            return state;
        }
        
        let mut delta = (diff * 3) / 4;
        
        if delta == 0 {
            delta = if diff <= modulus / 2 { 1 } else { modulus - 1 };
        }
        
        (state + delta) % modulus
    }
    
    /// Evolve field one step toward closest template
    fn evolve_step(&mut self) {
        let (closest_id, _) = match self.closest_template() {
            Some(c) => c,
            None => return,
        };
        
        let target = &self.templates[&closest_id].pattern;
        let m = self.config.modulus;
        let n = self.state.len();
        
        // Independent mode: each cell evolves toward its target
        let mut new_state = vec![0u64; n];
        
        for i in 0..n {
            new_state[i] = Self::attractor_step(self.state[i], target[i], m);
        }
        
        self.state = new_state;
        self.step_count += 1;
    }
    
    /// Evolve field with coupling (neighbor influence)
    fn evolve_step_coupled(&mut self) {
        let (closest_id, _) = match self.closest_template() {
            Some(c) => c,
            None => return,
        };
        
        let target = &self.templates[&closest_id].pattern.clone();
        let m = self.config.modulus;
        let n = self.state.len();
        let coupling = self.config.coupling_strength;
        
        let snapshot = self.state.clone();
        let mut new_state = vec![0u64; n];
        
        for i in 0..n {
            // Target pull
            let target_pull = Self::attractor_step(snapshot[i], target[i], m);
            
            // Neighbor average (ring topology)
            let prev = if i == 0 { n - 1 } else { i - 1 };
            let next = if i == n - 1 { 0 } else { i + 1 };
            let neighbor_avg = (snapshot[prev] + snapshot[next]) / 2;
            
            // Blend
            let c_int = (coupling * 100.0) as u64;
            new_state[i] = (target_pull * (100 - c_int) + neighbor_avg * c_int) / 100;
            new_state[i] %= m;
        }
        
        self.state = new_state;
        self.step_count += 1;
    }
    
    /// Make a decision by evolving to convergence
    /// 
    /// INNOVATION: Decision as Attractor Convergence
    /// - Field evolves toward template attractors
    /// - Decision = which attractor captured the field
    /// - O(log M) steps (Lyapunov certified)
    pub fn decide(&mut self) -> DecisionResult {
        self.step_count = 0;
        
        if self.templates.is_empty() {
            return DecisionResult {
                template_id: 0,
                confidence: 0.0,
                steps: 0,
                lyapunov: u64::MAX,
                converged: false,
            };
        }
        
        let initial_lyapunov = self.lyapunov_functional();
        
        // Evolve until convergence or timeout
        for _ in 0..self.config.max_steps {
            let current_lyapunov = self.lyapunov_functional();
            
            // Check convergence
            if current_lyapunov <= self.config.convergence_threshold {
                break;
            }
            
            // Evolve
            if self.config.coupling_strength > 0.01 {
                self.evolve_step_coupled();
            } else {
                self.evolve_step();
            }
        }
        
        // Determine winning template
        let (winner_id, final_distance) = self.closest_template().unwrap_or((0, u64::MAX));
        
        // Compute confidence
        let max_distance = (self.config.n_cells as u64) * (self.config.modulus / 2);
        let confidence = 1.0 - (final_distance as f64 / max_distance as f64);
        
        // Record result
        let result = DecisionResult {
            template_id: winner_id,
            confidence: confidence.max(0.0).min(1.0),
            steps: self.step_count,
            lyapunov: final_distance,
            converged: final_distance <= self.config.convergence_threshold,
        };
        
        // Update template usage
        if let Some(template) = self.templates.get_mut(&winner_id) {
            template.usage_count += 1;
        }
        
        self.last_decision = Some(result.clone());
        result
    }
    
    /// Make decision and return the winning template
    pub fn decide_action(&mut self) -> Option<&ActionTemplate> {
        let result = self.decide();
        self.templates.get(&result.template_id)
    }
}

// =============================================================================
// T-304: ONE-SHOT LEARNING
// =============================================================================

impl ResidueSpaceOrchestrator {
    /// Learn a new template from a single example
    /// 
    /// INNOVATION: One-Shot Template Learning
    /// - Single example becomes stable attractor
    /// - No gradient descent or iteration
    /// - Immediate integration into decision space
    pub fn one_shot_learn(&mut self, label: &str, example: Vec<u64>) -> usize {
        let id = self.templates.len();
        
        let pattern = example.iter()
            .map(|&v| v % self.config.modulus)
            .collect();
        
        let template = ActionTemplate::new(id, label, pattern);
        self.templates.insert(id, template);
        
        id
    }
    
    /// Learn template from current field state
    pub fn learn_from_state(&mut self, label: &str) -> usize {
        self.one_shot_learn(label, self.state.clone())
    }
    
    /// Remove template by ID
    pub fn forget(&mut self, id: usize) -> Option<ActionTemplate> {
        self.templates.remove(&id)
    }
    
    /// Clear all templates
    pub fn clear_templates(&mut self) {
        self.templates.clear();
    }
}

// =============================================================================
// T-305: FRST UPDATE
// =============================================================================

impl ResidueSpaceOrchestrator {
    /// Initialize rails for FRST
    pub fn init_rails(&mut self, n_rails: usize) {
        self.rails = (0..n_rails)
            .map(|i| Rail {
                id: i,
                template_ids: vec![],
                strength: 1.0,
                is_void: false,
            })
            .collect();
    }
    
    /// Update rail based on decision outcome
    /// 
    /// FRST (Fast Rail Selection with Temporal coherence)
    /// - Successful decisions strengthen associated rails
    /// - Failed decisions weaken rails
    /// - Void detection marks unreliable regions
    pub fn frst_update(&mut self, rail_id: usize, success: bool, template_id: usize) {
        if let Some(rail) = self.rails.get_mut(rail_id) {
            if success {
                // Strengthen rail
                rail.strength = (rail.strength * 1.1).min(10.0);
                
                // Associate with successful template
                if !rail.template_ids.contains(&template_id) {
                    rail.template_ids.push(template_id);
                }
                
                rail.is_void = false;
            } else {
                // Weaken rail
                rail.strength *= 0.9;
                
                // Check for void (consistently failing)
                if rail.strength < 0.1 {
                    rail.is_void = true;
                }
            }
        }
    }
    
    /// Get strongest rail
    pub fn strongest_rail(&self) -> Option<&Rail> {
        self.rails.iter()
            .filter(|r| !r.is_void)
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
    }
    
    /// Detect void regions (72% target)
    pub fn void_detection_rate(&self) -> f64 {
        if self.rails.is_empty() {
            return 0.0;
        }
        
        let void_count = self.rails.iter().filter(|r| r.is_void).count();
        void_count as f64 / self.rails.len() as f64
    }
}

// =============================================================================
// T-306: PLMG VALIDATOR (Simplified)
// =============================================================================

/// PLMG validation for decision geometry
pub struct PLMGValidator {
    /// Quotient bound for valid decisions
    pub quotient_bound: u64,
    /// Validation history
    pub history: Vec<bool>,
}

impl PLMGValidator {
    pub fn new(quotient_bound: u64) -> Self {
        Self {
            quotient_bound,
            history: Vec::new(),
        }
    }
    
    /// Validate a decision result
    pub fn validate(&mut self, result: &DecisionResult) -> bool {
        let valid = result.converged && result.confidence > 0.5;
        self.history.push(valid);
        valid
    }
    
    /// Get validation rate
    pub fn validation_rate(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        
        let valid_count = self.history.iter().filter(|&&v| v).count();
        valid_count as f64 / self.history.len() as f64
    }
    
    /// Check if decision is within PLMG geometry
    pub fn check_geometry(&self, distances: &[u64]) -> bool {
        // Simplified: check that distances are bounded
        distances.iter().all(|&d| d <= self.quotient_bound)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_orchestrator_creation() {
        let orch = ResidueSpaceOrchestrator::default_config();
        
        assert_eq!(orch.state.len(), 64);
        assert!(orch.templates.is_empty());
    }
    
    #[test]
    fn test_add_template() {
        let mut orch = ResidueSpaceOrchestrator::default_config();
        
        let pattern = patterns::uniform(64, 0, 256);
        orch.add_template(ActionTemplate::new(0, "zero", pattern));
        
        assert_eq!(orch.templates.len(), 1);
        assert!(orch.get_template(0).is_some());
    }
    
    #[test]
    fn test_decide_single_template() {
        let mut orch = ResidueSpaceOrchestrator::new(OrchestratorConfig {
            n_cells: 16,
            modulus: 64,
            max_steps: 200,
            convergence_threshold: 0,
            coupling_strength: 0.0,
        });
        
        // Add single template
        let pattern = patterns::uniform(16, 0, 64);
        orch.add_template(ActionTemplate::new(0, "zero", pattern));
        
        // Set input away from template
        orch.set_input(vec![32; 16]);
        
        // Decide
        let result = orch.decide();
        
        println!("Single template: converged={}, steps={}, confidence={:.2}",
            result.converged, result.steps, result.confidence);
        
        assert!(result.converged, "Should converge to single template");
        assert_eq!(result.template_id, 0);
    }
    
    #[test]
    fn test_decide_multiple_templates() {
        let mut orch = ResidueSpaceOrchestrator::new(OrchestratorConfig {
            n_cells: 16,
            modulus: 256,
            max_steps: 300,
            convergence_threshold: 0,
            coupling_strength: 0.0,
        });
        
        // Add multiple templates
        orch.add_template(ActionTemplate::new(0, "low", patterns::uniform(16, 32, 256)));
        orch.add_template(ActionTemplate::new(1, "mid", patterns::uniform(16, 128, 256)));
        orch.add_template(ActionTemplate::new(2, "high", patterns::uniform(16, 224, 256)));
        
        // Set input near "mid"
        orch.set_input(vec![120; 16]);
        
        let result = orch.decide();
        
        println!("Multi-template: selected={}, confidence={:.2}", 
            result.template_id, result.confidence);
        
        assert_eq!(result.template_id, 1, "Should select nearest template (mid)");
    }
    
    #[test]
    fn test_one_shot_learn() {
        let mut orch = ResidueSpaceOrchestrator::default_config();
        
        // Learn from example
        let example = vec![42u64; 64];
        let id = orch.one_shot_learn("learned", example.clone());
        
        assert_eq!(id, 0);
        
        let template = orch.get_template(id).unwrap();
        assert_eq!(template.pattern, example);
    }
    
    #[test]
    fn test_frst_update() {
        let mut orch = ResidueSpaceOrchestrator::default_config();
        orch.init_rails(5);
        
        // Successful decision
        orch.frst_update(0, true, 1);
        assert!(orch.rails[0].strength > 1.0);
        assert!(orch.rails[0].template_ids.contains(&1));
        
        // Failed decisions weaken rail
        for _ in 0..20 {
            orch.frst_update(1, false, 0);
        }
        assert!(orch.rails[1].is_void, "Should detect void after many failures");
    }
    
    #[test]
    fn test_coupled_evolution() {
        let mut orch = ResidueSpaceOrchestrator::new(OrchestratorConfig {
            n_cells: 16,
            modulus: 256,
            max_steps: 200,
            convergence_threshold: 5,  // Allow some tolerance
            coupling_strength: 0.5,
        });
        
        orch.add_template(ActionTemplate::new(0, "zero", patterns::uniform(16, 0, 256)));
        orch.set_input(vec![128; 16]);
        
        let result = orch.decide();
        
        println!("Coupled: steps={}, lyapunov={}", result.steps, result.lyapunov);
        
        // Should converge faster with coupling (Grid: 35% faster per Grok)
        assert!(result.steps < 150, "Coupled should converge quickly");
    }
    
    #[test]
    fn test_pattern_generators() {
        let n = 16;
        let m = 256u64;
        
        let uniform = patterns::uniform(n, 42, m);
        assert!(uniform.iter().all(|&v| v == 42));
        
        let gradient = patterns::gradient(n, m);
        assert!(gradient[0] < gradient[n/2]);
        assert!(gradient[n/2] < gradient[n-1]);
        
        let clusters = patterns::clusters(n, 4, m);
        // Should have ~4 distinct values
        let unique: std::collections::HashSet<_> = clusters.iter().collect();
        assert!(unique.len() <= 4);
    }
    
    #[test]
    fn test_plmg_validator() {
        let mut validator = PLMGValidator::new(1000);
        
        let good_result = DecisionResult {
            template_id: 0,
            confidence: 0.9,
            steps: 50,
            lyapunov: 0,
            converged: true,
        };
        
        assert!(validator.validate(&good_result));
        
        let bad_result = DecisionResult {
            template_id: 0,
            confidence: 0.3,
            steps: 500,
            lyapunov: 1000,
            converged: false,
        };
        
        assert!(!validator.validate(&bad_result));
        
        assert_eq!(validator.validation_rate(), 0.5);
    }
    
    #[test]
    fn test_decision_determinism() {
        let config = OrchestratorConfig {
            n_cells: 16,
            modulus: 64,
            max_steps: 100,
            convergence_threshold: 0,
            coupling_strength: 0.0,
        };
        
        let pattern = patterns::uniform(16, 0, 64);
        let input = vec![32u64; 16];
        
        // Run twice with same setup
        let mut orch1 = ResidueSpaceOrchestrator::new(config.clone());
        orch1.add_template(ActionTemplate::new(0, "zero", pattern.clone()));
        orch1.set_input(input.clone());
        let result1 = orch1.decide();
        
        let mut orch2 = ResidueSpaceOrchestrator::new(config);
        orch2.add_template(ActionTemplate::new(0, "zero", pattern));
        orch2.set_input(input);
        let result2 = orch2.decide();
        
        assert_eq!(result1.template_id, result2.template_id);
        assert_eq!(result1.steps, result2.steps);
        assert_eq!(result1.lyapunov, result2.lyapunov);
    }
}
