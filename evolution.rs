//! GATE 6: AUTOPOIESIS - SELF-MODIFICATION AND EVOLUTION
//! 
//! Enables the QMNF/EPRAM system to:
//! - Self-analyze performance bottlenecks
//! - Generate optimization hypotheses
//! - Test and validate improvements
//! - Evolve its own algorithms
//!
//! INNOVATION: Computational Autopoiesis
//! - System observes its own execution
//! - Identifies patterns in failures/successes
//! - Proposes and tests modifications
//! - Integrates successful changes

use std::collections::HashMap;
use std::time::{Duration, Instant};

// =============================================================================
// SELF-OBSERVATION
// =============================================================================

/// Execution trace for self-analysis
#[derive(Clone, Debug)]
pub struct ExecutionTrace {
    /// Operation name
    pub operation: String,
    /// Input characteristics
    pub input_profile: InputProfile,
    /// Execution time
    pub duration: Duration,
    /// Memory usage delta
    pub memory_delta: i64,
    /// Success/failure
    pub success: bool,
    /// Error if any
    pub error: Option<String>,
    /// Convergence steps (for EPRAM)
    pub convergence_steps: Option<usize>,
    /// Accuracy metric
    pub accuracy: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct InputProfile {
    pub size: usize,
    pub modulus: u64,
    pub sparsity: f64,
    pub range: (u64, u64),
}

/// Self-observer that collects execution traces
pub struct SelfObserver {
    traces: Vec<ExecutionTrace>,
    operation_stats: HashMap<String, OperationStats>,
    bottleneck_threshold_ns: u64,
}

#[derive(Clone, Debug, Default)]
pub struct OperationStats {
    pub count: usize,
    pub total_time_ns: u128,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
    pub failures: usize,
    pub avg_convergence: Option<f64>,
}

impl SelfObserver {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            operation_stats: HashMap::new(),
            bottleneck_threshold_ns: 10_000,  // 10μs
        }
    }
    
    /// Record an execution trace
    pub fn record(&mut self, trace: ExecutionTrace) {
        let stats = self.operation_stats
            .entry(trace.operation.clone())
            .or_default();
        
        let duration_ns = trace.duration.as_nanos() as u64;
        stats.count += 1;
        stats.total_time_ns += duration_ns as u128;
        stats.min_time_ns = stats.min_time_ns.min(duration_ns).max(1);
        stats.max_time_ns = stats.max_time_ns.max(duration_ns);
        
        if !trace.success {
            stats.failures += 1;
        }
        
        if let Some(steps) = trace.convergence_steps {
            let prev_avg = stats.avg_convergence.unwrap_or(0.0);
            let new_avg = (prev_avg * (stats.count - 1) as f64 + steps as f64) / stats.count as f64;
            stats.avg_convergence = Some(new_avg);
        }
        
        self.traces.push(trace);
    }
    
    /// Identify bottleneck operations
    pub fn identify_bottlenecks(&self) -> Vec<(&str, &OperationStats)> {
        let mut bottlenecks: Vec<_> = self.operation_stats.iter()
            .filter(|(_, stats)| {
                let avg_ns = stats.total_time_ns / stats.count.max(1) as u128;
                avg_ns > self.bottleneck_threshold_ns as u128
            })
            .map(|(name, stats)| (name.as_str(), stats))
            .collect();
        
        bottlenecks.sort_by(|a, b| b.1.total_time_ns.cmp(&a.1.total_time_ns));
        bottlenecks
    }
    
    /// Identify high-failure operations
    pub fn identify_failures(&self) -> Vec<(&str, f64)> {
        self.operation_stats.iter()
            .filter(|(_, stats)| stats.failures > 0)
            .map(|(name, stats)| {
                let failure_rate = stats.failures as f64 / stats.count as f64;
                (name.as_str(), failure_rate)
            })
            .filter(|(_, rate)| *rate > 0.01)  // >1% failure rate
            .collect()
    }
}

impl Default for SelfObserver {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HYPOTHESIS GENERATION
// =============================================================================

/// Optimization hypothesis generated from observations
#[derive(Clone, Debug)]
pub struct OptimizationHypothesis {
    /// Unique ID
    pub id: usize,
    /// Target operation
    pub target_operation: String,
    /// Hypothesis type
    pub hypothesis_type: HypothesisType,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Confidence in hypothesis
    pub confidence: f64,
    /// Required changes
    pub changes: Vec<ProposedChange>,
}

#[derive(Clone, Debug)]
pub enum HypothesisType {
    /// Reduce iteration count
    ReduceIterations { current: usize, proposed: usize },
    /// Change algorithm
    AlgorithmSwitch { from: String, to: String },
    /// Adjust parameters
    ParameterTuning { param: String, from: f64, to: f64 },
    /// Add caching
    AddCaching { cache_key: String },
    /// Parallelize
    Parallelize { factor: usize },
    /// Change modulus
    ModulusOptimization { from: u64, to: u64 },
}

#[derive(Clone, Debug)]
pub struct ProposedChange {
    pub component: String,
    pub current_code: String,
    pub proposed_code: String,
    pub rationale: String,
}

/// Hypothesis generator based on observations
pub struct HypothesisGenerator {
    patterns: Vec<OptimizationPattern>,
}

#[derive(Clone)]
pub struct OptimizationPattern {
    pub name: String,
    pub condition: fn(&OperationStats) -> bool,
    pub generate: fn(&str, &OperationStats) -> Option<OptimizationHypothesis>,
}

impl HypothesisGenerator {
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }
    
    fn default_patterns() -> Vec<OptimizationPattern> {
        vec![
            // Pattern 1: High convergence steps → reduce k
            OptimizationPattern {
                name: "high_convergence".to_string(),
                condition: |stats| {
                    stats.avg_convergence.map(|c| c > 50.0).unwrap_or(false)
                },
                generate: |op, stats| {
                    let current = stats.avg_convergence.unwrap_or(100.0) as usize;
                    Some(OptimizationHypothesis {
                        id: 0,
                        target_operation: op.to_string(),
                        hypothesis_type: HypothesisType::ReduceIterations {
                            current,
                            proposed: (current * 3) / 4,
                        },
                        expected_improvement: 0.25,
                        confidence: 0.7,
                        changes: vec![ProposedChange {
                            component: "fourth_attractor".to_string(),
                            current_code: "k_num: 3, k_den: 4".to_string(),
                            proposed_code: "k_num: 7, k_den: 8".to_string(),
                            rationale: "Increase k for faster convergence".to_string(),
                        }],
                    })
                },
            },
            
            // Pattern 2: High failure rate → add validation
            OptimizationPattern {
                name: "high_failure".to_string(),
                condition: |stats| {
                    stats.failures as f64 / stats.count.max(1) as f64 > 0.05
                },
                generate: |op, stats| {
                    Some(OptimizationHypothesis {
                        id: 0,
                        target_operation: op.to_string(),
                        hypothesis_type: HypothesisType::ParameterTuning {
                            param: "validation_level".to_string(),
                            from: 0.0,
                            to: 1.0,
                        },
                        expected_improvement: stats.failures as f64 / stats.count as f64,
                        confidence: 0.8,
                        changes: vec![ProposedChange {
                            component: op.to_string(),
                            current_code: "// no validation".to_string(),
                            proposed_code: "guard.check(p_bound, q_bound)?;".to_string(),
                            rationale: "Add bound checking to prevent failures".to_string(),
                        }],
                    })
                },
            },
            
            // Pattern 3: High latency → try parallelization
            OptimizationPattern {
                name: "high_latency".to_string(),
                condition: |stats| {
                    let avg_ns = stats.total_time_ns / stats.count.max(1) as u128;
                    avg_ns > 100_000  // >100μs
                },
                generate: |op, stats| {
                    Some(OptimizationHypothesis {
                        id: 0,
                        target_operation: op.to_string(),
                        hypothesis_type: HypothesisType::Parallelize { factor: 4 },
                        expected_improvement: 0.6,  // ~4x with overhead
                        confidence: 0.5,
                        changes: vec![ProposedChange {
                            component: op.to_string(),
                            current_code: "for i in 0..n { ... }".to_string(),
                            proposed_code: "cells.par_iter().map(...).collect()".to_string(),
                            rationale: "Parallelize independent cell operations".to_string(),
                        }],
                    })
                },
            },
        ]
    }
    
    /// Generate hypotheses from observations
    pub fn generate(&self, observer: &SelfObserver) -> Vec<OptimizationHypothesis> {
        let mut hypotheses = Vec::new();
        let mut id = 0;
        
        for (op_name, stats) in &observer.operation_stats {
            for pattern in &self.patterns {
                if (pattern.condition)(stats) {
                    if let Some(mut hyp) = (pattern.generate)(op_name, stats) {
                        hyp.id = id;
                        id += 1;
                        hypotheses.push(hyp);
                    }
                }
            }
        }
        
        // Sort by expected improvement * confidence
        hypotheses.sort_by(|a, b| {
            let score_a = a.expected_improvement * a.confidence;
            let score_b = b.expected_improvement * b.confidence;
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        hypotheses
    }
}

impl Default for HypothesisGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HYPOTHESIS TESTING
// =============================================================================

/// Result of testing a hypothesis
#[derive(Clone, Debug)]
pub struct HypothesisTestResult {
    pub hypothesis_id: usize,
    pub baseline_metric: f64,
    pub optimized_metric: f64,
    pub improvement: f64,
    pub statistical_significance: f64,
    pub passed: bool,
    pub side_effects: Vec<String>,
}

/// Hypothesis tester using A/B comparison
pub struct HypothesisTester {
    pub min_samples: usize,
    pub significance_threshold: f64,
    pub improvement_threshold: f64,
}

impl HypothesisTester {
    pub fn new() -> Self {
        Self {
            min_samples: 100,
            significance_threshold: 0.95,
            improvement_threshold: 0.05,  // 5% improvement required
        }
    }
    
    /// Test a hypothesis by comparing baseline vs optimized
    pub fn test<F, G>(
        &self,
        hypothesis: &OptimizationHypothesis,
        baseline_fn: F,
        optimized_fn: G,
    ) -> HypothesisTestResult
    where
        F: Fn() -> f64,
        G: Fn() -> f64,
    {
        // Collect baseline samples
        let mut baseline_samples = Vec::with_capacity(self.min_samples);
        for _ in 0..self.min_samples {
            baseline_samples.push(baseline_fn());
        }
        
        // Collect optimized samples
        let mut optimized_samples = Vec::with_capacity(self.min_samples);
        for _ in 0..self.min_samples {
            optimized_samples.push(optimized_fn());
        }
        
        // Compute statistics
        let baseline_mean: f64 = baseline_samples.iter().sum::<f64>() / self.min_samples as f64;
        let optimized_mean: f64 = optimized_samples.iter().sum::<f64>() / self.min_samples as f64;
        
        let baseline_var: f64 = baseline_samples.iter()
            .map(|x| (x - baseline_mean).powi(2))
            .sum::<f64>() / self.min_samples as f64;
        let optimized_var: f64 = optimized_samples.iter()
            .map(|x| (x - optimized_mean).powi(2))
            .sum::<f64>() / self.min_samples as f64;
        
        // t-test (simplified)
        let pooled_std = ((baseline_var + optimized_var) / 2.0).sqrt();
        let t_stat = if pooled_std > 0.0 {
            (baseline_mean - optimized_mean).abs() / (pooled_std * (2.0 / self.min_samples as f64).sqrt())
        } else {
            0.0
        };
        
        // Approximate p-value (simplified)
        let significance = 1.0 - (-t_stat.abs() / 2.0).exp();
        
        let improvement = if baseline_mean > 0.0 {
            (baseline_mean - optimized_mean) / baseline_mean
        } else {
            0.0
        };
        
        let passed = improvement >= self.improvement_threshold 
            && significance >= self.significance_threshold;
        
        HypothesisTestResult {
            hypothesis_id: hypothesis.id,
            baseline_metric: baseline_mean,
            optimized_metric: optimized_mean,
            improvement,
            statistical_significance: significance,
            passed,
            side_effects: vec![],
        }
    }
}

impl Default for HypothesisTester {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// EVOLUTION ENGINE
// =============================================================================

/// Evolution engine that drives system improvement
pub struct EvolutionEngine {
    observer: SelfObserver,
    generator: HypothesisGenerator,
    tester: HypothesisTester,
    applied_changes: Vec<AppliedChange>,
    generation: usize,
}

#[derive(Clone, Debug)]
pub struct AppliedChange {
    pub hypothesis_id: usize,
    pub generation: usize,
    pub improvement: f64,
    pub changes: Vec<ProposedChange>,
    pub reverted: bool,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        Self {
            observer: SelfObserver::new(),
            generator: HypothesisGenerator::new(),
            tester: HypothesisTester::new(),
            applied_changes: Vec::new(),
            generation: 0,
        }
    }
    
    /// Record an execution
    pub fn observe(&mut self, trace: ExecutionTrace) {
        self.observer.record(trace);
    }
    
    /// Run one evolution cycle
    pub fn evolve_cycle(&mut self) -> EvolutionCycleResult {
        self.generation += 1;
        
        // Step 1: Generate hypotheses
        let hypotheses = self.generator.generate(&self.observer);
        
        // Step 2: Test top hypotheses
        let mut test_results = Vec::new();
        for hyp in hypotheses.iter().take(3) {
            // In real implementation, would actually run the optimized code
            // Here we simulate with dummy functions
            let result = self.tester.test(
                hyp,
                || rand_f64() * 100.0,  // Baseline (simulated)
                || rand_f64() * 80.0,   // Optimized (simulated, ~20% better)
            );
            test_results.push((hyp.clone(), result));
        }
        
        // Step 3: Apply successful changes
        let mut applied = Vec::new();
        for (hyp, result) in &test_results {
            if result.passed {
                applied.push(AppliedChange {
                    hypothesis_id: hyp.id,
                    generation: self.generation,
                    improvement: result.improvement,
                    changes: hyp.changes.clone(),
                    reverted: false,
                });
            }
        }
        
        self.applied_changes.extend(applied.clone());
        
        EvolutionCycleResult {
            generation: self.generation,
            hypotheses_generated: hypotheses.len(),
            hypotheses_tested: test_results.len(),
            changes_applied: applied.len(),
            total_improvement: applied.iter().map(|c| c.improvement).sum(),
        }
    }
    
    /// Get evolution history
    pub fn history(&self) -> &[AppliedChange] {
        &self.applied_changes
    }
    
    /// Revert a change
    pub fn revert(&mut self, hypothesis_id: usize) {
        if let Some(change) = self.applied_changes.iter_mut()
            .find(|c| c.hypothesis_id == hypothesis_id && !c.reverted) 
        {
            change.reverted = true;
        }
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct EvolutionCycleResult {
    pub generation: usize,
    pub hypotheses_generated: usize,
    pub hypotheses_tested: usize,
    pub changes_applied: usize,
    pub total_improvement: f64,
}

// Simulated random for testing
fn rand_f64() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

// =============================================================================
// SELF-MODIFYING CODE GENERATION
// =============================================================================

/// Generates actual code modifications based on hypotheses
pub struct CodeGenerator {
    templates: HashMap<String, CodeTemplate>,
}

#[derive(Clone)]
pub struct CodeTemplate {
    pub name: String,
    pub parameters: Vec<String>,
    pub template: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Template: Faster convergence
        templates.insert("faster_convergence".to_string(), CodeTemplate {
            name: "faster_convergence".to_string(),
            parameters: vec!["k_num".to_string(), "k_den".to_string()],
            template: r#"
fn fourth_attractor_step_optimized(state: u64, target: u64, m: u64) -> u64 {
    let diff = (target + m - state) % m;
    if diff == 0 { return state; }
    
    let delta = (diff * {k_num}) / {k_den};
    let delta = if delta == 0 {
        if diff <= m / 2 { 1 } else { m - 1 }
    } else { delta };
    
    (state + delta) % m
}
"#.to_string(),
        });
        
        // Template: Parallel field evolution
        templates.insert("parallel_field".to_string(), CodeTemplate {
            name: "parallel_field".to_string(),
            parameters: vec!["chunk_size".to_string()],
            template: r#"
fn evolve_field_parallel(cells: &mut [u64], targets: &[u64], m: u64) {
    use rayon::prelude::*;
    
    cells.par_chunks_mut({chunk_size})
        .zip(targets.par_chunks({chunk_size}))
        .for_each(|(cell_chunk, target_chunk)| {
            for (cell, target) in cell_chunk.iter_mut().zip(target_chunk.iter()) {
                *cell = fourth_attractor_step(*cell, *target, m);
            }
        });
}
"#.to_string(),
        });
        
        Self { templates }
    }
    
    /// Generate code for a hypothesis
    pub fn generate(&self, hypothesis: &OptimizationHypothesis) -> Option<String> {
        match &hypothesis.hypothesis_type {
            HypothesisType::ReduceIterations { proposed, .. } => {
                self.templates.get("faster_convergence").map(|t| {
                    t.template
                        .replace("{k_num}", &format!("{}", proposed * 7 / 8))
                        .replace("{k_den}", &format!("{}", proposed))
                })
            }
            HypothesisType::Parallelize { factor } => {
                self.templates.get("parallel_field").map(|t| {
                    t.template.replace("{chunk_size}", &format!("{}", factor * 16))
                })
            }
            _ => None,
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_self_observer() {
        let mut observer = SelfObserver::new();
        
        observer.record(ExecutionTrace {
            operation: "test_op".to_string(),
            input_profile: InputProfile {
                size: 100,
                modulus: 256,
                sparsity: 0.0,
                range: (0, 255),
            },
            duration: Duration::from_micros(50),
            memory_delta: 0,
            success: true,
            error: None,
            convergence_steps: Some(20),
            accuracy: Some(1.0),
        });
        
        assert_eq!(observer.operation_stats.len(), 1);
        assert_eq!(observer.operation_stats["test_op"].count, 1);
    }
    
    #[test]
    fn test_hypothesis_generation() {
        let mut observer = SelfObserver::new();
        
        // Record slow operation
        for _ in 0..10 {
            observer.record(ExecutionTrace {
                operation: "slow_op".to_string(),
                input_profile: InputProfile {
                    size: 100,
                    modulus: 256,
                    sparsity: 0.0,
                    range: (0, 255),
                },
                duration: Duration::from_millis(1),  // 1ms = slow
                memory_delta: 0,
                success: true,
                error: None,
                convergence_steps: Some(100),  // High convergence
                accuracy: Some(1.0),
            });
        }
        
        let generator = HypothesisGenerator::new();
        let hypotheses = generator.generate(&observer);
        
        assert!(!hypotheses.is_empty());
    }
    
    #[test]
    fn test_evolution_engine() {
        let mut engine = EvolutionEngine::new();
        
        // Record some traces
        for _ in 0..100 {
            engine.observe(ExecutionTrace {
                operation: "evolve_field".to_string(),
                input_profile: InputProfile {
                    size: 100,
                    modulus: 256,
                    sparsity: 0.0,
                    range: (0, 255),
                },
                duration: Duration::from_micros(500),
                memory_delta: 0,
                success: true,
                error: None,
                convergence_steps: Some(75),
                accuracy: Some(1.0),
            });
        }
        
        // Run evolution cycle
        let result = engine.evolve_cycle();
        
        println!("Evolution cycle: {:?}", result);
        assert_eq!(result.generation, 1);
    }
    
    #[test]
    fn test_code_generation() {
        let generator = CodeGenerator::new();
        
        let hypothesis = OptimizationHypothesis {
            id: 0,
            target_operation: "field_step".to_string(),
            hypothesis_type: HypothesisType::Parallelize { factor: 4 },
            expected_improvement: 0.6,
            confidence: 0.7,
            changes: vec![],
        };
        
        let code = generator.generate(&hypothesis);
        
        assert!(code.is_some());
        assert!(code.unwrap().contains("par_chunks"));
    }
}
