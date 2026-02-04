//! QMNF Error Handling Framework
//!
//! Comprehensive error types for the QMNF/EPRAM system.
//! Design principle: NO PANICS in production paths.
//! All errors are recoverable and informative.

use std::fmt;

/// Top-level QMNF error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QMNFError {
    /// EPRAM-related errors
    EPRAM(EPRAMError),
    /// Orchestrator errors
    Orchestrator(OrchestratorError),
    /// Rational arithmetic errors
    Rational(RationalError),
    /// Circular arithmetic errors
    Circular(CircularError),
    /// Montgomery arithmetic errors
    Montgomery(MontgomeryError),
    /// CRT-related errors
    CRT(CRTError),
    /// Generic validation error
    Validation(ValidationError),
}

impl fmt::Display for QMNFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QMNFError::EPRAM(e) => write!(f, "EPRAM error: {}", e),
            QMNFError::Orchestrator(e) => write!(f, "Orchestrator error: {}", e),
            QMNFError::Rational(e) => write!(f, "Rational error: {}", e),
            QMNFError::Circular(e) => write!(f, "Circular error: {}", e),
            QMNFError::Montgomery(e) => write!(f, "Montgomery error: {}", e),
            QMNFError::CRT(e) => write!(f, "CRT error: {}", e),
            QMNFError::Validation(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for QMNFError {}

/// Result type alias for QMNF operations
pub type QMNFResult<T> = Result<T, QMNFError>;

// ============================================================================
// EPRAM Errors
// ============================================================================

/// Errors related to EPRAM field operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EPRAMError {
    /// Cell count doesn't match target count
    CellTargetMismatch { cells: usize, targets: usize },
    /// Invalid topology for cell count
    InvalidTopology { cell_count: usize, topology: String },
    /// Convergence failed within step limit
    ConvergenceTimeout { steps: usize, max_steps: usize },
    /// Invalid cell index
    InvalidCellIndex { index: usize, max: usize },
    /// Invalid modulus (must be > 0)
    InvalidModulus { modulus: u64 },
    /// Cell value out of range
    ValueOutOfRange { value: u64, modulus: u64 },
    /// Empty field
    EmptyField,
}

impl fmt::Display for EPRAMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EPRAMError::CellTargetMismatch { cells, targets } => {
                write!(f, "Cell count ({}) doesn't match target count ({})", cells, targets)
            }
            EPRAMError::InvalidTopology { cell_count, topology } => {
                write!(f, "Topology '{}' invalid for {} cells", topology, cell_count)
            }
            EPRAMError::ConvergenceTimeout { steps, max_steps } => {
                write!(f, "Convergence failed after {} steps (max: {})", steps, max_steps)
            }
            EPRAMError::InvalidCellIndex { index, max } => {
                write!(f, "Cell index {} out of range [0, {})", index, max)
            }
            EPRAMError::InvalidModulus { modulus } => {
                write!(f, "Invalid modulus: {} (must be > 0)", modulus)
            }
            EPRAMError::ValueOutOfRange { value, modulus } => {
                write!(f, "Value {} out of range [0, {})", value, modulus)
            }
            EPRAMError::EmptyField => {
                write!(f, "Cannot operate on empty field")
            }
        }
    }
}

impl std::error::Error for EPRAMError {}

impl From<EPRAMError> for QMNFError {
    fn from(e: EPRAMError) -> Self {
        QMNFError::EPRAM(e)
    }
}

// ============================================================================
// Orchestrator Errors
// ============================================================================

/// Errors related to orchestrator operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrchestratorError {
    /// No templates registered
    NoTemplates,
    /// Template not found
    TemplateNotFound { id: String },
    /// Decision failed to converge
    DecisionTimeout { steps: usize },
    /// Invalid state transition
    InvalidTransition { from: String, to: String },
    /// Rail violation detected
    RailViolation { rail_id: u64, value: u64 },
    /// Void region entered
    VoidRegion { position: u64 },
    /// Learning failed
    LearningFailed { reason: String },
}

impl fmt::Display for OrchestratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrchestratorError::NoTemplates => {
                write!(f, "No templates registered")
            }
            OrchestratorError::TemplateNotFound { id } => {
                write!(f, "Template '{}' not found", id)
            }
            OrchestratorError::DecisionTimeout { steps } => {
                write!(f, "Decision failed to converge after {} steps", steps)
            }
            OrchestratorError::InvalidTransition { from, to } => {
                write!(f, "Invalid state transition from '{}' to '{}'", from, to)
            }
            OrchestratorError::RailViolation { rail_id, value } => {
                write!(f, "Rail {} violated with value {}", rail_id, value)
            }
            OrchestratorError::VoidRegion { position } => {
                write!(f, "Entered void region at position {}", position)
            }
            OrchestratorError::LearningFailed { reason } => {
                write!(f, "Learning failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for OrchestratorError {}

impl From<OrchestratorError> for QMNFError {
    fn from(e: OrchestratorError) -> Self {
        QMNFError::Orchestrator(e)
    }
}

// ============================================================================
// Rational Errors
// ============================================================================

/// Errors related to bounded rational arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RationalError {
    /// Reconstruction bound violated: 2PQ >= M
    BoundViolation { p_bound: u64, q_bound: u64, modulus: u64 },
    /// Denominator is zero
    DivisionByZero,
    /// Reconstruction failed (non-unique)
    ReconstructionFailed { residue: u64, modulus: u64 },
    /// Overflow in bound tracking
    BoundOverflow { operation: String, result_bound: u128 },
    /// Sign determination failed
    SignAmbiguous { value: i64, threshold: u64 },
    /// GCD requirement violated (p, q not coprime)
    NotCoprime { p: i64, q: u64 },
    /// Scaling failed
    ScalingFailed { reason: String },
}

impl fmt::Display for RationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RationalError::BoundViolation { p_bound, q_bound, modulus } => {
                write!(f, "Bound violation: 2*{}*{} >= {} (reconstruction not unique)",
                       p_bound, q_bound, modulus)
            }
            RationalError::DivisionByZero => {
                write!(f, "Division by zero in rational arithmetic")
            }
            RationalError::ReconstructionFailed { residue, modulus } => {
                write!(f, "Failed to reconstruct rational from residue {} mod {}", residue, modulus)
            }
            RationalError::BoundOverflow { operation, result_bound } => {
                write!(f, "Bound overflow in {}: result bound {} exceeds u64", operation, result_bound)
            }
            RationalError::SignAmbiguous { value, threshold } => {
                write!(f, "Sign ambiguous: |{}| near threshold {}", value, threshold)
            }
            RationalError::NotCoprime { p, q } => {
                write!(f, "Rational {}/{} not in lowest terms", p, q)
            }
            RationalError::ScalingFailed { reason } => {
                write!(f, "Scaling failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for RationalError {}

impl From<RationalError> for QMNFError {
    fn from(e: RationalError) -> Self {
        QMNFError::Rational(e)
    }
}

// ============================================================================
// Circular Errors
// ============================================================================

/// Errors related to circular/modular arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircularError {
    /// Cluster too spread for meaningful mean
    ClusterTooSpread { max_distance: u64, threshold: u64 },
    /// Empty value set
    EmptyValues,
    /// Invalid modulus
    InvalidModulus { modulus: u64 },
    /// Value out of range
    ValueOutOfRange { value: u64, modulus: u64 },
}

impl fmt::Display for CircularError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircularError::ClusterTooSpread { max_distance, threshold } => {
                write!(f, "Cluster too spread: max distance {} >= threshold {}",
                       max_distance, threshold)
            }
            CircularError::EmptyValues => {
                write!(f, "Cannot compute on empty value set")
            }
            CircularError::InvalidModulus { modulus } => {
                write!(f, "Invalid modulus: {}", modulus)
            }
            CircularError::ValueOutOfRange { value, modulus } => {
                write!(f, "Value {} out of range [0, {})", value, modulus)
            }
        }
    }
}

impl std::error::Error for CircularError {}

impl From<CircularError> for QMNFError {
    fn from(e: CircularError) -> Self {
        QMNFError::Circular(e)
    }
}

// ============================================================================
// Montgomery Errors
// ============================================================================

/// Errors related to Montgomery arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MontgomeryError {
    /// Modulus must be odd for Montgomery
    EvenModulus { modulus: u64 },
    /// Modulus must be > 1
    InvalidModulus { modulus: u64 },
    /// Value exceeds modulus
    ValueTooLarge { value: u64, modulus: u64 },
    /// R^2 computation failed
    R2ComputationFailed { modulus: u64 },
    /// Inverse computation failed
    InverseNotFound { value: u64, modulus: u64 },
}

impl fmt::Display for MontgomeryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MontgomeryError::EvenModulus { modulus } => {
                write!(f, "Montgomery requires odd modulus, got {}", modulus)
            }
            MontgomeryError::InvalidModulus { modulus } => {
                write!(f, "Invalid modulus {} (must be > 1)", modulus)
            }
            MontgomeryError::ValueTooLarge { value, modulus } => {
                write!(f, "Value {} >= modulus {}", value, modulus)
            }
            MontgomeryError::R2ComputationFailed { modulus } => {
                write!(f, "Failed to compute R^2 for modulus {}", modulus)
            }
            MontgomeryError::InverseNotFound { value, modulus } => {
                write!(f, "No inverse for {} mod {}", value, modulus)
            }
        }
    }
}

impl std::error::Error for MontgomeryError {}

impl From<MontgomeryError> for QMNFError {
    fn from(e: MontgomeryError) -> Self {
        QMNFError::Montgomery(e)
    }
}

// ============================================================================
// CRT Errors
// ============================================================================

/// Errors related to Chinese Remainder Theorem operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CRTError {
    /// No moduli provided
    NoModuli,
    /// Moduli not coprime
    ModuliNotCoprime { m1: u64, m2: u64, gcd: u64 },
    /// Residue vector length mismatch
    ResidueCountMismatch { residues: usize, moduli: usize },
    /// Prime generation failed
    PrimeGenerationFailed { bits: usize },
    /// Overflow in CRT reconstruction
    Overflow { operation: String },
    /// K-value tracking error
    KTrackingError { expected: u64, actual: u64 },
}

impl fmt::Display for CRTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CRTError::NoModuli => {
                write!(f, "No moduli provided for CRT")
            }
            CRTError::ModuliNotCoprime { m1, m2, gcd } => {
                write!(f, "Moduli {} and {} not coprime (gcd = {})", m1, m2, gcd)
            }
            CRTError::ResidueCountMismatch { residues, moduli } => {
                write!(f, "Residue count ({}) doesn't match moduli count ({})",
                       residues, moduli)
            }
            CRTError::PrimeGenerationFailed { bits } => {
                write!(f, "Failed to generate {}-bit prime", bits)
            }
            CRTError::Overflow { operation } => {
                write!(f, "Overflow in CRT operation: {}", operation)
            }
            CRTError::KTrackingError { expected, actual } => {
                write!(f, "K-tracking error: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for CRTError {}

impl From<CRTError> for QMNFError {
    fn from(e: CRTError) -> Self {
        QMNFError::CRT(e)
    }
}

// ============================================================================
// Validation Errors
// ============================================================================

/// Generic validation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub context: String,
    pub message: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl ValidationError {
    pub fn new(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            message: message.into(),
            expected: None,
            actual: None,
        }
    }
    
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }
    
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.context, self.message)?;
        if let Some(ref exp) = self.expected {
            write!(f, " (expected: {})", exp)?;
        }
        if let Some(ref act) = self.actual {
            write!(f, " (actual: {})", act)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

impl From<ValidationError> for QMNFError {
    fn from(e: ValidationError) -> Self {
        QMNFError::Validation(e)
    }
}

// ============================================================================
// Error utilities
// ============================================================================

/// Check a condition, return error if false
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err.into());
        }
    };
}

/// Check that a value is in range [0, max)
pub fn check_range(value: u64, max: u64, context: &str) -> QMNFResult<()> {
    if value >= max {
        Err(QMNFError::Validation(ValidationError::new(
            context,
            "Value out of range",
        ).with_expected(format!("[0, {})", max))
          .with_actual(value.to_string())))
    } else {
        Ok(())
    }
}

/// Check that modulus is valid (positive)
pub fn check_modulus(modulus: u64, context: &str) -> QMNFResult<()> {
    if modulus == 0 {
        Err(QMNFError::Validation(ValidationError::new(
            context,
            "Modulus must be positive",
        ).with_actual("0".to_string())))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let e = EPRAMError::CellTargetMismatch { cells: 5, targets: 3 };
        assert!(e.to_string().contains("5"));
        assert!(e.to_string().contains("3"));
    }
    
    #[test]
    fn test_error_conversion() {
        let epram_err = EPRAMError::EmptyField;
        let qmnf_err: QMNFError = epram_err.into();
        assert!(matches!(qmnf_err, QMNFError::EPRAM(_)));
    }
    
    #[test]
    fn test_validation_error_builder() {
        let err = ValidationError::new("test", "something wrong")
            .with_expected("good")
            .with_actual("bad");
        
        let s = err.to_string();
        assert!(s.contains("test"));
        assert!(s.contains("something wrong"));
        assert!(s.contains("good"));
        assert!(s.contains("bad"));
    }
    
    #[test]
    fn test_check_range() {
        assert!(check_range(5, 10, "test").is_ok());
        assert!(check_range(10, 10, "test").is_err());
        assert!(check_range(0, 10, "test").is_ok());
    }
    
    #[test]
    fn test_check_modulus() {
        assert!(check_modulus(256, "test").is_ok());
        assert!(check_modulus(1, "test").is_ok());
        assert!(check_modulus(0, "test").is_err());
    }
}
