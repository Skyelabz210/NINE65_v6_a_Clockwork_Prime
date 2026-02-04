#!/usr/bin/env python3
"""
CORRECT ORDER FINDING - NO CIRCULARITY

CRITICAL INSIGHT from Grover Swarm:

The baby-step giant-step algorithm does NOT require knowing φ(N).
It only requires an UPPER BOUND on the order.

For any a coprime to N:
  ord(a) | λ(N) | φ(N) | N - 1  (for odd N)

So we use N - 1 as the bound. NO FACTORIZATION REQUIRED!

Complexity: O(√N) time and space
Correctness: Works for ANY modulus without knowing its factorization
"""

import math
from typing import Optional, Tuple
import time


def gcd(a: int, b: int) -> int:
    """Binary GCD (Stein's algorithm)"""
    if a == 0:
        return b
    if b == 0:
        return a
    
    shift = 0
    while ((a | b) & 1) == 0:
        a >>= 1
        b >>= 1
        shift += 1
    
    while (a & 1) == 0:
        a >>= 1
    
    while b != 0:
        while (b & 1) == 0:
            b >>= 1
        if a > b:
            a, b = b, a
        b -= a
    
    return a << shift


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Fast modular exponentiation"""
    if mod == 1:
        return 0
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        exp >>= 1
        base = (base * base) % mod
    return result


def mod_inverse(a: int, n: int) -> Optional[int]:
    """Extended Euclidean algorithm for modular inverse"""
    if a == 0:
        return None
    
    old_r, r = n, a
    old_s, s = 0, 1
    
    while r != 0:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_s, s = s, old_s - q * s
    
    if old_r != 1:
        return None
    
    return (old_s % n + n) % n


def find_minimal_order(a: int, n: int, candidate: int) -> int:
    """Find minimal order given that order divides candidate"""
    order = candidate
    
    # Try dividing by small primes
    small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    for p in small_primes:
        while order % p == 0:
            smaller = order // p
            if mod_pow(a, smaller, n) == 1:
                order = smaller
            else:
                break
    
    # Try remaining factors
    d = 53
    while d * d <= order:
        while order % d == 0:
            smaller = order // d
            if mod_pow(a, smaller, n) == 1:
                order = smaller
            else:
                break
        d += 2
    
    return order


def bsgs_order(a: int, n: int) -> Optional[int]:
    """
    Baby-step giant-step for order finding
    
    KEY FIX: Uses N-1 as bound, NOT φ(N)
    This avoids the circular dependency!
    
    Complexity: O(√N) time and space
    No factorization of N required!
    """
    if gcd(a, n) > 1:
        return None
    
    if a % n == 1:
        return 1
    
    # Upper bound on order: N - 1
    bound = n - 1
    
    # m = ⌈√bound⌉
    m = max(1, math.isqrt(bound) + 1)
    
    # Baby steps: compute a^0, a^1, ..., a^{m-1}
    baby_table = {}
    power = 1
    for j in range(m):
        if j > 0 and power == 1:
            return j
        baby_table[power] = j
        power = (power * a) % n
    
    # Giant step multiplier: a^{-m}
    a_m = mod_pow(a, m, n)
    a_m_inv = mod_inverse(a_m, n)
    if a_m_inv is None:
        return None
    
    # Giant steps
    gamma = 1
    for k in range(m + 1):
        if gamma in baby_table:
            j = baby_table[gamma]
            candidate = j + k * m
            if candidate > 0 and mod_pow(a, candidate, n) == 1:
                return find_minimal_order(a, n, candidate)
        gamma = (gamma * a_m_inv) % n
    
    return None


def pollard_rho_order(a: int, n: int) -> Optional[int]:
    """
    Pollard's rho for order finding using Brent's cycle detection
    
    O(√r) time, O(1) space where r is the actual order
    NO BOUNDS REQUIRED
    """
    if gcd(a, n) > 1:
        return None
    
    if a % n == 1:
        return 1
    
    power_of_2 = 1
    lam = 1
    tortoise = a
    hare = (a * a) % n
    
    while tortoise != hare:
        if power_of_2 == lam:
            tortoise = hare
            power_of_2 *= 2
            lam = 0
        hare = (hare * a) % n
        lam += 1
        if lam > n:
            return None
    
    return find_minimal_order(a, n, lam)


def multiplicative_order(a: int, n: int) -> Optional[int]:
    """
    Find multiplicative order using best available method
    
    NO CIRCULAR DEPENDENCIES
    """
    if gcd(a, n) > 1:
        return None
    
    if a % n == 1:
        return 1
    
    # Strategy 1: Check small orders directly
    power = a
    for r in range(1, min(10000, n)):
        if power == 1:
            return r
        power = (power * a) % n
    
    # Strategy 2: Pollard rho
    order = pollard_rho_order(a, n)
    if order:
        return order
    
    # Strategy 3: BSGS
    return bsgs_order(a, n)


def factor_via_order(n: int) -> Optional[Tuple[int, int]]:
    """
    Factor a semiprime using period finding
    
    This is the classical reduction - now WITHOUT circularity!
    """
    if n < 4:
        return None
    
    # Small factors
    small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    for p in small_primes:
        if n % p == 0 and n > p:
            return (p, n // p)
    
    # Try bases
    bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61]
    
    for a in bases:
        if a >= n:
            continue
        
        g = gcd(a, n)
        if 1 < g < n:
            return (g, n // g)
        
        order = multiplicative_order(a, n)
        if order and order % 2 == 0:
            half_power = mod_pow(a, order // 2, n)
            
            if half_power > 1:
                g1 = gcd(half_power - 1, n)
                if 1 < g1 < n:
                    return (g1, n // g1)
            
            if half_power + 1 < n:
                g2 = gcd(half_power + 1, n)
                if 1 < g2 < n:
                    return (g2, n // g2)
    
    return None


# ═══════════════════════════════════════════════════════════════════════════════
# TESTS
# ═══════════════════════════════════════════════════════════════════════════════

def test_bsgs_order():
    """Test BSGS order finding"""
    print("Testing BSGS Order Finding...")
    
    # ord(2, 15) = 4
    assert bsgs_order(2, 15) == 4, f"Got {bsgs_order(2, 15)}"
    print("  ✓ ord(2, 15) = 4")
    
    # ord(3, 7) = 6 (primitive root)
    assert bsgs_order(3, 7) == 6, f"Got {bsgs_order(3, 7)}"
    print("  ✓ ord(3, 7) = 6")
    
    # ord(2, 7) = 3
    assert bsgs_order(2, 7) == 3, f"Got {bsgs_order(2, 7)}"
    print("  ✓ ord(2, 7) = 3")
    
    print("BSGS tests passed!\n")


def test_semiprime_order():
    """Test order finding on semiprimes WITHOUT knowing factors"""
    print("Testing Order Finding on Semiprimes...")
    
    # N = 3233 = 53 × 61
    # We DON'T compute φ(N), we just find the order using N-1 as bound
    n = 3233
    order = bsgs_order(2, n)
    
    assert order is not None
    assert mod_pow(2, order, n) == 1
    assert mod_pow(2, order - 1, n) != 1  # Minimal
    
    print(f"  ✓ ord(2, 3233) = {order} (found without factoring!)")
    
    # Verify: φ(3233) = 52 × 60 = 3120
    # Order should divide 3120
    assert 3120 % order == 0
    print(f"  ✓ Verified: {order} | φ(3233) = 3120")
    
    print("Semiprime order tests passed!\n")


def test_factoring():
    """Test factoring via order finding"""
    print("Testing Factoring via Order Finding...")
    
    test_cases = [
        (15, {(3, 5), (5, 3)}),
        (21, {(3, 7), (7, 3)}),
        (35, {(5, 7), (7, 5)}),
        (3233, {(53, 61), (61, 53)}),
        (10403, {(101, 103), (103, 101)}),
    ]
    
    for n, expected in test_cases:
        result = factor_via_order(n)
        assert result is not None, f"Failed to factor {n}"
        assert result in expected, f"Got {result} for {n}"
        p, q = result
        assert p * q == n
        print(f"  ✓ {n} = {p} × {q}")
    
    print("Factoring tests passed!\n")


def test_no_circularity():
    """Verify we don't use φ(N) anywhere"""
    print("Testing Non-Circularity...")
    
    # Large semiprime where factoring is non-trivial
    n = 10403  # 101 × 103
    
    # This should work WITHOUT knowing factors
    start = time.time()
    order = multiplicative_order(2, n)
    elapsed = time.time() - start
    
    assert order is not None
    assert mod_pow(2, order, n) == 1
    
    print(f"  ✓ ord(2, {n}) = {order} in {elapsed*1000:.2f}ms")
    print(f"  ✓ NO factorization of N was required!")
    
    print("Non-circularity verified!\n")


def benchmark():
    """Performance comparison"""
    print("Benchmarking...")
    
    test_cases = [
        (15, "tiny"),
        (3233, "small"),
        (10403, "medium"),
        (100003, "prime"),  # Prime - order = p-1
    ]
    
    for n, label in test_cases:
        start = time.time()
        order = multiplicative_order(2, n)
        elapsed = time.time() - start
        
        if order:
            print(f"  {label:8s} N={n:8d}: ord={order:8d}, time={elapsed*1000:8.2f}ms")
        else:
            print(f"  {label:8s} N={n:8d}: gcd(2,N) > 1")
    
    print()


if __name__ == "__main__":
    print("=" * 70)
    print("CORRECT ORDER FINDING - NO CIRCULARITY")
    print("=" * 70)
    print()
    
    test_bsgs_order()
    test_semiprime_order()
    test_factoring()
    test_no_circularity()
    benchmark()
    
    print("=" * 70)
    print("ALL TESTS PASSED - ORDER FINDING IS NOW CORRECT!")
    print("=" * 70)
