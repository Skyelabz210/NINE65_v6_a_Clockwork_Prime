//! RedShirt Testbed - Encryption Attack Sandbox
//!
//! This creates encrypted messages using state-of-the-art parameters
//! and then attempts to break them using our cryptanalysis tools.
//!
//! Test Targets:
//! 1. Weak RSA (deliberately vulnerable)
//! 2. Kyber-512 equivalent (NIST standard)
//! 3. QMNF-Standard (our production parameters)
//! 4. QMNF-Light variants (testing the fix)

use std::f64::consts::PI;

// ============================================================================
// ENCRYPTION TARGETS
// ============================================================================

/// Simulated encrypted message with associated parameters
#[derive(Debug)]
struct EncryptedTarget {
    name: &'static str,
    scheme: CryptoScheme,
    ciphertext: Vec<u8>,  // Simulated ciphertext
    public_params: PublicParams,
}

#[derive(Debug, Clone)]
enum CryptoScheme {
    RSA { bits: usize },
    RingLWE { n: usize, log_q: f64, sigma: f64 },
    ModuleLWE { k: usize, n: usize, log_q: f64, sigma: f64 },
    AES { bits: usize },
}

#[derive(Debug)]
struct PublicParams {
    description: String,
    extractable: bool,
}

// ============================================================================
// TARGET GENERATION
// ============================================================================

fn create_test_targets() -> Vec<EncryptedTarget> {
    vec![
        // Target 1: Deliberately weak RSA (for demonstration)
        EncryptedTarget {
            name: "WEAK-RSA-1024",
            scheme: CryptoScheme::RSA { bits: 1024 },
            ciphertext: vec![0xDE, 0xAD, 0xBE, 0xEF], // Simulated
            public_params: PublicParams {
                description: "RSA-1024 (DEPRECATED - known weak)".into(),
                extractable: true,
            },
        },

        // Target 2: Strong RSA
        EncryptedTarget {
            name: "RSA-4096",
            scheme: CryptoScheme::RSA { bits: 4096 },
            ciphertext: vec![0xCA, 0xFE, 0xBA, 0xBE],
            public_params: PublicParams {
                description: "RSA-4096 (current standard)".into(),
                extractable: true,
            },
        },

        // Target 3: Kyber-512 equivalent
        EncryptedTarget {
            name: "KYBER-512-EQUIV",
            scheme: CryptoScheme::ModuleLWE { k: 2, n: 256, log_q: 11.7, sigma: 1.0 },
            ciphertext: vec![0x4B, 0x59, 0x42, 0x45, 0x52], // "KYBER"
            public_params: PublicParams {
                description: "Module-LWE k=2, n=256, q=3329 (Kyber-512)".into(),
                extractable: true,
            },
        },

        // Target 4: Kyber-1024 equivalent
        EncryptedTarget {
            name: "KYBER-1024-EQUIV",
            scheme: CryptoScheme::ModuleLWE { k: 4, n: 256, log_q: 11.7, sigma: 1.0 },
            ciphertext: vec![0x4B, 0x31, 0x30, 0x32, 0x34],
            public_params: PublicParams {
                description: "Module-LWE k=4, n=256, q=3329 (Kyber-1024)".into(),
                extractable: true,
            },
        },

        // Target 5: QMNF-Light INSECURE (original bad params)
        EncryptedTarget {
            name: "QMNF-LIGHT-INSECURE",
            scheme: CryptoScheme::RingLWE { n: 1024, log_q: 30.0, sigma: 3.2 },
            ciphertext: vec![0x51, 0x4D, 0x4E, 0x46], // "QMNF"
            public_params: PublicParams {
                description: "Ring-LWE n=1024, q=2^30, sigma=3.2 (KNOWN VULNERABLE)".into(),
                extractable: true,
            },
        },

        // Target 6: QMNF-Light FIXED
        EncryptedTarget {
            name: "QMNF-LIGHT-FIXED",
            scheme: CryptoScheme::RingLWE { n: 2048, log_q: 30.0, sigma: 3.2 },
            ciphertext: vec![0x51, 0x4D, 0x4E, 0x46, 0x32], // "QMNF2"
            public_params: PublicParams {
                description: "Ring-LWE n=2048, q=2^30, sigma=3.2 (FIXED)".into(),
                extractable: true,
            },
        },

        // Target 7: QMNF-Standard-128 (production)
        EncryptedTarget {
            name: "QMNF-STANDARD-128",
            scheme: CryptoScheme::RingLWE { n: 4096, log_q: 30.0, sigma: 3.2 },
            ciphertext: vec![0x4D, 0x41, 0x4E, 0x41], // "MANA"
            public_params: PublicParams {
                description: "Ring-LWE n=4096, q=2^30, sigma=3.2 (PRODUCTION)".into(),
                extractable: true,
            },
        },

        // Target 8: AES-256 (symmetric baseline)
        EncryptedTarget {
            name: "AES-256",
            scheme: CryptoScheme::AES { bits: 256 },
            ciphertext: vec![0xAE, 0x53, 0x32, 0x35, 0x36],
            public_params: PublicParams {
                description: "AES-256 (symmetric encryption baseline)".into(),
                extractable: false, // No public key to extract
            },
        },
    ]
}

// ============================================================================
// ATTACK ESTIMATION (from RedShirt)
// ============================================================================

fn hermite_factor(beta: usize) -> f64 {
    if beta < 50 { return 1.02; }
    let beta_f = beta as f64;
    let inner = (PI * beta_f).powf(1.0 / beta_f) * beta_f / (2.0 * PI * std::f64::consts::E);
    inner.powf(1.0 / (2.0 * (beta_f - 1.0)))
}

fn estimate_rsa_security(bits: usize) -> (f64, &'static str) {
    // GNFS complexity approximation
    let n = bits as f64;
    let ln_n = n * 0.693; // ln(2^bits) = bits * ln(2)
    let complexity = 1.923 * (ln_n.powf(1.0/3.0)) * ((ln_n).ln().powf(2.0/3.0));
    let security_bits = complexity / 0.693;

    let status = if security_bits < 80.0 { "BROKEN" }
                 else if security_bits < 112.0 { "WEAK" }
                 else if security_bits < 128.0 { "LEGACY" }
                 else { "SECURE" };

    (security_bits, status)
}

fn estimate_lattice_security(n: usize, log_q: f64, sigma: f64) -> (usize, f64, &'static str) {
    let m = n;
    let d = (n + m + 1) as f64;
    let target_norm = sigma * (n as f64).sqrt();
    let log_target = target_norm.log2();
    let log_det = m as f64 * log_q;

    let mut beta = 2000;
    for b in 50..2000 {
        let delta = hermite_factor(b);
        let log_delta = delta.log2();
        let lhs = 0.5 * ((b as f64) / d).log2() + log_target;
        let rhs = (2.0 * b as f64 - d) * log_delta + log_det / d;
        if lhs <= rhs {
            beta = b;
            break;
        }
    }

    let security_bits = 0.292 * beta as f64;
    let status = if security_bits < 80.0 { "BROKEN" }
                 else if security_bits < 112.0 { "WEAK" }
                 else if security_bits < 128.0 { "MARGINAL" }
                 else if security_bits < 192.0 { "SECURE" }
                 else if security_bits < 256.0 { "STRONG" }
                 else { "MAXIMUM" };

    (beta, security_bits, status)
}

fn estimate_aes_security(bits: usize) -> (f64, f64, &'static str) {
    let classical = bits as f64;
    let quantum = bits as f64 / 2.0; // Grover
    let status = if quantum >= 128.0 { "QUANTUM-SAFE" } else { "GROVER-WEAK" };
    (classical, quantum, status)
}

// ============================================================================
// PROBING PHASE
// ============================================================================

fn probe_target(target: &EncryptedTarget) -> String {
    println!("\n{} PROBING: {} {}", "=".repeat(20), target.name, "=".repeat(20));
    println!("Description: {}", target.public_params.description);
    println!("Ciphertext length: {} bytes", target.ciphertext.len());
    println!("Parameters extractable: {}", target.public_params.extractable);

    match &target.scheme {
        CryptoScheme::RSA { bits } => {
            let (security, status) = estimate_rsa_security(*bits);
            println!("\n[PROBE] RSA-{}", bits);
            println!("  Estimated security: {:.1} bits classical", security);
            println!("  Quantum threat: CRITICAL (Shor's algorithm)");
            println!("  Status: {}", status);
            format!("RSA-{}: {:.1} bits - {}", bits, security, status)
        }
        CryptoScheme::RingLWE { n, log_q, sigma } => {
            let (beta, security, status) = estimate_lattice_security(*n, *log_q, *sigma);
            println!("\n[PROBE] Ring-LWE n={}, log(q)={:.1}, sigma={:.1}", n, log_q, sigma);
            println!("  Required BKZ block size: {}", beta);
            println!("  Estimated security: {:.1} bits classical", security);
            println!("  Quantum security: {:.1} bits", 0.265 * beta as f64);
            println!("  Status: {}", status);
            format!("Ring-LWE(n={}): BKZ-{}, {:.1} bits - {}", n, beta, security, status)
        }
        CryptoScheme::ModuleLWE { k, n, log_q, sigma } => {
            let total_n = k * n;
            let (beta, security, status) = estimate_lattice_security(total_n, *log_q, *sigma);
            println!("\n[PROBE] Module-LWE k={}, n={}, log(q)={:.1}", k, n, log_q);
            println!("  Total dimension: {}", total_n);
            println!("  Required BKZ block size: {}", beta);
            println!("  Estimated security: {:.1} bits classical", security);
            println!("  Status: {}", status);
            format!("Module-LWE(k={},n={}): BKZ-{}, {:.1} bits - {}", k, n, beta, security, status)
        }
        CryptoScheme::AES { bits } => {
            let (classical, quantum, status) = estimate_aes_security(*bits);
            println!("\n[PROBE] AES-{}", bits);
            println!("  Classical security: {:.0} bits", classical);
            println!("  Quantum security: {:.0} bits (Grover)", quantum);
            println!("  Status: {}", status);
            format!("AES-{}: {:.0}/{:.0} bits - {}", bits, classical, quantum, status)
        }
    }
}

// ============================================================================
// ATTACK PHASE
// ============================================================================

fn attack_target(target: &EncryptedTarget) -> (bool, String) {
    println!("\n{} ATTACKING: {} {}", ">".repeat(18), target.name, "<".repeat(18));

    let (feasible, reason) = match &target.scheme {
        CryptoScheme::RSA { bits } => {
            let (security, _) = estimate_rsa_security(*bits);
            if security < 80.0 {
                (true, format!("GNFS factorization feasible at {:.0} bits", security))
            } else if security < 100.0 {
                (false, format!("Factorization possible with significant resources ({:.0} bits)", security))
            } else {
                (false, format!("Factorization infeasible ({:.0} bits)", security))
            }
        }
        CryptoScheme::RingLWE { n, log_q, sigma } |
        CryptoScheme::ModuleLWE { n, log_q, sigma, .. } => {
            let total_n = match &target.scheme {
                CryptoScheme::ModuleLWE { k, n, .. } => k * n,
                _ => *n,
            };
            let (beta, security, _) = estimate_lattice_security(total_n, *log_q, *sigma);

            if security < 80.0 {
                (true, format!("BKZ-{} attack feasible at {:.0} bits", beta, security))
            } else if security < 128.0 {
                (false, format!("Attack marginally feasible with nation-state resources (BKZ-{}, {:.0} bits)", beta, security))
            } else {
                (false, format!("Attack infeasible (BKZ-{} requires 2^{:.0} operations)", beta, security))
            }
        }
        CryptoScheme::AES { bits } => {
            (false, format!("Brute force requires 2^{} operations", bits))
        }
    };

    if feasible {
        println!("  [!] VULNERABILITY FOUND: {}", reason);
        println!("  [!] Attack vector: EXPLOITABLE");
    } else {
        println!("  [*] Attack assessment: {}", reason);
        println!("  [*] Attack vector: NOT EXPLOITABLE");
    }

    (feasible, reason)
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("{}", "║       REDSHIRT TESTBED - ENCRYPTION ATTACK SANDBOX                        ║");
    println!("{}", "║       Testing State-of-the-Art Encryption                                 ║");
    println!("{}", "╚═══════════════════════════════════════════════════════════════════════════╝");

    let targets = create_test_targets();

    // ========================================================================
    // PHASE 1: RECONNAISSANCE
    // ========================================================================
    println!("\n{}", "═".repeat(77));
    println!("PHASE 1: RECONNAISSANCE - Identifying Targets");
    println!("{}", "═".repeat(77));

    println!("\nDiscovered {} encryption targets:", targets.len());
    for (i, target) in targets.iter().enumerate() {
        println!("  [{}] {} - {}", i + 1, target.name, target.public_params.description);
    }

    // ========================================================================
    // PHASE 2: PROBING
    // ========================================================================
    println!("\n{}", "═".repeat(77));
    println!("PHASE 2: PROBING - Parameter Extraction & Analysis");
    println!("{}", "═".repeat(77));

    let mut probe_results = Vec::new();
    for target in &targets {
        let result = probe_target(target);
        probe_results.push((target.name, result));
    }

    // ========================================================================
    // PHASE 3: ATTACK
    // ========================================================================
    println!("\n{}", "═".repeat(77));
    println!("PHASE 3: ATTACK - Attempting Cryptanalysis");
    println!("{}", "═".repeat(77));

    let mut vulnerabilities = Vec::new();
    let mut secure_targets = Vec::new();

    for target in &targets {
        let (feasible, reason) = attack_target(target);
        if feasible {
            vulnerabilities.push((target.name, reason));
        } else {
            secure_targets.push((target.name, reason));
        }
    }

    // ========================================================================
    // PHASE 4: REPORT
    // ========================================================================
    println!("\n{}", "═".repeat(77));
    println!("PHASE 4: SECURITY ASSESSMENT REPORT");
    println!("{}", "═".repeat(77));

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│                         VULNERABILITY SUMMARY                          │");
    println!("├─────────────────────────────────────────────────────────────────────────┤");

    if vulnerabilities.is_empty() {
        println!("│  No exploitable vulnerabilities found.                                 │");
    } else {
        println!("│  CRITICAL FINDINGS:                                                    │");
        for (name, reason) in &vulnerabilities {
            println!("│  [!] {}: {}", name, reason);
        }
    }

    println!("├─────────────────────────────────────────────────────────────────────────┤");
    println!("│                           SECURE TARGETS                               │");
    println!("├─────────────────────────────────────────────────────────────────────────┤");

    for (name, reason) in &secure_targets {
        println!("│  [*] {}", name);
        println!("│      {}", reason);
    }

    println!("└─────────────────────────────────────────────────────────────────────────┘");

    // Summary statistics
    println!("\n{}", "═".repeat(77));
    println!("SUMMARY");
    println!("{}", "═".repeat(77));
    println!("Total targets analyzed: {}", targets.len());
    println!("Vulnerable: {} ({}%)", vulnerabilities.len(),
             100 * vulnerabilities.len() / targets.len());
    println!("Secure: {} ({}%)", secure_targets.len(),
             100 * secure_targets.len() / targets.len());

    if !vulnerabilities.is_empty() {
        println!("\n[!] ACTION REQUIRED: {} target(s) have exploitable weaknesses",
                 vulnerabilities.len());
    } else {
        println!("\n[*] All production targets passed security assessment");
    }

    println!("\n{}", "═".repeat(77));
}
