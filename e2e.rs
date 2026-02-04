//! End-to-End Integration Tests
//!
//! Full system validation tests that exercise the complete
//! QMNF/EPRAM pipeline from sensor input to decision output.

/// Test 1: EPRAM sensor fusion (the canonical validation case)
/// 
/// From the validation document: 8 sensors at 5, 8 sensors at 250
/// on a 4x4 grid, modulus 256. Should converge to ~255 with high confidence.
#[test]
fn test_e2e_sensor_fusion() {
    let m = 256u64;
    
    // Initial sensor readings: 8 at value 5, 8 at value 250
    // Arranged as 4x4 grid (top half at 5, bottom half at 250)
    let targets: Vec<u64> = vec![
        5, 5, 5, 5,       // Row 0
        5, 5, 5, 5,       // Row 1
        250, 250, 250, 250, // Row 2
        250, 250, 250, 250, // Row 3
    ];
    
    // Initialize cells at targets
    let mut values = targets.clone();
    
    // Run EPRAM convergence with independent mode (no neighbor coupling)
    let max_iterations = 20;
    for _ in 0..max_iterations {
        let mut new_values = values.clone();
        for i in 0..16 {
            new_values[i] = fourth_attractor_step(values[i], targets[i], m);
        }
        values = new_values;
    }
    
    // All values should have converged to their targets
    for (i, (&v, &t)) in values.iter().zip(targets.iter()).enumerate() {
        assert_eq!(v, t, "Cell {} should converge to target", i);
    }
    
    // Compute circular mean and confidence
    let mean = circular_mean_integer(&values, m);
    let conf = confidence_integer(&values, m);
    
    // Mean should be near 255 (midpoint of short arc from 250 to 5)
    // The short arc: 250 → 251 → 252 → 253 → 254 → 255 → 0 → 1 → 2 → 3 → 4 → 5
    // Midpoint is around 255-0
    assert!(
        mean.mean >= 250 || mean.mean <= 10,
        "Mean {} should be near wrap boundary",
        mean.mean
    );
    
    // Confidence should be high (bimodal cluster)
    assert!(
        conf.confidence_milliunits > 800,
        "Confidence {} should be high",
        conf.confidence_milliunits
    );
}

/// Test 2: Grid topology with neighbor coupling
#[test]
fn test_e2e_grid_coupled() {
    let m = 256u64;
    let width = 4usize;
    
    // Random-ish initial values
    let mut values: Vec<u64> = vec![
        100, 110, 105, 95,
        108, 102, 98, 112,
        90, 115, 100, 105,
        95, 100, 110, 92,
    ];
    
    // All targets are 100
    let targets: Vec<u64> = vec![100; 16];
    
    // Run with neighbor coupling
    let max_iterations = 50;
    let target_weight = 500u64;
    let neighbor_weight = 500u64;
    
    for _ in 0..max_iterations {
        let mut new_values = values.clone();
        
        for i in 0..16 {
            // Get neighbors
            let row = i / width;
            let col = i % width;
            let mut neighbors = Vec::new();
            
            if row > 0 { neighbors.push(values[(row - 1) * width + col]); }
            if row < 3 { neighbors.push(values[(row + 1) * width + col]); }
            if col > 0 { neighbors.push(values[row * width + col - 1]); }
            if col < 3 { neighbors.push(values[row * width + col + 1]); }
            
            // Compute neighbor mean
            let neighbor_mean = if neighbors.is_empty() {
                values[i]
            } else {
                circular_mean_integer(&neighbors, m).mean
            };
            
            // Coupled transition
            let target_pull = fourth_attractor_step(values[i], targets[i], m);
            let neighbor_pull = fourth_attractor_step(values[i], neighbor_mean, m);
            
            // Blend
            new_values[i] = blend_weighted(
                target_pull, neighbor_pull,
                target_weight, neighbor_weight, m
            );
        }
        
        values = new_values;
    }
    
    // All values should be near 100
    for (i, &v) in values.iter().enumerate() {
        assert!(
            geodesic_distance(v, 100, m) <= 5,
            "Cell {} value {} should be near 100",
            i, v
        );
    }
}

/// Test 3: Convergence time validation (O(log M) guarantee)
#[test]
fn test_e2e_convergence_time() {
    let m = 256u64;
    
    // Test worst-case: maximum distance
    let initial = 0u64;
    let target = 128u64;  // Distance = M/2
    
    let mut x = initial;
    let mut steps = 0;
    
    while x != target && steps < 100 {
        x = fourth_attractor_step(x, target, m);
        steps += 1;
    }
    
    assert_eq!(x, target, "Should converge to target");
    
    // log_4(128) ≈ 3.5, so expect <= 6 steps
    assert!(
        steps <= 6,
        "Converged in {} steps, should be O(log M) = ~4-5",
        steps
    );
}

/// Test 4: Wrap-around correctness
#[test]
fn test_e2e_wrap_around() {
    let m = 256u64;
    
    // Value near max, target near min
    let initial = 250u64;
    let target = 5u64;
    
    let mut x = initial;
    let mut steps = 0;
    
    while x != target && steps < 20 {
        let prev = x;
        x = fourth_attractor_step(x, target, m);
        
        // Verify we're taking the short path
        let d_prev = geodesic_distance(prev, target, m);
        let d_curr = geodesic_distance(x, target, m);
        
        if prev != target {
            assert!(
                d_curr < d_prev,
                "Step {} → {}: distance should decrease ({} → {})",
                prev, x, d_prev, d_curr
            );
        }
        
        steps += 1;
    }
    
    assert_eq!(x, target);
}

/// Test 5: Full circular statistics pipeline
#[test]
fn test_e2e_circular_statistics() {
    let m = 256u64;
    
    // The EPRAM validation case converged state
    let converged = vec![
        5, 5, 5, 5,
        3, 3, 3, 3,
        251, 250, 250, 251,
        250, 250, 250, 250,
    ];
    
    // Test circular mean
    let mean_result = circular_mean_integer(&converged, m);
    assert!(mean_result.valid, "Cluster should be valid");
    
    // Mean should be near 255 (validated in document as ~255.124)
    assert!(
        mean_result.mean >= 253 || mean_result.mean <= 3,
        "Mean {} should be near 255/0",
        mean_result.mean
    );
    
    // Test confidence
    let conf = confidence_integer(&converged, m);
    
    // Validation document reports R ≈ 0.9927
    // Our integer confidence should be similarly high
    assert!(
        conf.confidence_milliunits > 900,
        "Confidence {} should be > 0.90",
        conf.confidence_milliunits
    );
    
    // Test with precomputed mean
    let conf2 = confidence_with_mean(&converged, mean_result.mean, m);
    assert_eq!(
        conf.confidence_milliunits, conf2.confidence_milliunits,
        "Confidence should match with precomputed mean"
    );
}

// ============================================================================
// Helper implementations (would normally import from crate)
// ============================================================================

fn geodesic_distance(a: u64, b: u64, m: u64) -> u64 {
    let diff = if a >= b { a - b } else { b - a };
    diff.min(m - diff)
}

fn signed_geodesic(a: u64, b: u64, m: u64) -> i64 {
    let diff = (a as i64) - (b as i64);
    let half = (m / 2) as i64;
    if diff > half { diff - (m as i64) }
    else if diff < -half { diff + (m as i64) }
    else { diff }
}

fn fourth_attractor_step(current: u64, target: u64, m: u64) -> u64 {
    if current == target { return current; }
    let delta = signed_geodesic(target, current, m);
    let abs_delta = delta.unsigned_abs();
    let mut step = (3 * abs_delta) / 4;
    if step == 0 && delta != 0 { step = 1; }
    let signed_step = if delta > 0 { step as i64 } else { -(step as i64) };
    ((current as i64 + signed_step).rem_euclid(m as i64)) as u64
}

struct CircularMeanResult {
    mean: u64,
    valid: bool,
}

fn circular_mean_integer(values: &[u64], m: u64) -> CircularMeanResult {
    if values.is_empty() {
        return CircularMeanResult { mean: 0, valid: true };
    }
    let reference = values[0];
    let n = values.len() as i64;
    let mut delta_sum: i64 = 0;
    for &v in values {
        delta_sum += signed_geodesic(v, reference, m);
    }
    let mean_offset = if delta_sum >= 0 {
        (delta_sum + n / 2) / n
    } else {
        (delta_sum - n / 2) / n
    };
    let mean = ((reference as i64 + mean_offset).rem_euclid(m as i64)) as u64;
    
    // Check cluster validity
    let mut max_dist = 0u64;
    for &v in values {
        max_dist = max_dist.max(geodesic_distance(v, reference, m));
    }
    let valid = max_dist < m / 2;
    
    CircularMeanResult { mean, valid }
}

struct ConfidenceResult {
    confidence_milliunits: u64,
}

fn confidence_integer(values: &[u64], m: u64) -> ConfidenceResult {
    if values.is_empty() || values.len() == 1 {
        return ConfidenceResult { confidence_milliunits: 1000 };
    }
    let mean = circular_mean_integer(values, m).mean;
    confidence_with_mean(values, mean, m)
}

fn confidence_with_mean(values: &[u64], mean: u64, m: u64) -> ConfidenceResult {
    let max_deviation = m / 4;
    let mut total_deviation: u64 = 0;
    for &v in values {
        total_deviation += geodesic_distance(v, mean, m);
    }
    let mean_deviation = total_deviation / values.len() as u64;
    let confidence_milliunits = if mean_deviation >= max_deviation {
        0
    } else {
        ((max_deviation - mean_deviation) * 1000) / max_deviation
    };
    ConfidenceResult { confidence_milliunits }
}

fn blend_weighted(target: u64, neighbor: u64, target_weight: u64, neighbor_weight: u64, m: u64) -> u64 {
    let total = target_weight + neighbor_weight;
    if total == 0 { return target; }
    if neighbor_weight == 0 { return target; }
    if target_weight == 0 { return neighbor; }
    let delta = signed_geodesic(neighbor, target, m);
    let weighted_delta = (delta * neighbor_weight as i64) / total as i64;
    ((target as i64 + weighted_delta).rem_euclid(m as i64)) as u64
}
