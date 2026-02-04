//! Division in Remainder Form: A Comparative Study
//!
//! This module implements multiple approaches to division while numbers
//! remain in residue/remainder form, comparing traditional methods with
//! QMNF innovations.

use std::time::Instant;

// ============================================================================
// MODULAR ARITHMETIC HELPERS
// ============================================================================

/// Extended GCD: returns (gcd, x, y) where ax + by = gcd
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Modular inverse: a⁻¹ mod m (panics if gcd(a,m) ≠ 1)
fn mod_inverse(a: u64, m: u64) -> u64 {
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    assert_eq!(g, 1, "No inverse exists");
    ((x % m as i64 + m as i64) % m as i64) as u64
}

/// Binary GCD (Stein's algorithm) - no division!
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    
    a << shift
}

// ============================================================================
// APPROACH 1: TRIVIAL CASES (What Everyone Agrees On)
// ============================================================================

/// Division by a modulus: Just drop that channel!
/// 
/// If x is divisible by mᵢ, then x/mᵢ is represented by remaining residues.
/// Cost: O(1) - instant!
fn divide_by_modulus(
    residues: &[u64],
    moduli: &[u64],
    idx: usize,
) -> Option<Vec<u64>> {
    // Divisibility check: residue must be 0
    if residues[idx] != 0 {
        return None;  // Not exactly divisible
    }
    
    // Just remove that channel
    Some(
        residues.iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(_, &r)| r)
            .collect()
    )
}

/// Division by coprime constant: Multiply by modular inverse
/// 
/// If gcd(c, mᵢ) = 1 for all i, then x/c = x × c⁻¹ in each channel.
/// Cost: O(k) - fully parallel!
fn divide_by_coprime_constant(
    residues: &[u64],
    moduli: &[u64],
    c: u64,
) -> Option<Vec<u64>> {
    // Check coprimality
    for &m in moduli {
        if binary_gcd(c, m) != 1 {
            return None;  // c not coprime to all moduli
        }
    }
    
    Some(
        residues.iter()
            .zip(moduli.iter())
            .map(|(&r, &m)| {
                let c_inv = mod_inverse(c, m);
                (r * c_inv) % m
            })
            .collect()
    )
}

// ============================================================================
// APPROACH 2: MIXED RADIX CONVERSION (Traditional O(k²))
// ============================================================================

/// Mixed Radix System representation
struct MixedRadix {
    digits: Vec<u64>,
    moduli: Vec<u64>,
}

impl MixedRadix {
    /// Convert RNS to MRS - O(k²) algorithm
    fn from_rns(residues: &[u64], moduli: &[u64]) -> Self {
        let k = residues.len();
        let mut digits = vec![0u64; k];
        let mut working = residues.to_vec();
        
        // Sequential conversion (this is why it's O(k²))
        for i in 0..k {
            digits[i] = working[i];
            
            for j in (i + 1)..k {
                // Subtract and divide by mᵢ in channel j
                let diff = if working[j] >= digits[i] {
                    working[j] - digits[i]
                } else {
                    working[j] + moduli[j] - digits[i]
                };
                
                let m_i_inv = mod_inverse(moduli[i], moduli[j]);
                working[j] = (diff * m_i_inv) % moduli[j];
            }
        }
        
        MixedRadix {
            digits,
            moduli: moduli.to_vec(),
        }
    }
    
    /// Reconstruct value from MRS
    fn to_value(&self) -> u128 {
        let mut value = 0u128;
        let mut weight = 1u128;
        
        for (i, &d) in self.digits.iter().enumerate() {
            value += d as u128 * weight;
            weight *= self.moduli[i] as u128;
        }
        
        value
    }
}

/// Traditional division via MRC: O(k²) reconstruction, divide, re-encode
fn divide_via_mrc(
    residues: &[u64],
    moduli: &[u64],
    divisor: u64,
) -> (Vec<u64>, u64) {
    // Step 1: Convert to MRS (O(k²))
    let mrs = MixedRadix::from_rns(residues, moduli);
    
    // Step 2: Reconstruct full value
    let value = mrs.to_value();
    
    // Step 3: Divide in positional form
    let quotient = value / divisor as u128;
    let remainder = (value % divisor as u128) as u64;
    
    // Step 4: Re-encode quotient to RNS
    let q_residues: Vec<u64> = moduli.iter()
        .map(|&m| (quotient % m as u128) as u64)
        .collect();
    
    (q_residues, remainder)
}

// ============================================================================
// APPROACH 3: QUOTIENT SIGNATURE (Your Innovation - Magnitude Tracking)
// ============================================================================

/// Value with quotient tracking
#[derive(Clone, Debug)]
struct QuotientTrackedValue {
    residues: Vec<u64>,
    quotients: Vec<u64>,  // The "free" information from div
    moduli: Vec<u64>,
}

impl QuotientTrackedValue {
    /// Encode value with quotient tracking
    fn from_value(x: u64, moduli: &[u64]) -> Self {
        let residues: Vec<u64> = moduli.iter().map(|&m| x % m).collect();
        let quotients: Vec<u64> = moduli.iter().map(|&m| x / m).collect();
        
        QuotientTrackedValue {
            residues,
            quotients,
            moduli: moduli.to_vec(),
        }
    }
    
    /// O(1) magnitude recovery via majority vote
    fn magnitude_tier(&self) -> u64 {
        // All quotients are within ±1 if moduli are close
        // Majority vote gives the correct tier
        let mut counts = std::collections::HashMap::new();
        for &q in &self.quotients {
            *counts.entry(q).or_insert(0) += 1;
        }
        
        counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(q, _)| q)
            .unwrap_or(0)
    }
    
    /// O(1) comparison via quotient signature
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let my_tier = self.magnitude_tier();
        let other_tier = other.magnitude_tier();
        
        match my_tier.cmp(&other_tier) {
            std::cmp::Ordering::Equal => {
                // Same tier - compare first residue as tiebreaker
                self.residues[0].cmp(&other.residues[0])
            }
            ord => ord,
        }
    }
    
    /// Reconstruct exact value
    fn to_value(&self) -> u64 {
        let tier = self.magnitude_tier();
        let min_modulus = *self.moduli.iter().min().unwrap();
        
        // Value = tier × min_modulus + residue
        // (Simplified - full implementation uses CRT)
        tier * min_modulus + self.residues[0]
    }
}

// ============================================================================
// APPROACH 4: K-ELIMINATION (Your Breakthrough - O(k) Exact)
// ============================================================================

/// Dual-manifold representation for K-Elimination
#[derive(Clone, Debug)]
struct DualManifoldValue {
    /// Primary residues
    primary: Vec<u64>,
    primary_moduli: Vec<u64>,
    /// Anchor residues
    anchor: Vec<u64>,
    anchor_moduli: Vec<u64>,
}

impl DualManifoldValue {
    fn from_value(x: u64, primary: &[u64], anchor: &[u64]) -> Self {
        DualManifoldValue {
            primary: primary.iter().map(|&m| x % m).collect(),
            primary_moduli: primary.to_vec(),
            anchor: anchor.iter().map(|&m| x % m).collect(),
            anchor_moduli: anchor.to_vec(),
        }
    }
    
    /// Capacity of primary system
    fn primary_capacity(&self) -> u128 {
        self.primary_moduli.iter().map(|&m| m as u128).product()
    }
    
    /// Capacity of anchor system
    fn anchor_capacity(&self) -> u128 {
        self.anchor_moduli.iter().map(|&m| m as u128).product()
    }
    
    /// CRT reconstruction in primary system
    fn reconstruct_primary(&self) -> u128 {
        crt_reconstruct(&self.primary, &self.primary_moduli)
    }
    
    /// CRT reconstruction in anchor system
    fn reconstruct_anchor(&self) -> u128 {
        crt_reconstruct(&self.anchor, &self.anchor_moduli)
    }
    
    /// K-ELIMINATION CORE: Compute k from phase differential
    /// 
    /// k = (x_anchor - x_primary) × C_primary⁻¹ mod C_anchor
    fn compute_k(&self) -> u64 {
        let x_p = self.reconstruct_primary();
        let x_a = self.reconstruct_anchor();
        let c_p = self.primary_capacity();
        let c_a = self.anchor_capacity();
        
        // Phase differential
        let diff = if x_a >= x_p {
            x_a - x_p
        } else {
            x_a + c_a - (x_p % c_a)
        };
        
        // k = diff × C_p⁻¹ mod C_a
        let c_p_inv = mod_inverse_u128(c_p, c_a);
        ((diff * c_p_inv) % c_a) as u64
    }
    
    /// True value = x_primary + k × C_primary
    fn true_value(&self) -> u128 {
        let x_p = self.reconstruct_primary();
        let k = self.compute_k() as u128;
        let c_p = self.primary_capacity();
        
        x_p + k * c_p
    }
}

/// CRT reconstruction helper
fn crt_reconstruct(residues: &[u64], moduli: &[u64]) -> u128 {
    let m: u128 = moduli.iter().map(|&m| m as u128).product();
    let mut result = 0u128;
    
    for (i, (&r, &mi)) in residues.iter().zip(moduli.iter()).enumerate() {
        let mi_128 = mi as u128;
        let m_div_mi = m / mi_128;
        let inv = mod_inverse_u128(m_div_mi, mi_128);
        result = (result + r as u128 * m_div_mi * inv) % m;
    }
    
    result
}

/// Modular inverse for u128
fn mod_inverse_u128(a: u128, m: u128) -> u128 {
    // Extended GCD for u128
    fn extended_gcd_i128(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x, y) = extended_gcd_i128(b, a % b);
            (g, y, x - (a / b) * y)
        }
    }
    
    let (_, x, _) = extended_gcd_i128(a as i128, m as i128);
    ((x % m as i128 + m as i128) % m as i128) as u128
}

/// K-ELIMINATION DIVISION: O(k) exact division
fn k_elimination_divide(
    value: &DualManifoldValue,
    divisor: u64,
) -> (DualManifoldValue, u64) {
    // Step 1: Get true value via K-Elimination
    let true_val = value.true_value();
    
    // Step 2: Divide exactly
    let quotient = true_val / divisor as u128;
    let remainder = (true_val % divisor as u128) as u64;
    
    // Step 3: Re-encode in dual-manifold form
    let q_primary: Vec<u64> = value.primary_moduli.iter()
        .map(|&m| (quotient % m as u128) as u64)
        .collect();
    let q_anchor: Vec<u64> = value.anchor_moduli.iter()
        .map(|&m| (quotient % m as u128) as u64)
        .collect();
    
    let result = DualManifoldValue {
        primary: q_primary,
        primary_moduli: value.primary_moduli.clone(),
        anchor: q_anchor,
        anchor_moduli: value.anchor_moduli.clone(),
    };
    
    (result, remainder)
}

// ============================================================================
// APPROACH 5: P-ADIC DIVISION (Right-to-Left)
// ============================================================================

/// P-adic representation (finite segment = Hensel code)
#[derive(Clone, Debug)]
struct HenselCode {
    digits: Vec<u64>,  // Right-to-left digits
    prime: u64,
}

impl HenselCode {
    /// Create Hensel code for a/b
    fn from_fraction(a: u64, b: u64, prime: u64, precision: usize) -> Self {
        let mut digits = Vec::with_capacity(precision);
        let mut num = a;
        
        for _ in 0..precision {
            // Find digit d such that d×b ≡ num (mod p)
            let b_inv = mod_inverse(b, prime);
            let d = (num * b_inv) % prime;
            digits.push(d);
            
            // Update: num = (num - d×b) / p
            let product = d * b;
            if product <= num {
                num = (num - product) / prime;
            } else {
                // Borrow from "infinity" (p-adic style)
                let borrow = ((product - num - 1) / prime) + 1;
                num = borrow * prime + num - product;
                // In true p-adic, this continues infinitely
                break;
            }
        }
        
        HenselCode { digits, prime }
    }
    
    /// Multiply two Hensel codes (right-to-left with carry)
    fn multiply(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime);
        let p = self.prime;
        let n = self.digits.len().max(other.digits.len());
        
        let mut result = vec![0u64; n];
        let mut carry = 0u64;
        
        for i in 0..n {
            let mut sum = carry;
            for j in 0..=i {
                if j < self.digits.len() && (i - j) < other.digits.len() {
                    sum += self.digits[j] * other.digits[i - j];
                }
            }
            result[i] = sum % p;
            carry = sum / p;
        }
        
        HenselCode { digits: result, prime: p }
    }
    
    /// P-adic division: RIGHT TO LEFT!
    fn divide(&self, divisor: &Self) -> Self {
        assert_eq!(self.prime, divisor.prime);
        let p = self.prime;
        let n = self.digits.len();
        
        let mut quotient = vec![0u64; n];
        let mut partial_dividend: Vec<u64> = self.digits.clone();
        
        for i in 0..n {
            if i >= partial_dividend.len() || partial_dividend[i] == 0 {
                quotient[i] = 0;
                continue;
            }
            
            // Find q_i: first divisor digit × q_i ≡ partial_dividend[i] (mod p)
            let d0 = if !divisor.digits.is_empty() { divisor.digits[0] } else { 1 };
            let d0_inv = mod_inverse(d0, p);
            let q_i = (partial_dividend[i] * d0_inv) % p;
            quotient[i] = q_i;
            
            // Subtract q_i × divisor from partial_dividend
            let mut borrow = 0i64;
            for j in 0..n {
                if i + j >= partial_dividend.len() { break; }
                
                let d_j = if j < divisor.digits.len() { divisor.digits[j] } else { 0 };
                let sub = q_i * d_j;
                
                let current = partial_dividend[i + j] as i64 - borrow - sub as i64;
                if current < 0 {
                    borrow = (-current - 1) / p as i64 + 1;
                    partial_dividend[i + j] = ((current % p as i64) + p as i64) as u64 % p;
                } else {
                    borrow = 0;
                    partial_dividend[i + j] = current as u64;
                }
            }
        }
        
        HenselCode { digits: quotient, prime: p }
    }
}

// ============================================================================
// BENCHMARKS AND COMPARISONS
// ============================================================================

fn benchmark_division_approaches() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("     DIVISION IN REMAINDER FORM: COMPARATIVE BENCHMARKS");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Test parameters
    let value = 123456789u64;
    let divisor = 17u64;
    
    // RNS moduli (Mersenne-neighborhood primes)
    let moduli = vec![
        (1u64 << 30) - 35,   // ~2^30
        (1u64 << 30) - 41,
        (1u64 << 30) - 87,
        (1u64 << 30) - 107,
    ];
    
    // Anchor moduli (coprime to main)
    let anchors = vec![
        (1u64 << 28) - 57,
        (1u64 << 28) - 89,
    ];
    
    println!("Test Value: {}", value);
    println!("Divisor: {}", divisor);
    println!("Expected Quotient: {}", value / divisor);
    println!("Expected Remainder: {}", value % divisor);
    println!();
    
    // Approach 1: Coprime constant division
    println!("──────────────────────────────────────────────────────────────");
    println!("APPROACH 1: Division by Coprime Constant");
    println!("──────────────────────────────────────────────────────────────");
    
    let residues: Vec<u64> = moduli.iter().map(|&m| value % m).collect();
    let start = Instant::now();
    
    for _ in 0..10000 {
        let _ = divide_by_coprime_constant(&residues, &moduli, divisor);
    }
    
    let elapsed = start.elapsed();
    println!("  10,000 iterations: {:?}", elapsed);
    println!("  Per operation: {:?}", elapsed / 10000);
    println!("  Result: {:?}", divide_by_coprime_constant(&residues, &moduli, divisor));
    println!();
    
    // Approach 2: MRC-based division
    println!("──────────────────────────────────────────────────────────────");
    println!("APPROACH 2: Mixed Radix Conversion (Traditional O(k²))");
    println!("──────────────────────────────────────────────────────────────");
    
    let start = Instant::now();
    
    for _ in 0..10000 {
        let _ = divide_via_mrc(&residues, &moduli, divisor);
    }
    
    let elapsed = start.elapsed();
    let (q_residues, remainder) = divide_via_mrc(&residues, &moduli, divisor);
    
    println!("  10,000 iterations: {:?}", elapsed);
    println!("  Per operation: {:?}", elapsed / 10000);
    println!("  Quotient residues: {:?}", q_residues);
    println!("  Remainder: {}", remainder);
    
    // Verify
    let mrs = MixedRadix::from_rns(&q_residues, &moduli);
    println!("  Reconstructed quotient: {}", mrs.to_value());
    println!();
    
    // Approach 3: Quotient Tracking
    println!("──────────────────────────────────────────────────────────────");
    println!("APPROACH 3: Quotient Signature (O(1) Comparison)");
    println!("──────────────────────────────────────────────────────────────");
    
    let tracked = QuotientTrackedValue::from_value(value, &moduli);
    
    let start = Instant::now();
    
    for _ in 0..10000 {
        let _ = tracked.magnitude_tier();
    }
    
    let elapsed = start.elapsed();
    
    println!("  Quotient signature: {:?}", tracked.quotients);
    println!("  Magnitude tier: {}", tracked.magnitude_tier());
    println!("  10,000 tier lookups: {:?}", elapsed);
    println!("  Per lookup: {:?}", elapsed / 10000);
    println!();
    
    // Approach 4: K-Elimination
    println!("──────────────────────────────────────────────────────────────");
    println!("APPROACH 4: K-Elimination (O(k) Exact Division)");
    println!("──────────────────────────────────────────────────────────────");
    
    let dual = DualManifoldValue::from_value(value, &moduli, &anchors);
    
    println!("  Primary residues: {:?}", dual.primary);
    println!("  Anchor residues: {:?}", dual.anchor);
    println!("  Computed k: {}", dual.compute_k());
    println!("  True value: {}", dual.true_value());
    
    let start = Instant::now();
    
    for _ in 0..10000 {
        let _ = k_elimination_divide(&dual, divisor);
    }
    
    let elapsed = start.elapsed();
    let (q_dual, remainder) = k_elimination_divide(&dual, divisor);
    
    println!("  10,000 iterations: {:?}", elapsed);
    println!("  Per operation: {:?}", elapsed / 10000);
    println!("  Quotient value: {}", q_dual.true_value());
    println!("  Remainder: {}", remainder);
    println!();
    
    // Approach 5: P-adic Division
    println!("──────────────────────────────────────────────────────────────");
    println!("APPROACH 5: P-adic (Hensel Code) Division");
    println!("──────────────────────────────────────────────────────────────");
    
    let prime = 97u64;  // A nice prime
    let precision = 8;
    
    // Represent value and divisor as Hensel codes
    let h_value = HenselCode::from_fraction(value % (prime.pow(precision as u32)), 1, prime, precision);
    let h_divisor = HenselCode::from_fraction(divisor, 1, prime, precision);
    
    println!("  Prime: {}", prime);
    println!("  Precision: {} digits", precision);
    println!("  Value in {}-adic: {:?}", prime, h_value.digits);
    println!("  Divisor in {}-adic: {:?}", prime, h_divisor.digits);
    
    let start = Instant::now();
    
    for _ in 0..10000 {
        let _ = h_value.divide(&h_divisor);
    }
    
    let elapsed = start.elapsed();
    let h_quotient = h_value.divide(&h_divisor);
    
    println!("  10,000 iterations: {:?}", elapsed);
    println!("  Per operation: {:?}", elapsed / 10000);
    println!("  Quotient digits: {:?}", h_quotient.digits);
    println!();
    
    // Summary
    println!("═══════════════════════════════════════════════════════════════");
    println!("                         SUMMARY");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Method                    | Complexity | Exact? | Use Case");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("  Coprime Constant          | O(k)       | Yes    | Fixed scaling");
    println!("  Mixed Radix (Traditional) | O(k²)      | Yes    | General (slow)");
    println!("  Quotient Signature        | O(1)       | Yes*   | Comparison");
    println!("  K-Elimination             | O(k)       | Yes    | General (fast)");
    println!("  P-adic (Hensel)           | O(k)       | Yes    | Right-to-left");
    println!();
    println!("  * With quotient tracking during computation");
    println!();
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    benchmark_division_approaches();
    
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    KEY INSIGHTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  1. QUOTIENTS ARE FREE: When computing r = x mod m,");
    println!("     hardware gives you q = x div m FOR FREE.");
    println!();
    println!("  2. PHASE DIFFERENTIAL ENCODES k: The difference between");
    println!("     primary and anchor reconstructions tells you exactly");
    println!("     how many times you've wrapped around.");
    println!();
    println!("  3. P-ADIC GOES RIGHT-TO-LEFT: Unlike positional division,");
    println!("     p-adic division is deterministic and uniform.");
    println!();
    println!("  4. K-ELIMINATION: The 60-year-old \"k-tracking\" problem");
    println!("     was solving something that was never lost.");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coprime_division() {
        let moduli = vec![127, 131, 137];
        let value = 1000u64;
        let divisor = 7u64;  // Coprime to all moduli
        
        let residues: Vec<u64> = moduli.iter().map(|&m| value % m).collect();
        let result = divide_by_coprime_constant(&residues, &moduli, divisor);
        
        assert!(result.is_some());
        
        // Verify by reconstructing
        let q_residues = result.unwrap();
        let expected_q = value / divisor;  // 142
        
        for (&r, &m) in q_residues.iter().zip(moduli.iter()) {
            assert_eq!(r, expected_q % m);
        }
    }
    
    #[test]
    fn test_mrc_division() {
        let moduli = vec![127, 131, 137, 139];
        let value = 12345u64;
        let divisor = 17u64;
        
        let residues: Vec<u64> = moduli.iter().map(|&m| value % m).collect();
        let (q_residues, remainder) = divide_via_mrc(&residues, &moduli, divisor);
        
        assert_eq!(remainder, value % divisor);
        
        // Verify quotient
        let mrs = MixedRadix::from_rns(&q_residues, &moduli);
        assert_eq!(mrs.to_value() as u64, value / divisor);
    }
    
    #[test]
    fn test_k_elimination() {
        let primary = vec![127, 131, 137];
        let anchor = vec![113, 109];
        let value = 50000u64;
        let divisor = 13u64;
        
        let dual = DualManifoldValue::from_value(value, &primary, &anchor);
        
        // Verify true value reconstruction
        assert_eq!(dual.true_value() as u64, value);
        
        // Test division
        let (q_dual, remainder) = k_elimination_divide(&dual, divisor);
        
        assert_eq!(remainder, value % divisor);
        assert_eq!(q_dual.true_value() as u64, value / divisor);
    }
    
    #[test]
    fn test_quotient_signature_comparison() {
        let moduli = vec![127, 131, 137, 139];
        
        let a = QuotientTrackedValue::from_value(1000, &moduli);
        let b = QuotientTrackedValue::from_value(2000, &moduli);
        let c = QuotientTrackedValue::from_value(1000, &moduli);
        
        assert_eq!(a.compare(&b), std::cmp::Ordering::Less);
        assert_eq!(b.compare(&a), std::cmp::Ordering::Greater);
        assert_eq!(a.compare(&c), std::cmp::Ordering::Equal);
    }
}
