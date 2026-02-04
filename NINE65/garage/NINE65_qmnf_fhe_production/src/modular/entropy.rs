//! Entropy source trait for pluggable randomness.

pub trait EntropySource {
    fn next_u64(&mut self) -> u64;

    fn uniform(&mut self, modulus: u64) -> u64 {
        if modulus == 0 {
            return 0;
        }
        self.next_u64() % modulus
    }

    fn ternary(&mut self) -> i64 {
        let v = self.next_u64() % 3;
        match v {
            0 => -1,
            1 => 0,
            _ => 1,
        }
    }

    fn ternary_vector(&mut self, n: usize) -> Vec<i64> {
        (0..n).map(|_| self.ternary()).collect()
    }
}
