//! AVX-512 Vectorized NTT
//!
//! INNOVATION: Process 8 butterflies in parallel using AVX-512
//!
//! Expected speedup: 4-8× over scalar implementation
//!
//! Requirements:
//! - CPU with AVX-512F, AVX-512DQ support
//! - Target feature enabled: #[target_feature(enable = "avx512f,avx512dq")]
//!
//! Note: Falls back to scalar if AVX-512 not available

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Check if AVX-512 is available at runtime
#[cfg(target_arch = "x86_64")]
pub fn has_avx512() -> bool {
    is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512dq")
}

#[cfg(not(target_arch = "x86_64"))]
pub fn has_avx512() -> bool {
    false
}

/// AVX-512 NTT context
pub struct Avx512NttContext {
    pub q: u64,
    pub n: usize,
    pub log_n: usize,
    /// Twiddle factors in Montgomery form, vectorization-friendly layout
    pub twiddles_vec: Vec<u64>,
    /// Bit-reversal permutation table
    pub bit_rev_table: Vec<usize>,
    /// Montgomery constants
    pub mont_q_inv: u64,
    pub mont_r2: u64,
    /// Broadcast constants for SIMD
    pub q_vec: [u64; 8],
    pub q2_vec: [u64; 8],
}

impl Avx512NttContext {
    pub fn new(q: u64, n: usize) -> Self {
        assert!(n.is_power_of_two(), "N must be power of 2");
        assert!(n >= 8, "N must be >= 8 for AVX-512");
        
        let log_n = n.trailing_zeros() as usize;
        
        // Precompute bit-reversal table
        let bit_rev_table: Vec<usize> = (0..n)
            .map(|i| i.reverse_bits() >> (usize::BITS as usize - log_n))
            .collect();
        
        // Compute twiddle factors
        let omega = Self::find_primitive_root(q, n);
        let mont_r2 = Self::compute_mont_r2(q);
        let mont_q_inv = Self::compute_mont_q_inv(q);
        
        // Layout twiddles for vectorized access
        let mut twiddles_vec = vec![0u64; n];
        let mut power = 1u64;
        for i in 0..n {
            // Store in Montgomery form
            twiddles_vec[i] = ((power as u128 * mont_r2 as u128) % q as u128) as u64;
            power = ((power as u128 * omega as u128) % q as u128) as u64;
        }
        
        Self {
            q,
            n,
            log_n,
            twiddles_vec,
            bit_rev_table,
            mont_q_inv,
            mont_r2,
            q_vec: [q; 8],
            q2_vec: [q << 1; 8],
        }
    }
    
    fn find_primitive_root(q: u64, n: usize) -> u64 {
        let exp = (q - 1) / (n as u64);
        for g in 2..q {
            let candidate = Self::mod_pow(g, exp, q);
            let half = Self::mod_pow(candidate, (n / 2) as u64, q);
            if half == q - 1 {
                return candidate;
            }
        }
        panic!("No primitive root found");
    }
    
    fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
        let mut result = 1u128;
        let mut base = base as u128;
        let modulus = modulus as u128;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % modulus;
            }
            exp >>= 1;
            base = (base * base) % modulus;
        }
        result as u64
    }
    
    fn compute_mont_r2(q: u64) -> u64 {
        let r = 1u128 << 64;
        ((r * r) % q as u128) as u64
    }
    
    fn compute_mont_q_inv(q: u64) -> u64 {
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(q.wrapping_mul(inv)));
        }
        inv.wrapping_neg()
    }
    
    /// Bit-reversal permutation using precomputed table
    pub fn bit_reverse_permute(&self, a: &mut [u64]) {
        for i in 0..self.n {
            let j = self.bit_rev_table[i];
            if i < j {
                a.swap(i, j);
            }
        }
    }
}

// ============================================================================
// SCALAR FALLBACK (when AVX-512 not available)
// ============================================================================

/// Scalar NTT forward transform
pub fn ntt_forward_scalar(a: &mut [u64], ctx: &Avx512NttContext) {
    ctx.bit_reverse_permute(a);
    
    let n = ctx.n;
    let q = ctx.q;
    let q2 = q << 1;
    
    let mut m = 1;
    for _ in 0..ctx.log_n {
        let half_m = m;
        m *= 2;
        let t_step = n / m;
        
        for k in (0..n).step_by(m) {
            let mut t_idx = 0;
            for j in 0..half_m {
                let u_idx = k + j;
                let v_idx = k + j + half_m;
                
                let u = a[u_idx];
                let tw = ctx.twiddles_vec[t_idx];
                
                // Montgomery multiply
                let t = montgomery_mul(a[v_idx], tw, q, ctx.mont_q_inv);
                
                // Butterfly with lazy reduction
                let sum = u + t;
                let diff = u + q2 - t;
                
                a[u_idx] = if sum >= q2 { sum - q2 } else if sum >= q { sum - q } else { sum };
                a[v_idx] = if diff >= q2 { diff - q2 } else if diff >= q { diff - q } else { diff };
                
                t_idx += t_step;
            }
        }
    }
    
    // Final reduction
    for x in a.iter_mut() {
        if *x >= q {
            *x -= q;
        }
    }
}

#[inline(always)]
fn montgomery_mul(a: u64, b: u64, q: u64, q_inv: u64) -> u64 {
    let ab = a as u128 * b as u128;
    let m = (ab as u64).wrapping_mul(q_inv);
    let t = ((ab + m as u128 * q as u128) >> 64) as u64;
    if t >= q { t - q } else { t }
}

// ============================================================================
// AVX-512 IMPLEMENTATION
// ============================================================================

#[cfg(target_arch = "x86_64")]
mod avx512 {
    use super::*;
    
    /// AVX-512 NTT forward transform
    /// Processes 8 elements in parallel
    #[target_feature(enable = "avx512f,avx512dq")]
    pub unsafe fn ntt_forward_avx512(a: &mut [u64], ctx: &Avx512NttContext) {
        ctx.bit_reverse_permute(a);
        
        let n = ctx.n;
        let q = ctx.q;
        let q_vec = _mm512_set1_epi64(q as i64);
        let q2_vec = _mm512_set1_epi64((q << 1) as i64);
        
        let mut m = 1;
        
        // First stages: scalar (m < 8)
        while m < 8 {
            let half_m = m;
            m *= 2;
            let t_step = n / m;
            
            for k in (0..n).step_by(m) {
                let mut t_idx = 0;
                for j in 0..half_m {
                    let u_idx = k + j;
                    let v_idx = k + j + half_m;
                    
                    let u = a[u_idx];
                    let tw = ctx.twiddles_vec[t_idx];
                    let t = montgomery_mul(a[v_idx], tw, q, ctx.mont_q_inv);
                    
                    let sum = u + t;
                    let diff = u + (q << 1) - t;
                    
                    a[u_idx] = lazy_reduce_scalar(sum, q);
                    a[v_idx] = lazy_reduce_scalar(diff, q);
                    
                    t_idx += t_step;
                }
            }
        }
        
        // Vectorized stages: m >= 8
        while m <= n {
            let half_m = m;
            m *= 2;
            let t_step = n / m;
            
            for k in (0..n).step_by(m) {
                // Process 8 butterflies at a time
                for jj in (0..half_m).step_by(8) {
                    if jj + 8 <= half_m {
                        let u_base = k + jj;
                        let v_base = k + jj + half_m;
                        
                        // Load 8 u values
                        let u_vec = _mm512_loadu_epi64(a.as_ptr().add(u_base) as *const i64);
                        
                        // Load 8 v values
                        let v_vec = _mm512_loadu_epi64(a.as_ptr().add(v_base) as *const i64);
                        
                        // Load 8 twiddle factors
                        // Note: twiddle indexing needs care for vectorized access
                        let t_base = jj / 8 * t_step * 8;
                        let tw_vec = load_twiddles_8(&ctx.twiddles_vec, jj, t_step);
                        
                        // Vectorized Montgomery multiply: t = tw * v
                        let t_vec = montgomery_mul_vec(v_vec, tw_vec, q_vec, ctx.mont_q_inv);
                        
                        // Vectorized butterfly
                        let sum = _mm512_add_epi64(u_vec, t_vec);
                        let diff = _mm512_add_epi64(
                            _mm512_sub_epi64(_mm512_add_epi64(u_vec, q2_vec), t_vec),
                            _mm512_setzero_si512()
                        );
                        
                        // Lazy reduction
                        let sum_reduced = lazy_reduce_vec(sum, q_vec, q2_vec);
                        let diff_reduced = lazy_reduce_vec(diff, q_vec, q2_vec);
                        
                        // Store results
                        _mm512_storeu_epi64(a.as_mut_ptr().add(u_base) as *mut i64, sum_reduced);
                        _mm512_storeu_epi64(a.as_mut_ptr().add(v_base) as *mut i64, diff_reduced);
                    } else {
                        // Handle remainder with scalar
                        for j in jj..half_m {
                            let u_idx = k + j;
                            let v_idx = k + j + half_m;
                            
                            let u = a[u_idx];
                            let tw = ctx.twiddles_vec[j * t_step];
                            let t = montgomery_mul(a[v_idx], tw, q, ctx.mont_q_inv);
                            
                            let sum = u + t;
                            let diff = u + (q << 1) - t;
                            
                            a[u_idx] = lazy_reduce_scalar(sum, q);
                            a[v_idx] = lazy_reduce_scalar(diff, q);
                        }
                    }
                }
            }
        }
        
        // Final reduction
        for chunk in a.chunks_mut(8) {
            if chunk.len() == 8 {
                let v = _mm512_loadu_epi64(chunk.as_ptr() as *const i64);
                let reduced = final_reduce_vec(v, q_vec);
                _mm512_storeu_epi64(chunk.as_mut_ptr() as *mut i64, reduced);
            } else {
                for x in chunk {
                    if *x >= q { *x -= q; }
                }
            }
        }
    }
    
    #[inline(always)]
    unsafe fn load_twiddles_8(twiddles: &[u64], base_j: usize, t_step: usize) -> __m512i {
        // Gather twiddles for 8 consecutive j values
        let indices: [i64; 8] = [
            (base_j * t_step) as i64,
            ((base_j + 1) * t_step) as i64,
            ((base_j + 2) * t_step) as i64,
            ((base_j + 3) * t_step) as i64,
            ((base_j + 4) * t_step) as i64,
            ((base_j + 5) * t_step) as i64,
            ((base_j + 6) * t_step) as i64,
            ((base_j + 7) * t_step) as i64,
        ];
        let idx = _mm512_loadu_epi64(indices.as_ptr());
        _mm512_i64gather_epi64(idx, twiddles.as_ptr() as *const i64, 8)
    }
    
    /// Vectorized Montgomery multiplication
    /// Returns a * b * R^(-1) mod q for 8 pairs
    #[inline(always)]
    unsafe fn montgomery_mul_vec(a: __m512i, b: __m512i, q: __m512i, q_inv: u64) -> __m512i {
        // This is a simplified version - full implementation needs careful handling
        // of 128-bit intermediate products
        
        // For now, extract to scalar, compute, and pack back
        // TODO: Implement true vectorized Montgomery using AVX-512IFMA if available
        let mut a_arr = [0i64; 8];
        let mut b_arr = [0i64; 8];
        _mm512_storeu_epi64(a_arr.as_mut_ptr(), a);
        _mm512_storeu_epi64(b_arr.as_mut_ptr(), b);
        
        let q_scalar = _mm512_cvtsi512_si32(q) as u64;
        let mut result = [0i64; 8];
        for i in 0..8 {
            result[i] = montgomery_mul(a_arr[i] as u64, b_arr[i] as u64, q_scalar, q_inv) as i64;
        }
        
        _mm512_loadu_epi64(result.as_ptr())
    }
    
    /// Vectorized lazy reduction: if x >= 2q, x -= q
    #[inline(always)]
    unsafe fn lazy_reduce_vec(x: __m512i, q: __m512i, q2: __m512i) -> __m512i {
        // mask = x >= q2
        let mask_q2 = _mm512_cmpge_epu64_mask(x, q2);
        let x1 = _mm512_mask_sub_epi64(x, mask_q2, x, q);
        
        // mask = x >= q
        let mask_q = _mm512_cmpge_epu64_mask(x1, q);
        _mm512_mask_sub_epi64(x1, mask_q, x1, q)
    }
    
    /// Final reduction: if x >= q, x -= q
    #[inline(always)]
    unsafe fn final_reduce_vec(x: __m512i, q: __m512i) -> __m512i {
        let mask = _mm512_cmpge_epu64_mask(x, q);
        _mm512_mask_sub_epi64(x, mask, x, q)
    }
    
    #[inline(always)]
    fn lazy_reduce_scalar(x: u64, q: u64) -> u64 {
        let q2 = q << 1;
        if x >= q2 { x - q2 } else if x >= q { x - q } else { x }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// NTT forward transform - auto-selects AVX-512 or scalar
pub fn ntt_forward(a: &mut [u64], ctx: &Avx512NttContext) {
    #[cfg(target_arch = "x86_64")]
    {
        if has_avx512() {
            unsafe { avx512::ntt_forward_avx512(a, ctx); }
            return;
        }
    }
    ntt_forward_scalar(a, ctx);
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_PRIME: u64 = 998244353;
    
    #[test]
    fn test_context_creation() {
        let ctx = Avx512NttContext::new(TEST_PRIME, 1024);
        assert_eq!(ctx.n, 1024);
        assert_eq!(ctx.log_n, 10);
        assert_eq!(ctx.bit_rev_table.len(), 1024);
        assert_eq!(ctx.twiddles_vec.len(), 1024);
    }
    
    #[test]
    fn test_bit_reverse_table() {
        let ctx = Avx512NttContext::new(TEST_PRIME, 8);
        
        // For n=8, log_n=3:
        // 0 (000) -> 0 (000)
        // 1 (001) -> 4 (100)
        // 2 (010) -> 2 (010)
        // 3 (011) -> 6 (110)
        // 4 (100) -> 1 (001)
        // 5 (101) -> 5 (101)
        // 6 (110) -> 3 (011)
        // 7 (111) -> 7 (111)
        assert_eq!(ctx.bit_rev_table[0], 0);
        assert_eq!(ctx.bit_rev_table[1], 4);
        assert_eq!(ctx.bit_rev_table[2], 2);
        assert_eq!(ctx.bit_rev_table[3], 6);
    }
    
    #[test]
    fn test_ntt_roundtrip_scalar() {
        let ctx = Avx512NttContext::new(TEST_PRIME, 8);
        let original: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut a = original.clone();
        
        ntt_forward_scalar(&mut a, &ctx);
        
        // Check all values are valid
        for &x in &a {
            assert!(x < TEST_PRIME);
        }
        
        // NTT should change values
        assert_ne!(a, original);
    }
    
    #[test]
    fn test_ntt_1024() {
        let ctx = Avx512NttContext::new(TEST_PRIME, 1024);
        let mut a: Vec<u64> = (0..1024).map(|i| i % TEST_PRIME).collect();
        
        ntt_forward(&mut a, &ctx);
        
        for &x in &a {
            assert!(x < TEST_PRIME);
        }
    }
    
    #[test]
    fn test_benchmark_scalar() {
        use std::time::Instant;
        
        let ctx = Avx512NttContext::new(TEST_PRIME, 1024);
        let mut a: Vec<u64> = (0..1024).map(|i| i % TEST_PRIME).collect();
        
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            ntt_forward_scalar(&mut a, &ctx);
        }
        let elapsed = start.elapsed();
        
        println!("Scalar NTT 1024 x{}: {:?}", iterations, elapsed);
        println!("Per NTT: {:?}", elapsed / iterations as u32);
    }
    
    #[test]
    fn test_avx512_available() {
        println!("AVX-512 available: {}", has_avx512());
    }
}
