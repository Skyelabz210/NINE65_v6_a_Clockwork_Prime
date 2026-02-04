//! T-503: QMNF/EPRAM Regression Test Suite
//! 
//! Guards against stdlib regressions and ensures QMNF innovations
//! remain properly wired throughout the codebase.
//!
//! Scans for:
//! - Forbidden patterns (floats, stdlib BigInt, etc.)
//! - Required patterns (QMNF innovations)
//! - Reversion bugs (old patterns replacing validated solutions)

use std::collections::HashSet;
use std::path::Path;

// =============================================================================
// PATTERN DEFINITIONS
// =============================================================================

/// Patterns that indicate stdlib regression (must NOT appear)
pub const FORBIDDEN_PATTERNS: &[ForbiddenPattern] = &[
    // Floating point types
    ForbiddenPattern {
        pattern: "f64",
        context: "floating_point",
        severity: Severity::Critical,
        exception: Some("// QMNF-ALLOW: f64"),
        reason: "Floating point introduces drift; use integer arithmetic",
    },
    ForbiddenPattern {
        pattern: "f32",
        context: "floating_point",
        severity: Severity::Critical,
        exception: Some("// QMNF-ALLOW: f32"),
        reason: "Floating point introduces drift; use integer arithmetic",
    },
    ForbiddenPattern {
        pattern: "as f64",
        context: "floating_point_cast",
        severity: Severity::Critical,
        exception: Some("// QMNF-ALLOW: cast"),
        reason: "Float casts lose exactness",
    },
    ForbiddenPattern {
        pattern: "as f32",
        context: "floating_point_cast",
        severity: Severity::Critical,
        exception: Some("// QMNF-ALLOW: cast"),
        reason: "Float casts lose exactness",
    },
    
    // Float operations
    ForbiddenPattern {
        pattern: ".exp()",
        context: "transcendental",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: exp"),
        reason: "Use Padé approximant or cyclotomic phase instead",
    },
    ForbiddenPattern {
        pattern: ".ln()",
        context: "transcendental",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: ln"),
        reason: "Use integer logarithm approximation",
    },
    ForbiddenPattern {
        pattern: ".sin()",
        context: "transcendental",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: sin"),
        reason: "Use cyclotomic phase for native trig",
    },
    ForbiddenPattern {
        pattern: ".cos()",
        context: "transcendental",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: cos"),
        reason: "Use cyclotomic phase for native trig",
    },
    ForbiddenPattern {
        pattern: ".sqrt()",
        context: "transcendental",
        severity: Severity::Medium,
        exception: Some("// QMNF-ALLOW: sqrt"),
        reason: "Use integer_sqrt for exact floor",
    },
    
    // Standard BigInt
    ForbiddenPattern {
        pattern: "num::BigInt",
        context: "stdlib_bigint",
        severity: Severity::Critical,
        exception: None,
        reason: "Use CRTBigInt for parallel exact arithmetic",
    },
    ForbiddenPattern {
        pattern: "num::BigUint",
        context: "stdlib_bigint",
        severity: Severity::Critical,
        exception: None,
        reason: "Use CRTBigInt for parallel exact arithmetic",
    },
    ForbiddenPattern {
        pattern: "bigint::",
        context: "stdlib_bigint",
        severity: Severity::Critical,
        exception: None,
        reason: "Use CRTBigInt for parallel exact arithmetic",
    },
    
    // Montgomery conversion (should stay in form)
    ForbiddenPattern {
        pattern: "to_standard()",
        context: "montgomery_conversion",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: conversion"),
        reason: "Persistent Montgomery: avoid conversion in computation chains",
    },
    ForbiddenPattern {
        pattern: "from_montgomery(",
        context: "montgomery_conversion",
        severity: Severity::High,
        exception: Some("// QMNF-ALLOW: conversion"),
        reason: "Persistent Montgomery: avoid conversion in computation chains",
    },
    
    // Decode/encode viewpoint
    ForbiddenPattern {
        pattern: "decode(",
        context: "residue_viewpoint",
        severity: Severity::Medium,
        exception: Some("// QMNF-ALLOW: decode"),
        reason: "Residues ARE the values, not encodings",
    },
    ForbiddenPattern {
        pattern: "encode(",
        context: "residue_viewpoint",
        severity: Severity::Medium,
        exception: Some("// QMNF-ALLOW: encode"),
        reason: "Residues ARE the values, not encodings",
    },
    
    // Stdlib random (use Shadow Entropy)
    ForbiddenPattern {
        pattern: "rand::thread_rng",
        context: "random",
        severity: Severity::Medium,
        exception: Some("// QMNF-ALLOW: rand"),
        reason: "Use Shadow Entropy for deterministic noise",
    },
    ForbiddenPattern {
        pattern: "OsRng",
        context: "random",
        severity: Severity::Medium,
        exception: Some("// QMNF-ALLOW: OsRng"),
        reason: "Use Shadow Entropy for reproducible results",
    },
];

/// Patterns that indicate QMNF innovations (must appear somewhere)
pub const REQUIRED_PATTERNS: &[RequiredPattern] = &[
    // EPRAM abstractions
    RequiredPattern {
        pattern: "impl EPRAMCell",
        context: "epram",
        min_occurrences: 4,
        description: "EPRAM cell implementations for permanent residents",
    },
    RequiredPattern {
        pattern: "EPRAMField",
        context: "epram",
        min_occurrences: 1,
        description: "EPRAM field type",
    },
    RequiredPattern {
        pattern: "fourth_attractor",
        context: "epram",
        min_occurrences: 1,
        description: "Dithered Fourth Attractor convergence",
    },
    
    // K-Elimination
    RequiredPattern {
        pattern: "recover_k",
        context: "k_elimination",
        min_occurrences: 1,
        description: "K-Elimination exact division",
    },
    RequiredPattern {
        pattern: "phase_differential",
        context: "k_elimination",
        min_occurrences: 0,  // Optional but preferred
        description: "Phase differential for overflow tracking",
    },
    
    // Signed arithmetic
    RequiredPattern {
        pattern: "MobiusInt",
        context: "signed",
        min_occurrences: 0,  // Optional
        description: "MobiusInt signed arithmetic",
    },
    
    // CRT
    RequiredPattern {
        pattern: "CRT",
        context: "crt",
        min_occurrences: 1,
        description: "Chinese Remainder Theorem usage",
    },
    
    // Montgomery
    RequiredPattern {
        pattern: "Montgomery",
        context: "montgomery",
        min_occurrences: 1,
        description: "Persistent Montgomery arithmetic",
    },
    
    // Shadow Entropy
    RequiredPattern {
        pattern: "Shadow",
        context: "entropy",
        min_occurrences: 1,
        description: "Shadow Entropy noise generation",
    },
    
    // Cyclotomic
    RequiredPattern {
        pattern: "Cyclotomic",
        context: "cyclotomic",
        min_occurrences: 1,
        description: "Cyclotomic ring operations",
    },
];

// =============================================================================
// PATTERN STRUCTURES
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,  // Must fix immediately
    High,      // Should fix soon
    Medium,    // Should fix eventually
    Low,       // Informational
}

#[derive(Debug, Clone)]
pub struct ForbiddenPattern {
    pub pattern: &'static str,
    pub context: &'static str,
    pub severity: Severity,
    pub exception: Option<&'static str>,
    pub reason: &'static str,
}

#[derive(Debug, Clone)]
pub struct RequiredPattern {
    pub pattern: &'static str,
    pub context: &'static str,
    pub min_occurrences: usize,
    pub description: &'static str,
}

// =============================================================================
// SCAN RESULTS
// =============================================================================

#[derive(Debug, Clone)]
pub struct ForbiddenMatch {
    pub pattern: String,
    pub file: String,
    pub line: usize,
    pub context: String,
    pub severity: Severity,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct RequiredMissing {
    pub pattern: String,
    pub context: String,
    pub found: usize,
    pub required: usize,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub files_scanned: usize,
    pub forbidden_matches: Vec<ForbiddenMatch>,
    pub required_missing: Vec<RequiredMissing>,
    pub passed: bool,
}

impl ScanResult {
    pub fn report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push_str("  QMNF REGRESSION SCAN REPORT\n");
        report.push_str("═══════════════════════════════════════════════════════════════\n\n");
        
        report.push_str(&format!("Files scanned: {}\n\n", self.files_scanned));
        
        // Forbidden patterns found
        if self.forbidden_matches.is_empty() {
            report.push_str("✓ No forbidden patterns found\n\n");
        } else {
            report.push_str(&format!("✗ {} forbidden patterns found:\n\n", self.forbidden_matches.len()));
            
            for m in &self.forbidden_matches {
                let severity_str = match m.severity {
                    Severity::Critical => "CRITICAL",
                    Severity::High => "HIGH",
                    Severity::Medium => "MEDIUM",
                    Severity::Low => "LOW",
                };
                
                report.push_str(&format!(
                    "  [{:8}] {} (line {})\n    Pattern: '{}'\n    Reason: {}\n\n",
                    severity_str, m.file, m.line, m.pattern, m.reason
                ));
            }
        }
        
        // Required patterns missing
        if self.required_missing.is_empty() {
            report.push_str("✓ All required patterns present\n\n");
        } else {
            report.push_str(&format!("✗ {} required patterns missing:\n\n", self.required_missing.len()));
            
            for m in &self.required_missing {
                report.push_str(&format!(
                    "  '{}' ({}/{}): {}\n",
                    m.pattern, m.found, m.required, m.description
                ));
            }
            report.push_str("\n");
        }
        
        // Summary
        report.push_str("───────────────────────────────────────────────────────────────\n");
        if self.passed {
            report.push_str("  STATUS: ✓ PASSED - No regressions detected\n");
        } else {
            let critical = self.forbidden_matches.iter()
                .filter(|m| m.severity == Severity::Critical)
                .count();
            if critical > 0 {
                report.push_str(&format!("  STATUS: ✗ FAILED - {} critical issues\n", critical));
            } else {
                report.push_str("  STATUS: ✗ FAILED - Issues detected (no critical)\n");
            }
        }
        report.push_str("═══════════════════════════════════════════════════════════════\n");
        
        report
    }
}

// =============================================================================
// SCANNER
// =============================================================================

pub struct RegressionScanner {
    forbidden: Vec<ForbiddenPattern>,
    required: Vec<RequiredPattern>,
}

impl RegressionScanner {
    pub fn new() -> Self {
        Self {
            forbidden: FORBIDDEN_PATTERNS.to_vec(),
            required: REQUIRED_PATTERNS.to_vec(),
        }
    }
    
    pub fn with_custom_patterns(
        forbidden: Vec<ForbiddenPattern>,
        required: Vec<RequiredPattern>,
    ) -> Self {
        Self { forbidden, required }
    }
    
    /// Scan a single source string
    pub fn scan_source(&self, filename: &str, source: &str) -> (Vec<ForbiddenMatch>, usize) {
        let mut matches = Vec::new();
        let mut required_counts: Vec<usize> = vec![0; self.required.len()];
        
        for (line_num, line) in source.lines().enumerate() {
            // Check forbidden patterns
            for fp in &self.forbidden {
                if line.contains(fp.pattern) {
                    // Check for exception comment
                    if let Some(exc) = fp.exception {
                        if line.contains(exc) {
                            continue;
                        }
                    }
                    
                    matches.push(ForbiddenMatch {
                        pattern: fp.pattern.to_string(),
                        file: filename.to_string(),
                        line: line_num + 1,
                        context: fp.context.to_string(),
                        severity: fp.severity,
                        reason: fp.reason.to_string(),
                    });
                }
            }
            
            // Count required patterns
            for (i, rp) in self.required.iter().enumerate() {
                if line.contains(rp.pattern) {
                    required_counts[i] += 1;
                }
            }
        }
        
        // Sum up total required pattern occurrences
        let total_required: usize = required_counts.iter().sum();
        
        (matches, total_required)
    }
    
    /// Scan multiple sources and produce full report
    pub fn scan_all(&self, sources: &[(&str, &str)]) -> ScanResult {
        let mut all_forbidden = Vec::new();
        let mut required_counts: Vec<usize> = vec![0; self.required.len()];
        
        for (filename, source) in sources {
            // Check forbidden
            let (forbidden_matches, _) = self.scan_source(filename, source);
            all_forbidden.extend(forbidden_matches);
            
            // Count required
            for (i, rp) in self.required.iter().enumerate() {
                for line in source.lines() {
                    if line.contains(rp.pattern) {
                        required_counts[i] += 1;
                    }
                }
            }
        }
        
        // Check required pattern minimums
        let mut required_missing = Vec::new();
        for (i, rp) in self.required.iter().enumerate() {
            if required_counts[i] < rp.min_occurrences {
                required_missing.push(RequiredMissing {
                    pattern: rp.pattern.to_string(),
                    context: rp.context.to_string(),
                    found: required_counts[i],
                    required: rp.min_occurrences,
                    description: rp.description.to_string(),
                });
            }
        }
        
        let has_critical = all_forbidden.iter()
            .any(|m| m.severity == Severity::Critical);
        
        let passed = all_forbidden.is_empty() && required_missing.is_empty();
        
        ScanResult {
            files_scanned: sources.len(),
            forbidden_matches: all_forbidden,
            required_missing,
            passed,
        }
    }
}

impl Default for RegressionScanner {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// REVERSION BUG DETECTION
// =============================================================================

/// Detects when validated QMNF patterns have been replaced with old stdlib patterns
pub struct ReversionDetector {
    /// Map of QMNF innovation -> stdlib pattern it replaces
    replacements: Vec<(&'static str, &'static str)>,
}

impl ReversionDetector {
    pub fn new() -> Self {
        Self {
            replacements: vec![
                ("recover_k", "/ divisor"),              // K-Elimination replaces division
                ("fourth_attractor", "while !converged"), // Attractor replaces loops
                ("Montgomery", "% modulus"),             // Montgomery replaces naive mod
                ("Shadow", "rand::"),                    // Shadow replaces rand
                ("Cyclotomic", ".sin("),                 // Cyclotomic replaces trig
                ("CRTBigInt", "BigInt"),                 // CRT replaces BigInt
            ],
        }
    }
    
    /// Check if source shows signs of reversion
    pub fn check_reversion(&self, source: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        
        for (qmnf, stdlib) in &self.replacements {
            let has_qmnf = source.contains(qmnf);
            let has_stdlib = source.contains(stdlib);
            
            // Warning: has stdlib pattern but not QMNF replacement
            if has_stdlib && !has_qmnf {
                warnings.push(format!(
                    "Possible reversion: found '{}' without '{}' - consider using QMNF innovation",
                    stdlib, qmnf
                ));
            }
        }
        
        warnings
    }
}

impl Default for ReversionDetector {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_forbidden_pattern_detection() {
        let scanner = RegressionScanner::new();
        
        let source = r#"
            let x: f64 = 3.14;
            let y = x.sin();
        "#;
        
        let (matches, _) = scanner.scan_source("test.rs", source);
        
        assert!(!matches.is_empty());
        assert!(matches.iter().any(|m| m.pattern == "f64"));
        assert!(matches.iter().any(|m| m.pattern == ".sin()"));
    }
    
    #[test]
    fn test_exception_comment() {
        let scanner = RegressionScanner::new();
        
        let source = r#"
            let x: f64 = 3.14;  // QMNF-ALLOW: f64
            let y: f64 = 2.0;   // No exception
        "#;
        
        let (matches, _) = scanner.scan_source("test.rs", source);
        
        // Should only find the second f64
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, 3);
    }
    
    #[test]
    fn test_required_pattern_counting() {
        let scanner = RegressionScanner::new();
        
        let source = r#"
            impl EPRAMCell for MontgomeryCell {}
            impl EPRAMCell for DualCodexCell {}
            struct CRTBigInt {}
            fn recover_k() {}
        "#;
        
        let sources = vec![("test.rs", source)];
        let result = scanner.scan_all(&sources);
        
        // Should find required patterns
        assert!(result.required_missing.iter()
            .all(|m| m.pattern != "impl EPRAMCell"));
    }
    
    #[test]
    fn test_clean_source() {
        let scanner = RegressionScanner::new();
        
        let source = r#"
            impl EPRAMCell for MontgomeryCell {
                fn transition(&self) -> Self {
                    fourth_attractor_step(self.value, target, m)
                }
            }
            
            fn recover_k(alpha: u64, beta: u64) -> u64 {
                // K-Elimination
            }
            
            struct CRTBigInt {}
            struct ShadowState {}
            struct CyclotomicElement {}
        "#;
        
        let (matches, _) = scanner.scan_source("clean.rs", source);
        
        assert!(matches.is_empty(), "Clean QMNF source should have no forbidden patterns");
    }
    
    #[test]
    fn test_reversion_detection() {
        let detector = ReversionDetector::new();
        
        let reverted_source = r#"
            let result = value / divisor;  // Using division instead of K-Elimination
            let random = rand::thread_rng();
        "#;
        
        let warnings = detector.check_reversion(reverted_source);
        
        assert!(!warnings.is_empty());
    }
    
    #[test]
    fn test_full_scan_result() {
        let scanner = RegressionScanner::new();
        
        let sources = vec![
            ("good.rs", "impl EPRAMCell for Test {}"),
            ("bad.rs", "let x: f64 = 0.0;"),
        ];
        
        let result = scanner.scan_all(&sources);
        
        assert_eq!(result.files_scanned, 2);
        assert!(!result.passed);
        assert!(!result.forbidden_matches.is_empty());
    }
}
