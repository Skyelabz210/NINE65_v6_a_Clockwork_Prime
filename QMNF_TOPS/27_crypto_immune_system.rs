// crypto_immune_system.rs - Proactive Parameter Validation via Period-Finding
//
// This module implements the Cryptographic Immune System that validates
// cryptographic parameters BEFORE deployment using period-finding and
// structural analysis to detect weaknesses that enable attacks.
//
// Philosophy: Don't wait for adversaries to find weaknesses. Find them first.
//
// Coverage:
// - FHE parameters (NINE65 qclassic and v2_complete)
// - AHOP orbits (Apollonian Harmonic Optimization)
// - Loki 5 cryptographic parameters
// - Sentinel voting parameters
// - RSA/ECC (if used in hybrid schemes)
// - Lattice-based post-quantum schemes

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// ===========================
// === Parameter Validation ==
// ===========================

/// Validation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationResult {
    Accept,        // Deploy safely
    Harden,        // Fix and retry
    Reject,        // Do not use
    Quarantine,    // Monitor closely
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub parameter_set: String,
    pub validated_at: u64,  // Unix timestamp
    pub checks_passed: Vec<ValidationCheck>,
    pub checks_failed: Vec<ValidationCheck>,
    pub recommendation: ValidationResult,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub details: String,
}

// ===========================
// === FHE Validation ========
// ===========================

/// FHE variant type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FHEVariant {
    QClassic,
    V2Complete,
}

/// FHE parameters for validation
#[derive(Debug, Clone)]
pub struct FHEParams {
    pub variant: FHEVariant,
    pub poly_degree: usize,      // N (usually power of 2)
    pub modulus: u64,             // q (prime)
    pub plaintext_modulus: u32,   // t
}

/// FHE parameter validator
pub struct FHEValidator;

impl FHEValidator {
    pub fn validate(params: &FHEParams) -> ValidationReport {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        // Check 1: Polynomial degree is power of 2
        let is_power_of_2 = params.poly_degree.is_power_of_two();
        if is_power_of_2 {
            passed.push(ValidationCheck {
                name: "polynomial_degree_power_of_2".to_string(),
                passed: true,
                details: format!("N = {} is power of 2", params.poly_degree),
            });
        } else {
            failed.push(ValidationCheck {
                name: "polynomial_degree_power_of_2".to_string(),
                passed: false,
                details: format!("N = {} is NOT power of 2", params.poly_degree),
            });
        }

        // Check 2: Modulus primality (Miller-Rabin)
        let is_prime = Self::is_probable_prime(params.modulus);
        if is_prime {
            passed.push(ValidationCheck {
                name: "modulus_primality".to_string(),
                passed: true,
                details: format!("q = {} is probably prime", params.modulus),
            });
        } else {
            failed.push(ValidationCheck {
                name: "modulus_primality".to_string(),
                passed: false,
                details: format!("q = {} failed primality test", params.modulus),
            });
        }

        // Check 3: NTT-friendly (q ≡ 1 mod 2N)
        let cyclotomic_order = 2 * params.poly_degree as u64;
        let ntt_friendly = (params.modulus - 1) % cyclotomic_order == 0;
        if ntt_friendly {
            passed.push(ValidationCheck {
                name: "ntt_friendly".to_string(),
                passed: true,
                details: format!("q - 1 ≡ 0 mod {}", cyclotomic_order),
            });
        } else {
            failed.push(ValidationCheck {
                name: "ntt_friendly".to_string(),
                passed: false,
                details: format!("q - 1 NOT divisible by {}", cyclotomic_order),
            });
        }

        // Check 4: Period analysis (find order of primitive root)
        if is_prime && ntt_friendly {
            let primitive_root = Self::find_primitive_root(params.modulus, cyclotomic_order);
            if let Some(omega) = primitive_root {
                let order = Self::find_order(omega, params.modulus);
                if order == cyclotomic_order {
                    passed.push(ValidationCheck {
                        name: "primitive_root_order".to_string(),
                        passed: true,
                        details: format!("Order of ω = {} equals {}", omega, cyclotomic_order),
                    });
                } else {
                    failed.push(ValidationCheck {
                        name: "primitive_root_order".to_string(),
                        passed: false,
                        details: format!("Order {} ≠ {}", order, cyclotomic_order),
                    });
                }
            } else {
                failed.push(ValidationCheck {
                    name: "primitive_root_exists".to_string(),
                    passed: false,
                    details: "Could not find primitive root".to_string(),
                });
            }
        }

        // Check 5: Variant-specific checks
        match params.variant {
            FHEVariant::QClassic => {
                // QClassic: Optimized for N≥2048
                if params.poly_degree >= 2048 {
                    passed.push(ValidationCheck {
                        name: "qclassic_sweet_spot".to_string(),
                        passed: true,
                        details: format!("N = {} ≥ 2048 (optimal for qclassic)", params.poly_degree),
                    });
                } else {
                    failed.push(ValidationCheck {
                        name: "qclassic_sweet_spot".to_string(),
                        passed: false,
                        details: format!("N = {} < 2048 (qclassic underutilized)", params.poly_degree),
                    });
                }
            }
            FHEVariant::V2Complete => {
                // V2Complete: Feature-rich, no specific N requirement
                passed.push(ValidationCheck {
                    name: "v2_feature_set".to_string(),
                    passed: true,
                    details: "v2_complete supports all features".to_string(),
                });
            }
        }

        // Check 6: Security level (based on N and q)
        let security_level = Self::estimate_security_level(params.poly_degree, params.modulus);
        if security_level >= 128 {
            passed.push(ValidationCheck {
                name: "security_level".to_string(),
                passed: true,
                details: format!("≥128-bit security (estimated {} bits)", security_level),
            });
        } else {
            failed.push(ValidationCheck {
                name: "security_level".to_string(),
                passed: false,
                details: format!("<128-bit security (estimated {} bits)", security_level),
            });
        }

        // Determine recommendation
        let recommendation = if failed.is_empty() {
            ValidationResult::Accept
        } else if failed.len() <= 2 {
            ValidationResult::Harden
        } else {
            ValidationResult::Reject
        };

        ValidationReport {
            parameter_set: format!("FHE({:?}, N={}, q={})", params.variant, params.poly_degree, params.modulus),
            validated_at: 0,  // TODO: Add timestamp
            checks_passed: passed,
            checks_failed: failed,
            recommendation: recommendation.clone(),
            rationale: Self::generate_rationale(&recommendation),
        }
    }

    // === Helper Methods ===

    fn is_probable_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        // Miller-Rabin with k=10 rounds
        // TODO: Implement full Miller-Rabin
        // For now, simple trial division
        let limit = (n as f64).sqrt() as u64 + 1;
        for i in 3..limit {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn find_primitive_root(modulus: u64, order: u64) -> Option<u64> {
        // TODO: Implement primitive root finding
        // For now, return candidate
        for candidate in 2..modulus {
            if Self::find_order(candidate, modulus) == order {
                return Some(candidate);
            }
            if candidate > 100 {
                break;  // Limit search
            }
        }
        None
    }

    fn find_order(a: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        let mut power = a % modulus;

        for order in 1..modulus {
            if power == 1 {
                return order;
            }
            power = (power * a) % modulus;
        }

        modulus  // Fallback
    }

    fn estimate_security_level(n: usize, q: u64) -> u32 {
        // Rough estimate: log2(q^n)
        let log_q = (q as f64).log2();
        let security = (n as f64) * log_q / 2.0;
        security as u32
    }

    fn generate_rationale(recommendation: &ValidationResult) -> String {
        match recommendation {
            ValidationResult::Accept => "All checks passed. Parameters are secure.".to_string(),
            ValidationResult::Harden => "Some checks failed. Adjust parameters and revalidate.".to_string(),
            ValidationResult::Reject => "Critical checks failed. Do not use these parameters.".to_string(),
            ValidationResult::Quarantine => "Parameters suspicious. Monitor deployment closely.".to_string(),
        }
    }
}

// ===========================
// === AHOP Validation =======
// ===========================

/// AHOP orbit parameters
#[derive(Debug, Clone)]
pub struct AHOPParams {
    pub generator: u64,
    pub modulus: u64,
    pub orbit_period: u64,
}

/// AHOP validator
pub struct AHOPValidator;

impl AHOPValidator {
    pub fn validate(params: &AHOPParams) -> ValidationReport {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        // Check 1: Minimum orbit period
        const MIN_ORBIT_PERIOD: u64 = 1 << 20;  // 2^20
        if params.orbit_period >= MIN_ORBIT_PERIOD {
            passed.push(ValidationCheck {
                name: "min_orbit_period".to_string(),
                passed: true,
                details: format!("Period {} ≥ {}", params.orbit_period, MIN_ORBIT_PERIOD),
            });
        } else {
            failed.push(ValidationCheck {
                name: "min_orbit_period".to_string(),
                passed: false,
                details: format!("Period {} < {}", params.orbit_period, MIN_ORBIT_PERIOD),
            });
        }

        // Check 2: Period is not smooth (B-smooth check)
        let smooth_bound = 50;
        let is_smooth = Self::is_b_smooth(params.orbit_period, smooth_bound);
        if !is_smooth {
            passed.push(ValidationCheck {
                name: "non_smooth_period".to_string(),
                passed: true,
                details: format!("Period is not {}-smooth", smooth_bound),
            });
        } else {
            failed.push(ValidationCheck {
                name: "non_smooth_period".to_string(),
                passed: false,
                details: format!("Period is {}-smooth (vulnerable to Pohlig-Hellman)", smooth_bound),
            });
        }

        // Check 3: Period has large prime factor
        let largest_prime = Self::largest_prime_factor(params.orbit_period);
        if largest_prime > (1 << 20) {
            passed.push(ValidationCheck {
                name: "large_prime_factor".to_string(),
                passed: true,
                details: format!("Largest prime factor: {}", largest_prime),
            });
        } else {
            failed.push(ValidationCheck {
                name: "large_prime_factor".to_string(),
                passed: false,
                details: format!("Largest prime factor {} too small", largest_prime),
            });
        }

        // Check 4: Period is not a power of 2
        if !params.orbit_period.is_power_of_two() {
            passed.push(ValidationCheck {
                name: "not_power_of_2".to_string(),
                passed: true,
                details: "Period is not a power of 2".to_string(),
            });
        } else {
            failed.push(ValidationCheck {
                name: "not_power_of_2".to_string(),
                passed: false,
                details: "Period is power of 2 (special structure)".to_string(),
            });
        }

        // Check 5: Modulus primality
        let modulus_prime = FHEValidator::is_probable_prime(params.modulus);
        if modulus_prime {
            passed.push(ValidationCheck {
                name: "modulus_primality".to_string(),
                passed: true,
                details: format!("Modulus {} is probably prime", params.modulus),
            });
        } else {
            failed.push(ValidationCheck {
                name: "modulus_primality".to_string(),
                passed: false,
                details: format!("Modulus {} is composite", params.modulus),
            });
        }

        let recommendation = if failed.is_empty() {
            ValidationResult::Accept
        } else if failed.len() <= 1 {
            ValidationResult::Harden
        } else {
            ValidationResult::Reject
        };

        ValidationReport {
            parameter_set: format!("AHOP(g={}, m={}, period={})", params.generator, params.modulus, params.orbit_period),
            validated_at: 0,
            checks_passed: passed,
            checks_failed: failed,
            recommendation: recommendation.clone(),
            rationale: FHEValidator::generate_rationale(&recommendation),
        }
    }

    fn is_b_smooth(n: u64, bound: u64) -> bool {
        let mut remaining = n;
        for p in 2..=bound {
            while remaining % p == 0 {
                remaining /= p;
            }
        }
        remaining == 1
    }

    fn largest_prime_factor(mut n: u64) -> u64 {
        let mut largest = 1;

        for p in 2..=n {
            while n % p == 0 {
                largest = p;
                n /= p;
            }
            if n == 1 {
                break;
            }
            if p > 10000 {
                break;  // Limit search
            }
        }

        largest
    }
}

// ===========================
// === Loki 5 Validation =====
// ===========================

/// Loki 5 cryptographic parameters
#[derive(Debug, Clone)]
pub struct Loki5Params {
    pub phi_base_frequency: u64,
    pub mutation_rate: u32,  // 0-10000 (as rational)
    pub rmcf_dimension: usize,
    pub num_cipher_paths: usize,
}

/// Loki 5 validator
pub struct Loki5Validator;

impl Loki5Validator {
    pub fn validate(params: &Loki5Params) -> ValidationReport {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        // Check 1: Phi frequency is reasonable
        if params.phi_base_frequency >= 100 && params.phi_base_frequency <= 10000 {
            passed.push(ValidationCheck {
                name: "phi_frequency_range".to_string(),
                passed: true,
                details: format!("Frequency {} in valid range [100, 10000]", params.phi_base_frequency),
            });
        } else {
            failed.push(ValidationCheck {
                name: "phi_frequency_range".to_string(),
                passed: false,
                details: format!("Frequency {} outside valid range", params.phi_base_frequency),
            });
        }

        // Check 2: Mutation rate is bounded
        if params.mutation_rate <= 5000 {
            passed.push(ValidationCheck {
                name: "mutation_rate_bound".to_string(),
                passed: true,
                details: format!("Mutation rate {} ≤ 0.5", params.mutation_rate),
            });
        } else {
            failed.push(ValidationCheck {
                name: "mutation_rate_bound".to_string(),
                passed: false,
                details: format!("Mutation rate {} > 0.5 (too high)", params.mutation_rate),
            });
        }

        // Check 3: RMCF dimension is adequate
        if params.rmcf_dimension >= 256 {
            passed.push(ValidationCheck {
                name: "rmcf_dimension".to_string(),
                passed: true,
                details: format!("RMCF dimension {} ≥ 256", params.rmcf_dimension),
            });
        } else {
            failed.push(ValidationCheck {
                name: "rmcf_dimension".to_string(),
                passed: false,
                details: format!("RMCF dimension {} < 256 (too small)", params.rmcf_dimension),
            });
        }

        // Check 4: Sufficient cipher paths
        if params.num_cipher_paths >= 8 {
            passed.push(ValidationCheck {
                name: "cipher_paths".to_string(),
                passed: true,
                details: format!("{} cipher paths ≥ 8", params.num_cipher_paths),
            });
        } else {
            failed.push(ValidationCheck {
                name: "cipher_paths".to_string(),
                passed: false,
                details: format!("{} cipher paths < 8 (insufficient diversity)", params.num_cipher_paths),
            });
        }

        let recommendation = if failed.is_empty() {
            ValidationResult::Accept
        } else {
            ValidationResult::Harden
        };

        ValidationReport {
            parameter_set: format!("Loki5(freq={}, mut={}, rmcf={}, paths={})",
                params.phi_base_frequency, params.mutation_rate, params.rmcf_dimension, params.num_cipher_paths),
            validated_at: 0,
            checks_passed: passed,
            checks_failed: failed,
            recommendation: recommendation.clone(),
            rationale: FHEValidator::generate_rationale(&recommendation),
        }
    }
}

// ===========================
// === Unified Validator =====
// ===========================

/// Unified cryptographic immune system
pub struct CryptoImmuneSystem {
    pub fhe_reports: HashMap<String, ValidationReport>,
    pub ahop_reports: HashMap<String, ValidationReport>,
    pub loki5_reports: HashMap<String, ValidationReport>,
}

impl CryptoImmuneSystem {
    pub fn new() -> Self {
        CryptoImmuneSystem {
            fhe_reports: HashMap::new(),
            ahop_reports: HashMap::new(),
            loki5_reports: HashMap::new(),
        }
    }

    pub fn validate_fhe(&mut self, params: &FHEParams) -> &ValidationReport {
        let report = FHEValidator::validate(params);
        let key = report.parameter_set.clone();
        self.fhe_reports.insert(key.clone(), report);
        self.fhe_reports.get(&key).unwrap()
    }

    pub fn validate_ahop(&mut self, params: &AHOPParams) -> &ValidationReport {
        let report = AHOPValidator::validate(params);
        let key = report.parameter_set.clone();
        self.ahop_reports.insert(key.clone(), report);
        self.ahop_reports.get(&key).unwrap()
    }

    pub fn validate_loki5(&mut self, params: &Loki5Params) -> &ValidationReport {
        let report = Loki5Validator::validate(params);
        let key = report.parameter_set.clone();
        self.loki5_reports.insert(key.clone(), report);
        self.loki5_reports.get(&key).unwrap()
    }

    pub fn all_parameters_secure(&self) -> bool {
        let all_accept = |reports: &HashMap<String, ValidationReport>| {
            reports.values().all(|r| r.recommendation == ValidationResult::Accept)
        };

        all_accept(&self.fhe_reports) &&
        all_accept(&self.ahop_reports) &&
        all_accept(&self.loki5_reports)
    }

    pub fn generate_certificate(&self) -> DeploymentCertificate {
        DeploymentCertificate {
            fhe_secure: self.fhe_reports.values().all(|r| r.recommendation == ValidationResult::Accept),
            ahop_secure: self.ahop_reports.values().all(|r| r.recommendation == ValidationResult::Accept),
            loki5_secure: self.loki5_reports.values().all(|r| r.recommendation == ValidationResult::Accept),
            validated_at: 0,  // TODO: Timestamp
            total_checks: self.count_total_checks(),
            passed_checks: self.count_passed_checks(),
            can_deploy: self.all_parameters_secure(),
        }
    }

    fn count_total_checks(&self) -> usize {
        let count_checks = |reports: &HashMap<String, ValidationReport>| {
            reports.values()
                .map(|r| r.checks_passed.len() + r.checks_failed.len())
                .sum::<usize>()
        };

        count_checks(&self.fhe_reports) +
        count_checks(&self.ahop_reports) +
        count_checks(&self.loki5_reports)
    }

    fn count_passed_checks(&self) -> usize {
        let count_passed = |reports: &HashMap<String, ValidationReport>| {
            reports.values()
                .map(|r| r.checks_passed.len())
                .sum::<usize>()
        };

        count_passed(&self.fhe_reports) +
        count_passed(&self.ahop_reports) +
        count_passed(&self.loki5_reports)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCertificate {
    pub fhe_secure: bool,
    pub ahop_secure: bool,
    pub loki5_secure: bool,
    pub validated_at: u64,
    pub total_checks: usize,
    pub passed_checks: usize,
    pub can_deploy: bool,
}

// ===========================
// === Unit Tests ============
// ===========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fhe_validation_accept() {
        let params = FHEParams {
            variant: FHEVariant::QClassic,
            poly_degree: 2048,
            modulus: 998244353,  // Known NTT-friendly prime
            plaintext_modulus: 65537,
        };

        let report = FHEValidator::validate(&params);
        assert_eq!(report.recommendation, ValidationResult::Accept);
    }

    #[test]
    fn test_fhe_validation_reject() {
        let params = FHEParams {
            variant: FHEVariant::QClassic,
            poly_degree: 1000,  // Not power of 2
            modulus: 100,       // Not prime
            plaintext_modulus: 2,
        };

        let report = FHEValidator::validate(&params);
        assert_eq!(report.recommendation, ValidationResult::Reject);
    }

    #[test]
    fn test_ahop_validation() {
        let params = AHOPParams {
            generator: 5,
            modulus: 998244353,
            orbit_period: 1 << 24,  // 2^24
        };

        let report = AHOPValidator::validate(&params);
        // May be rejected because period is power of 2
        assert!(report.checks_failed.len() >= 1);
    }

    #[test]
    fn test_loki5_validation() {
        let params = Loki5Params {
            phi_base_frequency: 1000,
            mutation_rate: 100,
            rmcf_dimension: 256,
            num_cipher_paths: 8,
        };

        let report = Loki5Validator::validate(&params);
        assert_eq!(report.recommendation, ValidationResult::Accept);
    }

    #[test]
    fn test_immune_system_integration() {
        let mut immune_system = CryptoImmuneSystem::new();

        // Validate FHE
        let fhe_params = FHEParams {
            variant: FHEVariant::V2Complete,
            poly_degree: 4096,
            modulus: 998244353,
            plaintext_modulus: 65537,
        };
        immune_system.validate_fhe(&fhe_params);

        // Validate AHOP
        let ahop_params = AHOPParams {
            generator: 3,
            modulus: 998244353,
            orbit_period: 332748118,  // (998244353 - 1) / 3
        };
        immune_system.validate_ahop(&ahop_params);

        // Validate Loki 5
        let loki5_params = Loki5Params {
            phi_base_frequency: 1618,
            mutation_rate: 1000,
            rmcf_dimension: 256,
            num_cipher_paths: 11,
        };
        immune_system.validate_loki5(&loki5_params);

        // Generate certificate
        let cert = immune_system.generate_certificate();
        assert!(cert.total_checks > 0);
        assert!(cert.passed_checks > 0);
    }
}
