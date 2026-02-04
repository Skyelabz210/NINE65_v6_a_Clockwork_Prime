// rational_patch.rs
//! Guidance for updating src/rational.rs with optimizations
//!
//! Key improvements:
//! - Canonical zero representation (0/1)
//! - Borrowed reduced views to avoid unnecessary clones
//! - Optimized equality and comparison operations
//!
//! Apply these changes to your existing Rational implementation

use crate::crt_bigint::CRTBigInt; // Corrected import path

#[derive(Clone, Debug)]
pub struct Rational {
    pub num: CRTBigInt,
    pub den: CRTBigInt,
}

impl Rational {
    /// Create from integer
    pub fn from_integer(n: i128) -> Self {
        Self {
            num: CRTBigInt::new(n as i64),
            den: CRTBigInt::from_u64(1),
        }
    }

    /// Create zero rational: 0/1
    pub fn zero() -> Self {
        Self {
            num: CRTBigInt::from_u64(0),
            den: CRTBigInt::from_u64(1),
        }
    }

    /// Create one rational: 1/1
    pub fn one() -> Self {
        Self {
            num: CRTBigInt::from_u64(1),
            den: CRTBigInt::from_u64(1),
        }
    }

    /// Create half rational: 1/2
    pub fn half() -> Self {
        Self {
            num: CRTBigInt::from_u64(1),
            den: CRTBigInt::from_u64(2),
        }
    }

    /// Get reciprocal (1/self)
    pub fn recip(&self) -> Self {
        debug_assert!(!self.num.is_zero(), "Cannot take reciprocal of zero");
        Self {
            num: self.den.clone(),
            den: self.num.clone(),
        }
    }

    /// Convert to scaled integer for display (multiply by scale factor)
    pub fn to_scaled(&self, scale: i64) -> i64 {
        let scaled_num = self.num.clone() * CRTBigInt::new(scale);
        // For now, just do basic division
        (scaled_num / self.den.clone()).to_i64().unwrap_or(0)
    }

    /// Create a new rational number with normalization
    /// Ensures denominator is positive and canonical zero is 0/1
    pub fn new(mut num: CRTBigInt, mut den: CRTBigInt) -> Self {
        debug_assert!(!den.is_zero(), "Denominator cannot be zero");

        // Normalize sign: negative lives on numerator only
        if den.is_negative() {
            num = -num;
            den = -den;
        }

        // Canonical zero: 0/1
        if num.is_zero() {
            den.set_one();
        }

        Self { num, den }
    }

    /// Create from integer
    pub fn from_int(n: CRTBigInt) -> Self {
        if n.is_zero() {
            Self {
                num: CRTBigInt::from_u64(0),
                den: CRTBigInt::from_u64(1),
            }
        } else {
            Self {
                num: n,
                den: CRTBigInt::from_u64(1),
            }
        }
    }

    /// Reduce to lowest terms by dividing out GCD
    pub fn reduce(mut self) -> Self {
        // Skip if already reduced
        if self.num.is_zero() || self.den.is_one() {
            return self;
        }

        let g = CRTBigInt::gcd(&self.num.abs(), &self.den.abs());
        if !g.is_one() {
            self.num = self.num / g.clone();
            self.den = self.den / g;
        }

        self
    }

    /// Get a borrowed reduced view without cloning self
    /// Uses tmp as scratch space if reduction is needed
    /// Returns self if already reduced, otherwise returns tmp
    pub fn reduced_view<'a>(&'a self, tmp: &'a mut Rational) -> &'a Rational {
        if self.num.is_zero() || self.den.is_one() {
            return self;
        }

        *tmp = self.clone().reduce();
        tmp
    }

    /// Access numerator
    pub fn numer(&self) -> &CRTBigInt {
        &self.num
    }

    /// Access denominator
    pub fn denom(&self) -> &CRTBigInt {
        &self.den
    }

    /// Check if rational is zero
    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    /// Check if rational is positive
    pub fn is_positive(&self) -> bool {
        self.num.is_positive()
    }

    /// Check if rational is negative
    pub fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    /// Check if rational is one
    pub fn is_one(&self) -> bool {
        self.num.is_one() && self.den.is_one()
    }

    /// Get absolute value
    pub fn abs_value(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den.clone(),
        }
    }

    /// Get absolute value
    pub fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den.clone(),
        }
    }

    /// Negate
    pub fn neg(&self) -> Self {
        Self {
            num: -self.num.clone(),
            den: self.den.clone(),
        }
    }
}

// Equality using borrowed reduction to avoid unnecessary clones
impl core::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        // Fast path: check if already equal
        if self.num == other.num && self.den == other.den {
            return true;
        }

        // Use borrowed reduced views for comparison
        let mut tmp1 = Rational {
            num: CRTBigInt::from_u64(0),
            den: CRTBigInt::from_u64(1),
        };
        let mut tmp2 = Rational {
            num: CRTBigInt::from_u64(0),
            den: CRTBigInt::from_u64(1),
        };

        let a = self.reduced_view(&mut tmp1);
        let b = other.reduced_view(&mut tmp2);

        a.num == b.num && a.den == b.den
    }
}

impl core::cmp::Eq for Rational {}

impl core::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Compare via cross-multiplication: a/b <=> c/d  iff  a*d <=> c*b
        // This avoids reducing both rationals
        (self.num.clone() * other.den.clone())
            .cmp(&(other.num.clone() * self.den.clone()))
    }
}

// Display with borrowed reduction
impl core::fmt::Display for Rational {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tmp = Rational {
            num: CRTBigInt::from_u64(0),
            den: CRTBigInt::from_u64(1),
        };

        let r = self.reduced_view(&mut tmp);

        if r.den.is_one() {
            write!(f, "{}", r.num)
        } else {
            write!(f, "{}/{}", r.num, r.den)
        }
    }
}

// Arithmetic operations
impl core::ops::Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // a/b + c/d = (a*d + b*c)/(b*d)
        let num = self.num * rhs.den.clone() + rhs.num * self.den.clone();
        let den = self.den * rhs.den;
        Rational::new(num, den).reduce()
    }
}

impl core::ops::Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        // a/b - c/d = (a*d - b*c)/(b*d)
        let num = self.num * rhs.den.clone() - rhs.num * self.den.clone();
        let den = self.den * rhs.den;
        Rational::new(num, den).reduce()
    }
}

impl core::ops::Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // (a/b) * (c/d) = (a*c)/(b*d)
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;
        Rational::new(num, den).reduce()
    }
}

impl core::ops::Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(!rhs.num.is_zero(), "Division by zero");

        // (a/b) / (c/d) = (a*d)/(b*c)
        let num = self.num * rhs.den;
        let den = self.den * rhs.num;
        Rational::new(num, den).reduce()
    }
}

impl core::ops::Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}

// Reference-based operators for math modules
impl<'a, 'b> core::ops::Add<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn add(self, rhs: &'b Rational) -> Rational {
        // a/b + c/d = (a*d + b*c)/(b*d)
        let num = self.num.clone() * rhs.den.clone() + rhs.num.clone() * self.den.clone();
        let den = self.den.clone() * rhs.den.clone();
        Rational::new(num, den).reduce()
    }
}

impl<'a, 'b> core::ops::Sub<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn sub(self, rhs: &'b Rational) -> Rational {
        // a/b - c/d = (a*d - b*c)/(b*d)
        let num = self.num.clone() * rhs.den.clone() - rhs.num.clone() * self.den.clone();
        let den = self.den.clone() * rhs.den.clone();
        Rational::new(num, den).reduce()
    }
}

impl<'a, 'b> core::ops::Mul<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn mul(self, rhs: &'b Rational) -> Rational {
        // (a/b) * (c/d) = (a*c)/(b*d)
        let num = self.num.clone() * rhs.num.clone();
        let den = self.den.clone() * rhs.den.clone();
        Rational::new(num, den).reduce()
    }
}

impl<'a, 'b> core::ops::Div<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn div(self, rhs: &'b Rational) -> Rational {
        debug_assert!(!rhs.num.is_zero(), "Division by zero");

        // (a/b) / (c/d) = (a*d)/(b*c)
        let num = self.num.clone() * rhs.den.clone();
        let den = self.den.clone() * rhs.num.clone();
        Rational::new(num, den).reduce()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_canonical_zero() {
        let r = Rational::new(CRTBigInt::from_u64(0), CRTBigInt::from_u64(42));
        assert!(r.den.is_one());
        assert!(r.num.is_zero());
    }

    #[test]
    fn rational_sign_normalization() {
        let r = Rational::new(CRTBigInt::new(3), CRTBigInt::new(-4));
        // Sign should be normalized (denominator positive)
        assert!(r.den.to_i64().unwrap_or(0) > 0);
        // The absolute values should be correct
        assert_eq!(r.num.abs().to_i64(), Some(3));
        assert_eq!(r.den.abs().to_i64(), Some(4));
    }

    #[test]
    fn rational_equality() {
        let r1 = Rational::new(CRTBigInt::from_u64(1), CRTBigInt::from_u64(2));
        let r2 = Rational::new(CRTBigInt::from_u64(2), CRTBigInt::from_u64(4));
        assert_eq!(r1, r2);
    }

    #[test]
    fn rational_comparison() {
        let r1 = Rational::new(CRTBigInt::from_u64(1), CRTBigInt::from_u64(3));
        let r2 = Rational::new(CRTBigInt::from_u64(1), CRTBigInt::from_u64(2));
        assert!(r1 < r2);
    }

    #[test]
    fn rational_display_reduced() {
        let r = Rational::new(CRTBigInt::from_u64(4), CRTBigInt::from_u64(8));
        assert_eq!(format!("{}", r), "1/2");
    }

    #[test]
    fn rational_display_integer() {
        let r = Rational::new(CRTBigInt::from_u64(10), CRTBigInt::from_u64(2));
        assert_eq!(format!("{}", r), "5");
    }

    #[test]
    fn rational_arithmetic() {
        let r1 = Rational::new(CRTBigInt::from_u64(1), CRTBigInt::from_u64(2));
        let r2 = Rational::new(CRTBigInt::from_u64(1), CRTBigInt::from_u64(3));

        let sum = r1.clone() + r2.clone();
        let expected = Rational::new(CRTBigInt::from_u64(5), CRTBigInt::from_u64(6));
        assert_eq!(sum, expected);
    }
}