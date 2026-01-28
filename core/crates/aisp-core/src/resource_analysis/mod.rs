//! Resource Analysis Module
//!
//! Comprehensive resource utilization analysis split into focused,
//! SRP-compliant modules under 300 LOC each.

pub mod types;
pub mod utilization;
pub mod allocation;
pub mod bottlenecks;
pub mod forecasting;
pub mod analyzer;

// Re-export key types for convenience
pub use types::*;
pub use utilization::ResourceUtilizationAnalysis;
pub use allocation::AllocationAnalysis;
pub use bottlenecks::BottleneckAnalysis;
pub use forecasting::ResourceForecasting;
pub use analyzer::ResourceUtilizationAnalyzer;