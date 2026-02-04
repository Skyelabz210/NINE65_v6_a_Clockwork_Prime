//! T-301: OrchestratorState Structure
//! 
//! The Residue Space Orchestrator state using DualCodexEPRAM.
//! 
//! ARCHITECTURE:
//! - State = DualCodexEPRAM field (Alpha-Beta pairs)
//! - Decision = Which template the field converges to
//! - Learning = Template discovery via stable patterns
//!
//! INNOVATION: Orchestrator as EPRAM Controller
//! - Decision-making via attractor convergence
//! - Templates stored in cyclotomic slots
//! - No external control flow - pure field dynamics

use std::collections::HashMap;

// Import permanent residents
use super::dual_codex_cell::{DualCodexConfig, DualCodexEPRAM, DualCodexLane};
use super::cyclotomic_cell::{CyclotomicElement, CyclotomicSlot, CyclotomicPhase};
use super::montgomery_cell::{EPRAMCell, FourthAttractorParams};

// =============================================================================
// ORCHESTRATOR CONFIGURATION
// =============================================================================

/// Configuration for the orchestrator
#[derive(Clone, Debug)]
pub struct OrchestratorConfig {
    /// Number of state cells
    pub n_cells: usize,
    /// Dual Codex configuration for state representation
    pub dual_codex: DualCodexConfig,
    /// Maximum templates to store
    pub max_templates: usize,
    /// Decision convergence threshold (Lyapunov)
    pub convergence_threshold: u64,
    /// Maximum steps for decision
    pub max_decision_steps: usize,
    /// Default coupling mode for decisions
    pub decision_coupling: f64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            n_cells: 64,
            dual_codex: DualCodexConfig::standard(),
            max_templates: 32,
            convergence_threshold: 0,
            max_decision_steps: 500,
            decision_coupling: 0.5,
        }
    }
}

impl OrchestratorConfig {
    /// Create config for small orchestrator (testing)
    pub fn small() -> Self {
        Self {
            n_cells: 16,
            dual_codex: DualCodexConfig::new(256, 251),
            max_templates: 8,
            convergence_threshold: 0,
            max_decision_steps: 200,
            decision_coupling: 0.5,
        }
    }
    
    /// Create config for production orchestrator
    pub fn production() -> Self {
        Self {
            n_cells: 256,
            dual_codex: DualCodexConfig::standard(),
            max_templates: 128,
            convergence_threshold: 0,
            max_decision_steps: 1000,
            decision_coupling: 0.3,
        }
    }
}

// =============================================================================
// ACTION TEMPLATE
// =============================================================================

/// Template representing a possible action/decision
/// 
/// Templates are stored as target patterns for EPRAM field convergence.
/// The orchestrator decides by determining which template the field
/// naturally converges toward.
#[derive(Clone, Debug)]
pub struct ActionTemplate {
    /// Unique identifier
    pub id: u32,
    /// Human-readable label
    pub label: String,
    /// Target pattern for Alpha lanes
    pub alpha_pattern: Vec<u64>,
    /// Target pattern for Beta lanes
    pub beta_pattern: Vec<u64>,
    /// Confidence score (from learning)
    pub confidence: f64,
    /// Usage count
    pub usage_count: u64,
    /// Last used timestamp (step count)
    pub last_used: u64,
}

impl ActionTemplate {
    /// Create uniform template (all cells same value)
    pub fn uniform(id: u32, label: &str, value: u128, config: &OrchestratorConfig) -> Self {
        let n = config.n_cells;
        let alpha_val = (value % config.dual_codex.m_alpha as u128) as u64;
        let beta_val = (value % config.dual_codex.m_beta as u128) as u64;
        
        Self {
            id,
            label: label.to_string(),
            alpha_pattern: vec![alpha_val; n],
            beta_pattern: vec![beta_val; n],
            confidence: 1.0,
            usage_count: 0,
            last_used: 0,
        }
    }
    
    /// Create gradient template
    pub fn gradient(id: u32, label: &str, config: &OrchestratorConfig) -> Self {
        let n = config.n_cells;
        let alpha_pattern: Vec<u64> = (0..n)
            .map(|i| (i as u64 * config.dual_codex.m_alpha / n as u64))
            .collect();
        let beta_pattern: Vec<u64> = (0..n)
            .map(|i| (i as u64 * config.dual_codex.m_beta / n as u64))
            .collect();
        
        Self {
            id,
            label: label.to_string(),
            alpha_pattern,
            beta_pattern,
            confidence: 1.0,
            usage_count: 0,
            last_used: 0,
        }
    }
    
    /// Create cluster template (two groups)
    pub fn cluster(id: u32, label: &str, val1: u128, val2: u128, config: &OrchestratorConfig) -> Self {
        let n = config.n_cells;
        let mid = n / 2;
        
        let alpha_pattern: Vec<u64> = (0..n)
            .map(|i| {
                let v = if i < mid { val1 } else { val2 };
                (v % config.dual_codex.m_alpha as u128) as u64
            })
            .collect();
        
        let beta_pattern: Vec<u64> = (0..n)
            .map(|i| {
                let v = if i < mid { val1 } else { val2 };
                (v % config.dual_codex.m_beta as u128) as u64
            })
            .collect();
        
        Self {
            id,
            label: label.to_string(),
            alpha_pattern,
            beta_pattern,
            confidence: 1.0,
            usage_count: 0,
            last_used: 0,
        }
    }
    
    /// Create from captured state
    pub fn from_state(id: u32, label: &str, state: &OrchestratorState) -> Self {
        Self {
            id,
            label: label.to_string(),
            alpha_pattern: state.alpha_lanes.iter().map(|l| l.value()).collect(),
            beta_pattern: state.beta_lanes.iter().map(|l| l.value()).collect(),
            confidence: 0.5,  // New templates start with medium confidence
            usage_count: 0,
            last_used: 0,
        }
    }
    
    /// Compute distance from state to this template
    pub fn distance(&self, state: &OrchestratorState) -> u64 {
        let alpha_dist: u64 = state.alpha_lanes.iter()
            .zip(self.alpha_pattern.iter())
            .map(|(lane, &target)| {
                let m = lane.modulus();
                let diff = (target + m - lane.value()) % m;
                diff.min(m - diff)
            })
            .sum();
        
        let beta_dist: u64 = state.beta_lanes.iter()
            .zip(self.beta_pattern.iter())
            .map(|(lane, &target)| {
                let m = lane.modulus();
                let diff = (target + m - lane.value()) % m;
                diff.min(m - diff)
            })
            .sum();
        
        alpha_dist + beta_dist
    }
}

// =============================================================================
// ORCHESTRATOR STATE
// =============================================================================

/// Orchestrator state: paired Alpha-Beta EPRAM fields
/// 
/// The state IS the computation substrate. Decisions emerge from
/// which attractor basin the state naturally falls into.
#[derive(Clone)]
pub struct OrchestratorState {
    /// Alpha channel lanes (primary residues)
    pub alpha_lanes: Vec<DualCodexLane>,
    /// Beta channel lanes (overflow tracking)
    pub beta_lanes: Vec<DualCodexLane>,
    /// Dual Codex configuration
    pub config: DualCodexConfig,
    /// Fourth Attractor parameters
    pub attractor_params: FourthAttractorParams,
    /// Current step count
    pub step_count: u64,
}

impl OrchestratorState {
    /// Create new orchestrator state from configuration
    pub fn new(config: &OrchestratorConfig) -> Self {
        let n = config.n_cells;
        let dc = &config.dual_codex;
        
        // Initialize all lanes to zero
        let alpha_lanes: Vec<DualCodexLane> = (0..n)
            .map(|_| DualCodexLane::alpha(0, dc))
            .collect();
        
        let beta_lanes: Vec<DualCodexLane> = (0..n)
            .map(|_| DualCodexLane::beta(0, dc))
            .collect();
        
        Self {
            alpha_lanes,
            beta_lanes,
            config: dc.clone(),
            attractor_params: FourthAttractorParams::default(),
            step_count: 0,
        }
    }
    
    /// Create from integer values
    pub fn from_values(values: &[u128], config: &OrchestratorConfig) -> Self {
        let n = values.len().min(config.n_cells);
        let dc = &config.dual_codex;
        
        let mut state = Self::new(config);
        
        for i in 0..n {
            let v = values[i] % dc.combined_modulus;
            state.alpha_lanes[i] = DualCodexLane::alpha(
                (v % dc.m_alpha as u128) as u64, dc
            );
            state.beta_lanes[i] = DualCodexLane::beta(
                (v % dc.m_beta as u128) as u64, dc
            );
        }
        
        state
    }
    
    /// Create from input (external data encoding)
    pub fn from_input(input: &[u8], config: &OrchestratorConfig) -> Self {
        // Encode bytes as values
        let values: Vec<u128> = input.chunks(8)
            .map(|chunk| {
                let mut bytes = [0u8; 8];
                bytes[..chunk.len()].copy_from_slice(chunk);
                u64::from_le_bytes(bytes) as u128
            })
            .collect();
        
        Self::from_values(&values, config)
    }
    
    /// Get combined value at index via K-Elimination
    pub fn value_at(&self, idx: usize) -> u128 {
        if idx >= self.alpha_lanes.len() {
            return 0;
        }
        
        let dc = DualCodexEPRAM {
            alpha: self.alpha_lanes[idx].clone(),
            beta: self.beta_lanes[idx].clone(),
            config: self.config.clone(),
        };
        
        dc.to_integer()
    }
    
    /// Get all values
    pub fn all_values(&self) -> Vec<u128> {
        (0..self.alpha_lanes.len())
            .map(|i| self.value_at(i))
            .collect()
    }
    
    /// Compute Lyapunov functional to target template
    pub fn lyapunov_to_template(&self, template: &ActionTemplate) -> u64 {
        template.distance(self)
    }
    
    /// Check if converged to template
    pub fn converged_to(&self, template: &ActionTemplate, threshold: u64) -> bool {
        self.lyapunov_to_template(template) <= threshold
    }
    
    /// Step toward template (one EPRAM evolution step)
    pub fn step_toward(&mut self, template: &ActionTemplate, coupling: f64) {
        let n = self.alpha_lanes.len();
        
        // Snapshot for synchronous update
        let alpha_snapshot = self.alpha_lanes.clone();
        let beta_snapshot = self.beta_lanes.clone();
        
        for i in 0..n {
            // Create target lanes from template
            let alpha_target = DualCodexLane::alpha(template.alpha_pattern[i], &self.config);
            let beta_target = DualCodexLane::beta(template.beta_pattern[i], &self.config);
            
            // Get neighbors (ring topology)
            let neighbors_alpha: Vec<DualCodexLane> = if n > 1 {
                vec![
                    alpha_snapshot[(i + n - 1) % n].clone(),
                    alpha_snapshot[(i + 1) % n].clone(),
                ]
            } else {
                vec![]
            };
            
            let neighbors_beta: Vec<DualCodexLane> = if n > 1 {
                vec![
                    beta_snapshot[(i + n - 1) % n].clone(),
                    beta_snapshot[(i + 1) % n].clone(),
                ]
            } else {
                vec![]
            };
            
            // Coupled transition
            if coupling < 0.01 {
                self.alpha_lanes[i] = alpha_snapshot[i].transition(&neighbors_alpha, &alpha_target);
                self.beta_lanes[i] = beta_snapshot[i].transition(&neighbors_beta, &beta_target);
            } else {
                self.alpha_lanes[i] = alpha_snapshot[i].coupled_transition(
                    &neighbors_alpha, &alpha_target, 1.0 - coupling * 0.5, coupling * 0.5
                );
                self.beta_lanes[i] = beta_snapshot[i].coupled_transition(
                    &neighbors_beta, &beta_target, 1.0 - coupling * 0.5, coupling * 0.5
                );
            }
        }
        
        self.step_count += 1;
    }
    
    /// Run evolution toward template until convergence
    pub fn evolve_to(&mut self, template: &ActionTemplate, max_steps: usize, coupling: f64) -> bool {
        for _ in 0..max_steps {
            if self.converged_to(template, 0) {
                return true;
            }
            self.step_toward(template, coupling);
        }
        self.converged_to(template, 0)
    }
}

// =============================================================================
// TEMPLATE STORE
// =============================================================================

/// Store for action templates
#[derive(Clone)]
pub struct TemplateStore {
    /// Templates indexed by ID
    templates: HashMap<u32, ActionTemplate>,
    /// Next available ID
    next_id: u32,
    /// Maximum templates
    max_templates: usize,
}

impl TemplateStore {
    /// Create new template store
    pub fn new(max_templates: usize) -> Self {
        Self {
            templates: HashMap::new(),
            next_id: 0,
            max_templates,
        }
    }
    
    /// Add template, returns ID
    pub fn add(&mut self, mut template: ActionTemplate) -> u32 {
        let id = self.next_id;
        template.id = id;
        self.next_id += 1;
        
        // If at capacity, remove least recently used
        if self.templates.len() >= self.max_templates {
            self.evict_lru();
        }
        
        self.templates.insert(id, template);
        id
    }
    
    /// Get template by ID
    pub fn get(&self, id: u32) -> Option<&ActionTemplate> {
        self.templates.get(&id)
    }
    
    /// Get mutable template by ID
    pub fn get_mut(&mut self, id: u32) -> Option<&mut ActionTemplate> {
        self.templates.get_mut(&id)
    }
    
    /// Remove template
    pub fn remove(&mut self, id: u32) -> Option<ActionTemplate> {
        self.templates.remove(&id)
    }
    
    /// Get all templates
    pub fn all(&self) -> Vec<&ActionTemplate> {
        self.templates.values().collect()
    }
    
    /// Find nearest template to state
    pub fn find_nearest(&self, state: &OrchestratorState) -> Option<&ActionTemplate> {
        self.templates.values()
            .min_by_key(|t| t.distance(state))
    }
    
    /// Find templates within distance threshold
    pub fn find_within(&self, state: &OrchestratorState, threshold: u64) -> Vec<&ActionTemplate> {
        self.templates.values()
            .filter(|t| t.distance(state) <= threshold)
            .collect()
    }
    
    /// Evict least recently used template
    fn evict_lru(&mut self) {
        if let Some((&id, _)) = self.templates.iter()
            .min_by_key(|(_, t)| t.last_used)
        {
            self.templates.remove(&id);
        }
    }
    
    /// Number of templates
    pub fn len(&self) -> usize {
        self.templates.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_orchestrator_config() {
        let config = OrchestratorConfig::default();
        assert_eq!(config.n_cells, 64);
        assert!(config.max_templates > 0);
    }
    
    #[test]
    fn test_orchestrator_state_creation() {
        let config = OrchestratorConfig::small();
        let state = OrchestratorState::new(&config);
        
        assert_eq!(state.alpha_lanes.len(), config.n_cells);
        assert_eq!(state.beta_lanes.len(), config.n_cells);
    }
    
    #[test]
    fn test_orchestrator_state_from_values() {
        let config = OrchestratorConfig::small();
        let values: Vec<u128> = vec![100, 200, 300, 400];
        let state = OrchestratorState::from_values(&values, &config);
        
        // First 4 cells should have values
        assert_eq!(state.value_at(0), 100);
        assert_eq!(state.value_at(1), 200);
        assert_eq!(state.value_at(2), 300);
        assert_eq!(state.value_at(3), 400);
    }
    
    #[test]
    fn test_action_template_uniform() {
        let config = OrchestratorConfig::small();
        let template = ActionTemplate::uniform(0, "test", 42, &config);
        
        assert_eq!(template.id, 0);
        assert_eq!(template.label, "test");
        assert!(template.alpha_pattern.iter().all(|&v| v == 42 % config.dual_codex.m_alpha));
    }
    
    #[test]
    fn test_action_template_gradient() {
        let config = OrchestratorConfig::small();
        let template = ActionTemplate::gradient(0, "gradient", &config);
        
        // Should be increasing
        for i in 1..template.alpha_pattern.len() {
            assert!(template.alpha_pattern[i] >= template.alpha_pattern[i-1]);
        }
    }
    
    #[test]
    fn test_template_distance() {
        let config = OrchestratorConfig::small();
        let template = ActionTemplate::uniform(0, "zero", 0, &config);
        let state = OrchestratorState::new(&config);  // All zeros
        
        let dist = template.distance(&state);
        assert_eq!(dist, 0, "Zero state should match zero template");
    }
    
    #[test]
    fn test_state_step_toward_template() {
        let config = OrchestratorConfig::small();
        let mut state = OrchestratorState::from_values(&[100, 200, 300, 400], &config);
        let template = ActionTemplate::uniform(0, "zero", 0, &config);
        
        let initial_dist = template.distance(&state);
        state.step_toward(&template, 0.0);
        let new_dist = template.distance(&state);
        
        assert!(new_dist <= initial_dist, "Distance should not increase");
    }
    
    #[test]
    fn test_state_evolve_to_convergence() {
        let config = OrchestratorConfig::small();
        let mut state = OrchestratorState::from_values(&[50, 100, 150, 200], &config);
        let template = ActionTemplate::uniform(0, "zero", 0, &config);
        
        let converged = state.evolve_to(&template, 500, 0.0);
        
        assert!(converged, "Should converge to uniform template");
        assert_eq!(template.distance(&state), 0);
    }
    
    #[test]
    fn test_template_store() {
        let mut store = TemplateStore::new(10);
        
        let config = OrchestratorConfig::small();
        let t1 = ActionTemplate::uniform(0, "first", 0, &config);
        let t2 = ActionTemplate::uniform(0, "second", 100, &config);
        
        let id1 = store.add(t1);
        let id2 = store.add(t2);
        
        assert_ne!(id1, id2);
        assert_eq!(store.len(), 2);
        
        assert!(store.get(id1).is_some());
        assert!(store.get(id2).is_some());
    }
    
    #[test]
    fn test_template_store_find_nearest() {
        let config = OrchestratorConfig::small();
        let mut store = TemplateStore::new(10);
        
        store.add(ActionTemplate::uniform(0, "zero", 0, &config));
        store.add(ActionTemplate::uniform(0, "hundred", 100, &config));
        
        // State closer to zero
        let state = OrchestratorState::from_values(&[10, 10, 10, 10], &config);
        let nearest = store.find_nearest(&state).unwrap();
        
        assert_eq!(nearest.label, "zero");
    }
    
    #[test]
    fn test_k_elimination_in_state() {
        let config = OrchestratorConfig::small();
        
        // Use values that span the combined modulus
        let test_vals: Vec<u128> = vec![0, 100, 1000, 10000, 50000];
        let state = OrchestratorState::from_values(&test_vals, &config);
        
        // Verify K-Elimination reconstructs correctly
        for (i, &expected) in test_vals.iter().enumerate() {
            if i < config.n_cells {
                let actual = state.value_at(i);
                let expected_mod = expected % config.dual_codex.combined_modulus;
                assert_eq!(actual, expected_mod, "K-Elimination failed at index {}", i);
            }
        }
    }
}
