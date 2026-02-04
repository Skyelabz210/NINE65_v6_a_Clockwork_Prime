//! T-501: QMNF/EPRAM Error Handling Framework
//! 
//! Comprehensive error types for all system components.
//! Designed for production use with:
//! - No panics in production paths
//! - Informative error messages
//! - Error recovery suggestions
//! - Tracing integration ready

use std::fmt;
use std::error::Error;

// =============================================================================
// ERROR HIERARCHY
// =============================================================================

/// Top-level QMNF error type
#[derive(Debug, Clone)]
pub enum QMNFError {
    /// Arithmetic operation error
    Arithmetic(ArithmeticError),
    /// EPRAM field evolution error
    EPRAM(EPRAMError),
    /// Orchestrator decision error
    Orchestrator(OrchestratorError),
    /// Rational reconstruction error
    Rational(RationalError),
    /// Configuration error
    Config(ConfigError),
    /// Resource exhaustion
    Resource(ResourceError),
}

impl fmt::Display for QMNFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QMNFError::Arithmetic(e) => write!(f, "Arithmetic error: {}", e),
            QMNFError::EPRAM(e) => write!(f, "EPRAM error: {}", e),
            QMNFError::Orchestrator(e) => write!(f, "Orchestrator error: {}", e),
            QMNFError::Rational(e) => write!(f, "Rational error: {}", e),
            QMNFError::Config(e) => write!(f, "Configuration error: {}", e),
            QMNFError::Resource(e) => write!(f, "Resource error: {}", e),
        }
    }
}

impl Error for QMNFError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QMNFError::Arithmetic(e) => Some(e),
            QMNFError::EPRAM(e) => Some(e),
            QMNFError::Orchestrator(e) => Some(e),
            QMNFError::Rational(e) => Some(e),
            QMNFError::Config(e) => Some(e),
            QMNFError::Resource(e) => Some(e),
        }
    }
}

// =============================================================================
// ARITHMETIC ERRORS
// =============================================================================

/// Errors in arithmetic operations
#[derive(Debug, Clone)]
pub enum ArithmeticError {
    /// Division by zero
    DivisionByZero {
        operation: &'static str,
        context: String,
    },
    /// Modular inverse does not exist
    NoInverse {
        value: u64,
        modulus: u64,
        reason: String,
    },
    /// Overflow in computation
    Overflow {
        operation: &'static str,
        operands: String,
        limit: String,
    },
    /// Underflow in computation
    Underflow {
        operation: &'static str,
        context: String,
    },
    /// Invalid modulus
    InvalidModulus {
        value: u64,
        requirement: &'static str,
    },
    /// CRT moduli not coprime
    NotCoprime {
        m1: u64,
        m2: u64,
        gcd: u64,
    },
    /// K-Elimination failed
    KEliminationFailed {
        alpha: u64,
        beta: u64,
        reason: String,
    },
    /// Montgomery form error
    MontgomeryError {
        operation: &'static str,
        details: String,
    },
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticError::DivisionByZero { operation, context } => {
                write!(f, "Division by zero in {}: {}", operation, context)
            }
            ArithmeticError::NoInverse { value, modulus, reason } => {
                write!(f, "No modular inverse: {} mod {} ({})", value, modulus, reason)
            }
            ArithmeticError::Overflow { operation, operands, limit } => {
                write!(f, "Overflow in {}: {} exceeds {}", operation, operands, limit)
            }
            ArithmeticError::Underflow { operation, context } => {
                write!(f, "Underflow in {}: {}", operation, context)
            }
            ArithmeticError::InvalidModulus { value, requirement } => {
                write!(f, "Invalid modulus {}: must be {}", value, requirement)
            }
            ArithmeticError::NotCoprime { m1, m2, gcd } => {
                write!(f, "Moduli {} and {} not coprime (gcd={})", m1, m2, gcd)
            }
            ArithmeticError::KEliminationFailed { alpha, beta, reason } => {
                write!(f, "K-Elimination failed for α={}, β={}: {}", alpha, beta, reason)
            }
            ArithmeticError::MontgomeryError { operation, details } => {
                write!(f, "Montgomery {} error: {}", operation, details)
            }
        }
    }
}

impl Error for ArithmeticError {}

// =============================================================================
// EPRAM ERRORS
// =============================================================================

/// Errors in EPRAM field operations
#[derive(Debug, Clone)]
pub enum EPRAMError {
    /// Field did not converge
    ConvergenceFailure {
        steps: usize,
        max_steps: usize,
        final_lyapunov: u64,
        threshold: u64,
    },
    /// Invalid topology
    InvalidTopology {
        topology: String,
        n_cells: usize,
        reason: String,
    },
    /// Cell count mismatch
    CellCountMismatch {
        expected: usize,
        actual: usize,
        context: &'static str,
    },
    /// Invalid cell state
    InvalidCellState {
        cell_index: usize,
        value: u64,
        modulus: u64,
    },
    /// Lyapunov violation (should never happen with dithered attractor)
    LyapunovViolation {
        step: usize,
        before: u64,
        after: u64,
    },
    /// Coupling mode error
    CouplingError {
        mode: String,
        reason: String,
    },
}

impl fmt::Display for EPRAMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EPRAMError::ConvergenceFailure { steps, max_steps, final_lyapunov, threshold } => {
                write!(f, "Convergence failed after {}/{} steps (L={}, threshold={})", 
                    steps, max_steps, final_lyapunov, threshold)
            }
            EPRAMError::InvalidTopology { topology, n_cells, reason } => {
                write!(f, "Invalid topology '{}' for {} cells: {}", topology, n_cells, reason)
            }
            EPRAMError::CellCountMismatch { expected, actual, context } => {
                write!(f, "Cell count mismatch in {}: expected {}, got {}", context, expected, actual)
            }
            EPRAMError::InvalidCellState { cell_index, value, modulus } => {
                write!(f, "Invalid cell state at {}: {} >= modulus {}", cell_index, value, modulus)
            }
            EPRAMError::LyapunovViolation { step, before, after } => {
                write!(f, "Lyapunov violation at step {}: {} -> {} (should decrease)", step, before, after)
            }
            EPRAMError::CouplingError { mode, reason } => {
                write!(f, "Coupling mode '{}' error: {}", mode, reason)
            }
        }
    }
}

impl Error for EPRAMError {}

// =============================================================================
// ORCHESTRATOR ERRORS
// =============================================================================

/// Errors in orchestrator operations
#[derive(Debug, Clone)]
pub enum OrchestratorError {
    /// No templates registered
    NoTemplates,
    /// Template not found
    TemplateNotFound {
        id: usize,
    },
    /// Decision ambiguous (multiple equally close templates)
    AmbiguousDecision {
        template_ids: Vec<usize>,
        distance: u64,
    },
    /// Decision confidence too low
    LowConfidence {
        template_id: usize,
        confidence: f64,
        threshold: f64,
    },
    /// Rail not found
    RailNotFound {
        id: usize,
    },
    /// FRST update failed
    FRSTError {
        rail_id: usize,
        reason: String,
    },
    /// PLMG validation failed
    PLMGValidationFailed {
        reason: String,
    },
    /// Template pattern size mismatch
    PatternSizeMismatch {
        template_id: usize,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for OrchestratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrchestratorError::NoTemplates => {
                write!(f, "No templates registered in orchestrator")
            }
            OrchestratorError::TemplateNotFound { id } => {
                write!(f, "Template {} not found", id)
            }
            OrchestratorError::AmbiguousDecision { template_ids, distance } => {
                write!(f, "Ambiguous decision: templates {:?} all at distance {}", template_ids, distance)
            }
            OrchestratorError::LowConfidence { template_id, confidence, threshold } => {
                write!(f, "Low confidence for template {}: {:.2} < {:.2}", template_id, confidence, threshold)
            }
            OrchestratorError::RailNotFound { id } => {
                write!(f, "Rail {} not found", id)
            }
            OrchestratorError::FRSTError { rail_id, reason } => {
                write!(f, "FRST update failed for rail {}: {}", rail_id, reason)
            }
            OrchestratorError::PLMGValidationFailed { reason } => {
                write!(f, "PLMG validation failed: {}", reason)
            }
            OrchestratorError::PatternSizeMismatch { template_id, expected, actual } => {
                write!(f, "Template {} pattern size mismatch: expected {}, got {}", template_id, expected, actual)
            }
        }
    }
}

impl Error for OrchestratorError {}

// =============================================================================
// RATIONAL ERRORS
// =============================================================================

/// Errors in rational arithmetic
#[derive(Debug, Clone)]
pub enum RationalError {
    /// Bounds exceeded (2PQ >= M)
    BoundsExceeded {
        p_bound: u64,
        q_bound: u64,
        modulus: u128,
        safety_ratio: f64,
    },
    /// Reconstruction failed
    ReconstructionFailed {
        residue: u128,
        p_bound: u64,
        q_bound: u64,
        reason: String,
    },
    /// Division by zero
    DivisionByZero,
    /// Denominator not coprime to modulus
    DenominatorNotCoprime {
        denominator: u64,
        modulus_factor: u64,
    },
    /// Scaling failed
    ScalingFailed {
        reason: String,
    },
    /// Sign determination failed
    SignUndetermined {
        residue: u64,
        anchor_modulus: u64,
        p_bound: u64,
    },
}

impl fmt::Display for RationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RationalError::BoundsExceeded { p_bound, q_bound, modulus, safety_ratio } => {
                write!(f, "Bounds exceeded: 2*{}*{} / {} = {:.2} >= 1", 
                    p_bound, q_bound, modulus, safety_ratio)
            }
            RationalError::ReconstructionFailed { residue, p_bound, q_bound, reason } => {
                write!(f, "Reconstruction failed for r={} (P={}, Q={}): {}", 
                    residue, p_bound, q_bound, reason)
            }
            RationalError::DivisionByZero => {
                write!(f, "Division by zero in rational arithmetic")
            }
            RationalError::DenominatorNotCoprime { denominator, modulus_factor } => {
                write!(f, "Denominator {} shares factor {} with modulus", denominator, modulus_factor)
            }
            RationalError::ScalingFailed { reason } => {
                write!(f, "CRT scaling failed: {}", reason)
            }
            RationalError::SignUndetermined { residue, anchor_modulus, p_bound } => {
                write!(f, "Sign undetermined: |r|={} may exceed anchor {}/2 (P={})", 
                    residue, anchor_modulus, p_bound)
            }
        }
    }
}

impl Error for RationalError {}

// =============================================================================
// CONFIGURATION ERRORS
// =============================================================================

/// Errors in configuration
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Invalid parameter value
    InvalidParameter {
        name: &'static str,
        value: String,
        requirement: String,
    },
    /// Missing required parameter
    MissingParameter {
        name: &'static str,
    },
    /// Incompatible parameters
    IncompatibleParameters {
        param1: &'static str,
        param2: &'static str,
        reason: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidParameter { name, value, requirement } => {
                write!(f, "Invalid {}: '{}' (must be {})", name, value, requirement)
            }
            ConfigError::MissingParameter { name } => {
                write!(f, "Missing required parameter: {}", name)
            }
            ConfigError::IncompatibleParameters { param1, param2, reason } => {
                write!(f, "Incompatible {} and {}: {}", param1, param2, reason)
            }
        }
    }
}

impl Error for ConfigError {}

// =============================================================================
// RESOURCE ERRORS
// =============================================================================

/// Resource exhaustion errors
#[derive(Debug, Clone)]
pub enum ResourceError {
    /// Memory allocation failed
    OutOfMemory {
        requested: usize,
        available: Option<usize>,
    },
    /// Timeout exceeded
    Timeout {
        operation: &'static str,
        elapsed_ms: u64,
        limit_ms: u64,
    },
    /// Too many iterations
    IterationLimit {
        operation: &'static str,
        iterations: usize,
        limit: usize,
    },
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::OutOfMemory { requested, available } => {
                match available {
                    Some(avail) => write!(f, "Out of memory: requested {} bytes, {} available", requested, avail),
                    None => write!(f, "Out of memory: requested {} bytes", requested),
                }
            }
            ResourceError::Timeout { operation, elapsed_ms, limit_ms } => {
                write!(f, "Timeout in {}: {}ms > {}ms limit", operation, elapsed_ms, limit_ms)
            }
            ResourceError::IterationLimit { operation, iterations, limit } => {
                write!(f, "Iteration limit in {}: {} > {} limit", operation, iterations, limit)
            }
        }
    }
}

impl Error for ResourceError {}

// =============================================================================
// ERROR CONVERSION TRAITS
// =============================================================================

impl From<ArithmeticError> for QMNFError {
    fn from(e: ArithmeticError) -> Self {
        QMNFError::Arithmetic(e)
    }
}

impl From<EPRAMError> for QMNFError {
    fn from(e: EPRAMError) -> Self {
        QMNFError::EPRAM(e)
    }
}

impl From<OrchestratorError> for QMNFError {
    fn from(e: OrchestratorError) -> Self {
        QMNFError::Orchestrator(e)
    }
}

impl From<RationalError> for QMNFError {
    fn from(e: RationalError) -> Self {
        QMNFError::Rational(e)
    }
}

impl From<ConfigError> for QMNFError {
    fn from(e: ConfigError) -> Self {
        QMNFError::Config(e)
    }
}

impl From<ResourceError> for QMNFError {
    fn from(e: ResourceError) -> Self {
        QMNFError::Resource(e)
    }
}

// =============================================================================
// RESULT TYPE ALIASES
// =============================================================================

/// Result type for QMNF operations
pub type QMNFResult<T> = Result<T, QMNFError>;

/// Result type for arithmetic operations
pub type ArithmeticResult<T> = Result<T, ArithmeticError>;

/// Result type for EPRAM operations
pub type EPRAMResult<T> = Result<T, EPRAMError>;

/// Result type for orchestrator operations
pub type OrchestratorResult<T> = Result<T, OrchestratorError>;

/// Result type for rational operations
pub type RationalResult<T> = Result<T, RationalError>;

// =============================================================================
// ERROR RECOVERY SUGGESTIONS
// =============================================================================

impl QMNFError {
    /// Get recovery suggestion for this error
    pub fn recovery_suggestion(&self) -> &'static str {
        match self {
            QMNFError::Arithmetic(ArithmeticError::DivisionByZero { .. }) => {
                "Check for zero divisors before division operations"
            }
            QMNFError::Arithmetic(ArithmeticError::NoInverse { .. }) => {
                "Ensure value is coprime to modulus, or use different modulus"
            }
            QMNFError::Arithmetic(ArithmeticError::NotCoprime { .. }) => {
                "Use coprime moduli for CRT operations"
            }
            QMNFError::EPRAM(EPRAMError::ConvergenceFailure { .. }) => {
                "Increase max_steps or check for oscillating states"
            }
            QMNFError::Orchestrator(OrchestratorError::NoTemplates) => {
                "Add templates using add_template() or one_shot_learn()"
            }
            QMNFError::Orchestrator(OrchestratorError::LowConfidence { .. }) => {
                "Add more distinctive templates or adjust confidence threshold"
            }
            QMNFError::Rational(RationalError::BoundsExceeded { .. }) => {
                "Enable automatic CRT scaling or reduce operation chain length"
            }
            QMNFError::Rational(RationalError::ReconstructionFailed { .. }) => {
                "Check that 2PQ < M invariant is maintained"
            }
            QMNFError::Resource(ResourceError::Timeout { .. }) => {
                "Increase timeout limit or optimize operation"
            }
            _ => "Review error details and adjust parameters"
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let e = ArithmeticError::DivisionByZero {
            operation: "k_elimination",
            context: "divisor was zero".to_string(),
        };
        
        let msg = format!("{}", e);
        assert!(msg.contains("Division by zero"));
        assert!(msg.contains("k_elimination"));
    }
    
    #[test]
    fn test_error_conversion() {
        let arith_err = ArithmeticError::NoInverse {
            value: 4,
            modulus: 8,
            reason: "gcd(4,8) = 4 > 1".to_string(),
        };
        
        let qmnf_err: QMNFError = arith_err.into();
        
        assert!(matches!(qmnf_err, QMNFError::Arithmetic(_)));
    }
    
    #[test]
    fn test_error_source_chain() {
        let e = QMNFError::EPRAM(EPRAMError::ConvergenceFailure {
            steps: 100,
            max_steps: 100,
            final_lyapunov: 50,
            threshold: 0,
        });
        
        assert!(e.source().is_some());
    }
    
    #[test]
    fn test_recovery_suggestion() {
        let e = QMNFError::Rational(RationalError::BoundsExceeded {
            p_bound: 1000,
            q_bound: 1000,
            modulus: 1000000,
            safety_ratio: 2.0,
        });
        
        let suggestion = e.recovery_suggestion();
        assert!(suggestion.contains("scaling") || suggestion.contains("reduce"));
    }
}
