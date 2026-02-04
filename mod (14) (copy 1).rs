//! QMNF/EPRAM Production Module
//! 
//! Production-ready components:
//! - Comprehensive error handling
//! - Performance benchmarks
//! - Regression testing
//! - End-to-end tests
//! - CI/CD pipeline

pub mod error;
pub mod benchmarks;
pub mod regression;
pub mod e2e_tests;

pub use error::*;

/// Run all production validations
pub fn validate_production() -> ProductionValidation {
    let benchmark_results = benchmarks::run_all_benchmarks();
    let e2e_results = e2e_tests::run_all_e2e_tests();
    
    let benchmarks_passed = benchmark_results.iter().all(|s| s.all_passed());
    let e2e_passed = e2e_results.iter().all(|r| r.passed);
    
    ProductionValidation {
        benchmarks_passed,
        e2e_passed,
        ready_for_production: benchmarks_passed && e2e_passed,
    }
}

#[derive(Debug)]
pub struct ProductionValidation {
    pub benchmarks_passed: bool,
    pub e2e_passed: bool,
    pub ready_for_production: bool,
}
