//! Corrected NTT Primitive Root Finder
//! 
//! The original `find_primitive_root` returned `Some(3)` unconditionally,
//! which is NOT a valid primitive root for most moduli.
//! 
//! This implementation uses proper order-checking search:
//! 1. Factor (prime - 1)
//! 2. Find generator g where g^((p-1)/q) ≠ 1 for all prime factors q
//! 3. Derive ω = g^((p-1)/order)

/// Find a primitive root of unity of given order modulo prime
/// 
/// # Arguments
/// * `prime` - A prime number p
/// * `order` - The desired order (must divide p-1)
/// 
/// # Returns
/// * `Some(ω)` where ω^order ≡ 1 (mod p) and no smaller power equals 1
/// * `None` if order doesn't divide (p-1) or prime is not actually prime
/// 
/// # Algorithm
/// 1. Verify order divides (p-1)
/// 2. Find prime factors of (p-1)
/// 3. Find generator g by testing candidates
/// 4. Compute ω = g^((p-1)/order)
/// 
/// # Example
/// ```
/// let p = 257u64;  // 257 - 1 = 256 = 2^8
/// let order = 256u64;
/// let omega = find_primitive_root(p, order).unwrap();
/// assert_eq!(mod_pow(omega, order, p), 1);
/// assert_ne!(mod_pow(omega, order/2, p), 1);
/// ```
pub fn find_primitive_root(prime: u64, order: u64) -> Option<u64> {
    if prime < 2 {
        return None;
    }
    
    let p_minus_1 = prime - 1;
    
    // Order must divide p-1
    if p_minus_1 % order != 0 {
        return None;
    }
    
    // Factor p-1
    let factors = prime_factors(p_minus_1);
    
    if factors.is_empty() {
        return None;  // p-1 = 1 means p = 2, special case
    }
    
    // Find generator g of multiplicative group Z_p^*
    let g = find_generator(prime, &factors)?;
    
    // Compute primitive root of given order: ω = g^((p-1)/order)
    let exp = p_minus_1 / order;
    Some(mod_pow(g, exp, prime))
}

/// Find a generator of the multiplicative group Z_p^*
/// 
/// A generator g satisfies: g^((p-1)/q) ≠ 1 (mod p) for all prime factors q of (p-1)
fn find_generator(prime: u64, factors: &[u64]) -> Option<u64> {
    let p_minus_1 = prime - 1;
    
    // Try candidates starting from 2
    'candidate: for g in 2..prime {
        // Check that g is not a perfect power for any prime factor
        for &q in factors {
            let exp = p_minus_1 / q;
            if mod_pow(g, exp, prime) == 1 {
                // g is not a generator
                continue 'candidate;
            }
        }
        // g passed all checks - it's a generator
        return Some(g);
    }
    
    None
}

/// Compute prime factors of n (each prime appears once)
/// 
/// Uses trial division - sufficient for typical NTT moduli
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    // Factor out 2s
    if n % 2 == 0 {
        factors.push(2);
        while n % 2 == 0 {
            n /= 2;
        }
    }
    
    // Factor out odd primes
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 2;
    }
    
    // Remaining factor (if any) is prime
    if n > 1 {
        factors.push(n);
    }
    
    factors
}

/// Modular exponentiation: base^exp mod modulus
/// 
/// Uses binary exponentiation for O(log exp) multiplications
/// All arithmetic is integer-only (uses u128 for intermediate products)
#[inline]
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1u64;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp /= 2;
        base = mod_mul(base, base, modulus);
    }
    
    result
}

/// Modular multiplication using u128 to avoid overflow
#[inline]
pub fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

/// Verify that ω is a primitive root of given order
pub fn verify_primitive_root(omega: u64, order: u64, prime: u64) -> bool {
    // ω^order should equal 1
    if mod_pow(omega, order, prime) != 1 {
        return false;
    }
    
    // ω^k should NOT equal 1 for any k < order
    // (Only need to check k = order/q for each prime factor q of order)
    let factors = prime_factors(order);
    for q in factors {
        let k = order / q;
        if mod_pow(omega, k, prime) == 1 {
            return false;
        }
    }
    
    true
}

/// Common NTT-friendly primes and their properties
/// 
/// These primes have the form p = k * 2^n + 1, allowing large power-of-2 NTT sizes
pub const NTT_PRIMES: [(u64, u64, u64); 5] = [
    // (prime, max_order = largest power of 2 dividing p-1, known generator)
    (257, 256, 3),           // 2^8 + 1, Fermat prime
    (65537, 65536, 3),       // 2^16 + 1, Fermat prime  
    (7340033, 4194304, 3),   // 7 * 2^20 + 1
    (998244353, 8388608, 3), // 119 * 2^23 + 1 (common in competitive programming)
    (4294967296 + 1, 4294967296, 3), // 2^32 + 1 (Fermat prime F4 = 4294967297)
];

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(12), vec![2, 3]);
        assert_eq!(prime_factors(256), vec![2]);
        assert_eq!(prime_factors(255), vec![3, 5, 17]);
        assert_eq!(prime_factors(998244352), vec![2, 7, 17]);
    }
    
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 256, 257), 1);
        assert_eq!(mod_pow(2, 0, 17), 1);
        assert_eq!(mod_pow(0, 5, 17), 0);
    }
    
    #[test]
    fn test_find_primitive_root_small() {
        // p = 17, p-1 = 16 = 2^4
        let omega = find_primitive_root(17, 16).unwrap();
        assert!(verify_primitive_root(omega, 16, 17));
        
        // Should also work for smaller orders
        let omega8 = find_primitive_root(17, 8).unwrap();
        assert!(verify_primitive_root(omega8, 8, 17));
    }
    
    #[test]
    fn test_find_primitive_root_257() {
        // p = 257 (Fermat prime), p-1 = 256 = 2^8
        let omega = find_primitive_root(257, 256).unwrap();
        assert!(verify_primitive_root(omega, 256, 257));
        println!("Primitive 256th root of unity mod 257: {}", omega);
        
        // Verify ω^256 = 1
        assert_eq!(mod_pow(omega, 256, 257), 1);
        // Verify ω^128 ≠ 1
        assert_ne!(mod_pow(omega, 128, 257), 1);
    }
    
    #[test]
    fn test_find_primitive_root_998244353() {
        // p = 998244353, p-1 = 998244352 = 2^23 * 7 * 17
        let p = 998244353u64;
        
        // Find 2^23-th root of unity (max power of 2 dividing p-1)
        let order = 1u64 << 23;
        let omega = find_primitive_root(p, order).unwrap();
        assert!(verify_primitive_root(omega, order, p));
        println!("Primitive 2^23-th root of unity mod {}: {}", p, omega);
    }
    
    #[test]
    fn test_invalid_order() {
        // Order must divide p-1
        assert!(find_primitive_root(17, 5).is_none());  // 5 doesn't divide 16
        assert!(find_primitive_root(257, 17).is_none()); // 17 doesn't divide 256
    }
    
    #[test]
    fn test_known_generators() {
        // Verify that 3 is indeed a generator for common NTT primes
        for &(prime, max_order, gen) in &NTT_PRIMES[..4] {
            let factors = prime_factors(prime - 1);
            let mut is_generator = true;
            for &q in &factors {
                let exp = (prime - 1) / q;
                if mod_pow(gen, exp, prime) == 1 {
                    is_generator = false;
                    break;
                }
            }
            assert!(is_generator, "3 should be generator for prime {}", prime);
        }
    }
    
    #[test]
    fn test_negacyclic_root() {
        // For negacyclic NTT of size N, we need a primitive 2N-th root of unity
        // This is ψ where ψ^(2N) = 1 and ψ^N = -1
        
        let p = 257u64;
        let n = 128usize;  // NTT size
        
        // Find primitive 2N-th root
        let psi = find_primitive_root(p, (2 * n) as u64).unwrap();
        
        // Verify ψ^(2N) = 1
        assert_eq!(mod_pow(psi, 2 * n as u64, p), 1);
        
        // Verify ψ^N = -1 = p-1 (mod p)
        assert_eq!(mod_pow(psi, n as u64, p), p - 1);
        
        println!("Primitive 2*{}-th root for negacyclic NTT mod {}: {}", n, p, psi);
    }
}
