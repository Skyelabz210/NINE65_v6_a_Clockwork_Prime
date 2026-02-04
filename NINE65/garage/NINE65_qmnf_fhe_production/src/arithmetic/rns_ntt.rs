//! RNS-NTT negacyclic convolution for arbitrary moduli.
//!
//! Uses two NTT-friendly primes and CRT reconstruction.

const Q0: u64 = 2013265921; // 15 * 2^27 + 1
const Q1: u64 = 1811939329; // 27 * 2^26 + 1
const Q2: u64 = 469762049;  // 7 * 2^26 + 1
const Q0_PRIMITIVE_ROOT: u64 = 31;
const Q1_PRIMITIVE_ROOT: u64 = 13;
const Q2_PRIMITIVE_ROOT: u64 = 3;

pub fn rns_ntt_multiply(a: &[u64], b: &[u64], modulus: u64) -> Vec<u64> {
    assert_eq!(a.len(), b.len(), "length mismatch");
    assert!(a.len().is_power_of_two(), "length must be power of two");
    let max_mod = (Q0 as u128) * (Q1 as u128) * (Q2 as u128);
    assert!((modulus as u128) < max_mod, "modulus too large for 2-prime RNS");

    let a_q0: Vec<u64> = a.iter().map(|&v| v % Q0).collect();
    let b_q0: Vec<u64> = b.iter().map(|&v| v % Q0).collect();
    let a_q1: Vec<u64> = a.iter().map(|&v| v % Q1).collect();
    let b_q1: Vec<u64> = b.iter().map(|&v| v % Q1).collect();
    let a_q2: Vec<u64> = a.iter().map(|&v| v % Q2).collect();
    let b_q2: Vec<u64> = b.iter().map(|&v| v % Q2).collect();

    let c_q0 = negacyclic_convolution_mod(&a_q0, &b_q0, Q0, Q0_PRIMITIVE_ROOT);
    let c_q1 = negacyclic_convolution_mod(&a_q1, &b_q1, Q1, Q1_PRIMITIVE_ROOT);
    let c_q2 = negacyclic_convolution_mod(&a_q2, &b_q2, Q2, Q2_PRIMITIVE_ROOT);

    crt_reconstruct(&c_q0, &c_q1, &c_q2, modulus)
}

fn negacyclic_convolution_mod(a: &[u64], b: &[u64], modulus: u64, primitive_root: u64) -> Vec<u64> {
    let n = a.len();
    let psi = mod_pow(primitive_root, (modulus - 1) / (2 * n as u64), modulus);
    let psi_inv = mod_inverse(psi, modulus);
    let omega = mul_mod(psi, psi, modulus);

    let psi_powers = pow_table(psi, n, modulus);
    let psi_inv_powers = pow_table(psi_inv, n, modulus);

    let mut a_twisted: Vec<u64> = a.iter().zip(psi_powers.iter())
        .map(|(&ai, &p)| mul_mod(ai, p, modulus))
        .collect();
    let mut b_twisted: Vec<u64> = b.iter().zip(psi_powers.iter())
        .map(|(&bi, &p)| mul_mod(bi, p, modulus))
        .collect();

    ntt_inplace(&mut a_twisted, modulus, omega, false);
    ntt_inplace(&mut b_twisted, modulus, omega, false);

    let mut c_twisted = vec![0u64; n];
    for i in 0..n {
        c_twisted[i] = mul_mod(a_twisted[i], b_twisted[i], modulus);
    }

    ntt_inplace(&mut c_twisted, modulus, omega, true);

    c_twisted.iter().zip(psi_inv_powers.iter())
        .map(|(&ci, &p)| mul_mod(ci, p, modulus))
        .collect()
}

fn ntt_inplace(a: &mut [u64], modulus: u64, root: u64, invert: bool) {
    let n = a.len();
    bit_reverse(a);

    let mut len = 2;
    while len <= n {
        let mut wlen = mod_pow(root, (n / len) as u64, modulus);
        if invert {
            wlen = mod_inverse(wlen, modulus);
        }
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for j in 0..(len / 2) {
                let u = a[i + j];
                let v = mul_mod(a[i + j + len / 2], w, modulus);
                let mut x = u + v;
                if x >= modulus {
                    x -= modulus;
                }
                a[i + j] = x;
                a[i + j + len / 2] = if u >= v { u - v } else { u + modulus - v };
                w = mul_mod(w, wlen, modulus);
            }
        }
        len <<= 1;
    }

    if invert {
        let n_inv = mod_inverse(n as u64, modulus);
        for v in a.iter_mut() {
            *v = mul_mod(*v, n_inv, modulus);
        }
    }
}

fn bit_reverse(a: &mut [u64]) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
}

fn pow_table(base: u64, n: usize, modulus: u64) -> Vec<u64> {
    let mut table = vec![1u64; n];
    for i in 1..n {
        table[i] = mul_mod(table[i - 1], base, modulus);
    }
    table
}

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        exp >>= 1;
        base = mul_mod(base, base, modulus);
    }
    result
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    let mut mn = (m as i128, a as i128);
    let mut xy = (0i128, 1i128);
    while mn.1 != 0 {
        let q = mn.0 / mn.1;
        mn = (mn.1, mn.0 - q * mn.1);
        xy = (xy.1, xy.0 - q * xy.1);
    }
    while xy.0 < 0 {
        xy.0 += m as i128;
    }
    (xy.0 % m as i128) as u64
}

fn crt_reconstruct(c0: &[u64], c1: &[u64], c2: &[u64], modulus: u64) -> Vec<u64> {
    assert_eq!(c0.len(), c1.len(), "length mismatch");
    assert_eq!(c0.len(), c2.len(), "length mismatch");
    let inv_q0_mod_q1 = mod_inverse(Q0 % Q1, Q1);
    let m01 = (Q0 as u128) * (Q1 as u128);
    let inv_m01_mod_q2 = mod_inverse((m01 % Q2 as u128) as u64, Q2);
    let q_prod = m01 * (Q2 as u128);
    let q_half = q_prod / 2;

    let mut out = vec![0u64; c0.len()];
    for i in 0..c0.len() {
        let a0 = c0[i];
        let a1 = c1[i];
        let a2 = c2[i];

        let t01 = (a1 + Q1 - (a0 % Q1)) % Q1;
        let t01 = mul_mod(t01, inv_q0_mod_q1, Q1);
        let x01 = a0 as u128 + (Q0 as u128) * (t01 as u128);

        let x01_mod_q2 = (x01 % Q2 as u128) as u64;
        let t02 = (a2 + Q2 - x01_mod_q2) % Q2;
        let t02 = mul_mod(t02, inv_m01_mod_q2, Q2);
        let x = x01 + m01 * (t02 as u128);
        let mut signed = if x > q_half {
            (x as i128) - (q_prod as i128)
        } else {
            x as i128
        };
        let q_mod = modulus as i128;
        signed %= q_mod;
        if signed < 0 {
            signed += q_mod;
        }
        out[i] = signed as u64;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::rns_ntt_multiply;
    use crate::arithmetic::NTTEngine;

    const TEST_PRIME: u64 = 998244353;

    #[test]
    fn test_rns_ntt_matches_reference() {
        let ntt = NTTEngine::new(TEST_PRIME, 8);
        let a: Vec<u64> = (0..8).map(|i| (i * 7 + 3) as u64).collect();
        let b: Vec<u64> = (0..8).map(|i| (i * 11 + 5) as u64).collect();

        let expected = ntt.multiply(&a, &b);
        let got = rns_ntt_multiply(&a, &b, TEST_PRIME);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_rns_ntt_negacyclic() {
        let ntt = NTTEngine::new(TEST_PRIME, 4);
        let a = vec![0, 0, 0, 1];
        let b = vec![0, 1, 0, 0];
        let expected = ntt.multiply(&a, &b);
        let got = rns_ntt_multiply(&a, &b, TEST_PRIME);
        assert_eq!(expected, got);
    }
}
