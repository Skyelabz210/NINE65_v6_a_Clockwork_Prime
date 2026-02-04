//! Adaptive CRTBigInt - Dynamic Precision Tier Management
//!
//! # Design Philosophy
//!
//! Automatic precision scaling with ZERO floating-point operations:
//! - All thresholds in permille (‰, parts per thousand)
//! - Adaptive adjustment via Q16 fixed-point EMA
//! - Operations-based cooldown prevents oscillation
//! - Integer-only cost model for deterministic decisions
//!
//! # Hysteresis Strategy
//!
//! Two-layer protection against tier thrashing:
//!
//! 1. **Value-space hysteresis**: 500‰ gap between promote/demote
//!    - Base promote: 900‰ (90% capacity)
//!    - Base demote: 400‰ (40% capacity)
//!    - Adaptive adjustment: ±50‰ based on bit-growth EMA
//!
//! 2. **Time-space hysteresis**: Cooldown period after transitions
//!    - Default: 2048 operations
//!    - Blocks all transitions during cooldown
//!    - Resets on each transition
//!
//! # Performance Model
//!
//! All costs measured in "mm units" (Montgomery multiplications):
//! - Tier operation cost: function of prime count
//! - Reconstruction cost: one-time overhead
//! - Amortization horizon: 1024 operations
//! - Transition only if: Δcost × horizon > reconstruction_cost
//!
//! # Determinism Guarantee
//!
//! Given identical operation sequence, tier transitions are:
//! - Reproducible across platforms
//! - Independent of wall-clock time
//! - Deterministic function of operation count
//!
//! This enables:
//! - Bitwise-identical results in distributed systems
//! - Reproducible performance profiling
//! - Formal verification of tier selection logic

use std::fmt;

// ============================================================================
// SECTION 1: TIER CONFIGURATION
// ============================================================================

/// Precision tier enumeration
///
/// Each tier doubles the prime count and capacity.
/// Start with 4 tiers; extend to 7+ via table edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PrecisionTier {
    Tier0 = 0,  // 1 prime   ~30 bits
    Tier1 = 1,  // 2 primes  ~60 bits
    Tier2 = 2,  // 4 primes  ~120 bits
    Tier3 = 3,  // 8 primes  ~240 bits
    // Future expansion (table-driven):
    // Tier4 = 4,  // 16 primes ~480 bits
    // Tier5 = 5,  // 32 primes ~960 bits
    // Tier6 = 6,  // 64 primes ~1920 bits
}

impl PrecisionTier {
    /// Get tier from index
    pub fn from_index(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(PrecisionTier::Tier0),
            1 => Some(PrecisionTier::Tier1),
            2 => Some(PrecisionTier::Tier2),
            3 => Some(PrecisionTier::Tier3),
            _ => None,
        }
    }

    /// Get next higher tier
    pub fn promote(self) -> Option<Self> {
        Self::from_index((self as u8) + 1)
    }

    /// Get next lower tier
    pub fn demote(self) -> Option<Self> {
        if self as u8 == 0 {
            None
        } else {
            Self::from_index((self as u8) - 1)
        }
    }

    /// Get tier index
    pub fn index(self) -> u8 {
        self as u8
    }
}

/// Tier metadata and cost model
///
/// All costs in "mm units" (Montgomery multiplication equivalent).
/// This makes cost comparison platform-independent and deterministic.
#[derive(Debug, Clone)]
pub struct TierInfo {
    pub tier: PrecisionTier,
    pub prime_count: u16,
    pub capacity_bits: u16,
    
    // Cost model (mm units)
    pub add_cost_mm: u16,      // Cost of one addition
    pub mul_cost_mm: u16,      // Cost of one multiplication
    pub recon_cost_mm: u32,    // Cost to reconstruct to i128/BigInt
}

/// Tier information table
///
/// Extend this table to support Tier4-Tier6 without logic changes.
pub const TIER_TABLE: [TierInfo; 4] = [
    TierInfo {
        tier: PrecisionTier::Tier0,
        prime_count: 1,
        capacity_bits: 30,
        add_cost_mm: 1,
        mul_cost_mm: 2,
        recon_cost_mm: 5,
    },
    TierInfo {
        tier: PrecisionTier::Tier1,
        prime_count: 2,
        capacity_bits: 60,
        add_cost_mm: 2,
        mul_cost_mm: 4,
        recon_cost_mm: 12,
    },
    TierInfo {
        tier: PrecisionTier::Tier2,
        prime_count: 4,
        capacity_bits: 120,
        add_cost_mm: 4,
        mul_cost_mm: 8,
        recon_cost_mm: 30,
    },
    TierInfo {
        tier: PrecisionTier::Tier3,
        prime_count: 8,
        capacity_bits: 240,
        add_cost_mm: 8,
        mul_cost_mm: 16,
        recon_cost_mm: 70,
    },
];

/// Get tier info by tier enum
#[inline]
pub fn tier_info(tier: PrecisionTier) -> &'static TierInfo {
    &TIER_TABLE[tier.index() as usize]
}

// ============================================================================
// SECTION 2: ADAPTIVE THRESHOLD MANAGEMENT
// ============================================================================

/// Adaptive threshold controller
///
/// Adjusts promote/demote thresholds based on observed bit growth.
/// All arithmetic in fixed-point Q16 format (no floats).
#[derive(Debug, Clone)]
pub struct AdaptiveThresholds {
    // Base thresholds (permille)
    base_promote_permille: u16,  // 900 = 90.0%
    base_demote_permille: u16,   // 400 = 40.0%
    
    // Adaptive adjustment (permille)
    adj_cap_permille: u16,       // 50 = max ±5.0% adjustment
    
    // EMA of bit growth per operation (Q16 fixed-point)
    ema_bits_per_op_q16: u32,
    ema_shift: u8,               // 4 → α = 1/16
    adj_scale_shift: u8,         // 10 → maps Q16 to permille
    
    // Cooldown (operations-based)
    cooldown_ops: u32,
    ops_since_transition: u32,
}

impl AdaptiveThresholds {
    /// Create with sensible defaults
    pub fn new() -> Self {
        Self {
            base_promote_permille: 900,
            base_demote_permille: 400,
            adj_cap_permille: 50,
            ema_bits_per_op_q16: 0,
            ema_shift: 4,
            adj_scale_shift: 10,
            cooldown_ops: 2048,
            ops_since_transition: 2048, // Start ready
        }
    }

    /// Update EMA with new bit growth sample
    ///
    /// # Arguments
    ///
    /// * `bit_growth` - Number of bits grown since last check
    /// * `ops_elapsed` - Number of operations since last check
    pub fn update_ema(&mut self, bit_growth: u32, ops_elapsed: u32) {
        if ops_elapsed == 0 {
            return;
        }

        // Calculate bits per op in Q16: (bit_growth << 16) / ops_elapsed
        let sample_q16 = ((bit_growth as u64) << 16) / ops_elapsed as u64;
        let sample_q16 = sample_q16.min(u32::MAX as u64) as u32;

        // EMA update: ema += (sample - ema) >> shift
        let delta = if sample_q16 >= self.ema_bits_per_op_q16 {
            sample_q16 - self.ema_bits_per_op_q16
        } else {
            self.ema_bits_per_op_q16 - sample_q16
        };
        
        let adjustment = delta >> self.ema_shift;
        
        if sample_q16 >= self.ema_bits_per_op_q16 {
            self.ema_bits_per_op_q16 += adjustment;
        } else {
            self.ema_bits_per_op_q16 = self.ema_bits_per_op_q16.saturating_sub(adjustment);
        }
    }

    /// Get current adaptive adjustment (permille)
    ///
    /// Maps EMA to 0..adj_cap_permille range
    fn adaptive_adjustment_permille(&self) -> u16 {
        let adj = (self.ema_bits_per_op_q16 >> self.adj_scale_shift) as u16;
        adj.min(self.adj_cap_permille)
    }

    /// Get current promote threshold (permille)
    pub fn promote_threshold_permille(&self) -> u16 {
        let adj = self.adaptive_adjustment_permille();
        self.base_promote_permille.saturating_sub(adj)
    }

    /// Get current demote threshold (permille)
    pub fn demote_threshold_permille(&self) -> u16 {
        let adj = self.adaptive_adjustment_permille();
        self.base_demote_permille.saturating_add(adj).min(999)
    }

    /// Check if transition is allowed (not in cooldown)
    pub fn can_transition(&self) -> bool {
        self.ops_since_transition >= self.cooldown_ops
    }

    /// Record a transition (resets cooldown)
    pub fn record_transition(&mut self) {
        self.ops_since_transition = 0;
    }

    /// Increment operation counter
    pub fn increment_ops(&mut self, count: u32) {
        self.ops_since_transition = self.ops_since_transition.saturating_add(count);
    }
}

impl Default for AdaptiveThresholds {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SECTION 3: UTILIZATION AND TIER SELECTION
// ============================================================================

/// Calculate utilization in permille (‰)
///
/// # Arguments
///
/// * `bits` - Current bit bound of value
/// * `cap_bits` - Capacity in bits of tier
///
/// # Returns
///
/// Utilization from 0‰ to 1000‰ (0% to 100%)
#[inline]
pub fn utilization_permille(bits: u32, cap_bits: u16) -> u16 {
    let cap = cap_bits as u32;
    if cap == 0 {
        return 1000;
    }
    let util = bits.saturating_mul(1000).saturating_add(cap - 1) / cap;
    util.min(1000) as u16
}

/// Select lowest tier that safely fits value
///
/// # Arguments
///
/// * `bits` - Bit bound of value
/// * `safety_bits` - Extra headroom (default: 24)
///
/// # Returns
///
/// Lowest tier that satisfies: tier.capacity >= bits + safety_bits
pub fn select_tier_for_bits(bits: u32, safety_bits: u16) -> PrecisionTier {
    let required_bits = bits.saturating_add(safety_bits as u32);
    
    for info in &TIER_TABLE {
        if info.capacity_bits as u32 >= required_bits {
            return info.tier;
        }
    }
    
    // If we exceed Tier3, stay at Tier3
    // (Future: extend table or return error)
    PrecisionTier::Tier3
}

/// Calculate transition cost delta (mm units)
///
/// Positive if candidate tier is cheaper per operation.
///
/// # Arguments
///
/// * `current` - Current tier
/// * `candidate` - Candidate tier
/// * `mul_heavy` - Weight toward mul cost (0.0 = all add, 1.0 = all mul)
///
/// # Returns
///
/// Cost delta in mm units (positive = candidate is cheaper)
pub fn tier_cost_delta_mm(current: PrecisionTier, candidate: PrecisionTier, mul_weight_q16: u32) -> i32 {
    let cur_info = tier_info(current);
    let cand_info = tier_info(candidate);
    
    // Weighted average: cost = add_cost * (1 - w) + mul_cost * w
    // Using Q16: cost = (add_cost << 16) * (1 - w) + (mul_cost << 16) * w
    //                 = (add_cost << 16) + (mul_cost - add_cost) * w
    
    let cur_add = cur_info.add_cost_mm as u32;
    let cur_mul = cur_info.mul_cost_mm as u32;
    let cand_add = cand_info.add_cost_mm as u32;
    let cand_mul = cand_info.mul_cost_mm as u32;
    
    // Q16 calculation
    let cur_cost_q16 = (cur_add << 16) + (cur_mul.saturating_sub(cur_add)) * mul_weight_q16;
    let cand_cost_q16 = (cand_add << 16) + (cand_mul.saturating_sub(cand_add)) * mul_weight_q16;
    
    // Delta in mm units (shift back from Q16)
    let delta_q16 = cur_cost_q16 as i64 - cand_cost_q16 as i64;
    (delta_q16 >> 16) as i32
}

/// Check if transition is worth the reconstruction cost
///
/// # Arguments
///
/// * `current` - Current tier
/// * `candidate` - Candidate tier
/// * `horizon_ops` - Amortization horizon in operations
/// * `mul_weight_q16` - Mul/add mix (Q16: 0..65536)
///
/// # Returns
///
/// True if transition saves cost over horizon
pub fn is_transition_worthwhile(
    current: PrecisionTier,
    candidate: PrecisionTier,
    horizon_ops: u32,
    mul_weight_q16: u32,
) -> bool {
    if current == candidate {
        return false;
    }

    let delta_mm = tier_cost_delta_mm(current, candidate, mul_weight_q16);
    if delta_mm <= 0 {
        return false; // Candidate isn't cheaper
    }

    let savings_mm = (delta_mm as u32).saturating_mul(horizon_ops);
    
    // Reconstruction cost depends on direction
    let recon_mm = if candidate > current {
        // Promotion: reconstruct current to get full value
        tier_info(current).recon_cost_mm
    } else {
        // Demotion: reconstruct to verify fit + initialize new tier
        tier_info(candidate).recon_cost_mm
    };

    savings_mm > recon_mm
}

// ============================================================================
// SECTION 4: ADAPTIVE CRTBigInt
// ============================================================================

/// Adaptive CRTBigInt with automatic tier management
///
/// # Guarantees
///
/// - Zero floating-point operations
/// - Deterministic tier selection
/// - Bounded oscillation (max 1 transition per cooldown)
/// - Exact integer arithmetic throughout
///
/// # Usage
///
/// ```ignore
/// let mut x = AdaptiveCRTBigInt::new(42);
/// let mut y = AdaptiveCRTBigInt::new(17);
/// 
/// // Automatic tier management during operations
/// x = x.add(&y)?;
/// x = x.mul(&y)?;
/// 
/// // Tier transitions happen invisibly when needed
/// ```
#[derive(Debug, Clone)]
pub struct AdaptiveCRTBigInt {
    // CRT representation
    residues: Vec<u64>,
    moduli: Vec<u64>,
    current_tier: PrecisionTier,
    
    // Bit tracking (upper bound)
    bit_bound: u32,
    last_check_bits: u32,
    
    // Evaluation cadence
    check_interval_ops: u32,
    ops_since_check: u32,
    
    // Adaptive thresholds
    thresholds: AdaptiveThresholds,
    
    // Amortization model
    horizon_ops: u32,
    mul_weight_q16: u32,  // Q16: 0 = all add, 65536 = all mul
    
    // Telemetry
    operations_count: u64,
    tier_transitions: u64,
}

/// Error type for adaptive operations
#[derive(Debug, Clone)]
pub enum AdaptiveCRTError {
    OverflowDetected,
    InvalidTier,
    ReconstructionFailed,
}

impl fmt::Display for AdaptiveCRTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverflowDetected => write!(f, "Value exceeds maximum tier capacity"),
            Self::InvalidTier => write!(f, "Invalid tier specification"),
            Self::ReconstructionFailed => write!(f, "CRT reconstruction failed"),
        }
    }
}

impl std::error::Error for AdaptiveCRTError {}

pub type AdaptiveCRTResult<T> = Result<T, AdaptiveCRTError>;

impl AdaptiveCRTBigInt {
    /// Create new adaptive CRT value
    ///
    /// Starts at lowest tier that fits the value.
    pub fn new(value: i64) -> Self {
        let bits = if value == 0 {
            1
        } else {
            (value.abs().ilog2() + 1) as u32
        };
        
        let tier = select_tier_for_bits(bits, 24);
        
        // Initialize CRT representation (stub - needs actual implementation)
        let info = tier_info(tier);
        let moduli = generate_primes_for_tier(tier);
        let residues = value_to_residues(value, &moduli);
        
        Self {
            residues,
            moduli,
            current_tier: tier,
            bit_bound: bits,
            last_check_bits: bits,
            check_interval_ops: 64,
            ops_since_check: 0,
            thresholds: AdaptiveThresholds::new(),
            horizon_ops: 1024,
            mul_weight_q16: 32768, // 50% mul, 50% add
            operations_count: 0,
            tier_transitions: 0,
        }
    }

    /// Check and perform tier adjustment if needed
    ///
    /// Called periodically (every check_interval_ops).
    fn maybe_adjust_tier(&mut self) -> AdaptiveCRTResult<()> {
        // Reset check counter
        self.ops_since_check = 0;
        
        // Update EMA
        let bit_growth = self.bit_bound.saturating_sub(self.last_check_bits);
        self.thresholds.update_ema(bit_growth, self.check_interval_ops);
        self.last_check_bits = self.bit_bound;
        
        // Can't transition during cooldown
        if !self.thresholds.can_transition() {
            return Ok(());
        }
        
        // Check utilization
        let cap_bits = tier_info(self.current_tier).capacity_bits;
        let util = utilization_permille(self.bit_bound, cap_bits);
        
        let promote_thresh = self.thresholds.promote_threshold_permille();
        let demote_thresh = self.thresholds.demote_threshold_permille();
        
        // Promotion check
        if util >= promote_thresh {
            if let Some(higher_tier) = self.current_tier.promote() {
                if is_transition_worthwhile(
                    self.current_tier,
                    higher_tier,
                    self.horizon_ops,
                    self.mul_weight_q16,
                ) {
                    self.transition_to_tier(higher_tier)?;
                }
            }
        }
        // Demotion check
        else if util < demote_thresh {
            if let Some(lower_tier) = self.current_tier.demote() {
                if is_transition_worthwhile(
                    self.current_tier,
                    lower_tier,
                    self.horizon_ops,
                    self.mul_weight_q16,
                ) {
                    self.transition_to_tier(lower_tier)?;
                }
            }
        }
        
        Ok(())
    }

    /// Transition to new tier
    ///
    /// Reconstructs value and initializes new CRT representation.
    fn transition_to_tier(&mut self, new_tier: PrecisionTier) -> AdaptiveCRTResult<()> {
        if new_tier == self.current_tier {
            return Ok(());
        }
        
        // Reconstruct value from current CRT
        let value = reconstruct_from_crt(&self.residues, &self.moduli)
            .ok_or(AdaptiveCRTError::ReconstructionFailed)?;
        
        // Initialize new tier
        let new_moduli = generate_primes_for_tier(new_tier);
        let new_residues = value_to_residues_i128(value, &new_moduli);
        
        self.residues = new_residues;
        self.moduli = new_moduli;
        self.current_tier = new_tier;
        
        // Update telemetry
        self.tier_transitions += 1;
        self.thresholds.record_transition();
        
        Ok(())
    }

    /// Force specific tier (manual override)
    ///
    /// Bypasses automatic tier selection.
    pub fn force_tier(&mut self, tier: PrecisionTier) -> AdaptiveCRTResult<()> {
        if tier as usize >= TIER_TABLE.len() {
            return Err(AdaptiveCRTError::InvalidTier);
        }
        
        self.transition_to_tier(tier)
    }

    /// Addition with automatic tier management
    pub fn add(&mut self, other: &Self) -> AdaptiveCRTResult<Self> {
        // Perform CRT addition
        let mut result_residues = Vec::with_capacity(self.residues.len());
        for ((&r1, &r2), &m) in self.residues.iter()
            .zip(other.residues.iter())
            .zip(self.moduli.iter())
        {
            result_residues.push((r1 + r2) % m);
        }
        
        // Update bit bound (conservative: sum of bits + 1)
        let new_bit_bound = self.bit_bound.max(other.bit_bound) + 1;
        
        let mut result = self.clone();
        result.residues = result_residues;
        result.bit_bound = new_bit_bound;
        result.operations_count += 1;
        result.ops_since_check += 1;
        result.thresholds.increment_ops(1);
        
        // Periodic tier check
        if result.ops_since_check >= result.check_interval_ops {
            result.maybe_adjust_tier()?;
        }
        
        Ok(result)
    }

    /// Multiplication with automatic tier management
    pub fn mul(&mut self, other: &Self) -> AdaptiveCRTResult<Self> {
        // Perform CRT multiplication
        let mut result_residues = Vec::with_capacity(self.residues.len());
        for ((&r1, &r2), &m) in self.residues.iter()
            .zip(other.residues.iter())
            .zip(self.moduli.iter())
        {
            let prod = ((r1 as u128) * (r2 as u128)) % (m as u128);
            result_residues.push(prod as u64);
        }
        
        // Update bit bound (conservative: sum of bits)
        let new_bit_bound = self.bit_bound.saturating_add(other.bit_bound);
        
        let mut result = self.clone();
        result.residues = result_residues;
        result.bit_bound = new_bit_bound;
        result.operations_count += 1;
        result.ops_since_check += 1;
        result.thresholds.increment_ops(1);
        
        // Periodic tier check
        if result.ops_since_check >= result.check_interval_ops {
            result.maybe_adjust_tier()?;
        }
        
        Ok(result)
    }

    /// Get current tier
    pub fn tier(&self) -> PrecisionTier {
        self.current_tier
    }

    /// Get operation count
    pub fn operations(&self) -> u64 {
        self.operations_count
    }

    /// Get transition count
    pub fn transitions(&self) -> u64 {
        self.tier_transitions
    }

    /// Get current utilization (permille)
    pub fn utilization_permille(&self) -> u16 {
        let cap_bits = tier_info(self.current_tier).capacity_bits;
        utilization_permille(self.bit_bound, cap_bits)
    }
}

// ============================================================================
// SECTION 5: CRT PRIMITIVES (STUBS - IMPLEMENT WITH EXISTING CRT CODE)
// ============================================================================

/// Generate primes for tier (stub)
fn generate_primes_for_tier(tier: PrecisionTier) -> Vec<u64> {
    let info = tier_info(tier);
    // TODO: Use your existing prime generation
    // For now, return dummy primes
    let base_primes = vec![
        4_294_967_291u64,
        4_294_967_279,
        4_294_967_231,
        4_294_967_197,
        4_294_967_189,
        4_294_967_161,
        4_294_967_143,
        4_294_967_111,
    ];
    
    base_primes[..info.prime_count as usize].to_vec()
}

/// Convert value to CRT residues (stub)
fn value_to_residues(value: i64, moduli: &[u64]) -> Vec<u64> {
    let val = if value < 0 {
        // Handle negative: convert to positive modulo
        let abs_val = value.unsigned_abs();
        moduli.iter().map(|&m| {
            let r = (abs_val % m) as u64;
            if r == 0 { 0 } else { m - r }
        }).collect()
    } else {
        let val = value as u64;
        moduli.iter().map(|&m| val % m).collect()
    };
    val
}

/// Convert i128 value to CRT residues (stub)
fn value_to_residues_i128(value: i128, moduli: &[u64]) -> Vec<u64> {
    let val = if value < 0 {
        let abs_val = value.unsigned_abs();
        moduli.iter().map(|&m| {
            let r = (abs_val % m as u128) as u64;
            if r == 0 { 0 } else { m - r }
        }).collect()
    } else {
        let val = value as u128;
        moduli.iter().map(|&m| (val % m as u128) as u64).collect()
    };
    val
}

/// Reconstruct value from CRT (stub)
fn reconstruct_from_crt(residues: &[u64], moduli: &[u64]) -> Option<i128> {
    // TODO: Use your existing CRT reconstruction
    // This is placeholder - need actual CRT reconstruction
    Some(0)
}

// ============================================================================
// SECTION 6: TESTING FRAMEWORK
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilization_calculation() {
        assert_eq!(utilization_permille(30, 60), 500); // 50%
        assert_eq!(utilization_permille(54, 60), 900); // 90%
        assert_eq!(utilization_permille(24, 60), 400); // 40%
        assert_eq!(utilization_permille(60, 60), 1000); // 100%
    }

    #[test]
    fn test_tier_selection() {
        assert_eq!(select_tier_for_bits(10, 24), PrecisionTier::Tier0);
        assert_eq!(select_tier_for_bits(40, 24), PrecisionTier::Tier1);
        assert_eq!(select_tier_for_bits(100, 24), PrecisionTier::Tier2);
        assert_eq!(select_tier_for_bits(220, 24), PrecisionTier::Tier3);
    }

    #[test]
    fn test_adaptive_thresholds() {
        let mut thresholds = AdaptiveThresholds::new();
        
        // Initial state
        assert_eq!(thresholds.promote_threshold_permille(), 900);
        assert_eq!(thresholds.demote_threshold_permille(), 400);
        
        // Simulate high bit growth
        thresholds.update_ema(100, 64); // ~1.5 bits/op
        
        // Should adjust thresholds
        let promote = thresholds.promote_threshold_permille();
        let demote = thresholds.demote_threshold_permille();
        
        assert!(promote < 900); // Tighter promotion
        assert!(demote > 400);  // Looser demotion
        assert!(promote > demote); // Gap maintained
    }

    #[test]
    fn test_cooldown_enforcement() {
        let mut thresholds = AdaptiveThresholds::new();
        
        assert!(thresholds.can_transition());
        
        thresholds.record_transition();
        assert!(!thresholds.can_transition());
        
        thresholds.increment_ops(2048);
        assert!(thresholds.can_transition());
    }

    #[test]
    fn test_basic_arithmetic() {
        let mut x = AdaptiveCRTBigInt::new(42);
        let y = AdaptiveCRTBigInt::new(17);
        
        // Addition
        let result = x.add(&y).unwrap();
        assert_eq!(result.tier(), PrecisionTier::Tier0);
        
        // Multiplication
        let result = x.mul(&y).unwrap();
        assert_eq!(result.tier(), PrecisionTier::Tier0);
    }

    // TODO: Property tests
    // TODO: Adversarial hovering tests
    // TODO: Monotone growth tests
    // TODO: Microbenchmarks
}
