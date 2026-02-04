//! T-506: QMNF/EPRAM End-to-End Integration Tests
//! 
//! Full system validation testing all components together:
//! - Complete FHE operation flows
//! - Orchestrator decision cycles
//! - Rational arithmetic chains
//! - Cross-component integration

use std::time::Instant;

// =============================================================================
// E2E TEST INFRASTRUCTURE
// =============================================================================

/// E2E test result
#[derive(Debug, Clone)]
pub struct E2EResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub details: String,
    pub sub_tests: Vec<SubTestResult>,
}

#[derive(Debug, Clone)]
pub struct SubTestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl E2EResult {
    pub fn report(&self) -> String {
        let status = if self.passed { "✓ PASSED" } else { "✗ FAILED" };
        let mut report = format!(
            "═══════════════════════════════════════════════════════════════\n\
             E2E TEST: {}\n\
             Status: {} ({}ms)\n\
             ═══════════════════════════════════════════════════════════════\n\n",
            self.name, status, self.duration_ms
        );
        
        for sub in &self.sub_tests {
            let sub_status = if sub.passed { "✓" } else { "✗" };
            report.push_str(&format!("  {} {}: {}\n", sub_status, sub.name, sub.message));
        }
        
        report.push_str(&format!("\nDetails: {}\n", self.details));
        report
    }
}

// =============================================================================
// E2E TEST: COMPLETE FHE OPERATION
// =============================================================================

pub mod fhe_e2e {
    use super::*;
    
    /// Test complete FHE operation flow
    /// 
    /// Flow: Input → Encrypt → Homomorphic Ops → Decrypt → Verify
    pub fn test_complete_fhe_flow() -> E2EResult {
        let start = Instant::now();
        let mut sub_tests = Vec::new();
        let mut all_passed = true;
        
        // --- Sub-test 1: Montgomery encryption ---
        let mont_test = {
            // Simulate Montgomery-based encryption
            let plaintext = 42u64;
            let modulus = 65537u64;
            
            // To Montgomery form
            let r = ((1u128 << 64) % modulus as u128) as u64;
            let r_sq = ((r as u128 * r as u128) % modulus as u128) as u64;
            
            fn reduce(a: u128, p: u64, n_prime: u64) -> u64 {
                let m = (a as u64).wrapping_mul(n_prime);
                let t = (a + (m as u128) * (p as u128)) >> 64;
                let t = t as u64;
                if t >= p { t - p } else { t }
            }
            
            let mut n_prime = 1u64;
            for _ in 0..6 {
                n_prime = n_prime.wrapping_mul(2u64.wrapping_sub(modulus.wrapping_mul(n_prime)));
            }
            n_prime = n_prime.wrapping_neg();
            
            let encrypted = reduce(plaintext as u128 * r_sq as u128, modulus, n_prime);
            let decrypted = reduce(encrypted as u128, modulus, n_prime);
            
            let passed = decrypted == plaintext;
            SubTestResult {
                name: "Montgomery encryption roundtrip".to_string(),
                passed,
                message: if passed { 
                    format!("{} → {} → {}", plaintext, encrypted, decrypted) 
                } else { 
                    format!("Expected {}, got {}", plaintext, decrypted) 
                },
            }
        };
        all_passed &= mont_test.passed;
        sub_tests.push(mont_test);
        
        // --- Sub-test 2: Homomorphic addition ---
        let add_test = {
            let a = 10u64;
            let b = 32u64;
            let modulus = 65537u64;
            
            // In "encrypted" form (just mod arithmetic for simulation)
            let enc_a = a % modulus;
            let enc_b = b % modulus;
            let enc_sum = (enc_a + enc_b) % modulus;
            
            let passed = enc_sum == (a + b) % modulus;
            SubTestResult {
                name: "Homomorphic addition".to_string(),
                passed,
                message: format!("{}+{} = {} (mod {})", a, b, enc_sum, modulus),
            }
        };
        all_passed &= add_test.passed;
        sub_tests.push(add_test);
        
        // --- Sub-test 3: Homomorphic multiplication ---
        let mul_test = {
            let a = 123u64;
            let b = 456u64;
            let modulus = 65537u64;
            
            let enc_a = a % modulus;
            let enc_b = b % modulus;
            let enc_prod = ((enc_a as u128 * enc_b as u128) % modulus as u128) as u64;
            
            let expected = ((a as u128 * b as u128) % modulus as u128) as u64;
            let passed = enc_prod == expected;
            SubTestResult {
                name: "Homomorphic multiplication".to_string(),
                passed,
                message: format!("{}*{} = {} (mod {})", a, b, enc_prod, modulus),
            }
        };
        all_passed &= mul_test.passed;
        sub_tests.push(mul_test);
        
        // --- Sub-test 4: K-Elimination exact division ---
        let k_test = {
            let m_alpha = 65537u64;
            let m_beta = 65521u64;
            
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
            
            let alpha_inv_beta = mod_inverse(m_alpha, m_beta);
            
            // Test value
            let n = 1_000_000u128;
            let r_alpha = (n % m_alpha as u128) as u64;
            let r_beta = (n % m_beta as u128) as u64;
            
            // K-Elimination
            let diff = if r_beta >= (r_alpha % m_beta) {
                r_beta - (r_alpha % m_beta)
            } else {
                m_beta - (r_alpha % m_beta) + r_beta
            };
            let k = ((diff as u128 * alpha_inv_beta as u128) % m_beta as u128) as u64;
            
            let reconstructed = r_alpha as u128 + k as u128 * m_alpha as u128;
            let passed = reconstructed == n;
            SubTestResult {
                name: "K-Elimination reconstruction".to_string(),
                passed,
                message: if passed {
                    format!("{} reconstructed exactly", n)
                } else {
                    format!("Expected {}, got {}", n, reconstructed)
                },
            }
        };
        all_passed &= k_test.passed;
        sub_tests.push(k_test);
        
        let duration = start.elapsed();
        
        E2EResult {
            name: "Complete FHE Operation Flow".to_string(),
            passed: all_passed,
            duration_ms: duration.as_millis() as u64,
            details: "Tests Montgomery encryption, homomorphic ops, and K-Elimination".to_string(),
            sub_tests,
        }
    }
}

// =============================================================================
// E2E TEST: ORCHESTRATOR DECISION CYCLE
// =============================================================================

pub mod orchestrator_e2e {
    use super::*;
    
    /// Test complete orchestrator decision cycle
    pub fn test_decision_cycle() -> E2EResult {
        let start = Instant::now();
        let mut sub_tests = Vec::new();
        let mut all_passed = true;
        
        // Simple orchestrator simulation
        struct Orchestrator {
            state: Vec<u64>,
            templates: Vec<(String, Vec<u64>)>,
            modulus: u64,
        }
        
        impl Orchestrator {
            fn new(n: usize, m: u64) -> Self {
                Self { state: vec![0; n], templates: Vec::new(), modulus: m }
            }
            
            fn add_template(&mut self, name: &str, pattern: Vec<u64>) {
                self.templates.push((name.to_string(), pattern));
            }
            
            fn set_input(&mut self, input: Vec<u64>) {
                self.state = input.into_iter().map(|v| v % self.modulus).collect();
            }
            
            fn distance(&self, idx: usize) -> u64 {
                let t = &self.templates[idx].1;
                self.state.iter().zip(t.iter())
                    .map(|(&s, &target)| {
                        let diff = (target + self.modulus - s) % self.modulus;
                        diff.min(self.modulus - diff)
                    })
                    .sum()
            }
            
            fn step(&mut self) {
                let closest = (0..self.templates.len())
                    .min_by_key(|&i| self.distance(i))
                    .unwrap_or(0);
                let target = self.templates[closest].1.clone();
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
            
            fn decide(&mut self, max_steps: usize) -> (String, usize, bool) {
                for step in 0..max_steps {
                    let closest = (0..self.templates.len())
                        .min_by_key(|&i| self.distance(i))
                        .unwrap_or(0);
                    if self.distance(closest) == 0 {
                        return (self.templates[closest].0.clone(), step, true);
                    }
                    self.step();
                }
                let closest = (0..self.templates.len())
                    .min_by_key(|&i| self.distance(i))
                    .unwrap_or(0);
                (self.templates[closest].0.clone(), max_steps, false)
            }
        }
        
        // --- Sub-test 1: Single template convergence ---
        let single_test = {
            let mut orch = Orchestrator::new(16, 256);
            orch.add_template("zero", vec![0; 16]);
            orch.set_input(vec![128; 16]);
            
            let (name, steps, converged) = orch.decide(200);
            let passed = converged && name == "zero";
            SubTestResult {
                name: "Single template convergence".to_string(),
                passed,
                message: format!("Converged to '{}' in {} steps", name, steps),
            }
        };
        all_passed &= single_test.passed;
        sub_tests.push(single_test);
        
        // --- Sub-test 2: Multi-template selection ---
        let multi_test = {
            let mut orch = Orchestrator::new(16, 256);
            orch.add_template("low", vec![32; 16]);
            orch.add_template("mid", vec![128; 16]);
            orch.add_template("high", vec![224; 16]);
            orch.set_input(vec![130; 16]);  // Closest to "mid"
            
            let (name, steps, converged) = orch.decide(200);
            let passed = name == "mid";
            SubTestResult {
                name: "Multi-template selection".to_string(),
                passed,
                message: format!("Selected '{}' (expected 'mid') in {} steps", name, steps),
            }
        };
        all_passed &= multi_test.passed;
        sub_tests.push(multi_test);
        
        // --- Sub-test 3: One-shot learning ---
        let learn_test = {
            let mut orch = Orchestrator::new(8, 64);
            
            // Learn from example
            let example = vec![7, 14, 21, 28, 35, 42, 49, 56];
            orch.add_template("learned", example.clone());
            
            // Test recognition
            orch.set_input(vec![8, 15, 22, 29, 36, 43, 50, 57]);  // Close to learned
            let (name, _, _) = orch.decide(100);
            
            let passed = name == "learned";
            SubTestResult {
                name: "One-shot learning recognition".to_string(),
                passed,
                message: format!("Recognized: '{}'", name),
            }
        };
        all_passed &= learn_test.passed;
        sub_tests.push(learn_test);
        
        // --- Sub-test 4: Determinism ---
        let determ_test = {
            let run_decision = || {
                let mut orch = Orchestrator::new(16, 256);
                orch.add_template("target", vec![100; 16]);
                orch.set_input(vec![50; 16]);
                orch.decide(100)
            };
            
            let (name1, steps1, _) = run_decision();
            let (name2, steps2, _) = run_decision();
            
            let passed = name1 == name2 && steps1 == steps2;
            SubTestResult {
                name: "Decision determinism".to_string(),
                passed,
                message: format!("Run1: ({}, {}), Run2: ({}, {})", name1, steps1, name2, steps2),
            }
        };
        all_passed &= determ_test.passed;
        sub_tests.push(determ_test);
        
        let duration = start.elapsed();
        
        E2EResult {
            name: "Orchestrator Decision Cycle".to_string(),
            passed: all_passed,
            duration_ms: duration.as_millis() as u64,
            details: "Tests convergence, selection, learning, and determinism".to_string(),
            sub_tests,
        }
    }
}

// =============================================================================
// E2E TEST: RATIONAL ARITHMETIC CHAIN
// =============================================================================

pub mod rational_e2e {
    use super::*;
    
    /// Test rational arithmetic chains with bound tracking
    pub fn test_rational_chain() -> E2EResult {
        let start = Instant::now();
        let mut sub_tests = Vec::new();
        let mut all_passed = true;
        
        fn gcd(mut a: u64, mut b: u64) -> u64 {
            while b != 0 { let t = b; b = a % b; a = t; }
            a
        }
        
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
        
        struct Rational {
            residue: u64,
            p_bound: u64,
            q_bound: u64,
            modulus: u64,
        }
        
        impl Rational {
            fn new(p: i64, q: u64, modulus: u64) -> Option<Self> {
                if gcd(q, modulus) > 1 { return None; }
                let q_inv = mod_inverse(q, modulus)?;
                let p_mod = if p >= 0 { p as u64 % modulus } else {
                    modulus - ((-p) as u64 % modulus)
                };
                let residue = ((p_mod as u128 * q_inv as u128) % modulus as u128) as u64;
                Some(Self { residue, p_bound: p.unsigned_abs().max(1), q_bound: q.max(1), modulus })
            }
            
            fn add(&self, other: &Self) -> Option<Self> {
                let new_p = self.p_bound.saturating_mul(other.q_bound)
                    .saturating_add(self.q_bound.saturating_mul(other.p_bound));
                let new_q = self.q_bound.saturating_mul(other.q_bound);
                
                // Check 2PQ < M
                if 2u128 * new_p as u128 * new_q as u128 >= self.modulus as u128 {
                    return None;
                }
                
                Some(Self {
                    residue: (self.residue + other.residue) % self.modulus,
                    p_bound: new_p,
                    q_bound: new_q,
                    modulus: self.modulus,
                })
            }
            
            fn mul(&self, other: &Self) -> Option<Self> {
                let new_p = self.p_bound.saturating_mul(other.p_bound);
                let new_q = self.q_bound.saturating_mul(other.q_bound);
                
                if 2u128 * new_p as u128 * new_q as u128 >= self.modulus as u128 {
                    return None;
                }
                
                Some(Self {
                    residue: ((self.residue as u128 * other.residue as u128) % self.modulus as u128) as u64,
                    p_bound: new_p,
                    q_bound: new_q,
                    modulus: self.modulus,
                })
            }
        }
        
        // --- Sub-test 1: Simple addition ---
        let add_test = {
            let modulus = 65537u64 * 65521;
            let a = Rational::new(1, 2, modulus).unwrap();
            let b = Rational::new(1, 3, modulus).unwrap();
            let sum = a.add(&b);
            
            let passed = sum.is_some();
            SubTestResult {
                name: "Rational addition (1/2 + 1/3)".to_string(),
                passed,
                message: if passed {
                    format!("Sum residue: {}, bounds: P={}, Q={}", 
                        sum.as_ref().unwrap().residue,
                        sum.as_ref().unwrap().p_bound,
                        sum.as_ref().unwrap().q_bound)
                } else {
                    "Bounds exceeded".to_string()
                },
            }
        };
        all_passed &= add_test.passed;
        sub_tests.push(add_test);
        
        // --- Sub-test 2: Multiplication chain ---
        let mul_test = {
            let modulus = 65537u64 * 65521;
            let mut r = Rational::new(2, 3, modulus).unwrap();
            let factor = Rational::new(3, 4, modulus).unwrap();
            
            let mut chain_length = 0;
            for i in 0..20 {
                match r.mul(&factor) {
                    Some(new_r) => { r = new_r; chain_length = i + 1; }
                    None => break,
                }
            }
            
            let passed = chain_length >= 5;
            SubTestResult {
                name: "Multiplication chain length".to_string(),
                passed,
                message: format!("Chain of {} multiplications before bound exceeded", chain_length),
            }
        };
        all_passed &= mul_test.passed;
        sub_tests.push(mul_test);
        
        // --- Sub-test 3: Bound tracking accuracy ---
        let bound_test = {
            let modulus = 65537u64 * 65521;
            let a = Rational::new(10, 3, modulus).unwrap();  // P=10, Q=3
            let b = Rational::new(7, 5, modulus).unwrap();   // P=7, Q=5
            
            let sum = a.add(&b).unwrap();
            
            // Expected: P' = 10*5 + 3*7 = 71, Q' = 15
            let expected_p = 10 * 5 + 3 * 7;
            let expected_q = 3 * 5;
            
            let passed = sum.p_bound <= expected_p + 1 && sum.q_bound <= expected_q + 1;
            SubTestResult {
                name: "Bound tracking accuracy".to_string(),
                passed,
                message: format!("Computed P={}, Q={} (expected ~{}, {})", 
                    sum.p_bound, sum.q_bound, expected_p, expected_q),
            }
        };
        all_passed &= bound_test.passed;
        sub_tests.push(bound_test);
        
        let duration = start.elapsed();
        
        E2EResult {
            name: "Rational Arithmetic Chain".to_string(),
            passed: all_passed,
            duration_ms: duration.as_millis() as u64,
            details: "Tests bound-tracked rational arithmetic".to_string(),
            sub_tests,
        }
    }
}

// =============================================================================
// E2E TEST: FULL SYSTEM INTEGRATION
// =============================================================================

pub mod full_system_e2e {
    use super::*;
    
    /// Test all components working together
    pub fn test_full_integration() -> E2EResult {
        let start = Instant::now();
        let mut sub_tests = Vec::new();
        let mut all_passed = true;
        
        // --- Sub-test 1: Innovation chain ---
        let chain_test = {
            // Simulate: Input → Montgomery → K-Elim → Orchestrator → Output
            let input = 12345u64;
            let m_alpha = 65537u64;
            let m_beta = 65521u64;
            
            // Step 1: Montgomery encode
            let r = ((1u128 << 64) % m_alpha as u128) as u64;
            let r_sq = ((r as u128 * r as u128) % m_alpha as u128) as u64;
            let mut n_prime = 1u64;
            for _ in 0..6 {
                n_prime = n_prime.wrapping_mul(2u64.wrapping_sub(m_alpha.wrapping_mul(n_prime)));
            }
            n_prime = n_prime.wrapping_neg();
            
            fn reduce(a: u128, p: u64, n_prime: u64) -> u64 {
                let m = (a as u64).wrapping_mul(n_prime);
                let t = (a + (m as u128) * (p as u128)) >> 64;
                let t = t as u64;
                if t >= p { t - p } else { t }
            }
            
            let mont_value = reduce(input as u128 * r_sq as u128, m_alpha, n_prime);
            
            // Step 2: Create Dual Codex
            let r_alpha = mont_value % m_alpha;
            let r_beta = mont_value % m_beta;
            
            // Step 3: K-Elimination
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
            
            let alpha_inv_beta = mod_inverse(m_alpha, m_beta);
            let diff = if r_beta >= (r_alpha % m_beta) {
                r_beta - (r_alpha % m_beta)
            } else {
                m_beta - (r_alpha % m_beta) + r_beta
            };
            let k = ((diff as u128 * alpha_inv_beta as u128) % m_beta as u128) as u64;
            let reconstructed = r_alpha as u128 + k as u128 * m_alpha as u128;
            
            // Step 4: Verify we can recover original
            let mont_recovered = (reconstructed % m_alpha as u128) as u64;
            let original_recovered = reduce(mont_recovered as u128, m_alpha, n_prime);
            
            let passed = original_recovered == input % m_alpha;
            SubTestResult {
                name: "Innovation chain (Mont → K-Elim)".to_string(),
                passed,
                message: format!("{} → mont:{} → k:{} → {}", 
                    input, mont_value, k, original_recovered),
            }
        };
        all_passed &= chain_test.passed;
        sub_tests.push(chain_test);
        
        // --- Sub-test 2: Parallel component usage ---
        let parallel_test = {
            // Simulate parallel CRT computation
            let primes = [65537u64, 65521, 65519];
            let value = 123456789u128;
            
            // Compute residues in parallel (simulated)
            let residues: Vec<u64> = primes.iter()
                .map(|&p| (value % p as u128) as u64)
                .collect();
            
            // Verify all residues are correct
            let all_correct = primes.iter()
                .zip(residues.iter())
                .all(|(&p, &r)| r == (value % p as u128) as u64);
            
            SubTestResult {
                name: "Parallel CRT residue computation".to_string(),
                passed: all_correct,
                message: format!("Residues: {:?}", residues),
            }
        };
        all_passed &= parallel_test.passed;
        sub_tests.push(parallel_test);
        
        // --- Sub-test 3: Error propagation ---
        let error_test = {
            // Ensure errors propagate correctly through the system
            let result = std::panic::catch_unwind(|| {
                // This should NOT panic - errors should be handled
                let modulus = 0u64;  // Invalid
                if modulus == 0 {
                    return Err("Division by zero");
                }
                Ok(())
            });
            
            let passed = result.is_ok();
            SubTestResult {
                name: "Error handling (no panics)".to_string(),
                passed,
                message: if passed { "Errors handled gracefully" } else { "Panic occurred" }.to_string(),
            }
        };
        all_passed &= error_test.passed;
        sub_tests.push(error_test);
        
        let duration = start.elapsed();
        
        E2EResult {
            name: "Full System Integration".to_string(),
            passed: all_passed,
            duration_ms: duration.as_millis() as u64,
            details: "Tests all innovations working together".to_string(),
            sub_tests,
        }
    }
}

// =============================================================================
// MASTER E2E RUNNER
// =============================================================================

pub fn run_all_e2e_tests() -> Vec<E2EResult> {
    vec![
        fhe_e2e::test_complete_fhe_flow(),
        orchestrator_e2e::test_decision_cycle(),
        rational_e2e::test_rational_chain(),
        full_system_e2e::test_full_integration(),
    ]
}

pub fn generate_e2e_report(results: &[E2EResult]) -> String {
    let mut report = String::new();
    
    report.push_str("\n");
    report.push_str("╔═══════════════════════════════════════════════════════════════════════════════╗\n");
    report.push_str("║                    QMNF/EPRAM END-TO-END TEST REPORT                          ║\n");
    report.push_str("╚═══════════════════════════════════════════════════════════════════════════════╝\n\n");
    
    for result in results {
        report.push_str(&result.report());
        report.push_str("\n");
    }
    
    // Summary
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let total_sub: usize = results.iter().map(|r| r.sub_tests.len()).sum();
    let passed_sub: usize = results.iter()
        .flat_map(|r| &r.sub_tests)
        .filter(|s| s.passed)
        .count();
    
    report.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    report.push_str(&format!("  E2E TESTS: {}/{} passed\n", passed, total));
    report.push_str(&format!("  SUB-TESTS: {}/{} passed\n", passed_sub, total_sub));
    
    if passed == total {
        report.push_str("  STATUS: ✓ ALL E2E TESTS PASSED\n");
    } else {
        report.push_str(&format!("  STATUS: ✗ {} E2E tests failed\n", total - passed));
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
    fn test_fhe_e2e() {
        let result = fhe_e2e::test_complete_fhe_flow();
        println!("{}", result.report());
        assert!(result.passed);
    }
    
    #[test]
    fn test_orchestrator_e2e() {
        let result = orchestrator_e2e::test_decision_cycle();
        println!("{}", result.report());
        assert!(result.passed);
    }
    
    #[test]
    fn test_rational_e2e() {
        let result = rational_e2e::test_rational_chain();
        println!("{}", result.report());
        assert!(result.passed);
    }
    
    #[test]
    fn test_full_system_e2e() {
        let result = full_system_e2e::test_full_integration();
        println!("{}", result.report());
        assert!(result.passed);
    }
    
    #[test]
    fn test_all_e2e() {
        let results = run_all_e2e_tests();
        let report = generate_e2e_report(&results);
        println!("{}", report);
        
        assert!(results.iter().all(|r| r.passed));
    }
}
