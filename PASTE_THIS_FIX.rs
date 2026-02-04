// ============================================================================
// COPY-PASTE FIX: Per-Prime Diagnostic for test_ntt_ternary_mul
// ============================================================================
// Paste this at the top of your tests module (inside mod tests { ... })

fn center_mod_m_to_i128(x_mod_m: u128, m: u128) -> i128 {
    let half = m / 2;
    if x_mod_m > half { -((m - x_mod_m) as i128) } else { x_mod_m as i128 }
}

fn mod_i128(x: i128, p: u64) -> u64 {
    let p_i = p as i128;
    let mut r = x % p_i;
    if r < 0 { r += p_i; }
    r as u64
}

// ============================================================================
// Then replace the overflowing debug section with:
// ============================================================================

let v_m = ctx.rns.to_int(&as_main_0); // u128 is fine for main
let true_value = center_mod_m_to_i128(v_m, ctx.q_product);

println!("v_m (main CRT, mod M) = {}", v_m);
println!("true_value (centered) = {}", true_value);
println!("M = {}", ctx.q_product);
println!("anchor primes = {:?}", ctx.dual_rns.anchor.primes);

// Per-prime consistency check
for (i, &a_i) in ctx.dual_rns.anchor.primes.iter().enumerate() {
    let expected = mod_i128(true_value, a_i);
    let actual = as_anchor_0[i];

    assert_eq!(
        expected, actual,
        "Anchor residue mismatch at prime[{}]={}:\n  expected (true_value mod a_i) = {}\n  actual   (anchor limb)        = {}\n  v_m(mod M)={}  true_value(centered)={}",
        i, a_i, expected, actual, v_m, true_value
    );
}

// ============================================================================
// DELETE these lines (they overflow):
// ============================================================================
// let anchor_product = ctx.dual_rns.anchor.product();  // DELETE
// let v_a = ctx.dual_rns.anchor.to_int(&as_anchor_0);  // DELETE
// let main_inv_anchor = ...;                            // DELETE
// ============================================================================
