//! Audit Logging for Division Operations
//!
//! Provides HMAC-signed audit trails for all division operations.
//! Enables forensic analysis and compliance verification.
//!
//! Features:
//! - Cryptographic integrity via HMAC-SHA256
//! - Tamper detection
//! - Path tracking (which algorithm was used)
//! - Timestamp recording

use crate::mod_residue::{DivStatus, ModResidue};
use hmac::{Hmac, Mac};
use num_bigint::BigInt;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Audit log entry for a division operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionAuditEntry {
    /// Timestamp (Unix epoch nanoseconds)
    pub timestamp: u64,

    /// SHA256 hash of dividend
    pub dividend_hash: [u8; 32],

    /// SHA256 hash of divisor
    pub divisor_hash: [u8; 32],

    /// SHA256 hash of modulus
    pub modulus_hash: [u8; 32],

    /// SHA256 hash of result residue
    pub result_hash: [u8; 32],

    /// Division path taken
    pub path_taken: String,

    /// Result status
    pub status: String,

    /// Whether result is exact
    pub is_exact: bool,

    /// HMAC signature of entry
    pub hmac: [u8; 32],
}

impl DivisionAuditEntry {
    /// Create new audit entry
    pub fn new(
        dividend: &BigInt,
        divisor: &BigInt,
        modulus: &BigInt,
        result: &ModResidue,
        path: &str,
        key: &[u8; 32],
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);

        let mut entry = Self {
            timestamp,
            dividend_hash: hash_bigint(dividend),
            divisor_hash: hash_bigint(divisor),
            modulus_hash: hash_bigint(modulus),
            result_hash: hash_bigint(&result.residue),
            path_taken: path.to_string(),
            status: format!("{}", result.status),
            is_exact: result.is_exact(),
            hmac: [0; 32],
        };

        entry.hmac = entry.compute_hmac(key);
        entry
    }

    /// Create entry for failed division
    pub fn new_failed(
        dividend: &BigInt,
        divisor: &BigInt,
        modulus: &BigInt,
        error: &str,
        key: &[u8; 32],
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);

        let mut entry = Self {
            timestamp,
            dividend_hash: hash_bigint(dividend),
            divisor_hash: hash_bigint(divisor),
            modulus_hash: hash_bigint(modulus),
            result_hash: [0; 32],
            path_taken: "failed".to_string(),
            status: error.to_string(),
            is_exact: false,
            hmac: [0; 32],
        };

        entry.hmac = entry.compute_hmac(key);
        entry
    }

    /// Compute HMAC of entry contents
    fn compute_hmac(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key size is valid");

        mac.update(&self.timestamp.to_le_bytes());
        mac.update(&self.dividend_hash);
        mac.update(&self.divisor_hash);
        mac.update(&self.modulus_hash);
        mac.update(&self.result_hash);
        mac.update(self.path_taken.as_bytes());
        mac.update(self.status.as_bytes());
        mac.update(&[self.is_exact as u8]);

        let result = mac.finalize();
        let mut hmac = [0u8; 32];
        hmac.copy_from_slice(&result.into_bytes());
        hmac
    }

    /// Verify HMAC signature
    pub fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = self.compute_hmac(key);
        constant_time_eq(&self.hmac, &expected)
    }

    /// Get entry as JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Parse entry from JSON
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

/// Hash a BigInt using SHA256
fn hash_bigint(value: &BigInt) -> [u8; 32] {
    let (_, bytes) = value.to_bytes_be();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Constant-time equality comparison
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }

    result == 0
}

/// Audit log for tracking division operations
#[derive(Debug)]
pub struct AuditLog {
    /// Entries in the log
    entries: Vec<DivisionAuditEntry>,

    /// HMAC key for signing
    key: [u8; 32],

    /// Maximum entries to keep
    max_entries: usize,
}

impl AuditLog {
    /// Create new audit log with given key
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            entries: Vec::new(),
            key,
            max_entries: 10000,
        }
    }

    /// Create with custom max entries
    pub fn with_max_entries(key: [u8; 32], max_entries: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_entries.min(1000)),
            key,
            max_entries,
        }
    }

    /// Log a successful division
    pub fn log_success(
        &mut self,
        dividend: &BigInt,
        divisor: &BigInt,
        modulus: &BigInt,
        result: &ModResidue,
        path: &str,
    ) {
        let entry = DivisionAuditEntry::new(dividend, divisor, modulus, result, path, &self.key);
        self.add_entry(entry);
    }

    /// Log a failed division
    pub fn log_failure(
        &mut self,
        dividend: &BigInt,
        divisor: &BigInt,
        modulus: &BigInt,
        error: &str,
    ) {
        let entry = DivisionAuditEntry::new_failed(dividend, divisor, modulus, error, &self.key);
        self.add_entry(entry);
    }

    /// Add entry to log
    fn add_entry(&mut self, entry: DivisionAuditEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get entry by index
    pub fn get(&self, index: usize) -> Option<&DivisionAuditEntry> {
        self.entries.get(index)
    }

    /// Get all entries
    pub fn entries(&self) -> &[DivisionAuditEntry] {
        &self.entries
    }

    /// Verify all entries
    pub fn verify_all(&self) -> bool {
        self.entries.iter().all(|e| e.verify(&self.key))
    }

    /// Find tampered entries
    pub fn find_tampered(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.verify(&self.key))
            .map(|(i, _)| i)
            .collect()
    }

    /// Get statistics
    pub fn statistics(&self) -> AuditStatistics {
        let mut stats = AuditStatistics::default();

        for entry in &self.entries {
            match entry.path_taken.as_str() {
                "fast_path" => stats.fast_path_count += 1,
                "gcd_reduction" => stats.gcd_reduction_count += 1,
                "piggyback" => stats.piggyback_count += 1,
                "failed" => stats.failure_count += 1,
                _ => stats.other_count += 1,
            }

            if entry.is_exact {
                stats.exact_count += 1;
            }
        }

        stats.total_count = self.entries.len();
        stats
    }

    /// Export log as JSON
    pub fn export_json(&self) -> String {
        let json_entries: Vec<String> = self.entries.iter().map(|e| e.to_json()).collect();
        format!("[{}]", json_entries.join(","))
    }

    /// Clear the log
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Statistics from audit log
#[derive(Debug, Default, Clone)]
pub struct AuditStatistics {
    pub total_count: usize,
    pub fast_path_count: usize,
    pub gcd_reduction_count: usize,
    pub piggyback_count: usize,
    pub failure_count: usize,
    pub exact_count: usize,
    pub other_count: usize,
}

impl AuditStatistics {
    /// Get fast path percentage
    pub fn fast_path_percentage(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            self.fast_path_count as f64 / self.total_count as f64 * 100.0
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.total_count - self.failure_count) as f64 / self.total_count as f64 * 100.0
        }
    }
}

/// Audit-enabled division
pub mod audited {
    use super::*;
    use crate::error::DivisionResult;
    use crate::{mod_div, DivisionConfig};

    /// Perform division with audit logging
    pub fn mod_div_audited(
        dividend: &BigInt,
        divisor: &BigInt,
        modulus: &BigInt,
        config: &DivisionConfig,
        log: &mut AuditLog,
    ) -> DivisionResult<ModResidue> {
        // Try fast path
        if let Ok(result) = crate::fast_path::mod_div_fast(dividend, divisor, modulus) {
            log.log_success(dividend, divisor, modulus, &result, "fast_path");
            return Ok(result);
        }

        // Try GCD reduction
        if let Ok(result) = crate::gcd_reduction::mod_div_gcd_reduction(dividend, divisor, modulus) {
            log.log_success(dividend, divisor, modulus, &result, "gcd_reduction");
            return Ok(result);
        }

        // Try piggyback
        match crate::piggyback::mod_div_piggyback(dividend, divisor, modulus, &config.anchors) {
            Ok(result) => {
                log.log_success(dividend, divisor, modulus, &result, "piggyback");
                Ok(result)
            }
            Err(e) => {
                log.log_failure(dividend, divisor, modulus, &e.to_string());
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [0u8; 32]
    }

    #[test]
    fn test_audit_entry_creation() {
        let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));

        let entry = DivisionAuditEntry::new(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(97),
            &result,
            "fast_path",
            &test_key(),
        );

        assert!(entry.verify(&test_key()));
        assert!(entry.is_exact);
        assert_eq!(entry.path_taken, "fast_path");
    }

    #[test]
    fn test_audit_entry_tamper_detection() {
        let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));

        let mut entry = DivisionAuditEntry::new(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(97),
            &result,
            "fast_path",
            &test_key(),
        );

        // Tamper with path
        entry.path_taken = "piggyback".to_string();

        assert!(!entry.verify(&test_key()));
    }

    #[test]
    fn test_audit_entry_json() {
        let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));

        let entry = DivisionAuditEntry::new(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(97),
            &result,
            "fast_path",
            &test_key(),
        );

        let json = entry.to_json();
        let parsed = DivisionAuditEntry::from_json(&json).unwrap();

        assert_eq!(parsed.path_taken, entry.path_taken);
        assert!(parsed.verify(&test_key()));
    }

    #[test]
    fn test_audit_log_basic() {
        let mut log = AuditLog::new(test_key());

        let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));
        log.log_success(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(97),
            &result,
            "fast_path",
        );

        assert_eq!(log.len(), 1);
        assert!(log.verify_all());
    }

    #[test]
    fn test_audit_log_max_entries() {
        let mut log = AuditLog::with_max_entries(test_key(), 5);

        for i in 0..10 {
            let result = ModResidue::exact(BigInt::from(i), BigInt::from(97));
            log.log_success(
                &BigInt::from(i),
                &BigInt::from(1),
                &BigInt::from(97),
                &result,
                "fast_path",
            );
        }

        assert_eq!(log.len(), 5);
    }

    #[test]
    fn test_audit_log_statistics() {
        let mut log = AuditLog::new(test_key());

        // Fast path entries
        for _ in 0..5 {
            let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));
            log.log_success(
                &BigInt::from(10),
                &BigInt::from(3),
                &BigInt::from(97),
                &result,
                "fast_path",
            );
        }

        // Piggyback entries
        for _ in 0..3 {
            let result = ModResidue::promoted(
                BigInt::from(42),
                BigInt::from(15),
                BigInt::from(97),
                0,
            );
            log.log_success(
                &BigInt::from(7),
                &BigInt::from(9),
                &BigInt::from(15),
                &result,
                "piggyback",
            );
        }

        // Failure
        log.log_failure(
            &BigInt::from(10),
            &BigInt::from(0),
            &BigInt::from(97),
            "division by zero",
        );

        let stats = log.statistics();
        assert_eq!(stats.total_count, 9);
        assert_eq!(stats.fast_path_count, 5);
        assert_eq!(stats.piggyback_count, 3);
        assert_eq!(stats.failure_count, 1);
        assert_eq!(stats.exact_count, 5);
    }

    #[test]
    fn test_audit_log_find_tampered() {
        let mut log = AuditLog::new(test_key());

        let result = ModResidue::exact(BigInt::from(42), BigInt::from(97));
        log.log_success(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(97),
            &result,
            "fast_path",
        );

        // Tamper with first entry
        log.entries[0].path_taken = "tampered".to_string();

        let tampered = log.find_tampered();
        assert_eq!(tampered, vec![0]);
    }

    #[test]
    fn test_audited_division() {
        use audited::mod_div_audited;
        use crate::DivisionConfig;

        let config = DivisionConfig::default();
        let mut log = AuditLog::new(test_key());

        let result = mod_div_audited(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(7),
            &config,
            &mut log,
        )
        .unwrap();

        assert!(result.is_exact());
        assert_eq!(log.len(), 1);
        assert!(log.verify_all());
    }
}
