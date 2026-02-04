//! Calibrated Lattice Attack Estimator
//!
//! This tool estimates the security of lattice-based cryptographic schemes
//! using models calibrated against known results from the lattice-estimator.
//!
//! VALIDATION TARGETS:
//! - Kyber-512:  ~118-143 bits (NIST Level 1, BKZ-413)
//! - Kyber-768:  ~182-207 bits (NIST Level 3, BKZ-637)
//! - Kyber-1024: ~256-272 bits (NIST Level 5, BKZ-894)
//!
//! References:
//! - CRYSTALS-Kyber specification
//! - Albrecht et al., "On the concrete hardness of Learning with Errors"
//! - MATZOV dual attack analysis (CRYPTO 2025)
//! - lattice-estimator (github.com/malb/lattice-estimator)

use std::f64::consts::{E, PI};

/// LWE/MLWE Parameters
#[derive(Clone, Debug)]
pub struct LWEParams {
    pub name: &'static str,
    /// Module rank (k for MLWE, 1 for standard LWE)
    pub k: usize,
    /// Ring dimension (n for MLWE, n for LWE)
    pub n: usize,
    /// Modulus
    pub q: u64,
    /// Error distribution standard deviation
    pub sigma: f64,
    /// Secret distribution type
    pub secret_dist: SecretDist,
    /// Number of samples (m)
    pub m: Option<usize>,
    /// Expected security (for validation)
    pub expected_classical_bits: Option<f64>,
    pub expected_quantum_bits: Option<f64>,
}

#[derive(Clone, Debug, Copy)]
pub enum SecretDist {
    /// Uniform in [-eta, eta]
    UniformBounded(i32),
    /// Centered binomial distribution with parameter eta
    CBD(usize),
    /// Ternary {-1, 0, 1}
    Ternary,
    /// Gaussian with given sigma
    Gaussian(f64),
}

impl SecretDist {
    /// Variance of the distribution
    fn variance(&self) -> f64 {
        match self {
            SecretDist::UniformBounded(eta) => {
                // Var[Uniform(-eta, eta)] = (2*eta+1)^2/12 - 1/12
                let range = 2 * (*eta as i64) + 1;
                (range * range - 1) as f64 / 12.0
            }
            SecretDist::CBD(eta) => {
                // CBD_eta has variance eta/2
                *eta as f64 / 2.0
            }
            SecretDist::Ternary => {
                // {-1, 0, 1} uniform: variance = 2/3
                2.0 / 3.0
            }
            SecretDist::Gaussian(sigma) => sigma * sigma,
        }
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}

/// BKZ cost models
#[derive(Clone, Copy, Debug)]
pub enum CostModel {
    /// Core-SVP with sieving: 2^(0.292*b + 16.4)
    CoreSVPSieving,
    /// Core-SVP with enumeration: 2^(0.187*b*log2(b))
    CoreSVPEnum,
    /// MATZOV improved model: 2^(0.257*b + 20)
    MATZOV,
    /// Gate count model (for NIST comparison)
    GateCount,
}

impl CostModel {
    /// Cost in log2 operations for given block size
    fn cost(&self, block_size: usize) -> f64 {
        let b = block_size as f64;
        match self {
            CostModel::CoreSVPSieving => 0.292 * b + 16.4,
            CostModel::CoreSVPEnum => 0.187 * b * b.log2(),
            CostModel::MATZOV => 0.257 * b + 20.0,
            CostModel::GateCount => 0.265 * b + 16.0, // Approximate gate count model
        }
    }

    fn name(&self) -> &'static str {
        match self {
            CostModel::CoreSVPSieving => "Core-SVP (Sieving)",
            CostModel::CoreSVPEnum => "Core-SVP (Enum)",
            CostModel::MATZOV => "MATZOV",
            CostModel::GateCount => "Gate Count",
        }
    }
}

/// Hermite factor from BKZ block size
/// δ = ((πb)^(1/b) * b / (2πe))^(1/(2(b-1)))
fn hermite_factor(block_size: usize) -> f64 {
    let b = block_size as f64;
    if b < 2.0 {
        return 2.0;
    }
    let inner = (PI * b).powf(1.0 / b) * b / (2.0 * PI * E);
    inner.powf(1.0 / (2.0 * (b - 1.0)))
}

/// Find block size needed to achieve target Hermite factor
fn required_block_size(target_delta: f64) -> usize {
    for b in 40..2000 {
        if hermite_factor(b) <= target_delta {
            return b;
        }
    }
    2000
}

/// Gaussian heuristic for shortest vector
/// λ1 ≈ √(dim/(2πe)) * det(Λ)^(1/dim)
fn gaussian_heuristic(dim: usize, log_det: f64) -> f64 {
    let d = dim as f64;
    (d / (2.0 * PI * E)).sqrt() * (log_det / d).exp2()
}

/// Attack result
#[derive(Clone, Debug)]
pub struct AttackResult {
    pub name: &'static str,
    pub block_size: usize,
    pub log2_classical: f64,
    pub log2_quantum: f64,
    pub cost_model: CostModel,
    pub details: String,
}

impl AttackResult {
    fn classical_bits(&self) -> usize {
        self.log2_classical.floor() as usize
    }

    fn quantum_bits(&self) -> usize {
        self.log2_quantum.floor() as usize
    }
}

/// Primal attack (uSVP) on LWE/MLWE using ADPS16 methodology
///
/// The ADPS16 estimate: For LWE with dimension n, modulus q, and error σ,
/// find the smallest β such that:
///   δ^(2β) * q^(n/β) ≤ τ * σ * √(β)
///
/// where δ = ((πβ)^(1/β) * β / (2πe))^(1/(2(β-1))) and τ is a success threshold.
///
/// Simplified formula (from Albrecht's work):
///   β ≈ (n + m) * log(q) / (log^2(q/σ) + (n+m) * log(δ))
pub fn primal_attack(params: &LWEParams, cost_model: CostModel) -> AttackResult {
    // Total dimension for MLWE: k*n
    let total_n = params.k * params.n;

    // Optimal number of samples for primal attack
    let log_q = (params.q as f64).log2();

    // For module-LWE, use m ≈ n samples
    let m = params.m.unwrap_or(total_n);

    // Lattice dimension d = n + m + 1
    let d = (total_n + m + 1) as f64;

    // Secret standard deviation
    let sigma_s = match params.secret_dist {
        SecretDist::CBD(eta) => (eta as f64 / 2.0).sqrt(),
        SecretDist::Ternary => (2.0_f64 / 3.0).sqrt(),
        SecretDist::Gaussian(s) => s,
        SecretDist::UniformBounded(eta) => ((2 * eta + 1) as f64).sqrt() / 3.46,
    };

    // Combined error/secret norm
    // ||v|| = ||(e, s)|| ≈ σ√m + σ_s√n
    let target_norm_sq = params.sigma * params.sigma * m as f64
                        + sigma_s * sigma_s * total_n as f64;
    let target_norm = target_norm_sq.sqrt();

    // ADPS16 Block size estimation
    //
    // The correct ADPS16 success condition (from "Post-quantum key exchange"):
    //   sqrt(β/d) × ||target|| ≤ δ^(2β-d) × det(Λ)^(1/d)
    //
    // Rearranging in log space (using log2):
    //   0.5*log2(β/d) + log2(||target||) ≤ (2β-d)*log2(δ) + log2(det)/d
    //
    // We search for the smallest β where the RHS exceeds the LHS.

    let log_det = m as f64 * log_q; // log2(det) = m * log2(q)
    let log_target = target_norm.log2();

    let mut block_size = 2000; // Start with maximum (attack fails)
    for beta in 50..2000 {
        let delta = hermite_factor(beta);
        let log_delta = delta.log2();

        // LHS: sqrt(β/d) × ||target||
        let lhs = 0.5 * ((beta as f64) / d).log2() + log_target;

        // RHS: δ^(2β-d) × det^(1/d)
        let rhs = (2.0 * beta as f64 - d) * log_delta + log_det / d;

        // Attack succeeds when LHS ≤ RHS (target is short enough to find)
        if lhs <= rhs {
            block_size = beta;
            break;
        }
    }

    let log2_cost = cost_model.cost(block_size);

    // Quantum: Grover gives sqrt speedup on core SVP oracle
    let log2_quantum = 0.265 * block_size as f64; // Quantum sieving

    AttackResult {
        name: "Primal (uSVP)",
        block_size,
        log2_classical: log2_cost,
        log2_quantum,
        cost_model,
        details: format!("d={:.0}, ||target||={:.2}, ADPS16", d, target_norm),
    }
}

/// Dual attack on LWE/MLWE
///
/// Find short v in dual lattice such that <v, e> reveals information.
/// For distinguishing, need ||v|| × σ < q/τ for some threshold τ.
///
/// The dual attack uses similar BKZ reduction but on the dual lattice.
/// For Kyber, primal and dual attacks have very similar costs.
pub fn dual_attack(params: &LWEParams, cost_model: CostModel) -> AttackResult {
    let total_n = params.k * params.n;
    let log_q = (params.q as f64).log2();

    // Use same number of samples as primal
    let m = params.m.unwrap_or(total_n);
    let d = (total_n + m) as f64;

    // Dual lattice has determinant 1/q^m relative to primal
    // The short vector in dual has norm ≈ q^(m/d) after reduction
    //
    // For distinguishing: ||v|| × σ ≤ q/4
    // After BKZ-β: ||v|| ≈ δ^d × q^(m/d)
    //
    // Success condition: δ^d × q^(m/d) × σ ≤ q/4
    // Log form: d×log(δ) + (m/d)×log(q) + log(σ) ≤ log(q) - 2

    let log_sigma = params.sigma.log2();
    let threshold = log_q - 2.0; // log2(q/4)

    let mut block_size = 2000;
    for beta in 50..2000 {
        let delta = hermite_factor(beta);
        let log_delta = delta.log2();

        // ||v|| × σ in log space
        let lhs = d * log_delta + (m as f64 / d) * log_q + log_sigma;

        // Attack succeeds when lhs ≤ threshold
        if lhs <= threshold {
            block_size = beta;
            break;
        }
    }

    let log2_cost = cost_model.cost(block_size);
    let log2_quantum = 0.265 * block_size as f64; // Quantum sieving

    AttackResult {
        name: "Dual",
        block_size,
        log2_classical: log2_cost,
        log2_quantum,
        cost_model,
        details: format!("d={:.0}, m={}", d, m),
    }
}

/// Hybrid attack (meet-in-the-middle + lattice)
///
/// Guess some secret coordinates, reduce to smaller LWE instance.
/// For small secrets (CBD, ternary), guessing is expensive.
///
/// Cost = 3^k × BKZ(n-k) where k is number of guessed coordinates
/// Optimal k minimizes total cost.
pub fn hybrid_attack(params: &LWEParams, cost_model: CostModel) -> AttackResult {
    let total_n = params.k * params.n;

    // Search space size per coordinate
    let search_space = match params.secret_dist {
        SecretDist::CBD(eta) => 2 * eta + 1,
        SecretDist::Ternary => 3,
        SecretDist::UniformBounded(eta) => (2 * eta + 1) as usize,
        SecretDist::Gaussian(_) => 7,
    };

    let mut best_cost = f64::INFINITY;
    let mut best_k = 0;
    let mut best_block = 0;

    // For small secrets, guessing is very expensive
    // Only try guessing a few coordinates
    let max_guess = (total_n / 8).min(100);

    for k in 0..=max_guess {
        let guess_cost = (k as f64) * (search_space as f64).log2();

        let reduced_n = total_n - k;
        if reduced_n < 50 {
            break;
        }

        // Create reduced LWE parameters
        let reduced_params = LWEParams {
            name: "reduced",
            k: 1,
            n: reduced_n,
            q: params.q,
            sigma: params.sigma,
            secret_dist: params.secret_dist,
            m: Some(reduced_n),
            expected_classical_bits: None,
            expected_quantum_bits: None,
        };

        // Get primal attack cost on reduced instance
        let primal = primal_attack(&reduced_params, cost_model);
        let bkz_cost = primal.log2_classical;

        let total = guess_cost + bkz_cost;

        if total < best_cost {
            best_cost = total;
            best_k = k;
            best_block = primal.block_size;
        }
    }

    // If no guessing helps, just use primal
    if best_k == 0 {
        let primal = primal_attack(params, cost_model);
        return AttackResult {
            name: "Hybrid",
            block_size: primal.block_size,
            log2_classical: primal.log2_classical,
            log2_quantum: primal.log2_quantum,
            cost_model,
            details: format!("No guessing advantage, same as primal"),
        };
    }

    // Quantum speedup: Grover on guessing phase
    let guess_bits = (best_k as f64) * (search_space as f64).log2();
    let log2_quantum = (best_cost - guess_bits) + guess_bits / 2.0;

    AttackResult {
        name: "Hybrid",
        block_size: best_block,
        log2_classical: best_cost,
        log2_quantum,
        cost_model,
        details: format!("guess {} coords (2^{:.1}), BKZ-{}", best_k, guess_bits, best_block),
    }
}

/// Run all attacks and return best (lowest cost)
pub fn estimate_security(params: &LWEParams, cost_model: CostModel) -> Vec<AttackResult> {
    vec![
        primal_attack(params, cost_model),
        dual_attack(params, cost_model),
        hybrid_attack(params, cost_model),
    ]
}

/// Get best attack result
pub fn best_attack(params: &LWEParams, cost_model: CostModel) -> AttackResult {
    estimate_security(params, cost_model)
        .into_iter()
        .min_by(|a, b| a.log2_classical.partial_cmp(&b.log2_classical).unwrap())
        .unwrap()
}

// ============================================================================
// KNOWN PARAMETER SETS FOR VALIDATION
// ============================================================================

/// Kyber-512 (ML-KEM-512) - NIST Level 1
pub fn kyber_512() -> LWEParams {
    LWEParams {
        name: "Kyber-512",
        k: 2,
        n: 256,
        q: 3329,
        sigma: 1.0, // CBD with eta=2 has std dev ≈ 1
        secret_dist: SecretDist::CBD(2),
        m: None,
        expected_classical_bits: Some(118.0), // Published: 118-143 bits
        expected_quantum_bits: Some(107.0),
    }
}

/// Kyber-768 (ML-KEM-768) - NIST Level 3
pub fn kyber_768() -> LWEParams {
    LWEParams {
        name: "Kyber-768",
        k: 3,
        n: 256,
        q: 3329,
        sigma: 1.0,
        secret_dist: SecretDist::CBD(2),
        m: None,
        expected_classical_bits: Some(182.0), // Published: 182-207 bits
        expected_quantum_bits: Some(161.0),
    }
}

/// Kyber-1024 (ML-KEM-1024) - NIST Level 5
pub fn kyber_1024() -> LWEParams {
    LWEParams {
        name: "Kyber-1024",
        k: 4,
        n: 256,
        q: 3329,
        sigma: 1.0,
        secret_dist: SecretDist::CBD(2),
        m: None,
        expected_classical_bits: Some(256.0), // Published: 256-272 bits
        expected_quantum_bits: Some(230.0),
    }
}

/// SEAL BFV default (N=4096, log(q)≈109)
pub fn seal_default() -> LWEParams {
    LWEParams {
        name: "SEAL-BFV-4096",
        k: 1,
        n: 4096,
        q: 0xFFFFFFFFFFFFFFFF, // ~64 bits (simplified)
        sigma: 3.2,
        secret_dist: SecretDist::Ternary,
        m: None,
        expected_classical_bits: Some(128.0),
        expected_quantum_bits: None,
    }
}

/// OpenFHE BGV default
pub fn openfhe_default() -> LWEParams {
    LWEParams {
        name: "OpenFHE-BGV",
        k: 1,
        n: 8192,
        q: 0xFFFFFFFFFFFFFFFF,
        sigma: 3.2,
        secret_dist: SecretDist::Ternary,
        m: None,
        expected_classical_bits: Some(128.0),
        expected_quantum_bits: None,
    }
}

/// QMNF Light config
pub fn qmnf_light() -> LWEParams {
    LWEParams {
        name: "QMNF-Light",
        k: 1,
        n: 1024,
        q: 998244353,
        sigma: 1.0,
        secret_dist: SecretDist::CBD(2),
        m: None,
        expected_classical_bits: Some(80.0),
        expected_quantum_bits: None,
    }
}

/// QMNF Standard 128
pub fn qmnf_standard_128() -> LWEParams {
    LWEParams {
        name: "QMNF-Standard-128",
        k: 1,
        n: 4096,
        q: 998244353,
        sigma: 1.22,
        secret_dist: SecretDist::CBD(3),
        m: None,
        expected_classical_bits: Some(128.0),
        expected_quantum_bits: None,
    }
}

/// QMNF High 192
pub fn qmnf_high_192() -> LWEParams {
    LWEParams {
        name: "QMNF-High-192",
        k: 1,
        n: 8192,
        q: 998244353,
        sigma: 1.22,
        secret_dist: SecretDist::CBD(3),
        m: None,
        expected_classical_bits: Some(192.0),
        expected_quantum_bits: None,
    }
}

// ============================================================================
// VALIDATION AND REPORTING
// ============================================================================

pub fn validate_estimator() {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║           CALIBRATED LATTICE ATTACK ESTIMATOR - VALIDATION               ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Validating against known security estimates from lattice-estimator and NIST.");
    println!();

    let test_cases = vec![
        kyber_512(),
        kyber_768(),
        kyber_1024(),
    ];

    let cost_models = vec![
        CostModel::CoreSVPSieving,
        CostModel::MATZOV,
    ];

    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("KYBER VALIDATION (NIST Post-Quantum Standard)");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();

    for params in &test_cases {
        println!("┌───────────────────────────────────────────────────────────────────────────┐");
        println!("│ {:^73} │", params.name);
        println!("├───────────────────────────────────────────────────────────────────────────┤");
        println!("│ Parameters: k={}, n={}, q={}, σ={:.2}                              │",
                 params.k, params.n, params.q, params.sigma);
        if let Some(expected) = params.expected_classical_bits {
            println!("│ Expected (published): ~{:.0} bits classical                              │", expected);
        }
        println!("├───────────────────────────────────────────────────────────────────────────┤");

        for cost_model in &cost_models {
            let attacks = estimate_security(params, *cost_model);
            let best = attacks.iter().min_by(|a, b|
                a.log2_classical.partial_cmp(&b.log2_classical).unwrap()).unwrap();

            println!("│ Cost Model: {:20}                                          │", cost_model.name());
            println!("│   Best Attack: {:15} BKZ-{:<4} = {:.1} bits classical       │",
                     best.name, best.block_size, best.log2_classical);

            // Validation check
            if let Some(expected) = params.expected_classical_bits {
                let diff = best.log2_classical - expected;
                let status = if diff.abs() < 20.0 { "✓ PASS" } else { "✗ FAIL" };
                println!("│   Difference from expected: {:+.1} bits  {}                        │",
                         diff, status);
            }
            println!("│                                                                           │");
        }
        println!("└───────────────────────────────────────────────────────────────────────────┘");
        println!();
    }
}

pub fn analyze_all_schemes() {
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("COMPREHENSIVE SCHEME ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();

    let schemes = vec![
        kyber_512(),
        kyber_768(),
        kyber_1024(),
        seal_default(),
        openfhe_default(),
        qmnf_light(),
        qmnf_standard_128(),
        qmnf_high_192(),
    ];

    println!("┌─────────────────────┬────────┬────────┬───────────┬───────────┬──────────┐");
    println!("│ Scheme              │ N      │ log(q) │ Classical │ Quantum   │ BKZ-b    │");
    println!("├─────────────────────┼────────┼────────┼───────────┼───────────┼──────────┤");

    for params in &schemes {
        let total_n = params.k * params.n;
        let log_q = (params.q as f64).log2();

        let best = best_attack(&params, CostModel::CoreSVPSieving);

        println!("│ {:19} │ {:6} │ {:6.1} │ {:9.1} │ {:9.1} │ {:8} │",
                 params.name,
                 total_n,
                 log_q,
                 best.log2_classical,
                 best.log2_quantum,
                 best.block_size);
    }

    println!("└─────────────────────┴────────┴────────┴───────────┴───────────┴──────────┘");
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║       MANA FHE - CALIBRATED ATTACK ESTIMATOR                              ║");
    println!("║       Proving Our Tools Against Known Standards                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("This tool validates our security estimates against published results.");
    println!();
    println!("References:");
    println!("  - CRYSTALS-Kyber: https://pq-crystals.org/kyber/");
    println!("  - Lattice Estimator: https://github.com/malb/lattice-estimator");
    println!("  - NIST PQC: https://csrc.nist.gov/projects/post-quantum-cryptography");
    println!();

    validate_estimator();
    analyze_all_schemes();

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("CONCLUSION");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();
    println!("If our estimates match Kyber's published security levels (±20 bits),");
    println!("then our tools are properly calibrated for analyzing QMNF/MANA FHE.");
    println!();
}
