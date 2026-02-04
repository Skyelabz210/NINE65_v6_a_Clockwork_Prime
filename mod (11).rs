//! GATE 6: QMNF/EPRAM Autopoiesis Module
//! 
//! Self-modification and evolution capabilities:
//! - Self-observation of execution patterns
//! - Hypothesis generation for optimization
//! - Automated testing of improvements
//! - Code generation for validated changes
//!
//! INNOVATION: Computational Autopoiesis
//! A self-modifying system that evolves its own algorithms
//! while maintaining mathematical correctness guarantees.

pub mod evolution;

pub use evolution::*;

/// Quick access to evolution engine
pub fn create_evolution_engine() -> EvolutionEngine {
    EvolutionEngine::new()
}

/// Run a full autopoiesis cycle
pub fn run_autopoiesis_cycle(engine: &mut EvolutionEngine) -> AutopoiesisReport {
    let cycle_result = engine.evolve_cycle();
    
    AutopoiesisReport {
        generation: cycle_result.generation,
        hypotheses_explored: cycle_result.hypotheses_generated,
        changes_applied: cycle_result.changes_applied,
        cumulative_improvement: engine.history()
            .iter()
            .filter(|c| !c.reverted)
            .map(|c| c.improvement)
            .sum(),
    }
}

#[derive(Debug)]
pub struct AutopoiesisReport {
    pub generation: usize,
    pub hypotheses_explored: usize,
    pub changes_applied: usize,
    pub cumulative_improvement: f64,
}
