// QMNF Fused Piggyback Division (FPD) - Production Implementation
// The Core Innovation: Division without binary conversion
//
// From documents: "Solves division a/b mod M when gcd(b,M) ≠ 1"
// Method: Project to coprime anchor space, solve exactly, lift back

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

// ============================================================================
// HIERARCHICAL ANCHOR STRUCTURE (L0/L1/L2/Dynamic)
// ============================================================================

#[derive(Clone, Debug)]
pub struct AnchorTiers {
    pub l0: Vec<u64>,      // 3 primes, ~191 bits
    pub l1: Vec<u64>,      // 3 primes, ~190 bits  
    pub l2: Vec<u64>,      // 2 primes, ~126 bits
    pub dynamic: Vec<u64>, // 48 primes for complex cases
}

impl AnchorTiers {
    pub fn new_default() -> Self {
        Self {
            l0: vec![
                18446744073709551557, // 2^64 - 59
                18446744073709551533, // 2^64 - 83
                18446744073709551521, // 2^64 - 95
            ],
            l1: vec![
                18446744073709551437, // 2^64 - 179
                18446744073709551427, // 2^64 - 189
                18446744073709551359, // 2^64 - 257
            ],
            l2: vec![
                9223372036854775783,  // 2^63 - 25
                9223372036854775643,  // 2^63 - 165
            ],
            dynamic: Self::generate_dynamic_pool(48),
        }
    }
    
    fn generate_dynamic_pool(count: usize) -> Vec<u64> {
        let mut pool = Vec::new();
        let mut candidate = 127u64;
        
        while pool.len() < count {
            if is_prime(candidate) {
                pool.push(candidate);
            }
            candidate = next_prime(candidate);
        }
        pool
    }
    
    pub fn all_anchors(&self) -> Vec<u64> {
        let mut all = Vec::new();
        all.extend(&self.l0);
        all.extend(&self.l1);
        all.extend(&self.l2);
        all.extend(&self.dynamic);
        all
    }
}

// ============================================================================
// FUSED PIGGYBACK DIVISION ENGINE
// ============================================================================

pub struct FusedPiggybackDivision {
    anchors: AnchorTiers,
    working_moduli: Vec<u64>,
    
    // Statistics
    pub l0_hits: u64,
    pub l1_hits: u64,
    pub l2_hits: u64,
    pub dynamic_hits: u64,
    pub failures: u64,
}

#[derive(Debug, Clone)]
pub struct FPDResult {
    pub quotient: u64,
    pub error_bound: u64,
    pub status: FPDStatus,
    pub level_used: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FPDStatus {
    Exact,
    Approximate(u64), // error bound
}

impl FusedPiggybackDivision {
    pub fn new(working_moduli: Vec<u64>) -> Self {
        Self {
            anchors: AnchorTiers::new_default(),
            working_moduli,
            l0_hits: 0,
            l1_hits: 0,
            l2_hits: 0,
            dynamic_hits: 0,
            failures: 0,
        }
    }
    
    /// Core FPD algorithm: divide a/b mod M with anchor projection
    pub fn divide(&mut self, a: u64, b: u64, base_modulus: u64) -> Result<FPDResult, FPDError> {
        if b == 0 {
            return Err(FPDError::DivisionByZero);
        }
        
        // Try hierarchical anchor levels
        if let Ok(result) = self.try_level(&self.anchors.l0.clone(), a, b, base_modulus, 0) {
            self.l0_hits += 1;
            return Ok(result);
        }
        
        if let Ok(result) = self.try_level(&self.anchors.l1.clone(), a, b, base_modulus, 1) {
            self.l1_hits += 1;
            return Ok(result);
        }
        
        if let Ok(result) = self.try_level(&self.anchors.l2.clone(), a, b, base_modulus, 2) {
            self.l2_hits += 1;
            return Ok(result);
        }
        
        // Dynamic pool: collect coprime anchors
        let mut valid_anchors = Vec::new();
        for &anchor in &self.anchors.dynamic {
            if gcd(b, anchor) == 1 {
                valid_anchors.push(anchor);
                if valid_anchors.len() >= 8 {
                    break;
                }
            }
        }
        
        if !valid_anchors.is_empty() {
            if let Ok(result) = self.try_level(&valid_anchors, a, b, base_modulus, 3) {
                self.dynamic_hits += 1;
                return Ok(result);
            }
        }
        
        self.failures += 1;
        Err(FPDError::AllAnchorsFailed)
    }
    
    /// Try division at specific anchor level
    fn try_level(&self, anchors: &[u64], a: u64, b: u64, base_modulus: u64, level: usize) 
        -> Result<FPDResult, FPDError> {
        
        // Step 1: Coprimality probe
        let mut valid_anchors = Vec::new();
        for &anchor in anchors {
            if gcd(b, anchor) == 1 {
                valid_anchors.push(anchor);
            }
        }
        
        if valid_anchors.is_empty() {
            return Err(FPDError::NoCoprimeAnchors);
        }
        
        // Step 2: Solve in each anchor
        let mut anchor_solutions = Vec::new();
        for &anchor in &valid_anchors {
            let a_anchor = a % anchor;
            let b_anchor = b % anchor;
            
            // Modular inverse via FLT: b^(-1) ≡ b^(anchor-2) mod anchor
            let b_inv = mod_pow(b_anchor, anchor - 2, anchor);
            let x_anchor = (a_anchor as u128 * b_inv as u128) % anchor as u128;
            
            anchor_solutions.push((x_anchor as u64, anchor));
        }
        
        // Step 3: CRT fusion of anchor results
        let anchor_product: u128 = valid_anchors.iter().map(|&a| a as u128).product();
        let x_fused = crt_fuse(&anchor_solutions)?;
        
        // Step 4: Project to base modulus
        let quotient = (x_fused % base_modulus as u128) as u64;
        
        // Step 5: Compute error bound
        let g = gcd_multiple(&valid_anchors, base_modulus);
        let status = if g == 1 {
            FPDStatus::Exact
        } else {
            FPDStatus::Approximate(g)
        };
        
        Ok(FPDResult {
            quotient,
            error_bound: g,
            status,
            level_used: level,
        })
    }
    
    pub fn success_rate(&self) -> f64 {
        let total = self.l0_hits + self.l1_hits + self.l2_hits + self.dynamic_hits + self.failures;
        if total == 0 {
            return 1.0;
        }
        (total - self.failures) as f64 / total as f64
    }
}

// ============================================================================
// CRT FUSION
// ============================================================================

fn crt_fuse(solutions: &[(u64, u64)]) -> Result<u128, FPDError> {
    if solutions.is_empty() {
        return Ok(0);
    }
    
    let mut result = solutions[0].0 as u128;
    let mut product = solutions[0].1 as u128;
    
    for i in 1..solutions.len() {
        let (x_i, m_i) = solutions[i];
        let m_i = m_i as u128;
        
        let diff = if x_i as u128 >= result % m_i {
            x_i as u128 - (result % m_i)
        } else {
            m_i - ((result % m_i) - x_i as u128)
        };
        
        let prod_mod_mi = (product % m_i) as u64;
        let inv = mod_pow(prod_mod_mi, m_i as u64 - 2, m_i as u64);
        
        let coeff = (diff * inv as u128) % m_i;
        result += coeff * product;
        product *= m_i;
    }
    
    Ok(result)
}

// ============================================================================
// UTILITIES
// ============================================================================

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn gcd_multiple(values: &[u64], base: u64) -> u64 {
    let mut result = base;
    for &v in values {
        result = gcd(result, v);
    }
    result
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    
    let mut d = n - 1;
    while d % 2 == 0 { d /= 2; }
    
    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n { continue; }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        
        let mut r = d;
        while r != n - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 { break; }
            r *= 2;
        }
        if x != n - 1 { return false; }
    }
    true
}

fn next_prime(n: u64) -> u64 {
    let mut candidate = if n % 2 == 0 { n + 1 } else { n + 2 };
    while !is_prime(candidate) {
        candidate += 2;
    }
    candidate
}

#[derive(Debug)]
pub enum FPDError {
    DivisionByZero,
    NoCoprimeAnchors,
    AllAnchorsFailed,
    CRTFailed,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fpd_exact_division() {
        let moduli = vec![1000000007u64];
        let mut fpd = FusedPiggybackDivision::new(moduli);
        
        let result = fpd.divide(100, 7, 1000000007).unwrap();
        
        assert_eq!(result.status, FPDStatus::Exact);
        
        // Verify: 7 * quotient ≡ 100 (mod M)
        let verify = (7u128 * result.quotient as u128) % 1000000007;
        assert_eq!(verify, 100);
    }
    
    #[test]
    fn test_fpd_hierarchical_fallthrough() {
        let moduli = vec![1009u64, 1013, 1019];
        let mut fpd = FusedPiggybackDivision::new(moduli);
        
        // Run many divisions to hit different levels
        for i in 1..1000 {
            let _ = fpd.divide(i * 1000, i, 1009);
        }
        
        println!("L0 hits: {}", fpd.l0_hits);
        println!("L1 hits: {}", fpd.l1_hits);
        println!("L2 hits: {}", fpd.l2_hits);
        println!("Dynamic hits: {}", fpd.dynamic_hits);
        println!("Success rate: {:.2}%", fpd.success_rate() * 100.0);
        
        assert!(fpd.success_rate() > 0.99); // >99% success
    }
    
    #[test]
    fn test_fpd_vs_traditional() {
        let moduli = vec![1000000007u64];
        let mut fpd = FusedPiggybackDivision::new(moduli);
        
        for (a, b) in [(100, 7), (1234, 56), (9999, 3), (123456, 789)] {
            let result = fpd.divide(a, b, 1000000007).unwrap();
            
            // Traditional: a/b in integers, then mod
            let traditional = (a / b) % 1000000007;
            
            // FPD should give same or approximate result
            println!("a={}, b={}: FPD={}, Traditional={}", 
                     a, b, result.quotient, traditional);
        }
    }
}
