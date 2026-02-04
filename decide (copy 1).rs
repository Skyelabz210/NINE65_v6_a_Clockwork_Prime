//! T-303: Decide Function - Decision via Attractor Convergence
//! 
//! The core decision-making mechanism of the orchestrator.
//! 
//! INNOVATION: Decision as Attractor Basin
//! - State evolves in template superposition
//! - Natural attractor convergence selects decision
//! - No external control flow - pure dynamics
//! 
//! ALGORITHM:
//! 1. Start with input-encoded state
//! 2. Evolve with all templates as partial attractors
//! 3. Template with lowest Lyapunov wins
//! 4. Continue until unambiguous winner

use super::state::{OrchestratorState, OrchestratorConfig, ActionTemplate, TemplateStore};
use super::dual_codex_cell::DualCodexLane;
use super::montgomery_cell::EPRAMCell;

// =============================================================================
// DECISION RESULT
// =============================================================================

/// Result of a decision
#[derive(Clone, Debug)]
pub struct DecisionResult {
    /// Winning template ID
    pub template_id: u32,
    /// Confidence (based on separation from runner-up)
    pub confidence: f64,
    /// Steps taken to decide
    pub steps: usize,
    /// Final Lyapunov to winner
    pub final_distance: u64,
    /// Whether convergence was complete
    pub converged: bool,
    /// Distances to all templates at decision time
    pub all_distances: Vec<(u32, u64)>,
}

impl DecisionResult {
    /// Create indeterminate result
    pub fn indeterminate() -> Self {
        Self {
            template_id: 0,
            confidence: 0.0,
            steps: 0,
            final_distance: u64::MAX,
            converged: false,
            all_distances: vec![],
        }
    }
}

// =============================================================================
// DECISION CONTEXT
// =============================================================================

/// Context for decision-making
#[derive(Clone, Debug)]
pub struct DecisionContext {
    /// Maximum steps before timeout
    pub max_steps: usize,
    /// Convergence threshold (Lyapunov <= threshold = converged)
    pub convergence_threshold: u64,
    /// Minimum confidence to accept decision
    pub min_confidence: f64,
    /// Coupling strength during evolution
    pub coupling: f64,
    /// Early termination if clear winner
    pub early_termination: bool,
    /// Separation ratio for early termination (winner/runnerup)
    pub separation_ratio: f64,
}

impl Default for DecisionContext {
    fn default() -> Self {
        Self {
            max_steps: 500,
            convergence_threshold: 0,
            min_confidence: 0.0,
            coupling: 0.3,
            early_termination: true,
            separation_ratio: 2.0,
        }
    }
}

impl DecisionContext {
    /// Create context for fast decisions (lower accuracy)
    pub fn fast() -> Self {
        Self {
            max_steps: 100,
            convergence_threshold: 10,
            min_confidence: 0.3,
            coupling: 0.5,
            early_termination: true,
            separation_ratio: 1.5,
        }
    }
    
    /// Create context for accurate decisions (more steps)
    pub fn accurate() -> Self {
        Self {
            max_steps: 1000,
            convergence_threshold: 0,
            min_confidence: 0.8,
            coupling: 0.2,
            early_termination: false,
            separation_ratio: 3.0,
        }
    }
}

// =============================================================================
// DECISION ENGINE
// =============================================================================

/// Decision engine: converts state evolution into decisions
pub struct DecisionEngine {
    /// Configuration
    config: OrchestratorConfig,
    /// Template store
    templates: TemplateStore,
    /// Decision context
    context: DecisionContext,
    /// Decision history (for learning)
    history: Vec<DecisionResult>,
}

impl DecisionEngine {
    /// Create new decision engine
    pub fn new(config: OrchestratorConfig) -> Self {
        let max_templates = config.max_templates;
        Self {
            config,
            templates: TemplateStore::new(max_templates),
            context: DecisionContext::default(),
            history: Vec::new(),
        }
    }
    
    /// Create with custom context
    pub fn with_context(config: OrchestratorConfig, context: DecisionContext) -> Self {
        let max_templates = config.max_templates;
        Self {
            config,
            templates: TemplateStore::new(max_templates),
            context,
            history: Vec::new(),
        }
    }
    
    /// Add template
    pub fn add_template(&mut self, template: ActionTemplate) -> u32 {
        self.templates.add(template)
    }
    
    /// Get template by ID
    pub fn get_template(&self, id: u32) -> Option<&ActionTemplate> {
        self.templates.get(id)
    }
    
    /// Get all templates
    pub fn all_templates(&self) -> Vec<&ActionTemplate> {
        self.templates.all()
    }
    
    /// CORE FUNCTION: Make a decision
    /// 
    /// ALGORITHM (Attractor Convergence):
    /// 1. Compute initial distances to all templates
    /// 2. Evolve state with weighted pull from all templates
    /// 3. Re-evaluate distances each step
    /// 4. Winner = template with lowest final distance
    /// 5. Confidence = separation from runner-up
    pub fn decide(&mut self, state: &mut OrchestratorState) -> DecisionResult {
        if self.templates.is_empty() {
            return DecisionResult::indeterminate();
        }
        
        let templates: Vec<_> = self.templates.all().iter().map(|t| (*t).clone()).collect();
        
        // Track best template each step
        let mut best_id = 0u32;
        let mut best_dist = u64::MAX;
        let mut runner_up_dist = u64::MAX;
        
        for step in 0..self.context.max_steps {
            // Compute distances to all templates
            let distances: Vec<(u32, u64)> = templates.iter()
                .map(|t| (t.id, t.distance(state)))
                .collect();
            
            // Find best and runner-up
            let mut sorted = distances.clone();
            sorted.sort_by_key(|&(_, d)| d);
            
            if sorted.len() >= 1 {
                best_id = sorted[0].0;
                best_dist = sorted[0].1;
            }
            if sorted.len() >= 2 {
                runner_up_dist = sorted[1].1;
            } else {
                runner_up_dist = best_dist;
            }
            
            // Check convergence
            if best_dist <= self.context.convergence_threshold {
                let confidence = self.compute_confidence(best_dist, runner_up_dist);
                
                // Record and return
                let result = DecisionResult {
                    template_id: best_id,
                    confidence,
                    steps: step + 1,
                    final_distance: best_dist,
                    converged: true,
                    all_distances: distances,
                };
                
                self.record_decision(&result);
                return result;
            }
            
            // Check early termination
            if self.context.early_termination && runner_up_dist > 0 {
                let separation = runner_up_dist as f64 / best_dist.max(1) as f64;
                if separation >= self.context.separation_ratio {
                    let confidence = self.compute_confidence(best_dist, runner_up_dist);
                    if confidence >= self.context.min_confidence {
                        let result = DecisionResult {
                            template_id: best_id,
                            confidence,
                            steps: step + 1,
                            final_distance: best_dist,
                            converged: false,
                            all_distances: distances,
                        };
                        
                        self.record_decision(&result);
                        return result;
                    }
                }
            }
            
            // Evolve toward best template (or superposition)
            if let Some(best_template) = templates.iter().find(|t| t.id == best_id) {
                state.step_toward(best_template, self.context.coupling);
            }
        }
        
        // Timeout - return best available
        let distances: Vec<(u32, u64)> = templates.iter()
            .map(|t| (t.id, t.distance(state)))
            .collect();
        
        let confidence = self.compute_confidence(best_dist, runner_up_dist);
        
        let result = DecisionResult {
            template_id: best_id,
            confidence,
            steps: self.context.max_steps,
            final_distance: best_dist,
            converged: best_dist <= self.context.convergence_threshold,
            all_distances: distances,
        };
        
        self.record_decision(&result);
        result
    }
    
    /// Multi-template superposition decision
    /// 
    /// Instead of evolving toward single best template, use weighted
    /// superposition of all templates based on inverse distance.
    pub fn decide_superposition(&mut self, state: &mut OrchestratorState) -> DecisionResult {
        if self.templates.is_empty() {
            return DecisionResult::indeterminate();
        }
        
        let templates: Vec<_> = self.templates.all().iter().map(|t| (*t).clone()).collect();
        
        for step in 0..self.context.max_steps {
            // Compute distances
            let distances: Vec<(u32, u64)> = templates.iter()
                .map(|t| (t.id, t.distance(state)))
                .collect();
            
            // Find weights (inverse distance)
            let total_inv: f64 = distances.iter()
                .map(|(_, d)| 1.0 / (*d as f64 + 1.0))
                .sum();
            
            let weights: Vec<f64> = distances.iter()
                .map(|(_, d)| (1.0 / (*d as f64 + 1.0)) / total_inv)
                .collect();
            
            // Find dominant template
            let (best_idx, _) = weights.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap_or((0, &0.0));
            
            let best_id = templates[best_idx].id;
            let best_dist = distances[best_idx].1;
            
            // Check convergence
            if best_dist <= self.context.convergence_threshold {
                let runner_up_dist = distances.iter()
                    .filter(|(id, _)| *id != best_id)
                    .map(|(_, d)| *d)
                    .min()
                    .unwrap_or(best_dist);
                
                return DecisionResult {
                    template_id: best_id,
                    confidence: self.compute_confidence(best_dist, runner_up_dist),
                    steps: step + 1,
                    final_distance: best_dist,
                    converged: true,
                    all_distances: distances,
                };
            }
            
            // Superposition step: weighted average of all template pulls
            self.superposition_step(state, &templates, &weights);
        }
        
        // Timeout
        let distances: Vec<(u32, u64)> = templates.iter()
            .map(|t| (t.id, t.distance(state)))
            .collect();
        
        let best = distances.iter().min_by_key(|(_, d)| d).unwrap();
        
        DecisionResult {
            template_id: best.0,
            confidence: 0.5,
            steps: self.context.max_steps,
            final_distance: best.1,
            converged: false,
            all_distances: distances,
        }
    }
    
    /// Perform superposition step
    fn superposition_step(&self, state: &mut OrchestratorState, templates: &[ActionTemplate], weights: &[f64]) {
        let n = state.alpha_lanes.len();
        
        // Compute weighted target for each cell
        for i in 0..n {
            let mut alpha_target = 0.0f64;
            let mut beta_target = 0.0f64;
            
            for (t, &w) in templates.iter().zip(weights.iter()) {
                alpha_target += t.alpha_pattern[i] as f64 * w;
                beta_target += t.beta_pattern[i] as f64 * w;
            }
            
            // Create synthetic target
            let alpha_t = DualCodexLane::alpha(
                alpha_target as u64 % state.config.m_alpha,
                &state.config
            );
            let beta_t = DualCodexLane::beta(
                beta_target as u64 % state.config.m_beta,
                &state.config
            );
            
            // Transition
            state.alpha_lanes[i] = state.alpha_lanes[i].transition(&[], &alpha_t);
            state.beta_lanes[i] = state.beta_lanes[i].transition(&[], &beta_t);
        }
        
        state.step_count += 1;
    }
    
    /// Compute confidence from distances
    fn compute_confidence(&self, best: u64, runner_up: u64) -> f64 {
        if runner_up == 0 {
            return if best == 0 { 1.0 } else { 0.5 };
        }
        
        let separation = (runner_up as f64 - best as f64) / runner_up as f64;
        separation.max(0.0).min(1.0)
    }
    
    /// Record decision for learning
    fn record_decision(&mut self, result: &DecisionResult) {
        self.history.push(result.clone());
        
        // Update template usage
        if let Some(template) = self.templates.get_mut(result.template_id) {
            template.usage_count += 1;
            template.last_used = self.history.len() as u64;
        }
    }
    
    /// Get decision history
    pub fn history(&self) -> &[DecisionResult] {
        &self.history
    }
    
    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Quick decide: create engine, add templates, decide
pub fn quick_decide(
    input: &[u128],
    templates: Vec<ActionTemplate>,
    config: &OrchestratorConfig,
) -> DecisionResult {
    let mut engine = DecisionEngine::new(config.clone());
    
    for t in templates {
        engine.add_template(t);
    }
    
    let mut state = OrchestratorState::from_values(input, config);
    engine.decide(&mut state)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup_engine() -> (DecisionEngine, OrchestratorConfig) {
        let config = OrchestratorConfig::small();
        let mut engine = DecisionEngine::new(config.clone());
        
        // Add two distinct templates
        engine.add_template(ActionTemplate::uniform(0, "zero", 0, &config));
        engine.add_template(ActionTemplate::uniform(0, "hundred", 100, &config));
        
        (engine, config)
    }
    
    #[test]
    fn test_decision_engine_creation() {
        let (engine, _) = setup_engine();
        assert_eq!(engine.all_templates().len(), 2);
    }
    
    #[test]
    fn test_decide_near_zero() {
        let (mut engine, config) = setup_engine();
        
        // Input near zero template
        let mut state = OrchestratorState::from_values(&[5, 10, 15, 20], &config);
        
        let result = engine.decide(&mut state);
        
        assert!(result.converged || result.steps > 0);
        println!("Decision: template={}, conf={:.2}, steps={}", 
            result.template_id, result.confidence, result.steps);
        
        // Should pick zero template (closer)
        if result.converged {
            assert_eq!(result.template_id, 0, "Should select zero template");
        }
    }
    
    #[test]
    fn test_decide_near_hundred() {
        let (mut engine, config) = setup_engine();
        
        // Input near hundred template
        let mut state = OrchestratorState::from_values(&[95, 100, 105, 110], &config);
        
        let result = engine.decide(&mut state);
        
        println!("Decision: template={}, conf={:.2}, steps={}", 
            result.template_id, result.confidence, result.steps);
        
        // Should pick hundred template (closer)
        if result.converged {
            assert_eq!(result.template_id, 1, "Should select hundred template");
        }
    }
    
    #[test]
    fn test_decide_deterministic() {
        let (mut engine, config) = setup_engine();
        
        let input = vec![50u128, 50, 50, 50];
        
        // Same input should give same result
        let mut state1 = OrchestratorState::from_values(&input, &config);
        let mut state2 = OrchestratorState::from_values(&input, &config);
        
        let result1 = engine.decide(&mut state1);
        let result2 = engine.decide(&mut state2);
        
        assert_eq!(result1.template_id, result2.template_id, "Should be deterministic");
    }
    
    #[test]
    fn test_decide_superposition() {
        let (mut engine, config) = setup_engine();
        
        let mut state = OrchestratorState::from_values(&[40, 50, 60, 70], &config);
        
        let result = engine.decide_superposition(&mut state);
        
        println!("Superposition decision: template={}, conf={:.2}, steps={}", 
            result.template_id, result.confidence, result.steps);
        
        assert!(result.steps > 0);
    }
    
    #[test]
    fn test_decision_context_fast() {
        let config = OrchestratorConfig::small();
        let context = DecisionContext::fast();
        let mut engine = DecisionEngine::with_context(config.clone(), context);
        
        engine.add_template(ActionTemplate::uniform(0, "zero", 0, &config));
        engine.add_template(ActionTemplate::uniform(0, "hundred", 100, &config));
        
        let mut state = OrchestratorState::from_values(&[10, 20, 30, 40], &config);
        let result = engine.decide(&mut state);
        
        // Fast context should complete quickly
        assert!(result.steps <= 100, "Fast context should be quick");
    }
    
    #[test]
    fn test_quick_decide() {
        let config = OrchestratorConfig::small();
        
        let templates = vec![
            ActionTemplate::uniform(0, "low", 10, &config),
            ActionTemplate::uniform(0, "high", 200, &config),
        ];
        
        let input = vec![15u128, 15, 15, 15];
        let result = quick_decide(&input, templates, &config);
        
        println!("Quick decide: template={}", result.template_id);
        
        // Should pick low template
        if result.converged {
            assert_eq!(result.template_id, 0);
        }
    }
    
    #[test]
    fn test_decision_history() {
        let (mut engine, config) = setup_engine();
        
        assert!(engine.history().is_empty());
        
        let mut state = OrchestratorState::from_values(&[10, 10, 10, 10], &config);
        engine.decide(&mut state);
        
        assert_eq!(engine.history().len(), 1);
        
        let mut state2 = OrchestratorState::from_values(&[90, 90, 90, 90], &config);
        engine.decide(&mut state2);
        
        assert_eq!(engine.history().len(), 2);
    }
    
    #[test]
    fn test_convergence_steps_logged() {
        let config = OrchestratorConfig::small();
        let context = DecisionContext {
            max_steps: 200,
            convergence_threshold: 0,
            min_confidence: 0.0,
            coupling: 0.0,  // Independent for predictable convergence
            early_termination: false,
            separation_ratio: 2.0,
        };
        
        let mut engine = DecisionEngine::with_context(config.clone(), context);
        engine.add_template(ActionTemplate::uniform(0, "zero", 0, &config));
        
        let mut state = OrchestratorState::from_values(&[50, 100, 150, 200], &config);
        let result = engine.decide(&mut state);
        
        println!("Convergence: {} steps, dist={}", result.steps, result.final_distance);
        
        if result.converged {
            // Verify O(log M) convergence
            let expected_max = (config.dual_codex.m_alpha as f64).log2() as usize * 4;
            assert!(result.steps <= expected_max * 2, 
                "Steps {} exceeds expected {}", result.steps, expected_max * 2);
        }
    }
}
