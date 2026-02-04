#![allow(missing_docs)]
//! Minimal, fast-enough big integer for CRT reconstruction and IO.
//! - Base: 2^64 limbs, little-endian
//! - Signed via `neg` bit with canonical zero = {limbs=[], neg=false}
//! - Operations: add/sub/mul by u64, full add/sub/mul, shifts, cmp, abs,
//!   cloning, to/from small ints, decimal Display, low_u128, % u64.

#![allow(clippy::needless_return)]

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HCVLangBigInt {
    pub limbs: Vec<u64>,
    pub neg: bool, // false => non-negative, true => negative
}

impl HCVLangBigInt {
    /* Constructors */

    /// Creates from an i64.
    pub fn new(value: i64) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let neg = value < 0;
        let v = value.unsigned_abs() as u64;
        Self {
            limbs: if v == 0 { Vec::new() } else { vec![v] },
            neg,
        }
        .normalize()
    }

    /// From u64 (always non-negative).
    pub fn from_u64(value: u64) -> Self {
        if value == 0 {
            Self::zero()
        } else {
            Self {
                limbs: vec![value],
                neg: false,
            }
        }
    }

    /// From i128.
    pub fn from_i128(value: i128) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let neg = value < 0;
        let mut x = value.unsigned_abs();
        let mut limbs = Vec::with_capacity(2);
        limbs.push((x & 0xFFFF_FFFF_FFFF_FFFF) as u64);
        x >>= 64;
        if x != 0 {
            limbs.push((x & 0xFFFF_FFFF_FFFF_FFFF) as u64);
        }
        Self { limbs, neg }.normalize()
    }

    /// Zero
    pub fn zero() -> Self {
        Self {
            limbs: Vec::new(),
            neg: false,
        }
    }

    /// One
    pub fn one() -> Self {
        Self {
            limbs: vec![1],
            neg: false,
        }
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        self.neg && !self.is_zero()
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        let mut t = self.clone();
        t.neg = false;
        t
    }

    #[inline(always)]
    fn normalize(mut self) -> Self {
        while let Some(&last) = self.limbs.last() {
            if last == 0 {
                self.limbs.pop();
            } else {
                break;
            }
        }
        if self.limbs.is_empty() {
            self.neg = false;
        }
        self
    }

    /* Comparisons */

    pub fn cmp_abs(&self, other: &Self) -> Ordering {
        let la = self.limbs.len();
        let lb = other.limbs.len();
        if la != lb {
            return la.cmp(&lb);
        }
        for i in (0..la).rev() {
            let a = self.limbs[i];
            let b = other.limbs[i];
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }

    /* Low-level limb helpers */

    #[inline(always)]
    fn add_abs_assign(&mut self, rhs: &Self) {
        if rhs.is_zero() {
            return;
        }
        if self.limbs.len() < rhs.limbs.len() {
            self.limbs.resize(rhs.limbs.len(), 0);
        }

        let mut carry = 0u128;
        let n = rhs.limbs.len();
        for i in 0..n {
            let sum = self.limbs[i] as u128 + rhs.limbs[i] as u128 + carry;
            self.limbs[i] = sum as u64;
            carry = sum >> 64;
        }
        let m = self.limbs.len();
        let mut i = n;
        while carry != 0 && i < m {
            let sum = self.limbs[i] as u128 + carry;
            self.limbs[i] = sum as u64;
            carry = sum >> 64;
            i += 1;
        }
        if carry != 0 {
            self.limbs.push(carry as u64);
        }
    }

    #[inline(always)]
    fn sub_abs_assign(&mut self, rhs: &Self) {
        // require |self| >= |rhs|
        debug_assert!(self.cmp_abs(rhs) != Ordering::Less);
        if rhs.is_zero() {
            return;
        }
        let mut borrow = 0u128;
        let n = rhs.limbs.len();
        for i in 0..n {
            let a = self.limbs[i] as u128;
            let b = rhs.limbs[i] as u128 + borrow;
            let (res, did_borrow) = a.overflowing_sub(b);
            self.limbs[i] = res as u64;
            borrow = if did_borrow { 1 } else { 0 };
        }
        let m = self.limbs.len();
        let mut i = n;
        while borrow != 0 && i < m {
            let a = self.limbs[i] as u128;
            let (res, did_borrow) = a.overflowing_sub(1);
            self.limbs[i] = res as u64;
            borrow = if did_borrow { 1 } else { 0 };
            i += 1;
        }
        *self = self.clone().normalize();
    }

    #[inline(always)]
    pub fn mul_u64_assign(&mut self, rhs: u64) {
        if self.is_zero() || rhs == 0 {
            *self = Self::zero();
            return;
        }
        if rhs == 1 {
            return;
        }
        let mut carry = 0u128;
        for limb in &mut self.limbs {
            let t = (*limb as u128) * (rhs as u128) + carry;
            *limb = t as u64;
            carry = t >> 64;
        }
        if carry != 0 {
            self.limbs.push(carry as u64);
        }
    }

    /* Public helpers */

    /// Returns the low 128 bits of |self| (ignores sign).
    pub fn low_u128(&self) -> u128 {
        let lo = *self.limbs.get(0).unwrap_or(&0) as u128;
        let hi = *self.limbs.get(1).unwrap_or(&0) as u128;
        (hi << 64) | lo
    }

    /// self % m where m is u64 (non-negative)
    pub fn rem_u64(&self, m: u64) -> u64 {
        if m == 0 {
            return 0;
        }
        let mut rem: u128 = 0;
        for &w in self.limbs.iter().rev() {
            rem = ((rem << 64) | (w as u128)) % (m as u128);
        }
        rem as u64
    }

    /* Conversions */

    pub fn to_i64(&self) -> Option<i64> {
        if self.limbs.len() > 1 {
            return None;
        }
        let v = *self.limbs.get(0).unwrap_or(&0) as i128;
        let s = if self.is_negative() { -v } else { v };
        s.try_into().ok()
    }

    pub fn to_i128(&self) -> Option<i128> {
        if self.limbs.len() > 2 {
            return None;
        }
        let mut v = self.low_u128() as i128;
        if self.is_negative() {
            v = -v;
        }
        Some(v)
    }

    /// Construct from little-endian limbs (least-significant limb first).
    pub fn from_limbs_le(limbs: &[u64]) -> Self {
        // Fast path: trim high zeros
        let mut v = limbs.to_vec();
        while v.last().copied().unwrap_or(0) == 0 && v.len() > 1 {
            v.pop();
        }
        // assuming you already store little-endian limbs internally:
        Self { limbs: v, neg: false }.normalize() // Use normalize to handle potential zero
    }

    /// Get a read-only view of the internal limbs (LE).
    pub fn as_limbs(&self) -> &[u64] {
        &self.limbs
    }

    /// Number of limbs.
    pub fn len_limbs(&self) -> usize {
        self.limbs.len()
    }

    /// Returns the highest non-zero limb.
    pub fn top_word(&self) -> u64 {
        *self.limbs.last().unwrap_or(&0)
    }

    /// Check if the big integer is one.
    pub fn is_one(&self) -> bool {
        self.limbs.len() == 1 && self.limbs[0] == 1 && !self.neg
    }

    /// Check if number is even.
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        self.is_zero() || (self.limbs[0] & 1) == 0
    }

    /// Count trailing zero bits.
    #[inline(always)]
    pub fn trailing_zeros(&self) -> u32 {
        if self.is_zero() {
            return 0; // Or handle as appropriate for your use case, e.g., 64 * self.len_limbs()
        }
        let mut count = 0;
        for (_i, &limb) in self.limbs.iter().enumerate() {
            if limb == 0 {
                count += 64;
            } else {
                count += limb.trailing_zeros();
                break;
            }
        }
        count
    }

    /// Right shift (arithmetic) in place.
    #[inline(always)]
    pub fn shr_in_place(&mut self, bits: u32) {
        self.shr_assign(bits);
    }

    /// Left shift in place.
    pub fn shl_in_place(&mut self, bits: u32) {
        self.shl_assign(bits);
    }

    /// Binary GCD (Stein's algorithm) - Much faster than Euclidean GCD
    ///
    /// Uses only shifts, comparisons, and subtraction - NO DIVISION!
    /// Expected 2-4x faster than Euclidean GCD for large numbers.
    ///
    /// # Algorithm
    /// 1. Handle zero cases: gcd(0,v)=v, gcd(u,0)=u
    /// 2. Make both numbers positive
    /// 3. Extract common power of 2
    /// 4. Remove all powers of 2 from both numbers
    /// 5. While u != v:
    ///    - Remove powers of 2 from whichever is even
    ///    - Subtract smaller from larger, shift result right by 1
    /// 6. Multiply result by common power of 2
    #[inline(always)]
    pub fn gcd(a: &Self, b: &Self) -> Self {
        // Handle zero cases
        if a.is_zero() {
            return b.abs();
        }
        if b.is_zero() {
            return a.abs();
        }

        // Work with absolute values
        let mut u = a.abs();
        let mut v = b.abs();

        // Find common power of 2 (number of trailing zeros in both)
        let u_zeros = u.trailing_zeros();
        let v_zeros = v.trailing_zeros();
        let common_twos = u_zeros.min(v_zeros);

        // Remove all factors of 2 from u and v
        u.shr_in_place(u_zeros);
        v.shr_in_place(v_zeros);

        // Now u and v are both odd
        // Repeatedly apply: if diff is even, divide by 2; otherwise continue
        while u != v {
            if u > v {
                // u > v, so swap to make u smaller
                core::mem::swap(&mut u, &mut v);
            }
            // Now v > u, both odd
            // v - u is even (odd - odd = even), so we can shift
            // OPTIMIZED: Use in-place operations to avoid clones
            v -= &u;  // In-place subtraction (no clone of v!)
            v >>= 1;  // In-place shift

            // Remove factors of 2 from v (it might have become even)
            let v_zeros = v.trailing_zeros();
            if v_zeros > 0 {
                v.shr_in_place(v_zeros);
            }
        }

        // Restore the common power of 2
        u << common_twos
    }

    /// Fast multiply by u64.
    pub fn mul_small(&self, k: u64) -> Self {
        let mut res = self.clone();
        res.mul_u64_assign(k);
        res
    }

    /// self % rhs, rhs != 0.
    pub fn mod_rem(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        res %= rhs;
        res
    }

    /// Most significant bit position (for division optimizer compatibility)
    pub fn msb(&self) -> u32 {
        if let Some(&last) = self.limbs.last() {
            64 - last.leading_zeros()
        } else {
            0
        }
    }

    /// Create from single limb (for division optimizer compatibility)
    pub fn from_limb(limb: u64) -> Self {
        if limb == 0 {
            Self::zero()
        } else {
            Self {
                limbs: vec![limb],
                neg: false,
            }
        }
    }

    /// Left shift by bit count (for division optimizer compatibility)
    pub fn shl_bits(&self, shift: u32) -> Self {
        if shift == 0 || self.is_zero() {
            return self.clone();
        }
        let mut result = self.clone();
        result.shl_assign(shift);
        result
    }

    /// Right shift by bit count (for division optimizer compatibility)
    pub fn shr_bits(&self, shift: u32) -> Self {
        if shift == 0 || self.is_zero() {
            return self.clone();
        }
        let mut result = self.clone();
        result.shr_assign(shift);
        result
    }

    /// Access limb at index (for division optimizer compatibility)
    pub fn limb(&self, index: usize) -> u64 {
        self.limbs.get(index).copied().unwrap_or(0)
    }

    /// Number of limbs (alias for len_limbs for optimizer compatibility)
    pub fn limb_len(&self) -> usize {
        self.limbs.len()
    }
}

/* Add/Sub with signs */

impl AddAssign<&HCVLangBigInt> for HCVLangBigInt {
    fn add_assign(&mut self, rhs: &HCVLangBigInt) {
        match (self.neg, rhs.neg) {
            (false, false) => self.add_abs_assign(rhs),
            (true, true) => {
                // (-a) + (-b) = -(a+b)
                self.neg = false;
                self.add_abs_assign(rhs);
                self.neg = true && !self.is_zero();
            }
            (false, true) => {
                // a + (-b) = a - b
                match self.cmp_abs(rhs) {
                    Ordering::Greater | Ordering::Equal => self.sub_abs_assign(rhs),
                    Ordering::Less => {
                        let mut tmp = rhs.clone();
                        tmp.neg = false;
                        tmp.sub_abs_assign(self);
                        *self = tmp;
                        self.neg = true && !self.is_zero();
                    }
                }
            }
            (true, false) => {
                // (-a) + b = b - a
                let mut lhs_abs = self.clone();
                lhs_abs.neg = false;
                match lhs_abs.cmp_abs(rhs) {
                    Ordering::Greater | Ordering::Equal => {
                        lhs_abs.sub_abs_assign(rhs);
                        *self = lhs_abs;
                        self.neg = true && !self.is_zero();
                    }
                    Ordering::Less => {
                        let mut t = rhs.clone();
                        t.sub_abs_assign(&lhs_abs);
                        *self = t;
                        self.neg = false;
                    }
                }
            }
        }
        *self = self.clone().normalize();
    }
}

impl SubAssign<&HCVLangBigInt> for HCVLangBigInt {
    fn sub_assign(&mut self, rhs: &HCVLangBigInt) {
        // a - b = a + (-b)
        let mut t = rhs.clone();
        t.neg = !t.neg;
        *self += &t;
    }
}

impl MulAssign<&HCVLangBigInt> for HCVLangBigInt {
    fn mul_assign(&mut self, rhs: &HCVLangBigInt) {
        if self.is_zero() || rhs.is_zero() {
            *self = Self::zero();
            return;
        }
        let n = self.limbs.len();
        let m = rhs.limbs.len();
        let mut out = vec![0u64; n + m];
        for i in 0..n {
            let a = self.limbs[i] as u128;
            let mut carry = 0u128;
            for j in 0..m {
                let t = a * (rhs.limbs[j] as u128)
                    + (out[i + j] as u128)
                    + carry;
                out[i + j] = t as u64;
                carry = t >> 64;
            }
            if carry != 0 {
                out[i + m] = (out[i + m] as u128 + carry) as u64;
            }
        }
        let neg = self.neg ^ rhs.neg;
        *self = HCVLangBigInt { limbs: out, neg }.normalize();
    }
}

impl DivAssign<&HCVLangBigInt> for HCVLangBigInt {
    fn div_assign(&mut self, rhs: &HCVLangBigInt) {
        if rhs.is_zero() {
            panic!("division by zero");
        }
        if self.is_zero() {
            return;
        }
        if self.cmp_abs(rhs) == Ordering::Less {
            *self = Self::zero();
            return;
        }

        let a = self.abs();
        let b = rhs.abs();
        let mut q = Self::zero();
        let mut current = Self::zero();
        for i in (0..a.limbs.len() * 64).rev() {
            current <<= 1;
            if (a.limbs[i / 64] >> (i % 64)) & 1 == 1 {
                if current.limbs.is_empty() {
                    current.limbs.push(0);
                }
                current.limbs[0] |= 1;
            }
            current = current.normalize();
            if current.cmp_abs(&b) != Ordering::Less {
                current -= &b;
                if q.limbs.is_empty() {
                    q.limbs.resize(a.limbs.len(), 0);
                }
                q.limbs[i / 64] |= 1 << (i % 64);
            }
        }
        q = q.normalize();
        q.neg = self.neg ^ rhs.neg;
        *self = q.normalize();
    }
}

impl Div for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn div(mut self, rhs: HCVLangBigInt) -> HCVLangBigInt {
        self /= &rhs;
        self
    }
}

impl<'a> Div<&'a HCVLangBigInt> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn div(mut self, rhs: &HCVLangBigInt) -> HCVLangBigInt {
        self /= rhs;
        self
    }
}

impl RemAssign<&HCVLangBigInt> for HCVLangBigInt {
    fn rem_assign(&mut self, rhs: &HCVLangBigInt) {
        if rhs.is_zero() {
            panic!("division by zero");
        }
        if self.is_zero() {
            return;
        }
        let a = self.abs();
        let b = rhs.abs();
        if a.cmp_abs(&b) == Ordering::Less {
            return;
        }

        let mut q = Self::zero();
        let mut current = Self::zero();
        for i in (0..a.limbs.len() * 64).rev() {
            current <<= 1;
            if (a.limbs[i / 64] >> (i % 64)) & 1 == 1 {
                if current.limbs.is_empty() {
                    current.limbs.push(0);
                }
                current.limbs[0] |= 1;
            }
            current = current.normalize();
            if current.cmp_abs(&b) != Ordering::Less {
                current -= &b;
                if q.limbs.is_empty() {
                    q.limbs.resize(a.limbs.len(), 0);
                }
                q.limbs[i / 64] |= 1 << (i % 64);
            }
        }
        *self = current.normalize();
        self.neg = a.neg;
    }
}

impl Rem for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn rem(mut self, rhs: HCVLangBigInt) -> HCVLangBigInt {
        self %= &rhs;
        self
    }
}

impl<'a> Rem<&'a HCVLangBigInt> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn rem(mut self, rhs: &HCVLangBigInt) -> HCVLangBigInt {
        self %= rhs;
        self
    }
}


impl Add for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn add(mut self, rhs: HCVLangBigInt) -> HCVLangBigInt {
        self += &rhs;
        self
    }
}
impl<'a> Add<&'a HCVLangBigInt> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn add(mut self, rhs: &HCVLangBigInt) -> HCVLangBigInt {
        self += rhs;
        self
    }
}

impl Sub for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn sub(mut self, rhs: HCVLangBigInt) -> HCVLangBigInt {
        self -= &rhs;
        self
    }
}
impl<'a> Sub<&'a HCVLangBigInt> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn sub(mut self, rhs: &HCVLangBigInt) -> HCVLangBigInt {
        self -= rhs;
        self
    }
}

impl Mul for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn mul(mut self, rhs: HCVLangBigInt) -> HCVLangBigInt {
        self *= &rhs;
        self
    }
}
impl<'a> Mul<&'a HCVLangBigInt> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn mul(mut self, rhs: &HCVLangBigInt) -> HCVLangBigInt {
        self *= rhs;
        self
    }
}

impl<'a> Mul<u64> for &'a HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn mul(self, other: u64) -> HCVLangBigInt {
        let mut t = self.clone();
        t.mul_u64_assign(other);
        t
    }
}

impl Neg for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn neg(mut self) -> HCVLangBigInt {
        if !self.is_zero() {
            self.neg = !self.neg;
        }
        self
    }
}

/* Shifts */

impl ShlAssign<u32> for HCVLangBigInt {
    fn shl_assign(&mut self, rhs: u32) {
        if self.is_zero() || rhs == 0 {
            return;
        }
        let word_shift = (rhs / 64) as usize;
        let bit_shift = (rhs % 64) as u32;

        if word_shift > 0 {
            let mut v = vec![0u64; word_shift];
            v.extend_from_slice(&self.limbs);
            self.limbs = v;
        }
        if bit_shift == 0 {
            return;
        }
        let mut carry = 0u64;
        for w in &mut self.limbs {
            let new_carry = (*w >> (64 - bit_shift)) & ((1u64 << bit_shift) - 1);
            let t = ((*w as u128) << bit_shift) | (carry as u128);
            *w = t as u64;
            carry = new_carry;
        }
        if carry != 0 {
            self.limbs.push(carry);
        }
    }
}
impl Shl<u32> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn shl(mut self, rhs: u32) -> HCVLangBigInt {
        self <<= rhs;
        self
    }
}

impl ShrAssign<u32> for HCVLangBigInt {
    fn shr_assign(&mut self, rhs: u32) {
        if self.is_zero() || rhs == 0 {
            return;
        }
        let word_shift = (rhs / 64) as usize;
        let bit_shift = (rhs % 64) as u32;

        if word_shift >= self.limbs.len() {
            *self = Self::zero();
            return;
        }
        if word_shift > 0 {
            self.limbs.drain(0..word_shift);
        }
        if bit_shift == 0 {
            *self = self.clone().normalize();
            return;
        }
        
        let mut carry = 0u64; // Carry from the previous (more significant) limb
        for i in (0..self.limbs.len()).rev() {
            let limb = self.limbs[i];
            self.limbs[i] = (limb >> bit_shift) | (carry << (64 - bit_shift));
            carry = limb & ((1u64 << bit_shift) - 1); // Bits shifted out from current limb
        }
        *self = self.clone().normalize();
    }
}
impl Shr<u32> for HCVLangBigInt {
    type Output = HCVLangBigInt;
    fn shr(mut self, rhs: u32) -> HCVLangBigInt {
        self >>= rhs;
        self
    }
}

/* Display in base 10 */

impl fmt::Display for HCVLangBigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        // Divide by 1e9 chunks for pretty printing
        const BASE: u32 = 1_000_000_000;
        let mut tmp = self.abs();
        let mut parts: Vec<u32> = Vec::new();
        while !tmp.is_zero() {
            // divmod by BASE
            let mut rem: u128 = 0;
            for w in tmp.limbs.iter_mut().rev() {
                let cur = ((rem << 64) | (*w as u128)) as u128;
                *w = (cur / (BASE as u128)) as u64;
                rem = cur % (BASE as u128);
            }
            tmp = tmp.normalize();
            parts.push(rem as u32);
        }
        if self.is_negative() {
            write!(f, "-")?;
        }
        // print highest part without padding, rest with 9 digits
        if let Some(last) = parts.pop() {
            write!(f, "{}", last)?;
            while let Some(p) = parts.pop() {
                write!(f, "{:09}", p)?;
            }
        }
        Ok(())
    }
}

/* Froms */

impl From<i64> for HCVLangBigInt {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl PartialOrd for HCVLangBigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HCVLangBigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.neg, other.neg) {
            (false, false) => self.cmp_abs(other),
            (true, true) => other.cmp_abs(self),
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
        }
    }
}