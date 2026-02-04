//! Dense vs Sparse Grover Comparison Test
//!
//! Purpose: Determine whether the probability readout variance is caused by:
//! 1. The SPARSE representation compromise (only 2 amplitudes stored), OR
//! 2. The F_p² modular coordinate system (modular arithmetic readout)
//!
//! Method: Compare at small qubit counts where dense representation is feasible
//!
//! Expected outcome:
//! - If dense MobiusInt gives ~96% at π/4√N iterations → sparseness caused variance
//! - If dense MobiusInt also shows variance → F_p² coordinate system is the cause

use nine65::quantum::{
    amplitude::{QuantumState, grover_search, QuantumAmplitude},
    coherence::{SparseGroverFp2, TEST_PRIME},
};

/// Calculate theoretical optimal iterations: floor(π/4 × √N)
fn optimal_iterations(num_qubits: usize) -> usize {
    let n = 1usize << num_qubits;
    let sqrt_n = (n as f64).sqrt();
    let optimal = (std::f64::consts::PI / 4.0) * sqrt_n;
    optimal.floor() as usize
}

/// Dense Grover using MobiusInt (from amplitude.rs)
/// Returns (target_probability_ratio, total_weight)
fn run_dense_grover(num_qubits: usize, target: usize, iterations: usize) -> (f64, u64) {
    let scale = 1000u64;
    let mut state = QuantumState::uniform_superposition(num_qubits, scale);

    for _ in 0..iterations {
        state.oracle_mark(target);
        state.grover_diffusion();
    }

    let probs = state.probabilities();
    let total: u64 = probs.iter().sum();
    let target_prob = probs[target];

    let ratio = if total > 0 {
        target_prob as f64 / total as f64
    } else {
        0.0
    };

    (ratio, total)
}

/// Sparse Grover using F_p² (from coherence.rs)
/// Returns (target_probability_ratio, total_weight)
fn run_sparse_grover(num_qubits: usize, iterations: usize) -> (f64, u64) {
    let mut state = SparseGroverFp2::uniform(num_qubits, TEST_PRIME);
    state.disable_wassan_storage(); // Pure F_p² comparison

    for _ in 0..iterations {
        state.grover_iteration();
    }

    let prob = state.target_probability();
    let weight = state.total_weight();

    (prob, weight)
}

#[test]
fn test_dense_vs_sparse_4_qubit() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║     DENSE vs SPARSE GROVER COMPARISON (4 qubits)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let num_qubits = 4;
    let target = 7; // arbitrary target
    let n = 1usize << num_qubits;
    let optimal = optimal_iterations(num_qubits);

    println!("Configuration:");
    println!("  Qubits:             {}", num_qubits);
    println!("  State space:        {} states", n);
    println!("  Target:             {}", target);
    println!("  Optimal iterations: {} (π/4 × √{})", optimal, n);
    println!();

    // Run at various iteration counts
    let test_iterations = [1, 2, 3, 4, 5, 6, 7, 8, 10];

    println!("{:>6} | {:>12} | {:>12} | {:>10}", "Iter", "Dense Prob", "Sparse Prob", "Diff");
    println!("{:-<6}-+-{:-<12}-+-{:-<12}-+-{:-<10}", "", "", "", "");

    for &iters in &test_iterations {
        let (dense_prob, _dense_weight) = run_dense_grover(num_qubits, target, iters);
        let (sparse_prob, _sparse_weight) = run_sparse_grover(num_qubits, iters);

        let diff = (dense_prob - sparse_prob).abs();

        println!(
            "{:>6} | {:>11.4}% | {:>11.4}% | {:>10.4}%",
            iters,
            dense_prob * 100.0,
            sparse_prob * 100.0,
            diff * 100.0
        );
    }

    println!();
    println!("At optimal ({} iterations):", optimal);
    let (dense_prob, dense_weight) = run_dense_grover(num_qubits, target, optimal);
    let (sparse_prob, sparse_weight) = run_sparse_grover(num_qubits, optimal);

    println!("  Dense (MobiusInt):  {:.2}% probability, weight={}", dense_prob * 100.0, dense_weight);
    println!("  Sparse (F_p²):      {:.2}% probability, weight={}", sparse_prob * 100.0, sparse_weight);

    // Standard quantum mechanics predicts ~96% at optimal for single target
    let expected_quantum = 0.96; // sin²((2*optimal+1) × arcsin(1/√N))
    println!("  Expected (QM):      ~{:.0}%", expected_quantum * 100.0);

    if (dense_prob - expected_quantum).abs() < 0.10 {
        println!("\n✓ Dense MobiusInt matches quantum prediction!");
        println!("  → Sparse F_p² readout variance is due to the SPARSE COMPROMISE");
    } else {
        println!("\n✗ Dense MobiusInt also differs from quantum prediction");
        println!("  → Variance is NOT from sparseness, but from integer coordinate system");
    }
}

#[test]
fn test_dense_vs_sparse_5_qubit() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║     DENSE vs SPARSE GROVER COMPARISON (5 qubits)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let num_qubits = 5;
    let target = 13;
    let n = 1usize << num_qubits;
    let optimal = optimal_iterations(num_qubits);

    println!("Configuration:");
    println!("  Qubits:             {}", num_qubits);
    println!("  State space:        {} states", n);
    println!("  Target:             {}", target);
    println!("  Optimal iterations: {} (π/4 × √{})", optimal, n);
    println!();

    println!("{:>6} | {:>12} | {:>12} | {:>10}", "Iter", "Dense Prob", "Sparse Prob", "Diff");
    println!("{:-<6}-+-{:-<12}-+-{:-<12}-+-{:-<10}", "", "", "", "");

    for iters in 1..=8 {
        let (dense_prob, _) = run_dense_grover(num_qubits, target, iters);
        let (sparse_prob, _) = run_sparse_grover(num_qubits, iters);

        let diff = (dense_prob - sparse_prob).abs();
        let marker = if iters == optimal { " *" } else { "" };

        println!(
            "{:>6} | {:>11.4}% | {:>11.4}% | {:>10.4}%{}",
            iters,
            dense_prob * 100.0,
            sparse_prob * 100.0,
            diff * 100.0,
            marker
        );
    }

    println!("\n* = optimal iteration count");
}

#[test]
fn test_dense_vs_sparse_6_qubit() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║     DENSE vs SPARSE GROVER COMPARISON (6 qubits)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let num_qubits = 6;
    let target = 42;
    let n = 1usize << num_qubits;
    let optimal = optimal_iterations(num_qubits);

    println!("Configuration:");
    println!("  Qubits:             {}", num_qubits);
    println!("  State space:        {} states", n);
    println!("  Target:             {}", target);
    println!("  Optimal iterations: {}", optimal);
    println!();

    // Test specifically at and around optimal
    for iters in (optimal.saturating_sub(2))..=(optimal + 3) {
        let (dense_prob, dense_weight) = run_dense_grover(num_qubits, target, iters);
        let (sparse_prob, sparse_weight) = run_sparse_grover(num_qubits, iters);

        let diff = (dense_prob - sparse_prob).abs();
        let marker = if iters == optimal { " ← optimal" } else { "" };

        println!(
            "Iter {:>2}: Dense={:>6.2}% (w={:>10}) | Sparse={:>6.2}% (w={:>8}) | Δ={:>5.2}%{}",
            iters,
            dense_prob * 100.0, dense_weight,
            sparse_prob * 100.0, sparse_weight,
            diff * 100.0,
            marker
        );
    }
}

#[test]
fn test_weight_preservation_comparison() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║     WEIGHT PRESERVATION: Dense vs Sparse                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let num_qubits = 4;
    let scale = 1000u64;

    // Dense initial weight
    let mut dense = QuantumState::uniform_superposition(num_qubits, scale);
    let dense_initial: u64 = dense.probabilities().iter().sum();

    // Sparse initial weight
    let mut sparse = SparseGroverFp2::uniform(num_qubits, TEST_PRIME);
    sparse.disable_wassan_storage();
    let sparse_initial = sparse.total_weight();

    println!("Initial weights:");
    println!("  Dense:  {}", dense_initial);
    println!("  Sparse: {} (mod {})", sparse_initial, TEST_PRIME);
    println!();

    // Run 1000 iterations
    let iterations = 1000;
    for _ in 0..iterations {
        dense.oracle_mark(0);
        dense.grover_diffusion();
        sparse.grover_iteration();
    }

    let dense_final: u64 = dense.probabilities().iter().sum();
    let sparse_final = sparse.total_weight();

    println!("After {} iterations:", iterations);
    println!("  Dense:  {} (Δ = {})", dense_final, (dense_final as i128 - dense_initial as i128).abs());
    println!("  Sparse: {} (Δ = {})", sparse_final,
             if sparse_initial == sparse_final { 0 } else { 1 });

    // Dense will drift due to integer division in mean calculation
    // Sparse should be EXACTLY preserved (modular arithmetic)

    let sparse_preserved = sparse_initial == sparse_final;
    println!();
    println!("Weight preservation:");
    println!("  Dense:  {} (integer division introduces drift)",
             if dense_initial == dense_final { "EXACT" } else { "DRIFTED" });
    println!("  Sparse: {} (modular arithmetic is exact)",
             if sparse_preserved { "EXACT" } else { "DRIFTED" });
}

#[test]
fn test_full_comparison_report() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                          ║");
    println!("║       COMPREHENSIVE DENSE vs SPARSE GROVER ANALYSIS                      ║");
    println!("║       Isolating the Source of Probability Readout Variance               ║");
    println!("║                                                                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\n");

    // Test across multiple qubit counts
    for num_qubits in [3, 4, 5, 6, 7].iter() {
        let n = 1usize << num_qubits;
        let optimal = optimal_iterations(*num_qubits);
        let target = n / 3; // arbitrary target

        let (dense_prob, _) = run_dense_grover(*num_qubits, target, optimal);
        let (sparse_prob, _) = run_sparse_grover(*num_qubits, optimal);

        // Theoretical quantum probability at optimal: sin²((2k+1)θ) where θ = arcsin(1/√N)
        let theta = (1.0 / (n as f64).sqrt()).asin();
        let theoretical = ((2 * optimal + 1) as f64 * theta).sin().powi(2);

        println!("{}-qubit (N={}, opt={}):", num_qubits, n, optimal);
        println!("  Theoretical (QM): {:.2}%", theoretical * 100.0);
        println!("  Dense (MobiusInt): {:.2}% (Δ = {:.2}%)",
                 dense_prob * 100.0, (dense_prob - theoretical).abs() * 100.0);
        println!("  Sparse (F_p²):     {:.2}% (Δ = {:.2}%)",
                 sparse_prob * 100.0, (sparse_prob - theoretical).abs() * 100.0);
        println!();
    }

    println!("════════════════════════════════════════════════════════════════════════════");
    println!("CONCLUSION:");
    println!("  If Dense matches Theoretical → Sparse variance is from the sparse compromise");
    println!("  If Dense differs from Theoretical → Variance is from integer coordinate system");
    println!("════════════════════════════════════════════════════════════════════════════");
}
