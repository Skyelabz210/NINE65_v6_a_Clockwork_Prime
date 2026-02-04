//! T-502: QMNF/EPRAM Performance Benchmark Suite
//! 
//! Criterion-based benchmarks for all core operations.
//! Run with: cargo bench --features bench
//!
//! Target benchmarks:
//! - Montgomery multiply: <30ns
//! - EPRAM step (N=100): <5μs
//! - Orchestrator decide: <50μs
//! - Rational reconstruct: <1μs
//! - Shadow entropy sample: <10ns

use std::time::{Duration, Instant};
use std::collections::HashMap;

// =============================================================================
// BENCHMARK INFRASTRUCTURE
// =============================================================================

/// Benchmark result for a single operation
#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub mean_ns: f64,
    pub stddev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub throughput: f64,  // ops/sec
    pub target_ns: Option<f64>,
    pub passed: bool,
}

impl BenchmarkResult {
    pub fn new(name: &str, times: &[Duration], target_ns: Option<f64>) -> Self {
        let iterations = times.len();
        let total_time: Duration = times.iter().sum();
        
        let ns_times: Vec<f64> = times.iter()
            .map(|d| d.as_nanos() as f64)
            .collect();
        
        let mean_ns = ns_times.iter().sum::<f64>() / iterations as f64;
        let variance = ns_times.iter()
            .map(|t| (t - mean_ns).powi(2))
            .sum::<f64>() / iterations as f64;
        let stddev_ns = variance.sqrt();
        
        let min_ns = ns_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_ns = ns_times.iter().cloned().fold(0.0, f64::max);
        
        let throughput = 1_000_000_000.0 / mean_ns;
        
        let passed = target_ns.map(|t| mean_ns <= t).unwrap_or(true);
        
        Self {
            name: name.to_string(),
            iterations,
            total_time,
            mean_ns,
            stddev_ns,
            min_ns,
            max_ns,
            throughput,
            target_ns,
            passed,
        }
    }
    
    pub fn report(&self) -> String {
        let status = if self.passed { "✓" } else { "✗" };
        let target_str = self.target_ns
            .map(|t| format!(" (target: {:.0}ns)", t))
            .unwrap_or_default();
        
        format!(
            "{} {}: {:.2}ns ± {:.2}ns [{:.0}-{:.0}ns] ({:.2}M ops/sec){}",
            status,
            self.name,
            self.mean_ns,
            self.stddev_ns,
            self.min_ns,
            self.max_ns,
            self.throughput / 1_000_000.0,
            target_str
        )
    }
}

/// Run a benchmark with warmup and measurement phases
pub fn run_benchmark<F>(name: &str, iterations: usize, warmup: usize, target_ns: Option<f64>, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    // Warmup
    for _ in 0..warmup {
        f();
    }
    
    // Measurement
    let mut times = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    }
    
    BenchmarkResult::new(name, &times, target_ns)
}

/// Benchmark suite runner
pub struct BenchmarkSuite {
    pub results: Vec<BenchmarkResult>,
    pub name: String,
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self {
            results: Vec::new(),
            name: name.to_string(),
        }
    }
    
    pub fn add(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }
    
    pub fn report(&self) -> String {
        let mut report = format!("═══════════════════════════════════════════════════════════════\n");
        report.push_str(&format!("  BENCHMARK SUITE: {}\n", self.name));
        report.push_str("═══════════════════════════════════════════════════════════════\n\n");
        
        let passed = self.results.iter().filter(|r| r.passed).count();
        let total = self.results.len();
        
        for result in &self.results {
            report.push_str(&format!("  {}\n", result.report()));
        }
        
        report.push_str("\n───────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("  SUMMARY: {}/{} benchmarks passed\n", passed, total));
        report.push_str("═══════════════════════════════════════════════════════════════\n");
        
        report
    }
    
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

// =============================================================================
// MONTGOMERY BENCHMARKS
// =============================================================================

pub mod montgomery_bench {
    use super::*;
    
    const TEST_PRIME: u64 = 65537;
    const ITERATIONS: usize = 100_000;
    const WARMUP: usize = 10_000;
    
    /// Montgomery context (simplified for benchmark)
    struct MontCtx {
        p: u64,
        n_prime: u64,
        r_sq: u64,
    }
    
    impl MontCtx {
        fn new(p: u64) -> Self {
            let r = ((1u128 << 64) % p as u128) as u64;
            let r_sq = ((r as u128 * r as u128) % p as u128) as u64;
            
            let mut x = 1u64;
            for _ in 0..6 {
                x = x.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(x)));
            }
            let n_prime = x.wrapping_neg();
            
            Self { p, n_prime, r_sq }
        }
        
        #[inline]
        fn reduce(&self, a: u128) -> u64 {
            let m = (a as u64).wrapping_mul(self.n_prime);
            let t = (a + (m as u128) * (self.p as u128)) >> 64;
            let t = t as u64;
            if t >= self.p { t - self.p } else { t }
        }
        
        #[inline]
        fn mul(&self, a: u64, b: u64) -> u64 {
            self.reduce(a as u128 * b as u128)
        }
        
        #[inline]
        fn to_mont(&self, a: u64) -> u64 {
            self.reduce(a as u128 * self.r_sq as u128)
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Montgomery Arithmetic");
        
        let ctx = MontCtx::new(TEST_PRIME);
        let a = ctx.to_mont(12345);
        let b = ctx.to_mont(67890);
        
        // Montgomery multiply
        let mut result = a;
        suite.add(run_benchmark(
            "montgomery_multiply",
            ITERATIONS,
            WARMUP,
            Some(30.0),  // Target: 30ns
            || {
                result = ctx.mul(result, b);
            },
        ));
        
        // Montgomery reduction
        let val = a as u128 * b as u128;
        suite.add(run_benchmark(
            "montgomery_reduce",
            ITERATIONS,
            WARMUP,
            Some(20.0),
            || {
                let _ = ctx.reduce(val);
            },
        ));
        
        // To Montgomery form
        suite.add(run_benchmark(
            "to_montgomery",
            ITERATIONS,
            WARMUP,
            Some(25.0),
            || {
                let _ = ctx.to_mont(12345);
            },
        ));
        
        suite
    }
}

// =============================================================================
// EPRAM BENCHMARKS
// =============================================================================

pub mod epram_bench {
    use super::*;
    
    const ITERATIONS: usize = 10_000;
    const WARMUP: usize = 1_000;
    
    #[inline]
    fn fourth_attractor_step(state: u64, target: u64, m: u64) -> u64 {
        let diff = (target + m - state) % m;
        if diff == 0 { return state; }
        
        let mut delta = (diff * 3) / 4;
        if delta == 0 {
            delta = if diff <= m / 2 { 1 } else { m - 1 };
        }
        (state + delta) % m
    }
    
    fn evolve_field(cells: &mut [u64], targets: &[u64], m: u64) {
        for i in 0..cells.len() {
            cells[i] = fourth_attractor_step(cells[i], targets[i], m);
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("EPRAM Field Operations");
        
        // Single cell step
        let mut state = 100u64;
        suite.add(run_benchmark(
            "single_cell_step",
            ITERATIONS * 10,
            WARMUP,
            Some(10.0),
            || {
                state = fourth_attractor_step(state, 0, 256);
            },
        ));
        
        // N=100 field step
        let mut cells_100: Vec<u64> = (0..100).map(|i| i * 2).collect();
        let targets_100: Vec<u64> = vec![0; 100];
        suite.add(run_benchmark(
            "field_step_n100",
            ITERATIONS,
            WARMUP,
            Some(5000.0),  // Target: 5μs
            || {
                evolve_field(&mut cells_100, &targets_100, 256);
            },
        ));
        
        // N=1000 field step
        let mut cells_1000: Vec<u64> = (0..1000).map(|i| i % 256).collect();
        let targets_1000: Vec<u64> = vec![0; 1000];
        suite.add(run_benchmark(
            "field_step_n1000",
            ITERATIONS / 10,
            WARMUP / 10,
            Some(50000.0),  // Target: 50μs
            || {
                evolve_field(&mut cells_1000, &targets_1000, 256);
            },
        ));
        
        suite
    }
}

// =============================================================================
// ORCHESTRATOR BENCHMARKS
// =============================================================================

pub mod orchestrator_bench {
    use super::*;
    
    const ITERATIONS: usize = 1_000;
    const WARMUP: usize = 100;
    
    struct SimpleOrchestrator {
        state: Vec<u64>,
        templates: Vec<Vec<u64>>,
        modulus: u64,
    }
    
    impl SimpleOrchestrator {
        fn new(n_cells: usize, modulus: u64) -> Self {
            Self {
                state: vec![0; n_cells],
                templates: Vec::new(),
                modulus,
            }
        }
        
        fn add_template(&mut self, pattern: Vec<u64>) {
            self.templates.push(pattern);
        }
        
        fn set_input(&mut self, input: Vec<u64>) {
            self.state = input;
        }
        
        fn distance(&self, template_idx: usize) -> u64 {
            let t = &self.templates[template_idx];
            self.state.iter()
                .zip(t.iter())
                .map(|(&s, &target)| {
                    let diff = (target + self.modulus - s) % self.modulus;
                    diff.min(self.modulus - diff)
                })
                .sum()
        }
        
        fn closest_template(&self) -> (usize, u64) {
            self.templates.iter()
                .enumerate()
                .map(|(i, _)| (i, self.distance(i)))
                .min_by_key(|&(_, d)| d)
                .unwrap_or((0, u64::MAX))
        }
        
        fn step(&mut self) {
            let (closest, _) = self.closest_template();
            let target = &self.templates[closest].clone();
            let m = self.modulus;
            
            for i in 0..self.state.len() {
                let diff = (target[i] + m - self.state[i]) % m;
                if diff == 0 { continue; }
                let delta = (diff * 3) / 4;
                let delta = if delta == 0 {
                    if diff <= m / 2 { 1 } else { m - 1 }
                } else { delta };
                self.state[i] = (self.state[i] + delta) % m;
            }
        }
        
        fn decide(&mut self, max_steps: usize) -> (usize, usize) {
            for step in 0..max_steps {
                let (closest, dist) = self.closest_template();
                if dist == 0 {
                    return (closest, step);
                }
                self.step();
            }
            let (closest, _) = self.closest_template();
            (closest, max_steps)
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Orchestrator Operations");
        
        // Template distance calculation
        let mut orch = SimpleOrchestrator::new(64, 256);
        orch.add_template(vec![0; 64]);
        orch.set_input(vec![128; 64]);
        
        suite.add(run_benchmark(
            "template_distance",
            ITERATIONS * 10,
            WARMUP,
            Some(500.0),
            || {
                let _ = orch.distance(0);
            },
        ));
        
        // Full decision (N=64)
        suite.add(run_benchmark(
            "decide_n64",
            ITERATIONS,
            WARMUP,
            Some(50000.0),  // Target: 50μs
            || {
                orch.set_input(vec![128; 64]);
                let _ = orch.decide(200);
            },
        ));
        
        // Multi-template decision
        let mut orch_multi = SimpleOrchestrator::new(64, 256);
        orch_multi.add_template(vec![32; 64]);
        orch_multi.add_template(vec![128; 64]);
        orch_multi.add_template(vec![224; 64]);
        
        suite.add(run_benchmark(
            "decide_3_templates",
            ITERATIONS,
            WARMUP,
            Some(60000.0),
            || {
                orch_multi.set_input(vec![120; 64]);
                let _ = orch_multi.decide(200);
            },
        ));
        
        suite
    }
}

// =============================================================================
// RATIONAL BENCHMARKS
// =============================================================================

pub mod rational_bench {
    use super::*;
    
    const ITERATIONS: usize = 10_000;
    const WARMUP: usize = 1_000;
    
    fn mod_inverse(a: u64, m: u64) -> Option<u64> {
        let mut t = 0i64;
        let mut new_t = 1i64;
        let mut r = m as i64;
        let mut new_r = a as i64;
        
        while new_r != 0 {
            let q = r / new_r;
            (t, new_t) = (new_t, t - q * new_t);
            (r, new_r) = (new_r, r - q * new_r);
        }
        
        if r > 1 { return None; }
        if t < 0 { t += m as i64; }
        Some(t as u64)
    }
    
    fn extended_gcd(a: u128, b: u128) -> (i128, i128, u128) {
        if b == 0 {
            (1, 0, a)
        } else {
            let (x, y, g) = extended_gcd(b, a % b);
            (y, x - (a / b) as i128 * y, g)
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Rational Arithmetic");
        
        let m = 65537u64 * 65521;
        
        // Modular inverse
        suite.add(run_benchmark(
            "mod_inverse",
            ITERATIONS,
            WARMUP,
            Some(100.0),
            || {
                let _ = mod_inverse(12345, 65537);
            },
        ));
        
        // Extended GCD (for reconstruction)
        suite.add(run_benchmark(
            "extended_gcd",
            ITERATIONS,
            WARMUP,
            Some(200.0),
            || {
                let _ = extended_gcd(12345, m as u128);
            },
        ));
        
        // Rational encode
        suite.add(run_benchmark(
            "rational_encode",
            ITERATIONS,
            WARMUP,
            Some(150.0),
            || {
                let p = 42i64;
                let q = 17u64;
                let q_inv = mod_inverse(q, 65537).unwrap();
                let _ = ((p as u64) as u128 * q_inv as u128) % 65537 as u128;
            },
        ));
        
        suite
    }
}

// =============================================================================
// SHADOW ENTROPY BENCHMARKS
// =============================================================================

pub mod shadow_bench {
    use super::*;
    
    const ITERATIONS: usize = 1_000_000;
    const WARMUP: usize = 100_000;
    
    struct ShadowState {
        state: u64,
        shadow: u64,
        counter: u64,
    }
    
    impl ShadowState {
        fn new(seed: u64) -> Self {
            let state = Self::mix(seed);
            let shadow = Self::mix(state ^ 0xDEADBEEF);
            Self { state, shadow, counter: 0 }
        }
        
        #[inline]
        fn mix(mut x: u64) -> u64 {
            x ^= x >> 33;
            x = x.wrapping_mul(0xff51afd7ed558ccd);
            x ^= x >> 33;
            x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
            x ^= x >> 33;
            x
        }
        
        #[inline]
        fn next(&mut self) -> u64 {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            
            self.shadow ^= Self::mix(self.counter);
            self.counter = self.counter.wrapping_add(1);
            
            Self::mix(self.state ^ self.shadow)
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Shadow Entropy");
        
        let mut shadow = ShadowState::new(12345);
        
        // Single sample
        suite.add(run_benchmark(
            "shadow_sample",
            ITERATIONS,
            WARMUP,
            Some(10.0),  // Target: 10ns
            || {
                let _ = shadow.next();
            },
        ));
        
        // Mix function
        suite.add(run_benchmark(
            "shadow_mix",
            ITERATIONS,
            WARMUP,
            Some(5.0),
            || {
                let _ = ShadowState::mix(12345);
            },
        ));
        
        suite
    }
}

// =============================================================================
// K-ELIMINATION BENCHMARKS
// =============================================================================

pub mod k_elim_bench {
    use super::*;
    
    const ITERATIONS: usize = 100_000;
    const WARMUP: usize = 10_000;
    
    fn mod_inverse(a: u64, m: u64) -> u64 {
        let mut t = 0i64;
        let mut new_t = 1i64;
        let mut r = m as i64;
        let mut new_r = a as i64;
        
        while new_r != 0 {
            let q = r / new_r;
            (t, new_t) = (new_t, t - q * new_t);
            (r, new_r) = (new_r, r - q * new_r);
        }
        
        if t < 0 { t += m as i64; }
        t as u64
    }
    
    struct DualCodex {
        m_alpha: u64,
        m_beta: u64,
        alpha_inv_beta: u64,
    }
    
    impl DualCodex {
        fn new(m_alpha: u64, m_beta: u64) -> Self {
            let alpha_inv_beta = mod_inverse(m_alpha, m_beta);
            Self { m_alpha, m_beta, alpha_inv_beta }
        }
        
        #[inline]
        fn recover_k(&self, r_alpha: u64, r_beta: u64) -> u64 {
            let diff = if r_beta >= (r_alpha % self.m_beta) {
                r_beta - (r_alpha % self.m_beta)
            } else {
                self.m_beta - (r_alpha % self.m_beta) + r_beta
            };
            
            ((diff as u128 * self.alpha_inv_beta as u128) % self.m_beta as u128) as u64
        }
    }
    
    pub fn run() -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("K-Elimination");
        
        let dc = DualCodex::new(65537, 65521);
        let r_alpha = 12345u64;
        let r_beta = 67890u64 % dc.m_beta;
        
        // K recovery
        suite.add(run_benchmark(
            "k_recover",
            ITERATIONS,
            WARMUP,
            Some(20.0),
            || {
                let _ = dc.recover_k(r_alpha, r_beta);
            },
        ));
        
        // Full reconstruction
        suite.add(run_benchmark(
            "full_reconstruct",
            ITERATIONS,
            WARMUP,
            Some(30.0),
            || {
                let k = dc.recover_k(r_alpha, r_beta);
                let _ = r_alpha as u128 + k as u128 * dc.m_alpha as u128;
            },
        ));
        
        suite
    }
}

// =============================================================================
// MASTER BENCHMARK RUNNER
// =============================================================================

pub fn run_all_benchmarks() -> Vec<BenchmarkSuite> {
    vec![
        montgomery_bench::run(),
        epram_bench::run(),
        orchestrator_bench::run(),
        rational_bench::run(),
        shadow_bench::run(),
        k_elim_bench::run(),
    ]
}

pub fn generate_report(suites: &[BenchmarkSuite]) -> String {
    let mut report = String::new();
    
    report.push_str("\n");
    report.push_str("╔═══════════════════════════════════════════════════════════════════════════════╗\n");
    report.push_str("║                    QMNF/EPRAM PERFORMANCE BENCHMARK REPORT                    ║\n");
    report.push_str("╚═══════════════════════════════════════════════════════════════════════════════╝\n\n");
    
    for suite in suites {
        report.push_str(&suite.report());
        report.push_str("\n");
    }
    
    // Summary
    let total_benchmarks: usize = suites.iter().map(|s| s.results.len()).sum();
    let passed_benchmarks: usize = suites.iter()
        .map(|s| s.results.iter().filter(|r| r.passed).count())
        .sum();
    
    report.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    report.push_str(&format!("  OVERALL: {}/{} benchmarks passed\n", passed_benchmarks, total_benchmarks));
    
    if passed_benchmarks == total_benchmarks {
        report.push_str("  STATUS: ✓ ALL PERFORMANCE TARGETS MET\n");
    } else {
        report.push_str(&format!("  STATUS: ✗ {} benchmarks below target\n", total_benchmarks - passed_benchmarks));
    }
    report.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    
    report
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_result() {
        let times: Vec<Duration> = (0..100)
            .map(|_| Duration::from_nanos(50))
            .collect();
        
        let result = BenchmarkResult::new("test", &times, Some(100.0));
        
        assert!(result.passed);
        assert!((result.mean_ns - 50.0).abs() < 1.0);
    }
    
    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("Test Suite");
        
        suite.add(BenchmarkResult::new(
            "fast_op",
            &vec![Duration::from_nanos(10); 100],
            Some(20.0),
        ));
        
        suite.add(BenchmarkResult::new(
            "slow_op",
            &vec![Duration::from_nanos(100); 100],
            Some(50.0),
        ));
        
        assert!(!suite.all_passed());  // slow_op exceeds target
    }
    
    #[test]
    #[ignore]  // Run with: cargo test --release -- --ignored
    fn test_run_all_benchmarks() {
        let suites = run_all_benchmarks();
        let report = generate_report(&suites);
        println!("{}", report);
    }
}
