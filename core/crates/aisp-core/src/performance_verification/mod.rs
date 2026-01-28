//! Performance Verification Module
//!
//! Comprehensive performance constraint verification system split into
//! focused, SRP-compliant modules under 300 LOC each.

pub mod types;
pub mod timing;
pub mod throughput;
pub mod resources;
pub mod qos;
pub mod sla;
pub mod degradation;
pub mod verifier;

#[cfg(test)]
mod integration_test;

// Re-export key types for convenience
pub use types::*;
pub use timing::TimingConstraintAnalysis;
pub use throughput::ThroughputAnalysis;
pub use resources::ResourceBoundAnalysis;
pub use qos::QoSAnalysis;
pub use sla::SLACompliance;
pub use degradation::PerformanceDegradationAnalysis;
pub use verifier::PerformanceConstraintVerifier;