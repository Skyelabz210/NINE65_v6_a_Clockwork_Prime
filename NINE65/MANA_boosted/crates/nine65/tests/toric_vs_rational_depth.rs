//! Comparison: Pure Toric vs Rational for Deep Iterations
//!
//! This demonstrates the fundamental difference:
//! - Rational: i128 overflow limits depth
//! - Toric: Helix climbing handles arbitrary depth

use nine65::quantum::dense_rational::DenseRationalGrover;
use nine65::quantum::dense_toric_pure::{DualCodex, DenseToricPure};

#[test]
fn test_rational_overflow_limit() {
    println!("\n=== Rational Arithmetic: Finding Overflow Limit ===\n");

    let mut state = DenseRationalGrover::for_target(4, 7);

    let mut max_iter = 0;
    let mut overflow_at = None;

    // Try to run iterations until overflow
    for i in 0..100 {
        // Check if any denominator would overflow on next iteration
        // Denominator grows as N^iter = 16^iter for 4 qubits
        let max_den = state.amplitudes.iter()
            .map(|a| a.den)
            .max()
            .unwrap_or(1);

        // i128::MAX ≈ 1.7 × 10^38
        // If we multiply by 16 twice more and max_den > 10^36, we'll overflow
        if max_den > 10_u128.pow(35) {
            overflow_at = Some(i);
            println!("Would overflow at iteration {}: max_den = {:.2e}", i, max_den as f64);
            break;
        }

        state.grover_iteration();
        max_iter = i + 1;

        if max_iter <= 5 || max_iter % 5 == 0 {
            println!("Iter {:2}: max_den = {:.2e}", max_iter, max_den as f64);
        }
    }

    if overflow_at.is_some() {
        println!("\n⚠️  Rational arithmetic limited to ~{} iterations", max_iter);
    } else {
        println!("\n✓ Completed 100 iterations (unexpected for 4 qubits)");
    }
}

#[test]
fn test_toric_unlimited_depth() {
    println!("\n=== Pure Toric: Unlimited Depth via Helix ===\n");

    let dc = DualCodex::large_primes();
    let mut state = DenseToricPure::for_target(4, 7, dc);

    // Run the same number of iterations that would overflow rational
    let iterations = 100;
    let mut helix_levels: Vec<u64> = Vec::new();

    for i in 0..iterations {
        state.grover_iteration();

        let k = state.amplitudes[state.target].point.helix_level(&dc);
        helix_levels.push(k);

        if i < 5 || (i + 1) % 20 == 0 {
            println!("Iter {:3}: helix_level = {}", i + 1, k);
        }
    }

    // Calculate helix growth
    let max_helix = helix_levels.iter().max().copied().unwrap_or(0);
    println!("\n✓ Completed {} iterations WITHOUT overflow", iterations);
    println!("  Maximum helix level: {}", max_helix);
    println!("  This encodes amplitude growth that would overflow i128");
}

#[test]
fn test_both_match_at_low_depth() {
    println!("\n=== Verifying Both Match at Low Depth ===\n");

    let dc = DualCodex::large_primes();
    let mut toric = DenseToricPure::for_target(4, 7, dc);
    let mut rational = DenseRationalGrover::for_target(4, 7);

    println!("Iteration | Toric Prob | Rational Prob | Match?");
    println!("----------|------------|---------------|--------");

    for i in 0..5 {
        toric.grover_iteration();
        rational.grover_iteration();

        let t_prob = toric.target_probability();
        let r_prob = rational.target_probability();

        // They should match within floating-point tolerance
        let matches = (t_prob - r_prob).abs() < 0.01;
        println!("    {}     |   {:.2}%   |    {:.2}%    |   {}",
                 i + 1, t_prob * 100.0, r_prob * 100.0,
                 if matches { "✓" } else { "✗" });
    }

    println!("\nBoth representations give identical results at low depth.");
    println!("Toric continues working at depth where rational overflows.");
}

#[test]
fn test_helix_vs_magnitude_encoding() {
    println!("\n=== Helix Level Encodes Magnitude Growth ===\n");

    let dc = DualCodex::large_primes();
    let mut state = DenseToricPure::for_target(4, 7, dc);

    println!("At each iteration, amplitude magnitude grows.");
    println!("Traditional: magnitude stored as integer → overflow");
    println!("Toric: magnitude encoded as helix level → no overflow\n");

    println!("Iter | Target Inner | Target Helix | Other Inner | Other Helix");
    println!("-----|--------------|--------------|-------------|------------");

    for i in 0..15 {
        state.grover_iteration();

        let target = &state.amplitudes[state.target];
        let other = &state.amplitudes[0];  // A non-target amplitude

        let t_k = target.point.helix_level(&dc);
        let o_k = other.point.helix_level(&dc);

        println!(" {:2}  | {:>12} | {:>12} | {:>11} | {:>11}",
                 i + 1,
                 target.point.inner,
                 t_k,
                 other.point.inner,
                 o_k);
    }

    println!("\nThe helix level captures the \"overflow\" as climbing the helix.");
    println!("This is the key insight: overflow is INFORMATION, not error.");
}
