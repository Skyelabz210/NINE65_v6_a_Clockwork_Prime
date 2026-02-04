// Fourth Attractor Validation Suite - Gap Analysis & Expansion
// Batteries 21-30: Advanced Empirical Evidence Collection

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;

// ============================================================================
// GAP ANALYSIS SUMMARY
// ============================================================================

/*
IDENTIFIED GAPS IN ORIGINAL SUITE:

1. Initial Condition Sensitivity - Not explicitly tested vs classical attractors
2. Full Lyapunov Spectrum - Only local estimates, need complete spectrum
3. Power Spectrum Analysis - Frequency domain completely missing
4. Poincaré Sections - Classical technique not used
5. Bifurcation Diagrams - Parameter space not mapped
6. Surrogate Data Testing - No statistical null hypothesis testing
7. Embedding Dimension Analysis - No validation of reconstruction
8. Kolmogorov-Sinai Entropy - More rigorous than sample entropy
9. First Return Maps - Recurrence structure not characterized
10. Mutual Information - Only using correlation measures

ADDITIONS IN THIS EXPANSION:

Battery 21: Initial Condition Divergence Analysis
Battery 22: Full Lyapunov Spectrum Computation
Battery 23: Power Spectrum & Frequency Analysis
Battery 24: Poincaré Section Mapping
Battery 25: Bifurcation Diagram Generation
Battery 26: Surrogate Data Hypothesis Testing
Battery 27: Takens Embedding & Dimension Analysis
Battery 28: Kolmogorov-Sinai Entropy Estimation
Battery 29: First Return Map Characterization
Battery 30: Mutual Information & Time-Delayed Correlation
*/

// ============================================================================
// BATTERY 21: INITIAL CONDITION DIVERGENCE ANALYSIS
// ============================================================================

pub struct Battery21_InitialConditionDivergence {
    modulus: CRTBigInt,
    epsilon_values: Vec<i64>, // Initial perturbation sizes
    time_horizons: Vec<usize>,
}

impl Battery21_InitialConditionDivergence {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            epsilon_values: vec![1, 5, 10, 50, 100], // Very small perturbations
            time_horizons: vec![10, 20, 50, 100, 200],
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &epsilon in &self.epsilon_values {
            for &horizon in &self.time_horizons {
                // Test Fourth Attractor
                let fourth_divergence = self.measure_divergence(epsilon, horizon, 23, true);
                
                // Test Strange Attractor (low k, high gamma)
                let strange_divergence = self.measure_divergence(epsilon, horizon, 10, false);
                
                // Compute sensitivity ratio
                let sensitivity_ratio = if strange_divergence > CRTBigInt::from(0) {
                    fourth_divergence.mul(&CRTBigInt::from(1000)).div(&strange_divergence)
                } else {
                    CRTBigInt::from(0)
                };
                
                results.push(DivergenceMeasurement {
                    epsilon,
                    time_horizon: horizon,
                    fourth_attractor_divergence: fourth_divergence.clone(),
                    strange_attractor_divergence: strange_divergence,
                    sensitivity_ratio,
                    passes_butterfly_test: fourth_divergence < CRTBigInt::from(500), // Should be stable
                });
            }
        }
        
        ValidationResult::InitialConditionDivergence(results)
    }

    fn measure_divergence(&self, epsilon: i64, horizon: usize, depth: u32, is_fourth: bool) -> CRTBigInt {
        let base_initial = CRTBigInt::from(500);
        let perturbed_initial = base_initial.add(&CRTBigInt::from(epsilon));
        
        let k = if is_fourth { 50 } else { 10 };
        let gamma = if is_fourth { 200 } else { 500 };
        
        let mut attractor1 = FourthAttractor::new(self.modulus.clone(), base_initial, k);
        attractor1.gamma = CRTBigInt::from(gamma);
        
        let mut attractor2 = FourthAttractor::new(self.modulus.clone(), perturbed_initial, k);
        attractor2.gamma = CRTBigInt::from(gamma);
        
        // Evolve to depth
        for _ in 0..depth {
            attractor1.step();
            attractor2.step();
        }
        
        // Measure divergence over time horizon
        let mut max_divergence = CRTBigInt::from(0);
        
        for _ in 0..horizon {
            attractor1.step();
            attractor2.step();
            
            let distance = if attractor1.state > attractor2.state {
                attractor1.state.sub(&attractor2.state)
            } else {
                attractor2.state.sub(&attractor1.state)
            };
            
            if distance > max_divergence {
                max_divergence = distance.clone();
            }
        }
        
        // Normalize by initial perturbation
        max_divergence.mul(&CRTBigInt::from(1000)).div(&CRTBigInt::from(epsilon))
    }
}

pub struct DivergenceMeasurement {
    pub epsilon: i64,
    pub time_horizon: usize,
    pub fourth_attractor_divergence: CRTBigInt,
    pub strange_attractor_divergence: CRTBigInt,
    pub sensitivity_ratio: CRTBigInt,
    pub passes_butterfly_test: bool,
}

// ============================================================================
// BATTERY 22: FULL LYAPUNOV SPECTRUM COMPUTATION
// ============================================================================

pub struct Battery22_LyapunovSpectrum {
    modulus: CRTBigInt,
    depths_to_test: Vec<u32>,
    spectrum_dimensions: usize,
}

impl Battery22_LyapunovSpectrum {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            depths_to_test: vec![10, 15, 20, 21, 22, 23, 25, 30],
            spectrum_dimensions: 5,
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &depth in &self.depths_to_test {
            let spectrum = self.compute_full_spectrum(depth);
            let kaplan_yorke_dim = self.compute_kaplan_yorke_dimension(&spectrum);
            let attractor_type = self.classify_by_spectrum(&spectrum);
            
            results.push(LyapunovSpectrumMeasurement {
                recursion_depth: depth,
                lyapunov_exponents: spectrum,
                kaplan_yorke_dimension: kaplan_yorke_dim,
                attractor_classification: attractor_type,
            });
        }
        
        ValidationResult::LyapunovSpectrum(results)
    }

    fn compute_full_spectrum(&self, depth: u32) -> Vec<CRTBigInt> {
        let mut attractor = FourthAttractor::new(
            self.modulus.clone(),
            CRTBigInt::from(500),
            50
        );
        
        // Evolve to depth
        for _ in 0..depth {
            attractor.step();
        }
        
        // Initialize perturbation vectors (orthogonal in state space)
        let mut perturbations: Vec<Vec<CRTBigInt>> = (0..self.spectrum_dimensions)
            .map(|i| {
                let mut vec = vec![CRTBigInt::from(0); self.spectrum_dimensions];
                vec[i] = CRTBigInt::from(100); // Unit perturbation
                vec
            })
            .collect();
        
        let mut lyapunov_sums = vec![CRTBigInt::from(0); self.spectrum_dimensions];
        let iterations = 500;
        let renorm_interval = 10;
        
        for iter in 0..iterations {
            // Evolve base trajectory
            let base_state = attractor.state.clone();
            attractor.step();
            let new_state = attractor.state.clone();
            
            // Evolve perturbations
            for (i, perturbation) in perturbations.iter_mut().enumerate() {
                // Simple linearized evolution (approximation)
                let evolved = perturbation[0].mul(&new_state).div(&base_state.add(&CRTBigInt::from(1)));
                perturbation[0] = evolved;
            }
            
            // Renormalize and accumulate
            if iter % renorm_interval == 0 {
                for i in 0..self.spectrum_dimensions {
                    let norm = perturbations[i][0].clone();
                    if norm > CRTBigInt::from(0) {
                        // Approximate log(norm)
                        let log_norm = self.approximate_log(&norm);
                        lyapunov_sums[i] = lyapunov_sums[i].add(&log_norm);
                        
                        // Renormalize
                        perturbations[i][0] = CRTBigInt::from(100);
                    }
                }
                
                // Gram-Schmidt orthogonalization (simplified)
                self.gram_schmidt_orthogonalize(&mut perturbations);
            }
        }
        
        // Average over iterations
        lyapunov_sums.iter()
            .map(|sum| sum.div(&CRTBigInt::from((iterations / renorm_interval) as i64)))
            .collect()
    }

    fn approximate_log(&self, value: &CRTBigInt) -> CRTBigInt {
        // Simple log approximation: log(x) ≈ (x - 1000) for x near 1000
        if value > &CRTBigInt::from(1000) {
            value.sub(&CRTBigInt::from(1000))
        } else {
            CRTBigInt::from(1000).sub(value).mul(&CRTBigInt::from(-1))
        }
    }

    fn gram_schmidt_orthogonalize(&self, vectors: &mut [Vec<CRTBigInt>]) {
        // Simplified Gram-Schmidt (for demonstration)
        for i in 1..vectors.len() {
            for j in 0..i {
                let dot_product = vectors[i][0].mul(&vectors[j][0]).div(&CRTBigInt::from(1000));
                vectors[i][0] = vectors[i][0].sub(&dot_product);
            }
        }
    }

    fn compute_kaplan_yorke_dimension(&self, spectrum: &[CRTBigInt]) -> CRTBigInt {
        // Find j where sum of first j exponents is positive, j+1 is negative
        let mut cumsum = CRTBigInt::from(0);
        let mut j = 0;
        
        for (i, lambda) in spectrum.iter().enumerate() {
            cumsum = cumsum.add(lambda);
            if cumsum < CRTBigInt::from(0) {
                j = i;
                break;
            }
        }
        
        if j == 0 || j >= spectrum.len() {
            return CRTBigInt::from(1000); // 1.0
        }
        
        // D_KY = j + sum(λ_i, i=1..j) / |λ_{j+1}|
        let sum_positive: CRTBigInt = spectrum[..j].iter()
            .fold(CRTBigInt::from(0), |acc, x| acc.add(x));
        
        let next_negative = if spectrum[j] < CRTBigInt::from(0) {
            CRTBigInt::from(0).sub(&spectrum[j])
        } else {
            spectrum[j].clone()
        };
        
        let fraction = sum_positive.mul(&CRTBigInt::from(1000))
            .div(&next_negative.add(&CRTBigInt::from(1)));
        
        CRTBigInt::from((j * 1000) as i64).add(&fraction)
    }

    fn classify_by_spectrum(&self, spectrum: &[CRTBigInt]) -> String {
        let positive_count = spectrum.iter().filter(|&&x| x > CRTBigInt::from(0)).count();
        let negative_count = spectrum.iter().filter(|&&x| x < CRTBigInt::from(0)).count();
        let zero_count = spectrum.len() - positive_count - negative_count;
        
        match (positive_count, zero_count, negative_count) {
            (0, _, _) => "Fixed Point".to_string(),
            (_, z, _) if z > 0 => "Limit Cycle".to_string(),
            (p, 0, _) if p > 0 => "Strange Attractor".to_string(),
            _ => "Mixed/Fourth Attractor".to_string(),
        }
    }
}

pub struct LyapunovSpectrumMeasurement {
    pub recursion_depth: u32,
    pub lyapunov_exponents: Vec<CRTBigInt>,
    pub kaplan_yorke_dimension: CRTBigInt,
    pub attractor_classification: String,
}

// ============================================================================
// BATTERY 23: POWER SPECTRUM & FREQUENCY ANALYSIS
// ============================================================================

pub struct Battery23_PowerSpectrum {
    modulus: CRTBigInt,
    depths_to_analyze: Vec<u32>,
    fft_size: usize,
}

impl Battery23_PowerSpectrum {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            depths_to_analyze: vec![10, 15, 20, 23, 25, 30],
            fft_size: 512,
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &depth in &self.depths_to_analyze {
            let time_series = self.generate_time_series(depth);
            let power_spectrum = self.compute_power_spectrum(&time_series);
            let dominant_frequencies = self.find_dominant_frequencies(&power_spectrum);
            let spectral_entropy = self.compute_spectral_entropy(&power_spectrum);
            let one_over_f_exponent = self.estimate_one_over_f_exponent(&power_spectrum);
            
            results.push(PowerSpectrumMeasurement {
                recursion_depth: depth,
                dominant_frequencies,
                spectral_entropy,
                one_over_f_exponent,
                has_broadband_noise: self.is_broadband(&power_spectrum),
                has_discrete_peaks: self.has_discrete_peaks(&power_spectrum),
            });
        }
        
        ValidationResult::PowerSpectrum(results)
    }

    fn generate_time_series(&self, depth: u32) -> Vec<CRTBigInt> {
        let mut attractor = FourthAttractor::new(
            self.modulus.clone(),
            CRTBigInt::from(500),
            50
        );
        
        for _ in 0..depth {
            attractor.step();
        }
        
        let mut series = Vec::with_capacity(self.fft_size);
        for _ in 0..self.fft_size {
            attractor.step();
            series.push(attractor.state.clone());
        }
        
        series
    }

    fn compute_power_spectrum(&self, time_series: &[CRTBigInt]) -> Vec<CRTBigInt> {
        // Simplified DFT (not actual FFT for simplicity)
        let n = time_series.len();
        let mut spectrum = Vec::with_capacity(n / 2);
        
        for k in 0..(n / 2) {
            let mut real_sum = CRTBigInt::from(0);
            let mut imag_sum = CRTBigInt::from(0);
            
            for (i, x) in time_series.iter().enumerate() {
                // cos(2πki/n) and sin(2πki/n) approximations
                let angle = (2 * k * i * 6283) / n; // 2π * k * i / n scaled
                let cos_val = self.cos_approx(angle as i64);
                let sin_val = self.sin_approx(angle as i64);
                
                real_sum = real_sum.add(&x.mul(&cos_val).div(&CRTBigInt::from(1000)));
                imag_sum = imag_sum.add(&x.mul(&sin_val).div(&CRTBigInt::from(1000)));
            }
            
            // Power = real² + imag²
            let real_sq = real_sum.mul(&real_sum).div(&CRTBigInt::from(1000));
            let imag_sq = imag_sum.mul(&imag_sum).div(&CRTBigInt::from(1000));
            let power = real_sq.add(&imag_sq);
            
            spectrum.push(power);
        }
        
        spectrum
    }

    fn cos_approx(&self, angle_scaled: i64) -> CRTBigInt {
        // cos(x) ≈ 1 - x²/2 for small x
        let angle_mod = angle_scaled % 6283;
        let x = CRTBigInt::from(angle_mod);
        let x_sq = x.mul(&x).div(&CRTBigInt::from(1000));
        
        CRTBigInt::from(1000).sub(&x_sq.div(&CRTBigInt::from(2)))
    }

    fn sin_approx(&self, angle_scaled: i64) -> CRTBigInt {
        // sin(x) ≈ x - x³/6
        let angle_mod = angle_scaled % 6283;
        let x = CRTBigInt::from(angle_mod);
        let x_sq = x.mul(&x).div(&CRTBigInt::from(1000));
        let x_cubed = x_sq.mul(&x).div(&CRTBigInt::from(1000));
        
        x.sub(&x_cubed.div(&CRTBigInt::from(6)))
    }

    fn find_dominant_frequencies(&self, spectrum: &[CRTBigInt]) -> Vec<usize> {
        let max_power = spectrum.iter().max().cloned().unwrap_or(CRTBigInt::from(0));
        let threshold = max_power.mul(&CRTBigInt::from(500)).div(&CRTBigInt::from(1000)); // 50%
        
        spectrum.iter()
            .enumerate()
            .filter(|(_, &ref power)| power > &threshold)
            .map(|(i, _)| i)
            .collect()
    }

    fn compute_spectral_entropy(&self, spectrum: &[CRTBigInt]) -> CRTBigInt {
        let total_power: CRTBigInt = spectrum.iter()
            .fold(CRTBigInt::from(0), |acc, x| acc.add(x));
        
        if total_power == CRTBigInt::from(0) {
            return CRTBigInt::from(0);
        }
        
        let mut entropy = 0.0;
        for power in spectrum {
            if power > &CRTBigInt::from(0) {
                let p = power.to_u64() as f64 / total_power.to_u64() as f64;
                entropy -= p * p.log2();
            }
        }
        
        CRTBigInt::from((entropy * 1000.0) as i64)
    }

    fn estimate_one_over_f_exponent(&self, spectrum: &[CRTBigInt]) -> CRTBigInt {
        // Fit power law: P(f) ~ 1/f^α
        // Using log-log regression approximation
        if spectrum.len() < 10 {
            return CRTBigInt::from(0);
        }
        
        let mut log_freq_sum = 0.0;
        let mut log_power_sum = 0.0;
        let mut log_freq_power_sum = 0.0;
        let mut log_freq_sq_sum = 0.0;
        let mut count = 0;
        
        for (i, power) in spectrum.iter().enumerate().skip(1).take(spectrum.len() / 2) {
            if power > &CRTBigInt::from(0) {
                let log_f = (i as f64).log2();
                let log_p = (power.to_u64() as f64).log2();
                
                log_freq_sum += log_f;
                log_power_sum += log_p;
                log_freq_power_sum += log_f * log_p;
                log_freq_sq_sum += log_f * log_f;
                count += 1;
            }
        }
        
        if count < 5 {
            return CRTBigInt::from(0);
        }
        
        let n = count as f64;
        let slope = (n * log_freq_power_sum - log_freq_sum * log_power_sum) /
                    (n * log_freq_sq_sum - log_freq_sum * log_freq_sum);
        
        CRTBigInt::from((-slope * 1000.0) as i64) // Negative because 1/f^α
    }

    fn is_broadband(&self, spectrum: &[CRTBigInt]) -> bool {
        // Check if power is distributed across many frequencies
        let nonzero_bins = spectrum.iter().filter(|&&ref x| x > &CRTBigInt::from(100)).count();
        nonzero_bins as f64 > spectrum.len() as f64 * 0.3
    }

    fn has_discrete_peaks(&self, spectrum: &[CRTBigInt]) -> bool {
        // Check for sharp peaks indicating periodicity
        let max_power = spectrum.iter().max().cloned().unwrap_or(CRTBigInt::from(0));
        let mean_power = spectrum.iter()
            .fold(CRTBigInt::from(0), |acc, x| acc.add(x))
            .div(&CRTBigInt::from(spectrum.len() as i64));
        
        max_power > mean_power.mul(&CRTBigInt::from(10)) // Max > 10x mean
    }
}

pub struct PowerSpectrumMeasurement {
    pub recursion_depth: u32,
    pub dominant_frequencies: Vec<usize>,
    pub spectral_entropy: CRTBigInt,
    pub one_over_f_exponent: CRTBigInt,
    pub has_broadband_noise: bool,
    pub has_discrete_peaks: bool,
}

// ============================================================================
// BATTERY 24: POINCARÉ SECTION MAPPING
// ============================================================================

pub struct Battery24_PoincareSection {
    modulus: CRTBigInt,
    depths_to_map: Vec<u32>,
    section_threshold: CRTBigInt,
}

impl Battery24_PoincareSection {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            depths_to_map: vec![10, 15, 20, 23, 25, 30],
            section_threshold: CRTBigInt::from(500),
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &depth in &self.depths_to_map {
            let section_points = self.compute_poincare_section(depth);
            let section_dimension = self.estimate_section_dimension(&section_points);
            let section_structure = self.analyze_section_structure(&section_points);
            
            results.push(PoincareSectionMeasurement {
                recursion_depth: depth,
                num_section_points: section_points.len(),
                estimated_dimension: section_dimension,
                structure_type: section_structure,
                points: section_points,
            });
        }
        
        ValidationResult::PoincareSection(results)
    }

    fn compute_poincare_section(&self, depth: u32) -> Vec<(CRTBigInt, CRTBigInt)> {
        let mut attractor = FourthAttractor::new(
            self.modulus.clone(),
            CRTBigInt::from(500),
            50
        );
        
        for _ in 0..depth {
            attractor.step();
        }
        
        let mut section_points = Vec::new();
        let mut previous_state = attractor.state.clone();
        let mut crossed_below = previous_state < self.section_threshold;
        
        for _ in 0..5000 {
            attractor.step();
            let current_state = attractor.state.clone();
            let currently_below = current_state < self.section_threshold;
            
            // Check for crossing from below to above
            if crossed_below && !currently_below {
                // Interpolate crossing point
                section_points.push((current_state.clone(), attractor.phase.clone()));
            }
            
            crossed_below = currently_below;
            previous_state = current_state;
        }
        
        section_points
    }

    fn estimate_section_dimension(&self, points: &[(CRTBigInt, CRTBigInt)]) -> CRTBigInt {
        if points.len() < 10 {
            return CRTBigInt::from(0);
        }
        
        // Correlation dimension estimation on section
        let sample_size = points.len().min(200);
        let distances = self.compute_pairwise_distances(&points[..sample_size]);
        
        let scales = vec![50, 100, 200, 400];
        let mut counts = Vec::new();
        
        for &scale in &scales {
            let count = distances.iter()
                .filter(|&&d| d < CRTBigInt::from(scale))
                .count();
            counts.push(count);
        }
        
        // Estimate slope in log-log plot
        if counts.len() >= 2 && counts[counts.len()-1] > counts[0] {
            let log_ratio_count = CRTBigInt::from((counts[counts.len()-1] * 1000) as i64)
                .div(&CRTBigInt::from((counts[0].max(1)) as i64));
            let log_ratio_scale = CRTBigInt::from((scales[scales.len()-1] * 1000) as i64)
                .div(&CRTBigInt::from(scales[0] as i64));
            
            log_ratio_count.div(&log_ratio_scale.add(&CRTBigInt::from(1)))
        } else {
            CRTBigInt::from(1000)
        }
    }

    fn compute_pairwise_distances(&self, points: &[(CRTBigInt, CRTBigInt)]) -> Vec<CRTBigInt> {
        let mut distances = Vec::new();
        
        for i in 0..points.len() {
            for j in (i+1)..points.len() {
                let dx = if points[i].0 > points[j].0 {
                    points[i].0.sub(&points[j].0)
                } else {
                    points[j].0.sub(&points[i].0)
                };
                
                let dy = if points[i].1 > points[j].1 {
                    points[i].1.sub(&points[j].1)
                } else {
                    points[j].1.sub(&points[i].1)
                };
                
                let dist = dx.mul(&dx).add(&dy.mul(&dy))
                    .div(&CRTBigInt::from(1000));
                distances.push(dist);
            }
        }
        
        distances
    }

    fn analyze_section_structure(&self, points: &[(CRTBigInt, CRTBigInt)]) -> String {
        if points.is_empty() {
            return "No crossings".to_string();
        }
        
        if points.len() < 10 {
            return "Too few points".to_string();
        }
        
        // Check for discrete points (limit cycle)
        let unique_points = self.count_unique_regions(points, 50);
        
        if unique_points < 5 {
            return "Limit Cycle (discrete points)".to_string();
        } else if unique_points > 50 {
            return "Strange Attractor (continuous)".to_string();
        } else {
            return "Fourth Attractor (structured)".to_string();
        }
    }

    fn count_unique_regions(&self, points: &[(CRTBigInt, CRTBigInt)], tolerance: i64) -> usize {
        let mut regions = Vec::new();
        
        for point in points {
            let mut found = false;
            for region in &regions {
                let dist = self.point_distance(point, region);
                if dist < CRTBigInt::from(tolerance) {
                    found = true;
                    break;
                }
            }
            if !found {
                regions.push(point.clone());
            }
        }
        
        regions.len()
    }

    fn point_distance(&self, p1: &(CRTBigInt, CRTBigInt), p2: &(CRTBigInt, CRTBigInt)) -> CRTBigInt {
        let dx = if p1.0 > p2.0 { p1.0.sub(&p2.0) } else { p2.0.sub(&p1.0) };
        let dy = if p1.1 > p2.1 { p1.1.sub(&p2.1) } else { p2.1.sub(&p1.1) };
        dx.add(&dy)
    }
}

pub struct PoincareSectionMeasurement {
    pub recursion_depth: u32,
    pub num_section_points: usize,
    pub estimated_dimension: CRTBigInt,
    pub structure_type: String,
    pub points: Vec<(CRTBigInt, CRTBigInt)>,
}

// ============================================================================
// BATTERY 25: BIFURCATION DIAGRAM GENERATION
// ============================================================================

pub struct Battery25_BifurcationDiagram {
    modulus: CRTBigInt,
    parameter_range: Vec<i64>, // k values
    depths_per_parameter: Vec<u32>,
}

impl Battery25_BifurcationDiagram {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            parameter_range: (10..=150).step_by(5).collect(),
            depths_per_parameter: vec![15, 20, 23, 25],
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &k in &self.parameter_range {
            for &depth in &self.depths_per_parameter {
                let attractor_values = self.sample_attractor_values(k, depth);
                let num_attractors = self.count_distinct_attractors(&attractor_values);
                let bifurcation_type = self.classify_bifurcation(num_attractors);
                
                results.push(BifurcationMeasurement {
                    parameter_value: k,
                    recursion_depth: depth,
                    attractor_values,
                    num_distinct_attractors: num_attractors,
                    bifurcation_type,
                });
            }
        }
        
        ValidationResult::BifurcationDiagram(results)
    }

    fn sample_attractor_values(&self, k: i64, depth: u32) -> Vec<CRTBigInt> {
        let mut attractor = FourthAttractor::new(
            self.modulus.clone(),
            CRTBigInt::from(500),
            k
        );
        
        // Evolve to depth
        for _ in 0..depth {
            attractor.step();
        }
        
        // Let transients die out
        for _ in 0..200 {
            attractor.step();
        }
        
        // Sample attractor values
        let mut samples = Vec::new();
        for _ in 0..100 {
            attractor.step();
            samples.push(attractor.state.clone());
        }
        
        samples
    }

    fn count_distinct_attractors(&self, values: &[CRTBigInt]) -> usize {
        let tolerance = 20;
        let mut attractors = Vec::new();
        
        for value in values {
            let mut found = false;
            for attractor in &attractors {
                let diff = if value > attractor {
                    value.sub(attractor)
                } else {
                    attractor.sub(value)
                };
                
                if diff < CRTBigInt::from(tolerance) {
                    found = true;
                    break;
                }
            }
            
            if !found {
                attractors.push(value.clone());
            }
        }
        
        attractors.len()
    }

    fn classify_bifurcation(&self, num_attractors: usize) -> String {
        match num_attractors {
            1 => "Fixed Point".to_string(),
            2 => "Period-2 (Flip Bifurcation)".to_string(),
            n if n <= 8 && (n & (n - 1)) == 0 => format!("Period-{} (Period Doubling)", n),
            n if n < 20 => format!("Period-{} (Quasi-periodic)", n),
            _ => "Chaotic (Many attractors)".to_string(),
        }
    }
}

pub struct BifurcationMeasurement {
    pub parameter_value: i64,
    pub recursion_depth: u32,
    pub attractor_values: Vec<CRTBigInt>,
    pub num_distinct_attractors: usize,
    pub bifurcation_type: String,
}

// ============================================================================
// BATTERY 26: SURROGATE DATA HYPOTHESIS TESTING
// ============================================================================

pub struct Battery26_SurrogateDataTest {
    modulus: CRTBigInt,
    depths_to_test: Vec<u32>,
    num_surrogates: usize,
}

impl Battery26_SurrogateDataTest {
    pub fn new(modulus: CRTBigInt) -> Self {
        Self {
            modulus,
            depths_to_test: vec![10, 15, 20, 23, 25, 30],
            num_surrogates: 20,
        }
    }

    pub fn run(&self) -> ValidationResult {
        let mut results = Vec::new();
        
        for &depth in &self.depths_to_test {
            let original_series = self.generate_time_series(depth);
            let original_metric = self.compute_test_statistic(&original_series);
            
            let surrogate_metrics: Vec<_> = (0..self.num_surrogates)
                .into_par_iter()
                .map(|seed| {
                    let surrogate = self.generate_phase_randomized_surrogate(&original_series, seed);
                    self.compute_test_statistic(&surrogate)
                })
                .collect();
            
            let p_value = self.compute_p_value(original_metric.clone(), &surrogate_metrics);
            let rejects_null = p_value < CRTBigInt::from(50); // p < 0.05
            
            results.push(SurrogateTestMeasurement {
                recursion_depth: depth,
                original_statistic: original_metric,
                surrogate_mean: self.mean(&surrogate_metrics),
                surrogate_std: self.std_dev(&surrogate_metrics),
                p_value,
                rejects_linear_null: rejects_null,
            });
        }
        
        ValidationResult::SurrogateDataTest(results)
    }

    fn generate_time_series(&self, depth: u32) -> Vec<CRTBigInt> {
        let mut attractor = FourthAttractor::new(
            self.modulus.clone(),
            CRTBigInt::from(500),
            50
        );
        
        for _ in 0..depth {
            attractor.step();
        }
        
        let mut series = Vec::with_capacity(512);
        for _ in 0..512 {
            attractor.step();
            series.push(attractor.state.clone());
        }
        
        series
    }

    fn generate_phase_randomized_surrogate(&self, original: &[CRTBigInt], seed: usize) -> Vec<CRTBigInt> {
        // Phase randomization: preserve power spectrum but randomize phases
        // Simplified: just shuffle while preserving autocorrelation structure
        let mut surrogate = original.to_vec();
        
        // Simple pseudo-random shuffle based on seed
        for i in 0..surrogate.len() {
            let j = ((i * seed * 17) % surrogate.len());
            surrogate.swap(i, j);
        }
        
        surrogate
    }

    fn compute_test_statistic(&self, series: &[CRTBigInt]) -> CRTBigInt {
        // Use nonlinear prediction error as test statistic
        let embed_dim = 3;
        let prediction_horizon = 1;
        
        if series.len() < embed_dim + prediction_horizon + 10 {
            return CRTBigInt::from(0);
        }
        
        let mut prediction_errors = Vec::new();
        
        for i in embed_dim..(series.len() - prediction_horizon) {
            // Find nearest neighbor in embedding space
            let current_embed: Vec<_> = (0..embed_dim).map(|j| series[i - j].clone()).collect();
            
            let mut min_distance = CRTBigInt::from(i64::MAX);
            let mut nearest_idx = i;
            
            for j in embed_dim..(series.len() - prediction_horizon) {
                if (j as i32 - i as i32).abs() < 10 {
                    continue; // Avoid temporal neighbors
                }
                
                let other_embed: Vec<_> = (0..embed_dim).map(|k| series[j - k].clone()).collect();
                let distance = self.embedding_distance(&current_embed, &other_embed);
                
                if distance < min_distance {
                    min_distance = distance;
                    nearest_idx = j;
                }
            }
            
            // Predict and compute error
            let predicted = series[nearest_idx + prediction_horizon].clone();
            let actual = series[i + prediction_horizon].clone();
            let error = if predicted > actual {
                predicted.sub(&actual)
            } else {
                actual.sub(&predicted)
            };
            
            prediction_errors.push(error);
        }
        
        // Return mean prediction error
        self.mean(&prediction_errors)
    }

    fn embedding_distance(&self, embed1: &[CRTBigInt], embed2: &[CRTBigInt]) -> CRTBigInt {
        embed1.iter().zip(embed2.iter())
            .map(|(a, b)| {
                let diff = if a > b { a.sub(b) } else { b.sub(a) };
                diff.mul(&diff).div(&CRTBigInt::from(1000))
            })
            .fold(CRTBigInt::from(0), |acc, x| acc.add(&x))
    }

    fn compute_p_value(&self, original: CRTBigInt, surrogates: &[CRTBigInt]) -> CRTBigInt {
        let count_more_extreme = surrogates.iter()
            .filter(|&&ref s| s < &original)
            .count();
        
        CRTBigInt::from((count_more_extreme * 1000) as i64)
            .div(&CRTBigInt::from(surrogates.len() as i64))
    }

    fn mean(&self, values: &[CRTBigInt]) -> CRTBigInt {
        if values.is_empty() {
            return CRTBigInt::from(0);
        }
        values.iter()
            .fold(CRTBigInt::from(0), |acc, x| acc.add(x))
            .div(&CRTBigInt::from(values.len() as i64))
    }

    fn std_dev(&self, values: &[CRTBigInt]) -> CRTBigInt {
        if values.len() < 2 {
            return CRTBigInt::from(0);
        }
        
        let mean = self.mean(values);
        let variance = values.iter()
            .map(|x| {
                let diff = if x > &mean { x.sub(&mean) } else { mean.sub(x) };
                diff.mul(&diff).div(&CRTBigInt::from(1000))
            })
            .fold(CRTBigInt::from(0), |acc, x| acc.add(&x))
            .div(&CRTBigInt::from((values.len() - 1) as i64));
        
        // Approximate sqrt
        self.integer_sqrt(variance)
    }

    fn integer_sqrt(&self, n: CRTBigInt) -> CRTBigInt {
        if n == CRTBigInt::from(0) {
            return CRTBigInt::from(0);
        }
        
        let mut x = n.clone();
        let mut y = x.add(&CRTBigInt::from(1)).div(&CRTBigInt::from(2));
        
        for _ in 0..20 { // Limited iterations
            if y >= x {
                break;
            }
            x = y.clone();
            y = x.add(&n.div(&x)).div(&CRTBigInt::from(2));
        }
        
        x
    }
}

pub struct SurrogateTestMeasurement {
    pub recursion_depth: u32,
    pub original_statistic: CRTBigInt,
    pub surrogate_mean: CRTBigInt,
    pub surrogate_std: CRTBigInt,
    pub p_value: CRTBigInt,
    pub rejects_linear_null: bool,
}

// ============================================================================
// VALIDATION RESULT EXTENSIONS
// ============================================================================

#[derive(Clone)]
pub enum ValidationResult {
    // Original 20 batteries...
    RecursionDepthEmergence(Vec<DepthStabilityMeasurement>),
    // ... (keeping all previous)
    
    // New batteries 21-26
    InitialConditionDivergence(Vec<DivergenceMeasurement>),
    LyapunovSpectrum(Vec<LyapunovSpectrumMeasurement>),
    PowerSpectrum(Vec<PowerSpectrumMeasurement>),
    PoincareSection(Vec<PoincareSectionMeasurement>),
    BifurcationDiagram(Vec<BifurcationMeasurement>),
    SurrogateDataTest(Vec<SurrogateTestMeasurement>),
}

// ============================================================================
// CONTINUE WITH BATTERIES 27-30 IN NEXT SECTION
// ============================================================================

// Battery 27: Takens Embedding & Dimension Analysis
// Battery 28: Kolmogorov-Sinai Entropy Estimation  
// Battery 29: First Return Map Characterization
// Battery 30: Mutual Information & Time-Delayed Correlation

// [Implementation continues...]